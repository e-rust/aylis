//! Os tipos de gráfico de BI sem os eixos de sempre: cascata, funil, treemap e radar.

use aylis_core::{Brush, Canvas, Color, FontWeight, PaintCtx, Path, PointF, RectF, Stroke, TextAlign};

use super::{Chart, ChartKind, Hit, Series, Shape, nice_scale, short, small_text};

/// Eixo Y à esquerda com grade, de `min` a `max`. Devolve a área do gráfico.
fn y_axis(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, lo: f64, hi: f64, bottom: f32) -> (RectF, f64, f64) {
    let (min, max, step) = nice_scale(lo, hi);
    let s = small_text(ctx);
    let label_w = [min, max].iter().map(|v| canvas.measure_text(&short(*v), &s).width).fold(0.0, f32::max) + 10.0;
    let plot = RectF::new(area.x + label_w, area.y + 4.0, area.width - label_w, area.height - bottom - 4.0);
    let grid = Stroke::new(ctx.theme.color("border").with_opacity(0.8), 1.0);
    let mut l = s.clone();
    l.h_align = TextAlign::Trailing;
    let mut tick = min;
    while tick <= max + step * 0.01 {
        let y = (plot.bottom() - ((tick - min) / (max - min)) as f32 * plot.height).round() + 0.5;
        canvas.draw_line(PointF::new(plot.x, y), PointF::new(plot.right(), y), &grid);
        canvas.draw_text(&short(tick), RectF::new(area.x, y - 9.0, label_w - 6.0, 18.0), &l);
        tick += step;
    }
    (plot, min, max)
}

fn y_of(plot: RectF, min: f64, max: f64, v: f64) -> f32 {
    plot.bottom() - ((v - min) / (max - min)) as f32 * plot.height
}

/// Rótulos do eixo X, um por coluna (pulando se não couberem).
fn x_labels(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, plot: RectF, labels: &[String], n: usize) {
    if n == 0 {
        return;
    }
    let slot = plot.width / n as f32;
    let mut l = small_text(ctx);
    l.h_align = TextAlign::Center;
    let every = ((50.0 / slot).ceil() as usize).max(1);
    for (i, label) in labels.iter().enumerate().take(n) {
        if i % every == 0 {
            let x = plot.x + slot * (i as f32 + 0.5);
            canvas.draw_text(label, RectF::new(x - slot.max(50.0) / 2.0, plot.bottom() + 4.0, slot.max(50.0), 18.0), &l);
        }
    }
}

impl Chart {
    /// Os tipos deste módulo.
    pub(super) fn paint_more(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        match self.kind {
            ChartKind::Waterfall => self.paint_waterfall(canvas, ctx, area, data),
            ChartKind::Funnel => self.paint_funnel(canvas, ctx, area, data),
            ChartKind::Treemap => self.paint_treemap(canvas, ctx, area, data),
            ChartKind::Radar => self.paint_radar(canvas, ctx, area, data),
            ChartKind::Sankey => self.paint_sankey(canvas, ctx, area, data),
            ChartKind::Waffle => self.paint_waffle(canvas, ctx, area, data),
            _ => {}
        }
    }

    /// Waffle: 10x10 quadrados, cada um 1% do total da 1ª série, preenchidos na ordem dos
    /// rótulos (de cima para baixo, coluna a coluna, como uma barra de 100%).
    fn paint_waffle(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(s) = data.first() else { return };
        let items: Vec<(usize, f64)> = s.values.iter().copied().enumerate().filter(|(i, v)| *v > 0.0 && !self.label_hidden(*i)).collect();
        let cells = waffle_cells(&items.iter().map(|(_, v)| *v).collect::<Vec<_>>());
        let side = area.width.min(area.height);
        let gap = (side / 60.0).max(1.0);
        let cell = (side - gap * 9.0) / 10.0;
        let origin = PointF::new(area.x + (area.width - side) / 2.0, area.y + (area.height - side) / 2.0);
        let empty = ctx.theme.color("border").with_opacity(0.5);
        let mut hits = self.hits.borrow_mut();
        for k in 0..100 {
            let (col, row) = (k / 10, k % 10);
            let r = RectF::new(origin.x + col as f32 * (cell + gap), origin.y + row as f32 * (cell + gap), cell, cell);
            match cells.get(k).and_then(|c| *c).map(|c| items[c].0) {
                Some(i) => {
                    canvas.fill_rect(r, cell * 0.18, &Brush::Solid(self.color(ctx, i)));
                    hits.push(Hit { shape: Shape::Rect(r), series: 0, index: i, at: r.center() });
                }
                None => canvas.fill_rect(r, cell * 0.18, &Brush::Solid(empty)),
            }
        }
    }

