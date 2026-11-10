//! O agente remoto (`aylis-tool debug-agent`): roda noutro PC ou servidor (Windows ou Linux) e
//! recebe o projeto, compila, executa e depura lá, pelo mesmo canal cifrado do canal de
//! desenvolvimento (Noise `NKpsk0` sobre WebSocket, `aylis-devlink`).
//!
//! O pareamento é pela chave: o agente gera o par dele e um segredo na primeira vez
//! (`agent.json`, na pasta de dados do aylis) e mostra o código `host:porta#pública.segredo`.
//! Quem não tem o código não completa o aperto de mão. Do lado de cá:
//! - `aylis-tool agent-run app.aylisproj --agent CÓDIGO`: manda o projeto, compila e executa,
//!   com a saída linha a linha;
//! - `aylis-tool agent-dap --agent CÓDIGO --project app.aylisproj -- adaptador...`: um adaptador
//!   de depuração (DAP) local que repassa tudo ao adaptador que roda no agente, trocando o
//!   caminho do projeto de uma máquina pelo da outra. O depurador da IDE ou do VS Code usa como
//!   se fosse local.
//!
//! Mensagens (JSON): `{"id", "cmd": "hello" | "put_zip" | "run" | "stop" | "dap_start" | "dap",
//! ...}`, com a resposta `{"id", "ok"}` ou `{"id", "error"}`; o agente manda sozinho
//! `{"event": "line", "stream", "text"}`, `{"event": "exit", "code"}` e
//! `{"event": "dap", "data"}` (os bytes do adaptador, em hexadecimal).

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

use aylis_devlink::{Session, Target, hex, initiate, respond, unhex32};
use serde_json::{Value, json};
use tungstenite::{Message, WebSocket};

/// Onde o agente escuta por padrão.
pub const DEFAULT_ADDRESS: &str = "0.0.0.0:7444";

/// O código de pareamento: onde o agente está, a chave pública dele e o segredo.
#[derive(Clone, Debug, PartialEq)]
pub struct Pairing {
    pub address: String,
    pub public: [u8; 32],
    pub secret: [u8; 32],
}

impl Pairing {
    /// `host:porta#pública.segredo` (em hexadecimal).
    pub fn code(&self) -> String {
        format!("{}#{}.{}", self.address, hex(&self.public), hex(&self.secret))
    }

    pub fn parse(code: &str) -> Result<Pairing, String> {
        let bad = || format!("código de pareamento inválido: {code}");
        let (address, keys) = code.trim().split_once('#').ok_or_else(bad)?;
        let (public, secret) = keys.split_once('.').ok_or_else(bad)?;
        Ok(Pairing { address: address.to_string(), public: unhex32(public).ok_or_else(bad)?, secret: unhex32(secret).ok_or_else(bad)? })
    }
}

/// As chaves do agente nesta máquina (privada, pública, segredo): geradas na primeira vez.
pub fn agent_keys() -> Result<([u8; 32], [u8; 32], [u8; 32]), String> {
    let path = crate::dap::tools_dir().and_then(|d| d.parent().map(Path::to_path_buf)).ok_or("sem pasta de dados do usuário")?.join("agent.json");
    if let Some(v) = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
        let get = |k: &str| v[k].as_str().and_then(unhex32);
        if let (Some(a), Some(b), Some(c)) = (get("private"), get("public"), get("secret")) {
            return Ok((a, b, c));
        }
    }
    let (private, public) = aylis_devlink::keypair()?;
    let secret = aylis_devlink::secret()?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = json!({ "private": hex(&private), "public": hex(&public), "secret": hex(&secret) }).to_string();
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok((private, public, secret))
}

/// Bytes de um texto hexadecimal (qualquer tamanho).
fn unhex(text: &str) -> Option<Vec<u8>> {
    (0..text.len()).step_by(2).map(|i| text.get(i..i + 2).and_then(|b| u8::from_str_radix(b, 16).ok())).collect()
}

type Socket<S> = WebSocket<S>;

