//! # aylis-remote
//!
//! O protocolo do aylis: a tela é o frontend e o backend pode ser em **qualquer
//! linguagem**. Tudo é JSON:
//!
//! - **comandos** (backend → tela): criam e alteram componentes, mexem nos dados;
//! - **respostas**: quando o comando tem `"id"`, volta `{"id": .., "result": ..}` ou
//!   `{"id": .., "error": ".."}`;
//! - **eventos** (tela → backend): os eventos ligados a um handler por nome que não foi
//!   registrado em Rust, por exemplo `"on_click": "btn_salvar_click"`.
//!
//! O mesmo [`handle_message`] serve para um backend em outro processo (`aylis-host`, uma
//! mensagem por linha em stdin/stdout) e para a C ABI (`aylis-capi`, no mesmo processo).
//!
//! ## Comandos
//!
//! | `cmd` | Campos | Resultado |
//! |---|---|---|
//! | `load` | `layout` (documento ou nó), `parent`? | `{id, name}` |
//! | `create` | `class`, `parent`?, `name`?, `props`? | `{id}` |
//! | `set` | `target`, `prop` e `value` ou `props` | — |
//! | `get` | `target`, `prop` ou `props: [..]` | valor ou objeto |
//! | `free`, `click`, `focus` | `target` | — |
//! | `find` | `name` | id ou `null` |
//! | `save` | `target` | documento de layout |
//! | `theme` | `theme` (JSON de tema) ou `name: "light"/"dark"` | — |
//! | `locale` | `locale: "pt-BR"/"en-US"` | — |
//! | `data` | `target` (ClientDataSet), `op`, ... (veja abaixo) | depende |
//! | `batch` | `commands: [..]` | lista de resultados |
//! | `create_form` | `name`, `new`? | cria um form do projeto (como `Application.CreateForm`); se já existe, devolve o mesmo (com `new: true`, mais uma instância, outra janela): `{id, name}` |
//! | `show` | `target` (form) | abre numa janela (como `Show`) |
//! | `show_modal` | `target` (form), `handler`?, `wait`? | abre como modal; ao fechar, o `handler` recebe o evento `on_modal_result` com o resultado (`"ok"`, `"cancel"`...); com `wait: true`, a resposta (o resultado) só sai quando o form fecha, e o cliente trata os eventos enquanto espera (o `ShowModal` do Delphi) |
//! | `close` | `target`? | fecha o form (passa pelo `on_close_query`; devolve se fechou); sem `target`, encerra o aplicativo |
//! | `modal_result` | `target` (form), `result` | define o resultado (num modal, fecha) |
//! | `tree` | `target` (opcional) | árvore de componentes (id, nome, classe, caminho, filhos) |
//! | `props` | `target` | todas as propriedades: nome, tipo, categoria, opções e valor |
//! | `highlight` | `target` | destaca o controle na tela por um instante |
//! | `tr` | `text` | o texto traduzido para o idioma atual |
//! | `set_language` | `language` (`en-US`) | troca o idioma: formatos, traduções e textos abertos |
//! | `toast` | `text` | aviso rápido que some sozinho |
//! | `message_dlg` | `text`, `kind` (`information`, `warning`, `error`, `confirmation`), `buttons` (`["yes", "no"]`), `handler`, `wait`? | caixa de mensagem; o handler recebe `on_modal_result`; com `wait: true`, a resposta é o botão (`"yes"`), quando a caixa fecha |
//! | `execute` | `target` (OpenDialog ou SaveDialog) | abre o diálogo de arquivo; o resultado chega em `on_select`/`on_cancel` |
//! | `media` | `target` (MediaPlayer ou VideoPlayer), `op`: `play`, `pause`, `stop`, `seek` (`seconds`), `position`, `duration`, `playing` | depende da operação |
//! | `hello` | `protocol`, `client`?, `update_url`? | a **primeira** mensagem do backend: a versão do protocolo ([`PROTOCOL_VERSION`]); a tela devolve `{protocol, capabilities}` ou recusa, dizendo qual lado atualizar (e, na tela antiga, "há uma versão nova do aplicativo" com o `update_url`). Antes dela, todo comando é recusado |
//! | `exception` | `message`, `path`?, `handler`? | um handler do backend falhou: a tela mostra o erro (a caixa do aylis ou o `on_exception` da aplicação), como um erro do próprio app |
//! | `components` | `target` (o form) | os componentes da instância do form: `[{"name", "id", "class"}]` (as classes de form dos clientes se montam com eles) |
//! | `wake` | — | devolve o evento `{"event": "__post"}`: o `Post` de outra thread do backend acorda o laço dele, parado lendo a entrada (o `TThread.Queue` do Delphi) |
//! | `done` | `seq` | o handler do evento `seq` terminou (o cliente manda sozinho; a tela deixa de mostrar o backend ocupado). Só vale para o cliente que anuncia `"features": ["done"]` no `hello` |
//! | `answer` | `ask`, `values`? ou `error`? | a resposta de um evento-pergunta (o evento veio com `ask`): `values` com os campos respondidos (`can_close`, `key`, `key_char`, `data`, `label`, `accept`) ou `error` para recusar (a mensagem vai para a tela) |
//!
//! `target`/`parent` é o **nome** do componente, um **caminho** com pontos
//! (`frmClientes.btnSalvar`, sem ambiguidade quando dois forms têm componentes com o
//! mesmo nome) ou o **id** numérico.
//!
//! Operações de `data`: `load` (`rows`), `load_columns` (`fields`, `columns`: uma lista de valores
//! por campo, para a carga grande), `append` (`rows`, `has_more`), `reconcile`
//! (`results`), `first`, `prior`, `next`, `last`, `move_to` (`index`), `edit`, `insert`,
//! `append_record`, `post`, `cancel`, `delete`, `apply`, `cancel_updates`, `refresh`,
//! `set_value` (`field`, `value`), `set_text` (`field`, `text`), `value` (`field`),
//! `text` (`field`), `record`, `info`, `delta`, `set_has_more` (`value`),
//! `set_values` (`values`: objeto com vários campos, numa ida e volta só), `locate`
//! (`field`, `value`), `set_filter` (`filter`), `set_index` (`index`), `lookup`
//! (`key_fields`, `keys`, `result_field`), `set_index_name` (`name`), `set_param`
//! (`name`, `value`), `revert_record`, `field_texts` (`field`, `texts`) e `field_error`
//! (`field`, `message`).
//!
//! ## Eventos de Field
//!
//! Um `Field` com `"on_get_text": "uf_nome"` tem o texto de exibição dado pelo backend,
//! sem uma ida e volta por célula: a tela mostra o valor, junta os que faltam e manda
//! **um** evento (`args: {"field": "uf", "values": ["SP", "RJ"]}`); o backend responde
//! quando puder com `{"cmd": "data", "op": "field_texts", "field": "uf", "texts":
//! [["SP", "São Paulo"], ["RJ", "Rio de Janeiro"]]}` e os textos ficam guardados por valor.
//!
//! Com `"on_validate": "cpf_valido"`, cada valor novo vai ao backend
//! (`args: {"field", "value", "text"}`) e o valor entra na hora; a resposta
//! `{"op": "field_error", "field": "cpf", "message": "CPF inválido"}` faz o `post` recusar
//! até chegar outro valor ou `message: null`.
//!
//! ## Eventos
//!
//! ```json
//! {"event": "on_click", "handler": "btn_salvar_click", "form": "frmClientes",
//!  "sender": {"id": 4294967301, "name": "btnSalvar", "path": "frmClientes.btnSalvar", "class": "Button"},
//!  "args": null}
//! ```
//!
//! `form` é o form do componente: o backend pode ter handlers com o mesmo nome em forms
//! diferentes (`frmClientes.btn_salvar_click`). `form_id` é a instância dele: o mesmo form
//! aberto duas vezes manda ids diferentes, e o cliente fala com a instância certa.
//!
//! O delta de `on_apply_updates` chega em `args` já como objeto JSON.
//!
//! Cada evento leva `seq` (com o cliente que responde `done`); `{"event": "__freed", "forms":
//! [ids]}` avisa os forms liberados (o cliente descarta as instâncias e os `PostTo` deles).
//!
//! ## Eventos que perguntam
//!
//! `on_close_query`, `before_post`, `before_delete`, `on_validate` (Field), `on_key_down` e
//! `on_key_press` (ligados a um handler remoto), `on_drag_start` e `on_drag_over` esperam a
//! resposta: o evento chega com `"ask": n` e a ação (fechar, gravar, a tecla, o arrasto)
//! fica suspensa até o backend mandar `{"cmd": "answer", "ask": n, "values": {..}}` (aceito,
//! com os campos respondidos) ou `{"cmd": "answer", "ask": n, "error": "Informe o CPF"}`
//! (recusado). Os clientes respondem sozinhos quando o handler termina (a exceção recusa) e
//! tratam um evento-pergunta mesmo enquanto esperam a resposta de um comando (o `post` do
//! próprio backend dispara o `before_post`).

