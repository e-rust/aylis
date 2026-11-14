//! O app como servidor MCP (Model Context Protocol): um agente de IA (Claude Code, Claude
//! Desktop e outros) usa o app pelos componentes, sem olhar a tela nem mandar cliques para a
//! janela. Dois componentes não visuais:
//!
//! - [`McpServer`]: liga o servidor (HTTP em `127.0.0.1`, com token). Com `expose_ui`, dá ao
//!   agente as ferramentas genéricas de tela (`ui_forms`, `ui_tree`, `ui_get`, `ui_set`,
//!   `ui_click`, `ui_type`, `ui_rows`...), que são os comandos do protocolo remoto; com
//!   `read_only`, só as que leem.
//! - [`McpTool`] (filho do servidor): uma ferramenta do domínio do app, com nome, descrição e
//!   parâmetros; o agente chama e o `on_execute` responde.
//!   É aqui que mora a regra de negócio: `criar_pedido(cliente, itens)` em vez de
//!   preencher campo por campo.
//!
//! Tudo roda na thread da interface: a conexão chega numa thread, a chamada vai pelo
//! [`Poster`] e a resposta volta por um canal (até `call_timeout`). O `on_call` do servidor vê
//! cada chamada antes (o registro, ou recusar: a resposta do agente é o erro).
//!
//! Para conectar o Claude Code: o método `client_command` devolve a linha
//! (`claude mcp add --transport http ...`, com o endereço e o token; com o `claude` fora do
//! PATH, pelo caminho completo: [`claude_executable`]).

pub mod http;

use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

use aylis_core::prop::PropKind;
use aylis_core::{Answer, Asked, Component, ComponentId, EventArgs, EventCtx, MethodInfo, Poster, PropValue, Ui, properties};
use serde_json::{Map, Value, json};

/// O executável do Claude Code achado nesta máquina.
#[derive(Clone, Debug, PartialEq)]
pub struct ClaudeCode {
    pub path: std::path::PathBuf,
    /// Está no PATH (um terminal comum roda `claude`); fora dele, só pelo caminho completo.
    pub in_path: bool,
}

/// Acha o `claude` (o Claude Code): no PATH e, fora dele, nos lugares em que os instaladores
/// põem (`~/.local/bin`, `~/.claude/local`, o npm global) e no que vem com o Claude Desktop
/// (`%APPDATA%\Claude\claude-code\<versão>\...\claude.exe`, a versão mais nova). No navegador,
/// nenhum.
pub fn claude_executable() -> Option<ClaudeCode> {
    use std::path::{Path, PathBuf};
    let names: &[&str] = if cfg!(windows) { &["claude.exe", "claude.cmd", "claude"] } else { &["claude"] };
    let first_in = |dir: &Path| names.iter().map(|n| dir.join(n)).find(|p| p.is_file());
    if let Some(paths) = std::env::var_os("PATH") {
        if let Some(path) = std::env::split_paths(&paths).find_map(|d| first_in(&d)) {
            return Some(ClaudeCode { path, in_path: true });
        }
    }
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from);
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(home) = &home {
        dirs.push(home.join(".local").join("bin"));
        dirs.push(home.join(".claude").join("local"));
    }
    if let Some(appdata) = std::env::var_os("APPDATA").filter(|_| cfg!(windows)) {
        dirs.push(PathBuf::from(appdata).join("npm"));
    }
    if let Some(path) = dirs.iter().find_map(|d| first_in(d)) {
        return Some(ClaudeCode { path, in_path: false });
    }
    // O que vem com o Claude Desktop: claude-code/<versão>/<marca>/claude(.exe).
    let bundled = if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| PathBuf::from(a).join("Claude").join("claude-code"))
    } else if cfg!(target_os = "macos") {
        home.as_ref().map(|h| h.join("Library/Application Support/Claude/claude-code"))
    } else {
        None
    }?;
    let version = |p: &Path| -> Vec<u64> { p.file_name().map(|n| n.to_string_lossy().split('.').map(|s| s.parse().unwrap_or(0)).collect()).unwrap_or_default() };
    let mut versions: Vec<PathBuf> = std::fs::read_dir(&bundled).ok()?.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    versions.sort_by_key(|p| std::cmp::Reverse(version(p)));
    versions.iter().find_map(|v| {
        let inner = std::fs::read_dir(v).ok()?.flatten().map(|e| e.path()).filter(|p| p.is_dir());
        std::iter::once(v.clone()).chain(inner).find_map(|d| first_in(&d))
    })
    .map(|path| ClaudeCode { path, in_path: false })
}

/// A versão do MCP que o aylis fala.
pub const PROTOCOL: &str = "2025-06-18";

/// O tempo padrão para uma chamada responder, em milissegundos.
const DEFAULT_TIMEOUT: u32 = 30_000;

