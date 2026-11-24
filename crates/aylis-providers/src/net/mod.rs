//! Os clientes de rede do app: dois componentes não visuais, com propriedades, métodos e eventos
//! (valem no JSON, no protocolo e nas outras linguagens), e a biblioteca por provedor
//! (`aylis_core::providers`):
//!
//! - [`HttpClient`]: pedidos HTTP com método, cabeçalhos, corpo e JSON. O `request` (e o `get`,
//!   o `post_json`) devolve o número do pedido, e a resposta chega no `on_response` (ou o erro
//!   no `on_error`), na thread da tela. No Rust, [`HttpClient::fetch`] e [`http`] são `async`.
//! - [`WebSocketClient`]: uma conexão WebSocket com reconexão (espera crescente entre
//!   `reconnect_delay` e `reconnect_max_delay`), as mensagens no `on_message`/`on_binary`.
//!
//! Sem provedor ligado (`provider`), vale o do Aylis ([`AylisHttpProvider`],
//! [`AylisWebSocketProvider`]): o `ureq` e o `tungstenite` (com rustls, sem OpenSSL) no desktop
//! e no celular; o `fetch` e o `WebSocket` do próprio navegador na web.
//!
//! A segurança é a mesma nos dois: TLS verificado sempre (as raízes do Mozilla, embutidas);
//! `http://`/`ws://` só para este computador, a menos que o app ligue `allow_http` /
//! `allow_insecure`; prazos; limite do tamanho da resposta e da mensagem; redirecionamentos
//! contados, sem descer de `https` para `http` e sem levar o `Authorization` para outro host.

// Com uma feature só, parte do módulo fica sem uso.
#![cfg_attr(not(all(feature = "http", feature = "websocket")), allow(dead_code, unused_imports))]

#[cfg(all(feature = "http", not(target_arch = "wasm32")))]
mod native_http;
#[cfg(all(feature = "websocket", not(target_arch = "wasm32")))]
mod native_ws;
#[cfg(all(any(feature = "http", feature = "websocket"), target_arch = "wasm32"))]
mod web;

use aylis_core::prop::PropKind;
use aylis_core::providers::{HttpProvider, HttpRequest, HttpResponse, Provided, WebSocketConnection, WebSocketEvent, WebSocketProvider, WebSocketRequest};
use aylis_core::tasks::CallbackFuture;
use aylis_core::{Component, ComponentId, EventArgs, EventCtx, EventValue, MethodInfo, PropValue, RecordInfo, Ui, properties};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Os registros do acesso tipado

/// Um cabeçalho HTTP (nome e valor).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

/// O registro do [`HttpHeader`] no acesso tipado.
pub const HTTP_HEADER: RecordInfo = RecordInfo { name: "HttpHeader", fields: &[("name", PropKind::String), ("value", PropKind::String)] };

/// O fim de um pedido do [`HttpClient`] (o `on_response` e o `on_error`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpResult {
    /// O número que o `request` devolveu.
    pub request: i64,
    pub method: String,
    /// O endereço final (depois dos redirecionamentos).
    pub url: String,
    /// O status HTTP (0: não houve resposta; veja `error`).
    pub status: i64,
    /// Status 2xx.
    pub ok: bool,
    /// O corpo como texto (UTF-8).
    pub body: String,
    pub content_type: String,
    /// Os cabeçalhos da resposta (nomes em minúsculas).
    pub headers: Vec<HttpHeader>,
    /// A falha de transporte (endereço, TLS, prazo, limite); vazio quando houve resposta.
    pub error: String,
    /// Quanto o pedido levou, em milissegundos.
    pub elapsed_ms: i64,
}

/// O registro do [`HttpResult`] no acesso tipado.
pub const HTTP_RESULT: RecordInfo = RecordInfo {
    name: "HttpResult",
    fields: &[
        ("request", PropKind::Int),
        ("method", PropKind::String),
        ("url", PropKind::String),
        ("status", PropKind::Int),
        ("ok", PropKind::Bool),
        ("body", PropKind::String),
        ("content_type", PropKind::String),
        ("headers", PropKind::RecordList(&HTTP_HEADER)),
        ("error", PropKind::String),
        ("elapsed_ms", PropKind::Int),
    ],
};

impl EventValue for HttpResult {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(PropKind::Record(&HTTP_RESULT));
    fn from_args(args: &EventArgs) -> Option<Self> {
        from_event(args)
    }
}

