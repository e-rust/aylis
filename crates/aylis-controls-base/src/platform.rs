//! Componentes dos catálogos do Material 3 e do iOS (Human Interface Guidelines): cada um
//! desenha conforme o look do tema — o mesmo `AppBar` vira a "top app bar" do Material 3
//! ou a "navigation bar" do iOS, com Liquid Glass no iOS 26.
//!
//! `AppBar`, `NavigationRail`, `SideSheet`, `PageControl`, `UpDown` (o Stepper do iOS),
//! `WheelPicker` (a roda do iOS), `ButtonGroup` (Material 3 Expressive), `Gauge`,
//! `ActivityRings` e `LoadingIndicator` (as formas que se transformam do Material 3).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Align, Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, GradientStop, Key, KeyEvent,
    Look, PaintCtx, Path, PointF, PointerButton, PointerEvent, PropEnum, RectF, Shadow, Stroke, TextAlign, WheelEvent, properties,
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

fn token(ctx: &PaintCtx<'_>, name: &str, fallback: &str) -> Color {
    ctx.theme.colors.get(name).copied().unwrap_or_else(|| ctx.theme.color(fallback))
}

/// Liquid Glass desenhado por um componente (em partes dele, como os botões redondos da
/// barra do iOS 26): fundo desfocado, translúcido, com brilho e borda de luz.
pub fn paint_glass(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, r: RectF, radius: f32, tint: Option<Color>) {
    canvas.draw_shadow(r, radius, &Shadow::new(Color::rgba(0, 0, 0, 30), 14.0, 0.0, 4.0));
    let blurred = canvas.backdrop_blur(r, radius, 8.0);
    let dark = ctx.theme.dark;
    let base = match tint {
        Some(c) => c.with_opacity(0.82),
        None if dark => Color::rgba(40, 40, 46, if blurred { 120 } else { 220 }),
        None => Color::rgba(255, 255, 255, if blurred { 120 } else { 225 }),
    };
    canvas.fill_rect(r, radius, &Brush::Solid(base));
    aylis_core::paint::paint_glass_light(canvas, r, radius, dark || tint.is_some());
}

fn glass_look(ctx: &PaintCtx<'_>) -> bool {
    ctx.look() == Look::Ios26
}

// ===========================================================================
// AppBar
// ===========================================================================

/// Tamanho da barra: `small` e `center` (Material: título à esquerda ou no centro; iOS:
/// título central), `medium` e `large` (título grande embaixo, como o "large title").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppBarSize {
    #[default]
    Small,
    Center,
    Medium,
    Large,
}

impl PropEnum for AppBarSize {
    const NAMES: &'static [&'static str] = &["small", "center", "medium", "large"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Small, Self::Center, Self::Medium, Self::Large].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Barra do topo (a "top app bar" do Material 3, a "navigation bar" do iOS): título,
/// botão de navegação à esquerda (`nav_icon`: `arrow-left`, `menu`; no iOS com
/// `back_text`) e ações à direita (`actions`: ícones separados por `;`). `on_nav` e
/// `on_action` (índice da ação).
pub struct AppBar {
    pub base: ControlBase,
    pub title: String,
    pub subtitle: String,
    pub size: AppBarSize,
    pub nav_icon: String,
    pub back_text: String,
    pub actions: String,
    hits: RefCell<Vec<(RectF, i64)>>,
}

impl Default for AppBar {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 64.0);
        base.set_align(Align::Top).set_cursor(CursorShape::Pointer);
        Self {
            base,
            title: "Título".into(),
            subtitle: String::new(),
            size: AppBarSize::Small,
            nav_icon: "arrow-left".into(),
            back_text: String::new(),
            actions: "search;menu".into(),
            hits: RefCell::new(Vec::new()),
        }
    }
}

impl AppBar {
    /// A altura certa para o tamanho no look (Material 3: 64/64/112/152; iOS: 52 ou 96).
    pub fn height_for(size: AppBarSize, look: Look) -> f32 {
        let ios = look.is_ios();
        match (size, ios) {
            (AppBarSize::Small | AppBarSize::Center, true) => 52.0,
            (_, true) => 104.0,
            (AppBarSize::Small | AppBarSize::Center, false) => 64.0,
            (AppBarSize::Medium, false) => 112.0,
            (AppBarSize::Large, false) => 152.0,
        }
    }
}

impl Component for AppBar {
    leaf_control!("AppBar");

