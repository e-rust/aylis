//! TrayIcon: ícone na bandeja do sistema (perto do relógio). Ícone
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
    /// A dica que aparece com o mouse parado em cima do ícone.
    pub hint: String,
    /// Indicador: vazio, `•` ou um texto curto (`3`, `9+`).
    pub badge: String,
    /// A cor do fundo do indicador.
    pub badge_color: Color,
    /// O ícone aparece na bandeja; desligado, sai dela.
    pub visible: bool,
    /// O menu do clique direito: itens separados por `;` (`-` é uma linha).
    pub menu_items: String,
    /// `light`, `dark` ou uma cor (`#1E293B`).
    pub menu_theme: String,
    /// O menu do clique direito por um `PopupMenu`: os
    /// `MenuItem` dele, com `enabled`, `checked`, `header`, divisória e submenu (recuado).
    /// Escolher um item dispara o `on_click` dele (e o `on_item_click` do menu) e o `on_menu`
    /// daqui, com o texto. Vence o `menu_items`. O relatório do Tabernáculo, C26.
    pub popup_menu: Option<ComponentId>,
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
            popup_menu: None,
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
        let items: Vec<Entry> = match tray.popup_menu.filter(|&m| ui.contains(m)) {
            Some(menu) => {
                let mut out = Vec::new();
                menu_entries(ui, menu, 0, &mut out);
                out
            }
            None => tray.menu_items.split(';').map(str::trim).filter(|s| !s.is_empty()).map(Entry::text).collect(),
        };
        if items.is_empty() {
            return;
        }
        let colors = tray.menu_colors();
        if let Some(old) = tray.menu.get().filter(|&m| ui.contains(m)) {
            ui.close(old);
            ui.free(old);
        }
        let height = items.iter().map(|i| if i.separator { 9.0 } else { 32.0 }).sum::<f32>() + 8.0;
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
        ui.on(list, "on_select", move |ui, list, args| {
            ui.close(form);
            let EventArgs::Text(text) = args else { return };
            // O item do PopupMenu: o `on_click` dele (e o `auto_check`), como no menu do form.
            let chosen = ui.get::<MenuList>(list).and_then(|l| l.chosen.get());
            if let Some(item) = chosen.filter(|&i| ui.contains(i)) {
                if let Some(it) = ui.get_mut::<crate::MenuItem>(item).filter(|it| it.auto_check) {
                    it.checked = !it.checked;
                }
                ui.click(item);
                if let Some(menu) = ui.get::<TrayIcon>(id).and_then(|t| t.popup_menu) {
                    ui.queue_event(menu, "on_item_click", EventArgs::Value(aylis_core::PropValue::Component(Some(item))));
                }
            }
            ui.fire(id, "on_menu", &EventArgs::Text(text.clone()));
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
        menu_theme: String => "Appearance",
        popup_menu: Option<ComponentId> => "Behavior";
        events: on_click, on_dbl_click, on_menu
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[MethodInfo::new("show_balloon", &[PropKind::String, PropKind::String], None).params(&["title", "text"])];
        METHODS
    }

    /// `show_balloon(title, text)`: o balão do ícone, pelo
    /// serviço de notificações do sistema (C27 do Tabernáculo).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        match method {
            "show_balloon" => {
                let text = |i: usize| match args.get(i) {
                    Some(aylis_core::PropValue::String(s)) => s.clone(),
                    _ => String::new(),
                };
                let (title, body, id) = (text(0), text(1), ctx.id);
                TrayIcon::show_balloon(ctx.ui, id, &title, &body)?;
                Ok(aylis_core::PropValue::Null)
            }
            _ => Err(format!("método desconhecido: {method}")),
        }
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

/// Uma linha do menu da bandeja.
#[derive(Default)]
struct Entry {
    text: String,
    /// O `MenuItem` (com o `popup_menu`).
    item: Option<ComponentId>,
    enabled: bool,
    checked: bool,
    header: bool,
    separator: bool,
    /// O nível do submenu (recuado).
    depth: usize,
}

impl Entry {
    fn text(text: &str) -> Self {
        Self { separator: text == "-", text: text.to_string(), enabled: true, ..Default::default() }
    }
}

