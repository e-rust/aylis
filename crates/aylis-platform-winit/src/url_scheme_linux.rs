//! O prefixo de URL próprio no Linux (o componente `UrlScheme`; decidido em 14/11), como no
//! Windows:
//!
//! - **Registro:** um `.desktop` do usuário em `~/.local/share/applications`
//!   (`aylis-<prefixo>-<executável>.desktop`, com `MimeType=x-scheme-handler/<prefixo>` e o
//!   `Exec` apontando para este executável com `%u`), escolhido pelo `xdg-mime default`: só do
//!   usuário, sem administrador. A publicação (`.deb`) põe o prefixo no `.desktop` do pacote.
//! - **O app já aberto:** a cópia nova, aberta pelo link, repassa o endereço à que já está
//!   rodando por um socket local na pasta do usuário (`$XDG_RUNTIME_DIR`, que só ele lê) e
//!   fecha. Sem essa pasta, cada cópia recebe os próprios links.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};

/// O maior pedido que o socket aceita (vários links de uma vez).
const MAX_MESSAGE: u64 = 64 * 1024;

fn exe() -> Result<String, String> {
    std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).map_err(|e| e.to_string())
}

/// FNV-1a do executável: o mesmo número nas duas cópias (o `DefaultHasher` pode mudar entre
/// versões do Rust).
fn exe_hash() -> String {
    let hash = exe().unwrap_or_default().bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
    format!("{hash:016x}")
}

fn applications() -> Option<PathBuf> {
    let data = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))?;
    Some(data.join("applications"))
}

fn desktop_name(scheme: &str) -> String {
    format!("aylis-{scheme}-{}.desktop", exe_hash())
}

/// O conteúdo do `.desktop` que abre `exe` com os links do `scheme` (o `%u` é o endereço).
pub(crate) fn desktop_entry(scheme: &str, description: &str, exe: &str) -> String {
    let name = if description.trim().is_empty() { scheme } else { description.trim() };
    // No `Exec`, as aspas protegem os espaços do caminho; `"`, `` ` ``, `$` e `\` dentro delas
    // levam a barra (a especificação do `.desktop`).
    let quoted: String = exe.chars().flat_map(|c| if matches!(c, '"' | '`' | '$' | '\\') { vec!['\\', c] } else { vec![c] }).collect();
    format!(
        "[Desktop Entry]\nType=Application\nName={name}\nExec=\"{quoted}\" %u\nTerminal=false\nNoDisplay=true\nMimeType=x-scheme-handler/{scheme};\n"
    )
}

pub(crate) fn register(scheme: &str, description: &str) -> Result<(), String> {
    let dir = applications().ok_or("sem a pasta do usuário (HOME)")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let file = desktop_name(scheme);
    std::fs::write(dir.join(&file), desktop_entry(scheme, description, &exe()?)).map_err(|e| format!("{file}: {e}"))?;
    // O `xdg-mime` escolhe este `.desktop` para o prefixo (argumentos separados, sem shell).
    let ok = Command::new("xdg-mime").args(["default", &file, &format!("x-scheme-handler/{scheme}")]).status().map(|s| s.success()).unwrap_or(false);
    let _ = Command::new("update-desktop-database").arg(&dir).status();
    if ok { Ok(()) } else { Err("o xdg-mime não está instalado (xdg-utils)".into()) }
}

/// O prefixo abre este executável?
pub(crate) fn registered(scheme: &str) -> bool {
    let Ok(out) = Command::new("xdg-mime").args(["query", "default", &format!("x-scheme-handler/{scheme}")]).output() else { return false };
    String::from_utf8_lossy(&out.stdout).trim() == desktop_name(scheme)
}

pub(crate) fn unregister(scheme: &str) -> Result<(), String> {
    let Some(dir) = applications() else { return Ok(()) };
    let path = dir.join(desktop_name(scheme));
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let _ = Command::new("update-desktop-database").arg(&dir).status();
    }
    Ok(())
}

/// O socket: um por executável, na pasta do usuário (`None` sem `XDG_RUNTIME_DIR`).
fn socket_path() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?);
    dir.is_dir().then(|| dir.join(format!("aylis-links-{}.sock", exe_hash())))
}

type Receivers = Arc<Mutex<Vec<Box<dyn Fn(String) + Send>>>>;

/// Passa a receber os links das outras cópias (o servidor do socket sobe na primeira vez).
pub(crate) fn receive(on_link: Box<dyn Fn(String) + Send>) {
    static RECEIVERS: OnceLock<Receivers> = OnceLock::new();
    let mut started = false;
    let receivers = RECEIVERS.get_or_init(|| {
        started = true;
        Arc::new(Mutex::new(Vec::new()))
    });
    if let Ok(mut list) = receivers.lock() {
        list.push(on_link);
    }
    if started {
        let receivers = receivers.clone();
        let _ = std::thread::Builder::new().name("aylis-links".into()).spawn(move || serve(&receivers));
    }
}

fn serve(receivers: &Receivers) {
    let Some(path) = socket_path() else { return };
    // Outra cópia já atende: esta não recebe. Um socket velho (a cópia caiu) é trocado.
    if UnixStream::connect(&path).is_ok() {
        return;
    }
    let _ = std::fs::remove_file(&path);
    let Ok(listener) = UnixListener::bind(&path) else { return };
    // Parado, a thread dorme no `accept`.
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let mut text = String::new();
        let _ = stream.take(MAX_MESSAGE).read_to_string(&mut text);
        let Ok(list) = receivers.lock() else { return };
        for url in text.lines().map(str::trim).filter(|l| l.contains(':')) {
            for on_link in list.iter() {
                on_link(url.to_string());
            }
        }
    }
}

/// A cópia aberta pelo link: entrega os `links` à cópia que já está aberta. `true`: entregue
/// (esta pode fechar); `false`: não há outra aberta (esta segue e recebe ela mesma).
pub(crate) fn forward(links: &[String]) -> bool {
    let Some(path) = socket_path() else { return false };
    match UnixStream::connect(&path) {
        Ok(mut stream) => stream.write_all(links.join("\n").as_bytes()).is_ok(),
        Err(_) => false,
    }
}
