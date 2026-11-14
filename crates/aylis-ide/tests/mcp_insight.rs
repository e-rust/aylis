//! As ferramentas do MCP que medem e conferem (2b: `profile`, `recording`, `coverage`), a
//! cobertura de handlers na IDE (4b: Projeto → Cobertura de handlers…) e o guia para agentes do
//! Novo projeto (2a), pelo servidor MCP de verdade.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app, mcp};
use serde_json::{Value, json};

const TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn post(ui: &mut aylis_core::Ui, port: u16, body: Value) -> Value {
    let client = std::thread::spawn(move || {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let text = body.to_string();
        write!(s, "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nAuthorization: Bearer {TOKEN}\r\nContent-Length: {}\r\n\r\n{text}", text.len()).unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    });
    while !client.is_finished() {
        ui.process_posted();
        std::thread::yield_now();
    }
    let raw = client.join().unwrap();
    let body = raw.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or_default();
    serde_json::from_str(body).unwrap_or(Value::Null)
}

fn call(ui: &mut aylis_core::Ui, port: u16, tool: &str, args: Value) -> (bool, String) {
    let r = post(ui, port, json!({ "jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": { "name": tool, "arguments": args } }));
    let result = &r["result"];
    (result["isError"] == true, result["content"][0]["text"].as_str().unwrap_or_default().to_string())
}

/// Um form com dois handlers ligados (um em dois botões), e o código gerado.
fn project_with_handlers(base: &std::path::Path) -> aylis_project::Project {
    let project = aylis_project::create(base, "Loja", aylis_project::Language::Rust).unwrap();
    let form = project.dir().join(&project.form("frmPrincipal").unwrap().file);
    let layout = json!({
        "aylis": 1,
        "root": {
            "class": "Form", "name": "frmPrincipal", "caption": "Loja de teste do Aylis", "on_show": "frmPrincipal_show",
            "children": [
                { "class": "Button", "name": "btnSalvar", "on_click": "btnSalvar_click" },
                { "class": "Button", "name": "btnGravar", "on_click": "btnSalvar_click" }
            ]
        }
    });
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    project
}

#[test]
fn an_agent_measures_reads_the_recording_and_the_coverage() {
    let base = temp("mcp_insight");
    let project = project_with_handlers(&base);
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();
    let ui = &mut ide.ui;

    let list = post(ui, port, json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }));
    let names: Vec<&str> = list["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    for tool in ["profile", "recording", "coverage"] {
        assert!(names.contains(&tool), "{tool}: {names:?}");
    }

    // profile: sem app em execução, diz como ter um.
    let (err, out) = call(ui, port, "profile", json!({ "action": "start" }));
    assert!(err && out.contains("run_without_debug"), "{out}");

    // coverage: nada rodou; os que faltam com o evento, o arquivo e a linha.
    let (err, out) = call(ui, port, "coverage", json!({}));
    assert!(!err, "{out}");
    let report: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(report["handlers"], 2);
    assert_eq!(report["covered"], 0);
    assert!(report["note"].as_str().unwrap().contains("app_command"), "{report}");
    let salvar = report["missing"].as_array().unwrap().iter().find(|m| m["handler"] == "btnSalvar_click").unwrap_or_else(|| panic!("{report}"));
    assert_eq!(salvar["events"], json!(["btnSalvar.on_click", "btnGravar.on_click"]));
    assert!(salvar["file"].as_str().unwrap().ends_with(".rs") && salvar["line"].as_u64().unwrap() > 1, "{salvar}");
    // O app avisou que o handler rodou (aqui, direto no arquivo da cobertura).
    aylis_project::coverage::record(&project, &["frmPrincipal.btnSalvar_click".into()]).unwrap();
    let (_, out) = call(ui, port, "coverage", json!({}));
    let report: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(report["covered"], 1);
    assert_eq!(report["percent"], 50.0);
    assert_eq!(report["missing"][0]["handler"], "frmPrincipal_show");

    // A IDE mostra o mesmo em Projeto → Cobertura de handlers…
    let (err, out) = call(ui, port, "menu", json!({ "path": "Projeto/Cobertura de handlers…" }));
    assert!(!err, "{out}");
    let (_, s) = call(ui, port, "dialog_state", json!({}));
    assert!(s.contains("1 de 2 handlers rodaram (50%)"), "{s}");
    assert!(s.contains("frmPrincipal.frmPrincipal_show") && s.contains("NÃO RODARAM (1)"), "{s}");
    let (err, out) = call(ui, port, "dialog_click", json!({ "text": "Limpar" }));
    assert!(!err, "{out}");
    let (_, s) = call(ui, port, "dialog_state", json!({}));
    assert!(s.contains("0 de 2 handlers rodaram") && s.contains("NÃO RODARAM (2)"), "Limpar recomeça: {s}");
    assert!(!call(ui, port, "dialog_click", json!({ "text": "Fechar" })).0);
    let (_, out) = call(ui, port, "coverage", json!({ "clear": true }));
    assert!(out.contains("\"covered\": 0"), "{out}");

    // recording: uma gravação dada, resumida.
    let file = base.join("uso.aylisrec");
    let lines = [
        json!({ "type": "start", "format": 1, "app": "Loja", "start_ms": 1_700_000_000_000u64, "os": "windows" }),
        json!({ "t": 0.5, "type": "form_shown", "form": "frmPrincipal" }),
        json!({ "t": 1.0, "type": "pointer_down", "button": "left", "target": "frmPrincipal.btnSalvar", "target_class": "Button", "target_text": "Salvar" }),
        json!({ "t": 1.1, "type": "event", "event": "on_click", "component": "frmPrincipal.btnSalvar", "handler": "btnSalvar_click" }),
        json!({ "t": 2.0, "type": "error", "message": "falhou ao gravar", "location": "principal.rs:10" }),
    ];
    std::fs::write(&file, lines.iter().map(Value::to_string).collect::<Vec<_>>().join("\n")).unwrap();
    let (err, out) = call(ui, port, "recording", json!({ "file": file.display().to_string(), "last": 2 }));
    assert!(!err, "{out}");
    let summary: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(summary["app"], "Loja");
    assert_eq!(summary["entries"], 4);
    assert_eq!(summary["errors"][0]["message"], "falhou ao gravar");
    assert_eq!(summary["handlers"][0]["name"], "btnSalvar_click");
    assert_eq!(summary["most_clicked"][0]["count"], 1);
    assert_eq!(summary["forms_shown"], json!(["frmPrincipal"]));
    assert_eq!(summary["last"].as_array().unwrap().len(), 2);
    // Sem EventRecorder nem gravação do app: diz onde procurou.
    let (err, out) = call(ui, port, "recording", json!({}));
    assert!(err && out.contains("EventRecorder"), "{out}");
    server.stop();
    let _ = std::fs::remove_dir_all(base);
}

