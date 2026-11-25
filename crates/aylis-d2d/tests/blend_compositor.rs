//! Os dois compositores da mistura (`blend_compositor`): o efeito Blend do Direct2D e o shader
//! do Aylis em Direct3D 11 dão a mesma cor que o Skia em todos os modos, na placa de vídeo
//! (a captura na placa, o caminho da janela) e com muitas camadas seguidas, cada uma sobre o
//! próprio fundo.
#![cfg(windows)]

use aylis_core::gpu::{BlendCompositor, set_blend_compositor};
use aylis_core::{BlendMode, Brush, Canvas, Color, PropEnum, RectF};

/// O compositor e o contador são do processo: um teste por vez.
static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// O tamanho de cada célula; uma coluna por modo e uma linha por cor de fundo.
const CELL: f32 = 24.0;
const BACKGROUNDS: [Color; 6] = [
    Color::rgb(255, 0, 0),
    Color::rgb(0, 160, 255),
    Color::rgb(0, 0, 0),
    Color::rgb(255, 255, 255),
    Color::rgb(128, 128, 128),
    Color::rgb(40, 200, 90),
];

/// Uma célula por modo e por fundo: o fundo, e por cima, na camada misturada, um quadrado da
/// cor `mark` (com `opacity`).
fn paint(c: &mut dyn Canvas, mark: Color, opacity: f32) {
    for (row, &background) in BACKGROUNDS.iter().enumerate() {
        for (col, &mode) in BlendMode::ALL.iter().enumerate() {
            let cell = RectF::new(col as f32 * CELL, row as f32 * CELL, CELL, CELL);
            c.fill_rect(cell, 0.0, &Brush::Solid(background));
            c.save_layer_blend(opacity, mode, cell);
            c.fill_rect(RectF::new(cell.x + 4.0, cell.y + 4.0, CELL - 8.0, CELL - 8.0), 0.0, &Brush::Solid(mark));
            c.restore();
        }
    }
}

fn size() -> (u32, u32) {
    ((BlendMode::ALL.len() as f32 * CELL) as u32, (BACKGROUNDS.len() as f32 * CELL) as u32)
}

/// A cor do meio de cada célula e de um canto fora da marca.
fn samples(px: &[u8], w: u32) -> Vec<(String, [u8; 3])> {
    let mut out = Vec::new();
    for row in 0..BACKGROUNDS.len() {
        for (col, mode) in BlendMode::ALL.iter().enumerate() {
            for (dx, dy, place) in [(12.0, 12.0, "meio"), (1.0, 1.0, "canto")] {
                let (x, y) = ((col as f32 * CELL + dx) as usize, (row as f32 * CELL + dy) as usize);
                let at = (y * w as usize + x) * 4;
                out.push((format!("{} no fundo {row}, {place}", mode.name()), [px[at], px[at + 1], px[at + 2]]));
            }
        }
    }
    out
}

fn compare(name: &str, got: &[(String, [u8; 3])], reference: &[(String, [u8; 3])]) {
    for ((what, g), (_, r)) in got.iter().zip(reference) {
        assert!(g.iter().zip(r).all(|(a, b)| a.abs_diff(*b) <= 8), "{name}, {what}: {g:?} × Skia {r:?}");
    }
}

/// Os dois compositores na placa, contra o Skia (na CPU), com uma cor cheia e uma do meio, com
/// e sem opacidade.
#[test]
fn both_compositors_agree_with_skia_on_every_mode() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    let (w, h) = size();
    for (mark, opacity) in [(Color::WHITE, 1.0), (Color::rgb(64, 128, 192), 1.0), (Color::rgb(64, 128, 192), 0.5)] {
        let reference = samples(&skia.render_rgba(w, h, 1.0, Color::WHITE, |c| paint(c, mark, opacity)).unwrap(), w);
        for compositor in [BlendCompositor::Direct2d, BlendCompositor::Direct3d] {
            set_blend_compositor(compositor);
            let before = aylis_d2d::shader_blends();
            let Some(px) = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, mark, opacity)) else {
                eprintln!("sem placa de vídeo: pulado");
                return;
            };
            let shaded = aylis_d2d::shader_blends() - before;
            let name = format!("{} com {mark:?} a {opacity}", compositor.name());
            compare(&name, &samples(&px, w), &reference);
            // O `normal` não abre camada; todos os outros passam pelo shader no Direct3D.
            let blended = ((BlendMode::ALL.len() - 1) * BACKGROUNDS.len()) as u64;
            match compositor {
                BlendCompositor::Direct3d => assert_eq!(shaded, blended, "{name}: o shader juntou {shaded} de {blended} camadas"),
                BlendCompositor::Direct2d => assert_eq!(shaded, 0, "{name}: o shader rodou no lugar do efeito"),
            }
        }
    }
    set_blend_compositor(BlendCompositor::Direct2d);
}

/// Os recortes de fora da camada valem nos dois: o retângulo vira o *scissor* do shader; o
/// arredondado (que o shader não sabe recortar) cai no efeito do Direct2D, com a mesma cor.
#[test]
fn the_clips_outside_the_layer_hold_with_the_shader() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    let (w, h) = (120u32, 60u32);
    let paint = |c: &mut dyn Canvas, rounded: bool| {
        c.fill_rect(RectF::new(0.0, 0.0, 120.0, 60.0), 0.0, &Brush::Solid(Color::rgb(255, 0, 0)));
        c.save();
        let clip = RectF::new(20.0, 10.0, 60.0, 40.0);
        if rounded { c.clip_round_rect(clip, 12.0) } else { c.clip_rect(clip) }
        c.save_layer_blend(1.0, BlendMode::Difference, RectF::new(0.0, 0.0, 120.0, 60.0));
        c.fill_rect(RectF::new(0.0, 0.0, 120.0, 60.0), 0.0, &Brush::Solid(Color::WHITE));
        c.restore();
        c.restore();
    };
    // Dentro do recorte, fora dele e (no arredondado) no canto cortado.
    let points = [(50usize, 30usize), (5, 5), (100, 50), (21, 11)];
    let pick = |px: &[u8]| points.iter().map(|&(x, y)| { let at = (y * w as usize + x) * 4; [px[at], px[at + 1], px[at + 2]] }).collect::<Vec<_>>();
    for rounded in [false, true] {
        let reference = pick(&skia.render_rgba(w, h, 1.0, Color::WHITE, |c| paint(c, rounded)).unwrap());
        assert_eq!(reference[0], [0, 255, 255], "o Skia inverte dentro");
        set_blend_compositor(BlendCompositor::Direct3d);
        let before = aylis_d2d::shader_blends();
        let Some(px) = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, rounded)) else { return };
        let shaded = aylis_d2d::shader_blends() - before;
        set_blend_compositor(BlendCompositor::Direct2d);
        for (i, (g, r)) in pick(&px).iter().zip(&reference).enumerate() {
            assert!(g.iter().zip(r).all(|(a, b)| a.abs_diff(*b) <= 40), "arredondado {rounded}, ponto {:?}: {g:?} × Skia {r:?}", points[i]);
        }
        assert_eq!(shaded, if rounded { 0 } else { 1 }, "arredondado {rounded}: o shader juntou {shaded} camadas");
    }
}
