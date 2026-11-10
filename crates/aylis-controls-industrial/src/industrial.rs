//! Componentes industriais (categoria "Industrial") e elétricos ("Electrical"): para telas de
//! supervisão (o SCADA/IHM de quem usa). Desenhados em vetor, com `value`, `state`,
//! `running` e `alarm`; quem liga ao processo é o app (Modbus, OPC UA... ficam com ele).
//!
//! - Instrumentos: [`Manometer`], [`Thermometer`], [`TankLevel`] (líquido animado),
//!   [`FlowMeter`], [`SevenSegment`] e [`SignalLight`] (o sinaleiro).
//! - Equipamentos: [`Valve`], [`Motor`], [`Pump`], [`Conveyor`], [`Fan`] e [`Silo`].
//! - Comandos: [`EmergencyStop`], [`SelectorSwitch`] e [`PushButtons`] (a botoeira).
//! - Ligações: [`Pipe`] (com o fluxo animado) e [`Wire`], presos aos pontos de ligação de
//!   dois componentes (seguem quando eles mudam de lugar; os cotovelos saem sozinhos).
//! - Símbolos elétricos: [`ElectricalSymbol`] (resistor, capacitor, indutor, diodo, LED,
//!   transistor, fonte, terra, chave, relé, lâmpada e motor), cada um um subtipo na paleta.

use std::f32::consts::PI;

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, Connection, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight,
    GradientStop, PaintCtx, Path, PointF, PropEnum, RectF, Stroke, TextAlign, Ui, properties,
};

use crate::leaf_control;

// ---------------------------------------------------------------------------
// Ajudantes de desenho

fn ink(ctx: &PaintCtx<'_>) -> Color {
    ctx.theme.color("text")
}

fn metal(ctx: &PaintCtx<'_>) -> Color {
    let dark = ctx.theme.dark;
    if dark { Color::rgb(120, 128, 140) } else { Color::rgb(176, 184, 196) }
}

/// A cor do estado: alarme (vermelho), ligado (verde) ou parado (cinza).
fn state_color(ctx: &PaintCtx<'_>, running: bool, alarm: bool) -> Color {
    if alarm {
        ctx.theme.color("danger")
    } else if running {
        ctx.theme.color("success")
    } else {
        ctx.theme.color("text_muted")
    }
}

/// O contorno piscando do alarme (pede o próximo quadro).
fn alarm_ring(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, alarm: bool) {
    if !alarm {
        return;
    }
    let on = (ctx.time * 2.0).fract() < 0.5;
    if on {
        canvas.stroke_rect(ctx.rect.inflate(-1.5), 6.0, &Stroke::new(ctx.theme.color("danger"), 3.0));
    }
    blink_wake(ctx);
}

/// O próximo quadro de quem pisca duas vezes por segundo: só na troca (aceso/apagado), não a
/// cada atualização do monitor.
fn blink_wake(ctx: &PaintCtx<'_>) {
    ctx.request_frame_at(((ctx.time * 4.0).floor() + 1.0) / 4.0);
}

fn fraction(value: f64, min: f64, max: f64) -> f32 {
    ((value - min) / (max - min).max(1e-9)).clamp(0.0, 1.0) as f32
}

fn caption(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, text: &str, rect: RectF, size: f32, bold: bool) {
    if text.is_empty() {
        return;
    }
    let mut t = ctx.text_style();
    t.h_align = TextAlign::Center;
    t.font.size = size.max(8.0);
    if bold {
        t.font.weight = FontWeight::Bold;
    }
    canvas.draw_text(text, rect, &t);
}

fn number(value: f64, decimals: u32) -> String {
    format!("{value:.*}", decimals as usize)
}

/// O giro das peças que rodam (rotações por segundo), parado quando não `running`.
fn spin(ctx: &PaintCtx<'_>, running: bool, speed: f64) -> f32 {
    if !running {
        return 0.0;
    }
    ctx.request_frame();
    ((ctx.time * speed).fract() * 360.0) as f32
}

macro_rules! paint_value_props {
    () => {
        fn paint_only_property(&self, name: &str) -> bool {
            matches!(name, "value" | "running" | "alarm" | "state" | "pressed" | "item_index" | "text")
        }

        fn transparent_background(&self) -> bool {
            true
        }
    };
}

// ===========================================================================
// Instrumentos
// ===========================================================================

/// Manômetro: o mostrador redondo com o ponteiro, as marcas e a faixa de alarme.
pub struct Manometer {
    pub base: ControlBase,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    /// A partir daqui o arco fica vermelho (0 = sem faixa).
    pub alarm_above: f64,
    pub alarm: bool,
    pub decimals: u32,
}

impl Default for Manometer {
    fn default() -> Self {
        Self { base: ControlBase::new(140.0, 140.0), value: 4.2, min: 0.0, max: 10.0, unit: "bar".into(), alarm_above: 8.0, alarm: false, decimals: 1 }
    }
}

impl Component for Manometer {
    leaf_control!("Manometer");
    paint_value_props!();

    properties! {
        value: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        unit: String => "Appearance",
        alarm_above: f64 => "Data",
        alarm: bool => "Data",
        decimals: u32 => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = r.width.min(r.height);
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let rad = side / 2.0 - 3.0;
        let face = RectF::new(c.x - rad, c.y - rad, rad * 2.0, rad * 2.0);
        canvas.fill_ellipse(face, &Brush::Radial { center: c, radius: rad, stops: vec![GradientStop::new(0.0, ctx.theme.color("surface")), GradientStop::new(1.0, ctx.theme.color("surface").lerp(Color::BLACK, 0.08))] });
        canvas.stroke_ellipse(face, &Stroke::new(metal(ctx), (side * 0.05).max(2.0)));
        // 270° de escala, do canto de baixo à esquerda ao de baixo à direita.
        let (start, sweep) = (135.0f32, 270.0f32);
        if self.alarm_above > self.min && self.alarm_above < self.max {
            let f = fraction(self.alarm_above, self.min, self.max);
            let mut band = Path::new();
            band.arc(c, rad * 0.82, rad * 0.82, start + sweep * f, sweep * (1.0 - f));
            canvas.stroke_path(&band, &Stroke::new(ctx.theme.color("danger").with_opacity(0.7), rad * 0.08));
        }
        for i in 0..=10 {
            let a = (start + sweep * i as f32 / 10.0).to_radians();
            let (inner, outer) = (if i % 5 == 0 { 0.7 } else { 0.76 }, 0.88);
            let p = |k: f32| PointF::new(c.x + a.cos() * rad * k, c.y + a.sin() * rad * k);
            canvas.draw_line(p(inner), p(outer), &Stroke::new(ink(ctx), if i % 5 == 0 { 2.0 } else { 1.0 }));
        }
        let f = fraction(self.value, self.min, self.max);
        let a = (start + sweep * f).to_radians();
        let tip = PointF::new(c.x + a.cos() * rad * 0.78, c.y + a.sin() * rad * 0.78);
        canvas.draw_line(c, tip, &Stroke::new(ctx.theme.color("danger"), (side * 0.025).max(1.5)).round());
        canvas.fill_ellipse(RectF::new(c.x - rad * 0.08, c.y - rad * 0.08, rad * 0.16, rad * 0.16), &Brush::Solid(ink(ctx)));
        caption(canvas, ctx, &format!("{} {}", number(self.value, self.decimals), self.unit), RectF::new(r.x, c.y + rad * 0.3, r.width, rad * 0.35), side * 0.11, true);
        alarm_ring(canvas, ctx, self.alarm);
    }
}