    /// Sankey: os nós em colunas (cada um na coluna do caminho mais longo desde uma origem) e
    /// as faixas entre eles, com a largura pelo valor.
    fn paint_sankey(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(s) = data.first() else { return };
        let flows: Vec<(usize, &str, &str, f64)> = self
            .data_labels
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.label_hidden(*i))
            .filter_map(|(i, l)| {
                let (from, to) = l.split_once('>').or_else(|| l.split_once('→'))?;
                let v = s.values.get(i).copied().unwrap_or(0.0);
                (v > 0.0).then(|| (i, from.trim(), to.trim(), v))
            })
            .collect();
        let layout = sankey_layout(&flows.iter().map(|(_, a, b, v)| (*a, *b, *v)).collect::<Vec<_>>());
        if layout.nodes.is_empty() {
            return;
        }
        let mut text = small_text(ctx);
        text.color = ctx.theme.color("text");
        let label_w = layout.nodes.iter().map(|n| canvas.measure_text(&n.name, &text).width).fold(0.0, f32::max).min(area.width / 4.0) + 8.0;
        let node_w = 12.0;
        let plot = RectF::new(area.x + label_w, area.y + 4.0, (area.width - label_w * 2.0).max(node_w), area.height - 8.0);
        let columns = layout.columns.max(1);
        let pad = 10.0;
        // A mesma escala para todas as colunas: a mais cheia ocupa a altura toda.
        let scale = (0..columns)
            .map(|c| {
                let nodes: Vec<&SankeyNode> = layout.nodes.iter().filter(|n| n.column == c).collect();
                let total: f64 = nodes.iter().map(|n| n.size).sum();
                (plot.height - pad * nodes.len().saturating_sub(1) as f32).max(1.0) as f64 / total.max(1e-9)
            })
            .fold(f64::MAX, f64::min);
        let x_of = |c: usize| if columns == 1 { plot.x } else { plot.x + (plot.width - node_w) * c as f32 / (columns - 1) as f32 };
        let mut rects = vec![RectF::default(); layout.nodes.len()];
        for c in 0..columns {
            let ids: Vec<usize> = (0..layout.nodes.len()).filter(|i| layout.nodes[*i].column == c).collect();
            let used: f32 = ids.iter().map(|i| (layout.nodes[*i].size * scale) as f32).sum::<f32>() + pad * ids.len().saturating_sub(1) as f32;
            let mut y = plot.y + (plot.height - used) / 2.0;
            for i in ids {
                let h = ((layout.nodes[i].size * scale) as f32).max(1.0);
                rects[i] = RectF::new(x_of(c), y, node_w, h);
                y += h + pad;
            }
        }
        // As faixas: saem empilhadas da direita da origem e chegam empilhadas à esquerda do destino.
        let mut out_y: Vec<f32> = rects.iter().map(|r| r.y).collect();
        let mut in_y = out_y.clone();
        let mut hits = self.hits.borrow_mut();
        for ((index, _, _, v), (from, to)) in flows.iter().zip(&layout.links) {
            let t = (*v * scale) as f32;
            let (a, b) = (rects[*from], rects[*to]);
            let (x0, x1) = (a.right(), b.x);
            let (y0, y1) = (out_y[*from], in_y[*to]);
            out_y[*from] += t;
            in_y[*to] += t;
            let mid = (x0 + x1) / 2.0;
            let mut p = Path::new();
            p.move_to(x0, y0).cubic_to(mid, y0, mid, y1, x1, y1).line_to(x1, y1 + t).cubic_to(mid, y1 + t, mid, y0 + t, x0, y0 + t).close();
            let hovered = self.hovered.get().is_some_and(|h| h.index == *index);
            canvas.fill_path(&p, &Brush::Solid(self.color(ctx, *from).with_opacity(if hovered { 0.6 } else { 0.35 })));
            let at = PointF::new(mid, (y0 + y1 + t) / 2.0);
            hits.push(Hit { shape: Shape::Circle(at, (t / 2.0).max(4.0)), series: 0, index: *index, at });
        }
        for (i, (node, r)) in layout.nodes.iter().zip(&rects).enumerate() {
            canvas.fill_rect(*r, 2.0, &Brush::Solid(self.color(ctx, i)));
            let mut l = text.clone();
            let label = if node.column + 1 == columns {
                l.h_align = TextAlign::Leading;
                RectF::new(r.right() + 6.0, r.center().y - 9.0, label_w - 6.0, 18.0)
            } else {
                l.h_align = TextAlign::Trailing;
                RectF::new(r.x - label_w, r.center().y - 9.0, label_w - 6.0, 18.0)
            };
            canvas.draw_text(&node.name, label, &l);
        }
    }

    /// Cascata: a 1ª série são as variações (positivas sobem, negativas descem), com a
    /// barra do total no fim.
    fn paint_waterfall(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(s) = data.first() else { return };
        let mut running = 0.0;
        let mut steps = Vec::new();
        for v in &s.values {
            steps.push((running, running + v));
            running += v;
        }
        steps.push((0.0, running));
        let (lo, hi) = steps.iter().fold((0.0f64, 0.0f64), |(a, b), (s, e)| (a.min(*s).min(*e), b.max(*s).max(*e)));
        let (plot, min, max) = y_axis(canvas, ctx, area, lo, hi, 22.0);
        let mut labels = self.data_labels.clone();
        labels.truncate(s.values.len());
        labels.push("Total".into());
        x_labels(canvas, ctx, plot, &labels, steps.len());
        let slot = plot.width / steps.len() as f32;
        let (up, down, total) = (ctx.theme.color("success"), ctx.theme.color("danger"), ctx.theme.color("primary"));
        let link = Stroke::new(ctx.theme.color("text_muted"), 1.0).dashed(vec![3.0, 3.0]);
        let mut hits = self.hits.borrow_mut();
        for (i, (start, end)) in steps.iter().enumerate() {
            let last = i == steps.len() - 1;
            let color = if last { total } else if end >= start { up } else { down };
            let (y0, y1) = (y_of(plot, min, max, *start), y_of(plot, min, max, *end));
            let x = plot.x + slot * i as f32 + slot * 0.15;
            let bar = RectF::from_points(PointF::new(x, y0), PointF::new(x + slot * 0.7, y1));
            canvas.fill_rect(bar, 2.0, &Brush::Solid(color));
            if !last {
                canvas.draw_line(PointF::new(x + slot * 0.7, y1), PointF::new(x + slot, y1), &link);
                hits.push(Hit { shape: Shape::Rect(bar), series: 0, index: i, at: PointF::new(bar.center().x, bar.y) });
            }
        }
    }

    /// Funil: as etapas da 1ª série, centradas e proporcionais, com a conversão.
    fn paint_funnel(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(s) = data.first() else { return };
        let steps: Vec<(usize, f64)> = s.values.iter().copied().enumerate().filter(|(i, _)| !self.label_hidden(*i)).collect();
        let first = steps.first().map_or(0.0, |(_, v)| *v);
        let max = steps.iter().map(|(_, v)| *v).fold(0.0, f64::max);
        if max <= 0.0 {
            return;
        }
        let h = area.height / steps.len() as f32;
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        text.color = Color::WHITE;
        text.font.weight = FontWeight::Bold;
        let mut side = small_text(ctx);
        side.h_align = TextAlign::Leading;
        let label_w = area.width * 0.28;
        let width = area.width - label_w;
        let mut hits = self.hits.borrow_mut();
        for (row, (i, v)) in steps.iter().enumerate() {
            let w = (width * (*v / max) as f32).max(4.0);
            let x = area.x + label_w + (width - w) / 2.0;
            let rect = RectF::new(x, area.y + h * row as f32 + 2.0, w, h - 4.0);
            canvas.fill_rect(rect, 4.0, &Brush::Solid(self.color(ctx, *i)));
            hits.push(Hit { shape: Shape::Rect(rect), series: 0, index: *i, at: PointF::new(rect.right(), rect.center().y) });
            let value = short(*v);
            if canvas.measure_text(&value, &text).width + 8.0 <= w {
                canvas.draw_text(&value, rect, &text);
            } else {
                // Barra estreita: o número ao lado.
                let mut beside = text.clone();
                beside.color = ctx.theme.color("text");
                beside.h_align = TextAlign::Leading;
                canvas.draw_text(&value, RectF::new(rect.right() + 6.0, rect.y, 80.0, rect.height), &beside);
            }
            let label = self.data_labels.get(*i).cloned().unwrap_or_default();
            let pct = if first > 0.0 { format!("{label} · {:.0}%", v / first * 100.0) } else { label };
            canvas.draw_text(&pct, RectF::new(area.x, rect.y, label_w - 8.0, rect.height), &side);
        }
    }

    /// Treemap: a 1ª série como retângulos de área proporcional (divisão "squarified").
    fn paint_treemap(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(s) = data.first() else { return };
        let mut items: Vec<(usize, f64)> = s.values.iter().copied().enumerate().filter(|(i, v)| *v > 0.0 && !self.label_hidden(*i)).collect();
        items.sort_by(|a, b| b.1.total_cmp(&a.1));
        let rects = squarify(&items.iter().map(|(_, v)| *v).collect::<Vec<_>>(), area);
        let mut text = ctx.text_style();
        text.color = Color::WHITE;
        text.font.weight = FontWeight::Bold;
        text.h_align = TextAlign::Leading;
        let mut sub = text.clone();
        sub.font.weight = FontWeight::Normal;
        sub.font.size = (sub.font.size - 2.0).max(9.0);
        let mut hits = self.hits.borrow_mut();
        for ((i, v), r) in items.iter().zip(rects) {
            let r = r.deflate(aylis_core::Bounds::all(1.0));
            canvas.fill_rect(r, 3.0, &Brush::Solid(self.color(ctx, *i)));
            hits.push(Hit { shape: Shape::Rect(r), series: 0, index: *i, at: r.center() });
            if r.width > 40.0 && r.height > 34.0 {
                let label = self.data_labels.get(*i).cloned().unwrap_or_default();
                canvas.draw_text(&label, RectF::new(r.x + 8.0, r.y + 6.0, r.width - 12.0, 18.0), &text);
                canvas.draw_text(&short(*v), RectF::new(r.x + 8.0, r.y + 24.0, r.width - 12.0, 16.0), &sub);
            }
        }
    }

    /// Radar: um eixo por rótulo; cada série visível é um polígono (valores de 0 ao máximo).
    fn paint_radar(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let n = self.points();
        if n < 3 {
            return;
        }
        let max = data.iter().filter(|s| !s.hidden).flat_map(|s| s.values.iter()).copied().fold(0.0, f64::max).max(1e-9);
        let c = area.center();
        let radius = area.width.min(area.height) / 2.0 - 24.0;
        let at = |i: usize, f: f32| {
            let a = std::f32::consts::TAU * i as f32 / n as f32 - std::f32::consts::FRAC_PI_2;
            PointF::new(c.x + radius * f * a.cos(), c.y + radius * f * a.sin())
        };
        let grid = Stroke::new(ctx.theme.color("border"), 1.0);
        for ring in 1..=4 {
            let pts: Vec<PointF> = (0..n).map(|i| at(i, ring as f32 / 4.0)).collect();
            let mut p = Path::new();
            p.polygon(&pts, true);
            canvas.stroke_path(&p, &grid);
        }
        let mut l = small_text(ctx);
        l.h_align = TextAlign::Center;
        for i in 0..n {
            canvas.draw_line(c, at(i, 1.0), &grid);
            let p = at(i, 1.12);
            canvas.draw_text(self.data_labels.get(i).map_or("", String::as_str), RectF::new(p.x - 40.0, p.y - 9.0, 80.0, 18.0), &l);
        }
        let mut hits = self.hits.borrow_mut();
        for (si, s) in data.iter().enumerate().filter(|(_, s)| !s.hidden) {
            let color = self.color(ctx, si);
            let pts: Vec<PointF> = (0..n).map(|i| at(i, (s.values.get(i).copied().unwrap_or(0.0) / max) as f32)).collect();
            let mut p = Path::new();
            p.polygon(&pts, true);
            canvas.fill_path(&p, &Brush::Solid(color.with_opacity(0.22)));
            canvas.stroke_path(&p, &Stroke::new(color, s.width.unwrap_or(2.0)).dashed(s.dash.clone()));
            for (i, p) in pts.iter().enumerate() {
                super::cartesian::draw_point(canvas, s.point, *p, 3.0, color, ctx.style.fill);
                hits.push(Hit { shape: Shape::Circle(*p, 5.0), series: si, index: i, at: *p });
            }
        }
    }
}

