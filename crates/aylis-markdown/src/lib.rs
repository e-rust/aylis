//! # aylis-markdown
//!
//! O componente [`Markdown`]: mostra texto em Markdown (CommonMark com tabelas, riscado e
//! listas de tarefas) com as cores e fontes do tema, desenhado pelo próprio `Canvas`
//! (igual no Skia, no Direct2D e no navegador). Rola com a roda do mouse e avisa o clique
//! num link pelo evento `on_link` (o endereço vem em `EventArgs::Text`).
//!
//! O que lê: títulos, parágrafos, **negrito**, *itálico*, ~~riscado~~, `código`, blocos
//! de código, citações, listas (numeradas, com marcadores e de tarefas, aninhadas),
//! tabelas (com alinhamento), linhas horizontais, links e imagens (do projeto).

pub mod layout;
pub mod parse;

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Image, Key, KeyEvent, PaintCtx, PointF, PointerButton,
    PointerEvent, PointerKind, RectF, Shadow, SizeF, Stroke, TextStyle, Ui, WheelEvent, properties,
};

pub use layout::{EditorCode, Heading, Item, Layout, Palette, Sep, TextPos, slug};
pub use parse::{Block, Inline, parse};

/// Texto em Markdown (um rótulo com texto rico).
///
/// - `text`: o Markdown; ou `source`, um arquivo do projeto (`assets/docs/ajuda.md`);
/// - `on_link`: clique num link (o endereço em `EventArgs::Text`);
/// - rola sozinho quando o texto passa da altura (roda do mouse).
pub struct Markdown {
    pub base: ControlBase,
    /// O texto em Markdown; vale quando `source` está vazio.
    pub text: String,
    /// Um arquivo do projeto com o Markdown (`assets/docs/ajuda.md`); vence o `text`. Vazio:
    /// mostra o `text`.
    pub source: String,
    /// Tamanho da fonte do corpo (0 = o do tema).
    pub font_size: f32,
    /// Os blocos de código como no editor de código ([`EditorCode`]: as cores e o realce dele,
    /// a linha longa quebrando, sem o botão Copiar); `None`, como numa página. Pelo código (a
    /// dica do servidor de linguagem na IDE), ou pelas propriedades abaixo.
    pub editor_code: Option<EditorCode>,
    /// Os blocos de código com a cara do editor (liga o [`EditorCode`] com as duas de baixo).
    pub editor_look: bool,
    /// O tema de cores dos blocos com a cara do editor (vazio: o do app).
    pub code_theme: String,
    /// A linguagem dos blocos sem linguagem na cerca (`rust`, `python`...).
    pub code_language: String,
    /// O título para onde rolar: o identificador (`gravar-no-banco`, com ou sem `#`) ou o texto
    /// dele (o "Nesta página" de uma documentação, um link `pagina.md#secao`).
    pub anchor: String,
    /// A página e o título até onde já rolou (o `anchor` vale uma vez).
    anchored: RefCell<(String, String)>,
    /// O texto a destacar na página (sem diferença de maiúsculas), como o achado de uma busca:
    /// cada ocorrência ganha um fundo, e a página rola até a primeira.
    pub highlight: String,
    /// O texto (o hash do layout) e o destaque até onde já rolou (rola uma vez, como o
    /// `anchor`).
    highlighted: RefCell<(u64, String)>,
    loaded: RefCell<Option<(String, String)>>,
    cache: RefCell<Option<(u64, f32, Layout)>>,
    scroll: Cell<f32>,
    content_height: Cell<f32>,
    /// Arrastando a barra: onde o polegar foi pego (distância do topo dele).
    bar_drag: Option<f32>,
    /// A linguagem escolhida nas abas de código (vale para todos os grupos, também nas
    /// próximas páginas, como nos sites de documentação).
    lang: String,
    /// Quadros seguidos esperando uma imagem carregar (com limite: um arquivo que nunca
    /// decodifica não deixa a página pedindo quadro para sempre).
    image_waits: Cell<u32>,
    /// As imagens do texto, pelo caminho: a mesma para o layout e o desenho, quadro a quadro
    /// (no navegador, cada `Image` nova recomeçava a decodificação e nunca ficava pronta).
    images: RefCell<std::collections::HashMap<String, Image>>,
    /// O texto selecionado: de onde começou (a âncora) até onde está (o cursor). Some quando o
    /// layout muda (outro texto ou outra largura).
    selection: Cell<Option<(TextPos, TextPos)>>,
    /// Arrastando para selecionar (e se já andou: aí soltar sobre um link não o abre).
    selecting: bool,
    dragged: bool,
    pressed_at: PointF,
    /// O botão Copiar sob o mouse e o que acabou de copiar (mostra "Copiado" até o mouse sair).
    copy_hover: Option<usize>,
    copied: Option<usize>,
    /// O menu do botão direito (só "Copiar"), onde abriu.
    menu: Option<PointF>,
}

