//! O PDF pelo krilla (decidido pelo autor em 15/11, no lugar do PDF escrito à mão no núcleo):
//! o texto sai com a fonte embutida, com acentos e símbolos fora do Latin-1 e, no Windows, com
//! o chinês pela fonte de reserva do sistema. Conferido desenhando o PDF de volta.

use aylis_core::canvas::{TextAlign, TextStyle};
use aylis_core::pages::{Op, Page, PageSet};
use aylis_core::providers::PdfProvider;
use aylis_core::*;

fn text(text: &str, y: f32) -> Op {
    let mut style = TextStyle { color: Color::BLACK, h_align: TextAlign::Leading, ..Default::default() };
    style.font.size = 20.0;
    Op::Text { text: text.into(), rect: RectF::new(10.0, y, 380.0, 30.0), style }
}

/// Os pixels escuros na faixa `y0..y1` da página desenhada.
fn dark(width: u32, rgba: &[u8], y0: u32, y1: u32) -> usize {
    (y0..y1).flat_map(|y| (0..width).map(move |x| ((y * width + x) * 4) as usize)).filter(|&i| rgba.get(i).is_some_and(|&r| r < 100)).count()
}

#[test]
fn the_text_is_drawn_with_the_font_embedded() {
    let pages = PageSet {
        width: 400.0,
        height: 140.0,
        pages: vec![Page { ops: vec![text("Relatório: ação € ✓ — “teste”", 10.0), text("中文 测试", 70.0)] }],
    };
    let pdf = aylis_providers::KrillaPdf.write(&pages).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.windows(9).any(|w| w == b"/FontFile"), "a fonte vai embutida");

    let (tx, rx) = std::sync::mpsc::channel();
    let renderer = aylis_docview::pdf::Renderer::new(std::sync::Arc::new(pdf), 4_000_000.0, move |page| {
        let _ = tx.send(page);
    })
    .unwrap();
    let _ = renderer.request(0, 100);
    let (_, _, w, h, rgba) = rx.recv_timeout(std::time::Duration::from_secs(20)).expect("a página desenhada");
    assert_eq!((w, h), (400, 140));
    assert!(dark(w, &rgba, 10, 40) > 300, "a primeira linha aparece");
    if cfg!(windows) {
        assert!(dark(w, &rgba, 70, 100) > 150, "o chinês pela fonte de reserva");
    }
}
