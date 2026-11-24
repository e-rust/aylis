//! O componente McpServer: HTTP de verdade em 127.0.0.1, as chamadas rodando na thread da
//! interface (aqui, o teste processa as mensagens no lugar do laço da janela).

use std::cell::Cell;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::rc::Rc;

use aylis_core::{Answer, EventArgs, PropValue, Ui};
use aylis_remote::mcp::{McpResource, McpServer, McpTool};
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

// ---------------------------------------------------------------------------
// Versão, anotações, JSON, on_result, recursos, prompts, avisos e a ponte stdio.

/// Um app com as ferramentas novas, os recursos e os prompts.
fn rich_app() -> Ui {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    let layout = json!({ "aylis": 1, "root": {
        "class": "Form", "name": "frmEstoque", "children": [{
            "class": "McpServer", "name": "mcpEstoque", "token": TOKEN, "server_name": "estoque", "server_version": "2.4.1",
            "children": [
                { "class": "McpTool", "name": "toolTabela", "tool_name": "tabela", "title": "Consultar tabela", "params": "nome: string",
                  "output_schema": "nome: string, colunas: integer", "read_only": true, "idempotent": true },
                { "class": "McpTool", "name": "toolApagar", "tool_name": "apagar", "description": "Apaga um rascunho" },
                { "class": "McpTool", "name": "toolSalvar", "tool_name": "salvar", "destructive": false, "idempotent": true },
                { "class": "McpResource", "name": "resLeia", "uri": "app://leia-me", "title": "Leia-me", "mime_type": "text/markdown", "text": "# Estoque" },
                { "class": "McpResource", "name": "resTabela", "uri": "app://tabelas/{tabela}/colunas", "mime_type": "application/json" },
                { "class": "McpPrompt", "name": "prmResumo", "prompt_name": "resumo", "title": "Resumo de tabela",
                  "arguments": "tabela: o nome da tabela; foco?: o que destacar", "text": "Resuma a tabela {tabela}, destacando {foco}." },
                { "class": "McpPrompt", "name": "prmPerg", "prompt_name": "pergunta" }
            ]
        }]
    }});
    aylis_json::load_value(&mut ui, &layout, None).unwrap();
    ui
}

fn start_named(ui: &mut Ui, name: &str) -> u16 {
    let id = ui.find(name).unwrap();
    ui.set_prop(id, "active", true).unwrap();
    let server = ui.get::<McpServer>(id).unwrap();
    assert!(server.running(), "não ligou: {}", server.last_error());
    server.url().trim_start_matches("http://127.0.0.1:").trim_end_matches("/mcp").parse().unwrap()
}

fn rpc_raw(ui: &mut Ui, port: u16, method: &str, params: Value) -> Value {
    let (status, r) = post_with(ui, port, json!({ "jsonrpc": "2.0", "id": 7, "method": method, "params": params }), None, Some(TOKEN), &mut |_| {});
    assert_eq!(status, 200);
    r
}

