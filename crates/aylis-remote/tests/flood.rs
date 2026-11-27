//! A enxurrada de mudanças vindas do backend (26/11): um terminal de cotações com 50 painéis
//! de 20 cotações (1000 rótulos) e um backend que manda `set` o mais rápido que consegue,
//! sem esperar a tela. Mede o que a tela faz enquanto isso, como no `aylis-host`: uma thread
//! lê e interpreta o JSON, e a da tela trata as mensagens e pinta (Skia na CPU, sem janela,
//! só as áreas que mudaram).
//!
//! - **antes:** o canal sem limite, esvaziado inteiro a cada rodada (o que o host fazia);
//! - **depois:** a [`aylis_remote::inbox`], com limite e orçamento de tempo por rodada.
//!
//! Grava `medicoes/enxurrada.json`:
//!
//! ```text
//! cargo test -p aylis-remote --release --test flood -- --ignored --nocapture
//! ```
//!
//! Sem `--ignored`, roda uma versão curta que só confere as garantias (a fila não passa do
//! limite e a tela pinta durante a enxurrada).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use aylis_core::{Color, ComponentId, FrameDamage, Ui};
use aylis_remote::inbox::{Drain, INBOX_BUDGET, INBOX_CAPACITY, inbox};
use serde_json::{Value, json};

const PANELS: usize = 50;
const QUOTES: usize = 20;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn screen() -> (Ui, ComponentId, aylis_skia::SkiaRenderer) {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    aylis_remote::register_all(&mut ui);
    let (pw, ph) = (WIDTH as usize / 10, HEIGHT as usize / 5);
    let panels: Vec<Value> = (0..PANELS)
        .map(|p| {
            let quotes: Vec<Value> = (0..QUOTES)
                .map(|q| json!({ "class": "Label", "name": format!("q{p}_{q}"), "left": 4, "top": 4 + q * (ph - 8) / QUOTES, "width": pw - 8, "height": (ph - 8) / QUOTES, "text": "ATIV3 00,00" }))
                .collect();
            json!({ "class": "Panel", "name": format!("pnl{p}"), "left": (p % 10) * pw, "top": (p / 10) * ph, "width": pw, "height": ph, "children": quotes })
        })
        .collect();
    let layout = json!({ "cmd": "load", "layout": { "root": { "class": "Form", "name": "frmCotacoes", "width": WIDTH, "height": HEIGHT, "children": panels } } });
    aylis_remote::execute(&mut ui, &layout).unwrap();
    let form = ui.find("frmCotacoes").unwrap();
    ui.show(form).unwrap();
    (ui, form, renderer)
}

/// A mensagem `i` do backend: uma cotação nova num rótulo qualquer (como texto, como chega
/// pelo pipe).
fn message(i: usize) -> String {
    let k = (i.wrapping_mul(2_654_435_761)) % (PANELS * QUOTES);
    json!({ "cmd": "set", "target": format!("q{}_{}", k / QUOTES, k % QUOTES), "prop": "text", "value": format!("ATIV{} {},{:02}", k % 10, 10 + i % 90, i % 100) }).to_string()
}

#[derive(Default)]
struct Run {
    messages: usize,
    total_ms: f64,
    frames: usize,
    /// O maior intervalo sem pintar (ms), do primeiro quadro ao fim.
    worst_gap_ms: f64,
    /// A mediana do intervalo entre quadros (ms).
    median_gap_ms: f64,
    paint_ms_p50: f64,
    /// O máximo de mensagens esperando na fila.
    peak_queue: usize,
    /// O máximo tratado numa rodada.
    longest_round: usize,
}

impl Run {
    fn json(&self) -> Value {
        json!({
            "mensagens": self.messages,
            "mensagens_por_s": (self.messages as f64 / (self.total_ms / 1000.0)).round(),
            "total_ms": round(self.total_ms),
            "quadros": self.frames,
            "quadros_por_s": round(self.frames as f64 / (self.total_ms / 1000.0)),
            "intervalo_mediano_ms": round(self.median_gap_ms),
            "pior_intervalo_ms": round(self.worst_gap_ms),
            "pintura_p50_ms": round(self.paint_ms_p50),
            "pico_da_fila": self.peak_queue,
            "maior_rodada": self.longest_round,
        })
    }
}

