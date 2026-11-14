//! `aylis-tool handler-times`: os tempos dos handlers dos exemplos (o rastro dos roteiros, com
//! `"profile"`; veja `aylis_remote::script::play_profiled`) contra a base em
//! `medicoes/handler_times.json` (item 38, 3b). Decisão do autor de 08/11: na CI o tempo só
//! AVISA (as máquinas da CI variam mais que a do autor); quem reprova é o limite que o próprio
//! roteiro pede (`"max_handler_ms"`).
//!
//! ```text
//! aylis-tool handler-times medicoes/handler_times.json calculadora.handlers.json paint.handlers.json [--summary arquivo.md]
//! aylis-tool handler-times medicoes/handler_times.json *.handlers.json --update
//! ```
//!
//! Cada arquivo é um exemplo (o nome antes de `.handlers.json`). A média de um handler que
//! passar de 1,5 vez a da base, e por mais de 2 ms (o ruído de handler rápido), vira aviso
//! (`::warning::` na CI).

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

/// Quantas vezes a média da base vira aviso.
pub const FACTOR: f64 = 1.5;
/// Abaixo disso de diferença (ms), não avisa.
pub const FLOOR_MS: f64 = 2.0;

/// O exemplo de um arquivo de tempos (`calculadora.handlers.json` → `calculadora`).
pub fn app_of(path: &Path) -> String {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    name.strip_suffix(".handlers.json").or_else(|| name.strip_suffix(".json")).unwrap_or(name).to_string()
}

/// Um aviso: o exemplo, o handler, a média da base e a de agora.
#[derive(Clone, Debug, PartialEq)]
pub struct Slower {
    pub app: String,
    pub handler: String,
    pub base_ms: f64,
    pub now_ms: f64,
}

/// Compara os tempos de um exemplo (`{"handlers": {...}}`) com a base dele.
pub fn compare(app: &str, now: &Value, base: &Value) -> Vec<Slower> {
    let Some(handlers) = now["handlers"].as_object() else { return Vec::new() };
    handlers
        .iter()
        .filter_map(|(name, t)| {
            let now_ms = t["mean_ms"].as_f64()?;
            let base_ms = base[name]["mean_ms"].as_f64()?;
            (now_ms > base_ms * FACTOR && now_ms - base_ms > FLOOR_MS).then(|| Slower { app: app.to_string(), handler: name.clone(), base_ms, now_ms })
        })
        .collect()
}

/// O relatório (Markdown) e as linhas de aviso da CI.
pub fn report(slower: &[Slower], measured: &BTreeMap<String, Value>, base: &Value) -> String {
    let mut out = String::from("## Tempos dos handlers dos exemplos\n\n");
    for (app, now) in measured {
        let count = now["handlers"].as_object().map_or(0, |h| h.len());
        let known = base["apps"][app].as_object().map_or(0, |h| h.len());
        out.push_str(&format!("- {app}: {count} handlers medidos ({})\n", if known == 0 { "sem base".to_string() } else { format!("{known} na base") }));
    }
    if slower.is_empty() {
        out.push_str(&format!("\nNenhum handler passou de {FACTOR} vez a média da base.\n"));
    } else {
        out.push_str("\n| Exemplo | Handler | Base (média) | Agora (média) |\n|---|---|---:|---:|\n");
        for s in slower {
            out.push_str(&format!("| {} | {} | {:.2} ms | {:.2} ms |\n", s.app, s.handler, s.base_ms, s.now_ms));
        }
        out.push_str("\nSó aviso (decisão de 08/11). Mais lento de propósito: regrave a base com `--update`.\n");
    }
    out
}

/// O `handler-times`: lê os arquivos, compara (ou regrava a base) e devolve o relatório e as
/// linhas `::warning::`. Erro só de leitura: o tempo não reprova.
pub fn check(base_path: &Path, files: &[std::path::PathBuf], update: bool) -> Result<(String, Vec<String>), String> {
    let mut measured = BTreeMap::new();
    for f in files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("{}: {e}", f.display()))?;
        let value: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", f.display()))?;
        measured.insert(app_of(f), value);
    }
    let mut base: Value = std::fs::read_to_string(base_path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_else(|| json!({}));
    if update {
        if !base.is_object() {
            base = json!({});
        }
        base["note"] = json!("Tempos dos handlers dos exemplos (roteiros com \"profile\", na CI). Só avisam: média acima de 1,5 vez a daqui e por mais de 2 ms. aylis-tool handler-times --update regrava.");
        if !base["apps"].is_object() {
            base["apps"] = json!({});
        }
        for (app, now) in &measured {
            base["apps"][app] = now["handlers"].clone();
        }
        let text = serde_json::to_string_pretty(&base).map_err(|e| e.to_string())? + "\n";
        std::fs::write(base_path, text).map_err(|e| format!("{}: {e}", base_path.display()))?;
        return Ok((format!("base regravada em {} ({} exemplos)", base_path.display(), measured.len()), Vec::new()));
    }
    let slower: Vec<Slower> = measured.iter().flat_map(|(app, now)| compare(app, now, &base["apps"][app])).collect();
    let warnings = slower.iter().map(|s| format!("::warning title=Handler mais lento::{} {}: {:.2} ms (base {:.2} ms)", s.app, s.handler, s.now_ms, s.base_ms)).collect();
    Ok((report(&slower, &measured, &base), warnings))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn times(mean: f64) -> Value {
        json!({"handlers": {"frmCalc.btnIgualClick": {"count": 3, "mean_ms": mean, "p95_ms": mean, "max_ms": mean, "total_ms": mean * 3.0}}})
    }

    #[test]
    fn only_a_clearly_slower_handler_warns() {
        let base = times(4.0)["handlers"].clone();
        assert!(compare("calc", &times(5.9), &base).is_empty(), "abaixo de 1,5 vez");
        assert!(compare("calc", &times(6.5), &base).len() == 1);
        let fast = times(0.5)["handlers"].clone();
        assert!(compare("calc", &times(2.0), &fast).is_empty(), "4 vezes, mas menos de 2 ms");
        assert!(compare("calc", &times(9.0), &json!(null)).is_empty(), "sem base, sem aviso");
    }

    #[test]
    fn the_base_is_written_and_read_per_app() {
        let dir = std::env::temp_dir().join(format!("aylis_handler_times_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("calculadora.handlers.json");
        let base = dir.join("handler_times.json");
        std::fs::write(&file, times(4.0).to_string()).unwrap();
        check(&base, std::slice::from_ref(&file), true).unwrap();
        let (text, warnings) = check(&base, std::slice::from_ref(&file), false).unwrap();
        assert!(warnings.is_empty() && text.contains("calculadora: 1 handlers medidos (1 na base)"), "{text}");
        std::fs::write(&file, times(40.0).to_string()).unwrap();
        let (text, warnings) = check(&base, &[file], false).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].starts_with("::warning") && text.contains("| calculadora | frmCalc.btnIgualClick | 4.00 ms | 40.00 ms |"), "{text}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
