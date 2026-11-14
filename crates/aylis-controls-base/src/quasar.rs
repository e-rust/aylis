//! Componentes no estilo do Quasar (quasar.dev) que o aylis ainda não tinha: o cartão
//! (`QCard`), o botão giratório (`QKnob`), o carregando por cima (`QInnerLoading`), a
//! tabela simples (`QMarkupTable`), o item deslizante (`QSlideItem`), o fundo com
//! paralaxe (`QParallax`), o envio de arquivos (`QUploader`), a edição no lugar
//! (`QPopupEdit`), o grupo de opções (`QOptionGroup`), a barra de carregamento do topo
//! (`QAjaxBar`) e o balão de conversa (`QChatMessage`, aqui `ChatBubble`). Os estilos de
//! spinner do Quasar (`QSpinnerDots`, `QSpinnerBars`...) são o `kind` do
//! [`ActivityIndicator`](crate::ActivityIndicator) e do `InnerLoading`.
//!
//! As cores vêm do tema (`primary`, `surface`, `text_muted`...); com o tema "Quasar 2", são
//! as da marca do Quasar.

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, Image, Key, KeyEvent,
    PaintCtx, Path, PointF, PointerButton, PointerEvent, PropEnum, PropValue, RectF, Stroke, TextAlign, Trimming, WheelEvent,
    properties,
};

use crate::basic::Panel;
use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

/// O texto em negrito (ou não) num retângulo.
fn text(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, s: &str, rect: RectF, size: f32, color: Color, align: TextAlign, bold: bool) {
    let mut style = ctx.text_style();
    style.font = ctx.font(size);
    if bold {
        style.font.weight = FontWeight::SemiBold;
    }
    style.color = color;
    style.h_align = align;
    style.v_align = TextAlign::Center;
    style.trimming = Trimming::Character;
    canvas.draw_text(s, rect, &style);
}

/// Uma cor do tema, ou a outra se o tema não tiver.
fn token(ctx: &PaintCtx<'_>, name: &str, fallback: &str) -> Color {
    ctx.theme.colors.get(name).copied().unwrap_or_else(|| ctx.theme.color(fallback))
}

/// A cor de destaque: a própria do controle (`accent_color`) ou a `primary` do tema.
fn accent(ctx: &PaintCtx<'_>) -> Color {
    ctx.colors.accent.unwrap_or_else(|| ctx.theme.color("primary"))
}

// ===========================================================================
// Spinners
// ===========================================================================

/// Os desenhos de "carregando" do Quasar (`QSpinner`, `QSpinnerDots`, `QSpinnerBars`...).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerKind {
    /// O anel que gira e "respira" (o `QSpinner` padrão).
    #[default]
    Ring,
    Dots,
    Bars,
    Audio,
    Ball,
    Oval,
    Puff,
    Grid,
    Ios,
    Pie,
    Tail,
    Hourglass,
    Rings,
    Orbit,
}

impl PropEnum for SpinnerKind {
    const NAMES: &'static [&'static str] =
        &["ring", "dots", "bars", "audio", "ball", "oval", "puff", "grid", "ios", "pie", "tail", "hourglass", "rings", "orbit"];
    fn from_index(index: usize) -> Option<Self> {
        use SpinnerKind::*;
        [Ring, Dots, Bars, Audio, Ball, Oval, Puff, Grid, Ios, Pie, Tail, Hourglass, Rings, Orbit].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Desenha o spinner `kind` centralizado em `rect`, no instante `t` (segundos).
pub fn paint_spinner(canvas: &mut dyn Canvas, rect: RectF, kind: SpinnerKind, color: Color, t: f32) {
    let side = rect.width.min(rect.height).max(4.0);
    let c = PointF::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
    let r = side / 2.0;
    let tau = std::f32::consts::TAU;
    let dot = |canvas: &mut dyn Canvas, p: PointF, radius: f32, opacity: f32| {
        canvas.fill_ellipse(RectF::new(p.x - radius, p.y - radius, radius * 2.0, radius * 2.0), &Brush::Solid(color.with_opacity(opacity)));
    };
    match kind {
        SpinnerKind::Ring => {
            let width = (side / 9.0).max(2.0);
            let rr = r - width / 2.0;
            canvas.stroke_ellipse(RectF::new(c.x - rr, c.y - rr, rr * 2.0, rr * 2.0), &Stroke::new(color.with_opacity(0.18), width));
            let mut arc = Path::new();
            arc.arc(c, rr, rr, (t * 360.0) % 360.0, 60.0 + 200.0 * (0.5 + 0.5 * (t * 2.6).sin()));
            canvas.stroke_path(&arc, &Stroke::new(color, width).round());
        }
        SpinnerKind::Dots => {
            // Três bolinhas que crescem uma depois da outra.
            for i in 0..3 {
                let phase = ((t * 2.0 - i as f32 * 0.18) * tau).sin() * 0.5 + 0.5;
                let x = c.x + (i as f32 - 1.0) * r * 0.66;
                dot(canvas, PointF::new(x, c.y), r * (0.12 + 0.12 * phase), 0.4 + 0.6 * phase);
            }
        }
        SpinnerKind::Bars => {
            // Barras verticais que sobem e descem em onda.
            let n = 5;
            let w = side / (n as f32 * 1.7);
            for i in 0..n {
                let phase = ((t * 1.6 - i as f32 * 0.12) * tau).sin() * 0.5 + 0.5;
                let h = side * (0.35 + 0.6 * phase);
                let x = c.x - side / 2.0 + i as f32 * (side / n as f32) + (side / n as f32 - w) / 2.0;
                canvas.fill_rect(RectF::new(x, c.y - h / 2.0, w, h), w / 3.0, &Brush::Solid(color));
            }
        }
        SpinnerKind::Audio => {
            // Barras do fundo, como um equalizador.
            let n = 4;
            let w = side / (n as f32 * 1.8);
            for i in 0..n {
                let phase = ((t * (1.1 + i as f32 * 0.37)) * tau).sin() * 0.5 + 0.5;
                let h = side * (0.25 + 0.7 * phase);
                let x = c.x - side / 2.0 + i as f32 * (side / n as f32) + (side / n as f32 - w) / 2.0;
                canvas.fill_rect(RectF::new(x, c.y + side / 2.0 - h, w, h), w / 3.0, &Brush::Solid(color));
            }
        }
        SpinnerKind::Ball => {
            // A bola que quica, com a sombra.
            let bounce = ((t * 1.8 * tau).sin()).abs();
            let y = c.y + r * 0.45 - bounce * r * 0.9;
            dot(canvas, PointF::new(c.x, c.y + r * 0.85), r * (0.3 - 0.12 * bounce), 0.18);
            dot(canvas, PointF::new(c.x, y), r * 0.28, 1.0);
        }
        SpinnerKind::Oval => {
            let width = (side / 10.0).max(2.0);
            let rr = r - width / 2.0;
            canvas.stroke_ellipse(RectF::new(c.x - rr, c.y - rr, rr * 2.0, rr * 2.0), &Stroke::new(color.with_opacity(0.25), width));
            let mut arc = Path::new();
            arc.arc(c, rr, rr, (t * 300.0) % 360.0, 90.0);
            canvas.stroke_path(&arc, &Stroke::new(color, width).round());
        }
        SpinnerKind::Puff => {
            // Anéis que nascem no centro e se apagam ao crescer.
            for i in 0..2 {
                let phase = (t * 0.8 + i as f32 * 0.5).fract();
                let rr = r * phase;
                canvas.stroke_ellipse(RectF::new(c.x - rr, c.y - rr, rr * 2.0, rr * 2.0), &Stroke::new(color.with_opacity(1.0 - phase), (side / 14.0).max(1.5)));
            }
        }
        SpinnerKind::Grid => {
            // Nove quadradinhos que piscam em diagonal.
            let cell = side / 3.0;
            for row in 0..3 {
                for col in 0..3 {
                    let phase = ((t * 1.4 - (row + col) as f32 * 0.12) * tau).sin() * 0.5 + 0.5;
                    let x = c.x - side / 2.0 + col as f32 * cell;
                    let y = c.y - side / 2.0 + row as f32 * cell;
                    let rr = cell * 0.32;
                    dot(canvas, PointF::new(x + cell / 2.0, y + cell / 2.0), rr, 0.25 + 0.75 * phase);
                }
            }
        }
        SpinnerKind::Ios => {
            // Os doze raios do iOS, um apagando atrás do outro.
            let head = ((t * 12.0) as usize) % 12;
            for i in 0..12 {
                let a = i as f32 / 12.0 * tau;
                let age = (head + 12 - i) % 12;
                let from = PointF::new(c.x + a.sin() * r * 0.45, c.y - a.cos() * r * 0.45);
                let to = PointF::new(c.x + a.sin() * r * 0.92, c.y - a.cos() * r * 0.92);
                canvas.draw_line(from, to, &Stroke::new(color.with_opacity(1.0 - age as f32 / 13.0), (side / 12.0).max(1.5)).round());
            }
        }
        SpinnerKind::Pie => {
            canvas.fill_ellipse(RectF::new(c.x - r, c.y - r, r * 2.0, r * 2.0), &Brush::Solid(color.with_opacity(0.2)));
            let mut slice = Path::new();
            slice.move_to(c.x, c.y);
            slice.arc(c, r, r, (t * 360.0) % 360.0 - 90.0, 90.0);
            slice.close();
            canvas.fill_path(&slice, &Brush::Solid(color));
        }
        SpinnerKind::Tail => {
            // O arco com rastro: segmentos cada vez mais claros atrás da ponta.
            let width = (side / 9.0).max(2.0);
            let rr = r - width / 2.0;
            let head = (t * 360.0) % 360.0;
            for i in 0..12 {
                let mut arc = Path::new();
                arc.arc(c, rr, rr, head - i as f32 * 18.0, 18.0);
                canvas.stroke_path(&arc, &Stroke::new(color.with_opacity(1.0 - i as f32 / 12.0), width));
            }
        }
        SpinnerKind::Hourglass => {
            // A ampulheta que vira a cada ciclo.
            let phase = (t * 0.6).fract();
            canvas.save();
            canvas.translate(c.x, c.y);
            canvas.rotate(if phase > 0.8 { (phase - 0.8) / 0.2 * 180.0 } else { 0.0 });
            let w = r * 0.8;
            let h = r * 0.95;
            let mut glass = Path::new();
            glass.polygon(&[PointF::new(-w, -h), PointF::new(w, -h), PointF::new(0.0, 0.0), PointF::new(w, h), PointF::new(-w, h), PointF::new(0.0, 0.0)], true);
            canvas.stroke_path(&glass, &Stroke::new(color, (side / 16.0).max(1.5)));
            let sand = (phase / 0.8).min(1.0);
            let mut top = Path::new();
            let tw = w * (1.0 - sand);
            top.polygon(&[PointF::new(-tw, -h * (1.0 - sand)), PointF::new(tw, -h * (1.0 - sand)), PointF::new(0.0, 0.0)], true);
            canvas.fill_path(&top, &Brush::Solid(color));
            let mut bottom = Path::new();
            let bw = w * sand;
            bottom.polygon(&[PointF::new(-bw, h), PointF::new(bw, h), PointF::new(0.0, h * (1.0 - sand))], true);
            canvas.fill_path(&bottom, &Brush::Solid(color));
            canvas.restore();
        }
        SpinnerKind::Rings => {
            // Dois arcos girando em sentidos contrários.
            let width = (side / 12.0).max(1.5);
            for (i, (rr, speed)) in [(r - width, 300.0), (r * 0.55, -420.0)].into_iter().enumerate() {
                let mut arc = Path::new();
                arc.arc(c, rr, rr, (t * speed) % 360.0, 220.0 - i as f32 * 40.0);
                canvas.stroke_path(&arc, &Stroke::new(color, width).round());
            }
        }
        SpinnerKind::Orbit => {
            // Uma bolinha grande no centro e duas em órbita.
            dot(canvas, c, r * 0.2, 1.0);
            canvas.stroke_ellipse(RectF::new(c.x - r * 0.75, c.y - r * 0.75, r * 1.5, r * 1.5), &Stroke::new(color.with_opacity(0.2), 1.0));
            for (i, speed) in [1.0f32, 1.6].into_iter().enumerate() {
                let a = t * speed * tau + i as f32 * 2.0;
                let rr = r * (0.75 - i as f32 * 0.25);
                dot(canvas, PointF::new(c.x + a.cos() * rr, c.y + a.sin() * rr), r * 0.12, 0.9);
            }
        }
    }
}

// ===========================================================================
// Card
// ===========================================================================

/// O cartão (`QCard`): um painel com a cara de cartão do tema (fundo, cantos e sombra), que
/// recebe outros componentes. As variantes `flat` (sem sombra) e `bordered` (com borda).
pub struct Card {
    pub base: Panel,
}

impl Default for Card {
    fn default() -> Self {
        let mut base = Panel::new(300.0, 200.0);
        if let Some(cb) = base.control_mut() {
            cb.set_padding(16.0);
        }
        Self { base }
    }
}

impl Component for Card {
    fn class_name(&self) -> &'static str {
        "Card"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }
}

