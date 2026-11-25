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
            let (before, fixed_before) = (aylis_d2d::shader_blends(), aylis_d2d::fixed_blends());
            let Some(px) = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, mark, opacity)) else {
                eprintln!("sem placa de vídeo: pulado");
                return;
            };
            let shaded = aylis_d2d::shader_blends() - before;
            let fixed = aylis_d2d::fixed_blends() - fixed_before;
            let name = format!("{} com {mark:?} a {opacity}", compositor.name());
            compare(&name, &samples(&px, w), &reference);
            // O `normal` não abre camada; a `exclusion` vai pela mistura fixa nos dois; os
            // outros passam pelo shader no Direct3D.
            let rows = BACKGROUNDS.len() as u64;
            let blended = (BlendMode::ALL.len() as u64 - 2) * rows;
            assert_eq!(fixed, rows, "{name}: a mistura fixa juntou {fixed} das {rows} camadas exclusion");
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

/// Uma cena com o fundo de várias cores (cheias e meio transparentes) e, por cima, uma camada
/// `mode` com um quadrado branco, um colorido e (com `opacity`) a camada inteira mais fraca.
fn scene(c: &mut dyn Canvas, mode: BlendMode, opacity: f32) {
    let colors = [Color::rgb(200, 100, 50), Color::rgb(0, 160, 255), Color::rgb(30, 30, 30), Color::rgba(40, 200, 90, 128)];
    for (i, &color) in colors.iter().enumerate() {
        c.fill_rect(RectF::new(i as f32 * 30.0, 0.0, 30.0, 60.0), 0.0, &Brush::Solid(color));
    }
    let bounds = RectF::new(0.0, 0.0, 120.0, 60.0);
    c.save_layer_blend(opacity, mode, bounds);
    c.fill_rect(RectF::new(10.0, 10.0, 100.0, 15.0), 0.0, &Brush::Solid(Color::WHITE));
    c.fill_rect(RectF::new(10.0, 35.0, 100.0, 15.0), 0.0, &Brush::Solid(Color::rgba(64, 128, 192, 200)));
    c.restore();
}

/// A maior diferença por canal entre duas imagens RGBA, só onde `inside` diz, e só nos pixels
/// com alfa (o RGB sem alfa não conta).
fn max_diff(a: &[u8], b: &[u8], w: u32, inside: impl Fn(u32, u32) -> bool) -> u8 {
    let mut worst = 0;
    for (i, (p, q)) in a.chunks_exact(4).zip(b.chunks_exact(4)).enumerate() {
        let (x, y) = (i as u32 % w, i as u32 / w);
        if !inside(x, y) || (p[3] == 0 && q[3] == 0) {
            continue;
        }
        // Com pouco alfa, o RGB sem a pré-multiplicação amplia o arredondamento.
        let channels = if p[3] < 64 { 3..4 } else { 0..4 };
        for k in channels {
            worst = worst.max(p[k].abs_diff(q[k]));
        }
    }
    worst
}

/// A `exclusion` pela mistura fixa da placa (sem copiar o fundo nem rodar o shader) dá o pixel
/// do caminho de antes (o efeito do Direct2D, aqui na CPU) e do Skia, com o fundo opaco ou
/// transparente e com opacidade; e só custa o `Flush`, sem a cópia.
#[test]
fn the_exclusion_without_copy_matches_the_effect_and_skia() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    let (w, h) = (120u32, 60u32);
    for compositor in [BlendCompositor::Direct2d, BlendCompositor::Direct3d] {
        set_blend_compositor(compositor);
        for background in [Color::WHITE, Color::TRANSPARENT] {
            for opacity in [1.0, 0.5] {
                let name = format!("{} sobre {background:?} a {opacity}", compositor.name());
                let (fixed, stats) = (aylis_d2d::fixed_blends(), aylis_d2d::blend_stats());
                let Some(gpu) = d2d.render_rgba_on_gpu(w, h, 1.0, background, |c| scene(c, BlendMode::Exclusion, opacity)) else {
                    eprintln!("sem placa de vídeo: pulado");
                    set_blend_compositor(BlendCompositor::Direct2d);
                    return;
                };
                let after = aylis_d2d::blend_stats();
                assert_eq!(aylis_d2d::fixed_blends() - fixed, 1, "{name}: não foi pela mistura fixa");
                assert_eq!((after.layers - stats.layers, after.flushes - stats.flushes, after.copies - stats.copies), (1, 1, 0), "{name}: uma camada, um Flush e nenhuma cópia");
                let effect = d2d.render_rgba(w, h, 1.0, background, |c| scene(c, BlendMode::Exclusion, opacity)).unwrap();
                let reference = skia.render_rgba(w, h, 1.0, background, |c| scene(c, BlendMode::Exclusion, opacity)).unwrap();
                let everywhere = |_, _| true;
                assert!(max_diff(&gpu, &effect, w, everywhere) <= 3, "{name}: a placa × o efeito do Direct2D ({})", max_diff(&gpu, &effect, w, everywhere));
                // O Skia devolve o fundo transparente com as cores pré-multiplicadas, e o
                // Direct2D sem: a comparação com ele fica no fundo opaco.
                if background.a == 255 {
                    assert!(max_diff(&gpu, &reference, w, everywhere) <= 3, "{name}: a placa × o Skia ({})", max_diff(&gpu, &reference, w, everywhere));
                }
            }
        }
    }
    set_blend_compositor(BlendCompositor::Direct2d);
}

