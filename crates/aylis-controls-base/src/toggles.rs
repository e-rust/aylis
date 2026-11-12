//! Controles de marcar: CheckBox, RadioButton e Switch.
//!
//! Os três guardam `checked` também no estado do controle (`state.checked`), então o
//! tema (estilo `checked`) e as animações com `trigger: checked` funcionam sozinhos.

use aylis_core::{
    Canvas, Component, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, PaintCtx, RectF, TextAlign,
    properties,
};

use crate::leaf_control;

const GAP: f32 = 8.0;

fn toggle_base(width: f32) -> ControlBase {
    let mut base = ControlBase::new(width, 24.0);
    base.set_can_focus(true).set_cursor(CursorShape::Pointer);
    base
}

/// Espaço também clica (Enter fica para o botão padrão do form).
pub(crate) fn space_clicks(component: &mut dyn Component, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
    if event.key == Key::Space && !event.repeat {
        component.click(ctx);
        ctx.set_handled();
    }
}

/// Rótulo à direita da marca.
fn paint_caption(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, text: &str, left: f32) {
    let mut style = ctx.text_style();
    style.h_align = TextAlign::Leading;
    let r = ctx.rect;
    // "&Aceito": Alt+A marca (o & some e a letra fica sublinhada).
    let (text, at) = crate::mnemonic::caption(text);
    crate::mnemonic::draw_text(canvas, &text, at, RectF::new(left, 0.0, (r.width - left).max(0.0), r.height), &style);
}

/// A largura de uma marca com o texto ao lado: a marca, o vão e o texto (o `auto_size` do
/// CheckBox e do RadioButton; C21 do Tabernáculo).
fn caption_width(ui: &aylis_core::Ui, id: aylis_core::ComponentId, text: &str, side: f32) -> f32 {
    let (plain, _) = crate::mnemonic::caption(text);
    if plain.is_empty() {
        return side + 2.0;
    }
    let style = aylis_core::TextStyle { font: ui.font_of(id), ..Default::default() };
    (1.0 + side + GAP + ui.measure_text(&plain, &style).width + 1.0).ceil()
}

fn mark_rect(ctx: &PaintCtx<'_>, width: f32, height: f32) -> RectF {
    RectF::new(1.0, ((ctx.rect.height - height) / 2.0).round(), width, height)
}

// ---------------------------------------------------------------------------

/// Caixa de marcação (como o `TCheckBox`).
pub struct CheckBox {
    pub base: ControlBase,
    pub text: String,
    pub checked: bool,
    /// Lado da caixinha (0 = o do visual do tema).
    pub box_size: f32,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self { base: toggle_base(160.0), text: String::new(), checked: false, box_size: 0.0 }
    }
}

impl CheckBox {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    fn sync(&mut self, _name: &str) {
        self.base.state_mut().checked = self.checked;
    }
}

impl Component for CheckBox {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "checked")
    }

    leaf_control!("CheckBox");

    fn checked_state(&self) -> Option<bool> {
        Some(self.checked)
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
    }

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        text: String => "Appearance",
        checked: bool => "Data",
        box_size: f32 => "Appearance";
        events: on_change;
        on_set: sync
    }

    /// Inverte a marca, dispara `on_change` e depois `on_click`.
    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        self.checked = !self.checked;
        self.sync("checked");
        ctx.fire("on_change", EventArgs::Value(self.checked.into()));
        ctx.fire("on_click", EventArgs::None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        space_clicks(self, ctx, event);
    }

    /// `auto_size`: uma linha de texto (ou a caixinha, se for maior).
    fn auto_height(&self, ui: &aylis_core::Ui, id: aylis_core::ComponentId, _width: f32) -> Option<f32> {
        let style = aylis_core::TextStyle { font: ui.font_of(id), ..Default::default() };
        Some(ui.measure_text("Ág", &style).height.max(self.box_size))
    }

    fn auto_width(&self, ui: &aylis_core::Ui, id: aylis_core::ComponentId) -> Option<f32> {
        let side = if self.box_size > 0.0 { self.box_size } else { crate::looks::mark_size(ui.theme().look) };
        Some(caption_width(ui, id, &self.text, side))
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let side = if self.box_size > 0.0 { self.box_size } else { crate::looks::mark_size(ctx.look()) };
        let b = mark_rect(ctx, side, side);
        crate::looks::paint_check(canvas, ctx, b, self.checked);
        paint_caption(canvas, ctx, &self.text, side + GAP);
    }
}