    properties! {
        title: String => "Appearance",
        subtitle: String => "Appearance",
        size: AppBarSize => "Appearance",
        nav_icon: String => "Appearance",
        back_text: String => "Appearance",
        actions: String => "Appearance";
        events: on_nav, on_action
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let hit = self.hits.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, i)| *i);
        match hit {
            Some(-1) => ctx.fire("on_nav", EventArgs::None),
            Some(i) => ctx.fire("on_action", EventArgs::Value(i.into())),
            None => {}
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let look = ctx.look();
        let ios = look.is_ios();
        let glass = glass_look(ctx);
        let mut hits = Vec::new();
        let actions = split(&self.actions);
        let fg = theme.color("text");
        let tint = if ios { theme.color("primary") } else { fg };
        // Fundo: a superfície (no iOS 26 o conteúdo passa por trás; a barra é transparente
        // e só os botões são de vidro).
        if !glass {
            canvas.fill_rect(r, 0.0, &Brush::Solid(if ios { token(ctx, "surface", "surface").with_opacity(0.94) } else { theme.color("background") }));
            if ios {
                canvas.draw_line(PointF::new(r.x, r.bottom() - 0.5), PointF::new(r.right(), r.bottom() - 0.5), &Stroke::new(theme.color("border"), 0.5));
            }
        }
        let row_h = if ios { 52.0 } else { 64.0 };
        let cy = r.y + row_h / 2.0;
        // Navegação.
        let mut left = r.x + if ios { 8.0 } else { 4.0 };
        if !self.nav_icon.is_empty() {
            if glass {
                let b = RectF::new(left + 4.0, cy - 22.0, 44.0, 44.0);
                paint_glass(canvas, ctx, b, 22.0, None);
                ctx.draw_icon(canvas, if self.nav_icon == "arrow-left" { "chevron-left" } else { &self.nav_icon }, b.inflate(-11.0), fg);
                hits.push((b, -1));
                left = b.right() + 8.0;
            } else if ios {
                let icon = if self.nav_icon == "arrow-left" { "chevron-left" } else { self.nav_icon.as_str() };
                let ir = RectF::new(left, cy - 12.0, 24.0, 24.0);
                ctx.draw_icon(canvas, icon, ir, tint);
                let mut t = ctx.text_style();
                t.color = tint;
                t.font.size = 17.0;
                let tw = if self.back_text.is_empty() { 0.0 } else { canvas.measure_text(&self.back_text, &t).width };
                canvas.draw_text(&self.back_text, RectF::new(ir.right() + 2.0, cy - 12.0, tw + 4.0, 24.0), &t);
                let hit = RectF::new(left - 4.0, cy - 22.0, 34.0 + tw, 44.0);
                hits.push((hit, -1));
                left = hit.right() + 4.0;
            } else {
                let b = RectF::new(left, cy - 24.0, 48.0, 48.0);
                ctx.draw_icon(canvas, &self.nav_icon, b.inflate(-12.0), fg);
                hits.push((b, -1));
                left = b.right() + 4.0;
            }
        }
        // Ações (no iOS 26, juntas numa cápsula de vidro).
        let mut right = r.right() - if ios { 8.0 } else { 4.0 };
        if !actions.is_empty() {
            let w = 44.0;
            let total = w * actions.len() as f32;
            if glass {
                let cap = RectF::new(right - total - 4.0, cy - 22.0, total, 44.0);
                paint_glass(canvas, ctx, cap, 22.0, None);
                for (i, icon) in actions.iter().enumerate() {
                    let b = RectF::new(cap.x + i as f32 * w, cap.y, w, 44.0);
                    ctx.draw_icon(canvas, icon, b.inflate(-11.0), fg);
                    hits.push((b, i as i64));
                }
                right = cap.x - 8.0;
            } else {
                for (i, icon) in actions.iter().enumerate().rev() {
                    let b = RectF::new(right - w, cy - 22.0, w, 44.0);
                    ctx.draw_icon(canvas, icon, b.inflate(if ios { 11.0 } else { 10.0 }).inflate(-22.0), if ios { tint } else { fg });
                    hits.push((b, i as i64));
                    right = b.x;
                }
            }
        }
        // Título.
        let mut t = ctx.text_style();
        t.color = fg;
        let big = matches!(self.size, AppBarSize::Medium | AppBarSize::Large);
        if big {
            // O título grande embaixo (e o pequeno some, como no rolar do iOS/Material).
            t.font.size = match (ios, self.size) {
                (true, _) => 34.0,
                (false, AppBarSize::Large) => 32.0,
                _ => 24.0,
            };
            t.font.weight = FontWeight::Bold;
            let y = r.y + row_h + if ios { 0.0 } else { (r.height - row_h - 40.0) / 2.0 };
            canvas.draw_text(&self.title, RectF::new(r.x + 16.0, y, r.width - 32.0, 44.0), &t);
            if !self.subtitle.is_empty() {
                let mut s = ctx.text_style();
                s.color = theme.color("text_muted");
                canvas.draw_text(&self.subtitle, RectF::new(r.x + 16.0, y + 40.0, r.width - 32.0, 20.0), &s);
            }
        } else {
            let centered = ios || self.size == AppBarSize::Center;
            t.font.size = if ios { 17.0 } else { 22.0 };
            t.font.weight = if ios { FontWeight::SemiBold } else { FontWeight::Normal };
            let title_r = if centered {
                let half = (r.width / 2.0 - (r.right() - right).max(left - r.x)).max(40.0);
                t.h_align = TextAlign::Center;
                RectF::new(r.x + r.width / 2.0 - half, r.y, half * 2.0, row_h)
            } else {
                RectF::new(left + 8.0, r.y, (right - left - 8.0).max(0.0), row_h)
            };
            if self.subtitle.is_empty() {
                canvas.draw_text(&self.title, title_r, &t);
            } else {
                canvas.draw_text(&self.title, RectF::new(title_r.x, title_r.y + 6.0, title_r.width, title_r.height / 2.0), &t);
                let mut s = ctx.text_style();
                s.h_align = t.h_align;
                s.color = theme.color("text_muted");
                s.font.size = 13.0;
                canvas.draw_text(&self.subtitle, RectF::new(title_r.x, title_r.y + title_r.height / 2.0, title_r.width, title_r.height / 2.0 - 4.0), &s);
            }
        }
        *self.hits.borrow_mut() = hits;
    }
}