pub mod inspect;
pub mod mcp;
pub mod script;

use aylis_core::platform::LocaleInfo;
use aylis_core::{ComponentId, EventArgs, Key, ModalResult, PointerButton, PropEnum, PropValue, Theme, Ui};
use aylis_data::json::{json_to_value, value_to_json};
use aylis_data::{DataSet, data_set};
use serde_json::{Map, Value, json};

thread_local! {
    /// O `id` do comando que roda agora (um comando que responde depois o pega com
    /// [`defer_reply`]).
    static CURRENT_ID: std::cell::RefCell<Option<Value>> = const { std::cell::RefCell::new(None) };
    /// Os `post`/`delete`/`apply` do backend que esperam um evento-pergunta: (id, dataset, operação).
    /// As marcas do profiler abertas pelo backend (`profile_begin`): o nome e o começo.
    static PROFILE_MARKS: std::cell::RefCell<Vec<(String, f64)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Registra todas as classes padrão (controles, dados e controles de dados).
pub fn register_all(ui: &mut Ui) {
    aylis_controls::register(ui);
    aylis_data::register(ui);
    aylis_dbcontrols::register(ui);
    aylis_providers::register(ui);
    aylis_media::register(ui);
    aylis_vector::register(ui);
    aylis_markdown::register(ui);
    aylis_webview::register(ui);
    mcp::register(ui);
}

/// A versão do protocolo. Sobe só numa mudança incompatível; a tela e o backend precisam
/// da mesma (o `hello`): versões diferentes se recusam, com a mensagem dizendo qual lado
/// atualizar.
pub const PROTOCOL_VERSION: u64 = 1;

/// O que esta tela sabe fazer além do básico (vai na resposta do `hello`).
pub const CAPABILITIES: &[&str] = &["wake", "exception", "deflate", "ask"];

/// Acima disto, a ponte manda a mensagem compactada para a tela que anunciou `deflate`.
pub const COMPRESS_ABOVE: usize = 1024;

/// A compactação entre a ponte e a tela: o texto da mensagem (uma ou mais linhas JSON) num
/// quadro binário com deflate. O JSON do aylis, cheio de nomes repetidos, fica de 5 a 10
/// vezes menor.
pub fn compress(text: &str) -> Vec<u8> {
    miniz_oxide::deflate::compress_to_vec(text.as_bytes(), 6)
}

/// O inverso de [`compress`]: `None` se o quadro não for deflate de um texto UTF-8.
pub fn decompress(bytes: &[u8]) -> Option<String> {
    let raw = miniz_oxide::inflate::decompress_to_vec(bytes).ok()?;
    String::from_utf8(raw).ok()
}

/// A resposta do `hello` que anuncia `deflate` (a ponte olha as mensagens que vêm da tela).
pub fn announces_deflate(line: &str) -> bool {
    line.contains("\"capabilities\"")
        && serde_json::from_str::<Value>(line).ok().is_some_and(|m| {
            m["result"]["capabilities"].as_array().is_some_and(|c| c.iter().any(|v| v == "deflate"))
        })
}

/// O aperto de mão: a primeira mensagem do backend tem de ser o `hello`. Quem fala com um
/// backend (o `aylis-host`, o app ligado ao servidor, a tela no navegador) guarda um destes
/// e passa cada mensagem por [`Handshake::check`] antes de [`handle_message`].
#[derive(Default)]
pub struct Handshake {
    greeted: bool,
}

impl Handshake {
    /// `None`: siga com a mensagem. `Some(resposta)`: recusada (o backend não se apresentou
    /// antes); mande a resposta e não execute.
    pub fn check(&mut self, message: &Value) -> Option<Value> {
        if message.get("cmd").and_then(Value::as_str) == Some("hello") {
            self.greeted = true;
            return None;
        }
        if self.greeted {
            return None;
        }
        let error = "o backend não se apresentou: a primeira mensagem tem de ser {\"cmd\": \"hello\", \"protocol\": 1} (atualize o cliente do aylis)";
        Some(match message.get("id") {
            Some(id) => json!({ "id": id, "error": error }),
            None => json!({ "event": "error", "cmd": message.get("cmd").cloned().unwrap_or(Value::Null), "message": error }),
        })
    }
}

/// O `hello` do backend: a versão confere, ou a tela recusa e mostra ao usuário o que fazer
/// (a tela antiga: "há uma versão nova do aplicativo", com o link da loja que o backend
/// mandou em `update_url`).
fn hello(ui: &mut Ui, obj: &Map<String, Value>) -> Result<Value, String> {
    let theirs = obj.get("protocol").and_then(Value::as_u64).ok_or("hello: falta \"protocol\" (a versão do protocolo do cliente)")?;
    // O cliente que responde `done` ao fim de cada handler (os de antes não): só com ele a
    // tela sabe que o backend está ocupado.
    let done = obj.get("features").and_then(Value::as_array).is_some_and(|f| f.iter().any(|v| v == "done"));
    ui.extension::<Remote>().busy.tracking = done;
    if theirs == PROTOCOL_VERSION {
        return Ok(json!({ "protocol": PROTOCOL_VERSION, "capabilities": CAPABILITIES }));
    }
    let update_url = obj.get("update_url").and_then(Value::as_str).unwrap_or_default();
    let (user, backend) = if theirs > PROTOCOL_VERSION {
        let mut text = aylis_core::messages::t("Há uma versão nova do aplicativo. Atualize para continuar.").to_string();
        if !update_url.is_empty() {
            text = format!("{text}\n{update_url}");
        }
        (text, format!("a tela é do protocolo {PROTOCOL_VERSION} e o backend do {theirs}: atualize o aplicativo (a tela)"))
    } else {
        (
            aylis_core::messages::t("O servidor do aplicativo é de uma versão anterior. Tente de novo mais tarde.").to_string(),
            format!("a tela é do protocolo {PROTOCOL_VERSION} e o backend do {theirs}: atualize o backend (o cliente do aylis)"),
        )
    };
    ui.raise_exception(aylis_core::Exception { message: user, path: String::new(), event: "hello".into() });
    Err(backend)
}

/// Processa uma mensagem do backend. Devolve a resposta (se a mensagem tinha `"id"`) ou,
/// sem `"id"`, um evento `error` quando o comando falha (para o erro não sumir).
pub fn handle_message(ui: &mut Ui, message: &Value) -> Option<Value> {
    // O eco do `Post`: volta como evento, na ordem das outras mensagens.
    if message.get("cmd").and_then(Value::as_str) == Some("wake") {
        return Some(json!({ "event": "__post" }));
    }
    let id = message.get("id").cloned();
    CURRENT_ID.with(|c| *c.borrow_mut() = id.clone());
    let result = execute(ui, message);
    // O comando responde depois (o diálogo que espera, o `post` com `before_post` remoto).
    let deferred = id.is_some() && CURRENT_ID.with(|c| c.borrow_mut().take()).is_none();
    if deferred {
        return result.err().map(|error| json!({ "event": "error", "cmd": message.get("cmd").cloned().unwrap_or(Value::Null), "message": error }));
    }
    match (id, result) {
        (Some(id), Ok(result)) => Some(json!({ "id": id, "result": result })),
        (Some(id), Err(error)) => Some(json!({ "id": id, "error": error })),
        (None, Ok(_)) => None,
        (None, Err(error)) => {
            let cmd = message.get("cmd").cloned().unwrap_or(Value::Null);
            Some(json!({ "event": "error", "cmd": cmd, "message": error }))
        }
    }
}

/// Executa um comando e devolve o resultado.
pub fn execute(ui: &mut Ui, command: &Value) -> Result<Value, String> {
    let obj = command.as_object().ok_or("o comando deve ser um objeto JSON")?;
    let cmd = obj.get("cmd").and_then(Value::as_str).ok_or("falta \"cmd\"")?;
    match cmd {
        "load" => {
            let layout = obj.get("layout").ok_or("falta \"layout\"")?;
            let document = if layout.get("root").is_some() { layout.clone() } else { json!({ "root": layout }) };
            let parent = optional_target(ui, obj.get("parent"))?;
            let id = aylis_json::load_value(ui, &document, parent).map_err(|e| e.to_string())?;
            aylis_data::open_active(ui);
            Ok(json!({ "id": id.to_u64(), "name": ui.name(id) }))
        }
        "create" => {
            let class = string(obj, "class")?;
            let parent = optional_target(ui, obj.get("parent"))?;
            let id = ui.create(class, parent).map_err(|e| e.to_string())?;
            let setup = (|| {
                if let Some(name) = obj.get("name").and_then(Value::as_str) {
                    ui.set_name(id, name).map_err(|e| e.to_string())?;
                }
                if let Some(props) = obj.get("props") {
                    let props = props.as_object().ok_or("\"props\" deve ser um objeto")?;
                    // Os valores de quem nasce não disparam o `on_change` (como ao carregar).
                    ui.without_change_events(|ui| aylis_json::set_props(ui, id, props))?;
                }
                Ok::<_, String>(())
            })();
            if let Err(e) = setup {
                ui.free(id);
                return Err(e);
            }
            aylis_data::open_active(ui);
            Ok(json!({ "id": id.to_u64() }))
        }
        "set" => {
            let id = target(ui, obj)?;
            // Uma propriedade só (`prop` e `value`, como no `get`) ou várias (`props`).
            if let Some(name) = obj.get("prop").and_then(Value::as_str) {
                let value = obj.get("value").ok_or("falta \"value\" (com \"prop\")")?;
                let mut one = Map::new();
                one.insert(name.to_string(), value.clone());
                aylis_json::set_props(ui, id, &one)?;
                return Ok(Value::Null);
            }
            let props = obj.get("props").and_then(Value::as_object).ok_or("informe \"prop\" e \"value\", ou \"props\" (objeto)")?;
            aylis_json::set_props(ui, id, props)?;
            Ok(Value::Null)
        }
        "get" => {
            let id = target(ui, obj)?;
            if let Some(name) = obj.get("prop").and_then(Value::as_str) {
                return get(ui, id, name);
            }
            let names = obj.get("props").and_then(Value::as_array).ok_or("informe \"prop\" ou \"props\"")?;
            let mut out = Map::new();
            for name in names {
                let name = name.as_str().ok_or("\"props\" deve ser uma lista de nomes")?;
                out.insert(name.to_string(), get(ui, id, name)?);
            }
            Ok(Value::Object(out))
        }
        "free" => {
            let id = target(ui, obj)?;
            ui.free(id);
            Ok(Value::Null)
        }
        // Um método do componente: {"cmd": "invoke", "target": "pbDesenho", "method": "undo", "args": []}
        "invoke" => {
            let id = target(ui, obj)?;
            let method = string(obj, "method")?;
            let args: Vec<PropValue> = obj.get("args").and_then(Value::as_array).map(|a| a.iter().map(json_arg).collect()).unwrap_or_default();
            let result = ui.invoke(id, method, &args)?;
            Ok(aylis_json::prop_to_json(ui, &result))
        }
        "click" => {
            let id = target(ui, obj)?;
            ui.click(id);
            Ok(Value::Null)
        }
        // Os itens que o controle desenha (abas, linhas, itens de menu, células).
        "items" => {
            let id = target(ui, obj)?;
            let items: Vec<Value> = ui
                .access_items(id)
                .into_iter()
                .map(|it| {
                    serde_json::json!({
                        "role": format!("{:?}", it.role).to_lowercase(),
                        "name": it.name,
                        "selected": it.selected,
                        "checked": it.checked,
                        "disabled": it.disabled,
                        "parent": it.parent,
                        "row": it.row,
                        "column": it.column,
                        "level": it.level,
                        "expanded": it.expanded,
                    })
                })
                .collect();
            Ok(Value::Array(items))
        }
        // Clica num item pelo texto (`"item": "Clientes"`) ou pela posição (`"index": 2`);
        // `"action"`: `click` (padrão), `select`, `expand` ou `collapse`.
        "click_item" => {
            use aylis_core::automation::ItemAction;
            let id = target(ui, obj)?;
            let action = match obj.get("action").and_then(Value::as_str).unwrap_or("click") {
                "select" => ItemAction::Select,
                "expand" => ItemAction::Expand,
                "collapse" => ItemAction::Collapse,
                _ => ItemAction::Click,
            };
            let named = obj.get("item").and_then(Value::as_str).and_then(|name| ui.access_items(id).iter().position(|i| i.name.trim().eq_ignore_ascii_case(name.trim())));
            let done = match (named, obj.get("index").and_then(Value::as_u64)) {
                (Some(i), _) => ui.access_action(id, i, action),
                (None, Some(i)) => ui.access_action(id, i as usize, action),
                _ => return Err("click_item: informe \"item\" (o texto) ou \"index\"".into()),
            };
            if done { Ok(Value::Null) } else { Err("item não encontrado ou sem ação".into()) }
        }
        "focus" => {
            let id = target(ui, obj)?;
            Ok(Value::Bool(ui.set_focus(id)))
        }
        // O controle com foco no form (o nome; `null` sem foco), como o `ActiveControl`.
        "focused" => {
            let form = target(ui, obj)?;
            let root = ui.root_of(form);
            Ok(ui.focused(root).and_then(|f| ui.name(f).map(|n| Value::String(n.to_string()))).unwrap_or(Value::Null))
        }
        // Uma tecla no form (`"Enter"`, `"Shift+Tab"`, `"Alt+S"`), como se fosse apertada:
        // vai ao controle com foco, e valem o Tab, as `tab_keys` e o Alt+letra.
        "key" => {
            let form = target(ui, obj)?;
            let root = ui.root_of(form);
            let (modifiers, key) = aylis_core::parse_shortcut(string(obj, "key")?).ok_or("tecla inválida (ex.: \"Enter\", \"Shift+Tab\", \"Alt+S\")")?;
            let event = aylis_core::KeyEvent { key, modifiers, repeat: false };
            ui.key_down(root, event);
            ui.key_up(root, event);
            Ok(Value::Null)
        }
        // Texto digitado no controle com foco (passa pelos caracteres liberados e bloqueados).
        "type" => {
            let form = target(ui, obj)?;
            let root = ui.root_of(form);
            ui.text_input(root, string(obj, "text")?);
            Ok(Value::Null)
        }
        // Próximo / anterior na ordem do Tab, como o `SelectNext` do Delphi.
        "focus_next" | "focus_prev" => {
            let form = target(ui, obj)?;
            let root = ui.root_of(form);
            Ok(Value::Bool(ui.focus_next(root, cmd == "focus_next")))
        }
        "find" => {
            let name = string(obj, "name")?;
            Ok(ui.find(name).map_or(Value::Null, |id| Value::from(id.to_u64())))
        }
        "save" => {
            let id = target(ui, obj)?;
            aylis_json::save_value(ui, id).map_err(|e| e.to_string())
        }
        "theme" => {
            let theme = match (obj.get("theme"), obj.get("name").and_then(Value::as_str)) {
                (Some(theme), _) => aylis_json::theme_from_value(theme).map_err(|e| e.to_string())?,
                (None, Some("dark")) => Theme::dark(),
                (None, Some("light")) => Theme::light(),
                _ => return Err("informe \"theme\" (JSON) ou \"name\": \"light\"/\"dark\"".into()),
            };
            ui.set_theme(theme);
            Ok(Value::Null)
        }
        "locale" => {
            let locale = match string(obj, "locale")? {
                "pt-BR" | "pt_BR" | "pt" => LocaleInfo::pt_br(),
                "en-US" | "en_US" | "en" => LocaleInfo::en_us(),
                other => return Err(format!("idioma '{other}' não suportado (use pt-BR ou en-US)")),
            };
            ui.services_mut().locale = Some(Box::new(locale));
            Ok(Value::Null)
        }
        "data" => data_command(ui, obj),
        "batch" => {
            let commands = obj.get("commands").and_then(Value::as_array).ok_or("falta \"commands\" (lista)")?;
            Ok(Value::Array(
                commands
                    .iter()
                    .map(|c| execute(ui, c).unwrap_or_else(|e| json!({ "error": e })))
                    .collect(),
            ))
        }
        "create_form" => {
            // `new`: mais uma instância (outra janela do mesmo form), mesmo que ele já exista.
            let name = string(obj, "name")?;
            let fresh = obj.get("new").and_then(Value::as_bool).unwrap_or(false);
            let id = if fresh { ui.create_form_instance(name) } else { ui.create_form(name) }.map_err(|e| e.to_string())?;
            aylis_data::open_active(ui);
            Ok(json!({ "id": id.to_u64(), "name": ui.name(id) }))
        }
        "show" => {
            let id = target(ui, obj)?;
            ui.show(id).map_err(|e| e.to_string())?;
            Ok(Value::Null)
        }
        "show_modal" => {
            let id = target(ui, obj)?;
            let handler = obj.get("handler").and_then(Value::as_str).map(str::to_string);
            // `wait`: a resposta só sai quando o form fecha (o `ShowModal` que espera, como no Delphi).
            let reply = if obj.get("wait").and_then(Value::as_bool) == Some(true) { defer_reply(ui) } else { None };
            // Aberto pelo pedido que roda: o backend espera o usuário (não está travado).
            let waiting = ui.backend_busy();
            if waiting {
                ui.backend_waiting_user(true);
            }
            ui.show_modal(id, move |ui, result| {
                if waiting {
                    ui.backend_waiting_user(false);
                }
                if let Some(reply) = reply {
                    send_reply(ui, reply, Ok(json!(result.name())));
                }
                if let Some(handler) = handler {
                    let args = EventArgs::Value(PropValue::Enum(result.name().to_string()));
                    ui.call_handler(id, "on_modal_result", &handler, &args);
                }
            })
            .map_err(|e| e.to_string())?;
            Ok(Value::Null)
        }
        // Os componentes de uma instância de form (o cliente monta a classe do form com eles):
        // {"cmd": "components", "target": 4294967300} → [{"name", "id", "class"}...]. Só os
        // do próprio form (um form embutido num frame é outra instância).
        "components" => {
            let form = target(ui, obj)?;
            let mut out = Vec::new();
            let mut stack = ui.children(form).to_vec();
            while let Some(id) = stack.pop() {
                if ui.form_of(id) != form {
                    continue;
                }
                if let Some(name) = ui.name(id).filter(|n| !n.is_empty()) {
                    out.push(json!({ "name": name, "id": id.to_u64(), "class": ui.class_name(id).unwrap_or_default() }));
                }
                stack.extend(ui.children(id).iter().copied());
            }
            Ok(Value::Array(out))
        }
        "close" => match obj.get("target") {
            Some(t) => {
                let id = resolve(ui, t)?;
                Ok(Value::Bool(ui.close(id)))
            }
            None => {
                ui.request_close();
                Ok(Value::Null)
            }
        },
        "media" => media(ui, obj),
        "tree" => {
            let root = optional_target(ui, obj.get("target"))?;
            // `depth` (níveis abaixo da raiz) e `visible_only` para não trazer a árvore inteira.
            let depth = obj.get("depth").and_then(Value::as_u64).map_or(usize::MAX, |d| d as usize);
            let visible_only = obj.get("visible_only").and_then(Value::as_bool).unwrap_or(false);
            Ok(inspect::tree_with(ui, root, depth, visible_only))
        }
        "props" => {
            let id = target(ui, obj)?;
            Ok(inspect::props(ui, id))
        }
        "highlight" => {
            let id = target(ui, obj)?;
            inspect::highlight(ui, id)?;
            Ok(Value::Null)
        }
        "tr" => Ok(Value::String(ui.tr(string(obj, "text")?))),
        "set_language" => {
            aylis_project::i18n::set_language(ui, string(obj, "language")?);
            Ok(Value::Null)
        }
        "toast" => {
            aylis_controls::toast(ui, string(obj, "text")?);
            Ok(Value::Null)
        }
        // A marca à mão do profiler nas outras linguagens (`aylis.profile("nome")`): a tela
        // mede do começo ao fim (o relógio é o dela). Desligado, não guarda nada.
        "profile_begin" => {
            let name = string(obj, "name")?.to_string();
            PROFILE_MARKS.with(|m| m.borrow_mut().push((name, aylis_core::profile::now())));
            Ok(Value::Null)
        }
        "profile_end" => {
            if let Some((name, start)) = PROFILE_MARKS.with(|m| m.borrow_mut().pop()) {
                aylis_core::profile::record(aylis_core::profile::Lane::Mark, &name, start, (aylis_core::profile::now() - start).max(0.0));
            }
            Ok(Value::Null)
        }
        "message_dlg" => {
            use aylis_controls::DlgKind;
            let text = string(obj, "text")?.to_string();
            let kind = match obj.get("kind").and_then(Value::as_str).unwrap_or("information") {
                "warning" => DlgKind::Warning,
                "error" => DlgKind::Error,
                "confirmation" => DlgKind::Confirmation,
                _ => DlgKind::Information,
            };
            let buttons: Vec<ModalResult> = obj
                .get("buttons")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(|b| b.as_str().and_then(ModalResult::from_name)).collect())
                .unwrap_or_default();
            let handler = obj.get("handler").and_then(Value::as_str).map(str::to_string);
            let reply = if obj.get("wait").and_then(Value::as_bool) == Some(true) { defer_reply(ui) } else { None };
            let waiting = ui.backend_busy();
            if waiting {
                ui.backend_waiting_user(true);
            }
            let form = aylis_controls::message_dlg(ui, &text, kind, &buttons, move |ui, result| {
                if waiting {
                    ui.backend_waiting_user(false);
                }
                if let Some(reply) = reply {
                    send_reply(ui, reply, Ok(json!(result.name())));
                }
                if let (Some(handler), Some(main)) = (handler, ui.main_form()) {
                    let args = EventArgs::Value(PropValue::Enum(result.name().to_string()));
                    ui.call_handler(main, "on_modal_result", &handler, &args);
                }
            });
            Ok(json!(ui.path_of(form)))
        }
        "execute" => {
            let id = target(ui, obj)?;
            aylis_controls::OpenDialog::execute(ui, id);
            Ok(Value::Null)
        }
        "hello" => hello(ui, obj),
        // Um handler do backend falhou (o cliente pegou a exceção): a tela mostra como um erro
        // do próprio app (a caixa, ou o `on_exception` da aplicação).
        "exception" => {
            let message = string(obj, "message")?.to_string();
            let path = obj.get("path").and_then(Value::as_str).unwrap_or_default().to_string();
            let event = obj.get("handler").and_then(Value::as_str).unwrap_or_default().to_string();
            ui.raise_exception(aylis_core::Exception { message, path, event });
            Ok(Value::Null)
        }
        // A resposta de um evento-pergunta (o evento veio com `ask`).
        // O handler do evento `seq` terminou: o backend fica livre (e um evento que se juntou
        // enquanto isso sai agora).
        "done" => {
            let seq = obj.get("seq").and_then(Value::as_u64).ok_or("falta 'seq'")?;
            backend_done(ui, seq);
            Ok(Value::Null)
        }
        "answer" => {
            let token = obj.get("ask").and_then(Value::as_u64).ok_or("falta 'ask' (o número da pergunta)")?;
            let answer = match obj.get("error").and_then(Value::as_str) {
                Some(message) => aylis_core::Answer::Refused(message.to_string()),
                None => {
                    let values = obj.get("values").and_then(Value::as_object).map(|m| m.iter().map(|(k, v)| (k.clone(), answer_value(v))).collect()).unwrap_or_default();
                    aylis_core::Answer::Accepted(values)
                }
            };
            ui.answer(token, answer);
            flush_waiting_data(ui);
            Ok(Value::Null)
        }
        "modal_result" => {
            let id = target(ui, obj)?;
            let name = string(obj, "result")?;
            let result = ModalResult::from_name(name).ok_or_else(|| format!("resultado '{name}' inválido (use {})", ModalResult::NAMES.join(", ")))?;
            ui.set_modal_result(id, result);
            Ok(Value::Null)
        }
        other => Err(format!("comando desconhecido '{other}'")),
    }
}

