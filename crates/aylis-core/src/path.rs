//! Caminhos vetoriais (o `TPathData` do FMX): retas, curvas e arcos, para ícones, cabos,
//! gráficos e formas livres. Lê o atributo `d` do SVG.

use crate::geometry::{PointF, RectF};

/// Um comando do caminho (coordenadas absolutas).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathCmd {
    MoveTo(PointF),
    LineTo(PointF),
    QuadTo(PointF, PointF),
    CubicTo(PointF, PointF, PointF),
    Close,
}

/// Um caminho: uma ou mais figuras, abertas ou fechadas.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Path {
    pub cmds: Vec<PathCmd>,
    /// Regra de preenchimento "par-ímpar" (`fill-rule="evenodd"` do SVG); o padrão é
    /// "não zero".
    pub even_odd: bool,
}

fn pt(x: f32, y: f32) -> PointF {
    PointF::new(x, y)
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }

    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.cmds.push(PathCmd::MoveTo(pt(x, y)));
        self
    }

    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.cmds.push(PathCmd::LineTo(pt(x, y)));
        self
    }

    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) -> &mut Self {
        self.cmds.push(PathCmd::QuadTo(pt(cx, cy), pt(x, y)));
        self
    }

    pub fn cubic_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32) -> &mut Self {
        self.cmds.push(PathCmd::CubicTo(pt(c1x, c1y), pt(c2x, c2y), pt(x, y)));
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.cmds.push(PathCmd::Close);
        self
    }

    /// Arco de elipse em volta de `center`, de `start` a `start + sweep` (graus; 0 aponta
    /// para a direita e o ângulo cresce no sentido horário, como na tela). Liga com uma
    /// reta ao ponto atual (ou começa uma figura, se o caminho estiver vazio).
    pub fn arc(&mut self, center: PointF, rx: f32, ry: f32, start: f32, sweep: f32) -> &mut Self {
        let (a0, da) = (start.to_radians(), sweep.to_radians());
        let first = pt(center.x + rx * a0.cos(), center.y + ry * a0.sin());
        if self.current().is_none() {
            self.cmds.push(PathCmd::MoveTo(first));
        } else {
            self.cmds.push(PathCmd::LineTo(first));
        }
        self.arc_segments(center, rx, ry, 0.0, a0, da);
        self
    }

    /// Retângulo (de cantos arredondados com `radius` > 0).
    pub fn rect(&mut self, r: RectF, radius: f32) -> &mut Self {
        let radius = radius.min(r.width / 2.0).min(r.height / 2.0).max(0.0);
        if radius <= 0.0 {
            self.move_to(r.x, r.y).line_to(r.right(), r.y).line_to(r.right(), r.bottom()).line_to(r.x, r.bottom()).close();
            return self;
        }
        let k = radius * 0.552_284_8;
        let (l, t, rr, b) = (r.x, r.y, r.right(), r.bottom());
        self.move_to(l + radius, t)
            .line_to(rr - radius, t)
            .cubic_to(rr - radius + k, t, rr, t + radius - k, rr, t + radius)
            .line_to(rr, b - radius)
            .cubic_to(rr, b - radius + k, rr - radius + k, b, rr - radius, b)
            .line_to(l + radius, b)
            .cubic_to(l + radius - k, b, l, b - radius + k, l, b - radius)
            .line_to(l, t + radius)
            .cubic_to(l, t + radius - k, l + radius - k, t, l + radius, t)
            .close()
    }

    /// Elipse inscrita no retângulo.
    pub fn ellipse(&mut self, r: RectF) -> &mut Self {
        let (cx, cy, rx, ry) = (r.x + r.width / 2.0, r.y + r.height / 2.0, r.width / 2.0, r.height / 2.0);
        let (kx, ky) = (rx * 0.552_284_8, ry * 0.552_284_8);
        self.move_to(cx + rx, cy)
            .cubic_to(cx + rx, cy + ky, cx + kx, cy + ry, cx, cy + ry)
            .cubic_to(cx - kx, cy + ry, cx - rx, cy + ky, cx - rx, cy)
            .cubic_to(cx - rx, cy - ky, cx - kx, cy - ry, cx, cy - ry)
            .cubic_to(cx + kx, cy - ry, cx + rx, cy - ky, cx + rx, cy)
            .close()
    }

    /// Polígono (ou linha quebrada, se `closed` for falso).
    pub fn polygon(&mut self, points: &[PointF], closed: bool) -> &mut Self {
        for (i, p) in points.iter().enumerate() {
            if i == 0 { self.move_to(p.x, p.y) } else { self.line_to(p.x, p.y) };
        }
        if closed && !points.is_empty() {
            self.close();
        }
        self
    }

    /// Último ponto do caminho.
    pub fn current(&self) -> Option<PointF> {
        let mut start = None;
        let mut cur = None;
        for c in &self.cmds {
            match *c {
                PathCmd::MoveTo(p) => {
                    start = Some(p);
                    cur = Some(p);
                }
                PathCmd::LineTo(p) | PathCmd::QuadTo(_, p) | PathCmd::CubicTo(_, _, p) => cur = Some(p),
                PathCmd::Close => cur = start,
            }
        }
        cur
    }

    /// Arcos em pedaços de no máximo 90°, cada um como uma curva cúbica.
    fn arc_segments(&mut self, c: PointF, rx: f32, ry: f32, phi: f32, a0: f32, da: f32) {
        let n = (da.abs() / std::f32::consts::FRAC_PI_2).ceil().max(1.0) as usize;
        let step = da / n as f32;
        let k = 4.0 / 3.0 * (step / 4.0).tan();
        let (sp, cp) = phi.sin_cos();
        let map = |x: f32, y: f32| pt(c.x + x * rx * cp - y * ry * sp, c.y + x * rx * sp + y * ry * cp);
        let mut a = a0;
        for _ in 0..n {
            let b = a + step;
            let (sa, ca) = a.sin_cos();
            let (sb, cb) = b.sin_cos();
            let p1 = map(ca - k * sa, sa + k * ca);
            let p2 = map(cb + k * sb, sb - k * cb);
            let p3 = map(cb, sb);
            self.cmds.push(PathCmd::CubicTo(p1, p2, p3));
            a = b;
        }
    }

    /// Lê o atributo `d` do SVG (`M L H V C S Q T A Z`, maiúsculas e minúsculas).
    /// `None` se estiver mal formado.
    pub fn from_svg(d: &str) -> Option<Path> {
        let mut path = Path::new();
        let mut tokens = SvgTokens { s: d.as_bytes(), i: 0 };
        let mut cmd = b'M';
        let mut cur = pt(0.0, 0.0);
        let mut start = cur;
        // Último ponto de controle (para S e T).
        let mut last_cubic: Option<PointF> = None;
        let mut last_quad: Option<PointF> = None;
        loop {
            tokens.skip_separators();
            if tokens.done() {
                break;
            }
            if let Some(c) = tokens.command() {
                cmd = c;
            } else if cmd == b'Z' || cmd == b'z' {
                return None; // números depois de Z
            }
            let rel = cmd.is_ascii_lowercase();
            let base = if rel { cur } else { pt(0.0, 0.0) };
            let mut num = || tokens.number();
            match cmd.to_ascii_uppercase() {
                b'M' => {
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::MoveTo(p));
                    cur = p;
                    start = p;
                    // Pares seguintes são retas.
                    cmd = if rel { b'l' } else { b'L' };
                    last_cubic = None;
                    last_quad = None;
                }
                b'L' => {
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::LineTo(p));
                    cur = p;
                    last_cubic = None;
                    last_quad = None;
                }
                b'H' => {
                    let x = num()? + if rel { cur.x } else { 0.0 };
                    cur = pt(x, cur.y);
                    path.cmds.push(PathCmd::LineTo(cur));
                    last_cubic = None;
                    last_quad = None;
                }
                b'V' => {
                    let y = num()? + if rel { cur.y } else { 0.0 };
                    cur = pt(cur.x, y);
                    path.cmds.push(PathCmd::LineTo(cur));
                    last_cubic = None;
                    last_quad = None;
                }
                b'C' => {
                    let c1 = pt(base.x + num()?, base.y + num()?);
                    let c2 = pt(base.x + num()?, base.y + num()?);
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::CubicTo(c1, c2, p));
                    last_cubic = Some(c2);
                    last_quad = None;
                    cur = p;
                }
                b'S' => {
                    let c1 = last_cubic.map_or(cur, |c| pt(2.0 * cur.x - c.x, 2.0 * cur.y - c.y));
                    let c2 = pt(base.x + num()?, base.y + num()?);
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::CubicTo(c1, c2, p));
                    last_cubic = Some(c2);
                    last_quad = None;
                    cur = p;
                }
                b'Q' => {
                    let c = pt(base.x + num()?, base.y + num()?);
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::QuadTo(c, p));
                    last_quad = Some(c);
                    last_cubic = None;
                    cur = p;
                }
                b'T' => {
                    let c = last_quad.map_or(cur, |q| pt(2.0 * cur.x - q.x, 2.0 * cur.y - q.y));
                    let p = pt(base.x + num()?, base.y + num()?);
                    path.cmds.push(PathCmd::QuadTo(c, p));
                    last_quad = Some(c);
                    last_cubic = None;
                    cur = p;
                }
                b'A' => {
                    let (rx, ry, rot) = (num()?, num()?, num()?);
                    let large = tokens.flag()?;
                    let sweep = tokens.flag()?;
                    let p = pt(base.x + tokens.number()?, base.y + tokens.number()?);
                    path.svg_arc(cur, p, rx, ry, rot, large, sweep);
                    last_cubic = None;
                    last_quad = None;
                    cur = p;
                }
                b'Z' => {
                    path.cmds.push(PathCmd::Close);
                    cur = start;
                    last_cubic = None;
                    last_quad = None;
                }
                _ => return None,
            }
        }
        Some(path)
    }

    /// Arco do SVG (por pontas) convertido para o centro e daí para cúbicas.
    #[allow(clippy::too_many_arguments)]
    fn svg_arc(&mut self, from: PointF, to: PointF, rx: f32, ry: f32, rotation: f32, large: bool, sweep: bool) {
        let (mut rx, mut ry) = (rx.abs(), ry.abs());
        if rx == 0.0 || ry == 0.0 || from == to {
            self.cmds.push(PathCmd::LineTo(to));
            return;
        }
        let phi = rotation.to_radians();
        let (sp, cp) = phi.sin_cos();
        let dx = (from.x - to.x) / 2.0;
        let dy = (from.y - to.y) / 2.0;
        let x1 = cp * dx + sp * dy;
        let y1 = -sp * dx + cp * dy;
        let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
        if lambda > 1.0 {
            rx *= lambda.sqrt();
            ry *= lambda.sqrt();
        }
        let num = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
        let den = rx * rx * y1 * y1 + ry * ry * x1 * x1;
        let mut coef = (num / den).max(0.0).sqrt();
        if large == sweep {
            coef = -coef;
        }
        let cx1 = coef * rx * y1 / ry;
        let cy1 = -coef * ry * x1 / rx;
        let center = pt(cp * cx1 - sp * cy1 + (from.x + to.x) / 2.0, sp * cx1 + cp * cy1 + (from.y + to.y) / 2.0);
        let angle = |ux: f32, uy: f32, vx: f32, vy: f32| {
            let a = (ux * vy - uy * vx).atan2(ux * vx + uy * vy);
            if a.is_nan() { 0.0 } else { a }
        };
        let a0 = angle(1.0, 0.0, (x1 - cx1) / rx, (y1 - cy1) / ry);
        let mut da = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
        let tau = std::f32::consts::TAU;
        if !sweep && da > 0.0 {
            da -= tau;
        } else if sweep && da < 0.0 {
            da += tau;
        }
        self.arc_segments(center, rx, ry, phi, a0, da);
        // Garante a ponta exata.
        if let Some(PathCmd::CubicTo(_, _, p)) = self.cmds.last_mut() {
            *p = to;
        }
    }

    /// O mesmo caminho escalado e deslocado (`p * scale + offset`).
    pub fn transformed(&self, scale: f32, offset: PointF) -> Path {
        self.map_points(|p| pt(p.x * scale + offset.x, p.y * scale + offset.y))
    }

    /// Acrescenta as figuras de outro caminho.
    pub fn append(&mut self, other: &Path) -> &mut Self {
        self.cmds.extend_from_slice(&other.cmds);
        self
    }

    /// O mesmo caminho com cada ponto passado por `f` (uma transformação qualquer).
    pub fn map_points(&self, f: impl Fn(PointF) -> PointF) -> Path {
        let cmds = self
            .cmds
            .iter()
            .map(|c| match *c {
                PathCmd::MoveTo(p) => PathCmd::MoveTo(f(p)),
                PathCmd::LineTo(p) => PathCmd::LineTo(f(p)),
                PathCmd::QuadTo(a, p) => PathCmd::QuadTo(f(a), f(p)),
                PathCmd::CubicTo(a, b, p) => PathCmd::CubicTo(f(a), f(b), f(p)),
                PathCmd::Close => PathCmd::Close,
            })
            .collect();
        Path { cmds, even_odd: self.even_odd }
    }

    /// Retângulo que contém todos os pontos (inclusive os de controle).
    pub fn bounds(&self) -> RectF {
        let mut min = pt(f32::MAX, f32::MAX);
        let mut max = pt(f32::MIN, f32::MIN);
        let mut add = |p: PointF| {
            min = pt(min.x.min(p.x), min.y.min(p.y));
            max = pt(max.x.max(p.x), max.y.max(p.y));
        };
        for c in &self.cmds {
            match *c {
                PathCmd::MoveTo(p) | PathCmd::LineTo(p) => add(p),
                PathCmd::QuadTo(a, p) => {
                    add(a);
                    add(p);
                }
                PathCmd::CubicTo(a, b, p) => {
                    add(a);
                    add(b);
                    add(p);
                }
                PathCmd::Close => {}
            }
        }
        if min.x > max.x {
            return RectF::default();
        }
        RectF::new(min.x, min.y, max.x - min.x, max.y - min.y)
    }

    /// As figuras como linhas quebradas (curvas divididas em `segments` retas cada):
    /// `(pontos, fechada)`. Usado por canvases sem curvas e para testes de clique.
    pub fn flatten(&self, segments: usize) -> Vec<(Vec<PointF>, bool)> {
        let segments = segments.max(1);
        let mut out: Vec<(Vec<PointF>, bool)> = Vec::new();
        let mut cur: Vec<PointF> = Vec::new();
        let mut last = pt(0.0, 0.0);
        for c in &self.cmds {
            match *c {
                PathCmd::MoveTo(p) => {
                    if cur.len() > 1 {
                        out.push((std::mem::take(&mut cur), false));
                    }
                    cur = vec![p];
                    last = p;
                }
                PathCmd::LineTo(p) => {
                    if cur.is_empty() {
                        cur.push(last);
                    }
                    cur.push(p);
                    last = p;
                }
                PathCmd::QuadTo(a, p) => {
                    if cur.is_empty() {
                        cur.push(last);
                    }
                    for i in 1..=segments {
                        let t = i as f32 / segments as f32;
                        let u = 1.0 - t;
                        cur.push(pt(u * u * last.x + 2.0 * u * t * a.x + t * t * p.x, u * u * last.y + 2.0 * u * t * a.y + t * t * p.y));
                    }
                    last = p;
                }
                PathCmd::CubicTo(a, b, p) => {
                    if cur.is_empty() {
                        cur.push(last);
                    }
                    for i in 1..=segments {
                        let t = i as f32 / segments as f32;
                        let u = 1.0 - t;
                        let (w0, w1, w2, w3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
                        cur.push(pt(
                            w0 * last.x + w1 * a.x + w2 * b.x + w3 * p.x,
                            w0 * last.y + w1 * a.y + w2 * b.y + w3 * p.y,
                        ));
                    }
                    last = p;
                }
                PathCmd::Close => {
                    if let Some(&first) = cur.first() {
                        last = first;
                        out.push((std::mem::take(&mut cur), true));
                    }
                }
            }
        }
        if cur.len() > 1 {
            out.push((cur, false));
        }
        out
    }

    /// O caminho como o atributo `d` do SVG (coordenadas absolutas, até 2 casas).
    pub fn to_svg(&self) -> String {
        let n = |v: f32| {
            let t = format!("{:.2}", v);
            let t = t.trim_end_matches('0').trim_end_matches('.');
            if t == "-0" { "0".to_string() } else { t.to_string() }
        };
        let mut out = String::new();
        for c in &self.cmds {
            let part = match *c {
                PathCmd::MoveTo(p) => format!("M{} {}", n(p.x), n(p.y)),
                PathCmd::LineTo(p) => format!("L{} {}", n(p.x), n(p.y)),
                PathCmd::QuadTo(a, p) => format!("Q{} {} {} {}", n(a.x), n(a.y), n(p.x), n(p.y)),
                PathCmd::CubicTo(a, b, p) => format!("C{} {} {} {} {} {}", n(a.x), n(a.y), n(b.x), n(b.y), n(p.x), n(p.y)),
                PathCmd::Close => "Z".to_string(),
            };
            out.push_str(&part);
        }
        out
    }

    /// O ponto está dentro da área preenchida (regra não zero, a do `fill_path`)? As
    /// figuras abertas contam como fechadas, como no preenchimento.
    pub fn contains(&self, p: PointF) -> bool {
        let mut winding = 0i32;
        for (points, _) in self.flatten(16) {
            let n = points.len();
            if n < 3 {
                continue;
            }
            for i in 0..n {
                let (a, b) = (points[i], points[(i + 1) % n]);
                let cross = (b.x - a.x) * (p.y - a.y) - (p.x - a.x) * (b.y - a.y);
                if a.y <= p.y {
                    if b.y > p.y && cross > 0.0 {
                        winding += 1;
                    }
                } else if b.y <= p.y && cross < 0.0 {
                    winding -= 1;
                }
            }
        }
        winding != 0
    }

    /// Distância de um ponto até o caminho (como linha), para clicar em cabos e traços.
    pub fn distance_to(&self, p: PointF) -> f32 {
        let mut best = f32::MAX;
        for (points, closed) in self.flatten(16) {
            let n = points.len();
            let count = if closed { n } else { n.saturating_sub(1) };
            for i in 0..count {
                let (a, b) = (points[i], points[(i + 1) % n]);
                best = best.min(segment_distance(p, a, b));
            }
        }
        best
    }
}