// ===========================================================================
// NavigationRail
// ===========================================================================

/// Trilho de navegação (o "navigation rail" do Material 3, para telas largas): ícones
/// com o nome embaixo, o escolhido com a pílula; um FAB opcional em cima (`fab_icon`,
/// `on_fab`). `item_index`, `on_change`.
pub struct NavigationRail {
    pub base: ControlBase,
    pub items: String,
    pub icons: String,
    pub item_index: i64,
    pub fab_icon: String,
    hits: RefCell<Vec<(RectF, i64)>>,
}

impl Default for NavigationRail {
    fn default() -> Self {
        let mut base = ControlBase::new(88.0, 400.0);
        base.set_align(Align::Left).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: "Início;Buscar;Salvos;Perfil".into(),
            icons: "home;search;heart;user".into(),
            item_index: 0,
            fab_icon: "edit".into(),
            hits: RefCell::new(Vec::new()),
        }
    }
}

impl Component for NavigationRail {
    leaf_control!("NavigationRail");

    properties! {
        items: String => "Data",
        icons: String => "Appearance",
        item_index: i64 => "Data",
        fab_icon: String => "Appearance";
        events: on_change, on_fab
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.hits.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, i)| *i);
        match hit {
            Some(-1) => ctx.fire("on_fab", EventArgs::None),
            Some(i) if i != self.item_index => {
                self.item_index = i;
                ctx.fire("on_change", EventArgs::Value(i.into()));
                ctx.repaint();
            }
            _ => {}
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("background")));
        let mut hits = Vec::new();
        let mut y = r.y + 16.0;
        let cx = r.x + r.width / 2.0;
        if !self.fab_icon.is_empty() {
            let fab = RectF::new(cx - 28.0, y, 56.0, 56.0);
            canvas.draw_shadow(fab, 16.0, &Shadow::new(Color::rgba(0, 0, 0, 40), 6.0, 0.0, 2.0));
            canvas.fill_rect(fab, 16.0, &Brush::Solid(token(ctx, "primary_container", "primary_soft")));
            ctx.draw_icon(canvas, &self.fab_icon, fab.inflate(-16.0), token(ctx, "on_primary_container", "primary"));
            hits.push((fab, -1));
            y = fab.bottom() + 28.0;
        }
        let icons = split(&self.icons);
        let (pill, on) = (token(ctx, "secondary_container", "primary_soft"), token(ctx, "on_secondary_container", "primary"));
        for (i, item) in split(&self.items).iter().enumerate() {
            let selected = i as i64 == self.item_index;
            let p = RectF::new(cx - 28.0, y, 56.0, 32.0);
            if selected {
                canvas.fill_rect(p, 16.0, &Brush::Solid(pill));
            }
            if let Some(icon) = icons.get(i).filter(|s| !s.is_empty()) {
                ctx.draw_icon(canvas, icon, RectF::new(cx - 12.0, y + 4.0, 24.0, 24.0), if selected { on } else { theme.color("text_muted") });
            }
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = 12.0;
            t.font.weight = if selected { FontWeight::SemiBold } else { FontWeight::Normal };
            t.color = if selected { theme.color("text") } else { theme.color("text_muted") };
            canvas.draw_text(item, RectF::new(r.x, p.bottom() + 4.0, r.width, 16.0), &t);
            hits.push((RectF::new(r.x, y - 4.0, r.width, 60.0), i as i64));
            y += 68.0;
        }
        *self.hits.borrow_mut() = hits;
    }
}

// ===========================================================================
// SideSheet
// ===========================================================================

/// Folha lateral (o "side sheet" do Material 3): um contêiner à direita com título e
/// botão de fechar (`on_close`, que também desliga `open`). Coloque os controles dentro.
pub struct SideSheet {
    pub base: ControlBase,
    pub title: String,
    pub open: bool,
    pub sheet_width: f32,
    close: Cell<RectF>,
}

impl Default for SideSheet {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 400.0);
        base.set_align(Align::Right).set_padding([24.0, 72.0, 24.0, 24.0]).set_cursor(CursorShape::Default);
        Self { base, title: "Filtros".into(), open: true, sheet_width: 320.0, close: Cell::new(RectF::default()) }
    }
}

impl SideSheet {
    fn sync(&mut self, name: &str) {
        if matches!(name, "open" | "sheet_width") {
            self.base.set_width(self.sheet_width);
            self.base.set_visible(self.open);
        }
    }
}

impl Component for SideSheet {
    leaf_control!("SideSheet");

