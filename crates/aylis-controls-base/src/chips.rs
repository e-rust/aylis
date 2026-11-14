//! Entradas compactas: `Chip` e `ChipGroup` (etiquetas e filtros), `Rating` (estrelas),
//! `RangeSlider` (intervalo), `OtpInput` (código de verificação) e `SplitButton` (ação
//! com menu).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, Path, PointF,
    PointerButton, PointerEvent, RectF, Stroke, TextAlign, properties,
};

use crate::leaf_control;

fn split(items: &str) -> Vec<&str> {
    if items.is_empty() { Vec::new() } else { items.split(';').collect() }
}

/// Cor de destaque da variante (`primary`, `success`...); sem variante, `primary`.
fn accent(ctx: &PaintCtx<'_>) -> Color {
    let v = ctx.variant();
    let token = match v {
        "success" | "warning" | "danger" | "info" => v,
        _ => "primary",
    };
    ctx.theme.color(token)
}

/// Desenha um chip: fundo, ícone, texto e o "x". Devolve o retângulo do "x".
#[allow(clippy::too_many_arguments)]
fn paint_chip(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, r: RectF, text: &str, icon: &str, selected: bool, closable: bool, outlined: bool) -> RectF {
    let theme = ctx.theme;
    let color = accent(ctx);
    // O Material 3 usa chips de cantos 8, não pílulas.
    let radius = if ctx.look() == aylis_core::Look::Material3 { 8.0 } else { r.height / 2.0 };
    let toned = matches!(ctx.variant(), "success" | "warning" | "danger" | "info");
    let (fill, fg) = if selected || toned {
        (color.with_opacity(0.16), color)
    } else if outlined {
        (Color::TRANSPARENT, theme.color("text"))
    } else {
        (theme.color("surface_2"), theme.color("text"))
    };
    canvas.fill_rect(r, radius, &Brush::Solid(fill));
    if outlined || selected {
        canvas.stroke_rect(r.inflate(-0.5), radius, &Stroke::new(if selected { color.with_opacity(0.5) } else { theme.color("border") }, 1.0));
    }
    let mut x = r.x + 12.0;
    let side = (r.height * 0.55).round();
    let icon = if selected && icon.is_empty() { "check" } else { icon };
    if !icon.is_empty() {
        ctx.draw_icon(canvas, icon, RectF::new(x - 2.0, r.y + (r.height - side) / 2.0, side, side), fg);
        x += side + 4.0;
    }
    let close = RectF::new(r.right() - side - 8.0, r.y + (r.height - side) / 2.0, side, side);
    let right = if closable { close.x - 4.0 } else { r.right() - 12.0 };
    let mut t = ctx.text_style();
    t.color = fg;
    canvas.draw_text(text, RectF::new(x, r.y, (right - x).max(0.0), r.height), &t);
    if closable {
        ctx.draw_icon(canvas, "close", close, fg.with_opacity(0.7));
    }
    close
}

fn chip_width(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, text: &str, icon: bool, closable: bool, height: f32) -> f32 {
    let side = (height * 0.55).round();
    let tw = canvas.measure_text(text, &ctx.text_style()).width;
    28.0 + tw.ceil() + if icon { side + 4.0 } else { 0.0 } + if closable { side + 4.0 } else { 0.0 }
}

// ===========================================================================
// Chip
// ===========================================================================

/// Etiqueta compacta (o `v-chip`): texto, ícone opcional, marcável (`checkable`) e com
/// botão de fechar (`closable`, evento `on_close`). Variantes: `outline`, `success`...
pub struct Chip {
    pub base: ControlBase,
    /// O texto da etiqueta.
    pub text: String,
    /// Ícone pelo nome, antes do texto; vazio: sem ícone.
    pub icon: String,
    /// Mostra o "×" de fechar, que dispara o `on_close`.
    pub closable: bool,
    /// Cada clique alterna o `checked` (e dispara o `on_change`).
    pub checkable: bool,
    /// Marcado, com `checkable`.
    pub checked: bool,
    close_rect: Cell<RectF>,
}

impl Default for Chip {
    fn default() -> Self {
        let mut base = ControlBase::new(96.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, text: "Etiqueta".into(), icon: String::new(), closable: false, checkable: false, checked: false, close_rect: Cell::new(RectF::default()) }
    }
}