impl HttpResult {
    /// O corpo lido como JSON (`Value::Null` se não for).
    pub fn json(&self) -> Value {
        serde_json::from_str(&self.body).unwrap_or(Value::Null)
    }
}

/// O fechamento de um [`WebSocketClient`] (o `on_close`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WebSocketClose {
    /// O código do fechamento (1000 normal, 1006 caiu sem aviso, 1009 mensagem grande demais).
    pub code: i64,
    pub reason: String,
    /// Vai tentar de novo sozinho (`auto_reconnect`).
    pub reconnecting: bool,
    /// Daqui a quanto tenta, em milissegundos (0 sem reconexão).
    pub retry_ms: i64,
}

/// O registro do [`WebSocketClose`] no acesso tipado.
pub const WEBSOCKET_CLOSE: RecordInfo = RecordInfo {
    name: "WebSocketClose",
    fields: &[("code", PropKind::Int), ("reason", PropKind::String), ("reconnecting", PropKind::Bool), ("retry_ms", PropKind::Int)],
};

impl EventValue for WebSocketClose {
    const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(PropKind::Record(&WEBSOCKET_CLOSE));
    fn from_args(args: &EventArgs) -> Option<Self> {
        from_event(args)
    }
}

fn from_event<T: serde::de::DeserializeOwned>(args: &EventArgs) -> Option<T> {
    match args {
        EventArgs::Text(t) | EventArgs::Value(PropValue::String(t)) => serde_json::from_str(t).ok(),
        _ => None,
    }
}

fn to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// Os cabeçalhos de uma propriedade `headers` (a lista de `{name, value}` em JSON).
fn parse_headers(text: &str) -> Vec<(String, String)> {
    serde_json::from_str::<Vec<HttpHeader>>(text).unwrap_or_default().into_iter().filter(|h| !h.name.trim().is_empty()).map(|h| (h.name.trim().to_string(), h.value)).collect()
}

/// Os cabeçalhos de um argumento de método: um objeto JSON (`{"Authorization": "Bearer x"}`),
/// a lista de `{name, value}` ou linhas `Nome: valor`.
fn parse_header_arg(text: &str) -> Result<Vec<(String, String)>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(Vec::new());
    }
    if text.starts_with('{') {
        let map: serde_json::Map<String, Value> = serde_json::from_str(text).map_err(|e| format!("cabeçalhos: JSON inválido: {e}"))?;
        return Ok(map.into_iter().map(|(k, v)| (k, v.as_str().map_or_else(|| v.to_string(), str::to_string))).collect());
    }
    if text.starts_with('[') {
        return Ok(parse_headers(text));
    }
    text.lines().map(str::trim).filter(|l| !l.is_empty()).map(|l| l.split_once(':').map(|(k, v)| (k.trim().to_string(), v.trim().to_string())).ok_or_else(|| format!("cabeçalho sem ':': '{l}'"))).collect()
}

