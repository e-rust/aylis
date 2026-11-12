//! Os gráficos com eixos: linha, barra, área, dispersão, bolhas, os empilhados, barras
//! deitadas e o misto.
//!
//! - Eixo Y: à esquerda e, se alguma série pedir, à direita; linear ou logarítmico,
//!   automático ou de `y_min` a `y_max`, invertido com `reverse`.
//! - Eixo X: os rótulos (categorias), datas (`time_axis`, posições pelo tempo) ou números
//!   (dispersão e bolhas); o zoom mostra uma parte dele.
//! - Séries: linha reta, em degraus ou suavizada (monotônica: não passa dos pontos),
//!   tracejada, com pontos de vários desenhos e área até o zero, o fundo, o topo ou outra
//!   série.

use aylis_core::{Brush, Canvas, Color, PaintCtx, Path, PointF, RectF, Stroke, TextAlign};

use super::{Chart, ChartKind, Hit, PointStyle, Series, SeriesFill, SeriesKind, Shape, nice_scale, short, small_text};

/// Como uma série é desenhada neste gráfico.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Draw {
    Line,
    Bar,
    Area,
    Scatter,
    Bubble,
}

fn draw_of(kind: ChartKind, s: &Series) -> Draw {
    match kind {
        ChartKind::Bar | ChartKind::StackedBar | ChartKind::HorizontalBar => Draw::Bar,
        ChartKind::Area | ChartKind::StackedArea => Draw::Area,
        ChartKind::Scatter => Draw::Scatter,
        ChartKind::Bubble => Draw::Bubble,
        ChartKind::Mixed => match s.kind {
            Some(SeriesKind::Bar) => Draw::Bar,
            Some(SeriesKind::Area) => Draw::Area,
            Some(SeriesKind::Scatter) => Draw::Scatter,
            _ => Draw::Line,
        },
        _ => Draw::Line,
    }
}

fn stacked(kind: ChartKind) -> bool {
    matches!(kind, ChartKind::StackedBar | ChartKind::StackedLine | ChartKind::StackedArea)
}

/// Um eixo de valores.
#[derive(Clone, Copy, Debug)]
struct Axis {
    min: f64,
    max: f64,
    step: f64,
    log: bool,
    reverse: bool,
}

impl Axis {
    /// De `lo` a `hi` (no logarítmico, `lo` é o menor positivo); `fixed`: mínimo e máximo à mão.
    fn new(lo: f64, hi: f64, log: bool, reverse: bool, fixed: Option<(f64, f64)>) -> Axis {
        if log {
            let lo = if lo > 0.0 && lo.is_finite() { lo } else { 1.0 };
            let hi = if hi > lo { hi } else { lo * 10.0 };
            let (mut min, mut max) = (10f64.powf(lo.log10().floor()), 10f64.powf(hi.log10().ceil()));
            if let Some((a, b)) = fixed.filter(|(a, b)| *a > 0.0 && b > a) {
                (min, max) = (a, b);
            }
            if max <= min {
                max = min * 10.0;
            }
            return Axis { min, max, step: 0.0, log, reverse };
        }
        let (min, max, step) = match fixed {
            Some((a, b)) if b > a => (a, b, nice_scale(a, b).2),
            _ => nice_scale(lo, hi),
        };
        Axis { min, max, step, log, reverse }
    }

    /// Onde `v` cai, de 0 (embaixo) a 1 (em cima).
    fn frac(&self, v: f64) -> f64 {
        let t = if self.log {
            (v.max(self.min * 1e-6).log10() - self.min.log10()) / (self.max.log10() - self.min.log10())
        } else {
            (v - self.min) / (self.max - self.min)
        };
        if self.reverse { 1.0 - t } else { t }
    }

    fn ticks(&self) -> Vec<f64> {
        let mut out = Vec::new();
        if self.log {
            let mut v = self.min;
            while v <= self.max * 1.0001 {
                out.push(v);
                v *= 10.0;
            }
        } else if self.step > 0.0 {
            let mut v = self.min;
            while v <= self.max + self.step * 0.01 {
                out.push(v);
                v += self.step;
            }
        }
        out
    }

    /// Onde fica o zero (ou a ponta de baixo, se o zero não estiver no eixo).
    fn base(&self) -> f64 {
        if self.log { self.min } else { 0.0f64.clamp(self.min, self.max) }
    }
}

