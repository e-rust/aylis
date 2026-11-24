//! O texto e as camadas iguais aos outros renderizadores.


use aylis_core::{Brush, Canvas, Color, RectF, TextStyle};

/// Pixels com cor de verdade (o preto sobre branco só dá cinzas, mesmo na borda das letras).
fn colored(px: &[u8]) -> usize {
    px.chunks(4)
        .filter(|p| {
            let (r, g, b) = (p[0] as i32, p[1] as i32, p[2] as i32);
            (r - g).abs() + (g - b).abs() + (r - b).abs() > 60
        })
        .count()
}

fn text(text: &str) -> Vec<u8> {
    let r = aylis_skia::SkiaRenderer::new();
    r.render_rgba(120, 60, 1.0, Color::WHITE, |c| {
        let mut s = TextStyle::default();
        s.color = Color::BLACK;
        s.font.size = 32.0;
        c.draw_text(text, RectF::new(0.0, 0.0, 120.0, 60.0), &s);
    })
    .unwrap()
}

/// Item 69: `⚠` e `ℹ` (apresentação de texto no Unicode) saíam como emoji colorido no Direct2D e
/// no Skia, e o U+FE0E não mudava nada; o navegador já os mostra na cor do texto. Com U+FE0F, e
/// os emoji de verdade (`😀`), seguem coloridos.
#[test]
fn text_presentation_symbols_take_the_text_color() {
    for mono in ["\u{26A0}", "\u{26A0}\u{FE0E}", "\u{2139}", "\u{260E}"] {
        let px = text(mono);
        assert_eq!(colored(&px), 0, "{mono:?} na cor do texto");
        assert!(px.chunks(4).any(|p| p[0] < 128), "{mono:?} aparece");
    }
    for emoji in ["\u{26A0}\u{FE0F}", "\u{1F600}"] {
        assert!(colored(&text(emoji)) > 100, "{emoji:?} colorido");
    }
    // Na mesma linha: só o emoji colorido.
    let alone = colored(&text("\u{1F600}"));
    let mixed = colored(&text("A \u{26A0} \u{1F600}"));
    assert!(mixed.abs_diff(alone) < alone / 10, "linha mista: {mixed} pixels coloridos, o emoji sozinho {alone}");
}

/// O `save_layer(0.9)` "bem mais transparente que 90%" no Direct2D. Preto numa camada de 90%
/// sobre branco dá 25 (10% de 255), igual no Direct2D; em escala de tela, com recorte, aninhada
/// (81%), com sombra e com texto dentro.
#[test]
fn an_opacity_layer_is_exactly_its_opacity() {
    let r = aylis_skia::SkiaRenderer::new();
    let full = RectF::new(0.0, 0.0, 40.0, 40.0);
    let black = Brush::Solid(Color::BLACK);
    let cases: [(&str, f32, u8, Box<dyn Fn(&mut dyn Canvas)>); 6] = [
        ("simples", 1.0, 25, Box::new(|c| { c.save_layer(0.9); c.fill_rect(full, 0.0, &black); c.restore(); })),
        ("escala 1,5", 1.5, 25, Box::new(|c| { c.save_layer(0.9); c.fill_rect(full, 0.0, &black); c.restore(); })),
        ("recorte", 1.0, 25, Box::new(|c| { c.save(); c.clip_rect(RectF::new(5.0, 5.0, 30.0, 30.0)); c.save_layer(0.9); c.fill_rect(full, 0.0, &black); c.restore(); c.restore(); })),
        ("aninhada", 1.0, 48, Box::new(|c| { c.save_layer(0.9); c.save_layer(0.9); c.fill_rect(full, 0.0, &black); c.restore(); c.restore(); })),
        ("sombra", 1.0, 25, Box::new(|c| {
            c.save_layer(0.9);
            c.draw_shadow(RectF::new(10.0, 10.0, 20.0, 20.0), 4.0, &aylis_core::Shadow::new(Color::BLACK.with_opacity(0.3), 6.0, 0.0, 2.0));
            c.fill_rect(full, 0.0, &black);
            c.restore();
        })),
        ("texto", 1.0, 25, Box::new(|c| {
            c.save_layer(0.9);
            let mut s = TextStyle::default();
            s.color = Color::BLACK;
            c.draw_text("Ag", full, &s);
            c.fill_rect(full, 0.0, &black);
            c.restore();
        })),
    ];
    for (name, scale, expected, paint) in cases {
        let w = (40.0 * scale) as u32;
        let px = r.render_rgba(w, w, scale, Color::WHITE, |c| paint(c)).unwrap();
        let i = ((w / 2) * w + w / 2) as usize * 4;
        assert!(px[i].abs_diff(expected) <= 1 && px[i + 3] == 255, "{name}: {:?}, esperado {expected}", &px[i..i + 4]);
    }
}