/// As ferramentas de tela que só leem (as que `read_only` deixa).
const READ_TOOLS: &[&str] = &["ui_forms", "ui_tree", "ui_get", "ui_items", "ui_rows"];

/// As operações de dados que não mudam registros (as que `read_only` deixa no `ui_data`).
const READ_DATA_OPS: &[&str] = &["first", "prior", "next", "last", "move_to", "locate", "value", "text", "record", "info", "field_texts", "field_error", "lookup"];

// ---------------------------------------------------------------------------
// McpServer

/// Liga o app a agentes de IA pelo MCP. `active` liga (fora do designer); `port` 0 escolhe
/// uma porta livre (o endereço fica em `url`); `token` vazio gera um aleatório ao ligar.
/// Só aceita conexões deste computador, com o token.
///
/// `expose_ui` dá ao agente as ferramentas genéricas de tela; `read_only` deixa só as que
/// leem; `forms` (nomes separados por vírgula) limita a quais forms ele chega. O próprio
/// servidor e as ferramentas nunca são alcançáveis por elas (o agente não se dá mais poder).
/// As ferramentas do app são os [`McpTool`] filhos.
///
/// `on_call` dispara antes de cada chamada (o texto é `{"tool": .., "arguments": ..}`):
/// recusar ([`Ui::refuse`]) devolve o erro ao agente. `on_error` recebe as falhas ao ligar.
pub struct McpServer {
    /// Liga o servidor (fora do designer); desligado, ele para.
    pub active: bool,
    /// A porta em `127.0.0.1`; 0: uma porta livre (o endereço fica em `url`).
    pub port: u32,
    /// O segredo que o agente precisa mandar; vazio: um aleatório gerado ao ligar.
    pub token: String,
    /// O nome que o agente vê (vazio: o nome do componente).
    pub server_name: String,
    /// O que o agente lê ao conectar: para que serve o app e como usar as ferramentas.
    pub instructions: String,
    /// Dá ao agente as ferramentas genéricas de tela (ler a árvore, mexer em propriedades,
    /// clicar, os dados).
    pub expose_ui: bool,
    /// Das ferramentas de tela, só as que leem (nenhuma muda a tela nem os registros).
    pub read_only: bool,
    /// Os forms que o agente alcança, por nome separados por vírgula; vazio: todos.
    pub forms: String,
    /// Quanto uma chamada pode demorar, em milissegundos (vale ao ligar).
    pub call_timeout: u32,
    running: Option<http::HttpServer>,
    /// Já tentou ligar com a configuração atual (uma falha não repete a cada volta do timer).
    attempted: bool,
    last_error: String,
}

impl Default for McpServer {
    fn default() -> Self {
        Self {
            active: false,
            port: 0,
            token: String::new(),
            server_name: String::new(),
            instructions: String::new(),
            expose_ui: false,
            read_only: false,
            forms: String::new(),
            call_timeout: DEFAULT_TIMEOUT,
            running: None,
            attempted: false,
            last_error: String::new(),
        }
    }
}

impl McpServer {
    /// O endereço dos clientes (vazio: parado).
    pub fn url(&self) -> String {
        self.running.as_ref().map(http::HttpServer::url).unwrap_or_default()
    }

    pub fn running(&self) -> bool {
        self.running.is_some()
    }

    /// A falha da última tentativa de ligar (vazia: nenhuma).
    pub fn last_error(&self) -> String {
        self.last_error.clone()
    }

    /// A linha que liga o Claude Code a este app (vazia: parado). Com o `claude` fora do PATH
    /// (o que vem com o Claude Desktop, por exemplo), a linha leva o caminho completo dele,
    /// para rodar num terminal comum.
    pub fn client_command(&self, ui: &Ui, id: ComponentId) -> String {
        if self.running.is_none() {
            return String::new();
        }
        let name = self.display_name(ui, id).replace(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_', "-");
        let claude = match claude_executable() {
            Some(ClaudeCode { path, in_path: false }) => format!("\"{}\"", path.display()),
            _ => "claude".to_string(),
        };
        format!("{claude} mcp add --transport http {name} {} --header \"Authorization: Bearer {}\"", self.url(), self.token)
    }

    fn display_name(&self, ui: &Ui, id: ComponentId) -> String {
        if !self.server_name.trim().is_empty() {
            return self.server_name.trim().to_string();
        }
        ui.name(id).filter(|n| !n.is_empty()).unwrap_or("aylis-app").to_string()
    }

    fn allowed_forms(&self) -> Vec<String> {
        self.forms.split([',', ';']).map(str::trim).filter(|f| !f.is_empty()).map(str::to_string).collect()
    }

    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}

