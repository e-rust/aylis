//! TrayIcon: ícone na bandeja do sistema (perto do relógio), como o `TTrayIcon`. Ícone
//! parado ou animado, dica, indicador (ponto ou número, como os apps de mensagem) e um menu
//! próprio no clique direito, claro, escuro ou de qualquer cor.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, PaintCtx, PointF, PointerButton,
    PointerEvent, RectF, TextAlign, TrayEvent, TrayEventKind, TraySpec, Ui, WindowPlacement, properties,
};

use crate::leaf_control;

/// Ícone na bandeja. Não visual (fica no form, como o `Timer`).
pub struct TrayIcon {
    /// Arquivos dos quadros (PNG, ICO), separados por `;`: um = parado; vários = animação.
    pub icons: String,
    /// Tempo de cada quadro da animação (ms).
    pub interval: u32,
    pub hint: String,
    /// Indicador: vazio, `•` ou um texto curto (`3`, `9+`).
    pub badge: String,
    pub badge_color: Color,
    pub visible: bool,
    /// O menu do clique direito: itens separados por `;` (`-` é uma linha).
    pub menu_items: String,
    /// `light`, `dark` ou uma cor (`#1E293B`).
    pub menu_theme: String,
    loaded: RefCell<(String, Vec<Arc<Vec<u8>>>)>,
    menu: Cell<Option<ComponentId>>,
}

impl Default for TrayIcon {
    fn default() -> Self {
        Self {
            icons: String::new(),
            interval: 250,
            hint: String::new(),
            badge: String::new(),
            badge_color: Color::rgb(0xE5, 0x39, 0x35),
            visible: true,
            menu_items: String::new(),
            menu_theme: "light".into(),
            loaded: RefCell::default(),
            menu: Cell::new(None),
        }
    }
}

impl TrayIcon {
    fn files(&self) -> Vec<String> {
        self.icons.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()
    }

    /// Lê os quadros (do app ou do disco), se mudaram.
    fn load(&self, ui: Option<&Ui>) {
        if self.loaded.borrow().0 == self.icons {
            return;
        }
        let frames = self
            .files()
            .iter()
            .filter_map(|f| {
                ui.and_then(|ui| ui.services().resources.as_ref().and_then(|r| r.read(f))).or_else(|| std::fs::read(f).ok()).map(Arc::new)
            })
            .collect();
        *self.loaded.borrow_mut() = (self.icons.clone(), frames);
    }

    /// As cores do menu: (fundo, texto, destaque).
    fn menu_colors(&self) -> (Color, Color, Color) {
        match self.menu_theme.trim() {
            "dark" => (Color::rgb(0x2B, 0x2B, 0x2B), Color::rgb(0xF2, 0xF2, 0xF2), Color::rgb(0x3D, 0x3D, 0x3D)),
            "" | "light" => (Color::rgb(0xF9, 0xF9, 0xF9), Color::rgb(0x1A, 0x1A, 0x1A), Color::rgb(0xE5, 0xE5, 0xE5)),
            hex => {
                let bg = Color::from_hex(hex).unwrap_or(Color::WHITE);
                let light = 0.299 * bg.r as f32 + 0.587 * bg.g as f32 + 0.114 * bg.b as f32 > 140.0;
                let fg = if light { Color::rgb(0x1A, 0x1A, 0x1A) } else { Color::WHITE };
                (bg, fg, fg.with_opacity(0.12))
            }
        }
    }

    /// Abre o menu no ponto `screen` (pixels físicos), por cima de tudo; fecha ao perder o
    /// foco ou ao escolher (`on_menu` com o item).
    fn open_menu(ui: &mut Ui, id: ComponentId, screen: PointF, scale: f32) {
        let Some(tray) = ui.get::<TrayIcon>(id) else { return };
        let items: Vec<String> = tray.menu_items.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        if items.is_empty() {
            return;
        }
        let colors = tray.menu_colors();
        if let Some(old) = tray.menu.get().filter(|&m| ui.contains(m)) {
            ui.close(old);
            ui.free(old);
        }
        let height = items.iter().map(|i| if i == "-" { 9.0 } else { 32.0 }).sum::<f32>() + 8.0;
        let width = 220.0;
        let form = ui.add_root(crate::Form::new("", width, height));
        for (name, value) in [("border_style", "none"), ("topmost", "true"), ("show_in_taskbar", "false")] {
            let _ = match value {
                "true" | "false" => ui.set_prop(form, name, value == "true"),
                v => ui.set_prop(form, name, aylis_core::PropValue::Enum(v.into())),
            };
        }
        let list = ui.add(MenuList { items, colors, ..Default::default() }, form);
        ui.control_mut(list).unwrap().set_align(aylis_core::Align::Client);
        // Acima do clique (a barra de tarefas costuma estar embaixo), dentro da tela.
        let (w, h) = ((width * scale) as i32, (height * scale) as i32);
        let x = (screen.x as i32 - w).max(0);
        let y = (screen.y as i32 - h).max(0);
        ui.set_window_placement(form, WindowPlacement { x, y, width: w as u32, height: h as u32, maximized: false });
        ui.on(list, "on_select", move |ui, _, args| {
            if let EventArgs::Text(item) = args {
                ui.fire(id, "on_menu", &EventArgs::Text(item.clone()));
            }
            ui.close(form);
        });
        ui.on(form, "on_deactivate", move |ui, _, _| {
            ui.close(form);
        });
        if let Some(tray) = ui.get::<TrayIcon>(id) {
            tray.menu.set(Some(form));
        }
        let _ = ui.show(form);
    }
}

