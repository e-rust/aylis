//! Listas: ListBox, RadioGroup, ListView e TreeView.
//!
//! As listas são virtualizadas: só as linhas visíveis são pintadas, então 100 mil itens
//! custam o mesmo que 10. Os itens vêm de `items` (texto, para o JSON e o Object
//! Inspector) ou da API de cada controle.

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Image, Key, KeyEvent, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, WheelEvent, properties,
};
use aylis_core::automation::{AccessItem, ItemAction, ItemRole};

use crate::leaf_control;

/// Rolagem vertical de uma lista: deslocamento em pixels, com o limite calculado na pintura.
#[derive(Default)]
struct Rows {
    /// A rolagem (limitada ao máximo na pintura: a lista que encolheu não fica com a rolagem
    /// velha, que deslocava o hover e o clique algumas linhas para baixo).
    offset: Cell<f32>,
    /// Altura visível e o máximo de rolagem (da última pintura).
    view: Cell<f32>,
    max: Cell<f32>,
    /// A área das linhas na última pintura (para achar a barra no clique).
    area: Cell<RectF>,
    /// Arrastando a barra: onde ela foi pega (distância do topo dela).
    drag: Option<f32>,
}

/// Largura da faixa da barra que aceita o clique (a barra pintada é mais fina).
const BAR_HIT: f32 = 12.0;

impl Rows {
    /// A barra de rolagem (topo e altura), se há o que rolar.
    fn bar(&self) -> Option<(f32, f32)> {
        let (view, max, area) = (self.view.get(), self.max.get(), self.area.get());
        if max <= 0.5 || view <= 0.0 {
            return None;
        }
        let h = (view * view / (view + max)).max(24.0);
        Some((area.y + (view - h) * (self.offset.get() / max).clamp(0.0, 1.0), h))
    }

    /// Clique na faixa da barra: pega a barra (fora dela, a barra pula para o clique). `true`
    /// se o clique era da barra.
    fn bar_down(&mut self, at: PointF) -> bool {
        let Some((top, h)) = self.bar() else { return false };
        if at.x < self.area.get().right() - BAR_HIT {
            return false;
        }
        let grab = if at.y >= top && at.y <= top + h { at.y - top } else { h / 2.0 };
        self.drag = Some(grab);
        self.bar_move(at);
        true
    }

    /// Arrastando a barra: rola junto. `true` se estava arrastando.
    fn bar_move(&mut self, at: PointF) -> bool {
        let (Some(grab), Some((_, h))) = (self.drag, self.bar()) else { return false };
        let (area, view, max) = (self.area.get(), self.view.get(), self.max.get());
        let room = (view - h).max(1.0);
        self.offset.set(((at.y - grab - area.y) / room).clamp(0.0, 1.0) * max);
        true
    }

    fn bar_up(&mut self) -> bool {
        self.drag.take().is_some()
    }

    /// Pinta a barrinha de rolagem (só quando há o que rolar).
    fn paint_bar(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF) {
        self.area.set(area);
        let Some((y, h)) = self.bar() else { return };
        let bar = RectF::new(area.right() - 5.0, y + 2.0, 3.0, h - 4.0);
        let alpha = if self.drag.is_some() { 0.7 } else { 0.45 };
        canvas.fill_rect(bar, 1.5, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(alpha)));
    }

    fn scroll(&mut self, event: &WheelEvent, row: f32) -> bool {
        let dy = if event.touch { event.delta.y } else { event.delta.y.signum() * row * 3.0 };
        let next = (self.offset.get() - dy).clamp(0.0, self.max.get());
        if (next - self.offset.get()).abs() < 0.5 {
            return false; // no limite: a roda passa para quem está fora
        }
        self.offset.set(next);
        true
    }

    /// Rola para a linha `index` ficar visível.
    fn reveal(&mut self, index: usize, row: f32) {
        let top = index as f32 * row;
        let view = self.view.get();
        if top < self.offset.get() {
            self.offset.set(top);
        } else if view > 0.0 && top + row > self.offset.get() + view {
            self.offset.set(top + row - view);
        }
    }

    fn update(&self, count: usize, row: f32, view: f32) -> f32 {
        self.view.set(view);
        let max = (count as f32 * row - view).max(0.0);
        self.max.set(max);
        // Gravada, não só devolvida: o mouse e o clique contam a linha com ela.
        let offset = self.offset.get().min(max);
        self.offset.set(offset);
        offset
    }

    fn page(&self, row: f32) -> usize {
        ((self.view.get() / row) as usize).max(1)
    }
}

fn split_items(items: &str) -> Vec<String> {
    // `;` ou quebra de linha.
    if items.is_empty() { Vec::new() } else { items.split([';', '\n']).map(|s| s.trim_end_matches('\r').to_string()).collect() }
}

/// Moldura padrão (fundo e borda do estilo).
fn frame(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) -> RectF {
    let s = ctx.style;
    canvas.fill_rect(ctx.rect, s.radius, &Brush::Solid(s.fill));
    if s.border_width > 0.0 && !s.border_color.is_transparent() {
        canvas.stroke_rect(ctx.rect, s.radius, &Stroke::new(s.border_color, s.border_width));
    }
    ctx.rect.inflate(-ctx.style.border_width.max(1.0))
}

/// Navegação por teclado comum: devolve o novo índice.
fn key_index(key: Key, current: i64, count: usize, page: usize) -> Option<usize> {
    if count == 0 {
        return None;
    }
    let last = count as i64 - 1;
    let i = match key {
        Key::Up => (current - 1).max(0),
        Key::Down => (current + 1).min(last),
        Key::Home => 0,
        Key::End => last,
        Key::PageUp => (current - page as i64).max(0),
        Key::PageDown => (current + page as i64).min(last),
        _ => return None,
    };
    Some(i.max(0) as usize)
}

// ---------------------------------------------------------------------------
// ListBox

