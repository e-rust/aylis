//! Chart: gráfico como o do Chart.js. Linhas (retas, em degraus
//! ou suavizadas), barras (de pé ou deitadas), área, dispersão, bolhas, pizza, rosca, área
//! polar, radar, empilhados e o misto (cada série com o seu tipo); e os de BI: cascata,
//! funil, treemap, mapa de calor, sparkline, Sankey e waffle (em [`more`]).
//!
//! Escalas: dois eixos Y (a série vai para a direita), logarítmica, de tempo (rótulos com
//! datas), mínimo e máximo à mão, eixo invertido. Interação: clicar na legenda esconde ou
//! mostra a série, dica com o valor em todos os tipos, `on_click` com a série e o ponto,
//! animação ao trocar os dados, zoom (roda) e arrasto. Os eixos e as séries ficam em
//! [`cartesian`].

mod cartesian;

pub use cartesian::parse_time;
mod more;

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, PaintCtx, Path, PointF, PointerEvent, PropEnum, RectF,
    Stroke, TextAlign, TextStyle, WheelEvent, properties,
};

use crate::leaf_control;

/// Tipo do gráfico.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartKind {
    #[default]
    Line,
    Bar,
    Area,
    Pie,
    /// Pizza com furo, o total no meio.
    Donut,
    /// Linha pequena sem eixos nem legenda (o `v-sparkline`): a tendência num cantinho.
    Sparkline,
    /// Mapa de calor: uma linha por série, uma coluna por rótulo, a cor pelo valor.
    Heatmap,
    /// Dispersão: pontos X e Y (`x:y` nos valores; sem eles, a 1ª série é o X).
    Scatter,
    /// Barras com as séries empilhadas.
    StackedBar,
    /// Cascata: variações acumuladas até o total.
    Waterfall,
    /// Funil: etapas decrescentes com a conversão.
    Funnel,
    /// Retângulos de área proporcional ao valor.
    Treemap,
    /// Teia: um eixo por rótulo, um polígono por série.
    Radar,
    /// Barras deitadas: os rótulos no eixo vertical.
    HorizontalBar,
    /// Bolhas: X, Y e o raio (`x:y:r` nos valores).
    Bubble,
    /// Área polar: fatias iguais, o raio pelo valor.
    PolarArea,
    /// Linhas empilhadas (cada uma soma às de baixo).
    StackedLine,
    /// Áreas empilhadas.
    StackedArea,
    /// Misto: cada série com o seu tipo (barras e linha juntas, por exemplo).
    Mixed,
    /// Fluxos entre nós (Sankey): cada rótulo é um fluxo `origem>destino`, a 1ª série o valor;
    /// a largura da faixa é o valor.
    Sankey,
    /// Grade de 10x10 quadrados, cada um 1% do total, uma cor por rótulo (a 1ª série).
    Waffle,
}

impl ChartKind {
    /// Pizza, rosca ou área polar (sem eixos).
    pub fn is_round(self) -> bool {
        matches!(self, ChartKind::Pie | ChartKind::Donut | ChartKind::PolarArea)
    }

    /// A legenda mostra os rótulos (uma cor por item), não as séries.
    fn legend_by_label(self) -> bool {
        matches!(self, ChartKind::Pie | ChartKind::Donut | ChartKind::PolarArea | ChartKind::Funnel | ChartKind::Treemap | ChartKind::Waffle)
    }

    /// Com eixos X e Y (o resto do [`cartesian`]).
    fn is_cartesian(self) -> bool {
        matches!(
            self,
            ChartKind::Line
                | ChartKind::Bar
                | ChartKind::Area
                | ChartKind::Scatter
                | ChartKind::Bubble
                | ChartKind::StackedBar
                | ChartKind::StackedLine
                | ChartKind::StackedArea
                | ChartKind::HorizontalBar
                | ChartKind::Mixed
        )
    }
}

impl PropEnum for ChartKind {
    const NAMES: &'static [&'static str] = &[
        "line",
        "bar",
        "area",
        "pie",
        "donut",
        "sparkline",
        "heatmap",
        "scatter",
        "stacked_bar",
        "waterfall",
        "funnel",
        "treemap",
        "radar",
        "horizontal_bar",
        "bubble",
        "polar_area",
        "stacked_line",
        "stacked_area",
        "mixed",
        "sankey",
        "waffle",
    ];
    fn from_index(index: usize) -> Option<Self> {
        [
            Self::Line,
            Self::Bar,
            Self::Area,
            Self::Pie,
            Self::Donut,
            Self::Sparkline,
            Self::Heatmap,
            Self::Scatter,
            Self::StackedBar,
            Self::Waterfall,
            Self::Funnel,
            Self::Treemap,
            Self::Radar,
            Self::HorizontalBar,
            Self::Bubble,
            Self::PolarArea,
            Self::StackedLine,
            Self::StackedArea,
            Self::Mixed,
            Self::Sankey,
            Self::Waffle,
        ]
        .get(index)
        .copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O tipo de uma série no gráfico misto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeriesKind {
    Line,
    Bar,
    Area,
    Scatter,
}

impl SeriesKind {
    fn parse(text: &str) -> Option<SeriesKind> {
        Some(match text {
            "line" => SeriesKind::Line,
            "bar" => SeriesKind::Bar,
            "area" => SeriesKind::Area,
            "scatter" => SeriesKind::Scatter,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            SeriesKind::Line => "line",
            SeriesKind::Bar => "bar",
            SeriesKind::Area => "area",
            SeriesKind::Scatter => "scatter",
        }
    }
}

/// O desenho dos pontos de uma série.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PointStyle {
    #[default]
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    /// Sem pontos (só a linha).
    None,
}

impl PointStyle {
    fn parse(text: &str) -> Option<PointStyle> {
        Some(match text {
            "circle" => PointStyle::Circle,
            "square" | "rect" => PointStyle::Square,
            "triangle" => PointStyle::Triangle,
            "diamond" | "rectRot" => PointStyle::Diamond,
            "cross" | "crossRot" => PointStyle::Cross,
            "none" | "false" => PointStyle::None,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            PointStyle::Circle => "circle",
            PointStyle::Square => "square",
            PointStyle::Triangle => "triangle",
            PointStyle::Diamond => "diamond",
            PointStyle::Cross => "cross",
            PointStyle::None => "none",
        }
    }
}

/// Até onde a área de uma série é pintada (o `fill` do Chart.js).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeriesFill {
    /// Só a linha (nas áreas, vale `Origin`).
    #[default]
    None,
    /// Até o zero.
    Origin,
    /// Até o fundo do eixo.
    Start,
    /// Até o topo do eixo.
    End,
    /// Até outra série (pelo índice).
    Series(usize),
}

impl SeriesFill {
    /// `origin`, `start`, `end`, um índice (`0`) ou relativo (`-1`, `+1`) à série `own`.
    fn parse(text: &str, own: usize) -> Option<SeriesFill> {
        Some(match text {
            "" | "false" | "none" => SeriesFill::None,
            "origin" | "true" => SeriesFill::Origin,
            "start" => SeriesFill::Start,
            "end" => SeriesFill::End,
            t if t.starts_with('+') || t.starts_with('-') => SeriesFill::Series(own.checked_add_signed(t.parse::<isize>().ok()?)?),
            t => SeriesFill::Series(t.parse().ok()?),
        })
    }

    fn text(self) -> String {
        match self {
            SeriesFill::None => String::new(),
            SeriesFill::Origin => "origin".into(),
            SeriesFill::Start => "start".into(),
            SeriesFill::End => "end".into(),
            SeriesFill::Series(i) => i.to_string(),
        }
    }
}

/// Uma série de valores, com a aparência dela.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Series {
    pub name: String,
    pub values: Vec<f64>,
    /// Cor fixa; `None` usa a paleta.
    pub color: Option<Color>,
    /// No gráfico misto, o tipo desta série (`None`: linha).
    pub kind: Option<SeriesKind>,
    /// No eixo Y da direita.
    pub right_axis: bool,
    /// Espessura da linha (`None`: 2,5).
    pub width: Option<f32>,
    /// Tracejado da linha (traço, espaço...); vazio = contínua.
    pub dash: Vec<f32>,
    pub point: PointStyle,
    pub fill: SeriesFill,
    /// Escondida (clique na legenda).
    pub hidden: bool,
    /// Dispersão e bolhas: o X de cada ponto.
    pub x: Vec<f64>,
    /// Bolhas: o raio de cada ponto, em pixels.
    pub radius: Vec<f64>,
}

