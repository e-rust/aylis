//! Menus: [`MainMenu`] (a barra), [`PopupMenu`] (menu de contexto) e [`MenuItem`].
//!
//! Os itens são componentes filhos (como no Delphi): um `MenuItem` dentro de outro é um
//! submenu; `text: "-"` é um separador. O atalho (`shortcut: "Ctrl+S"`) funciona no form
//! inteiro, mesmo com o menu fechado. Cada item tem o seu `on_click`; o menu (`PopupMenu`,
//! `MainMenu`) também avisa em `on_item_click`, com o item clicado, para quem prefere um
//! handler só para o menu inteiro.
//!
//! Enquanto um menu está aberto, uma camada transparente cobre o form: ela pinta os painéis,
//! trata mouse e teclado (setas, Enter, Esc) e fecha tudo com um clique fora.

use std::any::Any;
use std::cell::RefCell;

use aylis_core::{
    Brush, Canvas, Component, ComponentId, ControlBase, EventArgs, EventCtx, Key, KeyEvent, PaintCtx,
    PointF, PointerButton, PointerEvent, PopupRequest, RectF, Stroke, TextAlign, TextStyle, Ui, properties,
};
use aylis_core::automation::{AccessItem, ItemAction, ItemRole};

use crate::leaf_control;

const ROW: f32 = 30.0;
const SEPARATOR: f32 = 9.0;
const PAD: f32 = 4.0;

/// Item de menu (como o `TMenuItem`). Filhos são o submenu.
pub struct MenuItem {
    pub text: String,
    /// Atalho: `"Ctrl+S"`, `"Ctrl+Shift+N"`, `"F9"`, `"Delete"`...
    pub shortcut: String,
    pub enabled: bool,
    pub checked: bool,
    /// Clicar alterna `checked` sozinho (como o `AutoCheck` do Delphi).
    pub auto_check: bool,
    /// Ícone à esquerda do texto (`"save"`, `"pacote:nome"`); marcado, aparece o ✓.
    pub icon: String,
    pub visible: bool,
    /// Ação destrutiva ("Excluir"): texto na cor `danger_text` (ou `danger`) do tema e,
    /// sob o mouse, fundo `danger` com texto branco.
    pub danger: bool,
    /// Título de uma seção do menu (a cabeça de um menu de contexto: o nome do item clicado):
    /// texto fraco em negrito, sem clique nem realce. O relatório do Tabernáculo, C15.
    pub header: bool,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self { text: String::new(), shortcut: String::new(), enabled: true, checked: false, auto_check: false, icon: String::new(), visible: true, danger: false, header: false }
    }
}

impl MenuItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = shortcut.into();
        self
    }

    pub fn is_separator(&self) -> bool {
        self.text == "-"
    }
}

impl Component for MenuItem {
    fn class_name(&self) -> &'static str {
        "MenuItem"
    }

    properties! {
        text: String => "Appearance",
        shortcut: String => "Behavior",
        enabled: bool => "Behavior",
        checked: bool => "Appearance",
        auto_check: bool => "Behavior",
        icon: String => "Appearance",
        visible: bool => "Behavior",
        danger: bool => "Appearance",
        header: bool => "Appearance";
        events: on_click
    }

    fn shortcut(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if self.enabled && self.visible && !self.header && parse_shortcut(&self.shortcut).is_some_and(|s| shortcut_matches(s, event)) {
            if self.auto_check {
                self.checked = !self.checked;
            }
            ctx.fire("on_click", EventArgs::None);
            notify_menu(ctx.ui, ctx.id);
            ctx.set_handled();
        }
    }
}

pub use aylis_core::keys::{parse_shortcut, shortcut_matches};

/// O menu (`PopupMenu` ou `MainMenu`) que contém o item.
fn menu_of(ui: &Ui, item: ComponentId) -> Option<ComponentId> {
    let mut current = ui.parent(item);
    while let Some(id) = current {
        if matches!(ui.class_name(id), Some("PopupMenu" | "MainMenu")) {
            return Some(id);
        }
        current = ui.parent(id);
    }
    None
}