impl Component for TrayIcon {
    fn class_name(&self) -> &'static str {
        "TrayIcon"
    }

    properties! {
        icons: String => "Appearance",
        interval: u32 as Duration => "Appearance",
        hint: String => "Behavior",
        badge: String => "Appearance",
        badge_color: Color => "Appearance",
        visible: bool => "Behavior",
        menu_items: String => "Behavior",
        menu_theme: String => "Appearance";
        events: on_click, on_dbl_click, on_menu
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "icons" {
            self.load(Some(ctx.ui));
        }
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        let Some(e) = message.downcast_ref::<TrayEvent>() else { return };
        match e.kind {
            TrayEventKind::Click => ctx.fire("on_click", EventArgs::None),
            TrayEventKind::DoubleClick => ctx.fire("on_dbl_click", EventArgs::None),
            TrayEventKind::RightClick => {
                let (id, screen, scale) = (ctx.id, e.screen, e.scale);
                ctx.ui.poster().post(move |ui| TrayIcon::open_menu(ui, id, screen, scale));
            }
        }
    }
}

/// A lista do menu da bandeja (as cores dela, não as do tema).
#[derive(Default)]
struct MenuList {
    base: ControlBase,
    items: Vec<String>,
    colors: (Color, Color, Color),
    hover: Cell<Option<usize>>,
}

impl MenuList {
    fn rows(&self) -> Vec<(usize, RectF)> {
        let mut y = 4.0;
        let w = self.base.bounds().width;
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let h = if item == "-" { 9.0 } else { 32.0 };
                let r = RectF::new(4.0, y, w - 8.0, h);
                y += h;
                (i, r)
            })
            .collect()
    }

    fn at(&self, p: PointF) -> Option<usize> {
        self.rows().into_iter().find(|(i, r)| r.contains(p) && self.items[*i] != "-").map(|(i, _)| i)
    }
}

impl Component for MenuList {
    leaf_control!("TrayMenuList");

    fn props(&self) -> &'static [aylis_core::PropInfo] {
        const PROPS: &[aylis_core::PropInfo] = &[aylis_core::PropInfo::new("on_select", aylis_core::PropKind::Event, "Events")];
        PROPS
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let hit = self.at(event.position);
        if hit != self.hover.get() {
            self.hover.set(hit);
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            if let Some(i) = self.at(event.position) {
                ctx.fire("on_select", EventArgs::Text(self.items[i].clone()));
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (bg, fg, hover) = self.colors;
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(bg));
        canvas.stroke_rect(r.inflate(-0.5), 0.0, &aylis_core::Stroke::new(fg.with_opacity(0.15), 1.0));
        let mut t = ctx.text_style();
        t.color = fg;
        t.h_align = TextAlign::Leading;
        for (i, row) in self.rows() {
            let row = row.offset(r.x, r.y);
            if self.items[i] == "-" {
                let y = row.y + row.height / 2.0;
                canvas.draw_line(PointF::new(row.x + 8.0, y), PointF::new(row.right() - 8.0, y), &aylis_core::Stroke::new(fg.with_opacity(0.2), 1.0));
                continue;
            }
            if self.hover.get() == Some(i) {
                canvas.fill_rect(row, 4.0, &Brush::Solid(hover));
            }
            canvas.draw_text(&self.items[i], RectF::new(row.x + 12.0, row.y, row.width - 16.0, row.height), &t);
        }
        let _ = CursorShape::Default;
    }
}

impl aylis_core::PlatformHooks for TrayIcon {
    fn tray(&self) -> Option<TraySpec> {
        self.load(None);
        Some(TraySpec {
            icons: self.loaded.borrow().1.clone(),
            interval_ms: self.interval,
            tooltip: self.hint.clone(),
            badge: self.badge.clone(),
            badge_color: self.badge_color,
            visible: self.visible,
        })
    }
}