/// Comandos dos players de mídia.
fn media(ui: &mut Ui, obj: &Map<String, Value>) -> Result<Value, String> {
    use aylis_media::{MediaPlayer, VideoPlayer};
    let id = target(ui, obj)?;
    let op = string(obj, "op")?;
    let video = ui.get::<VideoPlayer>(id).is_some();
    if !video && ui.get::<MediaPlayer>(id).is_none() {
        return Err("o alvo não é um MediaPlayer nem um VideoPlayer".into());
    }
    let seconds = obj.get("seconds").and_then(Value::as_f64).unwrap_or(0.0);
    Ok(match (op, video) {
        ("play", true) => VideoPlayer::play(ui, id).map(|_| Value::Null)?,
        ("play", false) => MediaPlayer::play(ui, id).map(|_| Value::Null)?,
        ("pause", true) => {
            VideoPlayer::pause(ui, id);
            Value::Null
        }
        ("pause", false) => {
            MediaPlayer::pause(ui, id);
            Value::Null
        }
        ("stop", true) => {
            VideoPlayer::stop(ui, id);
            Value::Null
        }
        ("stop", false) => {
            MediaPlayer::stop(ui, id);
            Value::Null
        }
        ("seek", true) => {
            VideoPlayer::seek(ui, id, seconds);
            Value::Null
        }
        ("seek", false) => {
            MediaPlayer::seek(ui, id, seconds);
            Value::Null
        }
        ("position", true) => json!(VideoPlayer::position(ui, id)),
        ("position", false) => json!(MediaPlayer::position(ui, id)),
        ("duration", true) => json!(VideoPlayer::duration(ui, id)),
        ("duration", false) => json!(MediaPlayer::duration(ui, id)),
        ("playing", true) => json!(VideoPlayer::is_playing(ui, id)),
        ("playing", false) => json!(MediaPlayer::is_playing(ui, id)),
        (other, _) => return Err(format!("operação de mídia desconhecida '{other}'")),
    })
}