// ===========================================================================
// Knob
// ===========================================================================

/// O botão giratório (`QKnob`): o valor de `min` a `max` num círculo, que se arrasta com o
/// mouse (ou o dedo), a roda e as setas. `thickness` é a grossura do arco (de 0 a 1 do
/// raio); `show_value` escreve o valor no meio, com `suffix` (`%`, `°C`).
pub struct Knob {
    pub base: ControlBase,
    /// O valor escolhido, entre `min` e `max`.
    pub value: f32,
    /// O valor do começo do arco.
    pub min: f32,
    /// O valor do fim do arco.
    pub max: f32,
    /// O passo do valor ao arrastar, na roda e nas setas (0: de 1 em 1).
    pub step: f32,
    /// A grossura do arco, de 0 a 1 do raio.
    pub thickness: f32,
    /// Escreve o valor no meio do círculo.
    pub show_value: bool,
    /// O texto depois do valor escrito no meio (`%`, `°C`).
    pub suffix: String,
    /// Só mostra: o mouse, a roda e as setas não mudam o valor.
    pub read_only: bool,
    dragging: Cell<bool>,
}

impl Default for Knob {
    fn default() -> Self {
        let mut base = ControlBase::new(96.0, 96.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, value: 40.0, min: 0.0, max: 100.0, step: 1.0, thickness: 0.22, show_value: true, suffix: String::new(), read_only: false, dragging: Cell::new(false) }
    }
}

impl Knob {
    fn face(&self) -> (PointF, f32) {
        let b = self.base.bounds();
        (PointF::new(b.width / 2.0, b.height / 2.0), (b.width.min(b.height) / 2.0 - 2.0).max(4.0))
    }

    fn fraction(&self) -> f32 {
        if self.max > self.min { ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0) } else { 0.0 }
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, value: f32) {
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        let v = (((value - self.min) / step).round() * step + self.min).clamp(self.min.min(self.max), self.max.max(self.min));
        if v != self.value {
            self.value = v;
            ctx.fire("on_change", EventArgs::Value(PropValue::Float(v as f64)));
        }
        ctx.repaint();
    }

    fn drag_to(&mut self, ctx: &mut EventCtx<'_>, p: PointF) {
        let (c, _) = self.face();
        // 0 em cima, crescendo no sentido horário, como o QKnob.
        let mut a = (p.x - c.x).atan2(-(p.y - c.y)).to_degrees();
        if a < 0.0 {
            a += 360.0;
        }
        self.set(ctx, self.min + a / 360.0 * (self.max - self.min));
    }
}

impl Component for Knob {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("Knob");