    properties! {
        title: String => "Appearance",
        open: bool => "Behavior",
        sheet_width: f32 => "Layout";
        events: on_close;
        on_set: sync
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.close.get().contains(event.position) {
            self.open = false;
            self.base.set_visible(false);
            ctx.fire("on_close", EventArgs::None);
            ctx.invalidate();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let radius = if ctx.look().is_material() { 16.0 } else { 12.0 };
        canvas.draw_shadow(r, radius, &Shadow::new(Color::rgba(0, 0, 0, 40), 16.0, -2.0, 0.0));
        // Cantos arredondados só do lado de dentro (esquerdo).
        canvas.save();
        canvas.clip_rect(r);
        canvas.fill_rect(RectF::new(r.x, r.y, r.width + radius, r.height), radius, &Brush::Solid(token(ctx, "surface", "surface")));
        canvas.restore();
        let mut t = ctx.text_style();
        t.font.size = 22.0;
        canvas.draw_text(&self.title, RectF::new(r.x + 24.0, r.y + 16.0, r.width - 80.0, 40.0), &t);
        let c = RectF::new(r.right() - 56.0, r.y + 12.0, 48.0, 48.0);
        ctx.draw_icon(canvas, "close", c.inflate(-12.0), theme.color("text_muted"));
        self.close.set(c);
    }
}

// ===========================================================================
// PageControl
// ===========================================================================

/// Pontos de página (o "page control" do iOS): `count` pontos, o atual (`index`)
/// destacado; clicar num lado anda uma página (`on_change`). No iOS 26 fica numa cápsula
/// de vidro; no Material, o atual vira um traço.
pub struct PageControl {
    pub base: ControlBase,
    pub count: i64,
    pub index: i64,
}

impl Default for PageControl {
    fn default() -> Self {
        let mut base = ControlBase::new(140.0, 28.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, count: 5, index: 0 }
    }
}

impl Component for PageControl {
    leaf_control!("PageControl");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        count: i64 => "Data",
        index: i64 => "Data";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let w = self.base.bounds().width;
        let next = if event.position.x < w / 2.0 { self.index - 1 } else { self.index + 1 };
        let next = next.clamp(0, (self.count - 1).max(0));
        if next != self.index {
            self.index = next;
            ctx.fire("on_change", EventArgs::Value(next.into()));
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let n = self.count.max(0) as usize;
        if n == 0 {
            return;
        }
        let material = ctx.look().is_material();
        let d = 8.0;
        let gap = 8.0;
        let current_w = if material { 20.0 } else { d };
        let total = (n - 1) as f32 * (d + gap) + current_w;
        let x0 = r.x + (r.width - total) / 2.0;
        let cy = r.y + r.height / 2.0;
        if glass_look(ctx) {
            let cap = RectF::new(x0 - 12.0, cy - 13.0, total + 24.0, 26.0);
            paint_glass(canvas, ctx, cap, 13.0, None);
        }
        let on = if ctx.look().is_ios() { ctx.theme.color("text") } else { accent(ctx) };
        let off = on.with_opacity(0.3);
        let mut x = x0;
        for i in 0..n {
            let cur = i as i64 == self.index;
            let w = if cur { current_w } else { d };
            canvas.fill_rect(RectF::new(x, cy - d / 2.0, w, d), d / 2.0, &Brush::Solid(if cur { on } else { off }));
            x += w + gap;
        }
    }
}

// ===========================================================================
// UpDown (o Stepper do iOS)
// ===========================================================================

/// Os botões − e + (o "stepper" do iOS, o `TUpDown` do Delphi): `value` entre `min` e
/// `max`, de `step` em `step`; segurar repete. `on_change` com o valor.
pub struct UpDown {
    pub base: ControlBase,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pressed: Cell<Option<bool>>,
}

impl Default for UpDown {
    fn default() -> Self {
        let mut base = ControlBase::new(94.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, value: 1.0, min: 0.0, max: 10.0, step: 1.0, pressed: Cell::new(None) }
    }
}

impl UpDown {
    fn bump(&mut self, ctx: &mut EventCtx<'_>, up: bool) {
        let v = (self.value + if up { self.step } else { -self.step }).clamp(self.min, self.max);
        if v != self.value {
            self.value = v;
            ctx.fire("on_change", EventArgs::Value(v.into()));
        }
        ctx.repaint();
    }
}

impl Component for UpDown {
    leaf_control!("UpDown");

    properties! {
        value: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        step: f64 => "Data";
        events: on_change
    }

