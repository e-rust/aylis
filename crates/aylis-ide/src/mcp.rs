//! A IDE como servidor MCP (Model Context Protocol): agentes de IA (Claude Code e outros)
//! abrem forms, leem e mudam componentes, salvam, geram o código, executam o projeto e
//! comandam o app em execução, pelas mesmas ações da IDE.
//!
//! Transporte "streamable HTTP" do MCP, só em `127.0.0.1` (com a checagem de `Origin`
//! contra DNS rebinding): `POST /mcp` com JSON-RPC, resposta em `application/json`. Todo
//! pedido precisa do token (`Authorization: Bearer ...`, em `%LOCALAPPDATA%\aylis\mcp.json`):
//! sem ele, um programa qualquer da máquina executaria código pela IDE. Os clientes conectam
//! pela ponte, que lê o token sozinha (Ferramentas → Opções da IDE → MCP configura):
//!
//! ```text
//! claude mcp add --scope user aylis -- <pasta da IDE>/aylis-tool mcp
//! ```
//!
//! As conexões chegam numa thread própria; cada chamada é feita na thread da interface
//! (o estado da IDE não é `Send`) e a resposta volta por um canal.
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use aylis_core::{Poster, Ui};
use aylis_remote::mcp::http::{self, rpc_error as error};
use serde_json::{Value, json};

use crate::app::Ids;
use crate::state::Shared;

pub use aylis_remote::mcp::http::{Request, read_request};

/// A porta padrão.
pub const DEFAULT_PORT: u16 = aylis_tools::mcp::DEFAULT_PORT;
/// A versão do protocolo que a IDE fala.
const PROTOCOL: &str = aylis_remote::mcp::PROTOCOL;

thread_local! {
    /// O estado da IDE, na thread da interface (onde as chamadas rodam).
    static CONTEXT: RefCell<Option<(Shared, Ids)>> = const { RefCell::new(None) };
}

/// Guarda o estado da IDE para as chamadas (na thread da interface, ao montar a IDE).
pub fn install(state: &Shared, ids: &Ids) {
    CONTEXT.with(|c| *c.borrow_mut() = Some((state.clone(), *ids)));
}

/// O servidor ligado; parar (ou soltar) fecha o socket. O transporte é o mesmo do
/// componente `McpServer` dos apps ([`aylis_remote::mcp::http`]).
pub type McpServer = http::HttpServer;

/// Liga o servidor em `127.0.0.1:port` (0: uma porta livre); só atende quem traz `token`.
pub fn start(poster: Poster, port: u16, token: String) -> Result<McpServer, String> {
    http::start(port, token, "aylis-ide-mcp", Arc::new(move |message| handle_message(message, &poster)))
}

/// Responde uma mensagem JSON-RPC (`None` para notificações).
fn handle_message(message: Value, poster: &Poster) -> Option<Value> {
    let id = message.get("id").cloned();
    let method = message.get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    let params = message.get("params").cloned().unwrap_or(Value::Null);
    let id = id?; // notificação (`notifications/initialized`...)
    let result = match method.as_str() {
        "initialize" => Ok(json!({
            "protocolVersion": params.get("protocolVersion").and_then(Value::as_str).unwrap_or(PROTOCOL),
            "capabilities": { "tools": { "listChanged": false } },
            "serverInfo": { "name": "aylis-ide", "version": env!("CARGO_PKG_VERSION") },
            "instructions": aylis_tools::mcp::INSTRUCTIONS
        })),
        "ping" => Ok(json!({})),
        // A ponte avisa quem está ligado a esta IDE (a marca da sessão, o cliente e o título que o
        // agente deu à conversa) e quando sai (`connected` falso): aparece na barra da janela, e
        // o registro das IDEs diz às outras sessões que esta está ocupada.
        "aylis/session" => {
            let connected = params.get("connected").and_then(Value::as_bool).unwrap_or(true);
            let session = params.get("session").and_then(Value::as_str).unwrap_or_default().to_string();
            let client = params.get("client").and_then(Value::as_str).unwrap_or("Agente").to_string();
            let title = params.get("title").and_then(Value::as_str).unwrap_or_default().trim().to_string();
            on_ui(poster, move |_, state, _| {
                crate::instance::set_session(&session, connected);
                let mut st = state.borrow_mut();
                match st.agents.iter_mut().find(|a| a.0 == session) {
                    Some(a) if connected => *a = (session, client, title),
                    Some(_) => st.agents.retain(|a| a.0 != session),
                    None if connected => st.agents.push((session, client, title)),
                    None => {}
                }
            })
            .map(|_| json!({}))
            .map_err(|e| error(id.clone(), -32000, &e))
        }
        "tools/list" => Ok(json!({ "tools": tools() })),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            Ok(call_tool(&name, args, poster))
        }
        _ => Err(error(id.clone(), -32601, &format!("método desconhecido: {method}"))),
    };
    Some(match result {
        Ok(r) => json!({ "jsonrpc": "2.0", "id": id, "result": r }),
        Err(e) => e,
    })
}

/// As ferramentas que a IDE oferece (a lista está no `aylis-tools`, que a ponte também usa).
pub fn tools() -> Vec<Value> {
    aylis_tools::mcp::ide_tools()
}

fn text_result(text: impl Into<String>, is_error: bool) -> Value {
    json!({ "content": [{ "type": "text", "text": text.into() }], "isError": is_error })
}

/// Roda `f` na thread da interface e espera a resposta (até 60 s).
fn on_ui<R: Send + 'static>(poster: &Poster, f: impl FnOnce(&mut Ui, &Shared, &Ids) -> R + Send + 'static) -> Result<R, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    poster.post(move |ui| {
        let ctx = CONTEXT.with(|c| c.borrow().clone());
        if let Some((state, ids)) = ctx {
            let _ = tx.send(f(ui, &state, &ids));
            crate::app::refresh(ui, &state, &ids);
        }
    });
    rx.recv_timeout(Duration::from_secs(60)).map_err(|_| "a IDE não respondeu".to_string())
}

fn call_tool(name: &str, args: Value, poster: &Poster) -> Value {
    // O app em execução responde por outra thread: a chamada não prende a interface.
    if name == "app_command" {
        let link = on_ui(poster, |_, state, _| state.borrow().dev.clone().filter(|d| d.connected()));
        let command = args.get("command").cloned().unwrap_or(Value::Null);
        let snapshot = command["cmd"] == "snapshot";
        let result = match link {
            Ok(Some(dev)) => dev.call(command),
            _ => return text_result("nenhum app em execução conectado (use run e espere abrir)", true),
        };
        return match result {
            // A captura vai como imagem do MCP: o agente vê a tela do app (relatório do
            // RTSPlayer, 10/11: só havia a árvore, que num app médio passa de 130 mil caracteres).
            Ok(Value::String(png)) if snapshot => json!({ "content": [{ "type": "image", "data": png, "mimeType": "image/png" }], "isError": false }),
            Ok(v) => text_result(serde_json::to_string_pretty(&v).unwrap_or_default(), false),
            Err(e) => text_result(e, true),
        };
    }
    if name == "build" && args.get("wait").and_then(Value::as_bool).unwrap_or(true) {
        return build_and_wait(args, poster);
    }
    if name == "build_wait" {
        return build_wait(&args, poster);
    }
    // Executar: espera a compilação e devolve os erros dela (a resposta saía na hora com
    // "Executando…", e a compilação que falhava depois não era avisada; relatório do
    // RTSPlayer, 10/11).
    if matches!(name, "run" | "run_without_debug" | "debug") {
        let tool = name.to_string();
        let wait = args_wait(&args);
        let first = on_ui(poster, move |ui, state, ids| {
            let result = run_tool(ui, state, ids, &tool, &args);
            (state.borrow().job.is_some(), result)
        });
        return match first {
            Ok((true, Ok(_))) => after_run(&wait, poster),
            Ok((_, Ok(v))) => text_result(if let Value::String(s) = &v { s.clone() } else { v.to_string() }, false),
            Ok((_, Err(e))) | Err(e) => text_result(e, true),
        };
    }
    let name = name.to_string();
    match on_ui(poster, move |ui, state, ids| run_tool(ui, state, ids, &name, &args)) {
        Ok(Ok(v)) => text_result(if let Value::String(s) = &v { s.clone() } else { serde_json::to_string_pretty(&v).unwrap_or_default() }, false),
        Ok(Err(e)) => text_result(e, true),
        Err(e) => text_result(e, true),
    }
}

