//! Player de Lottie (o JSON do Bodymovin/After Effects), desenhado pelo [`Canvas`] do aylis:
//! o mesmo desenho no Skia, no Direct2D e no navegador.
//!
//! Cobre o que as animações de ícones e emojis usam (os emojis animados Noto do Google
//! inteiros): camadas de forma, sólidas, nulas e pré-composições (com tempo remapeado),
//! parentesco, transformações animadas (com curvas de velocidade e caminho de movimento),
//! grupos, caminhos, elipses, retângulos, preenchimentos e contornos sólidos ou em
//! gradiente (linear e radial), "Trim Paths", máscaras e "track mattes" (alfa e alfa
//! invertido, como recorte pelo contorno da camada). Fica de fora: texto, imagens,
//! efeitos, expressões e mattes de luminância (tratados como alfa).

use std::collections::HashMap;

use aylis_core::{Brush, Canvas, Color, GradientStop, LineCap, LineJoin, Path, PointF, RectF, Stroke};
use serde_json::Value;

use crate::geom::{Affine, dist, has_figures, rect_path, trim};

// ---------------------------------------------------------------------------
// Valores animados

#[derive(Clone, Copy, Debug, Default)]
struct Ease {
    o: (f32, f32),
    i: (f32, f32),
}

#[derive(Clone, Debug)]
struct Key {
    t: f32,
    s: Vec<f32>,
    /// Valor final explícito (formato antigo); sem ele, vale o `s` da próxima chave.
    e: Option<Vec<f32>>,
    ease: Ease,
    hold: bool,
    /// Tangentes do caminho de movimento (posição).
    to: Option<Vec<f32>>,
    ti: Option<Vec<f32>>,
}

/// Uma propriedade: valor fixo ou chaves no tempo. Números e caminhos viram `Vec<f32>`
/// (o caminho achatado: fechado, n, e por vértice v, i, o), então a interpolação é a mesma.
#[derive(Clone, Debug)]
enum Prop {
    Static(Vec<f32>),
    Anim(Vec<Key>),
}

fn num(v: &Value) -> Option<f32> {
    v.as_f64().map(|n| n as f32)
}

/// Número ou lista de números.
fn numbers(v: &Value) -> Option<Vec<f32>> {
    match v {
        Value::Number(_) => Some(vec![num(v)?]),
        Value::Array(a) => a.iter().map(num).collect(),
        _ => None,
    }
}

/// Um caminho Bezier (`{i, o, v, c}`) achatado.
fn bezier(v: &Value) -> Option<Vec<f32>> {
    let v = if let Value::Array(a) = v { a.first()? } else { v };
    let pts = |key: &str| -> Option<Vec<[f32; 2]>> {
        v.get(key)?.as_array()?.iter().map(|p| Some([num(p.get(0)?)?, num(p.get(1)?)?])).collect()
    };
    let (vs, is, os) = (pts("v")?, pts("i").unwrap_or_default(), pts("o").unwrap_or_default());
    let closed = v.get("c").and_then(Value::as_bool).unwrap_or(false);
    let mut out = vec![if closed { 1.0 } else { 0.0 }, vs.len() as f32];
    for (k, p) in vs.iter().enumerate() {
        let i = is.get(k).copied().unwrap_or([0.0, 0.0]);
        let o = os.get(k).copied().unwrap_or([0.0, 0.0]);
        out.extend_from_slice(&[p[0], p[1], i[0], i[1], o[0], o[1]]);
    }
    Some(out)
}

fn ease_part(v: Option<&Value>) -> Option<(f32, f32)> {
    let v = v?;
    let first = |k: &str| v.get(k).and_then(|x| if let Value::Array(a) = x { a.first().and_then(num) } else { num(x) });
    Some((first("x")?, first("y")?))
}

impl Prop {
    fn parse(v: Option<&Value>, conv: fn(&Value) -> Option<Vec<f32>>, default: &[f32]) -> Prop {
        let Some(v) = v else { return Prop::Static(default.to_vec()) };
        let k = v.get("k").unwrap_or(v);
        if let Value::Array(items) = k {
            if items.first().is_some_and(|f| f.get("t").is_some()) {
                let keys: Vec<Key> = items
                    .iter()
                    .filter_map(|f| {
                        Some(Key {
                            t: num(f.get("t")?)?,
                            s: f.get("s").and_then(conv).unwrap_or_default(),
                            e: f.get("e").and_then(conv),
                            ease: Ease {
                                o: ease_part(f.get("o")).unwrap_or((0.0, 0.0)),
                                i: ease_part(f.get("i")).unwrap_or((1.0, 1.0)),
                            },
                            hold: f.get("h").and_then(Value::as_i64) == Some(1),
                            to: f.get("to").and_then(numbers),
                            ti: f.get("ti").and_then(numbers),
                        })
                    })
                    .collect();
                if !keys.is_empty() {
                    return Prop::Anim(keys);
                }
            }
        }
        Prop::Static(conv(k).unwrap_or_else(|| default.to_vec()))
    }

