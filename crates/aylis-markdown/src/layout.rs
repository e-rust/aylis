//! Blocos → o que desenhar, com quebra de linha por palavra (o texto é medido com a fonte
//! de verdade do renderizador).

use aylis_core::text::Trimming;
use aylis_core::{Color, FontSpec, FontWeight, PointF, RectF, SizeF, TextAlign, TextStyle, Theme};

use crate::parse::{Block, Inline};

/// Cores e fontes (do tema).
#[derive(Clone, Debug)]
pub struct Palette {
    pub body: FontSpec,
    pub mono: FontSpec,
    pub text: Color,
    pub muted: Color,
    pub link: Color,
    pub code_bg: Color,
    pub border: Color,
    /// Tema escuro (as cores do realce do código).
    pub dark: bool,
    /// Os blocos de código como no editor de código ([`EditorCode`]); `None`: como numa página
    /// de documentação (as cores do tema, o botão Copiar, as abas por linguagem).
    pub editor: Option<EditorCode>,
}

/// Os blocos de código com a cara do editor de código (a dica do servidor de linguagem na
/// IDE, como no VS Code): as cores do tema de cores dele, a gramática dele, a linha longa
/// quebrando na largura em vez de terminar em "…", sem o botão Copiar e sem as abas.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EditorCode {
    /// O tema de cores do editor (o `color_theme` do `CodeEditor`; vazio: o do app).
    pub color_theme: String,
    /// A linguagem dos blocos sem linguagem (os exemplos da documentação do Rust vêm sem, ou
    /// só com `ignore`, `no_run`...).
    pub language: String,
}

impl EditorCode {
    /// A linguagem de um bloco: a da cerca ou, sem ela (ou só com os atributos dos exemplos do
    /// Rust), a do editor.
    pub fn language_of<'a>(&'a self, fence: &'a str) -> &'a str {
        let first = fence.split([',', ' ']).next().unwrap_or_default().trim();
        let attribute = matches!(first, "ignore" | "no_run" | "should_panic" | "compile_fail") || first.starts_with("edition");
        if first.is_empty() || attribute { &self.language } else { first }
    }
}

impl Palette {
    /// `size` > 0 troca o tamanho do corpo.
    pub fn from_theme(theme: &Theme, size: f32) -> Palette {
        let mut body = theme.fonts.get("default").cloned().unwrap_or_default();
        if size > 0.0 {
            body.size = size;
        }
        let mut mono = theme.fonts.get("mono").cloned().unwrap_or_else(|| FontSpec::new("monospace", 13.0));
        mono.size = body.size * 0.92;
        Palette {
            body,
            mono,
            text: theme.color("text"),
            muted: theme.color("text_muted"),
            link: theme.color("primary"),
            code_bg: theme.color("surface_2"),
            border: theme.color("border"),
            dark: theme.dark,
            editor: None,
        }
    }
}

/// Uma coisa a desenhar (coordenadas do conteúdo: 0 é o topo do texto).
#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    /// Um trecho de texto numa linha. `sep`: o que vem antes dele ao copiar um trecho que
    /// começa antes (nada no meio da linha, espaço na quebra, tabulação entre células...).
    Text { rect: RectF, text: String, style: TextStyle, strike: bool, underline: bool, background: Option<Color>, link: Option<String>, sep: Sep },
    Fill { rect: RectF, color: Color, radius: f32 },
    Line { from: PointF, to: PointF, color: Color, width: f32 },
    Check { rect: RectF, checked: bool, color: Color },
    Image { rect: RectF, url: String },
    /// Uma aba de um grupo de blocos de código (`lang`: a linguagem, a do clique).
    Tab { rect: RectF, label: String, lang: String, active: bool },
    /// O botão Copiar de um bloco de código (`text`: o código inteiro).
    Copy { rect: RectF, text: String },
}

/// O que vem antes de um trecho de texto quando a cópia começa antes dele (em ordem: o mais
/// forte de vários vale, como uma quebra de parágrafo que também é quebra de linha).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sep {
    /// Continua a mesma linha.
    #[default]
    None,
    /// A linha quebrou no meio do parágrafo.
    Space,
    /// A próxima célula da tabela.
    Tab,
    /// `n` quebras de linha (2: um parágrafo novo).
    Lines(u8),
}

impl Sep {
    pub fn as_str(self) -> &'static str {
        match self {
            Sep::None => "",
            Sep::Space => " ",
            Sep::Tab => "\t",
            Sep::Lines(1) => "\n",
            Sep::Lines(2) => "\n\n",
            Sep::Lines(_) => "\n\n\n",
        }
    }
}

/// Um ponto do texto: o trecho (índice em [`Layout::items`]) e o caractere dentro dele.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextPos {
    pub item: usize,
    pub ch: usize,
}

impl Item {
    pub fn rect(&self) -> RectF {
        match self {
            Item::Text { rect, .. } | Item::Fill { rect, .. } | Item::Check { rect, .. } | Item::Image { rect, .. } | Item::Tab { rect, .. } | Item::Copy { rect, .. } => *rect,
            Item::Line { from, to, .. } => RectF::new(from.x.min(to.x), from.y.min(to.y), (to.x - from.x).abs().max(1.0), (to.y - from.y).abs().max(1.0)),
        }
    }
}

/// O texto montado para uma largura.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    pub items: Vec<Item>,
    pub height: f32,
    /// Os títulos, na ordem: o nível, o texto, o identificador (`slug`, o dos links `#...`) e
    /// onde começam (o "Nesta página" de uma documentação e a rolagem até um deles).
    pub headings: Vec<Heading>,
}

/// Um título da página ([`Layout::headings`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub slug: String,
    pub y: f32,
}

/// O nome de uma linguagem na aba de código (`rust` vira "Rust", `csharp` vira "C#").
pub fn language_label(lang: &str) -> String {
    match lang.to_ascii_lowercase().as_str() {
        "rust" | "rs" => "Rust",
        "python" | "py" => "Python",
        "typescript" | "ts" => "TypeScript",
        "javascript" | "js" => "JavaScript",
        "csharp" | "cs" | "c#" => "C#",
        "java" => "Java",
        "go" => "Go",
        "cpp" | "c++" | "cxx" => "C++",
        "pascal" | "fpc" | "delphi" | "objectpascal" => "Free Pascal",
        "json" => "JSON",
        "toml" => "TOML",
        "bash" | "sh" | "shell" | "powershell" | "text" => "Terminal",
        _ => return lang.to_string(),
    }
    .to_string()
}