impl Series {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self { name: name.into(), values, ..Default::default() }
    }

    /// Pontos X e Y (dispersão).
    pub fn points(name: impl Into<String>, points: &[(f64, f64)]) -> Self {
        Self { name: name.into(), x: points.iter().map(|p| p.0).collect(), values: points.iter().map(|p| p.1).collect(), ..Default::default() }
    }

    /// Bolhas: X, Y e o raio.
    pub fn bubbles(name: impl Into<String>, bubbles: &[(f64, f64, f64)]) -> Self {
        Self {
            name: name.into(),
            x: bubbles.iter().map(|b| b.0).collect(),
            values: bubbles.iter().map(|b| b.1).collect(),
            radius: bubbles.iter().map(|b| b.2).collect(),
            ..Default::default()
        }
    }

    pub fn with_kind(mut self, kind: SeriesKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn on_right_axis(mut self) -> Self {
        self.right_axis = true;
        self
    }

    pub fn with_dash(mut self, dash: Vec<f32>) -> Self {
        self.dash = dash;
        self
    }

    pub fn with_point(mut self, point: PointStyle) -> Self {
        self.point = point;
        self
    }

    pub fn with_fill(mut self, fill: SeriesFill) -> Self {
        self.fill = fill;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// As opções no texto (`[bar;right;#FF0000;width 3;dash 4 2;triangle;fill origin]`).
    fn options_text(&self) -> String {
        let mut o = Vec::new();
        if let Some(k) = self.kind {
            o.push(k.name().to_string());
        }
        if self.right_axis {
            o.push("right".into());
        }
        if self.hidden {
            o.push("hidden".into());
        }
        if let Some(c) = self.color {
            o.push(c.to_hex());
        }
        if let Some(w) = self.width {
            o.push(format!("width {w}"));
        }
        if !self.dash.is_empty() {
            o.push(format!("dash {}", self.dash.iter().map(f32::to_string).collect::<Vec<_>>().join(" ")));
        }
        if self.point != PointStyle::Circle {
            o.push(self.point.name().into());
        }
        if self.fill != SeriesFill::None {
            o.push(format!("fill {}", self.fill.text()));
        }
        if o.is_empty() { String::new() } else { format!("[{}]", o.join(";")) }
    }

    /// Lê uma opção do texto; `false` se não conhecer.
    fn apply_option(&mut self, option: &str, own: usize) -> bool {
        let option = option.trim();
        let (word, rest) = option.split_once(' ').map_or((option, ""), |(w, r)| (w, r.trim()));
        if let Some(k) = SeriesKind::parse(word) {
            self.kind = Some(k);
        } else if let Some(p) = PointStyle::parse(word) {
            self.point = p;
        } else if word == "right" {
            self.right_axis = true;
        } else if word == "hidden" {
            self.hidden = true;
        } else if word.starts_with('#') {
            self.color = Color::from_hex(word);
        } else if word == "width" {
            self.width = rest.parse().ok();
        } else if word == "dash" {
            self.dash = rest.split_whitespace().filter_map(|d| d.parse().ok()).collect();
        } else if word == "fill" {
            self.fill = SeriesFill::parse(rest, own).unwrap_or_default();
        } else {
            return false;
        }
        true
    }
}

/// Paleta das séries (a primeira é a cor `primary` do tema).
const PALETTE: [&str; 7] = ["#F59E0B", "#10B981", "#EF4444", "#8B5CF6", "#06B6D4", "#EC4899", "#84CC16"];

/// Duração da animação ao trocar os dados, em segundos.
const ANIMATION: f64 = 0.45;

/// Uma área sensível do desenho: o que a dica mostra e o `on_click` informa.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hit {
    pub shape: Shape,
    pub series: usize,
    pub index: usize,
    /// Onde a dica aponta.
    pub at: PointF,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Shape {
    Rect(RectF),
    Circle(PointF, f32),
    /// Fatia: centro, raios de dentro e de fora, ângulo inicial e varredura (radianos).
    Wedge(PointF, f32, f32, f32, f32),
    /// A coluna (ou faixa) inteira de um rótulo: a dica mostra todas as séries nele.
    Column(RectF),
}

impl Hit {
    fn contains(&self, p: PointF) -> bool {
        match self.shape {
            Shape::Rect(r) | Shape::Column(r) => r.contains(p),
            Shape::Circle(c, r) => (p.x - c.x).powi(2) + (p.y - c.y).powi(2) <= (r + 3.0).powi(2),
            Shape::Wedge(c, r0, r1, a0, sweep) => {
                let (dx, dy) = (p.x - c.x, p.y - c.y);
                let d = (dx * dx + dy * dy).sqrt();
                let a = (dy.atan2(dx) - a0).rem_euclid(std::f32::consts::TAU);
                d >= r0 && d <= r1 && a <= sweep
            }
        }
    }

    fn is_column(&self) -> bool {
        matches!(self.shape, Shape::Column(_))
    }
}

/// Gráfico (como o do Chart.js): veja os tipos em [`ChartKind`].
///
/// Pelo texto (JSON, Object Inspector): `labels` = `Jan;Fev;Mar`, `series` =
/// `Vendas=10,20,30|Custos[line;right;#EF4444;dash 4 2]=5,8,9`; pontos `x:y` (dispersão) e
/// `x:y:r` (bolhas). Ou `json`, no formato do Chart.js (`{"labels": [...], "datasets":
/// [{"label": ..., "data": [...], "type": "bar", ...}]}`). Pelo código: [`Chart::set_data`].
pub struct Chart {
    pub base: ControlBase,
    /// O tipo do gráfico: barras, linhas, área, pizza, rosca, dispersão...
    pub kind: ChartKind,
    /// O título em cima do gráfico; vazio: sem título.
    pub title: String,
    /// Os rótulos do eixo X (ou das fatias), separados por `;` (`Jan;Fev;Mar`).
    pub labels: String,
    /// As séries, separadas por `|`: `Nome=10,20,30`, com opções entre colchetes
    /// (`Custos[line;right;#EF4444]=5,8,9`).
    pub series: String,
    /// Os dados no formato do Chart.js (vazio: `labels` e `series`).
    pub json: String,
    /// Mostra a legenda com o nome de cada série.
    pub legend: bool,
    /// Linhas em degraus.
    pub stepped: bool,
    /// Linhas suavizadas (0 = retas; até 1), sem passar dos pontos (monotônica).
    pub tension: f32,
    /// Animação ao trocar os dados.
    pub animate: bool,
    /// Roda do mouse aproxima, arrastar anda, duplo clique volta (eixo X).
    pub zoom: bool,
    /// Eixo Y automático; sem ele, de `y_min` a `y_max`.
    pub auto_scale: bool,
    /// O começo do eixo Y quando o `auto_scale` está desligado.
    pub y_min: f64,
    /// O fim do eixo Y quando o `auto_scale` está desligado.
    pub y_max: f64,
    /// Escala logarítmica no eixo Y.
    pub log_scale: bool,
    /// Eixo Y invertido (o menor em cima).
    pub reverse: bool,
    /// Os rótulos são datas (`2026-10-27`, `2026-10-27 14:30`): o eixo X é de tempo.
    pub time_axis: bool,
    /// O último ponto clicado (série e índice; -1 = nenhum).
    pub clicked_series: i64,
    /// O índice do último ponto clicado na série (-1 = nenhum).
    pub clicked_index: i64,
    // As escolhas de aparência pedidas no relatório do InsightDB (11/11); os padrões são o
    // desenho de antes.
    /// Números dos eixos e das dicas: vazio ou `short` (60k, 1.2M), `full` (60.000, com o
    /// separador do idioma) ou `percent` (60%).
    pub value_format: String,
    /// Ângulo dos rótulos do eixo X, em graus (45: inclinados, para nomes longos).
    pub label_angle: f32,
    /// Onde fica o título: `left`, `center` ou `right`.
    pub title_align: String,
    /// O título em negrito.
    pub title_bold: bool,
    /// Opacidade do preenchimento das áreas (0 a 1).
    pub area_opacity: f32,
    /// Onde fica a legenda: `top_right`, `top_left`, `top` ou `bottom`.
    pub legend_position: String,
    /// Barras deitadas empilhadas (com o tipo `horizontal_bar`).
    pub stacked: bool,
    /// As cores, em ordem (`#2563EB;#16A34A`): das séries e, na pizza e na rosca, das fatias (sem
    /// elas, as fatias usam a paleta do gráfico, não a cor das séries).
    pub colors: String,
    /// O texto no meio da rosca: vazio = o total; `-` = nada; ou um texto (`{total}` vira o total).
    pub center_text: String,
    /// Os rótulos das fatias: `percent` (dentro), `outside` (nome e percentual fora, com uma
    /// linha) ou `none`.
    pub slice_labels: String,
    data_labels: Vec<String>,
    data: Vec<Series>,
    /// Itens escondidos pela legenda, nos tipos em que ela mostra os rótulos.
    hidden_labels: Vec<usize>,
    hovered: Cell<Option<Hit>>,
    pressed: Cell<Option<Hit>>,
    hits: RefCell<Vec<Hit>>,
    legend_hits: RefCell<Vec<(RectF, usize)>>,
    plot: Cell<RectF>,
    /// Zoom: a parte visível do eixo X (de 0 a 1).
    window: Cell<(f64, f64)>,
    drag: Cell<Option<(f32, (f64, f64))>>,
    /// Animação: os valores de antes e quando começou (no primeiro quadro depois da troca).
    anim_from: RefCell<Vec<Vec<f64>>>,
    anim_start: Cell<Option<f64>>,
    painted: Cell<bool>,
}

impl Default for Chart {
    fn default() -> Self {
        let mut base = ControlBase::new(420.0, 280.0);
        base.set_cursor(CursorShape::Default);
        let mut c = Self {
            base,
            kind: ChartKind::Line,
            title: String::new(),
            labels: "Jan;Fev;Mar;Abr;Mai;Jun".into(),
            series: "Vendas=12,19,15,24,28,33|Custos=8,11,10,14,15,18".into(),
            json: String::new(),
            legend: true,
            stepped: false,
            tension: 0.0,
            animate: true,
            zoom: false,
            auto_scale: true,
            y_min: 0.0,
            y_max: 100.0,
            log_scale: false,
            reverse: false,
            time_axis: false,
            clicked_series: -1,
            clicked_index: -1,
            value_format: String::new(),
            label_angle: 0.0,
            title_align: "left".into(),
            title_bold: true,
            area_opacity: 0.22,
            legend_position: "top_right".into(),
            stacked: false,
            colors: String::new(),
            center_text: String::new(),
            slice_labels: "percent".into(),
            data_labels: Vec::new(),
            data: Vec::new(),
            hidden_labels: Vec::new(),
            hovered: Cell::new(None),
            pressed: Cell::new(None),
            hits: RefCell::new(Vec::new()),
            legend_hits: RefCell::new(Vec::new()),
            plot: Cell::new(RectF::default()),
            window: Cell::new((0.0, 1.0)),
            drag: Cell::new(None),
            anim_from: RefCell::new(Vec::new()),
            anim_start: Cell::new(None),
            painted: Cell::new(false),
        };
        c.sync("series");
        c
    }
}

impl Chart {
    pub fn new(kind: ChartKind) -> Self {
        Self { kind, ..Default::default() }
    }

    /// Troca rótulos e séries pelo código (com a animação, se ligada).
    pub fn set_data(&mut self, labels: Vec<String>, series: Vec<Series>) {
        self.start_animation();
        self.labels = labels.join(";");
        self.series = series_text(&series);
        self.json.clear();
        self.data_labels = labels;
        self.data = series;
        self.hidden_labels.clear();
    }

    pub fn data(&self) -> &[Series] {
        &self.data
    }

    pub fn data_labels(&self) -> &[String] {
        &self.data_labels
    }

    /// Esconde ou mostra a série `index` (como clicar na legenda).
    pub fn set_series_visible(&mut self, index: usize, visible: bool) {
        if let Some(s) = self.data.get_mut(index) {
            s.hidden = !visible;
            self.series = series_text(&self.data);
        }
    }

    /// Nos tipos com a legenda por rótulo (pizza, rosca, área polar, funil, treemap): o item
    /// `index` escondido.
    pub fn label_hidden(&self, index: usize) -> bool {
        self.hidden_labels.contains(&index)
    }

    /// Volta o zoom do eixo X ao todo.
    pub fn reset_zoom(&mut self) {
        self.window.set((0.0, 1.0));
    }

    /// A parte visível do eixo X, de 0 a 1.
    pub fn zoom_window(&self) -> (f64, f64) {
        self.window.get()
    }

    fn start_animation(&mut self) {
        if self.animate && self.painted.get() && !self.data.is_empty() {
            *self.anim_from.borrow_mut() = self.data.iter().map(|s| s.values.clone()).collect();
            self.anim_start.set(None);
        }
    }

    fn sync(&mut self, name: &str) {
        match name {
            "labels" | "series" => {
                self.start_animation();
                self.data_labels = if self.labels.is_empty() { Vec::new() } else { self.labels.split(';').map(|s| s.trim().to_string()).collect() };
                self.data = parse_series(&self.series);
                self.hidden_labels.clear();
            }
            "json" if !self.json.trim().is_empty() => {
                if let Some((labels, series)) = parse_json(&self.json) {
                    self.start_animation();
                    self.data_labels = labels;
                    self.data = series;
                    self.hidden_labels.clear();
                }
            }
            _ => {}
        }
    }

    pub(crate) fn color(&self, ctx: &PaintCtx<'_>, index: usize) -> Color {
        if let Some(c) = self.data.get(index).and_then(|s| s.color) {
            return c;
        }
        self.palette_color(ctx, index)
    }

    /// A cor de uma fatia (ou de um item da legenda pelos rótulos): `colors`, senão a paleta;
    /// nunca a cor da série de mesmo índice (a fatia 2 pegava a cor da série 2).
    pub(crate) fn slice_color(&self, ctx: &PaintCtx<'_>, index: usize) -> Color {
        self.palette_color(ctx, index)
    }

    fn palette_color(&self, ctx: &PaintCtx<'_>, index: usize) -> Color {
        let own: Vec<Color> = self.colors.split([';', ',']).filter_map(|c| Color::from_hex(c.trim())).collect();
        if !own.is_empty() {
            return own[index % own.len()];
        }
        if index == 0 {
            return ctx.theme.color("primary");
        }
        Color::from_hex(PALETTE[(index - 1) % PALETTE.len()]).unwrap_or(Color::BLACK)
    }

    pub(crate) fn points(&self) -> usize {
        self.data.iter().map(|s| s.values.len()).max().unwrap_or(0).max(self.data_labels.len())
    }

    /// As séries como aparecem neste quadro: no meio da animação, os valores vão dos de
    /// antes aos novos.
    fn shown(&self, ctx: &PaintCtx<'_>) -> Vec<Series> {
        let from = self.anim_from.borrow();
        if from.is_empty() {
            return self.data.clone();
        }
        let start = self.anim_start.get().unwrap_or_else(|| {
            self.anim_start.set(Some(ctx.time));
            ctx.time
        });
        let t = ((ctx.time - start) / ANIMATION).clamp(0.0, 1.0);
        if t >= 1.0 {
            drop(from);
            self.anim_from.borrow_mut().clear();
            return self.data.clone();
        }
        ctx.request_frame();
        // Desacelerando no fim (cúbica).
        let e = 1.0 - (1.0 - t).powi(3);
        self.data
            .iter()
            .enumerate()
            .map(|(si, s)| {
                let old = from.get(si);
                let mut s = s.clone();
                for (i, v) in s.values.iter_mut().enumerate() {
                    let a = old.and_then(|o| o.get(i)).copied().unwrap_or(0.0);
                    *v = a + (*v - a) * e;
                }
                s
            })
            .collect()
    }

    /// Esconde ou mostra o item da legenda `index` (uma série ou, nos tipos por rótulo, um
    /// rótulo).
    fn toggle_legend(&mut self, index: usize) {
        if self.kind.legend_by_label() {
            match self.hidden_labels.iter().position(|i| *i == index) {
                Some(at) => {
                    self.hidden_labels.remove(at);
                }
                None => self.hidden_labels.push(index),
            }
        } else if let Some(s) = self.data.get_mut(index) {
            s.hidden = !s.hidden;
            self.series = series_text(&self.data);
        }
    }

    /// O texto da dica de `hit`: o título e as linhas (cor e texto).
    fn tooltip(&self, ctx: &PaintCtx<'_>, data: &[Series], hit: &Hit) -> (String, Vec<(Color, String)>) {
        let name = |s: &Series| if s.name.is_empty() { "valor".to_string() } else { s.name.clone() };
        let label = self.data_labels.get(hit.index).cloned().unwrap_or_default();
        if hit.is_column() {
            let lines = data
                .iter()
                .enumerate()
                .filter(|(_, s)| !s.hidden)
                .filter_map(|(si, s)| s.values.get(hit.index).map(|v| (self.color(ctx, si), format!("{}: {}", name(s), short(*v)))))
                .collect();
            return (label, lines);
        }
        let Some(s) = data.get(hit.series) else { return (label, Vec::new()) };
        let v = s.values.get(hit.index).copied().unwrap_or(0.0);
        match self.kind {
            ChartKind::Pie | ChartKind::Donut | ChartKind::PolarArea | ChartKind::Funnel | ChartKind::Treemap | ChartKind::Waffle => {
                let total: f64 = s.values.iter().filter(|v| **v > 0.0).sum();
                let pct = if total > 0.0 { format!(" ({:.0}%)", v / total * 100.0) } else { String::new() };
                (label, vec![(self.color(ctx, hit.index), format!("{}: {}{pct}", name(s), short(v)))])
            }
            ChartKind::Scatter | ChartKind::Bubble => {
                let x = s.x.get(hit.index).copied().unwrap_or(hit.index as f64);
                let r = s.radius.get(hit.index).map(|r| format!(", r {}", short(*r))).unwrap_or_default();
                (name(s), vec![(self.color(ctx, hit.series), format!("({}, {}){r}", short(x), short(v)))])
            }
            _ => {
                let title = if self.kind == ChartKind::Heatmap { format!("{} · {label}", name(s)) } else { label };
                (title, vec![(self.color(ctx, hit.series), format!("{}: {}", name(s), short(v)))])
            }
        }
    }

    fn paint_tooltip(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, data: &[Series], hit: &Hit) {
        let (title, lines) = self.tooltip(ctx, data, hit);
        if lines.is_empty() {
            return;
        }
        let mut tip = small_text(ctx);
        tip.h_align = TextAlign::Leading;
        tip.color = ctx.theme.color("text");
        let width = lines.iter().map(|(_, l)| l).chain(std::iter::once(&title)).map(|l| canvas.measure_text(l, &tip).width).fold(0.0, f32::max) + 34.0;
        let rows = lines.len() + usize::from(!title.is_empty());
        let height = 12.0 + 18.0 * rows as f32;
        let bounds = ctx.rect;
        let mut x = hit.at.x + 12.0;
        if x + width > bounds.right() - 4.0 {
            x = hit.at.x - width - 12.0;
        }
        let y = (hit.at.y - height / 2.0).clamp(bounds.y + 4.0, (bounds.bottom() - height - 4.0).max(bounds.y + 4.0));
        let rect = RectF::new(x.max(bounds.x + 4.0), y, width, height);
        canvas.fill_rect(rect.offset(0.0, 2.0), 6.0, &Brush::Solid(Color::BLACK.with_opacity(0.12)));
        canvas.fill_rect(rect, 6.0, &Brush::Solid(ctx.theme.color("surface")));
        canvas.stroke_rect(rect, 6.0, &Stroke::new(ctx.theme.color("border"), 1.0));
        let mut row = 0;
        if !title.is_empty() {
            let mut bold = tip.clone();
            bold.font.weight = FontWeight::Bold;
            canvas.draw_text(&title, RectF::new(rect.x + 12.0, rect.y + 6.0, width - 16.0, 18.0), &bold);
            row = 1;
        }
        for (color, line) in &lines {
            let y = rect.y + 6.0 + 18.0 * row as f32;
            canvas.fill_rect(RectF::new(rect.x + 10.0, y + 5.0, 8.0, 8.0), 2.0, &Brush::Solid(*color));
            canvas.draw_text(line, RectF::new(rect.x + 22.0, y, width - 26.0, 18.0), &tip);
            row += 1;
        }
    }
}

/// O texto das séries (o de `series`), com as opções de cada uma.
fn series_text(series: &[Series]) -> String {
    series
        .iter()
        .map(|s| {
            let values: Vec<String> = s
                .values
                .iter()
                .enumerate()
                .map(|(i, v)| match (s.x.get(i), s.radius.get(i)) {
                    (Some(x), Some(r)) => format!("{x}:{v}:{r}"),
                    (Some(x), None) => format!("{x}:{v}"),
                    _ => v.to_string(),
                })
                .collect();
            format!("{}{}={}", s.name, s.options_text(), values.join(","))
        })
        .collect::<Vec<_>>()
        .join("|")
}

/// Lê o texto de `series`: `Nome[opções]=v1,v2` (ou `x:y`, `x:y:r`), separadas por `|`.
fn parse_series(text: &str) -> Vec<Series> {
    text.split('|')
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .map(|(own, s)| {
            let (head, values) = s.split_once('=').unwrap_or(("", s));
            let (name, options) = match head.split_once('[') {
                Some((n, o)) => (n, o.trim_end().trim_end_matches(']')),
                None => (head, ""),
            };
            let mut series = Series::new(name.trim(), Vec::new());
            for o in options.split(';').filter(|o| !o.trim().is_empty()) {
                series.apply_option(o, own);
            }
            for v in values.split(',') {
                let parts: Vec<f64> = v.trim().replace(' ', "").split(':').filter_map(|p| p.parse().ok()).collect();
                match parts.as_slice() {
                    [y] => series.values.push(*y),
                    [x, y] => {
                        series.x.push(*x);
                        series.values.push(*y);
                    }
                    [x, y, r, ..] => {
                        series.x.push(*x);
                        series.values.push(*y);
                        series.radius.push(*r);
                    }
                    _ => {}
                }
            }
            series
        })
        .collect()
}

/// Os dados no formato do Chart.js: `labels` e `datasets` (com `label`, `data`, `type`,
/// `color`/`borderColor`/`backgroundColor`, `yAxisID`/`axis` = `right`/`y1`, `borderWidth`,
/// `borderDash`, `pointStyle`, `fill`, `hidden`). Os pontos podem ser números ou objetos
/// `{x, y, r}`.
fn parse_json(text: &str) -> Option<(Vec<String>, Vec<Series>)> {
    let root: serde_json::Value = serde_json::from_str(text).ok()?;
    let root = root.get("data").filter(|d| d.get("datasets").is_some()).unwrap_or(&root);
    let labels = root
        .get("labels")
        .and_then(|l| l.as_array())
        .map(|a| a.iter().map(|v| v.as_str().map(str::to_string).unwrap_or_else(|| v.to_string())).collect())
        .unwrap_or_default();
    let datasets = root.get("datasets")?.as_array()?;
    let series = datasets
        .iter()
        .enumerate()
        .map(|(own, d)| {
            let mut s = Series::new(d.get("label").and_then(|l| l.as_str()).unwrap_or_default(), Vec::new());
            for p in d.get("data").and_then(|v| v.as_array()).into_iter().flatten() {
                match p {
                    serde_json::Value::Object(o) => {
                        let num = |k: &str| o.get(k).and_then(|v| v.as_f64());
                        let Some(y) = num("y") else { continue };
                        s.values.push(y);
                        if let Some(x) = num("x") {
                            s.x.push(x);
                        }
                        if let Some(r) = num("r") {
                            s.radius.push(r);
                        }
                    }
                    other => s.values.push(other.as_f64().unwrap_or(0.0)),
                }
            }
            let text = |k: &str| d.get(k).and_then(|v| v.as_str());
            s.kind = text("type").and_then(SeriesKind::parse);
            s.color = ["color", "borderColor", "backgroundColor"].iter().find_map(|k| text(k)).and_then(Color::from_hex);
            s.right_axis = matches!(text("yAxisID").or(text("axis")), Some("right" | "y1" | "y2"));
            s.width = d.get("borderWidth").or(d.get("width")).and_then(|v| v.as_f64()).map(|w| w as f32);
            s.dash = d.get("borderDash").or(d.get("dash")).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|v| v.as_f64()).map(|v| v as f32).collect()).unwrap_or_default();
            s.point = text("pointStyle").or(text("point")).and_then(PointStyle::parse).unwrap_or_default();
            s.fill = match d.get("fill") {
                Some(serde_json::Value::Bool(b)) => if *b { SeriesFill::Origin } else { SeriesFill::None },
                Some(serde_json::Value::Number(n)) => n.as_u64().map(|n| SeriesFill::Series(n as usize)).unwrap_or_default(),
                Some(serde_json::Value::String(t)) => SeriesFill::parse(t, own).unwrap_or_default(),
                _ => SeriesFill::None,
            };
            s.hidden = d.get("hidden").and_then(|v| v.as_bool()).unwrap_or(false);
            s
        })
        .collect();
    Some((labels, series))
}

