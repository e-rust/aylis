//! `color_glyphs: false` no `TextStyle` desenha os emoji na cor do texto, com a Noto Emoji sem
//! cor do Aylis (`resources/fonts/notoemoji`, SIL OFL 1.1), igual no Direct2D e no Skia. Num
//! processo à parte: a fonte registrada vale para o processo inteiro.
#![cfg(windows)]

use aylis_core::{Canvas, Color, RectF, TextStyle};

static NOTO_EMOJI: &[u8] = include_bytes!("../../../resources/fonts/notoemoji/NotoEmoji[wght].ttf");

/// Pixels com cor de verdade (o preto sobre branco só dá cinzas, mesmo na borda das letras).
fn colored(px: &[u8]) -> usize {
    px.chunks(4)
        .filter(|p| {
            let (r, g, b) = (p[0] as i32, p[1] as i32, p[2] as i32);
            (r - g).abs() + (g - b).abs() + (r - b).abs() > 60
        })
        .count()
}

/// A primeira e a última coluna com tinta.
fn ink_columns(px: &[u8], w: usize) -> (usize, usize) {
    let dark = |x: usize| px.chunks(4).skip(x).step_by(w).any(|p| p[0] < 128);
    ((0..w).find(|&x| dark(x)).unwrap_or(0), (0..w).rev().find(|&x| dark(x)).unwrap_or(0))
}

fn style(color_glyphs: bool) -> TextStyle {
    let mut s = TextStyle::default();
    s.font.size = 32.0;
    s.color = Color::BLACK;
    s.color_glyphs = color_glyphs;
    s
}

fn paint(text: &'static str, color_glyphs: bool) -> impl Fn(&mut dyn Canvas) {
    move |c: &mut dyn Canvas| c.draw_text(text, RectF::new(4.0, 0.0, 232.0, 60.0), &style(color_glyphs))
}

#[test]
fn emoji_without_color_are_the_same_in_direct2d_and_skia() {
    assert_eq!(aylis_core::fonts::register(NOTO_EMOJI.to_vec()).as_deref(), Some(aylis_core::canvas::MONO_EMOJI_FAMILY));
    let d2d = aylis_d2d::D2dRenderer::new().unwrap();
    let skia = aylis_skia::SkiaRenderer::new();
    for text in ["\u{1F600}", "\u{2705} ok", "A \u{1F44D}\u{1F3FD} 1", "\u{2764}\u{FE0F}"] {
        let a = d2d.render_rgba(240, 60, 1.0, Color::WHITE, |c| paint(text, false)(c)).unwrap();
        let b = skia.render_rgba(240, 60, 1.0, Color::WHITE, |c| paint(text, false)(c)).unwrap();
        for (name, px) in [("Direct2D", &a), ("Skia", &b)] {
            assert_eq!(colored(px), 0, "{name}: {text:?} sem cor");
            assert!(px.chunks(4).filter(|p| p[0] < 128).count() > 40, "{name}: {text:?} aparece");
        }
        let (ia, ib) = (ink_columns(&a, 240), ink_columns(&b, 240));
        assert!(ia.0.abs_diff(ib.0) <= 2 && ia.1.abs_diff(ib.1) <= 3, "{text:?}: Direct2D {ia:?}, Skia {ib:?}");
        let (mut wa, mut wb) = (0.0, 0.0);
        d2d.render_rgba(4, 4, 1.0, Color::WHITE, |c| wa = c.measure_text(text, &style(false)).width).unwrap();
        skia.render_rgba(4, 4, 1.0, Color::WHITE, |c| wb = c.measure_text(text, &style(false)).width).unwrap();
        assert!((wa - wb).abs() <= 2.0, "{text:?}: largura Direct2D {wa}, Skia {wb}");
    }
    // Com a cor (o padrão), o emoji segue colorido, mesmo com a fonte sem cor registrada.
    for (name, px) in [
        ("Direct2D", d2d.render_rgba(240, 60, 1.0, Color::WHITE, |c| paint("\u{1F600}", true)(c)).unwrap()),
        ("Skia", skia.render_rgba(240, 60, 1.0, Color::WHITE, |c| paint("\u{1F600}", true)(c)).unwrap()),
    ] {
        assert!(colored(&px) > 100, "{name}: o emoji colorido");
    }
}
