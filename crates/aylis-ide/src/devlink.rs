//! O canal de desenvolvimento do lado da IDE: um servidor WebSocket em que o app em execução
//! (compilado com a feature `dev`, no desktop, no celular ou no navegador) conecta, no
//! endereço embutido nele. Cada conexão passa pelo aperto de mão do Noise (`aylis-devlink`):
//! a chave privada da IDE e o segredo do projeto aberto; quem não tem os dois não fala com a
//! IDE, e a IDE não obedece a ninguém (é ela que manda). Por ele vão os comandos do Inspetor
//! do app, o hot reload (a IDE manda o form, o tema ou as traduções ao salvar) e o rastro de
//! eventos. Vários apps ao mesmo tempo: os comandos vão para o último que conectou, o hot
//! reload para todos.

use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_devlink::Session;
use serde_json::{Value, json};
use tungstenite::{Message, WebSocket};

/// Um app conectado.
struct Conn {
    ws: WebSocket<TcpStream>,
    session: Session,
    /// Quem é (`{"project": ..., "pid": ...}`).
    hello: Value,
}

/// O servidor (a thread aceita conexões; [`DevServer::call`] conversa com o app).
pub struct DevServer {
    /// O endereço pedido (`host:porta`, o do projeto).
    address: String,
    port: u16,
    conns: Arc<Mutex<Vec<Conn>>>,
    next_id: AtomicU64,
    /// O segredo do projeto aberto (trocar de projeto troca).
    secret: Arc<Mutex<[u8; 32]>>,
    /// O que os apps mandaram sem pedido: o rastro de eventos e os handlers que rodaram.
    traces: Arc<Inbox>,
    /// O rastro de eventos ligado (vale para quem conectar depois também).
    trace: Arc<AtomicBool>,
}

/// O servidor do projeto aberto: reaproveita o que já roda no mesmo endereço (trocando o
/// segredo, se o projeto mudou) ou abre um novo.
pub fn for_project(existing: Option<Arc<DevServer>>, project: &aylis_project::Project) -> Result<Arc<DevServer>, String> {
    let address = aylis_tools::devkeys::address(project);
    let secret = aylis_tools::devkeys::project_secret(project)?;
    if let Some(server) = existing.filter(|s| s.address == address) {
        server.set_secret(secret);
        return Ok(server);
    }
    let (private, _) = aylis_tools::devkeys::ide_keys()?;
    DevServer::start(&address, private, secret)
}

