//! Contêineres: TabControl e TabItem (abas), GroupBox, Expander, StatusBar e ToolBar.

use std::cell::{Cell, RefCell};

use aylis_core::{
    Align, Brush, Canvas, Color, GradientStop, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
    PaintCtx, PointF, PointerButton, PointerEvent, PointerKind, RectF, Stroke, TRANSITION_MS, TextAlign, Transition, Ui, WheelEvent, properties,
};
use aylis_core::automation::{AccessItem, ItemAction, ItemRole};

use crate::leaf_control;
use crate::lists::paint_group_frame;

const TAB_BAR: f32 = 38.0;
/// Largura do esmaecido na borda da barra quando há abas escondidas daquele lado.
const FADE: f32 = 28.0;

/// Abas (como o `TTabControl`): os filhos são [`TabItem`]s, cada um uma página; só a de
/// `tab_index` aparece. Clique numa aba (ou Ctrl+Tab com o foco) troca; `on_change`
/// traz o índice. Sem `show_tabs`, só as páginas, sem barra nem moldura (como um
/// `PageControl` com as abas escondidas): a tela tem a própria navegação e troca a página
/// pelo `tab_index`; no designer aparece uma página por vez.
pub struct TabControl {
    pub base: ControlBase,
    pub tab_index: i64,
    pub show_tabs: bool,
    /// Como a aba nova entra (`slide_left`/`slide_right` deslizam na direção da ordem das
    /// abas, `fade` esmaece).
    pub transition: Transition,
    /// Retângulos das abas (da última pintura), para o clique.
    headers: RefCell<Vec<(ComponentId, RectF)>>,
    /// Quanto a barra de abas está rolada (abas que não cabem: o dedo ou a roda rolam).
    scroll: Cell<f32>,
    /// Na próxima pintura, rolar até a aba escolhida (ela mudou).
    reveal: Cell<bool>,
    /// A aba tocada com o dedo, escolhida ao soltar se ele não arrastou.
    pressed: Cell<Option<usize>>,
}

impl Default for TabControl {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 300.0);
        base.set_padding([1.0, TAB_BAR, 1.0, 1.0]).set_can_focus(true).set_clip_children(true);
        Self { base, tab_index: 0, show_tabs: true, transition: Transition::None, headers: RefCell::default(), scroll: Cell::new(0.0), reveal: Cell::new(true), pressed: Cell::new(None) }
    }
}

impl TabControl {
    /// O índice da aba sob `p` (só as que estão à vista na barra).
    fn tab_at(&self, ui: &Ui, id: ComponentId, p: PointF) -> Option<usize> {
        let tab = self.headers.borrow().iter().find(|(_, r)| r.contains(p)).map(|(t, _)| *t)?;
        Self::tabs(ui, id).iter().position(|&t| t == tab)
    }

    /// Os TabItems (as abas), na ordem.
    pub fn tabs(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
        ui.children(id).iter().copied().filter(|&c| ui.get::<TabItem>(c).is_some()).collect()
    }

    /// Acrescenta uma aba e devolve o TabItem (para pôr os controles dentro).
    pub fn add_tab(ui: &mut Ui, id: ComponentId, caption: &str) -> ComponentId {
        let tab = ui.add(TabItem::new(caption), id);
        Self::show(ui, id);
        tab
    }

    /// Mostra a aba `index`.
    pub fn select(ui: &mut Ui, id: ComponentId, index: usize) {
        let _ = ui.set_prop(id, "tab_index", index as i64);
        Self::show(ui, id);
    }

    /// Deixa visível só a aba escolhida (depois de mudar as abas pelo código).
    pub fn show(ui: &mut Ui, id: ComponentId) {
        let index = ui.get::<TabControl>(id).map_or(0, |t| t.tab_index);
        let tabs = Self::tabs(ui, id);
        // Trocando com transição: ela mesma esconde a aba antiga no fim.
        if tabs.iter().any(|&t| ui.in_transition(t)) {
            return;
        }
        apply_visibility(ui, &tabs, index);
    }

    fn change(&mut self, ctx: &mut EventCtx<'_>, index: i64) {
        let tabs = Self::tabs(ctx.ui, ctx.id);
        if tabs.is_empty() {
            return;
        }
        let index = index.rem_euclid(tabs.len() as i64);
        if index != self.tab_index {
            let old = self.tab_index;
            self.tab_index = index;
            switch(ctx.ui, &tabs, old, index, self.transition);
            ctx.fire("on_change", EventArgs::Value(index.into()));
            show_page(ctx.ui, &tabs, index);
        }
        self.reveal.set(true);
        ctx.invalidate();
    }
}

