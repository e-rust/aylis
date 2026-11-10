//! Profiler (Executar → Profiler): grava o app em execução pelo canal de desenvolvimento
//! (só com `dev`) e mostra uma linha do tempo com uma faixa por tipo (handlers com form e
//! método, eventos, layout, pintura, timers, animações, dados, recursos, marcas à mão e a
//! memória) e a tabela de métodos (chamadas, total, próprio, média, máximo). Duplo clique
//! num handler da tabela abre o código dele.
//!
//! O app mede ([`aylis_core::profile`]); a IDE liga (`profile`), busca a cada 250 ms
//! (`profile_take`) e soma aqui.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use aylis_controls::{Button, Form, Layout, Splitter, Timer};
use aylis_core::profile::Lane;
use aylis_core::{Align, Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, PaintCtx, PointF, PointerEvent, RectF, Stroke, TextAlign, Ui, WheelEvent};
use serde_json::{Value, json};

use crate::app::{Ids, refresh};
use crate::state::Shared;
use crate::widgets::{label, muted};

/// Um bloco recebido do app.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub lane: Lane,
    pub name: String,
    pub start: f64,
    pub duration: f64,
    pub depth: u32,
    pub thread: u32,
}

/// O que foi gravado.
#[derive(Default)]
pub struct Recording {
    pub blocks: Vec<Block>,
    /// A memória do app (bytes) no tempo.
    pub memory: Vec<(f64, f64)>,
    /// As imagens guardadas pelos renderizadores na última amostra: quantas e os bytes.
    pub images: Option<(f64, f64)>,
    /// As amostras da pilha da thread da interface (de fora para dentro), para a árvore de
    /// chamadas e o flame graph.
    pub stacks: Vec<Vec<String>>,
    pub dropped: u64,
}

impl Recording {
    /// Acrescenta uma resposta do `profile_take`.
    pub fn add(&mut self, answer: &Value) {
        for s in answer["spans"].as_array().into_iter().flatten() {
            let Some(lane) = s[0].as_str().and_then(Lane::parse) else { continue };
            self.blocks.push(Block {
                lane,
                name: s[1].as_str().unwrap_or_default().to_string(),
                start: s[2].as_f64().unwrap_or(0.0),
                duration: s[3].as_f64().unwrap_or(0.0),
                depth: s[4].as_u64().unwrap_or(0) as u32,
                thread: s[5].as_u64().unwrap_or(0) as u32,
            });
        }
        for s in answer["samples"].as_array().into_iter().flatten() {
            let value = s[2].as_f64().unwrap_or(0.0);
            match s[0].as_str() {
                Some("memória") => self.memory.push((s[1].as_f64().unwrap_or(0.0), value)),
                Some("imagens no cache") => self.images.get_or_insert((0.0, 0.0)).0 = value,
                Some("memória das imagens") => self.images.get_or_insert((0.0, 0.0)).1 = value,
                _ => {}
            }
        }
        for s in answer["stacks"].as_array().into_iter().flatten() {
            let frames: Vec<String> = s[1].as_array().into_iter().flatten().filter_map(|f| f.as_str().map(str::to_string)).collect();
            if !frames.is_empty() {
                self.stacks.push(frames);
            }
        }
        self.dropped += answer["dropped"].as_u64().unwrap_or(0);
    }

    /// O fim do último bloco (ou amostra).
    pub fn end(&self) -> f64 {
        let blocks = self.blocks.iter().map(|b| b.start + b.duration).fold(0.0, f64::max);
        self.memory.last().map_or(blocks, |m| blocks.max(m.0))
    }
}

/// Uma linha da tabela de métodos.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub lane: Lane,
    pub name: String,
    pub calls: u64,
    pub total: f64,
    /// Sem o tempo dos blocos de dentro (um handler que dispara outro).
    pub own: f64,
    pub max: f64,
}

impl Row {
    pub fn average(&self) -> f64 {
        self.total / self.calls.max(1) as f64
    }
}