impl DevServer {
    /// Abre em `address` (`127.0.0.1:7342`; um IP da rede ouve em todas as interfaces) com
    /// a chave privada da IDE e o segredo do projeto.
    pub fn start(address: &str, private: [u8; 32], secret: [u8; 32]) -> Result<Arc<DevServer>, String> {
        let (host, port) = address.rsplit_once(':').ok_or_else(|| format!("endereço inválido: {address}"))?;
        let port: u16 = port.parse().map_err(|_| format!("porta inválida: {address}"))?;
        let local = matches!(host, "127.0.0.1" | "localhost" | "[::1]");
        let bind = if local { "127.0.0.1" } else { "0.0.0.0" };
        // A porta ocupada (outra IDE com outro projeto): a próxima livre, que o Executar avisa
        // ao app (`AYLIS_DEV_PORT`; no Android, pelo `adb reverse`). Antes a segunda IDE não
        // executava (o relatório do Tabernáculo, I04). Só no endereço local: o da rede vai
        // embutido num celular sem cabo, que não teria como saber outra porta.
        let tries = if local { 50 } else { 1 };
        let listener = (0..tries)
            .find_map(|i| TcpListener::bind((bind, port.saturating_add(i))).ok())
            .ok_or_else(|| format!("canal de desenvolvimento: as portas {port} a {} estão ocupadas", port.saturating_add(tries - 1)))?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        let server = Arc::new(DevServer {
            address: address.to_string(),
            port,
            conns: Arc::new(Mutex::new(Vec::new())),
            next_id: AtomicU64::new(1),
            secret: Arc::new(Mutex::new(secret)),
            traces: Arc::new(Inbox::default()),
            trace: Arc::new(AtomicBool::new(false)),
        });
        let (conns, secret, trace) = (server.conns.clone(), server.secret.clone(), server.trace.clone());
        std::thread::Builder::new()
            .name("aylis-ide-dev".into())
            .spawn(move || {
                for stream in listener.incoming().flatten() {
                    let (conns, secret, trace) = (conns.clone(), secret.clone(), trace.clone());
                    // O aperto de mão numa thread: um cliente lento não segura os outros.
                    std::thread::spawn(move || {
                        let secret = secret.lock().map(|s| *s).unwrap_or([0; 32]);
                        if let Ok(mut conn) = handshake(stream, &private, &secret) {
                            if trace.load(Ordering::Relaxed) {
                                let _ = send(&mut conn, &json!({ "cmd": "trace", "on": true }));
                            }
                            if let Ok(mut c) = conns.lock() {
                                c.push(conn);
                            }
                        }
                    });
                }
            })
            .map_err(|e| e.to_string())?;
        // O rastro chega sem pedido: uma thread recolhe o que os apps mandaram.
        let weak = Arc::downgrade(&server);
        std::thread::Builder::new()
            .name("aylis-ide-dev-trace".into())
            .spawn(move || {
                while let Some(server) = weak.upgrade() {
                    server.drain();
                    drop(server);
                    std::thread::sleep(Duration::from_millis(150));
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(server)
    }

    /// O endereço do projeto em que o servidor abriu.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Liga ou desliga o rastro de eventos nos apps conectados e nos que conectarem.
    pub fn set_trace(&self, on: bool) {
        self.trace.store(on, Ordering::Relaxed);
        if let Ok(mut conns) = self.conns.lock() {
            conns.retain_mut(|c| send(c, &json!({ "cmd": "trace", "on": on })).is_ok());
        }
    }

    /// Lê o que chegou sem esperar (as linhas do rastro); tira quem desconectou.
    fn drain(&self) {
        let Ok(mut conns) = self.conns.lock() else { return };
        conns.retain_mut(|conn| {
            let _ = conn.ws.get_ref().set_read_timeout(Some(Duration::from_millis(1)));
            let alive = loop {
                match conn.ws.read() {
                    Ok(Message::Binary(frame)) => {
                        let Ok(Some(text)) = conn.session.open(&frame) else { continue };
                        if let Ok(message) = serde_json::from_str::<Value>(&text) {
                            self.traces.accept(&message);
                        }
                    }
                    Ok(Message::Close(_)) => break false,
                    Ok(_) => {}
                    Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => break true,
                    Err(_) => break false,
                }
            };
            let _ = conn.ws.get_ref().set_read_timeout(Some(Duration::from_secs(5)));
            alive
        });
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// O endereço em que o app conecta (para a barra de status).
    pub fn url(&self) -> String {
        let host = self.address.rsplit_once(':').map_or("127.0.0.1", |(h, _)| h);
        format!("ws://{host}:{}", self.port)
    }

    /// Troca o segredo (outro projeto aberto).
    pub fn set_secret(&self, secret: [u8; 32]) {
        if let Ok(mut s) = self.secret.lock() {
            *s = secret;
        }
    }

    /// Algum app está conectado.
    pub fn connected(&self) -> bool {
        self.conns.lock().is_ok_and(|c| !c.is_empty())
    }

    /// Quem está conectado (`projeto (processo)`), do mais antigo ao mais novo.
    pub fn apps(&self) -> Vec<String> {
        self.conns
            .lock()
            .map(|c| c.iter().map(|x| format!("{} ({})", x.hello["project"].as_str().unwrap_or("?"), x.hello["pid"])).collect())
            .unwrap_or_default()
    }

    /// Espera um app conectar (até `timeout`).
    pub fn wait(&self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if self.connected() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(30));
        }
        self.connected()
    }

    /// As linhas do rastro que chegaram desde a última vez.
    pub fn take_traces(&self) -> Vec<String> {
        self.traces.lines.lock().map(|mut t| std::mem::take(&mut *t)).unwrap_or_default()
    }

    /// Os handlers que os apps avisaram ter rodado desde a última vez (`frmX.btnSalvar_click`;
    /// a cobertura de handlers).
    pub fn take_handlers(&self) -> Vec<String> {
        self.traces.handlers.lock().map(|mut t| std::mem::take(&mut *t)).unwrap_or_default()
    }

    /// Os apps conectados, do mais antigo ao mais novo: `{"project", "pid", "latest"}` (o
    /// `app_command` escolhe por eles; M21 do Tabernáculo).
    pub fn apps_json(&self) -> Value {
        let hellos: Vec<Value> = self.conns.lock().map(|c| c.iter().map(|x| x.hello.clone()).collect()).unwrap_or_default();
        apps_of(&hellos)
    }

    /// Manda um comando ao app mais recente e espera a resposta dele (`ok` ou `error`).
    pub fn call(&self, command: Value) -> Result<Value, String> {
        self.call_app(None, command)
    }

    /// Manda um comando ao app `app` (o pid ou o nome do projeto; `None`: o mais recente) e
    /// espera a resposta.
    pub fn call_app(&self, app: Option<&Value>, command: Value) -> Result<Value, String> {
        let mut conns = self.conns.lock().map_err(|_| "canal ocupado")?;
        let hellos: Vec<Value> = conns.iter().map(|c| c.hello.clone()).collect();
        let index = pick_app(&hellos, app)?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let result = call_on(&mut conns[index], id, command, &self.traces);
        if result.as_ref().is_err_and(|e| e.starts_with("conexão perdida") || e.starts_with("o app fechou")) {
            conns.remove(index);
        }
        result
    }

    /// Hot reload: manda o texto novo de um form (`kind = "form"`), do tema ou das
    /// traduções a todos os apps conectados.
    pub fn reload(&self, kind: &str, name: &str, text: &str) -> Result<String, String> {
        let mut conns = self.conns.lock().map_err(|_| "canal ocupado")?;
        if conns.is_empty() {
            return Err("o app não está conectado".into());
        }
        let mut done = Vec::new();
        let mut lost = Vec::new();
        for (i, conn) in conns.iter_mut().enumerate() {
            let id = self.next_id.fetch_add(1, Ordering::Relaxed);
            match call_on(conn, id, json!({ "cmd": "reload", "kind": kind, "name": name, "text": text }), &self.traces) {
                Ok(v) => done.push(v.as_str().unwrap_or_default().to_string()),
                Err(e) if e.starts_with("conexão perdida") || e.starts_with("o app fechou") => lost.push(i),
                Err(e) => return Err(e),
            }
        }
        for i in lost.into_iter().rev() {
            conns.remove(i);
        }
        Ok(done.into_iter().next().unwrap_or_default())
    }
}

/// A lista dos apps (o `hello` de cada um), com o mais recente marcado.
pub fn apps_of(hellos: &[Value]) -> Value {
    let last = hellos.len().saturating_sub(1);
    Value::Array(hellos.iter().enumerate().map(|(i, h)| json!({ "project": h["project"], "pid": h["pid"], "latest": i == last })).collect())
}

/// Qual app: o pid (número, ou texto com o número) ou o nome do projeto (sem diferença de
/// maiúsculas); sem escolha, o mais recente. Não achou, ou o nome vale para dois: o erro diz
/// quais estão conectados.
pub fn pick_app(hellos: &[Value], app: Option<&Value>) -> Result<usize, String> {
    let last = hellos.len().checked_sub(1).ok_or("o app não está conectado")?;
    let listed = || hellos.iter().map(|h| format!("{} (pid {})", h["project"].as_str().unwrap_or("?"), h["pid"])).collect::<Vec<_>>().join(", ");
    let wanted = match app {
        None | Some(Value::Null) => return Ok(last),
        Some(Value::String(s)) if s.trim().is_empty() => return Ok(last),
        Some(v) => v,
    };
    let pid = wanted.as_u64().or_else(|| wanted.as_str().and_then(|s| s.trim().parse().ok()));
    let found: Vec<usize> = match pid {
        Some(pid) => hellos.iter().enumerate().filter(|(_, h)| h["pid"].as_u64() == Some(pid)).map(|(i, _)| i).collect(),
        None => {
            let name = wanted.as_str().unwrap_or_default().trim();
            hellos.iter().enumerate().filter(|(_, h)| h["project"].as_str().is_some_and(|p| p.eq_ignore_ascii_case(name))).map(|(i, _)| i).collect()
        }
    };
    match found.as_slice() {
        [one] => Ok(*one),
        [] => Err(format!("nenhum app conectado é {wanted} (conectados: {})", listed())),
        _ => Err(format!("mais de um app conectado é {wanted}: escolha pelo pid ({})", listed())),
    }
}

/// O aperto de mão do lado da IDE: a primeira mensagem do app, a resposta.
fn handshake(stream: TcpStream, private: &[u8; 32], secret: &[u8; 32]) -> Result<Conn, String> {
    stream.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
    let mut ws = tungstenite::accept(stream).map_err(|e| e.to_string())?;
    let first = loop {
        match ws.read().map_err(|e| e.to_string())? {
            Message::Binary(b) => break b,
            Message::Close(_) => return Err("fechou".into()),
            _ => {}
        }
    };
    let (session, hello, reply) = aylis_devlink::respond(private, secret, &first)?;
    ws.send(Message::binary(reply)).map_err(|e| e.to_string())?;
    ws.get_ref().set_read_timeout(Some(Duration::from_secs(5))).map_err(|e| e.to_string())?;
    Ok(Conn { ws, session, hello: serde_json::from_str(&hello).unwrap_or(Value::Null) })
}

/// Manda um comando sem esperar a resposta.
fn send(conn: &mut Conn, command: &Value) -> Result<(), String> {
    for frame in conn.session.seal(&command.to_string())? {
        conn.ws.send(Message::binary(frame)).map_err(|e| format!("conexão perdida: {e}"))?;
    }
    Ok(())
}

/// O que os apps mandam sem pedido.
#[derive(Default)]
struct Inbox {
    /// As linhas do rastro de eventos (`{"trace": ".."}`).
    lines: Mutex<Vec<String>>,
    /// Os handlers que rodaram pela primeira vez (`{"handler": "frmX.h"}`).
    handlers: Mutex<Vec<String>>,
}

impl Inbox {
    /// Guarda a mensagem, se for uma das que chegam sem pedido; `false`: é resposta.
    fn accept(&self, message: &Value) -> bool {
        let (list, text) = if let Some(line) = message.get("trace").and_then(Value::as_str) {
            (&self.lines, line)
        } else if let Some(handler) = message.get("handler").and_then(Value::as_str) {
            (&self.handlers, handler)
        } else {
            return false;
        };
        if let Ok(mut l) = list.lock() {
            l.push(text.to_string());
        }
        true
    }
}

fn call_on(conn: &mut Conn, id: u64, mut command: Value, traces: &Inbox) -> Result<Value, String> {
    command["id"] = json!(id);
    send(conn, &command)?;
    loop {
        match conn.ws.read() {
            Ok(Message::Binary(frame)) => {
                let Some(text) = conn.session.open(&frame).map_err(|e| format!("conexão perdida: {e}"))? else { continue };
                let Ok(answer) = serde_json::from_str::<Value>(&text) else { continue };
                // O rastro e os handlers que rodaram chegam sem pedido.
                if traces.accept(&answer) {
                    continue;
                }
                if answer.get("id").and_then(Value::as_u64) != Some(id) {
                    continue; // resposta atrasada de um pedido anterior
                }
                return match answer.get("error").and_then(Value::as_str) {
                    Some(e) => Err(e.to_string()),
                    None => Ok(answer.get("ok").cloned().unwrap_or(Value::Null)),
                };
            }
            Ok(Message::Close(_)) => return Err("o app fechou a conexão".into()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(e)) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                return Err("o app não respondeu".into());
            }
            Err(e) => return Err(format!("conexão perdida: {e}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    /// M21 do Tabernáculo: com dois apps conectados, o `app_command` escolhe pelo pid ou pelo
    /// projeto; sem escolha, o último que conectou.
    #[test]
    fn the_app_is_chosen_by_pid_or_project() {
        let hellos = [json!({ "project": "frmPrincipal", "pid": 10 }), json!({ "project": "frmConsole", "pid": 20 })];
        assert_eq!(super::pick_app(&hellos, None), Ok(1));
        assert_eq!(super::pick_app(&hellos, Some(&json!(10))), Ok(0));
        assert_eq!(super::pick_app(&hellos, Some(&json!("10"))), Ok(0));
        assert_eq!(super::pick_app(&hellos, Some(&json!("FRMCONSOLE"))), Ok(1));
        let e = super::pick_app(&hellos, Some(&json!("outro"))).unwrap_err();
        assert!(e.contains("frmPrincipal (pid 10)") && e.contains("frmConsole (pid 20)"), "{e}");
        assert!(super::pick_app(&[], None).is_err());
        let list = super::apps_of(&hellos);
        assert_eq!(list[1]["latest"], true);
        assert_eq!(list[0]["pid"], 10);
    }

    /// O rastro e os handlers que rodaram chegam sem pedido, cada um na sua fila; a resposta de
    /// um comando não entra em nenhuma.
    #[test]
    fn unsolicited_messages_go_to_their_lists() {
        let inbox = super::Inbox::default();
        assert!(inbox.accept(&json!({ "trace": "evento frm.btn.on_click → salvar" })));
        assert!(inbox.accept(&json!({ "handler": "frmPrincipal.btnSalvar_click" })));
        assert!(!inbox.accept(&json!({ "id": 3, "ok": null })));
        assert_eq!(*inbox.lines.lock().unwrap(), ["evento frm.btn.on_click → salvar"]);
        assert_eq!(*inbox.handlers.lock().unwrap(), ["frmPrincipal.btnSalvar_click"]);
    }
}
