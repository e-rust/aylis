//! Navegação: `NavigationDrawer` (gaveta lateral), `BottomNavigation` (barra de abas do
//! celular), `BottomSheet` (folha que sobe do rodapé), `Carousel` (slides), `SpeedDial`
//! (botão flutuante que abre ações) e `AvatarGroup` (avatares empilhados).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Key, KeyEvent,
    PaintCtx, PointF, PointerButton, PointerEvent, RectF, Shadow, Stroke, TextAlign, Ui, properties,
};

use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

fn accent(ctx: &PaintCtx<'_>) -> Color {
    let v = ctx.variant();
    ctx.theme.color(match v {
        "success" | "warning" | "danger" | "info" => v,
        _ => "primary",
    })
}

fn indicator(ctx: &PaintCtx<'_>) -> (Color, Color) {
    let theme = ctx.theme;
    let fill = theme.colors.get("secondary_container").copied().unwrap_or_else(|| accent(ctx).with_opacity(0.16));
    let on = theme.colors.get("on_secondary_container").copied().unwrap_or_else(|| accent(ctx));
    (fill, on)
}

// ===========================================================================
// NavigationDrawer
// ===========================================================================

/// Gaveta de navegação: fica à esquerda
/// (`align` Left), aberta ou fechada (`open`); fechada em modo `rail` mostra só os ícones.
/// Com `items`/`icons` desenha a lista de destinos (`item_index`, `on_change`); também
/// aceita controles dentro.
pub struct NavigationDrawer {
    pub base: ControlBase,
    /// Aberta (na largura `drawer_width`); fechada some ou, com `rail`, fica só com os ícones.
    pub open: bool,
    /// Fechada, mostra a faixa estreita só com os ícones (na largura `rail_width`) em vez de
    /// sumir.
    pub rail: bool,
    /// A largura aberta, em pontos.
    pub drawer_width: f32,
    /// A largura fechada em modo `rail`, em pontos.
    pub rail_width: f32,
    /// O título no topo da gaveta aberta; vazio: sem título.
    pub title: String,
    /// Os destinos da lista, separados por `;`.
    pub items: String,
    /// Os ícones dos destinos, na mesma ordem, separados por `;`.
    pub icons: String,
    /// O destino escolhido (a partir de 0); -1: nenhum.
    pub item_index: i64,
    rows: RefCell<Vec<RectF>>,
}

impl Default for NavigationDrawer {
    fn default() -> Self {
        let mut base = ControlBase::new(256.0, 400.0);
        base.set_align(Align::Left).set_cursor(CursorShape::Pointer).set_clip_children(true);
        Self {
            base,
            open: true,
            rail: false,
            drawer_width: 256.0,
            rail_width: 64.0,
            title: "Menu".into(),
            items: "Início;Clientes;Pedidos;Relatórios;Configurações".into(),
            icons: "home;user;list;grid;settings".into(),
            item_index: 0,
            rows: RefCell::new(Vec::new()),
        }
    }
}

impl NavigationDrawer {
    fn collapsed(&self) -> bool {
        !self.open && self.rail
    }

    fn apply_width(&mut self) {
        let w = if self.open { self.drawer_width } else if self.rail { self.rail_width } else { 0.0 };
        self.base.set_width(w);
        self.base.set_visible(w > 0.0);
    }

    /// Abre ou fecha.
    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.apply_width();
    }

    fn sync(&mut self, name: &str) {
        if matches!(name, "open" | "rail" | "drawer_width" | "rail_width") {
            self.apply_width();
        }
    }

    /// Abre ou fecha pelo dedo (decidido em 14/11), como o `toggle`.
    fn set_open_by_swipe(&mut self, ctx: &mut EventCtx<'_>, open: bool) {
        if self.open != open {
            self.open = open;
            self.apply_width();
            ctx.invalidate();
        }
        ctx.set_handled();
    }
}

impl Component for NavigationDrawer {
    leaf_control!("NavigationDrawer");

