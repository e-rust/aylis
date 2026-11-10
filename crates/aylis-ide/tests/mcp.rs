//! O servidor MCP da IDE: HTTP de verdade em 127.0.0.1, as chamadas rodando na thread da
//! interface (aqui, o teste processa as mensagens no lugar do laço da janela).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app, mcp};
use serde_json::{Value, json};

/// O token do servidor nos testes (o da IDE de verdade fica em %LOCALAPPDATA%aylismcp.json).
const TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Um POST em /mcp numa thread; o teste processa as mensagens da IDE enquanto espera.
fn post(ui: &mut aylis_core::Ui, port: u16, body: Value, origin: Option<&str>) -> (u16, Value) {
    post_with(ui, port, body, origin, Some(TOKEN))
}

fn post_with(ui: &mut aylis_core::Ui, port: u16, body: Value, origin: Option<&str>, token: Option<&str>) -> (u16, Value) {
    let origin = origin.map(|o| format!("Origin: {o}\r\n")).unwrap_or_default();
    let auth = token.map(|t| format!("Authorization: Bearer {t}\r\n")).unwrap_or_default();
    let client = std::thread::spawn(move || {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let text = body.to_string();
        write!(s, "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\n{origin}{auth}Content-Length: {}\r\n\r\n{text}", text.len()).unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    });
    while !client.is_finished() {
        ui.process_posted();
        std::thread::yield_now();
    }
    let raw = client.join().unwrap();
    let status: u16 = raw.split_whitespace().nth(1).unwrap().parse().unwrap();
    let body = raw.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or_default();
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
}

fn call(ui: &mut aylis_core::Ui, port: u16, tool: &str, args: Value) -> (bool, String) {
    let (_, r) = post(ui, port, json!({ "jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": { "name": tool, "arguments": args } }), None);
    let result = &r["result"];
    (result["isError"] == true, result["content"][0]["text"].as_str().unwrap_or_default().to_string())
}

#[test]
fn an_agent_edits_the_open_form_over_mcp() {
    let base = temp("mcp");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();

    // Aperto de mão.
    let (status, init) = post(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "teste", "version": "1" } } }), None);
    assert_eq!(status, 200);
    assert_eq!(init["result"]["serverInfo"]["name"], "aylis-ide");
    let (status, _) = post(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }), None);
    assert_eq!(status, 202, "notificação: aceita sem corpo");
    let (_, list) = post(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }), None);
    let names: Vec<&str> = list["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"set_property") && names.contains(&"app_command"), "{names:?}");

    // O agente abre o form, põe um botão, muda o texto e salva.
    let (err, info) = call(&mut ide.ui, port, "project_info", json!({}));
    assert!(!err && info.contains("frmPrincipal"), "{info}");
    assert!(!call(&mut ide.ui, port, "open_form", json!({ "name": "frmPrincipal" })).0);
    let (err, name) = call(&mut ide.ui, port, "add_component", json!({ "class": "Button", "name": "btnSalvar" }));
    assert!(!err && name == "btnSalvar", "{name}");
    assert!(!call(&mut ide.ui, port, "set_property", json!({ "component": "btnSalvar", "property": "text", "value": "Salvar" })).0);
    let (_, props) = call(&mut ide.ui, port, "get_component", json!({ "component": "btnSalvar" }));
    assert!(props.contains("\"Salvar\""), "{props}");
    let (err, msg) = call(&mut ide.ui, port, "set_property", json!({ "component": "naoExiste", "property": "text", "value": "x" }));
    assert!(err && msg.contains("naoExiste"));
    assert!(!call(&mut ide.ui, port, "save", json!({})).0);
    let saved = std::fs::read_to_string(project.file("forms/principal.json")).unwrap();
    assert!(saved.contains("btnSalvar") && saved.contains("Salvar"), "{saved}");
    // E desfazer na IDE desfaz o que o agente fez.
    assert!(state.borrow().doc.can_undo());

    // Uma página de outro site não fala com a IDE.
    let (status, _) = post(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/list" }), Some("https://exemplo.com"));
    assert_eq!(status, 403);
    // Um programa da máquina sem o token também não.
    let (status, _) = post_with(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "id": 4, "method": "tools/list" }), None, None);
    assert_eq!(status, 401);
    let (status, _) = post_with(&mut ide.ui, port, json!({ "jsonrpc": "2.0", "id": 5, "method": "tools/list" }), None, Some(&"f".repeat(64)));
    assert_eq!(status, 401);
    server.stop();
}

