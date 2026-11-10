//! Componentes de BI (painéis de indicadores): [`KpiCard`], [`PivotTable`], [`Slicer`] e
//! [`ChoroplethMap`]. Os gráficos de BI (dispersão, empilhado, cascata, funil, treemap,
//! radar, mapa de calor, rosca) são tipos do `Chart`; o medidor é o `Gauge`.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

use aylis_core::platform::LocaleInfo;
use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, PaintCtx, Path, PointF, PointerEvent, PropEnum, RectF, Stroke, TextAlign,
    TextStyle, WheelEvent, properties,
};

use crate::leaf_control;

/// Número com separador de milhar e decimais do idioma (`1.234.567,89`).
pub fn format_number(v: f64, decimals: usize, locale: &LocaleInfo) -> String {
    if !v.is_finite() {
        return "—".into();
    }
    let text = format!("{:.*}", decimals, v.abs());
    let (int, frac) = text.split_once('.').unwrap_or((&text, ""));
    let mut grouped = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            grouped.push(locale.thousand_separator);
        }
        grouped.push(c);
    }
    let sign = if v < 0.0 && text.chars().any(|c| c != '0' && c != '.') { "-" } else { "" };
    if frac.is_empty() { format!("{sign}{grouped}") } else { format!("{sign}{grouped}{}{frac}", locale.decimal_separator) }
}

fn small(ctx: &PaintCtx<'_>) -> TextStyle {
    let mut t = ctx.text_style();
    t.font.size = (t.font.size - 2.0).max(9.0);
    t.color = ctx.theme.color("text_muted");
    t.h_align = TextAlign::Leading;
    t
}

fn card(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, r: RectF) {
    canvas.fill_rect(r, 10.0, &Brush::Solid(ctx.theme.color("surface")));
    canvas.stroke_rect(r, 10.0, &Stroke::new(ctx.theme.color("border"), 1.0));
}

// ===========================================================================
// KpiCard

/// Cartão de indicador: título, o valor grande, a variação contra o período anterior (▲ verde
/// quando melhora, ▼ vermelho quando piora; `higher_is_better` decide) e a tendência.
pub struct KpiCard {
    pub base: ControlBase,
    pub title: String,
    pub value: f64,
    /// O valor do período anterior (para a variação); 0: sem variação.
    pub previous: f64,
    pub prefix: String,
    pub suffix: String,
    pub decimals: u32,
    pub higher_is_better: bool,
    /// A tendência, `"10,12,9,15"` (vírgula ou `;`): uma linha embaixo.
    pub trend: String,
    pub subtitle: String,
}

impl Default for KpiCard {
    fn default() -> Self {
        Self {
            base: ControlBase::new(240.0, 130.0),
            title: "Receita".into(),
            value: 128_450.0,
            previous: 112_300.0,
            prefix: "R$ ".into(),
            suffix: String::new(),
            decimals: 0,
            higher_is_better: true,
            trend: "90;95;102;98;110;118;128".into(),
            subtitle: "vs. mês anterior".into(),
        }
    }
}

impl KpiCard {
    fn trend_values(&self) -> Vec<f64> {
        self.trend.split([';', ',']).filter_map(|v| v.trim().parse().ok()).collect()
    }
}

impl Component for KpiCard {
    leaf_control!("KpiCard");