    properties! {
        value: f32 => "Data",
        min: f32 => "Data",
        max: f32 => "Data",
        step: f32 => "Data",
        thickness: f32 => "Appearance",
        show_value: bool => "Appearance",
        suffix: String => "Appearance",
        read_only: bool => "Behavior";
        events: on_change
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.read_only || event.button != PointerButton::Primary {
            return;
        }
        self.dragging.set(true);
        self.drag_to(ctx, event.position);
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.dragging.get() {
            self.drag_to(ctx, event.position);
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging.set(false);
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        if self.read_only {
            return;
        }
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        self.set(ctx, self.value - event.delta.y.signum() * step);
        ctx.set_handled();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if self.read_only {
            return;
        }
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        let v = match event.key {
            Key::Right | Key::Up => self.value + step,
            Key::Left | Key::Down => self.value - step,
            _ => return,
        };
        self.set(ctx, v);
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (c, r) = self.face();
        let width = (r * self.thickness.clamp(0.05, 1.0)).max(2.0);
        let rr = r - width / 2.0;
        let color = accent(ctx);
        canvas.stroke_ellipse(RectF::new(c.x - rr, c.y - rr, rr * 2.0, rr * 2.0), &Stroke::new(token(ctx, "track", "border"), width));
        let sweep = self.fraction() * 360.0;
        if sweep > 0.0 {
            let mut arc = Path::new();
            arc.arc(c, rr, rr, -90.0, sweep);
            canvas.stroke_path(&arc, &Stroke::new(color, width));
        }
        if self.show_value && r >= 18.0 {
            let shown = if self.step.fract() == 0.0 { format!("{:.0}{}", self.value, self.suffix) } else { format!("{:.1}{}", self.value, self.suffix) };
            text(canvas, ctx, &shown, RectF::new(c.x - r, c.y - r * 0.4, r * 2.0, r * 0.8), (r * 0.36).clamp(10.0, 28.0), ctx.colors.text.unwrap_or(color), TextAlign::Center, true);
        }
        if self.base.state().focused {
            canvas.stroke_ellipse(RectF::new(c.x - r - 2.0, c.y - r - 2.0, r * 2.0 + 4.0, r * 2.0 + 4.0), &Stroke::new(color.with_opacity(0.4), 2.0));
        }
    }
}

// ===========================================================================
// InnerLoading
// ===========================================================================

/// O carregando por cima (`QInnerLoading`): cobre o contêiner onde está (ocupa a área
/// toda) com um véu e um spinner enquanto `showing`; bloqueia os cliques embaixo. Escondido,
/// não aparece nem atrapalha.
pub struct InnerLoading {
    pub base: ControlBase,
    /// Mostra o véu e o spinner e bloqueia os cliques; desligado, some e deixa os cliques passarem.
    pub showing: bool,
    /// O desenho do spinner (anel, pontos, barras...).
    pub kind: SpinnerKind,
    /// O texto embaixo do spinner (`Carregando...`); vazio: só o spinner.
    pub label: String,
    /// O tamanho do spinner.
    pub size: f32,
    /// A opacidade do véu (0 = só o spinner).
    pub dim: f32,
}

impl Default for InnerLoading {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 120.0);
        base.set_align(aylis_core::Align::Client);
        Self { base, showing: true, kind: SpinnerKind::Ring, label: String::new(), size: 42.0, dim: 0.6 }
    }
}

impl Component for InnerLoading {
    leaf_control!("InnerLoading");

    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        showing: bool => "Behavior",
        kind: SpinnerKind => "Appearance",
        label: String => "Appearance",
        size: f32 => "Appearance",
        dim: f32 => "Appearance",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "showing" {
            // Escondido, os cliques passam para o que está embaixo.
            self.base.set_hit_test(self.showing);
            ctx.repaint();
        }
    }

    fn contains_point(&self, _local: PointF, _size: aylis_core::SizeF) -> bool {
        self.showing
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !self.showing {
            if ctx.design {
                canvas.stroke_rect(ctx.rect.deflate(aylis_core::Bounds::all(1.0)), 4.0, &Stroke::new(ctx.theme.color("border"), 1.0));
            }
            return;
        }
        let veil = ctx.colors.fill.unwrap_or_else(|| ctx.theme.color("surface"));
        canvas.fill_rect(ctx.rect, ctx.style.radius, &Brush::Solid(veil.with_opacity(self.dim.clamp(0.0, 1.0))));
        let side = self.size.min(ctx.rect.width).min(ctx.rect.height).max(8.0);
        let label_h = if self.label.is_empty() { 0.0 } else { 24.0 };
        let top = ctx.rect.y + (ctx.rect.height - side - label_h) / 2.0;
        let spot = RectF::new(ctx.rect.x + (ctx.rect.width - side) / 2.0, top, side, side);
        paint_spinner(canvas, spot, self.kind, accent(ctx), ctx.time as f32);
        if !self.label.is_empty() {
            text(canvas, ctx, &self.label, RectF::new(ctx.rect.x, spot.bottom() + 4.0, ctx.rect.width, 20.0), 13.0, ctx.theme.color("text_muted"), TextAlign::Center, false);
        }
        ctx.request_frame();
    }
}

// ===========================================================================
// MarkupTable
// ===========================================================================

/// As linhas de separação da tabela (o `separator` do `QMarkupTable`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableSeparator {
    #[default]
    Horizontal,
    Vertical,
    Cell,
    None,
}

impl PropEnum for TableSeparator {
    const NAMES: &'static [&'static str] = &["horizontal", "vertical", "cell", "none"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Horizontal, Self::Vertical, Self::Cell, Self::None].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A tabela simples (`QMarkupTable`): `columns` com os títulos (`Nome;Idade;Cidade`) e
/// `rows` com uma linha por linha de texto (as células separadas por `;`). Os números
/// ficam à direita. `dense` aperta as linhas, `striped` alterna o fundo e `separator` escolhe
/// as linhas de separação. Para dados de banco, use o `DataGrid`.
pub struct MarkupTable {
    pub base: ControlBase,
    /// Os títulos das colunas, separados por `;` (`Nome;Idade;Cidade`).
    pub columns: String,
    /// As linhas da tabela, uma por linha de texto, com as células separadas por `;`.
    pub rows: String,
    /// Linhas mais baixas (28 pontos em vez de 44).
    pub dense: bool,
    /// Alterna o fundo das linhas (zebrado).
    pub striped: bool,
    /// As linhas de separação: horizontais, verticais, em volta de cada célula ou nenhuma.
    pub separator: TableSeparator,
}

impl Default for MarkupTable {
    fn default() -> Self {
        Self {
            base: ControlBase::new(420.0, 200.0),
            columns: "Produto;Qtde;Preço".into(),
            rows: "Café;2;12,50\nPão de queijo;6;18,00\nSuco;1;8,90".into(),
            dense: false,
            striped: false,
            separator: TableSeparator::Horizontal,
        }
    }
}

impl MarkupTable {
    fn cells(&self) -> Vec<Vec<String>> {
        self.rows.lines().filter(|l| !l.trim().is_empty()).map(|l| l.split(';').map(|c| c.trim().to_string()).collect()).collect()
    }
}

fn numeric(s: &str) -> bool {
    let t = s.trim().trim_start_matches(['R', '$', ' ', '-', '+']).trim_end_matches('%');
    !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || c == ',' || c == '.')
}

impl Component for MarkupTable {
    leaf_control!("MarkupTable");

    properties! {
        columns: String => "Data",
        rows: String => "Data",
        dense: bool => "Appearance",
        striped: bool => "Appearance",
        separator: TableSeparator => "Appearance",
    }