    /// Liga ou desliga conforme `active` (nunca no designer).
    fn sync(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        if !self.active {
            self.running = None;
            self.attempted = false;
            return;
        }
        if self.running.is_some() || self.attempted {
            return;
        }
        self.attempted = true;
        match self.start(ctx.ui.poster(), ctx.id) {
            Ok(server) => {
                self.last_error.clear();
                self.running = Some(server);
            }
            Err(e) => {
                self.last_error = e.clone();
                ctx.fire("on_error", EventArgs::Text(e));
            }
        }
    }

    fn start(&mut self, poster: Poster, id: ComponentId) -> Result<http::HttpServer, String> {
        let port = u16::try_from(self.port).map_err(|_| format!("MCP: porta inválida: {}", self.port))?;
        if self.token.is_empty() {
            self.token = random_token()?;
        }
        let timeout = Duration::from_millis(u64::from(if self.call_timeout == 0 { DEFAULT_TIMEOUT } else { self.call_timeout }));
        http::start(port, self.token.clone(), "aylis-mcp", Arc::new(move |message| handle_message(message, &poster, id, timeout)))
    }
}

impl Component for McpServer {
    fn class_name(&self) -> &'static str {
        "McpServer"
    }

    properties! {
        active: bool => "Behavior",
        port: u32 => "Behavior",
        token: String => "Behavior",
        server_name: String => "Behavior",
        instructions: String => "Behavior",
        expose_ui: bool => "Behavior",
        read_only: bool => "Behavior",
        forms: String => "Behavior",
        call_timeout: u32 as Duration => "Behavior";
        events: on_call, on_error;
        computed: url: String => url / set_s, running: bool => running / set_b, last_error: String => last_error / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[MethodInfo::new("client_command", &[], Some(PropKind::String))];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "client_command" => Ok(PropValue::String(self.client_command(ctx.ui, ctx.id))),
            _ => Err(format!("McpServer não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "port" | "token" | "call_timeout") && self.running.is_some() {
            // Religa com a configuração nova.
            self.running = None;
            self.attempted = false;
        }
        if name == "active" {
            self.attempted = false;
        }
        self.sync(ctx);
    }

    // Carregado de um form com `active` ligado, liga na primeira volta do timer.
    fn timer_interval(&self) -> Option<f64> {
        (self.active && self.running.is_none() && !self.attempted).then_some(0.2)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        self.sync(ctx);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn random_token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| format!("MCP: sem gerador aleatório do sistema: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(target_arch = "wasm32")]
fn random_token() -> Result<String, String> {
    Err("MCP: o navegador não abre portas (use o app de desktop)".into())
}

// ---------------------------------------------------------------------------
// McpTool

/// Uma ferramenta do app para o agente (filha de um [`McpServer`]). `tool_name` é o nome que
/// ele chama (vazio: o nome do componente), `description` diz quando usar, e `params` os
/// parâmetros: um JSON Schema (`{"type": "object", "properties": {..}}`), só as
/// propriedades (`{"cliente": {"type": "string"}}`) ou a forma curta
/// `cliente: string, quantidade: integer, obs?: string` (o `?` marca o opcional).
///
/// `on_execute` recebe os argumentos (o texto do evento é o JSON; [`McpTool::arguments`]
/// lê) e responde com [`McpTool::reply`] (ou `ui.reply("result", ..)`); recusar
/// ([`Ui::refuse`]) devolve o erro. Um backend em outra linguagem responde depois, como em
/// todo evento-pergunta. `read_only` avisa o agente que a ferramenta não muda nada.
pub struct McpTool {
    /// O nome que o agente chama; vazio: o nome do componente.
    pub tool_name: String,
    /// O que a ferramenta faz e quando usar (o agente lê para escolher).
    pub description: String,
    /// Os parâmetros: um JSON Schema, só as propriedades ou a forma curta
    /// `cliente: string, quantidade: integer, obs?: string` (`?`: opcional); vazio: nenhum.
    pub params: String,
    /// Avisa o agente que a ferramenta só lê, não muda nada.
    pub read_only: bool,
    /// Oferecida ao agente; desligada, ele não a vê.
    pub enabled: bool,
}

impl Default for McpTool {
    fn default() -> Self {
        Self { tool_name: String::new(), description: String::new(), params: String::new(), read_only: false, enabled: true }
    }
}

impl McpTool {
    /// Os argumentos que o agente mandou (no `on_execute`).
    pub fn arguments(args: &EventArgs) -> Value {
        args.text().and_then(|t| serde_json::from_str(t).ok()).unwrap_or_else(|| json!({}))
    }

    /// Responde o `on_execute`: um texto, ou JSON (vai como texto JSON).
    pub fn reply(ui: &mut Ui, result: impl Into<Value>) {
        let text = match result.into() {
            Value::String(s) => s,
            other => serde_json::to_string_pretty(&other).unwrap_or_default(),
        };
        ui.reply("result", PropValue::String(text));
    }

    /// O nome que o agente chama.
    pub fn name_in(&self, ui: &Ui, id: ComponentId) -> String {
        let name = self.tool_name.trim();
        if name.is_empty() { ui.name(id).unwrap_or_default().to_string() } else { name.to_string() }
    }

    /// O JSON Schema dos argumentos, a partir de `params`.
    pub fn schema(&self) -> Result<Value, String> {
        let text = self.params.trim();
        if text.is_empty() {
            return Ok(json!({ "type": "object", "properties": {} }));
        }
        if text.starts_with('{') {
            let v: Value = serde_json::from_str(text).map_err(|e| format!("params: JSON inválido: {e}"))?;
            if v.get("type").is_some() {
                return Ok(v);
            }
            return Ok(json!({ "type": "object", "properties": v }));
        }
        // A forma curta: `nome: tipo, outro?: tipo`.
        let mut properties = Map::new();
        let mut required = Vec::new();
        for part in text.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            let (name, kind) = part.split_once(':').map_or((part, "string"), |(n, k)| (n.trim(), k.trim()));
            let (name, optional) = match name.strip_suffix('?') {
                Some(n) => (n.trim(), true),
                None => (name, false),
            };
            if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(format!("params: nome inválido: '{part}'"));
            }
            let kind = match kind.to_ascii_lowercase().as_str() {
                "" | "string" | "text" | "texto" => "string",
                "integer" | "int" | "inteiro" => "integer",
                "number" | "float" | "numero" | "número" => "number",
                "boolean" | "bool" => "boolean",
                "object" | "objeto" => "object",
                "array" | "lista" => "array",
                other => return Err(format!("params: tipo desconhecido '{other}' em '{part}'")),
            };
            properties.insert(name.to_string(), json!({ "type": kind }));
            if !optional {
                required.push(Value::String(name.to_string()));
            }
        }
        Ok(json!({ "type": "object", "properties": properties, "required": required }))
    }
}

impl Component for McpTool {
    fn class_name(&self) -> &'static str {
        "McpTool"
    }

    properties! {
        tool_name: String => "Behavior",
        description: String => "Behavior",
        params: String => "Behavior",
        read_only: bool => "Behavior",
        enabled: bool => "Behavior";
        events: on_execute
    }

    fn default_event(&self) -> Option<&'static str> {
        Some("on_execute")
    }
}