/// Divisão "bonita" do eixo: (mínimo, máximo, passo).
pub(crate) fn nice_scale(min: f64, max: f64) -> (f64, f64, f64) {
    let (min, max) = if (max - min).abs() < f64::EPSILON { (min - 1.0, max + 1.0) } else { (min, max) };
    let raw = (max - min) / 5.0;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * mag).find(|s| *s >= raw).unwrap_or(raw);
    ((min / step).floor() * step, (max / step).ceil() * step, step)
}

thread_local! {
    /// O `value_format` do gráfico sendo pintado (e os separadores do idioma): a pintura é uma
    /// por vez, na thread da interface, e assim cada desenho de número (eixos, dicas, fatias)
    /// segue o formato sem levá-lo por todas as funções.
    static FORMAT: std::cell::RefCell<(String, char, char)> = std::cell::RefCell::new((String::new(), ',', '.'));
}

/// Usa o formato do gráfico nos números desenhados a partir daqui.
fn set_format(format: &str, ctx: &PaintCtx<'_>) {
    // Só no formato `full`/`percent` o idioma importa (o `locale()` monta a informação inteira).
    let wanted = matches!(format.trim(), "full" | "percent");
    let (decimal, thousands) = ctx.ui.services().locale.as_ref().filter(|_| wanted).map_or((',', '.'), |l| {
        let info = l.locale();
        (info.decimal_separator, info.thousand_separator)
    });
    FORMAT.with(|f| *f.borrow_mut() = (format.trim().to_ascii_lowercase(), decimal, thousands));
}