/// Divide `area` em retângulos de área proporcional aos valores (em ordem decrescente),
/// com proporções perto de quadrados (Bruls, Huizing e van Wijk).
pub(crate) fn squarify(values: &[f64], area: RectF) -> Vec<RectF> {
    let total: f64 = values.iter().sum();
    if total <= 0.0 || area.width <= 0.0 || area.height <= 0.0 {
        return vec![RectF::default(); values.len()];
    }
    let scale = (area.width * area.height) as f64 / total;
    let sizes: Vec<f64> = values.iter().map(|v| v * scale).collect();
    let mut out = Vec::with_capacity(values.len());
    let mut rest = area;
    let mut i = 0;
    while i < sizes.len() {
        let side = rest.width.min(rest.height) as f64;
        // Cresce a linha enquanto a pior proporção melhora.
        let worst = |row: &[f64]| {
            let sum: f64 = row.iter().sum();
            let (mx, mn) = row.iter().fold((f64::MIN, f64::MAX), |(a, b), v| (a.max(*v), b.min(*v)));
            ((side * side * mx) / (sum * sum)).max((sum * sum) / (side * side * mn))
        };
        let mut end = i + 1;
        while end < sizes.len() && worst(&sizes[i..=end]) <= worst(&sizes[i..end]) {
            end += 1;
        }
        let row = &sizes[i..end];
        let sum: f64 = row.iter().sum();
        if rest.width >= rest.height {
            let w = (sum / rest.height as f64) as f32;
            let mut y = rest.y;
            for v in row {
                let h = (*v / w as f64) as f32;
                out.push(RectF::new(rest.x, y, w, h));
                y += h;
            }
            rest = RectF::new(rest.x + w, rest.y, (rest.width - w).max(0.0), rest.height);
        } else {
            let h = (sum / rest.width as f64) as f32;
            let mut x = rest.x;
            for v in row {
                let w = (*v / h as f64) as f32;
                out.push(RectF::new(x, rest.y, w, h));
                x += w;
            }
            rest = RectF::new(rest.x, rest.y + h, rest.width, (rest.height - h).max(0.0));
        }
        i = end;
    }
    out
}

