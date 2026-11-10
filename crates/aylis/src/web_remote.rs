//! Navegador comandado por um backend remoto (WebSocket), em qualquer linguagem: a página
//! conecta, o backend manda os comandos do protocolo (`load`, `set`, `data`...) e recebe
//! os eventos — como o `aylis-host`, sem instalar nada no computador do usuário.

use std::cell::RefCell;

use aylis_core::{Poster, Ui};
use serde_json::{Value, json};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{MessageEvent, WebSocket};

#[derive(Default)]
struct Remote {
    ws: Option<WebSocket>,
    /// Antes do `run`: a tela sendo montada.
    ui: Option<Ui>,
    /// Depois do `run`: comandos vão pela fila da interface.
    poster: Option<Poster>,
}

thread_local! {
    static REMOTE: RefCell<Remote> = RefCell::new(Remote::default());
}

fn send(message: &Value) {
    REMOTE.with(|r| {
        if let Some(ws) = &r.borrow().ws {
            let _ = ws.send_with_str(&message.to_string());
        }
    });
}

fn log(text: &str) {
    web_sys::console::log_1(&text.into());
}

/// Conecta em `url` (`ws://servidor:9000`) e deixa o backend montar e controlar a tela.
pub fn run_remote_web(url: &str) -> Result<(), String> {
    let mut ui = Ui::new();
    crate::register_all(&mut ui);
    aylis_remote::forward_events(&mut ui, |_, event| send(&event));
    let ws = WebSocket::new(url).map_err(|e| format!("WebSocket {url}: {e:?}"))?;
    // A ponte manda as mensagens grandes compactadas (quadro binário): chegam como ArrayBuffer.
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let data = event.data();
        let text = match data.as_string() {
            Some(text) => Some(text),
            None => data.dyn_ref::<js_sys::ArrayBuffer>().and_then(|b| aylis_remote::decompress(&js_sys::Uint8Array::new(b).to_vec())),
        };
        if let Some(text) = text {
            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                match serde_json::from_str::<Value>(line) {
                    Ok(message) => handle(message),
                    Err(e) => log(&format!("aylis: mensagem inválida ({e}): {line}")),
                }
            }
        }
    });
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    let on_close = Closure::<dyn FnMut()>::new(move || {
        log("aylis: o backend desconectou");
        REMOTE.with(|r| {
            if let Some(p) = &r.borrow().poster {
                p.post(|ui| ui.request_close());
            }
        });
    });
    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
    REMOTE.with(|r| {
        let mut r = r.borrow_mut();
        r.ws = Some(ws);
        r.ui = Some(ui);
    });
    Ok(())
}

/// Um comando do backend: antes do `run`, montando a tela; depois, pela fila da interface.
thread_local! {
    static HANDSHAKE: std::cell::RefCell<aylis_remote::Handshake> = std::cell::RefCell::new(aylis_remote::Handshake::default());
}

fn handle(message: Value) {
    // A primeira mensagem tem de ser o `hello` (a versão do protocolo).
    if let Some(refused) = HANDSHAKE.with(|h| h.borrow_mut().check(&message)) {
        send(&refused);
        return;
    }
    let started = REMOTE.with(|r| r.borrow().poster.clone());
    if let Some(poster) = started {
        poster.post(move |ui| {
            if let Some(reply) = aylis_remote::handle_message(ui, &message) {
                send(&reply);
            }
        });
        return;
    }
    if message["cmd"] == "run" {
        let Some(ui) = REMOTE.with(|r| r.borrow_mut().ui.take()) else { return };
        let form = match message.get("form").filter(|f| !f.is_null()).map(|f| aylis_remote::resolve(&ui, f)) {
            Some(Ok(f)) => f,
            Some(Err(e)) => {
                log(&format!("aylis: run: {e}"));
                REMOTE.with(|r| r.borrow_mut().ui = Some(ui));
                return;
            }
            None => match ui.roots().first().copied() {
                Some(f) => f,
                None => return,
            },
        };
        if let Some(id) = message.get("id") {
            send(&json!({ "id": id, "result": null }));
        }
        let poster = ui.poster();
        REMOTE.with(|r| r.borrow_mut().poster = Some(poster));
        if let Err(e) = aylis_platform_winit::run(ui, form) {
            log(&format!("aylis: {e}"));
        }
        return;
    }
    REMOTE.with(|r| {
        let mut r = r.borrow_mut();
        if let Some(ui) = r.ui.as_mut() {
            let reply = aylis_remote::handle_message(ui, &message);
            drop(r);
            if let Some(reply) = reply {
                send(&reply);
            }
        }
    });
}