#[test]
fn version_annotations_json_results_and_on_result() {
    let mut ui = rich_app();
    let server = ui.find("mcpEstoque").unwrap();
    let results = Rc::new(std::cell::RefCell::new(Vec::<aylis_remote::mcp::McpResult>::new()));
    let r = results.clone();
    ui.on(server, "on_result", move |_, _, args| r.borrow_mut().push(<aylis_remote::mcp::McpResult as aylis_core::EventValue>::from_args(args).unwrap()));
    ui.on(ui.find("toolTabela").unwrap(), "on_execute", |ui, _, args| {
        let nome = McpTool::arguments(args)["nome"].as_str().unwrap_or_default().to_string();
        if nome == "ruim" {
            McpTool::reply(ui, json!({ "nome": nome }));
        } else {
            McpTool::reply(ui, json!({ "nome": nome, "colunas": 12 }));
        }
    });
    ui.on(ui.find("toolSalvar").unwrap(), "on_execute", |ui, _, _| McpTool::reply(ui, "salvo"));
    let port = start_named(&mut ui, "mcpEstoque");

    let init = rpc(&mut ui, port, "initialize", json!({}));
    assert_eq!(init["serverInfo"], json!({ "name": "estoque", "version": "2.4.1" }));
    assert_eq!(init["capabilities"]["tools"]["listChanged"], true);
    assert_eq!(init["capabilities"]["resources"]["listChanged"], true);
    assert_eq!(init["capabilities"]["prompts"]["listChanged"], true);

    let tools = rpc(&mut ui, port, "tools/list", json!({}));
    let find = |n: &str| tools["tools"].as_array().unwrap().iter().find(|t| t["name"] == n).unwrap().clone();
    let tabela = find("tabela");
    assert_eq!(tabela["title"], "Consultar tabela");
    assert_eq!(tabela["annotations"], json!({ "readOnlyHint": true, "idempotentHint": true, "title": "Consultar tabela" }), "só leitura: sem destructiveHint");
    assert_eq!(tabela["outputSchema"]["properties"]["colunas"]["type"], "integer");
    // O padrão do MCP: o que não é só leitura pode destruir, até dizer o contrário.
    assert_eq!(find("apagar")["annotations"]["destructiveHint"], true);
    assert_eq!(find("salvar")["annotations"], json!({ "readOnlyHint": false, "idempotentHint": true, "destructiveHint": false }));
    assert!(find("salvar").get("outputSchema").is_none());

    let r = rpc(&mut ui, port, "tools/call", json!({ "name": "tabela", "arguments": { "nome": "CLIENTES" } }));
    assert_eq!(r["structuredContent"], json!({ "nome": "CLIENTES", "colunas": 12 }));
    assert_eq!(r["isError"], false);
    assert!(r["content"][0]["text"].as_str().unwrap().contains("\"colunas\": 12"), "o texto vai junto");
    // Fora do output_schema: erro, com o motivo.
    let r = rpc(&mut ui, port, "tools/call", json!({ "name": "tabela", "arguments": { "nome": "ruim" } }));
    assert_eq!(r["isError"], true);
    assert!(r["content"][0]["text"].as_str().unwrap().contains("colunas"));
    // Sem output_schema, texto simples fica texto.
    let r = rpc(&mut ui, port, "tools/call", json!({ "name": "salvar", "arguments": {} }));
    assert_eq!((r["content"][0]["text"].as_str(), r.get("structuredContent")), (Some("salvo"), None));

    let results = results.borrow();
    assert_eq!(results.len(), 3);
    assert_eq!((results[0].tool.as_str(), results[0].is_error), ("tabela", false));
    assert_eq!(serde_json::from_str::<Value>(&results[0].arguments).unwrap(), json!({ "nome": "CLIENTES" }));
    assert_eq!(serde_json::from_str::<Value>(&results[0].result).unwrap()["colunas"], 12);
    assert!(results[1].is_error);
    assert_eq!(results[2].result, "salvo");
    assert!(results.iter().all(|r| r.elapsed_ms >= 0));
}

#[test]
fn resources_and_prompts() {
    let mut ui = rich_app();
    let server = ui.find("mcpEstoque").unwrap();
    ui.on(ui.find("resTabela").unwrap(), "on_read", |ui, _, args| {
        let a = McpTool::arguments(args);
        let tabela = a["params"]["tabela"].as_str().unwrap_or_default().to_string();
        if tabela == "SECRETA" {
            ui.refuse("tabela sem acesso");
        } else {
            McpTool::reply(ui, json!({ "tabela": tabela, "uri": a["uri"] }));
        }
    });
    ui.on(ui.find("prmPerg").unwrap(), "on_get", |ui, _, _| McpTool::reply(ui, "Que tabela guarda os pedidos?"));
    let calls = Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let c = calls.clone();
    ui.on(server, "on_call", move |_, _, args| c.borrow_mut().push(args.text().unwrap_or_default().to_string()));
    let port = start_named(&mut ui, "mcpEstoque");

    let list = rpc(&mut ui, port, "resources/list", json!({}));
    assert_eq!(list["resources"], json!([{ "name": "resLeia", "mimeType": "text/markdown", "uri": "app://leia-me", "title": "Leia-me" }]));
    let templates = rpc(&mut ui, port, "resources/templates/list", json!({}));
    assert_eq!(templates["resourceTemplates"][0]["uriTemplate"], "app://tabelas/{tabela}/colunas");

    let read = rpc(&mut ui, port, "resources/read", json!({ "uri": "app://leia-me" }));
    assert_eq!(read["contents"], json!([{ "uri": "app://leia-me", "mimeType": "text/markdown", "text": "# Estoque" }]));
    let read = rpc(&mut ui, port, "resources/read", json!({ "uri": "app://tabelas/PEDIDOS/colunas" }));
    let content = &read["contents"][0];
    assert_eq!((content["uri"].as_str(), content["mimeType"].as_str()), (Some("app://tabelas/PEDIDOS/colunas"), Some("application/json")));
    assert_eq!(serde_json::from_str::<Value>(content["text"].as_str().unwrap()).unwrap()["tabela"], "PEDIDOS");
    // O on_read recusou; um endereço que não bate é "não encontrado".
    let r = rpc_raw(&mut ui, port, "resources/read", json!({ "uri": "app://tabelas/SECRETA/colunas" }));
    assert_eq!(r["error"]["message"], "tabela sem acesso");
    let r = rpc_raw(&mut ui, port, "resources/read", json!({ "uri": "app://tabelas/a/b/colunas" }));
    assert_eq!(r["error"]["code"], -32002);

    let prompts = rpc(&mut ui, port, "prompts/list", json!({}));
    let resumo = prompts["prompts"].as_array().unwrap().iter().find(|p| p["name"] == "resumo").unwrap().clone();
    assert_eq!(resumo["title"], "Resumo de tabela");
    assert_eq!(resumo["arguments"], json!([
        { "name": "tabela", "description": "o nome da tabela", "required": true },
        { "name": "foco", "description": "o que destacar", "required": false }
    ]));
    let got = rpc(&mut ui, port, "prompts/get", json!({ "name": "resumo", "arguments": { "tabela": "CLIENTES", "foco": "as chaves" } }));
    assert_eq!(got["messages"], json!([{ "role": "user", "content": { "type": "text", "text": "Resuma a tabela CLIENTES, destacando as chaves." } }]));
    let r = rpc_raw(&mut ui, port, "prompts/get", json!({ "name": "resumo", "arguments": {} }));
    assert!(r["error"]["message"].as_str().unwrap().contains("tabela"), "{r}");
    let got = rpc(&mut ui, port, "prompts/get", json!({ "name": "pergunta" }));
    assert_eq!(got["messages"][0]["content"]["text"], "Que tabela guarda os pedidos?");

    // O on_call viu as leituras também.
    let calls = calls.borrow();
    assert!(calls.iter().any(|c| c.contains("\"resource\":\"app://leia-me\"")), "{calls:?}");
    assert!(calls.iter().any(|c| c.contains("\"prompt\":\"resumo\"")), "{calls:?}");
}

