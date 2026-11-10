//! Componentes e canvas de teste.
#![allow(dead_code)]

use aylis_core::*;

/// Controle simples, pinta um retângulo com o `fill` do tema.
#[derive(Default)]
pub struct Boxy {
    pub base: ControlBase,
    pub caption: String,
}

impl Component for Boxy {
    fn class_name(&self) -> &'static str {
        "Boxy"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn props(&self) -> &'static [PropInfo] {
        const PROPS: &[PropInfo] = &[PropInfo::new("caption", PropKind::String, "Appearance")];
        PROPS
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        match name {
            "caption" => Some(self.caption.clone().into()),
            _ => None,
        }
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        match name {
            "caption" => self.caption = value.to_str()?.to_string(),
            _ => return Err(Error::UnknownProperty(name.into())),
        }
        Ok(())
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        canvas.fill_rect(ctx.rect, ctx.style.radius, &Brush::Solid(ctx.style.fill));
        if !self.caption.is_empty() {
            canvas.draw_text(&self.caption, ctx.rect, &ctx.text_style());
        }
    }
}

pub fn boxy(w: f32, h: f32) -> Boxy {
    Boxy { base: ControlBase::new(w, h), caption: String::new() }
}

/// "Herda" de Boxy por composição: propriedade nova e pintura extra por cima.
#[derive(Default)]
pub struct Fancy {
    pub base: Boxy,
    pub glow: f32,
}

impl Component for Fancy {
    fn class_name(&self) -> &'static str {
        "Fancy"
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }

    fn props(&self) -> &'static [PropInfo] {
        const PROPS: &[PropInfo] = &[PropInfo::new("glow", PropKind::Float, "Appearance")];
        PROPS
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        (name == "glow").then(|| self.glow.into())
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        match name {
            "glow" => self.glow = value.to_f32()?,
            _ => return Err(Error::UnknownProperty(name.into())),
        }
        Ok(())
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.base.paint(canvas, ctx); // inherited
        canvas.stroke_rect(ctx.rect, 0.0, &Stroke::new(ctx.theme.color("accent"), self.glow));
    }
}

/// Componente não visual que referencia outro (como um DataSource -> DataSet).
#[derive(Default)]
pub struct Link {
    pub target: Option<ComponentId>,
}

impl Component for Link {
    fn class_name(&self) -> &'static str {
        "Link"
    }

    fn props(&self) -> &'static [PropInfo] {
        const PROPS: &[PropInfo] = &[
            PropInfo::new("target", PropKind::Component, "Data"),
            PropInfo::new("on_change", PropKind::Event, "Events"),
        ];
        PROPS
    }

    fn get_prop(&self, name: &str) -> Option<PropValue> {
        (name == "target").then(|| self.target.into())
    }

    fn set_prop(&mut self, name: &str, value: PropValue) -> Result<()> {
        match name {
            "target" => self.target = value.to_component()?,
            _ => return Err(Error::UnknownProperty(name.into())),
        }
        Ok(())
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.target == Some(freed) {
            self.target = None;
        }
    }
}

/// Operação gravada pelo canvas de teste, já em coordenadas absolutas.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Fill { rect: RectF, color: Color },
    Stroke { rect: RectF, width: f32 },
    Text { text: String, rect: RectF },
    Layer { opacity: f32 },
    Clip { rect: RectF },
}

/// Canvas que só grava o que foi pedido (com translações acumuladas).
#[derive(Default)]
pub struct RecordingCanvas {
    pub ops: Vec<Op>,
    offset: PointF,
    stack: Vec<PointF>,
}

impl RecordingCanvas {
    fn abs(&self, r: RectF) -> RectF {
        r.offset(self.offset.x, self.offset.y)
    }

    pub fn fills(&self) -> Vec<RectF> {
        self.ops
            .iter()
            .filter_map(|op| match op {
                Op::Fill { rect, .. } => Some(*rect),
                _ => None,
            })
            .collect()
    }

    pub fn balanced(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Canvas for RecordingCanvas {
    fn save(&mut self) {
        self.stack.push(self.offset);
    }

    fn restore(&mut self) {
        self.offset = self.stack.pop().expect("restore sem save");
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.offset = self.offset + PointF::new(dx, dy);
    }

    fn scale(&mut self, _sx: f32, _sy: f32) {}

    fn rotate(&mut self, _degrees: f32) {}

    fn clip_rect(&mut self, rect: RectF) {
        let rect = self.abs(rect);
        self.ops.push(Op::Clip { rect });
    }

    fn save_layer(&mut self, opacity: f32) {
        self.stack.push(self.offset);
        self.ops.push(Op::Layer { opacity });
    }

    fn fill_rect(&mut self, rect: RectF, _radius: f32, brush: &Brush) {
        let color = match brush {
            Brush::Solid(c) => *c,
            _ => Color::TRANSPARENT,
        };
        let rect = self.abs(rect);
        self.ops.push(Op::Fill { rect, color });
    }

    fn stroke_rect(&mut self, rect: RectF, _radius: f32, stroke: &Stroke) {
        let rect = self.abs(rect);
        self.ops.push(Op::Stroke { rect, width: stroke.width });
    }

    fn fill_ellipse(&mut self, _rect: RectF, _brush: &Brush) {}

    fn stroke_ellipse(&mut self, _rect: RectF, _stroke: &Stroke) {}

    fn draw_line(&mut self, _from: PointF, _to: PointF, _stroke: &Stroke) {}

    fn draw_text(&mut self, text: &str, rect: RectF, _style: &TextStyle) {
        let rect = self.abs(rect);
        self.ops.push(Op::Text { text: text.into(), rect });
    }

    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}