/// Pinta o que mudou (como a plataforma) e diz quanto levou.
fn frame(ui: &mut Ui, form: ComponentId, renderer: &aylis_skia::SkiaRenderer, pixels: &mut [u32]) -> f64 {
    let started = Instant::now();
    match ui.take_frame(form).snapped(1.0) {
        FrameDamage::Parts { moves, areas } => {
            renderer.render_parts_to_pixels(pixels, WIDTH, HEIGHT, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
        }
        FrameDamage::Whole => {
            renderer.render_area_to_pixels(pixels, WIDTH, HEIGHT, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
        }
    }
    ui.frame_drawn();
    started.elapsed().as_secs_f64() * 1000.0
}

/// O backend: interpreta e entrega as `n` mensagens o mais rápido que der.
fn backend(n: usize, sent: Arc<AtomicUsize>, deliver: impl Fn(Value) -> bool + Send + 'static) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        for i in 0..n {
            let value: Value = serde_json::from_str(&message(i)).unwrap();
            if !deliver(value) {
                return;
            }
            sent.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn finish(run: &mut Run, gaps: &mut [f64], paints: &mut [f64], started: Instant) {
    run.total_ms = started.elapsed().as_secs_f64() * 1000.0;
    gaps.sort_by(f64::total_cmp);
    paints.sort_by(f64::total_cmp);
    run.worst_gap_ms = gaps.last().copied().unwrap_or(run.total_ms);
    run.median_gap_ms = gaps.get(gaps.len() / 2).copied().unwrap_or(run.total_ms);
    run.paint_ms_p50 = paints.get(paints.len() / 2).copied().unwrap_or(0.0);
}

/// Antes: o canal sem limite, esvaziado inteiro antes de pintar.
fn before(n: usize) -> Run {
    let (mut ui, form, renderer) = screen();
    let mut pixels = vec![0u32; (WIDTH * HEIGHT) as usize];
    frame(&mut ui, form, &renderer, &mut pixels);
    let (tx, rx) = std::sync::mpsc::channel::<Value>();
    let sent = Arc::new(AtomicUsize::new(0));
    let started = Instant::now();
    let producer = backend(n, sent.clone(), move |v| tx.send(v).is_ok());
    let mut run = Run { messages: n, ..Default::default() };
    let (mut gaps, mut paints) = (Vec::new(), Vec::new());
    let mut last = Instant::now();
    let mut handled = 0usize;
    while handled < n {
        let mut round = 0;
        while let Ok(message) = rx.try_recv() {
            run.peak_queue = run.peak_queue.max(sent.load(Ordering::Relaxed).saturating_sub(handled));
            aylis_remote::handle_message(&mut ui, &message);
            handled += 1;
            round += 1;
        }
        run.longest_round = run.longest_round.max(round);
        if ui.needs_repaint() {
            paints.push(frame(&mut ui, form, &renderer, &mut pixels));
            gaps.push(last.elapsed().as_secs_f64() * 1000.0);
            last = Instant::now();
            run.frames += 1;
        } else {
            std::thread::yield_now();
        }
    }
    producer.join().unwrap();
    finish(&mut run, &mut gaps, &mut paints, started);
    run
}

/// Depois: a fila com limite e a rodada com orçamento.
fn after(n: usize) -> Run {
    let (mut ui, form, renderer) = screen();
    let mut pixels = vec![0u32; (WIDTH * HEIGHT) as usize];
    frame(&mut ui, form, &renderer, &mut pixels);
    let (tx, mut inbox) = inbox(INBOX_CAPACITY);
    let sent = Arc::new(AtomicUsize::new(0));
    let woken = Arc::new(AtomicBool::new(false));
    let w = woken.clone();
    let started = Instant::now();
    let producer = backend(n, sent.clone(), move |v| tx.send(v, || w.store(true, Ordering::Release)).is_ok());
    let mut run = Run { messages: n, ..Default::default() };
    let (mut gaps, mut paints) = (Vec::new(), Vec::new());
    let mut last = Instant::now();
    loop {
        run.peak_queue = run.peak_queue.max(sent.load(Ordering::Relaxed).saturating_sub(inbox.handled()));
        let result = inbox.drain(&mut ui, INBOX_BUDGET, |_| {});
        if ui.needs_repaint() {
            paints.push(frame(&mut ui, form, &renderer, &mut pixels));
            gaps.push(last.elapsed().as_secs_f64() * 1000.0);
            last = Instant::now();
            run.frames += 1;
        }
        match result {
            Drain::Closed => break,
            Drain::More => {}
            Drain::Empty => std::thread::yield_now(),
        }
    }
    producer.join().unwrap();
    run.longest_round = inbox.longest_round();
    finish(&mut run, &mut gaps, &mut paints, started);
    run
}

/// As garantias, numa enxurrada curta: a fila não passa do limite e a tela pinta no meio.
#[test]
fn the_flood_keeps_the_queue_bounded_and_the_screen_painting() {
    let run = after(20_000);
    assert!(run.peak_queue <= INBOX_CAPACITY + 1, "a fila chegou a {}", run.peak_queue);
    assert!(run.frames >= 2, "a tela pintou {} vezes durante a enxurrada", run.frames);
}

#[test]
#[ignore = "medição: rode em release (veja o cabeçalho)"]
fn flood_cost() {
    let n = 1_000_000;
    let before = before(n);
    let after = after(n);
    let report = json!({
        "descricao": "A enxurrada de mudanças do backend (26/11): 50 painéis de 20 cotações (1000 Label) num form de 1920 x 1080, e um backend que manda 1 milhão de `set` de texto sem esperar a tela (o JSON interpretado numa thread, como no aylis-host). A tela trata e pinta só o que mudou (Skia na CPU, sem janela), release, na máquina do autor. 'antes': o canal sem limite esvaziado inteiro antes de pintar; 'depois': a fila de 4096 mensagens e o orçamento de 8 ms por rodada (aylis_remote::inbox). crates/aylis-remote/tests/flood.rs.",
        "antes": before.json(),
        "depois": after.json(),
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../../medicoes/enxurrada.json");
    std::fs::write(out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    assert!(after.peak_queue <= INBOX_CAPACITY + 1);
    assert!(after.worst_gap_ms < 100.0, "pior intervalo sem pintar: {} ms", after.worst_gap_ms);
}

fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}