    // O deslizar para a esquerda em cima dela fecha; da borda esquerda do form, abre.
    fn swipe(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::SwipeEvent) {
        if event.direction == aylis_core::SwipeDirection::Left && self.open {
            self.set_open_by_swipe(ctx, false);
        }
    }

    fn edge_swipe(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::SwipeEvent) -> bool {
        // A borda esquerda do form, com ela fechada (no form, `event.start`).
        if event.direction != aylis_core::SwipeDirection::Right || self.open || event.start.x > 32.0 {
            return false;
        }
        self.set_open_by_swipe(ctx, true);
        true
    }

    properties! {
        open: bool => "Behavior",
        rail: bool => "Behavior",
        drawer_width: f32 => "Layout",
        rail_width: f32 => "Layout",
        title: String => "Appearance",
        items: String => "Data",
        icons: String => "Appearance",
        item_index: i64 => "Data";
        events: on_change;
        on_set: sync
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.rows.borrow().iter().position(|r| r.contains(event.position));
        if let Some(i) = hit {
            if i as i64 != self.item_index {
                self.item_index = i as i64;
                ctx.fire("on_change", EventArgs::Value((i as i64).into()));
            }
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.save();
        canvas.clip_rect(r);
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface")));
        canvas.draw_line(PointF::new(r.right() - 0.5, r.y), PointF::new(r.right() - 0.5, r.bottom()), &Stroke::new(theme.color("border"), 1.0));
        let collapsed = self.collapsed();
        let mut y = 12.0;
        if !self.title.is_empty() && !collapsed {
            let mut t = ctx.text_style();
            t.font.weight = FontWeight::SemiBold;
            t.font.size += 2.0;
            canvas.draw_text(&self.title, RectF::new(24.0, y, r.width - 32.0, 36.0), &t);
            y += 44.0;
        }
        let icons = split(&self.icons);
        let (sel_fill, sel_fg) = indicator(ctx);
        let mut rows = Vec::new();
        for (i, item) in split(&self.items).iter().enumerate() {
            let row = if collapsed { RectF::new(8.0, y, r.width - 16.0, 48.0) } else { RectF::new(12.0, y, r.width - 24.0, 48.0) };
            let selected = i as i64 == self.item_index;
            if selected {
                canvas.fill_rect(row, 24.0, &Brush::Solid(sel_fill));
            }
            let fg = if selected { sel_fg } else { theme.color("text") };
            let side = 22.0;
            if let Some(icon) = icons.get(i).filter(|s| !s.is_empty()) {
                let ix = if collapsed { row.x + (row.width - side) / 2.0 } else { row.x + 16.0 };
                ctx.draw_icon(canvas, icon, RectF::new(ix, row.y + (row.height - side) / 2.0, side, side), fg);
            }
            if !collapsed {
                let mut t = ctx.text_style();
                t.color = fg;
                t.font.weight = if selected { FontWeight::SemiBold } else { FontWeight::Normal };
                canvas.draw_text(item, RectF::new(row.x + 52.0, row.y, row.width - 60.0, row.height), &t);
            }
            rows.push(row);
            y += 52.0;
        }
        canvas.restore();
        *self.rows.borrow_mut() = rows;
    }
}

// ===========================================================================
// BottomNavigation
// ===========================================================================

/// Barra de navegação do rodapé (o `v-bottom-navigation`, a "navigation bar" do Material
/// 3): 3 a 5 destinos com ícone e nome; o escolhido ganha a pílula atrás do ícone.
pub struct BottomNavigation {
    pub base: ControlBase,
    /// Os nomes dos destinos, separados por `;`.
    pub items: String,
    /// Os ícones dos destinos, na mesma ordem, separados por `;`.
    pub icons: String,
    /// O destino escolhido (a partir de 0).
    pub item_index: i64,
}

impl Default for BottomNavigation {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 72.0);
        base.set_align(Align::Bottom).set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, items: "Início;Buscar;Favoritos;Perfil".into(), icons: "home;search;heart;user".into(), item_index: 0 }
    }
}