/// Avisa o menu do item clicado (`on_item_click`, com o item).
fn notify_menu(ui: &mut Ui, item: ComponentId) {
    if let Some(menu) = menu_of(ui, item) {
        ui.queue_event(menu, "on_item_click", EventArgs::Value(aylis_core::PropValue::Component(Some(item))));
    }
}

/// Itens visíveis (filhos `MenuItem`) de um componente.
fn visible_items(ui: &Ui, parent: ComponentId) -> Vec<ComponentId> {
    ui.children(parent).iter().copied().filter(|&c| ui.get::<MenuItem>(c).is_some_and(|i| i.visible)).collect()
}

fn item(ui: &Ui, id: ComponentId) -> Option<&MenuItem> {
    ui.get::<MenuItem>(id)
}

fn has_submenu(ui: &Ui, id: ComponentId) -> bool {
    !visible_items(ui, id).is_empty()
}

// ---------------------------------------------------------------------------

/// Barra de menu (como o `TMainMenu`), normalmente `align: top`.
pub struct MainMenu {
    pub base: ControlBase,
    /// Retângulos dos itens da barra no último frame (coordenadas do controle).
    slots: RefCell<Vec<(ComponentId, RectF)>>,
    open: Option<ComponentId>,
}

impl Default for MainMenu {
    fn default() -> Self {
        let mut base = ControlBase::new(0.0, 30.0);
        base.set_align(aylis_core::Align::Top);
        Self { base, slots: RefCell::default(), open: None }
    }
}

impl MainMenu {
    /// Abre o submenu do item `index` da barra.
    fn open_index(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        let slots = self.slots.borrow().clone();
        let origin = ctx.ui.to_form(ctx.id, self.base.bounds()).origin();
        let bar: Vec<(ComponentId, RectF)> = slots.iter().map(|(id, r)| (*id, r.offset(origin.x, origin.y))).collect();
        self.open = Some(bar[index].0);
        open_overlay(ctx.ui, ctx.id, Anchor::Bar { items: bar, index });
        ctx.repaint();
    }

    fn bar_text_style(ctx: &PaintCtx<'_>) -> TextStyle {
        let mut style = ctx.text_style();
        style.h_align = TextAlign::Center;
        style.color = ctx.theme.color("text");
        style
    }
}

impl Component for MainMenu {
    leaf_control!("MainMenu");