    fn timer_interval(&self) -> Option<f64> {
        self.pressed.get().map(|_| 0.12)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(up) = self.pressed.get() {
            self.bump(ctx, up);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let up = event.position.x > self.base.bounds().width / 2.0;
        self.bump(ctx, up);
        self.pressed.set(Some(up));
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.pressed.set(None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::Up | Key::Right => self.bump(ctx, true),
            Key::Down | Key::Left => self.bump(ctx, false),
            _ => return,
        }
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let radius = if ctx.look().is_ios() { r.height / 2.0 } else { ctx.style.radius.max(8.0) };
        if glass_look(ctx) {
            paint_glass(canvas, ctx, r, radius, None);
        } else if ctx.look().is_material() {
            canvas.stroke_rect(r.inflate(-0.5), radius, &Stroke::new(theme.color("border"), 1.0));
        } else {
            canvas.fill_rect(r, radius, &Brush::Solid(theme.color("surface_2")));
        }
        let half = r.width / 2.0;
        if let Some(up) = self.pressed.get() {
            let pr = RectF::new(if up { r.x + half } else { r.x }, r.y, half, r.height);
            canvas.save();
            canvas.clip_rect(pr);
            canvas.fill_rect(r, radius, &Brush::Solid(theme.color("text").with_opacity(0.08)));
            canvas.restore();
        }
        canvas.draw_line(PointF::new(r.x + half, r.y + 7.0), PointF::new(r.x + half, r.bottom() - 7.0), &Stroke::new(theme.color("border"), 1.0));
        let fg = theme.color("text");
        let can_down = self.value > self.min;
        let can_up = self.value < self.max;
        ctx.draw_icon(canvas, "minus", RectF::new(r.x + half / 2.0 - 9.0, r.y + r.height / 2.0 - 9.0, 18.0, 18.0), if can_down { fg } else { fg.with_opacity(0.3) });
        ctx.draw_icon(canvas, "plus", RectF::new(r.x + half * 1.5 - 9.0, r.y + r.height / 2.0 - 9.0, 18.0, 18.0), if can_up { fg } else { fg.with_opacity(0.3) });
    }
}

// ===========================================================================
// WheelPicker
// ===========================================================================

/// A roda de opções do iOS (o `UIPickerView`): arrastar ou rolar gira a roda; ao parar,
/// a opção do meio fica escolhida (`item_index`, `on_change` com o texto). As de longe
/// ficam menores e mais apagadas, como num cilindro.
pub struct WheelPicker {
    pub base: ControlBase,
    pub items: String,
    pub item_index: i64,
    pub row_height: f32,
    /// Posição contínua (em linhas) enquanto gira.
    offset: Cell<f32>,
    drag: Cell<Option<(f32, f32)>>,
}

impl Default for WheelPicker {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 180.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: "Janeiro;Fevereiro;Março;Abril;Maio;Junho;Julho;Agosto;Setembro;Outubro;Novembro;Dezembro".into(),
            item_index: 9,
            row_height: 34.0,
            offset: Cell::new(9.0),
            drag: Cell::new(None),
        }
    }
}

impl WheelPicker {
    fn count(&self) -> usize {
        split(&self.items).len()
    }

    fn settle(&mut self, ctx: &mut EventCtx<'_>) {
        let n = self.count();
        if n == 0 {
            return;
        }
        let i = self.offset.get().round().clamp(0.0, (n - 1) as f32) as i64;
        self.offset.set(i as f32);
        if i != self.item_index {
            self.item_index = i;
            let text = split(&self.items).get(i as usize).map(|s| s.to_string()).unwrap_or_default();
            ctx.fire("on_change", EventArgs::Text(text));
        }
        ctx.repaint();
    }

    fn sync(&mut self, name: &str) {
        if name == "item_index" {
            self.offset.set(self.item_index as f32);
        }
    }
}

impl Component for WheelPicker {
    leaf_control!("WheelPicker");

    properties! {
        items: String => "Data",
        item_index: i64 => "Data",
        row_height: f32 => "Appearance";
        events: on_change;
        on_set: sync
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.drag.set(Some((event.position.y, self.offset.get())));
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((y0, o0)) = self.drag.get() {
            let n = self.count().max(1) as f32;
            self.offset.set((o0 - (event.position.y - y0) / self.row_height.max(1.0)).clamp(-0.4, n - 0.6));
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((y0, _)) = self.drag.take() {
            // Toque sem arrastar numa linha: vai para ela.
            if (event.position.y - y0).abs() < 4.0 {
                let mid = self.base.bounds().height / 2.0;
                let rows = ((event.position.y - mid) / self.row_height).round();
                self.offset.set(self.offset.get() + rows);
            }
            self.settle(ctx);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let n = self.count().max(1) as f32;
        let step = if event.touch { -event.delta.y / self.row_height } else { -event.delta.y.signum() };
        self.offset.set((self.offset.get() + step).clamp(0.0, n - 1.0));
        if !event.touch {
            self.settle(ctx);
        } else {
            ctx.repaint();
        }
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let d = match event.key {
            Key::Up => -1.0,
            Key::Down => 1.0,
            _ => return,
        };
        self.offset.set((self.offset.get() + d).clamp(0.0, (self.count().max(1) - 1) as f32));
        self.settle(ctx);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let h = self.row_height;
        let mid = r.y + r.height / 2.0;
        // A faixa da escolha.
        canvas.fill_rect(RectF::new(r.x + 6.0, mid - h / 2.0, r.width - 12.0, h), 8.0, &Brush::Solid(theme.color("surface_2")));
        canvas.save();
        canvas.clip_rect(r);
        let off = self.offset.get();
        let half_rows = (r.height / h / 2.0).ceil() as i64 + 1;
        let items = split(&self.items);
        let center = off.round() as i64;
        for i in (center - half_rows)..=(center + half_rows) {
            let Some(item) = usize::try_from(i).ok().and_then(|i| items.get(i)) else { continue };
            let d = i as f32 - off;
            // No cilindro: a linha sobe/desce pelo seno e encolhe pelo cosseno.
            let angle = d * h / (r.height * 0.55);
            // Quase de lado no cilindro: não aparece.
            if angle.abs() > 1.25 {
                continue;
            }
            let y = mid + angle.sin() * r.height * 0.48;
            let k = angle.cos();
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = (20.0 * (0.5 + 0.5 * k)).round();
            t.color = theme.color("text").with_opacity((k * k * k).clamp(0.1, 1.0));
            canvas.draw_text(item, RectF::new(r.x, y - h / 2.0, r.width, h), &t);
        }
        canvas.restore();
    }
}

// ===========================================================================
// ButtonGroup
// ===========================================================================

/// Grupo de botões do Material 3 Expressive: lado a lado (`connected` = colados, com os
/// cantos de dentro pequenos). O escolhido muda de forma (pílula → retângulo arredondado,
/// ou o contrário quando colados). Um (`multiple` desligado) ou vários; `selected` são os
/// itens separados por `;`; `on_change`.
pub struct ButtonGroup {
    pub base: ControlBase,
    pub items: String,
    pub icons: String,
    pub selected: String,
    pub multiple: bool,
    pub connected: bool,
    rects: RefCell<Vec<RectF>>,
}

impl Default for ButtonGroup {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 40.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, items: "Dia;Semana;Mês".into(), icons: String::new(), selected: "Semana".into(), multiple: false, connected: true, rects: RefCell::new(Vec::new()) }
    }
}

