//! O Direct2D e o Skia lado a lado: o espaço entre letras do `TextStyle`, a medida com quebra do
//! `Canvas` e a camada de opacidade com uma mistura dentro.
#![cfg(windows)]

use aylis_core::{BlendMode, Brush, Canvas, Color, GradientStop, PointF, RectF, TextAlign, TextStyle};

const W: u32 = 240;
const H: u32 = 80;

/// Pinta o mesmo nos dois: (Direct2D, Skia), em RGBA.
fn both(w: u32, h: u32, paint: impl Fn(&mut dyn Canvas)) -> (Vec<u8>, Vec<u8>) {
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    (d2d.render_rgba(w, h, 1.0, Color::WHITE, |c| paint(c)).unwrap(), skia.render_rgba(w, h, 1.0, Color::WHITE, |c| paint(c)).unwrap())
}

/// A primeira e a última coluna com tinta (um pixel bem escuro).
fn ink_columns(px: &[u8], w: u32) -> (usize, usize) {
    let dark = |x: usize| px.chunks(4).skip(x).step_by(w as usize).any(|p| p[0] < 100);
    let first = (0..w as usize).find(|&x| dark(x)).unwrap_or(0);
    let last = (0..w as usize).rev().find(|&x| dark(x)).unwrap_or(0);
    (first, last)
}

/// A última linha com tinta.
fn ink_bottom(px: &[u8], w: u32) -> usize {
    px.chunks(4 * w as usize).rposition(|row| row.chunks(4).any(|p| p[0] < 100)).unwrap_or(0)
}

fn style(spacing: f32) -> TextStyle {
    let mut s = TextStyle::default();
    s.font.size = 20.0;
    s.letter_spacing = spacing;
    s
}

/// Item 70: o `letter_spacing` abre `espaço × (letras - 1)` na medida e no desenho, igual nos
/// dois, também centrado (a largura não conta o espaço depois da última letra).
#[test]
fn letter_spacing_widens_the_text_the_same_in_both() {
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    let mut widths = Vec::new();
    for spacing in [0.0, 4.0, -1.0] {
        let mut a = 0.0;
        d2d.render_rgba(10, 10, 1.0, Color::WHITE, |c| a = c.measure_text("HELLO", &style(spacing)).width).unwrap();
        let mut b = 0.0;
        skia.render_rgba(10, 10, 1.0, Color::WHITE, |c| b = c.measure_text("HELLO", &style(spacing)).width).unwrap();
        widths.push((a, b));
    }
    for (i, spacing) in [4.0f32, -1.0].into_iter().enumerate() {
        let ((a0, b0), (a, b)) = (widths[0], widths[i + 1]);
        assert!((a - a0 - spacing * 4.0).abs() < 0.6, "Direct2D: {a0} → {a} com {spacing}");
        assert!((b - b0 - spacing * 4.0).abs() < 0.6, "Skia: {b0} → {b} com {spacing}");
    }
    // O desenho: a tinta anda o mesmo, à esquerda e no meio.
    for align in [TextAlign::Leading, TextAlign::Center, TextAlign::Trailing] {
        let paint = |spacing: f32| {
            move |c: &mut dyn Canvas| {
                let mut s = style(spacing);
                s.color = Color::BLACK;
                s.h_align = align;
                c.draw_text("HELLO", RectF::new(10.0, 0.0, W as f32 - 20.0, H as f32), &s);
            }
        };
        let (a0, b0) = both(W, H, paint(0.0));
        let (a, b) = both(W, H, paint(6.0));
        let (ia0, ib0, ia, ib) = (ink_columns(&a0, W), ink_columns(&b0, W), ink_columns(&a, W), ink_columns(&b, W));
        let grown = |x: (usize, usize), y: (usize, usize)| (y.1 - y.0) as f32 - (x.1 - x.0) as f32;
        assert!((grown(ia0, ia) - 24.0).abs() <= 2.0, "Direct2D {align:?}: {ia0:?} → {ia:?}");
        assert!((grown(ib0, ib) - 24.0).abs() <= 2.0, "Skia {align:?}: {ib0:?} → {ib:?}");
        assert!(ia.0.abs_diff(ib.0) <= 2 && ia.1.abs_diff(ib.1) <= 2, "{align:?}: Direct2D {ia:?}, Skia {ib:?}");
    }
}