    fn props(&self) -> &'static [aylis_core::PropInfo] {
        const PROPS: &[aylis_core::PropInfo] = &[aylis_core::PropInfo::new("on_item_click", aylis_core::PropKind::Event, "Events")];
        PROPS
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.theme.color("surface")));
        canvas.draw_line(PointF::new(0.0, r.height - 0.5), PointF::new(r.width, r.height - 0.5), &Stroke::new(ctx.theme.color("border"), 1.0));
        let style = Self::bar_text_style(ctx);
        let mut slots = self.slots.borrow_mut();
        slots.clear();
        let mut x = 4.0;
        for id in visible_items(ctx.ui, ctx.id) {
            let Some(it) = item(ctx.ui, id) else { continue };
            let w = canvas.measure_text(&it.text, &style).width + 20.0;
            let slot = RectF::new(x, 3.0, w, r.height - 6.0);
            if self.open == Some(id) {
                canvas.fill_rect(slot, 5.0, &Brush::Solid(ctx.theme.color("primary").with_opacity(0.18)));
            }
            let mut s = style.clone();
            if !it.enabled {
                s.color = ctx.theme.color("text_muted");
            }
            canvas.draw_text(&it.text, slot, &s);
            slots.push((id, slot));
            x += w;
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let slots = self.slots.borrow().clone();
        let Some(index) = slots.iter().position(|(_, r)| r.contains(event.position)) else { return };
        self.open_index(ctx, index);
        ctx.set_handled();
    }

    // Os itens da barra para leitores de tela e automação; clicar abre o submenu.
    fn access_items(&self, ui: &Ui, id: ComponentId) -> Vec<AccessItem> {
        let slots = self.slots.borrow();
        visible_items(ui, id)
            .into_iter()
            .filter_map(|i| {
                let it = item(ui, i)?;
                let mut a = AccessItem::new(ItemRole::MenuItem, it.text.clone()).disabled(!it.enabled).has_popup(has_submenu(ui, i));
                if let Some((_, r)) = slots.iter().find(|(s, _)| *s == i) {
                    a = a.rect(*r);
                }
                Some(a.selected(self.open == Some(i)))
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: ItemAction) -> bool {
        let items = visible_items(ctx.ui, ctx.id);
        let Some(&id) = items.get(index) else { return false };
        // A barra pode não ter sido pintada ainda (automação logo ao abrir).
        if !self.slots.borrow().iter().any(|(s, _)| *s == id) {
            let height = self.base.bounds().height;
            *self.slots.borrow_mut() = items.iter().enumerate().map(|(i, &it)| (it, RectF::new(4.0 + i as f32 * 80.0, 3.0, 80.0, height - 6.0))).collect();
        }
        let Some(slot) = self.slots.borrow().iter().position(|(s, _)| *s == id) else { return false };
        self.open_index(ctx, slot);
        true
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(MenuClosed(open)) = message.downcast_ref::<MenuClosed>() {
            self.open = *open;
            ctx.repaint();
        }
    }
}

// ---------------------------------------------------------------------------

/// Menu de contexto (como o `TPopupMenu`): ligue na propriedade `popup_menu` de um controle
/// (abre no clique direito) ou abra por código com [`PopupMenu::popup`].
#[derive(Default)]
pub struct PopupMenu {}

impl PopupMenu {
    /// Abre o menu em `at` (coordenadas do form onde o menu está).
    pub fn popup(ui: &mut Ui, menu: ComponentId, at: PointF) {
        open_overlay(ui, menu, Anchor::Point(at));
    }
}

impl Component for PopupMenu {
    fn class_name(&self) -> &'static str {
        "PopupMenu"
    }

    fn props(&self) -> &'static [aylis_core::PropInfo] {
        const PROPS: &[aylis_core::PropInfo] = &[
            aylis_core::PropInfo::new("on_popup", aylis_core::PropKind::Event, "Events"),
            aylis_core::PropInfo::new("on_item_click", aylis_core::PropKind::Event, "Events"),
            aylis_core::PropInfo::new("on_close", aylis_core::PropKind::Event, "Events"),
        ];
        PROPS
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        // Fechou, por item, Escape, clique fora ou foco perdido: `on_close` (o `OnClose` do
        // `TPopupMenu`; C16 do Tabernáculo).
        if let Some(MenuClosed(None)) = message.downcast_ref::<MenuClosed>() {
            ctx.ui.queue_event(ctx.id, "on_close", EventArgs::None);
            return;
        }
        if let Some(request) = message.downcast_ref::<PopupRequest>() {
            // on_popup antes de abrir: dá para habilitar/esconder itens conforme o alvo.
            ctx.ui.fire(ctx.id, "on_popup", &EventArgs::Value(aylis_core::PropValue::Component(Some(request.source))));
            open_overlay(ctx.ui, ctx.id, Anchor::Point(request.at));
        }
    }
}

// ---------------------------------------------------------------------------
// A camada do menu aberto

/// Avisa a barra de qual item ficou aberto (ou nenhum).
struct MenuClosed(Option<ComponentId>);

enum Anchor {
    Point(PointF),
    Bar { items: Vec<(ComponentId, RectF)>, index: usize },
}

struct Level {
    items: Vec<ComponentId>,
    rect: RectF,
    hovered: Option<usize>,
}

struct MenuOverlay {
    base: ControlBase,
    owner: ComponentId,
    bar: Vec<(ComponentId, RectF)>,
    bar_index: usize,
    levels: Vec<Level>,
}