impl ButtonGroup {
    /// Aperta o botão `i`: marca (ou alterna, com `multiple`) e avisa no `on_change`.
    fn press(&mut self, ctx: &mut EventCtx<'_>, i: usize) {
        let items = split(&self.items);
        let Some(item) = items.get(i) else { return };
        let mut chosen: Vec<&str> = split(&self.selected);
        if self.multiple {
            if let Some(p) = chosen.iter().position(|c| c == item) {
                chosen.remove(p);
            } else {
                chosen.push(item);
                chosen.sort_by_key(|c| items.iter().position(|o| o == c));
            }
        } else {
            chosen = vec![item];
        }
        let next = chosen.join(";");
        if next != self.selected {
            self.selected = next;
            ctx.fire("on_change", EventArgs::Text(self.selected.clone()));
        }
        ctx.repaint();
    }
}

impl Component for ButtonGroup {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.selected.clone()))
    }

    leaf_control!("ButtonGroup");

    properties! {
        items: String => "Data",
        icons: String => "Appearance",
        selected: String => "Data",
        multiple: bool => "Behavior",
        connected: bool => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.rects.borrow().iter().position(|r| r.contains(event.position));
        if let Some(i) = hit {
            self.press(ctx, i);
        }
    }

    // Os botões para leitores de tela e automação: marcados ou não; clicar faz o mesmo que o
    // toque.
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        let rects = self.rects.borrow();
        let chosen = split(&self.selected);
        split(&self.items)
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                let on = chosen.contains(&t);
                let item = AccessItem::new(ItemRole::Button, t).checked(on).selected(on);
                match rects.get(i) {
                    Some(r) => item.rect(*r),
                    None => item,
                }
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        if index >= split(&self.items).len() {
            return false;
        }
        self.press(ctx, index);
        true
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let items = split(&self.items);
        let icons = split(&self.icons);
        if items.is_empty() {
            return;
        }
        let chosen = split(&self.selected);
        let gap = if self.connected { 2.0 } else { 8.0 };
        let w = (r.width - gap * (items.len() as f32 - 1.0)) / items.len() as f32;
        let full = r.height / 2.0;
        let (sel_fill, sel_fg) = (theme.color("primary"), theme.color("on_primary"));
        let (fill, fg) = (token(ctx, "secondary_container", "surface_2"), token(ctx, "on_secondary_container", "text"));
        let mut rects = Vec::new();
        for (i, item) in items.iter().enumerate() {
            let b = RectF::new(r.x + i as f32 * (w + gap), r.y, w, r.height);
            let on = chosen.contains(item);
            let first = i == 0;
            let last = i + 1 == items.len();
            // Colados: cantos de fora redondos, de dentro pequenos; o escolhido fica todo
            // redondo. Separados: pílulas; o escolhido vira retângulo arredondado.
            let corner = |outer: bool| -> f32 {
                if self.connected { if on || outer { full } else { 6.0 } } else if on { 12.0 } else { full }
            };
            let (left_r, right_r) = (corner(first || !self.connected), corner(last || !self.connected));
            // Cantos diferentes por lado: dois retângulos arredondados recortados ao meio.
            let paint_half = |canvas: &mut dyn Canvas, rect: RectF, clip: RectF, radius: f32, color: Color| {
                canvas.save();
                canvas.clip_rect(clip);
                canvas.fill_rect(rect, radius, &Brush::Solid(color));
                canvas.restore();
            };
            let color = if on { sel_fill } else { fill };
            paint_half(canvas, b, RectF::new(b.x, b.y, b.width / 2.0 + 0.5, b.height), left_r, color);
            paint_half(canvas, b, RectF::new(b.x + b.width / 2.0, b.y, b.width / 2.0, b.height), right_r, color);
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.color = if on { sel_fg } else { fg };
            t.font.weight = FontWeight::Medium;
            let mut tr = b;
            if let Some(icon) = icons.get(i).filter(|s| !s.is_empty()) {
                let tw = canvas.measure_text(item, &t).width;
                let x = b.x + (b.width - tw - 26.0) / 2.0;
                ctx.draw_icon(canvas, icon, RectF::new(x, b.y + (b.height - 18.0) / 2.0, 18.0, 18.0), t.color);
                tr = RectF::new(x + 24.0, b.y, tw + 4.0, b.height);
                t.h_align = TextAlign::Leading;
            }
            canvas.draw_text(item, tr, &t);
            rects.push(b);
        }
        *self.rects.borrow_mut() = rects;
    }
}