/// Os itens visíveis do menu `parent`, com os submenus logo abaixo de cada item, recuados.
fn menu_entries(ui: &Ui, parent: ComponentId, depth: usize, out: &mut Vec<Entry>) {
    for &child in ui.children(parent) {
        let Some(it) = ui.get::<crate::MenuItem>(child).filter(|it| it.visible) else { continue };
        let has_children = ui.children(child).iter().any(|&c| ui.get::<crate::MenuItem>(c).is_some());
        out.push(Entry {
            text: it.text.replace('&', ""),
            item: Some(child),
            enabled: it.enabled && !has_children,
            checked: it.checked,
            header: it.header || has_children,
            separator: it.is_separator(),
            depth,
        });
        if has_children {
            menu_entries(ui, child, depth + 1, out);
        }
    }
}

impl TrayIcon {
    /// O balão do ícone: título e texto pelo serviço de notificações (sem ele, um aviso na
    /// tela, como a `Notification`).
    pub fn show_balloon(ui: &mut Ui, id: ComponentId, title: &str, text: &str) -> Result<(), String> {
        let app_name = ui
            .main_form()
            .and_then(|f| match ui.get_prop(f, "caption") {
                Ok(aylis_core::PropValue::String(c)) if !c.is_empty() => Some(c),
                _ => None,
            })
            .unwrap_or_else(|| "aylis".into());
        let app_id = format!("aylis.{}", app_name.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>());
        let message = aylis_core::platform::SystemNotification {
            app_id,
            app_name,
            title: title.to_string(),
            body: text.to_string(),
            id: 0x7A1,
            component: Some(id),
            ..Default::default()
        };
        match ui.services_mut().notifications.as_mut() {
            Some(service) => service.notify(&message),
            None => {
                let shown = if text.is_empty() { title.to_string() } else { format!("{title}: {text}") };
                crate::system::toast(ui, &shown);
                Ok(())
            }
        }
    }
}

/// A lista do menu da bandeja (as cores dela, não as do tema).
#[derive(Default)]
struct MenuList {
    base: ControlBase,
    items: Vec<Entry>,
    colors: (Color, Color, Color),
    hover: Cell<Option<usize>>,
    /// O item escolhido (o `MenuItem`, com o `popup_menu`).
    chosen: Cell<Option<ComponentId>>,
}

impl MenuList {
    fn rows(&self) -> Vec<(usize, RectF)> {
        let mut y = 4.0;
        let w = self.base.bounds().width;
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let h = if item.separator { 9.0 } else { 32.0 };
                let r = RectF::new(4.0, y, w - 8.0, h);
                y += h;
                (i, r)
            })
            .collect()
    }

    fn at(&self, p: PointF) -> Option<usize> {
        self.rows().into_iter().find(|(i, r)| r.contains(p) && self.items[*i].enabled && !self.items[*i].separator && !self.items[*i].header).map(|(i, _)| i)
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
                self.chosen.set(self.items[i].item);
                ctx.fire("on_select", EventArgs::Text(self.items[i].text.clone()));
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
            let entry = &self.items[i];
            if entry.separator {
                let y = row.y + row.height / 2.0;
                canvas.draw_line(PointF::new(row.x + 8.0, y), PointF::new(row.right() - 8.0, y), &aylis_core::Stroke::new(fg.with_opacity(0.2), 1.0));
                continue;
            }
            if self.hover.get() == Some(i) {
                canvas.fill_rect(row, 4.0, &Brush::Solid(hover));
            }
            let mut s = t.clone();
            if entry.header || !entry.enabled {
                s.color = fg.with_opacity(0.55);
            }
            if entry.header {
                s.font.weight = aylis_core::FontWeight::SemiBold;
            }
            let left = row.x + 30.0 + entry.depth as f32 * 14.0;
            if entry.checked {
                canvas.draw_text("✓", RectF::new(row.x + 10.0, row.y, 18.0, row.height), &s);
            }
            canvas.draw_text(&entry.text, RectF::new(left, row.y, row.right() - left - 4.0, row.height), &s);
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