fn open_overlay(ui: &mut Ui, owner: ComponentId, anchor: Anchor) {
    let root = ui.root_of(owner);
    // Só um menu aberto por vez.
    let existing: Vec<ComponentId> = ui.children(root).iter().copied().filter(|&c| ui.class_name(c) == Some("MenuOverlay")).collect();
    for e in existing {
        ui.free(e);
    }
    let size = ui.bounds(root).map(|b| b.size()).unwrap_or_default();
    let mut overlay = MenuOverlay { base: ControlBase::new(size.width, size.height), owner, bar: Vec::new(), bar_index: 0, levels: Vec::new() };
    overlay.base.set_can_focus(true).set_tab_stop(false);
    let (parent, at) = match anchor {
        Anchor::Point(p) => (owner, p),
        Anchor::Bar { items, index } => {
            let (item, r) = items[index];
            overlay.bar = items;
            overlay.bar_index = index;
            (item, PointF::new(r.x, r.bottom() + 2.0))
        }
    };
    overlay.push_level(ui, parent, at, None, size);
    let id = ui.add(overlay, root);
    if let Some(cb) = ui.control_mut(id) {
        cb.set_rect(0.0, 0.0, size.width, size.height);
    }
    ui.set_focus(id);
}

impl MenuOverlay {
    /// O texto do menu com a fonte do form onde ele abre (a do tema com a fonte própria do
    /// form por cima): a medida da largura bate com o desenho.
    fn text_style(ui: &Ui, root: ComponentId) -> TextStyle {
        let (font, color) = match Self::look(ui.theme()) {
            Some((normal, _)) => (normal.font, normal.text),
            None => (ui.font_of(root), ui.theme().color("text")),
        };
        let mut s = TextStyle { font, color, ..Default::default() };
        s.v_align = TextAlign::Center;
        s
    }

    /// O estilo `PopupMenu` do tema, se houver (normal e sob o mouse): fundo, borda, raio,
    /// fonte e o recuo do texto (`padding` esquerdo; sem ícones no painel, o texto começa
    /// nele em vez de deixar a coluna dos ícones). Sem ele, o visual padrão do aylis.
    fn look(theme: &aylis_core::Theme) -> Option<(aylis_core::ResolvedStyle, aylis_core::ResolvedStyle)> {
        if !theme.styles.contains_key("PopupMenu") {
            return None;
        }
        let normal = theme.resolve(&["PopupMenu"], "", aylis_core::ControlState::default());
        let hover = theme.resolve(&["PopupMenu"], "", aylis_core::ControlState { hover: true, ..Default::default() });
        Some((normal, hover))
    }

    /// Onde começa o texto de um painel: depois da coluna dos ícones, ou no recuo do
    /// estilo quando o painel não tem ícone nem marca.
    fn text_x(ui: &Ui, items: &[ComponentId]) -> f32 {
        let marks = items.iter().filter_map(|&id| item(ui, id)).any(|it| !it.icon.is_empty() || it.checked || it.auto_check);
        match Self::look(ui.theme()) {
            Some((normal, _)) if !marks => normal.padding.left,
            Some((normal, _)) => normal.padding.left.max(30.0),
            None => 30.0,
        }
    }

    /// Abre o painel com os itens de `parent` em `at` (ou ao lado de `beside`), sem sair
    /// da tela.
    fn push_level(&mut self, ui: &Ui, parent: ComponentId, at: PointF, beside: Option<RectF>, screen: aylis_core::SizeF) {
        let items = visible_items(ui, parent);
        if items.is_empty() {
            return;
        }
        let style = Self::text_style(ui, ui.root_of(parent));
        let text_x = Self::text_x(ui, &items);
        let look = Self::look(ui.theme()).is_some();
        let mut width: f32 = if look { 150.0 } else { 180.0 };
        let mut height = PAD * 2.0;
        for &id in &items {
            let Some(it) = item(ui, id) else { continue };
            if it.is_separator() {
                height += SEPARATOR;
                continue;
            }
            height += ROW;
            let text = ui.measure_text(&it.text, &style).width;
            let shortcut = if it.shortcut.is_empty() { 0.0 } else { ui.measure_text(&it.shortcut, &style).width + 32.0 };
            width = width.max(if look { text_x * 2.0 + text + shortcut + PAD * 2.0 } else { text + shortcut + 64.0 });
        }
        let mut x = at.x;
        let mut y = at.y;
        if let Some(row) = beside {
            x = row.right() - 2.0;
            y = row.y - PAD;
            if x + width > screen.width {
                x = row.x - width + 2.0;
            }
        }
        x = x.min(screen.width - width).max(0.0);
        if y + height > screen.height {
            y = if beside.is_some() { screen.height - height } else { at.y - height };
        }
        y = y.max(0.0);
        self.levels.push(Level { items, rect: RectF::new(x, y, width, height), hovered: None });
    }

