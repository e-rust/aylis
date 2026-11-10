//! EmojiPicker: o seletor de emoji (como o painel de emoji do teclado ou o do WhatsApp).
//! Busca em português ou inglês, categorias, grade rolável e os usados por último.

use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

use aylis_core::{
    Brush, Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, PointF, PointerButton, PointerEvent,
    RectF, Stroke, TextAlign, WheelEvent, properties,
};

use crate::leaf_control;

/// Um emoji da lista (`data/emoji.tsv` deste crate, gerada por `aylis-tool emoji`; dentro do
/// crate para ir junto no pacote do crates.io).
pub struct EmojiInfo {
    pub group: usize,
    pub emoji: &'static str,
    /// Nome em português (o do CLDR).
    pub name: &'static str,
    /// Palavras de busca (português e inglês), já sem acento e em minúsculas.
    words: String,
}

const DATA: &str = include_str!("../data/emoji.tsv");

/// Os grupos (categorias), na ordem.
pub fn emoji_groups() -> &'static [&'static str] {
    static GROUPS: OnceLock<Vec<&'static str>> = OnceLock::new();
    GROUPS.get_or_init(|| DATA.lines().next().and_then(|l| l.strip_prefix("# grupos: ")).map(|g| g.split(';').collect()).unwrap_or_default())
}

/// Todos os emojis, na ordem do Unicode.
pub fn emoji_list() -> &'static [EmojiInfo] {
    static LIST: OnceLock<Vec<EmojiInfo>> = OnceLock::new();
    LIST.get_or_init(|| {
        DATA.lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let mut p = l.split('\t');
                let group = p.next()?.parse().ok()?;
                let emoji = p.next()?;
                let name = p.next()?;
                let words = crate::autocomplete::fold(&format!("{name} {}", p.next().unwrap_or("")));
                Some(EmojiInfo { group, emoji, name, words })
            })
            .collect()
    })
}

/// O ícone de cada categoria na barra (o "recentes" é o primeiro).
const GROUP_ICONS: [&str; 10] = ["🕘", "😀", "🧑", "🐻", "🍔", "✈️", "⚽", "💡", "❤️", "🏁"];

const SEARCH: f32 = 40.0;
const BAR: f32 = 40.0;
const CELL: f32 = 44.0;
const TITLE: f32 = 26.0;

/// Seletor de emoji. Tocar num emoji dispara `on_select` (o emoji em `EventArgs::Text`) e o
/// põe nos recentes (`recent`, separados por espaço; o app pode guardar e devolver).
pub struct EmojiPicker {
    pub base: ControlBase,
    /// A busca (o campo de cima; também dá para mudar pelo código).
    pub filter: String,
    pub recent: String,
    /// Quantos recentes guardar.
    pub max_recent: i64,
    scroll: f32,
    hover: Cell<Option<usize>>,
    /// Da última pintura: (retângulo, emoji) das células e (retângulo, seção) da barra.
    cells: RefCell<Vec<(RectF, &'static str)>>,
    tabs: RefCell<Vec<(RectF, usize)>>,
    /// Onde cada seção começa (no conteúdo rolável) e a altura total.
    sections: RefCell<(Vec<f32>, f32)>,
}

impl Default for EmojiPicker {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 360.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer).set_clip_children(true);
        Self {
            base,
            filter: String::new(),
            recent: String::new(),
            max_recent: 24,
            scroll: 0.0,
            hover: Cell::new(None),
            cells: RefCell::default(),
            tabs: RefCell::default(),
            sections: RefCell::default(),
        }
    }
}