/// Termômetro: o tubo com o bulbo, a coluna subindo com o valor e as marcas.
pub struct Thermometer {
    pub base: ControlBase,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    pub alarm: bool,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self { base: ControlBase::new(60.0, 180.0), value: 36.5, min: 0.0, max: 100.0, unit: "°C".into(), alarm: false }
    }
}

impl Component for Thermometer {
    leaf_control!("Thermometer");
    paint_value_props!();

    properties! {
        value: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        unit: String => "Appearance",
        alarm: bool => "Data",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let text_h = 18.0;
        let w = (r.width * 0.3).clamp(8.0, 24.0);
        let bulb = w * 1.8;
        let cx = r.x + r.width / 2.0;
        let top = r.y + 4.0;
        let bottom = r.bottom() - text_h - bulb;
        let tube = RectF::new(cx - w / 2.0, top, w, (bottom - top).max(1.0) + bulb / 2.0);
        let glass = ctx.theme.color("surface");
        canvas.fill_rect(tube, w / 2.0, &Brush::Solid(glass));
        canvas.stroke_rect(tube, w / 2.0, &Stroke::new(metal(ctx), 1.5));
        let red = if self.alarm { ctx.theme.color("danger") } else { Color::rgb(220, 60, 50) };
        let bulb_rect = RectF::new(cx - bulb / 2.0, bottom, bulb, bulb);
        let f = fraction(self.value, self.min, self.max);
        let col_top = bottom - (bottom - top - w / 2.0) * f;
        canvas.fill_rect(RectF::new(cx - w * 0.28, col_top, w * 0.56, bottom - col_top + bulb / 2.0), w * 0.28, &Brush::Solid(red));
        canvas.fill_ellipse(bulb_rect, &Brush::Solid(red));
        canvas.stroke_ellipse(bulb_rect, &Stroke::new(metal(ctx), 1.5));
        for i in 0..=5 {
            let y = bottom - (bottom - top - w / 2.0) * i as f32 / 5.0;
            canvas.draw_line(PointF::new(cx + w / 2.0 + 2.0, y), PointF::new(cx + w / 2.0 + 8.0, y), &Stroke::new(ink(ctx), 1.0));
        }
        caption(canvas, ctx, &format!("{:.1}{}", self.value, self.unit), RectF::new(r.x, r.bottom() - text_h, r.width, text_h), 12.0, true);
        alarm_ring(canvas, ctx, self.alarm);
    }
}

/// Nível de tanque: o tanque com o líquido ondulando na altura do valor (em %, ou entre
/// `min` e `max`), as marcas e o número.
pub struct TankLevel {
    pub base: ControlBase,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    pub liquid: Color,
    /// O líquido ondula (pede quadros enquanto ligado).
    pub running: bool,
    pub alarm: bool,
}

impl Default for TankLevel {
    fn default() -> Self {
        Self { base: ControlBase::new(120.0, 160.0), value: 65.0, min: 0.0, max: 100.0, unit: "%".into(), liquid: Color::rgb(40, 140, 230), running: true, alarm: false }
    }
}

impl Component for TankLevel {
    leaf_control!("TankLevel");
    paint_value_props!();

    properties! {
        value: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        unit: String => "Appearance",
        liquid: Color => "Appearance",
        running: bool => "Data",
        alarm: bool => "Data",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect.inflate(-3.0);
        let radius = (r.width * 0.12).min(14.0);
        canvas.fill_rect(r, radius, &Brush::Solid(ctx.theme.color("surface")));
        let f = fraction(self.value, self.min, self.max);
        let level = r.bottom() - r.height * f;
        canvas.save();
        canvas.clip_round_rect(r, radius);
        let mut wave = Path::new();
        let phase = if self.running { ctx.request_frame(); (ctx.time * 2.0) as f32 } else { 0.0 };
        let amp = if f > 0.0 && f < 1.0 { (r.height * 0.02).clamp(1.5, 4.0) } else { 0.0 };
        wave.move_to(r.x, r.bottom());
        let steps = 24;
        for i in 0..=steps {
            let x = r.x + r.width * i as f32 / steps as f32;
            let y = level + (x / r.width.max(1.0) * 2.0 * PI * 1.5 + phase).sin() * amp;
            wave.line_to(x, y);
        }
        wave.line_to(r.right(), r.bottom());
        wave.close();
        let top = self.liquid.lerp(Color::WHITE, 0.15);
        canvas.fill_path(&wave, &Brush::Linear { start: PointF::new(r.x, level), end: PointF::new(r.x, r.bottom()), stops: vec![GradientStop::new(0.0, top), GradientStop::new(1.0, self.liquid)] });
        canvas.restore();
        canvas.stroke_rect(r, radius, &Stroke::new(metal(ctx), 2.0));
        for i in 1..4 {
            let y = r.bottom() - r.height * i as f32 / 4.0;
            canvas.draw_line(PointF::new(r.right() - 10.0, y), PointF::new(r.right(), y), &Stroke::new(ink(ctx).with_opacity(0.6), 1.0));
        }
        caption(canvas, ctx, &format!("{:.0}{}", self.value, self.unit), RectF::new(r.x, r.y + r.height * 0.4, r.width, 22.0), (r.width * 0.16).min(18.0), true);
        alarm_ring(canvas, ctx, self.alarm);
    }
}

/// Medidor de vazão: a turbina girando na velocidade da vazão e o valor.
pub struct FlowMeter {
    pub base: ControlBase,
    pub value: f64,
    pub max: f64,
    pub unit: String,
    pub running: bool,
    pub alarm: bool,
}

impl Default for FlowMeter {
    fn default() -> Self {
        Self { base: ControlBase::new(120.0, 90.0), value: 12.5, max: 50.0, unit: "m³/h".into(), running: true, alarm: false }
    }
}

impl Component for FlowMeter {
    leaf_control!("FlowMeter");
    paint_value_props!();

    properties! {
        value: f64 => "Data",
        max: f64 => "Data",
        unit: String => "Appearance",
        running: bool => "Data",
        alarm: bool => "Data",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let body = RectF::new(r.x + 4.0, r.y + 4.0, r.width - 8.0, r.height - 26.0);
        canvas.fill_rect(body, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(body, 8.0, &Stroke::new(metal(ctx), 2.0));
        let c = PointF::new(body.x + body.width / 2.0, body.y + body.height / 2.0);
        let rad = body.height.min(body.width) * 0.35;
        let speed = (self.value / self.max.max(1e-9)).clamp(0.0, 1.0) * 3.0;
        let angle = spin(ctx, self.running && self.value > 0.0, speed);
        canvas.save();
        canvas.translate(c.x, c.y);
        canvas.rotate(angle);
        for k in 0..4 {
            canvas.save();
            canvas.rotate(k as f32 * 90.0);
            let mut blade = Path::new();
            blade.move_to(0.0, 0.0).quad_to(rad * 0.6, -rad * 0.4, rad, -rad * 0.1).line_to(rad, rad * 0.1).quad_to(rad * 0.4, 0.0, 0.0, 0.0);
            canvas.fill_path(&blade, &Brush::Solid(state_color(ctx, self.running, self.alarm)));
            canvas.restore();
        }
        canvas.restore();
        caption(canvas, ctx, &format!("{:.1} {}", self.value, self.unit), RectF::new(r.x, r.bottom() - 20.0, r.width, 20.0), 12.0, true);
        alarm_ring(canvas, ctx, self.alarm);
    }
}

/// Mostrador de 7 segmentos (números, `-`, `.` e algumas letras).
pub struct SevenSegment {
    pub base: ControlBase,
    pub text: String,
    /// Quantos dígitos (alinha à direita, como num painel).
    pub digits: u32,
    pub color: Color,
}

impl Default for SevenSegment {
    fn default() -> Self {
        Self { base: ControlBase::new(150.0, 60.0), text: "1234".into(), digits: 4, color: Color::rgb(255, 60, 40) }
    }
}

/// Os segmentos acesos de cada caractere (a, b, c, d, e, f, g).
fn segments(c: char) -> u8 {
    match c.to_ascii_uppercase() {
        '0' | 'O' | 'D' => 0b0111111,
        '1' | 'I' => 0b0000110,
        '2' | 'Z' => 0b1011011,
        '3' => 0b1001111,
        '4' => 0b1100110,
        '5' | 'S' => 0b1101101,
        '6' | 'G' => 0b1111101,
        '7' => 0b0000111,
        '8' | 'B' => 0b1111111,
        '9' => 0b1101111,
        'A' => 0b1110111,
        'C' => 0b0111001,
        'E' => 0b1111001,
        'F' => 0b1110001,
        'H' => 0b1110110,
        'L' => 0b0111000,
        'P' => 0b1110011,
        'U' => 0b0111110,
        '-' => 0b1000000,
        '_' => 0b0001000,
        _ => 0,
    }
}

impl Component for SevenSegment {
    leaf_control!("SevenSegment");
    paint_value_props!();

