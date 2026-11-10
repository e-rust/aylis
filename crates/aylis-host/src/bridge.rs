//! `aylis-host --bridge 0.0.0.0:9000 -- python src/main.py`: o servidor dos apps publicados
//! no celular e no navegador com o código em outra linguagem. Cada tela que conecta (por
//! WebSocket) ganha um backend próprio, iniciado com o comando dado e ligado a ela pelo
//! stdin/stdout de sempre — o backend não sabe que a tela está do outro lado da rede.
//!
//! Fica exposta na internet, então tem limites: conexões no total e por IP, prazo para o
//! handshake, tamanho de mensagem, fila e inatividade (veja `transport.rs`). Não tem senha
//! própria: um token embutido no app publicado seria público. O login e as permissões são do
//! backend, que deve tratar todo evento como vindo de qualquer um (veja `clients/README.md`).

use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, TcpListener};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use crate::transport::{self, Tls};

/// Os limites da ponte (`--max-connections`, `--max-per-ip`, `--allow-origin`).
#[derive(Clone, Debug)]
pub struct Limits {
    pub max_connections: usize,
    pub max_per_ip: usize,
    /// Origens aceitas (navegador); vazia, todas.
    pub origins: Vec<String>,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_connections: 200, max_per_ip: 50, origins: Vec::new() }
    }
}

/// As conexões abertas, por IP.
#[derive(Debug, Default)]
struct Open {
    total: usize,
    per_ip: HashMap<IpAddr, usize>,
}

/// Uma vaga ocupada; devolve ao sair (inclusive num erro).
#[derive(Debug)]
struct Slot {
    open: Arc<Mutex<Open>>,
    ip: IpAddr,
}

impl Drop for Slot {
    fn drop(&mut self) {
        if let Ok(mut open) = self.open.lock() {
            open.total = open.total.saturating_sub(1);
            if let Some(n) = open.per_ip.get_mut(&self.ip) {
                *n -= 1;
                if *n == 0 {
                    open.per_ip.remove(&self.ip);
                }
            }
        }
    }
}

/// Ocupa uma vaga para `ip`, se houver.
fn take_slot(open: &Arc<Mutex<Open>>, ip: IpAddr, limits: &Limits) -> Result<Slot, String> {
    let mut o = open.lock().map_err(|_| "estado da ponte")?;
    if o.total >= limits.max_connections {
        return Err(format!("limite de {} conexões atingido", limits.max_connections));
    }
    let n = o.per_ip.entry(ip).or_default();
    if *n >= limits.max_per_ip {
        return Err(format!("limite de {} conexões por IP atingido", limits.max_per_ip));
    }
    *n += 1;
    o.total += 1;
    Ok(Slot { open: open.clone(), ip })
}

/// Com `--sessions`: um backend só para todas as telas. Cada mensagem que vai a ele leva o
/// número da tela (`"session": 17`); as que voltam com o número vão só para ela. A tela que
/// conecta chega como `{"session": n, "event": "session_open"}` e a que sai como
/// `session_close`. Se o backend cair, as telas abertas fecham e a próxima o inicia de novo.
struct Router {
    backend: Vec<String>,
    cwd: Option<PathBuf>,
    state: Mutex<RouterState>,
}

#[derive(Default)]
struct RouterState {
    next: u64,
    screens: HashMap<u64, Box<dyn transport::Outgoing>>,
    /// A entrada do backend; `None` até a primeira tela (ou depois que ele caiu).
    stdin: Option<std::process::ChildStdin>,
    child: Option<std::process::Child>,
}

