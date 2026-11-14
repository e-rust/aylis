//! Componentes de mesa de operação (trader): [`CandleChart`] (candles/OHLC com volume,
//! médias, Bollinger, RSI, MACD, perfil de volume, zoom, cursor e desenhos de tendência e
//! Fibonacci), [`OrderBook`] (livro de ofertas), [`DepthChart`] (profundidade),
//! [`TickerTape`] (faixa de cotações), [`Watchlist`] (lista de ativos que pisca na alta e
//! na baixa) e [`TimeAndSales`] (negócios realizados).

use std::cell::{Cell, RefCell};

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, FontWeight, PaintCtx, Path, PointF, PointerEvent, PropEnum, PropValue,
    RectF, Stroke, TextAlign, TextStyle, Ui, WheelEvent, properties,
};

use crate::bi::format_number;
use crate::leaf_control;

fn small(ctx: &PaintCtx<'_>) -> TextStyle {
    let mut t = ctx.text_style();
    t.font.size = (t.font.size - 2.0).max(9.0);
    t.color = ctx.theme.color("text_muted");
    t
}

fn up(ctx: &PaintCtx<'_>) -> Color {
    ctx.theme.color("success")
}

fn down(ctx: &PaintCtx<'_>) -> Color {
    ctx.theme.color("danger")
}

/// `"38.50:1200;38.49:800"` → [(38.50, 1200), (38.49, 800)].
fn pairs(text: &str) -> Vec<(f64, f64)> {
    text.split(';')
        .filter_map(|p| {
            let (a, b) = p.split_once(':')?;
            Some((a.trim().replace(',', ".").parse().ok()?, b.trim().replace(',', ".").parse().ok()?))
        })
        .collect()
}

// ===========================================================================
// Indicadores

/// Média móvel simples de `n` períodos (`None` antes de ter `n` valores).
pub fn sma(values: &[f64], n: usize) -> Vec<Option<f64>> {
    let n = n.max(1);
    let mut out = Vec::with_capacity(values.len());
    let mut sum = 0.0;
    for (i, v) in values.iter().enumerate() {
        sum += v;
        if i >= n {
            sum -= values[i - n];
        }
        out.push((i + 1 >= n).then(|| sum / n as f64));
    }
    out
}

/// Média móvel exponencial de `n` períodos (começa na simples).
pub fn ema(values: &[f64], n: usize) -> Vec<Option<f64>> {
    let n = n.max(1);
    let k = 2.0 / (n as f64 + 1.0);
    let mut out = Vec::with_capacity(values.len());
    let mut prev: Option<f64> = None;
    for (i, v) in values.iter().enumerate() {
        let next = match prev {
            Some(p) => Some(v * k + p * (1.0 - k)),
            None if i + 1 >= n => Some(values[i + 1 - n..=i].iter().sum::<f64>() / n as f64),
            None => None,
        };
        prev = next;
        out.push(next);
    }
    out
}

/// Bandas de Bollinger: (média, banda de cima, banda de baixo), `k` desvios.
pub fn bollinger(values: &[f64], n: usize, k: f64) -> Vec<Option<(f64, f64, f64)>> {
    let mid = sma(values, n);
    mid.iter()
        .enumerate()
        .map(|(i, m)| {
            let m = (*m)?;
            let window = &values[i + 1 - n.max(1)..=i];
            let var = window.iter().map(|v| (v - m).powi(2)).sum::<f64>() / window.len() as f64;
            let sd = var.sqrt();
            Some((m, m + k * sd, m - k * sd))
        })
        .collect()
}

/// IFR (RSI) de Wilder, de 0 a 100.
pub fn rsi(values: &[f64], n: usize) -> Vec<Option<f64>> {
    let n = n.max(1);
    let mut out = vec![None; values.len()];
    if values.len() <= n {
        return out;
    }
    let (mut gain, mut loss) = (0.0, 0.0);
    for i in 1..=n {
        let d = values[i] - values[i - 1];
        if d > 0.0 { gain += d } else { loss -= d }
    }
    gain /= n as f64;
    loss /= n as f64;
    let value = |g: f64, l: f64| if l == 0.0 { 100.0 } else { 100.0 - 100.0 / (1.0 + g / l) };
    out[n] = Some(value(gain, loss));
    for i in n + 1..values.len() {
        let d = values[i] - values[i - 1];
        gain = (gain * (n as f64 - 1.0) + d.max(0.0)) / n as f64;
        loss = (loss * (n as f64 - 1.0) + (-d).max(0.0)) / n as f64;
        out[i] = Some(value(gain, loss));
    }
    out
}

/// MACD (12, 26, 9): (linha, sinal, histograma).
pub fn macd(values: &[f64]) -> Vec<Option<(f64, f64, f64)>> {
    let (fast, slow) = (ema(values, 12), ema(values, 26));
    let line: Vec<Option<f64>> = fast.iter().zip(&slow).map(|(f, s)| Some((*f)? - (*s)?)).collect();
    let start = line.iter().position(Option::is_some).unwrap_or(line.len());
    let flat: Vec<f64> = line[start..].iter().map(|v| v.unwrap_or(0.0)).collect();
    let signal = ema(&flat, 9);
    (0..values.len())
        .map(|i| {
            let l = line[i]?;
            let s = signal.get(i.checked_sub(start)?)?.as_ref().copied()?;
            Some((l, s, l - s))
        })
        .collect()
}

// ===========================================================================
// CandleChart

/// Um candle: tempo (rótulo), abertura, máxima, mínima, fechamento e volume.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Candle {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// O painel de baixo do gráfico.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LowerPanel {
    None,
    #[default]
    Volume,
    Rsi,
    Macd,
}

impl PropEnum for LowerPanel {
    const NAMES: &'static [&'static str] = &["none", "volume", "rsi", "macd"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Volume, Self::Rsi, Self::Macd].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O que arrastar no gráfico faz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawMode {
    /// Arrastar move o gráfico no tempo.
    #[default]
    Pan,
    /// Linha de tendência.
    Trend,
    /// Retração de Fibonacci (0, 23,6, 38,2, 50, 61,8, 100%).
    Fibonacci,
}

impl PropEnum for DrawMode {
    const NAMES: &'static [&'static str] = &["pan", "trend", "fibonacci"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Pan, Self::Trend, Self::Fibonacci].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Um desenho do usuário, em (índice do candle, preço).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drawing {
    pub fibonacci: bool,
    pub from: (f64, f64),
    pub to: (f64, f64),
}

const FIBO: [f64; 6] = [0.0, 0.236, 0.382, 0.5, 0.618, 1.0];

