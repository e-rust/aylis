//! Um adaptador DAP pequeno sobre o `jdb` do JDK, para a IDE depurar Java (o adaptador
//! da Microsoft depende do servidor de linguagem do Java inteiro). Fala DAP pela entrada e
//! saída padrão (`aylis-tool jdb-dap <jdb> <pasta dos fontes>`) e comanda o `jdb` ligado ao
//! JDWP do processo (`-agentlib:jdwp=...,server=y`):
//!
//! | DAP | jdb |
//! |---|---|
//! | `setBreakpoints` | `stop at pacote.Classe:linha` / `clear ...` |
//! | `continue`, `next`, `stepIn`, `stepOut`, `pause` | `cont`, `next`, `step`, `step up`, `suspend` |
//! | `stackTrace` | `where` |
//! | `variables` | `locals` (no quadro escolhido com `up`/`down`) |
//! | `evaluate` | `print` |
//!
//! As paradas que o `jdb` anuncia (`Breakpoint hit:`, `Step completed:`) viram o evento
//! `stopped`; o fim do programa, `terminated`.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

/// O que chega do jdb: um pedaço de texto (as linhas e o prompt, que não termina em \n).
enum Out {
    Text(String),
    Closed,
}

struct Jdb {
    child: Child,
    stdin: ChildStdin,
    output: Receiver<Out>,
    /// O que já chegou e ainda não foi lido.
    buffer: String,
}

impl Jdb {
    fn start(jdb: &Path, host: &str, port: u16) -> Result<Jdb, String> {
        let mut command = Command::new(jdb);
        command.arg("-connect").arg(format!("com.sun.jdi.SocketAttach:hostname={host},port={port}"));
        let mut child = crate::process::quiet(&mut command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("não consegui iniciar o jdb: {e}"))?;
        let stdin = child.stdin.take().ok_or("sem stdin")?;
        let (tx, rx) = mpsc::channel();
        for stream in [Box::new(child.stdout.take().ok_or("sem stdout")?) as Box<dyn Read + Send>, Box::new(child.stderr.take().ok_or("sem stderr")?)] {
            let tx = tx.clone();
            std::thread::spawn(move || {
                let mut stream = stream;
                let mut buf = [0u8; 4096];
                loop {
                    match stream.read(&mut buf) {
                        Ok(0) | Err(_) => {
                            let _ = tx.send(Out::Closed);
                            return;
                        }
                        Ok(n) => {
                            let _ = tx.send(Out::Text(String::from_utf8_lossy(&buf[..n]).into_owned()));
                        }
                    }
                }
            });
        }
        let mut jdb = Jdb { child, stdin, output: rx, buffer: String::new() };
        // Até o primeiro prompt (`> ` com o programa rodando).
        jdb.until_prompt(Duration::from_secs(20))?;
        Ok(jdb)
    }

    /// Lê até o prompt do jdb (`> ` ou `thread[1] `) e devolve o texto antes dele.
    fn until_prompt(&mut self, timeout: Duration) -> Result<String, String> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(end) = prompt_end(&self.buffer) {
                let text = self.buffer[..end].to_string();
                self.buffer.drain(..end);
                let rest = self.buffer.trim_start_matches(|c: char| c != '\n').to_string();
                let _ = rest;
                self.buffer.clear();
                return Ok(text);
            }
            let left = deadline.saturating_duration_since(Instant::now());
            match self.output.recv_timeout(left) {
                Ok(Out::Text(t)) => self.buffer.push_str(&t),
                Ok(Out::Closed) => return Err("o jdb terminou".into()),
                Err(RecvTimeoutError::Timeout) => return Err("o jdb não respondeu".into()),
                Err(RecvTimeoutError::Disconnected) => return Err("o jdb terminou".into()),
            }
        }
    }

    /// Manda um comando e espera a resposta (o texto até o próximo prompt).
    fn command(&mut self, line: &str) -> Result<String, String> {
        self.send(line)?;
        self.until_prompt(Duration::from_secs(15))
    }

    fn send(&mut self, line: &str) -> Result<(), String> {
        writeln!(self.stdin, "{line}").and_then(|_| self.stdin.flush()).map_err(|e| format!("jdb: {e}"))
    }

    /// O que chegou sem pedir (o programa rodando): `None` se nada.
    fn poll(&mut self) -> Option<Out> {
        self.output.try_recv().ok()
    }
}