const LONG: &str = "Um texto comprido que quebra em várias linhas, medido antes de desenhar";

/// Item 70: o `measure_text_wrapped` dá o tamanho que o `draw_text` com quebra ocupa (a mesma
/// regra de quebra): a tinta cabe na largura e acaba na última linha medida. Nos dois, e com o
/// espaço entre letras.
#[test]
fn measure_text_wrapped_is_the_size_draw_text_takes() {
    for spacing in [0.0, 1.5] {
        let mut s = style(spacing);
        s.color = Color::BLACK;
        s.wrap = true;
        s.v_align = TextAlign::Leading;
        let d2d = aylis_d2d::D2dRenderer::new().unwrap();
        let skia = aylis_skia::SkiaRenderer::new();
        let (mut sa, mut sb, mut la, mut lb) = (Default::default(), Default::default(), 0.0, 0.0);
        let a = d2d
            .render_rgba(W, 200, 1.0, Color::WHITE, |c| {
                sa = c.measure_text_wrapped(LONG, &s, 150.0);
                assert_eq!(c.measure_text_wrapped(LONG, &s, 150.0), sa, "o guardado é o mesmo");
                la = c.measure_text("Ag", &s).height;
                c.draw_text(LONG, RectF::new(0.0, 0.0, 150.0, sa.height), &s);
            })
            .unwrap();
        let b = skia
            .render_rgba(W, 200, 1.0, Color::WHITE, |c| {
                sb = c.measure_text_wrapped(LONG, &s, 150.0);
                lb = c.measure_text("Ag", &s).height;
                c.draw_text(LONG, RectF::new(0.0, 0.0, 150.0, sb.height), &s);
            })
            .unwrap();
        for (name, px, size, line) in [("Direct2D", &a, sa, la), ("Skia", &b, sb, lb)] {
            let lines = (size.height / line).round();
            assert!(lines >= 3.0 && (size.height - lines * line).abs() < 0.01, "{name}: {size:?} em linhas de {line}");
            assert!(size.width <= 150.0 && size.width > 100.0, "{name}: {size:?}");
            let (_, right) = ink_columns(px, W);
            assert!(right as f32 <= size.width + 1.0, "{name}: a tinta vai até {right}, medido {}", size.width);
            let bottom = ink_bottom(px, W) as f32;
            assert!(bottom < size.height && bottom > size.height - line, "{name}: a tinta acaba em {bottom}, medido {}", size.height);
        }
        assert_eq!((sa.height / la).round(), (sb.height / lb).round(), "as mesmas linhas nos dois");
    }
}

/// Uma "imagem" colorida (um gradiente) de fundo.
fn picture(c: &mut dyn Canvas, r: RectF) {
    let stops = vec![GradientStop::new(0.0, Color::rgb(200, 40, 20)), GradientStop::new(1.0, Color::rgb(30, 90, 220))];
    c.fill_rect(r, 0.0, &Brush::Linear { start: PointF::new(r.x, r.y), end: PointF::new(r.right(), r.bottom()), stops });
}

/// A marca d'água em `difference`: um retângulo branco no meio.
fn watermark(c: &mut dyn Canvas, r: RectF) {
    c.save_layer_blend(1.0, BlendMode::Difference, r);
    c.fill_rect(RectF::new(r.x + 20.0, r.y + 20.0, r.width - 40.0, r.height - 40.0), 0.0, &Brush::Solid(Color::WHITE));
    c.restore();
}

/// O maior desvio de um canal entre as duas imagens.
fn max_diff(a: &[u8], b: &[u8]) -> u8 {
    a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0)
}

