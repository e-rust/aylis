//! `TimePicker`: hora pelo relógio de ponteiros (o `v-time-picker` e o seletor de hora do
//! Material). Primeiro as horas (anel de fora 1–12, de dentro 13–00), depois os minutos;
//! arrastar o ponteiro ou clicar. O `TimeEdit` abre um destes no botão do relógio.

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Key, KeyEvent, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::leaf_control;

/// Mensagem do relógio suspenso para o `TimeEdit`: a hora escolhida (segundos do dia) e
/// se terminou (escolheu os minutos).
#[derive(Debug)]
pub(crate) struct PickedTime(pub u32, pub bool);

const HEADER: f32 = 64.0;

/// Seletor de hora com relógio. `time` é `14:30`; `on_change` a cada mudança.
pub struct TimePicker {
    pub base: ControlBase,
    pub time: String,
    /// Mostrando os minutos (senão, as horas).
    pub minutes_mode: bool,
    dragging: Cell<bool>,
    /// Aberto por um `TimeEdit`: avisa ele.
    pub(crate) owner: Option<ComponentId>,
}

impl Default for TimePicker {
    fn default() -> Self {
        let mut base = ControlBase::new(280.0, 340.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, time: "14:30".into(), minutes_mode: false, dragging: Cell::new(false), owner: None }
    }
}

impl TimePicker {
    pub fn new(time: &str) -> Self {
        Self { time: time.to_string(), ..Default::default() }
    }

    /// Hora e minuto.
    pub fn hm(&self) -> (u32, u32) {
        let secs = crate::TimeEdit::parse(&self.time).unwrap_or(0);
        (secs / 3600, secs / 60 % 60)
    }

    fn dial(&self) -> (PointF, f32) {
        let b = self.base.bounds();
        let side = (b.width - 24.0).min(b.height - HEADER - 16.0).max(40.0);
        (PointF::new(b.width / 2.0, HEADER + 8.0 + side / 2.0), side / 2.0)
    }

    /// O valor no ponto do relógio: hora (0–23) ou minuto (0–59).
    fn value_at(&self, p: PointF) -> Option<u32> {
        let (c, r) = self.dial();
        let (dx, dy) = (p.x - c.x, p.y - c.y);
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > r + 8.0 || dist < 8.0 {
            return None;
        }
        // Ângulo a partir do meio-dia, no sentido horário.
        let angle = dx.atan2(-dy).to_degrees().rem_euclid(360.0);
        if self.minutes_mode {
            return Some((angle / 6.0).round() as u32 % 60);
        }
        let h = (angle / 30.0).round() as u32 % 12;
        // Anel de dentro: 13–23 e 00; de fora: 1–12.
        let inner = dist < r * 0.62;
        Some(match (inner, h) {
            (false, 0) => 12,
            (false, h) => h,
            (true, 0) => 0,
            (true, h) => h + 12,
        })
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, h: u32, m: u32, done: bool) {
        let text = format!("{h:02}:{m:02}");
        if text != self.time {
            self.time = text;
            if self.owner.is_none() {
                ctx.fire("on_change", EventArgs::Text(self.time.clone()));
            }
        }
        if let Some(owner) = self.owner {
            ctx.ui.send_message(owner, &PickedTime(h * 3600 + m * 60, done));
        }
        ctx.repaint();
    }

    fn pick(&mut self, ctx: &mut EventCtx<'_>, p: PointF, done: bool) {
        let Some(v) = self.value_at(p) else { return };
        let (h, m) = self.hm();
        if self.minutes_mode { self.set(ctx, h, v, done) } else { self.set(ctx, v, m, false) }
    }
}