impl EmojiPicker {
    /// As seções a mostrar: (título, emojis). Com busca, uma seção só com o que combina.
    pub fn sections(&self) -> Vec<(String, Vec<&'static str>)> {
        let list = emoji_list();
        let query = crate::autocomplete::fold(self.filter.trim());
        if !query.is_empty() {
            let words: Vec<&str> = query.split_whitespace().collect();
            let found = list.iter().filter(|e| words.iter().all(|w| e.words.contains(w))).map(|e| e.emoji).collect();
            return vec![(aylis_core::messages::t("Resultados"), found)];
        }
        let mut out = Vec::new();
        let recent: Vec<&'static str> =
            self.recent.split_whitespace().filter_map(|r| list.iter().find(|e| e.emoji == r).map(|e| e.emoji)).collect();
        out.push((aylis_core::messages::t("Recentes"), recent));
        for (g, title) in emoji_groups().iter().enumerate() {
            out.push((aylis_core::messages::t(title), list.iter().filter(|e| e.group == g).map(|e| e.emoji).collect()));
        }
        out
    }

    fn columns(&self) -> usize {
        ((self.base.bounds().width - 8.0) / CELL).floor().max(1.0) as usize
    }

    fn pick(&mut self, ctx: &mut EventCtx<'_>, emoji: &'static str) {
        let mut recent: Vec<&str> = self.recent.split_whitespace().filter(|r| *r != emoji).collect();
        recent.insert(0, emoji);
        recent.truncate(self.max_recent.max(0) as usize);
        self.recent = recent.join(" ");
        ctx.fire("on_select", EventArgs::Text(emoji.to_string()));
        ctx.repaint();
    }

    fn max_scroll(&self) -> f32 {
        let view = self.base.bounds().height - SEARCH - BAR;
        (self.sections.borrow().1 - view).max(0.0)
    }
}

impl Component for EmojiPicker {
    leaf_control!("EmojiPicker");

    properties! {
        filter: String => "Data",
        recent: String => "Data",
        max_recent: i64 => "Behavior";
        events: on_select
    }