    properties! {
        title: String => "Appearance",
        value: f64 => "Data",
        previous: f64 => "Data",
        prefix: String => "Appearance",
        suffix: String => "Appearance",
        decimals: u32 => "Appearance",
        higher_is_better: bool => "Behavior",
        trend: String => "Data",
        subtitle: String => "Appearance"
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        card(canvas, ctx, r);
        let locale = ctx.ui.locale();
        let pad = 16.0;
        let mut t = small(ctx);
        canvas.draw_text(&self.title, RectF::new(r.x + pad, r.y + 12.0, r.width - pad * 2.0, 18.0), &t);
        let mut big = ctx.text_style();
        big.font.size = (r.height * 0.24).clamp(18.0, 36.0);
        big.font.weight = FontWeight::Bold;
        big.h_align = TextAlign::Leading;
        let value = format!("{}{}{}", self.prefix, format_number(self.value, self.decimals as usize, &locale), self.suffix);
        canvas.draw_text(&value, RectF::new(r.x + pad, r.y + 32.0, r.width - pad * 2.0, big.font.size * 1.3), &big);
        let mut y = r.y + 32.0 + big.font.size * 1.35;
        if let Some(prev) = Some(self.previous).filter(|p| p.abs() > f64::EPSILON) {
            let change = (self.value - prev) / prev.abs() * 100.0;
            let good = (change >= 0.0) == self.higher_is_better;
            let color = if change.abs() < 0.05 { ctx.theme.color("text_muted") } else if good { ctx.theme.color("success") } else { ctx.theme.color("danger") };
            let arrow = if change >= 0.0 { "▲" } else { "▼" };
            let mut c = small(ctx);
            c.color = color;
            c.font.weight = FontWeight::Bold;
            let label = format!("{arrow} {}%", format_number(change.abs(), 1, &locale));
            let w = canvas.measure_text(&label, &c).width;
            canvas.draw_text(&label, RectF::new(r.x + pad, y, w + 4.0, 18.0), &c);
            t.h_align = TextAlign::Leading;
            canvas.draw_text(&self.subtitle, RectF::new(r.x + pad + w + 8.0, y, r.width - pad * 2.0 - w - 8.0, 18.0), &t);
            y += 22.0;
        }
        // Tendência: uma linha suave no rodapé.
        let values = self.trend_values();
        let area = RectF::new(r.x + pad, y, r.width - pad * 2.0, r.bottom() - y - 10.0);
        if values.len() > 1 && area.height > 8.0 {
            let (min, max) = values.iter().fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
            let span = (max - min).max(1e-9);
            let n = values.len();
            let mut line = Path::new();
            for (i, v) in values.iter().enumerate() {
                let p = PointF::new(area.x + area.width * i as f32 / (n - 1) as f32, area.bottom() - area.height * ((v - min) / span) as f32);
                if i == 0 { line.move_to(p.x, p.y) } else { line.line_to(p.x, p.y) };
            }
            canvas.stroke_path(&line, &Stroke::new(ctx.theme.color("primary"), 2.0).round());
        }
    }
}

// ===========================================================================
// PivotTable

/// Como juntar os valores de uma célula da tabela dinâmica.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Aggregate {
    #[default]
    Sum,
    Count,
    Average,
    Min,
    Max,
}

impl PropEnum for Aggregate {
    const NAMES: &'static [&'static str] = &["sum", "count", "average", "min", "max"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Sum, Self::Count, Self::Average, Self::Min, Self::Max].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Acc {
    sum: f64,
    count: usize,
    min: f64,
    max: f64,
}

impl Acc {
    fn add(&mut self, v: f64) {
        if self.count == 0 {
            (self.min, self.max) = (v, v);
        }
        self.sum += v;
        self.count += 1;
        self.min = self.min.min(v);
        self.max = self.max.max(v);
    }

    fn get(&self, how: Aggregate) -> f64 {
        match how {
            Aggregate::Sum => self.sum,
            Aggregate::Count => self.count as f64,
            Aggregate::Average => self.sum / self.count.max(1) as f64,
            Aggregate::Min => self.min,
            Aggregate::Max => self.max,
        }
    }
}

/// O resultado da tabela dinâmica: rótulos das linhas e colunas e as células.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PivotResult {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    /// `cells[linha][coluna]`; `None` onde não há dados.
    pub cells: Vec<Vec<Option<f64>>>,
    pub row_totals: Vec<f64>,
    pub column_totals: Vec<f64>,
    pub total: f64,
}