/// Gráfico de candles (OHLC) com volume, indicadores e ferramentas de desenho.
///
/// `data`: um candle por linha, `tempo;abertura;máxima;mínima;fechamento;volume`; pelo
/// código, [`CandleChart::set_candles`] e [`CandleChart::push`] (o candle em formação:
/// mesmo tempo atualiza o último). `indicators` = `"sma:20;ema:9;bollinger:20"`. Roda do
/// mouse: zoom; arrastar: anda no tempo (ou desenha, com `draw_mode`); o cursor mostra o
/// candle sob o mouse.
pub struct CandleChart {
    pub base: ControlBase,
    /// Os candles, um por linha: `tempo;abertura;máxima;mínima;fechamento;volume`.
    pub data: String,
    /// Os indicadores sobre o preço, separados por `;`: `"sma:20;ema:9;bollinger:20"`.
    pub indicators: String,
    /// O painel de baixo: `none`, `volume`, `rsi` ou `macd`.
    pub lower: LowerPanel,
    /// Mostra o volume por faixa de preço, em barras da direita para dentro.
    pub volume_profile: bool,
    /// O que arrastar faz: `pan` (anda no tempo), `trend` (desenha uma linha de tendência) ou
    /// `fibonacci` (desenha uma retração).
    pub draw_mode: DrawMode,
    /// Casas decimais dos preços.
    pub decimals: u32,
    candles: Vec<Candle>,
    /// Candles à vista e quantos do fim estão escondidos à direita.
    visible: Cell<usize>,
    from_end: Cell<f64>,
    mouse: Cell<Option<PointF>>,
    drag: Option<(PointF, f64)>,
    drawing: Option<Drawing>,
    pub drawings: Vec<Drawing>,
    /// Área de preço da última pintura e a escala (mín, máx, primeiro índice, largura).
    geometry: Cell<(RectF, f64, f64, usize, f32)>,
}

impl Default for CandleChart {
    fn default() -> Self {
        let mut c = Self {
            base: ControlBase::new(640.0, 400.0),
            data: String::new(),
            indicators: "sma:9;sma:21".into(),
            lower: LowerPanel::Volume,
            volume_profile: false,
            draw_mode: DrawMode::Pan,
            decimals: 2,
            candles: Vec::new(),
            visible: Cell::new(60),
            from_end: Cell::new(0.0),
            mouse: Cell::new(None),
            drag: None,
            drawing: None,
            drawings: Vec::new(),
            geometry: Cell::new((RectF::default(), 0.0, 1.0, 0, 1.0)),
        };
        c.data = sample_candles(120);
        c.sync("data");
        c.base.set_cursor(CursorShape::Crosshair);
        c
    }
}

/// Candles de exemplo (um passeio aleatório determinístico).
fn sample_candles(n: usize) -> String {
    let mut price = 30.0f64;
    let mut seed = 7u64;
    let mut rnd = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f64 / (1u64 << 31) as f64) - 0.5
    };
    let mut lines = Vec::new();
    for i in 0..n {
        let open = price;
        let close = (open + rnd() * 1.2 + 0.03).max(1.0);
        let high = open.max(close) + rnd().abs() * 0.6;
        let low = open.min(close) - rnd().abs() * 0.6;
        let volume = 1000.0 + (rnd() + 0.5) * 4000.0;
        lines.push(format!("{:02}/{:02};{open:.2};{high:.2};{low:.2};{close:.2};{volume:.0}", i % 28 + 1, i / 28 + 1));
        price = close;
    }
    lines.join("\n")
}

impl CandleChart {
    pub fn candles(&self) -> &[Candle] {
        &self.candles
    }

    pub fn set_candles(&mut self, candles: Vec<Candle>) {
        self.candles = candles;
        self.data.clear();
    }

    /// Acrescenta um candle; com o mesmo `time` do último, atualiza o último (o candle do
    /// período em andamento, a cada negócio).
    pub fn push(ui: &mut Ui, id: ComponentId, candle: Candle) {
        if let Some(c) = ui.get_mut::<CandleChart>(id) {
            match c.candles.last_mut() {
                Some(last) if last.time == candle.time => *last = candle,
                _ => c.candles.push(candle),
            }
            c.data.clear();
        }
        ui.request_repaint();
    }

    fn sync(&mut self, name: &str) {
        if name == "data" && !self.data.is_empty() {
            self.candles = self
                .data
                .lines()
                .filter_map(|l| {
                    let f: Vec<&str> = l.split(';').map(str::trim).collect();
                    let n = |i: usize| f.get(i).and_then(|v| v.replace(',', ".").parse::<f64>().ok());
                    Some(Candle { time: f.first()?.to_string(), open: n(1)?, high: n(2)?, low: n(3)?, close: n(4)?, volume: n(5).unwrap_or(0.0) })
                })
                .collect();
        }
    }

    /// O intervalo à vista: (primeiro índice, quantos).
    fn window(&self) -> (usize, usize) {
        let n = self.candles.len();
        let count = self.visible.get().clamp(5, n.max(5));
        let end = n.saturating_sub(self.from_end.get().round().max(0.0) as usize);
        let start = end.saturating_sub(count);
        (start, end - start)
    }

    fn overlays(&self) -> Vec<(String, usize)> {
        self.indicators
            .split(';')
            .filter_map(|p| {
                let (name, n) = p.split_once(':').unwrap_or((p, "20"));
                Some((name.trim().to_lowercase(), n.trim().parse().ok()?))
            })
            .collect()
    }

    /// Ponto da tela → (índice, preço).
    fn to_data(&self, p: PointF) -> (f64, f64) {
        let (plot, min, max, start, slot) = self.geometry.get();
        let index = start as f64 + ((p.x - plot.x) / slot) as f64 - 0.5;
        let price = max - ((p.y - plot.y) / plot.height) as f64 * (max - min);
        (index, price)
    }

    fn to_screen(&self, (index, price): (f64, f64)) -> PointF {
        let (plot, min, max, start, slot) = self.geometry.get();
        PointF::new(plot.x + ((index - start as f64 + 0.5) as f32) * slot, plot.y + ((max - price) / (max - min)) as f32 * plot.height)
    }
}

impl Component for CandleChart {
    leaf_control!("CandleChart");

