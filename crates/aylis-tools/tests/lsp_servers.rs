//! Os servidores de linguagem de verdade, um por linguagem, na Calculadora: abrem, recebem o
//! fonte do form e respondem aos símbolos, à dica e às sugestões. Demora (o Java e o Rust
//! indexam o projeto): `cargo test -p aylis-tools --test lsp_servers -- --ignored --nocapture`.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use aylis_project::Language;
use aylis_tools::lsp::{self, Client, Incoming};
use serde_json::{Value, json};

/// Espera a resposta do pedido `id` (os avisos de erro que chegarem no meio vão contando).
fn wait(client: &mut Client, id: i64, secs: u64, diags: &mut usize) -> Option<Result<Value, String>> {
    let deadline = Instant::now() + Duration::from_secs(secs);
    while Instant::now() < deadline {
        for msg in client.poll() {
            match msg {
                Incoming::Response { id: got, result } if got == id => return Some(result),
                Incoming::Diagnostics { items, .. } => *diags += items.len(),
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    None
}

/// Pede `method` até vir algo que não seja vazio (o servidor ainda indexando), até `secs`.
fn ask(client: &mut Client, method: &str, params: &Value, secs: u64, diags: &mut usize, empty: impl Fn(&Value) -> bool) -> Option<Result<Value, String>> {
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut last = None;
    while Instant::now() < deadline {
        let id = client.request(method, params.clone()).ok()?;
        let left = deadline.saturating_duration_since(Instant::now()).as_secs().max(1);
        match wait(client, id, left, diags) {
            Some(Ok(v)) if !empty(&v) => return Some(Ok(v)),
            Some(Err(e)) if e.contains("not found") => return Some(Err(e)),
            other => last = other,
        }
        std::thread::sleep(Duration::from_millis(1500));
    }
    last
}

/// O fonte do form e uma palavra que existe nele (para a dica).
fn source(language: Language, dir: &Path) -> (PathBuf, &'static str) {
    let src = dir.join("src");
    match language {
        Language::Rust => (src.join("forms").join("calculadora.rs"), "apertar"),
        Language::Python => (src.join("forms").join("calculadora.py"), "apertar"),
        Language::TypeScript => (src.join("forms").join("calculadora.ts"), "apertar"),
        Language::CSharp => (src.join("forms").join("FrmCalculadora.cs"), "Apertar"),
        Language::Java => (src.join("forms").join("FrmCalculadora.java"), "apertar"),
        Language::Go => (src.join("forms").join("calculadora.go"), "apertar"),
        Language::Cpp => (src.join("forms").join("calculadora.cpp"), "apertar"),
        Language::Pascal => (src.join("forms").join("calculadora.pas"), "Apertar"),
    }
}

#[test]
#[ignore]
fn every_language_server_answers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Calculadora");
    let only = std::env::var("AYLIS_LSP_ONLY").ok();
    let mut failed = Vec::new();
    for language in Language::ALL {
        if only.as_deref().is_some_and(|o| !language.folder().eq_ignore_ascii_case(o)) {
            continue;
        }
        let dir = std::fs::canonicalize(root.join(language.folder())).unwrap();
        let dir = PathBuf::from(dir.display().to_string().trim_start_matches(r"\\?\"));
        let Some(server) = lsp::server(language, &dir) else {
            println!("{}: sem servidor instalado", language.name());
            failed.push(language.name());
            continue;
        };
        lsp::prepare(language, &dir);
        let started = Instant::now();
        let mut client = match Client::start(&server, &dir, None) {
            Ok(c) => c,
            Err(e) => {
                println!("{}: {e}", language.name());
                failed.push(language.name());
                continue;
            }
        };
        let (file, word) = source(language, &dir);
        let text = std::fs::read_to_string(&file).unwrap();
        client.open(&file, &text).unwrap();
        let mut diags = 0;
        // Os símbolos do documento.
        let symbols = match ask(&mut client, "textDocument/documentSymbol", &json!({ "textDocument": { "uri": lsp::uri(&file) } }), 120, &mut diags, |v| lsp::symbols(v).is_empty()) {
            Some(Ok(v)) => lsp::symbols(&v),
            Some(Err(e)) => {
                println!("   símbolos: erro {e}");
                Vec::new()
            }
            None => Vec::new(),
        };
        // A dica e a definição na primeira chamada da palavra (não a declaração).
        let index = {
            let lower = text.to_lowercase();
            let w = word.to_lowercase();
            let first = lower.find(&format!("{w}(")).unwrap_or(0);
            let second = lower[first + 1..].find(&format!("{w}(")).map_or(first, |i| first + 1 + i);
            text[..second].chars().count() + 1
        };
        let at = lsp::at(&file, lsp::position_in(&text, index));
        let hover = match ask(&mut client, "textDocument/hover", &at, 90, &mut diags, |v| lsp::hover_text(v).is_empty()) {
            Some(Ok(v)) => lsp::hover_text(&v),
            Some(Err(e)) => format!("erro {e}"),
            None => "sem resposta".into(),
        };
        let definition = match ask(&mut client, "textDocument/definition", &at, 30, &mut diags, |v| lsp::locations(v).is_empty()) {
            Some(Ok(v)) => lsp::locations(&v).first().map(|l| format!("{}:{}", l.file.file_name().unwrap().to_string_lossy(), l.start.0 + 1)).unwrap_or_else(|| "nenhuma".into()),
            Some(Err(e)) => format!("erro {e}"),
            None => "sem resposta".into(),
        };
        // As sugestões depois de "frmCalculadora." (ou só no começo da palavra).
        let completions = match ask(&mut client, "textDocument/completion", &at, 30, &mut diags, |v| lsp::completions(v).is_empty()) {
            Some(Ok(v)) => lsp::completions(&v).len(),
            _ => 0,
        };
        println!(
            "{} ({}, {:.1} s): {} símbolos ({}), definição {definition}, {completions} sugestões, {diags} avisos; dica: {}",
            language.name(),
            server.name,
            started.elapsed().as_secs_f32(),
            symbols.len(),
            symbols.iter().take(3).map(|s| s.name.as_str()).collect::<Vec<_>>().join(", "),
            hover.lines().next().unwrap_or("").chars().take(70).collect::<String>()
        );
        if symbols.is_empty() && hover.is_empty() {
            failed.push(language.name());
        }
        client.stop();
    }
    assert!(failed.is_empty(), "falharam: {failed:?}");
}

/// O progresso do servidor de verdade (07/11): o rust-analyzer avisa pelo `$/progress` que está
/// carregando e indexando o projeto, e termina. O que a IDE mostra no rótulo, em ordem:
/// `cargo test -p aylis-tools --test lsp_servers progress -- --ignored --nocapture`.
#[test]
#[ignore]
fn the_rust_server_reports_its_progress_until_ready() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Calculadora/rust");
    let dir = std::fs::canonicalize(root).unwrap();
    let dir = PathBuf::from(dir.display().to_string().trim_start_matches(r"\?\"));
    let Some(server) = lsp::server(Language::Rust, &dir) else {
        println!("sem rust-analyzer instalado");
        return;
    };
    lsp::prepare(Language::Rust, &dir);
    let started = Instant::now();
    let mut client = Client::start(&server, &dir, None).expect("o rust-analyzer abre");
    println!("{:.1} s: initialize respondido", started.elapsed().as_secs_f32());
    let (file, _) = source(Language::Rust, &dir);
    client.open(&file, &std::fs::read_to_string(&file).unwrap()).unwrap();
    let mut seen = Vec::new();
    let mut busy_seen = false;
    // Até 15 s sem novidade depois de pronto (as fases vêm uma atrás da outra: buscar os
    // metadados, montar o grafo dos crates, compilar os build scripts, indexar).
    let deadline = Instant::now() + Duration::from_secs(300);
    let mut quiet_since = Instant::now();
    while Instant::now() < deadline && !(busy_seen && client.work().is_empty() && quiet_since.elapsed() > Duration::from_secs(15)) {
        if client.poll().contains(&Incoming::Progress) {
            let (short, _) = lsp::work_text(server.name, &client.work());
            if seen.last() != Some(&short) {
                println!("{:.1} s: {short}", started.elapsed().as_secs_f32());
                seen.push(short);
            }
            busy_seen |= !client.work().is_empty();
            quiet_since = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    client.stop();
    assert!(busy_seen, "o servidor avisou o progresso: {seen:?}");
    assert_eq!(seen.last().map(String::as_str), Some(server.name), "terminou pronto: {seen:?}");
}