    /// Linhas de um painel: (índice, retângulo).
    fn rows(ui: &Ui, level: &Level) -> Vec<(usize, RectF)> {
        let mut y = level.rect.y + PAD;
        let mut out = Vec::new();
        for (i, &id) in level.items.iter().enumerate() {
            let h = if item(ui, id).is_some_and(MenuItem::is_separator) { SEPARATOR } else { ROW };
            out.push((i, RectF::new(level.rect.x + PAD, y, level.rect.width - PAD * 2.0, h)));
            y += h;
        }
        out
    }

    /// Painel e linha sob o ponto.
    fn hit(&self, ui: &Ui, p: PointF) -> Option<(usize, Option<usize>)> {
        let li = self.levels.iter().rposition(|l| l.rect.contains(p))?;
        let row = Self::rows(ui, &self.levels[li]).into_iter().find(|(_, r)| r.contains(p)).map(|(i, _)| i);
        Some((li, row))
    }

    fn selectable(ui: &Ui, id: ComponentId) -> bool {
        item(ui, id).is_some_and(|i| i.enabled && !i.header && !i.is_separator())
    }

    fn hover(&mut self, ui: &Ui, level: usize, row: Option<usize>) {
        self.levels.truncate(level + 1);
        self.levels[level].hovered = row;
        let Some(row) = row else { return };
        let id = self.levels[level].items[row];
        if Self::selectable(ui, id) && has_submenu(ui, id) {
            let rect = Self::rows(ui, &self.levels[level]).into_iter().find(|(i, _)| *i == row).map(|(_, r)| r);
            let screen = self.base.bounds().size();
            self.push_level(ui, id, PointF::ZERO, rect, screen);
        }
    }

    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        let owner = self.owner;
        ctx.ui.free(ctx.id);
        ctx.ui.send_message(owner, &MenuClosed(None));
    }

    /// Escolhe o item: fecha o menu e dispara o `on_click` do item.
    fn activate(&mut self, ctx: &mut EventCtx<'_>, id: ComponentId) {
        if !Self::selectable(ctx.ui, id) || has_submenu(ctx.ui, id) {
            return;
        }
        self.close(ctx);
        if let Some(it) = ctx.ui.get_mut::<MenuItem>(id).filter(|it| it.auto_check) {
            it.checked = !it.checked;
        }
        ctx.ui.click(id);
        notify_menu(ctx.ui, id);
    }

    fn switch_bar(&mut self, ctx: &mut EventCtx<'_>, index: usize) {
        if self.bar.is_empty() || index == self.bar_index && !self.levels.is_empty() {
            return;
        }
        self.bar_index = index;
        self.levels.clear();
        let (id, r) = self.bar[index];
        let screen = self.base.bounds().size();
        self.push_level(ctx.ui, id, PointF::new(r.x, r.bottom() + 2.0), None, screen);
        ctx.ui.send_message(self.owner, &MenuClosed(Some(id)));
        ctx.repaint();
    }

    fn move_hover(&mut self, ctx: &mut EventCtx<'_>, step: isize) {
        let Some(level) = self.levels.len().checked_sub(1) else { return };
        let count = self.levels[level].items.len() as isize;
        let mut i = self.levels[level].hovered.map_or(if step > 0 { -1 } else { count }, |h| h as isize);
        for _ in 0..count {
            i = (i + step).rem_euclid(count);
            if Self::selectable(ctx.ui, self.levels[level].items[i as usize]) {
                self.levels[level].hovered = Some(i as usize);
                break;
            }
        }
        ctx.repaint();
    }
}

