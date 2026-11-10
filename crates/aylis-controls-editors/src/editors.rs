//! Editores visuais de valores, como as ferramentas do DevTools dos navegadores: o relógio
//! de ângulo (`AngleDial`), o editor de sombra (`ShadowEditor`), o da curva de suavização
//! (`EasingEditor`) e a linha do tempo de uma animação (`TimingEditor`).
//!
//! A IDE usa cada um no Object Inspector conforme o tipo da propriedade (`PropKind::Angle`,
//! `Shadow`, `Easing` e `Duration`), mas são componentes comuns: servem em qualquer tela.
//! Todos disparam `on_change` enquanto o valor muda e `on_commit` quando o usuário solta o
//! mouse (ou usa o teclado).

use std::cell::Cell;

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, Easing, EventArgs, EventCtx, FontWeight, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerEvent, PropValue, RectF, Shadow, Stroke, TextAlign, TextStyle, WheelEvent, properties,
};

use crate::leaf_control;

fn token(ctx: &PaintCtx<'_>, name: &str, fallback: &str) -> Color {
    ctx.theme.colors.get(name).copied().unwrap_or_else(|| ctx.theme.color(fallback))
}

/// Texto na fonte do controle (`font_family`, `font_italic`...), no tamanho do desenho.
fn text(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: &str, r: RectF, size: f32, color: Color, align: TextAlign) {
    let style = TextStyle { font: ctx.font(size), color, h_align: align, v_align: TextAlign::Center, ..Default::default() };
    canvas.draw_text(s, r, &style);
}

fn bold(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: &str, r: RectF, size: f32, color: Color, align: TextAlign) {
    let style = TextStyle {
        font: ctx.font(size).weight(FontWeight::SemiBold),
        color,
        h_align: align,
        v_align: TextAlign::Center,
        ..Default::default()
    };
    canvas.draw_text(s, r, &style);
}

fn knob(canvas: &mut dyn Canvas, at: PointF, radius: f32, fill: Color, ring: Color) {
    let r = RectF::new(at.x - radius, at.y - radius, radius * 2.0, radius * 2.0);
    canvas.fill_ellipse(r.inflate(1.5), &Brush::Solid(ring));
    canvas.fill_ellipse(r, &Brush::Solid(fill));
}

fn distance(a: PointF, b: PointF) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn number(v: f32) -> String {
    format!("{}", (v * 100.0).round() / 100.0)
}

// ===========================================================================
// AngleDial
// ===========================================================================

/// Relógio de ângulo (o do DevTools): arraste o ponteiro; com Shift, ele anda de `snap`
/// em `snap` graus. 0° aponta para cima e o ângulo cresce no sentido horário, como no
/// `rotate()` e nos gradientes do CSS. Setas e roda do mouse andam 1° (com Shift, `snap`).
pub struct AngleDial {
    pub base: ControlBase,
    pub angle: f32,
    /// Passo com Shift, em graus.
    pub snap: f32,
    /// Mostra o valor no meio (quando o relógio tem espaço).
    pub show_value: bool,
    dragging: Cell<bool>,
}

impl Default for AngleDial {
    fn default() -> Self {
        let mut base = ControlBase::new(96.0, 96.0);
        base.set_cursor(CursorShape::Pointer).set_can_focus(true);
        Self { base, angle: 0.0, snap: 15.0, show_value: true, dragging: Cell::new(false) }
    }
}

impl AngleDial {
    pub fn new(angle: f32) -> Self {
        Self { angle, ..Default::default() }
    }

    fn face(&self) -> (PointF, f32) {
        let b = self.base.bounds();
        (PointF::new(b.width / 2.0, b.height / 2.0), (b.width.min(b.height) / 2.0 - 2.0).max(4.0))
    }

    /// O ângulo (0..360) do ponto `p` em relação ao centro.
    pub fn angle_at(&self, p: PointF) -> f32 {
        let (c, _) = self.face();
        let a = (p.x - c.x).atan2(-(p.y - c.y)).to_degrees();
        if a < 0.0 { a + 360.0 } else { a }
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, angle: f32, commit: bool) {
        let angle = angle.rem_euclid(360.0);
        let angle = if (angle - 360.0).abs() < 0.001 { 0.0 } else { angle };
        if angle != self.angle {
            self.angle = angle;
            ctx.fire("on_change", EventArgs::Value(PropValue::Float(angle as f64)));
        }
        if commit {
            ctx.fire("on_commit", EventArgs::Value(PropValue::Float(self.angle as f64)));
        }
        ctx.repaint();
    }

