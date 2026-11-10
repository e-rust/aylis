//! Formas geométricas (como as do FMX: `TEllipse`, `TCircle`, `TLine`, `TPie`, `TArc`,
//! `TPath`...), com preenchimento, borda e clique/hover que acertam a figura, não o
//! retângulo em volta: um clique no canto de um círculo passa para o que está embaixo.
//!
//! Cada forma monta o seu [`Path`] no retângulo do controle; o mesmo caminho é pintado e
//! usado no teste de clique ([`Component::contains_point`]). A rotação do controle vale
//! para todas (`rotation`).

use aylis_core::{Brush, Canvas, Color, Component, PaintCtx, Path, PointF, PropEnum, RectF, SizeF, Stroke, properties};

use crate::leaf_control;

/// Folga do clique em formas só de traço (linha, arco), em pixels.
const TOUCH: f32 = 4.0;

/// Pinta o caminho com o preenchimento (se `closed`) e a borda da forma.
fn paint_shape(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, path: &Path, closed: bool, fill: Option<Color>, stroke: Option<Color>, width: f32) {
    if closed {
        let fill = fill.unwrap_or_else(|| ctx.theme.color("primary"));
        if !fill.is_transparent() {
            canvas.fill_path(path, &Brush::Solid(fill));
        }
    }
    // Sem preenchimento (linha, arco), o traço usa a cor do texto do tema por padrão.
    let stroke = if closed { stroke } else { Some(stroke.unwrap_or_else(|| ctx.theme.color("text"))) };
    if let Some(color) = stroke.filter(|c| !c.is_transparent()) {
        if width > 0.0 {
            canvas.stroke_path(path, &Stroke::new(color, width));
        }
    }
}

/// O ponto acerta a figura: dentro do preenchimento (se fechada) ou perto do traço.
fn hit_shape(path: &Path, p: PointF, closed: bool, width: f32) -> bool {
    (closed && path.contains(p)) || path.distance_to(p) <= width / 2.0 + if closed { 0.5 } else { TOUCH }
}

fn local(size: SizeF) -> RectF {
    RectF::new(0.0, 0.0, size.width, size.height)
}

/// Ponto da elipse inscrita em `r` no ângulo `deg` (0 = em cima, sentido horário).
fn on_ellipse(r: RectF, deg: f32) -> PointF {
    let a = (deg - 90.0).to_radians();
    let c = r.center();
    PointF::new(c.x + r.width / 2.0 * a.cos(), c.y + r.height / 2.0 * a.sin())
}

/// As propriedades e o comportamento comuns: preenchimento, borda e o teste de clique.
macro_rules! shape {
    ($ty:ident, $name:literal, closed: $closed:expr, { $( $field:ident : $fty:ty $( as $kind:ident )? => $cat:literal ),* $(,)? }) => {
        impl Component for $ty {
            leaf_control!($name);

            properties! {
                fill: Option<Color> => "Appearance",
                stroke: Option<Color> => "Appearance",
                stroke_width: f32 => "Appearance",
                $( $field: $fty $( as $kind )? => $cat ),*
            }

            fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
                let closed: bool = $closed(self);
                paint_shape(canvas, ctx, &self.path(ctx.rect), closed, self.fill, self.stroke, self.stroke_width);
            }

            fn contains_point(&self, p: PointF, size: SizeF) -> bool {
                let closed: bool = $closed(self);
                hit_shape(&self.path(local(size)), p, closed, self.stroke_width)
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Ellipse / Circle

/// Elipse que ocupa o controle.
pub struct Ellipse {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(120.0, 80.0), fill: None, stroke: None, stroke_width: 1.0 }
    }
}

impl Ellipse {
    fn path(&self, r: RectF) -> Path {
        let mut p = Path::new();
        p.ellipse(r);
        p
    }
}

shape!(Ellipse, "Ellipse", closed: |_: &Ellipse| true, {});

/// Círculo: o maior que cabe, no centro do controle.
pub struct Circle {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(80.0, 80.0), fill: None, stroke: None, stroke_width: 1.0 }
    }
}

impl Circle {
    fn path(&self, r: RectF) -> Path {
        let side = r.width.min(r.height);
        let c = r.center();
        let mut p = Path::new();
        p.ellipse(RectF::new(c.x - side / 2.0, c.y - side / 2.0, side, side));
        p
    }
}

