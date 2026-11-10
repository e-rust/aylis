//! Uma tela gigante (um videowall: 19.200 × 6.480 pixels, 10 × 6 monitores de 1920 × 1080)
//! cheia de componentes, sem janela: o Skia na CPU pinta num buffer do tamanho da tela. Mede
//! montar, o layout, a pintura inteira e as situações do dia a dia pintando só o que mudou
//! (digitar, um relógio, barras de progresso, o mouse, um painel deslizando, rolar um Memo).
//!
//! ```text
//! cargo run --release -p aylis-controls --example giant_screen [largura] [altura]
//! ```
//!
//! O teste `giant_screen` roda o mesmo e compara com a base gravada em
//! `medicoes/giant_screen.json` (não aceita regressão).

use std::time::Instant;

use aylis_core::*;

/// O tamanho da tela gigante.
pub const WIDTH: u32 = 19_200;
pub const HEIGHT: u32 = 6_480;

/// Uma célula da grade: um componente de cada vez, em ciclo pelas classes.
const CELL: (f32, f32) = (160.0, 40.0);
const CLASSES: [&str; 8] = ["Label", "Edit", "Button", "CheckBox", "ProgressBar", "Switch", "ComboBox", "TrackBar"];

pub struct Giant {
    pub ui: Ui,
    pub form: ComponentId,
    edit: ComponentId,
    clock: ComponentId,
    bars: Vec<ComponentId>,
    button: ComponentId,
    slide: ComponentId,
    memo: ComponentId,
    pub controls: usize,
    pub width: u32,
    pub height: u32,
}

/// O resultado de um cenário: o tempo do evento e do quadro (a mediana, em ms), a fração da
/// tela repintada e quantos controles foram pintados por quadro (a mediana). A área e os
/// controles não dependem da máquina.
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: &'static str,
    pub event_ms: f64,
    pub frame_ms: f64,
    pub area: f64,
    pub painted: usize,
}

/// A medição inteira.
#[derive(Clone, Debug)]
pub struct Report {
    pub controls: usize,
    pub build_ms: f64,
    pub layout_ms: f64,
    pub full_ms: f64,
    pub full_painted: usize,
    pub scenarios: Vec<Scenario>,
}

pub fn build(width: u32, height: u32) -> Giant {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", width as f64).unwrap();
    ui.set_prop(form, "height", height as f64).unwrap();
    let set = |ui: &mut Ui, id: ComponentId, props: &[(&str, PropValue)]| {
        for (name, value) in props {
            let _ = ui.set_prop(id, name, value.clone());
        }
    };
    // A grade: uma linha (um Panel) por faixa de 40 px, e os componentes nela.
    let (cols, rows) = ((width as f32 / CELL.0) as usize, ((height as f32 - 600.0) / CELL.1) as usize);
    let (mut edit, mut button, mut bars) = (None, None, Vec::new());
    for r in 0..rows {
        let row = ui.create("Panel", Some(form)).unwrap();
        set(&mut ui, row, &[("align", "top".into()), ("height", (CELL.1 as f64).into())]);
        for c in 0..cols {
            let class = CLASSES[(r + c) % CLASSES.len()];
            let id = ui.create(class, Some(row)).unwrap();
            set(&mut ui, id, &[("align", "left".into()), ("width", (CELL.0 as f64 - 4.0).into()), ("text", format!("{r}.{c}").into())]);
            match class {
                "Edit" if edit.is_none() => edit = Some(id),
                "Button" if button.is_none() => button = Some(id),
                // Uma barra de progresso andando em cada monitor (60 telas de 1920 × 1080).
                "ProgressBar" if r % 27 == 3 && c % 12 == 5 => bars.push(id),
                _ => {}
            }
        }
    }
    // Embaixo: a barra de status com o relógio, e um Memo comprido.
    let status = ui.create("Panel", Some(form)).unwrap();
    set(&mut ui, status, &[("align", "bottom".into()), ("height", 28.0.into())]);
    let clock = ui.create("Label", Some(status)).unwrap();
    set(&mut ui, clock, &[("align", "right".into()), ("width", 90.0.into()), ("text", "12:00:00".into())]);
    let memo = ui.create("Memo", Some(form)).unwrap();
    let text: Vec<String> = (0..20_000).map(|i| format!("Linha {i} do registro de eventos do painel")).collect();
    set(&mut ui, memo, &[("align", "client".into()), ("text", text.join("\n").into())]);
    // Um painel com 30 rótulos que desliza (com o desenho guardado).
    let slide = ui.create("Panel", Some(form)).unwrap();
    set(&mut ui, slide, &[("align", "none".into()), ("left", 2000.0.into()), ("top", 1000.0.into()), ("width", 600.0.into()), ("height", 600.0.into()), ("cache_layer", true.into())]);
    for i in 0..30 {
        let label = ui.create("Label", Some(slide)).unwrap();
        set(&mut ui, label, &[("align", "top".into()), ("height", 20.0.into()), ("text", format!("Item {i} do painel").into())]);
    }
    ui.show(form).unwrap();
    ui.layout();
    let controls = ui.components().count();
    Giant { ui, form, edit: edit.unwrap(), clock, bars, button: button.unwrap(), slide, memo, controls, width, height }
}

