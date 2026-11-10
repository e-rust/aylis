//! O relato de problema (`.aylisreport`, um ZIP): o `report.json` com o problema, quem relatou,
//! a máquina e o projeto, e os anexos em `files/` (a captura da tela, a gravação de uso, o
//! form e o projeto). A IDE grava pelo Ajuda → Relatar problema… e abre com dois cliques.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

pub const EXTENSION: &str = "aylisreport";
/// A versão do formato (`"aylis_report"` no `report.json`).
pub const FORMAT: u64 = 1;
const INFO: &str = "report.json";
const FILES: &str = "files/";

/// O que o usuário escreveu.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Problem {
    pub what: String,
    pub expected: String,
    pub steps: String,
}

/// Quem relatou (opcional: para o autor responder).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reporter {
    pub name: String,
    pub contact: String,
}

#[derive(Clone, Debug, Default)]
pub struct Report {
    pub problem: Problem,
    pub reporter: Reporter,
    /// Quando foi gerado (milissegundos desde 1970).
    pub created_ms: u64,
    /// A máquina: [`machine_info`] mais o que a IDE acrescenta (tela, idioma, renderizador).
    pub machine: Map<String, Value>,
    /// O projeto aberto (nome, linguagem, forms, pacotes...); `None` sem projeto.
    pub project: Option<Map<String, Value>>,
    /// Os anexos: (nome, bytes).
    pub files: Vec<(String, Vec<u8>)>,
}

/// O sistema, o processador e a memória desta máquina, e a versão do aylis.
#[cfg(target_arch = "wasm32")]
pub fn machine_info() -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("aylis".into(), json!(env!("CARGO_PKG_VERSION")));
    m.insert("os".into(), json!("navegador"));
    m.insert("arch".into(), json!(std::env::consts::ARCH));
    m
}

/// O sistema, o processador e a memória desta máquina, e a versão do aylis.
#[cfg(not(target_arch = "wasm32"))]
pub fn machine_info() -> Map<String, Value> {
    use sysinfo::System;
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_all();
    let mb = |bytes: u64| bytes / (1024 * 1024);
    let mut m = Map::new();
    m.insert("aylis".into(), json!(env!("CARGO_PKG_VERSION")));
    m.insert("os".into(), json!(System::long_os_version().unwrap_or_else(|| std::env::consts::OS.to_string())));
    m.insert("os_build".into(), json!(System::os_version().unwrap_or_default()));
    m.insert("kernel".into(), json!(System::kernel_version().unwrap_or_default()));
    m.insert("arch".into(), json!(std::env::consts::ARCH));
    m.insert("cpu".into(), json!(sys.cpus().first().map(|c| c.brand().trim().to_string()).unwrap_or_default()));
    m.insert("cpu_threads".into(), json!(sys.cpus().len()));
    m.insert("memory_mb".into(), json!(mb(sys.total_memory())));
    m.insert("memory_free_mb".into(), json!(mb(sys.available_memory())));
    m
}

/// Grava o relato em `path`.
pub fn write(path: &Path, report: &Report) -> Result<(), String> {
    let info = json!({
        "aylis_report": FORMAT,
        "created_ms": report.created_ms,
        "problem": { "what": report.problem.what, "expected": report.problem.expected, "steps": report.problem.steps },
        "reporter": { "name": report.reporter.name, "contact": report.reporter.contact },
        "machine": report.machine,
        "project": report.project,
        "files": report.files.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
    });
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let mut z = zip::ZipWriter::new(std::fs::File::create(path).map_err(|e| e.to_string())?);
    let opts = zip::write::SimpleFileOptions::default();
    z.start_file(INFO, opts).map_err(|e| e.to_string())?;
    z.write_all(serde_json::to_string_pretty(&info).map_err(|e| e.to_string())?.as_bytes()).map_err(|e| e.to_string())?;
    for (name, bytes) in &report.files {
        z.start_file(format!("{FILES}{}", safe_name(name)), opts).map_err(|e| e.to_string())?;
        z.write_all(bytes).map_err(|e| e.to_string())?;
    }
    z.finish().map_err(|e| e.to_string())?;
    Ok(())
}

/// Só o nome do arquivo (sem pastas nem `..`), para o anexo não sair da pasta ao extrair.
fn safe_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let base = base.trim_start_matches('.');
    if base.is_empty() { "anexo".into() } else { base.to_string() }
}

fn text(v: &Value) -> String {
    v.as_str().unwrap_or_default().to_string()
}

