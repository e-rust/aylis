//! As transições e as animações de lugar numa janela de verdade, com o renderizador da
//! plataforma (o Direct2D no Windows): o par do exemplo `transitions` do `aylis-controls`,
//! que mede o mesmo sem janela, no Skia na CPU (pendente "Compositor em outra thread",
//! decidido em 13/11: medir no Direct2D com janela).
//!
//! ```text
//! cargo run --release -p aylis-platform-winit --example transitions --features d2d [largura] [altura] [zoom]
//! ```
//!
//! O `zoom` do form (padrão 1) compensa a escala do monitor: num monitor a 150%, `0.6667` faz
//! a largura e a altura valerem em pixels, como no exemplo sem janela.
//!
//! O quadro é o da plataforma (a pintura mais o mandar e o apresentar, pelo profiler), sem
//! esperar o monitor (`AYLIS_VSYNC=0`, ligado aqui), para medir o custo e não a espera. Cada
//! cenário roda `ROUNDS` vezes, uma por vez; no fim, a tabela sai no terminal e a janela fecha.
//! `AYLIS_RENDERER=cpu|gpu|d2d` troca o renderizador.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use aylis_controls::*;
use aylis_core::*;

const ROUNDS: usize = 6;
/// O intervalo entre um clique e o próximo (a transição dura 250 ms).
const GAP_MS: u32 = 700;
const FRAME_MS: f64 = 1000.0 / 60.0;
const CELL: (f32, f32) = (160.0, 40.0);
const CLASSES: [&str; 8] = ["Label", "Edit", "Button", "CheckBox", "ProgressBar", "Switch", "ComboBox", "TrackBar"];
const SCENARIOS: [&str; 4] = ["aba deslizar", "aba esmaecer", "painel animado", "painel cache_layer"];

fn fill(ui: &mut Ui, parent: ComponentId, width: f32, height: f32, tag: &str) {
    let (cols, rows) = ((width / CELL.0) as usize, ((height - 40.0) / CELL.1) as usize);
    for r in 0..rows {
        let row = ui.create("Panel", Some(parent)).unwrap();
        let _ = ui.set_prop(row, "align", "top");
        let _ = ui.set_prop(row, "height", CELL.1 as f64);
        for c in 0..cols {
            let id = ui.create(CLASSES[(r + c) % CLASSES.len()], Some(row)).unwrap();
            let _ = ui.set_prop(id, "align", "left");
            let _ = ui.set_prop(id, "width", CELL.0 as f64 - 4.0);
            let _ = ui.set_prop(id, "text", format!("{tag} {r}.{c}"));
        }
    }
}

/// Um clique de um cenário: quando foi (no relógio do profiler) e quanto o evento levou.
struct Click {
    scenario: usize,
    at: f64,
    event_ms: f64,
}

fn start(ui: &mut Ui, tabs: ComponentId, panel: ComponentId, width: f32, scenario: usize, round: usize) {
    match scenario {
        0 | 1 => {
            let _ = ui.set_prop(tabs, "transition", if scenario == 0 { "slide_left" } else { "fade" });
            let _ = ui.set_prop(tabs, "tab_index", ((round + 1) % 2) as i64);
        }
        _ => {
            let _ = ui.set_prop(panel, "cache_layer", scenario == 3);
            let _ = ui.set_prop(panel, "visible", true);
            let to = if round % 2 == 0 { width as f64 / 2.0 } else { 0.0 };
            let _ = ui.animate(panel, "translate_x", to, AnimationOptions::new(TRANSITION_MS, anim::Easing::OutCubic));
        }
    }
}

fn percentile(values: &[f64], p: f64) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v.get(((v.len().max(1) - 1) as f64 * p).round() as usize).copied().unwrap_or(0.0)
}