    fn at(&self, t: f32) -> Vec<f32> {
        let keys = match self {
            Prop::Static(v) => return v.clone(),
            Prop::Anim(keys) => keys,
        };
        let first = &keys[0];
        if t <= first.t || keys.len() == 1 {
            return if first.s.is_empty() { first.e.clone().unwrap_or_default() } else { first.s.clone() };
        }
        for w in keys.windows(2) {
            let (k, next) = (&w[0], &w[1]);
            if t >= next.t {
                continue;
            }
            let start = &k.s;
            let end = k.e.as_ref().unwrap_or(&next.s);
            if k.hold || end.is_empty() {
                return start.clone();
            }
            let span = (next.t - k.t).max(1e-6);
            let p = cubic_ease(k.ease, ((t - k.t) / span).clamp(0.0, 1.0));
            // Caminho de movimento: curva entre as posições.
            if let (Some(to), Some(ti)) = (&k.to, &k.ti) {
                if start.len() >= 2 && end.len() >= 2 && to.len() >= 2 && ti.len() >= 2 && (to.iter().any(|v| *v != 0.0) || ti.iter().any(|v| *v != 0.0)) {
                    let c = |i: usize| bez(start[i], start[i] + to[i], end[i] + ti[i], end[i], p);
                    let mut out: Vec<f32> = (0..start.len().min(end.len())).map(|i| if i < 2 { c(i) } else { start[i] + (end[i] - start[i]) * p }).collect();
                    out.truncate(start.len().min(end.len()));
                    return out;
                }
            }
            return start.iter().zip(end.iter()).map(|(a, b)| a + (b - a) * p).collect();
        }
        let last = &keys[keys.len() - 1];
        if !last.s.is_empty() {
            return last.s.clone();
        }
        // No formato antigo a última chave só marca o tempo: vale o fim da anterior.
        keys.iter().rev().find_map(|k| k.e.clone().or_else(|| (!k.s.is_empty()).then(|| k.s.clone()))).unwrap_or_default()
    }

    fn scalar(&self, t: f32) -> f32 {
        self.at(t).first().copied().unwrap_or(0.0)
    }

    fn point(&self, t: f32) -> PointF {
        let v = self.at(t);
        PointF::new(v.first().copied().unwrap_or(0.0), v.get(1).copied().unwrap_or(0.0))
    }
}

fn bez(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let u = 1.0 - t;
    u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d
}

/// A curva de velocidade (cubic-bezier com pontos `o` e `i`) no progresso `x`.
fn cubic_ease(e: Ease, x: f32) -> f32 {
    let (x1, y1, x2, y2) = (e.o.0, e.o.1, e.i.0, e.i.1);
    if (x1 - y1).abs() < 1e-6 && (x2 - y2).abs() < 1e-6 {
        return x;
    }
    // Acha t com x(t) = x (bisseção: robusta para qualquer curva) e devolve y(t).
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    let mut t = x;
    for _ in 0..24 {
        let xt = bez(0.0, x1, x2, 1.0, t);
        if (xt - x).abs() < 1e-5 {
            break;
        }
        if xt < x { lo = t } else { hi = t }
        t = (lo + hi) / 2.0;
    }
    bez(0.0, y1, y2, 1.0, t)
}

// ---------------------------------------------------------------------------
// Modelo

#[derive(Clone, Debug)]
enum Position {
    Joined(Prop),
    Split(Prop, Prop),
}

#[derive(Clone, Debug)]
struct Transform {
    anchor: Prop,
    position: Position,
    scale: Prop,
    rotation: Prop,
    opacity: Prop,
    skew: Prop,
    skew_axis: Prop,
}