/// O identificador de um título, como nos sites de documentação: minúsculas, os espaços viram
/// `-` e a pontuação sai (`## Gravar no banco` vira `gravar-no-banco`).
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    for c in text.trim().chars().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() || c == '_' || c == '-' {
            out.push(c);
        } else if c.is_whitespace() {
            out.push('-');
        }
    }
    out
}

impl Layout {
    /// Monta os blocos na largura `width`. `measure` dá a largura de um texto numa fonte;
    /// `images` o tamanho de uma imagem do projeto.
    pub fn build(
        blocks: &[Block],
        width: f32,
        palette: &Palette,
        measure: &mut dyn FnMut(&str, &TextStyle) -> f32,
        images: &mut dyn FnMut(&str) -> Option<SizeF>,
    ) -> Layout {
        Self::build_in(blocks, width, palette, "", measure, images)
    }

    /// Como [`Layout::build`], com a linguagem escolhida nas abas de código (`lang`: a aba
    /// dela fica à mostra em cada grupo; sem ela no grupo, a primeira).
    pub fn build_in(
        blocks: &[Block],
        width: f32,
        palette: &Palette,
        lang: &str,
        measure: &mut dyn FnMut(&str, &TextStyle) -> f32,
        images: &mut dyn FnMut(&str) -> Option<SizeF>,
    ) -> Layout {
        let mut b = Builder { items: Vec::new(), next_sep: Sep::None, lang, headings: Vec::new(), pal: palette, measure, images };
        let ctx = Ctx { color: palette.text, scale: 1.0, align: TextAlign::Leading, bold: false };
        let y = b.blocks(blocks, 0.0, width, 0.0, ctx);
        Layout { items: b.items, height: y.max(0.0), headings: b.headings }
    }

    /// A aba de código no ponto (coordenadas do conteúdo): a linguagem dela.
    pub fn tab_at(&self, p: PointF) -> Option<&str> {
        self.items.iter().find_map(|i| match i {
            Item::Tab { rect, lang, .. } if rect.contains(p) => Some(lang.as_str()),
            _ => None,
        })
    }

    /// O link no ponto (coordenadas do conteúdo).
    pub fn link_at(&self, p: PointF) -> Option<&str> {
        self.items.iter().find_map(|i| match i {
            Item::Text { rect, link: Some(l), .. } if rect.contains(p) => Some(l.as_str()),
            _ => None,
        })
    }

    /// O botão Copiar no ponto: o índice dele e o código.
    pub fn copy_at(&self, p: PointF) -> Option<(usize, &str)> {
        self.items.iter().enumerate().find_map(|(n, i)| match i {
            Item::Copy { rect, text } if rect.contains(p) => Some((n, text.as_str())),
            _ => None,
        })
    }

    /// O trecho de texto mais perto do ponto, para a seleção: o da linha do ponto (o que
    /// contém o x, ou o mais perto dele); entre linhas, o primeiro abaixo. Com `chars`, o
    /// caractere: `chars(item, x)` diz quantos caracteres do trecho ficam antes de `x`.
    pub fn text_pos_at(&self, p: PointF, chars: &mut dyn FnMut(usize, f32) -> usize) -> Option<TextPos> {
        let mut best: Option<(f32, usize)> = None;
        let mut below: Option<(f32, usize)> = None;
        let mut last = None;
        for (n, item) in self.items.iter().enumerate() {
            let Item::Text { rect, .. } = item else { continue };
            last = Some(n);
            if p.y >= rect.y && p.y < rect.bottom() {
                let d = if p.x < rect.x { rect.x - p.x } else if p.x > rect.right() { p.x - rect.right() } else { 0.0 };
                if best.is_none_or(|(bd, _)| d < bd) {
                    best = Some((d, n));
                }
            } else if rect.y >= p.y && below.is_none_or(|(by, bn)| rect.y < by || (rect.y == by && n < bn)) {
                below = Some((rect.y, n));
            }
        }
        if let Some((_, n)) = best {
            return Some(TextPos { item: n, ch: chars(n, p.x) });
        }
        if let Some((_, n)) = below {
            return Some(TextPos { item: n, ch: 0 });
        }
        // Abaixo de tudo: o fim do último trecho.
        let n = last?;
        let Item::Text { text, .. } = &self.items[n] else { return None };
        Some(TextPos { item: n, ch: text.chars().count() })
    }

    /// O texto entre dois pontos (em qualquer ordem), com as quebras de linha, as tabulações
    /// entre células e os espaços das linhas quebradas.
    pub fn text_between(&self, a: TextPos, b: TextPos) -> String {
        let (from, to) = if a <= b { (a, b) } else { (b, a) };
        let mut out = String::new();
        for n in from.item..=to.item.min(self.items.len().saturating_sub(1)) {
            let Some(Item::Text { text, sep, .. }) = self.items.get(n) else { continue };
            let start = if n == from.item { from.ch } else { 0 };
            let end = if n == to.item { to.ch } else { usize::MAX };
            if n != from.item {
                out.push_str(sep.as_str());
            }
            out.extend(text.chars().skip(start).take(end.saturating_sub(start)));
        }
        out
    }