impl BottomNavigation {
    fn select(&mut self, ctx: &mut EventCtx<'_>, i: i64) {
        let n = split(&self.items).len() as i64;
        let i = i.clamp(0, (n - 1).max(0));
        if i != self.item_index {
            self.item_index = i;
            ctx.fire("on_change", EventArgs::Value(i.into()));
            ctx.repaint();
        }
    }
}

impl Component for BottomNavigation {
    leaf_control!("BottomNavigation");

    properties! {
        items: String => "Data",
        icons: String => "Appearance",
        item_index: i64 => "Data";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let n = split(&self.items).len();
        if n > 0 && event.button == PointerButton::Primary {
            let w = self.base.bounds().width / n as f32;
            self.select(ctx, (event.position.x / w) as i64);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let to = match event.key {
            Key::Left => self.item_index - 1,
            Key::Right => self.item_index + 1,
            _ => return,
        };
        self.select(ctx, to);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if ctx.look().is_ios() {
            return self.paint_ios(canvas, ctx);
        }
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface_2")));
        let items = split(&self.items);
        let icons = split(&self.icons);
        if items.is_empty() {
            return;
        }
        let (pill, on) = indicator(ctx);
        let w = r.width / items.len() as f32;
        for (i, item) in items.iter().enumerate() {
            let cx = r.x + w * (i as f32 + 0.5);
            let selected = i as i64 == self.item_index;
            let pill_r = RectF::new(cx - 32.0, r.y + 12.0, 64.0, 32.0);
            if selected {
                canvas.fill_rect(pill_r, 16.0, &Brush::Solid(pill));
            }
            let fg = if selected { on } else { theme.color("text_muted") };
            if let Some(icon) = icons.get(i).filter(|s| !s.is_empty()) {
                ctx.draw_icon(canvas, icon, RectF::new(cx - 12.0, pill_r.y + 4.0, 24.0, 24.0), fg);
            }
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = (t.font.size * 0.85).round();
            t.font.weight = if selected { FontWeight::SemiBold } else { FontWeight::Normal };
            t.color = if selected { theme.color("text") } else { theme.color("text_muted") };
            canvas.draw_text(item, RectF::new(cx - w / 2.0, pill_r.bottom() + 2.0, w, 20.0), &t);
        }
    }
}

impl BottomNavigation {
    /// A "tab bar" do iOS: barra translúcida com ícone e nome; no iOS 26, uma cápsula de
    /// Liquid Glass flutuando sobre o conteúdo, com o item escolhido numa pílula.
    fn paint_ios(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let items = split(&self.items);
        let icons = split(&self.icons);
        let glass = ctx.look() == aylis_core::Look::Ios26;
        let area = if glass {
            let h = (r.height - 14.0).clamp(40.0, 64.0);
            let a = RectF::new(r.x + 14.0, r.y + (r.height - h) / 2.0, r.width - 28.0, h);
            crate::platform::paint_glass(canvas, ctx, a, h / 2.0, None);
            a
        } else {
            canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface").with_opacity(0.94)));
            canvas.draw_line(PointF::new(r.x, r.y + 0.5), PointF::new(r.right(), r.y + 0.5), &Stroke::new(theme.color("border"), 0.5));
            r
        };
        if items.is_empty() {
            return;
        }
        let w = area.width / items.len() as f32;
        let tint = theme.color("primary");
        for (i, item) in items.iter().enumerate() {
            let cell = RectF::new(area.x + i as f32 * w, area.y, w, area.height);
            let selected = i as i64 == self.item_index;
            if selected && glass {
                let pill = cell.inflate(-4.0);
                let bg = if theme.dark { Color::WHITE.with_opacity(0.14) } else { Color::BLACK.with_opacity(0.07) };
                canvas.fill_rect(pill, pill.height / 2.0, &Brush::Solid(bg));
            }
            let fg = if selected { tint } else { theme.color("text") };
            let icon_side = 24.0;
            let label_h = 13.0;
            let top = cell.y + (cell.height - icon_side - label_h - 2.0) / 2.0;
            if let Some(icon) = icons.get(i).filter(|s| !s.is_empty()) {
                ctx.draw_icon(canvas, icon, RectF::new(cell.x + (w - icon_side) / 2.0, top, icon_side, icon_side), fg);
            }
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = 10.0;
            t.font.weight = FontWeight::Medium;
            t.color = fg;
            canvas.draw_text(item, RectF::new(cell.x, top + icon_side + 2.0, w, label_h), &t);
        }
    }
}

// ===========================================================================
// BottomSheet
// ===========================================================================

/// Folha que sobe do rodapé (o `v-bottom-sheet`, a "sheet" do iOS): um contêiner com
/// cantos de cima arredondados e a alça; aparece com `open` (na altura `sheet_height`).
/// Com `detents` (alturas: frações do pai, como `0.4;0.9`, ou pontos), arrastar a alça
/// muda a altura e solta encaixando na mais perto; puxar bem para baixo fecha
/// (`on_close`). Coloque os controles dentro.
pub struct BottomSheet {
    pub base: ControlBase,
    /// Aberta (aparece); fechada, some.
    pub open: bool,
    /// A altura da folha aberta, em pontos.
    pub sheet_height: f32,
    /// As alturas de encaixe ao arrastar a alça, separadas por `;`: até 1 é fração do pai
    /// (`0.4;0.9`), acima são pontos. Vazio: a alça não muda a altura.
    pub detents: String,
    drag: Cell<Option<(f32, f32)>>,
}

impl Default for BottomSheet {
    fn default() -> Self {
        let mut base = ControlBase::new(360.0, 280.0);
        base.set_align(Align::Bottom).set_padding([16.0, 28.0, 16.0, 16.0]);
        Self { base, open: true, sheet_height: 280.0, detents: String::new(), drag: Cell::new(None) }
    }
}

impl BottomSheet {
    /// As alturas dos detents, em pontos (o pai dá a referência das frações).
    pub fn detent_heights(&self, parent_height: f32) -> Vec<f32> {
        let mut out: Vec<f32> = self
            .detents
            .split(';')
            .filter_map(|d| d.trim().parse::<f32>().ok())
            .map(|d| if d <= 1.0 { d * parent_height } else { d })
            .collect();
        out.sort_by(|a, b| a.total_cmp(b));
        out
    }