/// O eixo X.
#[derive(Clone, Copy, Debug)]
enum XAxis {
    /// Um lugar por rótulo (`n` deles).
    Category(usize),
    /// Datas (dias desde 1970): de `lo` a `hi`.
    Time(f64, f64),
    /// Números (dispersão e bolhas).
    Value(f64, f64),
}

/// Um ponto de uma série: o índice, o X (no modo do eixo), o valor e a base (no
/// empilhado, onde a série de baixo terminou).
#[derive(Clone, Copy, Debug)]
struct Point {
    index: usize,
    x: f64,
    y: f64,
    base: f64,
}

/// Monta e desenha. `area`: o espaço do gráfico (sem título nem legenda).
pub(super) fn paint(chart: &Chart, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
    if chart.kind == ChartKind::HorizontalBar {
        return paint_horizontal(chart, canvas, ctx, area, data);
    }
    let kind = chart.kind;
    let n = chart.points();
    // Dispersão antiga: sem X nos pontos, a 1ª série é o X das outras.
    let legacy_x = kind == ChartKind::Scatter && data.len() >= 2 && data.iter().all(|s| s.x.is_empty());
    let times: Option<Vec<Option<f64>>> = chart.time_axis.then(|| chart.data_labels.iter().map(|l| parse_time(l)).collect());
    let numeric = matches!(kind, ChartKind::Scatter | ChartKind::Bubble) || data.iter().any(|s| !s.hidden && matches!(draw_of(kind, s), Draw::Scatter | Draw::Bubble) && !s.x.is_empty());
    // Os pontos de cada série visível.
    let mut series_pts: Vec<(usize, Draw, Vec<Point>)> = Vec::new();
    for (si, s) in data.iter().enumerate() {
        if s.hidden || (legacy_x && si == 0) {
            continue;
        }
        let draw = draw_of(kind, s);
        let pts = s
            .values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let x = if legacy_x {
                    *data[0].values.get(i)?
                } else if numeric {
                    s.x.get(i).copied().unwrap_or(i as f64)
                } else if let Some(t) = &times {
                    t.get(i).copied().flatten()?
                } else {
                    i as f64
                };
                Some(Point { index: i, x, y: *v, base: 0.0 })
            })
            .collect();
        series_pts.push((si, draw, pts));
    }
    // Empilhar: cada série começa onde as de baixo (no mesmo eixo) terminaram; positivos
    // e negativos separados.
    if stacked(kind) {
        let mut pos: std::collections::HashMap<(bool, usize), f64> = Default::default();
        let mut neg: std::collections::HashMap<(bool, usize), f64> = Default::default();
        for (si, _, pts) in series_pts.iter_mut() {
            let right = data[*si].right_axis;
            for p in pts.iter_mut() {
                let acc = if p.y >= 0.0 { pos.entry((right, p.index)).or_insert(0.0) } else { neg.entry((right, p.index)).or_insert(0.0) };
                p.base = *acc;
                *acc += p.y;
                p.y = *acc;
            }
        }
    }
    // As faixas de cada eixo.
    let range = |right: bool| {
        let mut lo = f64::MAX;
        let mut hi = f64::MIN;
        let mut min_pos = f64::MAX;
        let mut any = false;
        for (si, draw, pts) in &series_pts {
            if data[*si].right_axis != right {
                continue;
            }
            any = true;
            for p in pts {
                for v in [p.y, if stacked(kind) || matches!(draw, Draw::Bar | Draw::Area) { p.base } else { p.y }] {
                    lo = lo.min(v);
                    hi = hi.max(v);
                    if v > 0.0 {
                        min_pos = min_pos.min(v);
                    }
                }
            }
        }
        any.then_some((lo, hi, min_pos))
    };
    let fixed = (!chart.auto_scale).then_some((chart.y_min, chart.y_max));
    let make_axis = |r: Option<(f64, f64, f64)>, fixed: Option<(f64, f64)>| {
        let (lo, hi, min_pos) = r.unwrap_or((0.0, 1.0, 1.0));
        let (lo, hi) = if lo > hi { (0.0, 1.0) } else if chart.log_scale { (min_pos, hi) } else { (lo.min(0.0), hi.max(0.0)) };
        // Dispersão e bolhas: folga em cima (a bolha não corta na borda).
        let hi = if numeric && !chart.log_scale { hi + (hi - lo) * if kind == ChartKind::Bubble { 0.12 } else { 0.05 } } else { hi };
        Axis::new(lo, hi, chart.log_scale, chart.reverse, fixed)
    };
    let left = make_axis(range(false).or_else(|| range(true)), fixed);
    let right = range(true).filter(|_| range(false).is_some()).map(|r| make_axis(Some(r), None));

    // O eixo X.
    let x_axis = if numeric || legacy_x {
        let (lo, hi) = series_pts.iter().flat_map(|(_, _, p)| p.iter().map(|p| p.x)).fold((f64::MAX, f64::MIN), |(a, b), x| (a.min(x), b.max(x)));
        let (lo, hi) = if lo > hi { (0.0, 1.0) } else { (lo, hi) };
        // Uma folga nas pontas: os pontos (e as bolhas) não ficam em cima da borda.
        let pad = (hi - lo).max(1e-9) * if kind == ChartKind::Bubble { 0.12 } else { 0.05 };
        let (min, max, _) = nice_scale(lo - pad, hi + pad);
        XAxis::Value(min, max)
    } else if let Some(t) = &times {
        let (lo, hi) = t.iter().flatten().fold((f64::MAX, f64::MIN), |(a, b), x| (a.min(*x), b.max(*x)));
        if lo > hi { XAxis::Category(n) } else if hi - lo < 1e-9 { XAxis::Time(lo - 0.5, hi + 0.5) } else { XAxis::Time(lo - (hi - lo) * 0.03, hi + (hi - lo) * 0.03) }
    } else {
        XAxis::Category(n)
    };

    // Margens dos rótulos.
    let small = small_text(ctx);
    let label = |v: f64| short(v);
    let width_of = |canvas: &mut dyn Canvas, a: &Axis| a.ticks().iter().map(|v| canvas.measure_text(&label(*v), &small).width).fold(0.0, f32::max) + 10.0;
    let left_w = width_of(canvas, &left);
    let right_w = right.as_ref().map_or(8.0, |a| width_of(canvas, a));
    // Rótulos do X inclinados (`label_angle`): mais espaço embaixo, pelo maior deles.
    let angle = chart.label_angle.clamp(-90.0, 90.0);
    let below = if angle.abs() > 0.5 && matches!(x_axis, XAxis::Category(_)) {
        let longest = chart.data_labels.iter().map(|l| canvas.measure_text(l, &small).width).fold(0.0, f32::max).min(area.height * 0.4);
        longest * angle.to_radians().sin().abs() + 22.0
    } else {
        26.0
    };
    let plot = RectF::new(area.x + left_w, area.y + 4.0, (area.width - left_w - right_w).max(10.0), (area.height - below).max(10.0));
    chart.plot.set(plot);
    let y_px = |a: &Axis, v: f64| plot.bottom() - a.frac(v) as f32 * plot.height;

    // Grade e rótulos do eixo Y (a grade só do da esquerda).
    let grid = Stroke::new(ctx.theme.color("border").with_opacity(0.8), 1.0);
    let mut yl = small.clone();
    yl.h_align = TextAlign::Trailing;
    for t in left.ticks() {
        let y = y_px(&left, t).round() + 0.5;
        canvas.draw_line(PointF::new(plot.x, y), PointF::new(plot.right(), y), &grid);
        canvas.draw_text(&label(t), RectF::new(area.x, y - 9.0, left_w - 6.0, 18.0), &yl);
    }
    if let Some(r) = &right {
        let mut rl = small.clone();
        rl.h_align = TextAlign::Leading;
        for t in r.ticks() {
            let y = y_px(r, t).round() + 0.5;
            canvas.draw_line(PointF::new(plot.right(), y), PointF::new(plot.right() + 4.0, y), &grid);
            canvas.draw_text(&label(t), RectF::new(plot.right() + 6.0, y - 9.0, right_w - 6.0, 18.0), &rl);
        }
    }

    // O eixo X, com o zoom.
    let (w0, w1) = chart.window.get();
    let span = (w1 - w0).max(1e-6);
    let x_frac = |x: f64| -> f64 {
        let t = match x_axis {
            XAxis::Category(n) => (x + 0.5) / n.max(1) as f64,
            XAxis::Time(lo, hi) => (x - lo) / (hi - lo),
            XAxis::Value(lo, hi) => (x - lo) / (hi - lo),
        };
        (t - w0) / span
    };
    let x_px = |x: f64| plot.x + x_frac(x) as f32 * plot.width;
    // A largura de um lugar (barras, colunas da dica).
    let slot = match x_axis {
        XAxis::Category(n) => plot.width / (n.max(1) as f64 * span) as f32,
        _ => {
            let count = series_pts.iter().map(|(_, _, p)| p.len()).max().unwrap_or(1).max(1);
            plot.width / (count as f64 * span) as f32
        }
    };
    let mut xl = small.clone();
    xl.h_align = TextAlign::Center;
    match x_axis {
        XAxis::Category(n) => {
            // Inclinados, cabem mais (a altura do texto conta, não a largura).
            let need = if angle.abs() > 0.5 { 20.0 } else { 50.0 };
            let every = ((need / slot).ceil() as usize).max(1);
            for (i, text) in chart.data_labels.iter().enumerate().take(n) {
                let x = x_px(i as f64);
                if i % every == 0 && x >= plot.x - 1.0 && x <= plot.right() + 1.0 {
                    if angle.abs() > 0.5 {
                        // Gira em volta do ponto embaixo do rótulo; o texto termina nele.
                        let mut tl = small.clone();
                        tl.h_align = TextAlign::Trailing;
                        canvas.save();
                        canvas.translate(x, plot.bottom() + 6.0);
                        canvas.rotate(-angle);
                        canvas.draw_text(text, RectF::new(-(below * 3.0), -9.0, below * 3.0, 18.0), &tl);
                        canvas.restore();
                    } else {
                        canvas.draw_text(text, RectF::new(x - slot.max(50.0) / 2.0, plot.bottom() + 4.0, slot.max(50.0), 18.0), &xl);
                    }
                }
            }
        }
        XAxis::Time(lo, hi) => {
            let (a, b) = (lo + (hi - lo) * w0, lo + (hi - lo) * w1);
            for (t, text) in time_ticks(a, b, (plot.width / 70.0).max(2.0) as usize) {
                let x = x_px(t);
                canvas.draw_line(PointF::new(x.round() + 0.5, plot.bottom()), PointF::new(x.round() + 0.5, plot.bottom() + 4.0), &grid);
                canvas.draw_text(&text, RectF::new(x - 45.0, plot.bottom() + 4.0, 90.0, 18.0), &xl);
            }
        }
        XAxis::Value(lo, hi) => {
            let (a, b) = (lo + (hi - lo) * w0, lo + (hi - lo) * w1);
            let (min, max, step) = nice_scale(a, b);
            let mut t = min;
            while t <= max + step * 0.01 {
                let x = x_px(t);
                if x >= plot.x - 1.0 && x <= plot.right() + 1.0 {
                    canvas.draw_text(&short(t), RectF::new(x - 30.0, plot.bottom() + 4.0, 60.0, 18.0), &xl);
                }
                t += step;
            }
        }
    }

    canvas.save();
    canvas.clip_rect(plot.inflate(1.0));
    let theme = ctx.theme;
    let hovered = chart.hovered.get();
    let hovered_column = hovered.filter(Hit::is_column).map(|h| h.index);
    if let Some(i) = hovered_column {
        let x = match x_axis {
            XAxis::Category(_) => x_px(i as f64),
            _ => series_pts.iter().find_map(|(_, _, p)| p.iter().find(|p| p.index == i)).map_or(-1e9, |p| x_px(p.x)),
        };
        canvas.fill_rect(RectF::new(x - slot / 2.0, plot.y, slot, plot.height), 0.0, &Brush::Solid(theme.color("hover_overlay")));
    }
    let axis_of = |si: usize| if data[si].right_axis { right.as_ref().unwrap_or(&left) } else { &left };
    let pixel = |si: usize, p: &Point| PointF::new(x_px(p.x), y_px(axis_of(si), p.y));
    let base_pixel = |si: usize, p: &Point| PointF::new(x_px(p.x), y_px(axis_of(si), p.base));

    // Áreas primeiro (ficam por baixo de tudo).
    for (si, draw, pts) in &series_pts {
        let s = &data[*si];
        if pts.is_empty() || !matches!(draw, Draw::Line | Draw::Area) {
            continue;
        }
        let fill = match (s.fill, draw) {
            (SeriesFill::None, Draw::Area) => SeriesFill::Origin,
            (f, _) => f,
        };
        if fill == SeriesFill::None {
            continue;
        }
        let top: Vec<PointF> = pts.iter().map(|p| pixel(*si, p)).collect();
        let mut path = line_path(&top, chart.stepped, chart.tension);
        let axis = axis_of(*si);
        let bottom: Vec<PointF> = match fill {
            // Empilhado: até a série de baixo.
            SeriesFill::Origin if stacked(kind) => pts.iter().map(|p| base_pixel(*si, p)).collect(),
            SeriesFill::Origin => vec![PointF::new(top[top.len() - 1].x, y_px(axis, axis.base())), PointF::new(top[0].x, y_px(axis, axis.base()))],
            SeriesFill::Start => vec![PointF::new(top[top.len() - 1].x, plot.bottom()), PointF::new(top[0].x, plot.bottom())],
            SeriesFill::End => vec![PointF::new(top[top.len() - 1].x, plot.y), PointF::new(top[0].x, plot.y)],
            SeriesFill::Series(j) => match series_pts.iter().find(|(sj, _, _)| *sj == j) {
                Some((sj, _, other)) => other.iter().map(|p| pixel(*sj, p)).collect(),
                None => continue,
            },
            SeriesFill::None => continue,
        };
        // A borda de baixo, de trás para a frente.
        let mut back = bottom;
        if back.len() > 2 || matches!(fill, SeriesFill::Series(_)) || (stacked(kind) && fill == SeriesFill::Origin) {
            back.reverse();
        }
        for p in &back {
            path.line_to(p.x, p.y);
        }
        path.close();
        canvas.fill_path(&path, &Brush::Solid(chart.color(ctx, *si).with_opacity(chart.area_opacity.clamp(0.0, 1.0))));
    }

    // Barras: agrupadas no lugar do rótulo (empilhadas, uma por cima da outra).
    let bar_series: Vec<usize> = series_pts.iter().filter(|(_, d, _)| *d == Draw::Bar).map(|(si, _, _)| *si).collect();
    let groups = if stacked(kind) { 1 } else { bar_series.len().max(1) };
    let group_w = slot * 0.72;
    let bar_w = group_w / groups as f32;
    for (si, draw, pts) in &series_pts {
        if *draw != Draw::Bar {
            continue;
        }
        let slot_index = if stacked(kind) { 0 } else { bar_series.iter().position(|b| b == si).unwrap_or(0) };
        let color = chart.color(ctx, *si);
        let axis = axis_of(*si);
        for p in pts {
            let x = x_px(p.x) - group_w / 2.0 + bar_w * slot_index as f32;
            let (y0, y1) = (y_px(axis, if stacked(kind) { p.base } else { axis.base() }), y_px(axis, p.y));
            let rect = RectF::from_points(PointF::new(x + 1.0, y1), PointF::new(x + bar_w - 1.0, y0));
            let lit = hovered_column == Some(p.index);
            canvas.fill_rect(rect, 3.0_f32.min(bar_w / 3.0), &Brush::Solid(if lit { color.lerp(Color::WHITE, 0.15) } else { color }));
        }
    }

    // Linhas e os pontos delas.
    for (si, draw, pts) in &series_pts {
        if !matches!(draw, Draw::Line | Draw::Area) || pts.is_empty() {
            continue;
        }
        let s = &data[*si];
        let color = chart.color(ctx, *si);
        let screen: Vec<PointF> = pts.iter().map(|p| pixel(*si, p)).collect();
        let path = line_path(&screen, chart.stepped, chart.tension);
        canvas.stroke_path(&path, &Stroke::new(color, s.width.unwrap_or(2.5)).dashed(s.dash.clone()).round());
        for (p, at) in pts.iter().zip(&screen) {
            let rad = if hovered_column == Some(p.index) { 5.0 } else { 3.5 };
            draw_point(canvas, s.point, *at, rad, color, ctx.style.fill);
        }
    }

    // Dispersão e bolhas (cada ponto tem a sua dica).
    let mut hits = chart.hits.borrow_mut();
    for (si, draw, pts) in &series_pts {
        if !matches!(draw, Draw::Scatter | Draw::Bubble) {
            continue;
        }
        let s = &data[*si];
        let color = chart.color(ctx, *si);
        for p in pts {
            let at = pixel(*si, p);
            let rad = if *draw == Draw::Bubble { s.radius.get(p.index).copied().unwrap_or(6.0).max(1.0) as f32 } else { 4.0 };
            let lit = hovered.is_some_and(|h| !h.is_column() && h.series == *si && h.index == p.index);
            if *draw == Draw::Bubble {
                let r = RectF::new(at.x - rad, at.y - rad, rad * 2.0, rad * 2.0);
                canvas.fill_ellipse(r, &Brush::Solid(color.with_opacity(if lit { 0.8 } else { 0.55 })));
                canvas.stroke_ellipse(r, &Stroke::new(color, 1.5));
            } else {
                let style = if s.point == PointStyle::None { PointStyle::Circle } else { s.point };
                draw_point(canvas, style, at, if lit { rad + 1.5 } else { rad }, color.with_opacity(0.85), color.with_opacity(0.85));
            }
            hits.push(Hit { shape: Shape::Circle(at, rad), series: *si, index: p.index, at });
        }
    }
    canvas.restore();

    // Uma coluna por rótulo para a dica (todas as séries nele), nos tipos de linha e barra.
    let columns = series_pts.iter().any(|(_, d, _)| matches!(d, Draw::Line | Draw::Bar | Draw::Area));
    if columns {
        let mut seen = std::collections::BTreeMap::new();
        for (si, draw, pts) in &series_pts {
            if !matches!(draw, Draw::Line | Draw::Bar | Draw::Area) {
                continue;
            }
            for p in pts {
                let at = pixel(*si, p);
                let e = seen.entry(p.index).or_insert(at);
                e.y = e.y.min(at.y);
            }
        }
        for (index, at) in seen {
            let column = RectF::new(at.x - slot / 2.0, plot.y, slot, plot.height);
            if column.right() >= plot.x && column.x <= plot.right() {
                hits.insert(0, Hit { shape: Shape::Column(column), series: 0, index, at });
            }
        }
    }
}