/// Lista de opções: `items` separados por `;`, `item_index`
/// escolhido. Setas, Home/End, PageUp/PageDown e a roda navegam; `on_change` traz o texto
/// e `on_dbl_click` funciona como sempre.
///
/// Com `multi_select` (no modo estendido do Windows): o clique
/// escolhe um, Ctrl+clique marca/desmarca, Shift+clique e Shift+setas estendem, Ctrl+A
/// seleciona tudo e Ctrl+Espaço alterna o atual. `selected` guarda os índices ("1;4;5") e
/// `on_change` traz os textos selecionados separados por `;`. `item_index` é o item atual.
pub struct ListBox {
    pub base: ControlBase,
    /// Os itens, separados por `;`.
    pub items: String,
    /// O item escolhido (o atual, com `multi_select`), a partir de 0; -1: nenhum.
    pub item_index: i64,
    /// Altura de cada linha, em pontos.
    pub item_height: f32,
    /// Escolhe vários (Ctrl+clique, Shift+clique, Ctrl+A); os escolhidos ficam em `selected`.
    pub multi_select: bool,
    /// Os índices escolhidos com `multi_select`, separados por `;` (`"1;4;5"`); vazio: nenhum.
    pub selected: String,
    list: Vec<String>,
    marks: Vec<bool>,
    anchor: Option<usize>,
    rows: Rows,
    hovered: Option<usize>,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

impl Default for ListBox {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 160.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { overscroll: aylis_core::Overscroll::Auto,
            base,
            items: String::new(),
            item_index: -1,
            item_height: 28.0,
            multi_select: false,
            selected: String::new(),
            list: Vec::new(),
            marks: Vec::new(),
            anchor: None,
            rows: Rows::default(),
            hovered: None,
        }
    }
}

impl ListBox {
    pub fn new(items: &[&str]) -> Self {
        let mut l = Self::default();
        l.set_items(&items.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        l
    }

    /// Troca os itens (pela API, sem o limite de usar `;` no texto).
    pub fn set_items(&mut self, items: &[String]) {
        self.list = items.to_vec();
        self.items = items.join(";");
        if self.item_index >= self.list.len() as i64 {
            self.item_index = -1;
        }
        self.marks.resize(self.list.len(), false);
        self.store_selected();
    }

    pub fn add_item(&mut self, item: &str) {
        self.list.push(item.to_string());
        self.items = self.list.join(";");
        self.marks.push(false);
    }

    /// Os índices selecionados (com `multi_select`; sem ele, o `item_index`).
    pub fn selected_indices(&self) -> Vec<usize> {
        if !self.multi_select {
            return usize::try_from(self.item_index).ok().filter(|&i| i < self.list.len()).into_iter().collect();
        }
        self.marks.iter().enumerate().filter(|(_, m)| **m).map(|(i, _)| i).collect()
    }

    /// Os textos selecionados.
    pub fn selected_texts(&self) -> Vec<String> {
        self.selected_indices().into_iter().filter_map(|i| self.list.get(i).cloned()).collect()
    }

    pub fn is_selected(&self, index: usize) -> bool {
        if self.multi_select { self.marks.get(index).copied().unwrap_or(false) } else { index as i64 == self.item_index }
    }

    /// Marca ou desmarca um item (com `multi_select`).
    pub fn set_selected(&mut self, index: usize, on: bool) {
        if let Some(m) = self.marks.get_mut(index) {
            *m = on;
            self.store_selected();
        }
    }

    pub fn select_all(&mut self) {
        self.marks.iter_mut().for_each(|m| *m = true);
        self.store_selected();
    }

    pub fn clear_selection(&mut self) {
        self.marks.iter_mut().for_each(|m| *m = false);
        self.store_selected();
    }

    fn store_selected(&mut self) {
        self.selected = self.marks.iter().enumerate().filter(|(_, m)| **m).map(|(i, _)| i.to_string()).collect::<Vec<_>>().join(";");
    }

    pub fn item_list(&self) -> &[String] {
        &self.list
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    /// Texto do item escolhido.
    pub fn text(&self) -> &str {
        usize::try_from(self.item_index).ok().and_then(|i| self.list.get(i)).map_or("", String::as_str)
    }

    fn sync(&mut self, name: &str) {
        if name == "items" {
            self.list = split_items(&self.items);
        }
        if self.item_index >= self.list.len() as i64 {
            self.item_index = -1;
        }
        self.marks.resize(self.list.len(), false);
        if name == "selected" {
            self.marks.iter_mut().for_each(|m| *m = false);
            for i in self.selected.split(';').filter_map(|s| s.trim().parse::<usize>().ok()) {
                if let Some(m) = self.marks.get_mut(i) {
                    *m = true;
                }
            }
            self.store_selected();
        }
    }

    /// Seleção múltipla: `toggle` (Ctrl) alterna; `extend` (Shift) vai da âncora até `index`.
    fn select_multi(&mut self, ctx: &mut EventCtx<'_>, index: usize, toggle: bool, extend: bool) {
        if extend {
            let anchor = self.anchor.unwrap_or(index);
            if !toggle {
                self.marks.iter_mut().for_each(|m| *m = false);
            }
            for i in anchor.min(index)..=anchor.max(index) {
                self.marks[i] = true;
            }
        } else if toggle {
            self.marks[index] = !self.marks[index];
            self.anchor = Some(index);
        } else {
            self.marks.iter_mut().for_each(|m| *m = false);
            self.marks[index] = true;
            self.anchor = Some(index);
        }
        self.item_index = index as i64;
        self.store_selected();
        ctx.fire("on_change", EventArgs::Text(self.selected_texts().join(";")));
        let row = self.row();
        self.rows.reveal(index, row);
        ctx.repaint();
    }

    fn row(&self) -> f32 {
        self.item_height.max(12.0)
    }

    fn index_at(&self, y: f32) -> Option<usize> {
        let i = ((y - 1.0 + self.rows.offset.get()) / self.row()) as usize;
        (y >= 0.0 && i < self.list.len()).then_some(i)
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if index as i64 != self.item_index {
            self.item_index = index as i64;
            ctx.fire("on_change", EventArgs::Text(self.text().to_string()));
        }
        let row = self.row();
        self.rows.reveal(index, row);
        ctx.repaint();
    }
}

impl Component for ListBox {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(if self.multi_select { self.selected_texts().join(";") } else { self.text().to_string() }))
    }

