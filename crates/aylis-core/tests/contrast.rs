//! O texto dos botões coloridos (Perigo, Sucesso...) precisa ser legível em todos os temas.

use aylis_core::{Color, ControlState, Theme};

/// Luminância relativa (WCAG).
fn luminance(c: Color) -> f32 {
    let ch = |v: u8| {
        let v = v as f32 / 255.0;
        if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch(c.r) + 0.7152 * ch(c.g) + 0.0722 * ch(c.b)
}

fn contrast(a: Color, b: Color) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[test]
fn colored_buttons_have_readable_text_in_every_theme() {
    let aylis = [Theme::light(), Theme::dark()];
    let mut bad = Vec::new();
    for theme in aylis.iter().chain(aylis_core::themes::all()) {
        for variant in ["danger", "success"] {
            let s = theme.resolve(&["Button"], variant, ControlState::default());
            // O texto é o token próprio (não o herdado do botão comum: o azul do iOS 26).
            let token = theme.color(if variant == "danger" { "on_danger" } else { "on_success" });
            if s.text != token {
                bad.push(format!("{} · {variant}: texto {} em vez do token {}", theme.name, s.text.to_hex(), token.to_hex()));
            }
            let c = contrast(s.text, s.fill);
            // 3:1 é o mínimo do WCAG para texto grande/em negrito, como o de um botão. No verde,
            // as plataformas (iOS, Windows) usam branco mesmo abaixo disso; o mínimo aqui aceita o
            // branco e recusa o que não dá para ler (o azul do botão comum por cima do verde).
            // O verde do iOS 7, o mais claro de todos, fica em 1,8:1 com o branco do sistema.
            let min = if variant == "success" { 1.7 } else { 3.0 };
            if c < min {
                bad.push(format!("{} · {variant}: texto {} sobre {} ({c:.1}:1)", theme.name, s.text.to_hex(), s.fill.to_hex()));
            }
        }
    }
    assert!(bad.is_empty(), "texto ilegível:\n{}", bad.join("\n"));
}
