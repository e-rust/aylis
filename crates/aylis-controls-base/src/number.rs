//! Números: NumberBox (campo com setas) e TrackBar (controle deslizante).

use aylis_core::{
    Brush, Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx,
    PointF, PointerButton, PointerEvent, RectF, Stroke, WheelEvent, properties,
};

use crate::{Edit, leaf_control};

/// Largura da área das setas do NumberBox.
const SPIN: f32 = 22.0;

/// Campo numérico com setas.
///
/// - `value` entre `min` e `max`, com `decimals` casas (no formato do idioma: `1234,5`);
/// - setas, roda do mouse (com o foco), ↑/↓ e PageUp/PageDown somam `increment`;
/// - o texto digitado vale ao sair do campo ou com Enter; `on_change` traz o valor.
pub struct NumberBox {
    pub edit: Edit,
    /// O valor, sempre entre `min` e `max`.
    pub value: f64,
    /// O menor valor aceito.
    pub min: f64,
    /// O maior valor aceito.
    pub max: f64,
    /// Quanto as setas, a roda e ↑/↓ somam ou tiram.
    pub increment: f64,
    /// Casas decimais mostradas (0: inteiro).
    pub decimals: u32,
    /// Separador decimal do idioma (lido do `Ui` no primeiro layout).
    separator: char,
}

impl Default for NumberBox {
    fn default() -> Self {
        let mut edit = Edit::default();
        edit.base.set_width(120.0);
        edit.filter = Some(|c| c.is_ascii_digit() || matches!(c, ',' | '.' | '-' | '+'));
        edit.silent = true;
        edit.right_inset = SPIN;
        let mut n = Self { edit, value: 0.0, min: 0.0, max: 100.0, increment: 1.0, decimals: 0, separator: '.' };
        n.show();
        n
    }
}

impl NumberBox {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        let mut n = Self { min, max, ..Default::default() };
        n.value = n.clamp(value);
        n.show();
        n
    }

    fn clamp(&self, v: f64) -> f64 {
        let (lo, hi) = if self.min <= self.max { (self.min, self.max) } else { (self.max, self.min) };
        let factor = 10f64.powi(self.decimals.min(12) as i32);
        ((v * factor).round() / factor).clamp(lo, hi)
    }

    /// O valor formatado (`1234,50`).
    pub fn format(&self, v: f64) -> String {
        format!("{v:.*}", self.decimals.min(12) as usize).replace('.', &self.separator.to_string())
    }

    fn show(&mut self) {
        self.edit.text = self.format(self.value);
    }

    fn after_set(&mut self, name: &str) {
        if name != "text" {
            self.value = self.clamp(self.value);
            self.show();
        }
    }

    /// Lê o texto (aceita `,` ou `.` como decimal).
    pub fn parse(text: &str) -> Option<f64> {
        let t = text.trim().replace(' ', "");
        if t.is_empty() {
            return None;
        }
        // "1.234,5" (pt-BR) e "1,234.5" (en-US): o último separador é o decimal.
        let last = t.rfind([',', '.']);
        let normalized: String = match last {
            Some(i) => {
                let (int, frac) = t.split_at(i);
                format!("{}.{}", int.replace([',', '.'], ""), &frac[1..])
            }
            None => t,
        };
        normalized.parse().ok()
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, v: f64) {
        let v = self.clamp(v);
        let changed = (v - self.value).abs() > f64::EPSILON;
        self.value = v;
        self.show();
        if changed {
            ctx.fire("on_change", EventArgs::Value(v.into()));
        }
        ctx.repaint();
    }

    /// Confirma o que foi digitado.
    fn commit(&mut self, ctx: &mut EventCtx<'_>) {
        match Self::parse(&self.edit.text) {
            Some(v) => self.set(ctx, v),
            None => {
                self.show();
                ctx.repaint();
            }
        }
    }

    fn step(&mut self, ctx: &mut EventCtx<'_>, times: f64) {
        let current = Self::parse(&self.edit.text).unwrap_or(self.value);
        self.set(ctx, current + self.increment * times);
    }
}

impl Component for NumberBox {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    // O `text` só vale ao confirmar: o número é o valor.
    fn change_args(&self, name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        (name == "value").then(|| EventArgs::Value(self.value.into()))
    }