    fn apply(&mut self) {
        self.base.set_height(self.sheet_height);
        self.base.set_visible(self.open);
    }

    fn sync(&mut self, name: &str) {
        if matches!(name, "open" | "sheet_height") {
            self.apply();
        }
    }
}

impl Component for BottomSheet {
    leaf_control!("BottomSheet");

    properties! {
        open: bool => "Behavior",
        sheet_height: f32 => "Layout",
        detents: String => "Behavior";
        events: on_close;
        on_set: sync
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        // Só a faixa da alça arrasta (o resto é do conteúdo).
        if !self.detents.is_empty() && event.position.y < 28.0 {
            let b = self.base.bounds();
            self.drag.set(Some((b.y + event.position.y, b.height)));
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((y0, h0)) = self.drag.get() {
            let y = self.base.bounds().y + event.position.y;
            let h = (h0 - (y - y0)).max(40.0);
            self.base.set_height(h);
            ctx.invalidate();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.drag.take().is_none() {
            return;
        }
        let parent = ctx.ui.parent(ctx.id).and_then(|p| ctx.ui.bounds(p)).map_or(800.0, |b| b.height);
        let heights = self.detent_heights(parent);
        let h = self.base.height();
        if heights.first().is_some_and(|min| h < min * 0.6) {
            self.open = false;
            self.base.set_visible(false);
            ctx.fire("on_close", EventArgs::None);
        } else if let Some(best) = heights.iter().copied().min_by(|a, b| (a - h).abs().total_cmp(&(b - h).abs())) {
            self.sheet_height = best;
            self.base.set_height(best);
        }
        ctx.invalidate();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let radius = if ctx.look().is_ios() { 34.0 } else { 28.0 };
        if ctx.look() == aylis_core::Look::Ios26 {
            // A "sheet" do iOS 26 é de vidro, com cantos grandes.
            canvas.save();
            canvas.clip_rect(r);
            crate::platform::paint_glass(canvas, ctx, RectF::new(r.x + 6.0, r.y, r.width - 12.0, r.height + radius), radius, None);
            canvas.restore();
            canvas.fill_rect(RectF::new(r.x + (r.width - 36.0) / 2.0, r.y + 8.0, 36.0, 5.0), 2.5, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(0.5)));
            return;
        }
        canvas.draw_shadow(r, radius, &Shadow::new(Color::rgba(0, 0, 0, 50), 16.0, 0.0, -2.0));
        // Cantos de baixo retos: um retângulo arredondado que passa da borda inferior.
        canvas.save();
        canvas.clip_rect(r);
        canvas.fill_rect(RectF::new(r.x, r.y, r.width, r.height + radius), radius, &Brush::Solid(ctx.theme.color("surface")));
        canvas.restore();
        canvas.fill_rect(RectF::new(r.x + (r.width - 32.0) / 2.0, r.y + 10.0, 32.0, 4.0), 2.0, &Brush::Solid(ctx.theme.color("text_muted").with_opacity(0.4)));
    }
}

// ===========================================================================
// Carousel
// ===========================================================================

/// Slides (o `v-carousel` / `v-window`): os controles de dentro, um de cada vez. `index`
/// é o slide; `interval` (ms, 0 = parado) passa sozinho; setas nas laterais
/// (`show_arrows`), bolinhas embaixo e arrastar para os lados. `on_change`.
pub struct Carousel {
    pub base: ControlBase,
    /// O slide mostrado (a partir de 0, na ordem dos filhos).
    pub index: i64,
    /// Passa para o próximo sozinho a cada tantos ms; 0 = parado.
    pub interval: u32,
    /// Mostra as setas de anterior e próximo nas laterais.
    pub show_arrows: bool,
    /// Depois do último volta ao primeiro (e antes do primeiro, o último); desligado, para
    /// nas pontas.
    pub cycle: bool,
    drag: Cell<Option<f32>>,
}

impl Default for Carousel {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 260.0);
        base.set_padding([44.0, 0.0, 44.0, 28.0]).set_clip_children(true).set_cursor(CursorShape::Pointer);
        Self { base, index: 0, interval: 0, show_arrows: true, cycle: true, drag: Cell::new(None) }
    }
}