fn string<'a>(obj: &'a Map<String, Value>, key: &str) -> Result<&'a str, String> {
    obj.get(key).and_then(Value::as_str).ok_or_else(|| format!("falta \"{key}\" (texto)"))
}

/// `"nome"` ou id numérico.
pub fn resolve(ui: &Ui, value: &Value) -> Result<ComponentId, String> {
    match value {
        Value::String(name) => ui.find_path(name).ok_or_else(|| format!("componente '{name}' não encontrado")),
        Value::Number(n) => {
            let id = ComponentId::from_u64(n.as_u64().ok_or("id inválido")?);
            if ui.contains(id) { Ok(id) } else { Err(format!("componente {n} não existe (ou foi liberado)")) }
        }
        other => Err(format!("alvo inválido: {other} (use o nome ou o id)")),
    }
}

fn target(ui: &Ui, obj: &Map<String, Value>) -> Result<ComponentId, String> {
    resolve(ui, obj.get("target").ok_or("falta \"target\"")?)
}

fn optional_target(ui: &Ui, value: Option<&Value>) -> Result<Option<ComponentId>, String> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(v) => resolve(ui, v).map(Some),
    }
}

/// Um argumento de `invoke`: o valor JSON como ele é (sem o tipo de uma propriedade).
fn json_arg(v: &Value) -> PropValue {
    match v {
        Value::Bool(b) => PropValue::Bool(*b),
        Value::Number(n) => n.as_i64().map(PropValue::Int).unwrap_or_else(|| PropValue::Float(n.as_f64().unwrap_or(0.0))),
        Value::String(s) => PropValue::String(s.clone()),
        Value::Null => PropValue::Null,
        other => PropValue::String(other.to_string()),
    }
}