    fn class_name(&self) -> &'static str {
        "NumberBox"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.edit)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.edit)
    }

    properties! {
        value: f64 => "Data",
        min: f64 => "Behavior",
        max: f64 => "Behavior",
        increment: f64 => "Behavior",
        decimals: u32 => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn resized(&mut self, ctx: &mut EventCtx<'_>) {
        let sep = ctx.ui.locale().decimal_separator;
        if sep != self.separator {
            self.separator = sep;
            self.show();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let width = self.edit.base.bounds().width;
        if event.button == PointerButton::Primary && event.position.x >= width - SPIN {
            let up = event.position.y < self.edit.base.bounds().height / 2.0;
            self.step(ctx, if up { 1.0 } else { -1.0 });
            ctx.set_handled();
            return;
        }
        self.edit.pointer_down(ctx, event);
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.touch || !self.edit.base.state().focused {
            return; // sem o foco, a roda rola a tela
        }
        self.step(ctx, if event.delta.y > 0.0 { 1.0 } else { -1.0 });
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::Up => self.step(ctx, 1.0),
            Key::Down => self.step(ctx, -1.0),
            Key::PageUp => self.step(ctx, 10.0),
            Key::PageDown => self.step(ctx, -10.0),
            Key::Enter => {
                self.commit(ctx);
                return; // Enter continua (botão padrão do form)
            }
            Key::Escape => {
                self.show();
                ctx.repaint();
            }
            _ => return self.edit.key_down(ctx, event),
        }
        ctx.set_handled();
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        self.commit(ctx);
        self.edit.focus_exit(ctx);
    }

    fn paint_after(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let x = r.right() - SPIN;
        let color = ctx.theme.color("text_muted");
        let line = Stroke::new(ctx.theme.color("border"), 1.0);
        canvas.draw_line(PointF::new(x, r.y + 5.0), PointF::new(x, r.bottom() - 5.0), &line);
        let stroke = Stroke::new(color, 1.5);
        let cx = x + SPIN / 2.0;
        for (cy, up) in [(r.y + r.height * 0.3, true), (r.y + r.height * 0.7, false)] {
            let d = if up { -2.0 } else { 2.0 };
            canvas.draw_line(PointF::new(cx - 4.0, cy - d), PointF::new(cx, cy + d), &stroke);
            canvas.draw_line(PointF::new(cx, cy + d), PointF::new(cx + 4.0, cy - d), &stroke);
        }
    }
}

// ---------------------------------------------------------------------------

/// Controle deslizante: arraste o polegar, clique no trilho, use as
/// setas, Home/End e PageUp/PageDown. `on_change` traz o valor (também durante o arrasto).
pub struct TrackBar {
    pub base: ControlBase,
    /// O valor, a posição do polegar entre `min` e `max`.
    pub value: f64,
    /// O valor da ponta esquerda (ou de baixo, na vertical).
    pub min: f64,
    /// O valor da ponta direita (ou de cima, na vertical).
    pub max: f64,
    /// Passo (0 = contínuo).
    pub frequency: f64,
    /// Vertical em vez de horizontal.
    pub vertical: bool,
    /// Diâmetro do polegar (0 = o do visual do tema).
    pub thumb_size: f32,
    /// Espessura do trilho (0 = a do visual do tema).
    pub track_size: f32,
    /// Polegar cheio na cor de destaque, sem anel (o `input[type=range]` das páginas web).
    pub solid_thumb: bool,
    /// Daqui para cima o trilho preenchido e o polegar ficam na cor de perigo, com uma
    /// marca fina no ponto (o volume acima de 100 %: amplificando). 0 = sem limite.
    pub limit: f64,
    dragging: bool,
}

const THUMB: f32 = 18.0;

impl Default for TrackBar {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 28.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer).set_pan_on_touch(false);
        Self { base, value: 0.0, min: 0.0, max: 100.0, frequency: 1.0, vertical: false, thumb_size: 0.0, track_size: 0.0, solid_thumb: false, limit: 0.0, dragging: false }
    }
}

impl TrackBar {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        let mut t = Self { min, max, ..Default::default() };
        t.value = t.clamp(value);
        t
    }

    fn clamp(&self, v: f64) -> f64 {
        let (lo, hi) = if self.min <= self.max { (self.min, self.max) } else { (self.max, self.min) };
        let v = if self.frequency > 0.0 { self.min + ((v - self.min) / self.frequency).round() * self.frequency } else { v };
        v.clamp(lo, hi)
    }

    fn after_set(&mut self, _name: &str) {
        self.value = self.clamp(self.value);
    }

    fn fraction(&self) -> f32 {
        let span = self.max - self.min;
        if span.abs() < f64::EPSILON { 0.0 } else { ((self.value - self.min) / span).clamp(0.0, 1.0) as f32 }
    }

    /// Trilho (onde o centro do polegar anda), em coordenadas locais.
    fn track(&self) -> (PointF, PointF) {
        let b = self.base.bounds();
        let pad = if self.thumb_size > 0.0 { self.thumb_size } else { THUMB } / 2.0 + 1.0;
        if self.vertical {
            (PointF::new(b.width / 2.0, b.height - pad), PointF::new(b.width / 2.0, pad))
        } else {
            (PointF::new(pad, b.height / 2.0), PointF::new(b.width - pad, b.height / 2.0))
        }
    }

    fn value_at(&self, p: PointF) -> f64 {
        let (a, b) = self.track();
        let t = if self.vertical { (p.y - a.y) / (b.y - a.y) } else { (p.x - a.x) / (b.x - a.x) };
        self.min + (self.max - self.min) * t.clamp(0.0, 1.0) as f64
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, v: f64) {
        let v = self.clamp(v);
        if (v - self.value).abs() > f64::EPSILON {
            self.value = v;
            ctx.fire("on_change", EventArgs::Value(v.into()));
        }
        ctx.repaint();
    }

    fn step_size(&self) -> f64 {
        if self.frequency > 0.0 { self.frequency } else { (self.max - self.min) / 100.0 }
    }
}

