//! Medidor: um pacote de código (tipo B) de exemplo. O crate registra a classe `PressureDial`
//! (`register`), e o projeto Rust que usa o pacote chama `medidor_aylis::register(ui)` no
//! código gerado. No form, ele aparece como um frame com `form = "pkg:Medidor"`; as
//! propriedades expostas (`valor`, `limite`, `titulo`) caem nas do `PressureDial` e o evento
//! `on_alarme` é o `on_alarm` dele.

use aylis_core::{Canvas, Component, ControlBase, EventArgs, EventCtx, LineCap, PaintCtx, Path, PointF, RectF, Stroke, TextAlign, Ui, properties};

/// Mostrador de 0 a `max`, com o arco vermelho acima de `alarm`. Passar do limite dispara
/// `on_alarm` (uma vez, até voltar para baixo).
pub struct PressureDial {
    pub base: ControlBase,
    pub value: f32,
    pub max: f32,
    pub alarm: f32,
    pub caption: String,
    alarmed: bool,
}

impl Default for PressureDial {
    fn default() -> Self {
        Self { base: ControlBase::new(160.0, 160.0), value: 0.0, max: 100.0, alarm: 80.0, caption: "Pressão".into(), alarmed: false }
    }
}

impl Component for PressureDial {
    fn class_name(&self) -> &'static str {
        "PressureDial"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        value: f32 => "Behavior",
        max: f32 => "Behavior",
        alarm: f32 => "Behavior",
        caption: String => "Appearance";
        events: on_alarm
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "value" | "alarm") {
            let over = self.value > self.alarm;
            if over && !self.alarmed {
                ctx.fire("on_alarm", EventArgs::None);
            }
            self.alarmed = over;
        }
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let size = r.width.min(r.height);
        let center = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let radius = size / 2.0 - 10.0;
        let max = self.max.max(0.001);
        // O arco de 270°, de baixo à esquerda a baixo à direita.
        let (start, sweep) = (135.0, 270.0);
        let arc = |from: f32, to: f32| {
            let mut p = Path::new();
            p.arc(center, radius, radius, start + sweep * from, sweep * (to - from));
            p
        };
        let mut track = Stroke::new(ctx.theme.color("border"), 10.0);
        track.cap = LineCap::Round;
        canvas.stroke_path(&arc(0.0, 1.0), &track);
        let limit = (self.alarm / max).clamp(0.0, 1.0);
        let mut danger = Stroke::new(ctx.theme.color("danger").with_opacity(0.35), 10.0);
        danger.cap = LineCap::Round;
        canvas.stroke_path(&arc(limit, 1.0), &danger);
        let level = (self.value / max).clamp(0.0, 1.0);
        let color = if self.value > self.alarm { ctx.theme.color("danger") } else { ctx.theme.color("primary") };
        let mut fill = Stroke::new(color, 10.0);
        fill.cap = LineCap::Round;
        if level > 0.0 {
            canvas.stroke_path(&arc(0.0, level), &fill);
        }
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        text.v_align = TextAlign::Center;
        text.font.size = size / 5.0;
        canvas.draw_text(&format!("{:.0}", self.value), RectF::new(r.x, center.y - size / 6.0, r.width, size / 3.0), &text);
        text.font.size = (size / 12.0).max(9.0);
        text.color = ctx.theme.color("text_muted");
        canvas.draw_text(&self.caption, RectF::new(r.x, center.y + size / 6.0, r.width, size / 6.0), &text);
    }
}

/// Registra as classes do pacote (o código gerado do projeto chama).
pub fn register(ui: &mut Ui) {
    ui.register_class::<PressureDial>("Industrial");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passing_the_limit_fires_the_alarm_once() {
        let mut ui = Ui::new();
        register(&mut ui);
        let g = ui.create("PressureDial", None).unwrap();
        let fired = std::rc::Rc::new(std::cell::Cell::new(0));
        let f = fired.clone();
        ui.on(g, "on_alarm", move |_, _, _| f.set(f.get() + 1));
        for v in [50.0, 85.0, 90.0, 40.0, 95.0] {
            ui.set_prop(g, "value", v as f32).unwrap();
            ui.process_events();
        }
        assert_eq!(fired.get(), 2, "ao passar de 80, duas vezes");
    }
}