/// Soma os blocos por faixa e nome. O tempo próprio tira o dos blocos de dentro, na mesma
/// thread (um bloco começa dentro de outro e termina antes dele).
pub fn aggregate(blocks: &[Block]) -> Vec<Row> {
    let mut order: Vec<usize> = (0..blocks.len()).collect();
    order.sort_by(|&a, &b| {
        let (x, y) = (&blocks[a], &blocks[b]);
        x.thread.cmp(&y.thread).then(x.start.total_cmp(&y.start)).then(y.duration.total_cmp(&x.duration))
    });
    let mut own: Vec<f64> = blocks.iter().map(|b| b.duration).collect();
    // A pilha dos blocos abertos (índice e fim).
    let mut open: Vec<(usize, f64)> = Vec::new();
    let mut thread = None;
    for &i in &order {
        let b = &blocks[i];
        if thread != Some(b.thread) {
            open.clear();
            thread = Some(b.thread);
        }
        while open.last().is_some_and(|&(_, end)| end <= b.start + 1e-9) {
            open.pop();
        }
        if let Some(&(parent, _)) = open.last() {
            own[parent] -= b.duration;
        }
        open.push((i, b.start + b.duration));
    }
    let mut rows: HashMap<(Lane, &str), Row> = HashMap::new();
    for (b, own) in blocks.iter().zip(own) {
        // O detalhe depois de " · " (a área de cada quadro) não separa linhas na tabela.
        let name = b.name.split(" · ").next().unwrap_or(&b.name);
        let row = rows.entry((b.lane, name)).or_insert_with(|| Row { lane: b.lane, name: name.to_string(), calls: 0, total: 0.0, own: 0.0, max: 0.0 });
        row.calls += 1;
        row.total += b.duration;
        row.own += own.max(0.0);
        row.max = row.max.max(b.duration);
    }
    let mut out: Vec<Row> = rows.into_values().collect();
    out.sort_by(|a, b| b.total.total_cmp(&a.total).then(a.name.cmp(&b.name)));
    out
}

/// O nome de cada faixa na tela (em português; o catálogo traduz).
fn lane_label(lane: Lane) -> &'static str {
    match lane {
        Lane::Handler => "Handlers",
        Lane::Event => "Eventos",
        Lane::Layout => "Layout",
        Lane::Paint => "Pintura",
        Lane::Timer => "Timers",
        Lane::Animation => "Animações",
        Lane::Data => "Dados",
        Lane::Resource => "Recursos",
        Lane::Mark => "Marcas",
        Lane::Gpu => "GPU",
    }
}

fn lane_color(lane: Lane) -> Color {
    match lane {
        Lane::Handler => Color::rgb(0x3B, 0x82, 0xF6),
        Lane::Event => Color::rgb(0x60, 0xA5, 0xFA),
        Lane::Layout => Color::rgb(0xA8, 0x55, 0xF7),
        Lane::Paint => Color::rgb(0x22, 0xC5, 0x5E),
        Lane::Timer => Color::rgb(0xF5, 0x9E, 0x0B),
        Lane::Animation => Color::rgb(0xEC, 0x48, 0x99),
        Lane::Data => Color::rgb(0x14, 0xB8, 0xA6),
        Lane::Resource => Color::rgb(0x84, 0x7B, 0x6F),
        Lane::Mark => Color::rgb(0xEF, 0x44, 0x44),
        Lane::Gpu => Color::rgb(0x0E, 0xA5, 0xE9),
    }
}

/// `12,3 ms`, `340 µs`, `1,20 s`.
pub fn ms(seconds: f64) -> String {
    aylis_tools::cputime::short_time(std::time::Duration::from_secs_f64(seconds.max(0.0)))
}

const LABELS: f32 = 96.0;
const AXIS: f32 = 20.0;
const LANE: f32 = 26.0;

/// A linha do tempo: uma faixa por tipo e a memória embaixo. Roda: zoom no ponteiro;
/// arrastar: anda no tempo. Gravando, acompanha o fim.
pub struct Timeline {
    pub base: ControlBase,
    data: Rc<RefCell<Recording>>,
    /// O começo e o tamanho (s) do que aparece; `None`: tudo (ou o fim, gravando).
    view: Cell<Option<(f64, f64)>>,
    drag: Option<(f32, f64)>,
    hover: RefCell<String>,
}

impl Timeline {
    pub fn new(data: Rc<RefCell<Recording>>) -> Self {
        Self { base: ControlBase::new(800.0, 300.0), data, view: Cell::new(None), drag: None, hover: RefCell::default() }
    }

    /// O trecho à vista: o escolhido, ou tudo o que foi gravado.
    fn window(&self) -> (f64, f64) {
        self.view.get().unwrap_or_else(|| (0.0, self.data.borrow().end().max(0.5)))
    }

    fn time_at(&self, x: f32, width: f32) -> f64 {
        let (start, len) = self.window();
        start + ((x - LABELS) / (width - LABELS).max(1.0)) as f64 * len
    }

    /// Volta a mostrar tudo.
    pub fn show_all(&mut self) {
        self.view.set(None);
    }
}