/// Troca da aba `old` para a `new`: com transição, as duas aparecem enquanto uma entra e a
/// outra sai (deslizando, a direção segue a ordem das abas).
fn switch(ui: &mut Ui, tabs: &[ComponentId], old: i64, new: i64, transition: Transition) {
    let (Some(&from), Some(&to)) = (tabs.get(old.max(0) as usize).filter(|_| old >= 0), tabs.get(new.max(0) as usize)) else {
        apply_visibility(ui, tabs, new);
        return;
    };
    if transition == Transition::None || from == to {
        apply_visibility(ui, tabs, new);
        return;
    }
    let kind = match transition {
        Transition::SlideLeft | Transition::SlideRight => {
            if new > old { Transition::SlideLeft } else { Transition::SlideRight }
        }
        other => other,
    };
    for &t in tabs {
        if t != from && t != to {
            if let Some(cb) = ui.control_mut(t) {
                cb.set_visible(false);
            }
        }
    }
    ui.transition(Some(from), Some(to), kind, TRANSITION_MS);
}

fn apply_visibility(ui: &mut Ui, tabs: &[ComponentId], index: i64) {
    for (i, &tab) in tabs.iter().enumerate() {
        if let Some(cb) = ui.control_mut(tab) {
            cb.set_visible(i as i64 == index);
        }
    }
    ui.invalidate();
}

impl Component for TabControl {
    leaf_control!("TabControl");

