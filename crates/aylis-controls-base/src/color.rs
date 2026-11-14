//! Cores: `ColorPicker` (quadro de saturação e brilho, matiz, transparência e as amostras)
//! e `ColorEdit` (amostra + `#rrggbb`, abre o seletor ao clicar).

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, GradientStop, PaintCtx, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::leaf_control;

/// Matiz (0–360), saturação e brilho (0–1).
pub fn to_hsv(c: Color) -> (f32, f32, f32) {
    let (r, g, b) = (c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, if max == 0.0 { 0.0 } else { d / max }, max)
}

pub fn from_hsv(h: f32, s: f32, v: f32, a: u8) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h.rem_euclid(360.0) / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to = |f: f32| ((f + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    Color::rgba(to(r), to(g), to(b), a)
}

/// `#rrggbb` (ou `#rrggbbaa` com transparência).
pub fn hex(c: Color) -> String {
    if c.a == 255 { format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b) } else { format!("#{:02X}{:02X}{:02X}{:02X}", c.r, c.g, c.b, c.a) }
}

const SWATCHES: [u32; 12] = [0xF44336, 0xE91E63, 0x9C27B0, 0x3F51B5, 0x2196F3, 0x00BCD4, 0x009688, 0x4CAF50, 0xFFC107, 0xFF9800, 0x795548, 0x607D8B];

#[derive(Clone, Copy, PartialEq)]
enum Part {
    Square,
    Hue,
    Alpha,
}

// ===========================================================================
// ColorPicker
// ===========================================================================

/// Seletor de cor (o `v-color-picker`): quadro de saturação × brilho, faixa de matiz,
/// faixa de transparência (`alpha`), `#hex` e amostras prontas. `on_change` com a cor.
pub struct ColorPicker {
    pub base: ControlBase,
    /// A cor escolhida.
    pub color: Color,
    /// Mostra a faixa de transparência (sem ela, a cor fica opaca).
    pub alpha: bool,
    /// Mostra as amostras prontas embaixo.
    pub swatches: bool,
    /// Matiz guardado à parte (cinzas não têm matiz: mexer no quadro não o perde).
    hue: Cell<f32>,
    dragging: Cell<Option<Part>>,
    /// Quem abriu o seletor (o `ColorEdit`), que recebe a cor.
    pub(crate) owner: Option<ComponentId>,
}

impl Default for ColorPicker {
    fn default() -> Self {
        let mut base = ControlBase::new(260.0, 300.0);
        base.set_cursor(CursorShape::Crosshair);
        let color = Color::rgb(0x21, 0x96, 0xF3);
        Self { base, color, alpha: true, swatches: true, hue: Cell::new(to_hsv(color).0), dragging: Cell::new(None), owner: None }
    }
}

struct Areas {
    square: RectF,
    hue: RectF,
    alpha: RectF,
    preview: RectF,
    swatches: RectF,
}

impl ColorPicker {
    fn areas(&self, w: f32) -> Areas {
        let pad = 10.0;
        let square = RectF::new(pad, pad, w - 2.0 * pad, 150.0);
        let bar_x = pad + 40.0;
        let hue = RectF::new(bar_x, square.bottom() + 12.0, w - bar_x - pad, 12.0);
        let alpha = RectF::new(bar_x, hue.bottom() + 10.0, w - bar_x - pad, if self.alpha { 12.0 } else { 0.0 });
        let preview = RectF::new(pad, hue.y - 2.0, 30.0, if self.alpha { 36.0 } else { 16.0 });
        let swatches = RectF::new(pad, alpha.bottom() + 36.0, w - 2.0 * pad, 24.0);
        Areas { square, hue, alpha, preview, swatches }
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, c: Color) {
        if c != self.color {
            self.color = c;
            ctx.fire("on_change", EventArgs::Value(hex(c).into()));
            if let Some(owner) = self.owner {
                let _ = ctx.ui.set_prop_quiet(owner, "color", c);
                ctx.ui.fire(owner, "on_change", &EventArgs::Value(hex(c).into()));
            }
        }
        ctx.repaint();
    }

    fn drag(&mut self, ctx: &mut EventCtx<'_>, part: Part, p: PointF) {
        let a = self.areas(self.base.bounds().width);
        let (_, s, v) = to_hsv(self.color);
        let frac = |r: RectF, x: f32| ((x - r.x) / r.width.max(1.0)).clamp(0.0, 1.0);
        let c = match part {
            Part::Square => {
                let s = frac(a.square, p.x);
                let v = 1.0 - ((p.y - a.square.y) / a.square.height).clamp(0.0, 1.0);
                from_hsv(self.hue.get(), s, v, self.color.a)
            }
            Part::Hue => {
                self.hue.set(frac(a.hue, p.x) * 360.0);
                from_hsv(self.hue.get(), s, v, self.color.a)
            }
            Part::Alpha => Color { a: (frac(a.alpha, p.x) * 255.0).round() as u8, ..self.color },
        };
        self.set(ctx, c);
    }

