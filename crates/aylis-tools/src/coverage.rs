//! `aylis-tool coverage-check`: a cobertura dos testes por crate contra a base em
//! `medicoes/cobertura.json` (item 38, 4a; decisão do autor de 08/11: cair mais de 1 ponto
//! reprova, como o `giant_screen`).
//!
//! ```text
//! aylis-tool coverage-check lcov.info --platform linux             # compara (sai com 1 se caiu)
//! aylis-tool coverage-check lcov.info --platform linux --update    # regrava a base dessa plataforma
//! ```
//!
//! A entrada é o `lcov` do `cargo llvm-cov --lcov` (o mesmo que a CI guarda como artefato). As
//! linhas de cada arquivo somam no crate da pasta (`crates/<crate>/...`); o que não está em
//! `crates/` fica de fora. A base tem uma tabela por plataforma, porque o Windows e o Linux
//! rodam testes diferentes.

use std::collections::BTreeMap;
use std::path::Path;

/// A base, relativa à raiz do repositório.
pub const BASELINE: &str = "medicoes/cobertura.json";

/// Quanto pode cair (em pontos percentuais) sem reprovar.
pub const TOLERANCE: f64 = 1.0;

/// As linhas de um crate: quantas têm código e quantas os testes passaram.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lines {
    pub found: u64,
    pub hit: u64,
}

impl Lines {
    pub fn percent(&self) -> f64 {
        if self.found == 0 { 100.0 } else { self.hit as f64 * 100.0 / self.found as f64 }
    }
}

/// O crate de um caminho (`.../crates/aylis-core/src/ui.rs` → `aylis-core`).
fn crate_of(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let i = path.rfind("/crates/").map(|i| i + 8).or_else(|| path.starts_with("crates/").then_some(7))?;
    let name = path[i..].split('/').next()?;
    (!name.is_empty()).then(|| name.to_string())
}

/// Soma o `lcov` por crate (as linhas `SF:`, `LF:` e `LH:` de cada arquivo).
pub fn parse_lcov(text: &str) -> BTreeMap<String, Lines> {
    let mut out: BTreeMap<String, Lines> = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(path) = line.strip_prefix("SF:") {
            current = crate_of(path);
        } else if let (Some(n), Some(name)) = (line.strip_prefix("LF:"), &current) {
            out.entry(name.clone()).or_default().found += n.trim().parse::<u64>().unwrap_or(0);
        } else if let (Some(n), Some(name)) = (line.strip_prefix("LH:"), &current) {
            out.entry(name.clone()).or_default().hit += n.trim().parse::<u64>().unwrap_or(0);
        } else if line == "end_of_record" {
            current = None;
        }
    }
    out
}

/// A comparação de um crate com a base.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub name: String,
    pub base: Option<f64>,
    pub now: Option<f64>,
}

impl Row {
    /// Caiu mais que a tolerância.
    pub fn failed(&self) -> bool {
        matches!((self.base, self.now), (Some(b), Some(n)) if b - n > TOLERANCE + 1e-9)
    }
}

/// Compara a medição com a base de uma plataforma: uma linha por crate das duas.
pub fn compare(now: &BTreeMap<String, Lines>, base: &BTreeMap<String, f64>) -> Vec<Row> {
    let mut names: Vec<&String> = now.keys().chain(base.keys()).collect();
    names.sort();
    names.dedup();
    names.into_iter().map(|n| Row { name: n.clone(), base: base.get(n).copied(), now: now.get(n).map(Lines::percent) }).collect()
}

/// A tabela em Markdown (o resumo da execução na CI).
pub fn report(platform: &str, rows: &[Row]) -> String {
    let fmt = |v: Option<f64>| v.map_or("—".to_string(), |v| format!("{v:.1}%"));
    let mut out = format!("## Cobertura dos testes ({platform})\n\n| Crate | Base | Agora | Diferença |\n|---|---:|---:|---:|\n");
    for r in rows {
        let diff = match (r.base, r.now) {
            (Some(b), Some(n)) => format!("{:+.1}{}", n - b, if r.failed() { " ❌" } else { "" }),
            (None, Some(_)) => "sem base".into(),
            (Some(_), None) => "não medido".into(),
            _ => String::new(),
        };
        out.push_str(&format!("| {} | {} | {} | {diff} |\n", r.name, fmt(r.base), fmt(r.now)));
    }
    let failed: Vec<&str> = rows.iter().filter(|r| r.failed()).map(|r| r.name.as_str()).collect();
    if failed.is_empty() {
        out.push_str(&format!("\nNenhum crate caiu mais de {TOLERANCE} ponto.\n"));
    } else {
        out.push_str(&format!("\nCaiu mais de {TOLERANCE} ponto: {}. Se foi de propósito, regrave a base (`aylis-tool coverage-check lcov.info --platform {platform} --update`) e diga o motivo nos pendentes.\n", failed.join(", ")));
    }
    out
}