/// Barras deitadas: os rótulos no eixo vertical, os valores no horizontal.
fn paint_horizontal(chart: &Chart, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
    let n = chart.points();
    let visible: Vec<usize> = (0..data.len()).filter(|i| !data[*i].hidden).collect();
    if n == 0 || visible.is_empty() {
        return;
    }
    let (mut lo, mut hi, mut min_pos) = (f64::MAX, f64::MIN, f64::MAX);
    for &si in &visible {
        for v in &data[si].values {
            lo = lo.min(*v);
            hi = hi.max(*v);
            if *v > 0.0 {
                min_pos = min_pos.min(*v);
            }
        }
    }
    // Empilhadas (`stacked`): a escala vai até a soma de cada linha (relatório do InsightDB,
    // 11/11: não havia barra deitada empilhada).
    if chart.stacked {
        for i in 0..n {
            let (neg, pos) = visible.iter().filter_map(|&si| data[si].values.get(i)).fold((0.0, 0.0), |(a, b), v| if *v < 0.0 { (a + v, b) } else { (a, b + v) });
            lo = lo.min(neg);
            hi = hi.max(pos);
        }
    }
    let (lo, hi) = if lo > hi { (0.0, 1.0) } else if chart.log_scale { (min_pos, hi) } else { (lo.min(0.0), hi.max(0.0)) };
    let axis = Axis::new(lo, hi, chart.log_scale, chart.reverse, (!chart.auto_scale).then_some((chart.y_min, chart.y_max)));
    let small = small_text(ctx);
    let label_w = chart.data_labels.iter().map(|l| canvas.measure_text(l, &small).width).fold(0.0, f32::max).min(area.width * 0.35) + 10.0;
    let plot = RectF::new(area.x + label_w, area.y + 4.0, (area.width - label_w - 12.0).max(10.0), (area.height - 26.0).max(10.0));
    chart.plot.set(plot);
    let x_px = |v: f64| plot.x + axis.frac(v) as f32 * plot.width;
    let grid = Stroke::new(ctx.theme.color("border").with_opacity(0.8), 1.0);
    let mut xl = small.clone();
    xl.h_align = TextAlign::Center;
    for t in axis.ticks() {
        let x = x_px(t).round() + 0.5;
        canvas.draw_line(PointF::new(x, plot.y), PointF::new(x, plot.bottom()), &grid);
        canvas.draw_text(&short(t), RectF::new(x - 30.0, plot.bottom() + 4.0, 60.0, 18.0), &xl);
    }
    let row = plot.height / n as f32;
    let mut yl = small.clone();
    yl.h_align = TextAlign::Trailing;
    let hovered = chart.hovered.get().filter(Hit::is_column).map(|h| h.index);
    let group = row * 0.72;
    let bar_h = group / visible.len() as f32;
    let mut hits = chart.hits.borrow_mut();
    for i in 0..n {
        let y = plot.y + row * i as f32;
        if let Some(text) = chart.data_labels.get(i) {
            canvas.draw_text(text, RectF::new(area.x, y, label_w - 6.0, row), &yl);
        }
        if hovered == Some(i) {
            canvas.fill_rect(RectF::new(plot.x, y, plot.width, row), 0.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
        }
        let mut tip = PointF::new(plot.x, y + row / 2.0);
        let (mut pos, mut neg) = (axis.base(), axis.base());
        for (k, &si) in visible.iter().enumerate() {
            let Some(v) = data[si].values.get(i) else { continue };
            let (top, height, from, to) = if chart.stacked {
                // Uma barra só por linha, cada série continuando a anterior.
                let from = if *v < 0.0 { neg } else { pos };
                let to = from + v;
                if *v < 0.0 { neg = to } else { pos = to }
                (y + (row - group) / 2.0, group, from, to)
            } else {
                (y + (row - group) / 2.0 + bar_h * k as f32, bar_h, axis.base(), *v)
            };
            let (x0, x1) = (x_px(from), x_px(to));
            let rect = RectF::from_points(PointF::new(x0, top + 1.0), PointF::new(x1, top + height - 1.0));
            canvas.fill_rect(rect, if chart.stacked { 0.0 } else { 3.0_f32.min(height / 3.0) }, &Brush::Solid(chart.color(ctx, si)));
            tip.x = tip.x.max(x1.max(x0));
        }
        hits.push(Hit { shape: Shape::Column(RectF::new(plot.x, y, plot.width, row)), series: 0, index: i, at: tip });
    }
}

/// A linha pelos pontos: reta, em degraus ou suavizada (monotônica, `tension` de 0 a 1).
fn line_path(points: &[PointF], stepped: bool, tension: f32) -> Path {
    let mut path = Path::new();
    let Some(first) = points.first() else { return path };
    path.move_to(first.x, first.y);
    if stepped {
        for w in points.windows(2) {
            path.line_to(w[1].x, w[0].y).line_to(w[1].x, w[1].y);
        }
        return path;
    }
    if tension <= 0.0 || points.len() < 3 {
        for p in &points[1..] {
            path.line_to(p.x, p.y);
        }
        return path;
    }
    // Fritsch-Carlson: inclinações que não criam picos entre os pontos.
    let n = points.len();
    let d: Vec<f32> = points.windows(2).map(|w| if (w[1].x - w[0].x).abs() < 1e-6 { 0.0 } else { (w[1].y - w[0].y) / (w[1].x - w[0].x) }).collect();
    let mut m = vec![0.0f32; n];
    m[0] = d[0];
    m[n - 1] = d[n - 2];
    for i in 1..n - 1 {
        m[i] = if d[i - 1] * d[i] <= 0.0 { 0.0 } else { (d[i - 1] + d[i]) / 2.0 };
    }
    for i in 0..n - 1 {
        if d[i] == 0.0 {
            m[i] = 0.0;
            m[i + 1] = 0.0;
            continue;
        }
        let (a, b) = (m[i] / d[i], m[i + 1] / d[i]);
        let s = a * a + b * b;
        if s > 9.0 {
            let t = 3.0 / s.sqrt();
            m[i] = t * a * d[i];
            m[i + 1] = t * b * d[i];
        }
    }
    // O tamanho das alças cresce com a tensão (0,4 = a curva do Chart.js).
    let k = (tension / 0.4).clamp(0.0, 1.5);
    for i in 0..n - 1 {
        let (p0, p1) = (points[i], points[i + 1]);
        let dx = (p1.x - p0.x) / 3.0 * k;
        path.cubic_to(p0.x + dx, p0.y + m[i] * dx, p1.x - dx, p1.y - m[i + 1] * dx, p1.x, p1.y);
    }
    path
}

/// Um ponto no desenho `style`, de raio `r`: borda na cor da série, miolo em `inside`.
pub(super) fn draw_point(canvas: &mut dyn Canvas, style: PointStyle, at: PointF, r: f32, color: Color, inside: Color) {
    let stroke = Stroke::new(color, 2.0);
    match style {
        PointStyle::None => {}
        PointStyle::Circle => {
            let dot = RectF::new(at.x - r, at.y - r, r * 2.0, r * 2.0);
            canvas.fill_ellipse(dot, &Brush::Solid(inside));
            canvas.stroke_ellipse(dot, &stroke);
        }
        PointStyle::Square => {
            let dot = RectF::new(at.x - r, at.y - r, r * 2.0, r * 2.0);
            canvas.fill_rect(dot, 0.0, &Brush::Solid(inside));
            canvas.stroke_rect(dot, 0.0, &stroke);
        }
        PointStyle::Triangle | PointStyle::Diamond => {
            let pts: Vec<PointF> = if style == PointStyle::Triangle {
                vec![PointF::new(at.x, at.y - r * 1.2), PointF::new(at.x + r * 1.1, at.y + r * 0.8), PointF::new(at.x - r * 1.1, at.y + r * 0.8)]
            } else {
                vec![PointF::new(at.x, at.y - r * 1.3), PointF::new(at.x + r * 1.3, at.y), PointF::new(at.x, at.y + r * 1.3), PointF::new(at.x - r * 1.3, at.y)]
            };
            let mut p = Path::new();
            p.polygon(&pts, true);
            canvas.fill_path(&p, &Brush::Solid(inside));
            canvas.stroke_path(&p, &stroke);
        }
        PointStyle::Cross => {
            canvas.draw_line(PointF::new(at.x - r, at.y - r), PointF::new(at.x + r, at.y + r), &stroke);
            canvas.draw_line(PointF::new(at.x - r, at.y + r), PointF::new(at.x + r, at.y - r), &stroke);
        }
    }
}

// ---------------------------------------------------------------------------
// Datas do eixo de tempo (dias desde 1970-01-01, com a fração do dia).

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// `2026-10-27`, `2026-10-27 14:30[:05]` (ou com `T`), `2026-10` e `27/10/2026`, em dias.
pub fn parse_time(text: &str) -> Option<f64> {
    let text = text.trim();
    let (date, time) = match text.split_once(['T', ' ']) {
        Some((d, t)) => (d, Some(t)),
        None => (text, None),
    };
    let nums = |s: &str, sep: char| s.split(sep).map(|p| p.trim().parse::<i64>().ok()).collect::<Option<Vec<i64>>>();
    let (y, m, d) = if date.contains('/') {
        match nums(date, '/')?.as_slice() {
            [d, m, y] => (*y, *m, *d),
            _ => return None,
        }
    } else {
        match nums(date, '-')?.as_slice() {
            [y, m, d] => (*y, *m, *d),
            [y, m] => (*y, *m, 1),
            _ => return None,
        }
    };
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let mut days = days_from_civil(y, m as u32, d as u32) as f64;
    if let Some(t) = time {
        let parts = nums(t.trim_end_matches('Z'), ':')?;
        let (h, mi, s) = (parts.first().copied().unwrap_or(0), parts.get(1).copied().unwrap_or(0), parts.get(2).copied().unwrap_or(0));
        days += (h * 3600 + mi * 60 + s) as f64 / 86400.0;
    }
    Some(days)
}

/// As marcas do eixo de tempo entre `lo` e `hi` (dias), no máximo `max` delas: horas, dias,
/// semanas, meses ou anos, com o texto de cada uma.
fn time_ticks(lo: f64, hi: f64, max: usize) -> Vec<(f64, String)> {
    let span = (hi - lo).max(1e-9);
    let max = max.max(2) as f64;
    let date = |t: f64| civil_from_days(t.floor() as i64);
    // Horas e dias: passos fixos.
    for step in [1.0 / 24.0, 2.0 / 24.0, 3.0 / 24.0, 6.0 / 24.0, 12.0 / 24.0, 1.0, 2.0, 7.0, 14.0] {
        if span / step <= max {
            let mut out = Vec::new();
            let mut t = (lo / step).ceil() * step;
            while t <= hi + 1e-9 {
                let (y, m, d) = date(t);
                let text = if step < 1.0 {
                    let minutes = ((t - t.floor()) * 1440.0).round() as i64;
                    if span > 1.0 { format!("{d:02}/{m:02} {:02}:{:02}", minutes / 60, minutes % 60) } else { format!("{:02}:{:02}", minutes / 60, minutes % 60) }
                } else if span > 366.0 {
                    format!("{d:02}/{m:02}/{y}")
                } else {
                    format!("{d:02}/{m:02}")
                };
                out.push((t, text));
                t += step;
            }
            return out;
        }
    }
    // Meses e anos: do primeiro dia do mês.
    let (y0, m0, _) = date(lo);
    for months in [1i64, 2, 3, 6, 12, 24, 60, 120, 240, 600] {
        if span / (months as f64 * 30.44) <= max {
            let mut out = Vec::new();
            let mut index = y0 * 12 + (m0 as i64 - 1);
            index = (index + months - 1) / months * months;
            loop {
                let (y, m) = (index.div_euclid(12), (index.rem_euclid(12) + 1) as u32);
                let t = days_from_civil(y, m, 1) as f64;
                if t > hi + 1e-9 {
                    break;
                }
                if t >= lo - 1e-9 {
                    out.push((t, if months >= 12 { y.to_string() } else { format!("{m:02}/{y}") }));
                }
                index += months;
            }
            return out;
        }
    }
    Vec::new()
}
