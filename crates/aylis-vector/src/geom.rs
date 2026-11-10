//! Transformação afim (a matriz 2×3 do SVG e do After Effects) e utilidades de caminho.

use aylis_core::{Path, PathCmd, PointF, RectF};

/// `x' = a·x + c·y + e`, `y' = b·x + d·y + f` (a ordem do `matrix(a b c d e f)` do SVG).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Default for Affine {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine {
    pub const IDENTITY: Affine = Affine { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self { a, b, c, d, e, f }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, x, y)
    }

    pub fn scale(x: f32, y: f32) -> Self {
        Self::new(x, 0.0, 0.0, y, 0.0, 0.0)
    }

    /// Gira `degrees` no sentido horário da tela (y para baixo).
    pub fn rotate(degrees: f32) -> Self {
        let (s, c) = degrees.to_radians().sin_cos();
        Self::new(c, s, -s, c, 0.0, 0.0)
    }

    pub fn skew_x(degrees: f32) -> Self {
        Self::new(1.0, 0.0, degrees.to_radians().tan(), 1.0, 0.0, 0.0)
    }

    pub fn skew_y(degrees: f32) -> Self {
        Self::new(1.0, degrees.to_radians().tan(), 0.0, 1.0, 0.0, 0.0)
    }

    /// `self · other`: aplica `other` primeiro.
    pub fn then(self, other: Affine) -> Affine {
        let m = self;
        let o = other;
        Affine {
            a: m.a * o.a + m.c * o.b,
            b: m.b * o.a + m.d * o.b,
            c: m.a * o.c + m.c * o.d,
            d: m.b * o.c + m.d * o.d,
            e: m.a * o.e + m.c * o.f + m.e,
            f: m.b * o.e + m.d * o.f + m.f,
        }
    }

    pub fn apply(&self, p: PointF) -> PointF {
        PointF::new(self.a * p.x + self.c * p.y + self.e, self.b * p.x + self.d * p.y + self.f)
    }

    /// Quanto a transformação aumenta uma medida (largura de traço, raio).
    pub fn scale_factor(&self) -> f32 {
        (self.a * self.d - self.b * self.c).abs().sqrt()
    }

    pub fn path(&self, path: &Path) -> Path {
        path.map_points(|p| self.apply(p))
    }

    /// Leva o retângulo `from` (conteúdo) para `to` (tela).
    pub fn rect_to_rect(from: RectF, to: RectF) -> Affine {
        let sx = if from.width > 0.0 { to.width / from.width } else { 1.0 };
        let sy = if from.height > 0.0 { to.height / from.height } else { 1.0 };
        Affine::new(sx, 0.0, 0.0, sy, to.x - from.x * sx, to.y - from.y * sy)
    }
}

/// O retângulo transformado, como caminho (para recortes).
pub fn rect_path(m: &Affine, r: RectF) -> Path {
    let mut p = Path::new();
    let pts = [PointF::new(r.x, r.y), PointF::new(r.right(), r.y), PointF::new(r.right(), r.bottom()), PointF::new(r.x, r.bottom())];
    let pts: Vec<PointF> = pts.iter().map(|q| m.apply(*q)).collect();
    p.polygon(&pts, true);
    p
}

/// Parte de cada figura do caminho, de `start` a `end` (0 a 1 do comprimento, com
/// `offset` somado e dando a volta). É o "Trim Paths" do After Effects: o resultado são
/// polilinhas abertas.
pub fn trim(path: &Path, start: f32, end: f32, offset: f32) -> Path {
    let (mut s, mut e) = (start.min(end), start.max(end));
    if e - s >= 0.9999 {
        return path.clone();
    }
    if e - s <= 0.0001 {
        return Path::new();
    }
    s += offset;
    e += offset;
    let shift = s.floor();
    s -= shift;
    e -= shift;
    let mut out = Path::new();
    for (mut points, closed) in path.flatten(24) {
        if closed && points.len() > 1 {
            points.push(points[0]);
        }
        let lengths: Vec<f32> = points.windows(2).map(|w| dist(w[0], w[1])).collect();
        let total: f32 = lengths.iter().sum();
        if total <= 0.0 {
            continue;
        }
        // O pedaço [s, e]; passando de 1, continua do começo.
        let mut pieces = vec![(s, e.min(1.0))];
        if e > 1.0 {
            pieces.push((0.0, e - 1.0));
        }
        for (a, b) in pieces {
            let (from, to) = (a * total, b * total);
            let mut acc = 0.0;
            let mut started = false;
            for (i, len) in lengths.iter().enumerate() {
                let (p0, p1) = (points[i], points[i + 1]);
                let (seg_start, seg_end) = (acc, acc + len);
                acc = seg_end;
                if seg_end < from || seg_start > to || *len <= 0.0 {
                    continue;
                }
                let at = |d: f32| {
                    let t = ((d - seg_start) / len).clamp(0.0, 1.0);
                    PointF::new(p0.x + (p1.x - p0.x) * t, p0.y + (p1.y - p0.y) * t)
                };
                if !started {
                    let p = at(from.max(seg_start));
                    out.move_to(p.x, p.y);
                    started = true;
                }
                let p = at(to.min(seg_end));
                out.line_to(p.x, p.y);
            }
        }
    }
    out
}

pub fn dist(a: PointF, b: PointF) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// O caminho tem algo para desenhar.
pub fn has_figures(path: &Path) -> bool {
    path.cmds.iter().any(|c| !matches!(c, PathCmd::MoveTo(_) | PathCmd::Close))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_and_apply() {
        let m = Affine::translate(10.0, 0.0).then(Affine::scale(2.0, 2.0));
        assert_eq!(m.apply(PointF::new(1.0, 1.0)), PointF::new(12.0, 2.0));
        let r = Affine::rotate(90.0).apply(PointF::new(1.0, 0.0));
        assert!((r.x).abs() < 1e-5 && (r.y - 1.0).abs() < 1e-5, "{r:?}");
        assert!((Affine::scale(2.0, 8.0).scale_factor() - 4.0).abs() < 1e-5);
    }

    #[test]
    fn trim_keeps_part_of_a_line() {
        let mut p = Path::new();
        p.move_to(0.0, 0.0).line_to(100.0, 0.0);
        let t = trim(&p, 0.25, 0.5, 0.0);
        let b = t.bounds();
        assert!((b.x - 25.0).abs() < 0.5 && (b.right() - 50.0).abs() < 0.5, "{b:?}");
        assert!(trim(&p, 0.3, 0.3, 0.0).is_empty());
    }
}