    fn sync(&mut self, name: &str) {
        if name == "color" {
            let (h, s, _) = to_hsv(self.color);
            if s > 0.0 {
                self.hue.set(h);
            }
        }
    }
}

impl Component for ColorPicker {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Value(hex(self.color).into()))
    }

    leaf_control!("ColorPicker");

    properties! {
        color: Color => "Data",
        alpha: bool => "Appearance",
        swatches: bool => "Appearance";
        events: on_change;
        on_set: sync
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let a = self.areas(self.base.bounds().width);
        let p = event.position;
        let part = if a.square.contains(p) {
            Some(Part::Square)
        } else if a.hue.inflate(4.0).contains(p) {
            Some(Part::Hue)
        } else if self.alpha && a.alpha.inflate(4.0).contains(p) {
            Some(Part::Alpha)
        } else {
            None
        };
        if let Some(part) = part {
            self.dragging.set(Some(part));
            self.drag(ctx, part, p);
        } else if self.swatches && a.swatches.contains(p) {
            let w = a.swatches.width / SWATCHES.len() as f32;
            let i = ((p.x - a.swatches.x) / w) as usize;
            if let Some(v) = SWATCHES.get(i) {
                let c = Color::rgb((v >> 16) as u8, (v >> 8) as u8, *v as u8);
                self.hue.set(to_hsv(c).0);
                self.set(ctx, c);
            }
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(part) = self.dragging.get() {
            self.drag(ctx, part, event.position);
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging.set(None);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), 8.0, &Stroke::new(ctx.theme.color("border"), 1.0));
        let a = self.areas(r.width);
        let (_, s, v) = to_hsv(self.color);
        let hue = self.hue.get();
        // Quadro: branco → matiz na horizontal, transparente → preto na vertical.
        let pure = from_hsv(hue, 1.0, 1.0, 255);
        canvas.fill_rect(a.square, 6.0, &Brush::Linear { start: PointF::new(a.square.x, 0.0), end: PointF::new(a.square.right(), 0.0), stops: vec![GradientStop::new(0.0, Color::WHITE), GradientStop::new(1.0, pure)] });
        canvas.fill_rect(a.square, 6.0, &Brush::Linear { start: PointF::new(0.0, a.square.y), end: PointF::new(0.0, a.square.bottom()), stops: vec![GradientStop::new(0.0, Color::TRANSPARENT), GradientStop::new(1.0, Color::BLACK)] });
        let knob = PointF::new(a.square.x + s * a.square.width, a.square.y + (1.0 - v) * a.square.height);
        canvas.stroke_ellipse(RectF::new(knob.x - 7.0, knob.y - 7.0, 14.0, 14.0), &Stroke::new(Color::WHITE, 2.0));
        canvas.stroke_ellipse(RectF::new(knob.x - 8.0, knob.y - 8.0, 16.0, 16.0), &Stroke::new(Color::rgba(0, 0, 0, 90), 1.0));
        // Matiz.
        let stops = (0..=6).map(|i| GradientStop::new(i as f32 / 6.0, from_hsv(i as f32 * 60.0, 1.0, 1.0, 255))).collect();
        canvas.fill_rect(a.hue, 6.0, &Brush::Linear { start: PointF::new(a.hue.x, 0.0), end: PointF::new(a.hue.right(), 0.0), stops });
        let thumb = |canvas: &mut dyn Canvas, bar: RectF, f: f32| {
            let c = PointF::new(bar.x + f * bar.width, bar.y + bar.height / 2.0);
            canvas.fill_ellipse(RectF::new(c.x - 8.0, c.y - 8.0, 16.0, 16.0), &Brush::Solid(Color::WHITE));
            canvas.stroke_ellipse(RectF::new(c.x - 8.0, c.y - 8.0, 16.0, 16.0), &Stroke::new(Color::rgba(0, 0, 0, 70), 1.0));
        };
        thumb(canvas, a.hue, hue / 360.0);
        // Transparência, sobre o xadrez.
        let checker = |canvas: &mut dyn Canvas, rect: RectF| {
            canvas.save();
            canvas.clip_round_rect(rect, 6.0);
            canvas.fill_rect(rect, 0.0, &Brush::Solid(Color::WHITE));
            let n = (rect.width / 6.0).ceil() as i32;
            let m = (rect.height / 6.0).ceil() as i32;
            for i in 0..n {
                for j in 0..m {
                    if (i + j) % 2 == 0 {
                        canvas.fill_rect(RectF::new(rect.x + i as f32 * 6.0, rect.y + j as f32 * 6.0, 6.0, 6.0), 0.0, &Brush::Solid(Color::rgb(0xDD, 0xDD, 0xDD)));
                    }
                }
            }
            canvas.restore();
        };
        if self.alpha {
            checker(canvas, a.alpha);
            let solid = Color { a: 255, ..self.color };
            canvas.fill_rect(a.alpha, 6.0, &Brush::Linear { start: PointF::new(a.alpha.x, 0.0), end: PointF::new(a.alpha.right(), 0.0), stops: vec![GradientStop::new(0.0, Color { a: 0, ..solid }), GradientStop::new(1.0, solid)] });
            thumb(canvas, a.alpha, self.color.a as f32 / 255.0);
        }
        checker(canvas, a.preview);
        canvas.fill_rect(a.preview, 6.0, &Brush::Solid(self.color));
        // O hex embaixo.
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.font = ctx.monospace_font(t.font.size);
        canvas.draw_text(&hex(self.color), RectF::new(r.x, a.alpha.bottom().max(a.preview.bottom()) + 6.0, r.width, 22.0), &t);
        if self.swatches {
            let w = a.swatches.width / SWATCHES.len() as f32;
            for (i, v) in SWATCHES.iter().enumerate() {
                let c = Color::rgb((v >> 16) as u8, (v >> 8) as u8, *v as u8);
                let sr = RectF::new(a.swatches.x + i as f32 * w + 2.0, a.swatches.y, w - 4.0, a.swatches.height);
                canvas.fill_rect(sr, 4.0, &Brush::Solid(c));
            }
        }
    }
}