impl Component for Timeline {
    fn class_name(&self) -> &'static str {
        "ProfileTimeline"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let width = ctx.ui.control(ctx.id).map_or(800.0, |c| c.width());
        let (start, len) = self.window();
        let at = self.time_at(event.position.x, width);
        // A roda para cima (delta positivo) aproxima, como nos mapas e no designer; estava ao
        // contrário (relatado pelo autor em 08/11).
        let factor = if event.delta.y > 0.0 { 0.8 } else { 1.25 };
        let len2 = (len * factor).clamp(0.000_05, self.data.borrow().end().max(0.5) * 2.0);
        let start2 = (at - (at - start) * len2 / len).max(0.0);
        self.view.set(Some((start2, len2)));
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        if event.click_count >= 2 {
            self.view.set(None);
        } else {
            self.drag = Some((event.position.x, self.window().0));
        }
        ctx.repaint();
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let width = ctx.ui.control(ctx.id).map_or(800.0, |c| c.width());
        if let Some((x0, start0)) = self.drag {
            let (_, len) = self.window();
            let dt = (x0 - event.position.x) as f64 / (width - LABELS).max(1.0) as f64 * len;
            self.view.set(Some(((start0 + dt).max(0.0), len)));
        }
        // O bloco sob o ponteiro (a dica embaixo).
        let lane = ((event.position.y - AXIS) / LANE).floor();
        let t = self.time_at(event.position.x, width);
        let hover = Lane::ALL.get(lane.max(0.0) as usize).filter(|_| lane >= 0.0).and_then(|&lane| {
            let data = self.data.borrow();
            data.blocks.iter().filter(|b| b.lane == lane && b.start <= t && t <= b.start + b.duration).max_by_key(|b| b.depth).map(|b| format!("{}  —  {}", b.name, ms(b.duration)))
        });
        *self.hover.borrow_mut() = hover.unwrap_or_default();
        ctx.repaint();
    }

    fn pointer_up(&mut self, ctx: &mut EventCtx<'_>, _: &PointerEvent) {
        self.drag = None;
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface")));
        let data = self.data.borrow();
        let (start, len) = self.window();
        let plot = RectF::new(r.x + LABELS, r.y + AXIS, (r.width - LABELS).max(1.0), r.height - AXIS);
        let x_of = |t: f64| plot.x + ((t - start) / len) as f32 * plot.width;
        let mut text = ctx.text_style();
        text.font.size = (text.font.size - 2.0).max(9.0);
        text.v_align = TextAlign::Center;
        let mut muted_text = text.clone();
        muted_text.color = theme.color("text_muted");
        // A régua do tempo: ~8 marcas em números redondos.
        let step = {
            let raw = len / 8.0;
            let p = 10f64.powf(raw.log10().floor());
            [1.0, 2.0, 5.0, 10.0].into_iter().map(|m| m * p).find(|s| *s >= raw).unwrap_or(raw)
        };
        let grid = Stroke::new(theme.color("border").with_opacity(0.5), 1.0);
        let mut t = (start / step).ceil() * step;
        while t <= start + len {
            let x = x_of(t);
            canvas.draw_line(PointF::new(x, r.y + AXIS - 4.0), PointF::new(x, r.bottom()), &grid);
            canvas.draw_text(&ms(t), RectF::new(x + 3.0, r.y, 90.0, AXIS), &muted_text);
            t += step;
        }
        // As faixas.
        let lanes = Lane::ALL.len();
        for (i, &lane) in Lane::ALL.iter().enumerate() {
            let y = plot.y + i as f32 * LANE;
            if i % 2 == 1 {
                canvas.fill_rect(RectF::new(r.x, y, r.width, LANE), 0.0, &Brush::Solid(theme.color("text").with_opacity(0.03)));
            }
            canvas.draw_text(&ctx.ui.tr(lane_label(lane)), RectF::new(r.x + 8.0, y, LABELS - 10.0, LANE), &text);
            let color = lane_color(lane);
            canvas.save();
            canvas.clip_rect(RectF::new(plot.x, y, plot.width, LANE));
            for b in data.blocks.iter().filter(|b| b.lane == lane && b.start + b.duration >= start && b.start <= start + len) {
                let x = x_of(b.start);
                let w = (b.duration / len) as f32 * plot.width;
                let depth = b.depth.min(3) as f32;
                let block = RectF::new(x, y + 3.0 + depth * 4.0, w.max(1.0), LANE - 6.0 - depth * 4.0);
                canvas.fill_rect(block, 2.0, &Brush::Solid(color.with_opacity(0.85 - depth * 0.12)));
                if w > 40.0 {
                    let mut on = text.clone();
                    on.color = Color::WHITE;
                    canvas.draw_text(&b.name, block.inflate(-3.0), &on);
                }
            }
            canvas.restore();
        }
        // A memória: uma linha entre o menor e o maior valor.
        let y = plot.y + lanes as f32 * LANE;
        let memory_lane = RectF::new(plot.x, y + 3.0, plot.width, (r.bottom() - y - 26.0).clamp(LANE, 80.0));
        canvas.draw_text(&ctx.ui.tr("Memória"), RectF::new(r.x + 8.0, y, LABELS - 10.0, LANE), &text);
        if data.memory.len() >= 2 {
            let (lo, hi) = data.memory.iter().fold((f64::MAX, f64::MIN), |(lo, hi), m| (lo.min(m.1), hi.max(m.1)));
            let span = (hi - lo).max(1.0);
            let stroke = Stroke::new(theme.color("primary"), 1.5);
            let point = |m: &(f64, f64)| PointF::new(x_of(m.0), memory_lane.bottom() - ((m.1 - lo) / span) as f32 * memory_lane.height);
            canvas.save();
            canvas.clip_rect(memory_lane);
            for pair in data.memory.windows(2) {
                canvas.draw_line(point(&pair[0]), point(&pair[1]), &stroke);
            }
            canvas.restore();
            let last = data.memory.last().map_or(0.0, |m| m.1);
            canvas.draw_text(&format!("{:.1} MB", last / 1_048_576.0).replace('.', ","), RectF::new(r.x + 8.0, y + 16.0, LABELS - 10.0, LANE), &muted_text);
        }
        if let Some((count, bytes)) = data.images {
            let text = ctx.ui.tr(&format!("{} imagens · {}", count as usize, aylis_core::memory_panel::format_bytes(bytes as usize)));
            canvas.draw_text(&text, RectF::new(r.x + 8.0, y + 32.0, LABELS - 10.0, LANE), &muted_text);
        }
        // A dica do bloco sob o ponteiro, ou como usar.
        let hint = self.hover.borrow();
        let tip = if hint.is_empty() { ctx.ui.tr("Roda: zoom · arrastar: andar no tempo · duplo clique: tudo") } else { hint.clone() };
        canvas.draw_text(&tip, RectF::new(plot.x, r.bottom() - 22.0, plot.width, 20.0), &muted_text);
        if data.dropped > 0 {
            let mut warn = muted_text.clone();
            warn.color = theme.color("danger");
            warn.h_align = TextAlign::Trailing;
            canvas.draw_text(&ctx.ui.tr(&format!("{} blocos perdidos (o app guardou demais)", data.dropped)), RectF::new(plot.x, r.bottom() - 22.0, plot.width - 8.0, 20.0), &warn);
        }
    }
}

