//! A ponte stdio do app publicado: o Claude Desktop (e outros clientes que só falam stdio) inicia
//! `app.exe --mcp-stdio`, e a ponte leva cada mensagem JSON-RPC ao [`McpServer`](super::McpServer)
//! do app que já está aberto (o login de quem usa o app), pelo HTTP de sempre: só `127.0.0.1`,
//! com o token, sem `Origin` de fora.
//!
//! Desligada por padrão: só com a opção `mcp_stdio` do projeto (Projeto → Opções → Publicar) o
//! app aceita o `--mcp-stdio` e o servidor deixa o endereço para a ponte achar.
//!
//! Como a ponte acha a porta e o token sem que eles fiquem onde outro usuário leia: o
//! `McpServer` ligado grava `<pasta>/<executável>-<servidor>.json` na pasta de dados **deste
//! usuário** (`%LOCALAPPDATA%\aylis\mcp-apps` no Windows, que só o próprio usuário lê;
//! `~/.local/share/aylis/mcp-apps` com permissão 0700, e o arquivo 0600, nos outros), e apaga
//! ao parar. O nome leva um resumo do caminho do executável: duas instalações do mesmo app não
//! se confundem. O token muda a cada vez que o servidor liga (o padrão, com `token` vazio).

use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use serde_json::{Value, json};

static ENABLED: AtomicBool = AtomicBool::new(false);
/// Este processo roda a ponte (`--mcp-stdio`), não o app.
static BRIDGING: AtomicBool = AtomicBool::new(false);
static APP_VERSION: RwLock<String> = RwLock::new(String::new());

/// Liga a ponte (a opção `mcp_stdio` do projeto; o `aylis::Application` chama ao abrir).
pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

/// A ponte está ligada neste app.
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// A versão do app (o `serverInfo.version` quando o `server_version` do servidor está vazio).
pub fn set_app_version(version: &str) {
    if let Ok(mut v) = APP_VERSION.write() {
        *v = version.to_string();
    }
}

pub(super) fn app_version() -> String {
    APP_VERSION.read().map(|v| v.clone()).unwrap_or_default()
}

/// A pasta dos endereços dos servidores deste usuário (`AYLIS_MCP_APPS_DIR` troca, para os
/// testes).
pub fn dir() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("AYLIS_MCP_APPS_DIR") {
        return Some(PathBuf::from(d));
    }
    if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join("AppData").join("Local")))
            .map(|d| d.join("aylis").join("mcp-apps"))
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support/aylis/mcp-apps"))
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
            .map(|d| d.join("aylis").join("mcp-apps"))
    }
}

/// O começo do nome dos arquivos deste executável: o nome dele e um resumo (FNV-1a) do caminho.
fn exe_prefix() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    let exe = std::fs::canonicalize(&exe).unwrap_or(exe);
    let text = exe.to_string_lossy().to_lowercase();
    let hash = text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
    let stem = exe.file_stem().map(|s| sanitize(&s.to_string_lossy())).unwrap_or_default();
    Some(format!("{stem}-{hash:016x}-"))
}

fn sanitize(text: &str) -> String {
    text.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect()
}

/// O endereço gravado para a ponte; apagado quando solto (o servidor parou).
pub(super) struct Discovery {
    path: PathBuf,
}

impl Drop for Discovery {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Grava o endereço do servidor `name` (a porta e o token) para a ponte deste executável.
pub(super) fn publish(name: &str, port: u16, token: &str) -> Result<Discovery, String> {
    if BRIDGING.load(Ordering::Relaxed) {
        return Err("MCP: este processo é a ponte stdio".into());
    }
    let dir = dir().ok_or("MCP: sem pasta de dados do usuário")?;
    create_private_dir(&dir)?;
    let prefix = exe_prefix().ok_or("MCP: não achei o executável")?;
    let path = dir.join(format!("{prefix}{}.json", sanitize(name)));
    let text = json!({ "name": name, "port": port, "token": token, "pid": std::process::id() }).to_string();
    write_private(&path, text.as_bytes())?;
    Ok(Discovery { path })
}

#[cfg(unix)]
fn create_private_dir(dir: &Path) -> Result<(), String> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)).map_err(|e| format!("{}: {e}", dir.display()))
}

#[cfg(not(unix))]
fn create_private_dir(dir: &Path) -> Result<(), String> {
    // Dentro de %LOCALAPPDATA%: a permissão herdada é só deste usuário (e do sistema).
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))
}

#[cfg(unix)]
fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(bytes).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(not(unix))]
fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

/// Um servidor aberto deste executável.
#[derive(Clone, Debug)]
struct Target {
    name: String,
    port: u16,
    token: String,
}

/// O servidor a usar: o de nome `name`, ou o único aberto.
fn find(name: Option<&str>) -> Result<Option<Target>, String> {
    let (Some(dir), Some(prefix)) = (dir(), exe_prefix()) else { return Ok(None) };
    let Ok(entries) = std::fs::read_dir(&dir) else { return Ok(None) };
    let mut found: Vec<Target> = entries
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with(&prefix))
        .filter_map(|e| serde_json::from_str::<Value>(&std::fs::read_to_string(e.path()).ok()?).ok())
        .filter_map(|v| Some(Target { name: v["name"].as_str()?.to_string(), port: u16::try_from(v["port"].as_u64()?).ok()?, token: v["token"].as_str()?.to_string() }))
        .collect();
    if let Some(name) = name {
        found.retain(|t| t.name.eq_ignore_ascii_case(name));
    }
    match found.len() {
        0 => Ok(None),
        1 => Ok(found.pop()),
        _ => Err(format!("o app tem mais de um servidor MCP aberto ({}): passe o nome depois de --mcp-stdio", found.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", "))),
    }
}