impl Router {
    /// Uma tela nova: inicia o backend se preciso e avisa que ela abriu.
    fn open(self: &Arc<Self>, outgoing: Box<dyn transport::Outgoing>) -> Result<u64, String> {
        let mut st = self.state.lock().map_err(|_| "estado da ponte")?;
        if st.stdin.is_none() {
            let (program, args) = self.backend.split_first().ok_or("informe o comando do backend depois de --")?;
            let mut command = Command::new(program);
            command.args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());
            if let Some(dir) = &self.cwd {
                command.current_dir(dir);
            }
            let mut child = command.spawn().map_err(|e| format!("não consegui iniciar o backend '{program}': {e}"))?;
            aylis_core::stderr_line!("aylis-host: backend das sessões iniciado ({})", child.id());
            let stdout = child.stdout.take().ok_or("stdout")?;
            st.stdin = Some(child.stdin.take().ok_or("stdin")?);
            st.child = Some(child);
            let router = self.clone();
            std::thread::spawn(move || router.from_backend(stdout));
        }
        st.next += 1;
        let session = st.next;
        st.screens.insert(session, outgoing);
        Self::write(&mut st, &serde_json::json!({ "session": session, "event": "session_open" }).to_string());
        Ok(session)
    }

    /// Uma mensagem da tela para o backend, com o número dela.
    fn to_backend(&self, session: u64, line: &str) -> bool {
        let Ok(mut st) = self.state.lock() else { return false };
        let line = match serde_json::from_str::<serde_json::Value>(line) {
            Ok(serde_json::Value::Object(mut map)) => {
                map.insert("session".into(), session.into());
                serde_json::Value::Object(map).to_string()
            }
            // Não é um objeto: não é do protocolo.
            _ => return true,
        };
        Self::write(&mut st, &line) && st.screens.contains_key(&session)
    }

    fn close(&self, session: u64) {
        if let Ok(mut st) = self.state.lock() {
            if st.screens.remove(&session).is_some() {
                Self::write(&mut st, &serde_json::json!({ "session": session, "event": "session_close" }).to_string());
            }
        }
    }

    fn write(st: &mut RouterState, line: &str) -> bool {
        let ok = st.stdin.as_mut().is_some_and(|s| writeln!(s, "{line}").and_then(|_| s.flush()).is_ok());
        if !ok {
            st.stdin = None;
        }
        ok
    }

    /// Lê o backend e manda cada mensagem à tela dela. Ao terminar (o backend caiu), fecha as
    /// telas abertas.
    fn from_backend(&self, stdout: std::process::ChildStdout) {
        use std::io::BufRead;
        for line in std::io::BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Ok(serde_json::Value::Object(mut map)) = serde_json::from_str::<serde_json::Value>(&line) else {
                aylis_core::stderr_line!("aylis-host: linha inválida do backend: {line}");
                continue;
            };
            let Some(session) = map.remove("session").and_then(|s| s.as_u64()) else {
                aylis_core::stderr_line!("aylis-host: mensagem do backend sem \"session\" (com --sessions, toda mensagem diz a tela): {line}");
                continue;
            };
            let text = serde_json::Value::Object(map).to_string();
            let Ok(mut st) = self.state.lock() else { break };
            let failed = st.screens.get_mut(&session).is_some_and(|out| out.send_line(&text).is_err());
            if failed {
                st.screens.remove(&session);
            }
        }
        aylis_core::stderr_line!("aylis-host: o backend das sessões terminou");
        if let Ok(mut st) = self.state.lock() {
            // Soltar as saídas fecha as conexões; a próxima tela inicia outro backend.
            st.screens.clear();
            st.stdin = None;
            if let Some(mut child) = st.child.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

/// Aceita conexões para sempre; uma thread por tela e um backend por tela (ou um só para
/// todas, com `sessions`).
pub fn run(address: &str, tls: Option<Tls<'_>>, backend: &[String], cwd: Option<PathBuf>, limits: Limits, sessions: bool) -> Result<(), String> {
    let config = transport::server_config(tls)?;
    let listener = TcpListener::bind(address).map_err(|e| format!("não consegui ouvir em {address}: {e}"))?;
    let local = listener.local_addr().map_err(|e| e.to_string())?;
    let scheme = if config.is_some() { "wss" } else { "ws" };
    aylis_core::stderr_line!("aylis-host: ponte em {scheme}://{local} → {}", backend.join(" "));
    if !local.ip().is_loopback() {
        if config.is_none() {
            aylis_core::stderr_line!("aylis-host: aviso: ouvindo na rede sem TLS (use --tls-cert e --tls-key)");
        }
        if limits.origins.is_empty() {
            aylis_core::stderr_line!("aylis-host: aviso: qualquer site pode conectar pelo navegador (use --allow-origin)");
        }
    }
    let router = sessions.then(|| Arc::new(Router { backend: backend.to_vec(), cwd: cwd.clone(), state: Mutex::default() }));
    let backend = Arc::new(backend.to_vec());
    let limits = Arc::new(limits);
    let open = Arc::new(Mutex::new(Open::default()));
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let Ok(peer) = stream.peer_addr() else { continue };
        // Acima do limite, fecha na hora: nem handshake, nem backend.
        let slot = match take_slot(&open, peer.ip(), &limits) {
            Ok(slot) => slot,
            Err(e) => {
                aylis_core::stderr_line!("aylis-host: {peer} recusado: {e}");
                continue;
            }
        };
        let (config, backend, cwd, limits, router) = (config.clone(), backend.clone(), cwd.clone(), limits.clone(), router.clone());
        std::thread::spawn(move || {
            let _slot = slot;
            let result = match router {
                Some(router) => serve_session(stream, config, &router, &limits.origins),
                None => serve(stream, config, &backend, cwd, &limits.origins),
            };
            if let Err(e) = result {
                aylis_core::stderr_line!("aylis-host: {peer}: {e}");
            }
            aylis_core::stderr_line!("aylis-host: {peer} saiu");
        });
    }
    Ok(())
}

