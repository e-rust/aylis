//! Estresse do DataGrid: 10 milhões de registros com 100 colunas (texto e números),
//! carregados em pacotes. Cada operação que o usuário faz (rolar, ir ao fim, ordenar,
//! filtrar, buscar, ajustar a coluna) tem de responder sem travar a interface.
//!
//! ```text
//! cargo test --release -p aylis-dbcontrols --test huge_grid -- --ignored --nocapture
//! AYLIS_HUGE_ROWS=1000000 cargo test ...      (menos registros)
//! ```

use std::time::{Duration, Instant};

use aylis_controls::Form;
use aylis_core::platform::LocaleInfo;
use aylis_core::*;
use aylis_data::*;
use aylis_dbcontrols::*;

const TEXT_COLUMNS: usize = 50;
const NUMBER_COLUMNS: usize = 49;
const BATCH: usize = 50_000;

/// Canvas que só conta o que seria desenhado.
#[derive(Default)]
struct Counter {
    texts: usize,
}

impl Canvas for Counter {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {
        self.texts += 1;
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
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

fn word(column: usize, row: usize) -> usize {
    (row.wrapping_mul(2654435761) ^ column.wrapping_mul(40503)) % 1000
}

/// Um pacote de registros a partir de `first`.
fn batch(first: usize, count: usize, vocabulary: &[Vec<String>]) -> Vec<Vec<Value>> {
    (first..first + count)
        .map(|row| {
            let mut values = Vec::with_capacity(1 + TEXT_COLUMNS + NUMBER_COLUMNS);
            values.push(Value::Int(row as i64 + 1));
            for (c, words) in vocabulary.iter().enumerate() {
                values.push(Value::String(words[word(c, row)].clone()));
            }
            for c in 0..NUMBER_COLUMNS {
                let x = word(c + 100, row) as f64;
                values.push(if c % 2 == 0 { Value::Int((x as i64) * 37 + c as i64) } else { Value::Float(x * 1.25 + c as f64 / 10.0) });
            }
            values
        })
        .collect()
}

/// Mede e confere o limite: uma ação do usuário não pode segurar a interface.
fn timed<T>(label: &str, limit: Duration, f: impl FnOnce() -> T) -> T {
    let t = Instant::now();
    let out = f();
    let spent = t.elapsed();
    println!("{label:<44} {:>10.1} ms", spent.as_secs_f64() * 1000.0);
    assert!(spent <= limit, "{label}: {spent:?} (limite {limit:?})");
    out
}

fn paint(ui: &mut Ui, form: ComponentId) -> usize {
    let mut canvas = Counter::default();
    ui.paint(form, &mut canvas);
    canvas.texts
}

fn key(k: Key, ctrl: bool) -> KeyEvent {
    KeyEvent { key: k, modifiers: Modifiers { ctrl, ..Default::default() }, ..Default::default() }
}

#[test]
#[ignore = "pesado: 10 milhões de registros (rode com --release)"]
fn ten_million_rows_hundred_columns() {
    let rows: usize = std::env::var("AYLIS_HUGE_ROWS").ok().and_then(|v| v.parse().ok()).unwrap_or(10_000_000);
    // Uma ação do usuário: até 100 ms (rolar, ir ao fim, pintar). Ordenar ou filtrar
    // tudo: até alguns segundos, proporcional ao tamanho.
    let instant = Duration::from_millis(100);
    let heavy = Duration::from_secs_f64((rows as f64 / 10_000_000.0 * 8.0).max(0.5));

    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let form = ui.add_root(Form::new("grid", 1400.0, 900.0));
    let cds = ui.add(ClientDataSet::new().with_key("id"), form);
    ui.add(Field::new("id", FieldType::Integer).label("Código"), cds);
    for c in 0..TEXT_COLUMNS {
        ui.add(Field::new(format!("t{c}"), FieldType::String).label(format!("Texto {c}")), cds);
    }
    for c in 0..NUMBER_COLUMNS {
        let kind = if c % 2 == 0 { FieldType::Integer } else { FieldType::Float };
        ui.add(Field::new(format!("n{c}"), kind).label(format!("Número {c}")).format("#,##0.00"), cds);
    }
    let ds = ui.add(DataSource::new(cds), form);
    let grid = ui.add(DataGrid::new(ds), form);
    ui.control_mut(grid).unwrap().set_rect(0.0, 0.0, 1400.0, 900.0);
    ui.set_prop(grid, "sortable", true).unwrap();
    ui.set_prop(grid, "filter_row", true).unwrap();
    ui.layout();
    ui.set_focus(grid);

    let vocabulary: Vec<Vec<String>> = (0..TEXT_COLUMNS).map(|c| (0..1000).map(|w| format!("Texto {c}-{w:03} de exemplo")).collect()).collect();
    let before = memory_mb();
    let t = Instant::now();
    let mut slowest = Duration::ZERO;
    let mut loaded = 0;
    while loaded < rows {
        let n = BATCH.min(rows - loaded);
        let data = batch(loaded, n, &vocabulary);
        let b = Instant::now();
        data_set(&mut ui, cds).append_rows(data, loaded + n < rows).unwrap();
        slowest = slowest.max(b.elapsed());
        loaded += n;
    }
    println!("carga de {rows} x 100 em pacotes de {BATCH}: {:.1} s (pacote mais lento {:.0} ms)", t.elapsed().as_secs_f64(), slowest.as_secs_f64() * 1000.0);
    println!("memória dos dados: {:.0} MB", memory_mb() - before);
    // Cada pacote é uma resposta do backend: entrar na tela não pode demorar mais que ele.
    assert!(slowest <= Duration::from_millis(1500), "pacote de {BATCH}: {slowest:?}");
    assert_eq!(view(&ui, cds).record_count(), rows);

    let texts = timed("primeira pintura", instant, || paint(&mut ui, form));
    assert!(texts > 100, "células desenhadas: {texts}");
    timed("repintar", instant, || paint(&mut ui, form));
    timed("Ctrl+End (último registro)", instant, || {
        ui.key_down(form, key(Key::End, true));
        paint(&mut ui, form)
    });
    assert_eq!(data_set(&mut ui, cds).rec_no(), Some(rows - 1));
    timed("Ctrl+Home", instant, || {
        ui.key_down(form, key(Key::Home, true));
        paint(&mut ui, form)
    });
    timed("10x Page Down", instant, || {
        for _ in 0..10 {
            ui.key_down(form, key(Key::PageDown, false));
        }
        paint(&mut ui, form)
    });
    timed("roda: 20 dentes", instant, || {
        for _ in 0..20 {
            ui.wheel(form, WheelEvent { position: PointF::new(400.0, 400.0), delta: PointF::new(0.0, -120.0), lines: true, ..Default::default() });
        }
        paint(&mut ui, form)
    });
    timed("ir ao meio (move_to)", instant, || {
        data_set(&mut ui, cds).move_to(rows / 2).unwrap();
        paint(&mut ui, form)
    });
    timed("ajustar a largura da coluna", instant, || {
        let g = ui.get_mut::<DataGrid>(grid).unwrap() as *mut DataGrid;
        // SAFETY: o grid continua no Ui durante a chamada; o `fit_column` só lê o Ui.
        unsafe { (*g).fit_column(&ui, 1) };
    });
    timed("ordenar por texto (t3)", heavy, || {
        data_set(&mut ui, cds).set_index("t3").unwrap();
        paint(&mut ui, form)
    });
    timed("ordenar por número desc (n4)", heavy, || {
        data_set(&mut ui, cds).set_index("n4 desc").unwrap();
        paint(&mut ui, form)
    });
    let shown = timed("filtrar (n0 > 30000)", heavy, || {
        data_set(&mut ui, cds).set_filter("n0 > 30000").unwrap();
        paint(&mut ui, form);
        view(&ui, cds).visible_count()
    });
    assert!(shown > 0 && shown < rows, "filtrados: {shown}");
    timed("tirar o filtro", heavy, || {
        data_set(&mut ui, cds).set_filter("").unwrap();
        paint(&mut ui, form)
    });
    timed("localizar no fim (id)", heavy, || data_set(&mut ui, cds).locate("id", rows as i64).unwrap());
    println!("memória no fim: {:.0} MB", memory_mb() - before);
}