fn get(ui: &Ui, id: ComponentId, name: &str) -> Result<Value, String> {
    let value = ui.get_prop(id, name).map_err(|e| e.to_string())?;
    Ok(aylis_json::prop_to_json(ui, &value))
}

// ---------------------------------------------------------------------------

/// Chave vinda do JSON para comparar com valores do dataset.
fn json_key(json: &Value) -> aylis_data::Value {
    match json {
        Value::Null => aylis_data::Value::Null,
        Value::Bool(b) => aylis_data::Value::Bool(*b),
        Value::Number(n) => n.as_i64().map(aylis_data::Value::Int).unwrap_or_else(|| aylis_data::Value::Float(n.as_f64().unwrap_or(0.0))),
        Value::String(s) => aylis_data::Value::String(s.clone()),
        other => aylis_data::Value::String(other.to_string()),
    }
}

fn data(ui: &mut Ui, obj: &Map<String, Value>) -> Result<Value, String> {
    let id = target(ui, obj)?;
    if ui.get::<aylis_data::ClientDataSet>(id).is_none() {
        return Err(format!("'{}' não é um ClientDataSet", ui.name(id).unwrap_or("?")));
    }
    let op = string(obj, "op")?;
    let mut ds = data_set(ui, id);
    let ok = |r: aylis_data::DataResult| r.map(|_| Value::Null).map_err(|e| e.message);
    match op {
        "load" => ok(ds.load_json(obj.get("rows").ok_or("falta \"rows\"")?)),
        // A carga grande no formato colunar (sem repetir os nomes dos campos em cada registro).
        "load_columns" => ok(ds.load_columns(obj.get("fields").ok_or("falta \"fields\"")?, obj.get("columns").ok_or("falta \"columns\"")?)),
        "append" => {
            let more = obj.get("has_more").and_then(Value::as_bool).unwrap_or(false);
            ok(ds.append_json(obj.get("rows").ok_or("falta \"rows\"")?, more))
        }
        // Recusas são respostas normais do backend: voltam no resultado, não como erro.
        "reconcile" => {
            let results = obj.get("results").ok_or("falta \"results\"")?;
            let results = ds.parse_results(&json!({ "results": results }).to_string()).map_err(|e| e.message)?;
            let rejected: Vec<&str> = results.iter().filter_map(|r| r.error.as_deref()).collect();
            let rejected = json!({ "rejected": rejected });
            let _ = ds.reconcile(&results);
            Ok(rejected)
        }
        "first" => ok(ds.first()),
        "prior" => ok(ds.prior()),
        "next" => ok(ds.next()),
        "last" => ok(ds.last()),
        "move_to" => ok(ds.move_to(obj.get("index").and_then(Value::as_u64).ok_or("falta \"index\"")? as usize)),
        "edit" => ok(ds.edit()),
        "insert" => ok(ds.insert()),
        "append_record" => ok(ds.append()),
        // Com `before_post` remoto (ou validação pendente), responde quando a gravação terminar.
        "post" => {
            ok(ds.post())?;
            if ds.is_asking() || ds.awaiting_text() {
                if let Some(reply) = defer_reply(ds.ui()) {
                    ds.ui().extension::<Remote>().waiting.push(Waiting { reply: Some(reply), ds: id, op: "post".into(), replay: None });
                }
            }
            Ok(Value::Null)
        }
        "cancel" => ok(ds.cancel()),
        "delete" => {
            ok(ds.delete())?;
            if ds.is_asking() {
                if let Some(reply) = defer_reply(ds.ui()) {
                    ds.ui().extension::<Remote>().waiting.push(Waiting { reply: Some(reply), ds: id, op: "delete".into(), replay: None });
                }
            }
            Ok(Value::Null)
        }
        // Como o `ApplyUpdates` do Delphi: responde quando tudo foi gravado, com o número de
        // registros que não foram (o `on_apply_updates` remoto é pergunta: espera a resposta).
        "apply" => {
            let result = ds.apply_updates();
            if ds.is_applying() {
                if let Some(reply) = defer_reply(ds.ui()) {
                    ds.ui().extension::<Remote>().waiting.push(Waiting { reply: Some(reply), ds: id, op: "apply".into(), replay: None });
                }
                return Ok(Value::Null);
            }
            match result {
                Err(e) if ds.apply_errors() == 0 => Err(e.message),
                _ => Ok(json!(ds.apply_errors())),
            }
        }
        "cancel_updates" => ok(ds.cancel_updates()),
        "refresh" => {
            ds.refresh_links();
            Ok(Value::Null)
        }
        "set_has_more" => ok(ds.set_has_more(obj.get("value").and_then(Value::as_bool).unwrap_or(false))),
        "set_value" => {
            let field = string(obj, "field")?;
            let json = obj.get("value").cloned().unwrap_or(Value::Null);
            let value = json_to_value(&json, ds.view().field(field).map_err(|e| e.message)?)?;
            ok(ds.set_value(field, value))
        }
        "set_text" => {
            let field = string(obj, "field")?;
            ok(ds.set_text(field, string(obj, "text")?))
        }
        // Vários campos de uma vez (uma ida e volta só): { "values": { "nome": "Ana", "limite": "100" } }.
        // Entra em edição se preciso; o primeiro erro interrompe e diz o campo.
        "set_values" => {
            let values = obj.get("values").and_then(Value::as_object).ok_or("falta \"values\" (objeto)")?;
            if !ds.state().is_editing() {
                ds.edit().map_err(|e| e.message)?;
            }
            for (field, json) in values {
                let value = json_to_value(json, ds.view().field(field).map_err(|e| e.message)?)?;
                ds.set_value(field, value).map_err(|e| format!("{field}: {}", e.message))?;
            }
            Ok(Value::Null)
        }
        "locate" => {
            let field = string(obj, "field")?;
            let json = obj.get("value").cloned().unwrap_or(Value::Null);
            let value = json_to_value(&json, ds.view().field(field).map_err(|e| e.message)?)?;
            ds.locate(field, value).map(Value::Bool).map_err(|e| e.message)
        }
        "set_filter" => ok(ds.set_filter(obj.get("filter").and_then(Value::as_str).unwrap_or_default())),
        "set_index" => ok(ds.set_index(obj.get("index").and_then(Value::as_str).unwrap_or_default())),
        "set_index_name" => ok(ds.set_index_name(obj.get("name").and_then(Value::as_str).unwrap_or_default())),
        // Parâmetro do filtro: { "name": "minimo", "value": 1000 }.
        "set_param" => {
            let name = string(obj, "name")?;
            ok(ds.set_param(name, json_key(obj.get("value").unwrap_or(&Value::Null))))
        }
        "revert_record" => ok(ds.revert_record()),
        // Resposta ao on_get_text: { "field": "valor", "texts": [[1, "um"], [2, "dois"]] }.
        "field_texts" => {
            let field = string(obj, "field")?;
            let pairs = obj.get("texts").and_then(Value::as_array).ok_or("falta \"texts\" (lista de [valor, texto])")?;
            let texts = pairs
                .iter()
                .filter_map(|p| {
                    let p = p.as_array()?;
                    Some((json_key(p.first()?), p.get(1)?.as_str().unwrap_or_default().to_string()))
                })
                .collect();
            ok(ds.set_field_texts(field, texts))
        }
        // Resposta ao on_validate: { "field": "cpf", "message": "CPF inválido" } (sem
        // message, ou null, limpa).
        "field_error" => {
            let field = string(obj, "field")?;
            let message = obj.get("message").and_then(Value::as_str).map(str::to_string);
            ok(ds.set_field_error(field, message))
        }
        "lookup" => {
            let key_fields = string(obj, "key_fields")?;
            let result = string(obj, "result_field")?;
            let keys: Vec<aylis_data::Value> = match obj.get("keys") {
                Some(Value::Array(list)) => list.iter().map(json_key).collect(),
                Some(k) => vec![json_key(k)],
                None => return Err("falta \"keys\"".into()),
            };
            Ok(ds.view().lookup(key_fields, &keys, result).map(|v| value_to_json(&v)).unwrap_or(Value::Null))
        }
        "value" => ds.value(string(obj, "field")?).map(|v| value_to_json(&v)).map_err(|e| e.message),
        "text" => ds.text(string(obj, "field")?).map(Value::String).map_err(|e| e.message),
        "record" => record(&ds),
        "info" => {
            let v = ds.view();
            Ok(json!({
                "state": format!("{:?}", v.state()).to_lowercase(),
                "rec_no": v.rec_no(),
                "count": v.record_count(),
                "changes": v.change_count(),
                "has_more": v.has_more(),
                "bof": v.bof(),
                "eof": v.eof(),
                // Situação do registro atual: "unmodified", "modified" ou "inserted".
                "status": v.update_status().map(|s| format!("{s:?}").to_lowercase()),
            }))
        }
        "delta" => {
            let changes = ds.delta_with_details().map_err(|e| e.message)?;
            let name = ds.ui().name(id).unwrap_or_default().to_string();
            serde_json::from_str(&aylis_data::json::delta_to_json(&name, &changes)).map_err(|e| e.to_string())
        }
        other => Err(format!("operação de dados desconhecida '{other}'")),
    }
}