// ===========================================================================
// ColorEdit
// ===========================================================================

/// Campo de cor (o `v-color-input`): amostra e `#rrggbb`; o clique abre um
/// [`ColorPicker`] logo abaixo (fecha ao clicar de novo ou ao sair do campo).
pub struct ColorEdit {
    pub base: ControlBase,
    /// A cor escolhida.
    pub color: Color,
    /// O seletor que abre mostra a faixa de transparência (sem ela, a cor fica opaca).
    pub alpha: bool,
    popup: Option<ComponentId>,
}

impl Default for ColorEdit {
    fn default() -> Self {
        let mut base = ControlBase::new(180.0, 34.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, color: Color::rgb(0x21, 0x96, 0xF3), alpha: false, popup: None }
    }
}

impl ColorEdit {
    fn close(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(p) = self.popup.take() {
            ctx.ui.free(p);
            ctx.repaint();
        }
    }
}

impl Component for ColorEdit {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Value(hex(self.color).into()))
    }

    leaf_control!("ColorEdit");

    properties! {
        color: Color => "Data",
        alpha: bool => "Appearance";
        events: on_change
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if self.popup.is_some() {
            return self.close(ctx);
        }
        let root = ctx.ui.root_of(ctx.id);
        let rect = ctx.ui.to_form(ctx.id, self.base.bounds());
        let picker = ColorPicker { color: self.color, alpha: self.alpha, hue: Cell::new(to_hsv(self.color).0), owner: Some(ctx.id), ..Default::default() };
        let id = ctx.ui.add(picker, root);
        if let Some(cb) = ctx.ui.control_mut(id) {
            cb.set_rect(rect.x, rect.bottom() + 4.0, 260.0, if self.alpha { 300.0 } else { 280.0 });
        }
        ctx.ui.follow(id, ctx.id);
        self.popup = Some(id);
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        // O clique no seletor não tira o foco (ele não é focável); sair para outro campo fecha.
        self.close(ctx);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let r = ctx.rect;
        canvas.fill_rect(r, s.radius.max(4.0), &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(r.inflate(-0.5), s.radius.max(4.0), &Stroke::new(if ctx.state.focused { ctx.theme.color("primary") } else { ctx.theme.color("border") }, 1.0));
        let sw = RectF::new(r.x + 6.0, r.y + 6.0, r.height - 12.0, r.height - 12.0);
        canvas.fill_rect(sw, 4.0, &Brush::Solid(self.color));
        canvas.stroke_rect(sw, 4.0, &Stroke::new(ctx.theme.color("border"), 1.0));
        let mut t = ctx.text_style();
        t.font = ctx.monospace_font(t.font.size);
        canvas.draw_text(&hex(self.color), RectF::new(sw.right() + 8.0, r.y, r.width - sw.width - 40.0, r.height), &t);
        ctx.draw_icon(canvas, "chevron-down", RectF::new(r.right() - 24.0, r.y + (r.height - 16.0) / 2.0, 16.0, 16.0), ctx.theme.color("text_muted"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_round_trip() {
        for c in [Color::rgb(0x21, 0x96, 0xF3), Color::rgb(255, 0, 0), Color::rgb(10, 200, 30), Color::rgb(128, 128, 128)] {
            let (h, s, v) = to_hsv(c);
            assert_eq!(from_hsv(h, s, v, 255), c);
        }
        assert_eq!(hex(Color::rgba(1, 2, 3, 4)), "#01020304");
    }
}