/// A largura da faixa da barra que aceita o clique (a barra pintada é mais fina).
const BAR_HIT: f32 = 12.0;

/// O menu do botão direito: largura e altura.
const MENU: SizeF = SizeF { width: 132.0, height: 32.0 };

impl Default for Markdown {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 200.0);
        // Com foco: o Ctrl+C e o Ctrl+A da seleção.
        base.set_padding(8.0).set_clip_children(true).set_can_focus(true);
        Self {
            base,
            text: "# Título\n\nTexto com **negrito**, *itálico* e `código`.\n\n- um\n- dois\n".into(),
            source: String::new(),
            font_size: 0.0,
            editor_code: None,
            editor_look: false,
            code_theme: String::new(),
            code_language: String::new(),
            anchor: String::new(),
            anchored: RefCell::default(),
            highlight: String::new(),
            highlighted: RefCell::default(),
            loaded: RefCell::new(None),
            cache: RefCell::new(None),
            scroll: Cell::new(0.0),
            content_height: Cell::new(0.0),
            bar_drag: None,
            lang: String::new(),
            image_waits: Cell::new(0),
            images: RefCell::default(),
            selection: Cell::new(None),
            selecting: false,
            dragged: false,
            pressed_at: PointF::default(),
            copy_hover: None,
            copied: None,
            menu: None,
        }
    }
}

impl Markdown {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// Altura do conteúdo no último desenho (para quem quiser ajustar o tamanho).
    pub fn content_height(&self) -> f32 {
        self.content_height.get()
    }

    /// Quanto o texto está rolado (0 = o começo).
    pub fn scroll_offset(&self) -> f32 {
        self.scroll.get()
    }

    /// Volta ao começo.
    pub fn scroll_to_top(&mut self) {
        self.scroll.set(0.0);
    }

    fn sync(&mut self, name: &str) {
        // O título pedido de novo (o mesmo do "Nesta página", depois de rolar) rola de novo; o
        // texto continua o mesmo, sem refazer o layout.
        if name == "anchor" {
            self.anchored.replace(Default::default());
            return;
        }
        if name == "highlight" {
            self.highlighted.replace(Default::default());
            return;
        }
        if matches!(name, "editor_look" | "code_theme" | "code_language") {
            self.editor_code = self.editor_look.then(|| EditorCode { color_theme: self.code_theme.clone(), language: self.code_language.clone() });
        }
        self.cache.replace(None);
        self.scroll.set(0.0);
        self.selection.set(None);
        self.menu = None;
    }

    /// O texto selecionado (vazio sem seleção).
    pub fn selected_text(&self) -> String {
        let cache = self.cache.borrow();
        match (cache.as_ref(), self.selection.get()) {
            (Some((_, _, layout)), Some((a, b))) if a != b => layout.text_between(a, b),
            _ => String::new(),
        }
    }

    /// Seleciona o texto todo (o Ctrl+A).
    pub fn select_all(&mut self) {
        let bounds = self.cache.borrow().as_ref().and_then(|(_, _, l)| l.text_bounds());
        self.selection.set(bounds);
    }

    /// Ponto do controle → ponto do conteúdo (o topo do texto é 0).
    fn content_point(&self, p: PointF) -> PointF {
        let inner = self.inner();
        PointF::new(p.x - inner.x, p.y - inner.y + self.scroll.get())
    }

    /// O ponto do texto sob `p` (coordenadas do controle), com o caractere medido pela fonte.
    fn text_pos(&self, ui: &Ui, p: PointF) -> Option<TextPos> {
        let cache = self.cache.borrow();
        let (_, _, layout) = cache.as_ref()?;
        let at = self.content_point(p);
        layout.text_pos_at(at, &mut |n, x| match &layout.items[n] {
            Item::Text { rect, text, style, .. } => char_at(ui, text, style, x - rect.x),
            _ => 0,
        })
    }

    /// Copia `text` para a área de transferência.
    fn copy_text(ctx: &mut EventCtx<'_>, text: &str) {
        if text.is_empty() {
            return;
        }
        aylis_core::clipboard::set_text(ctx.ui, text);
    }