    leaf_control!("ListBox");

    // Arrastar da lista leva o item escolhido (o texto), sem handler.
    fn drag_item(&self, _ui: &aylis_core::Ui) -> Option<(String, String)> {
        let item = self.item_list().get(usize::try_from(self.item_index).ok()?)?.clone();
        Some((item.clone(), item))
    }

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        items: String => "Data",
        item_index: i64 => "Data",
        item_height: f32 => "Appearance",
        multi_select: bool => "Behavior",
        selected: String => "Data";
        events: on_change;
        on_set: sync
    }

    // As linhas para leitores de tela e automação (as fora da parte visível, sem posição).
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<AccessItem> {
        let row = self.row();
        let (width, view) = (self.base.bounds().width, self.rows.view.get());
        self.list
            .iter()
            .enumerate()
            .map(|(i, text)| {
                let item = AccessItem::new(ItemRole::ListItem, text.clone()).selected(self.is_selected(i));
                let y = 1.0 + i as f32 * row - self.rows.offset.get();
                if y + row > 0.0 && (view <= 0.0 || y < view) { item.rect(RectF::new(2.0, y, width - 4.0, row)) } else { item }
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: ItemAction) -> bool {
        if index >= self.list.len() {
            return false;
        }
        if self.multi_select {
            self.select_multi(ctx, index, true, false);
        } else {
            self.choose(ctx, index);
        }
        true
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            if self.rows.bar_down(event.position) {
                ctx.repaint();
                return;
            }
            if let Some(i) = self.index_at(event.position.y) {
                if self.multi_select {
                    self.select_multi(ctx, i, event.modifiers.ctrl, event.modifiers.shift);
                } else {
                    self.choose(ctx, i);
                }
            }
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.rows.bar_up() {
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.rows.bar_move(event.position) {
            ctx.repaint();
            return;
        }
        let h = self.index_at(event.position.y);
        if h != self.hovered {
            self.hovered = h;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered = None;
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let row = self.row();
        if self.rows.scroll(event, row) {
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let page = self.rows.page(self.row());
        if self.multi_select && !self.list.is_empty() {
            match event.key {
                Key::Char('a') | Key::Char('A') if event.modifiers.ctrl => {
                    self.select_all();
                    ctx.fire("on_change", EventArgs::Text(self.selected_texts().join(";")));
                    ctx.repaint();
                    ctx.set_handled();
                    return;
                }
                Key::Space | Key::Char(' ') if event.modifiers.ctrl => {
                    let i = usize::try_from(self.item_index).unwrap_or(0).min(self.list.len() - 1);
                    self.select_multi(ctx, i, true, false);
                    ctx.set_handled();
                    return;
                }
                _ => {}
            }
            if let Some(i) = key_index(event.key, self.item_index, self.list.len(), page) {
                self.select_multi(ctx, i, false, event.modifiers.shift);
                ctx.set_handled();
            }
            return;
        }
        if let Some(i) = key_index(event.key, self.item_index, self.list.len(), page) {
            self.choose(ctx, i);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let area = frame(canvas, ctx);
        let row = self.row();
        let offset = self.rows.update(self.list.len(), row, area.height);
        canvas.save();
        canvas.clip_rect(area);
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Leading;
        let first = (offset / row) as usize;
        let theme = ctx.theme;
        for i in first..self.list.len() {
            let y = area.y + i as f32 * row - offset;
            if y > area.bottom() {
                break;
            }
            let line = RectF::new(area.x + 2.0, y, area.width - 4.0, row);
            let selected = self.is_selected(i);
            // Na seleção múltipla, o item atual tem um contorno.
            if self.multi_select && i as i64 == self.item_index && ctx.state.focused {
                canvas.stroke_rect(line.inflate(-1.0), 4.0, &Stroke::new(theme.color("primary"), 1.5));
            }
            if selected {
                let color = if ctx.state.focused { theme.color("primary") } else { theme.color("primary").with_opacity(0.35) };
                canvas.fill_rect(line.inflate(-1.0), 4.0, &Brush::Solid(color));
            } else if Some(i) == self.hovered {
                canvas.fill_rect(line.inflate(-1.0), 4.0, &Brush::Solid(theme.color("hover_overlay")));
            }
            let mut style = text.clone();
            if selected && ctx.state.focused {
                style.color = theme.color("on_primary");
            }
            canvas.draw_text(&self.list[i], RectF::new(line.x + 8.0, line.y, line.width - 16.0, row), &style);
        }
        self.rows.paint_bar(canvas, ctx, area);
        canvas.restore();
    }
}

// ---------------------------------------------------------------------------
// RadioGroup

/// Grupo de opções exclusivas com título e moldura. `items`
/// separados por `;`, `item_index` escolhido, `columns` colunas. `on_change` traz o texto.
pub struct RadioGroup {
    pub base: ControlBase,
    /// O título na moldura.
    pub caption: String,
    /// As opções, separadas por `;`.
    pub items: String,
    /// A opção escolhida, a partir de 0; -1: nenhuma.
    pub item_index: i64,
    /// Em quantas colunas as opções se distribuem.
    pub columns: u32,
    focus_index: Cell<usize>,
}

impl Default for RadioGroup {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 120.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, caption: String::new(), items: String::new(), item_index: -1, columns: 1, focus_index: Cell::new(0) }
    }
}

const RADIO_TOP: f32 = 26.0;
const RADIO_ROW: f32 = 28.0;

impl RadioGroup {
    pub fn new(caption: &str, items: &[&str]) -> Self {
        Self { caption: caption.into(), items: items.join(";"), ..Default::default() }
    }

    pub fn text(&self) -> String {
        usize::try_from(self.item_index).ok().and_then(|i| split_items(&self.items).get(i).cloned()).unwrap_or_default()
    }

    fn cell(&self, index: usize) -> RectF {
        let b = self.base.bounds();
        let cols = self.columns.max(1) as usize;
        let w = (b.width - 24.0) / cols as f32;
        let (row, col) = (index / cols, index % cols);
        RectF::new(12.0 + col as f32 * w, RADIO_TOP + row as f32 * RADIO_ROW, w, RADIO_ROW)
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if index as i64 != self.item_index {
            self.item_index = index as i64;
            ctx.fire("on_change", EventArgs::Text(self.text()));
        }
        ctx.repaint();
    }
}

impl Component for RadioGroup {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.text()))
    }

    leaf_control!("RadioGroup");

    properties! {
        caption: String => "Appearance",
        items: String => "Data",
        item_index: i64 => "Data",
        columns: u32 => "Appearance";
        events: on_change
    }

    // As opções para leitores de tela e automação.
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<AccessItem> {
        split_items(&self.items)
            .into_iter()
            .enumerate()
            .map(|(i, t)| AccessItem::new(ItemRole::Radio, t).checked(i as i64 == self.item_index).selected(i as i64 == self.item_index).rect(self.cell(i)))
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: ItemAction) -> bool {
        if index >= split_items(&self.items).len() {
            return false;
        }
        self.choose(ctx, index);
        true
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let count = split_items(&self.items).len();
        if let Some(i) = (0..count).find(|&i| self.cell(i).contains(event.position)) {
            self.choose(ctx, i);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let count = split_items(&self.items).len();
        let i = match event.key {
            Key::Left | Key::Up => (self.item_index - 1).max(0),
            Key::Right | Key::Down => (self.item_index + 1).min(count as i64 - 1),
            _ => return,
        };
        if i >= 0 {
            self.choose(ctx, i as usize);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let theme = ctx.theme;
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Leading;
        paint_group_frame(canvas, ctx, &self.caption);
        let primary = theme.color("primary");
        for (i, item) in split_items(&self.items).iter().enumerate() {
            let cell = self.cell(i);
            let dot = RectF::new(cell.x, cell.y + (RADIO_ROW - 18.0) / 2.0, 18.0, 18.0);
            let on = i as i64 == self.item_index;
            canvas.fill_ellipse(dot, &Brush::Solid(theme.color("surface")));
            canvas.stroke_ellipse(dot, &Stroke::new(if on { primary } else { theme.color("text_muted") }, 1.5));
            if on {
                canvas.fill_ellipse(dot.inflate(-4.5), &Brush::Solid(primary));
            }
            if ctx.state.focused && (on || (self.item_index < 0 && i == self.focus_index.get())) {
                canvas.stroke_ellipse(dot.inflate(2.0), &Stroke::new(theme.color("focus_ring"), 1.5));
            }
            canvas.draw_text(item, RectF::new(dot.right() + 8.0, cell.y, cell.width - 30.0, RADIO_ROW), &text);
        }
        let _ = s;
    }
}

/// Moldura com o título no alto (GroupBox, RadioGroup).
pub(crate) fn paint_group_frame(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, caption: &str) {
    let (caption, at) = crate::mnemonic::caption(caption);
    let caption = caption.as_str();
    let s = ctx.style;
    let r = ctx.rect;
    let top = 10.0;
    let frame = RectF::new(r.x + 0.5, r.y + top, r.width - 1.0, r.height - top - 0.5);
    if !s.fill.is_transparent() {
        canvas.fill_rect(frame, s.radius, &Brush::Solid(s.fill));
    }
    let mut text = ctx.text_style();
    text.h_align = TextAlign::Leading;
    text.font.size = (text.font.size - 1.0).max(9.0);
    text.font.weight = aylis_core::FontWeight::Bold;
    let width = if caption.is_empty() { 0.0 } else { canvas.measure_text(caption, &text).width + 12.0 };
    let stroke = Stroke::new(s.border_color, s.border_width.max(1.0));
    if width > 0.0 {
        // A borda é desenhada em três recortes, deixando o vão do título.
        let gap = (r.x + 10.0, r.x + 10.0 + width);
        for clip in [
            RectF::new(r.x, r.y, gap.0 - r.x, r.height),
            RectF::new(gap.1, r.y, r.right() - gap.1, r.height),
            RectF::new(r.x, r.y + 20.0, r.width, r.height - 20.0),
        ] {
            canvas.save();
            canvas.clip_rect(clip);
            canvas.stroke_rect(frame, s.radius, &stroke);
            canvas.restore();
        }
        crate::mnemonic::draw_text(canvas, caption, at, RectF::new(r.x + 16.0, r.y, width, 20.0), &text);
    } else {
        canvas.stroke_rect(frame, s.radius, &stroke);
    }
}

// ---------------------------------------------------------------------------
// ListView

/// Um item do ListView.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListItem {
    pub text: String,
    /// Texto menor embaixo (`detail`).
    pub detail: String,
    pub image: Option<Image>,
}

impl ListItem {
    pub fn new(text: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { text: text.into(), detail: detail.into(), image: None }
    }
}

/// Lista de itens com título, detalhe e imagem opcional,
/// pensada também para o celular (linhas altas, rolagem com o dedo). `items` no texto:
/// `Título|detalhe;Outro|mais`. `on_change` traz o índice; `on_item_click` também.
pub struct ListView {
    pub base: ControlBase,
    /// Os itens, separados por `;`, cada um `Título|detalhe` (o detalhe é opcional).
    pub items: String,
    /// O item escolhido, a partir de 0; -1: nenhum.
    pub item_index: i64,
    /// Altura de cada linha, em pontos.
    pub item_height: f32,
    /// Seta › à direita de cada item (navegação).
    pub accessory: bool,
    list: Vec<ListItem>,
    rows: Rows,
    hovered: Option<usize>,
}

impl Default for ListView {
    fn default() -> Self {
        let mut base = ControlBase::new(280.0, 300.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: String::new(),
            item_index: -1,
            item_height: 56.0,
            accessory: false,
            list: Vec::new(),
            rows: Rows::default(),
            hovered: None,
        }
    }
}

impl ListView {
    pub fn set_items(&mut self, items: Vec<ListItem>) {
        self.items = items.iter().map(|i| if i.detail.is_empty() { i.text.clone() } else { format!("{}|{}", i.text, i.detail) }).collect::<Vec<_>>().join(";");
        self.list = items;
        if self.item_index >= self.list.len() as i64 {
            self.item_index = -1;
        }
    }

    pub fn add(&mut self, item: ListItem) {
        self.list.push(item);
        let all = std::mem::take(&mut self.list);
        self.set_items(all);
    }

    pub fn item(&self, index: usize) -> Option<&ListItem> {
        self.list.get(index)
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    fn sync(&mut self, name: &str) {
        if name == "items" {
            self.list = split_items(&self.items)
                .into_iter()
                .map(|s| match s.split_once('|') {
                    Some((t, d)) => ListItem::new(t, d),
                    None => ListItem::new(s, ""),
                })
                .collect();
        }
        if self.item_index >= self.list.len() as i64 {
            self.item_index = -1;
        }
    }

    fn row(&self) -> f32 {
        self.item_height.max(20.0)
    }

    fn index_at(&self, y: f32) -> Option<usize> {
        let i = ((y - 1.0 + self.rows.offset.get()) / self.row()) as usize;
        (y >= 0.0 && i < self.list.len()).then_some(i)
    }

    fn choose(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if index as i64 != self.item_index {
            self.item_index = index as i64;
            ctx.fire("on_change", EventArgs::Value((index as i64).into()));
        }
        let row = self.row();
        self.rows.reveal(index, row);
        ctx.repaint();
    }
}

impl Component for ListView {
    leaf_control!("ListView");

    properties! {
        items: String => "Data",
        item_index: i64 => "Data",
        item_height: f32 => "Appearance",
        accessory: bool => "Appearance";
        events: on_change, on_item_click;
        on_set: sync
    }

    // Os itens (texto e detalhe) para leitores de tela e automação; clicar escolhe e dispara
    // o `on_item_click`, como o toque.
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<AccessItem> {
        let row = self.row();
        let width = self.base.bounds().width;
        self.list
            .iter()
            .enumerate()
            .map(|(i, it)| {
                let name = if it.detail.is_empty() { it.text.clone() } else { format!("{}, {}", it.text, it.detail) };
                AccessItem::new(ItemRole::ListItem, name)
                    .selected(i as i64 == self.item_index)
                    .rect(RectF::new(0.0, 1.0 + i as f32 * row - self.rows.offset.get(), width, row))
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, action: ItemAction) -> bool {
        if index >= self.list.len() {
            return false;
        }
        self.choose(ctx, index);
        if action == ItemAction::Click {
            ctx.fire("on_item_click", EventArgs::Value((index as i64).into()));
        }
        true
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.item_index >= 0 {
            ctx.fire("on_item_click", EventArgs::Value(self.item_index.into()));
        }
        ctx.fire("on_click", EventArgs::None);
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            if self.rows.bar_down(event.position) {
                ctx.repaint();
                return;
            }
            if let Some(i) = self.index_at(event.position.y) {
                self.choose(ctx, i);
            }
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.rows.bar_up() {
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.rows.bar_move(event.position) {
            ctx.repaint();
            return;
        }
        let h = self.index_at(event.position.y);
        if h != self.hovered {
            self.hovered = h;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered = None;
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let row = self.row();
        if self.rows.scroll(event, row) {
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Enter && self.item_index >= 0 {
            ctx.fire("on_item_click", EventArgs::Value(self.item_index.into()));
            return;
        }
        let page = self.rows.page(self.row());
        if let Some(i) = key_index(event.key, self.item_index, self.list.len(), page) {
            self.choose(ctx, i);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let area = frame(canvas, ctx);
        let row = self.row();
        let offset = self.rows.update(self.list.len(), row, area.height);
        let theme = ctx.theme;
        canvas.save();
        canvas.clip_rect(area);
        let mut title = ctx.text_style();
        title.h_align = TextAlign::Leading;
        let mut detail = title.clone();
        detail.font.size = (title.font.size - 2.0).max(9.0);
        detail.color = theme.color("text_muted");
        let first = (offset / row) as usize;
        let separator = Stroke::new(theme.color("border"), 1.0);
        for i in first..self.list.len() {
            let y = area.y + i as f32 * row - offset;
            if y > area.bottom() {
                break;
            }
            let item = &self.list[i];
            let line = RectF::new(area.x, y, area.width, row);
            if i as i64 == self.item_index {
                canvas.fill_rect(line, 0.0, &Brush::Solid(theme.color("primary").with_opacity(if ctx.state.focused { 0.18 } else { 0.1 })));
            } else if Some(i) == self.hovered {
                canvas.fill_rect(line, 0.0, &Brush::Solid(theme.color("hover_overlay")));
            }
            let mut left = line.x + 14.0;
            if let Some(image) = &item.image {
                let size = row - 16.0;
                let dest = RectF::new(left, y + 8.0, size, size);
                if let Some(natural) = canvas.image_size(image) {
                    let (src, dest) = aylis_core::ImageFit::Cover.place(natural, dest);
                    canvas.save();
                    canvas.clip_round_rect(RectF::new(left, y + 8.0, size, size), 6.0);
                    canvas.draw_image(image, src, dest, 1.0);
                    canvas.restore();
                }
                left += size + 12.0;
            }
            let right = line.right() - if self.accessory { 32.0 } else { 14.0 };
            if item.detail.is_empty() {
                canvas.draw_text(&item.text, RectF::new(left, y, right - left, row), &title);
            } else {
                let mid = y + row / 2.0;
                canvas.draw_text(&item.text, RectF::new(left, mid - 21.0, right - left, 22.0), &title);
                canvas.draw_text(&item.detail, RectF::new(left, mid + 1.0, right - left, 20.0), &detail);
            }
            if self.accessory {
                let c = PointF::new(line.right() - 18.0, y + row / 2.0);
                let s = Stroke::new(theme.color("text_muted"), 1.6);
                canvas.draw_line(PointF::new(c.x - 3.0, c.y - 6.0), PointF::new(c.x + 3.0, c.y), &s);
                canvas.draw_line(PointF::new(c.x + 3.0, c.y), PointF::new(c.x - 3.0, c.y + 6.0), &s);
            }
            if i + 1 < self.list.len() {
                canvas.draw_line(PointF::new(left, y + row - 0.5), PointF::new(line.right(), y + row - 0.5), &separator);
            }
        }
        self.rows.paint_bar(canvas, ctx, area);
        canvas.restore();
    }
}

// ---------------------------------------------------------------------------
// TreeView

/// Um nó do TreeView.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TreeNode {
    pub text: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    /// Valor livre (um id do banco, um caminho de arquivo).
    pub data: String,
}

impl TreeNode {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn expanded(mut self) -> Self {
        self.expanded = true;
        self
    }
}

/// Árvore, virtualizada: só as linhas visíveis são pintadas.
///
/// `items` no texto: um nó por item (`;`), com `>` na frente para cada nível:
/// `Brasil;>São Paulo;>>Campinas;>Rio de Janeiro;Portugal`. Pela API,
/// [`TreeView::set_nodes`]. Clique no triângulo (ou →/←, Enter) abre e fecha;
/// `on_change` traz o caminho do nó (`Brasil/São Paulo`).
pub struct TreeView {
    pub base: ControlBase,
    /// Os nós, separados por `;`, com um `>` na frente por nível
    /// (`Brasil;>São Paulo;>>Campinas;Portugal`).
    pub items: String,
    /// Altura de cada linha, em pontos.
    pub item_height: f32,
    nodes: Vec<TreeNode>,
    /// Caminho (índices) do nó escolhido.
    selected: Option<Vec<usize>>,
    rows: Rows,
    hovered: Option<usize>,
    /// O nó aberto ou fechado por último (para o `on_expand` saber qual foi).
    toggled: Option<Vec<usize>>,
    /// O nó do último clique com o botão direito (o do menu de contexto).
    context: Option<Vec<usize>>,
    /// O efeito de fim da rolagem no toque (`auto`: o do sistema; veja `aylis_core::overscroll`).
    pub overscroll: aylis_core::Overscroll,
}

const INDENT: f32 = 18.0;

impl Default for TreeView {
    fn default() -> Self {
        let mut base = ControlBase::new(240.0, 300.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { overscroll: aylis_core::Overscroll::Auto, base, items: String::new(), item_height: 26.0, nodes: Vec::new(), selected: None, rows: Rows::default(), hovered: None, toggled: None, context: None }
    }
}

/// Uma linha visível: o caminho do nó e a profundidade.
struct Visible {
    path: Vec<usize>,
    depth: usize,
}

impl TreeView {
    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>) {
        self.nodes = nodes;
        self.selected = None;
        let mut out = Vec::new();
        fn write(nodes: &[TreeNode], depth: usize, out: &mut Vec<String>) {
            for n in nodes {
                out.push(format!("{}{}", ">".repeat(depth), n.text));
                write(&n.children, depth + 1, out);
            }
        }
        write(&self.nodes, 0, &mut out);
        self.items = out.join(";");
    }

    pub fn nodes(&self) -> &[TreeNode] {
        &self.nodes
    }

    pub fn node(&self, path: &[usize]) -> Option<&TreeNode> {
        let (first, rest) = path.split_first()?;
        rest.iter().try_fold(self.nodes.get(*first)?, |n, &i| n.children.get(i))
    }

    fn node_mut(&mut self, path: &[usize]) -> Option<&mut TreeNode> {
        let (first, rest) = path.split_first()?;
        rest.iter().try_fold(self.nodes.get_mut(*first)?, |n, &i| n.children.get_mut(i))
    }

    /// O caminho do nó aberto ou fechado por último (no `on_expand` e no `on_collapse`, o
    /// dele). Com [`set_children`](Self::set_children), carrega os filhos sob demanda.
    pub fn last_toggled(&self) -> Option<Vec<usize>> {
        self.toggled.clone()
    }

    /// O nó do último clique com o botão direito: o menu de contexto (`popup_menu`) age sobre
    /// ele, sem mudar a seleção.
    pub fn context_node(&self) -> Option<&TreeNode> {
        self.context.as_deref().and_then(|p| self.node(p))
    }

    /// Escolhe o nó do caminho sem disparar `on_change` (o estado vindo de fora, como o form
    /// aberto). `None` tira a seleção.
    pub fn set_selected_path(&mut self, path: Option<Vec<usize>>) {
        self.selected = path.filter(|p| self.node(p).is_some());
    }

    /// Troca os filhos do nó do caminho (mantendo-o aberto ou fechado). `false` se não há o nó.
    pub fn set_children(&mut self, path: &[usize], children: Vec<TreeNode>) -> bool {
        let Some(n) = self.node_mut(path) else { return false };
        n.children = children;
        true
    }

    /// O nó escolhido.
    pub fn selected(&self) -> Option<&TreeNode> {
        self.selected.as_deref().and_then(|p| self.node(p))
    }

    /// Caminho de textos do nó escolhido (`Brasil/São Paulo`).
    pub fn selected_path(&self) -> String {
        let Some(path) = &self.selected else { return String::new() };
        (1..=path.len()).filter_map(|n| self.node(&path[..n]).map(|x| x.text.clone())).collect::<Vec<_>>().join("/")
    }

    /// Abre todos os nós disparando o `on_expand` de cada um: um por
    /// vez, de cima para baixo, sem um evento dentro do outro; os filhos que o handler
    /// acrescentar (carregamento sob demanda, com [`set_children`](Self::set_children))
    /// também são abertos. No handler, [`last_toggled`](Self::last_toggled) é o nó da vez.
    pub fn full_expand(ui: &mut aylis_core::Ui, id: aylis_core::ComponentId) {
        Self::toggle_all(ui, id, true);
    }

    /// Fecha todos disparando o `on_collapse` de cada um (os de dentro antes do pai).
    pub fn full_collapse(ui: &mut aylis_core::Ui, id: aylis_core::ComponentId) {
        Self::toggle_all(ui, id, false);
    }

    fn toggle_all(ui: &mut aylis_core::Ui, id: aylis_core::ComponentId, open: bool) {
        // Um nó por volta: o handler pode mudar a árvore entre uma e outra.
        for _ in 0..1_000_000 {
            let Some(tree) = ui.get_mut::<TreeView>(id) else { return };
            let Some(path) = next_to_toggle(&tree.nodes, open) else { break };
            let Some(node) = tree.node_mut(&path) else { break };
            node.expanded = open;
            let text = node.text.clone();
            tree.toggled = Some(path);
            ui.fire(id, if open { "on_expand" } else { "on_collapse" }, &EventArgs::Text(text));
        }
        ui.request_repaint();
    }

    /// Abre ou fecha todos, sem eventos (só mostra).
    pub fn expand_all(&mut self, expanded: bool) {
        fn walk(nodes: &mut [TreeNode], e: bool) {
            for n in nodes {
                n.expanded = e;
                walk(&mut n.children, e);
            }
        }
        walk(&mut self.nodes, expanded);
    }

    fn sync(&mut self, name: &str) {
        if name != "items" {
            return;
        }
        // ">" no começo = um nível abaixo do item anterior de nível menor.
        let mut roots: Vec<TreeNode> = Vec::new();
        let mut stack: Vec<Vec<usize>> = Vec::new();
        for raw in split_items(&self.items) {
            let depth = raw.chars().take_while(|c| *c == '>').count();
            let text = raw[depth..].trim().to_string();
            let depth = depth.min(stack.len());
            stack.truncate(depth);
            let node = TreeNode::new(text);
            let path = match stack.last().cloned() {
                Some(parent) => {
                    let p = parent.clone();
                    let n = self_node_mut(&mut roots, &p).map(|n| {
                        n.children.push(node);
                        n.children.len() - 1
                    });
                    let mut path = parent;
                    path.push(n.unwrap_or(0));
                    path
                }
                None => {
                    roots.push(node);
                    vec![roots.len() - 1]
                }
            };
            stack.push(path);
        }
        self.nodes = roots;
        self.selected = None;
    }

    fn visible(&self) -> Vec<Visible> {
        fn walk(nodes: &[TreeNode], prefix: &mut Vec<usize>, depth: usize, out: &mut Vec<Visible>) {
            for (i, n) in nodes.iter().enumerate() {
                prefix.push(i);
                out.push(Visible { path: prefix.clone(), depth });
                if n.expanded {
                    walk(&n.children, prefix, depth + 1, out);
                }
                prefix.pop();
            }
        }
        let mut out = Vec::new();
        walk(&self.nodes, &mut Vec::new(), 0, &mut out);
        out
    }

    fn row(&self) -> f32 {
        self.item_height.max(14.0)
    }

    fn select_row(&mut self, ctx: &mut EventCtx<'_>, rows: &[Visible], index: usize) {
        let path = rows[index].path.clone();
        if self.selected.as_ref() != Some(&path) {
            self.selected = Some(path);
            ctx.fire("on_change", EventArgs::Text(self.selected_path()));
        }
        let row = self.row();
        self.rows.reveal(index, row);
        ctx.repaint();
    }

    fn toggle(&mut self, ctx: &mut EventCtx<'_>, path: &[usize], open: Option<bool>) {
        self.toggled = Some(path.to_vec());
        if let Some(n) = self.node_mut(path) {
            if !n.children.is_empty() {
                n.expanded = open.unwrap_or(!n.expanded);
                let text = n.text.clone();
                ctx.fire(if n.expanded { "on_expand" } else { "on_collapse" }, EventArgs::Text(text));
            }
        }
        ctx.repaint();
    }
}

/// O próximo nó com filhos a abrir (o primeiro fechado, de cima para baixo) ou a fechar (o
/// primeiro aberto sem filhos abertos: os de dentro antes do pai).
fn next_to_toggle(nodes: &[TreeNode], open: bool) -> Option<Vec<usize>> {
    fn walk(nodes: &[TreeNode], open: bool, prefix: &mut Vec<usize>, found: &mut Option<Vec<usize>>) -> bool {
        for (i, n) in nodes.iter().enumerate() {
            prefix.push(i);
            if open {
                if !n.children.is_empty() && !n.expanded {
                    *found = Some(prefix.clone());
                    return true;
                }
                if walk(&n.children, open, prefix, found) {
                    return true;
                }
            } else {
                if walk(&n.children, open, prefix, found) {
                    return true;
                }
                if !n.children.is_empty() && n.expanded {
                    *found = Some(prefix.clone());
                    return true;
                }
            }
            prefix.pop();
        }
        false
    }
    let mut found = None;
    walk(nodes, open, &mut Vec::new(), &mut found);
    found
}

fn self_node_mut<'a>(nodes: &'a mut [TreeNode], path: &[usize]) -> Option<&'a mut TreeNode> {
    let (first, rest) = path.split_first()?;
    rest.iter().try_fold(nodes.get_mut(*first)?, |n, &i| n.children.get_mut(i))
}

impl Component for TreeView {
    leaf_control!("TreeView");

    properties! {
        overscroll: aylis_core::Overscroll => "Behavior",
        items: String => "Data",
        item_height: f32 => "Appearance";
        events: on_change, on_click, on_dbl_click, on_expand, on_collapse;
        on_set: sync
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::MethodInfo;
        const METHODS: &[MethodInfo] = &[MethodInfo::new("full_expand", &[], None), MethodInfo::new("full_collapse", &[], None)];
        METHODS
    }

    // Para as outras linguagens (`invoke`): a árvore está emprestada aqui e cada nó dispara o
    // evento dele, então abre (ou fecha) logo depois, pela fila.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        let open = match method {
            "full_expand" => true,
            "full_collapse" => false,
            _ => return Err(format!("TreeView não tem o método {method}")),
        };
        let id = ctx.id;
        ctx.ui.poster().post(move |ui| TreeView::toggle_all(ui, id, open));
        Ok(aylis_core::PropValue::Null)
    }

    // Os nós visíveis para leitores de tela e automação: nível, aberto/fechado e o escolhido.
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<AccessItem> {
        let row = self.row();
        let width = self.base.bounds().width;
        self.visible()
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let n = self.node(&v.path)?;
                let mut item = AccessItem::new(ItemRole::TreeItem, n.text.clone())
                    .level(v.depth)
                    .selected(self.selected.as_ref() == Some(&v.path))
                    .rect(RectF::new(0.0, 1.0 + i as f32 * row - self.rows.offset.get(), width, row));
                if !n.children.is_empty() {
                    item = item.expanded(n.expanded);
                }
                Some(item)
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, action: ItemAction) -> bool {
        let rows = self.visible();
        let Some(path) = rows.get(index).map(|r| r.path.clone()) else { return false };
        match action {
            ItemAction::Expand => self.toggle(ctx, &path, Some(true)),
            ItemAction::Collapse => self.toggle(ctx, &path, Some(false)),
            ItemAction::Click | ItemAction::Select => self.select_row(ctx, &rows, index),
        }
        true
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            // O botão direito marca o nó do menu de contexto.
            let rows = self.visible();
            let index = ((event.position.y - 1.0 + self.rows.offset.get()) / self.row()) as usize;
            self.context = (event.position.y >= 0.0).then(|| rows.get(index).map(|r| r.path.clone())).flatten();
            return;
        }
        if self.rows.bar_down(event.position) {
            ctx.repaint();
            return;
        }
        let rows = self.visible();
        let index = ((event.position.y - 1.0 + self.rows.offset.get()) / self.row()) as usize;
        if event.position.y < 0.0 || index >= rows.len() {
            return;
        }
        let toggle_x = 6.0 + rows[index].depth as f32 * INDENT;
        let path = rows[index].path.clone();
        let on_arrow = event.position.x >= toggle_x && event.position.x < toggle_x + INDENT;
        if on_arrow || event.click_count == 2 {
            self.toggle(ctx, &path, None);
        }
        self.select_row(ctx, &rows, index);
        // O clique num nó (também no já escolhido, que não dá `on_change`) e o duplo clique: quem
        // abre o que o nó aponta (a pesquisa no projeto) responde a cada um (conferência de 08/11:
        // mais de 30 cliques num resultado sem nada acontecer).
        if !on_arrow {
            ctx.fire("on_click", EventArgs::Text(self.selected_path()));
            if event.click_count == 2 {
                ctx.fire("on_dbl_click", EventArgs::Text(self.selected_path()));
            }
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.rows.bar_up() {
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.rows.bar_move(event.position) {
            ctx.repaint();
            return;
        }
        let index = ((event.position.y - 1.0 + self.rows.offset.get()) / self.row()) as usize;
        let h = (event.position.y >= 0.0 && index < self.visible().len()).then_some(index);
        if h != self.hovered {
            self.hovered = h;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered = None;
        ctx.repaint();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let row = self.row();
        if self.rows.scroll(event, row) {
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let rows = self.visible();
        let current = self.selected.as_ref().and_then(|p| rows.iter().position(|r| &r.path == p)).map_or(-1, |i| i as i64);
        match event.key {
            Key::Right | Key::Left | Key::Enter if current >= 0 => {
                let path = rows[current as usize].path.clone();
                let open = match event.key {
                    Key::Right => Some(true),
                    Key::Left => Some(false),
                    _ => None,
                };
                let has_children = self.node(&path).is_some_and(|n| !n.children.is_empty());
                let expanded = self.node(&path).is_some_and(|n| n.expanded);
                if event.key == Key::Left && (!has_children || !expanded) && path.len() > 1 {
                    // ← num nó fechado: vai para o pai.
                    let parent = path[..path.len() - 1].to_vec();
                    if let Some(i) = rows.iter().position(|r| r.path == parent) {
                        self.select_row(ctx, &rows, i);
                    }
                } else {
                    self.toggle(ctx, &path, open);
                }
                ctx.set_handled();
            }
            key => {
                let page = self.rows.page(self.row());
                if let Some(i) = key_index(key, current, rows.len(), page) {
                    self.select_row(ctx, &rows, i);
                    ctx.set_handled();
                }
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let area = frame(canvas, ctx);
        let rows = self.visible();
        let row = self.row();
        let offset = self.rows.update(rows.len(), row, area.height);
        let theme = ctx.theme;
        canvas.save();
        canvas.clip_rect(area);
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Leading;
        let first = (offset / row) as usize;
        let arrow = Stroke::new(theme.color("text_muted"), 1.5);
        for (i, r) in rows.iter().enumerate().skip(first) {
            let y = area.y + i as f32 * row - offset;
            if y > area.bottom() {
                break;
            }
            let Some(node) = self.node(&r.path) else { continue };
            let line = RectF::new(area.x + 2.0, y, area.width - 4.0, row);
            let selected = self.selected.as_ref() == Some(&r.path);
            let mut style = text.clone();
            if selected {
                let color = if ctx.state.focused { theme.color("primary") } else { theme.color("primary").with_opacity(0.35) };
                canvas.fill_rect(line.inflate(-1.0), 4.0, &Brush::Solid(color));
                if ctx.state.focused {
                    style.color = theme.color("on_primary");
                }
            } else if Some(i) == self.hovered {
                canvas.fill_rect(line.inflate(-1.0), 4.0, &Brush::Solid(theme.color("hover_overlay")));
            }
            let x = area.x + 6.0 + r.depth as f32 * INDENT;
            if !node.children.is_empty() {
                let c = PointF::new(x + INDENT / 2.0, y + row / 2.0);
                let a = if selected && ctx.state.focused { Stroke::new(theme.color("on_primary"), 1.5) } else { arrow.clone() };
                if node.expanded {
                    canvas.draw_line(PointF::new(c.x - 4.0, c.y - 2.0), PointF::new(c.x, c.y + 2.5), &a);
                    canvas.draw_line(PointF::new(c.x, c.y + 2.5), PointF::new(c.x + 4.0, c.y - 2.0), &a);
                } else {
                    canvas.draw_line(PointF::new(c.x - 2.0, c.y - 4.0), PointF::new(c.x + 2.5, c.y), &a);
                    canvas.draw_line(PointF::new(c.x + 2.5, c.y), PointF::new(c.x - 2.0, c.y + 4.0), &a);
                }
            }
            canvas.draw_text(&node.text, RectF::new(x + INDENT + 2.0, y, line.right() - x - INDENT - 6.0, row), &style);
        }
        self.rows.paint_bar(canvas, ctx, area);
        canvas.restore();
    }
}
