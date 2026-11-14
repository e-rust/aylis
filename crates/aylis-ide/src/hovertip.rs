//! A dica do código: o que o servidor de linguagem diz da palavra sob o mouse (o
//! `textDocument/hover`), mostrada como no VS Code. O texto do servidor é Markdown: os blocos
//! de código (o caminho do módulo, a assinatura) saem em monoespaçada com as cores e o realce
//! do editor, uma linha separa as partes (os erros daquele ponto, a assinatura, a
//! documentação), a documentação sai formatada (negrito, itálico, `código`, listas; os links,
//! como texto) e a caixa tem largura máxima, quebra a linha e rola quando é comprida.
//!
//! A caixa é um `Panel` com um `Markdown` dentro, por cima do editor: fica aberta com o mouse
//! em cima dela (para rolar e selecionar) e some quando o mouse sai, quando se digita, clica
//! ou rola o editor.

use std::cell::RefCell;

use aylis_controls::{Memo, Panel};
use aylis_core::{Align, ComponentId, RectF, Ui};
use aylis_markdown::{EditorCode, Layout, Markdown, Palette};

/// Uma parte da dica.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverPart {
    /// Um bloco de código (o caminho do módulo, a assinatura, um exemplo): a linguagem da cerca
    /// (vazia: a do editor) e o código.
    Code { lang: String, text: String },
    /// Texto em Markdown (a documentação, um erro), já com os links como texto.
    Text(String),
    /// A linha entre as partes.
    Separator,
}

/// Separa a dica do servidor (Markdown) nas partes: os blocos de código cercados (```` ``` ````
/// ou `~~~`), as linhas `---` entre as partes e o texto entre eles. Separadores repetidos, no
/// começo ou no fim saem.
pub fn parts(markdown: &str) -> Vec<HoverPart> {
    let mut out = Vec::new();
    let mut text: Vec<&str> = Vec::new();
    let mut fence: Option<(char, usize, String, Vec<&str>)> = None;
    fn flush(text: &mut Vec<&str>, out: &mut Vec<HoverPart>) {
        let joined = text.join("\n");
        let trimmed = joined.trim_matches('\n').trim_end();
        if !trimmed.trim().is_empty() {
            out.push(HoverPart::Text(plain_links(trimmed)));
        }
        text.clear();
    }
    for line in markdown.lines() {
        let t = line.trim();
        if let Some((mark, count, lang, body)) = fence.as_mut() {
            // Fecha com a mesma marca, pelo menos tantas quanto abriu, e nada mais.
            if t.len() >= *count && t.chars().all(|c| c == *mark) {
                out.push(HoverPart::Code { lang: std::mem::take(lang), text: body.join("\n") });
                fence = None;
            } else {
                body.push(line);
            }
            continue;
        }
        let mark = t.chars().next().filter(|c| *c == '`' || *c == '~');
        if let Some(mark) = mark {
            let count = t.chars().take_while(|c| *c == mark).count();
            if count >= 3 {
                flush(&mut text, &mut out);
                fence = Some((mark, count, t[count..].trim().to_string(), Vec::new()));
                continue;
            }
        }
        let rule = t.len() >= 3 && ['-', '*', '_'].iter().any(|&m| t.chars().all(|c| c == m));
        if rule {
            flush(&mut text, &mut out);
            out.push(HoverPart::Separator);
            continue;
        }
        text.push(line);
    }
    // Uma cerca sem fechar: o que veio fica como código.
    if let Some((_, _, lang, body)) = fence {
        out.push(HoverPart::Code { lang, text: body.join("\n") });
    }
    flush(&mut text, &mut out);
    // Sem separador sobrando: nem no começo, nem no fim, nem dois seguidos.
    let mut clean: Vec<HoverPart> = Vec::new();
    for part in out {
        if part == HoverPart::Separator && clean.last().is_none_or(|p| *p == HoverPart::Separator) {
            continue;
        }
        clean.push(part);
    }
    if clean.last() == Some(&HoverPart::Separator) {
        clean.pop();
    }
    clean
}