    properties! {
        data: String => "Data",
        indicators: String => "Behavior",
        lower: LowerPanel => "Appearance",
        volume_profile: bool => "Appearance",
        draw_mode: DrawMode => "Behavior",
        decimals: u32 => "Appearance";
        events: on_drawing;
        on_set: sync
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        // Roda: zoom (para cima aproxima); deslizar na horizontal anda no tempo.
        if event.delta.x.abs() > event.delta.y.abs() {
            let max = self.candles.len().saturating_sub(5) as f64;
            self.from_end.set((self.from_end.get() + event.delta.x as f64 / 8.0).clamp(0.0, max));
        } else {
            let factor = if event.delta.y > 0.0 { 0.85 } else { 1.0 / 0.85 };
            let next = ((self.visible.get() as f32 * factor).round() as usize).clamp(10, self.candles.len().max(10));
            self.visible.set(next);
        }
        ctx.set_handled();
        ctx.repaint();
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        match self.draw_mode {
            DrawMode::Pan => self.drag = Some((event.position, self.from_end.get())),
            mode => {
                let at = self.to_data(event.position);
                self.drawing = Some(Drawing { fibonacci: mode == DrawMode::Fibonacci, from: at, to: at });
            }
        }
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.mouse.set(Some(event.position));
        if let Some((start, from_end)) = self.drag {
            let (_, _, _, _, slot) = self.geometry.get();
            let moved = ((event.position.x - start.x) / slot.max(0.1)) as f64;
            let max = self.candles.len().saturating_sub(5) as f64;
            self.from_end.set((from_end + moved).clamp(0.0, max));
        }
        if let Some(d) = self.drawing.as_mut() {
            d.to = {
                let (plot, min, max, start, slot) = self.geometry.get();
                let p = event.position;
                (start as f64 + ((p.x - plot.x) / slot) as f64 - 0.5, max - ((p.y - plot.y) / plot.height) as f64 * (max - min))
            };
        }
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _event: &PointerEvent) {
        self.drag = None;
        if let Some(d) = self.drawing.take() {
            if (d.from.0 - d.to.0).abs() > 0.5 || (d.from.1 - d.to.1).abs() > f64::EPSILON {
                self.drawings.push(d);
                ctx.fire("on_drawing", EventArgs::Value(PropValue::Int(self.drawings.len() as i64)));
            }
        }
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.mouse.set(None);
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        let (start, count) = self.window();
        if count == 0 {
            return;
        }
        let shown = &self.candles[start..start + count];
        let s = small(ctx);
        let locale = ctx.ui.locale();
        let decimals = self.decimals as usize;
        let price_w = canvas.measure_text(&format_number(shown.iter().map(|c| c.high).fold(0.0, f64::max), decimals, &locale), &s).width + 14.0;
        let lower_h = if self.lower == LowerPanel::None { 0.0 } else { (r.height * 0.24).max(50.0) };
        let plot = RectF::new(r.x + 8.0, r.y + 10.0, r.width - price_w - 16.0, r.height - lower_h - 34.0);
        let lower = RectF::new(plot.x, plot.bottom() + 8.0, plot.width, (lower_h - 8.0).max(0.0));
        let closes: Vec<f64> = self.candles.iter().map(|c| c.close).collect();
        let overlays = self.overlays();
        // Escala: os candles e os indicadores à vista.
        let (mut lo, mut hi) = shown.iter().fold((f64::MAX, f64::MIN), |(a, b), c| (a.min(c.low), b.max(c.high)));
        let boll: Vec<(usize, Vec<Option<(f64, f64, f64)>>)> = overlays.iter().filter(|(k, _)| k == "bollinger").map(|(_, n)| (*n, bollinger(&closes, *n, 2.0))).collect();
        for (_, b) in &boll {
            for (_, u, l) in b[start..start + count].iter().flatten() {
                (lo, hi) = (lo.min(*l), hi.max(*u));
            }
        }
        let pad = (hi - lo).max(1e-9) * 0.06;
        let (min, max) = (lo - pad, hi + pad);
        let slot = plot.width / count as f32;
        self.geometry.set((plot, min, max, start, slot));
        let y = |v: f64| plot.y + ((max - v) / (max - min)) as f32 * plot.height;
        let x = |i: usize| plot.x + slot * ((i - start) as f32 + 0.5);
        // Grade e eixo de preço à direita.
        let grid = Stroke::new(ctx.theme.color("border").with_opacity(0.6), 1.0);
        let mut pl = s.clone();
        pl.h_align = TextAlign::Leading;
        for k in 0..=5 {
            let v = min + (max - min) * k as f64 / 5.0;
            let yy = y(v).round() + 0.5;
            canvas.draw_line(PointF::new(plot.x, yy), PointF::new(plot.right(), yy), &grid);
            canvas.draw_text(&format_number(v, decimals, &locale), RectF::new(plot.right() + 6.0, yy - 9.0, price_w, 18.0), &pl);
        }
        // Perfil de volume: volume por faixa de preço, da direita para dentro.
        if self.volume_profile {
            const BINS: usize = 24;
            let mut bins = [0.0f64; BINS];
            for c in shown {
                let mid = (c.high + c.low) / 2.0;
                let b = (((mid - min) / (max - min)) * BINS as f64).clamp(0.0, BINS as f64 - 1.0) as usize;
                bins[b] += c.volume;
            }
            let top = bins.iter().copied().fold(1.0, f64::max);
            let h = plot.height / BINS as f32;
            for (b, v) in bins.iter().enumerate() {
                let w = plot.width * 0.25 * (*v / top) as f32;
                let yy = plot.bottom() - h * (b + 1) as f32;
                canvas.fill_rect(RectF::new(plot.right() - w, yy + 1.0, w, h - 2.0), 0.0, &Brush::Solid(ctx.theme.color("primary").with_opacity(0.16)));
            }
        }
        // Bollinger (faixa) e médias.
        for (_, b) in &boll {
            let mut upper = Path::new();
            let mut lower_band = Path::new();
            let mut started = false;
            for i in start..start + count {
                if let Some((_, u, l)) = b[i] {
                    if started {
                        upper.line_to(x(i), y(u));
                        lower_band.line_to(x(i), y(l));
                    } else {
                        upper.move_to(x(i), y(u));
                        lower_band.move_to(x(i), y(l));
                        started = true;
                    }
                }
            }
            let band = Stroke::new(ctx.theme.color("primary").with_opacity(0.5), 1.0);
            canvas.stroke_path(&upper, &band);
            canvas.stroke_path(&lower_band, &band);
        }
        let colors = ["#F59E0B", "#8B5CF6", "#06B6D4", "#EC4899"];
        for (k, (kind, n)) in overlays.iter().filter(|(k, _)| k == "sma" || k == "ema").enumerate() {
            let values = if kind == "ema" { ema(&closes, *n) } else { sma(&closes, *n) };
            let mut path = Path::new();
            let mut started = false;
            for (i, v) in values.iter().enumerate().take(start + count).skip(start) {
                if let Some(v) = v {
                    if started { path.line_to(x(i), y(*v)) } else { path.move_to(x(i), y(*v)) };
                    started = true;
                }
            }
            canvas.stroke_path(&path, &Stroke::new(Color::from_hex(colors[k % colors.len()]).unwrap_or(Color::BLACK), 1.5));
        }
        // Candles.
        let body_w = (slot * 0.7).max(1.0);
        for (k, c) in shown.iter().enumerate() {
            let i = start + k;
            let color = if c.close >= c.open { up(ctx) } else { down(ctx) };
            canvas.draw_line(PointF::new(x(i), y(c.high)), PointF::new(x(i), y(c.low)), &Stroke::new(color, 1.0));
            let (top, bottom) = (y(c.open.max(c.close)), y(c.open.min(c.close)));
            canvas.fill_rect(RectF::new(x(i) - body_w / 2.0, top, body_w, (bottom - top).max(1.0)), 0.0, &Brush::Solid(color));
        }
        // Painel de baixo.
        if lower.height > 0.0 {
            canvas.draw_line(PointF::new(lower.x, lower.y - 4.0), PointF::new(lower.right(), lower.y - 4.0), &grid);
            match self.lower {
                LowerPanel::Volume => {
                    let top = shown.iter().map(|c| c.volume).fold(1.0, f64::max);
                    for (k, c) in shown.iter().enumerate() {
                        let h = lower.height * (c.volume / top) as f32;
                        let color = if c.close >= c.open { up(ctx) } else { down(ctx) };
                        canvas.fill_rect(RectF::new(x(start + k) - body_w / 2.0, lower.bottom() - h, body_w, h), 0.0, &Brush::Solid(color.with_opacity(0.5)));
                    }
                }
                LowerPanel::Rsi => {
                    let values = rsi(&closes, 14);
                    let ly = |v: f64| lower.bottom() - (v / 100.0) as f32 * lower.height;
                    for level in [30.0, 70.0] {
                        canvas.draw_line(PointF::new(lower.x, ly(level)), PointF::new(lower.right(), ly(level)), &Stroke::new(ctx.theme.color("text_muted"), 1.0).dashed(vec![3.0, 3.0]));
                    }
                    let mut path = Path::new();
                    let mut started = false;
                    for (i, v) in values.iter().enumerate().take(start + count).skip(start) {
                        if let Some(v) = v {
                            if started { path.line_to(x(i), ly(*v)) } else { path.move_to(x(i), ly(*v)) };
                            started = true;
                        }
                    }
                    canvas.stroke_path(&path, &Stroke::new(ctx.theme.color("primary"), 1.5));
                    canvas.draw_text("IFR 14", RectF::new(lower.x + 4.0, lower.y, 80.0, 16.0), &pl);
                }
                LowerPanel::Macd => {
                    let values = macd(&closes);
                    let range = values[start..start + count].iter().flatten().fold(1e-9f64, |a, (l, s, h)| a.max(l.abs()).max(s.abs()).max(h.abs()));
                    let ly = |v: f64| lower.y + lower.height / 2.0 - (v / range) as f32 * lower.height / 2.0;
                    let (mut line, mut signal) = (Path::new(), Path::new());
                    let mut started = false;
                    for (i, v) in values.iter().enumerate().take(start + count).skip(start) {
                        let Some((l, s, h)) = v else { continue };
                        let color = if *h >= 0.0 { up(ctx) } else { down(ctx) };
                        let (a, b) = (ly(0.0), ly(*h));
                        canvas.fill_rect(RectF::new(x(i) - body_w / 2.0, a.min(b), body_w, (a - b).abs().max(1.0)), 0.0, &Brush::Solid(color.with_opacity(0.5)));
                        if started {
                            line.line_to(x(i), ly(*l));
                            signal.line_to(x(i), ly(*s));
                        } else {
                            line.move_to(x(i), ly(*l));
                            signal.move_to(x(i), ly(*s));
                            started = true;
                        }
                    }
                    canvas.stroke_path(&line, &Stroke::new(ctx.theme.color("primary"), 1.5));
                    canvas.stroke_path(&signal, &Stroke::new(Color::from_hex("#F59E0B").unwrap_or(Color::BLACK), 1.5));
                    canvas.draw_text("MACD 12 26 9", RectF::new(lower.x + 4.0, lower.y, 120.0, 16.0), &pl);
                }
                LowerPanel::None => {}
            }
        }
        // Tempo embaixo.
        let mut tl = s.clone();
        tl.h_align = TextAlign::Center;
        let every = ((70.0 / slot).ceil() as usize).max(1);
        for (k, c) in shown.iter().enumerate() {
            if (start + k) % every == 0 {
                canvas.draw_text(&c.time, RectF::new(x(start + k) - 35.0, r.bottom() - 22.0, 70.0, 18.0), &tl);
            }
        }
        // Desenhos.
        canvas.save();
        canvas.clip_rect(plot);
        for d in self.drawings.iter().chain(self.drawing.iter()) {
            let (a, b) = (self.to_screen(d.from), self.to_screen(d.to));
            if d.fibonacci {
                let mut fl = s.clone();
                fl.h_align = TextAlign::Leading;
                for level in FIBO {
                    let price = d.from.1 + (d.to.1 - d.from.1) * (1.0 - level);
                    let yy = y(price);
                    canvas.draw_line(PointF::new(a.x.min(b.x), yy), PointF::new(plot.right(), yy), &Stroke::new(ctx.theme.color("warning"), 1.0));
                    canvas.draw_text(&format!("{:.1}%  {}", level * 100.0, format_number(price, decimals, &locale)), RectF::new(a.x.min(b.x) + 4.0, yy - 16.0, 160.0, 16.0), &fl);
                }
            } else {
                canvas.draw_line(a, b, &Stroke::new(ctx.theme.color("primary"), 2.0));
            }
        }
        canvas.restore();
        // Cursor: linhas cruzadas, o preço e o candle sob o mouse.
        if let Some(m) = self.mouse.get().filter(|m| plot.contains(*m)) {
            let cross = Stroke::new(ctx.theme.color("text_muted"), 1.0).dashed(vec![4.0, 3.0]);
            canvas.draw_line(PointF::new(plot.x, m.y), PointF::new(plot.right(), m.y), &cross);
            canvas.draw_line(PointF::new(m.x, plot.y), PointF::new(m.x, r.bottom() - 24.0), &cross);
            let price = max - ((m.y - plot.y) / plot.height) as f64 * (max - min);
            let tag = RectF::new(plot.right() + 2.0, m.y - 10.0, price_w, 20.0);
            canvas.fill_rect(tag, 3.0, &Brush::Solid(ctx.theme.color("text")));
            let mut tt = pl.clone();
            tt.color = ctx.theme.color("background");
            canvas.draw_text(&format_number(price, decimals, &locale), RectF::new(tag.x + 4.0, tag.y, tag.width - 4.0, 20.0), &tt);
            let i = start + (((m.x - plot.x) / slot) as usize).min(count - 1);
            if let Some(c) = self.candles.get(i) {
                let f = |v: f64| format_number(v, decimals, &locale);
                let info = format!("{}  A {}  M {}  m {}  F {}  V {}", c.time, f(c.open), f(c.high), f(c.low), f(c.close), format_number(c.volume, 0, &locale));
                let mut it = s.clone();
                it.h_align = TextAlign::Leading;
                it.color = ctx.theme.color("text");
                canvas.draw_text(&info, RectF::new(plot.x + 4.0, plot.y, plot.width - 8.0, 18.0), &it);
            }
        }
    }
}