// ===========================================================================
// Gauge
// ===========================================================================

/// Medidor (o `Gauge` do iOS e do watchOS): arco de 270° com o valor entre `min` e `max`,
/// em gradiente (`gradient` liga as cores verde → amarelo → vermelho); o número e o
/// `label` no meio.
pub struct Gauge {
    pub base: ControlBase,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub label: String,
    pub gradient: bool,
}

impl Default for Gauge {
    fn default() -> Self {
        let mut base = ControlBase::new(120.0, 120.0);
        base.set_hit_test(false);
        Self { base, value: 72.0, min: 0.0, max: 100.0, label: "bpm".into(), gradient: true }
    }
}

impl Component for Gauge {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("Gauge");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        value: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        label: String => "Appearance",
        gradient: bool => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = r.width.min(r.height);
        let w = (side * 0.1).max(4.0);
        let rad = (side - w) / 2.0;
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let f = ((self.value - self.min) / (self.max - self.min).max(1e-9)).clamp(0.0, 1.0) as f32;
        let mut track = Path::new();
        track.arc(c, rad, rad, 135.0, 270.0);
        canvas.stroke_path(&track, &Stroke::new(ctx.theme.color("text").with_opacity(0.12), w).round());
        if f > 0.0 {
            let mut arc = Path::new();
            arc.arc(c, rad, rad, 135.0, 270.0 * f);
            let brush = if self.gradient {
                Brush::Sweep {
                    center: c,
                    start_angle: 135.0,
                    // As cores de sentido do tema (trocadas por `success_color`,
                    // `warning_color` e `danger_color`).
                    stops: vec![
                        GradientStop::new(0.0, ctx.theme.color("success")),
                        GradientStop::new(0.4, ctx.theme.color("warning")),
                        GradientStop::new(0.75, ctx.theme.color("danger")),
                        GradientStop::new(1.0, ctx.theme.color("danger")),
                    ],
                }
            } else {
                Brush::Solid(accent(ctx))
            };
            canvas.stroke_path(&arc, &Stroke::new(brush, w).round());
        }
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.font.size = (side * 0.26).round();
        t.font.weight = FontWeight::Bold;
        canvas.draw_text(&format!("{:.0}", self.value), RectF::new(r.x, c.y - side * 0.2, r.width, side * 0.32), &t);
        let mut l = ctx.text_style();
        l.h_align = TextAlign::Center;
        l.color = ctx.theme.color("text_muted");
        l.font.size = (side * 0.12).max(9.0);
        canvas.draw_text(&self.label, RectF::new(r.x, c.y + side * 0.12, r.width, side * 0.16), &l);
    }
}

// ===========================================================================
// ActivityRings
// ===========================================================================

/// Os anéis de atividade da Apple: até três valores (`values`, frações; passando de 1 dá
/// outra volta, com a sombra na ponta), por fora vermelho (Mover), verde (Exercício) e
/// azul (Em pé). `ring_colors` troca as cores (`#FF0000;#00AA00;#0066FF`; vazio = as da
/// Apple); o `accent_color` troca a do anel de fora.
pub struct ActivityRings {
    pub base: ControlBase,
    pub values: String,
    pub ring_colors: String,
}

impl Default for ActivityRings {
    fn default() -> Self {
        let mut base = ControlBase::new(120.0, 120.0);
        base.set_hit_test(false);
        Self { base, values: "0.82;1.25;0.5".into(), ring_colors: String::new() }
    }
}

impl ActivityRings {
    /// A cor de cada anel: a do começo do arco e a da ponta (mais clara).
    fn ring_color(&self, ctx: &PaintCtx<'_>, i: usize) -> (Color, Color) {
        let rgb = |v: u32| Color::rgb((v >> 16) as u8, (v >> 8) as u8, v as u8);
        let own = split(&self.ring_colors).get(i).and_then(|c| Color::from_hex(c.trim()));
        let own = if i == 0 { ctx.colors.accent.or(own) } else { own };
        match own {
            Some(c) => (c, c.lerp(Color::WHITE, 0.3)),
            None => (rgb(RINGS[i].0), rgb(RINGS[i].1)),
        }
    }
}

const RINGS: [(u32, u32); 3] = [(0xFA114F, 0xF95C8A), (0x92E82A, 0xC9F94C), (0x1EEAEF, 0x75F7FA)];