/// Nome de cabeçalho válido (sem quebra de linha nem espaço: um valor vindo do usuário não
/// injeta outro cabeçalho).
fn check_headers(headers: &[(String, String)]) -> Result<(), String> {
    for (k, v) in headers {
        if k.is_empty() || !k.bytes().all(|b| b.is_ascii_graphic() && b != b':') {
            return Err(format!("nome de cabeçalho inválido: '{k}'"));
        }
        if v.contains(['\r', '\n']) {
            return Err(format!("o cabeçalho '{k}' tem quebra de linha"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Os provedores do Aylis

/// O provedor HTTP do Aylis: o `ureq` (rustls) fora do navegador, o `fetch` dentro. É o padrão
/// do [`HttpClient`] sem provedor ligado; ligue um para mudar o `user_agent`.
#[derive(Default)]
pub struct AylisHttpProvider {
    /// O `User-Agent` dos pedidos (vazio: `aylis/<versão>`); no navegador quem manda é ele.
    pub user_agent: String,
}

impl Component for AylisHttpProvider {
    fn class_name(&self) -> &'static str {
        "AylisHttpProvider"
    }

    properties! {
        user_agent: String => "Behavior",
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Http(self))
    }
}

impl HttpProvider for AylisHttpProvider {
    fn send(&self, mut request: HttpRequest, done: aylis_core::providers::HttpDone) {
        if !self.user_agent.trim().is_empty() && request.header_value("user-agent").is_none() {
            request.set_header("User-Agent", self.user_agent.trim());
        }
        default_send(request, done);
    }
}

/// Manda pelo provedor padrão (sem componente).
fn default_send(request: HttpRequest, done: aylis_core::providers::HttpDone) {
    if let Err(e) = request.check_url(&request.url).and_then(|()| check_headers(&request.headers)) {
        return done(Err(e));
    }
    #[cfg(all(feature = "http", not(target_arch = "wasm32")))]
    native_http::send(request, done);
    #[cfg(all(feature = "http", target_arch = "wasm32"))]
    web::fetch(request, done);
    #[cfg(not(feature = "http"))]
    {
        let _ = request;
        done(Err("HTTP: o app foi compilado sem a feature `http`".into()));
    }
}

/// Um pedido pelo provedor padrão, sem componente: `let r = aylis_providers::net::http(req).await?`
/// (dentro de um `ui.spawn_local`). A resposta volta para a tela pelo `Future`.
pub fn http(request: HttpRequest) -> CallbackFuture<Result<HttpResponse, String>> {
    aylis_core::tasks::from_callback(|done| default_send(request, done))
}

/// O provedor WebSocket do Aylis: o `tungstenite` (rustls) fora do navegador, o `WebSocket`
/// do navegador dentro. É o padrão do [`WebSocketClient`] sem provedor ligado.
#[derive(Default)]
pub struct AylisWebSocketProvider {}

impl Component for AylisWebSocketProvider {
    fn class_name(&self) -> &'static str {
        "AylisWebSocketProvider"
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::WebSocket(self))
    }
}

impl WebSocketProvider for AylisWebSocketProvider {
    fn connect(&self, request: WebSocketRequest, events: aylis_core::providers::WebSocketEvents) -> Box<dyn WebSocketConnection> {
        default_connect(request, events)
    }
}

/// Uma conexão que já nasceu fechada (o erro foi para os eventos).
struct Failed;

impl WebSocketConnection for Failed {
    fn send_text(&mut self, _: &str) -> Result<(), String> {
        Err("WebSocket: a conexão não abriu".into())
    }
    fn send_binary(&mut self, _: &[u8]) -> Result<(), String> {
        Err("WebSocket: a conexão não abriu".into())
    }
    fn close(&mut self, _: u16, _: &str) {}
}

/// Conecta pelo provedor `provider` (um componente provedor) ou pelo do Aylis.
fn connect_via(ui: &Ui, provider: Option<ComponentId>, request: WebSocketRequest, mut events: aylis_core::providers::WebSocketEvents) -> Box<dyn WebSocketConnection> {
    match provider.and_then(|p| ui.provided(p)) {
        Some(Provided::WebSocket(p)) => {
            if let Err(e) = request.check_url().and_then(|()| check_headers(&request.headers)) {
                events(WebSocketEvent::Error(e.clone()));
                events(WebSocketEvent::Closed { code: 1006, reason: e });
                return Box::new(Failed);
            }
            p.connect(request, events)
        }
        _ => default_connect(request, events),
    }
}

fn default_connect(request: WebSocketRequest, mut events: aylis_core::providers::WebSocketEvents) -> Box<dyn WebSocketConnection> {
    if let Err(e) = request.check_url().and_then(|()| check_headers(&request.headers)) {
        events(WebSocketEvent::Error(e.clone()));
        events(WebSocketEvent::Closed { code: 1006, reason: e });
        return Box::new(Failed);
    }
    #[cfg(all(feature = "websocket", not(target_arch = "wasm32")))]
    return native_ws::connect(request, events);
    #[cfg(all(feature = "websocket", target_arch = "wasm32"))]
    return web::websocket(request, events);
    #[cfg(not(feature = "websocket"))]
    {
        let _ = request;
        let e = "WebSocket: o app foi compilado sem a feature `websocket`".to_string();
        events(WebSocketEvent::Error(e.clone()));
        events(WebSocketEvent::Closed { code: 1006, reason: e });
        Box::new(Failed)
    }
}

// ---------------------------------------------------------------------------
// HttpClient

/// Um cliente HTTP (componente não visual). `base_url` é o começo dos endereços relativos;
/// `headers` vão em todo pedido (o `Authorization`, o `Accept`); `request_timeout`,
/// `max_response_size` e `max_redirects` são os limites; `allow_http` aceita `http://` para
/// fora deste computador (desligado: só `https://`).
///
/// O `request(method, url, body, headers)` devolve o número do pedido; a resposta chega no
/// `on_response` (qualquer status: um 404 também é resposta) e a falha de transporte no
/// `on_error`, os dois com um [`HttpResult`], na thread da tela. No Rust, [`HttpClient::fetch`]
/// devolve um `Future` (sem os eventos).
pub struct HttpClient {
    /// O começo dos endereços relativos (`https://api.exemplo.com/v1`).
    pub base_url: String,
    /// Os cabeçalhos de todo pedido (a lista de `{name, value}`).
    pub headers: String,
    /// O prazo de cada pedido, em milissegundos (0: sem prazo).
    pub request_timeout: u32,
    /// A maior resposta aceita, em bytes.
    pub max_response_size: u32,
    /// Quantos redirecionamentos seguir (0: nenhum).
    pub max_redirects: u32,
    /// Aceita `http://` sem TLS para fora deste computador.
    pub allow_http: bool,
    /// O provedor (vazio: o do Aylis).
    pub provider: Option<ComponentId>,
    next: i64,
    pending: Vec<i64>,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self { base_url: String::new(), headers: String::new(), request_timeout: 30_000, max_response_size: 16 * 1024 * 1024, max_redirects: 5, allow_http: false, provider: None, next: 0, pending: Vec::new() }
    }
}

