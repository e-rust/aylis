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

/// O backend em outro processo na mesma máquina: o app publicado no desktop por um projeto
/// em outra linguagem é a tela (este executável, com o projeto embutido) mais o programa da
/// linguagem ao lado, que conversa por stdin/stdout como no `aylis-host`.
#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
mod local {
    use std::io::{BufRead, BufReader, Write};
    use std::path::Path;
    use std::process::{Child, ChildStdin, Command, Stdio};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use aylis_core::Ui;
    use serde_json::Value;

    type Stdin = Arc<Mutex<Option<ChildStdin>>>;

    fn write_line(stdin: &Stdin, text: &str) {
        if let Ok(mut guard) = stdin.lock() {
            if let Some(pipe) = guard.as_mut() {
                // O backend pode ter terminado; nesse caso não há a quem avisar.
                let _ = writeln!(pipe, "{text}").and_then(|_| pipe.flush());
            }
        }
    }

    /// O processo do backend. Ao sair de cena (o app fechou), o backend recebe
    /// `{"event": "closed"}`, o stdin dele fecha e ele tem até 3 s para terminar sozinho.
    pub struct BackendProcess {
        child: Child,
        stdin: Stdin,
    }

    impl Drop for BackendProcess {
        fn drop(&mut self) {
            write_line(&self.stdin, &serde_json::json!({ "event": "closed" }).to_string());
            if let Ok(mut guard) = self.stdin.lock() {
                guard.take();
            }
            let deadline = Instant::now() + Duration::from_secs(3);
            while Instant::now() < deadline {
                if let Ok(Some(_)) = self.child.try_wait() {
                    return;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }

    /// Inicia o backend (`command`: o programa e os argumentos) na pasta `dir` e liga a tela a
    /// ele. Um programa com caminho relativo é procurado em `dir` (`backend/app.exe`); um nome
    /// solto (`dotnet`, `java`, `python`), no PATH. Se o backend terminar, o app fecha (como no
    /// `aylis-host`). Guarde o resultado até o app fechar.
    pub fn spawn_backend(ui: &mut Ui, command: &[&str], dir: &Path) -> Result<BackendProcess, String> {
        let (program, args) = command.split_first().ok_or("backend: comando vazio")?;
        let local = dir.join(program);
        let mut c = if local.is_file() { Command::new(local) } else { Command::new(program) };
        c.args(args).current_dir(dir).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());
        // O backend é um programa de console: sem isto, o app abriria uma janela preta junto.
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            c.creation_flags(CREATE_NO_WINDOW);
        }
        let mut child = c.spawn().map_err(|e| format!("não consegui iniciar o backend '{program}': {e}"))?;
        let stdout = child.stdout.take().ok_or("backend: sem stdout")?;
        let stdin: Stdin = Arc::new(Mutex::new(child.stdin.take()));
        let events = stdin.clone();
        aylis_remote::forward_events(ui, move |_, event| write_line(&events, &event.to_string()));
        let poster = ui.poster();
        let replies = stdin.clone();
        std::thread::Builder::new()
            .name("aylis-backend".into())
            .spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    let Ok(line) = line else { break };
                    if line.trim().is_empty() {
                        continue;
                    }
                    let Ok(message) = serde_json::from_str::<Value>(&line) else { continue };
                    let replies = replies.clone();
                    poster.post(move |ui| {
                        if let Some(reply) = super::handle(ui, &message) {
                            write_line(&replies, &reply.to_string());
                        }
                        ui.process_events();
                    });
                }
                // O backend terminou: o app fecha junto.
                poster.post(|ui| ui.request_close());
            })
            .map_err(|e| e.to_string())?;
        Ok(BackendProcess { child, stdin })
    }
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
pub use local::{BackendProcess, spawn_backend};

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
