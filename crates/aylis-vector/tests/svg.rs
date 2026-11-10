//! SVG de verdade (arquivo): lê e desenha; com `AYLIS_SHOTS`, grava o PNG para conferir.

use aylis_core::{Color, RectF};
use aylis_vector::SvgImage;

#[test]
fn a_sample_with_gradients_use_and_evenodd() {
    let text = include_str!("data/sample.svg");
    let svg = SvgImage::parse(text).unwrap();
    assert_eq!(svg.view_box(), RectF::new(0.0, 0.0, 240.0, 120.0));
    assert_eq!(svg.shape_count(), 7, "rect, circle, 2 use, path, polyline, ellipse (o texto fica de fora)");
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let renderer = aylis_skia::SkiaRenderer::new();
    let png = renderer
        .render_png(720, 360, 1.0, Color::WHITE, |c| svg.render(c, RectF::new(0.0, 0.0, 720.0, 360.0), Color::rgb(0x21, 0x21, 0x21)))
        .unwrap();
    std::fs::write(std::path::Path::new(&dir).join("svg-amostra.png"), png).unwrap();
}