    properties! {
        text: String => "Data",
        digits: u32 => "Appearance",
        color: Color => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 4.0, &Brush::Solid(Color::rgb(18, 18, 20)));
        // Os caracteres (o ponto vai no anterior).
        let mut cells: Vec<(char, bool)> = Vec::new();
        for ch in self.text.chars() {
            if ch == '.' || ch == ',' {
                if let Some(last) = cells.last_mut() {
                    last.1 = true;
                    continue;
                }
                cells.push((' ', true));
            } else {
                cells.push((ch, false));
            }
        }
        let n = (self.digits as usize).max(cells.len()).max(1);
        while cells.len() < n {
            cells.insert(0, (' ', false));
        }
        let pad = r.height * 0.12;
        let cw = (r.width - pad * 2.0) / n as f32;
        let h = r.height - pad * 2.0;
        let w = (cw * 0.7).min(h * 0.55);
        let t = (w * 0.16).max(2.0);
        let off = self.color.with_opacity(0.08);
        for (i, (ch, dot)) in cells.iter().enumerate() {
            let x = r.x + pad + cw * i as f32 + (cw - w) / 2.0;
            let y = r.y + pad;
            let bits = segments(*ch);
            let seg = |k: u8| if bits & (1 << k) != 0 { self.color } else { off };
            let mid = y + h / 2.0;
            let horiz = [(0u8, y), (6, mid - t / 2.0), (3, y + h - t)];
            for (k, yy) in horiz {
                canvas.fill_rect(RectF::new(x + t, yy, w - 2.0 * t, t), t / 2.0, &Brush::Solid(seg(k)));
            }
            let vert = [(5u8, x, y + t / 2.0), (1, x + w - t, y + t / 2.0), (4, x, mid), (2, x + w - t, mid)];
            for (k, xx, yy) in vert {
                canvas.fill_rect(RectF::new(xx, yy, t, h / 2.0 - t), t / 2.0, &Brush::Solid(seg(k)));
            }
            let dot_color = if *dot { self.color } else { off };
            canvas.fill_ellipse(RectF::new(x + w + t * 0.3, y + h - t, t, t), &Brush::Solid(dot_color));
        }
    }
}

/// O que o sinaleiro mostra.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SignalState {
    #[default]
    Off,
    Green,
    Yellow,
    Red,
    Blue,
    White,
}

impl PropEnum for SignalState {
    const NAMES: &'static [&'static str] = &["off", "green", "yellow", "red", "blue", "white"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Off, Self::Green, Self::Yellow, Self::Red, Self::Blue, Self::White].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl SignalState {
    fn color(self) -> Color {
        match self {
            Self::Off => Color::rgb(90, 90, 90),
            Self::Green => Color::rgb(40, 200, 80),
            Self::Yellow => Color::rgb(250, 200, 30),
            Self::Red => Color::rgb(235, 50, 40),
            Self::Blue => Color::rgb(40, 120, 240),
            Self::White => Color::rgb(245, 245, 245),
        }
    }
}

/// Sinaleiro (torre de luz): vermelho, amarelo e verde empilhados; `state` acende um (ou
/// `lights`: vários, `red;green`), `blink` pisca.
pub struct SignalLight {
    pub base: ControlBase,
    pub state: SignalState,
    pub lights: String,
    pub blink: bool,
}

impl Default for SignalLight {
    fn default() -> Self {
        Self { base: ControlBase::new(50.0, 140.0), state: SignalState::Green, lights: String::new(), blink: false }
    }
}

impl Component for SignalLight {
    leaf_control!("SignalLight");
    paint_value_props!();

    properties! {
        state: SignalState => "Data",
        lights: String => "Data",
        blink: bool => "Data",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let stack = [SignalState::Red, SignalState::Yellow, SignalState::Green];
        let on: Vec<SignalState> = if self.lights.trim().is_empty() {
            vec![self.state]
        } else {
            self.lights.split(';').filter_map(|s| SignalState::from_name(s.trim())).collect()
        };
        let lit = !self.blink || (ctx.time * 2.0).fract() < 0.5;
        if self.blink {
            blink_wake(ctx);
        }
        let pole = 10.0;
        let seg = (r.height - pole) / stack.len() as f32;
        for (i, s) in stack.iter().enumerate() {
            let rect = RectF::new(r.x + 4.0, r.y + seg * i as f32, r.width - 8.0, seg - 2.0);
            let active = lit && on.contains(s);
            let color = if active { s.color() } else { s.color().with_opacity(0.18) };
            canvas.fill_rect(rect, 4.0, &Brush::Solid(color));
            canvas.stroke_rect(rect, 4.0, &Stroke::new(metal(ctx), 1.0));
            if active {
                canvas.fill_rect(RectF::new(rect.x + rect.width * 0.25, rect.y + rect.height * 0.3, rect.width * 0.5, rect.height * 0.4), 3.0, &Brush::Solid(Color::WHITE.with_opacity(0.35)));
            }
        }
        canvas.fill_rect(RectF::new(r.x + r.width / 2.0 - 4.0, r.bottom() - pole, 8.0, pole), 2.0, &Brush::Solid(metal(ctx)));
    }
}

// ===========================================================================
// Equipamentos
// ===========================================================================

/// O estado de uma válvula.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValveState {
    Open,
    #[default]
    Closed,
    /// Abrindo ou fechando (amarelo).
    Transit,
}

impl PropEnum for ValveState {
    const NAMES: &'static [&'static str] = &["open", "closed", "transit"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Open, Self::Closed, Self::Transit].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Válvula (gaveta): os dois triângulos e o atuador; verde aberta, cinza fechada, amarela
/// em trânsito. Clicar dispara `on_click` (quem abre é o app).
pub struct Valve {
    pub base: ControlBase,
    pub state: ValveState,
    pub alarm: bool,
}

impl Default for Valve {
    fn default() -> Self {
        let mut base = ControlBase::new(80.0, 70.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, state: ValveState::Open, alarm: false }
    }
}

impl Component for Valve {
    leaf_control!("Valve");
    paint_value_props!();