shape!(Circle, "Circle", closed: |_: &Circle| true, {});

// ---------------------------------------------------------------------------
// Line

/// Para onde a linha vai dentro do controle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineDirection {
    /// Do canto de cima à esquerda ao de baixo à direita.
    #[default]
    Down,
    /// Do canto de baixo à esquerda ao de cima à direita.
    Up,
    /// No meio, na horizontal.
    Horizontal,
    /// No meio, na vertical.
    Vertical,
}

impl PropEnum for LineDirection {
    const NAMES: &'static [&'static str] = &["down", "up", "horizontal", "vertical"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Down, Self::Up, Self::Horizontal, Self::Vertical].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Uma linha reta; o clique acerta perto dela (`stroke_width` mais uma folga).
pub struct Line {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub direction: LineDirection,
}

impl Default for Line {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(120.0, 40.0), fill: None, stroke: None, stroke_width: 2.0, direction: LineDirection::Down }
    }
}

impl Line {
    fn path(&self, r: RectF) -> Path {
        let (a, b) = match self.direction {
            LineDirection::Down => (PointF::new(r.x, r.y), PointF::new(r.right(), r.bottom())),
            LineDirection::Up => (PointF::new(r.x, r.bottom()), PointF::new(r.right(), r.y)),
            LineDirection::Horizontal => (PointF::new(r.x, r.center().y), PointF::new(r.right(), r.center().y)),
            LineDirection::Vertical => (PointF::new(r.center().x, r.y), PointF::new(r.center().x, r.bottom())),
        };
        let mut p = Path::new();
        p.move_to(a.x, a.y).line_to(b.x, b.y);
        p
    }
}

shape!(Line, "Line", closed: |_: &Line| false, { direction: LineDirection => "Appearance" });

// ---------------------------------------------------------------------------
// Polygon / Star

/// Polígono regular de `sides` lados, ou os pontos de `points` (`"0.5,0 1,1 0,1"`, de 0 a 1
/// na largura e na altura do controle).
pub struct Polygon {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub sides: u32,
    pub points: String,
}

impl Default for Polygon {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(100.0, 100.0), fill: None, stroke: None, stroke_width: 1.0, sides: 6, points: String::new() }
    }
}

impl Polygon {
    /// Os pontos de `points`, se houver pelo menos três válidos.
    fn custom(&self, r: RectF) -> Option<Vec<PointF>> {
        let pts: Vec<PointF> = self
            .points
            .split_whitespace()
            .filter_map(|pair| {
                let (x, y) = pair.split_once(',')?;
                Some(PointF::new(r.x + x.trim().parse::<f32>().ok()? * r.width, r.y + y.trim().parse::<f32>().ok()? * r.height))
            })
            .collect();
        (pts.len() >= 3).then_some(pts)
    }

    fn path(&self, r: RectF) -> Path {
        let pts = self.custom(r).unwrap_or_else(|| {
            let n = self.sides.max(3);
            (0..n).map(|i| on_ellipse(r, 360.0 * i as f32 / n as f32)).collect()
        });
        let mut p = Path::new();
        p.polygon(&pts, true);
        p
    }
}

shape!(Polygon, "Polygon", closed: |_: &Polygon| true, { sides: u32 => "Appearance", points: String => "Appearance" });

/// Estrela de `points` pontas; `inner` é o raio de dentro em relação ao de fora (0 a 1).
pub struct Star {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub points: u32,
    pub inner: f32,
}

impl Default for Star {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(100.0, 100.0), fill: None, stroke: None, stroke_width: 1.0, points: 5, inner: 0.45 }
    }
}

impl Star {
    fn path(&self, r: RectF) -> Path {
        let n = self.points.max(2);
        let c = r.center();
        let inner = RectF::new(
            c.x - r.width / 2.0 * self.inner.clamp(0.05, 1.0),
            c.y - r.height / 2.0 * self.inner.clamp(0.05, 1.0),
            r.width * self.inner.clamp(0.05, 1.0),
            r.height * self.inner.clamp(0.05, 1.0),
        );
        let step = 180.0 / n as f32;
        let pts: Vec<PointF> = (0..n * 2).map(|i| on_ellipse(if i % 2 == 0 { r } else { inner }, step * i as f32)).collect();
        let mut p = Path::new();
        p.polygon(&pts, true);
        p
    }
}