    fn auto_height(&self, _ui: &aylis_core::Ui, _id: ComponentId, _width: f32) -> Option<f32> {
        let row = if self.dense { 28.0 } else { 44.0 };
        Some(row * (self.cells().len() as f32 + 1.0) + 2.0)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let radius = ctx.style.radius.max(4.0);
        canvas.fill_rect(r, radius, &Brush::Solid(ctx.colors.fill.unwrap_or_else(|| ctx.theme.color("surface"))));
        canvas.stroke_rect(r, radius, &Stroke::new(ctx.theme.color("border"), 1.0));
        let head: Vec<&str> = split(&self.columns);
        let rows = self.cells();
        let n = head.len().max(rows.iter().map(Vec::len).max().unwrap_or(0)).max(1);
        let row_h = if self.dense { 28.0 } else { 44.0 };
        let col_w = r.width / n as f32;
        let line = Stroke::new(ctx.theme.color("border"), 1.0);
        let pad = if self.dense { 8.0 } else { 16.0 };
        canvas.save();
        canvas.clip_round_rect(r, radius);
        // O cabeçalho: menor e discreto, como no Quasar.
        for (i, h) in head.iter().enumerate() {
            let right = rows.iter().all(|row| row.get(i).is_none_or(|c| numeric(c))) && !rows.is_empty();
            let cell = RectF::new(r.x + i as f32 * col_w + pad, r.y, col_w - pad * 2.0, row_h);
            text(canvas, ctx, h, cell, 12.0, ctx.theme.color("text_muted"), if right { TextAlign::Trailing } else { TextAlign::Leading }, true);
        }
        let mut y = r.y + row_h;
        canvas.draw_line(PointF::new(r.x, y), PointF::new(r.right(), y), &line);
        for (k, row) in rows.iter().enumerate() {
            if y >= r.bottom() {
                break;
            }
            if self.striped && k % 2 == 1 {
                canvas.fill_rect(RectF::new(r.x, y, r.width, row_h), 0.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
            }
            for i in 0..n {
                let value = row.get(i).map(String::as_str).unwrap_or("");
                let cell = RectF::new(r.x + i as f32 * col_w + pad, y, col_w - pad * 2.0, row_h);
                text(canvas, ctx, value, cell, 14.0, ctx.theme.color("text"), if numeric(value) { TextAlign::Trailing } else { TextAlign::Leading }, false);
            }
            y += row_h;
            if matches!(self.separator, TableSeparator::Horizontal | TableSeparator::Cell) && k + 1 < rows.len() {
                canvas.draw_line(PointF::new(r.x, y), PointF::new(r.right(), y), &line);
            }
        }
        if matches!(self.separator, TableSeparator::Vertical | TableSeparator::Cell) {
            for i in 1..n {
                let x = r.x + i as f32 * col_w;
                canvas.draw_line(PointF::new(x, r.y), PointF::new(x, y.min(r.bottom())), &line);
            }
        }
        canvas.restore();
    }
}

// ===========================================================================
// SlideItem
// ===========================================================================

/// O item que desliza (`QSlideItem`): uma linha de lista (`text`, `detail`, `icon`) que se
/// arrasta para os lados e mostra a ação de baixo (`left_text`/`left_icon` na cor
/// `left_color`, ao puxar para a direita; `right_*` ao puxar para a esquerda). Soltando
/// depois de um terço, dispara `on_left` ou `on_right` e volta.
pub struct SlideItem {
    pub base: ControlBase,
    /// O texto principal da linha.
    pub text: String,
    /// A segunda linha, menor e apagada; vazio: só o `text`, centrado.
    pub detail: String,
    /// O ícone à esquerda da linha (vazio: sem ícone).
    pub icon: String,
    /// O texto da ação que aparece ao puxar para a direita.
    pub left_text: String,
    /// O ícone da ação da esquerda; com ele e o `left_text` vazios, não puxa para a direita.
    pub left_icon: String,
    /// O fundo da ação da esquerda; vazio: a cor de sucesso do tema.
    pub left_color: Option<Color>,
    /// O texto da ação que aparece ao puxar para a esquerda.
    pub right_text: String,
    /// O ícone da ação da direita; com ele e o `right_text` vazios, não puxa para a esquerda.
    pub right_icon: String,
    /// O fundo da ação da direita; vazio: a cor de perigo do tema.
    pub right_color: Option<Color>,
    /// Onde o arrasto começou (x) e o quanto andou.
    drag: Cell<Option<f32>>,
    offset: Cell<f32>,
    /// Voltando ao lugar: de onde e quando começou.
    back: Cell<Option<(f32, f64)>>,
}

impl Default for SlideItem {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 56.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            text: "Mensagem".into(),
            detail: "Arraste para os lados".into(),
            icon: String::new(),
            left_text: "Arquivar".into(),
            left_icon: "check".into(),
            left_color: None,
            right_text: "Excluir".into(),
            right_icon: "trash".into(),
            right_color: None,
            drag: Cell::new(None),
            offset: Cell::new(0.0),
            back: Cell::new(None),
        }
    }
}

impl Component for SlideItem {
    leaf_control!("SlideItem");

    properties! {
        text: String => "Appearance",
        detail: String => "Appearance",
        icon: String => "Appearance",
        left_text: String => "Appearance",
        left_icon: String => "Appearance",
        left_color: Option<Color> => "Appearance",
        right_text: String => "Appearance",
        right_icon: String => "Appearance",
        right_color: Option<Color> => "Appearance";
        events: on_left, on_right, on_click
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.drag.set(Some(event.position.x - self.offset.get()));
            self.back.set(None);
            ctx.repaint();
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(start) = self.drag.get() {
            let w = self.base.bounds().width;
            let mut dx = event.position.x - start;
            // Sem ação de um lado, não deixa puxar para ele.
            if self.left_text.is_empty() && self.left_icon.is_empty() {
                dx = dx.min(0.0);
            }
            if self.right_text.is_empty() && self.right_icon.is_empty() {
                dx = dx.max(0.0);
            }
            self.offset.set(dx.clamp(-w, w));
            ctx.repaint();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        if self.drag.take().is_none() {
            return;
        }
        let w = self.base.bounds().width.max(1.0);
        let dx = self.offset.get();
        if dx.abs() < 4.0 {
            self.offset.set(0.0);
            ctx.fire("on_click", EventArgs::None);
        } else if dx > w / 3.0 {
            ctx.fire("on_left", EventArgs::None);
        } else if dx < -w / 3.0 {
            ctx.fire("on_right", EventArgs::None);
        }
        // Volta ao lugar, animado.
        self.back.set(Some((self.offset.get(), -1.0)));
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // A volta: 180 ms, desacelerando.
        if let Some((from, started)) = self.back.get() {
            let started = if started < 0.0 { ctx.time } else { started };
            let k = (((ctx.time - started) / 0.18) as f32).clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - k).powi(3);
            self.offset.set(from * (1.0 - eased));
            self.back.set(if k >= 1.0 { None } else { Some((from, started)) });
            ctx.request_frame();
        }
        let dx = self.offset.get();
        // A ação de baixo.
        if dx != 0.0 {
            let (label, icon, color, left) = if dx > 0.0 {
                (&self.left_text, &self.left_icon, self.left_color.unwrap_or_else(|| token(ctx, "positive", "success")), true)
            } else {
                (&self.right_text, &self.right_icon, self.right_color.unwrap_or_else(|| token(ctx, "negative", "danger")), false)
            };
            canvas.fill_rect(r, 0.0, &Brush::Solid(color));
            let white = Color::rgb(0xFF, 0xFF, 0xFF);
            let x = if left { r.x + 16.0 } else { r.right() - 16.0 - 20.0 };
            if !icon.is_empty() {
                ctx.draw_icon(canvas, icon, RectF::new(x, r.y + (r.height - 20.0) / 2.0, 20.0, 20.0), white);
            }
            let tx = if left { RectF::new(x + 28.0, r.y, r.width / 2.0, r.height) } else { RectF::new(r.right() - 44.0 - r.width / 2.0, r.y, r.width / 2.0, r.height) };
            text(canvas, ctx, label, tx, 14.0, white, if left { TextAlign::Leading } else { TextAlign::Trailing }, true);
        }
        // A linha, deslocada.
        let row = RectF::new(r.x + dx, r.y, r.width, r.height);
        canvas.fill_rect(row, 0.0, &Brush::Solid(ctx.colors.fill.unwrap_or_else(|| ctx.theme.color("surface"))));
        let mut x = row.x + 16.0;
        if !self.icon.is_empty() {
            ctx.draw_icon(canvas, &self.icon, RectF::new(x, row.y + (row.height - 22.0) / 2.0, 22.0, 22.0), ctx.theme.color("text_muted"));
            x += 38.0;
        }
        let w = (row.right() - x - 16.0).max(0.0);
        if self.detail.is_empty() {
            text(canvas, ctx, &self.text, RectF::new(x, row.y, w, row.height), 14.0, ctx.theme.color("text"), TextAlign::Leading, false);
        } else {
            text(canvas, ctx, &self.text, RectF::new(x, row.y + row.height * 0.14, w, row.height * 0.4), 14.0, ctx.theme.color("text"), TextAlign::Leading, false);
            text(canvas, ctx, &self.detail, RectF::new(x, row.y + row.height * 0.5, w, row.height * 0.36), 12.0, ctx.theme.color("text_muted"), TextAlign::Leading, false);
        }
        canvas.draw_line(PointF::new(r.x, r.bottom() - 0.5), PointF::new(r.right(), r.bottom() - 0.5), &Stroke::new(ctx.theme.color("border"), 1.0));
    }
}