/// Uma consulta na thread da interface sem refazer a IDE (o `on_ui` refaz a cada chamada).
fn peek<R: Send + 'static>(poster: &Poster, f: impl FnOnce(&Shared) -> R + Send + 'static) -> Result<R, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    poster.post(move |_| {
        if let Some((state, _)) = CONTEXT.with(|c| c.borrow().clone()) {
            let _ = tx.send(f(&state));
        }
    });
    rx.recv_timeout(Duration::from_secs(60)).map_err(|_| "a IDE não respondeu".to_string())
}

/// `build`: começa a compilação e espera ela terminar, nesta thread (a da conexão, não a da
/// interface: a IDE segue respondendo). A resposta diz se compilou, com os erros e as últimas
/// linhas (pedido do autor em 08/11: "a sessão tem que ficar aguardando o Aylis compilar e o Aylis
/// deve avisar quando terminar"). Até `max_wait` (45 s por padrão; depois, `build_wait`);
/// `wait: false` só começa, como antes.
fn build_and_wait(args: Value, poster: &Poster) -> Value {
    let wait = args_wait(&args);
    let started = on_ui(poster, move |ui, state, ids| {
        // Um trabalho já em andamento (o app executando, outra compilação): recusa, em vez de
        // esperar por ele como se fosse esta compilação (conferência de 08/11).
        if let Some(title) = state.borrow().job.as_ref().map(|j| j.title.clone()) {
            return (false, Err(format!("já há um trabalho em andamento ({title}): pare com o `stop` antes de compilar")));
        }
        let status = run_tool(ui, state, ids, "build", &args);
        (state.borrow().job.is_some(), status)
    });
    let status = match started {
        Ok((true, _)) => None,
        Ok((false, Ok(status))) => Some(match status {
            Value::String(s) => s,
            other => other.to_string(),
        }),
        Ok((false, Err(e))) | Err(e) => Some(e),
    };
    if let Some(status) = status {
        // Não começou (sem projeto, outra linguagem, um trabalho em andamento...): a situação diz.
        return text_result(status, true);
    }
    match wait_job(poster, wait, false) {
        Ok(Phase::Done) => job_summary(poster),
        Ok(_) => still_compiling(wait),
        Err(e) => text_result(e, true),
    }
}

/// Quanto uma chamada espera (`max_wait`, em segundos): 45 por padrão, dentro do prazo de um
/// cliente MCP (a primeira compilação de um projeto passa disso, e o cliente desistia da chamada
/// sem resposta; relatório do RTSPlayer, 10/11). O resto se espera com `build_wait`.
fn args_wait(args: &Value) -> u64 {
    args.get("max_wait").and_then(Value::as_u64).unwrap_or(45).clamp(1, 30 * 60)
}

/// Onde o trabalho estava quando a espera acabou.
enum Phase {
    /// Terminou (compilou, falhou, ou o app fechou).
    Done,
    /// A compilação acabou e o app está rodando.
    Running,
    /// Ainda compilando.
    Busy,
}

