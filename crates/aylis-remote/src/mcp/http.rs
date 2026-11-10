//! O transporte do MCP ("streamable HTTP", só a parte de pedido e resposta): `POST /mcp` com
//! JSON-RPC, resposta em `application/json`, só em `127.0.0.1`. Todo pedido precisa do token
//! (`Authorization: Bearer ...`) e de uma origem local (o `Origin` de uma página de outro site
//! é recusado: sem isso, um site aberto no navegador falaria com o app pelo DNS rebinding).
//!
//! Serve à IDE (o MCP dela) e ao componente [`McpServer`](super::McpServer) dos apps: quem
//! liga dá a função que responde cada mensagem JSON-RPC, chamada na thread da conexão.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use serde_json::{Value, json};

/// Conexões atendidas ao mesmo tempo; acima disso, fecha na hora.
const MAX_CONNECTIONS: usize = 16;
/// Os cabeçalhos de um pedido, juntos, e quantas linhas no máximo.
const MAX_HEADERS: u64 = 64 * 1024;
const MAX_HEADER_LINES: usize = 100;
/// O corpo de um pedido, no máximo.
const MAX_BODY: usize = 16 * 1024 * 1024;

/// Responde uma mensagem JSON-RPC (`None` para notificações). Roda na thread da conexão.
pub type Handler = Arc<dyn Fn(Value) -> Option<Value> + Send + Sync>;

/// O servidor ligado; parar (ou soltar) fecha o socket.
pub struct HttpServer {
    port: u16,
    stop: Arc<AtomicBool>,
}

impl HttpServer {
    pub fn port(&self) -> u16 {
        self.port
    }

    /// O endereço que os clientes MCP usam.
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/mcp", self.port)
    }

    pub fn stop(&self) {
        if !self.stop.swap(true, Ordering::Relaxed) {
            // Acorda o `accept`.
            let _ = TcpStream::connect(("127.0.0.1", self.port));
        }
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Liga o servidor em `127.0.0.1:port` (0: uma porta livre); só atende quem traz `token`.
/// `thread` dá nome às threads (aparece no depurador).
pub fn start(port: u16, token: String, thread: &str, handler: Handler) -> Result<HttpServer, String> {
    if token.len() < 16 {
        return Err("MCP: o token precisa de pelo menos 16 caracteres".into());
    }
    let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|e| format!("MCP: porta {port}: {e}"))?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    let token = Arc::new(token);
    let open = Arc::new(AtomicUsize::new(0));
    let name = thread.to_string();
    std::thread::Builder::new()
        .name(name.clone())
        .spawn(move || {
            for stream in listener.incoming() {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
                let Ok(stream) = stream else { continue };
                if open.fetch_add(1, Ordering::SeqCst) >= MAX_CONNECTIONS {
                    open.fetch_sub(1, Ordering::SeqCst);
                    continue;
                }
                let (token, open, handler) = (token.clone(), open.clone(), handler.clone());
                let spawned = std::thread::Builder::new().name(format!("{name}-conexao")).spawn({
                    let open = open.clone();
                    move || {
                        serve(stream, &token, &*handler);
                        open.fetch_sub(1, Ordering::SeqCst);
                    }
                });
                if spawned.is_err() {
                    // Sem thread, a vaga volta aqui.
                    open.fetch_sub(1, Ordering::SeqCst);
                }
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(HttpServer { port, stop })
}

/// Uma requisição HTTP: método, caminho, cabeçalhos (minúsculos) e corpo.
#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
}

/// Lê uma requisição HTTP/1.1 (cabeçalhos e o corpo de `Content-Length`). Cabeçalhos acima
/// de 64 KB ou 100 linhas, ou corpo acima de 16 MB: recusa (sem esgotar a memória).
pub fn read_request(reader: &mut impl BufRead) -> Option<Request> {
    let mut head = std::io::Read::take(&mut *reader, MAX_HEADERS);
    let mut line = String::new();
    head.read_line(&mut line).ok()?;
    let mut parts = line.split_whitespace();
    let mut req = Request { method: parts.next()?.to_string(), path: parts.next()?.to_string(), ..Default::default() };
    loop {
        let mut h = String::new();
        if head.read_line(&mut h).ok()? == 0 {
            // O limite acabou antes da linha em branco: cabeçalhos grandes demais.
            if head.limit() == 0 {
                return None;
            }
            break;
        }
        if req.headers.len() >= MAX_HEADER_LINES {
            return None;
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            req.headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    let len: usize = req.header("content-length").and_then(|v| v.parse().ok()).unwrap_or(0);
    if len > MAX_BODY {
        return None;
    }
    req.body = vec![0; len];
    reader.read_exact(&mut req.body).ok()?;
    Some(req)
}

/// A origem é deste computador (ou não veio: clientes fora do navegador)?
pub fn local_origin(origin: Option<&str>) -> bool {
    origin.is_none_or(|o| {
        let host = o.trim_start_matches("http://").trim_start_matches("https://");
        let host = if host.starts_with('[') { host.split(']').next().map(|h| &h[1..]).unwrap_or_default() } else { host.split(['/', ':']).next().unwrap_or_default() };
        matches!(host, "127.0.0.1" | "localhost" | "::1")
    })
}

/// O cabeçalho `Authorization` traz o token? Comparação em tempo constante.
pub fn authorized(header: Option<&str>, token: &str) -> bool {
    let Some(given) = header.and_then(|h| h.strip_prefix("Bearer ")).map(str::trim) else { return false };
    let (a, b) = (given.as_bytes(), token.as_bytes());
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Um erro JSON-RPC.
pub fn rpc_error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn respond(stream: &mut TcpStream, status: &str, body: Option<&Value>) {
    let text = body.map(|b| b.to_string()).unwrap_or_default();
    let head = if body.is_some() {
        format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", text.len())
    } else {
        format!("HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
    };
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(text.as_bytes());
}

fn serve(mut stream: TcpStream, token: &str, handler: &(dyn Fn(Value) -> Option<Value> + Send + Sync)) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
    let Ok(clone) = stream.try_clone() else { return };
    let Some(req) = read_request(&mut BufReader::new(clone)) else {
        return respond(&mut stream, "431 Request Header Fields Too Large", None);
    };
    if !local_origin(req.header("origin")) {
        return respond(&mut stream, "403 Forbidden", None);
    }
    if !authorized(req.header("authorization"), token) {
        return respond(&mut stream, "401 Unauthorized", None);
    }
    if req.path.split('?').next() != Some("/mcp") {
        return respond(&mut stream, "404 Not Found", None);
    }
    // Sem fluxo de eventos do servidor (GET) nem sessões para encerrar (DELETE).
    if req.method != "POST" {
        return respond(&mut stream, "405 Method Not Allowed", None);
    }
    let Ok(message) = serde_json::from_slice::<Value>(&req.body) else {
        return respond(&mut stream, "400 Bad Request", Some(&rpc_error(Value::Null, -32700, "JSON inválido")));
    };
    let reply = match message {
        Value::Array(batch) => {
            let replies: Vec<Value> = batch.into_iter().filter_map(handler).collect();
            (!replies.is_empty()).then_some(Value::Array(replies))
        }
        single => handler(single),
    };
    match reply {
        Some(r) => respond(&mut stream, "200 OK", Some(&r)),
        // Só notificações: aceito, sem corpo.
        None => respond(&mut stream, "202 Accepted", None),
    }
}