#[test]
fn the_template_matches_by_pieces() {
    let r = McpResource { uri: "app://t/{a}/x/{b}".into(), ..Default::default() };
    assert_eq!(Value::Object(r.matches("app://t/1/x/2/3").unwrap()), json!({ "a": "1", "b": "2/3" }));
    assert!(r.matches("app://t//x/2").is_none());
    assert!(r.matches("app://u/1/x/2").is_none());
    let fixed = McpResource { uri: "app://a".into(), ..Default::default() };
    assert!(fixed.matches("app://a").is_some() && fixed.matches("app://ab").is_none());
}

/// Abre o fluxo de avisos (`GET /mcp`) e devolve o que chegar, numa thread.
fn listen_events(port: u16) -> std::sync::mpsc::Receiver<Value> {
    use std::io::BufRead;
    let (tx, rx) = std::sync::mpsc::channel();
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    write!(s, "GET /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nAccept: text/event-stream\r\nAuthorization: Bearer {TOKEN}\r\n\r\n").unwrap();
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(s);
        for line in reader.lines() {
            let Ok(line) = line else { return };
            if let Some(data) = line.strip_prefix("data: ") {
                let _ = tx.send(serde_json::from_str(data).unwrap());
            }
        }
    });
    rx
}

fn wait_event(ui: &mut Ui, rx: &std::sync::mpsc::Receiver<Value>) -> Value {
    let start = std::time::Instant::now();
    loop {
        ui.process_posted();
        if let Ok(v) = rx.recv_timeout(std::time::Duration::from_millis(10)) {
            return v;
        }
        assert!(start.elapsed().as_secs() < 10, "o aviso não chegou");
    }
}

