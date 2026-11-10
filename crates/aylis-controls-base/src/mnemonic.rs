//! A letra do atalho nas legendas: `"&Salvar"` aparece como "Salvar" com o S sublinhado, e
//! Alt+S clica no botão (o `Ui` acha o controle pela letra; aqui só o desenho).

use aylis_core::{Brush, Canvas, RectF, TextAlign, TextStyle};

/// A legenda sem o `&` e a posição da letra sublinhada.
pub fn caption(text: &str) -> (String, Option<usize>) {
    aylis_core::strip_mnemonic(text)
}

/// Desenha `text` (já sem o `&`) e sublinha a letra em `at`. Numa linha só: com quebra de
/// linha, o sublinhado fica de fora (a letra não tem um lugar fixo).
pub fn draw_text(canvas: &mut dyn Canvas, text: &str, at: Option<usize>, rect: RectF, style: &TextStyle) {
    canvas.draw_text(text, rect, style);
    let Some(at) = at else { return };
    if style.wrap {
        return;
    }
    let prefix: String = text.chars().take(at).collect();
    let letter: String = text.chars().skip(at).take(1).collect();
    if letter.trim().is_empty() {
        return;
    }
    let full = canvas.measure_text(text, style).width.min(rect.width);
    let x0 = match style.h_align {
        TextAlign::Leading | TextAlign::Justify => rect.x,
        TextAlign::Center => rect.x + (rect.width - full) / 2.0,
        TextAlign::Trailing => rect.x + rect.width - full,
    };
    let x = x0 + canvas.measure_text(&prefix, style).width;
    let width = canvas.measure_text(&letter, style).width;
    if x + width > rect.x + rect.width + 0.5 {
        return; // a letra foi cortada (reticências)
    }
    let size = style.font.size;
    let line = size * 1.25;
    let top = match style.v_align {
        TextAlign::Leading | TextAlign::Justify => rect.y,
        TextAlign::Center => rect.y + (rect.height - line) / 2.0,
        TextAlign::Trailing => rect.y + rect.height - line,
    };
    let thickness = (size / 14.0).max(1.0);
    canvas.fill_rect(RectF::new(x, top + size * 1.12, width, thickness), 0.0, &Brush::Solid(style.color));
}