    /// O retângulo do menu do botão direito (dentro do controle).
    fn menu_rect(&self) -> Option<RectF> {
        let at = self.menu?;
        let b = self.base.bounds();
        Some(RectF::new(at.x.min(b.width - MENU.width).max(0.0), at.y.min(b.height - MENU.height).max(0.0), MENU.width, MENU.height))
    }

    fn markdown(&self, ctx: &PaintCtx<'_>) -> String {
        if self.source.is_empty() {
            return self.text.clone();
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(s, _)| *s != self.source) {
            let text = ctx.resource(&self.source).map(|d| String::from_utf8_lossy(&d).to_string()).unwrap_or_default();
            *loaded = Some((self.source.clone(), text));
        }
        loaded.as_ref().map(|(_, t)| t.clone()).unwrap_or_default()
    }

    fn inner(&self) -> RectF {
        let b = self.base.bounds();
        let p = self.base.padding();
        RectF::new(p.left, p.top, (b.width - p.left - p.right).max(0.0), (b.height - p.top - p.bottom).max(0.0))
    }

    /// A barra de rolagem: o topo e a altura do polegar (coordenadas do controle) e o máximo da
    /// rolagem; `None` se o texto cabe.
    fn bar(&self) -> Option<(f32, f32, f32)> {
        let inner = self.inner();
        let height = self.content_height.get();
        let max = (height - inner.height).max(0.0);
        if max <= 0.0 || height <= 0.0 {
            return None;
        }
        let h = (inner.height * inner.height / height).max(24.0);
        Some((inner.y + (inner.height - h) * (self.scroll.get() / max).clamp(0.0, 1.0), h, max))
    }

    /// Arrastando a barra até `y` (coordenadas do controle): rola junto.
    fn drag_bar(&mut self, y: f32) {
        let (Some(grab), Some((_, h, max))) = (self.bar_drag, self.bar()) else { return };
        let inner = self.inner();
        let room = (inner.height - h).max(1.0);
        self.scroll.set(((y - grab - inner.y) / room).clamp(0.0, 1.0) * max);
    }

    /// A imagem do caminho `url` do texto (relativo à página), a mesma de quadro a quadro.
    fn image(&self, ctx: &PaintCtx<'_>, url: &str) -> Option<Image> {
        let path = resolve(&self.source, url);
        if let Some(image) = self.images.borrow().get(&path) {
            return Some(image.clone());
        }
        let image = Image::encoded(ctx.resource(&path)?);
        self.images.borrow_mut().insert(path, image.clone());
        Some(image)
    }

    /// A aba de código no ponto (coordenadas do controle): a linguagem dela.
    fn tab_at(&self, p: PointF) -> Option<String> {
        let cache = self.cache.borrow();
        let (_, _, layout) = cache.as_ref()?;
        let inner = self.inner();
        layout.tab_at(PointF::new(p.x - inner.x, p.y - inner.y + self.scroll.get())).map(str::to_string)
    }

    /// O link no ponto (coordenadas do controle).
    fn link_at(&self, p: PointF) -> Option<String> {
        let cache = self.cache.borrow();
        let (_, _, layout) = cache.as_ref()?;
        let inner = self.inner();
        let at = PointF::new(p.x - inner.x, p.y - inner.y + self.scroll.get());
        layout.link_at(at).map(str::to_string)
    }
}

fn hash(text: &str, size: f32, theme: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut h);
    size.to_bits().hash(&mut h);
    theme.hash(&mut h);
    h.finish()
}