/// Registra as classes do MCP.
pub fn register(ui: &mut Ui) {
    ui.register_class::<McpServer>("System");
    ui.register_class::<McpTool>("System");
}

// ---------------------------------------------------------------------------
// JSON-RPC

/// Responde uma mensagem JSON-RPC na thread da conexão (`None` para notificações).
fn handle_message(message: Value, poster: &Poster, server: ComponentId, timeout: Duration) -> Option<Value> {
    let id = message.get("id").cloned()?; // notificação (`notifications/initialized`...)
    let method = message.get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    let params = message.get("params").cloned().unwrap_or(Value::Null);
    let result = match method.as_str() {
        "initialize" => on_ui(poster, timeout, move |ui, reply| reply.send(initialize(ui, server, &params))),
        "ping" => Ok(json!({})),
        "tools/list" => on_ui(poster, timeout, move |ui, reply| reply.send(json!({ "tools": tools(ui, server) }))),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
            let args = params.get("arguments").cloned().filter(Value::is_object).unwrap_or_else(|| json!({}));
            on_ui(poster, timeout, move |ui, reply| call_tool(ui, server, name, args, reply)).or_else(|e| Ok(text_result(e, true)))
        }
        _ => return Some(http::rpc_error(id, -32601, &format!("método desconhecido: {method}"))),
    };
    Some(match result {
        Ok(r) => json!({ "jsonrpc": "2.0", "id": id, "result": r }),
        Err(e) => http::rpc_error(id, -32603, &e),
    })
}

/// Onde a resposta de uma chamada volta (pode ir para um `on_answer`, depois).
struct Reply(mpsc::Sender<Value>);

impl Reply {
    fn send(&self, value: Value) {
        let _ = self.0.send(value);
    }
}

/// Roda `f` na thread da interface e espera a resposta (até `timeout`).
fn on_ui(poster: &Poster, timeout: Duration, f: impl FnOnce(&mut Ui, Reply) + Send + 'static) -> Result<Value, String> {
    let (tx, rx) = mpsc::channel();
    poster.post(move |ui| f(ui, Reply(tx)));
    rx.recv_timeout(timeout).map_err(|_| "o app não respondeu a tempo".to_string())
}

