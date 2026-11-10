//! O app ligado a um backend remoto (WebSocket): a tela (com os forms do projeto já
//! carregados) roda no aparelho e o código dos eventos roda num servidor, em qualquer
//! linguagem — o projeto em Python, TypeScript, C#... publicado para o celular e o
//! navegador. No servidor, `aylis-host --bridge 0.0.0.0:9000 -- python src/main.py` inicia
//! um backend por conexão.
//!
//! Os comandos do backend chegam pela fila da interface; os eventos com handler por nome
//! vão para ele. O `run` só confirma: o form principal já está aberto.

use aylis_core::Ui;
use serde_json::{Value, json};

thread_local! {
    /// O aperto de mão com o backend (o `hello` primeiro), na thread da interface.
    static HANDSHAKE: std::cell::RefCell<aylis_remote::Handshake> = std::cell::RefCell::new(aylis_remote::Handshake::default());
}

/// Um comando do backend, na thread da interface. Devolve a resposta (se houver).
fn handle(ui: &mut Ui, message: &Value) -> Option<Value> {
    if let Some(refused) = HANDSHAKE.with(|h| h.borrow_mut().check(message)) {
        return Some(refused);
    }
    if message["cmd"] == "run" {
        // O form já está aberto (o principal do projeto); outro form pedido abre por cima.
        if let Some(form) = message.get("form").filter(|f| !f.is_null()) {
            if let Ok(id) = aylis_remote::resolve(ui, form) {
                let _ = ui.show(id);
            }
        }
        return message.get("id").map(|id| json!({ "id": id, "result": null }));
    }
    aylis_remote::handle_message(ui, message)
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::io::ErrorKind;
    use std::sync::mpsc;
    use std::time::Duration;

    use aylis_core::Ui;
    use serde_json::Value;
    use tungstenite::Message;

    /// Liga a tela ao backend em `url` (`ws://` ou `wss://`). A conexão roda numa thread
    /// própria; se cair, a tela mostra o aviso e segue aberta.
    pub fn connect_backend(ui: &mut Ui, url: &str) -> Result<(), String> {
        let (mut socket, _) = tungstenite::connect(url).map_err(|e| format!("backend {url}: {e}"))?;
        // Leitura com prazo curto: a mesma thread lê e escreve.
        let tcp = match socket.get_mut() {
            tungstenite::stream::MaybeTlsStream::Plain(s) => s,
            tungstenite::stream::MaybeTlsStream::Rustls(s) => s.get_mut(),
            _ => return Err("backend: tipo de conexão não suportado".into()),
        };
        tcp.set_read_timeout(Some(Duration::from_millis(30))).map_err(|e| e.to_string())?;
        let (tx, rx) = mpsc::channel::<String>();
        let events = tx.clone();
        aylis_remote::forward_events(ui, move |_, event| {
            let _ = events.send(event.to_string());
        });
        let poster = ui.poster();
        std::thread::Builder::new()
            .name("aylis-backend".into())
            .spawn(move || {
                loop {
                    while let Ok(text) = rx.try_recv() {
                        if socket.send(Message::text(text)).is_err() {
                            break;
                        }
                    }
                    // A ponte manda as mensagens grandes compactadas (quadro binário).
                    let text = match socket.read() {
                        Ok(Message::Text(text)) => Ok(text.to_string()),
                        Ok(Message::Binary(bytes)) => Ok(aylis_remote::decompress(&bytes).unwrap_or_default()),
                        other => Err(other),
                    };
                    match text {
                        Ok(text) => {
                            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                                let Ok(message) = serde_json::from_str::<Value>(line) else { continue };
                                let tx = tx.clone();
                                poster.post(move |ui| {
                                    if let Some(reply) = super::handle(ui, &message) {
                                        let _ = tx.send(reply.to_string());
                                    }
                                    ui.process_events();
                                });
                            }
                        }
                        Err(Ok(Message::Close(_))) => break,
                        Err(Ok(_)) => {}
                        Err(Err(tungstenite::Error::Io(e))) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
                        Err(Err(_)) => break,
                    }
                }
                poster.post(|ui| {
                    aylis_controls::toast(ui, &aylis_core::messages::t("O servidor desconectou"));
                });
            })
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::connect_backend;

#[cfg(target_arch = "wasm32")]
mod web {
    use std::cell::RefCell;

    use aylis_core::Ui;
    use serde_json::Value;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    use web_sys::{MessageEvent, WebSocket};

    thread_local! {
        static SOCKET: RefCell<Option<WebSocket>> = const { RefCell::new(None) };
        /// O que sair antes de a conexão abrir.
        static WAITING: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    }

    fn send(text: String) {
        SOCKET.with(|s| match s.borrow().as_ref().filter(|ws| ws.ready_state() == WebSocket::OPEN) {
            Some(ws) => {
                let _ = ws.send_with_str(&text);
            }
            None => WAITING.with(|w| w.borrow_mut().push(text)),
        });
    }

    /// Liga a tela ao backend em `url` (`ws://` ou `wss://`).
    pub fn connect_backend(ui: &mut Ui, url: &str) -> Result<(), String> {
        let ws = WebSocket::new(url).map_err(|e| format!("backend {url}: {e:?}"))?;
        aylis_remote::forward_events(ui, |_, event| send(event.to_string()));
        let poster = ui.poster();
        let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
            let Some(text) = event.data().as_string() else { return };
            for line in text.lines().filter(|l| !l.trim().is_empty()) {
                let Ok(message) = serde_json::from_str::<Value>(line) else { continue };
                poster.post(move |ui| {
                    if let Some(reply) = super::handle(ui, &message) {
                        send(reply.to_string());
                    }
                    ui.process_events();
                });
            }
        });
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        let on_open = Closure::<dyn FnMut()>::new(move || {
            for text in WAITING.with(|w| std::mem::take(&mut *w.borrow_mut())) {
                send(text);
            }
        });
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        SOCKET.with(|s| *s.borrow_mut() = Some(ws));
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::connect_backend;