    fn drag_to(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let a = self.angle_at(event.position);
        let step = if event.modifiers.shift { self.snap.max(1.0) } else { 1.0 };
        self.set(ctx, (a / step).round() * step, false);
    }
}

impl Component for AngleDial {
    leaf_control!("AngleDial");

    properties! {
        angle: f32 as Angle => "Data",
        snap: f32 => "Behavior",
        show_value: bool => "Appearance";
        events: on_change, on_commit
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.dragging.set(true);
        self.drag_to(ctx, event);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging.get() {
            self.drag_to(ctx, event);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging.replace(false) {
            self.set(ctx, self.angle, true);
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let step = if event.modifiers.shift { self.snap.max(1.0) } else { 1.0 };
        self.set(ctx, self.angle - event.delta.y.signum() * step, true);
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let d = match event.key {
            Key::Right | Key::Up => 1.0,
            Key::Left | Key::Down => -1.0,
            _ => return,
        };
        let step = if event.modifiers.shift { self.snap.max(1.0) } else { 1.0 };
        self.set(ctx, self.angle + d * step, true);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (c, r) = self.face();
        let primary = ctx.theme.color("primary");
        let muted = ctx.theme.color("text_muted");
        let face = RectF::new(c.x - r, c.y - r, r * 2.0, r * 2.0);
        canvas.fill_ellipse(face, &Brush::Solid(token(ctx, "surface_2", "surface")));
        canvas.stroke_ellipse(face, &Stroke::new(ctx.theme.color("border"), 1.0));
        let at = |deg: f32, radius: f32| {
            let a = deg.to_radians();
            PointF::new(c.x + a.sin() * radius, c.y - a.cos() * radius)
        };
        // Marcas de 15 em 15 graus (maiores nos quatro pontos cardeais).
        if r >= 20.0 {
            for i in 0..24 {
                let deg = i as f32 * 15.0;
                let long = i % 6 == 0;
                let inner = r - if long { 7.0 } else { 4.0 };
                canvas.draw_line(at(deg, inner), at(deg, r - 1.5), &Stroke::new(muted.with_opacity(if long { 0.8 } else { 0.45 }), 1.0));
            }
        }
        // O arco percorrido desde 0° e o ponteiro.
        if self.angle > 0.0 && r >= 12.0 {
            let mut arc = Path::new();
            arc.arc(c, r - 3.0, r - 3.0, -90.0, self.angle);
            canvas.stroke_path(&arc, &Stroke::new(primary.with_opacity(0.35), 3.0));
        }
        let tip = at(self.angle, r * 0.78);
        canvas.draw_line(c, tip, &Stroke::new(primary, if r >= 20.0 { 2.0 } else { 1.5 }).round());
        let ring = token(ctx, "surface", "background");
        knob(canvas, tip, if r >= 20.0 { 4.5 } else { 2.5 }, primary, ring);
        knob(canvas, c, if r >= 20.0 { 2.5 } else { 1.5 }, primary, primary);
        if self.show_value && r >= 30.0 {
            // Na metade oposta à do ponteiro, para não cruzar com ele.
            let down = self.angle > 90.0 && self.angle < 270.0;
            let y = if down { c.y - r * 0.62 } else { c.y + r * 0.22 };
            let label = RectF::new(c.x - r, y, r * 2.0, r * 0.4);
            bold(canvas, ctx, &format!("{}°", number(self.angle)), label, (r * 0.24).clamp(10.0, 16.0), ctx.theme.color("text"), TextAlign::Center);
        }
        if self.base.state().focused {
            canvas.stroke_ellipse(face.inflate(2.0), &Stroke::new(primary.with_opacity(0.5), 2.0));
        }
    }
}

// ===========================================================================
// ShadowEditor
// ===========================================================================

#[derive(Clone, Copy, PartialEq)]
enum ShadowPart {
    Pad,
    Blur,
    Spread,
}

/// Editor de sombra (o do DevTools para `box-shadow`): o quadro arrasta o deslocamento
/// (Shift trava num eixo), as faixas ajustam desfoque e espalhamento e a amostra mostra o
/// resultado. A cor vem de `color` (a IDE abre o seletor de cores ao lado).
pub struct ShadowEditor {
    pub base: ControlBase,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: Color,
    /// Deslocamento máximo do quadro (o desfoque vai até o dobro).
    pub range: f32,
    dragging: Cell<Option<(ShadowPart, PointF)>>,
}

impl Default for ShadowEditor {
    fn default() -> Self {
        let mut base = ControlBase::new(340.0, 196.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            offset_x: 0.0,
            offset_y: 4.0,
            blur: 12.0,
            spread: 0.0,
            color: Color::BLACK.with_opacity(0.25),
            range: 24.0,
            dragging: Cell::new(None),
        }
    }
}

struct ShadowAreas {
    pad: RectF,
    preview: RectF,
    blur: RectF,
    spread: RectF,
}

impl ShadowEditor {
    pub fn new(shadow: Shadow) -> Self {
        let mut e = Self::default();
        e.set_shadow(shadow);
        e
    }

    pub fn shadow(&self) -> Shadow {
        Shadow { color: self.color, blur: self.blur, offset: PointF::new(self.offset_x, self.offset_y), spread: self.spread }
    }

    pub fn set_shadow(&mut self, s: Shadow) {
        self.offset_x = s.offset.x;
        self.offset_y = s.offset.y;
        self.blur = s.blur;
        self.spread = s.spread;
        self.color = s.color;
    }

    fn areas(&self) -> ShadowAreas {
        let b = self.base.bounds();
        let side = (b.height - 8.0).min(b.width * 0.45).max(40.0);
        let pad = RectF::new(4.0, 4.0, side, side);
        let x = pad.right() + 18.0;
        let w = (b.width - x - 8.0).max(40.0);
        let rows = 38.0;
        let preview = RectF::new(x, 4.0, w, (b.height - 8.0 - rows * 2.0).max(24.0));
        let blur = RectF::new(x, preview.bottom(), w, rows);
        let spread = RectF::new(x, blur.bottom(), w, rows);
        ShadowAreas { pad, preview, blur, spread }
    }

    /// Trilho de uma faixa (a linha onde corre o botão).
    fn track(row: RectF) -> RectF {
        RectF::new(row.x + 8.0, row.y + 27.0, row.width - 16.0, 4.0)
    }

    fn changed(&self, ctx: &mut EventCtx<'_>, commit: bool) {
        let spec = self.shadow().to_spec();
        ctx.fire(if commit { "on_commit" } else { "on_change" }, EventArgs::Text(spec));
        ctx.repaint();
    }

    fn drag(&mut self, ctx: &mut EventCtx<'_>, part: ShadowPart, start: PointF, event: &PointerEvent) {
        let a = self.areas();
        let p = event.position;
        let range = self.range.max(1.0);
        match part {
            ShadowPart::Pad => {
                let (mut x, mut y) = (p.x, p.y);
                if event.modifiers.shift {
                    if (p.x - start.x).abs() > (p.y - start.y).abs() { y = start.y } else { x = start.x }
                }
                let c = a.pad.center();
                let half = a.pad.width / 2.0 - 8.0;
                self.offset_x = (((x - c.x) / half).clamp(-1.0, 1.0) * range).round();
                self.offset_y = (((y - c.y) / half).clamp(-1.0, 1.0) * range).round();
            }
            ShadowPart::Blur => {
                let t = Self::track(a.blur);
                self.blur = (((p.x - t.x) / t.width).clamp(0.0, 1.0) * range * 2.0).round();
            }
            ShadowPart::Spread => {
                let t = Self::track(a.spread);
                self.spread = ((((p.x - t.x) / t.width).clamp(0.0, 1.0) * 2.0 - 1.0) * range).round();
            }
        }
        self.changed(ctx, false);
    }

    fn slider(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, row: RectF, (title, value): (&str, f32), frac: f32, centered: bool) {
        let muted = ctx.theme.color("text_muted");
        let primary = ctx.theme.color("primary");
        let head = RectF::new(row.x + 8.0, row.y + 4.0, row.width - 16.0, 16.0);
        text(canvas, ctx, title, head, 12.0, muted, TextAlign::Leading);
        bold(canvas, ctx, &format!("{} px", number(value)), head, 12.0, ctx.theme.color("text"), TextAlign::Trailing);
        let t = Self::track(row);
        canvas.fill_rect(t, 2.0, &Brush::Solid(ctx.theme.color("border")));
        let from = if centered { t.x + t.width / 2.0 } else { t.x };
        let to = t.x + t.width * frac.clamp(0.0, 1.0);
        canvas.fill_rect(RectF::new(from.min(to), t.y, (to - from).abs(), t.height), 2.0, &Brush::Solid(primary));
        knob(canvas, PointF::new(to, t.y + 2.0), 7.0, primary, token(ctx, "surface", "background"));
    }
}

impl Component for ShadowEditor {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.shadow().to_spec()))
    }

    leaf_control!("ShadowEditor");

    properties! {
        offset_x: f32 => "Data",
        offset_y: f32 => "Data",
        blur: f32 => "Data",
        spread: f32 => "Data",
        color: Color => "Data",
        range: f32 => "Behavior";
        events: on_change, on_commit
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let a = self.areas();
        let p = event.position;
        let part = if a.pad.contains(p) {
            ShadowPart::Pad
        } else if a.blur.contains(p) {
            ShadowPart::Blur
        } else if a.spread.contains(p) {
            ShadowPart::Spread
        } else {
            return;
        };
        self.dragging.set(Some((part, p)));
        self.drag(ctx, part, p, event);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((part, start)) = self.dragging.get() {
            self.drag(ctx, part, start, event);
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging.take().is_some() {
            self.changed(ctx, true);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let a = self.areas();
        let theme = ctx.theme;
        let border = theme.color("border");
        let muted = theme.color("text_muted");
        let primary = theme.color("primary");
        // O quadro do deslocamento: eixos, ponto de origem e o ponto arrastável.
        canvas.fill_rect(a.pad, 8.0, &Brush::Solid(token(ctx, "surface_2", "surface")));
        canvas.stroke_rect(a.pad, 8.0, &Stroke::new(border, 1.0));
        let c = a.pad.center();
        let half = a.pad.width / 2.0 - 8.0;
        canvas.draw_line(PointF::new(a.pad.x + 6.0, c.y), PointF::new(a.pad.right() - 6.0, c.y), &Stroke::new(border, 1.0));
        canvas.draw_line(PointF::new(c.x, a.pad.y + 6.0), PointF::new(c.x, a.pad.bottom() - 6.0), &Stroke::new(border, 1.0));
        let range = self.range.max(1.0);
        let dot = PointF::new(
            c.x + (self.offset_x / range).clamp(-1.0, 1.0) * half,
            c.y + (self.offset_y / range).clamp(-1.0, 1.0) * half,
        );
        canvas.draw_line(c, dot, &Stroke::new(primary.with_opacity(0.6), 1.5).dashed(vec![3.0, 3.0]));
        knob(canvas, c, 2.5, muted, muted);
        knob(canvas, dot, 7.0, primary, token(ctx, "surface", "background"));
        let info = RectF::new(a.pad.x + 8.0, a.pad.bottom() - 20.0, a.pad.width - 16.0, 16.0);
        text(canvas, ctx, &format!("x {}  y {}", number(self.offset_x), number(self.offset_y)), info, 11.0, muted, TextAlign::Leading);

        // A amostra com a sombra.
        canvas.fill_rect(a.preview, 8.0, &Brush::Solid(token(ctx, "background", "surface")));
        canvas.stroke_rect(a.preview, 8.0, &Stroke::new(border, 1.0));
        let card_w = (a.preview.width * 0.42).clamp(30.0, 90.0);
        let card_h = (a.preview.height * 0.46).clamp(18.0, 54.0);
        let pc = a.preview.center();
        let card = RectF::new(pc.x - card_w / 2.0, pc.y - card_h / 2.0, card_w, card_h);
        canvas.save();
        canvas.clip_rect(a.preview);
        canvas.draw_shadow(card, 8.0, &self.shadow());
        canvas.fill_rect(card, 8.0, &Brush::Solid(token(ctx, "surface", "background")));
        canvas.restore();

        let max_blur = range * 2.0;
        self.slider(canvas, ctx, a.blur, ("Desfoque", self.blur), self.blur / max_blur, false);
        self.slider(canvas, ctx, a.spread, ("Espalhamento", self.spread), (self.spread / range + 1.0) / 2.0, true);
    }
}

// ===========================================================================
// EasingEditor
// ===========================================================================

/// Editor da curva de suavização (o de `cubic-bezier` do DevTools): arraste os dois pontos
/// de controle; o `y` pode passar de 0..1 (curvas que "passam do ponto"). Embaixo, uma
/// bolinha mostra o movimento com a curva. `compact` desenha só a curva (para miniaturas).
pub struct EasingEditor {
    pub base: ControlBase,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    /// A bolinha animada embaixo da curva.
    pub preview: bool,
    /// Só a curva, sem pontos de controle (miniatura).
    pub compact: bool,
    dragging: Cell<Option<u8>>,
}

impl Default for EasingEditor {
    fn default() -> Self {
        let mut base = ControlBase::new(300.0, 360.0);
        base.set_cursor(CursorShape::Pointer);
        let [x1, y1, x2, y2] = Easing::OutQuad.to_bezier();
        Self { base, x1, y1, x2, y2, preview: true, compact: false, dragging: Cell::new(None) }
    }
}

/// Folga acima e abaixo do quadrado 0..1, em frações do lado (para os pontos que passam).
const OVERSHOOT: f32 = 0.3;

impl EasingEditor {
    pub fn new(easing: Easing) -> Self {
        let mut e = Self::default();
        e.set_easing(easing);
        e
    }

    pub fn easing(&self) -> Easing {
        Easing::CubicBezier(self.x1, self.y1, self.x2, self.y2)
    }

    pub fn set_easing(&mut self, easing: Easing) {
        [self.x1, self.y1, self.x2, self.y2] = easing.to_bezier();
    }

    /// O quadrado 0..1 da curva (sem a folga).
    fn unit(&self) -> RectF {
        let b = self.base.bounds();
        if self.compact {
            let m = 3.0;
            let side = ((b.height - 2.0 * m) / (1.0 + OVERSHOOT)).min(b.width - 2.0 * m).max(4.0);
            return RectF::new((b.width - side) / 2.0, (b.height - side) / 2.0, side, side);
        }
        let below = if self.preview { 34.0 } else { 0.0 };
        let side = ((b.height - 16.0 - below) / (1.0 + 2.0 * OVERSHOOT)).min(b.width - 24.0).max(20.0);
        RectF::new((b.width - side) / 2.0, 8.0 + side * OVERSHOOT, side, side)
    }

    fn to_screen(&self, u: RectF, x: f32, y: f32) -> PointF {
        PointF::new(u.x + x * u.width, u.bottom() - y * u.height)
    }

    fn handles(&self, u: RectF) -> [PointF; 2] {
        [self.to_screen(u, self.x1, self.y1), self.to_screen(u, self.x2, self.y2)]
    }

    fn changed(&self, ctx: &mut EventCtx<'_>, commit: bool) {
        ctx.fire(if commit { "on_commit" } else { "on_change" }, EventArgs::Text(self.easing().to_spec()));
        ctx.repaint();
    }
}

impl Component for EasingEditor {
    leaf_control!("EasingEditor");

    properties! {
        x1: f32 => "Data",
        y1: f32 => "Data",
        x2: f32 => "Data",
        y2: f32 => "Data",
        preview: bool => "Appearance",
        compact: bool => "Appearance";
        events: on_change, on_commit
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.compact {
            return;
        }
        let [h1, h2] = self.handles(self.unit());
        let (d1, d2) = (distance(event.position, h1), distance(event.position, h2));
        // O ponto mais perto (o clique em qualquer lugar do quadro puxa o mais próximo).
        let which = if d1 <= d2 { 0 } else { 1 };
        self.dragging.set(Some(which));
        self.pointer_move(ctx, event);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some(which) = self.dragging.get() else { return };
        let u = self.unit();
        let round = |v: f32| (v * 100.0).round() / 100.0;
        let x = round(((event.position.x - u.x) / u.width).clamp(0.0, 1.0));
        let y = round(((u.bottom() - event.position.y) / u.height).clamp(-OVERSHOOT, 1.0 + OVERSHOOT));
        if which == 0 {
            (self.x1, self.y1) = (x, y);
        } else {
            (self.x2, self.y2) = (x, y);
        }
        self.changed(ctx, false);
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging.take().is_some() {
            self.changed(ctx, true);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let u = self.unit();
        let theme = ctx.theme;
        let border = theme.color("border");
        let muted = theme.color("text_muted");
        let primary = theme.color("primary");
        if self.compact {
            canvas.fill_rect(ctx.rect, 4.0, &Brush::Solid(token(ctx, "surface_2", "surface")));
        } else {
            canvas.fill_rect(u, 4.0, &Brush::Solid(token(ctx, "surface_2", "surface")));
            for i in 1..4 {
                let f = i as f32 / 4.0;
                let s = Stroke::new(border.with_opacity(0.6), 1.0);
                canvas.draw_line(PointF::new(u.x + u.width * f, u.y), PointF::new(u.x + u.width * f, u.bottom()), &s);
                canvas.draw_line(PointF::new(u.x, u.y + u.height * f), PointF::new(u.right(), u.y + u.height * f), &s);
            }
            canvas.stroke_rect(u, 4.0, &Stroke::new(border, 1.0));
            // A reta (linear), para comparar.
            canvas.draw_line(self.to_screen(u, 0.0, 0.0), self.to_screen(u, 1.0, 1.0), &Stroke::new(muted.with_opacity(0.5), 1.0).dashed(vec![4.0, 4.0]));
        }
        let [h1, h2] = self.handles(u);
        let (p0, p3) = (self.to_screen(u, 0.0, 0.0), self.to_screen(u, 1.0, 1.0));
        let mut curve = Path::new();
        curve.move_to(p0.x, p0.y).cubic_to(h1.x, h1.y, h2.x, h2.y, p3.x, p3.y);
        if self.compact {
            canvas.save();
            canvas.clip_rect(ctx.rect);
            canvas.stroke_path(&curve, &Stroke::new(primary, 1.5).round());
            canvas.restore();
            return;
        }
        let arm = Stroke::new(muted, 1.5);
        canvas.draw_line(p0, h1, &arm);
        canvas.draw_line(p3, h2, &arm);
        canvas.stroke_path(&curve, &Stroke::new(primary, 2.5).round());
        knob(canvas, p0, 3.0, muted, muted);
        knob(canvas, p3, 3.0, muted, muted);
        let ring = token(ctx, "surface", "background");
        knob(canvas, h1, 6.5, primary, ring);
        knob(canvas, h2, 6.5, theme.colors.get("secondary").copied().unwrap_or(primary), ring);
        let label = RectF::new(4.0, 2.0, ctx.rect.width - 8.0, 14.0);
        text(canvas, ctx, &self.easing().to_spec(), label, 11.0, muted, TextAlign::Center);

        if self.preview {
            // A bolinha vai de um lado ao outro com a curva (1 s), para e recomeça.
            let track_y = ctx.rect.height - 18.0;
            let (x0, x1) = (u.x, u.right());
            canvas.draw_line(PointF::new(x0, track_y), PointF::new(x1, track_y), &Stroke::new(border, 2.0).round());
            let cycle = 1.6;
            let t = ((ctx.time % cycle) / 1.0).min(1.0) as f32;
            let v = self.easing().apply(t);
            let at = PointF::new(x0 + (x1 - x0) * v, track_y);
            knob(canvas, at, 6.0, primary, ring);
            ctx.request_frame();
        }
    }
}

// ===========================================================================
// TimingEditor
// ===========================================================================

#[derive(Clone, Copy, PartialEq)]
enum TimingPart {
    /// O começo da animação (muda o atraso).
    Start,
    /// O fim (muda a duração).
    End,
    /// A barra inteira (move o atraso).
    Bar,
}

/// Linha do tempo de uma animação (o painel Animations do DevTools): o atraso (tracejado),
/// a duração (a barra), a volta (`auto_reverse`) e as repetições (`looping`), com a curva
/// do valor ao longo do tempo. Arraste o começo para mudar o atraso, o fim para mudar a
/// duração ou a barra para mover tudo. Os tempos andam de `snap` em `snap` ms (Shift: 1 ms).
/// Com `playing`, uma agulha percorre a linha do tempo.
pub struct TimingEditor {
    pub base: ControlBase,
    pub delay: f32,
    pub duration: f32,
    pub easing: Easing,
    pub auto_reverse: bool,
    pub looping: bool,
    pub snap: f32,
    pub playing: bool,
    /// Escala da régua travada durante o arrasto (para a barra não fugir do mouse).
    frozen: Cell<Option<f32>>,
    dragging: Cell<Option<(TimingPart, f32, f32, f32)>>,
}

impl Default for TimingEditor {
    fn default() -> Self {
        let mut base = ControlBase::new(480.0, 170.0);
        base.set_cursor(CursorShape::ResizeHorizontal);
        Self {
            base,
            delay: 0.0,
            duration: 300.0,
            easing: Easing::OutQuad,
            auto_reverse: false,
            looping: false,
            snap: 10.0,
            playing: true,
            frozen: Cell::new(None),
            dragging: Cell::new(None),
        }
    }
}

/// Passo "redondo" da régua para caber umas seis marcas.
fn nice_step(total: f32) -> f32 {
    let raw = total / 6.0;
    [10.0, 20.0, 25.0, 50.0, 100.0, 200.0, 250.0, 500.0, 1000.0, 2000.0, 5000.0, 10000.0]
        .into_iter()
        .find(|s| *s >= raw)
        .unwrap_or(raw.ceil())
}

fn ms(v: f32) -> String {
    if v >= 1000.0 { format!("{} s", (v / 10.0).round() / 100.0) } else { format!("{} ms", v.round()) }
}

impl TimingEditor {
    /// Ida (ou ida e volta) em ms.
    pub fn span(&self) -> f32 {
        self.duration.max(0.0) * if self.auto_reverse { 2.0 } else { 1.0 }
    }

    /// Fim da régua em ms.
    pub fn scale(&self) -> f32 {
        if let Some(s) = self.frozen.get() {
            return s;
        }
        let total = (self.delay.max(0.0) + self.span()) * if self.looping { 2.0 } else { 1.25 };
        let step = nice_step(total.max(400.0));
        (total.max(400.0) / step).ceil() * step
    }

    fn lane(&self) -> RectF {
        let b = self.base.bounds();
        RectF::new(12.0, 26.0, (b.width - 24.0).max(20.0), 30.0)
    }

    fn plot(&self) -> RectF {
        let b = self.base.bounds();
        let lane = self.lane();
        RectF::new(lane.x, lane.bottom() + 10.0, lane.width, (b.height - lane.bottom() - 36.0).max(16.0))
    }

    fn x_of(&self, t: f32) -> f32 {
        let lane = self.lane();
        lane.x + lane.width * (t / self.scale()).clamp(0.0, 1.0)
    }

    fn t_of(&self, x: f32) -> f32 {
        let lane = self.lane();
        ((x - lane.x) / lane.width).clamp(0.0, 1.0) * self.scale()
    }

    /// O valor (0..1) da animação no instante `t` (ms), ou `None` antes de começar.
    pub fn value_at(&self, t: f32) -> Option<f32> {
        let d = self.duration.max(1.0);
        let mut e = t - self.delay;
        if e < 0.0 {
            return None;
        }
        let span = self.span().max(1.0);
        if self.looping {
            e %= span;
        } else if e > span {
            return Some(if self.auto_reverse { 0.0 } else { 1.0 });
        }
        let p = e / d;
        let p = if self.auto_reverse && p > 1.0 { 2.0 - p } else { p.min(1.0) };
        Some(self.easing.apply(p))
    }

    fn summary(&self) -> String {
        let mut s = format!("duração {}", ms(self.duration));
        if self.delay > 0.0 {
            s = format!("atraso {} · {s}", ms(self.delay));
        }
        if self.auto_reverse {
            s += " · ida e volta";
        }
        if self.looping {
            s += " · repete";
        }
        s
    }

    fn changed(&self, ctx: &mut EventCtx<'_>, commit: bool) {
        ctx.fire(if commit { "on_commit" } else { "on_change" }, EventArgs::Text(self.summary()));
        ctx.repaint();
    }
}

impl Component for TimingEditor {
    leaf_control!("TimingEditor");

    properties! {
        delay: f32 as Duration => "Animation",
        duration: f32 as Duration => "Animation",
        easing: Easing => "Animation",
        auto_reverse: bool => "Animation",
        looping: bool => "Animation",
        snap: f32 => "Behavior",
        playing: bool => "Behavior";
        events: on_change, on_commit
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        let (start, end) = (self.x_of(self.delay), self.x_of(self.delay + self.duration));
        let part = if (p.x - end).abs() <= 7.0 {
            TimingPart::End
        } else if (p.x - start).abs() <= 7.0 {
            TimingPart::Start
        } else if p.x > start && p.x < end {
            TimingPart::Bar
        } else if p.x > end {
            TimingPart::End
        } else {
            TimingPart::Start
        };
        self.frozen.set(Some(self.scale()));
        self.dragging.set(Some((part, self.t_of(p.x), self.delay, self.duration)));
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let Some((part, t0, delay0, duration0)) = self.dragging.get() else { return };
        let step = if event.modifiers.shift { 1.0 } else { self.snap.max(1.0) };
        let snap = |v: f32| ((v / step).round() * step).max(0.0);
        let t = self.t_of(event.position.x);
        match part {
            TimingPart::Start => {
                // O fim fica onde está: o atraso cresce e a duração encolhe.
                let end = delay0 + duration0;
                self.delay = snap(t).min(end - step);
                self.duration = end - self.delay;
            }
            TimingPart::End => self.duration = snap(t - self.delay).max(step),
            TimingPart::Bar => self.delay = snap(delay0 + t - t0),
        }
        self.changed(ctx, false);
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.dragging.take().is_some() {
            self.frozen.set(None);
            self.changed(ctx, true);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let theme = ctx.theme;
        let border = theme.color("border");
        let muted = theme.color("text_muted");
        let primary = theme.color("primary");
        let lane = self.lane();
        let plot = self.plot();
        let scale = self.scale();

        // Régua.
        let step = nice_step(scale);
        let mut t = 0.0;
        while t <= scale + 0.5 {
            let x = self.x_of(t);
            canvas.draw_line(PointF::new(x, 16.0), PointF::new(x, plot.bottom()), &Stroke::new(border.with_opacity(0.5), 1.0));
            // O primeiro rótulo encosta à esquerda e o último à direita (não saem da área).
            let (label, align) = if t == 0.0 {
                (RectF::new(x, 0.0, 80.0, 14.0), TextAlign::Leading)
            } else if t + step > scale + 0.5 {
                (RectF::new(x - 80.0, 0.0, 80.0, 14.0), TextAlign::Trailing)
            } else {
                (RectF::new(x - 40.0, 0.0, 80.0, 14.0), TextAlign::Center)
            };
            text(canvas, ctx, &if t == 0.0 { "0".to_string() } else { ms(t) }, label, 10.5, muted, align);
            t += step;
        }

        // A faixa: atraso tracejado, a ida, a volta e as repetições (mais claras).
        canvas.fill_rect(lane, 6.0, &Brush::Solid(token(ctx, "surface_2", "surface")));
        let (x_start, x_end) = (self.x_of(self.delay), self.x_of(self.delay + self.duration));
        if self.delay > 0.0 {
            let r = RectF::new(lane.x, lane.y + 6.0, x_start - lane.x, lane.height - 12.0);
            canvas.stroke_rect(r, 4.0, &Stroke::new(muted.with_opacity(0.7), 1.0).dashed(vec![3.0, 3.0]));
            if r.width > 70.0 {
                text(canvas, ctx, &format!("atraso {}", ms(self.delay)), r, 11.0, muted, TextAlign::Center);
            }
        }
        let segment = |canvas: &mut dyn Canvas, from: f32, to: f32, opacity: f32| {
            let (a, b) = (self.x_of(from), self.x_of(to));
            if b - a >= 1.0 {
                canvas.fill_rect(RectF::new(a, lane.y + 3.0, b - a, lane.height - 6.0), 5.0, &Brush::Solid(primary.with_opacity(opacity)));
            }
        };
        let span = self.span().max(1.0);
        segment(canvas, self.delay, self.delay + self.duration, 1.0);
        if self.auto_reverse {
            segment(canvas, self.delay + self.duration + 1.5, self.delay + span, 0.55);
        }
        if self.looping {
            let mut from = self.delay + span;
            while from < scale {
                segment(canvas, from + 1.5, (from + span).min(scale), 0.25);
                from += span;
            }
        }
        let bar = RectF::new(x_start, lane.y, (x_end - x_start).max(1.0), lane.height);
        if bar.width > 60.0 {
            bold(canvas, ctx, &ms(self.duration), bar, 11.5, theme.colors.get("on_primary").copied().unwrap_or(Color::WHITE), TextAlign::Center);
        }
        // As alças do começo e do fim.
        for x in [x_start, x_end] {
            let r = RectF::new(x - 3.0, lane.y - 2.0, 6.0, lane.height + 4.0);
            canvas.fill_rect(r, 3.0, &Brush::Solid(token(ctx, "surface", "background")));
            canvas.stroke_rect(r, 3.0, &Stroke::new(primary, 1.5));
        }

        // O valor ao longo do tempo (a curva aplicada).
        canvas.fill_rect(plot, 4.0, &Brush::Solid(token(ctx, "surface_2", "surface").with_opacity(0.5)));
        let mut path = Path::new();
        let n = (plot.width / 2.0).max(8.0) as usize;
        let y_of = |v: f32| plot.bottom() - 4.0 - v * (plot.height - 8.0);
        for i in 0..=n {
            let t = scale * i as f32 / n as f32;
            let v = self.value_at(t).unwrap_or(0.0);
            let (x, y) = (self.x_of(t), y_of(v));
            if i == 0 { path.move_to(x, y) } else { path.line_to(x, y) };
        }
        canvas.save();
        canvas.clip_rect(plot.inflate(8.0));
        canvas.stroke_path(&path, &Stroke::new(primary, 2.0).round());
        canvas.restore();

        // A agulha: percorre a régua toda e recomeça.
        if self.playing && self.dragging.get().is_none() {
            let cycle = (scale / 1000.0) as f64 + 0.5;
            let now = ((ctx.time % cycle) * 1000.0) as f32;
            if now <= scale {
                let x = self.x_of(now);
                canvas.draw_line(PointF::new(x, lane.y - 6.0), PointF::new(x, plot.bottom()), &Stroke::new(theme.color("danger"), 1.5));
                if let Some(v) = self.value_at(now) {
                    knob(canvas, PointF::new(x, y_of(v)), 4.5, theme.color("danger"), token(ctx, "surface", "background"));
                }
            }
            ctx.request_frame();
        }
        let foot = RectF::new(lane.x, plot.bottom() + 6.0, lane.width, 16.0);
        text(canvas, ctx, &self.summary(), foot, 11.5, muted, TextAlign::Leading);
        text(canvas, ctx, &self.easing.to_spec(), foot, 11.5, muted, TextAlign::Trailing);
    }
}