// ===========================================================================
// Parallax
// ===========================================================================

/// O fundo com paralaxe (`QParallax`): a imagem (`image`, um arquivo do app) anda mais
/// devagar que a página quando a tela rola (`speed` de 0 a 1), atrás dos componentes
/// colocados dentro dele (um título, um botão).
pub struct Parallax {
    pub base: ControlBase,
    /// A imagem do fundo (um arquivo do app); vazio: um degradê da cor de destaque.
    pub image: String,
    /// Quanto a imagem se desloca com a rolagem, de 0 (parada com o componente) a 1.
    pub speed: f32,
    loaded: RefCell<Option<(String, Option<Image>)>>,
}

impl Default for Parallax {
    fn default() -> Self {
        Self { base: ControlBase::new(480.0, 260.0), image: String::new(), speed: 0.5, loaded: RefCell::new(None) }
    }
}

impl Component for Parallax {
    leaf_control!("Parallax");

    properties! {
        image: String => "Appearance",
        speed: f32 => "Behavior",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let image = {
            let mut loaded = self.loaded.borrow_mut();
            if loaded.as_ref().is_none_or(|(p, _)| *p != self.image) {
                let image = if self.image.is_empty() { None } else { ctx.resource(&self.image).map(Image::encoded) };
                *loaded = Some((self.image.clone(), image));
            }
            loaded.as_ref().and_then(|(_, i)| i.clone())
        };
        canvas.save();
        canvas.clip_rect(r);
        // Onde o componente está na janela: no meio dela, a imagem fica centrada; acima ou
        // abaixo, ela se desloca na proporção da `speed`.
        let window = ctx.ui.absolute_rect(ctx.ui.root_of(ctx.id)).map_or(r.height, |w| w.height);
        let at = ctx.ui.absolute_rect(ctx.id).map_or(0.0, |a| a.y + a.height / 2.0);
        let shift = (at - window / 2.0) * self.speed.clamp(0.0, 1.0);
        let tall = r.height + window * self.speed.clamp(0.0, 1.0);
        match image.as_ref().and_then(|img| canvas.image_size(img).map(|s| (img, s))) {
            Some((img, size)) if size.width > 0.0 && size.height > 0.0 => {
                // Cobre a largura e a altura com folga para o deslocamento.
                let scale = (r.width / size.width).max(tall / size.height);
                let (w, h) = (size.width * scale, size.height * scale);
                let dest = RectF::new(r.x + (r.width - w) / 2.0, r.y + (r.height - h) / 2.0 - shift, w, h);
                canvas.draw_image(img, RectF::new(0.0, 0.0, size.width, size.height), dest, 1.0);
            }
            _ => {
                // Sem imagem: um degradê da cor primária, para ver o efeito no designer.
                let base = accent(ctx);
                let bands = 8;
                for i in 0..bands {
                    let y = r.y + i as f32 * r.height / bands as f32 - shift * 0.3;
                    canvas.fill_rect(RectF::new(r.x, y, r.width, r.height / bands as f32 + 1.0), 0.0, &Brush::Solid(base.with_opacity(0.35 + 0.08 * i as f32)));
                }
            }
        }
        canvas.restore();
    }
}

// ===========================================================================
// Uploader
// ===========================================================================

/// O envio de arquivos (`QUploader`): o cabeçalho com o título, o "+" (escolher, ou soltar
/// arquivos em cima) e o "enviar"; embaixo, os arquivos com o tamanho e a barra de
/// progresso. O envio é do app: `on_upload` traz os arquivos (`;`) e o app vai pondo o
/// andamento em `progress` (uma fração de 0 a 1 por arquivo, `0.5;1`).
pub struct Uploader {
    pub base: ControlBase,
    /// O título do cabeçalho.
    pub title: String,
    /// Os caminhos dos arquivos escolhidos, separados por `;` (vazio: nenhum).
    pub files: String,
    /// O andamento de cada arquivo, de 0 a 1, separados por `;` na ordem dos `files` (`0.5;1`).
    pub progress: String,
    /// Os tipos aceitos no "+", como no diálogo de abrir (`Imagens|*.png;*.jpg`); vazio: todos.
    pub filter: String,
    /// Deixa escolher vários arquivos de uma vez.
    pub multiple: bool,
    /// Envia assim que os arquivos são escolhidos.
    pub auto_upload: bool,
    zones: RefCell<UploaderZones>,
}

#[derive(Default)]
struct UploaderZones {
    add: RectF,
    send: RectF,
    clear: RectF,
    remove: Vec<RectF>,
}

impl Default for Uploader {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 220.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            title: "Enviar arquivos".into(),
            files: String::new(),
            progress: String::new(),
            filter: String::new(),
            multiple: true,
            auto_upload: false,
            zones: RefCell::new(UploaderZones::default()),
        }
    }
}

fn file_size(path: &str) -> String {
    match std::fs::metadata(path).map(|m| m.len()) {
        Ok(b) if b >= 1024 * 1024 => format!("{:.1} MB", b as f64 / 1048576.0),
        Ok(b) if b >= 1024 => format!("{:.1} KB", b as f64 / 1024.0),
        Ok(b) => format!("{b} B"),
        Err(_) => String::new(),
    }
}

impl Uploader {
    fn send(&self, ctx: &mut EventCtx<'_>) {
        if !self.files.is_empty() {
            ctx.fire("on_upload", EventArgs::Text(self.files.clone()));
        }
    }
}