fn segment_distance(p: PointF, a: PointF, b: PointF) -> f32 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let len2 = dx * dx + dy * dy;
    let t = if len2 == 0.0 { 0.0 } else { (((p.x - a.x) * dx + (p.y - a.y) * dy) / len2).clamp(0.0, 1.0) };
    let (x, y) = (a.x + t * dx - p.x, a.y + t * dy - p.y);
    (x * x + y * y).sqrt()
}

struct SvgTokens<'a> {
    s: &'a [u8],
    i: usize,
}

impl SvgTokens<'_> {
    fn done(&self) -> bool {
        self.i >= self.s.len()
    }

    fn skip_separators(&mut self) {
        while self.i < self.s.len() && (self.s[self.i].is_ascii_whitespace() || self.s[self.i] == b',') {
            self.i += 1;
        }
    }

    fn command(&mut self) -> Option<u8> {
        let c = *self.s.get(self.i)?;
        if c.is_ascii_alphabetic() && c != b'e' && c != b'E' {
            self.i += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Bandeira do arco: `0` ou `1`, que pode vir colada no próximo número.
    fn flag(&mut self) -> Option<bool> {
        self.skip_separators();
        let c = *self.s.get(self.i)?;
        self.i += 1;
        match c {
            b'0' => Some(false),
            b'1' => Some(true),
            _ => None,
        }
    }

    fn number(&mut self) -> Option<f32> {
        self.skip_separators();
        let start = self.i;
        let s = self.s;
        if self.i < s.len() && (s[self.i] == b'-' || s[self.i] == b'+') {
            self.i += 1;
        }
        let mut dot = false;
        while self.i < s.len() {
            match s[self.i] {
                b'0'..=b'9' => self.i += 1,
                b'.' if !dot => {
                    dot = true;
                    self.i += 1;
                }
                b'e' | b'E' => {
                    self.i += 1;
                    if self.i < s.len() && (s[self.i] == b'-' || s[self.i] == b'+') {
                        self.i += 1;
                    }
                }
                _ => break,
            }
        }
        std::str::from_utf8(&s[start..self.i]).ok()?.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_svg_paths() {
        let p = Path::from_svg("M10 20 L30 20 h10 v-5 Z").unwrap();
        assert_eq!(p.cmds.len(), 5);
        assert_eq!(p.current(), Some(pt(10.0, 20.0)), "Z volta ao começo");
        // pares depois do M são retas; números colados com sinal
        let p = Path::from_svg("m1 1 2 2-1-1").unwrap();
        assert_eq!(p.cmds[1], PathCmd::LineTo(pt(3.0, 3.0)));
        assert_eq!(p.cmds[2], PathCmd::LineTo(pt(2.0, 2.0)));
        // arco termina no ponto pedido, com bandeiras coladas
        let p = Path::from_svg("M2 12a10 10 0 1012 0").unwrap();
        assert_eq!(p.current(), Some(pt(14.0, 12.0)));
        assert!(Path::from_svg("M1 x").is_none());
    }

    #[test]
    fn bounds_and_distance() {
        let mut p = Path::new();
        p.rect(RectF::new(0.0, 0.0, 10.0, 10.0), 0.0);
        assert_eq!(p.bounds(), RectF::new(0.0, 0.0, 10.0, 10.0));
        assert!(p.distance_to(pt(5.0, 1.0)) < 1.01);
        assert!(p.distance_to(pt(5.0, 5.0)) > 4.9);
    }

    #[test]
    fn every_builtin_icon_parses() {
        let names = crate::icons::builtin_names();
        assert!(names.len() > 70);
        for n in names {
            let icon = crate::icons::builtin(n).unwrap();
            assert!(!icon.stroke.is_empty() || !icon.fill.is_empty(), "{n}");
            let b = icon.stroke.bounds();
            assert!(b.x >= -1.0 && b.right() <= 25.0, "{n} fora da grade: {b:?}");
        }
    }
}
