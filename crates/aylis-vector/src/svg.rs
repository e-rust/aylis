//! Imagens SVG desenhadas pelo [`Canvas`] do aylis (vetor de verdade: nítido em qualquer
//! tamanho e com `currentColor` seguindo o tema).
//!
//! Lê o que ícones, logos e ilustrações usam: `path`, `rect` (com cantos), `circle`,
//! `ellipse`, `line`, `polyline`, `polygon`, grupos com `transform`, `use`, `svg`
//! aninhado, preenchimento e contorno (cor, `currentColor`, gradiente linear e radial,
//! opacidades, `fill-rule`, pontas, junções e tracejado), atributos ou `style="..."`.
//! Fica de fora: texto, filtros, máscaras, `clipPath`, imagens e folhas `<style>`.

use std::collections::HashMap;

use aylis_core::{Brush, Canvas, Color, GradientStop, LineCap, LineJoin, Path, PointF, RectF, Stroke};
use roxmltree::{Document, Node};

use crate::geom::{Affine, has_figures};

#[derive(Clone, Debug)]
enum Paint {
    Color(Color),
    /// `currentColor`, com a opacidade.
    Current(f32),
    Brush(Brush),
}

#[derive(Clone, Debug)]
struct Shape {
    path: Path,
    /// O `id` do elemento (ou do grupo mais próximo que tem um).
    id: Option<String>,
    /// `vector-effect="non-scaling-stroke"`: a espessura do traço é em pixels (não cresce
    /// com o zoom).
    fixed_stroke: bool,
    fill: Option<Paint>,
    stroke: Option<(Paint, Stroke)>,
}

/// Um SVG lido.
#[derive(Clone, Debug)]
pub struct SvgImage {
    view: RectF,
    shapes: Vec<Shape>,
}

#[derive(Clone, Debug)]
struct Gradient {
    radial: bool,
    user_units: bool,
    coords: HashMap<&'static str, f32>,
    transform: Affine,
    stops: Vec<GradientStop>,
}

/// O que passa de pai para filho.
#[derive(Clone, Debug)]
struct Inherited {
    fill: String,
    stroke: String,
    stroke_width: f32,
    fill_opacity: f32,
    stroke_opacity: f32,
    fill_rule_even_odd: bool,
    cap: LineCap,
    join: LineJoin,
    dash: Option<Vec<f32>>,
    color: Option<Color>,
    id: Option<String>,
    fixed_stroke: bool,
}

impl Default for Inherited {
    fn default() -> Self {
        Self {
            fill: "black".into(),
            stroke: "none".into(),
            stroke_width: 1.0,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
            fill_rule_even_odd: false,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            dash: None,
            color: None,
            id: None,
            fixed_stroke: false,
        }
    }
}

/// Valor de uma propriedade: do `style="..."` (vale mais) ou do atributo.
fn prop<'a>(node: Node<'a, 'a>, name: &str) -> Option<&'a str> {
    if let Some(style) = node.attribute("style") {
        for decl in style.split(';') {
            if let Some((k, v)) = decl.split_once(':') {
                if k.trim() == name {
                    return Some(v.trim());
                }
            }
        }
    }
    node.attribute(name).map(str::trim)
}

/// Um número com unidade (px, pt...); `%` vira fração de `percent_of`.
fn length(s: &str, percent_of: f32) -> Option<f32> {
    let s = s.trim();
    if let Some(p) = s.strip_suffix('%') {
        return p.trim().parse::<f32>().ok().map(|v| v / 100.0 * percent_of);
    }
    let end = s.find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+' | 'e' | 'E'))).unwrap_or(s.len());
    let v: f32 = s[..end].parse().ok()?;
    Some(match &s[end..] {
        "pt" => v * 4.0 / 3.0,
        "pc" => v * 16.0,
        "in" => v * 96.0,
        "cm" => v * 96.0 / 2.54,
        "mm" => v * 96.0 / 25.4,
        "em" | "rem" => v * 16.0,
        _ => v,
    })
}

fn numbers(s: &str) -> Vec<f32> {
    s.split(|c: char| c == ',' || c.is_whitespace()).filter(|t| !t.is_empty()).filter_map(|t| t.parse().ok()).collect()
}