impl Component for Uploader {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.files.clone()))
    }

    leaf_control!("Uploader");

    properties! {
        title: String => "Appearance",
        files: String => "Data",
        progress: String => "Data",
        filter: String => "Behavior",
        multiple: bool => "Behavior",
        auto_upload: bool => "Behavior";
        events: on_change, on_upload
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let p = event.position;
        let (add, send, clear, remove) = {
            let z = self.zones.borrow();
            (z.add, z.send, z.clear, z.remove.iter().position(|r| r.inflate(4.0).contains(p)))
        };
        if add.inflate(4.0).contains(p) {
            let (id, filter, multiple) = (ctx.id, self.filter.clone(), self.multiple);
            ctx.ui.poster().post(move |ui| crate::files::choose(ui, id, &filter, multiple, multiple));
        } else if send.inflate(4.0).contains(p) {
            self.send(ctx);
        } else if clear.inflate(4.0).contains(p) {
            self.files.clear();
            self.progress.clear();
            ctx.fire("on_change", EventArgs::Text(String::new()));
        } else if let Some(i) = remove {
            let mut all: Vec<&str> = split(&self.files);
            if i < all.len() {
                all.remove(i);
                let mut progress: Vec<&str> = split(&self.progress);
                if i < progress.len() {
                    progress.remove(i);
                }
                self.progress = progress.join(";");
                self.files = all.join(";");
                ctx.fire("on_change", EventArgs::Text(self.files.clone()));
            }
        }
        ctx.repaint();
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // Escolhidos pelo diálogo (o `choose` grava `files`): com `auto_upload`, já envia.
        if name == "files" && self.auto_upload && !ctx.ui.design_mode() {
            self.send(ctx);
        }
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let radius = ctx.style.radius.max(4.0);
        let color = accent(ctx);
        let on = token(ctx, "on_primary", "background");
        canvas.fill_rect(r, radius, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(r, radius, &Stroke::new(ctx.theme.color("border"), 1.0));
        // O cabeçalho na cor primária.
        let head = RectF::new(r.x, r.y, r.width, 56.0_f32.min(r.height));
        canvas.save();
        canvas.clip_round_rect(r, radius);
        canvas.fill_rect(head, 0.0, &Brush::Solid(color));
        canvas.restore();
        let files: Vec<&str> = split(&self.files);
        let progress: Vec<f32> = split(&self.progress).iter().map(|p| p.trim().replace(',', ".").parse::<f32>().unwrap_or(0.0)).collect();
        let done = files.iter().enumerate().filter(|(i, _)| progress.get(*i).is_some_and(|p| *p >= 1.0)).count();
        let total: u64 = files.iter().filter_map(|f| std::fs::metadata(f).ok().map(|m| m.len())).sum();
        text(canvas, ctx, &self.title, RectF::new(head.x + 14.0, head.y + 6.0, head.width - 140.0, 24.0), 14.0, on, TextAlign::Leading, true);
        let summary = if files.is_empty() { "Nenhum arquivo".to_string() } else { format!("{done}/{} · {}", files.len(), file_size_bytes(total)) };
        text(canvas, ctx, &summary, RectF::new(head.x + 14.0, head.y + 28.0, head.width - 140.0, 20.0), 12.0, on.with_opacity(0.8), TextAlign::Leading, false);
        let button = |i: usize| RectF::new(head.right() - 40.0 * (i as f32 + 1.0), head.y + (head.height - 32.0) / 2.0, 32.0, 32.0);
        let (clear, send, add) = (button(2), button(1), button(0));
        for (zone, icon, visible) in [(clear, "close", !files.is_empty()), (send, "upload", !files.is_empty()), (add, "plus", true)] {
            if visible {
                ctx.draw_icon(canvas, icon, zone.deflate(aylis_core::Bounds::all(6.0)), on);
            }
        }
        // Os arquivos.
        let mut remove = Vec::new();
        let mut y = head.bottom() + 6.0;
        if files.is_empty() {
            text(canvas, ctx, "Clique no + ou solte os arquivos aqui", RectF::new(r.x, y, r.width, (r.bottom() - y).max(0.0)), 13.0, ctx.theme.color("text_muted"), TextAlign::Center, false);
        }
        for (i, f) in files.iter().enumerate() {
            if y + 44.0 > r.bottom() {
                break;
            }
            let name = f.rsplit(['/', '\\']).next().unwrap_or(f);
            let p = progress.get(i).copied().unwrap_or(0.0).clamp(0.0, 1.0);
            ctx.draw_icon(canvas, if p >= 1.0 { "check" } else { "file" }, RectF::new(r.x + 12.0, y + 10.0, 20.0, 20.0), if p >= 1.0 { token(ctx, "positive", "success") } else { ctx.theme.color("text_muted") });
            let w = r.width - 44.0 - 44.0;
            text(canvas, ctx, name, RectF::new(r.x + 44.0, y + 2.0, w, 20.0), 13.0, ctx.theme.color("text"), TextAlign::Leading, false);
            let detail = format!("{}{}", file_size(f), if p > 0.0 && p < 1.0 { format!(" · {:.0}%", p * 100.0) } else { String::new() });
            text(canvas, ctx, &detail, RectF::new(r.x + 44.0, y + 20.0, w, 16.0), 11.0, ctx.theme.color("text_muted"), TextAlign::Leading, false);
            let bar = RectF::new(r.x + 44.0, y + 38.0, w, 3.0);
            canvas.fill_rect(bar, 1.5, &Brush::Solid(token(ctx, "track", "border")));
            if p > 0.0 {
                canvas.fill_rect(RectF::new(bar.x, bar.y, bar.width * p, bar.height), 1.5, &Brush::Solid(color));
            }
            let x = RectF::new(r.right() - 34.0, y + 10.0, 20.0, 20.0);
            ctx.draw_icon(canvas, "close", x, ctx.theme.color("text_muted"));
            remove.push(x);
            y += 48.0;
        }
        *self.zones.borrow_mut() = UploaderZones { add, send: if files.is_empty() { RectF::default() } else { send }, clear: if files.is_empty() { RectF::default() } else { clear }, remove };
    }
}

fn file_size_bytes(b: u64) -> String {
    if b >= 1024 * 1024 {
        format!("{:.1} MB", b as f64 / 1048576.0)
    } else if b >= 1024 {
        format!("{:.1} KB", b as f64 / 1024.0)
    } else {
        format!("{b} B")
    }
}

// ===========================================================================
// PopupEdit
// ===========================================================================

/// A edição no lugar (`QPopupEdit`): mostra o `text` (como um rótulo com um lápis) e, ao
/// clicar (ou Enter), vira um campo; Enter (ou sair do campo) grava e dispara `on_save`
/// com o texto novo; Esc desiste. Bom para tabelas e fichas editáveis.
pub struct PopupEdit {
    pub base: ControlBase,
    /// O texto mostrado e editado; gravado ao confirmar a edição.
    pub text: String,
    /// O texto apagado que aparece com o `text` vazio.
    pub placeholder: String,
    editor: Cell<Option<ComponentId>>,
}

impl Default for PopupEdit {
    fn default() -> Self {
        let mut base = ControlBase::new(200.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, text: "Clique para editar".into(), placeholder: "Vazio".into(), editor: Cell::new(None) }
    }
}

impl PopupEdit {
    fn open(&self, ctx: &mut EventCtx<'_>) {
        if self.editor.get().is_some_and(|e| ctx.ui.contains(e)) || ctx.ui.design_mode() {
            return;
        }
        let me = ctx.id;
        let Ok(edit) = ctx.ui.create("Edit", Some(me)) else { return };
        if let Some(cb) = ctx.ui.control_mut(edit) {
            cb.set_align(aylis_core::Align::Client);
        }
        let _ = ctx.ui.set_prop(edit, "text", self.text.clone());
        self.editor.set(Some(edit));
        // Grava (ou desiste) e tira o campo; quem chama já sabe qual.
        let finish = move |ui: &mut aylis_core::Ui, save: bool| {
            if !ui.contains(edit) {
                return;
            }
            let value = match ui.get_prop(edit, "text") {
                Ok(PropValue::String(s)) => s,
                _ => String::new(),
            };
            ui.free(edit);
            if save {
                let _ = ui.set_prop(me, "text", value.clone());
                ui.fire(me, "on_save", &EventArgs::Text(value));
            }
            ui.set_focus(me);
            ui.request_repaint();
        };
        ctx.ui.on(edit, "on_key_down", move |ui, _, args| {
            if let EventArgs::Key(k) = args {
                match k.key {
                    Key::Enter => finish(ui, true),
                    Key::Escape => finish(ui, false),
                    _ => {}
                }
            }
        });
        ctx.ui.on(edit, "on_exit", move |ui, _, _| {
            let post = ui.poster();
            post.post(move |ui| finish(ui, true));
        });
        ctx.request_focus(edit);
        ctx.ui.invalidate();
        ctx.repaint();
    }
}

impl Component for PopupEdit {
    leaf_control!("PopupEdit");

