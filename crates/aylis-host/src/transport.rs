//! Como as mensagens chegam e saem: stdin/stdout de um processo filho (o padrão) ou
//! WebSocket (o backend pode estar em outra máquina e em qualquer linguagem com cliente ou
//! servidor WebSocket). Uma mensagem JSON por linha (stdio) ou por mensagem de texto (WS).

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Child, ChildStdin};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tungstenite::client::IntoClientRequest;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

/// Para onde vão as mensagens do host (respostas e eventos).
pub trait Outgoing: Send {
    fn send_line(&mut self, line: &str) -> std::io::Result<()>;
}

impl Outgoing for ChildStdin {
    fn send_line(&mut self, line: &str) -> std::io::Result<()> {
        writeln!(self, "{line}")?;
        self.flush()
    }
}

/// Saída por WebSocket: entrega à thread que cuida da conexão.
struct WsOut(Sender<String>);

impl Outgoing for WsOut {
    fn send_line(&mut self, line: &str) -> std::io::Result<()> {
        self.0.send(line.to_string()).map_err(|_| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "conexão fechada"))
    }
}

/// As mensagens que chegam do backend (uma por item; o fim = backend desconectou).
pub type Incoming = Box<dyn Iterator<Item = std::io::Result<String>> + Send>;

/// Uma conexão com o backend.
pub struct Connection {
    pub incoming: Incoming,
    pub outgoing: Box<dyn Outgoing>,
    /// O processo do backend (só no stdio).
    pub child: Option<Child>,
}

/// O backend como processo filho, por stdin/stdout.
pub fn stdio(mut child: Child) -> Connection {
    let stdin = child.stdin.take().expect("stdin configurado");
    let stdout = child.stdout.take().expect("stdout configurado");
    Connection { incoming: Box::new(BufReader::new(stdout).lines()), outgoing: Box::new(stdin), child: Some(child) }
}

/// Conecta num servidor WebSocket do backend: `ws://host:porta/caminho` ou `wss://...`
/// (TLS, conferindo o certificado com as raízes do sistema e, se dado, com `extra_ca`, um
/// PEM com o certificado de uma autoridade interna ou do próprio servidor).
pub fn connect(url: &str, extra_ca: Option<&Path>) -> Result<Connection, String> {
    let fail = |e: &dyn std::fmt::Display| format!("não consegui conectar em {url}: {e}");
    let request = url.into_client_request().map_err(|e| fail(&e))?;
    let secure = request.uri().scheme_str() == Some("wss");
    let host = request.uri().host().ok_or_else(|| fail(&"a URL não tem servidor"))?.trim_matches(['[', ']']).to_string();
    let port = request.uri().port_u16().unwrap_or(if secure { 443 } else { 80 });
    let stream = TcpStream::connect((host.as_str(), port)).map_err(|e| fail(&e))?;
    let connector = if secure { Some(tungstenite::Connector::Rustls(Arc::new(client_config(extra_ca)?))) } else { None };
    let (socket, _) = tungstenite::client_tls_with_config(request, stream, None, connector).map_err(|e| fail(&e))?;
    let tcp = match socket.get_ref() {
        MaybeTlsStream::Plain(s) => s,
        MaybeTlsStream::Rustls(s) => &s.sock,
        _ => return Err(fail(&"transporte inesperado")),
    };
    tcp.set_read_timeout(Some(POLL)).map_err(|e| e.to_string())?;
    Ok(pump(socket))
}

/// O TLS do cliente: as raízes do sistema (como o navegador) mais `extra_ca`.
fn client_config(extra_ca: Option<&Path>) -> Result<rustls::ClientConfig, String> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        // Uma raiz que o rustls não entende não impede as outras.
        let _ = roots.add(cert);
    }
    if let Some(path) = extra_ca {
        for cert in read_certs(path)? {
            roots.add(cert).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth())
}

fn read_certs(path: &Path) -> Result<Vec<CertificateDer<'static>>, String> {
    let certs: Vec<_> = CertificateDer::pem_file_iter(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .collect::<Result<_, _>>()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if certs.is_empty() {
        return Err(format!("{}: nenhum certificado no arquivo", path.display()));
    }
    Ok(certs)
}

/// Certificado e chave (PEM) para a ponte (`--bridge`) aceitar `wss://`.
pub struct Tls<'a> {
    pub cert: &'a Path,
    pub key: &'a Path,
}

/// A configuração TLS do servidor (`None`: sem TLS).
pub fn server_config(tls: Option<Tls<'_>>) -> Result<Option<Arc<rustls::ServerConfig>>, String> {
    Ok(match &tls {
        Some(t) => {
            let key = PrivateKeyDer::from_pem_file(t.key).map_err(|e| format!("{}: {e}", t.key.display()))?;
            let config = rustls::ServerConfig::builder().with_no_client_auth().with_single_cert(read_certs(t.cert)?, key).map_err(|e| format!("TLS: {e}"))?;
            Some(Arc::new(config))
        }
        None => None,
    })
}

