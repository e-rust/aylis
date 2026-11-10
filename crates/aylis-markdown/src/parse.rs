//! Markdown → blocos (com o pulldown-cmark).

use aylis_core::TextAlign;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Estilo de um trecho.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Emphasis {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub strike: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Inline {
    Text { text: String, emphasis: Emphasis, link: Option<String> },
    /// Quebra de linha forçada (dois espaços ou `\` no fim da linha).
    Break,
    Image { url: String, alt: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Heading(u8, Vec<Inline>),
    Paragraph(Vec<Inline>),
    /// Um bloco de código e a linguagem dele (a primeira palavra depois das crases, `rust`;
    /// vazia sem ela). Blocos seguidos em linguagens diferentes viram abas.
    Code { lang: String, text: String },
    Quote(Vec<Block>),
    /// `start`: o número do primeiro item (lista numerada). Cada item: tarefa (marcada ou
    /// não) e os blocos dele.
    List { start: Option<u64>, items: Vec<(Option<bool>, Vec<Block>)> },
    Rule,
    /// A primeira linha é o cabeçalho.
    Table { aligns: Vec<TextAlign>, rows: Vec<Vec<Vec<Inline>>> },
}

enum Frame {
    Blocks(Vec<Block>),
    Inlines(Vec<Inline>),
    Heading(u8, Vec<Inline>),
    Quote(Vec<Block>),
    List(Option<u64>, Vec<(Option<bool>, Vec<Block>)>),
    Item(Option<bool>, Vec<Block>, Vec<Inline>),
    Code(String, String),
    Table(Vec<TextAlign>, Vec<Vec<Vec<Inline>>>),
    Row(Vec<Vec<Inline>>),
    Cell(Vec<Inline>),
    Image(String, String),
}

/// Lê o Markdown.
pub fn parse(text: &str) -> Vec<Block> {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    let mut stack: Vec<Frame> = vec![Frame::Blocks(Vec::new())];
    let mut emphasis = Emphasis::default();
    let mut links: Vec<String> = Vec::new();

    fn push_block(stack: &mut [Frame], block: Block) {
        for frame in stack.iter_mut().rev() {
            match frame {
                Frame::Blocks(b) | Frame::Quote(b) => return b.push(block),
                Frame::Item(_, b, inl) => {
                    // Texto solto do item (lista "apertada") vira parágrafo antes do bloco.
                    if !inl.is_empty() {
                        b.push(Block::Paragraph(std::mem::take(inl)));
                    }
                    return b.push(block);
                }
                _ => {}
            }
        }
    }
    fn push_inline(stack: &mut [Frame], inline: Inline) {
        for frame in stack.iter_mut().rev() {
            match frame {
                Frame::Inlines(v) | Frame::Heading(_, v) | Frame::Cell(v) | Frame::Item(_, _, v) => return v.push(inline),
                Frame::Image(_, alt) => {
                    if let Inline::Text { text, .. } = inline {
                        alt.push_str(&text);
                    }
                    return;
                }
                _ => {}
            }
        }
    }

    for event in Parser::new_ext(text, options) {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => stack.push(Frame::Inlines(Vec::new())),
                Tag::Heading { level, .. } => {
                    let n = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    stack.push(Frame::Heading(n, Vec::new()));
                }
                Tag::BlockQuote(_) => stack.push(Frame::Quote(Vec::new())),
                Tag::CodeBlock(kind) => {
                    let lang = match kind {
                        CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or_default().to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    stack.push(Frame::Code(lang, String::new()));
                }
                Tag::List(start) => stack.push(Frame::List(start, Vec::new())),
                Tag::Item => stack.push(Frame::Item(None, Vec::new(), Vec::new())),
                Tag::Table(aligns) => stack.push(Frame::Table(
                    aligns
                        .iter()
                        .map(|a| match a {
                            Alignment::Center => TextAlign::Center,
                            Alignment::Right => TextAlign::Trailing,
                            _ => TextAlign::Leading,
                        })
                        .collect(),
                    Vec::new(),
                )),
                Tag::TableHead | Tag::TableRow => stack.push(Frame::Row(Vec::new())),
                Tag::TableCell => stack.push(Frame::Cell(Vec::new())),
                Tag::Emphasis => emphasis.italic = true,
                Tag::Strong => emphasis.bold = true,
                Tag::Strikethrough => emphasis.strike = true,
                Tag::Link { dest_url, .. } => links.push(dest_url.to_string()),
                Tag::Image { dest_url, .. } => stack.push(Frame::Image(dest_url.to_string(), String::new())),
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => {
                    if let Some(Frame::Inlines(v)) = stack.pop() {
                        push_block(&mut stack, Block::Paragraph(v));
                    }
                }
                TagEnd::Heading(_) => {
                    if let Some(Frame::Heading(n, v)) = stack.pop() {
                        push_block(&mut stack, Block::Heading(n, v));
                    }
                }
                TagEnd::BlockQuote(_) => {
                    if let Some(Frame::Quote(b)) = stack.pop() {
                        push_block(&mut stack, Block::Quote(b));
                    }
                }
                TagEnd::CodeBlock => {
                    if let Some(Frame::Code(lang, s)) = stack.pop() {
                        push_block(&mut stack, Block::Code { lang, text: s.trim_end_matches('\n').to_string() });
                    }
                }
                TagEnd::List(_) => {
                    if let Some(Frame::List(start, items)) = stack.pop() {
                        push_block(&mut stack, Block::List { start, items });
                    }
                }
                TagEnd::Item => {
                    if let Some(Frame::Item(task, mut blocks, inl)) = stack.pop() {
                        if !inl.is_empty() {
                            blocks.push(Block::Paragraph(inl));
                        }
                        if let Some(Frame::List(_, items)) = stack.last_mut() {
                            items.push((task, blocks));
                        }
                    }
                }
                TagEnd::Table => {
                    if let Some(Frame::Table(aligns, rows)) = stack.pop() {
                        push_block(&mut stack, Block::Table { aligns, rows });
                    }
                }
                TagEnd::TableHead | TagEnd::TableRow => {
                    if let Some(Frame::Row(cells)) = stack.pop() {
                        if let Some(Frame::Table(_, rows)) = stack.last_mut() {
                            rows.push(cells);
                        }
                    }
                }
                TagEnd::TableCell => {
                    if let Some(Frame::Cell(v)) = stack.pop() {
                        if let Some(Frame::Row(cells)) = stack.last_mut() {
                            cells.push(v);
                        }
                    }
                }
                TagEnd::Emphasis => emphasis.italic = false,
                TagEnd::Strong => emphasis.bold = false,
                TagEnd::Strikethrough => emphasis.strike = false,
                TagEnd::Link => {
                    links.pop();
                }
                TagEnd::Image => {
                    if let Some(Frame::Image(url, alt)) = stack.pop() {
                        push_inline(&mut stack, Inline::Image { url, alt });
                    }
                }
                _ => {}
            },
            Event::Text(t) => {
                if let Some(Frame::Code(_, s)) = stack.last_mut() {
                    s.push_str(&t);
                } else {
                    push_inline(&mut stack, Inline::Text { text: t.to_string(), emphasis, link: links.last().cloned() });
                }
            }
            Event::Code(t) => {
                let e = Emphasis { code: true, ..emphasis };
                push_inline(&mut stack, Inline::Text { text: t.to_string(), emphasis: e, link: links.last().cloned() });
            }
            Event::SoftBreak => push_inline(&mut stack, Inline::Text { text: " ".into(), emphasis, link: links.last().cloned() }),
            Event::HardBreak => push_inline(&mut stack, Inline::Break),
            Event::Rule => push_block(&mut stack, Block::Rule),
            Event::TaskListMarker(done) => {
                if let Some(Frame::Item(task, _, _)) = stack.iter_mut().rev().find(|f| matches!(f, Frame::Item(..))) {
                    *task = Some(done);
                }
            }
            _ => {}
        }
    }
    while stack.len() > 1 {
        // Markdown mal fechado: o que sobrou vira parágrafo.
        if let Some(Frame::Inlines(v)) = stack.pop() {
            push_block(&mut stack, Block::Paragraph(v));
        }
    }
    match stack.pop() {
        Some(Frame::Blocks(b)) => b,
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_and_inline_styles() {
        let blocks = parse(
            "# Título\n\nUm **forte** e *leve* com `x` e [link](https://aylis.dev).\n\n> citação\n\n- [x] feito\n- [ ] falta\n  1. sub\n\n```\nfn main() {}\n```\n\n| a | b |\n|:-|-:|\n| 1 | 2 |\n\n---\n",
        );
        assert!(matches!(&blocks[0], Block::Heading(1, v) if v.len() == 1));
        let Block::Paragraph(p) = &blocks[1] else { panic!("{:?}", blocks[1]) };
        assert!(p.iter().any(|i| matches!(i, Inline::Text { text, emphasis, .. } if text == "forte" && emphasis.bold)));
        assert!(p.iter().any(|i| matches!(i, Inline::Text { emphasis, .. } if emphasis.code)));
        assert!(p.iter().any(|i| matches!(i, Inline::Text { link: Some(l), .. } if l == "https://aylis.dev")));
        assert!(matches!(&blocks[2], Block::Quote(_)));
        let Block::List { items, start: None } = &blocks[3] else { panic!("{:?}", blocks[3]) };
        assert_eq!(items[0].0, Some(true));
        assert_eq!(items[1].0, Some(false));
        assert!(items[1].1.iter().any(|b| matches!(b, Block::List { start: Some(1), .. })), "lista aninhada");
        assert_eq!(blocks[4], Block::Code { lang: String::new(), text: "fn main() {}".into() });
        let Block::Table { aligns, rows } = &blocks[5] else { panic!("{:?}", blocks[5]) };
        assert_eq!(aligns, &[TextAlign::Leading, TextAlign::Trailing]);
        assert_eq!(rows.len(), 2);
        assert_eq!(blocks[6], Block::Rule);
    }
}