/// Um campo da resposta de um evento-pergunta (`can_close`, `key`, `data`...).
fn answer_value(v: &Value) -> PropValue {
    match v {
        Value::Bool(b) => PropValue::Bool(*b),
        Value::Number(n) if n.is_i64() => PropValue::Int(n.as_i64().unwrap_or_default()),
        Value::Number(n) => PropValue::Float(n.as_f64().unwrap_or_default()),
        Value::String(s) => PropValue::String(s.clone()),
        Value::Null => PropValue::Null,
        other => PropValue::String(other.to_string()),
    }
}

fn record(ds: &DataSet<'_>) -> Result<Value, String> {
    let v = ds.view();
    let mut out = Map::new();
    for name in v.field_names() {
        out.insert(name.clone(), value_to_json(&v.value(&name).map_err(|e| e.message)?));
    }
    Ok(Value::Object(out))
}

// ---------------------------------------------------------------------------

/// Evento em JSON (veja o formato na documentação do crate).
pub fn event_to_json(ui: &Ui, sender: ComponentId, event: &str, handler: &str, args: &EventArgs) -> Value {
    let mut message = json!({
        "event": event,
        "handler": handler,
        // O form dos handlers: a segunda janela (`frmCliente2`) é do `frmCliente`; quem é
        // quem, o `form_id` e o `path` dizem.
        "form": ui.form_class(ui.form_of(sender)).or_else(|| ui.name(ui.form_of(sender))).unwrap_or_default(),
        // A instância do form (duas janelas do mesmo form têm ids diferentes).
        "form_id": ui.form_of(sender).to_u64(),
        "sender": {
            "id": sender.to_u64(),
            "name": ui.name(sender).unwrap_or_default(),
            "path": ui.path_of(sender),
            "class": ui.class_name(sender).unwrap_or_default(),
        },
        "args": args_to_json(ui, args),
    });
    // Dados extras do evento (a URL de quem mandou, no `on_message` do WebView): só nos
    // eventos que têm, sem mudar o `args`.
    let info = ui.component(sender).map(|c| c.event_info(event)).unwrap_or_default();
    if !info.is_empty() {
        message["info"] = Value::Object(info.into_iter().map(|(k, v)| (k.to_string(), Value::String(v))).collect());
    }
    message
}