/// A tabela de métodos. Clique no título ordena; duplo clique numa linha dispara
/// `on_open` com o nome (o form e o handler).
pub struct MethodTable {
    pub base: ControlBase,
    data: Rc<RefCell<Recording>>,
    sort: Cell<usize>,
    scroll: Cell<f32>,
    /// As linhas da última pintura (para o clique).
    shown: RefCell<Vec<Row>>,
}

const COLUMNS: [&str; 7] = ["Faixa", "Nome", "Chamadas", "Total", "Próprio", "Média", "Máximo"];
const ROW: f32 = 22.0;

impl MethodTable {
    pub fn new(data: Rc<RefCell<Recording>>) -> Self {
        Self { base: ControlBase::new(800.0, 220.0), data, sort: Cell::new(3), scroll: Cell::new(0.0), shown: RefCell::default() }
    }

    fn widths(width: f32) -> [f32; 7] {
        let numbers = 86.0;
        [90.0, (width - 90.0 - numbers * 5.0).max(120.0), numbers, numbers, numbers, numbers, numbers]
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows = aggregate(&self.data.borrow().blocks);
        match self.sort.get() {
            0 => rows.sort_by(|a, b| a.lane.cmp(&b.lane).then(b.total.total_cmp(&a.total))),
            1 => rows.sort_by(|a, b| a.name.cmp(&b.name)),
            2 => rows.sort_by(|a, b| b.calls.cmp(&a.calls)),
            4 => rows.sort_by(|a, b| b.own.total_cmp(&a.own)),
            5 => rows.sort_by(|a, b| b.average().total_cmp(&a.average())),
            6 => rows.sort_by(|a, b| b.max.total_cmp(&a.max)),
            _ => {}
        }
        rows
    }
}

