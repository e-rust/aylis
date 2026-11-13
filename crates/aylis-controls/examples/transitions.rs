//! O custo, na thread da interface, de cada quadro de uma transição ou de uma animação de
//! lugar/transparência: a base para decidir se vale um compositor em outra thread (pendente
//! "Compositor em outra thread", 12/11/2026). Sem janela, o Skia na CPU (como o
//! `giant_screen`), quadro a quadro num relógio de 60 Hz.
//!
//! ```text
//! cargo run --release -p aylis-controls --example transitions [largura] [altura]
//! ```
//!
//! Cada cenário roda várias vezes; os números são de todos os quadros juntos. O primeiro
//! quadro (o do clique: a página nova aparece) é separado dos outros, porque o compositor não
//! o evitaria (a página nova tem de ser pintada pela interface ao menos uma vez).

use std::time::Instant;

use aylis_core::*;

/// O quadro de 60 Hz, em ms.
const FRAME_MS: f64 = 1000.0 / 60.0;
/// Quanto dura a animação do painel (a mesma da transição das abas).
const ANIMATION_MS: f32 = TRANSITION_MS;
/// Uma célula das páginas: um componente de cada vez, em ciclo pelas classes.
const CELL: (f32, f32) = (160.0, 40.0);
const CLASSES: [&str; 8] = ["Label", "Edit", "Button", "CheckBox", "ProgressBar", "Switch", "ComboBox", "TrackBar"];

struct Screen {
    ui: Ui,
    form: ComponentId,
    tabs: ComponentId,
    panel: ComponentId,
    controls: usize,
    width: u32,
    height: u32,
}

/// O resultado de um cenário (tempos em ms).
#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: &'static str,
    /// O clique (trocar de aba, começar a animação), sem a pintura.
    pub event_ms: f64,
    /// O primeiro quadro depois do clique (a mediana das rodadas).
    pub first_ms: f64,
    /// Os quadros seguintes, até a transição acabar.
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
    /// Quadros (todos, o primeiro inclusive) que passaram de 16,7 ms.
    pub over_budget: usize,
    pub frames: usize,
    /// A fração média da tela repintada nos quadros seguintes.
    pub area: f64,
    /// Controles pintados por quadro (a mediana dos quadros seguintes).
    pub painted: usize,
    /// Megabytes que ficaram nas camadas guardadas do renderizador depois do cenário.
    pub kept_mb: f64,
}

#[derive(Clone, Debug)]
pub struct Report {
    pub width: u32,
    pub height: u32,
    pub controls: usize,
    pub full_ms: f64,
    pub scenarios: Vec<Scenario>,
}

/// Enche `parent` com linhas de controles (como uma tela de cadastro cheia).
fn fill(ui: &mut Ui, parent: ComponentId, width: u32, height: u32, tag: &str) {
    let (cols, rows) = ((width as f32 / CELL.0) as usize, ((height as f32 - 40.0) / CELL.1) as usize);
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

fn build(width: u32, height: u32) -> Screen {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    let _ = ui.set_prop(form, "width", width as f64);
    let _ = ui.set_prop(form, "height", height as f64);
    // Duas abas cheias ocupando a tela.
    let tabs = ui.create("TabControl", Some(form)).unwrap();
    let _ = ui.set_prop(tabs, "align", "client");
    for tag in ["A", "B"] {
        let page = ui.create("TabItem", Some(tabs)).unwrap();
        let _ = ui.set_prop(page, "caption", format!("Página {tag}"));
        fill(&mut ui, page, width, height, tag);
    }
    // Um painel cheio, por cima, que desliza (escondido fora dos cenários dele).
    let panel = ui.create("Panel", Some(form)).unwrap();
    for (name, value) in [("align", PropValue::from("none")), ("left", 0.0.into()), ("top", 0.0.into()), ("width", (width as f64 / 2.0).into()), ("height", (height as f64).into()), ("visible", false.into())] {
        let _ = ui.set_prop(panel, name, value);
    }
    fill(&mut ui, panel, width / 2, height, "P");
    ui.show(form).unwrap();
    ui.layout();
    let controls = ui.components().count();
    Screen { ui, form, tabs, panel, controls, width, height }
}

fn percentile(values: &[f64], p: f64) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    if v.is_empty() {
        return 0.0;
    }
    v[((v.len() - 1) as f64 * p).round() as usize]
}

/// O que cada cenário faz no clique (`round` alterna o sentido).
type Start = fn(&mut Screen, usize);

