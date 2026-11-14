//! RichEditor: o editor de texto rico com barra de ferramentas (o `QEditor` do Quasar).
//!
//! O conteúdo é um texto com o formato de cada caractere (negrito, itálico, sublinhado,
//! tachado, link) e o de cada parágrafo (título, item de lista, alinhamento). A propriedade
//! `html` é esse conteúdo em HTML simples (`<b>`, `<i>`, `<u>`, `<s>`, `<a href>`, `<p>`,
//! `<h1>` a `<h3>`, `<ul>`/`<ol>` com `<li>`), refeito a cada mudança e lido de volta quando
//! o app (ou o designer) troca o `html`. Um parágrafo só, sem título nem alinhamento, sai
//! sem o `<p>` em volta (`Olá <b>mundo</b>`), como o `QEditor`.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Key, KeyEvent, PaintCtx,
    PointF, PointerButton, PointerEvent, PropValue, RectF, Stroke, TextAlign, TextStyle, Trimming, Ui, WheelEvent, properties,
};

use aylis_core::messages::t;

use crate::leaf_control;

/// Os botões da barra, na ordem do `QEditor`; `|` separa os grupos.
pub const DEFAULT_TOOLBAR: &str =
    "bold;italic;underline;strike;|;size;h1;h2;h3;|;unordered;ordered;|;left;center;right;justify;|;link;|;undo;redo;|;removeFormat";

/// Os nomes que a barra entende (os mesmos do `QEditor`, mais `p`: parágrafo comum).
const TOOLS: &[&str] = &[
    "bold", "italic", "underline", "strike", "size", "h1", "h2", "h3", "p", "unordered", "ordered", "left", "center", "right", "justify", "link", "undo",
    "redo",
    "removeFormat",
];

// Medidas da barra e do texto.
const BUTTON: f32 = 30.0;
/// A largura de um botão (mais estreito que alto: a barra padrão cabe em 600).
const BUTTON_WIDTH: f32 = 26.0;
const ROW: f32 = 36.0;
const SEPARATOR: f32 = 9.0;
const PAD: f32 = 4.0;
const CONTENT_PAD: f32 = 10.0;
const LIST_INDENT: f32 = 26.0;
const UNDO_LIMIT: usize = 200;

// ===========================================================================
// O documento
// ===========================================================================

/// O formato de um caractere.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Format {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    /// O endereço, se o caractere faz parte de um link.
    pub link: Option<String>,
    /// O tamanho da letra, de 1 a 7 como o `<font size>` do HTML (3 é o normal); 0: o do
    /// parágrafo.
    pub size: u8,
}

/// A escala de cada tamanho do `<font size>` (os pixels do navegador sobre os 16 do
/// normal): 1 = 10, 2 = 13, 4 = 18, 5 = 24, 6 = 32, 7 = 48.
fn size_scale(size: u8) -> f32 {
    match size {
        1 => 0.625,
        2 => 0.8125,
        4 => 1.125,
        5 => 1.5,
        6 => 2.0,
        7 => 3.0,
        _ => 1.0,
    }
}

/// O tamanho de um botão `size-N` da barra.
fn size_name(name: &str) -> Option<u8> {
    name.strip_prefix("size-").and_then(|n| n.parse().ok()).filter(|n| (1..=7).contains(n))
}

/// Os nomes do menu do botão `size`.
const SIZE_NAMES: [&str; 7] = ["Muito pequeno", "Pequeno", "Normal", "Médio", "Grande", "Muito grande", "Enorme"];

#[derive(Clone, Copy)]
enum Flag {
    Bold,
    Italic,
    Underline,
    Strike,
}

impl Flag {
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "bold" => Self::Bold,
            "italic" => Self::Italic,
            "underline" => Self::Underline,
            "strike" => Self::Strike,
            _ => return None,
        })
    }

    fn get(self, f: &Format) -> bool {
        match self {
            Self::Bold => f.bold,
            Self::Italic => f.italic,
            Self::Underline => f.underline,
            Self::Strike => f.strike,
        }
    }

    fn set(self, f: &mut Format, on: bool) {
        match self {
            Self::Bold => f.bold = on,
            Self::Italic => f.italic = on,
            Self::Underline => f.underline = on,
            Self::Strike => f.strike = on,
        }
    }
}

/// O tipo de um parágrafo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlockKind {
    #[default]
    Paragraph,
    H1,
    H2,
    H3,
    /// Item de lista com marcador (`<ul>`).
    Bullet,
    /// Item de lista numerada (`<ol>`).
    Numbered,
}

impl BlockKind {
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "p" => Self::Paragraph,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "unordered" => Self::Bullet,
            "ordered" => Self::Numbered,
            _ => return None,
        })
    }

    fn is_list(self) -> bool {
        matches!(self, Self::Bullet | Self::Numbered)
    }

    fn is_heading(self) -> bool {
        matches!(self, Self::H1 | Self::H2 | Self::H3)
    }

    /// O tamanho da letra em relação ao texto comum.
    fn scale(self) -> f32 {
        match self {
            Self::H1 => 2.0,
            Self::H2 => 1.6,
            Self::H3 => 1.3,
            _ => 1.0,
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Self::Paragraph => "p",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::Bullet | Self::Numbered => "li",
        }
    }
}

/// O formato de um parágrafo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockKind,
    pub align: TextAlign,
}

impl Default for Block {
    fn default() -> Self {
        Self { kind: BlockKind::Paragraph, align: TextAlign::Leading }
    }
}

fn align_name(name: &str) -> Option<TextAlign> {
    Some(match name {
        "left" => TextAlign::Leading,
        "center" => TextAlign::Center,
        "right" => TextAlign::Trailing,
        "justify" => TextAlign::Justify,
        _ => return None,
    })
}

/// O texto com o formato de cada caractere e o de cada linha (as linhas são separadas por
/// `\n`; cada uma é um parágrafo).
#[derive(Clone, Debug, PartialEq)]
struct Doc {
    chars: Vec<char>,
    /// Um por caractere (o do `\n` não conta).
    fmts: Vec<Format>,
    /// Um por linha.
    blocks: Vec<Block>,
}

impl Default for Doc {
    fn default() -> Self {
        Self { chars: Vec::new(), fmts: Vec::new(), blocks: vec![Block::default()] }
    }
}

impl Doc {
    fn len(&self) -> usize {
        self.chars.len()
    }

    /// A linha (a partir de 0) do índice.
    fn line_of(&self, index: usize) -> usize {
        self.chars[..index.min(self.len())].iter().filter(|c| **c == '\n').count()
    }

    /// Começo e fim (sem o `\n`) de cada linha.
    fn lines(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(self.blocks.len());
        let mut start = 0;
        for (i, c) in self.chars.iter().enumerate() {
            if *c == '\n' {
                out.push((start, i));
                start = i + 1;
            }
        }
        out.push((start, self.len()));
        out
    }

    /// Começo e fim da linha onde está o índice.
    fn line_range(&self, index: usize) -> (usize, usize) {
        let i = index.min(self.len());
        let start = self.chars[..i].iter().rposition(|c| *c == '\n').map_or(0, |p| p + 1);
        let end = self.chars[i..].iter().position(|c| *c == '\n').map_or(self.len(), |p| i + p);
        (start, end)
    }

    fn text(&self, a: usize, b: usize) -> String {
        self.chars[a.min(self.len())..b.min(self.len())].iter().collect()
    }

    /// Insere `text` em `at`; cada `\n` abre uma linha com o formato `block` (a de cima fica
    /// com o que tinha).
    fn insert(&mut self, at: usize, text: &[char], fmt: &Format, block: Block) {
        let at = at.min(self.len());
        let line = self.line_of(at);
        let breaks = text.iter().filter(|c| **c == '\n').count();
        self.chars.splice(at..at, text.iter().copied());
        self.fmts.splice(at..at, std::iter::repeat(fmt.clone()).take(text.len()));
        self.blocks.splice(line + 1..line + 1, std::iter::repeat(block).take(breaks));
    }

    /// Apaga de `a` a `b`; as linhas juntadas ficam com o formato da primeira.
    fn remove(&mut self, a: usize, b: usize) {
        let (a, b) = (a.min(self.len()), b.min(self.len()));
        if a >= b {
            return;
        }
        let line = self.line_of(a);
        let breaks = self.chars[a..b].iter().filter(|c| **c == '\n').count();
        self.blocks.drain(line + 1..line + 1 + breaks);
        self.chars.drain(a..b);
        self.fmts.drain(a..b);
    }

    // -----------------------------------------------------------------------
    // HTML

