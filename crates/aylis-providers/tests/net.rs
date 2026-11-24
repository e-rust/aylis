//! O `HttpClient` e o `WebSocketClient` contra servidores de teste em 127.0.0.1: o pedido com
//! método, cabeçalhos e corpo; os redirecionamentos (sem levar o `Authorization` para outro
//! host); os limites (tamanho, prazo, esquema); a conexão WebSocket, as mensagens e a reconexão.
//! O teste faz o papel do laço da janela (`process_posted` e `advance`).

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use aylis_core::providers::HttpRequest;
use aylis_core::{EventValue, PropValue, Ui};
use aylis_providers::net::{HttpClient, HttpResult, WebSocketClient, WebSocketClose};
use serde_json::{Value, json};

// ---------------------------------------------------------------------------
// O servidor HTTP de teste

struct Req {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn read_req(stream: &mut TcpStream) -> Option<Req> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    let mut parts = line.split_whitespace();
    let (method, path) = (parts.next()?.to_string(), parts.next()?.to_string());
    let mut headers = Vec::new();
    loop {
        let mut h = String::new();
        reader.read_line(&mut h).ok()?;
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    let len: usize = headers.iter().find(|(k, _)| k == "content-length").and_then(|(_, v)| v.parse().ok()).unwrap_or(0);
    let mut body = vec![0; len];
    reader.read_exact(&mut body).ok()?;
    Some(Req { method, path, headers, body })
}

fn reply(stream: &mut TcpStream, status: &str, extra: &str, body: &[u8]) {
    let _ = write!(stream, "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n", body.len());
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

/// Um servidor HTTP em 127.0.0.1: `/echo` devolve o pedido em JSON, `/status/N` o status N,
/// `/redirect` manda (302) para `/echo`, `/away` manda para `/echo` em `localhost` (outro host),
/// `/big` 5000 bytes, `/slow` demora 3 s.
fn http_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            std::thread::spawn(move || {
                let Some(req) = read_req(&mut stream) else { return };
                let path = req.path.clone();
                match path.as_str() {
                    "/echo" => {
                        let headers: serde_json::Map<String, Value> = req.headers.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect();
                        let body = json!({ "method": req.method, "headers": headers, "body": String::from_utf8_lossy(&req.body) }).to_string();
                        reply(&mut stream, "200 OK", "Content-Type: application/json\r\n", body.as_bytes());
                    }
                    "/redirect" => reply(&mut stream, "302 Found", "Location: /echo\r\n", b""),
                    "/away" => reply(&mut stream, "307 Temporary Redirect", &format!("Location: http://localhost:{port}/echo\r\n"), b""),
                    "/loop" => reply(&mut stream, "302 Found", "Location: /loop\r\n", b""),
                    "/big" => reply(&mut stream, "200 OK", "", &[b'x'; 5000]),
                    "/slow" => {
                        std::thread::sleep(Duration::from_secs(3));
                        reply(&mut stream, "200 OK", "", b"tarde");
                    }
                    p if p.starts_with("/status/") => reply(&mut stream, &format!("{} X", &p[8..]), "", b"status"),
                    _ => reply(&mut stream, "404 Not Found", "", b""),
                }
            });
        }
    });
    port
}

// ---------------------------------------------------------------------------
// O app de teste

fn app(component: Value) -> (Ui, aylis_core::ComponentId) {
    let mut ui = Ui::new();
    aylis_providers::register(&mut ui);
    aylis_controls::register(&mut ui);
    let name = component["name"].as_str().unwrap().to_string();
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmRede", "children": [component] } });
    aylis_json::load_value(&mut ui, &layout, None).unwrap();
    let id = ui.find(&name).unwrap();
    (ui, id)
}

/// Os eventos (nome e texto) que o componente disparou.
fn record(ui: &mut Ui, id: aylis_core::ComponentId, events: &[&'static str]) -> Rc<RefCell<Vec<(String, String)>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    for &event in events {
        let log = log.clone();
        ui.on(id, event, move |_, _, args| log.borrow_mut().push((event.to_string(), args.text().unwrap_or_default().to_string())));
    }
    log
}

/// Roda o laço até `done` (ou o prazo).
fn pump_until(ui: &mut Ui, secs: f64, mut done: impl FnMut(&mut Ui) -> bool) {
    let start = Instant::now();
    while !done(ui) {
        assert!(start.elapsed().as_secs_f64() < secs, "o evento não chegou em {secs} s");
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(5));
        ui.advance(0.005);
    }
}

fn http_result(text: &str) -> HttpResult {
    HttpResult::from_args(&aylis_core::EventArgs::Text(text.into())).unwrap()
}