    properties! {
        state: ValveState => "Data",
        alarm: bool => "Data";
        events: on_click
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = match self.state {
            _ if self.alarm => ctx.theme.color("danger"),
            ValveState::Open => ctx.theme.color("success"),
            ValveState::Closed => metal(ctx),
            ValveState::Transit => ctx.theme.color("warning"),
        };
        let body_top = r.y + r.height * 0.4;
        let (x0, x1, y0, y1) = (r.x + 4.0, r.right() - 4.0, body_top, r.bottom() - 4.0);
        let cx = r.x + r.width / 2.0;
        let cy = (y0 + y1) / 2.0;
        let mut tri = Path::new();
        tri.move_to(x0, y0).line_to(cx, cy).line_to(x0, y1).close();
        tri.move_to(x1, y0).line_to(cx, cy).line_to(x1, y1).close();
        canvas.fill_path(&tri, &Brush::Solid(color));
        canvas.stroke_path(&tri, &Stroke::new(ink(ctx), 1.5));
        canvas.draw_line(PointF::new(cx, cy), PointF::new(cx, r.y + r.height * 0.18), &Stroke::new(ink(ctx), 2.0));
        let act = RectF::new(cx - r.width * 0.18, r.y + 4.0, r.width * 0.36, r.height * 0.16);
        canvas.fill_rect(act, 3.0, &Brush::Solid(color));
        canvas.stroke_rect(act, 3.0, &Stroke::new(ink(ctx), 1.5));
        alarm_ring(canvas, ctx, self.alarm);
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_click", EventArgs::None);
    }
}

/// Os equipamentos que giram: motor, bomba e ventilador (o mesmo jeito de pintar o estado).
macro_rules! rotating {
    ($name:ident, $class:literal, $w:expr, $h:expr, $doc:literal) => {
        #[doc = $doc]
        pub struct $name {
            pub base: ControlBase,
            pub running: bool,
            pub alarm: bool,
            /// Voltas por segundo do desenho, ligado.
            pub speed: f64,
            pub label: String,
        }

        impl Default for $name {
            fn default() -> Self {
                let mut base = ControlBase::new($w, $h);
                base.set_cursor(CursorShape::Pointer);
                Self { base, running: true, alarm: false, speed: 1.0, label: String::new() }
            }
        }
    };
}

rotating!(Motor, "Motor", 110.0, 80.0, "Motor: a carcaça com o eixo; verde ligado, cinza parado, vermelho em alarme. Clicar dispara `on_click`.");
rotating!(Pump, "Pump", 100.0, 90.0, "Bomba centrífuga: o rotor gira ligado.");
rotating!(Fan, "Fan", 90.0, 90.0, "Ventilador: as pás giram na velocidade (`speed`).");

macro_rules! rotating_component {
    ($name:ident, $class:literal) => {
        impl Component for $name {
            leaf_control!($class);
            paint_value_props!();

            properties! {
                running: bool => "Data",
                alarm: bool => "Data",
                speed: f64 => "Behavior",
                label: String => "Appearance";
                events: on_click
            }

            fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
                self.draw(canvas, ctx);
                alarm_ring(canvas, ctx, self.alarm);
            }

            fn click(&mut self, ctx: &mut EventCtx<'_>) {
                ctx.fire("on_click", EventArgs::None);
            }
        }
    };
}

rotating_component!(Motor, "Motor");
rotating_component!(Pump, "Pump");
rotating_component!(Fan, "Fan");

impl Motor {
    fn draw(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = state_color(ctx, self.running, self.alarm);
        let body = RectF::new(r.x + 4.0, r.y + r.height * 0.15, r.width * 0.68, r.height * 0.62);
        canvas.fill_rect(body, 6.0, &Brush::Linear { start: PointF::new(0.0, body.y), end: PointF::new(0.0, body.bottom()), stops: vec![GradientStop::new(0.0, metal(ctx).lerp(Color::WHITE, 0.2)), GradientStop::new(1.0, metal(ctx).lerp(Color::BLACK, 0.15))] });
        for i in 1..5 {
            let x = body.x + body.width * i as f32 / 5.0;
            canvas.draw_line(PointF::new(x, body.y + 3.0), PointF::new(x, body.bottom() - 3.0), &Stroke::new(ink(ctx).with_opacity(0.25), 1.0));
        }
        canvas.stroke_rect(body, 6.0, &Stroke::new(ink(ctx), 1.5));
        let shaft = RectF::new(body.right(), body.y + body.height * 0.4, r.right() - body.right() - 4.0, body.height * 0.2);
        canvas.fill_rect(shaft, 2.0, &Brush::Solid(metal(ctx)));
        // O acoplamento girando (uma marca que dá a volta).
        let a = spin(ctx, self.running, self.speed).to_radians();
        let mark_y = shaft.y + shaft.height / 2.0 + a.sin() * shaft.height * 0.4;
        canvas.draw_line(PointF::new(shaft.x + shaft.width * 0.6, mark_y - 1.0), PointF::new(shaft.x + shaft.width * 0.6, mark_y + 1.0), &Stroke::new(ink(ctx), 2.0));
        let feet = RectF::new(body.x + 6.0, body.bottom(), body.width - 12.0, r.height * 0.1);
        canvas.fill_rect(feet, 2.0, &Brush::Solid(metal(ctx).lerp(Color::BLACK, 0.2)));
        canvas.fill_ellipse(RectF::new(body.x + 6.0, body.y + 6.0, 10.0, 10.0), &Brush::Solid(color));
        caption(canvas, ctx, if self.label.is_empty() { "M" } else { &self.label }, body, (body.height * 0.4).min(22.0), true);
    }
}

impl Pump {
    fn draw(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = state_color(ctx, self.running, self.alarm);
        let side = r.width.min(r.height) * 0.8;
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height * 0.45);
        let circle = RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side);
        let base = RectF::new(r.x + r.width * 0.2, r.bottom() - r.height * 0.12, r.width * 0.6, r.height * 0.1);
        canvas.fill_rect(base, 2.0, &Brush::Solid(metal(ctx).lerp(Color::BLACK, 0.2)));
        canvas.fill_rect(RectF::new(c.x, circle.y, side / 2.0 + 4.0, side * 0.24), 2.0, &Brush::Solid(metal(ctx)));
        canvas.fill_ellipse(circle, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_ellipse(circle, &Stroke::new(ink(ctx), 2.0));
        let a = spin(ctx, self.running, self.speed);
        canvas.save();
        canvas.translate(c.x, c.y);
        canvas.rotate(a);
        for k in 0..6 {
            let t = (k as f32 * 60.0).to_radians();
            canvas.draw_line(PointF::new(0.0, 0.0), PointF::new(t.cos() * side * 0.38, t.sin() * side * 0.38), &Stroke::new(color, 3.0).round());
        }
        canvas.restore();
        canvas.fill_ellipse(RectF::new(c.x - 4.0, c.y - 4.0, 8.0, 8.0), &Brush::Solid(ink(ctx)));
        if !self.label.is_empty() {
            caption(canvas, ctx, &self.label, RectF::new(r.x, r.bottom() - 14.0, r.width, 14.0), 10.0, false);
        }
    }
}

impl Fan {
    fn draw(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = state_color(ctx, self.running, self.alarm);
        let side = r.width.min(r.height) - 6.0;
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let ring = RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side);
        canvas.stroke_ellipse(ring, &Stroke::new(metal(ctx), 3.0));
        let a = spin(ctx, self.running, self.speed * 2.0);
        canvas.save();
        canvas.translate(c.x, c.y);
        canvas.rotate(a);
        for k in 0..3 {
            canvas.save();
            canvas.rotate(k as f32 * 120.0);
            let rad = side * 0.45;
            let mut blade = Path::new();
            blade.move_to(0.0, 0.0).cubic_to(rad * 0.3, -rad * 0.5, rad * 0.9, -rad * 0.35, rad, 0.0).cubic_to(rad * 0.7, rad * 0.15, rad * 0.3, rad * 0.1, 0.0, 0.0);
            canvas.fill_path(&blade, &Brush::Solid(color));
            canvas.restore();
        }
        canvas.restore();
        canvas.fill_ellipse(RectF::new(c.x - side * 0.08, c.y - side * 0.08, side * 0.16, side * 0.16), &Brush::Solid(ink(ctx)));
    }
}