fn text_result(text: impl Into<String>, is_error: bool) -> Value {
    json!({ "content": [{ "type": "text", "text": text.into() }], "isError": is_error })
}

fn json_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "ok".into(),
        other => serde_json::to_string_pretty(other).unwrap_or_default(),
    }
}

fn initialize(ui: &Ui, server: ComponentId, params: &Value) -> Value {
    let (name, mut instructions, expose_ui) = match ui.get::<McpServer>(server) {
        Some(s) => (s.display_name(ui, server), s.instructions.trim().to_string(), s.expose_ui),
        None => ("aylis-app".to_string(), String::new(), false),
    };
    if expose_ui {
        if !instructions.is_empty() {
            instructions.push_str("\n\n");
        }
        instructions.push_str("As ferramentas ui_* usam a tela do app pelos componentes (sem olhar pixels): ui_forms lista os forms, ui_tree mostra os componentes de um form (nome, classe, caminho), ui_get lê propriedades, ui_items os itens que um controle desenha (abas, linhas, nós), ui_rows os registros de um ClientDataSet. O alvo é o nome ou o caminho (frmClientes.edtNome).");
    }
    json!({
        "protocolVersion": params.get("protocolVersion").and_then(Value::as_str).unwrap_or(PROTOCOL),
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": name, "version": env!("CARGO_PKG_VERSION") },
        "instructions": instructions,
    })
}

/// As ferramentas filhas do servidor (as ligadas).
fn app_tools(ui: &Ui, server: ComponentId) -> Vec<(ComponentId, String)> {
    ui.children(server)
        .iter()
        .filter_map(|&c| ui.get::<McpTool>(c).filter(|t| t.enabled).map(|t| (c, t.name_in(ui, c))))
        .filter(|(_, n)| !n.is_empty())
        .collect()
}

fn tool(name: &str, description: &str, properties: Value, required: &[&str], read_only: bool) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": { "type": "object", "properties": properties, "required": required },
        "annotations": { "readOnlyHint": read_only },
    })
}

/// A lista que o agente vê: as do app e, com `expose_ui`, as de tela.
pub fn tools(ui: &Ui, server: ComponentId) -> Vec<Value> {
    let Some(srv) = ui.get::<McpServer>(server) else { return Vec::new() };
    let mut out: Vec<Value> = app_tools(ui, server)
        .into_iter()
        .filter_map(|(id, name)| {
            let t = ui.get::<McpTool>(id)?;
            let schema = t.schema().unwrap_or_else(|_| json!({ "type": "object" }));
            Some(json!({ "name": name, "description": t.description, "inputSchema": schema, "annotations": { "readOnlyHint": t.read_only } }))
        })
        .collect();
    if srv.expose_ui {
        out.extend(ui_tools().into_iter().filter(|t| !srv.read_only || READ_TOOLS.contains(&t["name"].as_str().unwrap_or_default())));
        if srv.read_only {
            out.push(data_tool(true));
        }
    }
    out
}