fn args_to_json(ui: &Ui, args: &EventArgs) -> Value {
    match args {
        EventArgs::None => Value::Null,
        EventArgs::Pointer(p) => json!({
            "x": p.position.x,
            "y": p.position.y,
            "button": match p.button {
                PointerButton::Primary => "primary",
                PointerButton::Secondary => "secondary",
                PointerButton::Middle => "middle",
                PointerButton::Back => "back",
                PointerButton::Forward => "forward",
            },
            "clicks": p.click_count,
        }),
        EventArgs::Wheel(w) => json!({ "x": w.position.x, "y": w.position.y, "dx": w.delta.x, "dy": w.delta.y }),
        EventArgs::Gesture(g) => json!({
            "phase": match g.phase { aylis_core::GesturePhase::Begin => "begin", aylis_core::GesturePhase::Update => "update", aylis_core::GesturePhase::End => "end" },
            "x": g.center.x, "y": g.center.y, "scale": g.scale, "delta_scale": g.delta_scale,
            "rotation": g.rotation, "delta_rotation": g.delta_rotation, "pan_x": g.pan.x, "pan_y": g.pan.y,
        }),
        EventArgs::Key(k) => json!({
            "key": key_name(k.key),
            "ctrl": k.modifiers.ctrl,
            "shift": k.modifiers.shift,
            "alt": k.modifiers.alt,
        }),
        // Textos que são JSON (o delta, por exemplo) vão como objeto.
        EventArgs::Text(text) => match serde_json::from_str::<Value>(text) {
            Ok(v @ (Value::Object(_) | Value::Array(_))) => v,
            _ => Value::String(text.clone()),
        },
        EventArgs::Value(PropValue::Component(c)) => c.map_or(Value::Null, |id| Value::from(id.to_u64())),
        EventArgs::Value(v) => aylis_json::prop_to_json(ui, v),
        // O `on_intercept` de um EventInterceptor: o evento interceptado (o form, quem
        // disparou, o nome, os argumentos e o handler ligado).
        EventArgs::Intercept(i) => {
            let sender = i.sender.map_or(Value::Null, |id| {
                json!({
                    "id": id.to_u64(),
                    "name": ui.name(id).unwrap_or_default(),
                    "path": ui.path_of(id),
                    "class": ui.class_name(id).unwrap_or_default(),
                })
            });
            json!({
                "form": i.form.and_then(|f| ui.name(f)).unwrap_or_default(),
                "form_id": i.form.map_or(Value::Null, |f| Value::from(f.to_u64())),
                "sender": sender,
                "event": i.event,
                "handler": i.handler,
                "args": args_to_json(ui, &i.args),
            })
        }
    }
}

fn key_name(key: Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::F(n) => format!("f{n}"),
        other => format!("{other:?}").to_lowercase(),
    }
}

/// Faz os eventos sem handler em Rust chegarem ao backend, já em JSON. `send` pode ser
/// chamado de novo enquanto roda (um evento disparado de dentro de outro).
pub fn forward_events(ui: &mut Ui, send: impl Fn(&mut Ui, Value) + 'static) {
    let send: std::rc::Rc<dyn Fn(&mut Ui, Value)> = std::rc::Rc::new(send);
    // As respostas adiadas (o diálogo que espera, o `post` com pergunta) saem pelo mesmo caminho.
    ui.extension::<Remote>().sink = Some(send.clone());
    // Os forms liberados: o backend descarta as instâncias e os `PostTo` com eles como dono.
    ui.on_forms_freed(|ui, forms| {
        let ids: Vec<Value> = forms.iter().map(|f| Value::from(f.to_u64())).collect();
        send_event(ui, None, json!({ "event": "__freed", "forms": ids }));
    });
    ui.set_event_sink(move |ui, sender, event, handler, args| {
        let mut json = event_to_json(ui, sender, event, handler, args);
        // Um evento-pergunta: o backend responde depois com `answer` e o mesmo número, e quem
        // perguntou (fechar, gravar, a tecla) espera a resposta.
        if ui.asking() {
            json["ask"] = Value::from(ui.defer_question());
        }
        // Um igual (componente, evento) ainda no backend: este espera, e só o último sai.
        let key = (sender.to_u64(), event.to_string());
        let joined = {
            let coalesce = coalesces(ui, sender, event);
            let b = &mut ui.extension::<Remote>().busy;
            if b.tracking && json.get("ask").is_none() && b.running.values().any(|k| *k == key) && coalesce {
                b.latest.insert(key.clone(), json.clone());
                true
            } else {
                false
            }
        };
        if !joined {
            send_event(ui, Some(sender), json);
        }
    });
}