impl Carousel {
    fn slides(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
        ui.children(id).to_vec()
    }

    fn show(&mut self, ctx: &mut EventCtx<'_>, index: i64) {
        if self.place(ctx, index) {
            let index = self.index;
            ctx.fire("on_change", EventArgs::Value(index.into()));
        }
    }

    /// Mostra o slide `index` (sem avisar); devolve se mudou.
    fn place(&mut self, ctx: &mut EventCtx<'_>, index: i64) -> bool {
        let slides = Self::slides(ctx.ui, ctx.id);
        let n = slides.len() as i64;
        if n == 0 {
            return false;
        }
        let index = if self.cycle { index.rem_euclid(n) } else { index.clamp(0, n - 1) };
        let changed = index != self.index;
        self.index = index;
        for (i, s) in slides.iter().enumerate() {
            if let Some(cb) = ctx.ui.control_mut(*s) {
                cb.set_visible(i as i64 == index);
            }
        }
        ctx.ui.invalidate();
        changed
    }

    pub fn next(ui: &mut Ui, id: ComponentId) {
        let i = ui.get::<Carousel>(id).map_or(0, |c| c.index);
        let _ = ui.set_prop(id, "index", i + 1);
    }
}

impl Component for Carousel {
    leaf_control!("Carousel");

    // No toque, o arrasto vira rolagem e o soltar não chega aqui: o deslizar do núcleo troca
    // de item (o mouse continua pelo `pointer_up`).
    fn swipe(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::SwipeEvent) {
        let step = match event.direction {
            aylis_core::SwipeDirection::Left => 1,
            aylis_core::SwipeDirection::Right => -1,
            _ => return,
        };
        let i = self.index + step;
        self.show(ctx, i);
        ctx.set_handled();
    }