/// Esteira: a correia com os rolos girando e as marcas andando quando ligada.
pub struct Conveyor {
    pub base: ControlBase,
    pub running: bool,
    pub alarm: bool,
    /// Velocidade do desenho (pixels por segundo); negativa anda para a esquerda.
    pub speed: f64,
}

impl Default for Conveyor {
    fn default() -> Self {
        Self { base: ControlBase::new(220.0, 50.0), running: true, alarm: false, speed: 40.0 }
    }
}

impl Component for Conveyor {
    leaf_control!("Conveyor");
    paint_value_props!();

    properties! {
        running: bool => "Data",
        alarm: bool => "Data",
        speed: f64 => "Behavior",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let h = (r.height - 8.0).max(10.0);
        let belt = RectF::new(r.x + 2.0, r.y + (r.height - h) / 2.0, r.width - 4.0, h);
        canvas.fill_rect(belt, h / 2.0, &Brush::Solid(Color::rgb(50, 52, 58)));
        let roll = h * 0.7;
        let offset = if self.running {
            ctx.request_frame();
            ((ctx.time * self.speed) % 20.0) as f32
        } else {
            0.0
        };
        // As marcas da correia.
        canvas.save();
        canvas.clip_round_rect(belt, h / 2.0);
        let mut x = belt.x - 20.0 + offset;
        while x < belt.right() + 20.0 {
            canvas.draw_line(PointF::new(x, belt.y + 2.0), PointF::new(x + 6.0, belt.y + 2.0), &Stroke::new(Color::rgb(140, 140, 150), 2.0));
            canvas.draw_line(PointF::new(x, belt.bottom() - 2.0), PointF::new(x + 6.0, belt.bottom() - 2.0), &Stroke::new(Color::rgb(140, 140, 150), 2.0));
            x += 20.0;
        }
        canvas.restore();
        let n = ((belt.width / (roll * 1.6)) as usize).max(2);
        for i in 0..n {
            let cx = belt.x + h / 2.0 + (belt.width - h) * i as f32 / (n - 1) as f32;
            let rr = RectF::new(cx - roll / 2.0, belt.y + (h - roll) / 2.0, roll, roll);
            canvas.fill_ellipse(rr, &Brush::Solid(metal(ctx)));
            let a = (offset / 20.0 * 2.0 * PI) as f32;
            let c = PointF::new(cx, rr.y + roll / 2.0);
            canvas.draw_line(c, PointF::new(c.x + a.cos() * roll * 0.4, c.y + a.sin() * roll * 0.4), &Stroke::new(ink(ctx), 1.5));
        }
        canvas.stroke_rect(belt, h / 2.0, &Stroke::new(state_color(ctx, self.running, self.alarm), 2.0));
        alarm_ring(canvas, ctx, self.alarm);
    }
}

/// Silo: o corpo com o funil embaixo e o nível do material (`value`, %).
pub struct Silo {
    pub base: ControlBase,
    pub value: f64,
    pub material: Color,
    pub alarm: bool,
}

impl Default for Silo {
    fn default() -> Self {
        Self { base: ControlBase::new(100.0, 170.0), value: 55.0, material: Color::rgb(214, 170, 90), alarm: false }
    }
}

impl Component for Silo {
    leaf_control!("Silo");
    paint_value_props!();

    properties! {
        value: f64 => "Data",
        material: Color => "Appearance",
        alarm: bool => "Data",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect.inflate(-3.0);
        let cone = r.height * 0.28;
        let mut body = Path::new();
        body.move_to(r.x, r.y + r.width * 0.15)
            .quad_to(r.x + r.width / 2.0, r.y - r.width * 0.05, r.right(), r.y + r.width * 0.15)
            .line_to(r.right(), r.bottom() - cone)
            .line_to(r.x + r.width * 0.62, r.bottom())
            .line_to(r.x + r.width * 0.38, r.bottom())
            .line_to(r.x, r.bottom() - cone)
            .close();
        canvas.fill_path(&body, &Brush::Solid(ctx.theme.color("surface")));
        let f = fraction(self.value, 0.0, 100.0);
        canvas.save();
        canvas.clip_path(&body, false);
        let level = r.bottom() - r.height * f;
        canvas.fill_rect(RectF::new(r.x, level, r.width, r.bottom() - level), 0.0, &Brush::Solid(self.material));
        canvas.restore();
        canvas.stroke_path(&body, &Stroke::new(metal(ctx), 2.0));
        caption(canvas, ctx, &format!("{:.0}%", self.value), RectF::new(r.x, r.y + r.height * 0.35, r.width, 20.0), 14.0, true);
        alarm_ring(canvas, ctx, self.alarm);
    }
}

// ===========================================================================
// Comandos
// ===========================================================================

/// Botão de emergência (cogumelo vermelho sobre fundo amarelo): o clique trava e destrava
/// (`pressed`), com `on_change`.
pub struct EmergencyStop {
    pub base: ControlBase,
    pub pressed: bool,
}

impl Default for EmergencyStop {
    fn default() -> Self {
        let mut base = ControlBase::new(90.0, 90.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, pressed: false }
    }
}

impl Component for EmergencyStop {
    leaf_control!("EmergencyStop");
    paint_value_props!();

    properties! {
        pressed: bool => "Data";
        events: on_change
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = r.width.min(r.height) - 4.0;
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        let plate = RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side);
        canvas.fill_ellipse(plate, &Brush::Solid(Color::rgb(250, 205, 30)));
        canvas.stroke_ellipse(plate, &Stroke::new(ink(ctx), 1.5));
        let head = side * if self.pressed { 0.56 } else { 0.66 };
        let hr = RectF::new(c.x - head / 2.0, c.y - head / 2.0, head, head);
        let red = Color::rgb(210, 30, 30);
        canvas.fill_ellipse(hr, &Brush::Radial { center: PointF::new(c.x - head * 0.15, c.y - head * 0.15), radius: head * 0.7, stops: vec![GradientStop::new(0.0, red.lerp(Color::WHITE, 0.3)), GradientStop::new(1.0, red.lerp(Color::BLACK, 0.2))] });
        if self.pressed {
            canvas.stroke_ellipse(hr.inflate(3.0), &Stroke::new(red, 2.0).dashed(vec![4.0, 3.0]));
        }
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        self.pressed = !self.pressed;
        ctx.fire("on_change", EventArgs::Value(aylis_core::PropValue::Bool(self.pressed)));
        ctx.repaint();
    }
}

/// Chave seletora (`Man;0;Auto`): a manopla aponta para a posição escolhida; o clique passa
/// para a próxima (com `on_change`).
pub struct SelectorSwitch {
    pub base: ControlBase,
    pub positions: String,
    pub item_index: i64,
}

impl Default for SelectorSwitch {
    fn default() -> Self {
        let mut base = ControlBase::new(110.0, 100.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, positions: "Man;0;Auto".into(), item_index: 1 }
    }
}

impl SelectorSwitch {
    fn list(&self) -> Vec<&str> {
        self.positions.split(';').map(str::trim).filter(|s| !s.is_empty()).collect()
    }
}

impl Component for SelectorSwitch {
    leaf_control!("SelectorSwitch");
    paint_value_props!();