fn ui_tools() -> Vec<Value> {
    let s = |d: &str| json!({ "type": "string", "description": d });
    let target = s("o componente: nome ou caminho (frmClientes.edtNome)");
    vec![
        tool("ui_forms", "Os forms criados (e DataModules): nome, classe, título, se está aberto (`open`) e visível.", json!({}), &[], true),
        tool("ui_tree", "A árvore de componentes de um form: nome, classe, caminho, visível e filhos.", json!({ "form": s("o form (vazio: todos)") }), &[], true),
        tool("ui_get", "As propriedades de um componente (todas, com tipo e valor) ou só uma (`prop`).", json!({ "target": target, "prop": s("a propriedade, opcional") }), &["target"], true),
        tool("ui_items", "Os itens que um controle desenha sozinho (abas, linhas de lista e grid, nós de árvore, itens de menu): papel, texto, selecionado, marcado.", json!({ "target": target }), &["target"], true),
        tool("ui_rows", "Os registros de um ClientDataSet (como o grid mostra, sem rolar), a partir de `offset` (padrão 0), até `limit` (padrão 50), e o total.", json!({ "target": target, "offset": { "type": "integer" }, "limit": { "type": "integer" } }), &["target"], true),
        tool("ui_set", "Muda uma propriedade (`value` é o valor JSON, como no layout: texto, número, booleano, \"#rrggbb\").", json!({ "target": target, "prop": s("a propriedade"), "value": {} }), &["target", "prop", "value"], false),
        tool(
            "ui_click",
            "Clica num controle (botão, caixa, item de menu), como o usuário. Com `modifiers` (\"Ctrl+Shift\" ou [\"ctrl\",\"shift\"]; ctrl, shift, alt, meta), `x`/`y` (dentro do controle; padrão o meio), `button` (left, right, middle) ou `double`, é o mouse de verdade naquele ponto (Ctrl+clique na lista, o menu do botão direito).",
            json!({ "target": target, "modifiers": {}, "x": { "type": "number" }, "y": { "type": "number" }, "button": s("left | right | middle"), "double": { "type": "boolean" } }),
            &["target"],
            false,
        ),
        tool(
            "ui_drag",
            "Arrasta de um controle (`from`) a outro (`to`): aperta no meio de `from`, anda em `steps` passos (10) e solta no meio de `to` (o arrastar e soltar entre listas; `from` igual a `to` arrasta dentro dele), com `from_x`/`from_y`/`to_x`/`to_y` (dentro de cada um), `modifiers` e `button` como no ui_click.",
            json!({ "from": target, "to": target, "from_x": { "type": "number" }, "from_y": { "type": "number" }, "to_x": { "type": "number" }, "to_y": { "type": "number" }, "steps": { "type": "integer" }, "modifiers": {}, "button": s("left | right | middle") }),
            &["from", "to"],
            false,
        ),
        tool("ui_click_item", "Clica num item desenhado (de ui_items) pelo texto (`item`) ou pela posição (`index`); `action`: click, select, expand ou collapse.", json!({ "target": target, "item": s("o texto do item"), "index": { "type": "integer" }, "action": s("click | select | expand | collapse") }), &["target"], false),
        tool("ui_type", "Põe o foco no controle e digita o texto, como o teclado (passa pelas máscaras e validações).", json!({ "target": target, "text": s("o texto") }), &["target", "text"], false),
        tool("ui_key", "Aperta uma tecla no form do controle (\"Enter\", \"Tab\", \"Escape\", \"Ctrl+S\"; `modifiers` soma: {\"key\":\"A\",\"modifiers\":[\"ctrl\"]}); com `target`, o foco vai antes para ele.", json!({ "target": target, "key": s("a tecla"), "modifiers": {} }), &["target", "key"], false),
        tool("ui_invoke", "Chama um método do componente (os do ui_get de cada classe: refresh, clear...).", json!({ "target": target, "method": s("o método"), "args": { "type": "array" } }), &["target", "method"], false),
        data_tool(false),
    ]
}

fn data_tool(read_only: bool) -> Value {
    let ops = if read_only { READ_DATA_OPS.join(", ") } else { "first, next, prior, last, locate, record, info, edit, insert, set_values, post, cancel, delete, apply...".to_string() };
    tool(
        "ui_data",
        &format!("Uma operação num ClientDataSet (`op`: {ops}), com os campos dela (`field`, `value`, `values`, `index`)."),
        json!({ "target": { "type": "string" }, "op": { "type": "string" }, "field": { "type": "string" }, "value": {}, "values": { "type": "object" }, "index": { "type": "integer" } }),
        &["target", "op"],
        read_only,
    )
}

/// Uma chamada, na thread da interface: o `on_call`, depois a ferramenta.
fn call_tool(ui: &mut Ui, server: ComponentId, name: String, args: Value, reply: Reply) {
    if !ui.get::<McpServer>(server).is_some_and(|s| s.active) {
        return reply.send(text_result("o servidor MCP foi desligado", true));
    }
    if ui.has_handler(server, "on_call") {
        let info = json!({ "tool": name, "arguments": args }).to_string();
        match ui.ask(server, "on_call", &EventArgs::Text(info)) {
            Asked::Answered(Answer::Refused(m)) => return reply.send(text_result(refused(m), true)),
            Asked::Answered(Answer::Accepted(_)) => {}
            Asked::Deferred(token) => {
                return ui.on_answer(token, move |ui, answer| match answer {
                    Answer::Refused(m) => reply.send(text_result(refused(m), true)),
                    Answer::Accepted(_) => run_tool(ui, server, &name, &args, reply),
                });
            }
        }
    }
    run_tool(ui, server, &name, &args, reply);
}

fn refused(message: String) -> String {
    if message.trim().is_empty() { "chamada recusada pelo app".into() } else { message }
}

fn run_tool(ui: &mut Ui, server: ComponentId, name: &str, args: &Value, reply: Reply) {
    if let Some((tool, _)) = app_tools(ui, server).into_iter().find(|(_, n)| n == name) {
        return execute_tool(ui, tool, args, reply);
    }
    let Some(srv) = ui.get::<McpServer>(server) else { return reply.send(text_result("o servidor MCP foi liberado", true)) };
    let listed = srv.expose_ui && (!srv.read_only || READ_TOOLS.contains(&name) || name == "ui_data");
    if !listed || !name.starts_with("ui_") {
        return reply.send(text_result(format!("ferramenta desconhecida: {name}"), true));
    }
    let result = ui_tool(ui, server, name, args);
    // O que a ação disparou (o on_click, o on_change) roda antes da resposta.
    ui.process_events();
    reply.send(match result {
        Ok(v) => text_result(json_text(&v), false),
        Err(e) => text_result(e, true),
    });
}