/// A tabela: os quadros de cada clique, do clique até o próximo (a janela parada não pinta).
fn report(ui: &Ui, clicks: &[Click], spans: &[profile::Span], samples: &[profile::Sample], (width, height, zoom): (f32, f32, f32)) {
    // Um quadro = a pintura e, logo depois, o bloco da GPU (mandar e apresentar).
    let mut frames: Vec<(f64, f64, f64, f64, f64)> = Vec::new(); // (começo, ms, % repintado, pintura ms, controles)
    // Os controles de cada pintura: a amostra gravada no fim dela.
    let painted: Vec<(f64, f64)> = samples.iter().filter(|s| s.name == "controles pintados").map(|s| (s.time, s.value)).collect();
    let mut paint: Option<&profile::Span> = None;
    for s in spans {
        match s.lane {
            profile::Lane::Paint if s.name.starts_with("pintura") && s.depth == 0 => paint = Some(s),
            profile::Lane::Gpu => {
                if let Some(p) = paint.take() {
                    let pct = s.name.split('·').nth(1).and_then(|t| t.trim().split('%').next()).and_then(|n| n.trim().parse().ok()).unwrap_or(100.0);
                    let end = p.start + p.duration;
                    let count = painted.iter().find(|(t, _)| *t >= p.start && *t <= end + 1e-4).map_or(0.0, |c| c.1);
                    frames.push((p.start, (s.start + s.duration - p.start) * 1000.0, pct, p.duration * 1000.0, count));
                }
            }
            _ => {}
        }
    }
    let info = ui.platform_info();
    let get = |k: &str| info.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str()).unwrap_or("?");
    println!("janela {width} × {height} (lógico, zoom {zoom:.3}), {} componentes; renderizador {} · {}", ui.components().count(), get("renderizador"), get("gpu"));
    // "pintura": só a thread da interface (os controles, sem mandar nem apresentar o quadro).
    println!("{:20} {:>8} {:>9} {:>8} {:>8} {:>8} {:>10} {:>9} {:>7} {:>15} {:>15}", "cenário", "evento", "1º quadro", "p50", "p95", "máx", "> 16,7 ms", "quadros/s", "área", "pintura p50/95", "controles p50/máx");
    for (index, name) in SCENARIOS.iter().enumerate() {
        let (mut firsts, mut rest, mut areas, mut events, mut rates, mut paints, mut counts) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        // O primeiro clique de cada cenário é o aquecimento (as fontes, as camadas).
        for (i, click) in clicks.iter().enumerate().filter(|(_, c)| c.scenario == index).skip(1) {
            let end = clicks.get(i + 1).map_or(f64::MAX, |c| c.at);
            let mine: Vec<_> = frames.iter().filter(|f| f.0 >= click.at && f.0 < end).collect();
            let Some((first, others)) = mine.split_first() else { continue };
            events.push(click.event_ms);
            firsts.push(first.1);
            rest.extend(others.iter().map(|f| f.1));
            areas.extend(others.iter().map(|f| f.2));
            paints.extend(others.iter().map(|f| f.3));
            counts.extend(others.iter().map(|f| f.4));
            if let Some(last) = others.last() {
                let span = (last.0 - first.0).max(1e-6);
                rates.push(others.len() as f64 / span);
            }
        }
        let over = firsts.iter().chain(rest.iter()).filter(|&&ms| ms > FRAME_MS).count();
        println!(
            "{:20} {:8.2} {:9.2} {:8.2} {:8.2} {:8.2} {:>10} {:9.0} {:6.1}% {:6.2}/{:6.2} {:>7}/{:>7}",
            name,
            percentile(&events, 0.5),
            percentile(&firsts, 0.5),
            percentile(&rest, 0.5),
            percentile(&rest, 0.95),
            percentile(&rest, 1.0),
            format!("{over}/{}", firsts.len() + rest.len()),
            percentile(&rates, 0.5),
            areas.iter().sum::<f64>() / areas.len().max(1) as f64,
            percentile(&paints, 0.5),
            percentile(&paints, 0.95),
            percentile(&counts, 0.5),
            percentile(&counts, 1.0)
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: antes de qualquer outra thread (o laço da janela ainda não começou).
    unsafe { std::env::set_var("AYLIS_VSYNC", "0") };
    let args: Vec<f32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let (width, height) = (args.first().copied().unwrap_or(1920.0), args.get(1).copied().unwrap_or(1080.0));
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Aylis — transições", width, height));
    let zoom = args.get(2).copied().unwrap_or(1.0);
    let _ = ui.set_prop(form, "zoom", zoom as f64);
    let tabs = ui.create("TabControl", Some(form)).unwrap();
    let _ = ui.set_prop(tabs, "align", "client");
    for tag in ["A", "B"] {
        let page = ui.create("TabItem", Some(tabs)).unwrap();
        let _ = ui.set_prop(page, "caption", format!("Página {tag}"));
        fill(&mut ui, page, width, height, tag);
    }
    let panel = ui.create("Panel", Some(form)).unwrap();
    for (name, value) in [("align", PropValue::from("none")), ("left", 0.0.into()), ("top", 0.0.into()), ("width", (width as f64 / 2.0).into()), ("height", (height as f64).into()), ("visible", false.into())] {
        let _ = ui.set_prop(panel, name, value);
    }
    fill(&mut ui, panel, width / 2.0, height, "P");

    profile::start();
    let clicks: Rc<RefCell<Vec<Click>>> = Rc::default();
    let step = std::cell::Cell::new(0usize);
    let timer = ui.add(Timer::new(GAP_MS), form);
    ui.on(timer, "on_timer", move |ui, _, _| {
        let n = step.get();
        step.set(n + 1);
        // Cada cenário: o aquecimento e as `ROUNDS` rodadas; o painel some no fim dos dele.
        let per = ROUNDS + 1;
        if n == SCENARIOS.len() * per {
            let (spans, samples, _) = profile::take();
            report(ui, &clicks.borrow(), &spans, &samples, (width, height, zoom));
            ui.request_close();
            return;
        }
        let (scenario, round) = (n / per, n % per);
        if round == 0 {
            let _ = ui.set_prop(panel, "visible", false);
        }
        let at = profile::now();
        let started = Instant::now();
        start(ui, tabs, panel, width, scenario, round);
        clicks.borrow_mut().push(Click { scenario, at, event_ms: started.elapsed().as_secs_f64() * 1000.0 });
    });
    aylis_platform_winit::run(ui, form)
}