/// Espera o trabalho da IDE terminar (ou, com `until_program`, o app começar), até `seconds`.
fn wait_job(poster: &Poster, seconds: u64, until_program: bool) -> Result<Phase, String> {
    let deadline = std::time::Instant::now() + Duration::from_secs(seconds);
    loop {
        let phase = peek(poster, move |state| match state.borrow().job.as_ref() {
            None => Some(Phase::Done),
            Some(job) if until_program && job.program_started() => Some(Phase::Running),
            Some(_) => None,
        })?;
        if let Some(phase) = phase {
            return Ok(phase);
        }
        if std::time::Instant::now() >= deadline {
            return Ok(Phase::Busy);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn still_compiling(waited: u64) -> Value {
    text_result(format!("{{\"compiling\": true, \"note\": \"ainda compilando depois de {waited} s: chame build_wait para esperar mais (o resultado vem quando terminar)\"}}"), false)
}

/// `build_wait`: espera a compilação em andamento (do `build` ou do `run`) e devolve o
/// resultado; sem nada em andamento, o da última.
fn build_wait(args: &Value, poster: &Poster) -> Value {
    match wait_job(poster, args_wait(args), true) {
        Ok(Phase::Done) => job_summary(poster),
        Ok(Phase::Running) => text_result("{\"compiled\": true, \"running\": true, \"note\": \"compilou; o app está rodando (app_command fala com ele)\"}", false),
        Ok(Phase::Busy) => still_compiling(args_wait(args)),
        Err(e) => text_result(e, true),
    }
}

/// Depois de mandar executar: espera a compilação. Compilou e abriu: espera um pouco para ver se
/// o app não cai logo (um pânico na inicialização). Falhou: os erros, como no `build`.
fn after_run(wait: &u64, poster: &Poster) -> Value {
    match wait_job(poster, *wait, true) {
        Ok(Phase::Running) => {
            std::thread::sleep(Duration::from_secs(2));
            match peek(poster, |state| state.borrow().job.is_none()) {
                Ok(true) => {
                    let mut v = job_summary(poster);
                    if let Some(text) = v["content"][0]["text"].as_str() {
                        v["content"][0]["text"] = Value::String(format!("o app abriu e fechou logo em seguida (veja as últimas linhas):\n{text}"));
                    }
                    v["isError"] = Value::Bool(true);
                    v
                }
                _ => text_result("{\"compiled\": true, \"running\": true, \"note\": \"o app está rodando (app_command fala com ele)\"}", false),
            }
        }
        Ok(Phase::Done) => job_summary(poster),
        Ok(Phase::Busy) => still_compiling(*wait),
        Err(e) => text_result(e, true),
    }
}

/// O resultado da última compilação (o que as MENSAGENS mostram): se compilou, os erros e as
/// últimas linhas.
fn job_summary(poster: &Poster) -> Value {
    let summary = peek(poster, |state| {
        use crate::jobs::OutKind;
        let st = state.borrow();
        let ok = st.output.iter().any(|l| l.kind == OutKind::Success);
        let errors: Vec<String> = st.output.iter().filter(|l| l.kind == OutKind::Error).take(40).map(|l| l.text.clone()).collect();
        let last: Vec<String> = st.output.iter().rev().take(15).rev().map(|l| l.text.clone()).collect();
        (ok, json!({ "compiled": ok, "status": st.status, "errors": errors, "last_lines": last }))
    });
    match summary {
        Ok((ok, v)) => text_result(serde_json::to_string_pretty(&v).unwrap_or_default(), !ok),
        Err(e) => text_result(e, true),
    }
}

fn arg<'a>(args: &'a Value, key: &str) -> &'a str {
    args.get(key).and_then(Value::as_str).unwrap_or_default()
}

/// O arquivo de código: um caminho, ou o nome de um form do projeto (vazio: o form aberto).
fn code_file(state: &Shared, file: &str) -> Result<std::path::PathBuf, String> {
    let path = std::path::PathBuf::from(file);
    if !file.is_empty() && path.is_file() {
        return Ok(path);
    }
    let st = state.borrow();
    let project = st.project.as_ref().ok_or("abra um projeto antes (open_project)")?;
    let form = if file.is_empty() { st.doc.form_name() } else { file.to_string() };
    aylis_project::codegen::form_source(project, &form).ok_or_else(|| format!("o form {form} não tem código"))
}

/// O item de menu de um caminho (`"Executar/Configurar linguagens…"`) nos forms abertos.
fn menu_item(ui: &Ui, path: &str) -> Option<aylis_core::ComponentId> {
    let norm = |t: &str| t.replace(['&', '…'], "").replace("...", "").trim().to_lowercase();
    let wanted: Vec<String> = path.split('/').map(norm).filter(|p| !p.is_empty()).collect();
    for form in ui.open_forms() {
        let mut all = vec![form];
        let mut i = 0;
        while i < all.len() {
            all.extend(ui.children(all[i]).iter().copied());
            i += 1;
        }
        for id in all {
            // O caminho do item: os textos dele e dos itens de menu acima.
            let mut texts = Vec::new();
            let mut cur = Some(id);
            while let Some(c) = cur {
                match ui.get::<aylis_controls::MenuItem>(c) {
                    Some(m) => texts.push(norm(&m.text)),
                    None => break,
                }
                cur = ui.parent(c);
            }
            texts.reverse();
            if !texts.is_empty() && texts == wanted {
                return Some(id);
            }
        }
    }
    None
}

/// Onde fica um menu de contexto (`PopupMenu`): o painel da IDE que tem o controle dono dele
/// e, no Object Inspector, "(propriedade)" no lugar da linha (cada propriedade tem o seu, com
/// os mesmos itens).
fn context_place(ui: &Ui, ids: &Ids, popup: aylis_core::ComponentId) -> String {
    let areas = [
        (ids.inspector, "Object Inspector › (propriedade)"),
        (ids.structure, "Estrutura"),
        (ids.project, "Projeto"),
        (ids.palette, "Paleta"),
        (ids.messages_panel, "Mensagens"),
        (ids.tabs, "Abas dos forms"),
        (ids.code_host, "Código"),
        (ids.surface, "Designer"),
        (ids.home, "Tela inicial"),
    ];
    let mut cur = ui.parent(popup);
    while let Some(c) = cur {
        if let Some((_, name)) = areas.iter().find(|(id, _)| *id == c) {
            return name.to_string();
        }
        cur = ui.parent(c);
    }
    "Menu de contexto".into()
}

/// Os itens de menu dos forms abertos (a IDE e a janela de código), com o caminho que o
/// `menu` aceita. Os menus de contexto (clique direito) vêm com o lugar na frente
/// ("Object Inspector › (propriedade) › Ligar a campo…") e `context`, uma vez só cada um:
/// a mesma entrada repetida em cada linha não diz nada a mais ao agente.
fn menu_items(ui: &Ui, ids: &Ids) -> Vec<Value> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut forms = ui.open_forms();
    if !forms.contains(&ids.form) {
        forms.insert(0, ids.form);
    }
    for form in forms {
        for id in descendants(ui, form) {
            let Some(item) = ui.get::<aylis_controls::MenuItem>(id) else { continue };
            // Só as folhas (os submenus são o caminho), sem os separadores.
            if item.text == "-" || ui.children(id).iter().any(|&c| ui.get::<aylis_controls::MenuItem>(c).is_some()) {
                continue;
            }
            let mut texts = Vec::new();
            let mut cur = Some(id);
            let mut top = id;
            while let Some(c) = cur {
                match ui.get::<aylis_controls::MenuItem>(c) {
                    Some(m) => texts.push(m.text.replace('&', "")),
                    None => break,
                }
                top = c;
                cur = ui.parent(c);
            }
            texts.reverse();
            let popup = ui.parent(top).filter(|&p| ui.class_name(p) == Some("PopupMenu"));
            let entry = match popup {
                Some(p) => {
                    let path = format!("{} › {}", context_place(ui, ids, p), texts.join(" › "));
                    seen.insert(path.clone()).then(|| json!({ "path": path, "context": true, "shortcut": item.shortcut, "enabled": item.enabled, "checked": item.checked }))
                }
                None => {
                    let path = texts.join("/");
                    seen.insert(path.clone()).then(|| json!({ "path": path, "shortcut": item.shortcut, "enabled": item.enabled, "checked": item.checked }))
                }
            };
            out.extend(entry);
        }
    }
    out
}

/// Um alvo pelo nome (`desktop`, `linux`, `android`, `web`, `ios`, ou o nome mostrado).
fn target_of(name: &str) -> Result<aylis_tools::Target, String> {
    let wanted = name.trim().to_lowercase();
    aylis_tools::Target::ALL
        .into_iter()
        .find(|t| format!("{t:?}").to_lowercase() == wanted || t.name().to_lowercase() == wanted)
        .ok_or_else(|| format!("alvo desconhecido: {name} (desktop, linux, android, web, ios)"))
}

/// O diálogo por cima (o último form aberto que não é a IDE nem a janela de código).
fn top_dialog(ui: &Ui) -> Option<aylis_core::ComponentId> {
    let code = crate::codewindow::code_form(ui);
    let forms = ui.open_forms();
    forms.iter().skip(1).rev().copied().find(|&f| Some(f) != code)
}

/// O diálogo por cima, para os `dialog_*` que mexem nele: a pergunta "Confiar neste
/// projeto?" fica de fora (só o usuário confirma; um agente não libera um projeto de fora).
fn automatable_dialog(ui: &Ui) -> Result<aylis_core::ComponentId, String> {
    let form = top_dialog(ui).ok_or("nenhum diálogo aberto")?;
    if ui.name(form) == Some(crate::trust::DIALOG) {
        return Err("a confiança num projeto de fora só pode ser confirmada pelo usuário, na IDE".into());
    }
    Ok(form)
}

/// O form e todos os componentes dentro dele, de cima para baixo.
fn descendants(ui: &Ui, form: aylis_core::ComponentId) -> Vec<aylis_core::ComponentId> {
    let mut all = vec![form];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    all
}

/// O componente aparece: ele e os pais até o form estão visíveis (o assistente esconde as
/// páginas que não são a do passo, e os botões que não valem nele).
fn shown(ui: &Ui, id: aylis_core::ComponentId) -> bool {
    let mut cur = Some(id);
    while let Some(c) = cur {
        if ui.control(c).is_some_and(|c| !c.visible()) {
            return false;
        }
        cur = ui.parent(c);
    }
    true
}

/// O rótulo de um campo (o `field` dos diálogos põe o texto num Label irmão, na mesma linha).
fn field_caption(ui: &Ui, id: aylis_core::ComponentId) -> String {
    let Some(parent) = ui.parent(id) else { return String::new() };
    ui.children(parent).iter().find_map(|&c| ui.get::<aylis_controls::Label>(c).map(|l| l.text.clone())).unwrap_or_default()
}

/// Os ComboBox e as ListBox visíveis do diálogo, na ordem da tela (a ordem é o `index` do
/// `dialog_select`).
fn dialog_choosers(ui: &Ui, form: aylis_core::ComponentId) -> Vec<aylis_core::ComponentId> {
    descendants(ui, form)
        .into_iter()
        .filter(|&c| (ui.get::<aylis_controls::ComboBox>(c).is_some() || ui.get::<aylis_controls::ListBox>(c).is_some()) && shown(ui, c))
        .collect()
}