impl Component for TrackBar {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("TrackBar");

    properties! {
        value: f64 => "Data",
        min: f64 => "Behavior",
        max: f64 => "Behavior",
        frequency: f64 => "Behavior",
        vertical: bool => "Appearance",
        thumb_size: f32 => "Appearance",
        track_size: f32 => "Appearance",
        solid_thumb: bool => "Appearance",
        limit: f64 => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.dragging = true;
            let v = self.value_at(event.position);
            self.set(ctx, v);
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging {
            let v = self.value_at(event.position);
            self.set(ctx, v);
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging = false;
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let step = self.step_size();
        let v = match event.key {
            Key::Right | Key::Up => self.value + step,
            Key::Left | Key::Down => self.value - step,
            Key::PageUp => self.value + step * 10.0,
            Key::PageDown => self.value - step * 10.0,
            Key::Home => self.min,
            Key::End => self.max,
            _ => return,
        };
        self.set(ctx, v);
        ctx.set_handled();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if event.touch || !self.base.state().focused {
            return;
        }
        let v = self.value + if event.delta.y > 0.0 { self.step_size() } else { -self.step_size() };
        self.set(ctx, v);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let s = ctx.style;
        let (a, b) = self.track();
        let f = self.fraction();
        let thumb = PointF::new(a.x + (b.x - a.x) * f, a.y + (b.y - a.y) * f);
        // O Metro não arredonda o trilho.
        let (thick, side) = crate::looks::slider_metrics(ctx.look());
        let thick = if self.track_size > 0.0 { self.track_size } else { thick };
        let side = if self.thumb_size > 0.0 { self.thumb_size } else { side };
        let rail_radius = if ctx.look() == aylis_core::Look::WinPhone { 0.0 } else { thick / 2.0 };
        // O `input[type=range]` da web: o trilho vai de ponta a ponta (o polegar anda por dentro).
        let bounds = self.base.bounds();
        let rail = match (self.vertical, self.solid_thumb) {
            (true, true) => RectF::new(a.x - thick / 2.0, 0.0, thick, bounds.height),
            (true, false) => RectF::new(a.x - thick / 2.0, b.y, thick, a.y - b.y),
            (false, true) => RectF::new(0.0, a.y - thick / 2.0, bounds.width, thick),
            (false, false) => RectF::new(a.x, a.y - thick / 2.0, b.x - a.x, thick),
        };
        canvas.fill_rect(rail, rail_radius, &Brush::Solid(s.fill));
        let done = if self.vertical {
            RectF::new(rail.x, thumb.y, thick, rail.bottom() - thumb.y)
        } else {
            RectF::new(rail.x, rail.y, thumb.x - rail.x, thick)
        };
        let primary = ctx.theme.color("primary");
        canvas.fill_rect(done, rail_radius, &Brush::Solid(primary));
        let span = self.max - self.min;
        let over = self.limit > 0.0 && span.abs() > f64::EPSILON && self.value > self.limit;
        if self.limit > 0.0 && span.abs() > f64::EPSILON {
            // A parte acima do limite, na cor de perigo, e a marca no limite.
            let lf = ((self.limit - self.min) / span).clamp(0.0, 1.0) as f32;
            let at = PointF::new(a.x + (b.x - a.x) * lf, a.y + (b.y - a.y) * lf);
            let danger = ctx.theme.color("danger");
            if over {
                let part = if self.vertical {
                    RectF::new(rail.x, thumb.y, thick, at.y - thumb.y)
                } else {
                    RectF::new(at.x, rail.y, thumb.x - at.x, thick)
                };
                canvas.fill_rect(part, 0.0, &Brush::Solid(danger));
            }
            let mark = if self.vertical {
                RectF::new(at.x - thick, at.y, thick * 2.0, 1.0)
            } else {
                RectF::new(at.x, at.y - thick, 1.0, thick * 2.0)
            };
            canvas.fill_rect(mark, 0.0, &Brush::Solid(ctx.theme.color("text").with_opacity(0.45)));
        }
        if self.solid_thumb {
            let knob = RectF::new(thumb.x - side / 2.0, thumb.y - side / 2.0, side, side);
            let color = if over { ctx.theme.color("danger") } else { primary };
            canvas.fill_ellipse(knob, &Brush::Solid(color));
        } else {
            crate::looks::paint_thumb(canvas, ctx, thumb, side, self.vertical);
        }
    }
}