/// Prazo para o handshake (TLS e WebSocket): quem abre a conexão e não completa não prende
/// uma thread para sempre.
const HANDSHAKE: Duration = Duration::from_secs(10);
/// Maior mensagem aceita da tela (e o maior quadro): uma conexão não esgota a memória.
const MAX_MESSAGE: usize = 1024 * 1024;
const MAX_FRAME: usize = 256 * 1024;
/// Mensagens da tela esperando o backend; acima disso a tela manda mais rápido do que o
/// backend consegue tratar, e a conexão fecha.
const MAX_QUEUED: usize = 1024;
/// De quanto em quanto tempo a ponte manda um ping (o app aberto responde sozinho).
const PING_EVERY: Duration = Duration::from_secs(60);
/// Sem nada da tela (nem a resposta do ping) por este tempo, a conexão fecha.
const IDLE: Duration = Duration::from_secs(30 * 60);

fn ws_config() -> tungstenite::protocol::WebSocketConfig {
    tungstenite::protocol::WebSocketConfig::default().max_message_size(Some(MAX_MESSAGE)).max_frame_size(Some(MAX_FRAME))
}

/// O handshake (TLS e WebSocket) de uma conexão aceita, já com a bomba de mensagens.
/// `origins`: as origens aceitas (`https://app.empresa.com`); uma conexão com `Origin` fora
/// da lista é recusada, e sem `Origin` (o app nativo não manda) é aceita. Lista vazia: todas.
pub fn accept(stream: TcpStream, config: Option<Arc<rustls::ServerConfig>>, origins: &[String]) -> Result<Connection, String> {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    stream.set_read_timeout(Some(HANDSHAKE)).map_err(|e| e.to_string())?;
    stream.set_write_timeout(Some(HANDSHAKE)).map_err(|e| e.to_string())?;
    let check = |request: &tungstenite::handshake::server::Request, response| match request.headers().get("origin").and_then(|o| o.to_str().ok()) {
        Some(origin) if !origins.is_empty() && !origins.iter().any(|o| o.trim_end_matches('/').eq_ignore_ascii_case(origin.trim_end_matches('/'))) => {
            let mut refused = tungstenite::handshake::server::ErrorResponse::new(Some(format!("origem não permitida: {origin}")));
            *refused.status_mut() = tungstenite::http::StatusCode::FORBIDDEN;
            Err(refused)
        }
        _ => Ok(response),
    };
    let fail = |e: &dyn std::fmt::Display| format!("handshake WebSocket com {peer}: {e}");
    Ok(match config {
        Some(config) => {
            let tls = rustls::ServerConnection::new(config).map_err(|e| format!("TLS: {e}"))?;
            let socket = tungstenite::accept_hdr_with_config(rustls::StreamOwned::new(tls, stream), check, Some(ws_config())).map_err(|e| fail(&e))?;
            socket.get_ref().sock.set_read_timeout(Some(POLL)).map_err(|e| e.to_string())?;
            socket.get_ref().sock.set_write_timeout(None).map_err(|e| e.to_string())?;
            pump(socket)
        }
        None => {
            let socket = tungstenite::accept_hdr_with_config(stream, check, Some(ws_config())).map_err(|e| fail(&e))?;
            // O prazo vai no próprio socket (no Windows, num clone ele não vale para o original).
            socket.get_ref().set_read_timeout(Some(POLL)).map_err(|e| e.to_string())?;
            socket.get_ref().set_write_timeout(None).map_err(|e| e.to_string())?;
            pump(socket)
        }
    })
}

/// Prazo de cada leitura: a thread alterna entre receber e enviar.
const POLL: Duration = Duration::from_millis(20);

