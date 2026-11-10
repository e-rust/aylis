//! `wss://` direto, com um certificado gerado aqui (de "localhost").

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use serde_json::Value;
use tungstenite::{Message, WebSocket};

struct Cert {
    dir: PathBuf,
    der: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
}

impl Cert {
    fn cert_path(&self) -> PathBuf {
        self.dir.join("cert.pem")
    }
}

/// Certificado autoassinado de "localhost", gravado em PEM numa pasta temporária.
fn certificate(name: &str) -> Cert {
    let dir = std::env::temp_dir().join(format!("aylis_wss_{name}_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    std::fs::write(dir.join("cert.pem"), cert.cert.pem()).unwrap();
    std::fs::write(dir.join("key.pem"), cert.signing_key.serialize_pem()).unwrap();
    let der = cert.cert.der().clone();
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der()));
    Cert { dir, der, key }
}

fn host(args: &[&str], png: &Path) -> Child {
    Command::new(env!("CARGO_BIN_EXE_aylis-host")).args(args).arg("--png").arg(png).stderr(Stdio::null()).spawn().unwrap()
}

/// Manda o mesmo roteiro do teste de WebSocket: monta, abre, escreve, lê e fecha.
fn script<S: std::io::Read + std::io::Write>(ws: &mut WebSocket<S>) {
    let call = |ws: &mut WebSocket<S>, json: &str| {
        ws.send(Message::text(json)).unwrap();
        loop {
            if let Message::Text(t) = ws.read().unwrap() {
                let v: Value = serde_json::from_str(&t).unwrap();
                if v.get("id").is_some() {
                    return v;
                }
            }
        }
    };
    ws.send(Message::text(r#"{"cmd":"hello","protocol":1}"#)).unwrap();
    ws.send(Message::text(r#"{"cmd":"load","layout":{"class":"Form","name":"frm","width":200,"height":80,"children":[{"class":"Edit","name":"edt"}]}}"#)).unwrap();
    assert_eq!(call(ws, r#"{"id":1,"cmd":"run","form":"frm"}"#)["id"], 1);
    ws.send(Message::text(r#"{"cmd":"set","target":"edt","props":{"text":"com TLS"}}"#)).unwrap();
    assert_eq!(call(ws, r#"{"id":2,"cmd":"get","target":"edt","prop":"text"}"#)["result"], "com TLS");
    ws.send(Message::text(r#"{"cmd":"close"}"#)).unwrap();
}

#[test]
fn the_host_connects_to_a_wss_backend() {
    let cert = certificate("connect");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let png = cert.dir.join("tela.png");
    // O backend: servidor wss com o certificado gerado.
    let config = Arc::new(rustls::ServerConfig::builder().with_no_client_auth().with_single_cert(vec![cert.der.clone()], cert.key.clone_key()).unwrap());
    let backend = std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let tls = rustls::StreamOwned::new(rustls::ServerConnection::new(config).unwrap(), stream);
        let mut ws = tungstenite::accept(tls).unwrap();
        script(&mut ws);
    });
    // Sem a autoridade, o host recusa o certificado (de um servidor TLS de verdade).
    let untrusted = TcpListener::bind("127.0.0.1:0").unwrap();
    let untrusted_port = untrusted.local_addr().unwrap().port();
    let untrusted_config = Arc::new(rustls::ServerConfig::builder().with_no_client_auth().with_single_cert(vec![cert.der.clone()], cert.key.clone_key()).unwrap());
    std::thread::spawn(move || {
        if let Ok((stream, _)) = untrusted.accept() {
            let tls = rustls::StreamOwned::new(rustls::ServerConnection::new(untrusted_config).unwrap(), stream);
            let _ = tungstenite::accept(tls);
        }
    });
    let refused = Command::new(env!("CARGO_BIN_EXE_aylis-host")).args(["--connect", &format!("wss://localhost:{untrusted_port}")]).stderr(Stdio::piped()).output().unwrap();
    assert!(!refused.status.success());
    let message = String::from_utf8_lossy(&refused.stderr);
    assert!(message.contains("UnknownIssuer") || message.to_lowercase().contains("certific"), "{message}");
    let mut child = host(&["--connect", &format!("wss://localhost:{port}"), "--tls-ca", cert.cert_path().to_str().unwrap()], &png);
    backend.join().unwrap();
    assert!(child.wait().unwrap().success());
    assert!(std::fs::metadata(&png).unwrap().len() > 100);
    let _ = std::fs::remove_dir_all(&cert.dir);
}
