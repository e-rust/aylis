//! # aylis-devlink
//!
//! O canal de desenvolvimento entre o app em execução (compilado com a feature `dev`) e a IDE:
//! o app conecta no endereço da IDE embutido nele na compilação e os dois conversam cifrados
//! com o Noise `NKpsk0` (X25519, ChaCha20-Poly1305, BLAKE2s), por cima de um WebSocket comum.
//!
//! - **NK**: o app já conhece a chave pública da IDE (embutida). Só quem tem a chave privada
//!   (a IDE desta máquina) completa o aperto de mão: outro processo na mesma porta, ou outra
//!   máquina com aquele IP, não se passa pela IDE.
//! - **psk0**: o segredo do projeto entra já na primeira mensagem. A IDE recusa na hora quem
//!   não tem o segredo (antes de pôr o app na lista ou ler o que ele diz), e o app só aceita
//!   a resposta de quem tem o segredo e a chave privada. Um app vazado (um APK de testes) dá
//!   o segredo, mas não a chave privada da IDE.
//!
//! Cada mensagem JSON vai cifrada em um ou mais quadros binários (o Noise limita a 64 KB por
//! mensagem): o primeiro byte do texto claro diz se há continuação.

use serde_json::{Value, json};

/// O padrão do Noise.
pub const PATTERN: &str = "Noise_NKpsk0_25519_ChaChaPoly_BLAKE2s";
/// A porta padrão do canal (a do MCP é a 7341).
pub const DEFAULT_PORT: u16 = 7342;
/// Texto claro por quadro (o Noise aceita 65535 bytes com a etiqueta de 16).
const CHUNK: usize = 65_000;

fn params() -> snow::params::NoiseParams {
    PATTERN.parse().expect("padrão do Noise válido")
}