    properties! {
        tab_index: i64 => "Behavior",
        show_tabs: bool => "Appearance",
        transition: Transition => "Behavior";
        events: on_change
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // Sem a barra, as páginas vão até as bordas.
        if name == "show_tabs" {
            let p = if self.show_tabs { [1.0, TAB_BAR, 1.0, 1.0] } else { [0.0; 4] };
            self.base.set_padding(p);
            ctx.invalidate();
        }
        if name == "tab_index" {
            self.reveal.set(true);
            let tabs = Self::tabs(ctx.ui, ctx.id);
            // A aba que estava à vista (para a transição sair dela).
            let old = tabs.iter().position(|&t| ctx.ui.control(t).is_some_and(|c| c.visible())).map_or(-1, |i| i as i64);
            switch(ctx.ui, &tabs, old, self.tab_index, self.transition);
            if old != self.tab_index {
                show_page(ctx.ui, &tabs, self.tab_index);
            }
        }
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let tabs = Self::tabs(ctx.ui, ctx.id);
        let visible: Vec<bool> = tabs.iter().map(|&t| ctx.ui.control(t).is_some_and(|c| c.visible())).collect();
        let wanted: Vec<bool> = (0..tabs.len()).map(|i| i as i64 == self.tab_index).collect();
        if visible != wanted && !tabs.iter().any(|&t| ctx.ui.in_transition(t)) {
            apply_visibility(ctx.ui, &tabs, self.tab_index);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if !self.show_tabs || event.button != PointerButton::Primary || event.position.y > TAB_BAR {
            return;
        }
        let Some(i) = self.tab_at(ctx.ui, ctx.id, event.position) else { return };
        // Com o dedo, a aba é escolhida ao soltar: o toque pode ser o começo de uma rolagem.
        if event.kind == PointerKind::Touch {
            self.pressed.set(Some(i));
        } else {
            self.change(ctx, i as i64);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(i) = self.pressed.take()
            && self.tab_at(ctx.ui, ctx.id, event.position) == Some(i)
        {
            self.change(ctx, i as i64);
        }
    }

    // Abas que não cabem: o dedo arrastando (ou a roda) sobre a barra rola as abas.
    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if !self.show_tabs || event.position.y > TAB_BAR {
            return;
        }
        let delta = if event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
        let before = self.scroll.get();
        // A pintura limita ao tamanho da barra (a largura das abas é medida lá).
        self.scroll.set((before - delta).max(0.0));
        self.reveal.set(false);
        // O dedo arrastou: não foi um toque na aba.
        self.pressed.set(None);
        ctx.repaint();
        ctx.set_handled();
    }

    // As abas para leitores de tela e automação: o título, onde está e qual está à vista.
    fn access_items(&self, ui: &Ui, id: ComponentId) -> Vec<AccessItem> {
        let headers = self.headers.borrow();
        TabControl::tabs(ui, id)
            .iter()
            .enumerate()
            .map(|(i, &tab)| {
                let caption = ui.get::<TabItem>(tab).map(|t| t.caption.clone()).unwrap_or_default();
                let mut item = AccessItem::new(ItemRole::Tab, caption).selected(i as i64 == self.tab_index);
                if let Some((_, r)) = headers.iter().find(|(t, _)| *t == tab) {
                    item = item.rect(*r);
                }
                item
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: ItemAction) -> bool {
        if index >= TabControl::tabs(ctx.ui, ctx.id).len() {
            return false;
        }
        self.change(ctx, index as i64);
        true
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let delta = match event.key {
            Key::Tab if event.modifiers.ctrl => if event.modifiers.shift { -1 } else { 1 },
            Key::Left => -1,
            Key::Right => 1,
            _ => return,
        };
        self.change(ctx, self.tab_index + delta);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !self.show_tabs {
            self.headers.borrow_mut().clear();
            return;
        }
        let s = ctx.style;
        let r = ctx.rect;
        let theme = ctx.theme;
        let body = RectF::new(r.x, r.y + TAB_BAR - 1.0, r.width, r.height - TAB_BAR + 1.0);
        canvas.fill_rect(body, s.radius, &Brush::Solid(s.fill));
        canvas.stroke_rect(body, s.radius, &Stroke::new(s.border_color, s.border_width.max(1.0)));
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        let tabs = TabControl::tabs(ctx.ui, ctx.id);
        let captions: Vec<String> = tabs.iter().map(|&t| ctx.ui.get::<TabItem>(t).map(|t| t.caption.clone()).unwrap_or_default()).collect();
        let widths: Vec<f32> = captions.iter().map(|c| canvas.measure_text(c, &text).width + 28.0).collect();
        // Abas que não cabem: a barra rola (até a escolhida, quando ela muda).
        let total: f32 = widths.iter().sum::<f32>() + 8.0;
        let max_scroll = (total - r.width).max(0.0);
        let mut scroll = self.scroll.get().min(max_scroll);
        if self.reveal.replace(false) {
            let index = (self.tab_index.max(0) as usize).min(widths.len());
            let left: f32 = widths[..index].iter().sum::<f32>() + 4.0;
            let right = left + widths.get(index).copied().unwrap_or(0.0);
            if left - 4.0 < scroll {
                scroll = (left - 4.0).max(0.0);
            } else if right + 4.0 > scroll + r.width {
                scroll = (right + 4.0 - r.width).min(max_scroll);
            }
        }
        self.scroll.set(scroll);
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, r.y, r.width, TAB_BAR));
        let mut headers = Vec::new();
        let mut x = r.x + 4.0 - scroll;
        // A aba pelo estilo `TabControl.tab` do tema, quando houver: normal, sob o mouse e
        // escolhida (`checked`), com fundo, texto, fonte, raio e borda (a borda de baixo faz o
        // sublinhado). Sem ele, o visual de sempre. O relatório do Tabernáculo, C17.
        let themed = theme.styles.contains_key("TabControl.tab");
        for (i, (&tab, caption)) in tabs.iter().zip(&captions).enumerate() {
            let width = widths[i];
            let rect = RectF::new(x, r.y, width, TAB_BAR - 1.0);
            let active = i as i64 == self.tab_index;
            let mut style = text.clone();
            if themed {
                let state = aylis_core::ControlState { checked: active, focused: active && ctx.state.focused, ..Default::default() };
                let look = theme.resolve(&["TabControl.tab"], "", state);
                if !look.fill.is_transparent() {
                    canvas.fill_rect(rect, look.radius, &Brush::Solid(look.fill));
                }
                crate::basic::stroke_style_border(canvas, rect, look.radius, look.border_color, look.border_width, &look);
                style.color = look.text;
                style.font = look.font.clone();
            } else if active {
                style.color = theme.color("primary");
                style.font.weight = aylis_core::FontWeight::Bold;
                canvas.fill_rect(RectF::new(rect.x + 6.0, rect.bottom() - 3.0, rect.width - 12.0, 3.0), 1.5, &Brush::Solid(theme.color("primary")));
            } else {
                style.color = theme.color("text_muted");
            }
            if active && ctx.state.focused {
                canvas.stroke_rect(rect.inflate(-3.0), 4.0, &Stroke::new(theme.color("focus_ring"), 1.5));
            }
            canvas.draw_text(caption, rect, &style);
            headers.push((tab, rect));
            x += width;
        }
        // Há abas escondidas de um lado: a borda esmaece para o fundo (dá para rolar).
        let back = theme.color("background");
        let fade = |canvas: &mut dyn Canvas, x: f32, from: Color, to: Color| {
            let rect = RectF::new(x, r.y, FADE, TAB_BAR - 1.0);
            let stops = vec![GradientStop::new(0.0, from), GradientStop::new(1.0, to)];
            canvas.fill_rect(rect, 0.0, &Brush::Linear { start: PointF::new(rect.x, 0.0), end: PointF::new(rect.right(), 0.0), stops });
        };
        if scroll > 0.5 {
            fade(canvas, r.x, back, back.with_opacity(0.0));
        }
        if scroll < max_scroll - 0.5 {
            fade(canvas, r.right() - FADE, back.with_opacity(0.0), back);
        }
        canvas.restore();
        // Só as abas à vista recebem o clique.
        headers.retain(|(_, rect)| rect.right() > r.x && rect.x < r.right());
        *self.headers.borrow_mut() = headers;
    }
}

/// Uma página do [`TabControl`] (como o `TTabItem`): um contêiner com o título da aba.
pub struct TabItem {
    pub base: ControlBase,
    pub caption: String,
}

impl Default for TabItem {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 200.0);
        base.set_align(Align::Client).set_padding(8.0);
        Self { base, caption: "Aba".into() }
    }
}

