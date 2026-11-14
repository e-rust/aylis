//! O componente McpServer: HTTP de verdade em 127.0.0.1, as chamadas rodando na thread da
//! interface (aqui, o teste processa as mensagens no lugar do laço da janela).

use std::cell::Cell;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::rc::Rc;

use aylis_core::{Answer, EventArgs, PropValue, Ui};
use aylis_remote::mcp::{McpServer, McpTool};
use serde_json::{Value, json};

const TOKEN: &str = "fedcba9876543210fedcba9876543210";

fn app(server: Value) -> Ui {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    let mut props = json!({ "class": "McpServer", "name": "mcpLoja", "token": TOKEN, "expose_ui": true });
    if let (Some(p), Some(extra)) = (props.as_object_mut(), server.as_object()) {
        p.extend(extra.clone());
    }
    props["children"] = json!([
        { "class": "McpTool", "name": "toolSaudar", "tool_name": "saudar", "description": "Cumprimenta alguém", "params": "nome: string, vezes?: integer" },
        { "class": "McpTool", "name": "toolDepois", "tool_name": "depois" },
        { "class": "McpTool", "name": "toolDesligada", "enabled": false }
    ]);
    let layout = json!({ "aylis": 1, "root": {
        "class": "Form", "name": "frmLoja", "caption": "Loja", "width": 400, "height": 300,
        "children": [
            { "class": "Edit", "name": "edtNome", "align": "top" },
            { "class": "Button", "name": "btnSalvar", "text": "Salvar", "align": "top" },
            { "class": "Label", "name": "lblStatus", "align": "top" },
            props
        ]
    }});
    aylis_json::load_value(&mut ui, &layout, None).unwrap();
    let other = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmSecreto", "children": [{ "class": "Label", "name": "lblSenha", "text": "123" }] } });
    aylis_json::load_value(&mut ui, &other, None).unwrap();
    ui
}

fn start(ui: &mut Ui) -> u16 {
    let id = ui.find("mcpLoja").unwrap();
    ui.set_prop(id, "active", true).unwrap();
    let server = ui.get::<McpServer>(id).unwrap();
    assert!(server.running(), "não ligou: {}", server.last_error());
    let url = server.url();
    url.trim_start_matches("http://127.0.0.1:").trim_end_matches("/mcp").parse().unwrap()
}

/// Um POST em /mcp numa thread; o teste processa as mensagens do app enquanto espera.
/// `pump` roda a cada volta (para responder perguntas adiadas).
fn post_with(ui: &mut Ui, port: u16, body: Value, origin: Option<&str>, token: Option<&str>, pump: &mut dyn FnMut(&mut Ui)) -> (u16, Value) {
    let origin = origin.map(|o| format!("Origin: {o}\r\n")).unwrap_or_default();
    let auth = token.map(|t| format!("Authorization: Bearer {t}\r\n")).unwrap_or_default();
    let client = std::thread::spawn(move || {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let text = body.to_string();
        write!(s, "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\n{origin}{auth}Content-Length: {}\r\n\r\n{text}", text.len()).unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    });
    while !client.is_finished() {
        ui.process_posted();
        pump(ui);
        std::thread::yield_now();
    }
    let raw = client.join().unwrap();
    let status: u16 = raw.split_whitespace().nth(1).unwrap().parse().unwrap();
    let body = raw.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or_default();
    (status, serde_json::from_str(body).unwrap_or(Value::Null))
}

fn rpc(ui: &mut Ui, port: u16, method: &str, params: Value) -> Value {
    let (status, r) = post_with(ui, port, json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params }), None, Some(TOKEN), &mut |_| {});
    assert_eq!(status, 200);
    r["result"].clone()
}

fn call(ui: &mut Ui, port: u16, tool: &str, args: Value) -> (bool, String) {
    call_pumping(ui, port, tool, args, &mut |_| {})
}

fn call_pumping(ui: &mut Ui, port: u16, tool: &str, args: Value, pump: &mut dyn FnMut(&mut Ui)) -> (bool, String) {
    let body = json!({ "jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": { "name": tool, "arguments": args } });
    let (_, r) = post_with(ui, port, body, None, Some(TOKEN), pump);
    let result = &r["result"];
    (result["isError"] == true, result["content"][0]["text"].as_str().unwrap_or_default().to_string())
}