/// O número inteiro com os milhares separados, e até duas casas.
fn full(v: f64, decimal: char, thousands: char) -> String {
    let text = if v.fract().abs() < 1e-9 { format!("{:.0}", v.abs()) } else { format!("{:.2}", v.abs()) };
    let (int, frac) = text.split_once('.').unwrap_or((&text, ""));
    let mut grouped = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            grouped.push(thousands);
        }
        grouped.push(c);
    }
    let sign = if v < 0.0 { "-" } else { "" };
    if frac.is_empty() { format!("{sign}{grouped}") } else { format!("{sign}{grouped}{decimal}{frac}") }
}

pub(crate) fn short(v: f64) -> String {
    let (format, decimal, thousands) = FORMAT.with(|f| f.borrow().clone());
    match format.as_str() {
        "full" => return full(v, decimal, thousands),
        "percent" => return format!("{}%", full(v, decimal, thousands)),
        _ => {}
    }
    let a = v.abs();
    if a >= 1e6 {
        format!("{:.1}M", v / 1e6)
    } else if a >= 1e4 {
        format!("{:.0}k", v / 1e3)
    } else if v.fract().abs() < 1e-9 {
        format!("{v:.0}")
    } else if a < 0.1 && a > 0.0 {
        format!("{v:.3}")
    } else {
        format!("{v:.1}")
    }
}

