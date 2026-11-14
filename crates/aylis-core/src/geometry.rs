//! Tipos geométricos: ponto, tamanho, retângulo e as medidas por lado.
//!
//! Todas as medidas são em unidades lógicas; a escala de DPI é aplicada pelo canvas.

use std::ops::{Add, Sub};

/// Um ponto.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

impl PointF {
    pub const ZERO: PointF = PointF::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for PointF {
    type Output = PointF;
    fn add(self, other: PointF) -> PointF {
        PointF::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for PointF {
    type Output = PointF;
    fn sub(self, other: PointF) -> PointF {
        PointF::new(self.x - other.x, self.y - other.y)
    }
}

/// Uma largura e uma altura.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SizeF {
    pub width: f32,
    pub height: f32,
}

impl SizeF {
    pub const ZERO: SizeF = SizeF::new(0.0, 0.0);

    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// Um retângulo definido por origem e tamanho.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectF {
    pub const ZERO: RectF = RectF::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Retângulo na origem com o tamanho dado.
    pub const fn from_size(size: SizeF) -> Self {
        Self::new(0.0, 0.0, size.width, size.height)
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn origin(&self) -> PointF {
        PointF::new(self.x, self.y)
    }

    pub fn size(&self) -> SizeF {
        SizeF::new(self.width, self.height)
    }

    pub fn center(&self) -> PointF {
        PointF::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }

    pub fn contains(&self, p: PointF) -> bool {
        p.x >= self.x && p.x < self.right() && p.y >= self.y && p.y < self.bottom()
    }

    /// `other` cabe inteiro dentro deste.
    /// Menor retângulo que contém os dois.
    pub fn union(&self, other: RectF) -> RectF {
        let (x, y) = (self.x.min(other.x), self.y.min(other.y));
        let (r, b) = (self.right().max(other.right()), self.bottom().max(other.bottom()));
        RectF::new(x, y, r - x, b - y)
    }

    pub fn contains_rect(&self, other: RectF) -> bool {
        other.x >= self.x && other.y >= self.y && other.right() <= self.right() && other.bottom() <= self.bottom()
    }

    /// Os dois retângulos se sobrepõem.
    pub fn intersects(&self, other: RectF) -> bool {
        self.x < other.right() && other.x < self.right() && self.y < other.bottom() && other.y < self.bottom()
    }

    /// Retângulo entre dois pontos, em qualquer ordem (seleção por área).
    pub fn from_points(a: PointF, b: PointF) -> Self {
        Self::new(a.x.min(b.x), a.y.min(b.y), (a.x - b.x).abs(), (a.y - b.y).abs())
    }

    pub fn offset(self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    /// Aumenta (ou diminui, com valor negativo) o retângulo em todos os lados.
    pub fn inflate(self, d: f32) -> Self {
        Self::new(self.x - d, self.y - d, self.width + 2.0 * d, self.height + 2.0 * d)
    }

    /// Retira as bordas dadas (como aplicar um padding). Nunca fica com tamanho negativo.
    pub fn deflate(self, b: Bounds) -> Self {
        Self::new(
            self.x + b.left,
            self.y + b.top,
            (self.width - b.horizontal()).max(0.0),
            (self.height - b.vertical()).max(0.0),
        )
    }

    pub fn intersect(self, other: RectF) -> Option<RectF> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let r = self.right().min(other.right());
        let b = self.bottom().min(other.bottom());
        (r > x && b > y).then(|| RectF::new(x, y, r - x, b - y))
    }
}

/// Quatro medidas, uma por lado: usado em `margins` e `padding`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Bounds {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Bounds {
    pub const ZERO: Bounds = Bounds::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self { left, top, right, bottom }
    }

    /// O mesmo valor nos quatro lados.
    pub const fn all(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    /// `h` à esquerda e à direita, `v` em cima e embaixo.
    pub const fn symmetric(h: f32, v: f32) -> Self {
        Self::new(h, v, h, v)
    }

    /// Os quatro lados multiplicados por `factor`.
    pub fn scaled(self, factor: f32) -> Self {
        Self::new(self.left * factor, self.top * factor, self.right * factor, self.bottom * factor)
    }

    /// Soma de esquerda + direita.
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// Soma de cima + embaixo.
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl From<f32> for Bounds {
    fn from(v: f32) -> Self {
        Bounds::all(v)
    }
}

impl From<(f32, f32)> for Bounds {
    fn from((h, v): (f32, f32)) -> Self {
        Bounds::symmetric(h, v)
    }
}

impl From<[f32; 4]> for Bounds {
    fn from([l, t, r, b]: [f32; 4]) -> Self {
        Bounds::new(l, t, r, b)
    }
}