fn execute_tool(ui: &mut Ui, tool: ComponentId, args: &Value, reply: Reply) {
    if !ui.has_handler(tool, "on_execute") {
        let name = ui.name(tool).unwrap_or_default().to_string();
        return reply.send(text_result(format!("a ferramenta '{name}' não tem o on_execute ligado"), true));
    }
    match ui.ask(tool, "on_execute", &EventArgs::Text(args.to_string())) {
        Asked::Answered(answer) => {
            ui.process_events();
            reply.send(answer_result(ui, answer));
        }
        Asked::Deferred(token) => ui.on_answer(token, move |ui, answer| reply.send(answer_result(ui, answer))),
    }
}

fn answer_result(ui: &Ui, answer: Answer) -> Value {
    match answer {
        Answer::Refused(m) => text_result(if m.trim().is_empty() { "a ferramenta recusou".into() } else { m }, true),
        Answer::Accepted(values) => {
            let text = values.iter().rev().find(|(k, _)| k == "result").map(|(_, v)| match v {
                PropValue::String(s) => s.clone(),
                other => json_text(&aylis_json::prop_to_json(ui, other)),
            });
            text_result(text.unwrap_or_else(|| "ok".into()), false)
        }
    }
}

// ---------------------------------------------------------------------------
// As ferramentas de tela

/// O componente do alvo, se o agente pode chegar nele: nunca o próprio MCP (servidor e
/// ferramentas, nem dentro deles), e só os forms de `forms`.
fn reach(ui: &Ui, server: ComponentId, value: Option<&Value>) -> Result<ComponentId, String> {
    let id = crate::resolve(ui, value.ok_or("falta \"target\"")?)?;
    let mut at = Some(id);
    while let Some(c) = at {
        let class = ui.component(c).map(|c| c.class_name()).unwrap_or_default();
        if c == server || class == "McpServer" || class == "McpTool" {
            return Err("o servidor MCP e as ferramentas dele não podem ser comandados por aqui".into());
        }
        at = ui.parent(c);
    }
    form_allowed(ui, server, ui.root_of(id))?;
    Ok(id)
}

fn form_allowed(ui: &Ui, server: ComponentId, form: ComponentId) -> Result<(), String> {
    let allowed = ui.get::<McpServer>(server).map(McpServer::allowed_forms).unwrap_or_default();
    let name = ui.name(form).unwrap_or_default();
    if allowed.is_empty() || allowed.iter().any(|f| f.eq_ignore_ascii_case(name)) {
        Ok(())
    } else {
        Err(format!("o form '{name}' não está liberado para o agente"))
    }
}