impl Component for Markdown {
    fn class_name(&self) -> &'static str {
        "Markdown"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        text: String => "Data",
        source: String => "Data",
        anchor: String => "Data",
        highlight: String => "Data",
        font_size: f32 => "Appearance",
        editor_look: bool => "Appearance",
        code_theme: String => "Appearance",
        code_language: String => "Appearance";
        events: on_link;
        on_set: sync
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let inner = self.inner();
        if inner.width <= 0.0 {
            return;
        }
        let mut palette = Palette::from_theme(ctx.theme, self.font_size);
        // A fonte do controle (`font_family`, `font_italic`...) no corpo; no código também,
        // se a família for escolhida (senão, a monoespaçada do tema). Com a cara do editor, a
        // do editor (a monoespaçada do tema), mesmo com a família escolhida num pai.
        palette.body = ctx.font(palette.body.size);
        let mono = ctx.monospace_font(palette.mono.size);
        if mono.family != "monospace" && self.editor_code.is_none() {
            palette.mono = mono;
        }
        palette.editor = self.editor_code.clone();
        let text = self.markdown(ctx);
        let font_key = format!("{}{}{:?}{:?}", palette.body.family, palette.body.italic, palette.body.weight, palette.editor);
        let key = hash(&text, palette.body.size, &format!("{}{font_key}{}", ctx.theme.name, self.lang)) ^ (palette.text.r as u64) << 1;
        let stale = self.cache.borrow().as_ref().is_none_or(|(k, w, _)| *k != key || (*w - inner.width).abs() > 0.5);
        // Uma imagem que ainda carrega (no navegador, a decodificação é assíncrona): o layout
        // desta vez sai com o texto alternativo e não fica guardado; o próximo quadro tenta de novo.
        let loading = Cell::new(false);
        if stale {
            let blocks = parse(&text);
            // As duas medidas usam o canvas (uma de cada vez).
            let shared = RefCell::new(&mut *canvas);
            let mut images = |url: &str| -> Option<SizeF> {
                let image = self.image(ctx, url)?;
                let size = shared.borrow_mut().image_size(&image);
                if size.is_none() {
                    loading.set(true);
                }
                size
            };
            let mut measure = |s: &str, style: &TextStyle| ctx_measure(&mut **shared.borrow_mut(), s, style);
            let layout = Layout::build_in(&blocks, inner.width, &palette, &self.lang, &mut measure, &mut images);
            // Outro layout: os índices da seleção apontariam para outros trechos.
            if self.cache.borrow().as_ref().is_none_or(|(_, _, old)| old.items.len() != layout.items.len() || old.items.iter().zip(&layout.items).any(|(a, b)| a.rect() != b.rect())) {
                self.selection.set(None);
            }
            self.cache.replace(Some((key, inner.width, layout)));
        }
        let cache = self.cache.borrow();
        let Some((_, _, layout)) = cache.as_ref() else { return };
        self.content_height.set(layout.height);
        let max = (layout.height - inner.height).max(0.0);
        // O título pedido em `anchor` (pelo identificador ou pelo texto): rola até ele uma vez,
        // quando a página e o título mudam (depois, a roda manda).
        let wanted = (self.source.clone(), self.anchor.clone());
        if !self.anchor.is_empty() && *self.anchored.borrow() != wanted {
            let target = self.anchor.trim_start_matches('#');
            if let Some(h) = layout.headings.iter().find(|h| h.slug == target || h.text == target || h.slug == crate::layout::slug(target)) {
                self.scroll.set((h.y - 4.0).clamp(0.0, max));
            }
            *self.anchored.borrow_mut() = wanted;
        }
        // O `highlight`: rola até a primeira ocorrência uma vez (quando o texto ou o destaque
        // mudam), se nenhum título foi pedido.
        let wanted = (key, self.highlight.clone());
        if *self.highlighted.borrow() != wanted {
            let needle = fold(self.highlight.trim());
            if self.anchor.is_empty() && !needle.is_empty() {
                let first = layout.items.iter().find_map(|i| match i {
                    Item::Text { rect, text, .. } if !occurrences(text, &needle).is_empty() => Some(rect.y),
                    _ => None,
                });
                if let Some(y) = first {
                    self.scroll.set((y - inner.height / 3.0).clamp(0.0, max));
                }
            }
            *self.highlighted.borrow_mut() = wanted;
        }
        if self.scroll.get() > max {
            self.scroll.set(max);
        }
        canvas.save();
        canvas.clip_rect(inner);
        canvas.translate(inner.x, inner.y - self.scroll.get());
        let visible = RectF::new(0.0, self.scroll.get(), inner.width, inner.height);
        let selection = self.selection.get().filter(|(a, b)| a != b).map(|(a, b)| if a <= b { (a, b) } else { (b, a) });
        let selected = Brush::Solid(ctx.theme.color("primary").with_opacity(0.28));
        let found = Brush::Solid(ctx.theme.color("warning").with_opacity(0.4));
        let needle = fold(self.highlight.trim());
        for (n, item) in layout.items.iter().enumerate() {
            // O trecho com fundo próprio (o `código` no meio do texto): o fundo vem antes, senão
            // cobriria a seleção.
            let own_background = matches!(item, Item::Text { background: Some(_), .. }) && item.rect().bottom() >= visible.y && item.rect().y <= visible.bottom();
            if let (true, Item::Text { rect, background: Some(bg), .. }) = (own_background, item) {
                canvas.fill_rect(rect.inflate(1.0), 3.0, &Brush::Solid(*bg));
            }
            // A seleção, por baixo do texto: do caractere inicial ao final de cada trecho.
            if let (Some((from, to)), Item::Text { rect, text, style, .. }) = (selection, item) {
                if n >= from.item && n <= to.item && rect.bottom() >= visible.y && rect.y <= visible.bottom() {
                    let len = text.chars().count();
                    let (start, end) = (if n == from.item { from.ch } else { 0 }, if n == to.item { to.ch.min(len) } else { len });
                    let x_of = |canvas: &mut dyn Canvas, c: usize| if c == 0 { 0.0 } else { canvas.measure_text(&text.chars().take(c).collect::<String>(), style).width };
                    let (x0, x1) = (x_of(canvas, start), x_of(canvas, end));
                    // Um trecho inteiro no meio da seleção sem largura (fim de linha): uma marca.
                    let x1 = if x1 <= x0 && n != to.item { x0 + 4.0 } else { x1 };
                    if x1 > x0 {
                        canvas.fill_rect(RectF::new(rect.x + x0, rect.y, x1 - x0, rect.height), 2.0, &selected);
                    }
                }
            }
            // O `highlight`: cada ocorrência no trecho, também por baixo do texto.
            if let (false, Item::Text { rect, text, style, .. }) = (needle.is_empty(), item) {
                if rect.bottom() >= visible.y && rect.y <= visible.bottom() {
                    for (start, end) in occurrences(text, &needle) {
                        let x_of = |canvas: &mut dyn Canvas, c: usize| if c == 0 { 0.0 } else { canvas.measure_text(&text.chars().take(c).collect::<String>(), style).width };
                        let (x0, x1) = (x_of(canvas, start), x_of(canvas, end));
                        canvas.fill_rect(RectF::new(rect.x + x0 - 1.0, rect.y, x1 - x0 + 2.0, rect.height), 2.0, &found);
                    }
                }
            }
            if let Item::Copy { rect, .. } = item {
                draw_copy(canvas, ctx, *rect, self.copy_hover == Some(n), self.copied == Some(n));
                continue;
            }
            if own_background {
                // Só o texto, por cima da seleção.
                if let Item::Text { rect, text, style, .. } = item {
                    canvas.draw_text(text, *rect, style);
                }
                continue;
            }
            draw_item(canvas, ctx, item, visible, &|url| self.image(ctx, url));
        }
        canvas.restore();
        // O menu do botão direito.
        if let Some(r) = self.menu_rect() {
            canvas.draw_shadow(r, 6.0, &Shadow::new(aylis_core::Color::BLACK.with_opacity(0.3), 12.0, 0.0, 4.0));
            canvas.fill_rect(r, 6.0, &Brush::Solid(ctx.theme.color("surface")));
            canvas.stroke_rect(r, 6.0, &Stroke::new(ctx.theme.color("border"), 1.0));
            let style = TextStyle { font: ctx.font(ctx.style.font.size), color: ctx.theme.color("text"), v_align: aylis_core::TextAlign::Center, ..Default::default() };
            canvas.draw_text(&aylis_core::messages::t("Copiar"), RectF::new(r.x + 12.0, r.y, r.width - 24.0, r.height), &style);
            let hint = TextStyle { color: ctx.theme.color("text_muted"), h_align: aylis_core::TextAlign::Trailing, ..style };
            canvas.draw_text(if cfg!(target_os = "macos") { "⌘C" } else { "Ctrl+C" }, RectF::new(r.x + 12.0, r.y, r.width - 24.0, r.height), &hint);
        }
        // A barra de rolagem, fina, quando o texto passa da altura (clicável e arrastável numa
        // faixa mais larga que ela, `BAR_HIT`).
        drop(cache);
        if let Some((y, h, _)) = self.bar() {
            let r = RectF::new(ctx.rect.width - 5.0, y, 3.0, h);
            let alpha = if self.bar_drag.is_some() { 0.7 } else { 0.4 };
            canvas.fill_rect(r, 1.5, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(alpha)));
        }
        // Imagem carregando: confere de novo 20 vezes por segundo (antes, a cada atualização do
        // monitor), por até 10 s.
        if loading.get() && self.image_waits.get() < 200 {
            self.image_waits.set(self.image_waits.get() + 1);
            self.cache.replace(None);
            ctx.request_frame_at(ctx.time + 0.05);
        } else if stale {
            self.image_waits.set(0);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let max = (self.content_height.get() - self.inner().height).max(0.0);
        let next = (self.scroll.get() - event.delta.y).clamp(0.0, max);
        if next != self.scroll.get() {
            self.scroll.set(next);
            ctx.repaint();
            ctx.set_handled();
        }
    }

    /// Na faixa da barra (a da direita): pega a barra; fora do polegar, ele pula para o clique.
    /// No texto: começa a seleção (duplo clique, a palavra; triplo, o parágrafo; com Shift,
    /// estende a que já existe). O botão direito com algo selecionado abre o menu Copiar.
    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        // O menu aberto: o clique nele copia; fora, só fecha.
        if let Some(r) = self.menu_rect() {
            self.menu = None;
            ctx.repaint();
            ctx.set_handled();
            if r.contains(event.position) && event.button == PointerButton::Primary {
                let text = self.selected_text();
                Self::copy_text(ctx, &text);
            }
            return;
        }
        if let Some((top, h, _)) = self.bar() {
            if event.position.x >= self.base.bounds().width - BAR_HIT {
                let grab = if event.position.y >= top && event.position.y <= top + h { event.position.y - top } else { h / 2.0 };
                self.bar_drag = Some(grab);
                self.drag_bar(event.position.y);
                ctx.repaint();
                ctx.set_handled();
                return;
            }
        }
        if event.button == PointerButton::Secondary {
            if !self.selected_text().is_empty() {
                self.menu = Some(event.position);
                ctx.repaint();
                ctx.set_handled();
            }
            return;
        }
        // No toque, arrastar rola; a seleção é do mouse.
        if event.button != PointerButton::Primary || event.kind == PointerKind::Touch {
            return;
        }
        let at = self.content_point(event.position);
        if self.cache.borrow().as_ref().is_some_and(|(_, _, l)| l.copy_at(at).is_some()) {
            return;
        }
        let Some(pos) = self.text_pos(ctx.ui, event.position) else { return };
        self.pressed_at = event.position;
        self.dragged = false;
        let cache = self.cache.borrow();
        let Some((_, _, layout)) = cache.as_ref() else { return };
        let selection = match event.click_count {
            2 => Some(layout.word_at(pos, false)),
            n if n >= 3 => Some(layout.word_at(pos, true)),
            _ if event.modifiers.shift => Some((self.selection.get().map_or(pos, |(a, _)| a), pos)),
            _ => Some((pos, pos)),
        };
        drop(cache);
        self.selection.set(selection);
        self.selecting = true;
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.bar_drag.is_some() {
            self.drag_bar(event.position.y);
            ctx.repaint();
            return;
        }
        if self.selecting {
            let moved = (event.position.x - self.pressed_at.x).abs() > 3.0 || (event.position.y - self.pressed_at.y).abs() > 3.0;
            self.dragged |= moved;
            // Passou da borda: rola junto (e continua selecionando o que aparece).
            let inner = self.inner();
            let max = (self.content_height.get() - inner.height).max(0.0);
            let over = if event.position.y < inner.y { event.position.y - inner.y } else if event.position.y > inner.bottom() { event.position.y - inner.bottom() } else { 0.0 };
            if over != 0.0 {
                self.scroll.set((self.scroll.get() + over * 0.5).clamp(0.0, max));
            }
            if self.dragged {
                if let (Some(pos), Some((anchor, _))) = (self.text_pos(ctx.ui, event.position), self.selection.get()) {
                    self.selection.set(Some((anchor, pos)));
                }
            }
            ctx.repaint();
            return;
        }
        let at = self.content_point(event.position);
        let copy = self.cache.borrow().as_ref().and_then(|(_, _, l)| l.copy_at(at).map(|(n, _)| n));
        if copy != self.copy_hover {
            if copy.is_none() {
                self.copied = None;
            }
            self.copy_hover = copy;
            ctx.repaint();
        }
        let over_text = self.cache.borrow().as_ref().is_some_and(|(_, _, l)| l.items.iter().any(|i| matches!(i, Item::Text { rect, .. } if rect.contains(at))));
        let cursor = if copy.is_some() || self.link_at(event.position).is_some() || self.tab_at(event.position).is_some() {
            CursorShape::Pointer
        } else if over_text {
            CursorShape::Text
        } else {
            CursorShape::Default
        };
        if self.base.cursor() != cursor {
            self.base.set_cursor(cursor);
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        if self.copy_hover.take().is_some() {
            self.copied = None;
            ctx.repaint();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let ctrl = event.modifiers.ctrl || event.modifiers.meta;
        match event.key {
            Key::Char('c') | Key::Char('C') if ctrl => {
                let text = self.selected_text();
                if !text.is_empty() {
                    Self::copy_text(ctx, &text);
                    ctx.set_handled();
                }
            }
            Key::Char('a') | Key::Char('A') if ctrl => {
                self.select_all();
                ctx.repaint();
                ctx.set_handled();
            }
            Key::Escape if self.menu.is_some() || self.selection.get().is_some_and(|(a, b)| a != b) => {
                self.menu = None;
                self.selection.set(None);
                ctx.repaint();
                ctx.set_handled();
            }
            _ => {}
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.bar_drag.take().is_some() {
            ctx.repaint();
            return;
        }
        let selecting = std::mem::take(&mut self.selecting);
        // Arrastou: era uma seleção, não um clique no link.
        if selecting && self.dragged {
            return;
        }
        // O botão Copiar de um bloco de código.
        let at = self.content_point(event.position);
        let copy = self.cache.borrow().as_ref().and_then(|(_, _, l)| l.copy_at(at).map(|(n, t)| (n, t.to_string())));
        if let Some((n, code)) = copy {
            if event.button == PointerButton::Primary {
                Self::copy_text(ctx, &code);
                self.copied = Some(n);
                ctx.repaint();
            }
            return;
        }
        if event.button != PointerButton::Primary {
            return;
        }
        // Uma aba de código: a linguagem dela passa a valer para todos os grupos.
        if let Some(lang) = self.tab_at(event.position) {
            self.lang = lang;
            self.cache.replace(None);
            ctx.repaint();
            return;
        }
        if let Some(url) = self.link_at(event.position) {
            ctx.fire("on_link", EventArgs::Text(url));
        }
    }
}

/// O texto em minúsculas, um caractere por caractere (as posições continuam as mesmas do
/// original, para medir onde cada ocorrência começa).
fn fold(text: &str) -> Vec<char> {
    text.chars().map(|c| c.to_lowercase().next().unwrap_or(c)).collect()
}

/// As ocorrências de `needle` (já em [`fold`]) em `text`, sem diferença de maiúsculas: o
/// caractere de início e o de fim de cada uma, sem sobrepor.
fn occurrences(text: &str, needle: &[char]) -> Vec<(usize, usize)> {
    let hay = fold(text);
    let mut found = Vec::new();
    if needle.is_empty() || needle.len() > hay.len() {
        return found;
    }
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if hay[i..i + needle.len()] == *needle {
            found.push((i, i + needle.len()));
            i += needle.len();
        } else {
            i += 1;
        }
    }
    found
}