/// Bytes em hexadecimal (as chaves nos arquivos JSON).
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Hexadecimal de 64 dígitos em 32 bytes.
pub fn unhex32(text: &str) -> Option<[u8; 32]> {
    if text.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(text.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

/// Um par de chaves X25519 novo (a IDE gera uma vez por máquina): (privada, pública).
pub fn keypair() -> Result<([u8; 32], [u8; 32]), String> {
    let k = snow::Builder::new(params()).generate_keypair().map_err(|e| e.to_string())?;
    Ok((k.private.try_into().map_err(|_| "chave privada")?, k.public.try_into().map_err(|_| "chave pública")?))
}

/// 32 bytes aleatórios (o segredo de um projeto).
pub fn secret() -> Result<[u8; 32], String> {
    // A chave privada de um par novo é aleatória e uniforme o bastante para o PSK.
    Ok(keypair()?.0)
}

/// O que o app de desenvolvimento leva embutido: onde a IDE está, a chave pública dela e o
/// segredo do projeto.
#[derive(Clone, Debug, PartialEq)]
pub struct Target {
    /// `host:porta` (`127.0.0.1:7342`).
    pub address: String,
    pub ide_public: [u8; 32],
    pub secret: [u8; 32],
}

impl Target {
    pub fn to_json(&self) -> Value {
        json!({ "address": self.address, "ide_public": hex(&self.ide_public), "secret": hex(&self.secret) })
    }

    pub fn from_json(v: &Value) -> Option<Target> {
        Some(Target {
            address: v["address"].as_str()?.to_string(),
            ide_public: unhex32(v["ide_public"].as_str()?)?,
            secret: unhex32(v["secret"].as_str()?)?,
        })
    }

    /// O endereço do WebSocket (`ws://127.0.0.1:7342`).
    pub fn url(&self) -> String {
        format!("ws://{}", self.address)
    }
}

/// O canal depois do aperto de mão: cifra e decifra as mensagens.
pub struct Session {
    state: snow::TransportState,
    pending: Vec<u8>,
}

impl Session {
    /// Cifra uma mensagem: os quadros binários, na ordem.
    pub fn seal(&mut self, text: &str) -> Result<Vec<Vec<u8>>, String> {
        let bytes = text.as_bytes();
        let pieces: Vec<&[u8]> = if bytes.is_empty() { vec![&[]] } else { bytes.chunks(CHUNK).collect() };
        let last = pieces.len() - 1;
        pieces
            .into_iter()
            .enumerate()
            .map(|(i, piece)| {
                let mut plain = Vec::with_capacity(piece.len() + 1);
                plain.push(u8::from(i == last));
                plain.extend_from_slice(piece);
                let mut out = vec![0u8; plain.len() + 16];
                let n = self.state.write_message(&plain, &mut out).map_err(|e| e.to_string())?;
                out.truncate(n);
                Ok(out)
            })
            .collect()
    }

    /// Decifra um quadro; devolve a mensagem quando ela termina. Um quadro adulterado (ou de
    /// quem não fez o aperto de mão) é erro, e a conexão deve fechar.
    pub fn open(&mut self, frame: &[u8]) -> Result<Option<String>, String> {
        let mut plain = vec![0u8; frame.len()];
        let n = self.state.read_message(frame, &mut plain).map_err(|_| "quadro do canal de desenvolvimento inválido".to_string())?;
        let (flag, data) = plain[..n].split_first().ok_or("quadro vazio")?;
        self.pending.extend_from_slice(data);
        if *flag == 1 {
            let text = String::from_utf8(std::mem::take(&mut self.pending)).map_err(|_| "mensagem não é UTF-8")?;
            Ok(Some(text))
        } else {
            Ok(None)
        }
    }
}

/// O lado do app, a meio do aperto de mão.
pub struct Initiator(snow::HandshakeState);

/// O app começa: a primeira mensagem (`hello`: o projeto e o processo, em JSON) vai cifrada
/// para a chave pública da IDE.
pub fn initiate(target: &Target, hello: &str) -> Result<(Initiator, Vec<u8>), String> {
    let mut hs = snow::Builder::new(params())
        .remote_public_key(&target.ide_public)
        .psk(0, &target.secret)
        .build_initiator()
        .map_err(|e| e.to_string())?;
    let mut out = vec![0u8; hello.len() + 128];
    let n = hs.write_message(hello.as_bytes(), &mut out).map_err(|e| e.to_string())?;
    out.truncate(n);
    Ok((Initiator(hs), out))
}

impl Initiator {
    /// A resposta da IDE: só confere com a chave privada dela e o segredo do projeto.
    pub fn finish(mut self, reply: &[u8]) -> Result<Session, String> {
        let mut payload = vec![0u8; reply.len()];
        self.0.read_message(reply, &mut payload).map_err(|_| "a IDE do outro lado não confere (chave ou segredo errados)".to_string())?;
        Ok(Session { state: self.0.into_transport_mode().map_err(|e| e.to_string())?, pending: Vec::new() })
    }
}

/// O lado da IDE: recebe a primeira mensagem e responde. Devolve o canal, o `hello` do app e
/// a resposta a mandar.
pub fn respond(private: &[u8; 32], secret: &[u8; 32], first: &[u8]) -> Result<(Session, String, Vec<u8>), String> {
    let mut hs = snow::Builder::new(params()).local_private_key(private).psk(0, secret).build_responder().map_err(|e| e.to_string())?;
    let mut hello = vec![0u8; first.len()];
    let n = hs.read_message(first, &mut hello).map_err(|_| "aperto de mão inválido".to_string())?;
    let hello = String::from_utf8_lossy(&hello[..n]).into_owned();
    let mut out = vec![0u8; 128];
    let m = hs.write_message(&[], &mut out).map_err(|e| e.to_string())?;
    out.truncate(m);
    Ok((Session { state: hs.into_transport_mode().map_err(|e| e.to_string())?, pending: Vec::new() }, hello, out))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(secret_app: [u8; 32], secret_ide: [u8; 32]) -> Result<(Session, Session), String> {
        let (private, public) = keypair()?;
        let target = Target { address: "127.0.0.1:7342".into(), ide_public: public, secret: secret_app };
        let (init, first) = initiate(&target, r#"{"project":"Loja","pid":1}"#)?;
        let (ide, hello, reply) = respond(&private, &secret_ide, &first)?;
        assert!(hello.contains("Loja"));
        Ok((init.finish(&reply)?, ide))
    }

    #[test]
    fn both_sides_talk_and_big_messages_are_split() {
        let s = secret().unwrap();
        let (mut app, mut ide) = pair(s, s).unwrap();
        let big = "x".repeat(200_000);
        let frames = ide.seal(&big).unwrap();
        assert!(frames.len() >= 4);
        let mut got = None;
        for f in &frames {
            got = app.open(f).unwrap().or(got);
        }
        assert_eq!(got.as_deref(), Some(big.as_str()));
        let frames = app.seal(r#"{"ok":1}"#).unwrap();
        assert_eq!(ide.open(&frames[0]).unwrap().as_deref(), Some(r#"{"ok":1}"#));
    }

    #[test]
    fn a_wrong_secret_or_impostor_fails() {
        // A IDE com outro segredo: o app recusa a resposta dela.
        assert!(pair(secret().unwrap(), secret().unwrap()).is_err());
        // Um impostor (outra chave privada) nem lê a primeira mensagem.
        let (_, public) = keypair().unwrap();
        let (other_private, _) = keypair().unwrap();
        let s = secret().unwrap();
        let target = Target { address: "x".into(), ide_public: public, secret: s };
        let (_, first) = initiate(&target, "{}").unwrap();
        assert!(respond(&other_private, &s, &first).is_err());
        // Um quadro adulterado é recusado.
        let (mut app, mut ide) = pair(s, s).unwrap();
        let mut frame = ide.seal("oi").unwrap().remove(0);
        frame[3] ^= 1;
        assert!(app.open(&frame).is_err());
    }

    #[test]
    fn target_round_trips_as_json() {
        let t = Target { address: "10.0.0.5:7342".into(), ide_public: [7; 32], secret: [9; 32] };
        assert_eq!(Target::from_json(&t.to_json()), Some(t));
    }
}

/// O lado do app fora do navegador: conecta na IDE e atende.
#[cfg(not(target_arch = "wasm32"))]
pub mod client {
    use std::net::TcpStream;
    use std::sync::mpsc::Receiver;
    use std::time::Duration;

    use tungstenite::stream::MaybeTlsStream;
    use tungstenite::{Message, WebSocket};

    use super::{Session, Target, initiate};

    /// Prazo de cada leitura: a thread alterna entre receber e mandar.
    const POLL: Duration = Duration::from_millis(50);
    /// Espera entre tentativas (a IDE fechada, outro projeto aberto nela).
    const RETRY: Duration = Duration::from_secs(3);

    /// Conecta e atende para sempre, numa thread própria: sem a IDE, tenta de novo em
    /// silêncio; quando a conexão cai, volta a tentar. `handle` roda um comando e devolve a
    /// resposta (o chamador leva para a thread da tela); `outgoing` são as mensagens que o app
    /// manda sozinho (o rastro de eventos). `hello`: quem é o app (projeto e processo).
    pub fn run(target: Target, hello: String, mut handle: impl FnMut(&str) -> Option<String> + Send + 'static, outgoing: Receiver<String>) {
        let _ = std::thread::Builder::new().name("aylis-dev".into()).spawn(move || {
            let mut announced = false;
            loop {
                match connect(&target, &hello) {
                    Ok((mut socket, mut session)) => {
                        if !announced {
                            println!("aylis: conectado na IDE ({})", target.address);
                            announced = true;
                        }
                        serve(&mut socket, &mut session, &mut handle, &outgoing);
                    }
                    Err(_) => std::thread::sleep(RETRY),
                }
                std::thread::sleep(Duration::from_millis(200));
            }
        });
    }

    type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

    fn connect(target: &Target, hello: &str) -> Result<(Socket, Session), String> {
        let (mut socket, _) = tungstenite::connect(target.url()).map_err(|e| e.to_string())?;
        if let MaybeTlsStream::Plain(s) = socket.get_ref() {
            s.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
        }
        let (init, first) = initiate(target, hello)?;
        socket.send(Message::binary(first)).map_err(|e| e.to_string())?;
        let reply = loop {
            match socket.read().map_err(|e| e.to_string())? {
                Message::Binary(b) => break b,
                Message::Close(_) => return Err("a IDE fechou a conexão".into()),
                _ => {}
            }
        };
        let session = init.finish(&reply)?;
        if let MaybeTlsStream::Plain(s) = socket.get_ref() {
            s.set_read_timeout(Some(POLL)).map_err(|e| e.to_string())?;
        }
        Ok((socket, session))
    }

    fn send(socket: &mut Socket, session: &mut Session, text: &str) -> bool {
        match session.seal(text) {
            Ok(frames) => frames.into_iter().all(|f| socket.send(Message::binary(f)).is_ok()),
            Err(_) => false,
        }
    }

    fn serve(socket: &mut Socket, session: &mut Session, handle: &mut impl FnMut(&str) -> Option<String>, outgoing: &Receiver<String>) {
        loop {
            while let Ok(text) = outgoing.try_recv() {
                if !send(socket, session, &text) {
                    return;
                }
            }
            match socket.read() {
                Ok(Message::Binary(frame)) => match session.open(&frame) {
                    Ok(Some(text)) => {
                        if let Some(reply) = handle(&text) {
                            if !send(socket, session, &reply) {
                                return;
                            }
                        }
                    }
                    Ok(None) => {}
                    // Quadro adulterado: fecha (e volta a tentar do zero).
                    Err(_) => return,
                },
                Ok(Message::Close(_)) => return,
                Ok(_) => {}
                Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                    let _ = socket.flush();
                }
                Err(_) => return,
            }
        }
    }
}