    /// O começo e o fim da palavra (`paragraph`: do parágrafo, ou da linha de código) no ponto.
    pub fn word_at(&self, pos: TextPos, paragraph: bool) -> (TextPos, TextPos) {
        let Some(Item::Text { text, .. }) = self.items.get(pos.item) else { return (pos, pos) };
        if paragraph {
            // Para trás até um separador de linha ou mais forte (o começo do parágrafo); para
            // frente até o próximo.
            let texts: Vec<usize> = self.items.iter().enumerate().filter(|(_, i)| matches!(i, Item::Text { .. })).map(|(n, _)| n).collect();
            let at = texts.iter().position(|&n| n == pos.item).unwrap_or(0);
            let breaks = |n: usize| matches!(&self.items[n], Item::Text { sep, .. } if *sep >= Sep::Tab);
            let mut first = at;
            while first > 0 && !breaks(texts[first]) {
                first -= 1;
            }
            let mut last = at;
            while last + 1 < texts.len() && !breaks(texts[last + 1]) {
                last += 1;
            }
            let end = match &self.items[texts[last]] {
                Item::Text { text, .. } => text.chars().count(),
                _ => 0,
            };
            return (TextPos { item: texts[first], ch: 0 }, TextPos { item: texts[last], ch: end });
        }
        let chars: Vec<char> = text.chars().collect();
        let word = |c: char| c.is_alphanumeric() || c == '_';
        let at = pos.ch.min(chars.len());
        let kind = chars.get(at).or(at.checked_sub(1).and_then(|i| chars.get(i))).map(|&c| word(c));
        let same = |c: char| Some(word(c)) == kind && !c.is_whitespace();
        let mut start = at;
        while start > 0 && same(chars[start - 1]) {
            start -= 1;
        }
        let mut end = at;
        while end < chars.len() && same(chars[end]) {
            end += 1;
        }
        (TextPos { item: pos.item, ch: start }, TextPos { item: pos.item, ch: end })
    }

    /// O primeiro e o último ponto do texto (o Ctrl+A).
    pub fn text_bounds(&self) -> Option<(TextPos, TextPos)> {
        let mut texts = self.items.iter().enumerate().filter_map(|(n, i)| match i {
            Item::Text { text, .. } => Some((n, text.chars().count())),
            _ => None,
        });
        let (first, _) = texts.next()?;
        let (last, len) = texts.last().unwrap_or((first, match &self.items[first] {
            Item::Text { text, .. } => text.chars().count(),
            _ => 0,
        }));
        Some((TextPos { item: first, ch: 0 }, TextPos { item: last, ch: len }))
    }
}

#[derive(Clone, Copy)]
struct Ctx {
    color: Color,
    scale: f32,
    align: TextAlign,
    bold: bool,
}

struct Builder<'a> {
    items: Vec<Item>,
    /// O separador do próximo trecho de texto (o mais forte pedido até ele).
    next_sep: Sep,
    /// A linguagem escolhida nas abas de código (a aba dela fica à mostra).
    lang: &'a str,
    headings: Vec<Heading>,
    pal: &'a Palette,
    measure: &'a mut dyn FnMut(&str, &TextStyle) -> f32,
    images: &'a mut dyn FnMut(&str) -> Option<SizeF>,
}

const HEADINGS: [f32; 6] = [1.8, 1.5, 1.25, 1.1, 1.0, 0.9];