impl Transform {
    fn parse(v: Option<&Value>) -> Transform {
        let get = |k: &str| v.and_then(|v| v.get(k));
        let position = match get("p") {
            Some(p) if p.get("s").and_then(Value::as_bool) == Some(true) => {
                Position::Split(Prop::parse(p.get("x"), numbers, &[0.0]), Prop::parse(p.get("y"), numbers, &[0.0]))
            }
            p => Position::Joined(Prop::parse(p, numbers, &[0.0, 0.0])),
        };
        Transform {
            anchor: Prop::parse(get("a"), numbers, &[0.0, 0.0]),
            position,
            scale: Prop::parse(get("s"), numbers, &[100.0, 100.0]),
            rotation: Prop::parse(get("r").or_else(|| get("rz")), numbers, &[0.0]),
            opacity: Prop::parse(get("o"), numbers, &[100.0]),
            skew: Prop::parse(get("sk"), numbers, &[0.0]),
            skew_axis: Prop::parse(get("sa"), numbers, &[0.0]),
        }
    }

    fn matrix(&self, t: f32) -> Affine {
        let a = self.anchor.point(t);
        let p = match &self.position {
            Position::Joined(p) => p.point(t),
            Position::Split(x, y) => PointF::new(x.scalar(t), y.scalar(t)),
        };
        let s = self.scale.at(t);
        let (sx, sy) = (s.first().copied().unwrap_or(100.0) / 100.0, s.get(1).copied().unwrap_or(100.0) / 100.0);
        let mut m = Affine::translate(p.x, p.y).then(Affine::rotate(self.rotation.scalar(t)));
        let sk = self.skew.scalar(t);
        if sk != 0.0 {
            let axis = self.skew_axis.scalar(t);
            m = m.then(Affine::rotate(axis)).then(Affine::skew_x(-sk)).then(Affine::rotate(-axis));
        }
        m.then(Affine::scale(sx, sy)).then(Affine::translate(-a.x, -a.y))
    }

    fn opacity(&self, t: f32) -> f32 {
        (self.opacity.scalar(t) / 100.0).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug)]
struct Gradient {
    radial: bool,
    start: Prop,
    end: Prop,
    count: usize,
    stops: Prop,
}

#[derive(Clone, Debug)]
enum Paint {
    Solid(Prop),
    Gradient(Gradient),
}

#[derive(Clone, Debug)]
struct StrokeStyle {
    width: Prop,
    cap: LineCap,
    join: LineJoin,
}

#[derive(Clone, Debug)]
enum Item {
    Group { items: Vec<Item>, transform: Option<Transform> },
    Shape { path: Prop, reversed: bool },
    Ellipse { center: Prop, size: Prop, reversed: bool },
    Rect { center: Prop, size: Prop, radius: Prop, reversed: bool },
    Fill { paint: Paint, opacity: Prop, even_odd: bool },
    Stroke { paint: Paint, opacity: Prop, style: StrokeStyle },
    Trim { start: Prop, end: Prop, offset: Prop },
}

fn parse_items(list: Option<&Value>) -> Vec<Item> {
    let Some(Value::Array(list)) = list else { return Vec::new() };
    list.iter().filter(|s| s.get("hd").and_then(Value::as_bool) != Some(true)).filter_map(parse_item).collect()
}

fn parse_paint(s: &Value, gradient: bool) -> Paint {
    if gradient {
        let g = s.get("g");
        Paint::Gradient(Gradient {
            radial: s.get("t").and_then(Value::as_i64) == Some(2),
            start: Prop::parse(s.get("s"), numbers, &[0.0, 0.0]),
            end: Prop::parse(s.get("e"), numbers, &[0.0, 0.0]),
            count: g.and_then(|g| g.get("p")).and_then(Value::as_u64).unwrap_or(0) as usize,
            stops: Prop::parse(g.and_then(|g| g.get("k")), numbers, &[]),
        })
    } else {
        Paint::Solid(Prop::parse(s.get("c"), numbers, &[0.0, 0.0, 0.0, 1.0]))
    }
}

