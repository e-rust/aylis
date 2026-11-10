//! Um servidor web mínimo (só arquivos, só nesta máquina) para o Executar no navegador:
//! o `.wasm` não abre direto do disco.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use crate::process::{CANCELLED, Control, Event, LineKind, Sink};

/// Serve `dir` em `http://127.0.0.1:<porta>` até `control` ser cancelado. Com `open`, abre
/// o navegador padrão.
pub fn serve(dir: &Path, open: bool, control: &Control, sink: Sink) -> Result<(), String> {
    let listener = [8080u16, 8081, 8082, 0]
        .iter()
        .find_map(|p| TcpListener::bind(("127.0.0.1", *p)).ok())
        .ok_or("não consegui abrir uma porta local")?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let url = format!("http://127.0.0.1:{port}/");
    sink(Event::Step(format!("Servindo em {url} (Parar encerra)")));
    if open {
        open_url(&url);
    }
    let root = dir.to_path_buf();
    loop {
        if control.is_cancelled() {
            return Err(CANCELLED.into());
        }
        match listener.accept() {
            Ok((stream, _)) => {
                let root = root.clone();
                std::thread::spawn(move || {
                    let _ = respond(stream, &root);
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(100)),
            Err(e) => {
                sink(Event::Line(format!("servidor: {e}"), LineKind::Tool));
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

/// Abre um endereço no navegador padrão.
pub fn open_url(url: &str) {
    // Sem `cmd /c start`: o `&` de uma query string não corta o endereço.
    let url = url.to_string();
    std::thread::spawn(move || {
        let _ = crate::shell::open(&url);
    });
}

fn respond(mut stream: TcpStream, root: &Path) -> std::io::Result<()> {
    stream.set_nonblocking(false)?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request = String::new();
    reader.read_line(&mut request)?;
    // Descarta os cabeçalhos.
    let mut header = String::new();
    while reader.read_line(&mut header)? > 2 {
        header.clear();
    }
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let path = path.split(['?', '#']).next().unwrap_or("/");
    let file = resolve(root, path);
    let (status, body, kind) = match file.as_ref().and_then(|f| std::fs::read(f).ok().map(|b| (f, b))) {
        Some((f, body)) => ("200 OK", body, content_type(f)),
        None => ("404 Not Found", b"nao encontrado".to_vec(), "text/plain"),
    };
    write!(stream, "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n", body.len())?;
    stream.write_all(&body)
}

/// O arquivo pedido, sem sair da pasta (`..` não vale).
fn resolve(root: &Path, url_path: &str) -> Option<PathBuf> {
    let relative = Path::new(url_path.trim_start_matches('/'));
    if relative.components().any(|c| !matches!(c, Component::Normal(_))) {
        return None;
    }
    let file = root.join(relative);
    if file.is_dir() { Some(file.join("index.html")) } else { Some(file) }
}

fn content_type(file: &Path) -> &'static str {
    match file.extension().and_then(|e| e.to_str()).unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript",
        "wasm" => "application/wasm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "json" => "application/json",
        "css" => "text/css",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
}