/// Texto pequeno e apagado (eixos, rótulos).
pub(crate) fn small_text(ctx: &PaintCtx<'_>) -> TextStyle {
    let mut t = ctx.text_style();
    t.font.size = (t.font.size - 3.0).max(9.0);
    t.color = ctx.theme.color("text_muted");
    t
}

impl Component for Chart {
    leaf_control!("Chart");

    properties! {
        kind: ChartKind => "Appearance",
        title: String => "Appearance",
        labels: String => "Data",
        series: String => "Data",
        json: String => "Data",
        legend: bool => "Appearance",
        stepped: bool => "Appearance",
        tension: f32 => "Appearance",
        animate: bool => "Behavior",
        zoom: bool => "Behavior",
        auto_scale: bool => "Layout",
        y_min: f64 => "Layout",
        y_max: f64 => "Layout",
        log_scale: bool => "Layout",
        reverse: bool => "Layout",
        time_axis: bool => "Layout",
        clicked_series: i64 => "Data",
        clicked_index: i64 => "Data",
        value_format: String => "Appearance",
        label_angle: f32 => "Appearance",
        title_align: String => "Appearance",
        title_bold: bool => "Appearance",
        area_opacity: f32 => "Appearance",
        legend_position: String => "Appearance",
        stacked: bool => "Appearance",
        colors: String => "Appearance",
        center_text: String => "Appearance",
        slice_labels: String => "Appearance";
        on_set: sync
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((start_x, (w0, w1))) = self.drag.get() {
            // Arrastar: o eixo X anda junto com o mouse.
            let plot = self.plot.get();
            let span = w1 - w0;
            let shift = -((event.position.x - start_x) / plot.width.max(1.0)) as f64 * span;
            let shift = shift.clamp(-w0, 1.0 - w1);
            self.window.set((w0 + shift, w1 + shift));
            ctx.repaint();
            return;
        }
        let hit = self.hits.borrow().iter().rev().find(|h| h.contains(event.position)).copied();
        let same = |a: Option<Hit>, b: Option<Hit>| match (a, b) {
            (Some(a), Some(b)) => a.series == b.series && a.index == b.index && a.is_column() == b.is_column(),
            (None, None) => true,
            _ => false,
        };
        if !same(hit, self.hovered.get()) {
            self.hovered.set(hit);
            ctx.repaint();
        }
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.hovered.set(None);
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let legend = self.legend_hits.borrow().iter().find(|(r, _)| r.contains(event.position)).map(|(_, i)| *i);
        if let Some(i) = legend {
            self.toggle_legend(i);
            self.pressed.set(None);
            ctx.repaint();
            return;
        }
        self.pressed.set(self.hits.borrow().iter().rev().find(|h| h.contains(event.position)).copied());
        if self.zoom && self.kind.is_cartesian() && self.plot.get().contains(event.position) {
            if event.click_count >= 2 {
                self.window.set((0.0, 1.0));
                ctx.repaint();
            } else {
                self.drag.set(Some((event.position.x, self.window.get())));
            }
        }
    }