// ===========================================================================
// OrderBook / DepthChart

/// Livro de ofertas: as vendas (`asks`, do menor preço para cima) em cima, as compras
/// (`bids`) embaixo, o spread no meio; a barra de cada linha mostra o acumulado. Clique
/// numa linha: `on_price_click` com o preço (para a boleta).
pub struct OrderBook {
    pub base: ControlBase,
    /// `"38.50:1200;38.49:800"` (preço:quantidade).
    pub bids: String,
    /// As ofertas de venda, no mesmo formato das `bids` (preço:quantidade).
    pub asks: String,
    /// Quantos preços aparecem de cada lado.
    pub levels: u32,
    /// Casas decimais dos preços.
    pub decimals: u32,
    rows: RefCell<Vec<(RectF, f64)>>,
    pressed: Option<PointF>,
}

impl Default for OrderBook {
    fn default() -> Self {
        let mut base = ControlBase::new(260.0, 420.0);
        base.set_cursor(CursorShape::Pointer);
        Self {
            base,
            bids: "38.49:1200;38.48:3400;38.47:900;38.46:5200;38.45:2100;38.44:800;38.43:4100;38.42:1500".into(),
            asks: "38.50:700;38.51:2600;38.52:1800;38.53:4400;38.54:900;38.55:3100;38.56:1200;38.57:2500".into(),
            levels: 8,
            decimals: 2,
            rows: RefCell::default(),
            pressed: None,
        }
    }
}