    properties! {
        index: i64 => "Data",
        interval: u32 as Duration => "Behavior",
        show_arrows: bool => "Appearance",
        cycle: bool => "Behavior";
        events: on_change
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // Pelo código: o `on_change` sai do `set_prop` (com o índice já ajustado aqui).
        if name == "index" {
            let i = self.index;
            self.index = -1;
            self.place(ctx, i);
        }
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let slides = Self::slides(ctx.ui, ctx.id);
        let wrong = slides.iter().enumerate().any(|(i, s)| ctx.ui.control(*s).is_some_and(|c| c.visible() != (i as i64 == self.index)));
        if wrong {
            let i = self.index;
            self.show(ctx, i);
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        (self.interval > 0).then(|| f64::from(self.interval) / 1000.0)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        let i = self.index + 1;
        self.show(ctx, i);
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.drag.set(Some(event.position.x));
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let b = self.base.bounds();
        let start = self.drag.take();
        // Arrastou: vai para o lado do gesto.
        if let Some(x0) = start {
            let dx = event.position.x - x0;
            if dx.abs() > 40.0 {
                let i = self.index + if dx < 0.0 { 1 } else { -1 };
                return self.show(ctx, i);
            }
        }
        let p = event.position;
        if self.show_arrows && p.x < 44.0 {
            let i = self.index - 1;
            self.show(ctx, i);
        } else if self.show_arrows && p.x > b.width - 44.0 {
            let i = self.index + 1;
            self.show(ctx, i);
        } else if p.y > b.height - 28.0 {
            let n = Self::slides(ctx.ui, ctx.id).len();
            if n > 0 {
                let dots_w = n as f32 * 16.0;
                let i = ((p.x - (b.width - dots_w) / 2.0) / 16.0).floor() as i64;
                if (0..n as i64).contains(&i) {
                    self.show(ctx, i);
                }
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let n = ctx.ui.children(ctx.id).len();
        let muted = ctx.theme.color("text_muted");
        if self.show_arrows && n > 1 {
            for (x, icon) in [(6.0, "chevron-left"), (r.width - 38.0, "chevron-right")] {
                let b = RectF::new(x, (r.height - 28.0 - 32.0) / 2.0, 32.0, 32.0);
                canvas.fill_ellipse(b, &Brush::Solid(ctx.theme.color("surface_2")));
                ctx.draw_icon(canvas, icon, b.inflate(-7.0), ctx.theme.color("text"));
            }
        }
        let dots_w = n as f32 * 16.0;
        for i in 0..n {
            let c = PointF::new((r.width - dots_w) / 2.0 + i as f32 * 16.0 + 8.0, r.height - 14.0);
            let current = i as i64 == self.index;
            let d = if current { 8.0 } else { 6.0 };
            canvas.fill_ellipse(RectF::new(c.x - d / 2.0, c.y - d / 2.0, d, d), &Brush::Solid(if current { accent(ctx) } else { muted.with_opacity(0.5) }));
        }
    }
}

// ===========================================================================
// SpeedDial
// ===========================================================================

/// Botão flutuante que abre ações (o `v-speed-dial`): o botão principal fica embaixo;
/// aberto (`open`), as ações (`icons`, com `labels` ao lado) sobem acima dele.
/// `on_item` com o índice da ação. O controle ocupa a altura toda das ações.
pub struct SpeedDial {
    pub base: ControlBase,
    /// O ícone do botão principal.
    pub icon: String,
    /// Os ícones das ações, separados por `;` (uma ação para cada).
    pub icons: String,
    /// Os nomes das ações ao lado dos ícones, na mesma ordem, separados por `;`; vazio: só os
    /// ícones.
    pub labels: String,
    /// Aberto: as ações aparecem acima do botão principal.
    pub open: bool,
    hits: RefCell<Vec<RectF>>,
}

impl Default for SpeedDial {
    fn default() -> Self {
        let mut base = ControlBase::new(180.0, 240.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, icon: "plus".into(), icons: "edit;upload;trash".into(), labels: "Editar;Enviar;Apagar".into(), open: false, hits: RefCell::new(Vec::new()) }
    }
}

const FAB: f32 = 56.0;

impl Component for SpeedDial {
    leaf_control!("SpeedDial");

    properties! {
        icon: String => "Appearance",
        icons: String => "Appearance",
        labels: String => "Appearance",
        open: bool => "Behavior";
        events: on_item
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let b = self.base.bounds();
        let main = RectF::new(b.width - FAB, b.height - FAB, FAB, FAB);
        if main.contains(event.position) {
            self.open = !self.open;
            ctx.repaint();
            return;
        }
        if self.open {
            let hit = self.hits.borrow().iter().position(|r| r.contains(event.position));
            if let Some(i) = hit {
                ctx.fire("on_item", EventArgs::Value((i as i64).into()));
                self.open = false;
                ctx.repaint();
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if ctx.look() == aylis_core::Look::Material3 {
            return self.paint_m3(canvas, ctx);
        }
        let r = ctx.rect;
        // O fundo próprio (`fill`) vale mais que o destaque.
        let color = ctx.colors.fill.unwrap_or_else(|| accent(ctx));
        let on = ctx.theme.color("on_primary");
        let main = RectF::new(r.width - FAB, r.height - FAB, FAB, FAB);
        let mut hits = Vec::new();
        if self.open || ctx.design {
            let labels = split(&self.labels);
            for (i, icon) in split(&self.icons).iter().enumerate() {
                let side = 40.0;
                let y = main.y - (i as f32 + 1.0) * (side + 12.0);
                let b = RectF::new(main.x + (FAB - side) / 2.0, y, side, side);
                canvas.draw_shadow(b, side / 2.0, &Shadow::new(Color::rgba(0, 0, 0, 50), 6.0, 0.0, 2.0));
                canvas.fill_ellipse(b, &Brush::Solid(ctx.theme.color("surface")));
                ctx.draw_icon(canvas, icon, b.inflate(-10.0), color);
                let mut hit = b;
                if let Some(label) = labels.get(i).filter(|l| !l.is_empty()) {
                    let mut t = ctx.text_style();
                    let w = canvas.measure_text(label, &t).width + 16.0;
                    let lr = RectF::new(b.x - w - 8.0, b.y + 8.0, w, 24.0);
                    canvas.fill_rect(lr, 6.0, &Brush::Solid(ctx.theme.color("surface_2")));
                    t.h_align = TextAlign::Center;
                    canvas.draw_text(label, lr, &t);
                    hit = RectF::new(lr.x, b.y, b.right() - lr.x, b.height);
                }
                hits.push(hit);
            }
        }
        canvas.draw_shadow(main, 16.0, &Shadow::new(Color::rgba(0, 0, 0, 60), 10.0, 0.0, 4.0));
        canvas.fill_rect(main, 16.0, &Brush::Solid(color));
        let icon = if self.open { "close" } else { self.icon.as_str() };
        ctx.draw_icon(canvas, icon, main.inflate(-16.0), on);
        *self.hits.borrow_mut() = hits;
    }
}

impl SpeedDial {
    /// O "FAB menu" do Material 3: as ações são pílulas com ícone e nome, empilhadas à
    /// direita; o FAB aberto vira um círculo com o "x".
    fn paint_m3(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let container = ctx.colors.fill.or_else(|| theme.colors.get("primary_container").copied()).unwrap_or_else(|| theme.color("primary_soft"));
        let on_container = theme.colors.get("on_primary_container").copied().unwrap_or_else(|| theme.color("primary"));
        let mut hits = Vec::new();
        let open = self.open || ctx.design;
        let main = RectF::new(r.width - FAB, r.height - FAB, FAB, FAB);
        if open {
            let labels = split(&self.labels);
            let mut y = main.y - 8.0;
            for (i, icon) in split(&self.icons).iter().enumerate() {
                let label = labels.get(i).copied().unwrap_or("");
                let mut t = ctx.text_style();
                t.font.weight = FontWeight::Medium;
                t.color = on_container;
                let w = canvas.measure_text(label, &t).width + 24.0 + 24.0 + 12.0 + 8.0;
                let pill = RectF::new(r.width - w, y - 56.0, w, 56.0);
                canvas.draw_shadow(pill, 28.0, &Shadow::new(Color::rgba(0, 0, 0, 40), 6.0, 0.0, 2.0));
                canvas.fill_rect(pill, 28.0, &Brush::Solid(container));
                ctx.draw_icon(canvas, icon, RectF::new(pill.x + 20.0, pill.y + 16.0, 24.0, 24.0), on_container);
                canvas.draw_text(label, RectF::new(pill.x + 52.0, pill.y, w - 60.0, 56.0), &t);
                hits.push(pill);
                y = pill.y - 8.0;
            }
        }
        canvas.draw_shadow(main, if open { FAB / 2.0 } else { 16.0 }, &Shadow::new(Color::rgba(0, 0, 0, 50), 8.0, 0.0, 3.0));
        if open {
            canvas.fill_ellipse(main, &Brush::Solid(theme.color("primary")));
            ctx.draw_icon(canvas, "close", main.inflate(-16.0), theme.color("on_primary"));
        } else {
            canvas.fill_rect(main, 16.0, &Brush::Solid(container));
            ctx.draw_icon(canvas, &self.icon, main.inflate(-16.0), on_container);
        }
        *self.hits.borrow_mut() = hits;
    }
}

// ===========================================================================
// AvatarGroup
// ===========================================================================

/// Avatares empilhados (o `v-avatar-group`): as iniciais de `names` (separados por `;`)
/// sobrepostas; passando de `max`, o último mostra "+N".
pub struct AvatarGroup {
    pub base: ControlBase,
    /// Os nomes das pessoas, separados por `;`: cada avatar mostra as iniciais.
    pub names: String,
    /// Quantos avatares aparecem; os que passarem viram o "+N" no último.
    pub max: u32,
    /// O diâmetro de cada avatar, em pontos.
    pub size: f32,
}

impl Default for AvatarGroup {
    fn default() -> Self {
        let mut base = ControlBase::new(160.0, 40.0);
        base.set_hit_test(false);
        Self { base, names: "Ana Lima;Bruno Reis;Carla Dias;Davi Souza;Eva Rocha".into(), max: 4, size: 36.0 }
    }
}

fn initials(name: &str) -> String {
    name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect::<String>().to_uppercase()
}

impl Component for AvatarGroup {
    leaf_control!("AvatarGroup");

    properties! {
        names: String => "Data",
        max: u32 => "Appearance",
        size: f32 => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let names = split(&self.names);
        let max = (self.max.max(1) as usize).min(names.len());
        let extra = names.len() - max;
        let d = self.size;
        let step = d * 0.7;
        let y = (ctx.rect.height - d) / 2.0;
        let ring = ctx.theme.color("background");
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.color = Color::WHITE;
        t.font.weight = FontWeight::SemiBold;
        t.font.size = (d * 0.38).round();
        let count = if extra > 0 { max } else { names.len() };
        for (i, name) in names.iter().take(count).enumerate() {
            let r = RectF::new(i as f32 * step, y, d, d);
            canvas.fill_ellipse(r.inflate(2.0), &Brush::Solid(ring));
            let (fill, text) = if extra > 0 && i == max - 1 {
                (ctx.theme.color("surface_2"), format!("+{}", extra + 1))
            } else {
                (crate::widgets::name_color(name), initials(name))
            };
            canvas.fill_ellipse(r, &Brush::Solid(fill));
            let mut tt = t.clone();
            if extra > 0 && i == max - 1 {
                tt.color = ctx.theme.color("text");
            }
            canvas.draw_text(&text, r, &tt);
        }
    }
}