impl TabItem {
    pub fn new(caption: &str) -> Self {
        Self { caption: caption.into(), ..Default::default() }
    }
}

impl Component for TabItem {
    leaf_control!("TabItem");

    properties! {
        caption: String => "Appearance";
        events: on_show
    }

    fn paint(&self, _canvas: &mut dyn Canvas, _ctx: &PaintCtx<'_>) {}
}

// ---------------------------------------------------------------------------

/// Moldura com título que agrupa controles (como o `TGroupBox`).
pub struct GroupBox {
    pub base: ControlBase,
    /// O título; com `&` antes de uma letra, Alt+letra leva ao primeiro campo do grupo.
    pub caption: String,
    /// Os caracteres dos campos dentro do grupo (veja o `Form`).
    pub allowed_chars: String,
    pub blocked_chars: String,
    pub only_allowed: bool,
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
}

impl Default for GroupBox {
    fn default() -> Self {
        let mut base = ControlBase::new(220.0, 140.0);
        base.set_padding([12.0, 28.0, 12.0, 12.0]);
        Self { base, caption: "Grupo".into(), allowed_chars: String::new(), blocked_chars: String::new(), only_allowed: false, chars_override: false, char_map: String::new() }
    }
}

impl GroupBox {
    pub fn new(caption: &str) -> Self {
        Self { caption: caption.into(), ..Default::default() }
    }
}

impl Component for GroupBox {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("GroupBox");

    properties! {
        caption: String => "Appearance",
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.caption)
    }

    fn mnemonic_focuses_child(&self) -> bool {
        true
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        paint_group_frame(canvas, ctx, &self.caption);
    }
}

// ---------------------------------------------------------------------------

const EXPANDER_HEADER: f32 = 40.0;

/// Seção que abre e fecha (como o `TExpander`): clique no cabeçalho (ou Espaço/Enter).
/// Fechada, fica só com o cabeçalho; `on_change` traz `expanded`.
pub struct Expander {
    pub base: ControlBase,
    pub caption: String,
    pub expanded: bool,
    /// Altura aberta (guardada ao fechar).
    full_height: f32,
}

impl Default for Expander {
    fn default() -> Self {
        let mut base = ControlBase::new(260.0, 160.0);
        base.set_padding([10.0, EXPANDER_HEADER + 4.0, 10.0, 10.0]).set_can_focus(true).set_clip_children(true);
        Self { base, caption: "Detalhes".into(), expanded: true, full_height: 160.0 }
    }
}

impl Expander {
    pub fn new(caption: &str) -> Self {
        Self { caption: caption.into(), ..Default::default() }
    }

    fn apply(&mut self, ui: &mut Ui, id: ComponentId) {
        if self.expanded {
            let h = self.full_height.max(EXPANDER_HEADER + 20.0);
            self.base.set_height(h);
        } else {
            if self.base.height() > EXPANDER_HEADER + 1.0 {
                self.full_height = self.base.height();
            }
            self.base.set_height(EXPANDER_HEADER);
        }
        for child in ui.children(id).to_vec() {
            if let Some(cb) = ui.control_mut(child) {
                cb.set_visible(self.expanded);
            }
        }
        ui.invalidate();
    }

    fn toggle(&mut self, ctx: &mut EventCtx<'_>) {
        self.expanded = !self.expanded;
        self.apply(ctx.ui, ctx.id);
        ctx.fire("on_change", EventArgs::Value(self.expanded.into()));
    }
}