impl Builder<'_> {
    fn gap(&self) -> f32 {
        self.pal.body.size * 0.75
    }

    /// Pede o separador `sep` antes do próximo trecho de texto (vale o mais forte).
    fn sep(&mut self, sep: Sep) {
        self.next_sep = self.next_sep.max(sep);
    }

    /// O separador pedido, para o trecho que entra agora.
    fn take_sep(&mut self) -> Sep {
        std::mem::take(&mut self.next_sep)
    }

    fn blocks(&mut self, blocks: &[Block], x: f32, width: f32, mut y: f32, ctx: Ctx) -> f32 {
        let mut i = 0;
        while i < blocks.len() {
            if i > 0 {
                y += self.gap();
                self.sep(Sep::Lines(2));
            }
            // Blocos de código seguidos, cada um numa linguagem: um grupo de abas (o mesmo
            // exemplo em Rust, Python, C#...), como nos sites de documentação.
            let mut run = Vec::new();
            for block in &blocks[i..] {
                match block {
                    Block::Code { lang, text } if !lang.is_empty() && !run.iter().any(|(l, _): &(&str, &str)| l.eq_ignore_ascii_case(lang)) => run.push((lang.as_str(), text.as_str())),
                    _ => break,
                }
            }
            // (como no editor, cada bloco aparece: a dica junta o caminho e a assinatura)
            if run.len() >= 2 && self.pal.editor.is_none() {
                y = self.code_tabs(&run, x, width, y, ctx);
                i += run.len();
                continue;
            }
            y = self.block(&blocks[i], x, width, y, ctx);
            i += 1;
        }
        y
    }

    /// Um grupo de abas de código: a faixa com o nome de cada linguagem e, embaixo, o código
    /// da escolhida (a de `self.lang`, ou a primeira).
    fn code_tabs(&mut self, run: &[(&str, &str)], x: f32, width: f32, y: f32, ctx: Ctx) -> f32 {
        let active = run.iter().position(|(l, _)| l.eq_ignore_ascii_case(self.lang)).unwrap_or(0);
        let mut font = self.pal.body.clone();
        font.size *= 0.92;
        let style = TextStyle { font, ..Default::default() };
        let (pad, strip) = (12.0, self.pal.body.size * 2.1);
        let mut cx = x;
        for (n, (lang, _)) in run.iter().enumerate() {
            let label = language_label(lang);
            let w = (self.measure)(&label, &style) + 2.0 * pad;
            self.items.push(Item::Tab { rect: RectF::new(cx, y, w, strip), label, lang: lang.to_string(), active: n == active });
            cx += w;
        }
        self.items.push(Item::Line { from: PointF::new(x, y + strip), to: PointF::new(x + width, y + strip), color: self.pal.border, width: 1.0 });
        let code = Block::Code { lang: run[active].0.to_string(), text: run[active].1.to_string() };
        self.block(&code, x, width, y + strip + 4.0, ctx)
    }

    /// Um bloco de código como no editor ([`EditorCode`]): o realce e as cores dele, e a linha
    /// longa continua embaixo (quebra depois de um espaço, vírgula ou parêntese, se houver um
    /// na linha; senão, onde acabar a largura).
    fn editor_code(&mut self, code: &str, lang: &str, x: f32, width: f32, y: f32, ctx: Ctx) -> f32 {
        let Some(editor) = self.pal.editor.clone() else { return y };
        let lang = editor.language_of(lang).to_string();
        let theme = aylis_controls_base::syntax::code_theme(&editor.color_theme);
        let dark = theme.map_or(self.pal.dark, |t| t.dark);
        let plain = theme.map_or(ctx.color, |t| t.foreground());
        let (pad, line_h) = (8.0, self.pal.mono.size * 1.45);
        let expanded = code.trim_end_matches('\n').replace('\t', "    ");
        let spans = aylis_controls_base::syntax::highlight_like_editor(&lang, &expanded);
        // O fundo entra antes do texto; a altura sai depois (com as linhas quebradas).
        let fill = self.items.len();
        self.items.push(Item::Fill { rect: RectF::new(x, y, width, 0.0), color: theme.map_or(self.pal.code_bg, |t| t.background()), radius: 4.0 });
        let right = x + width - pad;
        let mut row = 0usize;
        let mut previous: Option<usize> = None;
        for (i, line) in expanded.split('\n').enumerate() {
            let chars: Vec<char> = line.chars().collect();
            if chars.is_empty() {
                row += 1;
                continue;
            }
            if let Some(p) = previous {
                self.sep(Sep::Lines((i - p).min(3) as u8));
            }
            previous = Some(i);
            let mut sep = self.take_sep();
            // Um trecho por cor; o que não tem trecho fica na cor do texto.
            let mut runs: Vec<(usize, usize, Color)> = Vec::new();
            let mut at = 0;
            for s in spans.get(i).map(Vec::as_slice).unwrap_or(&[]) {
                let (start, end) = (s.start.min(chars.len()), s.end.min(chars.len()));
                if start > at {
                    runs.push((at, start, plain));
                }
                if end > start.max(at) {
                    let color = aylis_controls_base::syntax::span_color(theme, s, dark, plain);
                    match runs.last_mut() {
                        Some(last) if last.2 == color && last.1 == start.max(at) => last.1 = end,
                        _ => runs.push((start.max(at), end, color)),
                    }
                    at = end;
                }
            }
            if at < chars.len() {
                runs.push((at, chars.len(), plain));
            }
            let mut cx = x + pad;
            for (start, end, color) in runs {
                let style = TextStyle { font: self.pal.mono.clone(), color, trimming: Trimming::None, ..Default::default() };
                let mut from = start;
                while from < end {
                    let piece: String = chars[from..end].iter().collect();
                    let w = (self.measure)(&piece, &style);
                    let fits = if cx + w <= right + 0.5 {
                        end - from
                    } else {
                        // Quantos cabem no resto da linha (busca binária: as larguras crescem).
                        let (mut lo, mut hi) = (0usize, end - from);
                        while lo < hi {
                            let mid = (lo + hi).div_ceil(2);
                            let part: String = chars[from..from + mid].iter().collect();
                            if cx + (self.measure)(&part, &style) <= right { lo = mid } else { hi = mid - 1 }
                        }
                        // Quebra depois do último espaço, vírgula ou parêntese que coube.
                        let soft = (1..=lo).rev().find(|&n| matches!(chars[from + n - 1], ' ' | ',' | '(' | '<'));
                        match soft {
                            Some(n) => n,
                            // Nada coube no começo da linha: um caractere por vez, ao menos.
                            None if cx <= x + pad + 0.5 => lo.max(1),
                            // A palavra (o trecho) não cabe no resto: vai inteira para baixo.
                            None => 0,
                        }
                    };
                    if fits > 0 {
                        let text: String = chars[from..from + fits].iter().collect();
                        let w = (self.measure)(&text, &style);
                        let rect = RectF::new(cx, y + pad + row as f32 * line_h, w, line_h);
                        self.items.push(Item::Text { rect, text, style: style.clone(), strike: false, underline: false, background: None, link: None, sep: std::mem::take(&mut sep) });
                        cx += w;
                        from += fits;
                    }
                    if from < end {
                        // A continuação da mesma linha, embaixo (a cópia não ganha quebra).
                        row += 1;
                        cx = x + pad;
                    }
                }
            }
            row += 1;
        }
        let h = row as f32 * line_h + 2.0 * pad;
        if let Some(Item::Fill { rect, .. }) = self.items.get_mut(fill) {
            rect.height = h;
        }
        y + h
    }

    fn block(&mut self, block: &Block, x: f32, width: f32, y: f32, ctx: Ctx) -> f32 {
        let body = self.pal.body.size;
        match block {
            Block::Paragraph(inl) => self.inlines(inl, x, width, y, ctx),
            Block::Heading(level, inl) => {
                let text: String = inl.iter().map(|i| if let Inline::Text { text, .. } = i { text.as_str() } else { "" }).collect();
                self.headings.push(Heading { level: *level, slug: slug(&text), text, y });
                let scale = HEADINGS[(*level as usize).clamp(1, 6) - 1];
                let color = if *level >= 6 { self.pal.muted } else { ctx.color };
                let bottom = self.inlines(inl, x, width, y + if *level <= 2 { body * 0.3 } else { 0.0 }, Ctx { scale, bold: true, color, ..ctx });
                if *level <= 2 {
                    let line_y = bottom + body * 0.3;
                    self.items.push(Item::Line { from: PointF::new(x, line_y), to: PointF::new(x + width, line_y), color: self.pal.border, width: 1.0 });
                    return line_y + 1.0;
                }
                bottom
            }
            Block::Code { text: code, lang } if self.pal.editor.is_some() => self.editor_code(code, lang, x, width, y, ctx),
            Block::Code { text: code, lang } => {
                let pad = 10.0;
                let line_h = self.pal.mono.size * 1.5;
                // O realce do `Memo` (linguagem desconhecida: tudo na cor do texto).
                let expanded = code.replace('\t', "    ");
                let spans = aylis_controls_base::syntax::highlight(lang, &expanded);
                let lines: Vec<&str> = expanded.split('\n').collect();
                let h = lines.len() as f32 * line_h + 2.0 * pad;
                self.items.push(Item::Fill { rect: RectF::new(x, y, width, h), color: self.pal.code_bg, radius: 6.0 });
                // As linhas vazias não viram trecho: a quebra conta todas até a próxima.
                let mut previous: Option<usize> = None;
                for (i, line) in lines.iter().enumerate() {
                    if line.is_empty() {
                        continue;
                    }
                    if let Some(p) = previous {
                        self.sep(Sep::Lines((i - p).min(3) as u8));
                    }
                    previous = Some(i);
                    let mut sep = self.take_sep();
                    // Um trecho por cor, lado a lado; o que passa da largura termina em "…".
                    let chars: Vec<char> = line.chars().collect();
                    let right = x + width - pad;
                    let line_y = y + pad + i as f32 * line_h;
                    let mut cx = x + pad;
                    let mut runs: Vec<(usize, usize, Color)> = Vec::new();
                    for s in spans.get(i).map(Vec::as_slice).unwrap_or(&[]) {
                        let color = aylis_controls_base::syntax::color(s.kind, self.pal.dark, ctx.color);
                        let (start, end) = (s.start.min(chars.len()), s.end.min(chars.len()));
                        match runs.last_mut() {
                            Some(last) if last.2 == color && last.1 == start => last.1 = end,
                            _ if start < end => runs.push((start, end, color)),
                            _ => {}
                        }
                    }
                    if runs.first().is_none_or(|r| r.0 > 0) || runs.last().is_none_or(|r| r.1 < chars.len()) {
                        // Trechos sem cobertura (não deveria acontecer): a linha inteira, sem cor.
                        runs = vec![(0, chars.len(), ctx.color)];
                    }
                    for (start, end, color) in runs {
                        if cx >= right {
                            break;
                        }
                        let text: String = chars[start..end].iter().collect();
                        let style = TextStyle { font: self.pal.mono.clone(), color, trimming: Trimming::Character, ..Default::default() };
                        let w = (self.measure)(&text, &style).min(right - cx);
                        let rect = RectF::new(cx, line_y, w, line_h);
                        self.items.push(Item::Text { rect, text, style, strike: false, underline: false, background: None, link: None, sep: std::mem::take(&mut sep) });
                        cx += w;
                    }
                }
                // O botão Copiar, no canto de cima.
                let label = aylis_core::messages::t("Copiar");
                let mut font = self.pal.body.clone();
                font.size *= 0.8;
                let bw = (self.measure)(&label, &TextStyle { font, ..Default::default() }) + 16.0;
                let bh = self.pal.body.size * 1.5;
                self.items.push(Item::Copy { rect: RectF::new(x + width - bw - 6.0, y + 6.0, bw, bh), text: code.clone() });
                y + h
            }
            Block::Quote(inner) => {
                let bottom = self.blocks(inner, x + 16.0, width - 16.0, y, Ctx { color: self.pal.muted, ..ctx });
                self.items.push(Item::Fill { rect: RectF::new(x + 2.0, y, 3.0, (bottom - y).max(body)), color: self.pal.border, radius: 1.5 });
                bottom
            }
            Block::Rule => {
                let line_y = y + body * 0.5;
                self.items.push(Item::Line { from: PointF::new(x, line_y), to: PointF::new(x + width, line_y), color: self.pal.border, width: 1.0 });
                line_y + body * 0.5
            }
            Block::List { start, items } => {
                let indent = 24.0;
                let mut y = y;
                for (i, (task, content)) in items.iter().enumerate() {
                    if i > 0 {
                        y += self.gap() * 0.4;
                        self.sep(Sep::Lines(1));
                    }
                    let line_h = body * 1.5;
                    match (task, start) {
                        (Some(done), _) => {
                            let side = body * 0.95;
                            let rect = RectF::new(x + 2.0, y + (line_h - side) / 2.0, side, side);
                            self.items.push(Item::Check { rect, checked: *done, color: if *done { self.pal.link } else { self.pal.muted } });
                        }
                        (None, Some(n)) => {
                            let style = TextStyle { font: self.pal.body.clone(), color: self.pal.muted, h_align: TextAlign::Trailing, ..Default::default() };
                            let rect = RectF::new(x - 6.0, y, indent, line_h);
                            let sep = self.take_sep();
                            self.items.push(Item::Text { rect, text: format!("{}.", n + i as u64), style, strike: false, underline: false, background: None, link: None, sep });
                            self.sep(Sep::Space);
                        }
                        (None, None) => {
                            let d = body * 0.36;
                            let rect = RectF::new(x + 6.0, y + (line_h - d) / 2.0, d, d);
                            self.items.push(Item::Fill { rect, color: ctx.color, radius: d / 2.0 });
                        }
                    }
                    // Itens de lista ficam juntinhos: sem o espaço entre parágrafos.
                    let mut cy = y;
                    for (k, b) in content.iter().enumerate() {
                        if k > 0 {
                            cy += self.gap() * 0.4;
                            self.sep(Sep::Lines(1));
                        }
                        cy = self.block(b, x + indent + 6.0, width - indent - 6.0, cy, ctx);
                    }
                    y = cy.max(y + line_h);
                }
                y
            }
            Block::Table { aligns, rows } => self.table(aligns, rows, x, width, y, ctx),
        }
    }

    fn table(&mut self, aligns: &[TextAlign], rows: &[Vec<Vec<Inline>>], x: f32, width: f32, y: f32, ctx: Ctx) -> f32 {
        let cols = rows.iter().map(Vec::len).max().unwrap_or(0);
        if cols == 0 {
            return y;
        }
        let pad = 8.0;
        // Cada coluna: a largura natural (o texto numa linha) e a mínima (a palavra mais larga,
        // que não quebra), medidas com a fonte de cada trecho (o código em fonte mono).
        let mut natural = vec![40.0f32; cols];
        let mut minimum = vec![40.0f32; cols];
        for (r, row) in rows.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let (mut line, mut word) = (0.0f32, 0.0f32);
                for inline in cell {
                    let Inline::Text { text, emphasis, .. } = inline else { continue };
                    let mut font = if emphasis.code { self.pal.mono.clone() } else { self.pal.body.clone() };
                    font.size *= ctx.scale;
                    if r == 0 || emphasis.bold || ctx.bold {
                        font.weight = FontWeight::Bold;
                    }
                    let style = TextStyle { font, ..Default::default() };
                    line += (self.measure)(text, &style);
                    for token in text.split(' ').filter(|t| !t.is_empty()) {
                        word = word.max((self.measure)(token, &style));
                    }
                }
                natural[c] = natural[c].max(line + 2.0 * pad + 2.0);
                minimum[c] = minimum[c].max(word + 2.0 * pad + 2.0).min(natural[c]);
            }
        }
        // Cabe: cada coluna com a natural. Não cabe: cada uma com a mínima, e o que sobra
        // dividido pelo quanto cada uma ainda queria (encolher todas na mesma proporção deixava
        // a coluna curta mais estreita que a palavra dela, e o texto passava para a vizinha).
        // Nem as mínimas cabem: elas encolhem, e a palavra comprida quebra por caractere.
        let total: f32 = natural.iter().sum();
        let floor: f32 = minimum.iter().sum();
        let widths: Vec<f32> = if total <= width {
            natural.clone()
        } else if floor >= width {
            minimum.iter().map(|m| m * width / floor).collect()
        } else {
            let want = total - floor;
            minimum.iter().zip(&natural).map(|(m, n)| m + (n - m) / want * (width - floor)).collect()
        };
        let table_w: f32 = widths.iter().sum();
        let mut ry = y;
        for (r, row) in rows.iter().enumerate() {
            let header = r == 0;
            let start = self.items.len();
            let mut bottom = ry;
            let mut cx = x;
            for (c, w) in widths.iter().enumerate() {
                // Ao copiar: as células separadas por tabulação, as linhas por quebra.
                self.sep(if c > 0 { Sep::Tab } else if r > 0 { Sep::Lines(1) } else { Sep::None });
                if let Some(cell) = row.get(c) {
                    let align = aligns.get(c).copied().unwrap_or(TextAlign::Leading);
                    let b = self.inlines(cell, cx + pad, w - 2.0 * pad, ry + pad * 0.75, Ctx { align, bold: header || ctx.bold, ..ctx });
                    bottom = bottom.max(b);
                }
                cx += w;
            }
            let row_h = bottom - ry + pad * 0.75;
            if header {
                self.items.insert(start, Item::Fill { rect: RectF::new(x, ry, table_w, row_h), color: self.pal.code_bg, radius: 0.0 });
            }
            ry += row_h;
            self.items.push(Item::Line { from: PointF::new(x, ry), to: PointF::new(x + table_w, ry), color: self.pal.border, width: 1.0 });
        }
        // Bordas: em cima, dos lados e entre as colunas.
        let border = self.pal.border;
        self.items.push(Item::Line { from: PointF::new(x, y), to: PointF::new(x + table_w, y), color: border, width: 1.0 });
        let mut cx = x;
        for w in widths.iter().copied().chain([0.0]) {
            self.items.push(Item::Line { from: PointF::new(cx, y), to: PointF::new(cx, ry), color: border, width: 1.0 });
            cx += w;
        }
        ry
    }

    /// Texto corrido com quebra por palavra. Devolve o fim (y).
    fn inlines(&mut self, inlines: &[Inline], x: f32, width: f32, y: f32, ctx: Ctx) -> f32 {
        let size = self.pal.body.size * ctx.scale;
        let line_h = size * 1.5;
        let mut cx = x;
        let mut cy = y;
        let mut line_start = self.items.len();
        let mut used_line = false;
        let finish_line = |items: &mut Vec<Item>, from: usize, cx: f32| {
            let shift = match ctx.align {
                TextAlign::Leading | TextAlign::Justify => 0.0,
                TextAlign::Center => (x + width - cx) / 2.0,
                TextAlign::Trailing => x + width - cx,
            };
            if shift > 0.0 {
                for item in &mut items[from..] {
                    if let Item::Text { rect, .. } = item {
                        rect.x += shift;
                    }
                }
            }
        };
        for inline in inlines {
            match inline {
                Inline::Break => {
                    finish_line(&mut self.items, line_start, cx);
                    self.sep(Sep::Lines(1));
                    cx = x;
                    cy += line_h;
                    line_start = self.items.len();
                    used_line = true;
                }
                Inline::Image { url, alt } => {
                    let size = (self.images)(url);
                    if cx > x {
                        finish_line(&mut self.items, line_start, cx);
                        cx = x;
                        cy += line_h;
                    }
                    match size {
                        Some(s) if s.width > 0.0 && s.height > 0.0 => {
                            let k = (width / s.width).min(1.0);
                            let rect = RectF::new(x, cy, s.width * k, s.height * k);
                            self.items.push(Item::Image { rect, url: url.clone() });
                            cy += rect.height;
                        }
                        _ => {
                            let style = TextStyle { font: self.pal.body.clone(), color: self.pal.muted, ..Default::default() };
                            let rect = RectF::new(x, cy, width, line_h);
                            let sep = self.take_sep();
                            self.items.push(Item::Text { rect, text: format!("[{alt}]"), style, strike: false, underline: false, background: None, link: None, sep });
                            cy += line_h;
                        }
                    }
                    line_start = self.items.len();
                    used_line = false;
                }
                Inline::Text { text, emphasis, link } => {
                    let mut font = if emphasis.code { self.pal.mono.clone() } else { self.pal.body.clone() };
                    font.size *= ctx.scale;
                    if emphasis.bold || ctx.bold {
                        font.weight = FontWeight::Bold;
                    }
                    // Itálico pelo texto (`*assim*`) ou pela fonte do controle.
                    font.italic |= emphasis.italic;
                    let color = if link.is_some() { self.pal.link } else { ctx.color };
                    let style = TextStyle { font, color, trimming: Trimming::None, ..Default::default() };
                    let background = emphasis.code.then_some(self.pal.code_bg);
                    // Uma palavra mais larga que a linha inteira (um caminho comprido numa coluna
                    // estreita) quebra por caractere, em vez de passar por cima do vizinho.
                    let mut tokens: Vec<std::borrow::Cow<'_, str>> = Vec::new();
                    for token in text.split_inclusive(' ') {
                        if (self.measure)(token.trim_end(), &style) <= width {
                            tokens.push(token.into());
                            continue;
                        }
                        let mut piece = String::new();
                        for ch in token.chars() {
                            piece.push(ch);
                            if piece.chars().count() > 1 && (self.measure)(&piece, &style) > width {
                                piece.pop();
                                tokens.push(std::mem::take(&mut piece).into());
                                piece.push(ch);
                            }
                        }
                        if !piece.is_empty() {
                            tokens.push(piece.into());
                        }
                    }
                    for token in tokens.iter().map(|t| t.as_ref()) {
                        if cx == x && token.trim().is_empty() {
                            // O espaço que ficou no começo da linha ainda separa as palavras ao copiar.
                            self.sep(Sep::Space);
                            continue;
                        }
                        let w = (self.measure)(token, &style);
                        let fits = cx + w.min(width) <= x + width + 0.5 || cx == x;
                        if !fits {
                            // Quebra: o espaço do fim da linha não conta.
                            finish_line(&mut self.items, line_start, cx);
                            cx = x;
                            cy += line_h;
                            line_start = self.items.len();
                            if token.trim().is_empty() {
                                self.sep(Sep::Space);
                                continue;
                            }
                        }
                        used_line = true;
                        // Junta com o pedaço anterior se for o mesmo estilo, na mesma linha.
                        if let Some(Item::Text { rect, text: prev, style: s, link: l, background: bg, strike, .. }) = self.items.last_mut() {
                            if *s == style && l == link && *bg == background && *strike == emphasis.strike && (rect.right() - cx).abs() < 0.5 && rect.y == cy && self.next_sep == Sep::None {
                                prev.push_str(token);
                                rect.width += w;
                                cx += w;
                                continue;
                            }
                        }
                        let rect = RectF::new(cx, cy, w, line_h);
                        let sep = self.take_sep();
                        self.items.push(Item::Text {
                            rect,
                            text: token.to_string(),
                            style: style.clone(),
                            strike: emphasis.strike,
                            underline: link.is_some(),
                            background,
                            link: link.clone(),
                            sep,
                        });
                        cx += w;
                    }
                }
            }
        }
        finish_line(&mut self.items, line_start, cx);
        if used_line || cx > x { cy + line_h } else { cy }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    fn build(md: &str, width: f32) -> Layout {
        let pal = Palette::from_theme(&Theme::light(), 0.0);
        // Cada caractere mede 7 (fonte de teste).
        let mut measure = |s: &str, _: &TextStyle| s.chars().count() as f32 * 7.0;
        let mut images = |_: &str| None;
        Layout::build(&parse(md), width, &pal, &mut measure, &mut images)
    }

    /// O texto copiado volta com a forma do original: a linha quebrada pela largura vira espaço
    /// de novo, os parágrafos ficam separados por linha em branco, as células da tabela por
    /// tabulação, e o código mantém as linhas (as vazias também); o botão Copiar leva o código.
    #[test]
    fn the_copied_text_keeps_the_shape_of_the_original() {
        let md = "Um texto **bem** comprido que quebra.\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n1. primeiro\n2. segundo\n\n```rust\nlet a = 1;\n\nlet b = 2;\n```";
        let l = build(md, 120.0);
        let (from, to) = l.text_bounds().unwrap();
        assert_eq!(l.text_between(from, to), "Um texto bem comprido que quebra.\n\nA\tB\n1\t2\n\n1. primeiro\n2. segundo\n\nlet a = 1;\n\nlet b = 2;");
        // Em qualquer ordem, e com o pedaço de dentro de um trecho.
        assert_eq!(l.text_between(TextPos { item: from.item, ch: 3 }, TextPos { item: from.item, ch: 8 }), "texto");
        assert_eq!(l.text_between(TextPos { item: from.item, ch: 8 }, TextPos { item: from.item, ch: 3 }), "texto");
        let (_, code) = l.items.iter().enumerate().find_map(|(n, i)| if let Item::Copy { text, .. } = i { Some((n, text.clone())) } else { None }).unwrap();
        assert_eq!(code, "let a = 1;\n\nlet b = 2;");
        // A palavra e o parágrafo do duplo e do triplo clique.
        let (a, b) = l.word_at(TextPos { item: from.item, ch: 5 }, false);
        assert_eq!(l.text_between(a, b), "texto");
        let (a, b) = l.word_at(TextPos { item: from.item, ch: 5 }, true);
        assert_eq!(l.text_between(a, b), "Um texto bem comprido que quebra.");
    }

    /// Blocos de código seguidos em linguagens diferentes viram um grupo de abas: só o código
    /// da escolhida aparece (a primeira, sem escolha), e a escolha vale para todos os grupos.
    #[test]
    fn consecutive_code_blocks_become_language_tabs() {
        let md = "Antes.\n\n```rust\nlet a = 1;\n```\n\n```python\na = 1\n```\n\n```csharp\nvar a = 1;\n```\n\nDepois.\n\n```rust\nfn b() {}\n```\n\n```python\ndef b(): pass\n```";
        let pal = Palette::from_theme(&Theme::light(), 0.0);
        let mut measure = |s: &str, _: &TextStyle| s.chars().count() as f32 * 7.0;
        let mut images = |_: &str| None;
        // As linhas inteiras (o código realçado vem em trechos, um por cor).
        let texts = |l: &Layout| l.text_bounds().map_or(String::new(), |(a, b)| l.text_between(a, b)).split('\n').map(str::to_string).collect::<Vec<_>>();
        let tabs =|l: &Layout| l.items.iter().filter_map(|i| if let Item::Tab { label, active, .. } = i { Some((label.clone(), *active)) } else { None }).collect::<Vec<_>>();
        let first = Layout::build(&parse(md), 500.0, &pal, &mut measure, &mut images);
        assert_eq!(tabs(&first)[..3], [("Rust".to_string(), true), ("Python".to_string(), false), ("C#".to_string(), false)]);
        let t = texts(&first);
        assert!(t.contains(&"let a = 1;".to_string()) && !t.contains(&"a = 1".to_string()), "só o código da aba escolhida: {t:?}");
        let python = Layout::build_in(&parse(md), 500.0, &pal, "python", &mut measure, &mut images);
        let t = texts(&python);
        assert!(t.contains(&"a = 1".to_string()) && t.contains(&"def b(): pass".to_string()), "a escolha vale nos dois grupos: {t:?}");
        let tab = python.items.iter().find_map(|i| if let Item::Tab { rect, lang, .. } = i { (lang == "csharp").then_some(rect.center()) } else { None }).unwrap();
        assert_eq!(python.tab_at(tab), Some("csharp"));
    }

    /// O código dos blocos sai colorido pela linguagem (o realce do `Memo`): a palavra-chave numa
    /// cor, o texto entre aspas em outra; a linguagem desconhecida fica toda na cor do texto.
    #[test]
    fn code_blocks_are_highlighted_by_language() {
        let color_of = |l: &Layout, word: &str| l.items.iter().find_map(|i| if let Item::Text { text, style, .. } = i { (text.trim() == word).then_some(style.color) } else { None });
        let l = build("```rust\nlet nome = \"aylis\";\n```", 500.0);
        let (keyword, string) = (color_of(&l, "let").unwrap(), color_of(&l, "\"aylis\"").unwrap());
        assert_ne!(keyword, string);
        assert_ne!(keyword, Theme::light().color("text"));
        let plain = build("```\nlet nome = \"aylis\";\n```", 500.0);
        let t: Vec<_> = plain.items.iter().filter_map(|i| if let Item::Text { text, .. } = i { Some(text.as_str()) } else { None }).collect();
        assert_eq!(t, ["let nome = \"aylis\";"]);
    }

    /// Com a cara do editor (a dica do código da IDE): a assinatura comprida quebra depois de
    /// uma vírgula e continua embaixo (antes terminava em "…"), dentro da largura; o bloco sem
    /// linguagem fica com a do editor (colorido); não há botão Copiar nem abas; e copiar a
    /// linha quebrada a devolve inteira.
    #[test]
    fn editor_code_wraps_long_lines_and_uses_the_editor_language() {
        let mut pal = Palette::from_theme(&Theme::light(), 0.0);
        pal.editor = Some(EditorCode { color_theme: "Dark+".into(), language: "rust".into() });
        let mut measure = |s: &str, _: &TextStyle| s.chars().count() as f32 * 7.0;
        let mut images = |_: &str| None;
        let sig = "pub fn set_prop(&mut self, id: ComponentId, name: &str, value: PropValue) -> Result<(), Error>";
        let md = format!("```rust\naylis_core::Ui\n```\n\n```rust\n{sig}\n```\n\n---\n\nTexto.\n\n```\nlet a = 1;\n```");
        let l = Layout::build(&parse(&md), 300.0, &pal, &mut measure, &mut images);
        assert!(!l.items.iter().any(|i| matches!(i, Item::Copy { .. } | Item::Tab { .. })));
        let texts: Vec<(RectF, &str, Color)> = l.items.iter().filter_map(|i| if let Item::Text { rect, text, style, .. } = i { Some((*rect, text.as_str(), style.color)) } else { None }).collect();
        assert!(texts.iter().all(|(r, ..)| r.right() <= 300.5), "dentro da largura");
        assert!(!texts.iter().any(|(_, t, _)| t.contains('…')));
        // A assinatura ocupa várias linhas, quebradas num espaço, vírgula ou parêntese.
        let start = texts.iter().position(|(_, t, _)| t.starts_with("pub")).unwrap();
        let end = texts.iter().position(|(_, t, _)| t.contains("Error")).unwrap();
        let mut rows: Vec<f32> = texts[start..=end].iter().map(|(r, ..)| r.y).collect();
        rows.dedup();
        assert!(rows.len() >= 3, "{rows:?}");
        let first_row: String = texts.iter().filter(|(r, ..)| r.y == rows[0]).map(|(_, t, _)| *t).collect();
        assert!(first_row.ends_with([' ', ',', '(', '<']), "quebra depois de um espaço, vírgula ou parêntese: {first_row:?}");
        let (from, to) = l.text_bounds().unwrap();
        assert!(l.text_between(from, to).contains(sig), "copiar devolve a linha inteira");
        // O exemplo sem linguagem sai colorido como Rust, nas cores do tema do editor.
        let keyword = texts.iter().find(|(_, t, _)| t.trim() == "let").map(|(_, _, c)| *c).unwrap();
        assert_eq!(keyword, aylis_controls_base::syntax::code_theme("Dark+").unwrap().token(aylis_controls_base::syntax::TokenKind::Keyword));
    }

    /// A tabela da página "Fontes e ícones offline": um caminho em código na primeira coluna
    /// e um texto comprido na segunda, numa largura apertada. Antes, as colunas encolhiam na
    /// mesma proporção e o caminho passava por cima do texto da coluna ao lado.
    #[test]
    fn table_cells_never_overlap_the_next_column() {
        let md = "| Recurso | O que é | Licença |\n|---|---|---|\n| `resources/fonts/` | 32 famílias do Google Fonts: Inter, Roboto, Roboto Mono, Open Sans, Lato, Montserrat, Poppins, Noto Sans e outras, com a licença de cada uma | SIL OFL 1.1 |\n| `resources/icons/material-symbols-outlined/` | ícones | Apache-2.0 |";
        for width in [300.0, 500.0, 800.0] {
            let l = build(md, width);
            let texts: Vec<RectF> = l.items.iter().filter_map(|i| if let Item::Text { rect, .. } = i { Some(*rect) } else { None }).collect();
            for (i, a) in texts.iter().enumerate() {
                assert!(a.right() <= width + 0.5, "largura {width}: um texto sai da tabela: {a:?}");
                for b in &texts[i + 1..] {
                    let same_line = (a.y - b.y).abs() < 0.5;
                    let overlap = a.x < b.right() - 0.5 && b.x < a.right() - 0.5;
                    assert!(!(same_line && overlap), "largura {width}: textos um sobre o outro: {a:?} e {b:?}");
                }
            }
        }
    }

    #[test]
    fn words_wrap_at_the_width() {
        let l = build("um dois tres quatro cinco seis", 100.0);
        let lines: std::collections::BTreeSet<i32> = l.items.iter().map(|i| i.rect().y as i32).collect();
        assert!(lines.len() >= 2, "{l:?}");
        assert!(l.items.iter().all(|i| i.rect().right() <= 100.5), "{l:?}");
    }

    #[test]
    fn links_are_found_by_position() {
        let l = build("veja [aqui](https://x.dev) agora", 400.0);
        let link = l.items.iter().find_map(|i| if let Item::Text { rect, link: Some(_), .. } = i { Some(*rect) } else { None }).unwrap();
        assert_eq!(l.link_at(link.center()), Some("https://x.dev"));
        assert_eq!(l.link_at(PointF::new(1.0, 1.0)), None);
    }

    #[test]
    fn blocks_stack_down() {
        let l = build("# T\n\npar\n\n- a\n- b\n\n| x | y |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```", 300.0);
        assert!(l.height > 150.0, "{}", l.height);
        assert!(l.items.iter().any(|i| matches!(i, Item::Fill { radius, .. } if *radius == 6.0)), "bloco de código");
    }
}