/// O título, os campos (Edit), as escolhas (ComboBox), as listas (ListBox), as caixas e os
/// botões de um diálogo; só o que aparece (as páginas escondidas do assistente ficam de fora).
fn dialog_json(ui: &Ui, form: aylis_core::ComponentId) -> Value {
    let all: Vec<_> = descendants(ui, form).into_iter().filter(|&c| shown(ui, c)).collect();
    let title = match ui.get_prop(form, "caption") {
        Ok(aylis_core::PropValue::String(s)) => s,
        _ => String::new(),
    };
    let fields: Vec<String> = all.iter().filter_map(|&c| ui.get::<aylis_controls::Edit>(c).map(|e| e.text.clone())).collect();
    let combos: Vec<Value> = all
        .iter()
        .filter_map(|&c| ui.get::<aylis_controls::ComboBox>(c).map(|b| (c, b)))
        .map(|(c, b)| json!({ "label": field_caption(ui, c), "text": b.text(), "selected": b.item_index, "items": b.item_list(), "list": dialog_choosers(ui, form).iter().position(|&x| x == c) }))
        .collect();
    let lists: Vec<Value> = all
        .iter()
        .filter_map(|&c| ui.get::<aylis_controls::ListBox>(c).map(|l| (c, l)))
        .map(|(c, l)| {
            let items: Vec<&str> = l.item_list().iter().map(String::as_str).take(30).collect();
            json!({ "items": items, "count": l.count(), "selected": l.item_index, "list": dialog_choosers(ui, form).iter().position(|&x| x == c) })
        })
        .collect();
    let error = all.iter().filter_map(|&c| ui.get::<aylis_controls::Label>(c)).map(|l| l.text.clone()).find(|t| t.starts_with('⚠')).unwrap_or_default();
    let checks: Vec<Value> = all.iter().filter_map(|&c| ui.get::<aylis_controls::CheckBox>(c)).map(|c| json!({ "text": c.text, "checked": c.checked })).collect();
    let buttons: Vec<Value> = all
        .iter()
        .filter_map(|&c| ui.get::<aylis_controls::Button>(c).map(|b| (c, b)))
        .filter(|(_, b)| !b.text.trim().is_empty())
        .map(|(c, b)| json!({ "text": b.text.replace('&', ""), "enabled": ui.control(c).is_some_and(|c| c.enabled()) }))
        .collect();
    let labels: Vec<String> = all.iter().filter_map(|&c| ui.get::<aylis_controls::Label>(c)).map(|l| l.text.clone()).filter(|t| !t.trim().is_empty()).collect();
    let memo = all.iter().find_map(|&c| ui.get::<aylis_controls::Memo>(c)).map(|m| m.text.clone()).unwrap_or_default();
    json!({ "title": title, "fields": fields, "combos": combos, "lists": lists, "checks": checks, "buttons": buttons, "labels": labels, "text": memo, "error": error })
}

/// Escolhe um item de um ComboBox ou ListBox do diálogo, como o usuário: muda o `item_index`
/// e dispara o `on_change` (o diálogo reage como ao clique). `list` é a posição em
/// `dialog_choosers`, ou o rótulo do campo; sem ele, a primeira lista que tiver o item.
/// `item` é o texto (sem diferença de maiúsculas) ou, se for um número, o índice.
fn dialog_select(ui: &mut Ui, form: aylis_core::ComponentId, list: &Value, item: &Value) -> Result<(), String> {
    let items_of = |ui: &Ui, c: aylis_core::ComponentId| -> Vec<String> {
        match ui.get::<aylis_controls::ComboBox>(c) {
            Some(b) => b.item_list().iter().map(|s| s.to_string()).collect(),
            None => ui.get::<aylis_controls::ListBox>(c).map(|l| l.item_list().to_vec()).unwrap_or_default(),
        }
    };
    let index_in = |items: &[String]| -> Option<usize> {
        match item {
            Value::Number(n) => n.as_u64().map(|n| n as usize).filter(|&n| n < items.len()),
            Value::String(t) => items.iter().position(|i| i.eq_ignore_ascii_case(t.trim())),
            _ => None,
        }
    };
    let choosers = dialog_choosers(ui, form);
    if choosers.is_empty() {
        return Err("o diálogo não tem lista nem caixa de escolha".into());
    }
    let wanted = |t: &str| t.replace(['&', ':'], "").trim().to_lowercase();
    let candidates: Vec<_> = match list {
        Value::Number(n) => vec![*n.as_u64().and_then(|n| choosers.get(n as usize)).ok_or_else(|| format!("o diálogo tem {} lista(s) (0 a {})", choosers.len(), choosers.len() - 1))?],
        Value::String(t) if !t.trim().is_empty() => {
            let found: Vec<_> = choosers.iter().copied().filter(|&c| wanted(&field_caption(ui, c)) == wanted(t)).collect();
            if found.is_empty() {
                return Err(format!("o diálogo não tem a lista '{t}'"));
            }
            found
        }
        _ => choosers,
    };
    let (id, index) = candidates
        .iter()
        .find_map(|&c| index_in(&items_of(ui, c)).map(|i| (c, i)))
        .ok_or_else(|| format!("nenhuma lista do diálogo tem o item {item}"))?;
    if ui.control(id).is_some_and(|c| !c.enabled()) {
        return Err("a lista está desabilitada".into());
    }
    let before = ui.get_prop(id, "item_index").ok().and_then(|v| v.to_i64().ok()).unwrap_or(-1);
    ui.set_prop(id, "item_index", index as i64).map_err(|e| e.to_string())?;
    if before != index as i64 {
        let text = items_of(ui, id).get(index).cloned().unwrap_or_default();
        ui.fire(id, "on_change", &aylis_core::EventArgs::Text(text));
    }
    Ok(())
}

/// Um fonte: o caminho dado, ou relativo à pasta do código do projeto.
fn source_file(state: &Shared, path: &str) -> Result<std::path::PathBuf, String> {
    let direct = std::path::PathBuf::from(path);
    if direct.is_file() {
        return Ok(direct);
    }
    let st = state.borrow();
    let p = st.project.as_ref().ok_or("abra um projeto antes (open_project)")?;
    let inside = p.code_dir().join(path);
    if inside.is_file() { Ok(inside) } else { Err(format!("não achei o fonte {path}")) }
}

/// Confere o layout inteiro e devolve **todos** os erros de uma vez; sem erro, troca o form
/// aberto (com `dry_run`, só confere). Antes parava no primeiro erro, e um layout de 24 KB com
/// cinco erros ia e voltava cinco vezes (relatório do RTSPlayer, 10/11).
fn apply_layout(state: &Shared, layout: &str, dry_run: bool) -> Result<Value, String> {
    let errors = {
        let mut st = state.borrow_mut();
        aylis_json::validate(&mut st.doc.ui, layout)
    };
    if !errors.is_empty() {
        let list: Vec<String> = errors.iter().map(ToString::to_string).collect();
        return Err(format!("{} erro(s) no layout (nada foi trocado):\n- {}", list.len(), list.join("\n- ")));
    }
    if dry_run {
        return Ok(Value::String("o layout carrega sem erros (dry_run: nada foi trocado)".into()));
    }
    state.borrow_mut().doc.replace_json(layout)?;
    Ok(Value::String("layout trocado".into()))
}

/// Troca o nó com o nome `name` (na árvore do JSON do layout) por `node`.
fn replace_node(at: &mut Value, name: &str, node: Value) -> bool {
    if at.get("name").and_then(Value::as_str) == Some(name) {
        *at = node;
        return true;
    }
    let Some(children) = at.get_mut("children").and_then(Value::as_array_mut) else { return false };
    let mut node = Some(node);
    for child in children {
        if let Some(n) = node.take() {
            if replace_node(child, name, n.clone()) {
                return true;
            }
            node = Some(n);
        }
    }
    false
}

