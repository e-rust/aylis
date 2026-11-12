//! Mede o tempo de quadro dos exemplos (`AYLIS_BENCH`) e grava um JSON para comparar depois.
//!
//! ```text
//! cargo build --release -p aylis-platform-winit --features d2d --examples
//! aylis-tool bench base.json
//! aylis-tool bench depois.json --compare base.json
//! aylis-tool bench d2d.json --renderers d2d
//! ```
//!
//! Cada exemplo roda [`ROUNDS`] vezes com cada renderizador; vale a mediana das rodadas.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};

pub const EXAMPLES: [&str; 5] = ["components", "grid", "modern", "catalog", "gallery"];
pub const RENDERERS: [&str; 2] = ["d2d", "gpu"];
const FRAMES: u32 = 600;
pub const ROUNDS: usize = 3;
/// Os campos da linha `AYLIS_BENCH` que entram na medida.
const FIELDS: [&str; 4] = ["media_ms", "p50_ms", "p95_ms", "memoria_privada_mb"];

/// Os valores `chave=valor` da linha `AYLIS_BENCH` da saída.
fn parse_line(output: &str) -> Option<BTreeMap<String, f64>> {
    let line = output.lines().find(|l| l.starts_with("AYLIS_BENCH"))?;
    let values: BTreeMap<String, f64> = line.split_whitespace().filter_map(|w| w.split_once('=')).filter_map(|(k, v)| Some((k.to_string(), v.trim_matches('"').parse().ok()?))).collect();
    FIELDS.iter().all(|f| values.contains_key(*f)).then_some(values)
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}

/// Mede um exemplo com um renderizador (a mediana das rodadas); `None` se não rodou.
fn measure(root: &Path, example: &str, renderer: &str) -> Option<BTreeMap<&'static str, f64>> {
    let exe = crate::cargo_target_dir(root).join("release").join("examples").join(crate::env::exe(example));
    if !exe.is_file() {
        return None;
    }
    let mut rounds = Vec::new();
    for _ in 0..ROUNDS {
        let out = Command::new(&exe).env("AYLIS_RENDERER", renderer).env("AYLIS_BENCH", FRAMES.to_string()).current_dir(root).output().ok()?;
        if let Some(values) = parse_line(&String::from_utf8_lossy(&out.stdout)) {
            rounds.push(values);
        }
    }
    if rounds.is_empty() {
        return None;
    }
    Some(FIELDS.iter().map(|&f| (f, median(rounds.iter().map(|r| r[f]).collect()))).collect())
}

/// Mede todos, mostra (comparando com `compare`, se houver) e grava em `dest`.
pub fn run(root: &Path, dest: &Path, renderers: &[String], compare: Option<&Path>) -> Result<(), String> {
    let base: Option<Value> = match compare {
        Some(p) => Some(serde_json::from_str(&std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?).map_err(|e| e.to_string())?),
        None => None,
    };
    let mut result = serde_json::Map::new();
    for example in EXAMPLES {
        for renderer in renderers {
            let Some(m) = measure(root, example, renderer) else { continue };
            let key = format!("{example}/{renderer}");
            let mut text = format!("{key:22} média {:6.3} ms  p50 {:6.3}  p95 {:6.3}  memória {:6.1} MB", m["media_ms"], m["p50_ms"], m["p95_ms"], m["memoria_privada_mb"]);
            if let Some(before) = base.as_ref().and_then(|b| b[&key]["media_ms"].as_f64()) {
                text.push_str(&format!("   antes {before:6.3} ms ({:+.1}%)", (m["media_ms"] / before - 1.0) * 100.0));
            }
            println!("{text}");
            result.insert(key, json!(m));
        }
    }
    std::fs::write(dest, serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?).map_err(|e| format!("{}: {e}", dest.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_bench_line_and_takes_the_median() {
        let out = "x\nAYLIS_BENCH renderizador=d2d em_uso=\"Direct2D\" quadros=600 media_ms=1.250 p50_ms=1.100 p95_ms=2.000 fps=800 memoria_privada_mb=55.5 working_set_mb=80.0\n";
        let v = parse_line(out).unwrap();
        assert_eq!(v["media_ms"], 1.25);
        assert_eq!(v["memoria_privada_mb"], 55.5);
        assert!(parse_line("sem medida").is_none());
        assert_eq!(median(vec![3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(vec![4.0, 1.0, 2.0, 3.0]), 2.5);
    }
}