impl Component for TimePicker {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        self.owner.is_none().then(|| EventArgs::Text(self.time.clone()))
    }

    leaf_control!("TimePicker");

    properties! {
        time: String => "Data",
        minutes_mode: bool => "Behavior";
        events: on_change
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let w = self.base.bounds().width;
        if event.position.y < HEADER {
            // No cabeçalho: a hora (esquerda) ou os minutos (direita).
            self.minutes_mode = event.position.x > w / 2.0;
            ctx.repaint();
            return;
        }
        self.dragging.set(true);
        self.pick(ctx, event.position, false);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging.get() {
            self.pick(ctx, event.position, false);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if !self.dragging.replace(false) {
            return;
        }
        if self.minutes_mode {
            self.pick(ctx, event.position, true);
        } else if self.value_at(event.position).is_some() {
            // Escolheu a hora: passa para os minutos.
            self.minutes_mode = true;
            ctx.repaint();
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let (h, m) = self.hm();
        let (h, m) = match (event.key, self.minutes_mode) {
            (Key::Up, false) => ((h + 1) % 24, m),
            (Key::Down, false) => ((h + 23) % 24, m),
            (Key::Up, true) => (h, (m + 1) % 60),
            (Key::Down, true) => (h, (m + 59) % 60),
            (Key::Left | Key::Right | Key::Tab, _) => {
                self.minutes_mode = !self.minutes_mode;
                ctx.set_handled();
                return ctx.repaint();
            }
            (Key::Enter, _) => {
                self.set(ctx, h, m, true);
                return ctx.set_handled();
            }
            _ => return,
        };
        self.set(ctx, h, m, false);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        let primary = theme.color("primary");
        if self.owner.is_some() {
            canvas.draw_shadow(r, 12.0, &aylis_core::Shadow::new(aylis_core::Color::rgba(0, 0, 0, 50), 14.0, 0.0, 4.0));
        }
        canvas.fill_rect(r, 12.0, &Brush::Solid(theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), 12.0, &Stroke::new(theme.color("border"), 1.0));
        let (h, m) = self.hm();
        // Cabeçalho: [14] : [30], o campo ativo em destaque.
        let mut big = ctx.text_style();
        big.font.size = 32.0;
        big.font.weight = FontWeight::SemiBold;
        big.h_align = TextAlign::Center;
        let cw = 76.0;
        let cx = r.width / 2.0;
        for (i, (text, active)) in [(format!("{h:02}"), !self.minutes_mode), (format!("{m:02}"), self.minutes_mode)].iter().enumerate() {
            let box_r = RectF::new(if i == 0 { cx - cw - 10.0 } else { cx + 10.0 }, 10.0, cw, 48.0);
            canvas.fill_rect(box_r, 8.0, &Brush::Solid(if *active { primary.with_opacity(0.16) } else { theme.color("surface_2") }));
            let mut t = big.clone();
            t.color = if *active { primary } else { theme.color("text") };
            canvas.draw_text(text, box_r, &t);
        }
        canvas.draw_text(":", RectF::new(cx - 10.0, 10.0, 20.0, 48.0), &big);
        // Mostrador.
        let (c, radius) = self.dial();
        canvas.fill_ellipse(RectF::new(c.x - radius, c.y - radius, 2.0 * radius, 2.0 * radius), &Brush::Solid(theme.color("surface_2")));
        let point = |angle: f32, dist: f32| {
            let a = (angle - 90.0).to_radians();
            PointF::new(c.x + dist * a.cos(), c.y + dist * a.sin())
        };
        // O ponteiro até o valor atual.
        let (angle, dist) = if self.minutes_mode {
            (m as f32 * 6.0, radius - 20.0)
        } else {
            let inner = h == 0 || h > 12;
            ((h % 12) as f32 * 30.0, if inner { radius * 0.5 } else { radius - 20.0 })
        };
        let tip = point(angle, dist);
        canvas.draw_line(c, tip, &Stroke::new(primary, 2.0));
        canvas.fill_ellipse(RectF::new(c.x - 3.0, c.y - 3.0, 6.0, 6.0), &Brush::Solid(primary));
        canvas.fill_ellipse(RectF::new(tip.x - 18.0, tip.y - 18.0, 36.0, 36.0), &Brush::Solid(primary));
        let mut num = ctx.text_style();
        num.h_align = TextAlign::Center;
        let on = theme.color("on_primary");
        let label = |canvas: &mut dyn Canvas, text: &str, at: PointF, selected: bool, small: bool| {
            let mut t = num.clone();
            if small {
                t.font.size = (t.font.size - 2.0).max(9.0);
                t.color = theme.color("text_muted");
            }
            if selected {
                t.color = on;
            }
            canvas.draw_text(text, RectF::new(at.x - 16.0, at.y - 12.0, 32.0, 24.0), &t);
        };
        if self.minutes_mode {
            for i in 0..12u32 {
                let v = i * 5;
                label(canvas, &format!("{v:02}"), point(v as f32 * 6.0, radius - 20.0), v == m, false);
            }
        } else {
            for i in 1..=12u32 {
                label(canvas, &i.to_string(), point(i as f32 * 30.0, radius - 20.0), i == h, false);
                let inner = if i == 12 { 0 } else { i + 12 };
                label(canvas, &format!("{inner:02}"), point(i as f32 * 30.0, radius * 0.5), inner == h, true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_to_value() {
        let mut ui = aylis_core::Ui::new();
        let form = ui.add_root(crate::Form::new("f", 400.0, 400.0));
        let id = ui.add(TimePicker::default(), form);
        ui.control_mut(id).unwrap().set_rect(0.0, 0.0, 280.0, 340.0);
        ui.layout();
        let p = ui.get_mut::<TimePicker>(id).unwrap();
        let (c, r) = p.dial();
        // Em cima, por fora: 12; por dentro: 00. À direita, fora: 3; dentro: 15.
        assert_eq!(p.value_at(PointF::new(c.x, c.y - r + 10.0)), Some(12));
        assert_eq!(p.value_at(PointF::new(c.x, c.y - r * 0.4)), Some(0));
        assert_eq!(p.value_at(PointF::new(c.x + r - 10.0, c.y)), Some(3));
        assert_eq!(p.value_at(PointF::new(c.x + r * 0.4, c.y)), Some(15));
        p.minutes_mode = true;
        assert_eq!(p.value_at(PointF::new(c.x, c.y + r - 10.0)), Some(30));
        assert_eq!(p.hm(), (14, 30));
    }
}