impl OrderBook {
    /// Troca o livro pelo código.
    pub fn set_book(ui: &mut Ui, id: ComponentId, bids: &[(f64, f64)], asks: &[(f64, f64)]) {
        let text = |levels: &[(f64, f64)]| levels.iter().map(|(p, q)| format!("{p}:{q}")).collect::<Vec<_>>().join(";");
        let _ = ui.set_prop(id, "bids", text(bids));
        let _ = ui.set_prop(id, "asks", text(asks));
    }

    /// Compras (maior preço primeiro) e vendas (menor primeiro), até `levels`.
    fn book(&self) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let mut bids = pairs(&self.bids);
        let mut asks = pairs(&self.asks);
        bids.sort_by(|a, b| b.0.total_cmp(&a.0));
        asks.sort_by(|a, b| a.0.total_cmp(&b.0));
        bids.truncate(self.levels as usize);
        asks.truncate(self.levels as usize);
        (bids, asks)
    }
}

impl Component for OrderBook {
    leaf_control!("OrderBook");

    properties! {
        bids: String => "Data",
        asks: String => "Data",
        levels: u32 => "Appearance",
        decimals: u32 => "Appearance";
        events: on_price_click
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pressed = Some(event.position);
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        let Some(p) = self.pressed.take() else { return };
        let price = self.rows.borrow().iter().find(|(r, _)| r.contains(p)).map(|(_, price)| *price);
        if let Some(price) = price {
            ctx.fire("on_price_click", EventArgs::Value(PropValue::Float(price)));
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        let (bids, asks) = self.book();
        let locale = ctx.ui.locale();
        let d = self.decimals as usize;
        let mut t = ctx.text_style();
        t.font.size = (t.font.size - 1.0).max(10.0);
        let row_h = t.font.size * 1.7;
        let head = small(ctx);
        let cols = [r.width * 0.36, r.width * 0.30, r.width * 0.30];
        let mut hy = r.y + 4.0;
        for (k, label) in ["Preço", "Qtd.", "Total"].iter().enumerate() {
            let mut h = head.clone();
            h.h_align = TextAlign::Trailing;
            let x0 = r.x + cols[..k].iter().sum::<f32>();
            canvas.draw_text(label, RectF::new(x0, hy, cols[k] - 8.0, row_h), &h);
        }
        hy += row_h;
        let cum = |levels: &[(f64, f64)]| levels.iter().scan(0.0, |acc, (_, q)| { *acc += q; Some(*acc) }).collect::<Vec<f64>>();
        let (bid_cum, ask_cum) = (cum(&bids), cum(&asks));
        let top = bid_cum.last().copied().unwrap_or(0.0).max(ask_cum.last().copied().unwrap_or(0.0)).max(1.0);
        let mut rows = Vec::new();
        let mut row = |canvas: &mut dyn Canvas, y: f32, (price, qty): (f64, f64), total: f64, color: Color| {
            let rect = RectF::new(r.x, y, r.width, row_h);
            let w = r.width * (total / top) as f32;
            canvas.fill_rect(RectF::new(r.right() - w, y + 1.0, w, row_h - 2.0), 0.0, &Brush::Solid(color.with_opacity(0.14)));
            let values = [format_number(price, d, &locale), format_number(qty, 0, &locale), format_number(total, 0, &locale)];
            for (k, v) in values.iter().enumerate() {
                let mut s = t.clone();
                s.h_align = TextAlign::Trailing;
                if k == 0 {
                    s.color = color;
                    s.font.weight = FontWeight::Bold;
                }
                let x0 = r.x + cols[..k].iter().sum::<f32>();
                canvas.draw_text(v, RectF::new(x0, y, cols[k] - 8.0, row_h), &s);
            }
            rows.push((rect, price));
        };
        // Vendas: a melhor (menor) logo acima do spread.
        let ask_area = row_h * asks.len() as f32;
        for (k, level) in asks.iter().enumerate() {
            let y = hy + ask_area - row_h * (k + 1) as f32;
            row(canvas, y, *level, ask_cum[k], down(ctx));
        }
        let mut sy = hy + ask_area;
        if let (Some(b), Some(a)) = (bids.first(), asks.first()) {
            let mut s = small(ctx);
            s.h_align = TextAlign::Center;
            let spread = a.0 - b.0;
            canvas.fill_rect(RectF::new(r.x, sy, r.width, row_h), 0.0, &Brush::Solid(ctx.theme.color("surface_2")));
            canvas.draw_text(&format!("Spread {}", format_number(spread, d, &locale)), RectF::new(r.x, sy, r.width, row_h), &s);
            sy += row_h;
        }
        for (k, level) in bids.iter().enumerate() {
            row(canvas, sy + row_h * k as f32, *level, bid_cum[k], up(ctx));
        }
        *self.rows.borrow_mut() = rows.into_iter().map(|(rect, p)| (RectF::new(rect.x - r.x, rect.y - r.y, rect.width, rect.height), p)).collect();
    }
}

/// Gráfico de profundidade: compras acumuladas (verde, à esquerda do meio) e vendas
/// acumuladas (vermelho, à direita), em degraus; o cursor mostra preço e acumulado.
pub struct DepthChart {
    pub base: ControlBase,
    /// As ofertas de compra: `"38.50:1200;38.49:800"` (preço:quantidade).
    pub bids: String,
    /// As ofertas de venda, no mesmo formato (preço:quantidade).
    pub asks: String,
    /// Casas decimais dos preços.
    pub decimals: u32,
    mouse: Cell<Option<PointF>>,
}

impl Default for DepthChart {
    fn default() -> Self {
        let book = OrderBook::default();
        Self { base: ControlBase::new(420.0, 220.0), bids: book.bids, asks: book.asks, decimals: 2, mouse: Cell::new(None) }
    }
}

impl Component for DepthChart {
    leaf_control!("DepthChart");