impl HttpClient {
    /// O endereço completo: `url` absoluto fica como está; relativo vai depois do `base_url`.
    pub fn resolve(&self, url: &str) -> String {
        let url = url.trim();
        if url.contains("://") || self.base_url.trim().is_empty() {
            return url.to_string();
        }
        let base = self.base_url.trim().trim_end_matches('/');
        if url.is_empty() { base.to_string() } else { format!("{base}/{}", url.trim_start_matches('/')) }
    }

    /// O pedido com as propriedades do componente (endereço, cabeçalhos, limites). Os
    /// cabeçalhos de `request` vencem os do componente.
    pub fn prepare(&self, mut request: HttpRequest) -> HttpRequest {
        request.url = self.resolve(&request.url);
        let own = std::mem::take(&mut request.headers);
        request.headers = parse_headers(&self.headers);
        for (k, v) in own {
            request.set_header(&k, v);
        }
        request.timeout_ms = self.request_timeout;
        request.max_response_bytes = u64::from(self.max_response_size);
        request.max_redirects = self.max_redirects;
        request.allow_http = self.allow_http;
        request
    }

    /// Há pedidos esperando resposta.
    pub fn busy(&self) -> bool {
        !self.pending.is_empty()
    }

    fn set_busy(&mut self, _: bool) {}

    /// Manda pelo provedor do componente (ou o do Aylis) e devolve o `Future` da resposta, sem
    /// disparar os eventos: `let r = HttpClient::fetch(ui, id, HttpRequest::post("login").json_text(..)).await?`.
    pub fn fetch(ui: &Ui, id: ComponentId, request: HttpRequest) -> CallbackFuture<Result<HttpResponse, String>> {
        let Some(client) = ui.get::<HttpClient>(id) else {
            return CallbackFuture::ready(Err("HttpClient: componente não encontrado".into()));
        };
        let request = client.prepare(request);
        let provider = client.provider;
        aylis_core::tasks::from_callback(|done| send_with(ui, provider, request, done))
    }

    /// Manda e devolve o número do pedido; a resposta vem no `on_response`/`on_error`.
    pub fn request(ui: &mut Ui, id: ComponentId, request: HttpRequest) -> Result<i64, String> {
        let client = ui.get_mut::<HttpClient>(id).ok_or("HttpClient: componente não encontrado")?;
        client.next += 1;
        let number = client.next;
        client.pending.push(number);
        let request = client.prepare(request);
        let provider = client.provider;
        let (method, url) = (request.method.clone(), request.url.clone());
        let poster = ui.poster();
        let started = ui.time();
        send_with(
            ui,
            provider,
            request,
            Box::new(move |result| {
                poster.post_to(id, move |ui| finish(ui, id, number, method, url, started, result));
            }),
        );
        Ok(number)
    }

    /// Esquece a resposta de um pedido (ele pode ter chegado ao servidor).
    pub fn cancel(&mut self, request: i64) {
        self.pending.retain(|&p| p != request);
    }

    pub fn cancel_all(&mut self) {
        self.pending.clear();
    }

}

/// Manda pelo provedor `provider` (um componente provedor) ou pelo do Aylis.
fn send_with(ui: &Ui, provider: Option<ComponentId>, request: HttpRequest, done: aylis_core::providers::HttpDone) {
    match provider.and_then(|p| ui.provided(p)) {
        Some(Provided::Http(p)) => {
            if let Err(e) = request.check_url(&request.url).and_then(|()| check_headers(&request.headers)) {
                return done(Err(e));
            }
            p.send(request, done)
        }
        _ => default_send(request, done),
    }
}