impl Component for ActivityRings {
    leaf_control!("ActivityRings");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        values: String => "Data",
        ring_colors: String => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = r.width.min(r.height);
        let w = side * 0.13;
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        for (i, v) in split(&self.values).iter().take(3).enumerate() {
            let value: f32 = v.trim().parse().unwrap_or(0.0);
            let rad = side / 2.0 - w / 2.0 - i as f32 * (w + 2.0);
            if rad <= w / 2.0 {
                break;
            }
            let (from, to) = self.ring_color(ctx, i);
            let mut track = Path::new();
            track.arc(c, rad, rad, -90.0, 360.0);
            canvas.stroke_path(&track, &Stroke::new(from.with_opacity(0.22), w));
            if value <= 0.0 {
                continue;
            }
            let sweep = (value.min(1.0) * 360.0).min(359.9);
            let mut arc = Path::new();
            arc.arc(c, rad, rad, -90.0, sweep);
            let stops = vec![GradientStop::new(0.0, from), GradientStop::new(1.0, to)];
            canvas.stroke_path(&arc, &Stroke::new(Brush::Sweep { center: c, start_angle: -90.0, stops: stops.clone() }, w).round());
            if value > 1.0 {
                // Outra volta: o resto por cima, com a sombra na ponta.
                let extra = ((value - 1.0) * 360.0).min(359.0);
                let end = (-90.0 + extra).to_radians();
                let tip = PointF::new(c.x + rad * end.cos(), c.y + rad * end.sin());
                canvas.fill_ellipse(RectF::new(tip.x - w / 2.0 - 1.0, tip.y - w / 2.0 + 1.5, w + 2.0, w + 2.0), &Brush::Solid(Color::BLACK.with_opacity(0.35)));
                let mut lap = Path::new();
                lap.arc(c, rad, rad, -90.0, extra);
                canvas.stroke_path(&lap, &Stroke::new(to, w).round());
            }
        }
    }
}

// ===========================================================================
// LoadingIndicator
// ===========================================================================

/// O indicador de carregamento do Material 3 Expressive: uma forma que gira e se
/// transforma (flor, biscoito, pentágono, pílula, sol...). `contained` põe um círculo de
/// fundo.
pub struct LoadingIndicator {
    pub base: ControlBase,
    pub contained: bool,
    pub active: bool,
}

impl Default for LoadingIndicator {
    fn default() -> Self {
        let mut base = ControlBase::new(48.0, 48.0);
        base.set_hit_test(false);
        Self { base, contained: false, active: true }
    }
}

/// As formas (raio pelo ângulo): número de pontas, quanto elas saem e o quanto arredondam.
const SHAPES: [(f32, f32, f32); 7] = [(8.0, 0.16, 1.0), (9.0, 0.10, 2.0), (5.0, 0.12, 1.0), (2.0, 0.22, 1.0), (12.0, 0.07, 1.0), (4.0, 0.16, 1.0), (6.0, 0.13, 1.0)];

fn shape_radius(shape: (f32, f32, f32), angle: f32) -> f32 {
    let (n, amp, sharp) = shape;
    let w = (n * angle).cos();
    // `sharp` > 1 deixa as pontas mais arredondadas (o "biscoito").
    let w = w.signum() * w.abs().powf(1.0 / sharp);
    1.0 - amp + amp * w
}

/// A forma no instante `t` (segundos), já no círculo de raio `r` em `c`.
pub fn loading_shape(c: PointF, r: f32, t: f64) -> Path {
    let period = 0.65;
    let phase = (t / period) as f32;
    let i = phase.floor() as usize;
    let k = phase - phase.floor();
    // Muda de forma no fim de cada período, com aceleração (o "spring" do Expressive).
    let ease = if k < 0.6 { 0.0 } else { let x = (k - 0.6) / 0.4; x * x * (3.0 - 2.0 * x) };
    let (a, b) = (SHAPES[i % SHAPES.len()], SHAPES[(i + 1) % SHAPES.len()]);
    let spin = (t as f32 * 140.0 + ease * 90.0 + i as f32 * 90.0).to_radians();
    let n = 96;
    let pts: Vec<PointF> = (0..n)
        .map(|j| {
            let ang = j as f32 / n as f32 * std::f32::consts::TAU;
            let rr = r * (shape_radius(a, ang) * (1.0 - ease) + shape_radius(b, ang) * ease);
            PointF::new(c.x + rr * (ang + spin).cos(), c.y + rr * (ang + spin).sin())
        })
        .collect();
    let mut p = Path::new();
    p.polygon(&pts, true);
    p
}

impl Component for LoadingIndicator {
    leaf_control!("LoadingIndicator");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        contained: bool => "Appearance",
        active: bool => "Behavior",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = r.width.min(r.height);
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let color = if self.contained { token(ctx, "on_primary_container", "primary") } else { accent(ctx) };
        if self.contained {
            canvas.fill_ellipse(RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side), &Brush::Solid(token(ctx, "primary_container", "primary_soft")));
        }
        let t = if self.active { ctx.time } else { 0.0 };
        canvas.fill_path(&loading_shape(c, side * if self.contained { 0.3 } else { 0.42 }, t), &Brush::Solid(color));
        if self.active {
            ctx.request_frame();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heights_and_shapes() {
        assert_eq!(AppBar::height_for(AppBarSize::Large, Look::Material3), 152.0);
        assert_eq!(AppBar::height_for(AppBarSize::Small, Look::Ios26), 52.0);
        let p = loading_shape(PointF::new(0.0, 0.0), 10.0, 0.3);
        let b = p.bounds();
        assert!(b.width > 15.0 && b.width <= 20.5, "{b:?}");
        // As formas ficam no círculo.
        for t in [0.0, 0.5, 1.3, 2.7] {
            let b = loading_shape(PointF::new(0.0, 0.0), 10.0, t).bounds();
            assert!(b.x >= -10.5 && b.right() <= 10.5, "{t}: {b:?}");
        }
    }
}