// ---------------------------------------------------------------------------

/// Opção exclusiva (como o `TRadioButton`). Ao ser clicado, desmarca os irmãos (mesmo
/// pai) que tenham o mesmo `group`.
pub struct RadioButton {
    pub base: ControlBase,
    pub text: String,
    pub checked: bool,
    pub group: String,
}

impl Default for RadioButton {
    fn default() -> Self {
        Self { base: toggle_base(160.0), text: String::new(), checked: false, group: String::new() }
    }
}

impl RadioButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    fn sync(&mut self, _name: &str) {
        self.base.state_mut().checked = self.checked;
    }
}

impl Component for RadioButton {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "checked")
    }

    leaf_control!("RadioButton");

    fn checked_state(&self) -> Option<bool> {
        Some(self.checked)
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
    }

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        text: String => "Appearance",
        checked: bool => "Data",
        group: String => "Behavior";
        events: on_change;
        on_set: sync
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if !self.checked {
            self.checked = true;
            self.sync("checked");
            ctx.fire("on_change", EventArgs::Value(true.into()));
            if let Some(parent) = ctx.ui.parent(ctx.id) {
                for sibling in ctx.ui.children(parent).to_vec() {
                    let same_group = ctx
                        .ui
                        .get::<RadioButton>(sibling)
                        .is_some_and(|r| r.checked && r.group == self.group);
                    if sibling != ctx.id && same_group {
                        let _ = ctx.ui.set_prop_quiet(sibling, "checked", false);
                        ctx.ui.queue_event(sibling, "on_change", EventArgs::Value(false.into()));
                    }
                }
            }
        }
        ctx.fire("on_click", EventArgs::None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        space_clicks(self, ctx, event);
    }

    fn auto_width(&self, ui: &aylis_core::Ui, id: aylis_core::ComponentId) -> Option<f32> {
        Some(caption_width(ui, id, &self.text, crate::looks::mark_size(ui.theme().look)))
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let side = crate::looks::mark_size(ctx.look());
        let b = mark_rect(ctx, side, side);
        crate::looks::paint_radio(canvas, ctx, b, self.checked);
        paint_caption(canvas, ctx, &self.text, side + GAP);
    }
}

// ---------------------------------------------------------------------------

/// Chave liga/desliga (como o `TSwitch`). A bolinha desliza quando muda; o trilho muda
/// de cor pela transição do tema.
pub struct Switch {
    pub base: ControlBase,
    pub text: String,
    pub checked: bool,
    toggled_at: f64,
}

impl Default for Switch {
    fn default() -> Self {
        Self { base: toggle_base(180.0), text: String::new(), checked: false, toggled_at: f64::NEG_INFINITY }
    }
}

impl Switch {
    const SLIDE: f64 = 0.18;

    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    fn sync(&mut self, _name: &str) {
        self.base.state_mut().checked = self.checked;
    }
}

impl Component for Switch {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "checked")
    }

    leaf_control!("Switch");

    fn checked_state(&self) -> Option<bool> {
        Some(self.checked)
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
    }

    properties! {
        text: String => "Appearance",
        checked: bool => "Data";
        events: on_change;
        on_set: sync
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        self.checked = !self.checked;
        self.sync("checked");
        self.toggled_at = ctx.ui.time();
        ctx.fire("on_change", EventArgs::Value(self.checked.into()));
        ctx.fire("on_click", EventArgs::None);
        ctx.repaint();
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        space_clicks(self, ctx, event);
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let (w, h) = crate::looks::switch_track(ctx.look());
        // Cabe na altura do controle (mantendo a proporção do trilho).
        let k = (ctx.rect.height / h).min(1.0);
        let (w, h) = (w * k, h * k);
        let track = mark_rect(ctx, w, h);

        // Posição da bolinha: desliza nos primeiros instantes depois da troca.
        let elapsed = ((ctx.time - self.toggled_at) / Self::SLIDE) as f32;
        let progress = if elapsed < 1.0 {
            ctx.request_frame();
            aylis_core::Easing::OutCubic.apply(elapsed.max(0.0))
        } else {
            1.0
        };
        // 0 = desligado (esquerda), 1 = ligado (direita).
        let position = if self.checked { progress } else { 1.0 - progress };
        crate::looks::paint_switch(canvas, ctx, track, position, self.checked);
        paint_caption(canvas, ctx, &self.text, w + GAP + 2.0);
    }
}