fn ctx_measure(canvas: &mut dyn Canvas, s: &str, style: &TextStyle) -> f32 {
    canvas.measure_text(s, style).width
}

/// Quantos caracteres de `text` ficam antes de `x` (a partir do começo do trecho), arredondando
/// para a borda mais perto (busca binária: as larguras dos prefixos crescem).
fn char_at(ui: &Ui, text: &str, style: &TextStyle, x: f32) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let width = |n: usize| ui.measure_text(&chars[..n].iter().collect::<String>(), style).width;
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if width(mid + 1) <= x { lo = mid + 1 } else { hi = mid }
    }
    if lo < chars.len() && x - width(lo) > width(lo + 1) - x {
        return lo + 1;
    }
    lo
}

/// O botão Copiar de um bloco de código (no canto de cima, discreto; "Copiado" depois do clique).
fn draw_copy(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, rect: RectF, hover: bool, copied: bool) {
    let theme = ctx.theme;
    canvas.fill_rect(rect, 4.0, &Brush::Solid(theme.color("surface").with_opacity(if hover { 1.0 } else { 0.85 })));
    canvas.stroke_rect(rect, 4.0, &Stroke::new(theme.color(if hover { "primary" } else { "border" }), 1.0));
    let mut font = ctx.font(ctx.style.font.size * 0.8);
    font.weight = aylis_core::FontWeight::SemiBold;
    let color = if copied { theme.color("success") } else if hover { theme.color("text") } else { theme.color("text_muted") };
    let style = TextStyle { font, color, h_align: aylis_core::TextAlign::Center, v_align: aylis_core::TextAlign::Center, ..Default::default() };
    canvas.draw_text(&aylis_core::messages::t(if copied { "Copiado" } else { "Copiar" }), rect, &style);
}