/// Para texto (ou marca) branco, a `exclusion` dá o mesmo pixel da `difference`
/// (`|b − 1| = 1 − b`), com a borda suavizada inclusive: é a troca que a documentação do
/// `save_layer_blend` recomenda.
#[test]
fn white_text_in_exclusion_is_the_same_as_in_difference() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let (w, h) = (160u32, 48u32);
    let paint = |c: &mut dyn Canvas, mode: BlendMode| {
        for i in 0..8 {
            let shade = (i * 32) as u8;
            c.fill_rect(RectF::new(i as f32 * 20.0, 0.0, 20.0, 48.0), 0.0, &Brush::Solid(Color::rgb(shade, 255 - shade, 128)));
        }
        let bounds = RectF::new(0.0, 0.0, 160.0, 48.0);
        c.save_layer_blend(1.0, mode, bounds);
        let mut style = aylis_core::TextStyle { color: Color::WHITE, ..Default::default() };
        style.font.size = 28.0;
        c.draw_text("CÂMERA 07", bounds, &style);
        c.restore();
    };
    let Some(exclusion) = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, BlendMode::Exclusion)) else { return };
    let difference = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, BlendMode::Difference)).unwrap();
    let worst = max_diff(&exclusion, &difference, w, |_, _| true);
    assert!(worst <= 2, "exclusion × difference no texto branco: {worst}");
    assert_ne!(exclusion, d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| paint(c, BlendMode::Normal)).unwrap(), "o texto inverteu o fundo");
}

/// Na pintura por parte com várias áreas (a máscara da união aberta por baixo), a camada vai
/// pela placa com um *scissor* por área, e dentro das áreas o pixel é o da pintura inteira.
#[test]
fn the_blend_on_the_gpu_holds_with_several_areas() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let (w, h) = (120u32, 60u32);
    // A terceira cobre parte das outras duas: a mistura fixa não pode passar duas vezes ali.
    let areas = [RectF::new(5.0, 5.0, 40.0, 30.0), RectF::new(60.0, 20.0, 50.0, 35.0), RectF::new(30.0, 15.0, 40.0, 20.0)];
    let inside = |x: u32, y: u32| areas.iter().any(|a| (x as f32) >= a.x && (x as f32) < a.right() && (y as f32) >= a.y && (y as f32) < a.bottom());
    for (compositor, mode) in [(BlendCompositor::Direct2d, BlendMode::Exclusion), (BlendCompositor::Direct3d, BlendMode::Exclusion), (BlendCompositor::Direct3d, BlendMode::Difference)] {
        set_blend_compositor(compositor);
        let name = format!("{} em {}", mode.name(), compositor.name());
        let Some(whole) = d2d.render_rgba_on_gpu(w, h, 1.0, Color::WHITE, |c| scene(c, mode, 1.0)) else {
            set_blend_compositor(BlendCompositor::Direct2d);
            return;
        };
        let (fixed, shaded) = (aylis_d2d::fixed_blends(), aylis_d2d::shader_blends());
        let parts = d2d.render_areas_rgba_on_gpu(w, h, 1.0, Color::WHITE, &areas, |c| scene(c, mode, 1.0)).unwrap();
        let on_gpu = aylis_d2d::fixed_blends() - fixed + aylis_d2d::shader_blends() - shaded;
        assert_eq!(on_gpu, 1, "{name}: a camada não foi pela placa");
        let worst = max_diff(&parts, &whole, w, inside);
        assert!(worst <= 2, "{name}: por parte × inteira dentro das áreas ({worst})");
        assert!(parts.chunks_exact(4).enumerate().all(|(i, p)| inside(i as u32 % w, i as u32 / w) || p[3] == 0), "{name}: pintou fora das áreas");
    }
    set_blend_compositor(BlendCompositor::Direct2d);
}
