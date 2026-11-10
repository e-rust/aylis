//! O backend pela rede: um servidor WebSocket e o `aylis-host --connect` (a tela conecta no
//! servidor, nunca o contrário).

use std::net::TcpListener;
use std::process::{Command, Stdio};

use tungstenite::Message;

#[test]
fn a_backend_drives_the_screen_over_websocket() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let png = std::env::temp_dir().join(format!("aylis_ws_{}.png", std::process::id()));
    let mut host = Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .args(["--connect", &format!("ws://127.0.0.1:{port}"), "--png"])
        .arg(&png)
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    // O backend: um servidor que espera a tela.
    let (stream, _) = listener.accept().unwrap();
    let mut ws = tungstenite::accept(stream).unwrap();
    let call = |ws: &mut tungstenite::WebSocket<_>, json: &str| {
        ws.send(Message::text(json)).unwrap();
        loop {
            if let Message::Text(t) = ws.read().unwrap() {
                return serde_json::from_str::<serde_json::Value>(&t).unwrap();
            }
        }
    };
    // A primeira mensagem é o hello (a versão do protocolo).
    ws.send(Message::text(r#"{"cmd":"hello","protocol":1}"#)).unwrap();
    ws.send(Message::text(r#"{"cmd":"load","layout":{"class":"Form","name":"frm","width":200,"height":80,"children":[{"class":"Edit","name":"edt"}]}}"#)).unwrap();
    assert_eq!(call(&mut ws, r#"{"id":1,"cmd":"run","form":"frm"}"#)["id"], 1);
    ws.send(Message::text(r#"{"cmd":"set","target":"edt","props":{"text":"pela rede"}}"#)).unwrap();
    assert_eq!(call(&mut ws, r#"{"id":2,"cmd":"get","target":"edt","prop":"text"}"#)["result"], "pela rede");
    ws.send(Message::text(r#"{"cmd":"close"}"#)).unwrap();
    let status = host.wait().unwrap();
    assert!(status.success());
    assert!(std::fs::metadata(&png).unwrap().len() > 100);
    let _ = std::fs::remove_file(png);
}

#[test]
fn the_host_no_longer_waits_for_a_backend() {
    let out = Command::new(env!("CARGO_BIN_EXE_aylis-host")).args(["--listen", "127.0.0.1:0"]).output().unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("opção desconhecida '--listen'"));
}
