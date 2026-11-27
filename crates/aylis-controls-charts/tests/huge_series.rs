//! O gráfico com milhões de pontos (26/11): uma série de linha como a de um sensor ou de um
//! osciloscópio, num `Chart` de 1200 × 400, pintada no Skia da CPU sem janela. O custo de um
//! quadro tem de seguir a largura do gráfico (os pixels), não o tamanho da série, como no
//! DBGrid.
//!
//! Grava `medicoes/grafico_milhoes.json`:
//!
//! ```text
//! cargo test -p aylis-controls-charts --release --test huge_series -- --ignored --nocapture
//! ```
//!
//! `AYLIS_CHART_POINTS=10000,1000000` escolhe os tamanhos. Sem `--ignored`, a versão curta
//! confere que a linha reduzida desenha os mesmos extremos e que o número de operações não
//! cresce com a série.

use std::time::Instant;

use aylis_controls_charts::{Chart, ChartKind, Series};
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::*;
use serde_json::json;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 400.0;

/// Um sinal com ruído e picos raros (o pior caso da redução: o pico tem de aparecer).
fn signal(n: usize) -> Vec<f64> {
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    (0..n)
        .map(|i| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let noise = (seed % 1000) as f64 / 1000.0 - 0.5;
            // O pico tem sempre o mesmo valor: o eixo e o topo desenhado são os mesmos em
            // qualquer tamanho.
            if i % 97_003 == 50_000 {
                return 50.0;
            }
            (i as f64 / n as f64 * 12.0).sin() * 10.0 + noise
        })
        .collect()
}

fn chart(values: Vec<f64>, point: &str) -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    let c = ui.add_root(Chart::new(ChartKind::Line));
    ui.control_mut(c).unwrap().set_rect(0.0, 0.0, WIDTH, HEIGHT);
    ui.set_prop(c, "animate", false).unwrap();
    ui.set_prop(c, "legend", false).unwrap();
    let mut series = Series::new("sensor", values);
    series.point = match point {
        "none" => aylis_controls_charts::PointStyle::None,
        _ => aylis_controls_charts::PointStyle::Circle,
    };
    ui.get_mut::<Chart>(c).unwrap().set_data(Vec::new(), vec![series]);
    ui.layout();
    (ui, c)
}

fn record(ui: &mut Ui, c: ComponentId) -> Vec<Op> {
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(c, &mut canvas);
    canvas.ops
}

/// A maior e a menor coordenada Y desenhadas pelas linhas (o caminho da série).
fn path_extremes(ops: &[Op]) -> (usize, f32, f32) {
    let mut count = 0;
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for op in ops {
        if let Op::StrokePath { contours, .. } = op {
            for p in contours.iter().flat_map(|(c, _)| c) {
                count += 1;
                lo = lo.min(p.y);
                hi = hi.max(p.y);
            }
        }
    }
    (count, lo, hi)
}

/// A redução por pixel mantém os extremos (o pico aparece) e o desenho não cresce com a série.
#[test]
fn a_huge_line_keeps_the_extremes_with_few_operations() {
    let (mut small, c1) = chart(signal(200_000), "circle");
    let (mut big, c2) = chart(signal(2_000_000), "circle");
    let a = record(&mut small, c1);
    let b = record(&mut big, c2);
    let (pa, lo_a, hi_a) = path_extremes(&a);
    let (pb, lo_b, hi_b) = path_extremes(&b);
    // No máximo 4 pontos por coluna de pixel (o primeiro, o menor, o maior e o último).
    assert!(pa <= 4 * WIDTH as usize + 8 && pb <= 4 * WIDTH as usize + 8, "pontos no caminho: {pa} e {pb}");
    assert!(b.len() <= a.len() * 2, "operações: {} com 200 mil, {} com 2 milhões", a.len(), b.len());
    // O pico (50) chega ao mesmo topo nos dois.
    assert!(hi_a > lo_a && hi_b > lo_b);
    assert!((lo_a - lo_b).abs() < 2.0, "o topo desenhado: {lo_a} e {lo_b}");
}

fn ms(started: Instant) -> f64 {
    (started.elapsed().as_secs_f64() * 1e5).round() / 100.0
}

#[test]
#[ignore = "medição: rode em release (veja o cabeçalho)"]
fn huge_series_cost() {
    let sizes: Vec<usize> = std::env::var("AYLIS_CHART_POINTS")
        .ok()
        .map(|v| v.split(',').filter_map(|s| s.trim().parse().ok()).collect())
        .unwrap_or_else(|| vec![10_000, 100_000, 1_000_000, 10_000_000]);
    let renderer = aylis_skia::SkiaRenderer::new();
    let (w, h) = (WIDTH as u32, HEIGHT as u32);
    let mut pixels = vec![0u32; (w * h) as usize];
    let mut rows = Vec::new();
    for &n in &sizes {
        for point in ["circle", "none"] {
            let values = signal(n);
            let started = Instant::now();
            let (mut ui, c) = chart(values, point);
            let set_ms = ms(started);
            ui.services_mut().text = Some(Box::new(renderer.text_service()));
            // O primeiro quadro e a mediana de mais 5.
            let mut frames = Vec::new();
            for _ in 0..6 {
                let started = Instant::now();
                renderer.render_area_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, None, |canvas| ui.paint_region(c, canvas, None));
                frames.push(ms(started));
            }
            let first = frames[0];
            let mut rest = frames[1..].to_vec();
            rest.sort_by(f64::total_cmp);
            let ops = record(&mut ui, c).len();
            println!("{n} pontos ({point}): set_data {set_ms} ms, 1º quadro {first} ms, quadro {} ms, {ops} operações", rest[rest.len() / 2]);
            rows.push(json!({ "pontos": n, "marcador": point, "set_data_ms": set_ms, "primeiro_quadro_ms": first, "quadro_ms": rest[rest.len() / 2], "operacoes": ops }));
        }
    }
    let report = json!({
        "descricao": "O Chart de linha com uma série enorme (26/11): 1200 x 400, Skia na CPU sem janela, release, na máquina do autor. Um sinal com ruído e picos raros; 'circle' é o marcador padrão e 'none' só a linha. 'quadro_ms' é a mediana de 5 quadros depois do primeiro; 'operacoes', o que o gráfico manda ao Canvas. crates/aylis-controls-charts/tests/huge_series.rs.",
        "depois": rows,
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../../medicoes/grafico_milhoes.json");
    // Preserva o "antes" de uma rodada anterior (gravado à mão antes da mudança).
    let mut merged = std::fs::read_to_string(out).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).unwrap_or(json!({}));
    let label = std::env::var("AYLIS_CHART_LABEL").unwrap_or_else(|_| "depois".into());
    merged["descricao"] = report["descricao"].clone();
    merged[label] = report["depois"].clone();
    std::fs::write(out, serde_json::to_string_pretty(&merged).unwrap() + "\n").unwrap();
}
