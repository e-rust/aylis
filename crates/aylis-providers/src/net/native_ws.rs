//! O WebSocket do Aylis fora do navegador: o `tungstenite` com rustls (as raízes do Mozilla
//! embutidas, sem OpenSSL), uma thread por conexão. A thread lê com um prazo curto
//! ([`POLL`]) e, entre uma leitura e outra, manda o que a tela pediu (o `tungstenite` não lê e
//! escreve ao mesmo tempo no mesmo soquete com TLS): uma mensagem do app sai em até
//! [`POLL`]. A thread acaba quando a conexão fecha.

use std::io::ErrorKind;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::time::Duration;

use aylis_core::providers::{WebSocketConnection, WebSocketEvent, WebSocketEvents, WebSocketRequest};
use tungstenite::client::IntoClientRequest;
use tungstenite::protocol::frame::CloseFrame;
use tungstenite::protocol::WebSocketConfig;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Connector, Error, Message, WebSocket};

/// De quanto em quanto a thread olha se a tela mandou algo.
const POLL: Duration = Duration::from_millis(50);
/// Quanto esperar a resposta do servidor depois de pedir o fechamento.
const CLOSE_WAIT: Duration = Duration::from_secs(5);

enum Out {
    Text(String),
    Binary(Vec<u8>),
    Close(u16, String),
}

struct NativeWs {
    tx: Sender<Out>,
}

impl WebSocketConnection for NativeWs {
    fn send_text(&mut self, text: &str) -> Result<(), String> {
        self.tx.send(Out::Text(text.to_string())).map_err(|_| "WebSocket: a conexão fechou".to_string())
    }

    fn send_binary(&mut self, data: &[u8]) -> Result<(), String> {
        self.tx.send(Out::Binary(data.to_vec())).map_err(|_| "WebSocket: a conexão fechou".to_string())
    }

    fn close(&mut self, code: u16, reason: &str) {
        let _ = self.tx.send(Out::Close(code, reason.to_string()));
    }
}

pub(super) fn connect(request: WebSocketRequest, mut events: WebSocketEvents) -> Box<dyn WebSocketConnection> {
    let (tx, rx) = channel();
    let spawned = std::thread::Builder::new().name("aylis-websocket".into()).spawn(move || run(&request, &rx, &mut events));
    if let Err(e) = spawned {
        // A thread não nasceu: os eventos (que foram com ela) não chegam; a conexão fica morta.
        aylis_core::stderr_line!("WebSocket: sem thread: {e}");
    }
    Box::new(NativeWs { tx })
}

fn tls() -> Result<Arc<rustls::ClientConfig>, String> {
    let roots = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .map_err(|e| format!("TLS: {e}"))?
        .with_root_certificates(roots)
        .with_no_client_auth();
    Ok(Arc::new(config))
}

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

fn tcp(socket: &mut Socket) -> &mut TcpStream {
    match socket.get_mut() {
        MaybeTlsStream::Rustls(s) => s.get_mut(),
        MaybeTlsStream::Plain(s) => s,
        _ => unreachable!("só há TCP puro e rustls"),
    }
}

/// Abre: o TCP com prazo, o TLS e o aperto de mão do WebSocket.
fn open(request: &WebSocketRequest) -> Result<(Socket, String), String> {
    let mut req = request.url.as_str().into_client_request().map_err(|e| format!("WebSocket: endereço inválido: {e}"))?;
    let uri = req.uri().clone();
    let host = uri.host().ok_or("WebSocket: endereço sem host")?.trim_start_matches('[').trim_end_matches(']').to_string();
    let secure = uri.scheme_str() == Some("wss");
    let port = uri.port_u16().unwrap_or(if secure { 443 } else { 80 });
    for (k, v) in &request.headers {
        let name = tungstenite::http::HeaderName::from_bytes(k.as_bytes()).map_err(|e| format!("WebSocket: cabeçalho '{k}': {e}"))?;
        let value = tungstenite::http::HeaderValue::from_str(v).map_err(|e| format!("WebSocket: cabeçalho '{k}': {e}"))?;
        req.headers_mut().insert(name, value);
    }
    if !request.protocols.is_empty() {
        let value = tungstenite::http::HeaderValue::from_str(&request.protocols.join(", ")).map_err(|e| format!("WebSocket: subprotocolo: {e}"))?;
        req.headers_mut().insert("Sec-WebSocket-Protocol", value);
    }
    let timeout = (request.connect_timeout_ms > 0).then(|| Duration::from_millis(u64::from(request.connect_timeout_ms)));
    let addrs: Vec<_> = (host.as_str(), port).to_socket_addrs().map_err(|e| format!("WebSocket: {host}: {e}"))?.collect();
    let mut last = format!("WebSocket: {host}: sem endereço");
    let mut stream = None;
    for addr in addrs {
        let attempt = match timeout {
            Some(t) => TcpStream::connect_timeout(&addr, t),
            None => TcpStream::connect(addr),
        };
        match attempt {
            Ok(s) => {
                stream = Some(s);
                break;
            }
            Err(e) => last = format!("WebSocket: {host}:{port}: {e}"),
        }
    }
    let stream = stream.ok_or(last)?;
    let _ = stream.set_nodelay(true);
    // O aperto de mão inteiro (TLS e HTTP) dentro do prazo de abrir.
    stream.set_read_timeout(timeout).map_err(|e| e.to_string())?;
    stream.set_write_timeout(timeout).map_err(|e| e.to_string())?;
    let limit = usize::try_from(request.message_limit()).unwrap_or(usize::MAX);
    let config = WebSocketConfig::default().max_message_size(Some(limit)).max_frame_size(Some(limit));
    let connector = if secure { Connector::Rustls(tls()?) } else { Connector::Plain };
    let (mut socket, response) = tungstenite::client_tls_with_config(req, stream, Some(config), Some(connector)).map_err(|e| match e {
        tungstenite::HandshakeError::Interrupted(_) => "WebSocket: prazo esgotado ao abrir".to_string(),
        tungstenite::HandshakeError::Failure(Error::Io(io)) if matches!(io.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) => "WebSocket: prazo esgotado ao abrir".to_string(),
        tungstenite::HandshakeError::Failure(e) => format!("WebSocket: {e}"),
    })?;
    let protocol = response.headers().get("sec-websocket-protocol").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    let tcp = tcp(&mut socket);
    tcp.set_read_timeout(Some(POLL)).map_err(|e| e.to_string())?;
    tcp.set_write_timeout(Some(Duration::from_secs(30))).map_err(|e| e.to_string())?;
    Ok((socket, protocol))
}