/// A base de uma plataforma (vazia se ainda não há).
pub fn read_base(root: &Path, platform: &str) -> BTreeMap<String, f64> {
    let json = read_json(root);
    json["platforms"][platform].as_object().map(|m| m.iter().filter_map(|(k, v)| Some((k.clone(), v.as_f64()?))).collect()).unwrap_or_default()
}

fn read_json(root: &Path) -> serde_json::Value {
    std::fs::read_to_string(root.join(BASELINE)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_else(|| serde_json::json!({}))
}

/// Regrava a base de uma plataforma com a medição (as outras ficam).
pub fn write_base(root: &Path, platform: &str, now: &BTreeMap<String, Lines>) -> Result<(), String> {
    let mut json = read_json(root);
    if !json.is_object() {
        json = serde_json::json!({});
    }
    json["note"] = "Cobertura de linhas por crate (cargo llvm-cov), por plataforma da CI. Cair mais de 1 ponto reprova; aylis-tool coverage-check --update regrava.".into();
    let table: serde_json::Map<String, serde_json::Value> = now.iter().map(|(k, v)| (k.clone(), serde_json::json!((v.percent() * 10.0).round() / 10.0))).collect();
    if !json["platforms"].is_object() {
        json["platforms"] = serde_json::json!({});
    }
    json["platforms"][platform] = serde_json::Value::Object(table);
    let path = root.join(BASELINE);
    let text = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())? + "\n";
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// O `coverage-check`: lê o lcov, compara (ou regrava) e devolve o relatório; `Err` se algum
/// crate caiu mais que a tolerância.
pub fn check(root: &Path, lcov: &Path, platform: &str, update: bool) -> Result<String, String> {
    let text = std::fs::read_to_string(lcov).map_err(|e| format!("{}: {e}", lcov.display()))?;
    let now = parse_lcov(&text);
    if now.is_empty() {
        return Err(format!("{}: nenhum arquivo de crates/ no lcov", lcov.display()));
    }
    if update {
        write_base(root, platform, &now)?;
        return Ok(format!("base de {platform} regravada em {BASELINE} ({} crates)", now.len()));
    }
    let rows = compare(&now, &read_base(root, platform));
    let text = report(platform, &rows);
    if rows.iter().any(Row::failed) { Err(text) } else { Ok(text) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LCOV: &str = "SF:/home/runner/work/aylis/aylis/crates/aylis-core/src/ui.rs\nDA:1,1\nLF:100\nLH:80\nend_of_record\nSF:D:\\a\\aylis\\aylis\\crates\\aylis-core\\src\\layout.rs\nLF:100\nLH:60\nend_of_record\nSF:crates/aylis-json/src/lib.rs\nLF:10\nLH:10\nend_of_record\nSF:/rustc/abc/library/std/src/lib.rs\nLF:50\nLH:0\nend_of_record\n";

    #[test]
    fn the_lcov_is_summed_by_crate() {
        let m = parse_lcov(LCOV);
        assert_eq!(m.len(), 2);
        assert_eq!(m["aylis-core"], Lines { found: 200, hit: 140 });
        assert_eq!(m["aylis-json"].percent(), 100.0);
    }

    #[test]
    fn falling_more_than_one_point_fails() {
        let now = parse_lcov(LCOV);
        let base: BTreeMap<String, f64> = [("aylis-core".to_string(), 70.9), ("aylis-json".to_string(), 100.0), ("aylis-data".to_string(), 50.0)].into();
        let rows = compare(&now, &base);
        assert!(!rows.iter().any(Row::failed), "70.9 → 70.0 é menos de 1 ponto");
        let base: BTreeMap<String, f64> = [("aylis-core".to_string(), 71.2)].into();
        let rows = compare(&now, &base);
        assert_eq!(rows.iter().filter(|r| r.failed()).map(|r| r.name.as_str()).collect::<Vec<_>>(), ["aylis-core"]);
        let text = report("linux", &rows);
        assert!(text.contains("| aylis-core | 71.2% | 70.0% | -1.2 ❌ |"), "{text}");
        assert!(text.contains("| aylis-json | — | 100.0% | sem base |"), "{text}");
    }

    #[test]
    fn the_base_is_kept_per_platform() {
        let root = std::env::temp_dir().join(format!("aylis_coverage_{}", std::process::id()));
        std::fs::create_dir_all(root.join("medicoes")).unwrap();
        let lcov = root.join("lcov.info");
        std::fs::write(&lcov, LCOV).unwrap();
        check(&root, &lcov, "linux", true).unwrap();
        check(&root, &lcov, "windows", true).unwrap();
        assert_eq!(read_base(&root, "linux")["aylis-core"], 70.0);
        assert!(check(&root, &lcov, "linux", false).is_ok());
        // Caiu: o core com metade das linhas.
        std::fs::write(&lcov, LCOV.replace("LH:80", "LH:20")).unwrap();
        let err = check(&root, &lcov, "linux", false).unwrap_err();
        assert!(err.contains("Caiu mais de 1 ponto: aylis-core"), "{err}");
        assert_eq!(read_base(&root, "windows").len(), 2);
        let _ = std::fs::remove_dir_all(&root);
    }
}