const NAMED: [(&str, u32); 40] = [
    ("black", 0x000000), ("white", 0xFFFFFF), ("red", 0xFF0000), ("green", 0x008000), ("blue", 0x0000FF),
    ("yellow", 0xFFFF00), ("cyan", 0x00FFFF), ("aqua", 0x00FFFF), ("magenta", 0xFF00FF), ("fuchsia", 0xFF00FF),
    ("gray", 0x808080), ("grey", 0x808080), ("silver", 0xC0C0C0), ("maroon", 0x800000), ("olive", 0x808000),
    ("lime", 0x00FF00), ("teal", 0x008080), ("navy", 0x000080), ("purple", 0x800080), ("orange", 0xFFA500),
    ("pink", 0xFFC0CB), ("brown", 0xA52A2A), ("gold", 0xFFD700), ("indigo", 0x4B0082), ("violet", 0xEE82EE),
    ("darkgray", 0xA9A9A9), ("darkgrey", 0xA9A9A9), ("lightgray", 0xD3D3D3), ("lightgrey", 0xD3D3D3),
    ("darkred", 0x8B0000), ("darkgreen", 0x006400), ("darkblue", 0x00008B), ("lightblue", 0xADD8E6),
    ("skyblue", 0x87CEEB), ("tomato", 0xFF6347), ("coral", 0xFF7F50), ("salmon", 0xFA8072), ("crimson", 0xDC143C),
    ("whitesmoke", 0xF5F5F5), ("dimgray", 0x696969),
];

/// Uma cor CSS. `None` para `none`, `currentColor` e o que não for cor.
fn css_color(s: &str) -> Option<Color> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() == 3 || hex.len() == 4 {
            let expanded: String = hex.chars().flat_map(|c| [c, c]).collect();
            return Color::from_hex(&format!("#{expanded}"));
        }
        return Color::from_hex(s);
    }
    let lower = s.to_ascii_lowercase();
    if let Some(inner) = lower.strip_prefix("rgba(").or_else(|| lower.strip_prefix("rgb(")).and_then(|r| r.strip_suffix(')')) {
        let parts: Vec<&str> = inner.split([',', ' ', '/']).filter(|p| !p.is_empty()).collect();
        let channel = |p: &str| -> Option<u8> {
            if let Some(v) = p.strip_suffix('%') { v.parse::<f32>().ok().map(|v| (v * 2.55).round().clamp(0.0, 255.0) as u8) } else { p.parse::<f32>().ok().map(|v| v.round().clamp(0.0, 255.0) as u8) }
        };
        let (r, g, b) = (channel(parts.first()?)?, channel(parts.get(1)?)?, channel(parts.get(2)?)?);
        let a = parts.get(3).and_then(|a| if let Some(v) = a.strip_suffix('%') { v.parse::<f32>().ok().map(|v| v / 100.0) } else { a.parse::<f32>().ok() }).unwrap_or(1.0);
        return Some(Color::rgba(r, g, b, (a.clamp(0.0, 1.0) * 255.0).round() as u8));
    }
    if lower == "transparent" {
        return Some(Color::TRANSPARENT);
    }
    NAMED.iter().find(|(n, _)| *n == lower).map(|(_, v)| Color::rgb((v >> 16) as u8, (v >> 8) as u8, *v as u8))
}

/// O atributo `transform` (lista de funções).
fn parse_transform(s: &str) -> Affine {
    let mut m = Affine::IDENTITY;
    let mut rest = s;
    while let Some(open) = rest.find('(') {
        let name = rest[..open].trim().trim_start_matches(',').trim();
        let Some(close) = rest[open..].find(')') else { break };
        let v = numbers(&rest[open + 1..open + close]);
        let g = |i: usize, d: f32| v.get(i).copied().unwrap_or(d);
        let t = match name {
            "matrix" if v.len() >= 6 => Affine::new(v[0], v[1], v[2], v[3], v[4], v[5]),
            "translate" => Affine::translate(g(0, 0.0), g(1, 0.0)),
            "scale" => Affine::scale(g(0, 1.0), g(1, g(0, 1.0))),
            "rotate" => {
                let (cx, cy) = (g(1, 0.0), g(2, 0.0));
                Affine::translate(cx, cy).then(Affine::rotate(g(0, 0.0))).then(Affine::translate(-cx, -cy))
            }
            "skewX" => Affine::skew_x(g(0, 0.0)),
            "skewY" => Affine::skew_y(g(0, 0.0)),
            _ => Affine::IDENTITY,
        };
        m = m.then(t);
        rest = &rest[open + close + 1..];
    }
    m
}