fn run(request: &WebSocketRequest, rx: &Receiver<Out>, events: &mut WebSocketEvents) {
    let (mut socket, protocol) = match open(request) {
        Ok(s) => s,
        Err(e) => {
            events(WebSocketEvent::Error(e.clone()));
            events(WebSocketEvent::Closed { code: 1006, reason: e });
            return;
        }
    };
    events(WebSocketEvent::Open { protocol });
    let mut close: Option<(u16, String)> = None;
    let mut closing = false;
    let mut waited = Duration::ZERO;
    loop {
        // O que a tela mandou.
        loop {
            match rx.try_recv() {
                Ok(Out::Text(t)) => {
                    let _ = socket.send(Message::text(t));
                }
                Ok(Out::Binary(b)) => {
                    let _ = socket.send(Message::binary(b));
                }
                Ok(Out::Close(code, reason)) if !closing => {
                    closing = true;
                    let _ = socket.close(Some(CloseFrame { code: CloseCode::from(code), reason: reason.into() }));
                }
                Ok(Out::Close(..)) => {}
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // O componente foi liberado: fecha sem avisar ninguém.
                    if !closing {
                        closing = true;
                        let _ = socket.close(Some(CloseFrame { code: CloseCode::Away, reason: "".into() }));
                    }
                    break;
                }
            }
        }
        match socket.read() {
            Ok(Message::Text(t)) => events(WebSocketEvent::Text(t.to_string())),
            Ok(Message::Binary(b)) => events(WebSocketEvent::Binary(b.to_vec())),
            Ok(Message::Close(frame)) => {
                if let Some(f) = frame {
                    close = Some((u16::from(f.code), f.reason.to_string()));
                }
            }
            // O pong do ping sai no próximo `flush`.
            Ok(_) => {}
            Err(Error::Io(e)) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                let _ = socket.flush();
                if closing {
                    // O servidor não respondeu ao fechamento: desiste depois de um tempo.
                    waited += POLL;
                    if waited >= CLOSE_WAIT {
                        let (code, reason) = close.unwrap_or((1000, String::new()));
                        events(WebSocketEvent::Closed { code, reason });
                        return;
                    }
                }
            }
            Err(Error::ConnectionClosed | Error::AlreadyClosed) => {
                let (code, reason) = close.unwrap_or((if closing { 1000 } else { 1005 }, String::new()));
                events(WebSocketEvent::Closed { code, reason });
                return;
            }
            Err(Error::Capacity(e)) => {
                let reason = format!("mensagem maior que o limite: {e}");
                events(WebSocketEvent::Error(format!("WebSocket: {reason}")));
                let _ = socket.close(Some(CloseFrame { code: CloseCode::Size, reason: "".into() }));
                let _ = socket.flush();
                events(WebSocketEvent::Closed { code: 1009, reason });
                return;
            }
            Err(e) => {
                let reason = e.to_string();
                if !closing {
                    events(WebSocketEvent::Error(format!("WebSocket: {reason}")));
                }
                let (code, reason) = close.unwrap_or((1006, reason));
                events(WebSocketEvent::Closed { code, reason });
                return;
            }
        }
    }
}
