//! O componente [`Microphone`]: liga o microfone, mostra o nível do som e grava em WAV.

use std::cell::RefCell;

use aylis_core::{Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, PaintCtx, PointF, RectF, Ui, properties};

use crate::capture::AudioInput;

/// Microfone com medidor de nível (a barra acende com o som). `active` liga;
/// `Microphone::record`/`stop_recording` gravam em WAV.
pub struct Microphone {
    pub base: ControlBase,
    /// O dispositivo (vazio: o padrão do sistema); `capture::input_devices()` lista.
    pub device: String,
    pub active: bool,
    /// Manda a entrada para uma ou mais saídas (o formato do `output` dos players, com volume
    /// e atraso por saída); vazio: não manda.
    pub monitor: String,
    input: RefCell<Option<AudioInput>>,
    error: RefCell<Option<String>>,
}

impl Default for Microphone {
    fn default() -> Self {
        Self {
            base: ControlBase::new(200.0, 24.0),
            device: String::new(),
            active: false,
            monitor: String::new(),
            input: RefCell::default(),
            error: RefCell::default(),
        }
    }
}

impl Microphone {
    /// O nível do som agora (0 a 1); 0 com o microfone desligado.
    pub fn level(ui: &Ui, id: ComponentId) -> f32 {
        ui.get::<Microphone>(id).and_then(|m| m.input.borrow().as_ref().map(AudioInput::level)).unwrap_or(0.0)
    }

    /// O microfone aberto, para o [`AudioMixer`](crate::AudioMixer) escutar.
    pub(crate) fn tap(ui: &Ui, id: ComponentId) -> Option<crate::output::Tap> {
        ui.get::<Microphone>(id).and_then(|m| m.input.borrow().as_ref().map(AudioInput::tap))
    }

    fn apply_monitor(&self) {
        if let Some(input) = self.input.borrow().as_ref() {
            let specs = if self.monitor.trim().is_empty() { Vec::new() } else { crate::output::parse_outputs(&self.monitor) };
            input.set_monitor(specs);
        }
    }

    /// Liga (ou religa, se o dispositivo mudou) ou desliga, conforme `active`.
    fn sync(&mut self, ctx: &mut EventCtx<'_>) {
        *self.input.borrow_mut() = None;
        *self.error.borrow_mut() = None;
        if self.active && !ctx.ui.design_mode() {
            // Android: sem a permissão, pede (sem travar a tela) e religa quando o usuário
            // responder.
            #[cfg(target_os = "android")]
            if !crate::android_camera::has_permission("android.permission.RECORD_AUDIO") {
                let (poster, id) = (ctx.ui.poster(), ctx.id);
                *self.error.borrow_mut() = Some(aylis_core::messages::t("aguardando a permissão do microfone"));
                std::thread::spawn(move || {
                    if crate::android_camera::ensure_permission("android.permission.RECORD_AUDIO") {
                        poster.post(move |ui| {
                            let _ = ui.set_prop(id, "active", false);
                            let _ = ui.set_prop(id, "active", true);
                        });
                    }
                });
                ctx.repaint();
                return;
            }
            match AudioInput::open(&self.device) {
                Ok(input) => {
                    *self.input.borrow_mut() = Some(input);
                    self.apply_monitor();
                }
                Err(e) => {
                    ctx.fire("on_error", EventArgs::Text(e.clone()));
                    *self.error.borrow_mut() = Some(e);
                }
            }
        }
        ctx.repaint();
    }

    /// Começa a gravar (liga o microfone, se preciso).
    pub fn record(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        if !ui.get::<Microphone>(id).is_some_and(|m| m.active) {
            let _ = ui.set_prop(id, "active", true);
        }
        let m = ui.get::<Microphone>(id).ok_or("não é um Microphone")?;
        if let Some(e) = m.error.borrow().clone() {
            return Err(e);
        }
        m.input.borrow().as_ref().ok_or("microfone desligado")?.start_recording();
        Ok(())
    }

    /// Para de gravar e salva o WAV em `path`; devolve a duração (s).
    pub fn stop_recording(ui: &Ui, id: ComponentId, path: &std::path::Path) -> Result<f64, String> {
        let m = ui.get::<Microphone>(id).ok_or("não é um Microphone")?;
        let input = m.input.borrow();
        input.as_ref().ok_or("microfone desligado")?.stop_recording(path)
    }

    pub fn is_recording(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<Microphone>(id).is_some_and(|m| m.input.borrow().as_ref().is_some_and(AudioInput::is_recording))
    }
}

impl Component for Microphone {
    fn class_name(&self) -> &'static str {
        "Microphone"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        device: String => "Behavior",
        active: bool => "Behavior",
        monitor: String => "Behavior";
        events: on_error
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            "active" | "device" => self.sync(ctx),
            "monitor" => self.apply_monitor(),
            _ => {}
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, r.height / 2.0, &Brush::Solid(theme.color("background")));
        let level = self.input.borrow().as_ref().map_or(0.0, AudioInput::level);
        // Escala logarítmica: a fala comum fica no meio da barra.
        let shown = if level <= 0.0 { 0.0 } else { (1.0 + (level.max(0.001).log10() / 3.0)).clamp(0.0, 1.0) };
        let color = if shown > 0.9 { theme.color("danger") } else if shown > 0.7 { Color::rgb(0xF5, 0x9E, 0x0B) } else { theme.color("success") };
        if shown > 0.0 {
            canvas.fill_rect(RectF::new(r.x, r.y, r.width * shown, r.height), r.height / 2.0, &Brush::Solid(color));
        }
        let recording = self.input.borrow().as_ref().is_some_and(AudioInput::is_recording);
        if recording {
            canvas.fill_ellipse(RectF::new(r.right() - r.height + 6.0, r.y + 6.0, r.height - 12.0, r.height - 12.0), &Brush::Solid(theme.color("danger")));
        }
        if let Some(e) = self.error.borrow().as_ref() {
            let mut t = ctx.text_style();
            t.color = theme.color("danger");
            t.font.size = (t.font.size - 2.0).max(9.0);
            canvas.draw_text(&format!("⚠ {e}"), r.inflate(-6.0), &t);
        }
        // Ligado: o medidor anda (30 quadros por segundo bastam).
        if self.input.borrow().is_some() {
            ctx.request_frame_at(ctx.time + 1.0 / 30.0);
        }
        let _ = PointF::ZERO;
    }
}