fn draw_item(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, item: &Item, visible: RectF, image_of: &dyn Fn(&str) -> Option<Image>) {
    let r = item.rect();
    if r.bottom() < visible.y || r.y > visible.bottom() {
        return;
    }
    match item {
        Item::Text { rect, text, style, strike, underline, background, .. } => {
            if let Some(bg) = background {
                canvas.fill_rect(rect.inflate(1.0), 3.0, &Brush::Solid(*bg));
            }
            canvas.draw_text(text, *rect, style);
            let y_mid = rect.y + rect.height / 2.0;
            if *strike {
                canvas.draw_line(PointF::new(rect.x, y_mid), PointF::new(rect.right(), y_mid), &Stroke::new(style.color, 1.0));
            }
            if *underline {
                let y = (y_mid + style.font.size * 0.55).min(rect.bottom() - 1.0);
                canvas.draw_line(PointF::new(rect.x, y), PointF::new(rect.right(), y), &Stroke::new(style.color.with_opacity(0.6), 1.0));
            }
        }
        Item::Fill { rect, color, radius } => canvas.fill_rect(*rect, *radius, &Brush::Solid(*color)),
        Item::Line { from, to, color, width } => canvas.draw_line(*from, *to, &Stroke::new(*color, *width)),
        Item::Check { rect, checked, color } => {
            canvas.stroke_rect(*rect, 3.0, &Stroke::new(*color, 1.5));
            if *checked {
                canvas.fill_rect(rect.inflate(-3.0), 1.5, &Brush::Solid(*color));
            }
        }
        Item::Tab { rect, label, active, .. } => {
            let color = if *active { ctx.theme.color("primary") } else { ctx.theme.color("text_muted") };
            let mut font = ctx.font(ctx.style.font.size * 0.92);
            if *active {
                font.weight = aylis_core::FontWeight::SemiBold;
            }
            let style = TextStyle { font, color, h_align: aylis_core::TextAlign::Center, ..Default::default() };
            canvas.draw_text(label, *rect, &style);
            if *active {
                canvas.fill_rect(RectF::new(rect.x + 4.0, rect.bottom() - 2.0, rect.width - 8.0, 2.0), 1.0, &Brush::Solid(color));
            }
        }
        // Desenhado à parte (`draw_copy`), com o estado do mouse.
        Item::Copy { .. } => {}
        Item::Image { rect, url } => {
            if let Some(image) = image_of(url) {
                if let Some(size) = canvas.image_size(&image) {
                    canvas.draw_image(&image, RectF::new(0.0, 0.0, size.width, size.height), *rect, 1.0);
                }
            }
        }
    }
}