/// 2a: o Novo projeto grava o guia para agentes (a opção vem marcada), e o `new_project` do MCP
/// também, a não ser com `agent_guide` false.
#[test]
fn the_new_project_writes_the_agent_guide() {
    let base = temp("mcp_guide");
    let project = aylis_project::create(&base, "Atual", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();
    let ui = &mut ide.ui;

    // O assistente: três passos até as Configurações, onde a opção aparece marcada.
    assert!(!call(ui, port, "menu", json!({ "path": "Arquivo/Novo projeto…" })).0);
    for _ in 0..3 {
        let (err, out) = call(ui, port, "dialog_click", json!({ "text": "Avançar" }));
        assert!(!err, "{out}");
    }
    let (_, s) = call(ui, port, "dialog_state", json!({}));
    let s: Value = serde_json::from_str(&s).unwrap();
    let check = s["checks"].as_array().unwrap().iter().find(|c| c["text"].as_str().is_some_and(|t| t.starts_with("Guia para agentes de IA"))).unwrap_or_else(|| panic!("{s}"));
    assert_eq!(check["checked"], true, "marcada por padrão");
    assert!(!call(ui, port, "dialog_click", json!({ "text": "Avançar" })).0);
    let (err, out) = call(ui, port, "dialog_click", json!({ "text": "Criar" }));
    assert!(!err, "{out}");
    let created = base.join("MeuProjeto");
    assert!(created.join("AGENTS.md").is_file() && created.join("CLAUDE.md").is_file() && created.join(".mcp.json").is_file(), "{}", created.display());
    let agents = std::fs::read_to_string(created.join("AGENTS.md")).unwrap();
    assert!(agents.contains("O código em Rust") && agents.contains("MeuProjeto.aylisproj"), "{agents}");

    // Pelo MCP: com o guia (padrão) e sem.
    let dir = base.join("pelo_mcp");
    let (err, out) = call(ui, port, "new_project", json!({ "dir": dir.display().to_string(), "name": "Agente", "language": "Go" }));
    assert!(!err && out.contains("AGENTS.md"), "{out}");
    assert!(std::fs::read_to_string(dir.join("Agente/AGENTS.md")).unwrap().contains("O código em Go"));
    let (err, out) = call(ui, port, "new_project", json!({ "dir": dir.display().to_string(), "name": "Sem", "agent_guide": false }));
    assert!(!err, "{out}");
    assert!(!dir.join("Sem/AGENTS.md").exists() && !dir.join("Sem/.mcp.json").exists());
    server.stop();
    let _ = std::fs::remove_dir_all(base);
}