/// Os casos da marca d'água de quem pediu (a camada `save_layer(0.9)` com uma mistura
/// `difference` dentro), por cima de uma imagem: (nome, pintura).
fn watermark_cases() -> Vec<(&'static str, Box<dyn Fn(&mut dyn Canvas)>)> {
    let r = RectF::new(0.0, 0.0, 120.0, 80.0);
    vec![
        ("a mistura sozinha", Box::new(move |c: &mut dyn Canvas| {
            picture(c, r);
            watermark(c, r);
        })),
        ("a imagem fora, a marca numa camada de 90%", Box::new(move |c: &mut dyn Canvas| {
            picture(c, r);
            c.save_layer(0.9);
            watermark(c, r);
            c.restore();
        })),
        ("a imagem e a marca na camada de 90%", Box::new(move |c: &mut dyn Canvas| {
            c.save_layer(0.9);
            picture(c, r);
            watermark(c, r);
            c.restore();
        })),
    ]
}

/// A conferência do `save_layer` com uma mistura dentro: o Direct2D fechava a camada de
/// opacidade para copiar o fundo da mistura, e o que estava nela era composto duas vezes. O
/// resultado tem de ser o do Skia (e o do navegador e do CSS: a camada com opacidade é um grupo
/// isolado, e a mistura de dentro só se junta ao que está nela). Com a imagem dentro da camada, o Direct2D acerta a partir da segunda pintura (a
/// primeira mostra que a janela precisa de camadas isoladas).
#[test]
fn an_opacity_layer_with_a_blend_inside_is_the_same_as_in_skia() {
    let skia = aylis_skia::SkiaRenderer::new();
    let i = (40 * 120 + 60) * 4;
    for (k, (name, paint)) in watermark_cases().into_iter().enumerate() {
        // Um renderizador novo em cada caso: ele lembra que precisa de camadas isoladas.
        let d2d = aylis_d2d::D2dRenderer::new().unwrap();
        let b = skia.render_rgba(120, 80, 1.0, Color::WHITE, |c| paint(c)).unwrap();
        let first = d2d.render_rgba(120, 80, 1.0, Color::WHITE, |c| paint(c)).unwrap();
        let a = d2d.render_rgba(120, 80, 1.0, Color::WHITE, |c| paint(c)).unwrap();
        if k < 2 {
            assert!(max_diff(&first, &b) <= 3, "{name}, já na primeira pintura: Direct2D {:?}, Skia {:?}", &first[i..i + 3], &b[i..i + 3]);
        } else {
            // A reprodução: a imagem já estava na camada quando a mistura abriu.
            assert!(max_diff(&first, &b) > 30, "{name}: a primeira pintura mostra a diferença");
        }
        assert!(max_diff(&a, &b) <= 3, "{name}: Direct2D {:?}, Skia {:?} (no meio)", &a[i..i + 3], &b[i..i + 3]);
    }
}

/// O custo por quadro da camada isolada: 52 células com a imagem e a marca d'água numa camada de
/// 90% (a tela de câmeras de quem pediu), no Direct2D. `cargo test --release -p aylis-d2d --test
/// compare_skia -- --ignored --nocapture`.
#[test]
#[ignore]
fn the_cost_of_watermarks_in_opacity_layers() {
    use std::time::Instant;
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let cell = |i: usize| RectF::new((i % 8) as f32 * 240.0, (i / 8) as f32 * 135.0, 236.0, 131.0);
    let frame = |inside: bool| {
        move |c: &mut dyn Canvas| {
            for i in 0..52 {
                let r = cell(i);
                c.save();
                c.clip_rect(r);
                if inside {
                    c.save_layer(0.9);
                    picture(c, r);
                } else {
                    picture(c, r);
                    c.save_layer(0.9);
                }
                watermark(c, r);
                c.restore();
                c.restore();
            }
        }
    };
    for (name, inside) in [("a imagem fora da camada", false), ("a imagem dentro da camada", true)] {
        let paint = frame(inside);
        let _ = d2d.render_rgba(1920, 1080, 1.0, Color::WHITE, |c| paint(c));
        let mut best = f64::MAX;
        for _ in 0..10 {
            let t = Instant::now();
            let _ = d2d.render_rgba(1920, 1080, 1.0, Color::WHITE, |c| paint(c));
            best = best.min(t.elapsed().as_secs_f64() * 1000.0);
        }
        println!("Direct2D, 52 células 1920 × 1080, {name}: {best:.1} ms por quadro (o melhor de 10, com a cópia da GPU para a CPU)");
    }
}
