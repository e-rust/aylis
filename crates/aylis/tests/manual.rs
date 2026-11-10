//! O exemplo de docs/criar-componente.md, compilado e testado (se o manual ficar velho,
//! este teste quebra).

use aylis::core::properties;
use aylis::prelude::*;

pub struct StarRating {
    pub base: ControlBase,
    /// Nota de 0 a `max`.
    pub value: i64,
    pub max: i64,
    pub read_only: bool,
    hover: Option<i64>,
}

impl Default for StarRating {
    fn default() -> Self {
        let mut base = ControlBase::new(140.0, 28.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self { base, value: 3, max: 5, read_only: false, hover: None }
    }
}

impl Component for StarRating {
    fn class_name(&self) -> &'static str {
        "StarRating"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        value: i64 => "Data",
        max: i64 => "Behavior",
        read_only: bool => "Behavior";
        events: on_change;
        on_set: after_set
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let n = self.max.max(1) as f32;
        let side = (ctx.rect.width / n).min(ctx.rect.height);
        let shown = self.hover.unwrap_or(self.value);
        for i in 0..self.max {
            let r = RectF::new(i as f32 * side, (ctx.rect.height - side) / 2.0, side, side);
            let lit = i < shown;
            let color = if lit { ctx.theme.color("warning") } else { ctx.theme.color("track") };
            ctx.draw_icon(canvas, "star", r.deflate(Bounds::all(2.0)), color);
        }
        if ctx.state.focused {
            canvas.stroke_rect(ctx.rect, 4.0, &Stroke::new(ctx.theme.color("focus_ring"), 2.0));
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if self.read_only {
            return;
        }
        let side = self.base.bounds().width / self.max.max(1) as f32;
        let h = Some(((e.position.x / side).floor() as i64 + 1).clamp(0, self.max));
        if h != self.hover {
            self.hover = h;
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hover = None;
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, e: &PointerEvent) {
        if self.read_only || e.button != PointerButton::Primary {
            return;
        }
        if let Some(v) = self.hover {
            self.set(ctx, v);
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, e: &KeyEvent) {
        let v = match e.key {
            Key::Right | Key::Up => self.value + 1,
            Key::Left | Key::Down => self.value - 1,
            _ => return,
        };
        self.set(ctx, v);
        ctx.set_handled();
    }
}

impl StarRating {
    fn after_set(&mut self, _name: &str) {
        self.value = self.value.clamp(0, self.max.max(0));
    }

    fn set(&mut self, ctx: &mut EventCtx<'_>, v: i64) {
        let v = v.clamp(0, self.max);
        if v != self.value {
            self.value = v;
            ctx.fire("on_change", EventArgs::Value(v.into()));
        }
        ctx.repaint();
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Shape {
    #[default]
    Star,
    Heart,
}

impl PropEnum for Shape {
    const NAMES: &'static [&'static str] = &["star", "heart"];
    fn from_index(i: usize) -> Option<Self> {
        [Self::Star, Self::Heart].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

pub struct GlowButton {
    pub button: Button,
    pub glow: Color,
}

impl Default for GlowButton {
    fn default() -> Self {
        Self { button: Button::new("Glow"), glow: Color::rgb(0x60, 0xA5, 0xFA) }
    }
}

impl Component for GlowButton {
    fn class_name(&self) -> &'static str {
        "BotaoGlow"
    }
    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.button)
    }
    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.button)
    }

    properties! { glow: Color => "Appearance" }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if ctx.state.hover {
            canvas.draw_shadow(ctx.rect, ctx.style.radius, &Shadow::new(self.glow, 16.0, 0.0, 0.0));
        }
        self.button.paint(canvas, ctx);
    }
}

#[test]
fn stars() {
    let mut ui = Ui::new();
    ui.register_class::<StarRating>("Teste");
    let f = ui.add_root(aylis::controls::Form::new("t", 300.0, 100.0));
    let s = ui.create("StarRating", Some(f)).unwrap();
    ui.set_prop(s, "value", 4i64).unwrap();
    assert_eq!(ui.get::<StarRating>(s).unwrap().value, 4);
    ui.set_prop(s, "value", 99i64).unwrap();
    assert_eq!(ui.get::<StarRating>(s).unwrap().value, 5, "limitado pelo max");
}

#[test]
fn glow_inherits_from_button() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    ui.register_class::<GlowButton>("Teste");
    let f = ui.add_root(aylis::controls::Form::new("t", 300.0, 100.0));
    let g = ui.create("BotaoGlow", Some(f)).unwrap();
    ui.set_prop(g, "text", "Ok").unwrap(); // propriedade do Button, pela composição
    ui.set_prop(g, "glow", Color::rgb(1, 2, 3)).unwrap();
    assert_eq!(ui.get::<GlowButton>(g).unwrap().button.text, "Ok");
    // subtipo do Button também vale para a classe derivada pelo estilo
    ui.set_prop(g, "style", "danger").unwrap();
    // e o subtipo de usuário do manual
    let mut theme = ui.theme().clone();
    theme.set_style("Button.aviso", Style::new(StyleProps::new().fill("warning").text("on_primary")).hover(StyleProps::new().opacity(0.9)));
    ui.set_theme(theme);
    ui.register_variant("Button", "aviso", "Aviso", vec![("icon", PropValue::String("warning".into()))]);
    let b = ui.create("Button.aviso", Some(f)).unwrap();
    assert_eq!(ui.get::<Button>(b).unwrap().icon, "warning");
    let _ = Shape::default();
}