    properties! {
        positions: String => "Data",
        item_index: i64 => "Data";
        events: on_change
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let list = self.list();
        let n = list.len().max(1);
        let side = (r.width.min(r.height) * 0.62).max(10.0);
        let c = PointF::new(r.x + r.width / 2.0, r.y + r.height * 0.6);
        // As posições num arco de 90° em cima.
        let angle = |i: usize| -> f32 { if n == 1 { -90.0 } else { -135.0 + 90.0 * i as f32 / (n - 1) as f32 } };
        for (i, label) in list.iter().enumerate() {
            let a = angle(i).to_radians();
            let p = PointF::new(c.x + a.cos() * side * 0.78, c.y + a.sin() * side * 0.78);
            caption(canvas, ctx, label, RectF::new(p.x - 24.0, p.y - 8.0, 48.0, 16.0), 10.0, i as i64 == self.item_index);
        }
        let knob = RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side);
        canvas.fill_ellipse(knob, &Brush::Solid(Color::rgb(40, 40, 44)));
        let a = angle(self.item_index.clamp(0, n as i64 - 1) as usize);
        canvas.save();
        canvas.translate(c.x, c.y);
        canvas.rotate(a + 90.0);
        canvas.fill_rect(RectF::new(-side * 0.1, -side * 0.48, side * 0.2, side * 0.96), side * 0.08, &Brush::Solid(Color::rgb(230, 230, 230)));
        canvas.restore();
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        let n = self.list().len().max(1) as i64;
        self.item_index = (self.item_index + 1).rem_euclid(n);
        ctx.fire("on_change", EventArgs::Value(aylis_core::PropValue::Int(self.item_index)));
        ctx.repaint();
    }
}

/// Botoeira: botões de painel lado a lado (`buttons`: `Liga:green;Desliga:red`); o clique
/// num deles dispara `on_click` com o índice (e o texto) dele.
pub struct PushButtons {
    pub base: ControlBase,
    pub buttons: String,
    /// O botão que está aceso (o último clicado; -1 = nenhum).
    pub item_index: i64,
}

impl Default for PushButtons {
    fn default() -> Self {
        let mut base = ControlBase::new(170.0, 80.0);
        base.set_cursor(CursorShape::Pointer);
        Self { base, buttons: "Liga:green;Desliga:red".into(), item_index: -1 }
    }
}

impl PushButtons {
    fn list(&self) -> Vec<(String, Color)> {
        self.buttons
            .split(';')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let (label, color) = s.split_once(':').unwrap_or((s, "gray"));
                let color = match color.trim() {
                    "green" => Color::rgb(40, 170, 70),
                    "red" => Color::rgb(210, 40, 40),
                    "yellow" => Color::rgb(240, 190, 30),
                    "blue" => Color::rgb(40, 110, 220),
                    "black" => Color::rgb(30, 30, 30),
                    other => Color::from_hex(other).unwrap_or(Color::rgb(130, 130, 130)),
                };
                (label.trim().to_string(), color)
            })
            .collect()
    }

    fn slot(&self, r: RectF, i: usize, n: usize) -> RectF {
        let w = r.width / n.max(1) as f32;
        RectF::new(r.x + w * i as f32, r.y, w, r.height)
    }
}

impl Component for PushButtons {
    leaf_control!("PushButtons");
    paint_value_props!();

    properties! {
        buttons: String => "Data",
        item_index: i64 => "Data";
        events: on_click
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 6.0, &Brush::Solid(Color::rgb(70, 72, 78)));
        let list = self.list();
        for (i, (label, color)) in list.iter().enumerate() {
            let s = self.slot(r, i, list.len());
            let side = (s.width.min(s.height - 16.0) * 0.7).max(8.0);
            let c = PointF::new(s.x + s.width / 2.0, s.y + (s.height - 16.0) / 2.0 + 2.0);
            let ring = RectF::new(c.x - side / 2.0 - 3.0, c.y - side / 2.0 - 3.0, side + 6.0, side + 6.0);
            canvas.fill_ellipse(ring, &Brush::Solid(Color::rgb(180, 180, 186)));
            let active = i as i64 == self.item_index;
            let face = if active { color.lerp(Color::WHITE, 0.25) } else { *color };
            canvas.fill_ellipse(RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side), &Brush::Solid(face));
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Center;
            t.font.size = 10.0;
            // O texto do painel: branco, ou o `text_color` próprio.
            t.color = ctx.colors.text.unwrap_or(Color::WHITE);
            canvas.draw_text(label, RectF::new(s.x, s.bottom() - 16.0, s.width, 14.0), &t);
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::PointerEvent) {
        let r = ctx.ui.control(ctx.id).map_or(RectF::default(), |c| RectF::from_size(c.bounds().size()));
        let list = self.list();
        if let Some(i) = (0..list.len()).find(|&i| self.slot(r, i, list.len()).contains(event.position)) {
            self.item_index = i as i64;
            ctx.fire("on_click", EventArgs::Text(list[i].0.clone()));
            ctx.repaint();
        }
    }
}

// ===========================================================================
// Canos e fios
// ===========================================================================

/// Um lado de um componente (o ponto de ligação).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Port {
    Left,
    #[default]
    Right,
    Top,
    Bottom,
}

impl PropEnum for Port {
    const NAMES: &'static [&'static str] = &["left", "right", "top", "bottom"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Left, Self::Right, Self::Top, Self::Bottom].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl Port {
    fn at(self) -> (f32, f32) {
        match self {
            Port::Left => (0.0, 0.5),
            Port::Right => (1.0, 0.5),
            Port::Top => (0.5, 0.0),
            Port::Bottom => (0.5, 1.0),
        }
    }
}

/// O que canos e fios têm em comum: as pontas e o caminho.
fn connection_of(from: Option<ComponentId>, from_port: Port, to: Option<ComponentId>, to_port: Port) -> Option<Connection> {
    Some(Connection { from: from?, from_at: from_port.at(), to: to?, to_at: to_port.at() })
}

/// O caminho nas coordenadas do próprio conector (o layout já o pôs em volta dele).
fn local_route(ctx: &PaintCtx<'_>, c: Option<Connection>) -> Option<Vec<PointF>> {
    let c = c?;
    let ui: &Ui = ctx.ui;
    let (a, b, me) = (ui.control(c.from)?.bounds(), ui.control(c.to)?.bounds(), ui.control(ctx.id)?.bounds());
    Some(c.route(a, b).into_iter().map(|p| PointF::new(p.x - me.x + ctx.rect.x, p.y - me.y + ctx.rect.y)).collect())
}

fn polyline(points: &[PointF]) -> Path {
    let mut p = Path::new();
    if let Some(first) = points.first() {
        p.move_to(first.x, first.y);
        for q in &points[1..] {
            p.line_to(q.x, q.y);
        }
    }
    p
}

/// Cano preso a dois componentes (`from`/`to` e o lado de cada um): segue os dois e faz os
/// cotovelos; `running` mostra o fluxo andando na cor do produto.
pub struct Pipe {
    pub base: ControlBase,
    pub from: Option<ComponentId>,
    pub from_port: Port,
    pub to: Option<ComponentId>,
    pub to_port: Port,
    pub running: bool,
    pub alarm: bool,
    pub flow_color: Color,
    pub thickness: f32,
}

impl Default for Pipe {
    fn default() -> Self {
        let mut base = ControlBase::new(120.0, 40.0);
        base.set_hit_test(false);
        Self { base, from: None, from_port: Port::Right, to: None, to_port: Port::Left, running: true, alarm: false, flow_color: Color::rgb(40, 140, 230), thickness: 10.0 }
    }
}

impl Component for Pipe {
    leaf_control!("Pipe");
    paint_value_props!();