/// O agente faz o que o usuário faz: descobre os menus, cria o projeto e forms, muda a
/// configuração, escolhe o alvo, seleciona, desfaz e fecha o projeto.
#[test]
fn an_agent_drives_the_project_over_mcp() {
    let base = temp("mcp_project");
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();
    let ui = &mut ide.ui;

    let (err, menus) = call(ui, port, "ide_menus", json!({}));
    assert!(!err && menus.contains("Executar/Compilar") && menus.contains("Ctrl+F9"), "{menus}");

    let (err, made) = call(ui, port, "new_project", json!({ "dir": base.display().to_string(), "name": "Agenda" }));
    assert!(!err && made.contains("Agenda"), "{made}");
    assert!(state.borrow().project.is_some());
    let (err, msg) = call(ui, port, "new_form", json!({ "name": "frmContatos" }));
    assert!(!err, "{msg}");
    assert!(state.borrow().project.as_ref().unwrap().form("frmContatos").is_some());
    assert_eq!(state.borrow().doc.form_name(), "frmContatos");

    let (err, msg) = call(ui, port, "set_project_setting", json!({ "key": "publish.version", "value": "2.1.0" }));
    assert!(!err, "{msg}");
    let (_, settings) = call(ui, port, "project_settings", json!({}));
    assert!(settings.contains("2.1.0"), "{settings}");
    let saved = std::fs::read_to_string(&state.borrow().project.as_ref().unwrap().path).unwrap();
    assert!(saved.contains("2.1.0"), "salvo no .aylisproj: {saved}");

    assert!(!call(ui, port, "set_target", json!({ "target": "web" })).0);
    assert_eq!(state.borrow().run_target, aylis_tools::Target::Web);
    assert!(call(ui, port, "set_target", json!({ "target": "lua" })).0);

    let (_, name) = call(ui, port, "add_component", json!({ "class": "Button", "name": "btnNovo" }));
    assert_eq!(name, "btnNovo");
    assert!(!call(ui, port, "select", json!({ "components": ["btnNovo"] })).0);
    assert!(!call(ui, port, "undo", json!({})).0);
    assert!(state.borrow().doc.ui.find_in(state.borrow().doc.root, "btnNovo").is_none(), "desfeito");
    assert!(!call(ui, port, "redo", json!({})).0);
    assert!(state.borrow().doc.ui.find_in(state.borrow().doc.root, "btnNovo").is_some(), "refeito");

    let (err, _) = call(ui, port, "debug_variables", json!({ "reference": 5 }));
    assert!(err, "sem depuração: erro");
    assert!(!call(ui, port, "close_project", json!({})).0);
    assert!(state.borrow().project.is_none());
    assert!(aylis_ide::home::visible(&state), "fechou: a tela inicial volta");
    server.stop();
    // A IDE confia sozinha no projeto que cria: a pasta de teste sai da lista do usuário.
    aylis_tools::trust::forget_dir(&base.join("Agenda"));
    let _ = std::fs::remove_dir_all(base);
}

/// Os diálogos e os menus pelo MCP (buracos achados na conferência de 08/11): o
/// `dialog_state` mostra as escolhas (ComboBox), as caixas e os botões; o `dialog_select`
/// escolhe um item como o usuário; o Esc fecha a Galeria; o `ide_menus` dá cada menu de
/// contexto uma vez só, com o lugar.
#[test]
fn an_agent_fills_the_dialogs_and_reads_the_menus() {
    let base = temp("mcp_dialogs");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();
    let ui = &mut ide.ui;

    // O assistente do Novo projeto: a Linguagem é um ComboBox, e os botões do rodapé aparecem.
    let (err, out) = call(ui, port, "menu", json!({ "path": "Arquivo/Novo projeto…" }));
    assert!(!err, "{out}");
    let (err, s) = call(ui, port, "dialog_state", json!({}));
    assert!(!err, "{s}");
    let s: Value = serde_json::from_str(&s).unwrap();
    let combos = s["combos"].as_array().unwrap();
    let language = combos.iter().find(|c| c["label"] == "Linguagem").unwrap_or_else(|| panic!("{s}"));
    assert_eq!(language["text"], "Rust");
    assert!(language["items"].as_array().unwrap().iter().any(|i| i == "Python"), "{language}");
    assert!(combos.iter().all(|c| c["label"] != "Modelo"), "a página do Modelo está escondida: {s}");
    let buttons: Vec<&str> = s["buttons"].as_array().unwrap().iter().filter_map(|b| b["text"].as_str()).collect();
    assert!(buttons.contains(&"Avançar") && buttons.contains(&"Cancelar") && !buttons.contains(&"Criar"), "{buttons:?}");

    // Escolher pelo texto (sem diferença de maiúsculas) e pelo índice, na lista pelo rótulo.
    let (err, s) = call(ui, port, "dialog_select", json!({ "list": "Linguagem", "item": "python" }));
    assert!(!err, "{s}");
    let s: Value = serde_json::from_str(&s).unwrap();
    assert!(s["combos"].as_array().unwrap().iter().any(|c| c["label"] == "Linguagem" && c["text"] == "Python"), "{s}");
    let (err, s) = call(ui, port, "dialog_select", json!({ "list": "Linguagem", "item": 0 }));
    assert!(!err && s.contains("\"Rust\""), "{s}");
    let (err, msg) = call(ui, port, "dialog_select", json!({ "list": "Linguagem", "item": "Cobol" }));
    assert!(err && msg.contains("Cobol"), "{msg}");
    let (err, msg) = call(ui, port, "dialog_select", json!({ "list": "Sabor", "item": "Rust" }));
    assert!(err && msg.contains("Sabor"), "{msg}");
    let (err, out) = call(ui, port, "dialog_key", json!({ "key": "Escape" }));
    assert!(!err && out.contains("closed"), "{out}");

    // A Galeria fecha pelo Esc.
    let (err, out) = call(ui, port, "menu", json!({ "path": "Exibir/Galeria de componentes…" }));
    assert!(!err && out.contains("Componentes"), "{out}");
    let (err, out) = call(ui, port, "dialog_key", json!({ "key": "Escape" }));
    assert!(!err && out.contains("closed"), "a Galeria fechou: {out}");

    // Com um componente selecionado, o Object Inspector tem um menu de contexto por propriedade:
    // o `ide_menus` dá o "Ligar a campo…" uma vez só, com o lugar.
    assert!(!call(ui, port, "open_form", json!({ "name": "frmPrincipal" })).0);
    let (_, name) = call(ui, port, "add_component", json!({ "class": "Edit", "name": "edtNome" }));
    assert_eq!(name, "edtNome");
    assert!(!call(ui, port, "select", json!({ "components": ["edtNome"] })).0);
    let (err, menus) = call(ui, port, "ide_menus", json!({}));
    assert!(!err, "{menus}");
    let menus: Value = serde_json::from_str(&menus).unwrap();
    let paths: Vec<&str> = menus.as_array().unwrap().iter().filter_map(|m| m["path"].as_str()).collect();
    let link: Vec<&&str> = paths.iter().filter(|p| p.ends_with("Ligar a campo…")).collect();
    assert_eq!(link, [&"Object Inspector › (propriedade) › Ligar a campo…"], "{paths:?}");
    let unique: std::collections::HashSet<&&str> = paths.iter().collect();
    assert_eq!(unique.len(), paths.len(), "sem repetidos");
    assert!(paths.contains(&"Executar/Compilar"), "{paths:?}");
    server.stop();
    let _ = std::fs::remove_dir_all(base);
}