/// A resposta chegou (na thread da tela): o evento, se o pedido não foi cancelado.
fn finish(ui: &mut Ui, id: ComponentId, number: i64, method: String, url: String, started: f64, result: Result<HttpResponse, String>) {
    let Some(client) = ui.get_mut::<HttpClient>(id) else { return };
    let Some(i) = client.pending.iter().position(|&p| p == number) else { return };
    client.pending.remove(i);
    let elapsed_ms = ((ui.time() - started) * 1000.0).max(0.0).round() as i64;
    let (event, result) = match result {
        Ok(r) => (
            "on_response",
            HttpResult {
                request: number,
                method,
                ok: r.ok(),
                content_type: r.header("content-type").unwrap_or_default().to_string(),
                body: r.text(),
                headers: r.headers.iter().map(|(k, v)| HttpHeader { name: k.clone(), value: v.clone() }).collect(),
                status: i64::from(r.status),
                url: if r.url.is_empty() { url } else { r.url },
                error: String::new(),
                elapsed_ms,
            },
        ),
        Err(error) => ("on_error", HttpResult { request: number, method, url, error, elapsed_ms, ..Default::default() }),
    };
    ui.fire(id, event, &EventArgs::Text(to_json(&result)));
}

impl Component for HttpClient {
    fn class_name(&self) -> &'static str {
        "HttpClient"
    }

    properties! {
        base_url: String => "Behavior",
        headers: String as RecordList(&HTTP_HEADER) => "Behavior",
        request_timeout: u32 as Duration => "Behavior",
        max_response_size: u32 => "Behavior",
        max_redirects: u32 => "Behavior",
        allow_http: bool => "Behavior",
        provider: Option<ComponentId> => "Behavior";
        events: on_response: HttpResult, on_error: HttpResult;
        computed: busy: bool => busy / set_busy
    }

    fn default_event(&self) -> Option<&'static str> {
        Some("on_response")
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const S: PropKind = PropKind::String;
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("request", &[S, S, S, S], Some(PropKind::Int)).params(&["method", "url", "body", "headers"]),
            MethodInfo::new("get", &[S], Some(PropKind::Int)).params(&["url"]),
            MethodInfo::new("post_json", &[S, S], Some(PropKind::Int)).params(&["url", "json"]),
            MethodInfo::new("cancel", &[PropKind::Int], None).params(&["request"]),
            MethodInfo::new("cancel_all", &[], None),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = |i: usize| args.get(i).and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        let request = match method {
            "request" => {
                let m = text(0);
                if m.trim().is_empty() || !m.trim().bytes().all(|b| b.is_ascii_alphabetic()) {
                    return Err(format!("método HTTP inválido: '{m}'"));
                }
                let mut r = HttpRequest::new(&m, text(1)).body(text(2).into_bytes());
                for (k, v) in parse_header_arg(&text(3))? {
                    r.set_header(&k, v);
                }
                r
            }
            "get" => HttpRequest::get(text(0)),
            "post_json" => {
                let json = text(1);
                serde_json::from_str::<Value>(&json).map_err(|e| format!("post_json: JSON inválido: {e}"))?;
                HttpRequest::post(text(0)).json_text(json)
            }
            "cancel" => {
                self.cancel(args.first().and_then(|v| v.to_i64().ok()).unwrap_or_default());
                return Ok(PropValue::Null);
            }
            "cancel_all" => {
                self.cancel_all();
                return Ok(PropValue::Null);
            }
            _ => return Err(format!("HttpClient não tem o método '{method}'")),
        };
        Ok(PropValue::Int(self.start(ctx, request)))
    }
}

impl HttpClient {
    /// Manda de dentro do próprio componente (o `invoke`, que roda com ele fora do `Ui`).
    fn start(&mut self, ctx: &mut EventCtx<'_>, request: HttpRequest) -> i64 {
        self.next += 1;
        let number = self.next;
        self.pending.push(number);
        let request = self.prepare(request);
        let (method, url) = (request.method.clone(), request.url.clone());
        let (id, poster, started) = (ctx.id, ctx.ui.poster(), ctx.ui.time());
        send_with(
            ctx.ui,
            self.provider,
            request,
            Box::new(move |result| {
                poster.post_to(id, move |ui| finish(ui, id, number, method, url, started, result));
            }),
        );
        number
    }
}