/// Onde termina o texto antes do prompt, se o buffer acaba num prompt.
fn prompt_end(buffer: &str) -> Option<usize> {
    let line_start = buffer.rfind('\n').map_or(0, |i| i + 1);
    let last = &buffer[line_start..];
    // `> ` (rodando) ou `thread[1] ` (parado numa thread).
    let thread_prompt = last.strip_suffix("] ").and_then(|s| s.rsplit_once('[')).is_some_and(|(name, n)| !name.is_empty() && !name.contains(' ') && !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
    let is_prompt = last == "> " || thread_prompt;
    is_prompt.then_some(line_start)
}

/// O nome completo da classe de um `.java` (`pacote.Classe`).
fn class_of(file: &Path) -> Option<String> {
    let stem = file.file_stem()?.to_string_lossy().to_string();
    let text = std::fs::read_to_string(file).ok()?;
    let package = text.lines().map(str::trim).find_map(|l| l.strip_prefix("package ")).map(|p| p.trim_end_matches(';').trim().to_string());
    Some(match package {
        Some(p) if !p.is_empty() => format!("{p}.{stem}"),
        _ => stem,
    })
}

/// O `.java` de um nome de arquivo (`Principal.java`) dentro de `dir`.
fn find_source(dir: &Path, name: &str, cache: &mut BTreeMap<String, Option<PathBuf>>) -> Option<PathBuf> {
    if let Some(found) = cache.get(name) {
        return found.clone();
    }
    fn walk(dir: &Path, name: &str) -> Option<PathBuf> {
        for entry in std::fs::read_dir(dir).ok()?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = walk(&path, name) {
                    return Some(found);
                }
            } else if path.file_name().is_some_and(|n| n.to_string_lossy() == name) {
                return Some(path);
            }
        }
        None
    }
    let found = walk(dir, name);
    cache.insert(name.to_string(), found.clone());
    found
}

/// `  [1] pacote.Classe.metodo (Classe.java:12)` → (nome, arquivo, linha).
fn parse_frame(line: &str) -> Option<(String, String, usize)> {
    let line = line.trim();
    let rest = line.strip_prefix('[')?;
    let (_, rest) = rest.split_once("] ")?;
    let (method, location) = rest.rsplit_once(" (")?;
    let location = location.trim_end_matches(')');
    let (file, line_no) = location.rsplit_once(':')?;
    Some((method.to_string(), file.to_string(), line_no.trim().replace(['.', ','], "").parse().ok()?))
}

/// Mensagens DAP pela entrada e saída padrão.
struct Wire {
    seq: i64,
    out: std::io::Stdout,
}

impl Wire {
    fn send(&mut self, mut message: Value) {
        message["seq"] = json!(self.seq);
        self.seq += 1;
        let body = message.to_string();
        let mut out = self.out.lock();
        let _ = write!(out, "Content-Length: {}\r\n\r\n{body}", body.len());
        let _ = out.flush();
    }

    fn respond(&mut self, request: &Value, body: Value) {
        self.send(json!({ "type": "response", "request_seq": request["seq"], "command": request["command"], "success": true, "body": body }));
    }

    fn fail(&mut self, request: &Value, message: &str) {
        self.send(json!({ "type": "response", "request_seq": request["seq"], "command": request["command"], "success": false, "message": message }));
    }

    fn event(&mut self, event: &str, body: Value) {
        self.send(json!({ "type": "event", "event": event, "body": body }));
    }
}

/// Lê as mensagens DAP da entrada padrão (numa thread).
fn read_requests() -> Receiver<Value> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut input = BufReader::new(std::io::stdin());
        loop {
            let mut length = 0usize;
            loop {
                let mut header = String::new();
                if input.read_line(&mut header).unwrap_or(0) == 0 {
                    return;
                }
                let header = header.trim();
                if header.is_empty() {
                    break;
                }
                if let Some(v) = header.strip_prefix("Content-Length:") {
                    length = v.trim().parse().unwrap_or(0);
                }
            }
            let mut body = vec![0u8; length];
            if input.read_exact(&mut body).is_err() {
                return;
            }
            if let Ok(v) = serde_json::from_slice::<Value>(&body) {
                if tx.send(v).is_err() {
                    return;
                }
            }
        }
    });
    rx
}