/// O código pelo MCP (pedido do autor em 07/11: o agente não grava direto nos arquivos de um
/// projeto aberto): `read_code` lê o texto do editor (com o que não foi salvo) e `edit_code`
/// troca pelo editor, com desfazer, até salvar.
#[test]
fn an_agent_edits_the_code_through_the_ide_editor() {
    let base = temp("mcp_code");
    let project = aylis_project::create(&base, "Caixa", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let server = mcp::start(ide.ui.poster(), 0, TOKEN.into()).unwrap();
    let port = server.port();
    let ui = &mut ide.ui;

    let (err, read) = call(ui, port, "read_code", json!({}));
    assert!(!err, "{read}");
    let read: Value = serde_json::from_str(&read).unwrap();
    let file = PathBuf::from(read["file"].as_str().unwrap());
    let disk = std::fs::read_to_string(&file).unwrap();
    assert_eq!(read["text"], disk, "fechado no editor: o do disco");
    let first = disk.lines().next().unwrap().to_string();

    // A troca vai pelo editor: o disco fica como estava até salvar.
    let (err, out) = call(ui, port, "edit_code", json!({ "old": first, "new": "// trocado pelo agente" }));
    assert!(!err, "{out}");
    let out: Value = serde_json::from_str(&out).unwrap();
    assert!(out["replaced"] == 1 && out["line"] == 1, "{out}");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), disk, "ainda não salvo");
    let (_, read) = call(ui, port, "read_code", json!({ "start_line": 1, "end_line": 1 }));
    let read: Value = serde_json::from_str(&read).unwrap();
    assert_eq!(read["text"], "// trocado pelo agente");
    assert_eq!(read["unsaved"], true);

    // Ambíguo, ausente, form e gerado: recusados com o motivo.
    let (err, msg) = call(ui, port, "edit_code", json!({ "old": "\n", "new": "\n\n" }));
    assert!(err && msg.contains("vezes"), "{msg}");
    let (err, msg) = call(ui, port, "edit_code", json!({ "old": "não existe no arquivo", "new": "x" }));
    assert!(err && msg.contains("não aparece"), "{msg}");
    let form_file = project.dir().join("forms/principal.json");
    if form_file.is_file() {
        let (err, msg) = call(ui, port, "edit_code", json!({ "file": form_file.display().to_string(), "old": "{", "new": "{" }));
        assert!(err && msg.contains("set_property"), "{msg}");
    }

    // O save com `all` grava o código alterado.
    let (err, msg) = call(ui, port, "save", json!({ "all": true }));
    assert!(!err && msg.contains("1 fonte(s)"), "{msg}");
    assert!(std::fs::read_to_string(&file).unwrap().starts_with("// trocado pelo agente"));
    server.stop();
    let _ = std::fs::remove_dir_all(base);
}