    fn to_html(&self) -> String {
        let lines = self.lines();
        if lines.len() == 1 && self.blocks[0] == Block::default() {
            return self.inline_html(lines[0].0, lines[0].1);
        }
        let mut out = String::new();
        let mut list: Option<BlockKind> = None;
        for (n, &(start, end)) in lines.iter().enumerate() {
            let block = self.blocks[n];
            let wanted = block.kind.is_list().then_some(block.kind);
            if list != wanted {
                match list {
                    Some(BlockKind::Numbered) => out.push_str("</ol>"),
                    Some(_) => out.push_str("</ul>"),
                    None => {}
                }
                match wanted {
                    Some(BlockKind::Numbered) => out.push_str("<ol>"),
                    Some(_) => out.push_str("<ul>"),
                    None => {}
                }
                list = wanted;
            }
            let tag = block.kind.tag();
            let style = match block.align {
                TextAlign::Center => " style=\"text-align:center\"",
                TextAlign::Trailing => " style=\"text-align:right\"",
                TextAlign::Justify => " style=\"text-align:justify\"",
                TextAlign::Leading => "",
            };
            let inner = if start == end { "<br>".to_string() } else { self.inline_html(start, end) };
            out.push_str(&format!("<{tag}{style}>{inner}</{tag}>"));
        }
        match list {
            Some(BlockKind::Numbered) => out.push_str("</ol>"),
            Some(_) => out.push_str("</ul>"),
            None => {}
        }
        out
    }

    /// Os trechos de mesmo formato de `start` a `end`, com as marcas em volta.
    fn inline_html(&self, start: usize, end: usize) -> String {
        let mut out = String::new();
        let mut i = start;
        while i < end {
            let f = &self.fmts[i];
            let mut j = i + 1;
            while j < end && self.fmts[j] == *f {
                j += 1;
            }
            let tags: Vec<&str> = [(f.bold, "b"), (f.italic, "i"), (f.underline, "u"), (f.strike, "s")].into_iter().filter(|t| t.0).map(|t| t.1).collect();
            if let Some(href) = &f.link {
                out.push_str(&format!("<a href=\"{}\">", escape(href, true)));
            }
            if f.size != 0 {
                out.push_str(&format!("<font size=\"{}\">", f.size));
            }
            for t in &tags {
                out.push_str(&format!("<{t}>"));
            }
            out.push_str(&escape(&self.text(i, j), false));
            for t in tags.iter().rev() {
                out.push_str(&format!("</{t}>"));
            }
            if f.size != 0 {
                out.push_str("</font>");
            }
            if f.link.is_some() {
                out.push_str("</a>");
            }
            i = j;
        }
        out
    }

    /// Lê o HTML simples que o [`Doc::to_html`] escreve (e o que um navegador costuma
    /// gerar: `<strong>`, `<em>`, `<div>`, `<br>`, entidades). O resto das marcas é ignorado
    /// e o texto dentro delas fica.
    fn from_html(html: &str) -> Doc {
        let mut reader = HtmlReader::default();
        let mut rest = html;
        while !rest.is_empty() {
            if let Some(after) = rest.strip_prefix('<') {
                if let Some(close) = after.find('>') {
                    reader.tag(&after[..close]);
                    rest = &after[close + 1..];
                } else {
                    reader.text("<");
                    rest = after;
                }
                continue;
            }
            let end = rest.find('<').unwrap_or(rest.len());
            reader.text(&decode(&rest[..end]));
            rest = &rest[end..];
        }
        reader.doc
    }
}