    properties! {
        from: Option<ComponentId> => "Behavior",
        from_port: Port => "Behavior",
        to: Option<ComponentId> => "Behavior",
        to_port: Port => "Behavior",
        running: bool => "Data",
        alarm: bool => "Data",
        flow_color: Color => "Appearance",
        thickness: f32 => "Appearance",
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.from == Some(freed) {
            self.from = None;
        }
        if self.to == Some(freed) {
            self.to = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let points = local_route(ctx, aylis_core::PlatformHooks::connection(self)).unwrap_or_else(|| vec![PointF::new(r.x + 4.0, r.y + r.height / 2.0), PointF::new(r.right() - 4.0, r.y + r.height / 2.0)]);
        let path = polyline(&points);
        let t = self.thickness.max(2.0);
        let mut outer = Stroke::new(metal(ctx).lerp(Color::BLACK, 0.25), t + 3.0);
        outer.join = aylis_core::LineJoin::Round;
        canvas.stroke_path(&path, &outer);
        let mut inner = Stroke::new(metal(ctx).lerp(Color::WHITE, 0.15), t);
        inner.join = aylis_core::LineJoin::Round;
        canvas.stroke_path(&path, &inner);
        if self.running {
            ctx.request_frame();
            let offset = ((ctx.time * 30.0) % 16.0) as f32;
            let color = if self.alarm { ctx.theme.color("danger") } else { self.flow_color };
            let mut flow = Stroke::new(color, t * 0.5).dashed(vec![8.0, 8.0]);
            flow.join = aylis_core::LineJoin::Round;
            // O tracejado anda: começa um pouco antes a cada quadro.
            canvas.stroke_path(&shifted(&points, offset), &flow);
        }
    }
}

/// O caminho começando `offset` pixels "antes" (o tracejado parece andar).
fn shifted(points: &[PointF], offset: f32) -> Path {
    let mut pts = points.to_vec();
    if pts.len() >= 2 {
        let (a, b) = (pts[0], pts[1]);
        let len = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt().max(1e-3);
        pts.insert(0, PointF::new(a.x - (b.x - a.x) / len * offset, a.y - (b.y - a.y) / len * offset));
    }
    polyline(&pts)
}

/// Fio elétrico preso a dois componentes (como o [`Pipe`], fino); `energized` o pinta na
/// cor de energizado.
pub struct Wire {
    pub base: ControlBase,
    pub from: Option<ComponentId>,
    pub from_port: Port,
    pub to: Option<ComponentId>,
    pub to_port: Port,
    pub energized: bool,
    pub color: Color,
}

impl Default for Wire {
    fn default() -> Self {
        let mut base = ControlBase::new(120.0, 30.0);
        base.set_hit_test(false);
        Self { base, from: None, from_port: Port::Right, to: None, to_port: Port::Left, energized: false, color: Color::rgb(60, 60, 60) }
    }
}

impl Component for Wire {
    leaf_control!("Wire");
    paint_value_props!();