fn request(ui: &mut Ui, id: aylis_core::ComponentId, args: Vec<PropValue>, log: &Rc<RefCell<Vec<(String, String)>>>) -> (String, HttpResult) {
    let before = log.borrow().len();
    let number = ui.invoke(id, "request", &args).unwrap();
    pump_until(ui, 10.0, |_| log.borrow().len() > before);
    let (event, text) = log.borrow()[before].clone();
    let r = http_result(&text);
    assert_eq!(PropValue::Int(r.request), number);
    (event, r)
}

fn s(v: &str) -> PropValue {
    PropValue::String(v.into())
}

#[test]
fn the_http_client_sends_method_headers_and_body() {
    let port = http_server();
    let (mut ui, id) = app(json!({
        "class": "HttpClient", "name": "httpApi", "base_url": format!("http://127.0.0.1:{port}"),
        "headers": [{ "name": "Accept", "value": "application/json" }]
    }));
    let log = record(&mut ui, id, &["on_response", "on_error"]);

    let (event, r) = request(&mut ui, id, vec![s("POST"), s("/echo"), s(r#"{"usuario":"ana"}"#), s(r#"{"Authorization": "Bearer abc", "Content-Type": "application/json"}"#)], &log);
    assert_eq!(event, "on_response");
    assert_eq!((r.status, r.ok, r.content_type.as_str()), (200, true, "application/json"));
    let echo = r.json();
    assert_eq!(echo["method"], "POST");
    assert_eq!(echo["headers"]["authorization"], "Bearer abc");
    assert_eq!(echo["headers"]["accept"], "application/json", "os cabeçalhos do componente vão junto");
    assert_eq!(echo["body"], r#"{"usuario":"ana"}"#);
    assert!(!ui.get::<HttpClient>(id).unwrap().busy());

    // Um 404 é resposta, não falha.
    let (event, r) = request(&mut ui, id, vec![s("GET"), s("status/404"), s(""), s("")], &log);
    assert_eq!((event.as_str(), r.status, r.ok), ("on_response", 404, false));

    // O `post_json` confere o JSON antes de mandar.
    assert!(ui.invoke(id, "post_json", &[s("/echo"), s("{não é json")]).is_err());
    // Um valor de cabeçalho não injeta outro cabeçalho.
    let (event, r) = request(&mut ui, id, vec![s("GET"), s("/echo"), s(""), s("{\"X-A\": \"1\\r\\nX-B: 2\"}")], &log);
    assert_eq!(event, "on_error");
    assert!(r.error.contains("quebra de linha"), "{}", r.error);
}

#[test]
fn redirects_are_followed_without_carrying_credentials_to_another_host() {
    let port = http_server();
    let (mut ui, id) = app(json!({ "class": "HttpClient", "name": "httpApi", "base_url": format!("http://127.0.0.1:{port}") }));
    let log = record(&mut ui, id, &["on_response", "on_error"]);

    // 302 de um POST: segue com GET, sem corpo (como o navegador).
    let (_, r) = request(&mut ui, id, vec![s("POST"), s("/redirect"), s("corpo"), s("Authorization: Bearer abc")], &log);
    assert_eq!(r.status, 200);
    assert!(r.url.ends_with("/echo"), "{}", r.url);
    assert_eq!(r.json()["method"], "GET");
    assert_eq!(r.json()["headers"]["authorization"], "Bearer abc", "mesmo host: o Authorization vai");

    // 307 para outro host (localhost): o método fica, o Authorization não vai.
    let (_, r) = request(&mut ui, id, vec![s("PUT"), s("/away"), s("x"), s("Authorization: Bearer abc")], &log);
    assert_eq!((r.status, r.json()["method"].as_str()), (200, Some("PUT")));
    assert!(r.json()["headers"].get("authorization").is_none(), "o Authorization não vai para outro host: {}", r.body);

    // Sem fim: para no limite e devolve o 302.
    let (_, r) = request(&mut ui, id, vec![s("GET"), s("/loop"), s(""), s("")], &log);
    assert_eq!(r.status, 302);

    // `max_redirects: 0`: a resposta 3xx volta como está.
    ui.set_prop(id, "max_redirects", 0u32).unwrap();
    let (_, r) = request(&mut ui, id, vec![s("GET"), s("/redirect"), s(""), s("")], &log);
    assert_eq!(r.status, 302);
}

#[test]
fn the_limits_hold() {
    let port = http_server();
    let (mut ui, id) = app(json!({ "class": "HttpClient", "name": "httpApi", "base_url": format!("http://127.0.0.1:{port}"), "max_response_size": 1000, "request_timeout": 400 }));
    let log = record(&mut ui, id, &["on_response", "on_error"]);

    let (event, r) = request(&mut ui, id, vec![s("GET"), s("/big"), s(""), s("")], &log);
    assert_eq!(event, "on_error");
    assert!(r.error.contains("limite"), "{}", r.error);

    let started = Instant::now();
    let (event, r) = request(&mut ui, id, vec![s("GET"), s("/slow"), s(""), s("")], &log);
    assert_eq!(event, "on_error");
    assert!(r.error.contains("prazo"), "{}", r.error);
    assert!(started.elapsed() < Duration::from_secs(2), "o prazo cortou antes da resposta");

    // http:// sem TLS só para este computador.
    let (event, r) = request(&mut ui, id, vec![s("GET"), s("http://example.com/"), s(""), s("")], &log);
    assert_eq!(event, "on_error");
    assert!(r.error.contains("sem TLS"), "{}", r.error);
    let (event, r) = request(&mut ui, id, vec![s("GET"), s("ftp://example.com/"), s(""), s("")], &log);
    assert!(event == "on_error" && r.error.contains("esquema"), "{}", r.error);
}

#[test]
fn a_cancelled_request_fires_nothing_and_fetch_is_async() {
    let port = http_server();
    let (mut ui, id) = app(json!({ "class": "HttpClient", "name": "httpApi", "base_url": format!("http://127.0.0.1:{port}") }));
    let log = record(&mut ui, id, &["on_response", "on_error"]);
    let n = ui.invoke(id, "get", &[s("/echo")]).unwrap();
    assert!(ui.get::<HttpClient>(id).unwrap().busy());
    ui.invoke(id, "cancel", &[n]).unwrap();
    assert!(!ui.get::<HttpClient>(id).unwrap().busy());

    // O `fetch` (Rust, async): a resposta no Future, sem os eventos.
    let got: Rc<RefCell<Option<Result<aylis_core::providers::HttpResponse, String>>>> = Rc::default();
    let future = HttpClient::fetch(&ui, id, HttpRequest::post("/echo").bearer("xyz").json_text(r#"{"a":1}"#));
    let form = ui.find("frmRede").unwrap();
    let out = got.clone();
    ui.spawn_local(form, future, move |_, r| *out.borrow_mut() = Some(r));
    pump_until(&mut ui, 10.0, |_| got.borrow().is_some());
    let r = got.borrow_mut().take().unwrap().unwrap();
    let echo: Value = serde_json::from_slice(&r.body).unwrap();
    assert_eq!((echo["method"].as_str(), echo["headers"]["authorization"].as_str(), echo["body"].as_str()), (Some("POST"), Some("Bearer xyz"), Some(r#"{"a":1}"#)));
    // O cancelado não disparou nada.
    std::thread::sleep(Duration::from_millis(100));
    ui.process_posted();
    assert!(log.borrow().is_empty(), "{:?}", log.borrow());
}

// ---------------------------------------------------------------------------
// WebSocket

/// Um servidor WebSocket em 127.0.0.1 que devolve cada mensagem (texto e binário); `"cai"` faz
/// ele derrubar a conexão (sem fechamento), `"tchau"` fecha com 4000. Conta as conexões.
fn ws_server() -> (u16, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let count = Arc::new(AtomicUsize::new(0));
    let c = count.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            c.fetch_add(1, Ordering::SeqCst);
            std::thread::spawn(move || {
                let callback = |req: &tungstenite::handshake::server::Request, mut resp: tungstenite::handshake::server::Response| {
                    if req.headers().get("sec-websocket-protocol").is_some_and(|p| p.to_str().unwrap_or_default().contains("chat")) {
                        resp.headers_mut().insert("Sec-WebSocket-Protocol", "chat".parse().unwrap());
                    }
                    Ok(resp)
                };
                let Ok(mut ws) = tungstenite::accept_hdr(stream, callback) else { return };
                loop {
                    match ws.read() {
                        Ok(tungstenite::Message::Text(t)) if t.as_str() == "cai" => return,
                        Ok(tungstenite::Message::Text(t)) if t.as_str() == "tchau" => {
                            let _ = ws.close(Some(tungstenite::protocol::CloseFrame { code: 4000.into(), reason: "fim".into() }));
                        }
                        Ok(m @ (tungstenite::Message::Text(_) | tungstenite::Message::Binary(_))) => {
                            let _ = ws.send(m);
                        }
                        Ok(_) => {}
                        Err(_) => return,
                    }
                }
            });
        }
    });
    (port, count)
}

fn has(log: &Rc<RefCell<Vec<(String, String)>>>, event: &str) -> usize {
    log.borrow().iter().filter(|(e, _)| e == event).count()
}

#[test]
fn the_websocket_client_talks_and_reconnects() {
    let (port, connections) = ws_server();
    let (mut ui, id) = app(json!({
        "class": "WebSocketClient", "name": "wsAlertas", "url": format!("ws://127.0.0.1:{port}/alertas"),
        "protocols": "chat", "active": true, "reconnect_delay": 100, "reconnect_max_delay": 400
    }));
    let log = record(&mut ui, id, &["on_open", "on_message", "on_binary", "on_close", "on_error"]);
    // Carregado com `active`: conecta na primeira volta do timer.
    pump_until(&mut ui, 10.0, |_| has(&log, "on_open") == 1);
    assert!(ui.get::<WebSocketClient>(id).unwrap().connected());

    assert_eq!(ui.invoke(id, "send", &[s("olá")]).unwrap(), PropValue::Bool(true));
    pump_until(&mut ui, 10.0, |_| has(&log, "on_message") == 1);
    assert!(log.borrow().contains(&("on_message".into(), "olá".into())));
    ui.invoke(id, "send_binary", &[s(&aylis_core::image::encode_base64(&[1, 2, 3]))]).unwrap();
    pump_until(&mut ui, 10.0, |_| has(&log, "on_binary") == 1);
    let bin = log.borrow().iter().find(|(e, _)| e == "on_binary").unwrap().1.clone();
    assert_eq!(aylis_core::image::decode_base64(&bin).unwrap(), vec![1, 2, 3]);

    // O servidor derruba: reconecta sozinho.
    ui.invoke(id, "send", &[s("cai")]).unwrap();
    pump_until(&mut ui, 10.0, |_| has(&log, "on_close") == 1);
    let close = log.borrow().iter().find(|(e, _)| e == "on_close").unwrap().1.clone();
    let close = WebSocketClose::from_args(&aylis_core::EventArgs::Text(close)).unwrap();
    assert!(close.reconnecting && close.retry_ms >= 100, "{close:?}");
    assert!(!ui.get::<WebSocketClient>(id).unwrap().connected());
    pump_until(&mut ui, 10.0, |_| has(&log, "on_open") == 2);
    assert_eq!(connections.load(Ordering::SeqCst), 2);

    // O servidor fecha com código: o código e o motivo chegam.
    ui.invoke(id, "send", &[s("tchau")]).unwrap();
    pump_until(&mut ui, 10.0, |_| has(&log, "on_close") == 2);
    let close = log.borrow().iter().filter(|(e, _)| e == "on_close").nth(1).unwrap().1.clone();
    let close = WebSocketClose::from_args(&aylis_core::EventArgs::Text(close)).unwrap();
    assert_eq!((close.code, close.reason.as_str(), close.reconnecting), (4000, "fim", true));
    pump_until(&mut ui, 10.0, |_| has(&log, "on_open") == 3);

    // O app desliga: fecha sem reconectar.
    ui.set_prop(id, "active", false).unwrap();
    pump_until(&mut ui, 10.0, |_| has(&log, "on_close") == 3);
    let close = log.borrow().iter().filter(|(e, _)| e == "on_close").nth(2).unwrap().1.clone();
    assert!(!WebSocketClose::from_args(&aylis_core::EventArgs::Text(close)).unwrap().reconnecting);
    for _ in 0..40 {
        ui.process_posted();
        ui.advance(0.05);
    }
    assert_eq!(has(&log, "on_open"), 3, "desligado não reconecta");
    assert!(ui.invoke(id, "send", &[s("x")]).is_err(), "fechado não manda");
}

#[test]
fn the_websocket_client_backs_off_and_refuses_insecure_urls() {
    // Nada ouvindo na porta: falha, espera crescente.
    let free = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let (mut ui, id) = app(json!({
        "class": "WebSocketClient", "name": "wsNada", "url": format!("ws://127.0.0.1:{free}/"),
        "reconnect_delay": 100, "reconnect_max_delay": 250, "connect_timeout": 1000
    }));
    let log = record(&mut ui, id, &["on_open", "on_close", "on_error"]);
    ui.invoke(id, "connect", &[]).unwrap();
    pump_until(&mut ui, 15.0, |_| has(&log, "on_close") >= 3);
    let delays: Vec<i64> = log.borrow().iter().filter(|(e, _)| e == "on_close").map(|(_, t)| WebSocketClose::from_args(&aylis_core::EventArgs::Text(t.clone())).unwrap().retry_ms).collect();
    assert_eq!(&delays[..3], &[100, 200, 250], "dobra até o máximo");
    assert!(has(&log, "on_error") >= 3);
    ui.invoke(id, "disconnect", &[]).unwrap();

    // ws:// para fora deste computador: recusado sem tentar.
    let (mut ui, id) = app(json!({ "class": "WebSocketClient", "name": "wsFora", "url": "ws://example.com/x", "auto_reconnect": false }));
    let log = record(&mut ui, id, &["on_close", "on_error"]);
    ui.invoke(id, "connect", &[]).unwrap();
    pump_until(&mut ui, 5.0, |_| has(&log, "on_close") == 1);
    assert!(log.borrow().iter().any(|(e, t)| e == "on_error" && t.contains("sem TLS")), "{:?}", log.borrow());
}