impl Component for Expander {
    leaf_control!("Expander");

    properties! {
        caption: String => "Appearance",
        expanded: bool => "Behavior";
        events: on_change
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "expanded" => self.apply(ctx.ui, ctx.id),
            "height" if self.expanded => self.full_height = self.base.height(),
            _ => {}
        }
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        // Layout carregado fechado: os filhos começam escondidos.
        if !self.expanded && ctx.ui.children(ctx.id).iter().any(|&c| ctx.ui.control(c).is_some_and(|cb| cb.visible())) {
            self.apply(ctx.ui, ctx.id);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary && event.position.y <= EXPANDER_HEADER {
            self.toggle(ctx);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if matches!(event.key, Key::Space | Key::Enter) {
            self.toggle(ctx);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        canvas.stroke_rect(r, s.radius, &Stroke::new(if ctx.state.focused { ctx.theme.color("focus_ring") } else { s.border_color }, s.border_width.max(1.0)));
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Leading;
        text.font.weight = aylis_core::FontWeight::Bold;
        canvas.draw_text(&self.caption, RectF::new(r.x + 36.0, r.y, r.width - 44.0, EXPANDER_HEADER), &text);
        let c = PointF::new(r.x + 18.0, r.y + EXPANDER_HEADER / 2.0);
        let a = Stroke::new(ctx.theme.color("text_muted"), 1.8);
        if self.expanded {
            canvas.draw_line(PointF::new(c.x - 5.0, c.y - 2.5), PointF::new(c.x, c.y + 2.5), &a);
            canvas.draw_line(PointF::new(c.x, c.y + 2.5), PointF::new(c.x + 5.0, c.y - 2.5), &a);
            canvas.draw_line(PointF::new(r.x + 1.0, r.y + EXPANDER_HEADER), PointF::new(r.right() - 1.0, r.y + EXPANDER_HEADER), &Stroke::new(s.border_color, 1.0));
        } else {
            canvas.draw_line(PointF::new(c.x - 2.5, c.y - 5.0), PointF::new(c.x + 2.5, c.y), &a);
            canvas.draw_line(PointF::new(c.x + 2.5, c.y), PointF::new(c.x - 2.5, c.y + 5.0), &a);
        }
    }
}

// ---------------------------------------------------------------------------

/// Barra de status no rodapé (como o `TStatusBar`): `text` à esquerda; aceita controles
/// dentro (labels alinhados à direita, por exemplo).
pub struct StatusBar {
    pub base: ControlBase,
    pub text: String,
}

impl Default for StatusBar {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 28.0);
        base.set_align(Align::Bottom).set_padding([10.0, 2.0, 10.0, 2.0]);
        Self { base, text: String::new() }
    }
}

impl StatusBar {
    pub fn new(text: &str) -> Self {
        Self { text: text.into(), ..Default::default() }
    }
}

impl Component for StatusBar {
    leaf_control!("StatusBar");

    properties! {
        text: String => "Appearance"
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(s.fill));
        canvas.draw_line(PointF::new(r.x, r.y + 0.5), PointF::new(r.right(), r.y + 0.5), &Stroke::new(s.border_color, 1.0));
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Leading;
        text.font.size = (text.font.size - 1.0).max(9.0);
        canvas.draw_text(&self.text, RectF::new(r.x + 10.0, r.y, r.width - 20.0, r.height), &text);
    }
}

/// Barra de ferramentas no topo (como o `TToolBar`): os botões dentro ficam lado a lado
/// (use `align: left`).
pub struct ToolBar {
    pub base: ControlBase,
}

impl Default for ToolBar {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 44.0);
        base.set_align(Align::Top).set_padding(6.0).set_cursor(CursorShape::Default);
        Self { base }
    }
}

impl Component for ToolBar {
    leaf_control!("ToolBar");

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(s.fill));
        canvas.draw_line(PointF::new(r.x, r.bottom() - 0.5), PointF::new(r.right(), r.bottom() - 0.5), &Stroke::new(s.border_color, 1.0));
    }
}

/// A página que ficou à vista avisa: `on_show` (o `OnShow` da `TTabSheet`; C18 do Tabernáculo).
fn show_page(ui: &mut Ui, tabs: &[ComponentId], index: i64) {
    if let Some(&page) = usize::try_from(index).ok().and_then(|i| tabs.get(i)) {
        ui.queue_event(page, "on_show", EventArgs::None);
    }
}