    properties! {
        bids: String => "Data",
        asks: String => "Data",
        decimals: u32 => "Appearance"
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.mouse.set(Some(event.position));
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.mouse.set(None);
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        let mut bids = pairs(&self.bids);
        let mut asks = pairs(&self.asks);
        bids.sort_by(|a, b| b.0.total_cmp(&a.0));
        asks.sort_by(|a, b| a.0.total_cmp(&b.0));
        if bids.is_empty() || asks.is_empty() {
            return;
        }
        let plot = RectF::new(r.x + 8.0, r.y + 10.0, r.width - 16.0, r.height - 34.0);
        let (lo, hi) = (bids.last().map_or(0.0, |b| b.0), asks.last().map_or(1.0, |a| a.0));
        let total = |v: &[(f64, f64)]| v.iter().map(|(_, q)| q).sum::<f64>();
        let top = total(&bids).max(total(&asks)).max(1.0);
        let x = |p: f64| plot.x + ((p - lo) / (hi - lo).max(1e-9)) as f32 * plot.width;
        let y = |q: f64| plot.bottom() - (q / top) as f32 * plot.height;
        let side = |levels: &[(f64, f64)], color: Color, canvas: &mut dyn Canvas| {
            let mut path = Path::new();
            path.move_to(x(levels[0].0), plot.bottom());
            let mut acc = 0.0;
            for (p, q) in levels {
                path.line_to(x(*p), y(acc));
                acc += q;
                path.line_to(x(*p), y(acc));
            }
            let mut area = path.clone();
            area.line_to(x(levels.last().map_or(0.0, |l| l.0)), plot.bottom()).close();
            canvas.fill_path(&area, &Brush::Solid(color.with_opacity(0.18)));
            canvas.stroke_path(&path, &Stroke::new(color, 2.0));
        };
        side(&bids, up(ctx), canvas);
        side(&asks, down(ctx), canvas);
        let locale = ctx.ui.locale();
        let d = self.decimals as usize;
        let mut s = small(ctx);
        s.h_align = TextAlign::Center;
        for p in [lo, (bids[0].0 + asks[0].0) / 2.0, hi] {
            canvas.draw_text(&format_number(p, d, &locale), RectF::new(x(p) - 40.0, plot.bottom() + 4.0, 80.0, 18.0), &s);
        }
        if let Some(m) = self.mouse.get().filter(|m| plot.contains(*m)) {
            let price = lo + ((m.x - plot.x) / plot.width) as f64 * (hi - lo);
            let mid = (bids[0].0 + asks[0].0) / 2.0;
            let acc: f64 = if price <= mid { bids.iter().filter(|(p, _)| *p >= price).map(|(_, q)| q).sum() } else { asks.iter().filter(|(p, _)| *p <= price).map(|(_, q)| q).sum() };
            canvas.draw_line(PointF::new(m.x, plot.y), PointF::new(m.x, plot.bottom()), &Stroke::new(ctx.theme.color("text_muted"), 1.0).dashed(vec![4.0, 3.0]));
            let mut t = ctx.text_style();
            t.h_align = TextAlign::Leading;
            let label = format!("{} · {}", format_number(price, d, &locale), format_number(acc, 0, &locale));
            canvas.draw_text(&label, RectF::new((m.x + 8.0).min(plot.right() - 160.0), plot.y, 160.0, 20.0), &t);
        }
    }
}

// ===========================================================================
// TickerTape

/// Faixa de cotações rolando (como no rodapé dos canais de notícias). `items` =
/// `"PETR4:38.50:1.2;VALE3:61.10:-0.8"` (ativo:preço:variação %); `speed` em pontos por
/// segundo; para enquanto o mouse está em cima.
pub struct TickerTape {
    pub base: ControlBase,
    /// As cotações, separadas por `;`: `"PETR4:38.50:1.2"` (ativo:preço:variação %).
    pub items: String,
    /// A velocidade da rolagem, em pontos por segundo.
    pub speed: f32,
    /// Casas decimais dos preços.
    pub decimals: u32,
    paused_at: Cell<Option<f64>>,
    shift: Cell<f64>,
    last: Cell<Option<f64>>,
    hover: bool,
}

impl Default for TickerTape {
    fn default() -> Self {
        Self {
            base: ControlBase::new(600.0, 34.0),
            items: "PETR4:38.50:1.2;VALE3:61.10:-0.8;ITUB4:33.42:0.4;BBDC4:14.05:-1.6;ABEV3:12.88:0.1;WEGE3:52.30:2.3;MGLU3:9.12:-3.1".into(),
            speed: 60.0,
            decimals: 2,
            paused_at: Cell::new(None),
            shift: Cell::new(0.0),
            last: Cell::new(None),
            hover: false,
        }
    }
}

impl Component for TickerTape {
    leaf_control!("TickerTape");