/// O comando que roda agora vai responder depois: devolve o `id` dele (`None` se não tinha
/// `id` ou se não há por onde mandar depois, como na C ABI: aí responde já).
fn defer_reply(ui: &Ui) -> Option<Value> {
    if ui.extension_ref::<Remote>().is_none_or(|r| r.sink.is_none()) {
        return None;
    }
    CURRENT_ID.with(|c| c.borrow_mut().take())
}

/// Manda a resposta adiada do comando `id`.
fn send_reply(ui: &mut Ui, id: Value, result: Result<Value, String>) {
    let message = match result {
        Ok(result) => json!({ "id": id, "result": result }),
        Err(error) => json!({ "id": id, "error": error }),
    };
    if let Some(sink) = ui.extension::<Remote>().sink.clone() {
        sink(ui, message);
    }
}

/// Os `post`/`delete`/`apply` que esperavam o `before_post`/`before_delete`: os que já
/// tiveram resposta respondem ao backend (gravou, ou o erro).
fn flush_waiting_data(ui: &mut Ui) {
    let waiting = std::mem::take(&mut ui.extension::<Remote>().waiting);
    let mut still = Vec::new();
    for w in waiting {
        let ds = data_set(ui, w.ds);
        if ds.is_asking() || ds.awaiting_text() {
            still.push(w);
            continue;
        }
        let editing = ds.state().is_editing();
        let result = match w.replay {
            // A gravação recusada: a operação que esperava não roda.
            Some(_) if editing => Err("a gravação foi recusada".to_string()),
            Some(command) => {
                // Refeito com o `id` de antes: se esperar de novo, a resposta continua devida.
                let had_id = w.reply.is_some();
                CURRENT_ID.with(|c| *c.borrow_mut() = w.reply.clone());
                let result = data_command(ui, &command);
                if had_id && CURRENT_ID.with(|c| c.borrow_mut().take()).is_none() {
                    continue;
                }
                result
            }
            None if w.op == "post" && editing => Err("a gravação foi recusada".to_string()),
            None if w.op == "apply" => match ds.apply_errors() {
                usize::MAX => Err("a gravação foi recusada".to_string()),
                n => Ok(json!(n)),
            },
            None => Ok(Value::Null),
        };
        match (w.reply, result) {
            (Some(reply), result) => send_reply(ui, reply, result),
            // Sem `id`: o erro vai como evento, como nos outros comandos.
            (None, Err(error)) => {
                if let Some(sink) = ui.extension::<Remote>().sink.clone() {
                    sink(ui, json!({ "event": "error", "cmd": "data", "message": error }));
                }
            }
            (None, Ok(_)) => {}
        }
    }
    ui.extension::<Remote>().waiting.extend(still);
}

/// Um comando de dados que espera um evento-pergunta (veja [`flush_waiting_data`]).
struct Waiting {
    /// O `id` do comando (sem `id`, ninguém espera a resposta).
    reply: Option<Value>,
    ds: ComponentId,
    op: String,
    /// O comando para refazer (a operação não rodou: esperava a gravação pendente).
    replay: Option<Map<String, Value>>,
}

/// O comando `data`: a operação que esbarra numa gravação pendente (o `before_post` remoto)
/// espera a resposta e roda depois, como se o backend esperasse o `Post` terminar.
fn data_command(ui: &mut Ui, obj: &Map<String, Value>) -> Result<Value, String> {
    match data(ui, obj) {
        Err(error) if error == aylis_data::DataError::waiting().message => {
            let ds = target(ui, obj)?;
            let reply = defer_reply(ui);
            let op = string(obj, "op")?.to_string();
            ui.extension::<Remote>().waiting.push(Waiting { reply, ds, op, replay: Some(obj.clone()) });
            Ok(Value::Null)
        }
        other => other,
    }
}

/// O que a tela sabe do backend ocupado: os eventos sem `done` e os que se juntaram.
#[derive(Default)]
struct BusyTrack {
    /// O cliente anunciou `done` no `hello` (os de antes não respondem: sem isto, a tela
    /// ficaria ocupada para sempre).
    tracking: bool,
    next: u64,
    /// O evento que se junta (componente, evento) que está no backend, pelo `seq` dele.
    running: std::collections::HashMap<u64, (u64, String)>,
    /// O último que chegou enquanto o anterior rodava (só ele sai, depois do `done`).
    latest: std::collections::HashMap<(u64, String), Value>,
}

/// Eventos que se juntam (por componente e evento) enquanto o anterior não terminou: um
/// `on_timer` não sai de novo antes do `done`; `on_resize`, um slider e semelhantes ficam só
/// com o último valor. Clique e tecla nunca se juntam.
pub fn coalesces(ui: &Ui, sender: ComponentId, event: &str) -> bool {
    aylis_core::question::COALESCED_EVENTS.contains(&event)
        || (event == "on_change" && matches!(ui.class_name(sender), Some("TrackBar" | "Slider" | "RangeSlider" | "ScrollBar" | "Knob" | "Gauge")))
}

/// O `done` do evento `seq`: livre (no núcleo) e, se um igual esperava, ele sai agora.
fn backend_done(ui: &mut Ui, seq: u64) {
    let next = {
        let b = &mut ui.extension::<Remote>().busy;
        b.running.remove(&seq).and_then(|key| b.latest.remove(&key))
    };
    ui.backend_done(seq);
    if let Some(json) = next {
        let sender = json["sender"]["id"].as_u64().map(ComponentId::from_u64);
        send_event(ui, sender, json);
    }
}

/// Manda o evento ao backend com o `seq` (e marca a tela como ocupada até o `done`).
fn send_event(ui: &mut Ui, sender: Option<ComponentId>, mut json: Value) {
    if ui.extension::<Remote>().busy.tracking {
        let seq = {
            let b = &mut ui.extension::<Remote>().busy;
            b.next += 1;
            b.next
        };
        json["seq"] = Value::from(seq);
        if let Some(sender) = sender {
            ui.backend_event_sent(seq, sender);
            let event = json["event"].as_str().unwrap_or_default().to_string();
            if coalesces(ui, sender, &event) {
                ui.extension::<Remote>().busy.running.insert(seq, (sender.to_u64(), event));
            }
        }
    }
    if let Some(sink) = ui.extension::<Remote>().sink.clone() {
        sink(ui, json);
    }
}

/// O estado do protocolo de uma tela (guardado no `Ui`: duas telas no mesmo processo, como
/// nos testes da ponte, não se misturam).
#[derive(Default)]
struct Remote {
    /// Por onde saem os eventos e as respostas adiadas.
    sink: Option<std::rc::Rc<dyn Fn(&mut Ui, Value)>>,
    busy: BusyTrack,
    /// Os comandos de dados que esperam um evento-pergunta.
    waiting: Vec<Waiting>,
}