impl Component for MethodTable {
    fn class_name(&self) -> &'static str {
        "ProfileMethods"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        let rows = self.shown.borrow().len() as f32;
        let height = ctx.ui.control(ctx.id).map_or(200.0, |c| c.height());
        let max = (rows * ROW - (height - ROW)).max(0.0);
        // Como as outras listas: a roda para cima sobe (o delta positivo é para cima).
        self.scroll.set((self.scroll.get() - event.delta.y).clamp(0.0, max));
        ctx.repaint();
    }

    fn pointer_down(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let p = event.position;
        let width = ctx.ui.control(ctx.id).map_or(800.0, |c| c.width());
        if p.y < ROW {
            // O título: ordena pela coluna.
            let mut x = 0.0;
            for (i, w) in Self::widths(width).iter().enumerate() {
                if p.x >= x && p.x < x + w {
                    self.sort.set(i);
                }
                x += w;
            }
            ctx.repaint();
            return;
        }
        if event.click_count >= 2 {
            let index = ((p.y - ROW + self.scroll.get()) / ROW).floor() as usize;
            if let Some(row) = self.shown.borrow().get(index) {
                ctx.fire("on_open", EventArgs::Text(row.name.clone()));
            }
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let theme = ctx.theme;
        canvas.fill_rect(r, 0.0, &Brush::Solid(theme.color("surface")));
        let rows = self.rows();
        let widths = Self::widths(r.width);
        let mut text = ctx.text_style();
        text.font.size = (text.font.size - 1.0).max(9.0);
        text.v_align = TextAlign::Center;
        let mut head = text.clone();
        head.font.weight = aylis_core::FontWeight::Bold;
        canvas.fill_rect(RectF::new(r.x, r.y, r.width, ROW), 0.0, &Brush::Solid(theme.color("text").with_opacity(0.06)));
        let mut x = r.x;
        for (i, (title, w)) in COLUMNS.iter().zip(widths).enumerate() {
            let mut h = head.clone();
            h.h_align = if i >= 2 { TextAlign::Trailing } else { TextAlign::Leading };
            let mark = if self.sort.get() == i { " ▾" } else { "" };
            canvas.draw_text(&format!("{}{mark}", ctx.ui.tr(title)), RectF::new(x + 6.0, r.y, w - 12.0, ROW), &h);
            x += w;
        }
        canvas.save();
        canvas.clip_rect(RectF::new(r.x, r.y + ROW, r.width, r.height - ROW));
        for (n, row) in rows.iter().enumerate() {
            let y = r.y + ROW + n as f32 * ROW - self.scroll.get();
            if y + ROW < r.y + ROW || y > r.bottom() {
                continue;
            }
            if n % 2 == 1 {
                canvas.fill_rect(RectF::new(r.x, y, r.width, ROW), 0.0, &Brush::Solid(theme.color("text").with_opacity(0.03)));
            }
            let cells = [ctx.ui.tr(lane_label(row.lane)), row.name.clone(), row.calls.to_string(), ms(row.total), ms(row.own), ms(row.average()), ms(row.max)];
            let mut x = r.x;
            for (i, (cell, w)) in cells.iter().zip(widths).enumerate() {
                let mut t = text.clone();
                t.h_align = if i >= 2 { TextAlign::Trailing } else { TextAlign::Leading };
                if i == 0 {
                    t.color = lane_color(row.lane);
                }
                canvas.draw_text(cell, RectF::new(x + 6.0, y, w - 12.0, ROW), &t);
                x += w;
            }
        }
        canvas.restore();
        if rows.is_empty() {
            let mut t = text.clone();
            t.color = theme.color("text_muted");
            t.h_align = TextAlign::Center;
            canvas.draw_text(&ctx.ui.tr("Grave para ver os métodos (o app precisa estar rodando pelo F9)."), RectF::new(r.x, r.y + ROW, r.width, ROW * 2.0), &t);
        }
        *self.shown.borrow_mut() = rows;
    }
}

/// A janela do profiler.
struct View {
    /// O canal com o app em execução (`None`: ainda não abriu; o Executar da janela abre).
    link: Option<Arc<crate::devlink::DevServer>>,
    data: Rc<RefCell<Recording>>,
    recording: bool,
    status: ComponentId,
    record: ComponentId,
}