    properties! {
        items: String => "Data",
        speed: f32 => "Behavior",
        decimals: u32 => "Appearance"
    }

    fn pointer_enter(&mut self, _ctx: &mut EventCtx<'_>) {
        self.hover = true;
    }

    fn pointer_leave(&mut self, _ctx: &mut EventCtx<'_>) {
        self.hover = false;
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.theme.color("surface_2")));
        // Avança pelo tempo desde a última pintura (parado com o mouse em cima).
        let dt = self.last.get().map_or(0.0, |t| (ctx.time - t).clamp(0.0, 0.25));
        self.last.set(Some(ctx.time));
        if !self.hover {
            self.shift.set(self.shift.get() + dt * self.speed as f64);
        }
        let _ = self.paused_at.get();
        let locale = ctx.ui.locale();
        let mut t = ctx.text_style();
        t.h_align = TextAlign::Leading;
        let items: Vec<(String, f64, f64)> = self
            .items
            .split(';')
            .filter_map(|i| {
                let f: Vec<&str> = i.split(':').collect();
                Some((f.first()?.trim().to_string(), f.get(1)?.trim().parse().ok()?, f.get(2).and_then(|v| v.trim().parse().ok()).unwrap_or(0.0)))
            })
            .collect();
        if items.is_empty() {
            return;
        }
        let pieces: Vec<(String, String, f32, f32)> = items
            .iter()
            .map(|(sym, price, change)| {
                let a = format!("{sym}  {}", format_number(*price, self.decimals as usize, &locale));
                let b = format!("{} {}%", if *change >= 0.0 { "▲" } else { "▼" }, format_number(change.abs(), 2, &locale));
                let (wa, wb) = (canvas.measure_text(&a, &t).width, canvas.measure_text(&b, &t).width);
                (a, b, wa, wb)
            })
            .collect();
        let gap = 36.0;
        let cycle: f32 = pieces.iter().map(|(_, _, wa, wb)| wa + wb + 8.0 + gap).sum();
        let offset = (self.shift.get() % cycle as f64) as f32;
        canvas.save();
        canvas.clip_rect(r);
        let mut x = r.x - offset;
        while x < r.right() {
            for ((a, b, wa, wb), (_, _, change)) in pieces.iter().zip(&items) {
                if x + wa + wb > r.x {
                    canvas.draw_text(a, RectF::new(x, r.y, wa + 2.0, r.height), &t);
                    let mut c = t.clone();
                    c.color = if *change >= 0.0 { up(ctx) } else { down(ctx) };
                    canvas.draw_text(b, RectF::new(x + wa + 8.0, r.y, wb + 2.0, r.height), &c);
                }
                x += wa + wb + 8.0 + gap;
            }
        }
        canvas.restore();
        // Só enquanto anda: parado com o mouse em cima (ou sem velocidade), nada muda.
        if !self.hover && self.speed != 0.0 && !items.is_empty() {
            ctx.request_frame();
        }
    }
}

// ===========================================================================
// Watchlist

/// Uma linha da lista de ativos.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quote {
    pub symbol: String,
    pub last: f64,
    pub change: f64,
    /// Quando mudou e para que lado (para piscar).
    flash: Option<(f64, bool)>,
}

/// Lista de ativos: ativo, último preço e variação; a célula pisca verde na alta e
/// vermelha na baixa a cada [`Watchlist::update`]. Clique: `on_select` com o ativo.
pub struct Watchlist {
    pub base: ControlBase,
    /// `"PETR4;VALE3;ITUB4"` (os preços chegam por `update`).
    pub symbols: String,
    /// Casas decimais dos preços.
    pub decimals: u32,
    quotes: Vec<Quote>,
    selected: Option<usize>,
    pressed: Option<PointF>,
    row_h: Cell<f32>,
}

impl Default for Watchlist {
    fn default() -> Self {
        let mut w = Self {
            base: ControlBase::new(300.0, 260.0),
            symbols: "PETR4;VALE3;ITUB4;BBDC4;ABEV3;WEGE3".into(),
            decimals: 2,
            quotes: Vec::new(),
            selected: None,
            pressed: None,
            row_h: Cell::new(28.0),
        };
        w.base.set_cursor(CursorShape::Pointer);
        w.sync("symbols");
        w
    }
}

impl Watchlist {
    /// Novo preço de um ativo (o ativo entra na lista se não estiver): a célula pisca.
    pub fn update(ui: &mut Ui, id: ComponentId, symbol: &str, last: f64, change_pct: f64) {
        let now = ui.time();
        if let Some(w) = ui.get_mut::<Watchlist>(id) {
            let i = match w.quotes.iter().position(|q| q.symbol.eq_ignore_ascii_case(symbol)) {
                Some(i) => i,
                None => {
                    w.quotes.push(Quote { symbol: symbol.to_string(), ..Default::default() });
                    w.quotes.len() - 1
                }
            };
            let q = &mut w.quotes[i];
            if q.last != 0.0 && last != q.last {
                q.flash = Some((now, last > q.last));
            }
            q.last = last;
            q.change = change_pct;
        }
        ui.request_repaint();
    }

    pub fn quotes(&self) -> &[Quote] {
        &self.quotes
    }

    fn sync(&mut self, name: &str) {
        if name == "symbols" {
            let wanted: Vec<String> = self.symbols.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            self.quotes.retain(|q| wanted.iter().any(|w| w.eq_ignore_ascii_case(&q.symbol)));
            for s in wanted {
                if !self.quotes.iter().any(|q| q.symbol.eq_ignore_ascii_case(&s)) {
                    self.quotes.push(Quote { symbol: s, ..Default::default() });
                }
            }
        }
    }
}

/// Quanto tempo a célula fica acesa depois de mudar (s).
const FLASH: f64 = 0.9;

impl Component for Watchlist {
    leaf_control!("Watchlist");

    properties! {
        symbols: String => "Data",
        decimals: u32 => "Appearance";
        events: on_select;
        on_set: sync
    }