    fn pointer_up(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if let Some((start_x, _)) = self.drag.take() {
            // Arrastou: não é um clique no ponto.
            if (event.position.x - start_x).abs() > 4.0 {
                self.pressed.set(None);
            }
        }
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        match self.pressed.take() {
            Some(hit) => {
                // Na coluna (todas as séries), a série é a primeira visível.
                let series = if hit.is_column() { self.data.iter().position(|s| !s.hidden).unwrap_or(0) } else { hit.series };
                self.clicked_series = series as i64;
                self.clicked_index = hit.index as i64;
                let name = self.data.get(series).map(|s| s.name.clone()).unwrap_or_default();
                ctx.fire("on_click", EventArgs::Text(format!("{name};{}", hit.index)));
            }
            None => ctx.fire("on_click", EventArgs::None),
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let plot = self.plot.get();
        if !self.zoom || !self.kind.is_cartesian() || plot.width <= 0.0 {
            return;
        }
        // Aproxima em volta do mouse (até 1/50 do eixo).
        let (w0, w1) = self.window.get();
        let at = ((event.position.x - plot.x) / plot.width).clamp(0.0, 1.0) as f64;
        let focus = w0 + (w1 - w0) * at;
        let factor = if event.delta.y > 0.0 { 0.8 } else { 1.25 };
        let span = ((w1 - w0) * factor).clamp(0.02, 1.0);
        let start = (focus - span * at).clamp(0.0, 1.0 - span);
        self.window.set((start, start + span));
        ctx.set_handled();
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.painted.set(true);
        self.hits.borrow_mut().clear();
        self.legend_hits.borrow_mut().clear();
        let data = self.shown(ctx);
        match self.kind {
            ChartKind::Sparkline => return self.paint_sparkline(canvas, ctx, &data),
            ChartKind::Heatmap => {
                self.paint_heatmap(canvas, ctx, &data);
                if let Some(h) = self.hovered.get() {
                    self.paint_tooltip(canvas, ctx, &data, &h);
                }
                return;
            }
            _ => {}
        }
        let s = ctx.style;
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, s.radius, &Brush::Solid(s.fill));
        if s.border_width > 0.0 {
            canvas.stroke_rect(r, s.radius, &Stroke::new(s.border_color, s.border_width));
        }
        let small = small_text(ctx);
        set_format(&self.value_format, ctx);
        let mut top = r.y + 12.0;
        if !self.title.is_empty() {
            let mut t = ctx.text_style();
            if self.title_bold {
                t.font.weight = FontWeight::Bold;
            }
            t.h_align = match self.title_align.as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Trailing,
                _ => TextAlign::Leading,
            };
            canvas.draw_text(&self.title, RectF::new(r.x + 16.0, top, r.width - 32.0, 22.0), &t);
            top += 26.0;
        }
        // Legenda (`legend_position`: no alto à direita, à esquerda, no meio, ou embaixo);
        // clicar esconde ou mostra o item. No Sankey, os nomes já estão nos nós.
        let mut bottom = r.bottom() - 10.0;
        if self.legend && !data.is_empty() && self.kind != ChartKind::Sankey {
            let items: Vec<(usize, String, bool)> = if self.kind.legend_by_label() {
                self.data_labels.iter().enumerate().map(|(i, l)| (i, l.clone(), self.label_hidden(i))).collect()
            } else {
                data.iter().enumerate().map(|(i, s)| (i, s.name.clone(), s.hidden)).collect()
            };
            let widths: Vec<f32> = items.iter().map(|(_, name, _)| canvas.measure_text(name, &small).width).collect();
            let total: f32 = widths.iter().map(|w| w + 30.0).sum::<f32>() - 14.0;
            let position = self.legend_position.as_str();
            let y = if position == "bottom" { r.bottom() - 26.0 } else { top };
            let mut x = match position {
                "top_left" => r.x + 16.0,
                "top" | "bottom" => r.x + (r.width - total) / 2.0,
                _ => r.right() - 16.0 - total,
            };
            let mut hits = self.legend_hits.borrow_mut();
            for ((i, name, hidden), w) in items.iter().zip(widths) {
                let color = if self.kind.legend_by_label() { self.slice_color(ctx, *i) } else { self.color(ctx, *i) };
                canvas.fill_rect(RectF::new(x, y + 4.0, 10.0, 10.0), 3.0, &Brush::Solid(if *hidden { color.with_opacity(0.3) } else { color }));
                hits.push((RectF::new(x - 2.0, y, w + 20.0, 18.0), *i));
                x += 16.0;
                let mut l = small.clone();
                l.h_align = TextAlign::Leading;
                l.color = if *hidden { theme.color("text_muted") } else { theme.color("text") };
                canvas.draw_text(name, RectF::new(x, y, w + 2.0, 18.0), &l);
                if *hidden {
                    // Riscado, como no Chart.js.
                    canvas.draw_line(PointF::new(x, y + 9.5), PointF::new(x + w, y + 9.5), &Stroke::new(theme.color("text_muted"), 1.0));
                }
                x += w + 14.0;
            }
            if position == "bottom" {
                bottom -= 24.0;
            } else {
                top += 26.0;
            }
        }
        let area = RectF::new(r.x + 12.0, top, r.width - 24.0, bottom - top);
        if self.kind.is_cartesian() {
            cartesian::paint(self, canvas, ctx, area, &data);
        } else if self.kind.is_round() {
            self.paint_round(canvas, ctx, area, &data);
        } else {
            self.paint_more(canvas, ctx, area, &data);
        }
        if let Some(h) = self.hovered.get() {
            self.paint_tooltip(canvas, ctx, &data, &h);
        }
    }
}