fn href<'a>(node: Node<'a, 'a>) -> Option<&'a str> {
    node.attributes().find(|a| a.name() == "href").map(|a| a.value().trim_start_matches('#'))
}

fn read_gradients(doc: &Document) -> HashMap<String, Gradient> {
    let mut raw: Vec<(String, Node)> = Vec::new();
    for n in doc.descendants() {
        if matches!(n.tag_name().name(), "linearGradient" | "radialGradient") {
            if let Some(id) = n.attribute("id") {
                raw.push((id.to_string(), n));
            }
        }
    }
    let by_id: HashMap<&str, Node> = raw.iter().map(|(id, n)| (id.as_str(), *n)).collect();
    let mut out = HashMap::new();
    for (id, node) in &raw {
        // Os stops e atributos podem vir de outro gradiente (href).
        let mut chain = vec![*node];
        while let Some(next) = href(*chain.last().unwrap()).and_then(|h| by_id.get(h)) {
            if chain.len() > 8 {
                break;
            }
            chain.push(*next);
        }
        let attr = |name: &str| chain.iter().find_map(|n| n.attribute(name));
        let stops_node = chain.iter().find(|n| n.children().any(|c| c.tag_name().name() == "stop"));
        let stops: Vec<GradientStop> = stops_node
            .map(|n| {
                n.children()
                    .filter(|c| c.tag_name().name() == "stop")
                    .map(|s| {
                        let offset = prop(s, "offset").and_then(|o| length(o, 1.0)).unwrap_or(0.0).clamp(0.0, 1.0);
                        let color = prop(s, "stop-color").and_then(css_color).unwrap_or(Color::BLACK);
                        let opacity = prop(s, "stop-opacity").and_then(|o| o.parse::<f32>().ok()).unwrap_or(1.0);
                        GradientStop::new(offset, color.with_opacity(opacity))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let radial = node.tag_name().name() == "radialGradient";
        let user_units = attr("gradientUnits") == Some("userSpaceOnUse");
        let mut coords = HashMap::new();
        let defaults: &[(&'static str, f32)] =
            if radial { &[("cx", 0.5), ("cy", 0.5), ("r", 0.5)] } else { &[("x1", 0.0), ("y1", 0.0), ("x2", 1.0), ("y2", 0.0)] };
        for (k, d) in defaults {
            let v = attr(k).and_then(|v| if v.ends_with('%') { length(v, 1.0) } else { v.parse().ok() }).unwrap_or(*d);
            coords.insert(*k, v);
        }
        if radial {
            for (k, base) in [("fx", "cx"), ("fy", "cy")] {
                let v = attr(k).and_then(|v| v.parse().ok()).unwrap_or(coords[base]);
                coords.insert(k, v);
            }
        }
        let transform = attr("gradientTransform").map(parse_transform).unwrap_or_default();
        out.insert(id.clone(), Gradient { radial, user_units, coords, transform, stops });
    }
    out
}

struct Reader<'a> {
    gradients: HashMap<String, Gradient>,
    ids: HashMap<&'a str, Node<'a, 'a>>,
    shapes: Vec<Shape>,
    view_w: f32,
}

impl<'a> Reader<'a> {
    fn paint(&self, spec: &str, opacity: f32, local_bounds: RectF, m: Affine) -> Option<Paint> {
        let spec = spec.trim();
        if spec == "none" || opacity <= 0.0 {
            return None;
        }
        if spec == "currentColor" {
            return Some(Paint::Current(opacity));
        }
        if let Some(id) = spec.strip_prefix("url(").and_then(|r| r.split(')').next()) {
            let id = id.trim().trim_matches(['\'', '"']).trim_start_matches('#');
            let g = self.gradients.get(id)?;
            if g.stops.is_empty() {
                return None;
            }
            if g.stops.len() == 1 {
                return Some(Paint::Color(g.stops[0].color.with_opacity(opacity)));
            }
            // Coordenadas do gradiente → espaço do usuário → espaço do SVG.
            let unit = if g.user_units {
                Affine::IDENTITY
            } else {
                Affine::new(local_bounds.width, 0.0, 0.0, local_bounds.height, local_bounds.x, local_bounds.y)
            };
            let gm = m.then(unit).then(g.transform);
            let stops = g.stops.iter().map(|s| GradientStop::new(s.offset, s.color.with_opacity(opacity))).collect();
            let c = |k: &str| g.coords.get(k).copied().unwrap_or(0.0);
            let brush = if g.radial {
                Brush::Radial { center: gm.apply(PointF::new(c("cx"), c("cy"))), radius: (c("r") * gm.scale_factor()).max(0.01), stops }
            } else {
                Brush::Linear { start: gm.apply(PointF::new(c("x1"), c("y1"))), end: gm.apply(PointF::new(c("x2"), c("y2"))), stops }
            };
            return Some(Paint::Brush(brush));
        }
        css_color(spec).map(|c| Paint::Color(c.with_opacity(opacity)))
    }

    fn inherit(&self, node: Node, parent: &Inherited) -> Inherited {
        let mut s = parent.clone();
        if let Some(v) = prop(node, "fill") {
            s.fill = v.to_string();
        }
        if let Some(v) = prop(node, "stroke") {
            s.stroke = v.to_string();
        }
        if let Some(v) = prop(node, "stroke-width").and_then(|v| length(v, self.view_w)) {
            s.stroke_width = v;
        }
        if let Some(v) = prop(node, "fill-opacity").and_then(|v| length(v, 1.0)) {
            s.fill_opacity = v;
        }
        if let Some(v) = prop(node, "stroke-opacity").and_then(|v| length(v, 1.0)) {
            s.stroke_opacity = v;
        }
        if let Some(v) = prop(node, "fill-rule") {
            s.fill_rule_even_odd = v == "evenodd";
        }
        if let Some(v) = prop(node, "stroke-linecap") {
            s.cap = match v {
                "round" => LineCap::Round,
                "square" => LineCap::Square,
                _ => LineCap::Butt,
            };
        }
        if let Some(v) = prop(node, "stroke-linejoin") {
            s.join = match v {
                "round" => LineJoin::Round,
                "bevel" => LineJoin::Bevel,
                _ => LineJoin::Miter,
            };
        }
        if let Some(v) = prop(node, "stroke-dasharray") {
            let d = numbers(v);
            s.dash = (!d.is_empty() && d.iter().any(|x| *x > 0.0)).then_some(d);
        }
        if let Some(c) = prop(node, "color").and_then(css_color) {
            s.color = Some(c);
        }
        s
    }

    fn walk(&mut self, node: Node<'a, 'a>, m: Affine, style: &Inherited, opacity: f32, depth: usize) {
        if depth > 32 || !node.is_element() {
            return;
        }
        if prop(node, "display") == Some("none") || prop(node, "visibility") == Some("hidden") {
            return;
        }
        let name = node.tag_name().name();
        if matches!(
            name,
            "defs" | "clipPath" | "mask" | "symbol" | "title" | "desc" | "metadata" | "style" | "linearGradient" | "radialGradient" | "pattern" | "filter" | "marker" | "text" | "image"
        ) {
            return;
        }
        let m = match node.attribute("transform") {
            Some(t) => m.then(parse_transform(t)),
            None => m,
        };
        let mut style = self.inherit(node, style);
        if let Some(id) = node.attribute("id") {
            style.id = Some(id.to_string());
        }
        if let Some(effect) = prop(node, "vector-effect") {
            style.fixed_stroke = effect == "non-scaling-stroke";
        }
        let opacity = opacity * prop(node, "opacity").and_then(|v| length(v, 1.0)).unwrap_or(1.0);
        let num = |k: &str, of: f32| node.attribute(k).and_then(|v| length(v, of)).unwrap_or(0.0);
        let w = self.view_w;
        let mut path = Path::new();
        match name {
            "svg" | "g" | "a" | "switch" => {
                let m = if name == "svg" && node.parent_element().is_some() { m.then(Affine::translate(num("x", w), num("y", w))) } else { m };
                for child in node.children() {
                    self.walk(child, m, &style, opacity, depth + 1);
                }
                return;
            }
            "use" => {
                if let Some(target) = href(node).and_then(|h| self.ids.get(h).copied()) {
                    let m = m.then(Affine::translate(num("x", w), num("y", w)));
                    if target.tag_name().name() == "symbol" {
                        for child in target.children() {
                            self.walk(child, m, &style, opacity, depth + 1);
                        }
                    } else {
                        self.walk(target, m, &style, opacity, depth + 1);
                    }
                }
                return;
            }
            "path" => {
                if let Some(p) = node.attribute("d").and_then(Path::from_svg) {
                    path = p;
                }
            }
            "rect" => {
                let (x, y, rw, rh) = (num("x", w), num("y", w), num("width", w), num("height", w));
                let rx = node.attribute("rx").or(node.attribute("ry")).and_then(|v| length(v, w)).unwrap_or(0.0);
                if rw > 0.0 && rh > 0.0 {
                    path.rect(RectF::new(x, y, rw, rh), rx.min(rw / 2.0).min(rh / 2.0));
                }
            }
            "circle" => {
                let r = num("r", w);
                if r > 0.0 {
                    path.ellipse(RectF::new(num("cx", w) - r, num("cy", w) - r, 2.0 * r, 2.0 * r));
                }
            }
            "ellipse" => {
                let (rx, ry) = (num("rx", w), num("ry", w));
                if rx > 0.0 && ry > 0.0 {
                    path.ellipse(RectF::new(num("cx", w) - rx, num("cy", w) - ry, 2.0 * rx, 2.0 * ry));
                }
            }
            "line" => {
                path.move_to(num("x1", w), num("y1", w)).line_to(num("x2", w), num("y2", w));
            }
            "polyline" | "polygon" => {
                let v = numbers(node.attribute("points").unwrap_or(""));
                let pts: Vec<PointF> = v.chunks_exact(2).map(|c| PointF::new(c[0], c[1])).collect();
                if pts.len() >= 2 {
                    path.polygon(&pts, name == "polygon");
                }
            }
            _ => return,
        }
        if !has_figures(&path) {
            return;
        }
        let bounds = path.bounds();
        let fill = if name == "line" {
            None
        } else {
            self.paint(&style.fill, style.fill_opacity * opacity, bounds, m)
        };
        let stroke = self.paint(&style.stroke, style.stroke_opacity * opacity, bounds, m).map(|p| {
            let width = if style.fixed_stroke { style.stroke_width } else { style.stroke_width * m.scale_factor() };
            let mut s = Stroke::new(Color::BLACK, width);
            s.cap = style.cap;
            s.join = style.join;
            s.dash = style.dash.as_ref().map(|d| d.iter().map(|x| x * m.scale_factor()).collect());
            (p, s)
        });
        if fill.is_none() && stroke.is_none() {
            return;
        }
        let mut path = m.path(&path);
        path.even_odd = style.fill_rule_even_odd;
        // currentColor sem cor herdada fica para a hora de desenhar (cor do controle).
        let fix = |p: Paint| match (p, style.color) {
            (Paint::Current(o), Some(c)) => Paint::Color(c.with_opacity(o)),
            (p, _) => p,
        };
        self.shapes.push(Shape { path, id: style.id.clone(), fixed_stroke: style.fixed_stroke, fill: fill.map(fix), stroke: stroke.map(|(p, s)| (fix(p), s)) });
    }
}

impl SvgImage {
    /// Lê o texto do SVG.
    pub fn parse(text: &str) -> Result<SvgImage, String> {
        let doc = Document::parse_with_options(text, roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() })
            .map_err(|e| format!("SVG inválido: {e}"))?;
        let root = doc.root_element();
        if root.tag_name().name() != "svg" {
            return Err("não é um SVG (a raiz não é <svg>)".into());
        }
        let width = root.attribute("width").and_then(|v| length(v, 100.0));
        let height = root.attribute("height").and_then(|v| length(v, 100.0));
        let view = root
            .attribute("viewBox")
            .map(numbers)
            .filter(|v| v.len() == 4 && v[2] > 0.0 && v[3] > 0.0)
            .map(|v| RectF::new(v[0], v[1], v[2], v[3]))
            .unwrap_or_else(|| RectF::new(0.0, 0.0, width.unwrap_or(100.0), height.unwrap_or(100.0)));
        let ids = doc.descendants().filter_map(|n| Some((n.attribute("id")?, n))).collect();
        let mut reader = Reader { gradients: read_gradients(&doc), ids, shapes: Vec::new(), view_w: view.width };
        reader.walk(root, Affine::IDENTITY, &Inherited::default(), 1.0, 0);
        Ok(SvgImage { view, shapes: reader.shapes })
    }

    /// O `viewBox` (a área desenhada).
    pub fn view_box(&self) -> RectF {
        self.view
    }

    /// Quantas formas o SVG tem.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Os contornos das formas preenchidas (`<path>`, `<rect>`, `<circle>`, `<ellipse>`,
    /// `<polygon>`..., com os `transform` e `<use>` aplicados), nas coordenadas do `viewBox`,
    /// e quantas formas só têm traço (essas não viram contorno).
    pub fn fill_outlines(&self) -> (Vec<Path>, usize) {
        let filled = self.shapes.iter().filter(|s| s.fill.is_some()).map(|s| s.path.clone()).collect();
        let stroke_only = self.shapes.iter().filter(|s| s.fill.is_none() && s.stroke.is_some()).count();
        (filled, stroke_only)
    }

    /// A área (nas coordenadas do `viewBox`) das formas do elemento `id` (e dos filhos dele).
    pub fn bounds_of(&self, id: &str) -> Option<RectF> {
        self.shapes.iter().filter(|s| s.id.as_deref() == Some(id)).map(|s| s.path.bounds()).reduce(|a, b| {
            let (x, y) = (a.x.min(b.x), a.y.min(b.y));
            RectF::new(x, y, (a.x + a.width).max(b.x + b.width) - x, (a.y + a.height).max(b.y + b.height) - y)
        })
    }

    /// Desenha o `viewBox` inteiro em `dest`. `current` é a cor do `currentColor`.
    pub fn render(&self, canvas: &mut dyn Canvas, dest: RectF, current: Color) {
        self.render_part(canvas, self.view, dest, current, &[], current);
    }

    /// Desenha a área `view` (nas coordenadas do `viewBox`) em `dest`, com as formas dos
    /// elementos de `highlight` (pelo `id`) preenchidas com `highlight_color`: o mapa com o
    /// município escolhido em destaque.
    pub fn render_part(&self, canvas: &mut dyn Canvas, view: RectF, dest: RectF, current: Color, highlight: &[&str], highlight_color: Color) {
        self.render_visible(canvas, view, dest, None, current, highlight, highlight_color);
    }

    /// Como [`render_part`](Self::render_part), sem desenhar as formas que caem fora de
    /// `visible` (a área recortada, nas coordenadas de `dest`).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_visible(&self, canvas: &mut dyn Canvas, view: RectF, dest: RectF, visible: Option<RectF>, current: Color, highlight: &[&str], highlight_color: Color) {
        let m = Affine::rect_to_rect(view, dest);
        // A área visível nas coordenadas do SVG (a escala é a mesma nos dois eixos ou não:
        // cada eixo à parte).
        let visible = visible.map(|v| {
            let (sx, sy) = (dest.width / view.width.max(1e-6), dest.height / view.height.max(1e-6));
            RectF::new(view.x + (v.x - dest.x) / sx, view.y + (v.y - dest.y) / sy, v.width / sx, v.height / sy)
        });
        let brush = |p: &Paint| -> Brush {
            match p {
                Paint::Color(c) => Brush::Solid(*c),
                Paint::Current(o) => Brush::Solid(current.with_opacity(*o)),
                Paint::Brush(b) => map_brush(b, &m),
            }
        };
        for shape in &self.shapes {
            // Fora da área visível (o município com zoom no mapa do estado): nem desenha.
            if let Some(v) = visible {
                let reach = shape.stroke.as_ref().map_or(0.0, |(_, s)| if shape.fixed_stroke { s.width / m.scale_factor().max(1e-6) } else { s.width });
                if !shape.path.bounds().inflate(reach).intersects(v) {
                    continue;
                }
            }
            let path = m.path(&shape.path);
            if shape.id.as_deref().is_some_and(|id| highlight.contains(&id)) {
                canvas.fill_path(&path, &Brush::Solid(highlight_color));
            } else if let Some(f) = &shape.fill {
                canvas.fill_path(&path, &brush(f));
            }
            if let Some((p, s)) = &shape.stroke {
                let mut s = s.clone();
                s.brush = brush(p);
                if !shape.fixed_stroke {
                    s.width *= m.scale_factor();
                }
                s.dash = s.dash.map(|d| d.iter().map(|x| x * m.scale_factor()).collect());
                canvas.stroke_path(&path, &s);
            }
        }
    }
}

fn map_brush(b: &Brush, m: &Affine) -> Brush {
    match b {
        Brush::Linear { start, end, stops } => Brush::Linear { start: m.apply(*start), end: m.apply(*end), stops: stops.clone() },
        Brush::Radial { center, radius, stops } => Brush::Radial { center: m.apply(*center), radius: radius * m.scale_factor(), stops: stops.clone() },
        Brush::Sweep { center, start_angle, stops } => Brush::Sweep { center: m.apply(*center), start_angle: *start_angle, stops: stops.clone() },
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_styles_and_transforms() {
        let svg = SvgImage::parse(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 50">
                <defs><linearGradient id="g"><stop offset="0" stop-color="red"/><stop offset="1" stop-color="#00f"/></linearGradient></defs>
                <g transform="translate(10 0)" fill="currentColor">
                    <rect x="0" y="0" width="20" height="10" rx="2"/>
                    <circle cx="50" cy="25" r="10" style="fill:url(#g);stroke:#000;stroke-width:2"/>
                </g>
                <path d="M0 0L10 10" stroke="black" fill="none"/>
                <text>ignorado</text>
            </svg>"##,
        )
        .unwrap();
        assert_eq!(svg.view_box(), RectF::new(0.0, 0.0, 100.0, 50.0));
        assert_eq!(svg.shape_count(), 3);
        assert!(matches!(svg.shapes[0].fill, Some(Paint::Current(_))));
        assert_eq!(svg.shapes[0].path.bounds().x, 10.0, "o transform do grupo");
        assert!(matches!(svg.shapes[1].fill, Some(Paint::Brush(Brush::Linear { .. }))));
        assert!(svg.shapes[1].stroke.is_some());
        assert!(svg.shapes[2].fill.is_none());
    }

    #[test]
    fn ids_bounds_and_fixed_strokes() {
        // Um mapa: o grupo dá o estilo, cada área tem o seu id.
        let svg = SvgImage::parse(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                <g fill="currentColor" stroke="black" vector-effect="non-scaling-stroke">
                    <path id="m1" d="M10 10L30 10L30 40Z"/>
                    <g id="m2"><rect x="50" y="50" width="20" height="10"/><rect x="80" y="60" width="10" height="30"/></g>
                </g>
            </svg>"##,
        )
        .unwrap();
        assert_eq!(svg.shapes[0].id.as_deref(), Some("m1"));
        assert_eq!(svg.shapes[2].id.as_deref(), Some("m2"), "o id do grupo vale para os filhos");
        assert!(svg.shapes.iter().all(|s| s.fixed_stroke));
        assert_eq!(svg.bounds_of("m1"), Some(RectF::new(10.0, 10.0, 20.0, 30.0)));
        assert_eq!(svg.bounds_of("m2"), Some(RectF::new(50.0, 50.0, 40.0, 40.0)), "a união das formas do grupo");
        assert_eq!(svg.bounds_of("m3"), None);
    }

    #[test]
    fn colors_and_transforms_parse() {
        assert_eq!(css_color("#f00"), Some(Color::rgb(255, 0, 0)));
        assert_eq!(css_color("rgb(0, 128, 255)"), Some(Color::rgb(0, 128, 255)));
        assert_eq!(css_color("teal"), Some(Color::rgb(0, 128, 128)));
        let m = parse_transform("translate(10,20) scale(2)");
        assert_eq!(m.apply(PointF::new(1.0, 1.0)), PointF::new(12.0, 22.0));
        assert!(SvgImage::parse("<html/>").is_err());
    }
}