/// Lê um `.aylisreport`.
pub fn read(path: &Path) -> Result<Report, String> {
    let f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(f).map_err(|e| format!("não é um .{EXTENSION}: {e}"))?;
    let info: Value = {
        let mut entry = zip.by_name(INFO).map_err(|_| format!("não é um .{EXTENSION}: falta o {INFO}"))?;
        let mut s = String::new();
        entry.read_to_string(&mut s).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).map_err(|e| format!("{INFO}: {e}"))?
    };
    let format = info["aylis_report"].as_u64().ok_or(format!("não é um .{EXTENSION}"))?;
    if format > FORMAT {
        return Err(format!("relato de uma versão mais nova do Aylis (formato {format})"));
    }
    let mut files = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        let full = entry.name().map(|n| n.to_string()).map_err(|e| e.to_string())?;
        let Some(name) = full.strip_prefix(FILES).map(safe_name) else { continue };
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        files.push((name, bytes));
    }
    let p = &info["problem"];
    let r = &info["reporter"];
    Ok(Report {
        problem: Problem { what: text(&p["what"]), expected: text(&p["expected"]), steps: text(&p["steps"]) },
        reporter: Reporter { name: text(&r["name"]), contact: text(&r["contact"]) },
        created_ms: info["created_ms"].as_u64().unwrap_or(0),
        machine: info["machine"].as_object().cloned().unwrap_or_default(),
        project: info["project"].as_object().cloned(),
        files,
    })
}

/// Grava os anexos em `dir`; devolve os caminhos.
pub fn extract(report: &Report, dir: &Path) -> Result<Vec<PathBuf>, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for (name, bytes) in &report.files {
        let path = dir.join(safe_name(name));
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        out.push(path);
    }
    Ok(out)
}

fn value_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(value_text).collect::<Vec<_>>().join(", "),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn section(out: &mut String, title: &str, map: &Map<String, Value>) {
    out.push_str(&format!("\n{title}\n"));
    for (k, v) in map {
        let t = value_text(v);
        if !t.is_empty() {
            out.push_str(&format!("  {k}: {t}\n"));
        }
    }
}

/// O relato em texto, para ler (o visualizador da IDE e o "Ver o que será enviado").
/// `created` é a data já formatada.
pub fn summary(report: &Report, created: &str) -> String {
    let mut out = String::new();
    let p = &report.problem;
    out.push_str(&format!("O que aconteceu:\n  {}\n", p.what.replace('\n', "\n  ")));
    if !p.expected.trim().is_empty() {
        out.push_str(&format!("\nO que esperava:\n  {}\n", p.expected.replace('\n', "\n  ")));
    }
    if !p.steps.trim().is_empty() {
        out.push_str(&format!("\nComo reproduzir:\n  {}\n", p.steps.replace('\n', "\n  ")));
    }
    let r = &report.reporter;
    if !r.name.trim().is_empty() || !r.contact.trim().is_empty() {
        out.push_str(&format!("\nQuem relatou: {} {}\n", r.name.trim(), if r.contact.trim().is_empty() { String::new() } else { format!("<{}>", r.contact.trim()) }));
    }
    if !created.is_empty() {
        out.push_str(&format!("Gerado em: {created}\n"));
    }
    section(&mut out, "Máquina", &report.machine);
    match &report.project {
        Some(p) => section(&mut out, "Projeto", p),
        None => out.push_str("\nProjeto: nenhum aberto\n"),
    }
    if !report.files.is_empty() {
        out.push_str("\nAnexos\n");
        for (n, b) in &report.files {
            out.push_str(&format!("  {n} ({} KB)\n", b.len().div_ceil(1024)));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_report_round_trips_with_machine_project_and_attachments() {
        let dir = std::env::temp_dir().join(format!("aylis-report-{}", std::process::id()));
        let path = dir.join("r.aylisreport");
        let mut project = Map::new();
        project.insert("name".into(), json!("Vendas"));
        project.insert("forms".into(), json!(["frmMain", "frmClientes"]));
        let report = Report {
            problem: Problem { what: "o botão some".into(), expected: "continuar visível".into(), steps: "1. abrir\n2. clicar".into() },
            reporter: Reporter { name: "Ana".into(), contact: "ana@exemplo.com".into() },
            created_ms: 1_700_000_000_000,
            machine: machine_info(),
            project: Some(project),
            files: vec![("tela.png".into(), vec![1, 2, 3]), ("../../fora.txt".into(), b"x".to_vec())],
        };
        assert!(report.machine["memory_mb"].as_u64().unwrap() > 0);
        assert!(!report.machine["os"].as_str().unwrap().is_empty());
        write(&path, &report).unwrap();
        let back = read(&path).unwrap();
        assert_eq!(back.problem, report.problem);
        assert_eq!(back.reporter, report.reporter);
        assert_eq!(back.created_ms, report.created_ms);
        assert_eq!(back.machine, report.machine);
        assert_eq!(back.project.as_ref().unwrap()["name"], "Vendas");
        // O anexo com "..": fica só o nome, dentro da pasta.
        assert_eq!(back.files.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["tela.png", "fora.txt"]);
        let out = extract(&back, &dir.join("anexos")).unwrap();
        assert!(out.iter().all(|p| p.starts_with(dir.join("anexos"))));
        let text = summary(&back, "01/11/2026 10:00");
        for want in ["o botão some", "Como reproduzir", "ana@exemplo.com", "Máquina", "frmMain, frmClientes", "tela.png"] {
            assert!(text.contains(want), "falta {want:?} em\n{text}");
        }
        // Outro ZIP qualquer não é relato.
        let other = dir.join("o.aylisreport");
        let mut z = zip::ZipWriter::new(std::fs::File::create(&other).unwrap());
        z.start_file("x.txt", zip::write::SimpleFileOptions::default()).unwrap();
        z.finish().unwrap();
        assert!(read(&other).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