fn parse_item(s: &Value) -> Option<Item> {
    let reversed = s.get("d").and_then(Value::as_i64) == Some(3);
    Some(match s.get("ty")?.as_str()? {
        "gr" => {
            let list = s.get("it").and_then(Value::as_array);
            let transform = list.and_then(|l| l.iter().find(|i| i.get("ty").and_then(Value::as_str) == Some("tr"))).map(|tr| Transform::parse(Some(tr)));
            Item::Group { items: parse_items(s.get("it")), transform }
        }
        "sh" => Item::Shape { path: Prop::parse(s.get("ks"), bezier, &[]), reversed },
        "el" => Item::Ellipse { center: Prop::parse(s.get("p"), numbers, &[0.0, 0.0]), size: Prop::parse(s.get("s"), numbers, &[0.0, 0.0]), reversed },
        "rc" => Item::Rect {
            center: Prop::parse(s.get("p"), numbers, &[0.0, 0.0]),
            size: Prop::parse(s.get("s"), numbers, &[0.0, 0.0]),
            radius: Prop::parse(s.get("r"), numbers, &[0.0]),
            reversed,
        },
        ty @ ("fl" | "gf") => Item::Fill {
            paint: parse_paint(s, ty == "gf"),
            opacity: Prop::parse(s.get("o"), numbers, &[100.0]),
            even_odd: s.get("r").and_then(Value::as_i64) == Some(2),
        },
        ty @ ("st" | "gs") => Item::Stroke {
            paint: parse_paint(s, ty == "gs"),
            opacity: Prop::parse(s.get("o"), numbers, &[100.0]),
            style: StrokeStyle {
                width: Prop::parse(s.get("w"), numbers, &[1.0]),
                cap: match s.get("lc").and_then(Value::as_i64) {
                    Some(2) => LineCap::Round,
                    Some(3) => LineCap::Square,
                    _ => LineCap::Butt,
                },
                join: match s.get("lj").and_then(Value::as_i64) {
                    Some(2) => LineJoin::Round,
                    Some(3) => LineJoin::Bevel,
                    _ => LineJoin::Miter,
                },
            },
        },
        "tm" => Item::Trim {
            start: Prop::parse(s.get("s"), numbers, &[0.0]),
            end: Prop::parse(s.get("e"), numbers, &[100.0]),
            offset: Prop::parse(s.get("o"), numbers, &[0.0]),
        },
        _ => return None,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Matte {
    None,
    Alpha,
    Inverted,
}

#[derive(Clone, Debug)]
struct Mask {
    path: Prop,
    /// `a` soma (recorta para dentro), `s` subtrai (recorta para fora), `n` nenhuma.
    mode: char,
    inverted: bool,
}

#[derive(Clone, Debug)]
enum LayerKind {
    Shapes(Vec<Item>),
    Precomp { id: String, size: (f32, f32), remap: Option<Prop> },
    Solid { color: Color, size: (f32, f32) },
    Null,
}

#[derive(Clone, Debug)]
struct Layer {
    index: Option<i64>,
    parent: Option<i64>,
    kind: LayerKind,
    in_point: f32,
    out_point: f32,
    start: f32,
    stretch: f32,
    transform: Transform,
    matte: Matte,
    is_matte: bool,
    masks: Vec<Mask>,
}

fn parse_layers(v: Option<&Value>) -> Vec<Layer> {
    let Some(Value::Array(list)) = v else { return Vec::new() };
    list.iter().filter(|l| l.get("hd").and_then(Value::as_bool) != Some(true)).filter_map(parse_layer).collect()
}

fn parse_layer(l: &Value) -> Option<Layer> {
    let f = |k: &str, d: f32| l.get(k).and_then(num).unwrap_or(d);
    let kind = match l.get("ty")?.as_i64()? {
        4 => LayerKind::Shapes(parse_items(l.get("shapes"))),
        0 => LayerKind::Precomp {
            id: l.get("refId")?.as_str()?.to_string(),
            size: (f("w", 0.0), f("h", 0.0)),
            remap: l.get("tm").map(|tm| Prop::parse(Some(tm), numbers, &[0.0])),
        },
        1 => LayerKind::Solid {
            color: l.get("sc").and_then(Value::as_str).and_then(Color::from_hex).unwrap_or(Color::BLACK),
            size: (f("sw", 0.0), f("sh", 0.0)),
        },
        3 => LayerKind::Null,
        _ => return None,
    };
    let masks = l
        .get("masksProperties")
        .and_then(Value::as_array)
        .map(|list| {
            list.iter()
                .map(|m| Mask {
                    path: Prop::parse(m.get("pt"), bezier, &[]),
                    mode: m.get("mode").and_then(Value::as_str).and_then(|s| s.chars().next()).unwrap_or('a'),
                    inverted: m.get("inv").and_then(Value::as_bool).unwrap_or(false),
                })
                .collect()
        })
        .unwrap_or_default();
    Some(Layer {
        index: l.get("ind").and_then(Value::as_i64),
        parent: l.get("parent").and_then(Value::as_i64),
        kind,
        in_point: f("ip", 0.0),
        out_point: f("op", f32::MAX),
        start: f("st", 0.0),
        stretch: f("sr", 1.0).max(1e-3),
        transform: Transform::parse(l.get("ks")),
        matte: match l.get("tt").and_then(Value::as_i64) {
            Some(2) | Some(4) => Matte::Inverted,
            Some(_) => Matte::Alpha,
            None => Matte::None,
        },
        is_matte: l.get("td").and_then(Value::as_i64).is_some_and(|v| v != 0),
        masks,
    })
}

/// Uma animação Lottie lida.
#[derive(Clone, Debug)]
pub struct Animation {
    width: f32,
    height: f32,
    frame_rate: f32,
    in_point: f32,
    out_point: f32,
    layers: Vec<Layer>,
    assets: HashMap<String, Vec<Layer>>,
}

impl Animation {
    /// Lê o JSON do Lottie.
    pub fn parse(json: &[u8]) -> Result<Animation, String> {
        let v: Value = serde_json::from_slice(json).map_err(|e| format!("JSON inválido: {e}"))?;
        if v.get("layers").and_then(Value::as_array).is_none() {
            return Err("não é uma animação Lottie (sem \"layers\")".into());
        }
        let f = |k: &str, d: f32| v.get(k).and_then(num).unwrap_or(d);
        let assets = v
            .get("assets")
            .and_then(Value::as_array)
            .map(|list| {
                list.iter()
                    .filter_map(|a| Some((a.get("id")?.as_str()?.to_string(), parse_layers(Some(a.get("layers")?)))))
                    .collect()
            })
            .unwrap_or_default();
        Ok(Animation {
            width: f("w", 100.0),
            height: f("h", 100.0),
            frame_rate: f("fr", 30.0).max(1.0),
            in_point: f("ip", 0.0),
            out_point: f("op", 60.0),
            layers: parse_layers(v.get("layers")),
            assets,
        })
    }

    /// Tamanho da composição.
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn frame_rate(&self) -> f32 {
        self.frame_rate
    }

    /// Primeiro e último quadro (o último fica de fora).
    pub fn frames(&self) -> (f32, f32) {
        (self.in_point, self.out_point)
    }

    /// Duração em segundos.
    pub fn duration(&self) -> f32 {
        (self.out_point - self.in_point).max(0.0) / self.frame_rate
    }

    /// O quadro no instante `seconds` (com `repeat`, volta ao começo; sem, para no fim).
    pub fn frame_at(&self, seconds: f32, repeat: bool) -> f32 {
        let total = (self.out_point - self.in_point).max(1e-3);
        let f = seconds * self.frame_rate;
        if repeat { self.in_point + f.rem_euclid(total) } else { self.in_point + f.clamp(0.0, total - 1e-3) }
    }

    /// Desenha o quadro `frame` ocupando `dest` (a composição inteira, `width` × `height`,
    /// vai para `dest`; quem chama escolhe o encaixe).
    pub fn render(&self, canvas: &mut dyn Canvas, frame: f32, dest: RectF) {
        let m = Affine::rect_to_rect(RectF::new(0.0, 0.0, self.width, self.height), dest);
        self.render_comp(canvas, &self.layers, frame, m, 0);
    }

    fn render_comp(&self, canvas: &mut dyn Canvas, layers: &[Layer], t: f32, base: Affine, depth: usize) {
        if depth > 8 {
            return;
        }
        for i in (0..layers.len()).rev() {
            let layer = &layers[i];
            if layer.is_matte || !visible(layer, t) {
                continue;
            }
            if layer.matte == Matte::None || i == 0 {
                self.draw_layer(canvas, layers, i, t, base, depth);
                continue;
            }
            // Track matte: a camada de cima (`td`) recorta esta pelo alfa dela.
            let inverse = layer.matte == Matte::Inverted;
            canvas.save();
            if canvas.begin_masked(inverse) {
                self.draw_layer(canvas, layers, i, t, base, depth);
                canvas.begin_mask();
                if visible(&layers[i - 1], t) {
                    self.draw_layer(canvas, layers, i - 1, t, base, depth);
                }
                canvas.end_mask();
            } else {
                let mut outline = Path::new();
                if visible(&layers[i - 1], t) {
                    self.outline(&layers[i - 1], layers, i - 1, t, base, &mut outline, depth);
                }
                canvas.clip_path(&outline, inverse);
                self.draw_layer(canvas, layers, i, t, base, depth);
            }
            canvas.restore();
        }
    }

    /// O tempo dentro de uma pré-composição (as camadas dela contam do próprio começo).
    fn child_time(&self, layer: &Layer, remap: &Option<Prop>, t: f32) -> f32 {
        match remap {
            Some(tm) => tm.scalar(t) * self.frame_rate,
            None => (t - layer.start) / layer.stretch,
        }
    }

    /// Desenha uma camada (sem o matte dela). As propriedades usam o tempo da composição.
    fn draw_layer(&self, canvas: &mut dyn Canvas, layers: &[Layer], i: usize, t: f32, base: Affine, depth: usize) {
        let layer = &layers[i];
        let opacity = layer.transform.opacity(t);
        if opacity <= 0.001 || matches!(layer.kind, LayerKind::Null) {
            return;
        }
        let m = base.then(layer_matrix(layers, i, t, 0));
        canvas.save();
        for mask in &layer.masks {
            if mask.mode == 'n' {
                continue;
            }
            let path = m.path(&shape_path(&mask.path.at(t), false));
            canvas.clip_path(&path, (mask.mode == 's') != mask.inverted);
        }
        if opacity < 0.999 {
            canvas.save_layer(opacity);
        }
        match &layer.kind {
            LayerKind::Shapes(items) => draw_group(canvas, items, m, 1.0, t),
            LayerKind::Precomp { id, size, remap } => {
                if let Some(inner) = self.assets.get(id) {
                    if size.0 > 0.0 && size.1 > 0.0 {
                        canvas.clip_path(&rect_path(&m, RectF::new(0.0, 0.0, size.0, size.1)), false);
                    }
                    self.render_comp(canvas, inner, self.child_time(layer, remap, t), m, depth + 1);
                }
            }
            LayerKind::Solid { color, size } => {
                canvas.fill_path(&rect_path(&m, RectF::new(0.0, 0.0, size.0, size.1)), &Brush::Solid(*color));
            }
            LayerKind::Null => {}
        }
        if opacity < 0.999 {
            canvas.restore();
        }
        canvas.restore();
    }

    /// O contorno de uma camada (para usá-la como recorte): todas as formas, sem estilo.
    #[allow(clippy::too_many_arguments)]
    fn outline(&self, layer: &Layer, layers: &[Layer], i: usize, t: f32, base: Affine, out: &mut Path, depth: usize) {
        if depth > 8 {
            return;
        }
        let m = base.then(layer_matrix(layers, i, t, 0));
        match &layer.kind {
            LayerKind::Shapes(items) => collect(items, m, t, out),
            LayerKind::Solid { size, .. } => {
                out.append(&rect_path(&m, RectF::new(0.0, 0.0, size.0, size.1)));
            }
            LayerKind::Precomp { id, remap, .. } => {
                if let Some(inner) = self.assets.get(id) {
                    let child_t = self.child_time(layer, remap, t);
                    for (j, l) in inner.iter().enumerate() {
                        if !l.is_matte && child_t >= l.in_point && child_t < l.out_point {
                            self.outline(l, inner, j, child_t, m, out, depth + 1);
                        }
                    }
                }
            }
            LayerKind::Null => {}
        }
    }
}

/// A camada aparece no quadro `t` (da composição dela).
fn visible(layer: &Layer, t: f32) -> bool {
    t >= layer.in_point && t < layer.out_point
}

/// A transformação da camada `i` com a dos pais (o parentesco não leva a opacidade).
fn layer_matrix(layers: &[Layer], i: usize, t: f32, depth: usize) -> Affine {
    let layer = &layers[i];
    let local = layer.transform.matrix(t);
    match layer.parent.and_then(|p| layers.iter().position(|l| l.index == Some(p))) {
        Some(parent) if depth < 16 && parent != i => layer_matrix(layers, parent, t, depth + 1).then(local),
        _ => local,
    }
}

// ---------------------------------------------------------------------------
// Formas

/// O caminho de um valor achatado (`fechado, n, v i o...`).
fn shape_path(v: &[f32], reversed: bool) -> Path {
    let mut path = Path::new();
    if v.len() < 2 {
        return path;
    }
    let closed = v[0] > 0.5;
    let n = (v[1].max(0.0) as usize).min((v.len() - 2) / 6);
    if n == 0 {
        return path;
    }
    let mut verts: Vec<[PointF; 3]> = (0..n)
        .map(|k| {
            let b = 2 + k * 6;
            let p = PointF::new(v[b], v[b + 1]);
            [p, PointF::new(p.x + v[b + 2], p.y + v[b + 3]), PointF::new(p.x + v[b + 4], p.y + v[b + 5])]
        })
        .collect();
    if reversed {
        verts.reverse();
        for vt in &mut verts {
            vt.swap(1, 2);
        }
    }
    path.move_to(verts[0][0].x, verts[0][0].y);
    let seg = |path: &mut Path, a: &[PointF; 3], b: &[PointF; 3]| {
        path.cubic_to(a[2].x, a[2].y, b[1].x, b[1].y, b[0].x, b[0].y);
    };
    for k in 1..n {
        seg(&mut path, &verts[k - 1], &verts[k]);
    }
    if closed {
        seg(&mut path, &verts[n - 1], &verts[0]);
        path.close();
    }
    path
}

const KAPPA: f32 = 0.552_284_8;

fn ellipse_path(c: PointF, size: PointF, reversed: bool) -> Path {
    let (rx, ry) = (size.x / 2.0, size.y / 2.0);
    let (kx, ky) = (rx * KAPPA, ry * KAPPA);
    // Começa em cima e vai no sentido horário (como o After Effects).
    let v = [
        1.0, 4.0, c.x, c.y - ry, -kx, 0.0, kx, 0.0, c.x + rx, c.y, 0.0, -ky, 0.0, ky, c.x, c.y + ry, kx, 0.0, -kx, 0.0, c.x - rx, c.y, 0.0, ky,
        0.0, -ky,
    ];
    shape_path(&v, reversed)
}

fn rect_shape(c: PointF, size: PointF, radius: f32, reversed: bool) -> Path {
    let (w, h) = (size.x / 2.0, size.y / 2.0);
    let r = radius.min(w).min(h).max(0.0);
    let (l, t, rt, b) = (c.x - w, c.y - h, c.x + w, c.y + h);
    let v: Vec<f32> = if r <= 0.0 {
        vec![1.0, 4.0, rt, t, 0.0, 0.0, 0.0, 0.0, rt, b, 0.0, 0.0, 0.0, 0.0, l, b, 0.0, 0.0, 0.0, 0.0, l, t, 0.0, 0.0, 0.0, 0.0]
    } else {
        let k = r * KAPPA;
        vec![
            1.0, 8.0, rt, t + r, 0.0, -k, 0.0, 0.0, rt, b - r, 0.0, 0.0, 0.0, k, rt - r, b, k, 0.0, 0.0, 0.0, l + r, b, 0.0, 0.0, -k, 0.0, l,
            b - r, 0.0, k, 0.0, 0.0, l, t + r, 0.0, 0.0, 0.0, -k, l + r, t, -k, 0.0, 0.0, 0.0, rt - r, t, 0.0, 0.0, k, 0.0,
        ]
    };
    shape_path(&v, reversed)
}

fn geometry(item: &Item, t: f32) -> Option<Path> {
    Some(match item {
        Item::Shape { path, reversed } => shape_path(&path.at(t), *reversed),
        Item::Ellipse { center, size, reversed } => ellipse_path(center.point(t), size.point(t), *reversed),
        Item::Rect { center, size, radius, reversed } => rect_shape(center.point(t), size.point(t), radius.scalar(t), *reversed),
        _ => return None,
    })
}

/// As formas dos itens (e dos grupos dentro deles), já transformadas, com os "Trim Paths"
/// aplicados às formas que vêm antes deles.
fn collect(items: &[Item], m: Affine, t: f32, out: &mut Path) {
    let mut local = Path::new();
    for item in items {
        match item {
            Item::Group { items, transform } => {
                let gm = transform.as_ref().map_or(m, |tr| m.then(tr.matrix(t)));
                collect(items, gm, t, &mut local);
            }
            Item::Trim { start, end, offset } => {
                let (s, e, o) = (start.scalar(t) / 100.0, end.scalar(t) / 100.0, offset.scalar(t) / 360.0);
                local = trim(&local, s, e, o);
            }
            other => {
                if let Some(p) = geometry(other, t) {
                    local.append(&m.path(&p));
                }
            }
        }
    }
    out.append(&local);
}

fn color_of(v: &[f32], opacity: f32) -> Color {
    let c = |i: usize| (v.get(i).copied().unwrap_or(0.0).clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = v.get(3).copied().unwrap_or(1.0).clamp(0.0, 1.0) * opacity.clamp(0.0, 1.0);
    Color::rgba(c(0), c(1), c(2), (a * 255.0).round() as u8)
}

fn brush_of(paint: &Paint, m: Affine, t: f32, opacity: f32) -> Brush {
    match paint {
        Paint::Solid(c) => Brush::Solid(color_of(&c.at(t), opacity)),
        Paint::Gradient(g) => {
            let raw = g.stops.at(t);
            let count = g.count.min(raw.len() / 4);
            let alphas: Vec<(f32, f32)> = raw[count * 4..].chunks_exact(2).map(|c| (c[0], c[1])).collect();
            let alpha_at = |x: f32| -> f32 {
                if alphas.is_empty() {
                    return 1.0;
                }
                if x <= alphas[0].0 {
                    return alphas[0].1;
                }
                for w in alphas.windows(2) {
                    if x <= w[1].0 {
                        let span = (w[1].0 - w[0].0).max(1e-6);
                        return w[0].1 + (w[1].1 - w[0].1) * (x - w[0].0) / span;
                    }
                }
                alphas[alphas.len() - 1].1
            };
            let stops: Vec<GradientStop> = (0..count)
                .map(|k| {
                    let s = &raw[k * 4..k * 4 + 4];
                    GradientStop::new(s[0], color_of(&[s[1], s[2], s[3], alpha_at(s[0])], opacity))
                })
                .collect();
            if stops.is_empty() {
                return Brush::Solid(Color::TRANSPARENT);
            }
            let (s, e) = (g.start.point(t), g.end.point(t));
            if g.radial {
                Brush::Radial { center: m.apply(s), radius: (dist(s, e) * m.scale_factor()).max(0.01), stops }
            } else {
                Brush::Linear { start: m.apply(s), end: m.apply(e), stops }
            }
        }
    }
}

/// Desenha um grupo: de baixo para cima (o primeiro item fica por cima, como no After
/// Effects); cada estilo pinta as formas que vêm antes dele no grupo.
fn draw_group(canvas: &mut dyn Canvas, items: &[Item], m: Affine, opacity: f32, t: f32) {
    for idx in (0..items.len()).rev() {
        match &items[idx] {
            Item::Group { items: inner, transform } => {
                let (gm, go) = match transform {
                    Some(tr) => (m.then(tr.matrix(t)), opacity * tr.opacity(t)),
                    None => (m, opacity),
                };
                if go > 0.001 {
                    draw_group(canvas, inner, gm, go, t);
                }
            }
            Item::Fill { paint, opacity: o, even_odd } => {
                let mut path = Path::new();
                collect(&items[..idx], m, t, &mut path);
                if has_figures(&path) {
                    path.even_odd = *even_odd;
                    canvas.fill_path(&path, &brush_of(paint, m, t, opacity * o.scalar(t) / 100.0));
                }
            }
            Item::Stroke { paint, opacity: o, style } => {
                let width = style.width.scalar(t) * m.scale_factor();
                if width <= 0.0 {
                    continue;
                }
                let mut path = Path::new();
                collect(&items[..idx], m, t, &mut path);
                if has_figures(&path) {
                    let mut stroke = Stroke::new(brush_of(paint, m, t, opacity * o.scalar(t) / 100.0), width);
                    stroke.cap = style.cap;
                    stroke.join = style.join;
                    canvas.stroke_path(&path, &stroke);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyframes_ease_and_hold() {
        let v: Value = serde_json::from_str(
            r#"{"a":1,"k":[{"t":0,"s":[0],"o":{"x":[0.333],"y":[0]},"i":{"x":[0.667],"y":[1]}},{"t":10,"s":[100],"h":1},{"t":20,"s":[50]}]}"#,
        )
        .unwrap();
        let p = Prop::parse(Some(&v), numbers, &[0.0]);
        assert_eq!(p.scalar(-5.0), 0.0);
        assert!((p.scalar(5.0) - 50.0).abs() < 1.0, "meio da curva simétrica: {}", p.scalar(5.0));
        assert!(p.scalar(2.0) < 20.0, "começa devagar");
        assert_eq!(p.scalar(15.0), 100.0, "segura");
        assert_eq!(p.scalar(30.0), 50.0);
    }

    #[test]
    fn a_shape_path_is_rebuilt_from_vertices() {
        let v = bezier(&serde_json::json!({"c": true, "v": [[0,0],[10,0],[10,10]], "i": [[0,0],[0,0],[0,0]], "o": [[0,0],[0,0],[0,0]]})).unwrap();
        let p = shape_path(&v, false);
        let b = p.bounds();
        assert_eq!((b.width, b.height), (10.0, 10.0));
        assert!(matches!(p.cmds.last(), Some(aylis_core::PathCmd::Close)));
    }
}