fn scenarios() -> [(&'static str, Start); 4] {
    fn tab(s: &mut Screen, round: usize, transition: &str) {
        let _ = s.ui.set_prop(s.tabs, "transition", transition);
        let _ = s.ui.set_prop(s.tabs, "tab_index", ((round + 1) % 2) as i64);
    }
    fn slide(s: &mut Screen, round: usize, cached: bool) {
        let _ = s.ui.set_prop(s.panel, "cache_layer", cached);
        let _ = s.ui.set_prop(s.panel, "visible", true);
        let to = if round % 2 == 0 { s.width as f64 / 2.0 } else { 0.0 };
        let _ = s.ui.animate(s.panel, "translate_x", to, AnimationOptions::new(ANIMATION_MS, anim::Easing::OutCubic));
    }
    [
        ("aba deslizar", |s, r| tab(s, r, "slide_left")),
        ("aba esmaecer", |s, r| tab(s, r, "fade")),
        ("painel animado", |s, r| slide(s, r, false)),
        ("painel cache_layer", |s, r| slide(s, r, true)),
    ]
}

/// Mede todos os cenários, `rounds` vezes cada.
pub fn measure(width: u32, height: u32, rounds: usize) -> Report {
    let mut s = build(width, height);
    let renderer = aylis_skia::SkiaRenderer::new();
    let (w, h) = (s.width, s.height);
    let mut pixels = vec![0u32; w as usize * h as usize];
    // Um quadro como a plataforma faz: as animações andam, e as cópias e as áreas (ou tudo)
    // por cima do anterior.
    let mut frame = |s: &mut Screen, partial: bool| {
        let started = Instant::now();
        s.ui.tick();
        let damage = s.ui.take_frame(s.form).snapped(1.0);
        let (ui, form) = (&mut s.ui, s.form);
        let fraction = match damage {
            FrameDamage::Parts { moves, areas } if partial => {
                renderer.render_parts_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
                areas.iter().map(|a| (a.width * a.height) as f64).sum::<f64>() / (w as f64 * h as f64)
            }
            FrameDamage::Parts { .. } if partial => 0.0,
            _ => {
                renderer.render_area_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
                1.0
            }
        };
        (started.elapsed().as_secs_f64() * 1000.0, fraction.min(1.0), s.ui.last_paint_count())
    };
    let mut whole = Vec::new();
    for _ in 0..3 {
        s.ui.request_repaint();
        whole.push(frame(&mut s, false).0);
    }
    let mut out = Vec::new();
    for (name, start) in scenarios() {
        let (mut events, mut firsts, mut rest, mut areas, mut painted) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        // Uma rodada de aquecimento (as fontes, as imagens guardadas), fora da conta.
        for round in 0..=rounds {
            let started = Instant::now();
            start(&mut s, round);
            let event = started.elapsed().as_secs_f64() * 1000.0;
            let mut times = Vec::new();
            // Até a transição acabar, mais dois quadros (o fim volta os valores).
            let total = (TRANSITION_MS as f64 / FRAME_MS).ceil() as usize + 2;
            for i in 0..total {
                if i > 0 {
                    let t = s.ui.time() + FRAME_MS / 1000.0;
                    s.ui.set_time(t);
                }
                let (ms, fraction, count) = frame(&mut s, true);
                times.push(ms);
                if i > 0 && round > 0 {
                    areas.push(fraction);
                    painted.push(count as f64);
                }
            }
            // Parado até a próxima rodada (nada fica pedindo quadro).
            s.ui.set_time(s.ui.time() + 1.0);
            frame(&mut s, true);
            if round > 0 {
                events.push(event);
                firsts.push(times[0]);
                rest.extend_from_slice(&times[1..]);
            }
        }
        let _ = s.ui.set_prop(s.panel, "visible", false);
        frame(&mut s, true);
        // O que ficou nas camadas guardadas do renderizador depois do cenário (parado).
        let kept_mb = image_cache::stats().iter().filter(|c| c.name.contains("camadas guardadas")).map(|c| c.bytes).sum::<usize>() as f64 / (1 << 20) as f64;
        let over = firsts.iter().chain(rest.iter()).filter(|&&ms| ms > FRAME_MS).count();
        out.push(Scenario {
            name,
            event_ms: percentile(&events, 0.5),
            first_ms: percentile(&firsts, 0.5),
            p50_ms: percentile(&rest, 0.5),
            p95_ms: percentile(&rest, 0.95),
            max_ms: percentile(&rest, 1.0),
            over_budget: over,
            frames: firsts.len() + rest.len(),
            area: areas.iter().sum::<f64>() / areas.len().max(1) as f64,
            painted: percentile(&painted, 0.5) as usize,
            kept_mb,
        });
    }
    Report { width: w, height: h, controls: s.controls, full_ms: percentile(&whole, 0.5), scenarios: out }
}

pub fn print(report: &Report) {
    println!("tela {} × {}, {} componentes; pintura inteira {:.1} ms", report.width, report.height, report.controls, report.full_ms);
    println!(
        "{:20} {:>8} {:>9} {:>8} {:>8} {:>8} {:>10} {:>7} {:>9} {:>10}",
        "cenário", "evento", "1º quadro", "p50", "p95", "máx", "> 16,7 ms", "área", "controles", "guardado"
    );
    for s in &report.scenarios {
        println!(
            "{:20} {:8.2} {:9.2} {:8.2} {:8.2} {:8.2} {:>10} {:6.1}% {:9} {:7.1} MB",
            s.name,
            s.event_ms,
            s.first_ms,
            s.p50_ms,
            s.p95_ms,
            s.max_ms,
            format!("{}/{}", s.over_budget, s.frames),
            s.area * 100.0,
            s.painted,
            s.kept_mb
        );
    }
}

#[allow(dead_code)]
fn main() {
    let args: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let sizes: Vec<(u32, u32)> = match (args.first(), args.get(1)) {
        (Some(&w), Some(&h)) => vec![(w, h)],
        _ => vec![(1920, 1080), (3840, 2160)],
    };
    for (w, h) in sizes {
        print(&measure(w, h, 8));
        println!();
    }
}