/// O fonte de `read_code`/`edit_code`: `file` (caminho, ou relativo à pasta do código) ou o
/// código do form `form` (vazio: o aberto).
fn code_target(state: &Shared, args: &Value) -> Result<std::path::PathBuf, String> {
    match arg(args, "file") {
        "" => code_file(state, arg(args, "form")),
        path => source_file(state, path),
    }
}

/// Acha o componente pelo nome no form aberto (vazio: o form).
fn component(state: &Shared, name: &str) -> Result<aylis_core::ComponentId, String> {
    let st = state.borrow();
    if name.is_empty() {
        return Ok(st.doc.root);
    }
    st.doc.ui.find_in(st.doc.root, name).ok_or_else(|| format!("o form aberto não tem o componente '{name}'"))
}

/// Uma ferramenta, na thread da interface.
pub fn run_tool(ui: &mut Ui, state: &Shared, ids: &Ids, name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "project_info" => {
            let st = state.borrow();
            let open = json!({ "form": st.doc.form_name(), "file": st.doc.path.as_ref().map(|p| p.display().to_string()), "modified": st.doc.modified });
            let Some(p) = &st.project else { return Ok(json!({ "project": null, "open": open })) };
            let forms: Vec<Value> = p
                .forms
                .iter()
                .map(|f| {
                    json!({
                        "name": f.name, "file": f.file, "main": f.name == p.main_form, "auto_create": f.auto_create,
                        "data_module": p.is_data_module(&f.name), "variants": p.variants_of(&f.name), "uses": p.uses_of(&f.name),
                    })
                })
                .collect();
            Ok(json!({ "project": p.name, "dir": p.dir().display().to_string(), "language": p.language.name(), "forms": forms, "open": open }))
        }
        "open_form" => {
            let form = arg(args, "name").to_string();
            let variant = Some(arg(args, "variant")).filter(|v| !v.is_empty()).map(str::to_string);
            if state.borrow().project.as_ref().and_then(|p| p.form(&form)).is_none() {
                return Err(format!("o projeto não tem o form '{form}'"));
            }
            crate::actions::open_project_variant(state, &form, variant.as_deref());
            let st = state.borrow();
            if st.status.starts_with('⚠') {
                return Err(st.status.clone());
            }
            Ok(Value::String(format!("aberto: {}", st.doc.title())))
        }
        "list_components" => {
            let st = state.borrow();
            let doc = &st.doc;
            let list: Vec<Value> = doc
                .tree()
                .into_iter()
                .map(|(id, depth)| {
                    json!({
                        "name": doc.ui.name(id).unwrap_or_default(),
                        "class": doc.ui.class_name(id).unwrap_or_default(),
                        "parent": doc.ui.parent(id).and_then(|p| doc.ui.name(p)).unwrap_or_default(),
                        "depth": depth,
                    })
                })
                .collect();
            Ok(Value::Array(list))
        }
        "get_component" => {
            let id = component(state, arg(args, "component"))?;
            let st = state.borrow();
            let mut props = st.doc.props_json(id);
            props.insert("class".into(), Value::from(st.doc.ui.class_name(id).unwrap_or_default()));
            Ok(Value::Object(props))
        }
        "set_property" => {
            let id = component(state, arg(args, "component"))?;
            let property = arg(args, "property");
            let value = args.get("value").cloned().unwrap_or(Value::Null);
            // Um evento vazio (`""` ou `null`, que o agente às vezes manda como texto) desliga o
            // handler, como apagar no Object Inspector: antes dava erro ou ligava a um "null".
            let is_event = property.starts_with("on_") || property.starts_with("before_") || property.starts_with("after_");
            let value = if is_event && matches!(value.as_str(), Some("") | Some("null")) { Value::Null } else { value };
            let mut st = state.borrow_mut();
            st.doc.begin_change();
            if property == "name" {
                st.doc.rename_component(id, value.as_str().unwrap_or_default())?;
            } else {
                st.doc.set_prop(id, property, &value)?;
            }
            Ok(Value::String(format!("{property} alterado")))
        }
        "add_component" => {
            let class = arg(args, "class");
            let parent = arg(args, "parent");
            let mut st = state.borrow_mut();
            let parent_id = if parent.is_empty() { None } else { Some(st.doc.ui.find_in(st.doc.root, parent).ok_or_else(|| format!("não achei '{parent}'"))?) };
            // O dono escolhido vira a seleção: `add` põe os não visuais nele e os visuais
            // no contêiner sob o ponto (aqui, o canto dele).
            if let Some(p) = parent_id {
                st.doc.select(Some(p));
            }
            let at = parent_id.and_then(|p| st.doc.rect_of(p)).map(|r| aylis_core::PointF::new(r.x + 8.0, r.y + 8.0));
            st.doc.begin_change();
            let id = st.doc.add(class, at)?;
            let wanted = arg(args, "name");
            if !wanted.is_empty() {
                st.doc.rename_component(id, wanted)?;
            }
            st.doc.select(Some(id));
            Ok(Value::String(st.doc.ui.name(id).unwrap_or_default().to_string()))
        }
        "delete_component" => {
            let id = component(state, arg(args, "component"))?;
            let mut st = state.borrow_mut();
            if id == st.doc.root {
                return Err("o form não pode ser excluído".into());
            }
            st.doc.select(Some(id));
            st.doc.delete_selected();
            Ok(Value::String("excluído".into()))
        }
        "get_layout" => {
            let json = state.borrow().doc.to_json()?;
            Ok(serde_json::from_str(&json).map_err(|e| e.to_string())?)
        }
        "set_layout" => {
            // O layout pelo texto, ou por um arquivo (`layout_file`: um layout grande não precisa
            // ir e voltar inteiro na conversa).
            let layout = match (args.get("layout"), arg(args, "layout_file")) {
                (Some(Value::String(s)), _) => s.clone(),
                (Some(v), _) if !v.is_null() => v.to_string(),
                (_, file) if !file.is_empty() => std::fs::read_to_string(file).map_err(|e| format!("não consegui ler {file}: {e}"))?,
                _ => return Err("falta \"layout\" (ou \"layout_file\")".into()),
            };
            apply_layout(state, &layout, args.get("dry_run").and_then(Value::as_bool).unwrap_or(false))
        }
        "patch_layout" => {
            // Troca só o componente `component` (e os filhos dele) pelo JSON dado, sem mandar o
            // form inteiro.
            let name = arg(args, "component");
            let node: Value = match args.get("layout") {
                Some(Value::String(s)) => serde_json::from_str(s).map_err(|e| format!("JSON inválido: {e}"))?,
                Some(v) if v.is_object() => v.clone(),
                _ => return Err("falta \"layout\" (o objeto do componente, com class, name, propriedades e children)".into()),
            };
            let mut doc: Value = serde_json::from_str(&state.borrow().doc.to_json()?).map_err(|e| e.to_string())?;
            if !replace_node(&mut doc["root"], name, node) {
                return Err(format!("o form aberto não tem o componente '{name}'"));
            }
            apply_layout(state, &doc.to_string(), args.get("dry_run").and_then(Value::as_bool).unwrap_or(false))
        }
        "class_info" => {
            let class = arg(args, "class");
            let st = state.borrow();
            aylis_json::class_info(&st.doc.ui, class).ok_or_else(|| format!("não há a classe '{class}' (list_classes mostra as da paleta)"))
        }
        "list_classes" => {
            let st = state.borrow();
            let mut by_category: std::collections::BTreeMap<String, Vec<Value>> = Default::default();
            for c in st.doc.ui.registry().iter().filter(|c| c.name != "Form" && c.name != "DataModule") {
                let variants: Vec<String> = st.doc.ui.variants_of(c.name).into_iter().map(|v| v.name).collect();
                by_category.entry(c.category.to_string()).or_default().push(json!({ "class": c.name, "variants": variants }));
            }
            Ok(json!(by_category))
        }
        "save" => {
            if args.get("all").and_then(Value::as_bool).unwrap_or(false) {
                let modified = {
                    let st = state.borrow();
                    (0..st.tab_count()).filter(|&i| st.tab(i).is_some_and(|d| d.modified)).count()
                };
                // O save_all devolve os que ficaram sem salvar (forms novos, sem arquivo).
                let unsaved = crate::actions::save_all(state);
                let saved = modified - unsaved.min(modified);
                // E o código alterado nos editores (o do edit_code também).
                let code = crate::codewindow::save_embedded(ui, true);
                Ok(Value::String(if unsaved > 0 {
                    format!("{saved} form(s) e {code} fonte(s) salvo(s); {unsaved} form(s) sem arquivo (salve pela IDE)")
                } else {
                    format!("{saved} form(s) e {code} fonte(s) salvo(s)")
                }))
            } else {
                if state.borrow().doc.path.is_none() {
                    return Err("o form não tem arquivo (sem projeto): salve pela IDE".into());
                }
                crate::actions::save(state, false);
                Ok(Value::String(state.borrow().status.clone()))
            }
        }
        "generate_code" => {
            let files = crate::actions::generate_code(ui, state)?;
            Ok(json!(files.iter().map(|f| f.display().to_string()).collect::<Vec<_>>()))
        }
        "open_project" => {
            let path = std::path::PathBuf::from(arg(args, "path"));
            // Já aberto (outra sessão trouxe esta para cá, ou o usuário abriu): fica como está,
            // sem recarregar e perder o que ainda não foi salvo.
            let open = state.borrow().project.as_ref().is_some_and(|p| aylis_tools::instances::same_path(&p.path, &path));
            if !open {
                crate::actions::open_recent(ui, state, ids, &path);
            }
            let st = state.borrow();
            match &st.project {
                Some(p) => Ok(json!({ "project": p.name, "language": p.language.name(), "status": st.status })),
                None => Err(st.status.clone()),
            }
        }
        "open_code" => {
            let file = match arg(args, "file") {
                "" => code_file(state, arg(args, "form"))?,
                path => source_file(state, path)?,
            };
            let line = args.get("line").and_then(Value::as_u64).unwrap_or(1) as usize;
            crate::codewindow::open(ui, &file, line)?;
            Ok(Value::String(file.display().to_string()))
        }
        "read_code" => {
            let file = code_target(state, args)?;
            let (text, unsaved) = match crate::codewindow::text_of(ui, &file) {
                Some(text) => {
                    let unsaved = crate::codewindow::embedded_tabs(ui).iter().any(|(f, m)| *m && *f == crate::debugger::norm(&file));
                    (text, unsaved)
                }
                None => (std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?, false),
            };
            let total = text.lines().count();
            let from = args.get("start_line").and_then(Value::as_u64).unwrap_or(1).max(1) as usize;
            let to = args.get("end_line").and_then(Value::as_u64).map_or(total, |n| n as usize).min(total);
            let part = if from == 1 && to == total { text } else { text.lines().skip(from - 1).take(to.saturating_sub(from - 1)).collect::<Vec<_>>().join("\n") };
            let generated = file.components().any(|c| c.as_os_str() == "generated");
            Ok(json!({ "file": file.display().to_string(), "lines": total, "start_line": from, "end_line": to, "unsaved": unsaved, "read_only": generated, "text": part }))
        }
        "edit_code" => {
            // Um fonte que ainda não existe, com `old` vazio: é criado com `new` (o arquivo por
            // assunto; relatório do RTSPlayer, 10/11).
            let wanted = arg(args, "file");
            if arg(args, "old").is_empty() && !wanted.is_empty() && source_file(state, wanted).is_err() {
                let project = state.borrow().project.clone().ok_or("abra um projeto antes (open_project)")?;
                let file = crate::actions::new_source(&project, wanted, arg(args, "new"))?;
                let note = if project.language == aylis_project::Language::Rust { "criado; declare o mod dele (no lib.rs ou no módulo pai) com edit_code" } else { "criado" };
                return Ok(json!({ "file": file.display().to_string(), "created": true, "note": note }));
            }
            let file = code_target(state, args)?;
            if file.extension().is_some_and(|e| e == "json") && file.components().any(|c| c.as_os_str() == "forms") {
                return Err("os forms mudam pelo designer: set_property, add_component, set_layout (o arquivo é regravado ao salvar)".into());
            }
            let all = args.get("all").and_then(Value::as_bool).unwrap_or(false);
            let (count, line) = crate::codewindow::edit_text(ui, &file, arg(args, "old"), arg(args, "new"), all)?;
            let saved = args.get("save").and_then(Value::as_bool).unwrap_or(false) && crate::codewindow::save_file(ui, &file);
            Ok(json!({ "file": file.display().to_string(), "replaced": count, "line": line, "saved": saved }))
        }
        "editor_type" => {
            let form = crate::codewindow::code_form(ui).ok_or("a janela de código não está aberta (open_code)")?;
            ui.text_input(form, arg(args, "text"));
            Ok(crate::codewindow::editor_state(ui))
        }
        "editor_key" => {
            let form = crate::codewindow::code_form(ui).ok_or("a janela de código não está aberta (open_code)")?;
            let (modifiers, key) = aylis_core::parse_shortcut(arg(args, "key")).ok_or("tecla inválida")?;
            let event = aylis_core::KeyEvent { key, modifiers, repeat: false };
            ui.key_down(form, event);
            ui.key_up(form, event);
            Ok(crate::codewindow::editor_state(ui))
        }
        "editor_hover" => {
            let line = args.get("line").and_then(Value::as_u64).unwrap_or(1) as usize;
            let column = args.get("column").and_then(Value::as_u64).unwrap_or(1) as usize;
            if !crate::codewindow::hover_at(ui, line, column) {
                return Err("a janela de código não está aberta".into());
            }
            Ok(Value::String("pedido".into()))
        }
        "editor_state" => Ok(crate::codewindow::editor_state(ui)),
        "dialog_state" => {
            let form = top_dialog(ui).ok_or("nenhum diálogo aberto")?;
            Ok(dialog_json(ui, form))
        }
        "dialog_check" => {
            let form = automatable_dialog(ui)?;
            let text = arg(args, "text");
            let checked = args.get("checked").and_then(Value::as_bool).unwrap_or(true);
            let mut all = vec![form];
            let mut i = 0;
            while i < all.len() {
                all.extend(ui.children(all[i]).iter().copied());
                i += 1;
            }
            let id = all
                .into_iter()
                .find(|&c| ui.get::<aylis_controls::CheckBox>(c).is_some_and(|b| b.text.eq_ignore_ascii_case(text)))
                .ok_or_else(|| format!("o diálogo não tem a caixa '{text}'"))?;
            let _ = ui.set_prop(id, "checked", checked);
            Ok(dialog_json(ui, form))
        }
        "dialog_select" => {
            let form = automatable_dialog(ui)?;
            let list = args.get("list").cloned().unwrap_or(Value::Null);
            let item = args.get("item").cloned().ok_or("falta `item` (o texto ou o índice)")?;
            dialog_select(ui, form, &list, &item)?;
            Ok(match top_dialog(ui) {
                Some(f) if f == form => dialog_json(ui, f),
                _ => json!({ "closed": true }),
            })
        }
        "dialog_click" => {
            let form = automatable_dialog(ui)?;
            let text = arg(args, "text");
            let mut all = vec![form];
            let mut i = 0;
            while i < all.len() {
                all.extend(ui.children(all[i]).iter().copied());
                i += 1;
            }
            let id = all
                .into_iter()
                .find(|&c| ui.get::<aylis_controls::Button>(c).is_some_and(|b| b.text.replace('&', "").eq_ignore_ascii_case(text)))
                .ok_or_else(|| format!("o diálogo não tem o botão '{text}'"))?;
            if ui.control(id).is_some_and(|c| !c.enabled()) {
                return Err("o botão está desabilitado".into());
            }
            ui.click(id);
            Ok(match top_dialog(ui) {
                Some(f) if f == form => dialog_json(ui, f),
                _ => json!({ "closed": true }),
            })
        }
        "menu" => {
            let id = menu_item(ui, arg(args, "path")).ok_or_else(|| format!("item de menu não encontrado: {}", arg(args, "path")))?;
            if ui.get::<aylis_controls::MenuItem>(id).is_some_and(|m| !m.enabled) {
                return Err("o item está desabilitado".into());
            }
            ui.fire(id, "on_click", &aylis_core::EventArgs::None);
            Ok(json!({ "dialog": top_dialog(ui).map(|f| dialog_json(ui, f)) }))
        }
        "dialog_type" | "dialog_key" => {
            let form = automatable_dialog(ui)?;
            if name == "dialog_type" {
                ui.text_input(form, arg(args, "text"));
            } else {
                let (modifiers, key) = aylis_core::parse_shortcut(arg(args, "key")).ok_or("tecla inválida")?;
                let event = aylis_core::KeyEvent { key, modifiers, repeat: false };
                ui.key_down(form, event);
                ui.key_up(form, event);
            }
            Ok(match top_dialog(ui) {
                Some(f) if f == form => dialog_json(ui, f),
                _ => json!({ "closed": true }),
            })
        }
        "sources" => {
            let st = state.borrow();
            let p = st.project.as_ref().ok_or("abra um projeto antes (open_project)")?;
            let dir = p.code_dir();
            let list: Vec<String> = aylis_project::symbols::sources(p)
                .iter()
                .map(|f| f.strip_prefix(&dir).unwrap_or(f).to_string_lossy().replace('\\', "/"))
                .collect();
            Ok(json!({ "dir": dir.display().to_string(), "files": list }))
        }
        "usages" => {
            let project = state.borrow().project.clone().ok_or("abra um projeto antes (open_project)")?;
            let target = match (arg(args, "handler"), arg(args, "form"), arg(args, "component")) {
                (h, _, _) if !h.is_empty() => crate::usages::Target::Handler { name: h.to_string() },
                (_, form, component) if !form.is_empty() && !component.is_empty() => {
                    let entry = project.form(form).ok_or_else(|| format!("o projeto não tem o form '{form}'"))?;
                    let layout: Value = std::fs::read_to_string(project.file(&entry.file)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
                    crate::usages::target_of(&layout, form, component)
                }
                _ => return Err("informe `form` e `component`, ou `handler`".into()),
            };
            let results = crate::usages::find(&project, &target)?;
            let dir = project.dir();
            let code: Vec<Value> = results.text.iter().map(|h| json!({ "file": h.file.strip_prefix(&dir).unwrap_or(&h.file).to_string_lossy().replace('\\', "/"), "line": h.line + 1, "text": h.text.trim() })).collect();
            let forms: Vec<Value> = results.forms.iter().map(|h| json!({ "form": h.form, "component": h.component, "property": h.property, "value": h.value })).collect();
            Ok(json!({ "target": target.label(), "code": code, "forms": forms }))
        }
        "breakpoint" => {
            let file = code_file(state, arg(args, "file"))?;
            let line = args.get("line").and_then(Value::as_u64).filter(|&l| l > 0).ok_or("falta `line` (a partir de 1)")? as usize;
            let on = args.get("on").and_then(Value::as_bool).unwrap_or(true);
            if crate::debugger::breakpoints_of(state, &file).contains(&line) != on {
                crate::debugger::toggle_breakpoint(ui, state, &file, line);
            }
            Ok(json!({ "file": file.display().to_string(), "lines": crate::debugger::breakpoints_of(state, &file) }))
        }
        "run" => {
            crate::debugger::run(ui, state);
            Ok(Value::String(state.borrow().status.clone()))
        }
        "debug" => {
            restricted(state)?;
            crate::debugger::start(ui, state);
            Ok(Value::String(state.borrow().status.clone()))
        }
        "debug_command" => {
            use crate::debugger::Cmd;
            let cmd = match arg(args, "command") {
                "continue" => Cmd::Continue,
                "step_over" => Cmd::StepOver,
                "step_in" => Cmd::StepIn,
                "step_out" => Cmd::StepOut,
                "pause" => Cmd::Pause,
                "stop" => Cmd::Stop,
                "run_to" => {
                    let file = code_file(state, arg(args, "file"))?;
                    let line = args.get("line").and_then(Value::as_u64).ok_or("falta `line`")? as usize;
                    crate::debugger::run_to_cursor(ui, state, &file, line);
                    return Ok(Value::String(state.borrow().status.clone()));
                }
                other => return Err(format!("comando de depuração desconhecido: {other}")),
            };
            crate::debugger::command(ui, state, cmd);
            Ok(Value::String(state.borrow().status.clone()))
        }
        "debug_state" => Ok(crate::debugger::snapshot(state, Some(arg(args, "evaluate")))),
        "status" => Ok(Value::String(state.borrow().status.clone())),
        "stop" => {
            crate::actions::stop(state);
            Ok(Value::String("parando".into()))
        }
        "output" => {
            let n = args.get("lines").and_then(Value::as_u64).unwrap_or(50) as usize;
            let st = state.borrow();
            let lines: Vec<&str> = st.output.iter().rev().take(n).map(|l| l.text.as_str()).collect();
            Ok(Value::String(lines.into_iter().rev().collect::<Vec<_>>().join("\n")))
        }
        "ide_menus" => Ok(Value::Array(menu_items(ui, ids))),
        "new_project" => {
            let dir = std::path::PathBuf::from(arg(args, "dir"));
            let language = match arg(args, "language") {
                "" => aylis_project::Language::Rust,
                l => aylis_project::Language::from_name(l).ok_or_else(|| format!("linguagem desconhecida: {l}"))?,
            };
            std::fs::create_dir_all(&dir).map_err(|e| format!("não consegui criar {}: {e}", dir.display()))?;
            // `here`: o projeto direto em `dir`, que pode já ter outras coisas (o app dentro de um
            // repositório); sem ele, numa pasta nova `dir/<nome>`.
            let here = args.get("here").and_then(Value::as_bool).unwrap_or(false);
            let project = if here { aylis_project::create_at(&dir, arg(args, "name"), language)? } else { aylis_project::create(&dir, arg(args, "name"), language)? };
            // Criado aqui: confiável (um projeto que já existia não passa por este caminho).
            aylis_tools::trust::trust(&project.dir())?;
            crate::actions::open_recent(ui, state, ids, &project.path);
            let mut notes = vec!["Confiável nesta pasta: mover o projeto para outra pasta conta como projeto de fora (modo restrito até o usuário confiar). Para criar onde ele vai ficar, use `here`.".to_string()];
            // O gancho que impede gravar direto nos arquivos que a IDE guarda (com o MCP ligado).
            if project.dir().join(".claude").join("settings.local.json").is_file() {
                notes.push("Criado .claude/settings.local.json: o gancho que impede o agente de gravar direto nos arquivos que a IDE guarda (fora do git, pelo .claude/.gitignore).".into());
            }
            Ok(json!({ "project": project.name, "path": project.path.display().to_string(), "status": state.borrow().status, "notes": notes }))
        }
        "close_project" => {
            crate::actions::close_project(ui, state);
            Ok(Value::String("projeto fechado".into()))
        }
        "new_form" => {
            let form = arg(args, "name").to_string();
            if !aylis_project::is_identifier(&form) {
                return Err("use letras, números e _ (começando com letra)".into());
            }
            let result = {
                let mut st = state.borrow_mut();
                let project = st.project.as_mut().ok_or("abra um projeto antes (open_project)")?;
                if project.form(&form).is_some() {
                    return Err(format!("o projeto já tem um form '{form}'"));
                }
                let added = if args.get("data_module").and_then(Value::as_bool).unwrap_or(false) { project.add_data_module(&form) } else { project.add_form(&form, &form) };
                added.and_then(|_| project.save())
            };
            result?;
            crate::actions::open_project_form(state, &form);
            Ok(Value::String(format!("form {form} criado e aberto")))
        }
        "remove_form" => {
            let form = arg(args, "name").to_string();
            let mut st = state.borrow_mut();
            let project = st.project.as_mut().ok_or("abra um projeto antes (open_project)")?;
            project.remove_form(&form).and_then(|_| project.save())?;
            if let Some(i) = st.find_tab(&form) {
                st.close_tab(i);
            }
            Ok(Value::String(format!("form {form} removido do projeto")))
        }
        "set_main_form" => {
            let form = arg(args, "name");
            if state.borrow().project.as_ref().and_then(|p| p.form(form)).is_none() {
                return Err(format!("o projeto não tem o form '{form}'"));
            }
            crate::actions::set_main_form(state, form);
            Ok(Value::String(state.borrow().status.clone()))
        }
        "project_settings" => {
            let st = state.borrow();
            let p = st.project.as_ref().ok_or("abra um projeto antes (open_project)")?;
            let mut v = serde_json::to_value(p).map_err(|e| e.to_string())?;
            v["path"] = Value::from(p.path.display().to_string());
            Ok(v)
        }
        "set_project_setting" => {
            let key = arg(args, "key");
            if key.is_empty() {
                return Err("falta `key`".into());
            }
            let value = args.get("value").cloned().unwrap_or(Value::Null);
            let mut st = state.borrow_mut();
            let p = st.project.as_ref().ok_or("abra um projeto antes (open_project)")?;
            let with = |value: Value| -> Result<aylis_project::Project, String> {
                let mut v = serde_json::to_value(p).map_err(|e| e.to_string())?;
                let mut slot = &mut v;
                for part in key.split('.') {
                    if !slot.is_object() {
                        *slot = json!({});
                    }
                    slot = &mut slot[part];
                }
                *slot = value;
                serde_json::from_value(v).map_err(|e| format!("valor inválido para {key}: {e}"))
            };
            // O valor como veio; um texto que é JSON (`"\"Meu App\""`, `"[1,2]"`), também lido como
            // JSON: o agente manda das duas formas (conferência de 08/11).
            let parsed = value.as_str().and_then(|t| serde_json::from_str::<Value>(t).ok());
            let mut updated = match parsed {
                Some(parsed) if value.as_str().is_some_and(|t| t.trim_start().starts_with(['"', '[', '{'])) => with(parsed).or_else(|_| with(value.clone()))?,
                Some(parsed) => with(value.clone()).or_else(|_| with(parsed))?,
                None => with(value)?,
            };
            updated.path = p.path.clone();
            let language_changed = updated.language != p.language;
            updated.save()?;
            // Outra linguagem: o código dela em dia com o visual, e a situação com a nova (dizia
            // ainda "aberto (Free Pascal)" com o projeto em Rust).
            if language_changed {
                crate::codegen::generate(&updated)?;
                st.status = format!("Linguagem: {} (código em {}/)", updated.language.name(), updated.language.folder());
            } else {
                st.status = format!("{key} alterado e salvo");
            }
            st.project = Some(updated);
            Ok(Value::String(st.status.clone()))
        }
        "targets" => {
            let st = state.borrow();
            let list: Vec<Value> = aylis_tools::Target::ALL.iter().map(|t| json!({ "id": format!("{t:?}").to_lowercase(), "name": t.name(), "selected": *t == st.run_target })).collect();
            Ok(Value::Array(list))
        }
        "set_target" => {
            let target = target_of(arg(args, "target"))?;
            state.borrow_mut().run_target = target;
            Ok(Value::String(format!("alvo: {}", target.name())))
        }
        "build" => {
            restricted(state)?;
            crate::codewindow::save_all(ui);
            crate::jobs::build(state, args.get("release").and_then(Value::as_bool).unwrap_or(false));
            Ok(Value::String(state.borrow().status.clone()))
        }
        "run_without_debug" => {
            restricted(state)?;
            if args.get("release").and_then(Value::as_bool).unwrap_or(false) {
                crate::actions::run_release(ui, state);
            } else {
                crate::actions::run_project(ui, state);
            }
            Ok(Value::String(state.borrow().status.clone()))
        }
        "publish" => {
            let target = match arg(args, "target") {
                "" => state.borrow().run_target,
                t => target_of(t)?,
            };
            let out_dir = Some(arg(args, "out_dir")).filter(|d| !d.is_empty()).map(std::path::PathBuf::from);
            restricted(state)?;
            crate::codewindow::save_all(ui);
            crate::jobs::publish(state, target, out_dir);
            Ok(Value::String(state.borrow().status.clone()))
        }
        "undo" | "redo" => {
            let mut st = state.borrow_mut();
            let done = if name == "undo" { st.doc.undo() } else { st.doc.redo() };
            Ok(Value::String(if done { format!("{name} feito") } else { "nada a fazer".into() }))
        }
        "select" => {
            let names: Vec<String> = args.get("components").and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            let found = names.iter().map(|n| component(state, n)).collect::<Result<Vec<_>, _>>()?;
            state.borrow_mut().doc.select_many(found);
            Ok(Value::String(format!("{} selecionado(s)", names.len())))
        }
        "debug_variables" => {
            let reference = args.get("reference").and_then(Value::as_i64).filter(|r| *r > 0).ok_or("falta `reference` (o de uma variável com campos)")?;
            let vars = crate::debugger::children(state, reference)?;
            Ok(Value::Array(vars.iter().take(500).map(|v| json!({ "name": v.name, "value": v.value, "type": v.kind, "reference": v.children })).collect()))
        }
        "debug_frame" => {
            let index = args.get("index").and_then(Value::as_u64).ok_or("falta `index`")? as usize;
            crate::debugger::select_frame(ui, state, index)?;
            Ok(crate::debugger::snapshot(state, None))
        }
        "breakpoint_condition" => {
            let file = code_file(state, arg(args, "file"))?;
            let line = args.get("line").and_then(Value::as_u64).filter(|&l| l > 0).ok_or("falta `line` (a partir de 1)")? as usize;
            crate::debugger::set_condition(ui, state, &file, line, arg(args, "condition"), arg(args, "hits"));
            Ok(json!({ "file": file.display().to_string(), "line": line, "condition": crate::debugger::condition_of(state, &file, line) }))
        }
        _ => Err(format!("ferramenta desconhecida: {name}")),
    }
}

/// Compilar, executar e publicar num projeto em modo restrito: o erro, para o agente dizer ao
/// usuário (a confiança é só dele, pelo diálogo da IDE; o MCP não confirma).
fn restricted(state: &Shared) -> Result<(), String> {
    if state.borrow().restricted {
        return Err("o projeto está em modo restrito (não foi criado nem confiado nesta pasta; um projeto movido de pasta também conta como de fora): compilar, executar e publicar ficam desligados até o usuário confiar nele na IDE (o diálogo \"Confiar neste projeto?\" ou Projeto → Opções do projeto → Confiar neste projeto). Peça isso ao usuário; o MCP não confirma.".into());
    }
    Ok(())
}