impl Chart {
    /// A primeira série como linha suave, com a área embaixo e o último ponto marcado.
    fn paint_sparkline(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, data: &[Series]) {
        let r = ctx.rect.deflate(aylis_core::Bounds::all(4.0));
        let Some(series) = data.first().filter(|s| s.values.len() > 1) else { return };
        let (min, max) = series.values.iter().fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
        let span = (max - min).max(1e-9);
        let n = series.values.len();
        let pts: Vec<PointF> = series
            .values
            .iter()
            .enumerate()
            .map(|(i, v)| PointF::new(r.x + r.width * i as f32 / (n - 1) as f32, r.bottom() - r.height * ((v - min) / span) as f32))
            .collect();
        let color = self.color(ctx, 0);
        let mut line = Path::new();
        line.move_to(pts[0].x, pts[0].y);
        for w in pts.windows(2) {
            // Curva suave: controles no meio do caminho, na horizontal.
            let mx = (w[0].x + w[1].x) / 2.0;
            line.cubic_to(mx, w[0].y, mx, w[1].y, w[1].x, w[1].y);
        }
        let mut area = line.clone();
        area.line_to(r.right(), r.bottom()).line_to(r.x, r.bottom()).close();
        canvas.fill_path(&area, &Brush::Linear {
            start: PointF::new(0.0, r.y),
            end: PointF::new(0.0, r.bottom()),
            stops: vec![aylis_core::GradientStop::new(0.0, color.with_opacity(0.3)), aylis_core::GradientStop::new(1.0, color.with_opacity(0.0))],
        });
        canvas.stroke_path(&line, &Stroke::new(color, 2.0).round());
        let last = pts[n - 1];
        canvas.fill_ellipse(RectF::new(last.x - 3.0, last.y - 3.0, 6.0, 6.0), &Brush::Solid(color));
    }

    /// Uma linha por série, uma coluna por valor; a cor vai do fundo à cor da 1ª série.
    fn paint_heatmap(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, data: &[Series]) {
        let r = ctx.rect;
        let rows = data.len();
        let cols = data.iter().map(|s| s.values.len()).max().unwrap_or(0);
        if rows == 0 || cols == 0 {
            return;
        }
        let small = small_text(ctx);
        let left = data.iter().map(|s| canvas.measure_text(&s.name, &small).width).fold(0.0, f32::max) + 8.0;
        let bottom = if self.data_labels.is_empty() { 0.0 } else { 18.0 };
        let cell_w = (r.width - left) / cols as f32;
        let cell_h = (r.height - bottom) / rows as f32;
        let (min, max) = data.iter().flat_map(|s| s.values.iter()).fold((f64::MAX, f64::MIN), |(a, b), v| (a.min(*v), b.max(*v)));
        let span = (max - min).max(1e-9);
        let hot = self.color(ctx, 0);
        let cold = ctx.theme.color("surface_2");
        let mut hits = self.hits.borrow_mut();
        for (i, s) in data.iter().enumerate() {
            let y = r.y + i as f32 * cell_h;
            canvas.draw_text(&s.name, RectF::new(r.x, y, left - 6.0, cell_h), &small);
            for (j, v) in s.values.iter().enumerate() {
                let t = ((v - min) / span) as f32;
                let cell = RectF::new(r.x + left + j as f32 * cell_w + 1.0, y + 1.0, cell_w - 2.0, cell_h - 2.0);
                canvas.fill_rect(cell, 3.0, &Brush::Solid(cold.lerp(hot, t)));
                hits.push(Hit { shape: Shape::Rect(cell), series: i, index: j, at: cell.center() });
            }
        }
        let mut lab = small.clone();
        lab.h_align = TextAlign::Center;
        for (j, label) in self.data_labels.iter().enumerate().take(cols) {
            canvas.draw_text(label, RectF::new(r.x + left + j as f32 * cell_w, r.bottom() - bottom, cell_w, bottom), &lab);
        }
    }