    fn pointer_down(&mut self, _ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pressed = Some(event.position);
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        let Some(p) = self.pressed.take() else { return };
        let h = self.row_h.get();
        let i = ((p.y - h) / h).floor();
        if i >= 0.0 && (i as usize) < self.quotes.len() {
            self.selected = Some(i as usize);
            ctx.fire("on_select", EventArgs::Text(self.quotes[i as usize].symbol.clone()));
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        let locale = ctx.ui.locale();
        let mut t = ctx.text_style();
        t.font.size = (t.font.size - 1.0).max(10.0);
        let h = t.font.size * 2.2;
        self.row_h.set(h);
        let cols = [r.width * 0.36, r.width * 0.34, r.width * 0.30];
        let head = small(ctx);
        for (k, label) in ["Ativo", "Último", "Var. %"].iter().enumerate() {
            let mut s = head.clone();
            s.h_align = if k == 0 { TextAlign::Leading } else { TextAlign::Trailing };
            let x0 = r.x + cols[..k].iter().sum::<f32>();
            canvas.draw_text(label, RectF::new(x0 + 10.0, r.y, cols[k] - 20.0, h), &s);
        }
        let line = Stroke::new(ctx.theme.color("border"), 1.0);
        let mut animating = false;
        for (i, q) in self.quotes.iter().enumerate() {
            let y = r.y + h * (i + 1) as f32;
            if y > r.bottom() {
                break;
            }
            if self.selected == Some(i) {
                canvas.fill_rect(RectF::new(r.x, y, r.width, h), 0.0, &Brush::Solid(ctx.theme.color("primary").with_opacity(0.1)));
            }
            // A célula do preço acende e apaga.
            if let Some((at, rising)) = q.flash {
                let age = ctx.time - at;
                if (0.0..FLASH).contains(&age) {
                    let alpha = (1.0 - age / FLASH) as f32 * 0.45;
                    let color = if rising { up(ctx) } else { down(ctx) };
                    canvas.fill_rect(RectF::new(r.x + cols[0], y + 2.0, cols[1], h - 4.0), 4.0, &Brush::Solid(color.with_opacity(alpha)));
                    animating = true;
                }
            }
            let mut sym = t.clone();
            sym.font.weight = FontWeight::Bold;
            sym.h_align = TextAlign::Leading;
            canvas.draw_text(&q.symbol, RectF::new(r.x + 10.0, y, cols[0] - 20.0, h), &sym);
            let mut v = t.clone();
            v.h_align = TextAlign::Trailing;
            let last = if q.last == 0.0 { "—".to_string() } else { format_number(q.last, self.decimals as usize, &locale) };
            canvas.draw_text(&last, RectF::new(r.x + cols[0] + 10.0, y, cols[1] - 20.0, h), &v);
            let mut c = v.clone();
            c.color = if q.change >= 0.0 { up(ctx) } else { down(ctx) };
            canvas.draw_text(&format!("{}{}%", if q.change > 0.0 { "+" } else { "" }, format_number(q.change, 2, &locale)), RectF::new(r.x + cols[0] + cols[1] + 10.0, y, cols[2] - 20.0, h), &c);
            canvas.draw_line(PointF::new(r.x + 8.0, y + h - 0.5), PointF::new(r.right() - 8.0, y + h - 0.5), &line);
        }
        if animating {
            ctx.request_frame();
        }
    }
}

// ===========================================================================
// TimeAndSales

/// Um negócio realizado.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trade {
    pub time: String,
    pub price: f64,
    pub size: f64,
    /// Agressão de compra (verde) ou de venda (vermelho).
    pub buy: bool,
}

/// Times & trades: os negócios do mais novo para o mais velho, verde na compra e vermelho
/// na venda, em negrito a partir de `big_size`. [`TimeAndSales::add`] acrescenta.
pub struct TimeAndSales {
    pub base: ControlBase,
    /// Quantos negócios a lista guarda; os mais velhos saem.
    pub max_rows: u32,
    /// A partir desta quantidade o negócio aparece em negrito.
    pub big_size: f64,
    /// Casas decimais dos preços.
    pub decimals: u32,
    trades: std::collections::VecDeque<Trade>,
}

impl Default for TimeAndSales {
    fn default() -> Self {
        Self { base: ControlBase::new(280.0, 320.0), max_rows: 200, big_size: 5000.0, decimals: 2, trades: Default::default() }
    }
}

impl TimeAndSales {
    pub fn add(ui: &mut Ui, id: ComponentId, trade: Trade) {
        if let Some(t) = ui.get_mut::<TimeAndSales>(id) {
            t.trades.push_front(trade);
            t.trades.truncate(t.max_rows.max(1) as usize);
        }
        ui.request_repaint();
    }

    pub fn trades(&self) -> impl Iterator<Item = &Trade> {
        self.trades.iter()
    }
}

impl Component for TimeAndSales {
    leaf_control!("TimeAndSales");

    properties! {
        max_rows: u32 => "Behavior",
        big_size: f64 => "Appearance",
        decimals: u32 => "Appearance"
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 8.0, &Brush::Solid(ctx.theme.color("surface")));
        let locale = ctx.ui.locale();
        let mut t = ctx.text_style();
        t.font.size = (t.font.size - 2.0).max(9.0);
        let h = t.font.size * 1.9;
        let cols = [r.width * 0.34, r.width * 0.33, r.width * 0.33];
        let head = small(ctx);
        for (k, label) in ["Hora", "Preço", "Qtd."].iter().enumerate() {
            let mut s = head.clone();
            s.h_align = if k == 0 { TextAlign::Leading } else { TextAlign::Trailing };
            let x0 = r.x + cols[..k].iter().sum::<f32>();
            canvas.draw_text(label, RectF::new(x0 + 8.0, r.y, cols[k] - 16.0, h), &s);
        }
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, r.y + h, r.width, r.height - h));
        for (i, tr) in self.trades.iter().enumerate() {
            let y = r.y + h * (i + 1) as f32;
            if y > r.bottom() {
                break;
            }
            let mut s = t.clone();
            s.color = if tr.buy { up(ctx) } else { down(ctx) };
            if tr.size >= self.big_size {
                s.font.weight = FontWeight::Bold;
                canvas.fill_rect(RectF::new(r.x, y, r.width, h), 0.0, &Brush::Solid(s.color.with_opacity(0.1)));
            }
            let mut l = s.clone();
            l.h_align = TextAlign::Leading;
            l.color = ctx.theme.color("text_muted");
            canvas.draw_text(&tr.time, RectF::new(r.x + 8.0, y, cols[0] - 16.0, h), &l);
            s.h_align = TextAlign::Trailing;
            canvas.draw_text(&format_number(tr.price, self.decimals as usize, &locale), RectF::new(r.x + cols[0] + 8.0, y, cols[1] - 16.0, h), &s);
            canvas.draw_text(&format_number(tr.size, 0, &locale), RectF::new(r.x + cols[0] + cols[1] + 8.0, y, cols[2] - 16.0, h), &s);
        }
        canvas.restore();
    }
}