/// Os links (`[texto](endereço)`) viram só o texto (com o `código` de dentro): a dica não
/// navega. O que está entre crases fica como está.
fn plain_links(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut in_code = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '`' {
            in_code = !in_code;
            out.push(c);
            i += 1;
            continue;
        }
        if c == '[' && !in_code {
            // O `]` que fecha (os colchetes de dentro contam) e o `(...)` logo depois.
            let mut depth = 0;
            let mut close = None;
            let mut code = false;
            for (j, &d) in chars.iter().enumerate().skip(i) {
                match d {
                    '`' => code = !code,
                    '[' if !code => depth += 1,
                    ']' if !code => {
                        depth -= 1;
                        if depth == 0 {
                            close = Some(j);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(close) = close.filter(|&j| chars.get(j + 1) == Some(&'(')) {
                if let Some(end) = chars[close + 2..].iter().position(|&d| d == ')') {
                    out.extend(&chars[i + 1..close]);
                    i = close + 2 + end + 1;
                    continue;
                }
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// As partes de volta em Markdown (o que o componente `Markdown` mostra).
pub fn to_markdown(parts: &[HoverPart]) -> String {
    parts
        .iter()
        .map(|p| match p {
            HoverPart::Code { lang, text } => {
                let mut ticks = "```".to_string();
                while text.contains(ticks.as_str()) {
                    ticks.push('`');
                }
                format!("{ticks}{lang}\n{text}\n{ticks}")
            }
            HoverPart::Text(t) => t.clone(),
            HoverPart::Separator => "---".to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// As partes em texto simples (o `tip` do estado do editor, para o agente): o código e o texto,
/// um bloco por parte.
pub fn to_plain(parts: &[HoverPart]) -> String {
    parts
        .iter()
        .filter_map(|p| match p {
            HoverPart::Code { text, .. } => Some(text.clone()),
            HoverPart::Text(t) => Some(t.clone()),
            HoverPart::Separator => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Um erro ou aviso do ponto como parte da dica (o símbolo e a mensagem; as linhas da mensagem
/// ficam como estão).
pub fn problem(severity: u8, message: &str) -> HoverPart {
    let icon = if severity == 1 { "⊗" } else { "⚠" };
    HoverPart::Text(format!("{icon} {}", message.trim().replace('\n', "  \n")))
}

// ---------------------------------------------------------------------------
// A caixa

/// A dica aberta.
struct Tip {
    memo: ComponentId,
    frame: ComponentId,
    view: ComponentId,
    plain: String,
}

thread_local! {
    static TIP: RefCell<Option<Tip>> = const { RefCell::new(None) };
    /// O fechar agendado (o mouse saiu da palavra ou da dica).
    static HIDE: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

/// A largura máxima e a altura máxima da caixa (como as do VS Code).
const MAX_WIDTH: f32 = 600.0;
const MAX_HEIGHT: f32 = 340.0;
/// O respiro do texto dentro da caixa (o `padding` do `Markdown`) e a borda.
const PAD: f32 = 8.0;
const BORDER: f32 = 1.0;
/// Quanto o mouse tem para ir da palavra até a dica (ou voltar) antes de ela fechar (ms).
const GRACE: u64 = 350;

/// O texto simples da dica aberta no editor `memo` (o `tip` do estado do editor).
pub fn text_of(memo: ComponentId) -> Option<String> {
    TIP.with(|t| t.borrow().as_ref().filter(|t| t.memo == memo).map(|t| t.plain.clone()))
}

/// Mostra a dica `parts` perto do caractere `index` do editor `memo`. `language` é a do
/// arquivo (a dos blocos sem linguagem) e `color_theme` o tema de cores do editor. Sem
/// partes, fecha a que estiver aberta.
pub fn show(ui: &mut Ui, memo: ComponentId, index: usize, parts: &[HoverPart], language: &str, color_theme: &str) {
    hide(ui);
    if parts.is_empty() {
        return;
    }
    let Some(area) = ui.parent(memo) else { return };
    let (Some(area_abs), Some(memo_abs)) = (ui.absolute_rect(area), ui.absolute_rect(memo)) else { return };
    let Some(word) = ui.get::<Memo>(memo).and_then(|m| m.index_rect(ui, index)) else { return };
    // A palavra nas coordenadas da área do editor (onde a caixa entra).
    let word = RectF::new(word.x + memo_abs.x - area_abs.x, word.y + memo_abs.y - area_abs.y, word.width, word.height);
    let markdown = to_markdown(parts);
    let editor = EditorCode { color_theme: color_theme.to_string(), language: language.to_string() };
    // O tamanho pelo conteúdo: a largura do texto (até a máxima) e a altura dele (até a máxima,
    // e daí rola).
    let room = (area_abs.width - 16.0).max(160.0);
    let max_w = MAX_WIDTH.min(room);
    let mut palette = Palette::from_theme(ui.theme(), 0.0);
    palette.editor = Some(editor.clone());
    let blocks = aylis_markdown::parse(&markdown);
    let measure_at = |width: f32| {
        let mut measure = |s: &str, style: &aylis_core::TextStyle| ui.measure_text(s, style).width;
        Layout::build_in(&blocks, width, &palette, "", &mut measure, &mut |_: &str| None)
    };
    let inner_max = max_w - 2.0 * (PAD + BORDER);
    let layout = measure_at(inner_max);
    let used = layout.items.iter().filter_map(|i| matches!(i, aylis_markdown::Item::Text { .. }).then(|| i.rect().right())).fold(0.0f32, f32::max);
    let width = ((used.ceil() + 2.0 * (PAD + BORDER) + 2.0).max(180.0)).min(max_w);
    let layout = if width < max_w { measure_at(width - 2.0 * (PAD + BORDER)) } else { layout };
    // Acima da linha da palavra, se couber; senão, embaixo.
    let space_above = word.y - 8.0;
    let space_below = area_abs.height - word.bottom() - 8.0;
    let wanted = layout.height + 2.0 * (PAD + BORDER) + 2.0;
    let above = space_above >= wanted.min(MAX_HEIGHT) || space_above >= space_below;
    let height = wanted.min(MAX_HEIGHT).min(if above { space_above } else { space_below }.max(60.0));
    let top = if above { word.y - height - 4.0 } else { word.bottom() + 4.0 };
    let left = word.x.min(area_abs.width - width - 8.0).max(4.0);

    let frame = ui.add(Panel::default(), area);
    ui.control_mut(frame)
        .unwrap()
        .set_style("overlay")
        .set_padding(BORDER)
        .set_clip_children(true)
        .set_left(left)
        .set_top(top.max(0.0))
        .set_size(width, height);
    let mut md = Markdown::new(markdown);
    md.editor_code = Some(editor);
    md.base.set_padding(PAD);
    let view = ui.add(md, frame);
    ui.control_mut(view).unwrap().set_align(Align::Client);
    // O mouse saiu da dica (para fora, não para a borda dela): fecha daqui a pouco, se não
    // voltar.
    for id in [frame, view] {
        ui.on(id, "on_pointer_leave", move |ui, _, _| hide_later(ui));
        ui.on(id, "on_pointer_enter", move |ui, _, _| cancel_hide(ui));
    }
    TIP.with(|t| *t.borrow_mut() = Some(Tip { memo, frame, view, plain: to_plain(parts) }));
    ui.request_repaint();
    // A medida acima usa a fonte do tema; o texto pintado usa a do controle. Depois da primeira
    // pintura, a altura segue a do conteúdo (a borda de baixo fica no lugar quando a caixa
    // está acima da palavra).
    let limit = if above { space_above } else { space_below }.max(60.0).min(MAX_HEIGHT);
    ui.set_timeout(60, move |ui| {
        let Some(content) = ui.get::<Markdown>(view).map(|m| m.content_height()).filter(|h| *h > 0.0) else { return };
        let fitted = (content + 2.0 * (PAD + BORDER) + 2.0).min(limit);
        if (fitted - height).abs() < 2.0 || !ui.contains(frame) {
            return;
        }
        let top = if above { word.y - fitted - 4.0 } else { word.bottom() + 4.0 };
        ui.control_mut(frame).unwrap().set_top(top.max(0.0)).set_height(fitted);
        ui.request_repaint();
    });
}

/// Fecha a dica (se estiver aberta).
pub fn hide(ui: &mut Ui) {
    cancel_hide(ui);
    if let Some(tip) = TIP.with(|t| t.borrow_mut().take()) {
        if ui.contains(tip.frame) {
            ui.free(tip.frame);
            ui.request_repaint();
        }
    }
}

/// Fecha a dica do editor `memo` (digitou, clicou, rolou, trocou de aba).
pub fn hide_for(ui: &mut Ui, memo: ComponentId) {
    if TIP.with(|t| t.borrow().as_ref().is_some_and(|t| t.memo == memo)) {
        hide(ui);
    }
}

/// O mouse saiu da palavra (ou da dica): fecha daqui a pouco, a não ser que ele esteja sobre
/// a dica (o caminho da palavra até ela passa por cima de outras palavras).
pub fn hide_later(ui: &mut Ui) {
    if TIP.with(|t| t.borrow().is_none()) {
        return;
    }
    cancel_hide(ui);
    let id = ui.set_timeout(GRACE, |ui| {
        HIDE.with(|h| h.set(None));
        if !pointer_over(ui) {
            hide(ui);
        }
    });
    HIDE.with(|h| h.set(Some(id)));
}

fn cancel_hide(ui: &mut Ui) {
    if let Some(id) = HIDE.with(|h| h.take()) {
        ui.clear_timeout(id);
    }
}

/// O mouse está sobre a dica (a caixa ou o texto dela).
fn pointer_over(ui: &Ui) -> bool {
    let Some((frame, view)) = TIP.with(|t| t.borrow().as_ref().map(|t| (t.frame, t.view))) else { return false };
    if !ui.contains(frame) {
        return false;
    }
    let mut current = ui.hovered(ui.root_of(frame));
    while let Some(c) = current {
        if c == frame || c == view {
            return true;
        }
        current = ui.parent(c);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O hover do rust-analyzer: o caminho do módulo e a assinatura em blocos de código, a
    /// linha, e a documentação com Markdown (negrito, `código`, um link, um título e um
    /// exemplo sem linguagem).
    const RUST_ANALYZER: &str = "\n```rust\naylis_core::ui::Ui\n```\n\n```rust\npub fn set_prop(&mut self, id: ComponentId, name: &str, value: impl Into<PropValue>) -> Result<(), Error>\n```\n\n---\n\nMuda uma propriedade **pelo nome**. Veja [`get_prop`](https://docs.rs/aylis/latest/aylis/struct.Ui.html#method.get_prop).\n\n# Examples\n\n```\nui.set_prop(id, \"text\", \"Olá\")?;\n```\n";

    #[test]
    fn the_rust_analyzer_hover_becomes_the_right_parts() {
        let parts = parts(RUST_ANALYZER);
        assert_eq!(
            parts,
            vec![
                HoverPart::Code { lang: "rust".into(), text: "aylis_core::ui::Ui".into() },
                HoverPart::Code { lang: "rust".into(), text: "pub fn set_prop(&mut self, id: ComponentId, name: &str, value: impl Into<PropValue>) -> Result<(), Error>".into() },
                HoverPart::Separator,
                HoverPart::Text("Muda uma propriedade **pelo nome**. Veja `get_prop`.\n\n# Examples".into()),
                HoverPart::Code { lang: String::new(), text: "ui.set_prop(id, \"text\", \"Olá\")?;".into() },
            ]
        );
        // De volta em Markdown: as cercas e a linha, na ordem.
        let md = to_markdown(&parts);
        assert!(md.starts_with("```rust\naylis_core::ui::Ui\n```\n\n```rust\npub fn set_prop("), "{md}");
        assert!(md.contains("\n\n---\n\nMuda uma propriedade"), "{md}");
        assert!(md.ends_with("```\nui.set_prop(id, \"text\", \"Olá\")?;\n```"), "{md}");
        // O Markdown lê os mesmos blocos: dois de código, a linha, o texto, o título e o exemplo.
        let blocks = aylis_markdown::parse(&md);
        let codes = blocks.iter().filter(|b| matches!(b, aylis_markdown::Block::Code { .. })).count();
        assert_eq!(codes, 3);
        assert!(blocks.iter().any(|b| matches!(b, aylis_markdown::Block::Rule)));
        assert!(blocks.iter().any(|b| matches!(b, aylis_markdown::Block::Heading(1, _))));
        // O exemplo sem linguagem fica com a do editor.
        let editor = EditorCode { color_theme: String::new(), language: "rust".into() };
        assert_eq!(editor.language_of(""), "rust");
        assert_eq!(editor.language_of("no_run"), "rust");
        assert_eq!(editor.language_of("text"), "text");
        // O texto simples (o estado do editor, para o agente) não tem as cercas.
        let plain = to_plain(&parts);
        assert!(plain.starts_with("aylis_core::ui::Ui\n\npub fn set_prop("), "{plain}");
        assert!(!plain.contains("```"));
    }

    #[test]
    fn separators_and_links_are_cleaned() {
        assert_eq!(parts("---\n\nTexto\n\n---\n\n***\n"), vec![HoverPart::Text("Texto".into())]);
        assert_eq!(plain_links("Veja [a `b`](http://x) e [c]."), "Veja a `b` e [c].");
        assert_eq!(plain_links("`[x](y)` fica"), "`[x](y)` fica");
        // Uma cerca de quatro crases guarda as de três de dentro.
        assert_eq!(parts("````md\n```rust\nx\n```\n````"), vec![HoverPart::Code { lang: "md".into(), text: "```rust\nx\n```".into() }]);
        // Os erros do ponto vêm antes, com o símbolo.
        assert_eq!(problem(1, "tipo errado\nesperava `i32`"), HoverPart::Text("⊗ tipo errado  \nesperava `i32`".into()));
    }
}