    fn accepts_text(&self) -> bool {
        true
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        self.filter.extend(text.chars().filter(|c| !c.is_control()));
        self.scroll = 0.0;
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::Backspace => {
                self.filter.pop();
            }
            Key::Escape => self.filter.clear(),
            _ => return,
        }
        self.scroll = 0.0;
        ctx.set_handled();
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let target = (self.scroll - event.delta.y).clamp(0.0, self.max_scroll());
        if target != self.scroll {
            self.scroll = target;
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.cells.borrow().iter().position(|(r, _)| r.contains(event.position));
        if hit != self.hover.get() {
            self.hover.set(hit);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(None);
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        ctx.request_focus(ctx.id);
        let tab = self.tabs.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, s)| *s);
        if let Some(section) = tab {
            self.filter.clear();
            let start = self.sections.borrow().0.get(section).copied().unwrap_or(0.0);
            self.scroll = start.min(self.max_scroll());
            ctx.repaint();
            return;
        }
        let cell = self.cells.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, e)| *e);
        if let Some(emoji) = cell {
            self.pick(ctx, emoji);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, ctx.style.radius.max(8.0), &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), ctx.style.radius.max(8.0), &Stroke::new(theme.color("border"), 1.0));

        // Busca.
        let search = RectF::new(r.x + 8.0, r.y + 6.0, r.width - 16.0, SEARCH - 10.0);
        canvas.fill_rect(search, search.height / 2.0, &Brush::Solid(theme.color("background")));
        ctx.draw_icon(canvas, "search", RectF::new(search.x + 8.0, search.y + (search.height - 16.0) / 2.0, 16.0, 16.0), theme.color("text_muted"));
        let mut t = ctx.text_style();
        let text_r = RectF::new(search.x + 32.0, search.y, search.width - 40.0, search.height);
        if self.filter.is_empty() {
            t.color = theme.color("text_muted");
            canvas.draw_text(&ctx.ui.tr("Buscar emoji"), text_r, &t);
        } else {
            canvas.draw_text(&self.filter, text_r, &t);
        }
        if ctx.state.focused {
            let w = canvas.measure_text(&self.filter, &ctx.text_style()).width;
            let x = text_r.x + w + 1.0;
            canvas.draw_line(PointF::new(x, search.y + 7.0), PointF::new(x, search.bottom() - 7.0), &Stroke::new(theme.color("text"), 1.5));
        }

        // Seções: onde cada uma começa (para a barra e a rolagem).
        let sections = self.sections();
        let cols = self.columns();
        let mut starts = Vec::with_capacity(sections.len());
        let mut y = 0.0;
        for (_, items) in &sections {
            starts.push(y);
            if !items.is_empty() {
                y += TITLE + (items.len().div_ceil(cols)) as f32 * CELL;
            }
        }
        *self.sections.borrow_mut() = (starts.clone(), y);

        // Barra das categorias (sem busca).
        let mut tabs = Vec::new();
        let bar = RectF::new(r.x, r.y + SEARCH, r.width, BAR);
        if self.filter.trim().is_empty() {
            let n = GROUP_ICONS.len().min(sections.len());
            let w = (bar.width - 8.0) / n as f32;
            let current = starts.iter().rposition(|&s| s <= self.scroll + 1.0).unwrap_or(0);
            let mut et = ctx.text_style();
            et.h_align = TextAlign::Center;
            et.font.size = 18.0;
            for (i, icon) in GROUP_ICONS.iter().enumerate().take(n) {
                let cell = RectF::new(bar.x + 4.0 + w * i as f32, bar.y + 4.0, w, BAR - 8.0);
                if i == current {
                    canvas.fill_rect(cell.inflate(-2.0), 8.0, &Brush::Solid(theme.color("primary").with_opacity(0.15)));
                }
                canvas.draw_text(icon, cell, &et);
                tabs.push((cell, i));
            }
        }
        *self.tabs.borrow_mut() = tabs;
        canvas.draw_line(PointF::new(r.x, bar.bottom()), PointF::new(r.right(), bar.bottom()), &Stroke::new(theme.color("border"), 1.0));

        // Grade.
        let area = RectF::new(r.x + 4.0, bar.bottom(), r.width - 8.0, r.bottom() - bar.bottom());
        canvas.save();
        canvas.clip_rect(area);
        let mut cells = Vec::new();
        let mut title = ctx.text_style();
        title.color = theme.color("text_muted");
        title.font.size = (title.font.size - 1.0).max(10.0);
        let mut et = ctx.text_style();
        et.h_align = TextAlign::Center;
        et.font.size = 28.0;
        let hover = self.hover.get();
        for ((name, items), start) in sections.iter().zip(&starts) {
            if items.is_empty() {
                continue;
            }
            let top = area.y + start - self.scroll;
            let height = TITLE + items.len().div_ceil(cols) as f32 * CELL;
            if top + height < area.y || top > area.bottom() {
                continue; // fora da vista
            }
            canvas.draw_text(name, RectF::new(area.x + 6.0, top, area.width - 12.0, TITLE), &title);
            for (i, emoji) in items.iter().enumerate() {
                let cell = RectF::new(area.x + (i % cols) as f32 * CELL, top + TITLE + (i / cols) as f32 * CELL, CELL, CELL);
                if cell.bottom() < area.y || cell.y > area.bottom() {
                    continue;
                }
                if hover == Some(cells.len()) {
                    canvas.fill_rect(cell.inflate(-2.0), 8.0, &Brush::Solid(theme.color("text").with_opacity(0.08)));
                }
                canvas.draw_text(emoji, cell, &et);
                cells.push((cell, *emoji));
            }
        }
        if sections.iter().all(|(_, items)| items.is_empty()) {
            let mut nt = ctx.text_style();
            nt.h_align = TextAlign::Center;
            nt.color = theme.color("text_muted");
            canvas.draw_text(&ctx.ui.tr("Nenhum resultado"), RectF::new(area.x, area.y + 20.0, area.width, 30.0), &nt);
        }
        canvas.restore();
        *self.cells.borrow_mut() = cells;
    }
}