// ---------------------------------------------------------------------------
// WebSocketClient

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum WsState {
    /// Parado (ou desligado pelo app).
    #[default]
    Idle,
    Connecting,
    Open,
    /// Fechando a pedido do app (não reconecta).
    Closing,
    /// Esperando para reconectar.
    Waiting,
}

/// Uma conexão WebSocket (componente não visual) com reconexão. `active` liga (fora do
/// designer) e desligado fecha; `url` é o `wss://` (o `ws://` sem TLS só para este computador,
/// ou com `allow_insecure`); `protocols` os subprotocolos; `headers` os cabeçalhos da abertura
/// (no navegador não valem).
///
/// Caiu sem o app pedir: com `auto_reconnect`, tenta de novo depois de `reconnect_delay`,
/// dobrando a espera a cada falha até `reconnect_max_delay` (volta ao começo quando abre).
///
/// Eventos, na thread da tela: `on_open`, `on_message` (o texto), `on_binary` (os bytes em
/// base64), `on_close` (um [`WebSocketClose`]) e `on_error`. Mandar: `send(texto)` e
/// `send_binary(base64)` (no Rust, [`WebSocketClient::send_text`] e [`WebSocketClient::send_bytes`]).
pub struct WebSocketClient {
    /// O endereço (`wss://`; `ws://` só para este computador, salvo `allow_insecure`).
    pub url: String,
    /// Os subprotocolos oferecidos.
    pub protocols: String,
    /// Os cabeçalhos da abertura (a lista de `{name, value}`).
    pub headers: String,
    /// Conecta (fora do designer); desligado, fecha.
    pub active: bool,
    /// Reconecta sozinho quando a conexão cai, esperando de `reconnect_delay` até
    /// `reconnect_max_delay` (a espera dobra a cada tentativa).
    pub auto_reconnect: bool,
    /// A primeira espera para reconectar, em milissegundos.
    pub reconnect_delay: u32,
    /// A maior espera para reconectar, em milissegundos.
    pub reconnect_max_delay: u32,
    /// O prazo para abrir, em milissegundos.
    pub connect_timeout: u32,
    /// A maior mensagem aceita, em bytes (acima disso a conexão fecha com 1009).
    pub max_message_size: u32,
    /// Aceita `ws://` sem TLS para fora deste computador.
    pub allow_insecure: bool,
    /// O provedor (vazio: o do Aylis).
    pub provider: Option<ComponentId>,
    conn: Option<Box<dyn WebSocketConnection>>,
    state: WsState,
    /// A conexão atual (eventos de uma conexão antiga são descartados).
    generation: u64,
    /// Falhas seguidas (a espera dobra a cada uma).
    failures: u32,
    retry: Option<u64>,
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self {
            url: String::new(),
            protocols: String::new(),
            headers: String::new(),
            active: false,
            auto_reconnect: true,
            reconnect_delay: 1_000,
            reconnect_max_delay: 30_000,
            connect_timeout: 10_000,
            max_message_size: 16 * 1024 * 1024,
            allow_insecure: false,
            provider: None,
            conn: None,
            state: WsState::Idle,
            generation: 0,
            failures: 0,
            retry: None,
        }
    }
}

impl Drop for WebSocketClient {
    fn drop(&mut self) {
        if let Some(mut c) = self.conn.take() {
            c.close(1001, "");
        }
    }
}

impl WebSocketClient {
    /// A conexão está aberta.
    pub fn connected(&self) -> bool {
        self.state == WsState::Open
    }

    fn set_connected(&mut self, _: bool) {}

    /// O pedido de conexão com as propriedades.
    pub fn connect_request(&self) -> WebSocketRequest {
        WebSocketRequest {
            url: self.url.trim().to_string(),
            protocols: self.protocols.split(';').map(str::trim).filter(|p| !p.is_empty()).map(str::to_string).collect(),
            headers: parse_headers(&self.headers),
            connect_timeout_ms: self.connect_timeout,
            max_message_bytes: u64::from(self.max_message_size),
            allow_insecure: self.allow_insecure,
        }
    }

    /// Manda um texto (a conexão precisa estar aberta).
    pub fn send_text(&mut self, text: &str) -> Result<(), String> {
        match (&mut self.conn, self.state) {
            (Some(c), WsState::Open) => c.send_text(text),
            _ => Err("WebSocket: a conexão não está aberta".into()),
        }
    }