impl Chip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }
}

impl Component for Chip {
    leaf_control!("Chip");

    properties! {
        text: String => "Appearance",
        icon: String => "Appearance",
        closable: bool => "Behavior",
        checkable: bool => "Behavior",
        checked: bool => "Data";
        events: on_close, on_change
    }

    // Um chip é clicado; marcar (`on_change`) e fechar (`on_close`) são casos à parte.
    fn default_event(&self) -> Option<&'static str> {
        Some("on_click")
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        if self.closable && self.close_rect.get().inflate(4.0).contains(event.position) {
            ctx.fire("on_close", EventArgs::None);
            return;
        }
        if self.checkable {
            self.checked = !self.checked;
            ctx.fire("on_change", EventArgs::Value(self.checked.into()));
            ctx.repaint();
        }
        ctx.fire("on_click", EventArgs::None);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let outlined = ctx.variant() == "outline";
        let close = paint_chip(canvas, ctx, ctx.rect, &self.text, &self.icon, self.checked, self.closable, outlined);
        self.close_rect.set(close);
        if ctx.state.hover {
            canvas.fill_rect(ctx.rect, ctx.rect.height / 2.0, &Brush::Solid(ctx.theme.color("text").with_opacity(0.05)));
        }
        if ctx.state.focused {
            canvas.stroke_rect(ctx.rect, ctx.rect.height / 2.0, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
    }
}

// ===========================================================================
// ChipGroup
// ===========================================================================

/// Grupo de chips para escolher (o `v-chip-group`): uma opção ou várias (`multiple`).
/// `selected` são os itens escolhidos, separados por `;`. Quebra a linha sozinho; com
/// `wrap` desligado fica numa linha só e rola para os lados (o `v-slide-group`): roda do
/// mouse, arrasto no toque e setas nas pontas.
pub struct ChipGroup {
    pub base: ControlBase,
    /// As opções, separadas por `;`: um chip para cada.
    pub items: String,
    /// Os itens escolhidos, separados por `;`; vazio: nenhum.
    pub selected: String,
    /// Deixa escolher vários (sem ele, escolher um desmarca o outro).
    pub multiple: bool,
    /// Quebra a linha quando os chips não cabem; desligado, fica numa linha só que rola para
    /// os lados.
    pub wrap: bool,
    /// A altura de cada chip, em pontos.
    pub chip_height: f32,
    rects: RefCell<Vec<RectF>>,
    /// Rolagem lateral (sem `wrap`) e a largura de tudo.
    offset: Cell<f32>,
    content: Cell<f32>,
}

impl Default for ChipGroup {
    fn default() -> Self {
        let mut base = ControlBase::new(320.0, 40.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            items: "Todos;Abertos;Fechados".into(),
            selected: "Todos".into(),
            multiple: false,
            wrap: true,
            chip_height: 32.0,
            rects: RefCell::new(Vec::new()),
            offset: Cell::new(0.0),
            content: Cell::new(0.0),
        }
    }
}

impl ChipGroup {
    pub fn is_selected(&self, item: &str) -> bool {
        split(&self.selected).contains(&item)
    }

