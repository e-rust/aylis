//! Ícones na bandeja (os `TrayIcon` do app), pelo crate `tray-icon`: cria, troca o ícone
//! (quadros da animação, indicador desenhado por cima), a dica e a visibilidade; os cliques
//! voltam ao componente como [`TrayEvent`](aylis_core::TrayEvent).

use std::collections::HashMap;
use std::sync::Mutex;

use aylis_core::{ComponentId, PointF, TrayEvent, TrayEventKind, TraySpec, Ui};
use tray_icon::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::EventLoopProxy;

static EVENTS: Mutex<Vec<(u64, TrayEventKind, f64, f64)>> = Mutex::new(Vec::new());

struct Hosted {
    icon: TrayIcon,
    /// O que está na tela: (quadro, indicador, cor, primeiros bytes do quadro, tamanho em
    /// pixels) e a dica.
    shown: (usize, String, aylis_core::Color, usize, u32),
    tooltip: String,
    visible: bool,
}

#[derive(Default)]
pub(crate) struct Trays {
    hosted: HashMap<ComponentId, Hosted>,
    handler: bool,
}

/// O ícone padrão (sem arquivo): um círculo azul.
fn default_icon() -> (Vec<u8>, u32, u32) {
    let n = 32u32;
    let mut px = vec![0u8; (n * n * 4) as usize];
    for y in 0..n {
        for x in 0..n {
            let (dx, dy) = (x as f32 - 15.5, y as f32 - 15.5);
            if dx * dx + dy * dy <= 14.0 * 14.0 {
                let i = ((y * n + x) * 4) as usize;
                px[i..i + 4].copy_from_slice(&[0x25, 0x63, 0xEB, 255]);
            }
        }
    }
    (px, n, n)
}

/// O tamanho do ícone da bandeja em pixels, na escala `scale` (o `SM_CXSMICON` do Windows: 16
/// a 100%, 24 a 150%; a barra do macOS e a do Linux usam 22).
fn tray_pixels(scale: f32) -> u32 {
    let base = if cfg!(windows) { 16.0 } else { 22.0 };
    (base * scale.max(1.0)).round() as u32
}

fn make_icon(spec: &TraySpec, frame: usize, scale: f32) -> Option<tray_icon::Icon> {
    // Um `.ico` com vários tamanhos: o quadro do tamanho da bandeja nesta escala (C41).
    let decode = |b: &std::sync::Arc<Vec<u8>>| match aylis_core::image::ico_frame_for_size(b, tray_pixels(scale)) {
        Some(one) => crate::decode_rgba(&one),
        None => crate::decode_rgba(b),
    };
    let (mut px, w, h) = spec.icons.get(frame).and_then(decode).unwrap_or_else(default_icon);
    crate::badge::draw(&mut px, w, h, &spec.badge, spec.badge_color, 0.6);
    tray_icon::Icon::from_rgba(px, w, h).ok()
}

impl Trays {
    /// Cria, atualiza e tira os ícones conforme os componentes; entrega os cliques. Devolve
    /// quando a animação precisa do próximo quadro (segundos desde o início).
    pub(crate) fn sync(&mut self, ui: &mut Ui, proxy: &EventLoopProxy<()>, now: f64, scale: f32) -> Option<f64> {
        if !self.handler {
            self.handler = true;
            let proxy = std::sync::Mutex::new(proxy.clone());
            TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
                let (id, kind, p) = match event {
                    TrayIconEvent::Click { id, position, button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => (id, TrayEventKind::Click, position),
                    TrayIconEvent::Click { id, position, button: MouseButton::Right, button_state: MouseButtonState::Up, .. } => {
                        (id, TrayEventKind::RightClick, position)
                    }
                    TrayIconEvent::DoubleClick { id, position, button: MouseButton::Left, .. } => (id, TrayEventKind::DoubleClick, position),
                    _ => return,
                };
                let Ok(id) = id.0.parse::<u64>() else { return };
                if let Ok(mut q) = EVENTS.lock() {
                    q.push((id, kind, p.x, p.y));
                }
                if let Ok(p) = proxy.lock() {
                    let _ = p.send_event(());
                }
            }));
        }
        let specs = ui.trays();
        let mut next: Option<f64> = None;
        for (id, spec) in &specs {
            let frames = spec.icons.len().max(1);
            let interval = (spec.interval_ms.max(16) as f64) / 1000.0;
            let frame = if frames > 1 { ((now / interval) as usize) % frames } else { 0 };
            if frames > 1 && spec.visible {
                let at = ((now / interval).floor() + 1.0) * interval;
                next = Some(next.map_or(at, |n: f64| n.min(at)));
            }
            let key = (frame, spec.badge.clone(), spec.badge_color, spec.icons.get(frame).map_or(0, |b| b.len()), tray_pixels(scale));
            match self.hosted.get_mut(id) {
                Some(h) => {
                    if h.shown != key {
                        let _ = h.icon.set_icon(make_icon(spec, frame, scale));
                        h.shown = key;
                    }
                    if h.tooltip != spec.tooltip {
                        let _ = h.icon.set_tooltip(Some(&spec.tooltip));
                        h.tooltip = spec.tooltip.clone();
                    }
                    if h.visible != spec.visible {
                        let _ = h.icon.set_visible(spec.visible);
                        h.visible = spec.visible;
                    }
                }
                None => {
                    let mut builder = TrayIconBuilder::new().with_id(id.to_u64().to_string());
                    if let Some(icon) = make_icon(spec, frame, scale) {
                        builder = builder.with_icon(icon);
                    }
                    if !spec.tooltip.is_empty() {
                        builder = builder.with_tooltip(&spec.tooltip);
                    }
                    match builder.build() {
                        Ok(icon) => {
                            let _ = icon.set_visible(spec.visible);
                            self.hosted.insert(*id, Hosted { icon, shown: key, tooltip: spec.tooltip.clone(), visible: spec.visible });
                        }
                        Err(e) => aylis_core::stderr_line!("aylis: TrayIcon: {e}"),
                    }
                }
            }
        }
        self.hosted.retain(|id, _| specs.iter().any(|(s, _)| s == id));
        let events = EVENTS.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default();
        for (id, kind, x, y) in events {
            let id = ComponentId::from_u64(id);
            ui.send_message(id, &TrayEvent { kind, screen: PointF::new(x as f32, y as f32), scale });
        }
        next
    }
}