/// Tabela dinâmica (pivot): agrupa os registros de `data` (texto com cabeçalho, campos
/// separados por `;` e registros por linha) por `rows_field` nas linhas e `columns_field`
/// nas colunas, juntando `values_field` com `aggregate`, com totais. Clique numa célula:
/// `on_cell_click` com `"linha\tcoluna"` (o detalhamento, drill-down).
pub struct PivotTable {
    pub base: ControlBase,
    pub data: String,
    pub rows_field: String,
    pub columns_field: String,
    pub values_field: String,
    pub aggregate: Aggregate,
    pub decimals: u32,
    pub totals: bool,
    result: PivotResult,
    hovered: Cell<Option<(usize, usize)>>,
    offset: f32,
    layout: Cell<(f32, f32, f32)>,
}

impl Default for PivotTable {
    fn default() -> Self {
        let mut p = Self {
            base: ControlBase::new(520.0, 260.0),
            data: "regiao;produto;valor\nSul;Café;120\nSul;Chá;80\nSudeste;Café;300\nSudeste;Chá;95\nNordeste;Café;150\nNordeste;Suco;60".into(),
            rows_field: "regiao".into(),
            columns_field: "produto".into(),
            values_field: "valor".into(),
            aggregate: Aggregate::Sum,
            decimals: 0,
            totals: true,
            result: PivotResult::default(),
            hovered: Cell::new(None),
            offset: 0.0,
            layout: Cell::new((0.0, 0.0, 0.0)),
        };
        p.sync("data");
        p
    }
}

impl PivotTable {
    /// Os registros pelo código: o cabeçalho e as linhas.
    pub fn set_rows(&mut self, header: &[&str], rows: &[Vec<String>]) {
        let mut text = header.join(";");
        for r in rows {
            text.push('\n');
            text.push_str(&r.join(";"));
        }
        self.data = text;
        self.sync("data");
    }

    pub fn result(&self) -> &PivotResult {
        &self.result
    }

    /// Calcula a tabela.
    pub fn compute(data: &str, rows_field: &str, columns_field: &str, values_field: &str, how: Aggregate) -> PivotResult {
        let mut lines = data.lines().filter(|l| !l.trim().is_empty());
        let Some(header) = lines.next() else { return PivotResult::default() };
        let header: Vec<String> = header.split(';').map(|h| h.trim().to_lowercase()).collect();
        let col = |name: &str| header.iter().position(|h| *h == name.trim().to_lowercase());
        let (ri, ci, vi) = (col(rows_field), col(columns_field), col(values_field));
        let mut cells: BTreeMap<(String, String), Acc> = BTreeMap::new();
        let mut rows: BTreeMap<String, Acc> = BTreeMap::new();
        let mut cols: BTreeMap<String, Acc> = BTreeMap::new();
        let mut all = Acc::default();
        for line in lines {
            let f: Vec<&str> = line.split(';').map(str::trim).collect();
            let r = ri.and_then(|i| f.get(i)).copied().unwrap_or("").to_string();
            let c = ci.and_then(|i| f.get(i)).copied().unwrap_or("").to_string();
            let v = vi.and_then(|i| f.get(i)).and_then(|v| v.replace(',', ".").parse::<f64>().ok()).unwrap_or(if vi.is_none() { 1.0 } else { 0.0 });
            cells.entry((r.clone(), c.clone())).or_default().add(v);
            rows.entry(r).or_default().add(v);
            cols.entry(c).or_default().add(v);
            all.add(v);
        }
        let row_keys: Vec<String> = rows.keys().cloned().collect();
        let col_keys: Vec<String> = cols.keys().cloned().collect();
        PivotResult {
            cells: row_keys.iter().map(|r| col_keys.iter().map(|c| cells.get(&(r.clone(), c.clone())).map(|a| a.get(how))).collect()).collect(),
            row_totals: row_keys.iter().map(|r| rows[r].get(how)).collect(),
            column_totals: col_keys.iter().map(|c| cols[c].get(how)).collect(),
            total: all.get(how),
            rows: row_keys,
            columns: col_keys,
        }
    }