    fn toggle(&mut self, ctx: &mut EventCtx<'_>, item: &str) {
        let mut chosen: Vec<String> = split(&self.selected).into_iter().map(str::to_string).collect();
        if self.multiple {
            if let Some(i) = chosen.iter().position(|c| c == item) {
                chosen.remove(i);
            } else {
                // Na ordem dos itens.
                chosen.push(item.to_string());
                let order = split(&self.items);
                chosen.sort_by_key(|c| order.iter().position(|o| o == c));
            }
        } else {
            chosen = vec![item.to_string()];
        }
        let next = chosen.join(";");
        if next != self.selected {
            self.selected = next;
            ctx.fire("on_change", EventArgs::Text(self.selected.clone()));
        }
        ctx.repaint();
    }
}

impl Component for ChipGroup {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.selected.clone()))
    }

    leaf_control!("ChipGroup");

    properties! {
        items: String => "Data",
        selected: String => "Data",
        multiple: bool => "Behavior",
        wrap: bool => "Appearance",
        chip_height: f32 => "Appearance";
        events: on_change
    }

    // Os chips para leitores de tela e automação: botões que marcam; clicar alterna.
    fn access_items(&self, _: &aylis_core::Ui, _: aylis_core::ComponentId) -> Vec<aylis_core::automation::AccessItem> {
        use aylis_core::automation::{AccessItem, ItemRole};
        let rects = self.rects.borrow();
        split(&self.items)
            .into_iter()
            .enumerate()
            .map(|(i, t)| {
                let on = self.is_selected(t);
                let item = AccessItem::new(ItemRole::Button, t).checked(on).selected(on);
                match rects.get(i) {
                    Some(r) => item.rect(r.offset(if self.wrap { 0.0 } else { -self.offset.get() }, 0.0)),
                    None => item,
                }
            })
            .collect()
    }

    fn access_action(&mut self, ctx: &mut EventCtx<'_>, index: usize, _: aylis_core::automation::ItemAction) -> bool {
        let Some(item) = split(&self.items).get(index).map(|s| s.to_string()) else { return false };
        self.toggle(ctx, &item);
        true
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &aylis_core::WheelEvent) {
        if self.wrap {
            return;
        }
        let max = (self.content.get() - self.base.bounds().width).max(0.0);
        let delta = if event.delta.x != 0.0 { event.delta.x } else { event.delta.y };
        let next = (self.offset.get() - delta).clamp(0.0, max);
        if next != self.offset.get() {
            self.offset.set(next);
            ctx.repaint();
            ctx.set_handled();
        }
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if !self.wrap && self.content.get() > self.base.bounds().width {
            // As setas das pontas rolam meia largura.
            let w = self.base.bounds().width;
            let max = (self.content.get() - w).max(0.0);
            let step = w / 2.0;
            if event.position.x < ARROW_ZONE && self.offset.get() > 0.0 {
                self.offset.set((self.offset.get() - step).max(0.0));
                return ctx.repaint();
            }
            if event.position.x > w - ARROW_ZONE && self.offset.get() < max {
                self.offset.set((self.offset.get() + step).min(max));
                return ctx.repaint();
            }
        }
        let p = PointF::new(event.position.x + if self.wrap { 0.0 } else { self.offset.get() }, event.position.y);
        let hit = self.rects.borrow().iter().position(|r| r.contains(p));
        if let Some(i) = hit {
            let item = split(&self.items).get(i).map(|s| s.to_string());
            if let Some(item) = item {
                self.toggle(ctx, &item);
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let h = self.chip_height;
        let (mut x, mut y) = (0.0f32, 0.0f32);
        let mut rects = Vec::new();
        let offset = if self.wrap { 0.0 } else { self.offset.get() };
        canvas.save();
        canvas.clip_rect(ctx.rect);
        canvas.translate(-offset, 0.0);
        for item in split(&self.items) {
            let selected = self.is_selected(item);
            let w = chip_width(canvas, ctx, item, selected, false, h);
            if self.wrap && x > 0.0 && x + w > ctx.rect.width {
                x = 0.0;
                y += h + 8.0;
            }
            let r = RectF::new(x, y + if self.wrap { 0.0 } else { (ctx.rect.height - h) / 2.0 }, w, h);
            paint_chip(canvas, ctx, r, item, "", selected, false, true);
            rects.push(r);
            x += w + 8.0;
        }
        canvas.restore();
        self.content.set((x - 8.0).max(0.0));
        // Setas nas pontas quando há mais para os lados.
        if !self.wrap {
            let max = (x - 8.0 - ctx.rect.width).max(0.0);
            let bg = ctx.theme.color("background");
            for (show, left, icon) in [(offset > 0.0, true, "chevron-left"), (offset < max - 0.5, false, "chevron-right")] {
                if !show {
                    continue;
                }
                let zone = if left { RectF::new(0.0, 0.0, ARROW_ZONE, ctx.rect.height) } else { RectF::new(ctx.rect.width - ARROW_ZONE, 0.0, ARROW_ZONE, ctx.rect.height) };
                canvas.fill_rect(zone, 0.0, &Brush::Solid(bg.with_opacity(0.92)));
                ctx.draw_icon(canvas, icon, RectF::new(zone.x + (zone.width - 18.0) / 2.0, zone.y + (zone.height - 18.0) / 2.0, 18.0, 18.0), ctx.theme.color("text"));
            }
        }
        *self.rects.borrow_mut() = rects;
    }
}

const ARROW_ZONE: f32 = 28.0;

// ===========================================================================
// Rating
// ===========================================================================

/// Avaliação em estrelas (o `v-rating`): `value` de 0 a `max`, com meia estrela
/// (`half`). Mostra a prévia ao passar o mouse; `read_only` só mostra.
pub struct Rating {
    pub base: ControlBase,
    /// A nota, de 0 (nenhuma estrela) a `max`.
    pub value: f32,
    /// Quantas estrelas aparecem.
    pub max: u32,
    /// Aceita meia estrela: o clique na metade esquerda e as setas andam de 0,5 em 0,5.
    pub half: bool,
    /// Só mostra a nota: o clique e o teclado não mudam.
    pub read_only: bool,
    hover: Cell<Option<f32>>,
}

impl Default for Rating {
    fn default() -> Self {
        let mut base = ControlBase::new(150.0, 30.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, value: 3.0, max: 5, half: false, read_only: false, hover: Cell::new(None) }
    }
}

fn star(c: PointF, r: f32) -> Path {
    let mut p = Path::new();
    let points: Vec<PointF> = (0..10)
        .map(|i| {
            let a = (i as f32 * 36.0 - 90.0).to_radians();
            let rr = if i % 2 == 0 { r } else { r * 0.45 };
            PointF::new(c.x + rr * a.cos(), c.y + rr * a.sin())
        })
        .collect();
    p.polygon(&points, true);
    p
}

impl Rating {
    fn value_at(&self, x: f32) -> f32 {
        let w = self.base.bounds().width / self.max.max(1) as f32;
        let raw = x / w;
        let v = if self.half { (raw * 2.0).ceil() / 2.0 } else { raw.ceil() };
        v.clamp(0.0, self.max as f32)
    }
}

impl Component for Rating {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("Rating");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        value: f32 => "Data",
        max: u32 => "Data",
        half: bool => "Behavior",
        read_only: bool => "Behavior";
        events: on_change
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if !self.read_only {
            self.hover.set(Some(self.value_at(event.position.x)));
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover.set(None);
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if self.read_only || event.button != PointerButton::Primary {
            return;
        }
        let v = self.value_at(event.position.x);
        // Clicar de novo na mesma nota zera.
        self.value = if (v - self.value).abs() < 0.01 { 0.0 } else { v };
        ctx.fire("on_change", EventArgs::Value((self.value as f64).into()));
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if self.read_only {
            return;
        }
        let step = if self.half { 0.5 } else { 1.0 };
        let v = match event.key {
            Key::Left | Key::Down => self.value - step,
            Key::Right | Key::Up => self.value + step,
            _ => return,
        };
        self.value = v.clamp(0.0, self.max as f32);
        ctx.fire("on_change", EventArgs::Value((self.value as f64).into()));
        ctx.set_handled();
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let n = self.max.max(1);
        let w = ctx.rect.width / n as f32;
        let r = (w.min(ctx.rect.height) / 2.0 - 2.0).max(2.0);
        let shown = self.hover.get().unwrap_or(self.value);
        // A estrela acesa: o destaque próprio, a cor `rating` do tema ou o dourado.
        let on = ctx.colors.accent.or_else(|| ctx.theme.colors.get("rating").copied()).unwrap_or(Color::rgb(0xFF, 0xB4, 0x00));
        let off = ctx.theme.color("border");
        for i in 0..n {
            let c = PointF::new(w * (i as f32 + 0.5), ctx.rect.height / 2.0);
            let s = star(c, r);
            canvas.fill_path(&s, &Brush::Solid(off));
            let fill = (shown - i as f32).clamp(0.0, 1.0);
            if fill > 0.0 {
                canvas.save();
                canvas.clip_rect(RectF::new(c.x - r, c.y - r, 2.0 * r * fill, 2.0 * r));
                canvas.fill_path(&s, &Brush::Solid(on));
                canvas.restore();
            }
        }
        if ctx.state.focused {
            canvas.stroke_rect(ctx.rect, 4.0, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
    }
}

// ===========================================================================
// RangeSlider
// ===========================================================================

/// Controle deslizante de intervalo (o `v-range-slider`): `low` e `high` entre `min` e
/// `max`, com passo `step` (0 = contínuo). Arrasta o controle mais perto.
pub struct RangeSlider {
    pub base: ControlBase,
    /// O começo do intervalo escolhido (o controle da esquerda).
    pub low: f64,
    /// O fim do intervalo escolhido (o controle da direita).
    pub high: f64,
    /// O menor valor do trilho.
    pub min: f64,
    /// O maior valor do trilho.
    pub max: f64,
    /// O passo dos valores; 0 = contínuo.
    pub step: f64,
    dragging: Cell<Option<bool>>,
}

impl Default for RangeSlider {
    fn default() -> Self {
        let mut base = ControlBase::new(240.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, low: 20.0, high: 80.0, min: 0.0, max: 100.0, step: 1.0, dragging: Cell::new(None) }
    }
}

const RANGE_PAD: f32 = 12.0;

impl RangeSlider {
    fn x_of(&self, v: f64) -> f32 {
        let w = self.base.bounds().width - 2.0 * RANGE_PAD;
        let span = (self.max - self.min).max(1e-9);
        RANGE_PAD + (((v - self.min) / span) as f32).clamp(0.0, 1.0) * w
    }

    fn value_of(&self, x: f32) -> f64 {
        let w = (self.base.bounds().width - 2.0 * RANGE_PAD).max(1.0);
        let mut v = self.min + ((x - RANGE_PAD) / w).clamp(0.0, 1.0) as f64 * (self.max - self.min);
        if self.step > 0.0 {
            v = self.min + ((v - self.min) / self.step).round() * self.step;
        }
        v.clamp(self.min, self.max)
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, high: bool, v: f64) {
        let (old_low, old_high) = (self.low, self.high);
        if high { self.high = v.max(self.low) } else { self.low = v.min(self.high) }
        if (self.low, self.high) != (old_low, old_high) {
            ctx.fire("on_change", EventArgs::None);
            ctx.repaint();
        }
    }
}

impl Component for RangeSlider {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::None)
    }

    leaf_control!("RangeSlider");

    properties! {
        low: f64 => "Data",
        high: f64 => "Data",
        min: f64 => "Data",
        max: f64 => "Data",
        step: f64 => "Data";
        events: on_change
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let x = event.position.x;
        let high = (x - self.x_of(self.high)).abs() < (x - self.x_of(self.low)).abs() || (self.low == self.high && x > self.x_of(self.high));
        self.dragging.set(Some(high));
        self.set(ctx, high, self.value_of(x));
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some(high) = self.dragging.get() {
            self.set(ctx, high, self.value_of(event.position.x));
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.dragging.set(None);
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        let step = if self.step > 0.0 { self.step } else { (self.max - self.min) / 100.0 };
        let high = event.modifiers.shift;
        let base = if high { self.high } else { self.low };
        let v = match event.key {
            Key::Left | Key::Down => base - step,
            Key::Right | Key::Up => base + step,
            _ => return,
        };
        self.set(ctx, high, v.clamp(self.min, self.max));
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (thick, side) = crate::looks::slider_metrics(ctx.look());
        let y = ctx.rect.height / 2.0;
        let rail = RectF::new(RANGE_PAD, y - thick / 2.0, ctx.rect.width - 2.0 * RANGE_PAD, thick);
        canvas.fill_rect(rail, thick / 2.0, &Brush::Solid(ctx.theme.color("track")));
        let (a, b) = (self.x_of(self.low), self.x_of(self.high));
        canvas.fill_rect(RectF::new(a, rail.y, b - a, thick), thick / 2.0, &Brush::Solid(accent(ctx)));
        for x in [a, b] {
            crate::looks::paint_thumb(canvas, ctx, PointF::new(x, y), side, false);
        }
    }
}

// ===========================================================================
// OtpInput
// ===========================================================================

/// Código de verificação (o `v-otp-input`): `length` caixas de um caractere. `text` é o
/// código; `on_complete` quando todas as caixas estão cheias. `digits` aceita só números.
pub struct OtpInput {
    pub base: ControlBase,
    /// O código digitado até agora.
    pub text: String,
    /// Quantas caixas (caracteres) o código tem.
    pub length: u32,
    /// Só aceita números.
    pub digits: bool,
    /// Mostra `•` no lugar dos caracteres.
    pub password: bool,
}

impl Default for OtpInput {
    fn default() -> Self {
        let mut base = ControlBase::new(280.0, 48.0);
        base.set_can_focus(true).set_cursor(CursorShape::Text);
        Self { base, text: String::new(), length: 6, digits: true, password: false }
    }
}

impl OtpInput {
    fn changed(&mut self, ctx: &mut EventCtx<'_>) {
        ctx.fire("on_change", EventArgs::Text(self.text.clone()));
        if self.text.chars().count() == self.length as usize {
            ctx.fire("on_complete", EventArgs::Text(self.text.clone()));
        }
        ctx.repaint();
    }
}

impl Component for OtpInput {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.text.clone()))
    }

    leaf_control!("OtpInput");

    properties! {
        text: String => "Data",
        length: u32 => "Behavior",
        digits: bool => "Behavior",
        password: bool => "Behavior";
        events: on_change, on_complete
    }

    fn accepts_text(&self) -> bool {
        true
    }

    fn text_input(&mut self, ctx: &mut EventCtx<'_>, text: &str) {
        let mut changed = false;
        for c in text.chars() {
            if self.text.chars().count() >= self.length as usize {
                break;
            }
            if (self.digits && !c.is_ascii_digit()) || c.is_control() {
                continue;
            }
            self.text.push(c);
            changed = true;
        }
        if changed {
            self.changed(ctx);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Backspace && self.text.pop().is_some() {
            self.changed(ctx);
            ctx.set_handled();
        }
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        ctx.request_focus(ctx.id);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let n = self.length.max(1) as usize;
        let gap = 8.0;
        let w = ((ctx.rect.width - gap * (n as f32 - 1.0)) / n as f32).max(8.0);
        let chars: Vec<char> = self.text.chars().collect();
        let theme = ctx.theme;
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Center;
        t.font.size = (ctx.rect.height * 0.45).max(12.0);
        for i in 0..n {
            let r = RectF::new(i as f32 * (w + gap), 0.0, w, ctx.rect.height);
            let current = ctx.state.focused && i == chars.len().min(n - 1);
            canvas.fill_rect(r, 8.0, &Brush::Solid(theme.color("surface")));
            let (color, width) = if current { (theme.color("primary"), 2.0) } else { (theme.color("border"), 1.0) };
            canvas.stroke_rect(r.inflate(-width / 2.0), 8.0, &Stroke::new(color, width));
            if let Some(c) = chars.get(i) {
                let shown = if self.password { "•".to_string() } else { c.to_string() };
                canvas.draw_text(&shown, r, &t);
            } else if current {
                // Cursor piscando.
                if (ctx.time * 2.0) as i64 % 2 == 0 {
                    let x = r.x + r.width / 2.0;
                    canvas.draw_line(PointF::new(x, r.y + r.height * 0.28), PointF::new(x, r.bottom() - r.height * 0.28), &Stroke::new(t.color, 1.5));
                }
                ctx.request_frame_at((ctx.time * 2.0).floor() / 2.0 + 0.5);
            }
        }
    }
}

