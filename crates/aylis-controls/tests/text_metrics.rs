//! O mesmo texto nos renderizadores (item 10): com a fonte embutida no projeto, o Skia e o
//! DirectWrite medem e quebram as linhas quase igual. A maior causa de diferença entre as
//! plataformas é a fonte do sistema, diferente em cada uma; com a mesma fonte, sobram
//! diferenças pequenas de arredondamento e de desenho das letras.
//!
//! A web (Canvas 2D) mede no navegador e fica de fora deste teste, que roda no `cargo test`.

use aylis_core::*;

/// Textos que costumam dar diferença: acentos, árabe, emoji, japonês, números e um
/// parágrafo que quebra em várias linhas.
const TEXTS: &[&str] = &[
    "Ação de configuração: não há saída",
    "Pedido nº 1.234,56 — São João",
    "مرحبا بالعالم",
    "Olá 👋🎉 tudo certo?",
    "日本語のテキスト",
    "0123456789 R$ 1.000.000,00",
    "Um parágrafo comprido o bastante para quebrar em várias linhas na largura dada, com palavras de tamanhos diferentes e acentuação.",
];

/// A família embutida usada no teste (a dos projetos novos).
const FAMILY: &str = "Inter";

fn register_font() -> bool {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/fonts/inter/Inter[opsz,wght].ttf");
    match std::fs::read(&path) {
        Ok(data) => {
            aylis_core::fonts::register(data);
            true
        }
        Err(_) => false,
    }
}

fn style(wrap: bool) -> TextStyle {
    TextStyle { font: FontSpec { family: FAMILY.into(), size: 14.0, ..Default::default() }, wrap, ..Default::default() }
}

#[cfg(windows)]
#[test]
fn skia_and_directwrite_measure_the_embedded_font_alike() {
    if !register_font() {
        eprintln!("sem resources/fonts (rode aylis-tool resources): teste pulado");
        return;
    }
    let skia = aylis_skia::SkiaRenderer::new().text_service();
    let d2d = aylis_d2d::D2dRenderer::new().expect("Direct2D").text_service();
    let mut report = Vec::new();
    for text in TEXTS {
        // Numa linha: a largura pode variar um pouco (o desenho das letras de cada motor).
        let a = skia.measure(text, &style(false), None);
        let b = d2d.measure(text, &style(false), None);
        let width_gap = (a.width - b.width).abs() / a.width.max(1.0);
        // Quebrando em 240: o número de linhas (a altura) tem de ser o mesmo, com folga de
        // meia linha para o espaçamento de cada motor.
        let wa = skia.measure(text, &style(true), Some(240.0));
        let wb = d2d.measure(text, &style(true), Some(240.0));
        let lines = |h: f32| (h / (14.0 * 1.2)).round();
        report.push(format!("{text:?}: Skia {:.1}x{:.1}, DirectWrite {:.1}x{:.1}; em 240: {:.1} × {:.1}", a.width, a.height, b.width, b.height, wa.height, wb.height));
        // O árabe, o emoji e o japonês caem em fontes do sistema (a Inter não tem esses
        // caracteres): ali a diferença é a mesma de sempre, e o teste só registra.
        let fallback = text.chars().any(|c| (c as u32) > 0x2000 && !"—’“”".contains(c));
        if !fallback {
            assert!(width_gap < 0.03, "{text:?}: largura {:.1} no Skia e {:.1} no DirectWrite", a.width, b.width);
            assert_eq!(lines(wa.height), lines(wb.height), "{text:?}: quebra em linhas diferentes ({:.1} × {:.1})", wa.height, wb.height);
        }
    }
    eprintln!("{}", report.join("\n"));
}