    fn sync(&mut self, name: &str) {
        if matches!(name, "data" | "rows_field" | "columns_field" | "values_field" | "aggregate") {
            self.result = Self::compute(&self.data, &self.rows_field, &self.columns_field, &self.values_field, self.aggregate);
        }
    }

    fn cell_at(&self, p: PointF) -> Option<(usize, usize)> {
        let (first_w, col_w, row_h) = self.layout.get();
        let r = self.base.bounds();
        let (x, y) = (p.x, p.y + self.offset);
        if x < first_w || y < row_h || col_w <= 0.0 {
            return None;
        }
        let (row, col) = (((y - row_h) / row_h) as usize, ((x - first_w) / col_w) as usize);
        let _ = r;
        (row < self.result.rows.len() && col < self.result.columns.len()).then_some((row, col))
    }
}

impl Component for PivotTable {
    leaf_control!("PivotTable");

    properties! {
        data: String => "Data",
        rows_field: String => "Data",
        columns_field: String => "Data",
        values_field: String => "Data",
        aggregate: Aggregate => "Data",
        decimals: u32 => "Appearance",
        totals: bool => "Appearance";
        events: on_cell_click;
        on_set: sync
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let cell = self.cell_at(event.position);
        if cell != self.hovered.get() {
            self.hovered.set(cell);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered.set(None);
        ctx.repaint();
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some((r, c)) = self.hovered.get() {
            let text = format!("{}\t{}", self.result.rows[r], self.result.columns[c]);
            ctx.fire("on_cell_click", EventArgs::Text(text));
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let (_, _, row_h) = self.layout.get();
        let content = row_h * (self.result.rows.len() + 1 + usize::from(self.totals)) as f32;
        let max = (content - self.base.height()).max(0.0);
        let next = (self.offset - event.delta.y).clamp(0.0, max);
        if (next - self.offset).abs() > 0.1 {
            self.offset = next;
            ctx.set_handled();
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        card(canvas, ctx, r);
        let res = &self.result;
        let locale = ctx.ui.locale();
        let mut text = ctx.text_style();
        text.font.size = (text.font.size - 1.0).max(10.0);
        let row_h = text.font.size * 2.1;
        let mut bold = text.clone();
        bold.font.weight = FontWeight::Bold;
        let first_w = res.rows.iter().map(|s| canvas.measure_text(s, &bold).width).fold(80.0, f32::max) + 24.0;
        let cols = res.columns.len() + usize::from(self.totals);
        let col_w = ((r.width - first_w) / cols.max(1) as f32).max(60.0);
        self.layout.set((first_w, col_w, row_h));
        canvas.save();
        canvas.clip_round_rect(r, 10.0);
        let fmt = |v: f64| format_number(v, self.decimals as usize, &locale);
        let line = Stroke::new(ctx.theme.color("border"), 1.0);
        let mut num = text.clone();
        num.h_align = TextAlign::Trailing;
        let mut num_bold = num.clone();
        num_bold.font.weight = FontWeight::Bold;
        // Linhas (rolam embaixo do cabeçalho).
        let rows = res.rows.len() + usize::from(self.totals);
        for i in 0..rows {
            let y = r.y + row_h * (i + 1) as f32 - self.offset;
            if y + row_h < r.y + row_h || y > r.bottom() {
                continue;
            }
            let total_row = i == res.rows.len();
            if total_row {
                canvas.fill_rect(RectF::new(r.x, y, r.width, row_h), 0.0, &Brush::Solid(ctx.theme.color("primary").with_opacity(0.08)));
            }
            let label = if total_row { "Total".to_string() } else { res.rows[i].clone() };
            let mut l = bold.clone();
            l.h_align = TextAlign::Leading;
            canvas.draw_text(&label, RectF::new(r.x + 12.0, y, first_w - 16.0, row_h), &l);
            for j in 0..cols {
                let x = r.x + first_w + col_w * j as f32;
                let total_col = j == res.columns.len();
                let value = match (total_row, total_col) {
                    (true, true) => Some(res.total),
                    (true, false) => res.column_totals.get(j).copied(),
                    (false, true) => res.row_totals.get(i).copied(),
                    (false, false) => res.cells[i][j],
                };
                if self.hovered.get() == Some((i, j)) && !total_row && !total_col {
                    canvas.fill_rect(RectF::new(x, y, col_w, row_h), 0.0, &Brush::Solid(ctx.theme.color("hover_overlay")));
                }
                if let Some(v) = value {
                    let style = if total_row || total_col { &num_bold } else { &num };
                    canvas.draw_text(&fmt(v), RectF::new(x, y, col_w - 12.0, row_h), style);
                }
            }
            canvas.draw_line(PointF::new(r.x, y + row_h - 0.5), PointF::new(r.right(), y + row_h - 0.5), &line);
        }
        // Cabeçalho fixo.
        canvas.fill_rect(RectF::new(r.x, r.y, r.width, row_h), 0.0, &Brush::Solid(ctx.theme.color("surface_2")));
        let mut head = bold.clone();
        head.h_align = TextAlign::Trailing;
        let corner = format!("{} \\ {}", self.rows_field, self.columns_field);
        let mut c = small(ctx);
        c.h_align = TextAlign::Leading;
        canvas.draw_text(&corner, RectF::new(r.x + 12.0, r.y, first_w - 16.0, row_h), &c);
        for j in 0..cols {
            let label = res.columns.get(j).cloned().unwrap_or_else(|| "Total".into());
            canvas.draw_text(&label, RectF::new(r.x + first_w + col_w * j as f32, r.y, col_w - 12.0, row_h), &head);
        }
        canvas.draw_line(PointF::new(r.x, r.y + row_h - 0.5), PointF::new(r.right(), r.y + row_h - 0.5), &line);
        canvas.restore();
    }
}

// ===========================================================================
// Slicer

/// Segmentador de filtro (o "slicer" dos painéis): os valores como botões; marcar filtra.
/// `selected` = `"Sul;Sudeste"` (vazio: todos). `on_change` traz a seleção.
pub struct Slicer {
    pub base: ControlBase,
    pub title: String,
    pub items: String,
    pub selected: String,
    pub multi: bool,
    rects: RefCell<Vec<RectF>>,
    clear_rect: Cell<Option<RectF>>,
    pressed: Option<PointF>,
}

impl Default for Slicer {
    fn default() -> Self {
        let mut base = ControlBase::new(260.0, 110.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            title: "Região".into(),
            items: "Norte;Nordeste;Centro-Oeste;Sudeste;Sul".into(),
            selected: String::new(),
            multi: true,
            rects: RefCell::default(),
            clear_rect: Cell::new(None),
            pressed: None,
        }
    }
}

impl Slicer {
    fn items(&self) -> Vec<&str> {
        self.items.split(';').map(str::trim).filter(|s| !s.is_empty()).collect()
    }

    fn chosen(&self) -> Vec<&str> {
        self.selected.split(';').map(str::trim).filter(|s| !s.is_empty()).collect()
    }

    /// O item passa no filtro (sem seleção, todos passam).
    pub fn accepts(&self, value: &str) -> bool {
        let chosen = self.chosen();
        chosen.is_empty() || chosen.iter().any(|c| c.eq_ignore_ascii_case(value))
    }
}

impl Component for Slicer {
    // O `on_change` do valor mudado pelo código leva o mesmo que o do usuário.
    fn change_args(&self, _name: &str, _value: &aylis_core::PropValue) -> Option<EventArgs> {
        Some(EventArgs::Text(self.selected.clone()))
    }

    leaf_control!("Slicer");

    properties! {
        title: String => "Appearance",
        items: String => "Data",
        selected: String => "Data",
        multi: bool => "Behavior";
        events: on_change
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pressed = Some(event.position);
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        let Some(p) = self.pressed.take() else { return };
        if self.clear_rect.get().is_some_and(|r| r.contains(p)) {
            self.selected.clear();
        } else {
            let hit = self.rects.borrow().iter().position(|r| r.contains(p));
            let Some(i) = hit else { return };
            let item = self.items()[i].to_string();
            let mut chosen: Vec<String> = self.chosen().into_iter().map(str::to_string).collect();
            if let Some(k) = chosen.iter().position(|c| *c == item) {
                chosen.remove(k);
            } else if self.multi {
                chosen.push(item);
            } else {
                chosen = vec![item];
            }
            self.selected = chosen.join(";");
        }
        ctx.fire("on_change", EventArgs::Text(self.selected.clone()));
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        card(canvas, ctx, r);
        let mut t = small(ctx);
        t.font.weight = FontWeight::Bold;
        canvas.draw_text(&self.title, RectF::new(r.x + 12.0, r.y + 8.0, r.width - 90.0, 18.0), &t);
        let chosen = self.chosen();
        self.clear_rect.set(None);
        if !chosen.is_empty() {
            let mut c = small(ctx);
            c.color = ctx.theme.color("primary");
            c.h_align = TextAlign::Trailing;
            let rect = RectF::new(r.right() - 80.0, r.y + 8.0, 68.0, 18.0);
            canvas.draw_text("Limpar", rect, &c);
            self.clear_rect.set(Some(rect));
        }
        let mut label = ctx.text_style();
        label.font.size = (label.font.size - 1.0).max(10.0);
        label.h_align = TextAlign::Center;
        let (mut x, mut y) = (r.x + 12.0, r.y + 34.0);
        let h = label.font.size * 2.0;
        let mut rects = Vec::new();
        for item in self.items() {
            let w = canvas.measure_text(item, &label).width + 24.0;
            if x + w > r.right() - 12.0 && x > r.x + 12.0 {
                x = r.x + 12.0;
                y += h + 6.0;
            }
            let rect = RectF::new(x, y, w, h);
            let on = chosen.iter().any(|c| c.eq_ignore_ascii_case(item));
            let (fill, fg) = if on { (ctx.theme.color("primary"), ctx.theme.color("on_primary")) } else { (ctx.theme.color("surface_2"), ctx.theme.color("text")) };
            canvas.fill_rect(rect, h / 2.0, &Brush::Solid(fill));
            let mut l = label.clone();
            l.color = fg;
            canvas.draw_text(item, rect, &l);
            rects.push(rect);
            x += w + 6.0;
        }
        *self.rects.borrow_mut() = rects;
    }
}

// ===========================================================================
// ChoroplethMap

/// Uma região do mapa: nome e os anéis (lon, lat).
#[derive(Clone, Debug, Default)]
struct Region {
    name: String,
    rings: Vec<Vec<(f64, f64)>>,
}

/// Lê as regiões de um GeoJSON (`FeatureCollection` de `Polygon`/`MultiPolygon`).
fn parse_geojson(text: &str, name_property: &str) -> Vec<Region> {
    let Ok(doc) = serde_json::from_str::<serde_json::Value>(text) else { return Vec::new() };
    let ring = |v: &serde_json::Value| -> Vec<(f64, f64)> {
        v.as_array().into_iter().flatten().filter_map(|p| Some((p.get(0)?.as_f64()?, p.get(1)?.as_f64()?))).collect()
    };
    let mut out = Vec::new();
    for f in doc["features"].as_array().into_iter().flatten() {
        let props = &f["properties"];
        let name = props.get(name_property).or_else(|| props.get("name")).and_then(|v| v.as_str().map(str::to_string).or_else(|| Some(v.to_string()))).unwrap_or_default();
        let geom = &f["geometry"];
        let mut rings = Vec::new();
        match geom["type"].as_str() {
            Some("Polygon") => rings.extend(geom["coordinates"].as_array().into_iter().flatten().map(ring)),
            Some("MultiPolygon") => {
                for poly in geom["coordinates"].as_array().into_iter().flatten() {
                    rings.extend(poly.as_array().into_iter().flatten().map(ring));
                }
            }
            _ => {}
        }
        if !rings.is_empty() {
            out.push(Region { name, rings });
        }
    }
    out
}

/// Mapa coroplético: as regiões de um GeoJSON (`geojson`: o texto ou um arquivo/asset)
/// pintadas pela escala de cor do valor (`values` = `"SP=10;RJ=5"`, pelo nome em
/// `name_property`). Passar o mouse mostra a região e o valor; clique: `on_region_click`.
pub struct ChoroplethMap {
    pub base: ControlBase,
    pub geojson: String,
    pub name_property: String,
    pub values: String,
    pub low_color: Option<Color>,
    pub high_color: Option<Color>,
    pub decimals: u32,
    regions: RefCell<Option<(String, Vec<Region>)>>,
    /// Os caminhos desenhados (para o clique acertar a região).
    paths: RefCell<Vec<(usize, Path)>>,
    hovered: Cell<Option<usize>>,
    mouse: Cell<PointF>,
}

impl Default for ChoroplethMap {
    fn default() -> Self {
        let mut base = ControlBase::new(420.0, 320.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            // Três quadrados de exemplo (troque pelo GeoJSON do seu mapa).
            geojson: r#"{"type":"FeatureCollection","features":[
                {"type":"Feature","properties":{"name":"A"},"geometry":{"type":"Polygon","coordinates":[[[0,0],[10,0],[10,10],[0,10],[0,0]]]}},
                {"type":"Feature","properties":{"name":"B"},"geometry":{"type":"Polygon","coordinates":[[[10,0],[20,0],[20,10],[10,10],[10,0]]]}},
                {"type":"Feature","properties":{"name":"C"},"geometry":{"type":"Polygon","coordinates":[[[0,10],[20,10],[20,18],[0,18],[0,10]]]}}]}"#
                .into(),
            name_property: "name".into(),
            values: "A=10;B=40;C=25".into(),
            low_color: None,
            high_color: None,
            decimals: 0,
            regions: RefCell::default(),
            paths: RefCell::default(),
            hovered: Cell::new(None),
            mouse: Cell::new(PointF::ZERO),
        }
    }
}

impl ChoroplethMap {
    fn value_map(&self) -> BTreeMap<String, f64> {
        self.values
            .split(';')
            .filter_map(|p| {
                let (k, v) = p.split_once('=')?;
                Some((k.trim().to_lowercase(), v.trim().replace(',', ".").parse().ok()?))
            })
            .collect()
    }

