//! O `HttpClient` e o `WebSocketClient` sem janela: um form em JSON com os dois, os eventos
//! escritos no terminal. O laço aqui faz o papel do laço da janela (`process_posted` e o
//! relógio).
//!
//! ```text
//! cargo run -p aylis-providers --features http,websocket --example network -- [url-http] [url-ws]
//! ```
//!
//! Sem argumentos: um `GET` em `https://example.com/` e um eco WebSocket em
//! `wss://ws.postman-echo.com/raw`, que recebe "olá" e devolve.

use std::time::{Duration, Instant};

use aylis_core::{EventArgs, EventValue, PropValue, Ui};
use aylis_providers::net::{HttpResult, WebSocketClose};
use serde_json::json;

fn main() {
    let mut args = std::env::args().skip(1);
    let http = args.next().unwrap_or_else(|| "https://example.com/".into());
    let ws = args.next().unwrap_or_else(|| "wss://ws.postman-echo.com/raw".into());

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_providers::register(&mut ui);
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmRede", "children": [
        { "class": "HttpClient", "name": "httpApi", "headers": [{ "name": "Accept", "value": "text/html,application/json" }], "request_timeout": 15000 },
        { "class": "WebSocketClient", "name": "wsEco", "url": ws, "reconnect_delay": 1000 }
    ] } });
    aylis_json::load_value(&mut ui, &layout, None).expect("o form de exemplo");
    let (api, eco) = (ui.find("httpApi").unwrap(), ui.find("wsEco").unwrap());

    let done = std::rc::Rc::new(std::cell::Cell::new(0));
    for event in ["on_response", "on_error"] {
        let done = done.clone();
        ui.on(api, event, move |_, _, args: &EventArgs| {
            let r = HttpResult::from_args(args).unwrap_or_default();
            if r.error.is_empty() {
                println!("HTTP {} {} -> {} ({} bytes, {} ms, {})", r.method, r.url, r.status, r.body.len(), r.elapsed_ms, r.content_type);
            } else {
                println!("HTTP {} {} falhou: {}", r.method, r.url, r.error);
            }
            done.set(done.get() + 1);
        });
    }
    ui.on(eco, "on_open", |ui, id, _| {
        println!("WebSocket aberto");
        let _ = ui.invoke(id, "send", &[PropValue::String("olá".into())]);
    });
    {
        let done = done.clone();
        ui.on(eco, "on_message", move |ui, id, args| {
            println!("WebSocket recebeu: {}", args.text().unwrap_or_default());
            let _ = ui.invoke(id, "disconnect", &[]);
            done.set(done.get() + 1);
        });
    }
    ui.on(eco, "on_close", |_, _, args| {
        let c = WebSocketClose::from_args(args).unwrap_or_default();
        println!("WebSocket fechou: {} {} (reconecta: {}, em {} ms)", c.code, c.reason, c.reconnecting, c.retry_ms);
    });
    ui.on(eco, "on_error", |_, _, args| println!("WebSocket: {}", args.text().unwrap_or_default()));

    ui.invoke(api, "get", &[PropValue::String(http)]).expect("o pedido");
    ui.invoke(eco, "connect", &[]).expect("a conexão");

    let start = Instant::now();
    while done.get() < 2 && start.elapsed() < Duration::from_secs(30) {
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(10));
        ui.advance(0.01);
    }
}
