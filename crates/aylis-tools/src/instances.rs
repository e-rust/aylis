//! As IDEs abertas: cada uma se anota em `%LOCALAPPDATA%\aylis\instances\<pid>.json` com o
//! projeto aberto, a porta do servidor MCP dela, quando teve o foco pela última vez e um canal de
//! controle (uma porta local e um segredo). Serve para:
//!
//! - o duplo clique num `.aylisproj` ir para a IDE que já tem aquele projeto (ou para uma na tela
//!   inicial) em vez de abrir outra: [`send`];
//! - a ponte do MCP (`aylis-tool mcp`) escolher a IDE: a que teve o foco por último, fixada a
//!   partir da primeira chamada (veja [`crate::mcp`]).
//!
//! O canal de controle aceita uma linha JSON (`{"secret": ..., "open": "caminho"}` ou
//! `{"secret": ..., "focus": true}`) e responde `ok`. O segredo fica no arquivo da instância, na
//! pasta do usuário: outro usuário da máquina não manda nada.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

/// Uma IDE aberta.
#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    pub pid: u32,
    /// O projeto aberto (`None`: a tela inicial ou um form solto).
    pub project: Option<PathBuf>,
    /// A porta do servidor MCP dela (`None`: desligado).
    pub mcp_port: Option<u16>,
    /// O canal de controle.
    pub control_port: u16,
    pub secret: String,
    /// Quando teve o foco pela última vez (segundos desde 1970).
    pub focused_at: f64,
    /// As sessões de agente (cada ponte do MCP) ligadas a esta IDE. A que abriu a IDE vem pela
    /// `AYLIS_IDE_MCP_SESSION` (a ponte espera a instância com a marca dela); as outras entram e
    /// saem pelo `aylis/session`. Vazia: a IDE está livre para a próxima sessão. Várias: sessões
    /// no mesmo projeto, que ficam todas na IDE dele (decidido com o autor em 08/11).
    pub sessions: Vec<String>,
}

fn dir() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join("instances"))
}

fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64())
}

/// Um segredo novo para o canal de controle.
pub fn new_secret() -> String {
    let mut bytes = [0u8; 24];
    let _ = crate::random_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Dois caminhos do mesmo arquivo (no Windows, sem diferenciar maiúsculas).
pub fn same_path(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        let p = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        let s = p.to_string_lossy().replace('\\', "/");
        if cfg!(windows) { s.to_lowercase() } else { s }
    };
    norm(a) == norm(b)
}

impl Instance {
    fn to_json(&self) -> Value {
        json!({
            "pid": self.pid,
            "project": self.project.as_ref().map(|p| p.display().to_string()),
            "mcp_port": self.mcp_port,
            "control_port": self.control_port,
            "secret": self.secret,
            "focused_at": self.focused_at,
            "sessions": self.sessions,
        })
    }

    fn from_json(v: &Value) -> Option<Instance> {
        Some(Instance {
            pid: v["pid"].as_u64()? as u32,
            project: v["project"].as_str().map(PathBuf::from),
            mcp_port: v["mcp_port"].as_u64().and_then(|p| u16::try_from(p).ok()),
            control_port: u16::try_from(v["control_port"].as_u64()?).ok()?,
            secret: v["secret"].as_str()?.to_string(),
            focused_at: v["focused_at"].as_f64().unwrap_or(0.0),
            sessions: v["sessions"].as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default(),
        })
    }

    /// Grava (ou atualiza) o arquivo desta instância.
    pub fn save(&self) -> Result<(), String> {
        let dir = dir().ok_or("sem pasta de dados do usuário")?;
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        std::fs::write(dir.join(format!("{}.json", self.pid)), self.to_json().to_string()).map_err(|e| e.to_string())
    }

    /// Anota que esta IDE teve o foco agora.
    pub fn touch(&mut self) {
        self.focused_at = now();
        let _ = self.save();
    }

    /// Manda um comando pelo canal de controle; `true` se a IDE respondeu `ok`.
    pub fn send(&self, command: Value) -> bool {
        let Ok(mut stream) = TcpStream::connect_timeout(&(std::net::Ipv4Addr::LOCALHOST, self.control_port).into(), Duration::from_millis(800)) else { return false };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
        let mut message = command;
        message["secret"] = Value::String(self.secret.clone());
        if writeln!(stream, "{message}").is_err() {
            return false;
        }
        let mut reply = String::new();
        BufReader::new(stream).read_line(&mut reply).is_ok() && reply.trim() == "ok"
    }

    /// A IDE ainda responde no canal de controle.
    pub fn alive(&self) -> bool {
        TcpStream::connect_timeout(&(std::net::Ipv4Addr::LOCALHOST, self.control_port).into(), Duration::from_millis(300)).is_ok()
    }
}