#[test]
fn the_server_announces_list_changes() {
    let mut ui = rich_app();
    let port = start_named(&mut ui, "mcpEstoque");
    // Sem o token, o fluxo de avisos não abre.
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    write!(s, "GET /mcp HTTP/1.1\r\nAccept: text/event-stream\r\n\r\n").unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    assert!(out.starts_with("HTTP/1.1 401"), "{out}");

    let rx = listen_events(port);
    // O fluxo precisa estar registrado antes da mudança.
    let start = std::time::Instant::now();
    while ui.get::<McpServer>(ui.find("mcpEstoque").unwrap()).unwrap().listeners() == 0 {
        assert!(start.elapsed().as_secs() < 10, "o fluxo de avisos não abriu");
        ui.process_posted();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let tool = ui.find("toolApagar").unwrap();
    ui.set_prop(tool, "enabled", false).unwrap();
    assert_eq!(wait_event(&mut ui, &rx), json!({ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" }));
    let prompt = ui.find("prmPerg").unwrap();
    ui.set_prop(prompt, "title", "Pergunta livre").unwrap();
    assert_eq!(wait_event(&mut ui, &rx)["method"], "notifications/prompts/list_changed");
    let res = ui.find("resLeia").unwrap();
    ui.set_prop(res, "uri", "app://leia").unwrap();
    assert_eq!(wait_event(&mut ui, &rx)["method"], "notifications/resources/list_changed");
    // Mudar o que o agente não vê (o texto do recurso) não avisa nada.
    ui.set_prop(res, "text", "outro").unwrap();
    for _ in 0..20 {
        ui.process_posted();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert!(rx.try_recv().is_err());
}

#[test]
fn the_stdio_bridge_finds_the_running_server() {
    use std::sync::{Arc, Mutex};
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("mcp-apps-bridge");
    let _ = std::fs::remove_dir_all(&dir);
    // SAFETY dos testes: o único teste deste arquivo que lê a variável.
    unsafe { std::env::set_var("AYLIS_MCP_APPS_DIR", &dir) };
    aylis_remote::mcp::stdio::set_enabled(true);
    let mut ui = rich_app();
    // Um nome só deste teste (os outros, em paralelo, podem deixar endereços na mesma pasta).
    let server = ui.find("mcpEstoque").unwrap();
    ui.set_prop(server, "server_name", "ponte-teste").unwrap();
    ui.on(ui.find("toolSalvar").unwrap(), "on_execute", |ui, _, _| McpTool::reply(ui, "salvo pela ponte"));
    start_named(&mut ui, "mcpEstoque");
    let mine = |dir: &std::path::Path| -> Vec<std::path::PathBuf> {
        std::fs::read_dir(dir).unwrap().flatten().map(|e| e.path()).filter(|p| p.to_string_lossy().contains("ponte-teste")).collect()
    };
    let files = mine(&dir);
    assert_eq!(files.len(), 1, "o servidor deixou o endereço para a ponte");
    let saved: Value = serde_json::from_str(&std::fs::read_to_string(&files[0]).unwrap()).unwrap();
    assert_eq!(saved["name"], "ponte-teste");
    let config: Value = serde_json::from_str(ui.invoke(server, "stdio_config", &[]).unwrap().to_str().unwrap()).unwrap();
    assert_eq!(config["mcpServers"]["ponte-teste"]["args"], json!(["--mcp-stdio", "ponte-teste"]));

    let input = [
        json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }).to_string(),
        json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }).to_string(),
        "{ não é json".to_string(),
        json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": { "name": "salvar", "arguments": {} } }).to_string(),
    ]
    .join("\n");
    let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
    struct Shared(Arc<Mutex<Vec<u8>>>);
    impl Write for Shared {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let out: aylis_remote::mcp::stdio::Output = Arc::new(Mutex::new(Box::new(Shared(buffer.clone()))));
    let bridge = std::thread::spawn(move || aylis_remote::mcp::stdio::bridge(Some("ponte-teste"), std::io::Cursor::new(input), out));
    while !bridge.is_finished() {
        ui.process_posted();
        std::thread::yield_now();
    }
    bridge.join().unwrap().unwrap();
    let text = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
    let lines: Vec<Value> = text.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(lines.len(), 3, "{text}");
    assert_eq!(lines[0]["result"]["serverInfo"]["name"], "ponte-teste");
    assert_eq!(lines[1]["error"]["code"], -32700);
    assert_eq!(lines[2]["result"]["content"][0]["text"], "salvo pela ponte");

    // Parado, o endereço sai, e a ponte responde que o app não está aberto.
    ui.set_prop(server, "active", false).unwrap();
    assert!(mine(&dir).is_empty());
    let buffer2 = Arc::new(Mutex::new(Vec::<u8>::new()));
    let out: aylis_remote::mcp::stdio::Output = Arc::new(Mutex::new(Box::new(Shared(buffer2.clone()))));
    aylis_remote::mcp::stdio::bridge(Some("ponte-teste"), std::io::Cursor::new(json!({ "jsonrpc": "2.0", "id": 3, "method": "ping" }).to_string()), out).unwrap();
    let reply: Value = serde_json::from_slice(&buffer2.lock().unwrap()).unwrap();
    assert!(reply["error"]["message"].as_str().unwrap().contains("não está aberto"), "{reply}");
    aylis_remote::mcp::stdio::set_enabled(false);
}