/// O caminho de uma imagem do texto: relativo à página (`source`), como num site; um endereço
/// completo, ou sem página, fica como está.
fn resolve(base: &str, url: &str) -> String {
    if base.is_empty() || url.contains("://") || url.starts_with('/') {
        return url.to_string();
    }
    let mut parts: Vec<&str> = base.rsplit_once('/').map_or(Vec::new(), |(dir, _)| dir.split('/').collect());
    for part in url.split('/') {
        match part {
            ".." => {
                parts.pop();
            }
            "." | "" => {}
            p => parts.push(p),
        }
    }
    parts.join("/")
}

/// Registra `Markdown` (paleta "Additional").
pub fn register(ui: &mut Ui) {
    ui.register_class::<Markdown>("Additional");
}

#[cfg(test)]
mod tests {
    use super::{fold, occurrences};

    #[test]
    fn occurrences_ignore_case_and_count_characters() {
        assert_eq!(occurrences("O Aylis e o aylis", &fold("AYLIS")), vec![(2, 7), (12, 17)]);
        assert_eq!(occurrences("Ação AÇÃO", &fold("ação")), vec![(0, 4), (5, 9)]);
        assert!(occurrences("curto", &fold("mais longo")).is_empty());
        assert!(occurrences("texto", &[]).is_empty());
    }
}

#[cfg(test)]
mod editor_look_tests {
    use super::*;

    /// As propriedades `editor_look`, `code_theme` e `code_language` montam o `editor_code`.
    #[test]
    fn the_editor_look_comes_from_the_properties() {
        let mut m = Markdown::default();
        m.code_language = "rust".into();
        m.sync("code_language");
        assert!(m.editor_code.is_none(), "sem o editor_look, como numa página");
        m.editor_look = true;
        m.sync("editor_look");
        assert_eq!(m.editor_code.as_ref().map(|e| e.language.as_str()), Some("rust"));
        m.editor_look = false;
        m.sync("editor_look");
        assert!(m.editor_code.is_none());
    }
}
