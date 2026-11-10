//! As animações Noto do pacote padrão: todas abrem e desenham (e, com `AYLIS_SHOTS=pasta`,
//! viram um quadro de contato para conferir a aparência).

use std::path::PathBuf;

use aylis_core::{Color, RectF};
use aylis_vector::Animation;

fn noto_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/lottie/noto")
}

/// Conta o que foi desenhado.
#[derive(Default)]
struct Count {
    fills: usize,
    strokes: usize,
}

impl aylis_core::Canvas for Count {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &aylis_core::Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &aylis_core::Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &aylis_core::Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &aylis_core::Stroke) {}
    fn draw_line(&mut self, _: aylis_core::PointF, _: aylis_core::PointF, _: &aylis_core::Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &aylis_core::TextStyle) {}
    fn measure_text(&mut self, _: &str, _: &aylis_core::TextStyle) -> aylis_core::SizeF {
        aylis_core::SizeF::new(0.0, 0.0)
    }
    fn fill_path(&mut self, _: &aylis_core::Path, _: &aylis_core::Brush) {
        self.fills += 1;
    }
    fn stroke_path(&mut self, _: &aylis_core::Path, _: &aylis_core::Stroke) {
        self.strokes += 1;
    }
}

fn all() -> Vec<(String, Animation)> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(noto_dir())
        .map(|d| d.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "json") && !p.ends_with("catalog.json")).collect())
        .unwrap_or_default();
    files.sort();
    files
        .into_iter()
        .map(|p| {
            let name = p.file_stem().unwrap().to_string_lossy().to_string();
            let anim = Animation::parse(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{name}: {e}"));
            (name, anim)
        })
        .collect()
}

#[test]
fn every_noto_animation_parses_and_draws_every_frame() {
    let list = all();
    if list.is_empty() {
        eprintln!("sem resources/lottie (rode aylis-tool resources --lottie): teste pulado");
        return;
    }
    assert_eq!(list.len(), 55);
    for (name, anim) in &list {
        let (ip, op) = anim.frames();
        assert!(op > ip && anim.duration() > 0.5, "{name}");
        // Algumas começam e terminam vazias (o bolo entra no quadro 11 e sai no 136).
        let (mut drawn, mut total) = (0, 0);
        let mut f = ip;
        while f < op {
            let mut c = Count::default();
            anim.render(&mut c, f, RectF::new(0.0, 0.0, 128.0, 128.0));
            total += 1;
            if c.fills + c.strokes > 0 {
                drawn += 1;
            }
            f += 7.0;
        }
        assert!(drawn * 10 >= total * 7, "{name}: só {drawn} de {total} quadros desenham");
    }
}

#[test]
fn contact_sheet() {
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let list = all();
    let (cols, cell) = (11usize, 96.0f32);
    for (label, at) in [("noto-inicio.png", 0.0f32), ("noto-meio.png", 0.5)] {
        let rows = list.len().div_ceil(cols);
        let renderer = aylis_skia::SkiaRenderer::new();
        let png = renderer
            .render_png((cols as f32 * cell) as u32, (rows as f32 * cell) as u32, 1.0, Color::WHITE, |c| {
                for (i, (_, anim)) in list.iter().enumerate() {
                    let (ip, op) = anim.frames();
                    let frame = ip + (op - ip) * at;
                    let (x, y) = ((i % cols) as f32 * cell, (i / cols) as f32 * cell);
                    anim.render(c, frame, RectF::new(x + 4.0, y + 4.0, cell - 8.0, cell - 8.0));
                }
            })
            .unwrap();
        std::fs::write(PathBuf::from(&dir).join(label), png).unwrap();
    }
}

/// `AYLIS_LOTTIE=fogo,morri AYLIS_AT=0.25,0.5`: as animações lado a lado, grandes (para
/// comparar com o lottie-web).
#[test]
fn chosen_frames() {
    let (Ok(dir), Ok(names)) = (std::env::var("AYLIS_SHOTS"), std::env::var("AYLIS_LOTTIE")) else { return };
    let ats: Vec<f32> = std::env::var("AYLIS_AT").unwrap_or("0.5".into()).split(',').filter_map(|a| a.parse().ok()).collect();
    let list: Vec<(String, Animation)> = all().into_iter().filter(|(n, _)| names.split(',').any(|w| w == n)).collect();
    let cell = 300.0;
    let count = list.len() * ats.len();
    let renderer = aylis_skia::SkiaRenderer::new();
    let png = renderer
        .render_png((count as f32 * cell) as u32, cell as u32, 1.0, Color::WHITE, |c| {
            let mut i = 0;
            for (_, anim) in &list {
                for at in &ats {
                    let (ip, op) = anim.frames();
                    anim.render(c, ip + (op - ip) * at, RectF::new(i as f32 * cell, 0.0, cell, cell));
                    i += 1;
                }
            }
        })
        .unwrap();
    std::fs::write(PathBuf::from(dir).join("lottie-escolhidas.png"), png).unwrap();
}

/// O mesmo quadro no Direct2D (o padrão no Windows), com as máscaras fora da tela.
#[cfg(windows)]
#[test]
fn contact_sheet_direct2d() {
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let list = all();
    let (cols, cell) = (11usize, 96.0f32);
    let rows = list.len().div_ceil(cols);
    let (w, h) = ((cols as f32 * cell) as u32, (rows as f32 * cell) as u32);
    let renderer = aylis_d2d::D2dRenderer::new().unwrap();
    let rgba = renderer
        .render_rgba(w, h, 1.0, Color::WHITE, |c| {
            for (i, (_, anim)) in list.iter().enumerate() {
                let (ip, op) = anim.frames();
                let (x, y) = ((i % cols) as f32 * cell, (i / cols) as f32 * cell);
                anim.render(c, ip + (op - ip) * 0.5, RectF::new(x + 4.0, y + 4.0, cell - 8.0, cell - 8.0));
            }
        })
        .unwrap();
    std::fs::write(PathBuf::from(dir).join("noto-meio-d2d.png"), aylis_d2d::encode_png(&rgba, w, h).unwrap()).unwrap();
}