    properties! {
        text: String => "Data",
        placeholder: String => "Appearance";
        events: on_save
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button == PointerButton::Primary {
            self.open(ctx);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if matches!(event.key, Key::Enter | Key::Space) {
            self.open(ctx);
            ctx.set_handled();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if self.editor.get().is_some_and(|e| ctx.ui.contains(e)) {
            return;
        }
        let r = ctx.rect;
        let hovered = ctx.state.hover || ctx.state.focused;
        if hovered {
            canvas.fill_rect(r, 4.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
        }
        let (shown, color) = if self.text.is_empty() { (&self.placeholder, ctx.theme.color("text_muted")) } else { (&self.text, ctx.theme.color("text")) };
        text(canvas, ctx, shown, RectF::new(r.x + 6.0, r.y, (r.width - 34.0).max(0.0), r.height), 14.0, color, TextAlign::Leading, false);
        ctx.draw_icon(canvas, "edit", RectF::new(r.right() - 24.0, r.y + (r.height - 16.0) / 2.0, 16.0, 16.0), ctx.theme.color("text_muted").with_opacity(if hovered { 1.0 } else { 0.5 }));
        // A linha pontilhada do "editável".
        let y = r.bottom() - 2.0;
        let mut x = r.x + 6.0;
        while x < r.right() - 30.0 {
            canvas.draw_line(PointF::new(x, y), PointF::new((x + 3.0).min(r.right() - 30.0), y), &Stroke::new(ctx.theme.color("text_muted").with_opacity(0.6), 1.0));
            x += 6.0;
        }
    }
}

// ===========================================================================
// OptionGroup
// ===========================================================================

/// O tipo de opção do grupo (o `type` do `QOptionGroup`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OptionKind {
    #[default]
    Radio,
    Checkbox,
    Toggle,
}

impl PropEnum for OptionKind {
    const NAMES: &'static [&'static str] = &["radio", "checkbox", "toggle"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Radio, Self::Checkbox, Self::Toggle].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O grupo de opções (`QOptionGroup`): `items` (`Pequeno;Médio;Grande`) como botões de
/// rádio (uma escolha, `item_index`), caixas ou interruptores (várias, os textos marcados
/// em `selected`, separados por `;`). `inline` põe lado a lado. `on_change` a cada mudança.
pub struct OptionGroup {
    pub base: ControlBase,
    /// As opções, separadas por `;` (`Pequeno;Médio;Grande`).
    pub items: String,
    /// Botões de rádio (uma escolha), caixas de marcar ou interruptores (várias).
    pub kind: OptionKind,
    /// A opção escolhida no modo rádio (0 = a primeira; -1 = nenhuma).
    pub item_index: i64,
    /// As opções marcadas nas caixas e interruptores: os textos, separados por `;`.
    pub selected: String,
    /// As opções lado a lado em vez de uma embaixo da outra.
    pub inline: bool,
    rows: RefCell<Vec<RectF>>,
    focus: Cell<usize>,
}

impl Default for OptionGroup {
    fn default() -> Self {
        let mut base = ControlBase::new(220.0, 110.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: "Pequeno;Médio;Grande".into(),
            kind: OptionKind::Radio,
            item_index: 0,
            selected: String::new(),
            inline: false,
            rows: RefCell::new(Vec::new()),
            focus: Cell::new(0),
        }
    }
}

impl OptionGroup {
    fn is_on(&self, i: usize, item: &str) -> bool {
        match self.kind {
            OptionKind::Radio => self.item_index == i as i64,
            _ => split(&self.selected).contains(&item),
        }
    }

    fn toggle(&mut self, ctx: &mut EventCtx<'_>, i: usize) {
        let items: Vec<String> = split(&self.items).into_iter().map(str::to_string).collect();
        let Some(item) = items.get(i) else { return };
        match self.kind {
            OptionKind::Radio => {
                self.item_index = i as i64;
                ctx.fire("on_change", EventArgs::Value(PropValue::Int(i as i64)));
            }
            _ => {
                let mut on: Vec<String> = split(&self.selected).into_iter().map(str::to_string).collect();
                if let Some(k) = on.iter().position(|s| s == item) {
                    on.remove(k);
                } else {
                    on.push(item.clone());
                }
                // Na ordem dos itens.
                on.sort_by_key(|s| items.iter().position(|x| x == s).unwrap_or(usize::MAX));
                self.selected = on.join(";");
                ctx.fire("on_change", EventArgs::Text(self.selected.clone()));
            }
        }
        self.focus.set(i);
        ctx.repaint();
    }
}

impl Component for OptionGroup {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, name: &str, value: &aylis_core::PropValue) -> Option<EventArgs> {
        match name {
            "selected" => Some(EventArgs::Text(self.selected.clone())),
            _ => Some(EventArgs::Value(value.clone())),
        }
    }

    leaf_control!("OptionGroup");

    properties! {
        items: String => "Data",
        kind: OptionKind => "Behavior",
        item_index: i64 => "Data",
        selected: String => "Data",
        inline: bool => "Appearance";
        events: on_change
    }

    fn auto_height(&self, _ui: &aylis_core::Ui, _id: ComponentId, _width: f32) -> Option<f32> {
        Some(if self.inline { 40.0 } else { 36.0 * split(&self.items).len().max(1) as f32 + 4.0 })
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let hit = self.rows.borrow().iter().position(|r| r.contains(event.position));
        if let Some(i) = hit {
            self.toggle(ctx, i);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let n = split(&self.items).len();
        if n == 0 {
            return;
        }
        let f = self.focus.get().min(n - 1);
        match event.key {
            Key::Down | Key::Right => {
                self.focus.set((f + 1) % n);
                if self.kind == OptionKind::Radio {
                    self.toggle(ctx, (f + 1) % n);
                }
            }
            Key::Up | Key::Left => {
                self.focus.set((f + n - 1) % n);
                if self.kind == OptionKind::Radio {
                    self.toggle(ctx, (f + n - 1) % n);
                }
            }
            Key::Space | Key::Enter => self.toggle(ctx, f),
            _ => return,
        }
        ctx.set_handled();
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = accent(ctx);
        let border = ctx.theme.color("text_muted");
        let mut rows = Vec::new();
        let mut x = r.x;
        let mut y = r.y + 2.0;
        for (i, item) in split(&self.items).into_iter().enumerate() {
            let label_w = {
                let mut s = ctx.text_style();
                s.font = ctx.font(14.0);
                canvas.measure_text(item, &s).width
            };
            let mark_w = if self.kind == OptionKind::Toggle { 40.0 } else { 22.0 };
            let w = if self.inline { mark_w + 10.0 + label_w + 18.0 } else { r.width };
            if self.inline && x + w > r.right() && x > r.x {
                x = r.x;
                y += 36.0;
            }
            let row = RectF::new(x, y, w, 34.0);
            let on = self.is_on(i, item);
            let cy = row.y + row.height / 2.0;
            match self.kind {
                OptionKind::Radio => {
                    let c = RectF::new(row.x + 2.0, cy - 9.0, 18.0, 18.0);
                    canvas.stroke_ellipse(c, &Stroke::new(if on { color } else { border }, 2.0));
                    if on {
                        canvas.fill_ellipse(c.deflate(aylis_core::Bounds::all(4.5)), &Brush::Solid(color));
                    }
                }
                OptionKind::Checkbox => {
                    let c = RectF::new(row.x + 2.0, cy - 9.0, 18.0, 18.0);
                    if on {
                        canvas.fill_rect(c, 3.0, &Brush::Solid(color));
                        ctx.draw_icon(canvas, "check", c.deflate(aylis_core::Bounds::all(2.0)), token(ctx, "on_primary", "background"));
                    } else {
                        canvas.stroke_rect(c.deflate(aylis_core::Bounds::all(1.0)), 3.0, &Stroke::new(border, 2.0));
                    }
                }
                OptionKind::Toggle => {
                    let track = RectF::new(row.x + 2.0, cy - 7.0, 36.0, 14.0);
                    canvas.fill_rect(track, 7.0, &Brush::Solid(if on { color.with_opacity(0.5) } else { token(ctx, "track", "border") }));
                    let knob_x = if on { track.right() - 10.0 } else { track.x + 10.0 };
                    canvas.fill_ellipse(RectF::new(knob_x - 10.0, cy - 10.0, 20.0, 20.0), &Brush::Solid(if on { color } else { Color::rgb(0xFA, 0xFA, 0xFA) }));
                    if !on {
                        canvas.stroke_ellipse(RectF::new(knob_x - 10.0, cy - 10.0, 20.0, 20.0), &Stroke::new(ctx.theme.color("border"), 1.0));
                    }
                }
            }
            text(canvas, ctx, item, RectF::new(row.x + mark_w + 10.0, row.y, (row.width - mark_w - 10.0).max(0.0), row.height), 14.0, ctx.theme.color("text"), TextAlign::Leading, false);
            if self.base.state().focused && self.focus.get() == i {
                canvas.stroke_rect(row, 4.0, &Stroke::new(color.with_opacity(0.4), 1.5));
            }
            rows.push(row);
            if self.inline {
                x += w;
            } else {
                y += 36.0;
            }
        }
        *self.rows.borrow_mut() = rows;
    }
}

// ===========================================================================
// AjaxBar
// ===========================================================================

/// A barra de carregamento do topo (`QAjaxBar`): fina, na borda de cima (ou de baixo) da
/// janela. Ligando `active`, ela anda devagar até perto do fim (sem saber quanto falta);
/// desligando, completa e some.
pub struct AjaxBar {
    pub base: ControlBase,
    /// Ligado: a barra aparece e anda devagar; desligado: completa e some.
    pub active: bool,
    /// Quando ligou (e, ao desligar, quando desligou), na escala de tempo da pintura.
    started: Cell<Option<f64>>,
    finished: Cell<Option<(f32, f64)>>,
    shown: Cell<f32>,
}

impl Default for AjaxBar {
    fn default() -> Self {
        let mut base = ControlBase::new(400.0, 3.0);
        base.set_align(aylis_core::Align::Top).set_hit_test(false);
        Self { base, active: false, started: Cell::new(None), finished: Cell::new(None), shown: Cell::new(0.0) }
    }
}

impl Component for AjaxBar {
    leaf_control!("AjaxBar");

    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        active: bool => "Behavior",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "active" {
            if self.active {
                self.started.set(None);
                self.finished.set(None);
            } else if self.shown.get() > 0.0 {
                self.finished.set(Some((self.shown.get(), -1.0)));
            }
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let color = accent(ctx);
        let (fraction, opacity) = if self.active {
            let start = *self.started.get().get_or_insert(ctx.time);
            self.started.set(Some(start));
            // Chega a 90% sem nunca parar de andar (cada vez mais devagar).
            let t = (ctx.time - start) as f32;
            (0.9 * (1.0 - (-t / 2.5).exp()), 1.0)
        } else if let Some((from, at)) = self.finished.get() {
            let at = if at < 0.0 { ctx.time } else { at };
            self.finished.set(Some((from, at)));
            let k = (((ctx.time - at) / 0.4) as f32).clamp(0.0, 1.0);
            if k >= 1.0 {
                self.finished.set(None);
                self.shown.set(0.0);
            }
            (from + (1.0 - from) * (k * 2.0).min(1.0), 1.0 - ((k - 0.5).max(0.0) * 2.0))
        } else {
            if ctx.design {
                canvas.fill_rect(r, 0.0, &Brush::Solid(color.with_opacity(0.3)));
            }
            return;
        };
        self.shown.set(fraction);
        canvas.fill_rect(RectF::new(r.x, r.y, r.width * fraction, r.height), 0.0, &Brush::Solid(color.with_opacity(opacity)));
        ctx.request_frame();
    }
}

// ===========================================================================
// ChatBubble
// ===========================================================================

/// O balão de conversa (`QChatMessage`; aqui `ChatBubble`, porque o `ChatMessage` do aylis é a
/// mensagem do `ChatView`): `author` em cima, o `text` (várias linhas), a hora
/// (`stamp`) e as iniciais (ou `avatar`, um arquivo) ao lado. `sent` põe o balão à direita
/// na cor primária (a mensagem de quem usa o app). Para a conversa toda, o `ChatView`.
pub struct ChatBubble {
    pub base: ControlBase,
    /// O nome de quem escreveu, em cima do balão (e as iniciais do avatar).
    pub author: String,
    /// A mensagem (pode ter várias linhas).
    pub text: String,
    /// A hora embaixo do balão (`há 2 minutos`, `14:05`); vazio: sem hora.
    pub stamp: String,
    /// Mensagem de quem usa o app: o balão à direita, na cor primária.
    pub sent: bool,
    /// A foto ao lado do balão (um arquivo do app); vazio: as iniciais do `author`.
    pub avatar: String,
    loaded: RefCell<Option<(String, Option<Image>)>>,
}

impl Default for ChatBubble {
    fn default() -> Self {
        Self {
            base: ControlBase::new(320.0, 80.0),
            author: "Ana".into(),
            text: "Oi! Tudo certo para amanhã?".into(),
            stamp: "há 2 minutos".into(),
            sent: false,
            avatar: String::new(),
            loaded: RefCell::new(None),
        }
    }
}

impl Component for ChatBubble {
    leaf_control!("ChatBubble");

    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        author: String => "Appearance",
        text: String => "Appearance",
        stamp: String => "Appearance",
        sent: bool => "Appearance",
        avatar: String => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let side = 36.0;
        let avatar = if self.sent { RectF::new(r.right() - side, r.bottom() - side, side, side) } else { RectF::new(r.x, r.bottom() - side, side, side) };
        // O avatar: a imagem, ou as iniciais num círculo.
        let image = {
            let mut loaded = self.loaded.borrow_mut();
            if loaded.as_ref().is_none_or(|(p, _)| *p != self.avatar) {
                let image = if self.avatar.is_empty() { None } else { ctx.resource(&self.avatar).map(Image::encoded) };
                *loaded = Some((self.avatar.clone(), image));
            }
            loaded.as_ref().and_then(|(_, i)| i.clone())
        };
        match image.as_ref().and_then(|img| canvas.image_size(img).map(|s| (img, s))) {
            Some((img, size)) => {
                canvas.save();
                let mut clip = Path::new();
                clip.ellipse(avatar);
                canvas.clip_path(&clip, false);
                canvas.draw_image(img, RectF::new(0.0, 0.0, size.width, size.height), avatar, 1.0);
                canvas.restore();
            }
            None => {
                let initials: String = self.author.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect::<String>().to_uppercase();
                canvas.fill_ellipse(avatar, &Brush::Solid(crate::name_color(&self.author)));
                text(canvas, ctx, &initials, avatar, 13.0, Color::rgb(0xFF, 0xFF, 0xFF), TextAlign::Center, true);
            }
        }
        let primary = accent(ctx);
        let (fill, fg) = if self.sent { (primary, token(ctx, "on_primary", "background")) } else { (token(ctx, "surface_2", "surface"), ctx.theme.color("text")) };
        let name_h = if self.author.is_empty() { 0.0 } else { 18.0 };
        let mut name_rect = RectF::new(r.x + side + 10.0, r.y, r.width - side - 20.0, name_h);
        if self.sent {
            name_rect.x = r.x + 10.0;
        }
        if name_h > 0.0 {
            text(canvas, ctx, &self.author, name_rect, 12.0, ctx.theme.color("text_muted"), if self.sent { TextAlign::Trailing } else { TextAlign::Leading }, false);
        }
        // O balão: até 75% da largura, a ponta no lado do avatar.
        let max_w = (r.width - side - 20.0).max(40.0) * 0.9;
        let mut body = ctx.text_style();
        body.font = ctx.font(14.0);
        body.color = fg;
        body.wrap = true;
        body.v_align = TextAlign::Leading;
        let measured = canvas.measure_text(&self.text, &body);
        let w = (measured.width + 24.0).clamp(60.0, max_w);
        let bubble_top = r.y + name_h + 2.0;
        let h = (r.bottom() - bubble_top).max(24.0);
        let bubble = if self.sent { RectF::new(avatar.x - 10.0 - w, bubble_top, w, h) } else { RectF::new(avatar.right() + 10.0, bubble_top, w, h) };
        canvas.fill_rect(bubble, 8.0, &Brush::Solid(fill));
        let mut tail = Path::new();
        let ty = bubble.bottom() - 14.0;
        if self.sent {
            tail.polygon(&[PointF::new(bubble.right() - 2.0, ty), PointF::new(bubble.right() + 8.0, bubble.bottom()), PointF::new(bubble.right() - 10.0, bubble.bottom())], true);
        } else {
            tail.polygon(&[PointF::new(bubble.x + 2.0, ty), PointF::new(bubble.x - 8.0, bubble.bottom()), PointF::new(bubble.x + 10.0, bubble.bottom())], true);
        }
        canvas.fill_path(&tail, &Brush::Solid(fill));
        let stamp_h = if self.stamp.is_empty() { 0.0 } else { 16.0 };
        canvas.draw_text(&self.text, RectF::new(bubble.x + 12.0, bubble.y + 8.0, bubble.width - 24.0, (bubble.height - 12.0 - stamp_h).max(0.0)), &body);
        if stamp_h > 0.0 {
            text(canvas, ctx, &self.stamp, RectF::new(bubble.x + 12.0, bubble.bottom() - stamp_h - 4.0, bubble.width - 24.0, stamp_h), 11.0, fg.with_opacity(0.7), TextAlign::Trailing, false);
        }
    }
}

impl aylis_core::PlatformHooks for Uploader {
    fn files_dropped(&mut self, ctx: &mut EventCtx<'_>, paths: &[String]) -> bool {
        let mut all: Vec<String> = if self.multiple { split(&self.files).into_iter().map(str::to_string).collect() } else { Vec::new() };
        for p in paths.iter().take(if self.multiple { usize::MAX } else { 1 }) {
            if !all.contains(p) {
                all.push(p.clone());
            }
        }
        self.files = all.join(";");
        ctx.fire("on_change", EventArgs::Text(self.files.clone()));
        if self.auto_upload {
            self.send(ctx);
        }
        ctx.repaint();
        true
    }
}