/// Tira o arquivo desta IDE (ao fechar).
pub fn unregister(pid: u32) {
    if let Some(dir) = dir() {
        let _ = std::fs::remove_file(dir.join(format!("{pid}.json")));
    }
}

/// As IDEs abertas (as que não respondem mais saem da lista e o arquivo delas é apagado), da
/// que teve o foco por último para a mais antiga.
pub fn list() -> Vec<Instance> {
    let Some(dir) = dir() else { return Vec::new() };
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut out = Vec::new();
    for e in entries.flatten() {
        let path = e.path();
        let Some(inst) = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| Instance::from_json(&v)) else {
            continue;
        };
        if inst.pid != std::process::id() && !inst.alive() {
            let _ = std::fs::remove_file(&path);
            continue;
        }
        out.push(inst);
    }
    out.sort_by(|a, b| b.focused_at.total_cmp(&a.focused_at));
    out
}

/// Abrir `path` (um `.aylisproj`) numa IDE já aberta: a que tem esse projeto vem para a frente;
/// senão, uma na tela inicial abre ele. `true` se alguma aceitou (quem pediu pode fechar).
pub fn send(path: &Path) -> bool {
    let others: Vec<Instance> = list().into_iter().filter(|i| i.pid != std::process::id()).collect();
    allow_foreground();
    if let Some(i) = others.iter().find(|i| i.project.as_ref().is_some_and(|p| same_path(p, path))) {
        return i.send(json!({ "focus": true }));
    }
    let is_project = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("aylisproj"));
    if is_project
        && let Some(i) = others.iter().find(|i| i.project.is_none())
    {
        return i.send(json!({ "open": path.display().to_string() }));
    }
    false
}

/// O Windows só deixa a janela de outro processo vir para a frente se quem está na frente
/// (este, aberto pelo duplo clique) permitir.
fn allow_foreground() {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn AllowSetForegroundWindow(pid: u32) -> i32;
        }
        // ASFW_ANY
        unsafe { AllowSetForegroundWindow(u32::MAX) };
    }
}

/// Um comando que chegou pelo canal de controle (já conferido o segredo).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Focus,
    Open(PathBuf),
}

/// Liga o canal de controle desta IDE: devolve a porta; cada comando válido vai para `on`.
pub fn listen(secret: String, on: impl Fn(Command) + Send + 'static) -> Result<u16, String> {
    let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    std::thread::Builder::new()
        .name("aylis-controle".into())
        .spawn(move || {
            for stream in listener.incoming().flatten() {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
                let mut line = String::new();
                let mut reader = BufReader::new(&stream);
                if reader.read_line(&mut line).is_err() || line.len() > 64 * 1024 {
                    continue;
                }
                let Ok(v) = serde_json::from_str::<Value>(&line) else { continue };
                if v["secret"].as_str() != Some(secret.as_str()) {
                    continue;
                }
                let command = match (v["open"].as_str(), v["focus"].as_bool()) {
                    (Some(p), _) => Command::Open(PathBuf::from(p)),
                    (None, Some(true)) => Command::Focus,
                    _ => continue,
                };
                on(command);
                let _ = (&stream).write_all(b"ok\n");
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_control_channel_needs_the_secret() {
        let (tx, rx) = std::sync::mpsc::channel();
        let secret = new_secret();
        let port = listen(secret.clone(), move |c| {
            let _ = tx.send(c);
        })
        .unwrap();
        let me = Instance { pid: 1, project: None, mcp_port: None, control_port: port, secret, focused_at: 0.0, sessions: Vec::new() };
        assert!(me.send(json!({ "open": "C:/x/App.aylisproj" })));
        assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), Command::Open(PathBuf::from("C:/x/App.aylisproj")));
        let wrong = Instance { secret: "outro".into(), ..me.clone() };
        assert!(!wrong.send(json!({ "focus": true })));
        assert!(rx.recv_timeout(Duration::from_millis(300)).is_err());
    }

    /// As sessões ligadas a uma IDE (várias no mesmo projeto) vão e voltam do arquivo dela.
    #[test]
    fn the_sessions_of_an_ide_round_trip() {
        let me = Instance { pid: 7, project: Some(PathBuf::from("C:/x/App.aylisproj")), mcp_port: Some(7341), control_port: 1, secret: "s".into(), focused_at: 1.0, sessions: vec!["1-a".into(), "2-b".into()] };
        assert_eq!(Instance::from_json(&me.to_json()), Some(me));
        let none = Instance::from_json(&json!({ "pid": 1, "control_port": 2, "secret": "s" })).unwrap();
        assert!(none.sessions.is_empty());
    }
}