    /// Manda bytes (a conexão precisa estar aberta).
    pub fn send_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        match (&mut self.conn, self.state) {
            (Some(c), WsState::Open) => c.send_binary(data),
            _ => Err("WebSocket: a conexão não está aberta".into()),
        }
    }

    /// Abre (fecha a anterior, se houver), com o componente fora do `Ui` (dentro dos métodos
    /// dele).
    fn open(&mut self, ui: &mut Ui, id: ComponentId) {
        let (request, provider, events) = self.prepare(ui, id);
        self.conn = Some(connect_via(ui, provider, request, events));
    }

    /// Abre o componente `id`, que está no `Ui` (a reconexão, de um `set_timeout`).
    fn reopen(ui: &mut Ui, id: ComponentId) {
        let Some(client) = ui.get_mut::<WebSocketClient>(id) else { return };
        client.retry = None;
        if client.state != WsState::Waiting || !client.active {
            return;
        }
        let mut c = std::mem::take(client);
        let (request, provider, events) = c.prepare(ui, id);
        let _ = std::mem::replace(ui.get_mut::<WebSocketClient>(id).expect("o componente continua no Ui"), c);
        let conn = connect_via(ui, provider, request, events);
        if let Some(client) = ui.get_mut::<WebSocketClient>(id) {
            client.conn = Some(conn);
        }
    }

    /// Fecha a conexão anterior e monta a nova (o pedido, o provedor e os eventos).
    fn prepare(&mut self, ui: &mut Ui, id: ComponentId) -> (WebSocketRequest, Option<ComponentId>, aylis_core::providers::WebSocketEvents) {
        if let Some(t) = self.retry.take() {
            ui.clear_timeout(t);
        }
        if let Some(mut c) = self.conn.take() {
            c.close(1000, "");
        }
        self.generation += 1;
        self.state = WsState::Connecting;
        let (generation, poster) = (self.generation, ui.poster());
        let events: aylis_core::providers::WebSocketEvents = Box::new(move |event| {
            poster.post_to(id, move |ui| ws_event(ui, id, generation, event));
        });
        (self.connect_request(), self.provider, events)
    }

    /// Fecha a pedido do app (sem reconectar).
    fn shut(&mut self, ui: &mut Ui) {
        if let Some(t) = self.retry.take() {
            ui.clear_timeout(t);
        }
        self.failures = 0;
        match self.conn.as_mut() {
            Some(c) if matches!(self.state, WsState::Open | WsState::Connecting) => {
                self.state = WsState::Closing;
                c.close(1000, "");
            }
            _ => {
                self.conn = None;
                self.state = WsState::Idle;
            }
        }
    }

    /// Liga ou desliga conforme `active` (nunca no designer).
    fn sync(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        if !self.active {
            self.shut(ctx.ui);
        } else if self.state == WsState::Idle {
            self.open(ctx.ui, ctx.id);
        }
    }

    /// A espera antes da próxima tentativa (dobra a cada falha seguida).
    fn backoff_ms(&self) -> u64 {
        let first = u64::from(self.reconnect_delay.max(100));
        let max = u64::from(self.reconnect_max_delay).max(first);
        first.saturating_mul(1u64 << self.failures.saturating_sub(1).min(16)).min(max)
    }
}

/// Um evento da conexão `generation`, na thread da tela.
fn ws_event(ui: &mut Ui, id: ComponentId, generation: u64, event: WebSocketEvent) {
    let design = ui.design_mode();
    let Some(client) = ui.get_mut::<WebSocketClient>(id) else { return };
    if client.generation != generation {
        return;
    }
    match event {
        WebSocketEvent::Open { .. } => {
            client.state = WsState::Open;
            client.failures = 0;
            ui.fire(id, "on_open", &EventArgs::None);
        }
        WebSocketEvent::Text(text) => {
            ui.fire(id, "on_message", &EventArgs::Text(text));
        }
        WebSocketEvent::Binary(data) => {
            ui.fire(id, "on_binary", &EventArgs::Text(aylis_core::image::encode_base64(&data)));
        }
        WebSocketEvent::Error(e) => {
            ui.fire(id, "on_error", &EventArgs::Text(e));
        }
        WebSocketEvent::Closed { code, reason } => {
            client.conn = None;
            let by_app = client.state == WsState::Closing || !client.active;
            let was_open = client.state == WsState::Open;
            let reconnect = !by_app && client.auto_reconnect && !design;
            let mut retry_ms = 0;
            if reconnect {
                if !was_open {
                    client.failures = client.failures.saturating_add(1);
                }
                retry_ms = client.backoff_ms();
                client.state = WsState::Waiting;
                let timer = ui.set_timeout(retry_ms, move |ui| WebSocketClient::reopen(ui, id));
                if let Some(c) = ui.get_mut::<WebSocketClient>(id) {
                    c.retry = Some(timer);
                }
            } else if let Some(c) = ui.get_mut::<WebSocketClient>(id) {
                c.state = WsState::Idle;
                c.failures = 0;
            }
            let close = WebSocketClose { code: i64::from(code), reason, reconnecting: reconnect, retry_ms: retry_ms as i64 };
            ui.fire(id, "on_close", &EventArgs::Text(to_json(&close)));
            // Ligado de novo enquanto fechava: o timer do `active` volta a contar.
            ui.sync_timer(id);
        }
    }
}