fn ui_tool(ui: &mut Ui, server: ComponentId, name: &str, args: &Value) -> Result<Value, String> {
    let read_only = ui.get::<McpServer>(server).is_some_and(|s| s.read_only);
    let str_arg = |key: &str| args.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    match name {
        "ui_forms" => Ok(Value::Array(
            ui.roots()
                .iter()
                .filter(|&&r| form_allowed(ui, server, r).is_ok() && !ui.name(r).unwrap_or_default().is_empty())
                .map(|&r| {
                    let class = ui.component(r).map(|c| c.class_name()).unwrap_or_default();
                    let caption = ui.get_prop(r, "caption").ok().and_then(|v| v.to_str().ok().map(str::to_string));
                    // `open`: a janela está aberta. Um form criado no início e não mostrado
                    // existe mas não aparece: `visible` só vale aberto
                    // (antes dizia `true` para todos, e parecia que tinham aberto).
                    let open = ui.is_open(r);
                    json!({ "name": ui.name(r), "class": class, "caption": caption, "open": open, "visible": open && ui.control(r).is_some_and(|c| c.visible()) })
                })
                .collect(),
        )),
        "ui_tree" => {
            let form = str_arg("form");
            if form.is_empty() {
                let roots: Vec<ComponentId> = ui.roots().iter().copied().filter(|&r| form_allowed(ui, server, r).is_ok()).collect();
                return Ok(Value::Array(roots.into_iter().map(|r| crate::inspect::tree(ui, Some(r))).collect()));
            }
            let id = reach(ui, server, Some(&Value::String(form)))?;
            Ok(crate::inspect::tree(ui, Some(id)))
        }
        "ui_get" => {
            let id = reach(ui, server, args.get("target"))?;
            match args.get("prop").and_then(Value::as_str).filter(|p| !p.is_empty()) {
                Some(prop) => crate::get(ui, id, prop),
                None => Ok(crate::inspect::props(ui, id)),
            }
        }
        "ui_items" => {
            let id = reach(ui, server, args.get("target"))?;
            crate::execute(ui, &json!({ "cmd": "items", "target": id.to_u64() }))
        }
        "ui_rows" => {
            let id = reach(ui, server, args.get("target"))?;
            rows(ui, id, args)
        }
        "ui_data" => {
            let id = reach(ui, server, args.get("target"))?;
            let op = str_arg("op");
            if read_only && !READ_DATA_OPS.contains(&op.as_str()) {
                return Err(format!("o servidor está só para leitura: a operação '{op}' não vale"));
            }
            let mut command = args.as_object().cloned().unwrap_or_default();
            command.insert("cmd".into(), "data".into());
            command.insert("target".into(), id.to_u64().into());
            crate::execute(ui, &Value::Object(command))
        }
        _ if read_only => Err(format!("o servidor está só para leitura: '{name}' não vale")),
        "ui_set" => {
            let id = reach(ui, server, args.get("target"))?;
            let prop = str_arg("prop");
            if prop.is_empty() {
                return Err("falta \"prop\"".into());
            }
            let value = args.get("value").cloned().unwrap_or(Value::Null);
            crate::execute(ui, &json!({ "cmd": "set", "target": id.to_u64(), "props": { prop: value } }))
        }
        "ui_click" => {
            let id = reach(ui, server, args.get("target"))?;
            // As opções do clique (o mouse de verdade) passam como estão ao protocolo, o mesmo
            // caminho do `app_command` da IDE.
            let mut command = json!({ "cmd": "click", "target": id.to_u64() });
            for key in ["modifiers", "x", "y", "button", "double"] {
                if let Some(v) = args.get(key) {
                    command[key] = v.clone();
                }
            }
            crate::execute(ui, &command)
        }
        "ui_drag" => {
            // Os dois lados passam pelo alcance do servidor (os forms permitidos, nunca ele).
            let (from, to) = (reach(ui, server, args.get("from"))?, reach(ui, server, args.get("to"))?);
            let mut command = json!({ "cmd": "drag", "from": from.to_u64(), "to": to.to_u64() });
            for key in ["from_x", "from_y", "to_x", "to_y", "steps", "modifiers", "button"] {
                if let Some(v) = args.get(key) {
                    command[key] = v.clone();
                }
            }
            crate::execute(ui, &command)
        }
        "ui_click_item" => {
            let id = reach(ui, server, args.get("target"))?;
            let mut command = json!({ "cmd": "click_item", "target": id.to_u64() });
            for key in ["item", "index", "action"] {
                if let Some(v) = args.get(key) {
                    command[key] = v.clone();
                }
            }
            crate::execute(ui, &command)
        }
        "ui_type" => {
            let id = reach(ui, server, args.get("target"))?;
            ui.set_focus(id);
            crate::execute(ui, &json!({ "cmd": "type", "target": id.to_u64(), "text": str_arg("text") }))
        }
        "ui_key" => {
            let id = reach(ui, server, args.get("target"))?;
            if ui.root_of(id) != id {
                ui.set_focus(id);
            }
            let mut command = json!({ "cmd": "key", "target": id.to_u64(), "key": str_arg("key") });
            if let Some(v) = args.get("modifiers") {
                command["modifiers"] = v.clone();
            }
            crate::execute(ui, &command)
        }
        "ui_invoke" => {
            let id = reach(ui, server, args.get("target"))?;
            crate::execute(ui, &json!({ "cmd": "invoke", "target": id.to_u64(), "method": str_arg("method"), "args": args.get("args").cloned().unwrap_or_else(|| json!([])) }))
        }
        _ => Err(format!("ferramenta desconhecida: {name}")),
    }
}

/// Os registros de um ClientDataSet, de `offset` até `limit`.
fn rows(ui: &Ui, id: ComponentId, args: &Value) -> Result<Value, String> {
    if ui.get::<aylis_data::ClientDataSet>(id).is_none() {
        return Err(format!("'{}' não é um ClientDataSet", ui.name(id).unwrap_or("?")));
    }
    let v = aylis_data::view(ui, id);
    let count = v.record_count();
    let offset = args.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(50).clamp(1, 1000) as usize;
    let names = v.field_names();
    let list: Vec<Value> = (offset..count.min(offset.saturating_add(limit)))
        .map(|i| {
            let mut row = Map::new();
            for name in &names {
                let value = v.value_at(i, name).map(|x| aylis_data::json::value_to_json(&x)).unwrap_or(Value::Null);
                row.insert(name.clone(), value);
            }
            Value::Object(row)
        })
        .collect();
    Ok(json!({ "count": count, "offset": offset, "rows": list }))
}