/// Abre o profiler. Sem o app em execução, a janela abre do mesmo jeito, com o aviso e o
/// botão que executa o projeto, e se liga sozinha quando o app abrir (antes só a linha de
/// situação dizia, e parecia que o menu não fazia nada; relatado pelo autor em 07/11).
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let link = state.borrow().dev.clone().filter(|d| d.connected());
    let form = ui.add_root(Form::new("Profiler", 1100.0, 720.0));
    let _ = ui.set_name(form, "frmProfiler");
    ui.control_mut(form).unwrap().set_padding(12.0);
    let top = ui.add(Layout::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let record = ui.add(Button::new("● Gravar"), top);
    ui.control_mut(record).unwrap().set_align(Align::Left).set_width(120.0);
    let clear = ui.add(Button::new("Limpar"), top);
    ui.control_mut(clear).unwrap().set_align(Align::Left).set_width(90.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let all = ui.add(Button::new("Ver tudo"), top);
    ui.control_mut(all).unwrap().set_align(Align::Left).set_width(90.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let close_b = ui.add(Button::new("Fechar"), top);
    ui.control_mut(close_b).unwrap().set_align(Align::Right).set_width(90.0).set_style("flat");
    let _ = ui.set_prop(close_b, "modal_result", "ok");
    let status = label(ui, top, "", Align::Client, 0.0);
    ui.control_mut(status).unwrap().set_margins([12.0, 0.0, 0.0, 0.0]);
    muted(ui, status);

    let data = Rc::new(RefCell::new(Recording::default()));
    // Embaixo: a tabela de métodos e as chamadas pelas amostras da pilha (flame graph).
    let bottom = ui.add(aylis_controls::TabControl::default(), form);
    ui.control_mut(bottom).unwrap().set_align(Align::Bottom).set_height(280.0);
    let methods = aylis_controls::TabControl::add_tab(ui, bottom, "Métodos");
    let table = ui.add(MethodTable::new(data.clone()), methods);
    ui.control_mut(table).unwrap().set_align(Align::Client);
    let calls = aylis_controls::TabControl::add_tab(ui, bottom, "Chamadas (amostras)");
    let flame = ui.add(FlameGraph::new(data.clone()), calls);
    ui.control_mut(flame).unwrap().set_align(Align::Client);
    ui.add(Splitter::new(Align::Bottom), form);
    let timeline = ui.add(Timeline::new(data.clone()), form);
    ui.control_mut(timeline).unwrap().set_align(Align::Client);
    let timer = ui.add(Timer::new(250), form);
    let _ = ui.set_prop(timer, "enabled", false);

    // Esperando o app abrir (só enquanto espera: o canal avisa pelo estado da IDE).
    let waiting = ui.add(Timer::new(500), form);
    let _ = ui.set_prop(waiting, "enabled", false);
    if link.is_none() {
        let _ = ui.set_prop(record, "text", "▶ Executar");
        let _ = ui.set_prop(status, "text", "O app não está em execução: Executar abre o projeto e o Profiler se liga a ele");
    }
    let view = Rc::new(RefCell::new(View { link, data, recording: false, status, record }));
    {
        let (view, state) = (view.clone(), state.clone());
        ui.on(waiting, "on_timer", move |ui, _, _| {
            let Some(link) = state.borrow().dev.clone().filter(|d| d.connected()) else { return };
            view.borrow_mut().link = Some(link);
            let _ = ui.set_prop(waiting, "enabled", false);
            let (record, status) = (view.borrow().record, view.borrow().status);
            let _ = ui.set_prop(record, "text", "● Gravar");
            let _ = ui.set_prop(status, "text", "O app abriu: Gravar começa a medir");
        });
    }
    {
        let (view, state) = (view.clone(), state.clone());
        ui.on_click(record, move |ui, _| {
            let Some(link) = view.borrow().link.clone() else {
                // Sem o app: executa o projeto e espera ele se ligar.
                let status = view.borrow().status;
                if state.borrow().project.is_none() {
                    let _ = ui.set_prop(status, "text", "Abra um projeto para medir");
                    return;
                }
                crate::actions::run_project(ui, &state);
                let _ = ui.set_prop(status, "text", "Abrindo o app…");
                let _ = ui.set_prop(waiting, "enabled", true);
                return;
            };
            let on = !view.borrow().recording;
            let result = link.call(json!({ "cmd": "profile", "on": on }));
            if let Err(e) = result {
                let status = view.borrow().status;
                let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
                return;
            }
            if on {
                // Gravar de novo começa do zero (o relógio do app recomeça).
                *view.borrow().data.borrow_mut() = Recording::default();
            } else {
                take(ui, &view);
            }
            view.borrow_mut().recording = on;
            let _ = ui.set_prop(timer, "enabled", on);
            let record = view.borrow().record;
            let _ = ui.set_prop(record, "text", if on { "■ Parar" } else { "● Gravar" });
            show(ui, &view);
        });
    }
    {
        let view = view.clone();
        ui.on(timer, "on_timer", move |ui, _, _| take(ui, &view));
    }
    {
        let view = view.clone();
        ui.on_click(clear, move |ui, _| {
            *view.borrow().data.borrow_mut() = Recording::default();
            show(ui, &view);
        });
    }
    ui.on_click(all, move |ui, _| {
        if let Some(t) = ui.get_mut::<Timeline>(timeline) {
            t.show_all();
        }
        ui.request_repaint();
    });
    {
        let (state, view) = (state.clone(), view.clone());
        ui.on(table, "on_open", move |ui, _, args| {
            let EventArgs::Text(name) = args else { return };
            let status = view.borrow().status;
            let text = match open_handler(ui, &state, name) {
                Ok(()) => String::new(),
                Err(e) => format!("⚠ {e}"),
            };
            let _ = ui.set_prop(status, "text", text);
        });
    }
    let (state2, ids2, view2) = (state.clone(), *ids, view.clone());
    crate::lang::translate(ui);
    let _ = ui.show_modal(form, move |ui, _| {
        // Fechar a janela desliga a gravação no app.
        if let Some(link) = view2.borrow().link.clone() {
            let _ = link.call(json!({ "cmd": "profile", "on": false }));
        }
        ui.free(form);
        refresh(ui, &state2, &ids2);
    });
    show(ui, &view);
}

/// Busca o que o app mediu desde a última vez.
fn take(ui: &mut Ui, view: &Rc<RefCell<View>>) {
    let Some(link) = view.borrow().link.clone() else { return };
    let answer = link.call(json!({ "cmd": "profile_take" }));
    match answer {
        Ok(answer) => view.borrow().data.borrow_mut().add(&answer),
        Err(e) => {
            let status = view.borrow().status;
            let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
            return;
        }
    }
    show(ui, view);
}

fn show(ui: &mut Ui, view: &Rc<RefCell<View>>) {
    let v = view.borrow();
    let (blocks, end) = {
        let d = v.data.borrow();
        (d.blocks.len(), d.end())
    };
    let text = if v.recording { format!("Gravando… {} · {blocks} blocos", ms(end)) } else if blocks > 0 { format!("{} gravados · {blocks} blocos", ms(end)) } else { String::new() };
    let _ = ui.set_prop(v.status, "text", text);
    drop(v);
    ui.request_repaint();
}

/// Abre o código de um handler (`frmClientes.btn_salvar_click`) no projeto aberto.
fn open_handler(ui: &mut Ui, state: &Shared, name: &str) -> Result<(), String> {
    let (form, handler) = name.split_once('.').ok_or("só os handlers (form.método) levam ao código")?;
    let project = state.borrow().project.clone().ok_or("nenhum projeto aberto")?;
    let (file, line) = aylis_project::sync::handler_location(&project, project.language, form, handler).ok_or_else(|| format!("não achei {handler} no código de {form}"))?;
    crate::codewindow::open(ui, &file, line).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(lane: Lane, name: &str, start: f64, duration: f64) -> Block {
        Block { lane, name: name.into(), start, duration, depth: 0, thread: 0 }
    }

    #[test]
    fn own_time_leaves_out_the_blocks_inside() {
        let blocks = vec![
            block(Lane::Handler, "frm.salvar", 0.0, 0.010),
            block(Lane::Mark, "consulta", 0.002, 0.006),
            block(Lane::Handler, "frm.salvar", 0.020, 0.004),
            block(Lane::Paint, "pintura frm", 0.030, 0.002),
        ];
        let rows = aggregate(&blocks);
        let salvar = rows.iter().find(|r| r.name == "frm.salvar").unwrap();
        assert_eq!(salvar.calls, 2);
        assert!((salvar.total - 0.014).abs() < 1e-9);
        assert!((salvar.own - 0.008).abs() < 1e-9, "{}", salvar.own);
        assert!((salvar.max - 0.010).abs() < 1e-9);
        assert!((salvar.average() - 0.007).abs() < 1e-9);
        assert_eq!(rows[0].name, "frm.salvar", "o maior total primeiro");
    }

    #[test]
    fn reads_what_the_app_sends() {
        let mut rec = Recording::default();
        rec.add(&json!({ "spans": [["handler", "frm.ok", 0.5, 0.25, 0, 0], ["nada", "x", 0, 0, 0, 0]], "samples": [["memória", 0.75, 1048576.0], ["imagens no cache", 0.75, 12.0], ["memória das imagens", 0.75, 2097152.0]], "dropped": 3 }));
        assert_eq!(rec.blocks, vec![Block { lane: Lane::Handler, name: "frm.ok".into(), start: 0.5, duration: 0.25, depth: 0, thread: 0 }]);
        assert_eq!(rec.memory, vec![(0.75, 1048576.0)]);
        assert_eq!(rec.images, Some((12.0, 2097152.0)), "as imagens do cache, na última amostra");
        assert_eq!(rec.dropped, 3);
        assert_eq!(rec.end(), 0.75);
    }
}

/// Um nó da árvore de chamadas: a função, em quantas amostras ela aparece (`total`) e em
/// quantas era ela que rodava (`own`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CallNode {
    pub name: String,
    pub total: u32,
    pub own: u32,
    pub children: Vec<CallNode>,
}

/// A árvore de chamadas das amostras: cada pilha a partir do `main` do app (o que vem antes
/// é do sistema), os filhos pelo total, do maior ao menor.
pub fn call_tree(stacks: &[Vec<String>]) -> CallNode {
    let mut root = CallNode { name: "todas".into(), ..CallNode::default() };
    for frames in stacks {
        let start = frames.iter().position(|f| f == "main" || f.ends_with("::main")).unwrap_or(0);
        root.total += 1;
        let mut node = &mut root;
        for name in &frames[start..] {
            let i = match node.children.iter().position(|c| &c.name == name) {
                Some(i) => i,
                None => {
                    node.children.push(CallNode { name: name.clone(), ..CallNode::default() });
                    node.children.len() - 1
                }
            };
            node = &mut node.children[i];
            node.total += 1;
        }
        node.own += 1;
    }
    fn sort(node: &mut CallNode) {
        node.children.sort_by(|a, b| b.total.cmp(&a.total));
        node.children.iter_mut().for_each(sort);
    }
    sort(&mut root);
    root
}

/// O flame graph das amostras da pilha: a raiz em cima, cada função com a largura do tempo
/// em que aparece (o mais largo, onde o app passa mais tempo). O ponteiro mostra o nome, as
/// amostras e a porcentagem.
pub struct FlameGraph {
    pub base: ControlBase,
    data: Rc<RefCell<Recording>>,
    hover: RefCell<String>,
    pointer: Cell<Option<PointF>>,
}

/// A altura de uma linha do flame graph.
const FLAME_ROW: f32 = 20.0;

impl FlameGraph {
    pub fn new(data: Rc<RefCell<Recording>>) -> Self {
        Self { base: ControlBase::new(800.0, 300.0), data, hover: RefCell::default(), pointer: Cell::new(None) }
    }

    /// A cor de uma função (fixa pelo nome, em tons quentes).
    fn color(name: &str) -> Color {
        let h = name.bytes().fold(2166136261u32, |h, b| (h ^ u32::from(b)).wrapping_mul(16777619));
        Color::rgb(200 + (h % 55) as u8, 90 + ((h >> 8) % 110) as u8, 40 + ((h >> 16) % 40) as u8)
    }
}

impl Component for FlameGraph {
    fn class_name(&self) -> &'static str {
        "IdeFlameGraph"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn pointer_move(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        self.pointer.set(Some(event.position));
        ctx.repaint();
    }

    fn pointer_leave(&mut self, ctx: &mut EventCtx<'_>) {
        self.pointer.set(None);
        ctx.repaint();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.theme.color("surface")));
        let tree = call_tree(&self.data.borrow().stacks);
        let mut text = ctx.text_style();
        if tree.total == 0 {
            text.color = ctx.theme.color("text_muted");
            text.h_align = TextAlign::Center;
            // O dicionário do app (o da IDE), não só as mensagens do Aylis.
            canvas.draw_text(&aylis_core::i18n::tr("Grave para ver onde o app passa o tempo (amostras da pilha; hoje, no Windows)"), r, &text);
            return;
        }
        text.font.size = 11.0;
        text.color = Color::rgb(0x1A, 0x14, 0x10);
        let pointer = self.pointer.get();
        let mut hover = String::new();
        // Cada nó na faixa [x, x + largura) da linha `depth`.
        fn draw(node: &CallNode, x: f32, width: f32, depth: usize, top: f32, total: f32, canvas: &mut dyn Canvas, text: &aylis_core::TextStyle, pointer: Option<PointF>, hover: &mut String) {
            let rect = RectF::new(x, top + depth as f32 * FLAME_ROW, width, FLAME_ROW - 1.0);
            if width >= 1.0 {
                canvas.fill_rect(rect, 2.0, &Brush::Solid(FlameGraph::color(&node.name)));
                if width > 30.0 {
                    canvas.draw_text(&node.name, rect.inflate(-3.0), text);
                }
                if pointer.is_some_and(|p| rect.contains(p)) {
                    *hover = format!("{} — {} amostras ({:.1}%), {} nela mesma", node.name, node.total, node.total as f32 * 100.0 / total, node.own);
                }
            }
            let mut cx = x;
            for child in &node.children {
                let w = width * child.total as f32 / node.total.max(1) as f32;
                draw(child, cx, w, depth + 1, top, total, canvas, text, pointer, hover);
                cx += w;
            }
        }
        let area = RectF::new(r.x, r.y + 22.0, r.width, r.height - 22.0);
        canvas.save();
        canvas.clip_rect(area);
        draw(&tree, area.x, area.width, 0, area.y, tree.total as f32, canvas, &text, pointer, &mut hover);
        canvas.restore();
        let mut info = ctx.text_style();
        info.color = ctx.theme.color("text_muted");
        let line = if hover.is_empty() { format!("{} amostras da pilha (a cada 2 ms)", tree.total) } else { hover.clone() };
        canvas.draw_text(&line, RectF::new(r.x + 6.0, r.y, r.width - 12.0, 22.0), &info);
        *self.hover.borrow_mut() = hover;
    }
}

#[cfg(test)]
mod call_tree_tests {
    use super::*;

    #[test]
    fn the_call_tree_starts_at_main_and_counts_each_function() {
        let s = |f: &[&str]| f.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let stacks = vec![
            s(&["RtlUserThreadStart", "app::main", "aylis::run", "app::salvar"]),
            s(&["RtlUserThreadStart", "app::main", "aylis::run", "app::salvar", "consulta"]),
            s(&["RtlUserThreadStart", "app::main", "aylis::run", "pintar"]),
        ];
        let tree = call_tree(&stacks);
        assert_eq!(tree.total, 3);
        let main = &tree.children[0];
        assert_eq!((main.name.as_str(), main.total), ("app::main", 3), "o que vem antes do main fica de fora");
        let run = &main.children[0];
        assert_eq!((run.children[0].name.as_str(), run.children[0].total, run.children[0].own), ("app::salvar", 2, 1), "o maior primeiro");
        assert_eq!(run.children[1].name, "pintar");
    }
}