shape!(Star, "Star", closed: |_: &Star| true, { points: u32 => "Appearance", inner: f32 => "Appearance" });

// ---------------------------------------------------------------------------
// Pie / Arc

/// Fatia de pizza da elipse do controle, de `start_angle` a `end_angle` (graus; 0 = em
/// cima, sentido horário, como num relógio).
pub struct Pie {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}

impl Default for Pie {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(100.0, 100.0), fill: None, stroke: None, stroke_width: 1.0, start_angle: 0.0, end_angle: 270.0 }
    }
}

/// O arco da elipse de `r` entre os ângulos (0 = em cima, horário).
fn arc_path(path: &mut Path, r: RectF, start: f32, end: f32) {
    let sweep = if end >= start { end - start } else { 360.0 - (start - end) };
    path.arc(r.center(), r.width / 2.0, r.height / 2.0, start - 90.0, sweep.min(360.0));
}

impl Pie {
    fn path(&self, r: RectF) -> Path {
        let c = r.center();
        let mut p = Path::new();
        p.move_to(c.x, c.y);
        arc_path(&mut p, r, self.start_angle, self.end_angle);
        p.close();
        p
    }
}

shape!(Pie, "Pie", closed: |_: &Pie| true, { start_angle: f32 as Angle => "Appearance", end_angle: f32 as Angle => "Appearance" });

/// Arco (só o traço) da elipse do controle, de `start_angle` a `end_angle`.
pub struct Arc {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}

impl Default for Arc {
    fn default() -> Self {
        Self { base: aylis_core::ControlBase::new(100.0, 100.0), fill: None, stroke: None, stroke_width: 4.0, start_angle: 0.0, end_angle: 270.0 }
    }
}

impl Arc {
    fn path(&self, r: RectF) -> Path {
        // O traço fica todo dentro do controle.
        let half = self.stroke_width / 2.0;
        let mut p = Path::new();
        arc_path(&mut p, RectF::new(r.x + half, r.y + half, (r.width - self.stroke_width).max(0.0), (r.height - self.stroke_width).max(0.0)), self.start_angle, self.end_angle);
        p
    }
}

shape!(Arc, "Arc", closed: |_: &Arc| false, { start_angle: f32 as Angle => "Appearance", end_angle: f32 as Angle => "Appearance" });

// ---------------------------------------------------------------------------
// Path

/// Um caminho SVG qualquer (`data`, o atributo `d`), esticado para o controle
/// (`keep_aspect`: sem distorcer, centralizado). Figuras abertas só levam o traço.
pub struct PathShape {
    pub base: aylis_core::ControlBase,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f32,
    pub data: String,
    pub keep_aspect: bool,
}

impl Default for PathShape {
    fn default() -> Self {
        Self {
            base: aylis_core::ControlBase::new(100.0, 100.0),
            fill: None,
            stroke: None,
            stroke_width: 1.0,
            // Um coração.
            data: "M12 21 C12 21 3 14 3 8.5 A4.5 4.5 0 0 1 12 6 A4.5 4.5 0 0 1 21 8.5 C21 14 12 21 12 21 Z".into(),
            keep_aspect: true,
        }
    }
}

impl PathShape {
    fn path(&self, r: RectF) -> Path {
        let Some(source) = Path::from_svg(&self.data) else { return Path::new() };
        let b = source.bounds();
        if b.width <= 0.0 && b.height <= 0.0 {
            return source;
        }
        let (mut sx, mut sy) = (r.width / b.width.max(f32::EPSILON), r.height / b.height.max(f32::EPSILON));
        if self.keep_aspect {
            let s = sx.min(sy);
            (sx, sy) = (s, s);
        }
        let (w, h) = (b.width * sx, b.height * sy);
        let (ox, oy) = (r.x + (r.width - w) / 2.0, r.y + (r.height - h) / 2.0);
        source.map_points(|p| PointF::new(ox + (p.x - b.x) * sx, oy + (p.y - b.y) * sy))
    }

    /// Fechada se alguma figura do caminho fecha (`Z`).
    fn closed(&self) -> bool {
        self.data.contains(['Z', 'z'])
    }
}

shape!(PathShape, "Path", closed: |s: &PathShape| s.closed(), { data: String => "Appearance", keep_aspect: bool => "Appearance" });