fn send<S: Read + Write>(socket: &mut Socket<S>, session: &mut Session, value: &Value) -> bool {
    match session.seal(&value.to_string()) {
        Ok(frames) => frames.into_iter().all(|f| socket.send(Message::binary(f)).is_ok()),
        Err(_) => false,
    }
}

/// O que roda no agente para uma conexão: o programa e o adaptador de depuração.
#[derive(Default)]
struct Work {
    program: Option<Child>,
    adapter: Option<Child>,
}

impl Drop for Work {
    fn drop(&mut self) {
        for child in [self.program.as_mut(), self.adapter.as_mut()].into_iter().flatten() {
            let _ = child.kill();
        }
    }
}

/// Atende para sempre, uma conexão de cada vez, com o trabalho em `workdir`. `log` recebe o
/// que acontece (quem conectou, o que rodou).
pub fn serve(listener: TcpListener, private: [u8; 32], secret: [u8; 32], workdir: PathBuf, log: impl Fn(&str)) {
    for stream in listener.incoming().flatten() {
        let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
        match connection(stream, &private, &secret, &workdir, &log) {
            Ok(()) => log(&format!("{peer}: desconectou")),
            Err(e) => log(&format!("{peer}: {e}")),
        }
    }
}

fn connection(stream: TcpStream, private: &[u8; 32], secret: &[u8; 32], workdir: &Path, log: &impl Fn(&str)) -> Result<(), String> {
    stream.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
    let mut socket = tungstenite::accept(stream.try_clone().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let first = loop {
        match socket.read().map_err(|e| e.to_string())? {
            Message::Binary(b) => break b,
            Message::Close(_) => return Ok(()),
            _ => {}
        }
    };
    // Sem o segredo certo, o aperto de mão falha e a conexão cai aqui.
    let (mut session, hello, reply) = respond(private, secret, &first).map_err(|_| "pareamento recusado".to_string())?;
    socket.send(Message::binary(reply)).map_err(|e| e.to_string())?;
    log(&format!("conectado: {hello}"));
    // No fluxo do próprio WebSocket: o clone do Windows tem as opções dele.
    socket.get_ref().set_read_timeout(Some(Duration::from_millis(30))).map_err(|e| e.to_string())?;
    let (events_tx, events_rx) = channel::<Value>();
    let mut work = Work::default();
    loop {
        while let Ok(event) = events_rx.try_recv() {
            if !send(&mut socket, &mut session, &event) {
                return Ok(());
            }
        }
        // O programa terminou: avisa.
        if let Some(code) = work.program.as_mut().and_then(|c| c.try_wait().ok().flatten()) {
            work.program = None;
            // As últimas linhas antes do fim.
            std::thread::sleep(Duration::from_millis(50));
            while let Ok(event) = events_rx.try_recv() {
                send(&mut socket, &mut session, &event);
            }
            send(&mut socket, &mut session, &json!({ "event": "exit", "code": code.code() }));
        }
        let text = match socket.read() {
            Ok(Message::Binary(frame)) => match session.open(&frame) {
                Ok(Some(text)) => text,
                Ok(None) => continue,
                Err(_) => return Err("quadro adulterado".into()),
            },
            Ok(Message::Close(_)) => return Ok(()),
            Ok(_) => continue,
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => continue,
            Err(_) => return Ok(()),
        };
        let Ok(command) = serde_json::from_str::<Value>(&text) else { continue };
        let id = command["id"].clone();
        let reply = match handle(&command, workdir, &mut work, &events_tx, log) {
            Ok(Some(ok)) => json!({ "id": id, "ok": ok }),
            Ok(None) => continue,
            Err(e) => json!({ "id": id, "error": e }),
        };
        if !send(&mut socket, &mut session, &reply) {
            return Ok(());
        }
    }
}

/// A pasta de um projeto no agente (só o nome: nada de sair da pasta de trabalho).
fn project_dir(workdir: &Path, name: &str) -> Result<PathBuf, String> {
    let clean: String = name.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-').collect();
    if clean.is_empty() {
        return Err("nome de projeto inválido".into());
    }
    Ok(workdir.join(clean))
}

/// Lê as linhas de uma saída e manda como eventos.
fn forward_lines(reader: impl Read + Send + 'static, stream: &'static str, tx: Sender<Value>) {
    std::thread::spawn(move || {
        let reader = std::io::BufReader::new(reader);
        for line in std::io::BufRead::lines(reader).map_while(Result::ok) {
            if tx.send(json!({ "event": "line", "stream": stream, "text": line })).is_err() {
                break;
            }
        }
    });
}

fn handle(command: &Value, workdir: &Path, work: &mut Work, events: &Sender<Value>, log: &impl Fn(&str)) -> Result<Option<Value>, String> {
    let text = |k: &str| command[k].as_str().unwrap_or_default().to_string();
    let args: Vec<String> = command["args"].as_array().into_iter().flatten().filter_map(|a| a.as_str().map(str::to_string)).collect();
    match command["cmd"].as_str().unwrap_or_default() {
        "hello" => Ok(Some(json!({ "os": std::env::consts::OS, "arch": std::env::consts::ARCH, "dir": workdir.display().to_string() }))),
        "put_zip" => {
            let dir = project_dir(workdir, &text("project"))?;
            let bytes = unhex(&text("data")).ok_or("dados inválidos")?;
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| e.to_string())?.extract(&dir).map_err(|e| e.to_string())?;
            log(&format!("projeto recebido em {}", dir.display()));
            Ok(Some(json!(dir.display().to_string())))
        }
        "run" => {
            if let Some(mut old) = work.program.take() {
                let _ = old.kill();
            }
            let dir = project_dir(workdir, &text("project"))?.join(text("cwd"));
            let mut child = Command::new(text("program"))
                .args(&args)
                .current_dir(&dir)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("{}: {e}", text("program")))?;
            forward_lines(child.stdout.take().expect("piped"), "out", events.clone());
            forward_lines(child.stderr.take().expect("piped"), "err", events.clone());
            log(&format!("executando {} {}", text("program"), args.join(" ")));
            work.program = Some(child);
            Ok(Some(Value::Null))
        }
        "stop" => {
            if let Some(mut child) = work.program.take() {
                let _ = child.kill();
            }
            Ok(Some(Value::Null))
        }
        "dap_start" => {
            let dir = project_dir(workdir, &text("project"))?;
            let mut child = Command::new(text("program"))
                .args(&args)
                .current_dir(&dir)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| format!("{}: {e}", text("program")))?;
            let mut out = child.stdout.take().expect("piped");
            let tx = events.clone();
            std::thread::spawn(move || {
                let mut buffer = [0u8; 16 * 1024];
                while let Ok(n) = out.read(&mut buffer) {
                    if n == 0 || tx.send(json!({ "event": "dap", "data": hex(&buffer[..n]) })).is_err() {
                        break;
                    }
                }
            });
            work.adapter = Some(child);
            Ok(Some(json!({ "root": dir.display().to_string() })))
        }
        "dap" => {
            let bytes = unhex(&text("data")).ok_or("dados inválidos")?;
            let stdin = work.adapter.as_mut().and_then(|c| c.stdin.as_mut()).ok_or("o adaptador não está rodando")?;
            stdin.write_all(&bytes).and_then(|_| stdin.flush()).map_err(|e| e.to_string())?;
            // Sem resposta: o adaptador responde pelos eventos.
            Ok(None)
        }
        other => Err(format!("comando desconhecido: {other}")),
    }
}