    properties! {
        from: Option<ComponentId> => "Behavior",
        from_port: Port => "Behavior",
        to: Option<ComponentId> => "Behavior",
        to_port: Port => "Behavior",
        energized: bool => "Data",
        color: Color => "Appearance",
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.from == Some(freed) {
            self.from = None;
        }
        if self.to == Some(freed) {
            self.to = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let points = local_route(ctx, aylis_core::PlatformHooks::connection(self)).unwrap_or_else(|| vec![PointF::new(r.x + 2.0, r.y + r.height / 2.0), PointF::new(r.right() - 2.0, r.y + r.height / 2.0)]);
        let color = if self.energized { Color::rgb(230, 120, 20) } else { self.color };
        let mut s = Stroke::new(color, 2.0);
        s.join = aylis_core::LineJoin::Round;
        canvas.stroke_path(&polyline(&points), &s);
        for p in [points.first(), points.last()].into_iter().flatten() {
            canvas.fill_ellipse(RectF::new(p.x - 2.5, p.y - 2.5, 5.0, 5.0), &Brush::Solid(color));
        }
    }
}

// ===========================================================================
// Símbolos elétricos
// ===========================================================================

/// Qual símbolo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Symbol {
    #[default]
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Led,
    Transistor,
    Source,
    Ground,
    Switch,
    Relay,
    Lamp,
    Motor,
}

impl PropEnum for Symbol {
    const NAMES: &'static [&'static str] = &["resistor", "capacitor", "inductor", "diode", "led", "transistor", "source", "ground", "switch", "relay", "lamp", "motor"];
    fn from_index(index: usize) -> Option<Self> {
        use Symbol::*;
        [Resistor, Capacitor, Inductor, Diode, Led, Transistor, Source, Ground, Switch, Relay, Lamp, Motor].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os subtipos da paleta: (nome, texto).
pub const SYMBOLS: [(&str, &str); 12] = [
    ("resistor", "Resistor"),
    ("capacitor", "Capacitor"),
    ("inductor", "Indutor"),
    ("diode", "Diodo"),
    ("led", "LED"),
    ("transistor", "Transistor"),
    ("source", "Fonte"),
    ("ground", "Terra"),
    ("switch", "Chave"),
    ("relay", "Relé"),
    ("lamp", "Lâmpada"),
    ("motor", "Motor (símbolo)"),
];

/// Um símbolo elétrico (o diagrama, na horizontal; gire pelo `rotation`). `state` liga a
/// chave, o relé, a lâmpada e o LED; `label` é o valor (`10 kΩ`, `24 V`). Os terminais ficam
/// nos lados esquerdo e direito (os `Wire` prendem ali).
pub struct ElectricalSymbol {
    pub base: ControlBase,
    pub symbol: Symbol,
    pub state: bool,
    pub label: String,
}

impl Default for ElectricalSymbol {
    fn default() -> Self {
        Self { base: ControlBase::new(90.0, 50.0), symbol: Symbol::Resistor, state: false, label: String::new() }
    }
}

impl Component for ElectricalSymbol {
    leaf_control!("ElectricalSymbol");

    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "state" | "label")
    }

    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        symbol: Symbol => "Appearance",
        state: bool => "Data",
        label: String => "Appearance";
        events: on_click
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_click", EventArgs::None);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let label_h = if self.label.is_empty() { 0.0 } else { 14.0 };
        let area = RectF::new(r.x, r.y, r.width, r.height - label_h);
        let cy = area.y + area.height / 2.0;
        let (x0, x1) = (area.x + 2.0, area.right() - 2.0);
        let w = x1 - x0;
        let h = area.height * 0.6;
        let s = Stroke::new(ink(ctx), 2.0).round();
        let lead = |canvas: &mut dyn Canvas, a: f32, b: f32| {
            canvas.draw_line(PointF::new(x0, cy), PointF::new(a, cy), &s);
            canvas.draw_line(PointF::new(b, cy), PointF::new(x1, cy), &s);
        };
        let (a, b) = (x0 + w * 0.3, x1 - w * 0.3);
        let on = self.state;
        match self.symbol {
            Symbol::Resistor => {
                lead(canvas, a, b);
                let mut zig = Path::new();
                zig.move_to(a, cy);
                for i in 0..6 {
                    let x = a + (b - a) * (i as f32 + 0.5) / 6.0;
                    zig.line_to(x, cy + if i % 2 == 0 { -h / 3.0 } else { h / 3.0 });
                }
                zig.line_to(b, cy);
                canvas.stroke_path(&zig, &s);
            }
            Symbol::Capacitor => {
                let m = (a + b) / 2.0;
                lead(canvas, m - 4.0, m + 4.0);
                canvas.draw_line(PointF::new(m - 4.0, cy - h / 2.0), PointF::new(m - 4.0, cy + h / 2.0), &s);
                canvas.draw_line(PointF::new(m + 4.0, cy - h / 2.0), PointF::new(m + 4.0, cy + h / 2.0), &s);
            }
            Symbol::Inductor => {
                lead(canvas, a, b);
                let mut coil = Path::new();
                let n = 4;
                let step = (b - a) / n as f32;
                for i in 0..n {
                    let cx = a + step * (i as f32 + 0.5);
                    coil.arc(PointF::new(cx, cy), step / 2.0, h / 3.0, 180.0, 180.0);
                }
                canvas.stroke_path(&coil, &s);
            }
            Symbol::Diode | Symbol::Led => {
                lead(canvas, a, b);
                let mut tri = Path::new();
                tri.move_to(a, cy - h / 2.0).line_to(b - 3.0, cy).line_to(a, cy + h / 2.0).close();
                let fill = if self.symbol == Symbol::Led && on { Color::rgb(255, 60, 40) } else { Color::TRANSPARENT };
                canvas.fill_path(&tri, &Brush::Solid(fill));
                canvas.stroke_path(&tri, &s);
                canvas.draw_line(PointF::new(b - 3.0, cy - h / 2.0), PointF::new(b - 3.0, cy + h / 2.0), &s);
                if self.symbol == Symbol::Led {
                    for k in 0..2 {
                        let ox = a + (b - a) * 0.4 + k as f32 * 7.0;
                        canvas.draw_line(PointF::new(ox, cy - h / 2.0 - 2.0), PointF::new(ox + 6.0, cy - h / 2.0 - 8.0), &Stroke::new(ink(ctx), 1.5));
                    }
                }
            }
            Symbol::Transistor => {
                let c = PointF::new((a + b) / 2.0, cy);
                let rad = h / 2.0 + 2.0;
                canvas.stroke_ellipse(RectF::new(c.x - rad, c.y - rad, rad * 2.0, rad * 2.0), &s);
                canvas.draw_line(PointF::new(x0, cy), PointF::new(c.x - rad * 0.3, cy), &s);
                canvas.draw_line(PointF::new(c.x - rad * 0.3, cy - rad * 0.6), PointF::new(c.x - rad * 0.3, cy + rad * 0.6), &s);
                canvas.draw_line(PointF::new(c.x - rad * 0.3, cy - rad * 0.25), PointF::new(c.x + rad * 0.5, cy - rad * 0.8), &s);
                canvas.draw_line(PointF::new(c.x - rad * 0.3, cy + rad * 0.25), PointF::new(c.x + rad * 0.5, cy + rad * 0.8), &s);
                canvas.draw_line(PointF::new(c.x + rad * 0.5, cy + rad * 0.8), PointF::new(x1, cy + rad * 0.8), &s);
            }
            Symbol::Source => {
                let c = PointF::new((a + b) / 2.0, cy);
                let rad = h / 2.0 + 2.0;
                lead(canvas, c.x - rad, c.x + rad);
                canvas.stroke_ellipse(RectF::new(c.x - rad, c.y - rad, rad * 2.0, rad * 2.0), &s);
                caption(canvas, ctx, "+  −", RectF::new(c.x - rad, c.y - 8.0, rad * 2.0, 16.0), 11.0, true);
            }
            Symbol::Ground => {
                let m = (x0 + x1) / 2.0;
                canvas.draw_line(PointF::new(m, area.y + 2.0), PointF::new(m, cy), &s);
                for (k, half) in [(0.0, h / 2.0), (5.0, h / 3.0), (10.0, h / 6.0)] {
                    canvas.draw_line(PointF::new(m - half, cy + k), PointF::new(m + half, cy + k), &s);
                }
            }
            Symbol::Switch => {
                lead(canvas, a, b);
                canvas.fill_ellipse(RectF::new(a - 3.0, cy - 3.0, 6.0, 6.0), &Brush::Solid(ink(ctx)));
                canvas.fill_ellipse(RectF::new(b - 3.0, cy - 3.0, 6.0, 6.0), &Brush::Solid(ink(ctx)));
                let end = if on { PointF::new(b, cy) } else { PointF::new(b - 2.0, cy - h / 2.0) };
                canvas.draw_line(PointF::new(a, cy), end, &s);
            }
            Symbol::Relay => {
                let coil = RectF::new(a, cy - h / 2.0, (b - a) * 0.45, h);
                lead(canvas, a, b);
                canvas.fill_rect(coil, 2.0, &Brush::Solid(if on { ctx.theme.color("success").with_opacity(0.4) } else { Color::TRANSPARENT }));
                canvas.stroke_rect(coil, 2.0, &s);
                canvas.draw_line(PointF::new(coil.x, coil.bottom()), PointF::new(coil.right(), coil.y), &Stroke::new(ink(ctx), 1.0));
                let (ca, cb) = (coil.right() + 6.0, b);
                let end = if on { PointF::new(cb, cy) } else { PointF::new(cb - 2.0, cy - h / 2.0) };
                canvas.draw_line(PointF::new(ca, cy), end, &Stroke::new(ink(ctx), 2.0).dashed(vec![3.0, 2.0]));
            }
            Symbol::Lamp => {
                let c = PointF::new((a + b) / 2.0, cy);
                let rad = h / 2.0 + 2.0;
                lead(canvas, c.x - rad, c.x + rad);
                let ring = RectF::new(c.x - rad, c.y - rad, rad * 2.0, rad * 2.0);
                if on {
                    canvas.fill_ellipse(ring, &Brush::Solid(Color::rgb(255, 220, 80)));
                }
                canvas.stroke_ellipse(ring, &s);
                let d = rad * 0.7;
                canvas.draw_line(PointF::new(c.x - d, c.y - d), PointF::new(c.x + d, c.y + d), &s);
                canvas.draw_line(PointF::new(c.x - d, c.y + d), PointF::new(c.x + d, c.y - d), &s);
            }
            Symbol::Motor => {
                let c = PointF::new((a + b) / 2.0, cy);
                let rad = h / 2.0 + 2.0;
                lead(canvas, c.x - rad, c.x + rad);
                canvas.stroke_ellipse(RectF::new(c.x - rad, c.y - rad, rad * 2.0, rad * 2.0), &s);
                caption(canvas, ctx, "M", RectF::new(c.x - rad, c.y - 9.0, rad * 2.0, 18.0), 14.0, true);
            }
        }
        if !self.label.is_empty() {
            caption(canvas, ctx, &self.label, RectF::new(r.x, r.bottom() - label_h, r.width, label_h), 10.0, false);
        }
    }
}

/// Registra os componentes industriais e elétricos (e os subtipos dos símbolos).
pub fn register(ui: &mut Ui) {
    let industrial: [fn(&mut Ui, &'static str); 16] = [
        ui_reg::<Manometer>,
        ui_reg::<Thermometer>,
        ui_reg::<TankLevel>,
        ui_reg::<FlowMeter>,
        ui_reg::<SevenSegment>,
        ui_reg::<SignalLight>,
        ui_reg::<Valve>,
        ui_reg::<Motor>,
        ui_reg::<Pump>,
        ui_reg::<Conveyor>,
        ui_reg::<Fan>,
        ui_reg::<Silo>,
        ui_reg::<EmergencyStop>,
        ui_reg::<SelectorSwitch>,
        ui_reg::<PushButtons>,
        ui_reg::<Pipe>,
    ];
    for class in industrial {
        class(ui, "Industrial");
    }
    ui.register_class::<Wire>("Electrical");
    ui.register_class::<ElectricalSymbol>("Electrical");
    for (name, caption) in SYMBOLS {
        ui.register_variant("ElectricalSymbol", name, caption, vec![("symbol", aylis_core::PropValue::Enum(name.into()))]);
    }
}

fn ui_reg<T: Component + Default>(ui: &mut Ui, category: &'static str) {
    ui.register_class::<T>(category);
}

impl aylis_core::PlatformHooks for Pipe {
    fn connection(&self) -> Option<Connection> {
        connection_of(self.from, self.from_port, self.to, self.to_port)
    }
}

impl aylis_core::PlatformHooks for Wire {
    fn connection(&self) -> Option<Connection> {
        connection_of(self.from, self.from_port, self.to, self.to_port)
    }
}