/// Uma thread cuida da conexão: entrega o que chega e envia o que sai (o socket já tem o
/// prazo de leitura [`POLL`], para alternar entre os dois).
fn pump<S: std::io::Read + std::io::Write + Send + 'static>(mut socket: WebSocket<S>) -> Connection {
    // A entrada tem limite (a tela não enche a memória mais rápido do que o backend trata);
    // a saída vem do nosso backend.
    let (in_tx, in_rx) = mpsc::sync_channel::<String>(MAX_QUEUED);
    let (out_tx, out_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    std::thread::Builder::new()
        .name("aylis-ws".into())
        .spawn(move || {
            let debug = std::env::var("AYLIS_WS_DEBUG").is_ok();
            let (mut last_seen, mut last_ping) = (Instant::now(), Instant::now());
            // A tela anunciou `deflate` (na resposta do hello): as mensagens grandes vão
            // compactadas.
            let mut deflate = false;
            loop {
                if last_seen.elapsed() > IDLE {
                    let _ = socket.close(None);
                    let _ = socket.flush();
                    return;
                }
                if last_ping.elapsed() > PING_EVERY {
                    last_ping = Instant::now();
                    if socket.send(Message::Ping(Vec::new().into())).is_err() {
                        return;
                    }
                }
                // Envia o que estiver pendente.
                loop {
                    match out_rx.try_recv() {
                        Ok(line) => {
                            let message = if deflate && line.len() > aylis_remote::COMPRESS_ABOVE {
                                Message::binary(aylis_remote::compress(&line))
                            } else {
                                Message::text(line)
                            };
                            if socket.send(message).is_err() {
                                return;
                            }
                        }
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(mpsc::TryRecvError::Disconnected) => {
                            let _ = socket.close(None);
                            let _ = socket.flush();
                            return;
                        }
                    }
                }
                let result = socket.read();
                if debug {
                    if let Err(e) = &result {
                        if !matches!(e, tungstenite::Error::Io(io) if matches!(io.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut)) {
                            aylis_core::stderr_line!("aylis-ws: leitura: {e:?}");
                        }
                    } else {
                        aylis_core::stderr_line!("aylis-ws: recebido {:?}", result.as_ref().map(|m| m.len()));
                    }
                }
                if result.is_ok() {
                    last_seen = Instant::now();
                }
                match result {
                    Ok(Message::Text(text)) => {
                        // Uma mensagem pode trazer várias linhas (backends que mandam em lote).
                        for line in text.lines() {
                            if !deflate && aylis_remote::announces_deflate(line) {
                                deflate = true;
                            }
                            match in_tx.try_send(line.to_string()) {
                                Ok(()) => {}
                                Err(mpsc::TrySendError::Full(_)) => {
                                    aylis_core::stderr_line!("aylis-host: a tela mandou mensagens demais sem o backend dar conta; conexão fechada");
                                    let _ = socket.close(None);
                                    let _ = socket.flush();
                                    return;
                                }
                                Err(mpsc::TrySendError::Disconnected(_)) => return,
                            }
                        }
                    }
                    Ok(Message::Close(_)) => return,
                    Ok(_) => {}
                    Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                        let _ = socket.flush();
                    }
                    Err(_) => return,
                }
            }
        })
        .expect("thread do WebSocket");
    Connection { incoming: Box::new(in_rx.into_iter().map(Ok)), outgoing: Box::new(WsOut(out_tx)), child: None }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O lado servidor com TLS (o que a `--bridge` usa): handshake e mensagens nos dois
    /// sentidos, com um certificado gerado aqui.
    #[test]
    fn accepts_wss_with_the_given_certificate() {
        let dir = std::env::temp_dir().join(format!("aylis_accept_tls_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        std::fs::write(dir.join("cert.pem"), cert.cert.pem()).unwrap();
        std::fs::write(dir.join("key.pem"), cert.signing_key.serialize_pem()).unwrap();
        let (cert_path, key_path) = (dir.join("cert.pem"), dir.join("key.pem"));
        let config = server_config(Some(Tls { cert: &cert_path, key: &key_path })).unwrap();

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut connection = accept(stream, config, &[]).unwrap();
            connection.outgoing.send_line("{\"event\":\"ping\"}").unwrap();
            connection.incoming.next().unwrap().unwrap()
        });

        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert.cert.der().clone()).unwrap();
        let client = Arc::new(rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth());
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let (mut ws, _) = tungstenite::client_tls_with_config(format!("wss://localhost:{port}"), stream, None, Some(tungstenite::Connector::Rustls(client))).unwrap();
        let first = loop {
            if let Message::Text(t) = ws.read().unwrap() {
                break t.to_string();
            }
        };
        assert_eq!(first, "{\"event\":\"ping\"}");
        ws.send(Message::text("{\"cmd\":\"pong\"}")).unwrap();
        assert_eq!(server.join().unwrap(), "{\"cmd\":\"pong\"}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A ponte com `--allow-origin`: um site de fora é recusado no handshake; o app nativo
    /// (sem `Origin`) e a origem da lista entram.
    #[test]
    fn origins_outside_the_list_are_refused() {
        let origins = vec!["https://app.empresa.com".to_string()];
        let try_origin = |origin: Option<&'static str>| {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let port = listener.local_addr().unwrap().port();
            let origins = origins.clone();
            let server = std::thread::spawn(move || {
                let (stream, _) = listener.accept().unwrap();
                accept(stream, None, &origins).is_ok()
            });
            let mut request = tungstenite::client::IntoClientRequest::into_client_request(format!("ws://127.0.0.1:{port}")).unwrap();
            if let Some(o) = origin {
                request.headers_mut().insert("Origin", o.parse().unwrap());
            }
            let client = tungstenite::connect(request).is_ok();
            (server.join().unwrap(), client)
        };
        assert_eq!(try_origin(Some("https://evil.example")), (false, false));
        assert_eq!(try_origin(Some("https://app.empresa.com")), (true, true));
        assert_eq!(try_origin(None), (true, true));
    }

    /// Uma mensagem maior que o limite fecha a conexão em vez de ocupar a memória.
    #[test]
    fn huge_messages_close_the_connection() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let connection = accept(stream, None, &[]).unwrap();
            connection.incoming.count()
        });
        let (mut ws, _) = tungstenite::connect(format!("ws://127.0.0.1:{port}")).unwrap();
        let _ = ws.send(Message::text("x".repeat(2 * MAX_MESSAGE)));
        assert_eq!(server.join().unwrap(), 0, "nada chegou ao backend");
    }
}