impl Component for WebSocketClient {
    fn timed(&self) -> Option<&dyn aylis_core::Timed> {
        Some(self)
    }

    fn timed_mut(&mut self) -> Option<&mut dyn aylis_core::Timed> {
        Some(self)
    }

    fn class_name(&self) -> &'static str {
        "WebSocketClient"
    }

    properties! {
        url: String => "Behavior",
        protocols: String as List => "Behavior",
        headers: String as RecordList(&HTTP_HEADER) => "Behavior",
        active: bool => "Behavior",
        auto_reconnect: bool => "Behavior",
        reconnect_delay: u32 as Duration => "Behavior",
        reconnect_max_delay: u32 as Duration => "Behavior",
        connect_timeout: u32 as Duration => "Behavior",
        max_message_size: u32 => "Behavior",
        allow_insecure: bool => "Behavior",
        provider: Option<ComponentId> => "Behavior";
        events: on_open: (), on_message: String, on_binary: String, on_close: WebSocketClose, on_error: String;
        computed: connected: bool => connected / set_connected
    }

    fn default_event(&self) -> Option<&'static str> {
        Some("on_message")
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("connect", &[], None),
            MethodInfo::new("disconnect", &[], None),
            MethodInfo::new("send", &[PropKind::String], Some(PropKind::Bool)).params(&["text"]),
            MethodInfo::new("send_binary", &[PropKind::String], Some(PropKind::Bool)).params(&["base64"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let text = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        match method {
            "connect" => {
                self.active = true;
                if !ctx.ui.design_mode() {
                    self.failures = 0;
                    self.open(ctx.ui, ctx.id);
                }
                Ok(PropValue::Null)
            }
            "disconnect" => {
                self.active = false;
                self.shut(ctx.ui);
                Ok(PropValue::Null)
            }
            "send" => self.send_text(&text).map(|()| PropValue::Bool(true)),
            "send_binary" => {
                let bytes = aylis_core::image::decode_base64(&text).ok_or("send_binary: base64 inválido")?;
                self.send_bytes(&bytes).map(|()| PropValue::Bool(true))
            }
            _ => Err(format!("WebSocketClient não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "url" | "protocols" | "headers" | "provider" | "allow_insecure" | "max_message_size") && self.active && self.state != WsState::Idle && !ctx.ui.design_mode() {
            // Religa com a configuração nova.
            self.failures = 0;
            self.open(ctx.ui, ctx.id);
            return;
        }
        if name == "active" {
            self.failures = 0;
        }
        self.sync(ctx);
    }
}

impl aylis_core::Timed for WebSocketClient {
    // Carregado de um form com `active` ligado, conecta na primeira volta do timer (o `load`
    // não passa pelo `property_changed`); depois o timer para.
    fn timer_interval(&self) -> Option<f64> {
        (self.active && self.state == WsState::Idle && self.conn.is_none()).then_some(0.05)
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            // No designer não conecta; o timer não volta a pedir.
            self.state = WsState::Waiting;
            return;
        }
        self.sync(ctx);
    }
}

/// Registra as classes de rede (cada uma com a sua feature).
pub fn register(ui: &mut Ui) {
    #[cfg(feature = "http")]
    {
        ui.register_class::<HttpClient>("System");
        ui.register_class::<AylisHttpProvider>("Providers");
    }
    #[cfg(feature = "websocket")]
    {
        ui.register_class::<WebSocketClient>("System");
        ui.register_class::<AylisWebSocketProvider>("Providers");
    }
    let _ = ui;
}