fn escape(s: &str, attribute: bool) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attribute => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// Troca as entidades (`&amp;`, `&#233;`...) pelos caracteres.
fn decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find('&') {
        out.push_str(&rest[..p]);
        let tail = &rest[p..];
        let semi = tail.char_indices().take(12).find(|(_, c)| *c == ';').map(|(i, _)| i);
        let decoded = semi.and_then(|semi| {
            let name = &tail[1..semi];
            let c = match name {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                "nbsp" => Some(' '),
                _ => match name.strip_prefix('#') {
                    Some(hex) if hex.starts_with(['x', 'X']) => u32::from_str_radix(&hex[1..], 16).ok().and_then(char::from_u32),
                    Some(dec) => dec.parse().ok().and_then(char::from_u32),
                    None => None,
                },
            };
            c.map(|c| (c, semi))
        });
        match decoded {
            Some((c, semi)) => {
                out.push(c);
                rest = &tail[semi + 1..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// O valor de um atributo (`href="..."`) no texto de uma marca.
fn attribute(attrs: &str, name: &str) -> Option<String> {
    let lower = attrs.to_ascii_lowercase();
    let mut from = 0;
    while let Some(p) = lower[from..].find(name).map(|p| p + from) {
        from = p + name.len();
        let boundary = p == 0 || lower[..p].ends_with(|c: char| c.is_whitespace());
        let after = lower[from..].trim_start();
        if !boundary || !after.starts_with('=') {
            continue;
        }
        let value = attrs[attrs.len() - after.len() + 1..].trim_start();
        return Some(match value.chars().next() {
            Some(q @ ('"' | '\'')) => value[1..].split(q).next().unwrap_or("").to_string(),
            _ => value.split(|c: char| c.is_whitespace() || c == '/').next().unwrap_or("").to_string(),
        })
        .map(|v| decode(&v));
    }
    None
}

/// O alinhamento de um parágrafo (`style="text-align:center"` ou o antigo `align`).
fn html_align(attrs: &str) -> TextAlign {
    let value = attribute(attrs, "style")
        .and_then(|s| {
            let s = s.to_ascii_lowercase();
            let p = s.find("text-align")?;
            Some(s[p..].split(':').nth(1)?.split(';').next()?.trim().to_string())
        })
        .or_else(|| attribute(attrs, "align").map(|a| a.to_ascii_lowercase()));
    match value.as_deref() {
        Some("center") => TextAlign::Center,
        Some("right" | "end") => TextAlign::Trailing,
        Some("justify") => TextAlign::Justify,
        _ => TextAlign::Leading,
    }
}

#[derive(Default)]
struct HtmlReader {
    doc: Doc,
    bold: u32,
    italic: u32,
    underline: u32,
    strike: u32,
    links: Vec<Option<String>>,
    /// Os `<font size>` abertos (0: sem tamanho).
    sizes: Vec<u8>,
    lists: Vec<BlockKind>,
    /// A linha de agora já foi aberta (por uma marca de parágrafo ou por texto).
    used: bool,
    /// Um parágrafo acabou de fechar: o próximo texto abre outro.
    closed: bool,
}

impl HtmlReader {
    fn format(&self) -> Format {
        Format {
            bold: self.bold > 0,
            italic: self.italic > 0,
            underline: self.underline > 0,
            strike: self.strike > 0,
            link: self.links.iter().rev().find_map(|l| l.clone()),
            size: self.sizes.last().copied().unwrap_or(0),
        }
    }

    fn line_empty(&self) -> bool {
        self.doc.chars.last().is_none_or(|c| *c == '\n')
    }

    fn open(&mut self, block: Block) {
        if self.used {
            let at = self.doc.len();
            self.doc.insert(at, &['\n'], &Format::default(), block);
        } else if let Some(last) = self.doc.blocks.last_mut() {
            *last = block;
        }
        self.used = true;
        self.closed = false;
    }

    fn text(&mut self, s: &str) {
        if s.trim().is_empty() && (self.closed || self.line_empty()) {
            return;
        }
        if self.closed {
            self.open(Block::default());
        }
        let mut text: String = s.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).collect();
        if self.line_empty() {
            text = text.trim_start().to_string();
        }
        let chars: Vec<char> = text.chars().collect();
        let at = self.doc.len();
        let fmt = self.format();
        self.doc.insert(at, &chars, &fmt, Block::default());
        self.used = true;
    }

    fn tag(&mut self, raw: &str) {
        let raw = raw.trim();
        if raw.starts_with(['!', '?']) {
            return;
        }
        let closing = raw.starts_with('/');
        let body = raw.trim_start_matches('/');
        let name: String = body.chars().take_while(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase();
        let attrs = &body[name.len()..];
        let count = |n: &mut u32| if closing { *n = n.saturating_sub(1) } else { *n += 1 };
        match name.as_str() {
            "b" | "strong" => count(&mut self.bold),
            "i" | "em" => count(&mut self.italic),
            "u" | "ins" => count(&mut self.underline),
            "s" | "strike" | "del" => count(&mut self.strike),
            "a" if closing => {
                self.links.pop();
            }
            "a" => self.links.push(attribute(attrs, "href")),
            "font" if closing => {
                self.sizes.pop();
            }
            "font" => {
                // `size="5"`, ou relativo ao de fora (`+1`, `-2`); o 3 é o normal.
                let current = match self.sizes.last().copied().unwrap_or(0) {
                    0 => 3,
                    s => s,
                };
                let size = attribute(attrs, "size").and_then(|s| {
                    let s = s.trim();
                    let n: i32 = s.trim_start_matches('+').parse().ok()?;
                    Some(if s.starts_with(['+', '-']) { current as i32 + n } else { n }.clamp(1, 7) as u8)
                });
                self.sizes.push(match size {
                    Some(3) => 0,
                    Some(s) => s,
                    None => self.sizes.last().copied().unwrap_or(0),
                });
            }
            "ul" | "ol" if closing => {
                self.lists.pop();
                self.closed = true;
            }
            "ul" => self.lists.push(BlockKind::Bullet),
            "ol" => self.lists.push(BlockKind::Numbered),
            "br" => {
                // Quebra no meio do parágrafo: outro parágrafo igual (o `<br>` de um vazio
                // não conta).
                if !self.line_empty() {
                    let block = self.doc.blocks.last().copied().unwrap_or_default();
                    self.open(block);
                }
            }
            "p" | "div" | "li" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" if closing => self.closed = true,
            "p" | "div" | "li" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let kind = match name.as_str() {
                    "li" => self.lists.last().copied().unwrap_or(BlockKind::Bullet),
                    "h1" => BlockKind::H1,
                    "h2" => BlockKind::H2,
                    "h3" | "h4" | "h5" | "h6" => BlockKind::H3,
                    _ => BlockKind::Paragraph,
                };
                // Um `<div>` dentro de um item de lista continua o item.
                if name == "div" && self.lists.last().is_some() && !self.closed {
                    return;
                }
                self.open(Block { kind, align: html_align(attrs) });
            }
            _ => {}
        }
    }
}

// ===========================================================================
// Layout do texto
// ===========================================================================

/// Uma linha na tela: de `start` a `end` (sem o `\n`), com a posição de cada caractere.
struct VisualLine {
    /// A linha do documento (o parágrafo).
    line: usize,
    start: usize,
    end: usize,
    /// O topo, a partir do começo do texto.
    y: f32,
    height: f32,
    /// A posição de cada divisa entre caracteres (`end - start + 1`), a partir da esquerda
    /// do texto.
    xs: Vec<f32>,
    /// O marcador da lista (`•`, `3.`), na primeira linha do item.
    marker: Option<String>,
    /// A linha de base, a partir do topo da linha: letras de tamanhos diferentes se
    /// apoiam nela.
    baseline: f32,
}

/// A faixa de um trecho de letra `size` apoiado na linha de base `baseline` (o texto vai
/// centralizado nela): o topo e a altura.
fn run_band(baseline: f32, size: f32) -> (f32, f32) {
    (baseline - 1.05 * size, 1.5 * size)
}

struct Layout {
    lines: Vec<VisualLine>,
    height: f32,
}

/// O estilo de um caractere: o tamanho e o peso do parágrafo, mais o formato dele.
fn char_style(base: &TextStyle, block: Block, fmt: &Format) -> TextStyle {
    let mut s = base.clone();
    s.font.size = char_size(base, block, fmt);
    if fmt.bold || block.kind.is_heading() {
        s.font.weight = FontWeight::Bold;
    }
    // O itálico do controle (`font_italic`) vale para tudo; o do trecho soma.
    s.font.italic = base.font.italic || fmt.italic;
    s.font.underline = fmt.underline || fmt.link.is_some();
    s.font.strikeout = fmt.strike;
    s.h_align = TextAlign::Leading;
    s.v_align = TextAlign::Center;
    s.wrap = false;
    s.trimming = Trimming::None;
    s
}

/// O tamanho da letra: o do `<font size>` (vale mais, como no navegador) ou o do parágrafo.
fn char_size(base: &TextStyle, block: Block, fmt: &Format) -> f32 {
    if fmt.size != 0 { base.font.size * size_scale(fmt.size) } else { base.font.size * block.kind.scale() }
}

/// Quebra os parágrafos na largura `width` (nas palavras; palavra maior que a linha, no
/// caractere).
fn layout(doc: &Doc, width: f32, base: &TextStyle, measure: &mut dyn FnMut(&str, &TextStyle) -> f32) -> Layout {
    let mut widths_cache: HashMap<(char, u32, bool, bool), f32> = HashMap::new();
    let mut out = Vec::new();
    let mut y = 0.0;
    let mut number = 0;
    for (n, (start, end)) in doc.lines().into_iter().enumerate() {
        let block = doc.blocks.get(n).copied().unwrap_or_default();
        number = if block.kind == BlockKind::Numbered { number + 1 } else { 0 };
        let block_size = base.font.size * block.kind.scale();
        let indent = if block.kind.is_list() { LIST_INDENT } else { 0.0 };
        let room = (width - indent).max(10.0);
        let widths: Vec<f32> = (start..end)
            .map(|i| {
                let fmt = &doc.fmts[i];
                let bold = fmt.bold || block.kind.is_heading();
                let size = char_size(base, block, fmt);
                *widths_cache.entry((doc.chars[i], size.to_bits(), bold, fmt.italic)).or_insert_with(|| {
                    let style = char_style(base, block, fmt);
                    measure(&doc.chars[i].to_string(), &style)
                })
            })
            .collect();
        let mut rows = Vec::new();
        let (mut a, mut i, mut x) = (0, 0, 0.0);
        let mut space: Option<usize> = None;
        while i < widths.len() {
            if x + widths[i] > room && i > a {
                let cut = match space {
                    Some(s) if s > a => s,
                    _ => i,
                };
                rows.push((a, cut));
                a = cut;
                x = widths[a..i].iter().sum();
                space = None;
                continue;
            }
            x += widths[i];
            if doc.chars[start + i] == ' ' {
                space = Some(i + 1);
            }
            i += 1;
        }
        rows.push((a, widths.len()));
        let count = rows.len();
        for (k, (ra, rb)) in rows.into_iter().enumerate() {
            let w: f32 = widths[ra..rb].iter().sum();
            let x0 = indent
                + match block.align {
                    TextAlign::Leading | TextAlign::Justify => 0.0,
                    TextAlign::Center => ((room - w) / 2.0).max(0.0),
                    TextAlign::Trailing => (room - w).max(0.0),
                };
            // Justificado: nas linhas quebradas (não na última do parágrafo), o que sobra
            // vai para os espaços entre as palavras (os do fim da linha não contam).
            let is_space = |i: usize| doc.chars[start + i] == ' ';
            let body_end = (ra..rb).rev().find(|&i| !is_space(i)).map_or(ra, |i| i + 1);
            let gaps = (ra..body_end).filter(|&i| is_space(i)).count();
            let used: f32 = widths[ra..body_end].iter().sum();
            let extra = if block.align == TextAlign::Justify && k + 1 < count && gaps > 0 { ((room - used) / gaps as f32).max(0.0) } else { 0.0 };
            let mut xs = Vec::with_capacity(rb - ra + 1);
            let mut x = x0;
            xs.push(x);
            for (i, w) in widths[ra..rb].iter().enumerate() {
                x += w;
                if extra > 0.0 && ra + i < body_end && is_space(ra + i) {
                    x += extra;
                }
                xs.push(x);
            }
            let marker = match block.kind {
                BlockKind::Bullet if k == 0 => Some("•".to_string()),
                BlockKind::Numbered if k == 0 => Some(format!("{number}.")),
                _ => None,
            };
            // A altura vem da maior letra da linha (nunca menor que a do parágrafo, como a
            // linha do navegador); a base fica onde a maior se apoia.
            let tallest = (start + ra..start + rb).map(|i| char_size(base, block, &doc.fmts[i])).fold(block_size, f32::max);
            let height = (tallest * 1.5).round().max(10.0);
            let baseline = height / 2.0 + 0.3 * tallest;
            out.push(VisualLine { line: n, start: start + ra, end: start + rb, y, height, xs, marker, baseline });
            y += height;
        }
    }
    Layout { lines: out, height: y }
}

/// A linha na tela onde fica o índice (numa quebra automática, o começo da de baixo).
fn visual_line_of(layout: &Layout, index: usize) -> usize {
    layout.lines.iter().rposition(|l| l.start <= index).unwrap_or(0)
}

// ===========================================================================
// RichEditor
// ===========================================================================

type Snapshot = (Doc, usize, usize);

/// O editor de texto rico (`QEditor`): o texto com formato e, em cima, a barra de
/// ferramentas.
///
/// - `html`: o conteúdo (lido e escrito; muda a cada edição);
/// - `toolbar`: os botões, separados por `;` (`|` é um separador): `bold`, `italic`,
///   `underline`, `strike`, `size` (menu com os 7 tamanhos), `size-1` a `size-7` (um
///   tamanho direto), `h1`, `h2`, `h3`, `p`, `unordered`, `ordered`, `left`, `center`,
///   `right`, `justify`, `link`, `undo`, `redo` e `removeFormat`; vazio esconde a barra;
///   os títulos valem para o parágrafo inteiro e o tamanho, para o trecho selecionado;
/// - `on_change` a cada mudança, com o `html`.
///
/// Teclado: Ctrl+B/I/U (negrito, itálico, sublinhado), Ctrl+Z/Ctrl+Y (desfazer, refazer),
/// Ctrl+A/C/X/V (o colado entra como texto puro), setas, Home/End (com Ctrl, do texto todo).
/// Sem seleção, o negrito (e os outros) vale para o que for digitado em seguida. O link pede
/// o endereço num campo em cima da barra (Enter grava, Esc desiste); num link, o botão tira
/// o link.
pub struct RichEditor {
    pub base: ControlBase,
    /// O conteúdo em HTML (lido e escrito; muda a cada edição).
    pub html: String,
    /// Os botões da barra, separados por `;` (`|` é um separador): `bold`, `italic`, `h1`,
    /// `link`, `undo`... Vazio esconde a barra.
    pub toolbar: String,
    /// O texto apagado mostrado com o conteúdo vazio.
    pub placeholder: String,
    /// Só leitura: dá para selecionar e copiar, mas não editar.
    pub read_only: bool,
    doc: Doc,
    caret: usize,
    anchor: usize,
    /// O formato escolhido sem seleção, para o próximo texto digitado.
    typing: Option<Format>,
    goal_x: Option<f32>,
    undo_stack: Vec<Snapshot>,
    redo_stack: Vec<Snapshot>,
    /// A última mudança foi digitação: a próxima entra no mesmo desfazer.
    merging: bool,
    /// Sobe a cada mudança do documento (o layout guardado vale para uma versão).
    version: u64,
    cached: RefCell<Option<(u64, u32, u32, Rc<Layout>)>>,
    /// A fonte e a cor do texto na última pintura (o layout dos eventos usa a mesma).
    text_style: RefCell<TextStyle>,
    scroll: Cell<f32>,
    /// O cursor e a versão na última pintura: a rolagem só vai atrás do cursor quando um
    /// dos dois muda.
    followed: Cell<(usize, u64)>,
    /// A altura do texto e a da área visível na última pintura (para a roda).
    extent: Cell<(f32, f32)>,
    hover: Cell<Option<usize>>,
    dragging: bool,
    link_editor: Cell<Option<ComponentId>>,
    /// O menu do botão `size` (criado na primeira vez).
    size_menu: Option<ComponentId>,
    blink_start: f64,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

impl Default for RichEditor {
    fn default() -> Self {
        let mut base = ControlBase::new(460.0, 240.0);
        base.set_can_focus(true).set_cursor(CursorShape::Text);
        Self { overscroll: aylis_core::Overscroll::Auto,
            base,
            html: String::new(),
            toolbar: DEFAULT_TOOLBAR.into(),
            placeholder: "Digite aqui…".into(),
            read_only: false,
            doc: Doc::default(),
            caret: 0,
            anchor: 0,
            typing: None,
            goal_x: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            merging: false,
            version: 0,
            cached: RefCell::new(None),
            text_style: RefCell::default(),
            scroll: Cell::new(0.0),
            followed: Cell::new((usize::MAX, u64::MAX)),
            extent: Cell::new((0.0, 0.0)),
            hover: Cell::new(None),
            dragging: false,
            link_editor: Cell::new(None),
            size_menu: None,
            blink_start: 0.0,
        }
    }
}

impl RichEditor {
    pub fn new(html: impl Into<String>) -> Self {
        let mut editor = Self { html: html.into(), ..Default::default() };
        editor.after_set("html");
        editor
    }

    /// O texto sem formato (os parágrafos separados por `\n`).
    pub fn plain_text(&self) -> String {
        self.doc.chars.iter().collect()
    }

    /// A seleção (começo, fim), em caracteres do texto sem formato.
    pub fn selection(&self) -> (usize, usize) {
        (self.caret.min(self.anchor), self.caret.max(self.anchor))
    }

    /// Seleciona de `start` a `end` (em caracteres do texto sem formato; iguais: só o cursor).
    pub fn select(&mut self, start: usize, end: usize) {
        self.anchor = start.min(self.doc.len());
        self.caret = end.min(self.doc.len());
        self.typing = None;
        self.goal_x = None;
        self.merging = false;
    }

    pub fn select_all(&mut self) {
        self.select(0, self.doc.len());
    }

    /// O formato do texto selecionado (o que vale para todo ele) ou o do cursor.
    pub fn current_format(&self) -> Format {
        if let Some(f) = &self.typing {
            return f.clone();
        }
        let (a, b) = self.selection();
        if a == b {
            return self.format_at_caret();
        }
        let mut f = Format { bold: true, italic: true, underline: true, strike: true, link: None, size: 0 };
        // O tamanho: o de todos, ou `u8::MAX` se a seleção mistura tamanhos.
        let mut size = None;
        for i in (a..b).filter(|&i| self.doc.chars[i] != '\n') {
            let c = &self.doc.fmts[i];
            f.bold &= c.bold;
            f.italic &= c.italic;
            f.underline &= c.underline;
            f.strike &= c.strike;
            if f.link.is_none() {
                f.link = c.link.clone();
            }
            size = match size {
                None => Some(c.size),
                Some(s) if s == c.size => Some(s),
                Some(_) => Some(u8::MAX),
            };
        }
        f.size = size.unwrap_or(0);
        f
    }

    /// O formato do parágrafo do cursor.
    pub fn current_block(&self) -> Block {
        self.doc.blocks.get(self.doc.line_of(self.caret)).copied().unwrap_or_default()
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Executa um botão da barra pelo nome (`bold`, `h1`, `ordered`, `center`, `undo`,
    /// `removeFormat`...). O `link` daqui só tira o link (pôr um é [`RichEditor::set_link`]).
    /// Devolve se o conteúdo mudou (o `html` já está atualizado; o `on_change` fica com
    /// quem chamou).
    pub fn command(&mut self, name: &str) -> bool {
        let (a, b) = self.selection();
        if name == "undo" || name == "redo" {
            let (from, to) = if name == "undo" { (&mut self.undo_stack, &mut self.redo_stack) } else { (&mut self.redo_stack, &mut self.undo_stack) };
            let Some((doc, caret, anchor)) = from.pop() else { return false };
            to.push((std::mem::replace(&mut self.doc, doc), self.caret, self.anchor));
            self.caret = caret.min(self.doc.len());
            self.anchor = anchor.min(self.doc.len());
            self.typing = None;
            self.merging = false;
            self.commit();
            return true;
        }
        if self.read_only {
            return false;
        }
        if let Some(n) = size_name(name) {
            // O 3 é o normal: guarda sem tamanho (o `<font>` não aparece no html).
            let size = if n == 3 { 0 } else { n };
            if a == b {
                let mut f = self.typing.clone().unwrap_or_else(|| self.format_at_caret());
                f.size = size;
                self.typing = Some(f);
                self.merging = false;
                return false;
            }
            self.snapshot(false);
            for f in &mut self.doc.fmts[a..b] {
                f.size = size;
            }
            self.commit();
            return true;
        }
        if let Some(flag) = Flag::from_name(name) {
            if a == b {
                // Sem seleção: vale para o que for digitado.
                let mut f = self.typing.clone().unwrap_or_else(|| self.format_at_caret());
                let on = !flag.get(&f);
                flag.set(&mut f, on);
                self.typing = Some(f);
                // O que vier depois é outro passo do desfazer.
                self.merging = false;
                return false;
            }
            let all = (a..b).filter(|&i| self.doc.chars[i] != '\n').all(|i| flag.get(&self.doc.fmts[i]));
            self.snapshot(false);
            for f in &mut self.doc.fmts[a..b] {
                flag.set(f, !all);
            }
            self.commit();
            return true;
        }
        if let Some(kind) = BlockKind::from_name(name) {
            let lines = self.selected_lines();
            let all = lines.clone().all(|l| self.doc.blocks[l].kind == kind);
            let new = if all && kind != BlockKind::Paragraph { BlockKind::Paragraph } else { kind };
            self.snapshot(false);
            for l in lines {
                self.doc.blocks[l].kind = new;
            }
            self.commit();
            return true;
        }
        if let Some(align) = align_name(name) {
            self.snapshot(false);
            for l in self.selected_lines() {
                self.doc.blocks[l].align = align;
            }
            self.commit();
            return true;
        }
        match name {
            "removeFormat" => {
                // Sem seleção, limpa o parágrafo do cursor. O link fica (como no navegador).
                let (a, b) = if a == b { self.doc.line_range(a) } else { (a, b) };
                self.snapshot(false);
                for f in &mut self.doc.fmts[a..b] {
                    *f = Format { link: f.link.take(), ..Default::default() };
                }
                for l in self.doc.line_of(a)..=self.doc.line_of(b) {
                    let block = &mut self.doc.blocks[l];
                    if block.kind.is_heading() {
                        block.kind = BlockKind::Paragraph;
                    }
                    block.align = TextAlign::Leading;
                }
                self.typing = None;
                self.commit();
                true
            }
            "link" => self.unlink(),
            _ => false,
        }
    }

    /// Põe o link `url` no texto selecionado (vazio: tira). Devolve se mudou.
    pub fn set_link(&mut self, url: &str) -> bool {
        let (a, b) = self.selection();
        if a == b || self.read_only {
            return false;
        }
        let link = (!url.trim().is_empty()).then(|| url.trim().to_string());
        self.snapshot(false);
        for i in a..b {
            if self.doc.chars[i] != '\n' {
                self.doc.fmts[i].link = link.clone();
            }
        }
        self.commit();
        true
    }

    /// Há link na seleção (ou encostado no cursor).
    fn has_link(&self) -> bool {
        let (a, b) = self.selection();
        let d = &self.doc;
        if a < b {
            return d.fmts[a..b].iter().any(|f| f.link.is_some());
        }
        (a < d.len() && d.fmts[a].link.is_some() && d.chars[a] != '\n') || (a > 0 && d.fmts[a - 1].link.is_some() && d.chars[a - 1] != '\n')
    }

    /// Tira o link da seleção ou, sem seleção, o link inteiro onde está o cursor.
    fn unlink(&mut self) -> bool {
        if !self.has_link() {
            return false;
        }
        let (a, b) = self.selection();
        let (a, b) = if a < b {
            (a, b)
        } else {
            let d = &self.doc;
            let at = if a < d.len() && d.fmts[a].link.is_some() { a } else { a - 1 };
            let link = d.fmts[at].link.clone();
            let mut lo = at;
            while lo > 0 && d.fmts[lo - 1].link == link {
                lo -= 1;
            }
            let mut hi = at + 1;
            while hi < d.len() && d.fmts[hi].link == link {
                hi += 1;
            }
            (lo, hi)
        };
        self.snapshot(false);
        for f in &mut self.doc.fmts[a..b] {
            f.link = None;
        }
        self.commit();
        true
    }

    /// As linhas que a seleção pega (o fim encostado no começo de uma linha não a conta).
    fn selected_lines(&self) -> std::ops::RangeInclusive<usize> {
        let (a, b) = self.selection();
        let first = self.doc.line_of(a);
        let mut last = self.doc.line_of(b);
        if b > a && self.doc.chars[b - 1] == '\n' {
            last -= 1;
        }
        first..=last.max(first)
    }

    /// O formato para o que for digitado no cursor: o do caractere de antes (no começo da
    /// linha, o de depois); no fim de um link, fora do link.
    fn format_at_caret(&self) -> Format {
        let d = &self.doc;
        let c = self.caret.min(d.len());
        let next_link = (c < d.len() && d.chars[c] != '\n').then(|| d.fmts[c].link.clone()).flatten();
        if c > 0 && d.chars[c - 1] != '\n' {
            let mut f = d.fmts[c - 1].clone();
            if f.link != next_link {
                f.link = None;
            }
            f
        } else if c < d.len() && d.chars[c] != '\n' {
            Format { link: None, ..d.fmts[c].clone() }
        } else {
            Format::default()
        }
    }

    /// Guarda o estado de antes para o desfazer (digitações seguidas viram uma só).
    fn snapshot(&mut self, typing: bool) {
        if !(typing && self.merging) {
            self.undo_stack.push((self.doc.clone(), self.caret, self.anchor));
            if self.undo_stack.len() > UNDO_LIMIT {
                self.undo_stack.remove(0);
            }
        }
        self.redo_stack.clear();
        self.merging = typing;
    }

    /// O documento mudou: o `html` acompanha.
    fn commit(&mut self) {
        self.version += 1;
        self.html = self.doc.to_html();
    }

    fn after_set(&mut self, name: &str) {
        if name == "html" {
            self.doc = Doc::from_html(&self.html);
            let end = self.doc.len();
            self.caret = end;
            self.anchor = end;
            self.typing = None;
            self.undo_stack.clear();
            self.redo_stack.clear();
            self.merging = false;
            self.version += 1;
        }
    }

    /// Troca a seleção por `text` (`\n` abre parágrafos iguais ao do cursor).
    fn insert(&mut self, text: &str) -> bool {
        if self.read_only {
            return false;
        }
        let clean: Vec<char> = text.replace("\r\n", "\n").chars().map(|c| if c == '\t' { ' ' } else { c }).filter(|c| *c == '\n' || !c.is_control()).collect();
        let (a, b) = self.selection();
        if clean.is_empty() && a == b {
            return false;
        }
        // Escrevendo por cima de uma seleção, fica o formato do começo dela.
        let fmt = match &self.typing {
            Some(f) => f.clone(),
            None if a < b && self.doc.chars[a] != '\n' => self.doc.fmts[a].clone(),
            None => self.format_at_caret(),
        };
        self.snapshot(!clean.contains(&'\n') && a == b);
        self.doc.remove(a, b);
        let block = self.doc.blocks[self.doc.line_of(a)];
        self.doc.insert(a, &clean, &fmt, block);
        self.caret = a + clean.len();
        self.anchor = self.caret;
        self.commit();
        true
    }

    /// Enter: um parágrafo novo. Depois de um título vem um parágrafo comum; num item de
    /// lista vazio, sai da lista.
    fn new_paragraph(&mut self) -> bool {
        if self.read_only {
            return false;
        }
        let (a, b) = self.selection();
        let line = self.doc.line_of(a);
        let block = self.doc.blocks[line];
        let (start, end) = self.doc.line_range(a);
        if a == b && start == end && block.kind.is_list() {
            self.snapshot(false);
            self.doc.blocks[line].kind = BlockKind::Paragraph;
            self.commit();
            return true;
        }
        let fmt = self.typing.clone().unwrap_or_else(|| self.format_at_caret());
        self.snapshot(false);
        self.doc.remove(a, b);
        let at_start = a == start && start != end;
        let next = if block.kind.is_heading() && !at_start { Block { kind: BlockKind::Paragraph, ..block } } else { block };
        self.doc.insert(a, &['\n'], &fmt, next);
        if at_start && block.kind.is_heading() {
            // Enter no começo de um título: a linha vazia de cima é um parágrafo comum.
            self.doc.blocks[line] = Block { kind: BlockKind::Paragraph, ..block };
        }
        self.caret = a + 1;
        self.anchor = self.caret;
        self.commit();
        true
    }

    fn delete_selection(&mut self) -> bool {
        let (a, b) = self.selection();
        if a == b || self.read_only {
            return false;
        }
        self.snapshot(false);
        self.doc.remove(a, b);
        self.caret = a;
        self.anchor = a;
        self.commit();
        true
    }

    fn backspace(&mut self, word: bool) -> bool {
        if self.read_only {
            return false;
        }
        if self.delete_selection() {
            return true;
        }
        let c = self.caret;
        let line = self.doc.line_of(c);
        let block = self.doc.blocks[line];
        // No começo de um item de lista (ou de um título no começo do texto), volta a ser
        // parágrafo comum.
        if (c == 0 || self.doc.chars[c - 1] == '\n') && (block.kind.is_list() || (c == 0 && block.kind != BlockKind::Paragraph)) {
            self.snapshot(false);
            self.doc.blocks[line].kind = BlockKind::Paragraph;
            self.commit();
            return true;
        }
        if c == 0 {
            return false;
        }
        let from = if word { self.prev_word(c) } else { c - 1 };
        self.snapshot(false);
        self.doc.remove(from, c);
        self.caret = from;
        self.anchor = from;
        self.commit();
        true
    }

    fn delete_forward(&mut self, word: bool) -> bool {
        if self.read_only {
            return false;
        }
        if self.delete_selection() {
            return true;
        }
        let c = self.caret;
        if c >= self.doc.len() {
            return false;
        }
        let to = if word { self.next_word(c) } else { c + 1 };
        self.snapshot(false);
        self.doc.remove(c, to);
        self.commit();
        true
    }

    fn move_to(&mut self, index: usize, extend: bool) {
        self.caret = index.min(self.doc.len());
        if !extend {
            self.anchor = self.caret;
        }
        self.typing = None;
        self.merging = false;
    }

    fn word_bounds(&self, index: usize) -> (usize, usize) {
        let chars = &self.doc.chars;
        let word = |c: char| c.is_alphanumeric() || c == '_';
        let mut a = index.min(chars.len());
        let mut b = a;
        while a > 0 && word(chars[a - 1]) {
            a -= 1;
        }
        while b < chars.len() && word(chars[b]) {
            b += 1;
        }
        (a, b)
    }

    fn prev_word(&self, from: usize) -> usize {
        let chars = &self.doc.chars;
        let mut i = from.min(chars.len());
        while i > 0 && chars[i - 1].is_whitespace() {
            i -= 1;
        }
        while i > 0 && !chars[i - 1].is_whitespace() {
            i -= 1;
        }
        i
    }

    fn next_word(&self, from: usize) -> usize {
        let chars = &self.doc.chars;
        let mut i = from.min(chars.len());
        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }
        while i < chars.len() && chars[i].is_whitespace() && chars[i] != '\n' {
            i += 1;
        }
        i.max(from + 1).min(chars.len())
    }

    // -----------------------------------------------------------------------
    // Barra e layout

    fn tools(&self) -> Vec<&str> {
        self.toolbar.split(';').map(str::trim).filter(|t| *t == "|" || TOOLS.contains(t) || size_name(t).is_some()).collect()
    }

    /// Os botões (nome e retângulo; os separadores também) e a altura da barra.
    fn toolbar_layout(&self, width: f32) -> (Vec<(&str, RectF)>, f32) {
        let mut out = Vec::new();
        let (mut x, mut row) = (PAD, 0usize);
        for name in self.tools() {
            let w = if name == "|" { SEPARATOR } else { BUTTON_WIDTH };
            if x + w > width - PAD && x > PAD {
                x = PAD;
                row += 1;
            }
            if name == "|" && x == PAD {
                continue;
            }
            out.push((name, RectF::new(x, row as f32 * ROW + 3.0, w, BUTTON)));
            x += w + 2.0;
        }
        let height = if out.is_empty() { 0.0 } else { (row + 1) as f32 * ROW };
        (out, height)
    }

    /// A área do texto (abaixo da barra).
    fn content_rect(&self, size: RectF) -> RectF {
        let (_, bar) = self.toolbar_layout(size.width);
        RectF::new(size.x + CONTENT_PAD, size.y + bar + CONTENT_PAD, (size.width - 2.0 * CONTENT_PAD).max(1.0), (size.height - bar - 2.0 * CONTENT_PAD).max(1.0))
    }

    fn local_rect(&self) -> RectF {
        let b = self.base.bounds();
        RectF::new(0.0, 0.0, b.width, b.height)
    }

    fn layout_with(&self, width: f32, base: &TextStyle, measure: &mut dyn FnMut(&str, &TextStyle) -> f32) -> Rc<Layout> {
        let key = (self.version, width.to_bits(), base.font.size.to_bits());
        if let Some((v, w, s, l)) = &*self.cached.borrow() {
            if (*v, *w, *s) == key {
                return l.clone();
            }
        }
        let l = Rc::new(layout(&self.doc, width, base, measure));
        *self.cached.borrow_mut() = Some((key.0, key.1, key.2, l.clone()));
        l
    }

    /// O layout nos eventos (medido com o `Ui`).
    fn layout_ui(&self, ui: &Ui) -> Rc<Layout> {
        let content = self.content_rect(self.local_rect());
        let base = self.text_style.borrow().clone();
        self.layout_with(content.width, &base, &mut |t, s| ui.measure_text(t, s).width)
    }

    /// O último lugar do cursor numa linha da tela (numa quebra automática, antes do espaço
    /// da quebra).
    fn line_last(&self, l: &VisualLine) -> usize {
        if l.end >= self.doc.len() || self.doc.chars[l.end] == '\n' { l.end } else { l.end.saturating_sub(1).max(l.start) }
    }

    fn index_in_line(&self, l: &VisualLine, x: f32) -> usize {
        let last = self.line_last(l) - l.start;
        (0..=last).min_by(|&p, &q| (l.xs[p] - x).abs().total_cmp(&(l.xs[q] - x).abs())).map_or(l.start, |k| l.start + k)
    }

    /// O caractere sob um ponto (local).
    fn index_at(&self, ui: &Ui, p: PointF) -> usize {
        let content = self.content_rect(self.local_rect());
        let layout = self.layout_ui(ui);
        let y = p.y - content.y + self.scroll.get();
        let i = layout.lines.iter().position(|l| y < l.y + l.height).unwrap_or(layout.lines.len() - 1);
        self.index_in_line(&layout.lines[i], p.x - content.x)
    }

    fn caret_x(l: &VisualLine, index: usize) -> f32 {
        l.xs[index.saturating_sub(l.start).min(l.xs.len() - 1)]
    }

    /// Sobe ou desce uma linha da tela, mantendo a posição horizontal.
    fn vertical(&mut self, ui: &Ui, dir: isize, extend: bool) {
        let layout = self.layout_ui(ui);
        let current = visual_line_of(&layout, self.caret);
        let goal = *self.goal_x.get_or_insert(Self::caret_x(&layout.lines[current], self.caret));
        let target = current as isize + dir;
        let index = if target < 0 {
            0
        } else if target as usize >= layout.lines.len() {
            self.doc.len()
        } else {
            self.index_in_line(&layout.lines[target as usize], goal)
        };
        self.move_to(index, extend);
        self.goal_x = Some(goal);
    }

    fn home_end(&mut self, ui: &Ui, end: bool, extend: bool) {
        let layout = self.layout_ui(ui);
        let l = &layout.lines[visual_line_of(&layout, self.caret)];
        let to = if end { self.line_last(l) } else { l.start };
        self.move_to(to, extend);
    }

    fn is_active(&self, name: &str) -> bool {
        if let Some(flag) = Flag::from_name(name) {
            return flag.get(&self.current_format());
        }
        if let Some(kind) = BlockKind::from_name(name) {
            return kind != BlockKind::Paragraph && self.current_block().kind == kind;
        }
        if let Some(align) = align_name(name) {
            return align != TextAlign::Leading && self.current_block().align == align;
        }
        if let Some(n) = size_name(name) {
            let size = self.current_format().size;
            return size == n || (n == 3 && size == 0);
        }
        match name {
            "size" => !matches!(self.current_format().size, 0 | u8::MAX),
            "link" => self.has_link(),
            _ => false,
        }
    }

    fn is_enabled(&self, name: &str) -> bool {
        match name {
            "undo" => self.can_undo(),
            "redo" => self.can_redo(),
            "link" => !self.read_only && (self.has_link() || self.caret != self.anchor),
            _ => !self.read_only,
        }
    }

    fn changed(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_change", EventArgs::Value(self.html.clone().into()));
        self.blink_start = ctx.ui.time();
        ctx.repaint();
    }

    /// Um botão da barra foi apertado.
    fn press(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "link" && !self.read_only && !self.has_link() {
            if self.caret != self.anchor {
                self.open_link_editor(ctx);
            }
            return;
        }
        if name == "size" {
            self.open_size_menu(ctx);
            return;
        }
        if self.command(name) {
            self.changed(ctx);
        }
        ctx.repaint();
    }

    /// O menu do botão `size` (o tamanho do texto selecionado ou do que for digitado),
    /// embaixo do botão, com o tamanho de agora marcado.
    fn open_size_menu(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        let me = ctx.id;
        let menu = match self.size_menu.filter(|&m| ctx.ui.contains(m)) {
            Some(m) => m,
            None => {
                let m = ctx.ui.add(crate::PopupMenu::default(), me);
                self.size_menu = Some(m);
                m
            }
        };
        let children = ctx.ui.children(menu).to_vec();
        ctx.ui.free_many(&children);
        let current = match self.current_format().size {
            0 => 3,
            s => s,
        };
        for (i, label) in SIZE_NAMES.iter().enumerate() {
            let n = i as u8 + 1;
            let item = ctx.ui.add(crate::MenuItem::new(format!("{n} · {}", t(label))), menu);
            let _ = ctx.ui.set_prop(item, "checked", n == current);
            ctx.ui.on_click(item, move |ui, _| {
                let html = ui.get_mut::<RichEditor>(me).and_then(|r| r.command(&format!("size-{n}")).then(|| r.html.clone()));
                if let Some(html) = html {
                    ui.fire(me, "on_change", &EventArgs::Value(html.into()));
                }
                ui.set_focus(me);
                ui.request_repaint();
            });
        }
        let (tools, _) = self.toolbar_layout(self.local_rect().width);
        let below = tools.iter().find(|(n, _)| *n == "size").map_or(PointF::new(PAD, ROW), |(_, r)| PointF::new(r.x, r.bottom() + 2.0));
        let origin = ctx.ui.absolute_rect(me).map_or(PointF::ZERO, |r| r.origin());
        crate::PopupMenu::popup(ctx.ui, menu, PointF::new(origin.x + below.x, origin.y + below.y));
    }

    /// O campo do endereço do link, em cima da barra: Enter (ou sair dele) põe o link no
    /// texto selecionado; Esc desiste.
    fn open_link_editor(&self, ctx: &mut EventCtx<'_>) {
        if self.link_editor.get().is_some_and(|e| ctx.ui.contains(e)) || ctx.ui.design_mode() {
            return;
        }
        let me = ctx.id;
        let Ok(edit) = ctx.ui.create("Edit", Some(me)) else { return };
        let width = (self.base.bounds().width - 2.0 * PAD).clamp(60.0, 360.0);
        if let Some(cb) = ctx.ui.control_mut(edit) {
            cb.set_rect(PAD, 3.0, width, BUTTON);
        }
        let _ = ctx.ui.set_prop(edit, "placeholder", "https://".to_string());
        self.link_editor.set(Some(edit));
        let finish = move |ui: &mut Ui, save: bool| {
            if !ui.contains(edit) {
                return;
            }
            let url = match ui.get_prop(edit, "text") {
                Ok(PropValue::String(s)) => s.trim().to_string(),
                _ => String::new(),
            };
            ui.free(edit);
            if save && !url.is_empty() {
                let html = ui.get_mut::<RichEditor>(me).and_then(|r| r.set_link(&url).then(|| r.html.clone()));
                if let Some(html) = html {
                    ui.fire(me, "on_change", &EventArgs::Value(html.into()));
                }
            }
            ui.set_focus(me);
            ui.request_repaint();
        };
        ctx.ui.on(edit, "on_key_down", move |ui, _, args| {
            if let EventArgs::Key(k) = args {
                match k.key {
                    Key::Enter => finish(ui, true),
                    Key::Escape => finish(ui, false),
                    _ => {}
                }
            }
        });
        ctx.ui.on(edit, "on_exit", move |ui, _, _| {
            let post = ui.poster();
            post.post(move |ui| finish(ui, true));
        });
        ctx.request_focus(edit);
        ctx.ui.invalidate();
        ctx.repaint();
    }

    fn copy(&self, ctx: &mut EventCtx<'_>) {
        let (a, b) = self.selection();
        if a < b {
            let text = self.doc.text(a, b);
            aylis_core::clipboard::set_text(ctx.ui, &text);
        }
    }

    fn tool_at(&self, p: PointF) -> Option<usize> {
        let (tools, _) = self.toolbar_layout(self.local_rect().width);
        tools.iter().position(|(name, r)| *name != "|" && r.contains(p))
    }
}

impl Component for RichEditor {
    leaf_control!("RichEditor");

    // Sem estilo próprio no tema, fica com o do Memo (o fundo, a borda e o foco de campo).
    fn class_chain(&self, out: &mut Vec<&'static str>) {
        out.push("RichEditor");
        out.push("Memo");
    }

    fn accepts_text(&self) -> bool {
        true
    }

    fn captures_key(&self, event: &KeyEvent) -> bool {
        !event.modifiers.ctrl
            && matches!(event.key, Key::Enter | Key::Up | Key::Down | Key::Left | Key::Right | Key::Home | Key::End | Key::Backspace | Key::Delete)
    }

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        html: String => "Data",
        toolbar: String => "Appearance",
        placeholder: String => "Appearance",
        read_only: bool => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let shift = event.modifiers.shift;
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        let mut changed = false;
        let keep_goal = matches!(event.key, Key::Up | Key::Down);
        match event.key {
            Key::Left => {
                let (a, _) = self.selection();
                let to = if self.caret != self.anchor && !shift {
                    a
                } else if ctrl {
                    self.prev_word(self.caret)
                } else {
                    self.caret.saturating_sub(1)
                };
                self.move_to(to, shift);
            }
            Key::Right => {
                let (_, b) = self.selection();
                let to = if self.caret != self.anchor && !shift {
                    b
                } else if ctrl {
                    self.next_word(self.caret)
                } else {
                    self.caret + 1
                };
                self.move_to(to, shift);
            }
            Key::Up => self.vertical(ctx.ui, -1, shift),
            Key::Down => self.vertical(ctx.ui, 1, shift),
            Key::Home | Key::End if ctrl => {
                let to = if event.key == Key::Home { 0 } else { self.doc.len() };
                self.move_to(to, shift);
            }
            Key::Home | Key::End => self.home_end(ctx.ui, event.key == Key::End, shift),
            Key::Enter => changed = self.new_paragraph(),
            Key::Backspace => changed = self.backspace(ctrl),
            Key::Delete => changed = self.delete_forward(ctrl),
            Key::Char('a') if ctrl => self.select_all(),
            Key::Char('c') if ctrl => self.copy(ctx),
            Key::Char('x') if ctrl => {
                self.copy(ctx);
                changed = self.delete_selection();
            }
            Key::Char('v') if ctrl => {
                let pasted = aylis_core::clipboard::text(ctx.ui);
                if let Some(text) = pasted {
                    self.merging = false;
                    changed = self.insert(&text);
                    self.merging = false;
                }
            }
            Key::Char('b') if ctrl => changed = self.command("bold"),
            Key::Char('i') if ctrl => changed = self.command("italic"),
            Key::Char('u') if ctrl => changed = self.command("underline"),
            Key::Char('z') if ctrl && shift => changed = self.command("redo"),
            Key::Char('z') if ctrl => changed = self.command("undo"),
            Key::Char('y') if ctrl => changed = self.command("redo"),
            _ => return,
        }
        if !keep_goal {
            self.goal_x = None;
        }
        ctx.set_handled();
        if changed {
            self.changed(ctx);
        }
        self.blink_start = ctx.ui.time();
        ctx.repaint();
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.goal_x = None;
        if self.insert(text) {
            self.changed(ctx);
        }
        ctx.set_handled();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if let Some(i) = self.tool_at(event.position) {
            let (tools, _) = self.toolbar_layout(self.local_rect().width);
            let name = tools[i].0.to_string();
            if self.is_enabled(&name) {
                self.press(ctx, &name);
            }
            ctx.set_handled();
            return;
        }
        let (_, bar) = self.toolbar_layout(self.local_rect().width);
        if event.position.y < bar {
            return;
        }
        let index = self.index_at(ctx.ui, event.position);
        self.goal_x = None;
        match event.click_count {
            2 => {
                let (a, b) = self.word_bounds(index);
                self.select(a, b);
            }
            n if n >= 3 => {
                let (a, b) = self.doc.line_range(index);
                self.select(a, b);
            }
            _ => {
                self.move_to(index, event.modifiers.shift);
                self.dragging = true;
            }
        }
        self.blink_start = ctx.ui.time();
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging {
            let index = self.index_at(ctx.ui, event.position);
            if index != self.caret {
                self.move_to(index, true);
                ctx.repaint();
            }
            return;
        }
        let hover = self.tool_at(event.position);
        if hover != self.hover.replace(hover) {
            ctx.repaint();
        }
        let (_, bar) = self.toolbar_layout(self.local_rect().width);
        self.base.set_cursor(if event.position.y < bar { if hover.is_some() { CursorShape::Pointer } else { CursorShape::Default } } else { CursorShape::Text });
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = false;
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if self.hover.replace(None).is_some() {
            ctx.repaint();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let (total, view) = self.extent.get();
        let max = (total - view).max(0.0);
        let dy = if event.touch { event.delta.y } else { event.delta.y.signum() * 60.0 };
        let s = self.scroll.get();
        let next = (s - dy).clamp(0.0, max);
        if (next - s).abs() < 0.5 {
            return;
        }
        self.scroll.set(next);
        ctx.set_handled();
        ctx.repaint();
    }

    fn focus_enter(&mut self, ctx: &mut EventCtx<'_>) {
        self.blink_start = ctx.ui.time();
        ctx.repaint();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.dragging = false;
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 && !s.border_color.is_transparent() {
            canvas.stroke_rect(r, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let accent = ctx.colors.accent.unwrap_or_else(|| ctx.theme.color("primary"));
        let muted = ctx.theme.color("text_muted");
        let mut base = ctx.text_style();
        base.trimming = Trimming::None;
        base.v_align = TextAlign::Center;
        *self.text_style.borrow_mut() = base.clone();

        // A barra.
        let (tools, bar) = self.toolbar_layout(r.width);
        if bar > 0.0 {
            canvas.save();
            canvas.clip_rect(r);
            canvas.fill_rect(RectF::new(r.x, r.y, r.width, bar), 0.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
            canvas.restore();
            let line = if s.border_color.is_transparent() { muted.with_opacity(0.3) } else { s.border_color };
            canvas.draw_line(PointF::new(r.x, r.y + bar), PointF::new(r.right(), r.y + bar), &Stroke::new(line, 1.0));
        }
        for (i, (name, rect)) in tools.iter().enumerate() {
            if *name == "|" {
                let x = rect.x + rect.width / 2.0;
                canvas.draw_line(PointF::new(x, rect.y + 6.0), PointF::new(x, rect.bottom() - 6.0), &Stroke::new(muted.with_opacity(0.4), 1.0));
                continue;
            }
            let enabled = self.is_enabled(name);
            let active = self.is_active(name);
            if active {
                canvas.fill_rect(*rect, 4.0, &Brush::Solid(accent.with_opacity(0.16)));
            } else if enabled && self.hover.get() == Some(i) {
                canvas.fill_rect(*rect, 4.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
            }
            let color = if !enabled { s.text.with_opacity(0.35) } else if active { accent } else { s.text };
            paint_tool(canvas, ctx, name, *rect, color, &base);
        }

        // O texto.
        let content = self.content_rect(r);
        let layout = self.layout_with(content.width, &base, &mut |t, st| canvas.measure_text(t, st).width);
        let focused = ctx.state.focused;
        let current = visual_line_of(&layout, self.caret);
        let cl = &layout.lines[current];
        let mut scroll = self.scroll.get();
        let now = (self.caret, self.version);
        if focused && self.followed.replace(now) != now {
            if cl.y < scroll {
                scroll = cl.y;
            } else if cl.y + cl.height > scroll + content.height {
                scroll = cl.y + cl.height - content.height;
            }
        }
        scroll = scroll.clamp(0.0, (layout.height - content.height).max(0.0));
        self.scroll.set(scroll);
        self.extent.set((layout.height, content.height));

        canvas.save();
        canvas.clip_rect(RectF::new(r.x, content.y - CONTENT_PAD / 2.0, r.width, content.height + CONTENT_PAD));
        if self.doc.len() == 0 && !self.placeholder.is_empty() && !focused {
            let mut hint = char_style(&base, self.doc.blocks[0], &Format::default());
            hint.color = muted;
            canvas.draw_text(&self.placeholder, RectF::new(content.x, content.y, content.width, cl.height), &hint);
        }
        let (sa, sb) = self.selection();
        let selection = ctx.theme.color("selection");
        let selection = if focused { selection } else { selection.with_opacity(0.6) };
        for l in &layout.lines {
            let y = content.y + l.y - scroll;
            if y + l.height < content.y - CONTENT_PAD || y > content.bottom() + CONTENT_PAD {
                continue;
            }
            let block = self.doc.blocks[l.line];
            // A seleção (a quebra de parágrafo selecionada vira um pedacinho depois do fim).
            if sa < sb && sb > l.start && sa <= l.end {
                let x1 = Self::caret_x(l, sa.max(l.start));
                let mut x2 = Self::caret_x(l, sb.min(l.end));
                if sb > l.end && l.end < self.doc.len() && self.doc.chars[l.end] == '\n' {
                    x2 += 6.0;
                }
                if x2 > x1 {
                    canvas.fill_rect(RectF::new(content.x + x1, y, x2 - x1, l.height), 0.0, &Brush::Solid(selection));
                }
            }
            if let Some(marker) = &l.marker {
                let mut ms = char_style(&base, block, &Format::default());
                ms.h_align = TextAlign::Trailing;
                let (top, h) = run_band(l.baseline, ms.font.size);
                canvas.draw_text(marker, RectF::new(content.x, y + top, LIST_INDENT - 8.0, h), &ms);
            }
            // Justificado, cada palavra sai no lugar dela: o trecho também para depois de
            // cada espaço.
            let justify = block.align == TextAlign::Justify;
            let mut i = l.start;
            while i < l.end {
                let f = &self.doc.fmts[i];
                let mut j = i + 1;
                while j < l.end && self.doc.fmts[j] == *f && !(justify && self.doc.chars[j - 1] == ' ') {
                    j += 1;
                }
                let mut style = char_style(&base, block, f);
                if f.link.is_some() {
                    style.color = accent;
                }
                let x1 = Self::caret_x(l, i);
                let x2 = Self::caret_x(l, j);
                let (top, h) = run_band(l.baseline, style.font.size);
                canvas.draw_text(&self.doc.text(i, j), RectF::new(content.x + x1, y + top, x2 - x1 + 4.0, h), &style);
                i = j;
            }
        }
        if focused && !ctx.state.disabled && !self.read_only {
            let blink = 1.0;
            let phase = (ctx.time - self.blink_start).rem_euclid(blink);
            let on = phase < blink / 2.0;
            if on {
                // O cursor tem a altura da letra que vai ser digitada ali.
                let x = content.x + Self::caret_x(cl, self.caret);
                let y = content.y + cl.y - scroll;
                let size = char_size(&base, self.current_block(), &self.typing.clone().unwrap_or_else(|| self.format_at_caret()));
                let (top, h) = run_band(cl.baseline, size);
                canvas.draw_line(PointF::new(x, y + top + 2.0), PointF::new(x, y + top + h - 2.0), &Stroke::new(s.text, 1.5));
            }
            ctx.request_frame_at(ctx.time + if on { blink / 2.0 - phase } else { blink - phase });
        }
        // A barrinha de rolagem, quando o texto não cabe.
        if layout.height > content.height {
            let track = content.height;
            let thumb = (track * content.height / layout.height).max(16.0);
            let y = content.y + (track - thumb) * scroll / (layout.height - content.height);
            canvas.fill_rect(RectF::new(r.right() - 6.0, y, 3.0, thumb), 1.5, &Brush::Solid(muted.with_opacity(0.45)));
        }
        canvas.restore();
    }
}

/// O desenho de um botão da barra.
fn paint_tool(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, name: &str, rect: RectF, color: Color, base: &TextStyle) {
    let glyph = |canvas: &mut dyn Canvas, text: &str, size: f32, f: &dyn Fn(&mut TextStyle)| {
        let mut style = base.clone();
        style.font.size = size;
        style.color = color;
        style.h_align = TextAlign::Center;
        style.v_align = TextAlign::Center;
        f(&mut style);
        canvas.draw_text(text, rect, &style);
    };
    let icon = RectF::new(rect.x + (rect.width - 18.0) / 2.0, rect.y + (rect.height - 18.0) / 2.0, 18.0, 18.0);
    let stroke = Stroke::new(color, 1.6).round();
    let c = PointF::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
    match name {
        "bold" => glyph(canvas, "B", 15.0, &|s| s.font.weight = FontWeight::Bold),
        "italic" => glyph(canvas, "I", 15.0, &|s| s.font.italic = true),
        "underline" => glyph(canvas, "U", 15.0, &|s| s.font.underline = true),
        "strike" => glyph(canvas, "S", 15.0, &|s| s.font.strikeout = true),
        "h1" | "h2" | "h3" => glyph(canvas, &name.to_uppercase(), 13.0, &|s| s.font.weight = FontWeight::Bold),
        "size" => {
            // "Aa": o A grande e o a pequeno, encostados na mesma base.
            let mut style = base.clone();
            style.color = color;
            style.h_align = TextAlign::Center;
            style.v_align = TextAlign::Center;
            style.font.size = 16.0;
            canvas.draw_text("A", RectF::new(c.x - 10.0, rect.y, 12.0, rect.height), &style);
            style.font.size = 11.0;
            canvas.draw_text("a", RectF::new(c.x + 1.0, rect.y + 2.5, 9.0, rect.height), &style);
        }
        _ if size_name(name).is_some() => glyph(canvas, &name[5..], 13.0, &|_| {}),
        "p" => glyph(canvas, "¶", 15.0, &|_| {}),
        "removeFormat" => {
            glyph(canvas, "T", 15.0, &|_| {});
            canvas.draw_line(PointF::new(c.x - 7.0, c.y + 7.0), PointF::new(c.x + 7.0, c.y - 7.0), &stroke);
        }
        "unordered" | "ordered" => {
            for k in 0..3 {
                let y = c.y - 6.0 + k as f32 * 6.0;
                canvas.draw_line(PointF::new(c.x - 2.0, y), PointF::new(c.x + 8.0, y), &stroke);
                if name == "unordered" {
                    canvas.fill_ellipse(RectF::new(c.x - 8.5, y - 1.5, 3.0, 3.0), &Brush::Solid(color));
                }
            }
            if name == "ordered" {
                let mut style = base.clone();
                style.font.size = 8.0;
                style.color = color;
                style.h_align = TextAlign::Center;
                style.v_align = TextAlign::Center;
                for k in 0..3 {
                    canvas.draw_text(&(k + 1).to_string(), RectF::new(c.x - 11.0, c.y - 10.0 + k as f32 * 6.0, 6.0, 8.0), &style);
                }
            }
        }
        "left" | "center" | "right" | "justify" => {
            for (k, w) in [14.0f32, 9.0, 14.0, 9.0].into_iter().enumerate() {
                let y = c.y - 6.0 + k as f32 * 4.0;
                let w = if name == "justify" { 14.0 } else { w };
                let x = match name {
                    "left" | "justify" => c.x - 7.0,
                    "center" => c.x - w / 2.0,
                    _ => c.x + 7.0 - w,
                };
                canvas.draw_line(PointF::new(x, y), PointF::new(x + w, y), &stroke);
            }
        }
        _ => ctx.draw_icon(canvas, name, icon, color),
    }
}