/// Os 100 quadrados do waffle: o índice do item de cada um (`None` = vazio). Cada item fica
/// com a parte inteira da porcentagem e as sobras vão para os de maior resto, para somar 100.
pub(crate) fn waffle_cells(values: &[f64]) -> Vec<Option<usize>> {
    let total: f64 = values.iter().filter(|v| **v > 0.0).sum();
    if total <= 0.0 {
        return vec![None; 100];
    }
    let exact: Vec<f64> = values.iter().map(|v| v.max(0.0) / total * 100.0).collect();
    let mut counts: Vec<usize> = exact.iter().map(|e| e.floor() as usize).collect();
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|a, b| (exact[*b] - exact[*b].floor()).total_cmp(&(exact[*a] - exact[*a].floor())));
    let missing = 100usize.saturating_sub(counts.iter().sum());
    for i in order.into_iter().take(missing) {
        counts[i] += 1;
    }
    let mut cells: Vec<Option<usize>> = counts.iter().enumerate().flat_map(|(i, n)| std::iter::repeat_n(Some(i), *n)).collect();
    cells.resize(100, None);
    cells
}

/// Um nó do Sankey: o nome, a coluna e o tamanho (o maior entre o que entra e o que sai).
pub(crate) struct SankeyNode {
    pub name: String,
    pub column: usize,
    pub size: f64,
}