    /// As regiões (lidas uma vez por `geojson`).
    fn regions(&self, ctx: &PaintCtx<'_>) {
        let key = format!("{}|{}", self.name_property, self.geojson.len());
        if self.regions.borrow().as_ref().is_some_and(|(k, _)| *k == key) {
            return;
        }
        let text = if self.geojson.trim_start().starts_with('{') {
            self.geojson.clone()
        } else {
            ctx.resource(&self.geojson).map(|b| String::from_utf8_lossy(&b).into_owned()).or_else(|| std::fs::read_to_string(&self.geojson).ok()).unwrap_or_default()
        };
        *self.regions.borrow_mut() = Some((key, parse_geojson(&text, &self.name_property)));
    }

    /// Nome da região sob o mouse (para testes e código).
    pub fn hovered_region(&self) -> Option<String> {
        let i = self.hovered.get()?;
        self.regions.borrow().as_ref()?.1.get(i).map(|r| r.name.clone())
    }
}

impl Component for ChoroplethMap {
    leaf_control!("ChoroplethMap");

    properties! {
        geojson: String => "Data",
        name_property: String => "Data",
        values: String => "Data",
        low_color: Option<Color> => "Appearance",
        high_color: Option<Color> => "Appearance",
        decimals: u32 => "Appearance";
        events: on_region_click
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        self.mouse.set(p);
        let hit = self.paths.borrow().iter().rev().find(|(_, path)| path.contains(p)).map(|(i, _)| *i);
        if hit != self.hovered.get() {
            self.hovered.set(hit);
        }
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered.set(None);
        ctx.repaint();
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if let Some(name) = self.hovered_region() {
            ctx.fire("on_region_click", EventArgs::Text(name));
        }
    }