/// Um `POST /mcp` ao servidor: `Some(corpo)` com resposta, `None` só aceito (notificações).
fn post(target: &Target, body: &str) -> Result<Option<String>, String> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], target.port));
    let mut s = TcpStream::connect_timeout(&addr, Duration::from_secs(3)).map_err(|e| format!("o app não respondeu ({e})"))?;
    // O app espera até o `call_timeout` dele por uma resposta.
    let _ = s.set_read_timeout(Some(Duration::from_secs(600)));
    write!(s, "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nAccept: application/json\r\nAuthorization: Bearer {}\r\nContent-Length: {}\r\n\r\n{body}", target.port, target.token, body.len()).map_err(|e| e.to_string())?;
    let mut raw = Vec::new();
    s.read_to_end(&mut raw).map_err(|e| e.to_string())?;
    let raw = String::from_utf8_lossy(&raw);
    let status: u16 = raw.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let reply = raw.split_once("\r\n\r\n").map(|(_, b)| b.trim().to_string()).unwrap_or_default();
    match status {
        200 => Ok(Some(reply)),
        202 => Ok(None),
        _ if !reply.is_empty() => Ok(Some(reply)),
        other => Err(format!("o app recusou o pedido (HTTP {other})")),
    }
}

/// Os avisos do servidor (o `GET /mcp`) repassados ao stdout, enquanto a conexão durar.
fn listen(target: &Target, out: &Output) {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], target.port));
    let Ok(mut s) = TcpStream::connect_timeout(&addr, Duration::from_secs(3)) else { return };
    if write!(s, "GET /mcp HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAccept: text/event-stream\r\nAuthorization: Bearer {}\r\n\r\n", target.port, target.token).is_err() {
        return;
    }
    let mut reader = std::io::BufReader::new(s);
    let mut line = String::new();
    // Os cabeçalhos da resposta.
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) if line.trim().is_empty() => break,
            Ok(_) if line.starts_with("HTTP/") && !line.contains(" 200 ") => return,
            Ok(_) => {}
        }
    }
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => {
                if let Some(data) = line.trim_end().strip_prefix("data:") {
                    emit(out, data.trim());
                }
            }
        }
    }
}

fn emit(out: &Output, line: &str) {
    if let Ok(mut o) = out.lock() {
        let _ = writeln!(o, "{line}");
        let _ = o.flush();
    }
}

fn rpc_error(id: Value, message: &str) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32000, "message": message } }).to_string()
}

/// A ponte: lê JSON-RPC do stdin (uma mensagem por linha), manda ao servidor do app aberto e
/// escreve as respostas (e os avisos do servidor) no stdout. Acaba quando o stdin fecha.
/// `server`: o nome do servidor, quando o app tem mais de um.
pub fn run_bridge(server: Option<&str>) -> Result<(), String> {
    // Este processo é a ponte: um servidor que o código do app ligue aqui não grava endereço
    // (apagaria o do app aberto ao sair).
    BRIDGING.store(true, Ordering::Relaxed);
    let out: Output = Arc::new(Mutex::new(Box::new(std::io::stdout())));
    bridge(server, std::io::stdin().lock(), out)
}

/// Onde a ponte escreve (o stdout; nos testes, um buffer).
pub type Output = Arc<Mutex<Box<dyn Write + Send>>>;

/// A ponte com a entrada e a saída dadas (a [`run_bridge`] usa o stdin e o stdout).
pub fn bridge(server: Option<&str>, input: impl BufRead, out: Output) -> Result<(), String> {
    // Os avisos: uma thread ouve o servidor em que a última mensagem chegou.
    let (tx, rx) = mpsc::channel::<Target>();
    let listener_out = out.clone();
    let _ = std::thread::Builder::new().name("aylis-mcp-avisos".into()).spawn(move || {
        while let Ok(mut target) = rx.recv() {
            // Só a mais recente vale (o app pode ter sido reaberto em outra porta).
            while let Ok(t) = rx.try_recv() {
                target = t;
            }
            listen(&target, &listener_out);
        }
    });
    for line in input.lines() {
        let Ok(line) = line else { break };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let message: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                emit(&out, &json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": format!("JSON inválido: {e}") } }).to_string());
                continue;
            }
        };
        let id = message.get("id").cloned();
        let answer = |text: &str| {
            if let Some(id) = &id {
                emit(&out, &rpc_error(id.clone(), text));
            }
        };
        let target = match find(server) {
            Ok(Some(t)) => t,
            Ok(None) => {
                answer("o app não está aberto (ou o servidor MCP dele está desligado): abra o app e tente de novo");
                continue;
            }
            Err(e) => {
                answer(&e);
                continue;
            }
        };
        match post(&target, line) {
            Ok(Some(reply)) => emit(&out, &reply),
            Ok(None) => {}
            Err(e) => {
                answer(&e);
                continue;
            }
        }
        // A thread dos avisos pega a mais recente quando a conexão dela cai (ou na primeira).
        let _ = tx.send(target);
    }
    Ok(())
}
