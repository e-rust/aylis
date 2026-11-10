//! A tela gigante (19.200 × 6.480, quase 18 mil componentes) não pode piorar: o resultado é
//! comparado com a base em `medicoes/giant_screen.json`.
//!
//! - A área repintada e quantos controles cada quadro pinta não dependem da máquina: valem
//!   sempre, também em debug.
//! - Os tempos valem em release (`cargo test --release -p aylis-controls --test giant_screen`),
//!   com folga para o ruído da máquina.
//!
//! Melhorou de propósito (ou a base é de outra máquina): `AYLIS_UPDATE_BENCH=1` em release
//! regrava a base, e o arquivo entra no commit.

#[path = "../examples/giant_screen.rs"]
mod giant_screen;

use serde_json::{Value, json};

/// Quanto pode passar da base: a área e os controles quase nada (são exatos); o tempo, a
/// variação normal de uma máquina carregada.
const AREA_SLACK: f64 = 1.10;
const PAINTED_SLACK: f64 = 1.10;
const TIME_SLACK: f64 = 1.5;
const TIME_FLOOR_MS: f64 = 1.0;

fn baseline_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../medicoes/giant_screen.json")
}

fn to_json(report: &giant_screen::Report) -> Value {
    let scenarios: Vec<Value> = report
        .scenarios
        .iter()
        .map(|s| json!({ "nome": s.name, "evento_ms": round(s.event_ms), "quadro_ms": round(s.frame_ms), "area": s.area, "controles": s.painted }))
        .collect();
    json!({
        "descricao": "A tela gigante do item 5 (exemplo e teste giant_screen): 19.200 × 6.480 pixels, Skia na CPU, sem janela. Tempos em ms (mediana), medidos em release na máquina do autor; área = fração da tela repintada; controles = pintados por quadro. O teste não aceita passar disto (com folga nos tempos).",
        "largura": giant_screen::WIDTH,
        "altura": giant_screen::HEIGHT,
        "componentes": report.controls,
        "pintura_inteira_ms": round(report.full_ms),
        "pintura_inteira_controles": report.full_painted,
        "cenarios": scenarios,
    })
}

fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

#[test]
fn the_giant_screen_does_not_get_slower() {
    let release = !cfg!(debug_assertions);
    // Em debug, menos quadros (só a área e os controles contam).
    let report = giant_screen::measure(giant_screen::WIDTH, giant_screen::HEIGHT, if release { 3 } else { 1 }, if release { 40 } else { 8 });
    giant_screen::print(&report, giant_screen::WIDTH, giant_screen::HEIGHT);
    let path = baseline_path();
    if std::env::var("AYLIS_UPDATE_BENCH").is_ok() {
        assert!(release, "a base tem os tempos de release: rode com --release");
        std::fs::write(&path, serde_json::to_string_pretty(&to_json(&report)).unwrap() + "\n").unwrap();
        println!("base regravada em {}", path.display());
        return;
    }
    let base: Value = serde_json::from_str(&std::fs::read_to_string(&path).expect("a base (medicoes/giant_screen.json)")).unwrap();
    let mut failures = Vec::new();
    if base["componentes"].as_u64() != Some(report.controls as u64) {
        failures.push(format!("a tela mudou: {} componentes, a base tem {} (regrave a base)", report.controls, base["componentes"]));
    }
    if release {
        let limit = base["pintura_inteira_ms"].as_f64().unwrap() * TIME_SLACK + TIME_FLOOR_MS;
        if report.full_ms > limit {
            failures.push(format!("pintura inteira: {:.1} ms (base {}, limite {limit:.1})", report.full_ms, base["pintura_inteira_ms"]));
        }
    }
    for s in &report.scenarios {
        let Some(b) = base["cenarios"].as_array().and_then(|list| list.iter().find(|b| b["nome"] == s.name)) else {
            failures.push(format!("{}: sem base (regrave a base)", s.name));
            continue;
        };
        let area = b["area"].as_f64().unwrap() * AREA_SLACK + 1e-6;
        if s.area > area {
            failures.push(format!("{}: área {:.4}% (base {:.4}%)", s.name, s.area * 100.0, b["area"].as_f64().unwrap() * 100.0));
        }
        let painted = (b["controles"].as_f64().unwrap() * PAINTED_SLACK).ceil() + 2.0;
        if s.painted as f64 > painted {
            failures.push(format!("{}: {} controles pintados por quadro (base {})", s.name, s.painted, b["controles"]));
        }
        if release {
            let limit = b["quadro_ms"].as_f64().unwrap() * TIME_SLACK + TIME_FLOOR_MS;
            if s.frame_ms > limit {
                failures.push(format!("{}: quadro {:.2} ms (base {}, limite {limit:.2})", s.name, s.frame_ms, b["quadro_ms"]));
            }
            let limit = b["evento_ms"].as_f64().unwrap() * TIME_SLACK + TIME_FLOOR_MS;
            if s.event_ms > limit {
                failures.push(format!("{}: evento {:.2} ms (base {}, limite {limit:.2})", s.name, s.event_ms, b["evento_ms"]));
            }
        }
    }
    assert!(failures.is_empty(), "a tela gigante piorou em relação à base (medicoes/giant_screen.json):\n{}", failures.join("\n"));
}