    fn contains_point(&self, p: PointF, _size: aylis_core::SizeF) -> bool {
        let paths = self.paths.borrow();
        paths.is_empty() || paths.iter().any(|(_, path)| path.contains(p))
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.regions(ctx);
        let regions = self.regions.borrow();
        let Some((_, regions)) = regions.as_ref() else { return };
        let r = ctx.rect;
        // Projeção equirretangular corrigida pela latitude média, ajustada ao controle.
        let pts = regions.iter().flat_map(|g| g.rings.iter().flatten());
        let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for (x, y) in pts {
            (x0, y0, x1, y1) = (x0.min(*x), y0.min(*y), x1.max(*x), y1.max(*y));
        }
        if x0 > x1 {
            return;
        }
        let k = ((y0 + y1) / 2.0).to_radians().cos().max(0.2);
        let (w, h) = (((x1 - x0) * k).max(1e-9), (y1 - y0).max(1e-9));
        let scale = ((r.width - 8.0) as f64 / w).min((r.height - 8.0) as f64 / h);
        let (ox, oy) = (r.x as f64 + (r.width as f64 - w * scale) / 2.0, r.y as f64 + (r.height as f64 - h * scale) / 2.0);
        let project = |(x, y): (f64, f64)| PointF::new((ox + (x - x0) * k * scale) as f32, (oy + (y1 - y) * scale) as f32);
        let values = self.value_map();
        let (lo, hi) = values.values().fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
        let low = self.low_color.unwrap_or_else(|| ctx.theme.color("primary").with_opacity(0.15));
        let high = self.high_color.unwrap_or_else(|| ctx.theme.color("primary"));
        let empty = ctx.theme.color("surface_2");
        let border = Stroke::new(ctx.theme.color("background"), 1.0);
        let mut paths = Vec::new();
        for (i, region) in regions.iter().enumerate() {
            let mut path = Path::new();
            for ring in &region.rings {
                let pts: Vec<PointF> = ring.iter().copied().map(project).collect();
                path.polygon(&pts, true);
            }
            let color = match values.get(&region.name.to_lowercase()) {
                Some(v) => low.lerp(high, if hi > lo { ((v - lo) / (hi - lo)) as f32 } else { 1.0 }),
                None => empty,
            };
            canvas.fill_path(&path, &Brush::Solid(color));
            canvas.stroke_path(&path, &border);
            if self.hovered.get() == Some(i) {
                canvas.stroke_path(&path, &Stroke::new(ctx.theme.color("text"), 2.0));
            }
            // Caminhos em coordenadas do controle (o clique chega assim).
            paths.push((i, path.map_points(|p| PointF::new(p.x - r.x, p.y - r.y))));
        }
        *self.paths.borrow_mut() = paths;
        // A região sob o mouse e o valor.
        if let Some(region) = self.hovered.get().and_then(|i| regions.get(i)) {
            let locale = ctx.ui.locale();
            let value = values.get(&region.name.to_lowercase()).map_or("—".to_string(), |v| format_number(*v, self.decimals as usize, &locale));
            let label = format!("{}: {value}", region.name);
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Leading;
            let tw = canvas.measure_text(&label, &t).width + 20.0;
            let m = self.mouse.get();
            let tip = RectF::new((r.x + m.x + 12.0).min(r.right() - tw), (r.y + m.y + 12.0).min(r.bottom() - 28.0), tw, 26.0);
            canvas.fill_rect(tip, 6.0, &Brush::Solid(ctx.theme.color("surface")));
            canvas.stroke_rect(tip, 6.0, &Stroke::new(ctx.theme.color("border"), 1.0));
            canvas.draw_text(&label, RectF::new(tip.x + 10.0, tip.y, tw - 12.0, 26.0), &t);
        }
    }
}