fn names(tools: &Value) -> Vec<String> {
    tools["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap().to_string()).collect()
}

#[test]
fn an_agent_calls_the_app_tools() {
    let mut ui = app(json!({ "instructions": "A loja de teste." }));
    let saudar = ui.find("toolSaudar").unwrap();
    ui.on(saudar, "on_execute", |ui, _, args| {
        let a = McpTool::arguments(args);
        let vezes = a["vezes"].as_u64().unwrap_or(1) as usize;
        McpTool::reply(ui, vec![format!("Olá, {}!", a["nome"].as_str().unwrap_or("?")); vezes].join(" "));
    });
    let port = start(&mut ui);

    let init = rpc(&mut ui, port, "initialize", json!({ "protocolVersion": "2025-06-18" }));
    assert_eq!(init["serverInfo"]["name"], "mcpLoja");
    assert!(init["instructions"].as_str().unwrap().starts_with("A loja de teste."));

    let tools = rpc(&mut ui, port, "tools/list", json!({}));
    let list = names(&tools);
    assert!(list.contains(&"saudar".to_string()) && list.contains(&"depois".to_string()));
    assert!(!list.iter().any(|n| n == "toolDesligada"), "a desligada não aparece: {list:?}");
    assert!(list.contains(&"ui_click".to_string()));
    let schema = &tools["tools"].as_array().unwrap().iter().find(|t| t["name"] == "saudar").unwrap()["inputSchema"];
    assert_eq!(schema["properties"]["vezes"]["type"], "integer");
    assert_eq!(schema["required"], json!(["nome"]));

    assert_eq!(call(&mut ui, port, "saudar", json!({ "nome": "Ana", "vezes": 2 })), (false, "Olá, Ana! Olá, Ana!".into()));
    // Sem handler ligado: o erro diz qual.
    let (error, text) = call(&mut ui, port, "depois", json!({}));
    assert!(error && text.contains("on_execute"), "{text}");
    let (error, text) = call(&mut ui, port, "nao_existe", json!({}));
    assert!(error && text.contains("desconhecida"), "{text}");
}

#[test]
fn a_tool_answers_later() {
    // Como o backend em outra linguagem: o handler adia e a resposta chega depois.
    let mut ui = app(json!({}));
    let depois = ui.find("toolDepois").unwrap();
    let token = Rc::new(Cell::new(0u64));
    let t = token.clone();
    ui.on(depois, "on_execute", move |ui, _, _| t.set(ui.defer_question()));
    let port = start(&mut ui);
    let mut pump = |ui: &mut Ui| {
        if token.get() != 0 {
            ui.answer(token.replace(0), Answer::Accepted(vec![("result".into(), PropValue::String("pronto".into()))]));
        }
    };
    assert_eq!(call_pumping(&mut ui, port, "depois", json!({}), &mut pump), (false, "pronto".into()));
}

#[test]
fn the_screen_tools_use_the_components() {
    let mut ui = app(json!({}));
    let btn = ui.find("btnSalvar").unwrap();
    ui.on(btn, "on_click", |ui, _, _| {
        let nome = ui.get_prop(ui.find("edtNome").unwrap(), "text").unwrap().to_str().unwrap().to_string();
        let lbl = ui.find("lblStatus").unwrap();
        ui.set_prop(lbl, "text", format!("Salvo: {nome}")).unwrap();
    });
    let port = start(&mut ui);

    let (error, forms) = call(&mut ui, port, "ui_forms", json!({}));
    assert!(!error && forms.contains("frmLoja") && forms.contains("frmSecreto"), "{forms}");
    let (_, tree) = call(&mut ui, port, "ui_tree", json!({ "form": "frmLoja" }));
    assert!(tree.contains("edtNome") && tree.contains("btnSalvar"), "{tree}");

    assert!(!call(&mut ui, port, "ui_type", json!({ "target": "edtNome", "text": "Ana" })).0);
    assert_eq!(call(&mut ui, port, "ui_get", json!({ "target": "edtNome", "prop": "text" })), (false, "Ana".into()));
    assert_eq!(call(&mut ui, port, "ui_click", json!({ "target": "btnSalvar" })), (false, "ok".into()));
    assert_eq!(call(&mut ui, port, "ui_get", json!({ "target": "frmLoja.lblStatus", "prop": "text" })), (false, "Salvo: Ana".into()));

    // O agente não chega no próprio servidor (nem para se dar mais poder).
    let (error, text) = call(&mut ui, port, "ui_set", json!({ "target": "mcpLoja", "prop": "read_only", "value": false }));
    assert!(error && text.contains("servidor MCP"), "{text}");
    let (error, _) = call(&mut ui, port, "ui_get", json!({ "target": "toolSaudar" }));
    assert!(error);
}

/// O `ui_click` com as opções do mouse de verdade (Ctrl, botão) e o `ui_drag` entre dois
/// controles, como o `app_command` da IDE (decidido em 14/11); o arrastar também passa pelo
/// alcance do servidor.
#[test]
fn the_agent_clicks_with_modifiers_and_drags() {
    let mut ui = app(json!({ "forms": "frmLoja" }));
    let btn = ui.find("btnSalvar").unwrap();
    let pressed: Rc<std::cell::RefCell<Vec<(bool, bool)>>> = Rc::default();
    let p = pressed.clone();
    ui.on(btn, "on_pointer_down", move |_, _, args| {
        if let EventArgs::Pointer(e) = args {
            p.borrow_mut().push((e.modifiers.ctrl, e.button == aylis_core::PointerButton::Secondary));
        }
    });
    // A soltura vai a quem capturou o toque (o botão), no lugar do outro controle.
    let released: Rc<std::cell::RefCell<Vec<f32>>> = Rc::default();
    let r = released.clone();
    ui.on(btn, "on_pointer_up", move |_, _, args| {
        if let EventArgs::Pointer(e) = args {
            r.borrow_mut().push(e.position.y);
        }
    });
    let port = start(&mut ui);

    let list = names(&rpc(&mut ui, port, "tools/list", json!({})));
    assert!(list.contains(&"ui_drag".to_string()), "{list:?}");
    assert!(!call(&mut ui, port, "ui_click", json!({ "target": "btnSalvar", "modifiers": "Ctrl" })).0);
    assert!(!call(&mut ui, port, "ui_click", json!({ "target": "btnSalvar", "button": "right" })).0);
    assert_eq!(*pressed.borrow(), vec![(true, false), (false, true)]);

    // Do botão ao campo: solta sobre o campo.
    let (error, text) = call(&mut ui, port, "ui_drag", json!({ "from": "btnSalvar", "to": "edtNome", "steps": 4 }));
    assert!(!error, "{text}");
    let edt = ui.find("edtNome").unwrap();
    let below = ui.absolute_rect(edt).unwrap().y - ui.absolute_rect(btn).unwrap().y;
    let last = *released.borrow().last().unwrap();
    assert!((last - (below + ui.absolute_rect(edt).unwrap().height / 2.0)).abs() < 1.0, "soltou no meio do campo: {last} (o campo a {below})");
    // Fora dos forms do servidor, recusa.
    let (error, text) = call(&mut ui, port, "ui_drag", json!({ "from": "btnSalvar", "to": "frmSecreto.lblSenha" }));
    assert!(error && text.contains("frmSecreto"), "{text}");
}

#[test]
fn read_only_and_forms_limit_the_agent() {
    let mut ui = app(json!({ "read_only": true, "forms": "frmLoja" }));
    let port = start(&mut ui);
    let list = names(&rpc(&mut ui, port, "tools/list", json!({})));
    assert!(list.contains(&"ui_get".to_string()) && list.contains(&"ui_data".to_string()));
    assert!(!list.contains(&"ui_set".to_string()) && !list.contains(&"ui_click".to_string()), "{list:?}");
    assert!(!list.contains(&"ui_drag".to_string()), "{list:?}");

    let (error, text) = call(&mut ui, port, "ui_set", json!({ "target": "edtNome", "prop": "text", "value": "x" }));
    assert!(error, "{text}");
    let (error, forms) = call(&mut ui, port, "ui_forms", json!({}));
    assert!(!error && !forms.contains("frmSecreto"), "{forms}");
    let (error, text) = call(&mut ui, port, "ui_get", json!({ "target": "frmSecreto.lblSenha", "prop": "text" }));
    assert!(error && text.contains("frmSecreto"), "{text}");
}

#[test]
fn on_call_can_refuse() {
    let mut ui = app(json!({}));
    let server = ui.find("mcpLoja").unwrap();
    ui.on(server, "on_call", |ui, _, args| {
        let call: Value = serde_json::from_str(args.text().unwrap()).unwrap();
        if call["tool"] == "ui_click" {
            ui.refuse("cliques estão bloqueados");
        }
    });
    let port = start(&mut ui);
    assert_eq!(call(&mut ui, port, "ui_click", json!({ "target": "btnSalvar" })), (true, "cliques estão bloqueados".into()));
    assert_eq!(call(&mut ui, port, "ui_get", json!({ "target": "btnSalvar", "prop": "text" })), (false, "Salvar".into()));
}

#[test]
fn only_local_clients_with_the_token() {
    let mut ui = app(json!({}));
    let port = start(&mut ui);
    let body = json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
    assert_eq!(post_with(&mut ui, port, body.clone(), None, None, &mut |_| {}).0, 401);
    assert_eq!(post_with(&mut ui, port, body.clone(), None, Some("errado-errado-errado-errado-errado"), &mut |_| {}).0, 401);
    assert_eq!(post_with(&mut ui, port, body.clone(), Some("https://site.com"), Some(TOKEN), &mut |_| {}).0, 403);
    assert_eq!(post_with(&mut ui, port, body, Some("http://localhost:3000"), Some(TOKEN), &mut |_| {}).0, 200);

    // Desligar fecha a porta.
    let id = ui.find("mcpLoja").unwrap();
    ui.set_prop(id, "active", false).unwrap();
    assert!(!ui.get::<McpServer>(id).unwrap().running());
    assert!(TcpStream::connect(("127.0.0.1", port)).is_err());
}

#[test]
fn a_generated_token_and_the_client_command() {
    let mut ui = app(json!({ "token": "", "server_name": "Minha Loja" }));
    let id = ui.find("mcpLoja").unwrap();
    ui.set_prop(id, "active", true).unwrap();
    let token = ui.get_prop(id, "token").unwrap().to_str().unwrap().to_string();
    assert_eq!(token.len(), 64);
    let command = ui.invoke(id, "client_command", &[]).unwrap();
    let command = command.to_str().unwrap();
    // `claude` no PATH; fora dele, o caminho completo (entre aspas) para rodar num terminal.
    let program = match aylis_remote::mcp::claude_executable() {
        Some(c) if !c.in_path => format!("\"{}\"", c.path.display()),
        _ => "claude".to_string(),
    };
    assert!(command.starts_with(&format!("{program} mcp add --transport http Minha-Loja http://127.0.0.1:")), "{command}");
    assert!(command.ends_with(&format!("Bearer {token}\"")), "{command}");
}

#[test]
fn the_designer_never_starts_the_server() {
    let mut ui = app(json!({}));
    ui.set_design_mode(true);
    let id = ui.find("mcpLoja").unwrap();
    ui.set_prop(id, "active", true).unwrap();
    assert!(!ui.get::<McpServer>(id).unwrap().running());
}

#[test]
fn the_short_params_form() {
    let tool = McpTool { params: "cliente: string, itens: lista, obs?: texto".into(), ..Default::default() };
    assert_eq!(
        tool.schema().unwrap(),
        json!({ "type": "object", "properties": { "cliente": { "type": "string" }, "itens": { "type": "array" }, "obs": { "type": "string" } }, "required": ["cliente", "itens"] })
    );
    let props = McpTool { params: r#"{"id": {"type": "integer"}}"#.into(), ..Default::default() };
    assert_eq!(props.schema().unwrap()["properties"]["id"]["type"], "integer");
    assert!(McpTool { params: "x: data".into(), ..Default::default() }.schema().unwrap_err().contains("data"));
}