    /// Pizza e rosca (fatias pelo valor) e área polar (fatias iguais, o raio pelo valor), da
    /// primeira série; os itens escondidos pela legenda ficam de fora.
    fn paint_round(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, area: RectF, data: &[Series]) {
        let Some(series) = data.first() else { return };
        let visible: Vec<(usize, f64)> = series.values.iter().copied().enumerate().filter(|(i, v)| *v > 0.0 && !self.label_hidden(*i)).collect();
        let total: f64 = visible.iter().map(|(_, v)| v).sum();
        if total <= 0.0 {
            return;
        }
        // Com os nomes fora, sobra espaço em volta para eles.
        let outside = self.slice_labels == "outside";
        let room = if outside { area.width.min(area.height) * 0.18 + 30.0 } else { 0.0 };
        let radius = ((area.width - room * 2.0).min(area.height - room) / 2.0 - 4.0).max(10.0);
        let c = area.center();
        let polar = self.kind == ChartKind::PolarArea;
        let donut = self.kind == ChartKind::Donut;
        let hole = if donut { radius * 0.56 } else { 0.0 };
        let max = visible.iter().map(|(_, v)| *v).fold(0.0, f64::max);
        if polar {
            // Grade radial: círculos com os valores.
            let (_, top, step) = nice_scale(0.0, max);
            let grid = Stroke::new(ctx.theme.color("border"), 1.0);
            let mut l = small_text(ctx);
            l.h_align = TextAlign::Leading;
            let mut v = step;
            while v <= top + step * 0.01 {
                let rr = radius * (v / top) as f32;
                canvas.stroke_ellipse(RectF::new(c.x - rr, c.y - rr, rr * 2.0, rr * 2.0), &grid);
                // Do lado de fora do círculo, à direita: não cobre as fatias do meio.
                canvas.draw_text(&short(v), RectF::new(c.x + rr + 3.0, c.y - 9.0, 40.0, 18.0), &l);
                v += step;
            }
        }
        let polar_top = if polar { nice_scale(0.0, max).1 } else { max };
        let mut angle = -std::f32::consts::FRAC_PI_2;
        let mut label = ctx.text_style();
        label.h_align = TextAlign::Center;
        label.color = Color::WHITE;
        label.font.size = (label.font.size - if donut { 3.0 } else { 2.0 }).max(9.0);
        label.font.weight = FontWeight::Bold;
        let hovered = self.hovered.get().map(|h| h.index);
        let mut hits = self.hits.borrow_mut();
        for (i, v) in &visible {
            let sweep = if polar { std::f32::consts::TAU / visible.len() as f32 } else { (*v / total) as f32 * std::f32::consts::TAU };
            let outer = if polar { radius * (*v / polar_top) as f32 } else { radius } + if hovered == Some(*i) { 4.0 } else { 0.0 };
            let color = self.slice_color(ctx, *i);
            // Fatia: arco de fora, volta pelo de dentro (ou pelo centro, na pizza).
            let (a0, sw) = (angle.to_degrees(), sweep.to_degrees());
            let mut wedge = Path::new();
            wedge.arc(c, outer, outer, a0, sw);
            if hole > 0.0 {
                wedge.arc(c, hole, hole, a0 + sw, -sw);
            } else {
                wedge.line_to(c.x, c.y);
            }
            wedge.close();
            canvas.fill_path(&wedge, &Brush::Solid(if polar { color.with_opacity(0.7) } else { color }));
            // Separação fina entre as fatias.
            canvas.stroke_path(&wedge, &Stroke::new(ctx.style.fill, 1.5));
            let mid = angle + sweep / 2.0;
            let at = if donut { (outer + hole) / 2.0 } else { outer * 0.62 };
            let p = PointF::new(c.x + at * mid.cos(), c.y + at * mid.sin());
            let percent = format!("{:.0}%", v / total * 100.0);
            if outside && !polar {
                // O nome e o percentual fora da fatia, ligados a ela por uma linha.
                let (from, to) = (outer + 2.0, outer + 14.0);
                let a = PointF::new(c.x + from * mid.cos(), c.y + from * mid.sin());
                let b = PointF::new(c.x + to * mid.cos(), c.y + to * mid.sin());
                let right = mid.cos() >= 0.0;
                let end = PointF::new(b.x + if right { 10.0 } else { -10.0 }, b.y);
                let line = Stroke::new(color, 1.0);
                canvas.draw_line(a, b, &line);
                canvas.draw_line(b, end, &line);
                let mut text = small_text(ctx);
                text.color = ctx.theme.color("text");
                text.h_align = if right { TextAlign::Leading } else { TextAlign::Trailing };
                let name = self.data_labels.get(*i).cloned().unwrap_or_default();
                let words = if name.is_empty() { percent.clone() } else { format!("{name} {percent}") };
                let w = room + 40.0;
                let rect = if right { RectF::new(end.x + 4.0, end.y - 9.0, w, 18.0) } else { RectF::new(end.x - 4.0 - w, end.y - 9.0, w, 18.0) };
                canvas.draw_text(&words, rect, &text);
            } else if !polar && self.slice_labels != "none" && sweep > if donut { 0.5 } else { 0.35 } {
                canvas.draw_text(&percent, RectF::new(p.x - 30.0, p.y - 10.0, 60.0, 20.0), &label);
            }
            hits.push(Hit { shape: Shape::Wedge(c, hole, outer, angle, sweep), series: 0, index: *i, at: p });
            angle += sweep;
        }
        // O meio da rosca: o total (o padrão), um texto (`center_text`, com `{total}`) ou nada (`-`).
        if donut && self.center_text.trim() != "-" {
            let mut big = ctx.text_style();
            big.h_align = TextAlign::Center;
            big.font.size = (hole * 0.42).clamp(12.0, 34.0);
            big.font.weight = FontWeight::Bold;
            let total_text = if total.fract() == 0.0 { format!("{total:.0}") } else { format!("{total:.1}") };
            let total_text = if self.center_text.trim().is_empty() { total_text } else { self.center_text.replace("{total}", &total_text) };
            canvas.draw_text(&total_text, RectF::new(c.x - hole, c.y - big.font.size * 0.8, hole * 2.0, big.font.size * 1.2), &big);
            let mut small = ctx.text_style();
            small.h_align = TextAlign::Center;
            small.color = ctx.theme.color("text_muted");
            small.font.size = (small.font.size - 2.0).max(9.0);
            canvas.draw_text(&series.name, RectF::new(c.x - hole, c.y + big.font.size * 0.45, hole * 2.0, small.font.size * 1.4), &small);
        }
    }
}

impl Chart {
    /// Onde está cada item da legenda (no último desenho) e o índice dele.
    pub fn legend_items(&self) -> Vec<(RectF, usize)> {
        self.legend_hits.borrow().clone()
    }

    /// A área dos eixos (no último desenho).
    pub fn plot_rect(&self) -> RectF {
        self.plot.get()
    }

    /// O ponto sob `p` (no último desenho): a série e o índice. Na coluna de um rótulo (linha,
    /// barra, área), a série é a primeira visível.
    pub fn item_at(&self, p: PointF) -> Option<(usize, usize)> {
        let hit = self.hits.borrow().iter().rev().find(|h| h.contains(p)).copied()?;
        let series = if hit.is_column() { self.data.iter().position(|s| !s.hidden).unwrap_or(0) } else { hit.series };
        Some((series, hit.index))
    }

    /// No meio da animação de troca dos dados.
    pub fn animating(&self) -> bool {
        !self.anim_from.borrow().is_empty()
    }
}

#[cfg(test)]
mod format_tests {
    /// `value_format: "full"`: os milhares separados pelo idioma, em vez de "60k".
    #[test]
    fn full_numbers_keep_the_digits() {
        assert_eq!(super::full(60000.0, ',', '.'), "60.000");
        assert_eq!(super::full(1234567.5, ',', '.'), "1.234.567,50");
        assert_eq!(super::full(-950.0, '.', ','), "-950");
        assert_eq!(super::full(1000.0, '.', ','), "1,000");
    }
}