/// Os cenários: (nome, o que muda antes de cada quadro).
type Change = fn(&mut Giant, usize);

fn scenarios() -> [(&'static str, Change); 6] {
    [
        ("digitar", |g, i| {
            if i == 0 {
                g.ui.set_focus(g.edit);
            }
            g.ui.text_input(g.form, if i % 2 == 0 { "a" } else { "b" });
        }),
        ("relogio", |g, i| {
            let _ = g.ui.set_prop(g.clock, "text", format!("12:00:{:02}", i % 60));
        }),
        ("60 barras", |g, i| {
            for b in g.bars.clone() {
                let _ = g.ui.set_prop(b, "value", (i % 100) as f64);
            }
        }),
        ("mouse", |g, i| {
            let r = g.ui.absolute_rect(g.button).unwrap_or_default();
            let at = if i % 2 == 0 { PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0) } else { PointF::new(r.right() + 2.0, r.y + 2.0) };
            g.ui.pointer_move(g.form, PointerEvent { position: at, ..Default::default() });
            g.ui.set_time(g.ui.time() + 1.0);
        }),
        ("deslizar", |g, i| {
            let _ = g.ui.set_prop(g.slide, "translate_x", ((i % 20) * 8) as f64);
        }),
        ("rolar", |g, i| {
            let r = g.ui.absolute_rect(g.memo).unwrap_or_default();
            let at = PointF::new(r.x + 200.0, r.y + r.height / 2.0);
            let delta = if (i / 20) % 2 == 0 { -40.0 } else { 40.0 };
            g.ui.wheel(g.form, WheelEvent { position: at, delta: PointF::new(0.0, delta), ..Default::default() });
        }),
    ]
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.total_cmp(b));
    values.get(values.len() / 2).copied().unwrap_or(0.0)
}

/// Monta a tela e mede: a pintura inteira (`full` vezes) e cada cenário (`frames` quadros).
pub fn measure(width: u32, height: u32, full: usize, frames: usize) -> Report {
    let started = Instant::now();
    let mut g = build(width, height);
    let build_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    g.ui.invalidate();
    g.ui.layout();
    let layout_ms = started.elapsed().as_secs_f64() * 1000.0;
    let renderer = aylis_skia::SkiaRenderer::new();
    let (w, h) = (g.width, g.height);
    let mut pixels = vec![0u32; w as usize * h as usize];
    // Um quadro como a plataforma faz: as cópias e as áreas (ou tudo) por cima do anterior.
    let mut frame = |g: &mut Giant, partial: bool| {
        let started = Instant::now();
        let damage = g.ui.take_frame(g.form).snapped(1.0);
        let (ui, form) = (&mut g.ui, g.form);
        let fraction = match damage {
            FrameDamage::Parts { moves, areas } if partial => {
                renderer.render_parts_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
                areas.iter().map(|a| (a.width * a.height) as f64).sum::<f64>() / (w as f64 * h as f64)
            }
            _ => {
                renderer.render_area_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
                1.0
            }
        };
        (started.elapsed().as_secs_f64() * 1000.0, fraction.min(1.0), g.ui.last_paint_count())
    };
    let mut whole = Vec::new();
    let mut full_painted = 0;
    for _ in 0..full.max(1) {
        g.ui.request_repaint();
        let (ms, _, painted) = frame(&mut g, false);
        whole.push(ms);
        full_painted = painted;
    }
    let mut out = Vec::new();
    for (name, change) in scenarios() {
        let (mut events, mut times, mut areas, mut painted) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for i in 0..frames {
            let started = Instant::now();
            change(&mut g, i);
            events.push(started.elapsed().as_secs_f64() * 1000.0);
            let (ms, fraction, count) = frame(&mut g, true);
            times.push(ms);
            areas.push(fraction);
            painted.push(count as f64);
        }
        out.push(Scenario { name, event_ms: median(events), frame_ms: median(times), area: areas.iter().sum::<f64>() / frames.max(1) as f64, painted: median(painted) as usize });
    }
    Report { controls: g.controls, build_ms, layout_ms, full_ms: median(whole), full_painted, scenarios: out }
}

/// A tabela do relatório (o exemplo e o teste imprimem).
pub fn print(report: &Report, width: u32, height: u32) {
    println!("tela {width} × {height} ({:.0} megapixels), {} componentes: montar {:.0} ms, layout {:.1} ms", width as f64 * height as f64 / 1e6, report.controls, report.build_ms, report.layout_ms);
    println!("pintura inteira: {:.0} ms ({} controles)", report.full_ms, report.full_painted);
    println!("{:12} {:>10} {:>10} {:>10} {:>10}", "cenário", "evento", "quadro", "área", "controles");
    for s in &report.scenarios {
        println!("{:12} {:7.2} ms {:7.2} ms {:9.3}% {:10}", s.name, s.event_ms, s.frame_ms, s.area * 100.0, s.painted);
    }
}

#[allow(dead_code)]
fn main() {
    let args: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let (w, h) = (args.first().copied().unwrap_or(WIDTH), args.get(1).copied().unwrap_or(HEIGHT));
    let report = measure(w, h, 3, 40);
    print(&report, w, h);
}