// ===========================================================================
// SplitButton
// ===========================================================================

/// Botão com ação principal e uma seta para mais opções (o `v-split-btn`): o clique no
/// texto dispara `on_click`; na seta, abre o `popup_menu` (se houver) e dispara
/// `on_dropdown`.
pub struct SplitButton {
    pub base: ControlBase,
    /// O texto da ação principal.
    pub text: String,
    /// Ícone pelo nome, antes do texto; vazio: sem ícone.
    pub icon: String,
}

impl Default for SplitButton {
    fn default() -> Self {
        let mut base = ControlBase::new(150.0, 36.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, text: "Salvar".into(), icon: String::new() }
    }
}

const ARROW_W: f32 = 34.0;

impl Component for SplitButton {
    leaf_control!("SplitButton");

    properties! {
        text: String => "Appearance",
        icon: String => "Appearance";
        events: on_dropdown
    }

    fn click(&mut self, _ctx: &mut EventCtx<'_>) {}

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.button != PointerButton::Primary {
            return;
        }
        let b = self.base.bounds();
        if event.position.x >= b.width - ARROW_W {
            ctx.fire("on_dropdown", EventArgs::None);
            if let Some(menu) = self.base.popup_menu() {
                let at = ctx.ui.to_form(ctx.id, RectF::new(b.x, b.y, b.width, b.height));
                let at = PointF::new(at.x, at.bottom() + 2.0);
                ctx.ui.poster().post(move |ui| crate::PopupMenu::popup(ui, menu, at));
            }
        } else {
            ctx.fire("on_click", EventArgs::None);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // O fundo próprio (`fill`) vale mais que o destaque.
        let color = ctx.colors.fill.unwrap_or_else(|| accent(ctx));
        let on = ctx.theme.color("on_primary");
        let radius = ctx.style.radius.max(4.0);
        if ctx.look() == aylis_core::Look::Material3 {
            // Material 3: duas peças com um vão; cantos de fora inteiros, de dentro 4.
            let full = r.height / 2.0;
            let lead = RectF::new(r.x, r.y, r.width - ARROW_W - 2.0, r.height);
            let trail = RectF::new(r.right() - ARROW_W, r.y, ARROW_W, r.height);
            let half = |canvas: &mut dyn Canvas, rect: RectF, clip: RectF, rad: f32| {
                canvas.save();
                canvas.clip_rect(clip);
                canvas.fill_rect(rect, rad, &Brush::Solid(color));
                canvas.restore();
            };
            half(canvas, lead, RectF::new(lead.x, lead.y, lead.width / 2.0 + 1.0, lead.height), full);
            half(canvas, lead, RectF::new(lead.x + lead.width / 2.0, lead.y, lead.width / 2.0, lead.height), 4.0);
            half(canvas, trail, RectF::new(trail.x, trail.y, trail.width / 2.0 + 1.0, trail.height), 4.0);
            half(canvas, trail, RectF::new(trail.x + trail.width / 2.0, trail.y, trail.width / 2.0, trail.height), full);
            let mut t = ctx.text_style();
            t.color = on;
            t.h_align = TextAlign::Center;
            t.font.weight = aylis_core::FontWeight::Medium;
            let mut left = lead.x;
            if !self.icon.is_empty() {
                ctx.draw_icon(canvas, &self.icon, RectF::new(lead.x + 14.0, r.y + (r.height - 18.0) / 2.0, 18.0, 18.0), on);
                left += 26.0;
            }
            canvas.draw_text(&self.text, RectF::new(left, r.y, lead.right() - left, r.height), &t);
            ctx.draw_icon(canvas, "chevron-down", RectF::new(trail.x + (ARROW_W - 18.0) / 2.0 - 1.0, r.y + (r.height - 18.0) / 2.0, 18.0, 18.0), on);
            return;
        }
        canvas.fill_rect(r, radius, &Brush::Solid(color));
        if ctx.state.hover {
            canvas.fill_rect(r, radius, &Brush::Solid(Color::WHITE.with_opacity(0.08)));
        }
        let split_x = r.right() - ARROW_W;
        canvas.draw_line(PointF::new(split_x, r.y + 6.0), PointF::new(split_x, r.bottom() - 6.0), &Stroke::new(on.with_opacity(0.4), 1.0));
        let mut t = ctx.text_style();
        t.color = on;
        t.h_align = TextAlign::Center;
        let mut left = r.x;
        if !self.icon.is_empty() {
            ctx.draw_icon(canvas, &self.icon, RectF::new(r.x + 10.0, r.y + (r.height - 18.0) / 2.0, 18.0, 18.0), on);
            left += 24.0;
        }
        canvas.draw_text(&self.text, RectF::new(left, r.y, split_x - left, r.height), &t);
        ctx.draw_icon(canvas, "chevron-down", RectF::new(split_x + (ARROW_W - 16.0) / 2.0, r.y + (r.height - 16.0) / 2.0, 16.0, 16.0), on);
        if ctx.state.focused {
            canvas.stroke_rect(r, radius, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
    }
}
