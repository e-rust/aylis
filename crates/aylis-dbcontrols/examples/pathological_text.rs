//! O texto patológico (26/11), no Skia da CPU, sem janela, numa tela de 1600 × 900:
//!
//! - `linha [MB]`: um JSON minificado numa linha só (padrão 50 MB) no `CodeEditor`;
//! - `log [MB]`: um log de servidor (padrão 2048 MB, linhas de ~120 caracteres) no `Memo`;
//! - `grade [linhas]`: um `DataGrid` de 1 milhão de linhas (padrão) e 4 colunas com árabe,
//!   devanágari, emoji (com modificadores e famílias) e texto bidirecional misturado.
//!
//! Para cada um: carregar, o primeiro quadro, rolar (a roda), ir ao fim (Ctrl+End) e digitar
//! no meio, cada um com o tempo do evento e do quadro, e a memória do processo. A última
//! linha, `AYLIS_TEXT`, é o JSON do cenário (para `medicoes/texto_patologico.json`).
//!
//! ```text
//! cargo run --release -p aylis-dbcontrols --example pathological_text -- linha 50
//! ```

use std::time::Instant;

use aylis_core::*;
use serde_json::json;

const W: u32 = 1600;
const H: u32 = 900;

fn main() {
    let mut args = std::env::args().skip(1);
    let scenario = args.next().unwrap_or_else(|| "linha".into());
    let size: Option<usize> = args.next().and_then(|n| n.parse().ok());
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_dbcontrols::register(&mut ui);
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let memory_before = memory_mb();

    let started = Instant::now();
    let (target, amount) = match scenario.as_str() {
        "linha" => {
            let mb = size.unwrap_or(50);
            let text = minified_json(mb * 1_000_000);
            let built = started.elapsed().as_secs_f64() * 1000.0;
            println!("JSON de {:.1} MB montado em {built:.0} ms", text.len() as f64 / 1e6);
            let started = Instant::now();
            let editor = ui.create("CodeEditor", Some(form)).unwrap();
            ui.set_prop(editor, "align", "client").unwrap();
            // `AYLIS_TEXT_SYNTAX=` (vazio) mede sem o realce.
            ui.set_prop(editor, "syntax", std::env::var("AYLIS_TEXT_SYNTAX").unwrap_or_else(|_| "json".into())).unwrap();
            // `AYLIS_TEXT_OFF=minimap,folding` desliga partes do editor (para achar o custo).
            for prop in std::env::var("AYLIS_TEXT_OFF").unwrap_or_default().split(',').filter(|p| !p.is_empty()) {
                ui.set_prop(editor, prop, false).unwrap();
            }
            ui.set_prop(editor, "text", text).unwrap();
            ui.layout();
            println!("carregar no CodeEditor: {:.0} ms", started.elapsed().as_secs_f64() * 1000.0);
            (editor, format!("{mb} MB numa linha"))
        }
        "log" => {
            let mb = size.unwrap_or(2048);
            let text = server_log(mb * 1_000_000);
            println!("log de {:.1} MB montado em {:.0} ms", text.len() as f64 / 1e6, started.elapsed().as_secs_f64() * 1000.0);
            let started = Instant::now();
            let memo = ui.create("Memo", Some(form)).unwrap();
            ui.set_prop(memo, "align", "client").unwrap();
            ui.set_prop(memo, "text", text).unwrap();
            ui.layout();
            println!("carregar no Memo: {:.0} ms", started.elapsed().as_secs_f64() * 1000.0);
            (memo, format!("{mb} MB de log"))
        }
        "grade" => {
            let rows = size.unwrap_or(1_000_000);
            let grid = multilingual_grid(&mut ui, form, rows);
            println!("carregar o grid: {:.0} ms", started.elapsed().as_secs_f64() * 1000.0);
            (grid, format!("{rows} linhas x 4 colunas"))
        }
        other => panic!("cenário desconhecido: {other} (linha, log, grade)"),
    };
    let load_ms = started.elapsed().as_secs_f64() * 1000.0;
    ui.set_focus(target);

    let mut pixels = vec![0u32; (W * H) as usize];
    let mut frame = |ui: &mut Ui| {
        let t = Instant::now();
        match ui.take_frame(form).snapped(1.0) {
            FrameDamage::Parts { moves, areas } => {
                renderer.render_parts_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
            }
            FrameDamage::Whole => {
                renderer.render_area_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
            }
        }
        ui.frame_drawn();
        t.elapsed().as_secs_f64() * 1000.0
    };
    let first = frame(&mut ui);
    println!("primeiro quadro: {first:.1} ms");
    // O realce e o que vem aos poucos.
    for i in 0..30 {
        ui.set_time(1.0 + i as f64 * 0.1);
        frame(&mut ui);
    }
    let r = ui.absolute_rect(target).unwrap();
    let at = PointF::new(r.x + 300.0, r.y + 300.0);
    let mut step = |ui: &mut Ui, what: &str, n: usize, mut act: Box<dyn FnMut(&mut Ui, usize)>| {
        let (mut event, mut paint, mut worst) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..n {
            let t = Instant::now();
            act(ui, i);
            let e = t.elapsed().as_secs_f64() * 1000.0;
            let p = frame(ui);
            event += e;
            paint += p;
            worst = worst.max(e + p);
        }
        let (e, p) = (event / n as f64, paint / n as f64);
        println!("{what}: evento {e:.2} ms, quadro {p:.2} ms, pior {worst:.1} ms");
        json!({ "evento_ms": round(e), "quadro_ms": round(p), "pior_ms": round(worst) })
    };
    let wheel = step(&mut ui, "roda", 30, Box::new(move |ui, i| {
        ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, if i < 20 { -120.0 } else { 120.0 }), lines: true, ..Default::default() });
    }));
    let sideways = step(&mut ui, "roda para o lado", 20, Box::new(move |ui, _| {
        ui.wheel(form, WheelEvent { position: at, delta: PointF::new(-120.0, 0.0), lines: true, ..Default::default() });
    }));
    let end = step(&mut ui, "Ctrl+End e Ctrl+Home", 6, Box::new(move |ui, i| {
        let key = if i % 2 == 0 { Key::End } else { Key::Home };
        ui.key_down(form, KeyEvent { key, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    }));
    // No meio: o fim e depois metade das páginas de volta não é simples em todos; a seta para
    // a direita no começo de uma linha enorme serve de "meio" na linha única.
    let typing = step(&mut ui, "digitar", 20, Box::new(move |ui, _| ui.text_input(form, "x")));
    let memory = memory_mb() - memory_before;
    println!("memória a mais: {memory:.0} MB");
    let report = json!({
        "cenario": scenario,
        "tamanho": amount,
        "carregar_ms": round(load_ms),
        "primeiro_quadro_ms": round(first),
        "roda": wheel,
        "roda_para_o_lado": sideways,
        "ctrl_end_home": end,
        "digitar": typing,
        "memoria_mb": memory.round(),
    });
    println!("AYLIS_TEXT {report}");
}

/// Um JSON minificado de `bytes` bytes (registros de um array, sem quebra de linha).
fn minified_json(bytes: usize) -> String {
    let mut out = String::with_capacity(bytes + 256);
    out.push('[');
    let mut i = 0usize;
    while out.len() < bytes {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(r#"{{"id":{i},"nome":"Cliente {i}","cidade":"São Paulo","saldo":{}.{:02},"tags":["a","b"],"ativo":{}}}"#, i * 37 % 100_000, i % 100, i % 3 == 0));
        i += 1;
    }
    out.push(']');
    out
}

/// Um log de servidor de `bytes` bytes.
fn server_log(bytes: usize) -> String {
    let mut out = String::with_capacity(bytes + 256);
    let levels = ["INFO", "WARN", "DEBUG", "ERROR"];
    let mut i = 0usize;
    while out.len() < bytes {
        out.push_str(&format!(
            "2026-11-26T{:02}:{:02}:{:02}.{:03}Z {} [worker-{}] GET /api/pedidos/{} 200 {} ms user=u{} trace={:016x}\n",
            (i / 3_600_000) % 24,
            (i / 60_000) % 60,
            (i / 1000) % 60,
            i % 1000,
            levels[i % 7 % 4],
            i % 16,
            i * 13 % 1_000_000,
            i % 900,
            i % 5000,
            (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ));
        i += 1;
    }
    out
}

/// O grid com textos de várias escritas.
fn multilingual_grid(ui: &mut Ui, form: ComponentId, rows: usize) -> ComponentId {
    use aylis_data::{ClientDataSet, DataSource, Field, FieldType, Value, data_set};
    let arabic = ["مرحبا بالعالم", "طلب رقم ١٢٣٤", "العميل: محمد علي", "الرياض – المملكة"];
    let hindi = ["नमस्ते दुनिया", "आदेश संख्या ४२", "ग्राहक: श्रीमती क्षत्रिय", "दिल्ली, भारत"];
    let emoji = ["👍🏽 ok", "👨‍👩‍👧‍👦 família", "🇧🇷🇯🇵 bandeiras", "🧑🏿‍💻 dev ✨"];
    let bidi = ["Pedido 123 من الرياض to Delhi दिल्ली", "שלום abc 123 مرحبا", "R$ 10,00 = ١٠ ريال (≈ ₹ 200)", "\u{202B}RLE embutido\u{202C} e PDF"];
    let cds = ui.add(ClientDataSet::new(), form);
    for (name, label) in [("ar", "Árabe"), ("hi", "Devanágari"), ("emoji", "Emoji"), ("bidi", "Misturado")] {
        ui.add(Field::new(name, FieldType::String).label(label), cds);
    }
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(aylis_dbcontrols::DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, W as f32, H as f32);
    let mut loaded = 0;
    while loaded < rows {
        let n = 50_000.min(rows - loaded);
        let batch = (loaded..loaded + n)
            .map(|r| {
                vec![
                    Value::String(format!("{} {r}", arabic[r % 4])),
                    Value::String(format!("{} {r}", hindi[r % 4])),
                    Value::String(format!("{} {r}", emoji[r % 4])),
                    Value::String(format!("{} #{r}", bidi[r % 4])),
                ]
            })
            .collect();
        data_set(ui, cds).append_rows(batch, loaded + n < rows).unwrap();
        loaded += n;
    }
    ui.layout();
    grid
}

fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}


/// Memória do processo (privada), em MB.
#[cfg(windows)]
fn memory_mb() -> f64 {
    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        cb: u32,
        page_faults: u32,
        peak_working_set: usize,
        working_set: usize,
        quota_peak_paged: usize,
        quota_paged: usize,
        quota_peak_non_paged: usize,
        quota_non_paged: usize,
        pagefile: usize,
        peak_pagefile: usize,
        private: usize,
    }
    unsafe extern "system" {
        fn GetCurrentProcess() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut Counters, cb: u32) -> i32;
    }
    let mut c = Counters { cb: std::mem::size_of::<Counters>() as u32, ..Default::default() };
    // SAFETY: a struct é a PROCESS_MEMORY_COUNTERS_EX, com o tamanho informado.
    unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut c, c.cb) };
    c.private as f64 / (1024.0 * 1024.0)
}

#[cfg(not(windows))]
fn memory_mb() -> f64 {
    0.0
}