/// Uma tela no backend das sessões: o que ela manda vai com o número dela.
fn serve_session(stream: std::net::TcpStream, config: Option<Arc<rustls::ServerConfig>>, router: &Arc<Router>, origins: &[String]) -> Result<(), String> {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    let connection = transport::accept(stream, config, origins)?;
    let session = router.open(connection.outgoing)?;
    aylis_core::stderr_line!("aylis-host: {peer} conectou (sessão {session})");
    for line in connection.incoming {
        let Ok(line) = line else { break };
        if !router.to_backend(session, &line) {
            break;
        }
    }
    router.close(session);
    Ok(())
}

fn serve(stream: std::net::TcpStream, config: Option<Arc<rustls::ServerConfig>>, backend: &[String], cwd: Option<PathBuf>, origins: &[String]) -> Result<(), String> {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    let connection = transport::accept(stream, config, origins)?;
    let (program, args) = backend.split_first().ok_or("informe o comando do backend depois de --")?;
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    let mut child = command.spawn().map_err(|e| format!("não consegui iniciar o backend '{program}': {e}"))?;
    aylis_core::stderr_line!("aylis-host: {peer} conectou (backend {})", child.id());
    let mut stdin = child.stdin.take().ok_or("stdin")?;
    let stdout = child.stdout.take().ok_or("stdout")?;
    let mut outgoing = connection.outgoing;
    // Backend → tela.
    let to_screen = std::thread::spawn(move || {
        use std::io::BufRead;
        for line in std::io::BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if outgoing.send_line(&line).is_err() {
                break;
            }
        }
        // O backend terminou: a conexão fecha junto (o `outgoing` sai daqui).
    });
    // Tela → backend.
    for line in connection.incoming {
        let Ok(line) = line else { break };
        if writeln!(stdin, "{line}").and_then(|_| stdin.flush()).is_err() {
            break;
        }
    }
    // A tela saiu: o backend acaba.
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();
    let _ = to_screen.join();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_are_limited_per_ip_and_in_total() {
        let open = Arc::new(Mutex::new(Open::default()));
        let limits = Limits { max_connections: 3, max_per_ip: 2, origins: Vec::new() };
        let (a, b): (IpAddr, IpAddr) = ("10.0.0.1".parse().unwrap(), "10.0.0.2".parse().unwrap());
        let first = take_slot(&open, a, &limits).unwrap();
        let _second = take_slot(&open, a, &limits).unwrap();
        assert!(take_slot(&open, a, &limits).unwrap_err().contains("por IP"));
        let _third = take_slot(&open, b, &limits).unwrap();
        assert!(take_slot(&open, b, &limits).unwrap_err().contains("3 conexões"));
        // Uma conexão que sai devolve a vaga.
        drop(first);
        assert!(take_slot(&open, a, &limits).is_ok());
    }
}