/// Do lado de cá: a conexão com um agente.
pub struct AgentClient {
    socket: WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>,
    session: Session,
    next: u64,
    /// Os eventos que chegaram enquanto esperava uma resposta.
    pending: Vec<Value>,
}

impl AgentClient {
    pub fn connect(pairing: &Pairing) -> Result<AgentClient, String> {
        let target = Target { address: pairing.address.clone(), ide_public: pairing.public, secret: pairing.secret };
        let (mut socket, _) = tungstenite::connect(target.url()).map_err(|e| format!("{}: {e}", pairing.address))?;
        let hello = json!({ "client": "aylis-tool", "host": std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")).unwrap_or_default() }).to_string();
        let (init, first) = initiate(&target, &hello)?;
        socket.send(Message::binary(first)).map_err(|e| e.to_string())?;
        let reply = loop {
            match socket.read().map_err(|_| "o agente recusou o pareamento (confira o código)".to_string())? {
                Message::Binary(b) => break b,
                Message::Close(_) => return Err("o agente recusou o pareamento (confira o código)".into()),
                _ => {}
            }
        };
        let session = init.finish(&reply)?;
        if let tungstenite::stream::MaybeTlsStream::Plain(s) = socket.get_ref() {
            s.set_read_timeout(Some(Duration::from_millis(30))).map_err(|e| e.to_string())?;
        }
        Ok(AgentClient { socket, session, next: 0, pending: Vec::new() })
    }

    /// Manda sem esperar resposta (o `dap`).
    pub fn post(&mut self, mut command: Value) -> Result<(), String> {
        self.next += 1;
        command["id"] = self.next.into();
        if send(&mut self.socket, &mut self.session, &command) { Ok(()) } else { Err("a conexão com o agente caiu".into()) }
    }

    /// Manda e espera a resposta (até `timeout`); os eventos no meio ficam para [`next_event`](Self::next_event).
    pub fn call(&mut self, mut command: Value, timeout: Duration) -> Result<Value, String> {
        self.next += 1;
        let id = self.next;
        command["id"] = id.into();
        if !send(&mut self.socket, &mut self.session, &command) {
            return Err("a conexão com o agente caiu".into());
        }
        let deadline = std::time::Instant::now() + timeout;
        while std::time::Instant::now() < deadline {
            let Some(message) = self.read()? else { continue };
            if message["id"].as_u64() == Some(id) {
                return match message.get("error").and_then(Value::as_str) {
                    Some(e) => Err(e.to_string()),
                    None => Ok(message["ok"].clone()),
                };
            }
            self.pending.push(message);
        }
        Err("o agente não respondeu".into())
    }

    /// O próximo evento do agente (`None`: nada chegou agora).
    pub fn next_event(&mut self) -> Result<Option<Value>, String> {
        if !self.pending.is_empty() {
            return Ok(Some(self.pending.remove(0)));
        }
        self.read()
    }

    fn read(&mut self) -> Result<Option<Value>, String> {
        match self.socket.read() {
            Ok(Message::Binary(frame)) => match self.session.open(&frame)? {
                Some(text) => Ok(serde_json::from_str(&text).ok()),
                None => Ok(None),
            },
            Ok(Message::Close(_)) => Err("o agente fechou a conexão".into()),
            Ok(_) => Ok(None),
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}

/// Um ZIP com os arquivos do projeto (sem `target`, `.git` e as saídas).
pub fn project_zip(dir: &Path) -> Result<Vec<u8>, String> {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    fn walk(root: &Path, dir: &Path, zip: &mut zip::ZipWriter<std::io::Cursor<Vec<u8>>>, options: zip::write::SimpleFileOptions) -> Result<(), String> {
        for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if matches!(name.as_str(), "target" | ".git" | "dist" | ".aylis" | "node_modules" | "bin" | "obj") {
                continue;
            }
            let rel = path.strip_prefix(root).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
            if path.is_dir() {
                walk(root, &path, zip, options)?;
            } else {
                zip.start_file(rel, options).map_err(|e| e.to_string())?;
                zip.write_all(&std::fs::read(&path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
    walk(dir, dir, &mut zip, options)?;
    Ok(zip.finish().map_err(|e| e.to_string())?.into_inner())
}

/// Manda o projeto e devolve a pasta dele no agente.
pub fn upload(client: &mut AgentClient, name: &str, dir: &Path) -> Result<String, String> {
    let data = project_zip(dir)?;
    let reply = client.call(json!({ "cmd": "put_zip", "project": name, "data": hex(&data) }), Duration::from_secs(120))?;
    Ok(reply.as_str().unwrap_or_default().to_string())
}

/// Troca um caminho por outro dentro do JSON de uma mensagem DAP (os dois jeitos de escrever
/// a barra) e refaz o cabeçalho.
fn remap_dap(body: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return body.to_string();
    }
    let escaped = |p: &str| p.replace('\\', "\\\\");
    let to_json = escaped(to);
    body.replace(&escaped(from), &to_json).replace(&from.replace('\\', "/"), &to.replace('\\', "/"))
}

/// Separa as mensagens DAP completas de `buffer` (`Content-Length: n\r\n\r\n` + corpo).
fn take_dap_messages(buffer: &mut Vec<u8>) -> Vec<String> {
    let mut out = Vec::new();
    loop {
        let Some(end) = buffer.windows(4).position(|w| w == b"\r\n\r\n") else { break };
        let header = String::from_utf8_lossy(&buffer[..end]).to_string();
        let Some(len) = header.lines().find_map(|l| l.strip_prefix("Content-Length:")).and_then(|v| v.trim().parse::<usize>().ok()) else {
            buffer.drain(..end + 4);
            continue;
        };
        if buffer.len() < end + 4 + len {
            break;
        }
        let body = String::from_utf8_lossy(&buffer[end + 4..end + 4 + len]).to_string();
        buffer.drain(..end + 4 + len);
        out.push(body);
    }
    out
}

fn dap_frame(body: &str) -> Vec<u8> {
    let mut out = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    out.extend_from_slice(body.as_bytes());
    out
}

/// O adaptador DAP local que repassa ao agente (`aylis-tool agent-dap`): lê `input` (o stdin, o
/// que o depurador manda), escreve em `output`. `local` e `remote`: a pasta do projeto aqui e
/// lá, trocadas nas mensagens.
pub fn relay_dap(client: &mut AgentClient, local: &str, remote: &str, mut input: impl Read + Send + 'static, output: &mut impl Write) -> Result<(), String> {
    let (tx, rx): (Sender<Option<Vec<u8>>>, Receiver<Option<Vec<u8>>>) = channel();
    std::thread::spawn(move || {
        let mut buffer = [0u8; 16 * 1024];
        loop {
            match input.read(&mut buffer) {
                Ok(0) | Err(_) => {
                    let _ = tx.send(None);
                    break;
                }
                Ok(n) => {
                    if tx.send(Some(buffer[..n].to_vec())).is_err() {
                        break;
                    }
                }
            }
        }
    });
    let (mut up, mut down) = (Vec::new(), Vec::new());
    // A entrada fechou (o depurador saiu): ainda espera um pouco o que vem de lá.
    let mut closing: Option<std::time::Instant> = None;
    loop {
        if closing.is_some_and(|t| t.elapsed() > Duration::from_secs(2)) {
            return Ok(());
        }
        // Daqui para lá: as mensagens com o caminho daqui trocado pelo de lá.
        while let Ok(chunk) = rx.try_recv() {
            let Some(chunk) = chunk else {
                closing = Some(std::time::Instant::now());
                break;
            };
            up.extend_from_slice(&chunk);
            for body in take_dap_messages(&mut up) {
                client.post(json!({ "cmd": "dap", "data": hex(&dap_frame(&remap_dap(&body, local, remote))) }))?;
            }
        }
        // De lá para cá.
        match client.next_event()? {
            Some(event) if event["event"] == "dap" => {
                down.extend_from_slice(&unhex(event["data"].as_str().unwrap_or_default()).unwrap_or_default());
                for body in take_dap_messages(&mut down) {
                    output.write_all(&dap_frame(&remap_dap(&body, remote, local))).map_err(|e| e.to_string())?;
                    output.flush().map_err(|e| e.to_string())?;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent() -> (Pairing, PathBuf) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let (private, public) = aylis_devlink::keypair().unwrap();
        let secret = aylis_devlink::secret().unwrap();
        let workdir = std::env::temp_dir().join(format!("aylis_agent_{}_{}", std::process::id(), address.replace([':', '.'], "_")));
        let dir = workdir.clone();
        std::thread::spawn(move || serve(listener, private, secret, dir, |_| {}));
        (Pairing { address, public, secret }, workdir)
    }

    #[test]
    fn the_pairing_code_goes_and_comes_back() {
        let p = Pairing { address: "10.0.0.5:7444".into(), public: [7; 32], secret: [9; 32] };
        assert_eq!(Pairing::parse(&p.code()).unwrap(), p);
        assert!(Pairing::parse("10.0.0.5:7444#abc").is_err());
    }

    #[test]
    fn a_wrong_secret_is_refused() {
        let (mut pairing, _) = agent();
        pairing.secret = [1; 32];
        assert!(AgentClient::connect(&pairing).is_err());
    }

    #[test]
    fn the_agent_receives_the_project_runs_it_and_relays_the_debugger() {
        let (pairing, workdir) = agent();
        let mut client = AgentClient::connect(&pairing).unwrap();
        assert_eq!(client.call(json!({ "cmd": "hello" }), Duration::from_secs(5)).unwrap()["os"], std::env::consts::OS);
        // O projeto: um arquivo, enviado em ZIP.
        let local = std::env::temp_dir().join(format!("aylis_agent_src_{}", std::process::id()));
        std::fs::create_dir_all(local.join("target")).unwrap();
        std::fs::write(local.join("ola.txt"), "olá do agente\n").unwrap();
        std::fs::write(local.join("target/lixo"), "fica de fora").unwrap();
        let remote = upload(&mut client, "Teste", &local).unwrap();
        assert!(Path::new(&remote).join("ola.txt").is_file() && !Path::new(&remote).join("target").exists());
        // Executar: a saída chega linha a linha, e o fim.
        let (program, args) = if cfg!(windows) { ("cmd", vec!["/C", "type", "ola.txt"]) } else { ("cat", vec!["ola.txt"]) };
        client.call(json!({ "cmd": "run", "project": "Teste", "program": program, "args": args, "cwd": "" }), Duration::from_secs(5)).unwrap();
        let (mut lines, mut exit) = (Vec::new(), None);
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while exit.is_none() && std::time::Instant::now() < deadline {
            if let Some(e) = client.next_event().unwrap() {
                match e["event"].as_str() {
                    Some("line") => lines.push(e["text"].as_str().unwrap_or_default().to_string()),
                    Some("exit") => exit = Some(e["code"].clone()),
                    _ => {}
                }
            }
        }
        assert_eq!(exit, Some(json!(0)), "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("olá do agente")), "{lines:?}");
        // O adaptador de depuração: um eco dos bytes (no Windows, um PowerShell de poucas linhas;
        // no Linux, o cat), com o caminho trocado na ida e na volta.
        let echo = "$i=[Console]::OpenStandardInput();$o=[Console]::OpenStandardOutput();$b=New-Object byte[] 4096;while(($n=$i.Read($b,0,4096)) -gt 0){$o.Write($b,0,$n);$o.Flush()}";
        let (program, args) = if cfg!(windows) { ("powershell", vec!["-NoProfile", "-Command", echo]) } else { ("cat", vec![]) };
        let root = client.call(json!({ "cmd": "dap_start", "project": "Teste", "program": program, "args": args }), Duration::from_secs(5)).unwrap();
        let root = root["root"].as_str().unwrap().to_string();
        let here = local.display().to_string();
        let body = json!({ "seq": 1, "type": "request", "command": "setBreakpoints", "arguments": { "source": { "path": format!("{here}{}main.rs", std::path::MAIN_SEPARATOR) } } }).to_string();
        // O que vai para lá leva o caminho de lá; o eco volta e o repasse troca de novo.
        let input = std::io::Cursor::new(dap_frame(&body));
        let mut output = Vec::new();
        relay_dap(&mut client, &here, &root, input, &mut output).unwrap();
        let text = String::from_utf8_lossy(&output).to_string();
        let _ = std::fs::remove_dir_all(&local);
        let _ = std::fs::remove_dir_all(&workdir);
        // O que voltou tem o caminho daqui de novo (foi trocado pelo de lá na ida).
        assert!(text.contains("setBreakpoints") && text.contains(&here.replace('\\', "\\\\")), "{text}");
    }
}