/// O adaptador: roda até a IDE desligar (ou o programa terminar).
pub fn serve(jdb_path: &Path, sources: &Path) -> Result<(), String> {
    let requests = read_requests();
    let mut wire = Wire { seq: 1, out: std::io::stdout() };
    let mut jdb: Option<Jdb> = None;
    // Pontos de parada mandados: classe → linhas.
    let mut points: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut sources_cache = BTreeMap::new();
    let mut frame = 0usize;
    let mut running = true;
    let mut pending = String::new();
    loop {
        // O que o jdb disse sozinho: paradas e o fim do programa.
        if let Some(j) = jdb.as_mut() {
            while let Some(out) = j.poll() {
                match out {
                    Out::Closed => {
                        wire.event("terminated", json!({}));
                        return Ok(());
                    }
                    Out::Text(t) => pending.push_str(&t),
                }
            }
            if !pending.is_empty() {
                let stop = if pending.contains("Breakpoint hit:") {
                    Some("breakpoint")
                } else if pending.contains("Step completed:") {
                    Some("step")
                } else {
                    None
                };
                if pending.contains("The application exited") || pending.contains("application exited") {
                    wire.event("exited", json!({ "exitCode": 0 }));
                    wire.event("terminated", json!({}));
                    return Ok(());
                }
                if let Some(reason) = stop {
                    // Espera o prompt da thread parada antes de avisar.
                    if prompt_end(&pending).is_some() {
                        frame = 0;
                        running = false;
                        wire.event("stopped", json!({ "reason": reason, "threadId": 1, "allThreadsStopped": true }));
                        pending.clear();
                    }
                } else if prompt_end(&pending).is_some() {
                    pending.clear();
                }
            }
        }
        let request = match requests.recv_timeout(Duration::from_millis(50)) {
            Ok(r) => r,
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        };
        let command = request["command"].as_str().unwrap_or_default().to_string();
        let args = request["arguments"].clone();
        match command.as_str() {
            "initialize" => wire.respond(&request, json!({ "supportsConfigurationDoneRequest": true, "supportsEvaluateForHovers": true })),
            "attach" => {
                let host = args["hostName"].as_str().unwrap_or("127.0.0.1");
                let port = args["port"].as_u64().unwrap_or(5005) as u16;
                match Jdb::start(jdb_path, host, port) {
                    Ok(j) => {
                        jdb = Some(j);
                        wire.event("initialized", json!({}));
                        wire.respond(&request, json!({}));
                    }
                    Err(e) => wire.fail(&request, &e),
                }
            }
            "setBreakpoints" => {
                let file = PathBuf::from(args["source"]["path"].as_str().unwrap_or_default());
                let lines: Vec<usize> = args["breakpoints"].as_array().map(|a| a.iter().filter_map(|b| b["line"].as_u64()).map(|l| l as usize).collect()).unwrap_or_default();
                let (Some(j), Some(class)) = (jdb.as_mut(), class_of(&file)) else {
                    wire.respond(&request, json!({ "breakpoints": lines.iter().map(|l| json!({ "verified": false, "line": l })).collect::<Vec<_>>() }));
                    continue;
                };
                for old in points.remove(&class).unwrap_or_default() {
                    let _ = j.command(&format!("clear {class}:{old}"));
                }
                let mut result = Vec::new();
                for &line in &lines {
                    let out = j.command(&format!("stop at {class}:{line}")).unwrap_or_default();
                    let ok = out.contains("Set breakpoint") || out.contains("Deferring breakpoint") || out.contains("já") || !out.contains("Unable");
                    result.push(json!({ "verified": ok, "line": line }));
                }
                points.insert(class, lines);
                wire.respond(&request, json!({ "breakpoints": result }));
            }
            "configurationDone" => wire.respond(&request, json!({})),
            "threads" => wire.respond(&request, json!({ "threads": [{ "id": 1, "name": "main" }] })),
            "continue" | "next" | "stepIn" | "stepOut" => {
                let line = match command.as_str() {
                    "continue" => "cont",
                    "next" => "next",
                    "stepIn" => "step",
                    _ => "step up",
                };
                if let Some(j) = jdb.as_mut() {
                    let _ = j.send(line);
                }
                running = true;
                wire.respond(&request, json!({ "allThreadsContinued": true }));
                if command == "continue" {
                    wire.event("continued", json!({ "threadId": 1, "allThreadsContinued": true }));
                }
            }
            "pause" => {
                if let Some(j) = jdb.as_mut() {
                    let _ = j.command("suspend");
                }
                running = false;
                wire.respond(&request, json!({}));
                wire.event("stopped", json!({ "reason": "pause", "threadId": 1, "allThreadsStopped": true }));
            }
            "stackTrace" => {
                let Some(j) = jdb.as_mut().filter(|_| !running) else {
                    wire.respond(&request, json!({ "stackFrames": [], "totalFrames": 0 }));
                    continue;
                };
                let out = j.command("where").unwrap_or_default();
                let frames: Vec<Value> = out
                    .lines()
                    .filter_map(parse_frame)
                    .enumerate()
                    .map(|(i, (name, file, line))| {
                        let path = find_source(sources, &file, &mut sources_cache);
                        json!({ "id": i, "name": name, "line": line, "column": 1, "source": { "name": file, "path": path } })
                    })
                    .collect();
                let total = frames.len();
                wire.respond(&request, json!({ "stackFrames": frames, "totalFrames": total }));
            }
            "scopes" => {
                let id = args["frameId"].as_i64().unwrap_or(0);
                wire.respond(&request, json!({ "scopes": [{ "name": "Locais", "variablesReference": 1000 + id, "expensive": false }] }));
            }
            "variables" => {
                let wanted = (args["variablesReference"].as_i64().unwrap_or(1000) - 1000).max(0) as usize;
                let Some(j) = jdb.as_mut().filter(|_| !running) else {
                    wire.respond(&request, json!({ "variables": [] }));
                    continue;
                };
                // Vai ao quadro pedido (up = para fora, down = de volta).
                if wanted > frame {
                    let _ = j.command(&format!("up {}", wanted - frame));
                } else if wanted < frame {
                    let _ = j.command(&format!("down {}", frame - wanted));
                }
                frame = wanted;
                let out = j.command("locals").unwrap_or_default();
                let vars: Vec<Value> = out
                    .lines()
                    .filter_map(|l| l.split_once(" = "))
                    .map(|(name, value)| json!({ "name": name.trim(), "value": value.trim(), "variablesReference": 0 }))
                    .collect();
                wire.respond(&request, json!({ "variables": vars }));
            }
            "evaluate" => {
                let expression = args["expression"].as_str().unwrap_or_default();
                match jdb.as_mut().map(|j| j.command(&format!("print {expression}"))) {
                    Some(Ok(out)) => {
                        let value = out.lines().find_map(|l| l.split_once(" = ").map(|(_, v)| v.trim().to_string())).unwrap_or_else(|| out.trim().to_string());
                        wire.respond(&request, json!({ "result": value, "variablesReference": 0 }));
                    }
                    _ => wire.fail(&request, "sem o jdb"),
                }
            }
            "disconnect" | "terminate" => {
                if let Some(mut j) = jdb.take() {
                    let _ = j.send("resume");
                    let _ = j.send("exit");
                    std::thread::sleep(Duration::from_millis(200));
                    let _ = j.child.kill();
                }
                wire.respond(&request, json!({}));
                return Ok(());
            }
            _ => wire.fail(&request, "não suportado pelo adaptador do jdb"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_and_prompts_are_read() {
        assert_eq!(parse_frame("  [1] forms.Principal.salvar (Principal.java:12)"), Some(("forms.Principal.salvar".into(), "Principal.java".into(), 12)));
        assert_eq!(parse_frame("  [2] java.lang.Thread.run (Thread.java:1,583)"), Some(("java.lang.Thread.run".into(), "Thread.java".into(), 1583)));
        assert!(prompt_end("Set breakpoint x\n> ").is_some());
        assert!(prompt_end("Breakpoint hit: ...\nmain[1] ").is_some());
        assert!(prompt_end("algo sem prompt\n").is_none());
    }
}