pub(crate) struct SankeyLayout {
    pub nodes: Vec<SankeyNode>,
    /// A origem e o destino de cada fluxo (índices em `nodes`), na ordem dos fluxos.
    pub links: Vec<(usize, usize)>,
    pub columns: usize,
}

/// Os nós (na ordem em que aparecem) e a coluna de cada um: o caminho mais longo desde um nó
/// sem entrada. Um ciclo não trava: a coluna para de crescer depois de tantos passos quanto nós.
pub(crate) fn sankey_layout(flows: &[(&str, &str, f64)]) -> SankeyLayout {
    let mut names: Vec<String> = Vec::new();
    let mut id = |name: &str| match names.iter().position(|n| n == name) {
        Some(i) => i,
        None => {
            names.push(name.to_string());
            names.len() - 1
        }
    };
    let links: Vec<(usize, usize)> = flows.iter().map(|(a, b, _)| (id(a), id(b))).collect();
    let n = names.len();
    let mut column = vec![0usize; n];
    for _ in 0..n {
        let mut changed = false;
        for (a, b) in &links {
            if a != b && column[*b] < column[*a] + 1 && column[*a] + 1 < n {
                column[*b] = column[*a] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let (mut inflow, mut outflow) = (vec![0.0; n], vec![0.0; n]);
    for ((a, b), (_, _, v)) in links.iter().zip(flows) {
        outflow[*a] += v;
        inflow[*b] += v;
    }
    let columns = column.iter().max().map_or(0, |c| c + 1);
    let nodes = names.into_iter().enumerate().map(|(i, name)| SankeyNode { name, column: column[i], size: f64::max(inflow[i], outflow[i]) }).collect();
    SankeyLayout { nodes, links, columns }
}