impl Component for MenuOverlay {
    leaf_control!("MenuOverlay");

    // Some junto com quem a abriu (a tela refeita não deixa a lista órfã).
    fn dies_with(&self) -> Option<ComponentId> {
        Some(self.owner)
    }

    // Os itens dos painéis abertos (o menu e os submenus), sem as divisórias: clicar num
    // com submenu abre o submenu; nos outros, ativa como o clique.
    fn access_items(&self, ui: &Ui, _: ComponentId) -> Vec<AccessItem> {
        let mut out = Vec::new();
        for level in &self.levels {
            let rows = Self::rows(ui, level);
            for (i, &id) in level.items.iter().enumerate() {
                let Some(it) = item(ui, id).filter(|it| !it.is_separator()) else { continue };
                let role = if it.auto_check || it.checked { ItemRole::MenuItemCheck } else { ItemRole::MenuItem };
                let mut a = AccessItem::new(role, it.text.clone()).disabled(!it.enabled).has_popup(has_submenu(ui, id)).selected(level.hovered == Some(i));
                if role == ItemRole::MenuItemCheck {
                    a = a.checked(it.checked);
                }
                if let Some((_, r)) = rows.iter().find(|(r, _)| *r == i) {
                    a = a.rect(*r);
                }
                out.push(a);
            }
        }
        out
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: ItemAction) -> bool {
        // O mesmo percurso do `access_items`: (painel, linha) do índice.
        let mut n = 0;
        let mut found = None;
        for (li, level) in self.levels.iter().enumerate() {
            for (i, &id) in level.items.iter().enumerate() {
                if item(ctx.ui, id).is_some_and(|it| !it.is_separator()) {
                    if n == index {
                        found = Some((li, i, id));
                    }
                    n += 1;
                }
            }
        }
        let Some((level, row, id)) = found else { return false };
        if !Self::selectable(ctx.ui, id) {
            return false;
        }
        if has_submenu(ctx.ui, id) {
            self.hover(ctx.ui, level, Some(row));
            ctx.repaint();
        } else {
            self.activate(ctx, id);
        }
        true
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((level, row)) = self.hit(ctx.ui, event.position) {
            if self.levels[level].hovered != row || self.levels.len() != level + 1 {
                self.hover(ctx.ui, level, row);
                ctx.repaint();
            }
        } else if let Some(index) = self.bar.iter().position(|(_, r)| r.contains(event.position)) {
            self.switch_bar(ctx, index);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.hit(ctx.ui, event.position).is_some() {
            return; // escolhe ao soltar (click)
        }
        if let Some(index) = self.bar.iter().position(|(_, r)| r.contains(event.position)) {
            if index == self.bar_index {
                self.close(ctx);
            } else {
                self.switch_bar(ctx, index);
            }
            return;
        }
        // Clique fora: fecha.
        self.close(ctx);
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((level, Some(row))) = self.hit(ctx.ui, event.position) {
            let id = self.levels[level].items[row];
            self.activate(ctx, id);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        ctx.set_handled();
        match event.key {
            Key::Down => self.move_hover(ctx, 1),
            Key::Up => self.move_hover(ctx, -1),
            Key::Right => {
                let level = self.levels.len() - 1;
                let hovered = self.levels[level].hovered;
                if hovered.is_some_and(|h| has_submenu(ctx.ui, self.levels[level].items[h])) {
                    self.hover(ctx.ui, level, hovered);
                    let deeper = self.levels.len() - 1;
                    self.levels[deeper].hovered = None;
                    self.move_hover(ctx, 1);
                } else if !self.bar.is_empty() {
                    let next = (self.bar_index + 1) % self.bar.len();
                    self.switch_bar(ctx, next);
                }
            }
            Key::Left => {
                if self.levels.len() > 1 {
                    self.levels.pop();
                    ctx.repaint();
                } else if !self.bar.is_empty() {
                    let previous = (self.bar_index + self.bar.len() - 1) % self.bar.len();
                    self.switch_bar(ctx, previous);
                }
            }
            Key::Enter | Key::Space => {
                let level = self.levels.len() - 1;
                if let Some(h) = self.levels[level].hovered {
                    let id = self.levels[level].items[h];
                    if has_submenu(ctx.ui, id) {
                        self.hover(ctx.ui, level, Some(h));
                        self.move_hover(ctx, 1);
                    } else {
                        self.activate(ctx, id);
                    }
                }
            }
            Key::Escape => {
                if self.levels.len() > 1 {
                    self.levels.pop();
                    ctx.repaint();
                } else {
                    self.close(ctx);
                }
            }
            _ => {}
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.close(ctx);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let theme = ctx.theme;
        let style = Self::text_style(ctx.ui, ctx.ui.root_of(ctx.id));
        let look = Self::look(theme);
        let (fill, edge, edge_w, radius, hover_fill) = match &look {
            Some((n, h)) => (n.fill, n.border_color, n.border_width, n.radius, h.fill),
            None => (theme.color("surface"), theme.color("border"), 1.0, 7.0, theme.color("primary").with_opacity(0.16)),
        };
        for level in &self.levels {
            let r = level.rect;
            let text_x = Self::text_x(ctx.ui, &level.items);
            canvas.fill_rect(r.offset(0.0, 4.0).inflate(2.0), radius + 1.0, &Brush::Solid(aylis_core::Color::BLACK.with_opacity(0.16)));
            canvas.fill_rect(r, radius, &Brush::Solid(fill));
            if edge_w > 0.0 {
                canvas.stroke_rect(r, radius, &Stroke::new(edge, edge_w));
            }
            for (i, row) in Self::rows(ctx.ui, level) {
                let id = level.items[i];
                let Some(it) = item(ctx.ui, id) else { continue };
                if it.is_separator() {
                    let y = row.y + row.height / 2.0;
                    canvas.draw_line(PointF::new(row.x + 6.0, y), PointF::new(row.right() - 6.0, y), &Stroke::new(theme.color("border"), 1.0));
                    continue;
                }
                let mut s = style.clone();
                if it.header {
                    s.color = theme.color("text_muted");
                    s.font.weight = aylis_core::FontWeight::SemiBold;
                    canvas.draw_text(&it.text, RectF::new(row.x + 10.0, row.y, row.width - 20.0, row.height), &s);
                    continue;
                }
                let hovered = level.hovered == Some(i) && it.enabled;
                if hovered {
                    let bg = if it.danger { theme.color("danger") } else { hover_fill };
                    canvas.fill_rect(row, if look.is_some() { 4.0 } else { 5.0 }, &Brush::Solid(bg));
                }
                if it.danger {
                    s.color = if hovered {
                        aylis_core::Color::WHITE
                    } else {
                        theme.colors.get("danger_text").copied().unwrap_or(theme.color("danger"))
                    };
                }
                if !it.enabled {
                    s.color = theme.color("text_muted");
                }
                if it.checked {
                    canvas.draw_text("✓", RectF::new(row.x + 8.0, row.y, 20.0, row.height), &s);
                } else if !it.icon.is_empty() {
                    let side = 16.0;
                    ctx.draw_icon(canvas, &it.icon, RectF::new(row.x + 8.0, row.y + (row.height - side) / 2.0, side, side), s.color);
                }
                canvas.draw_text(&it.text, RectF::new(row.x + text_x, row.y, row.width - text_x - 10.0, row.height), &s);
                let mut right = s.clone();
                right.h_align = TextAlign::Trailing;
                if has_submenu(ctx.ui, id) {
                    canvas.draw_text("›", RectF::new(row.x, row.y, row.width - 10.0, row.height), &right);
                } else if !it.shortcut.is_empty() {
                    right.color = theme.color("text_muted");
                    canvas.draw_text(&it.shortcut, RectF::new(row.x, row.y, row.width - 12.0, row.height), &right);
                }
            }
        }
    }
}
