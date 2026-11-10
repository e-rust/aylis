//! O cliente do Language Server Protocol (o mesmo dos editores, como o VS Code): um servidor
//! por linguagem dá as sugestões, a declaração, as referências, a dica do tipo, a ajuda dos
//! parâmetros, o renomear, a formatação, os símbolos, as correções e os erros enquanto se
//! digita.
//!
//! Os servidores: rust-analyzer (Rust), gopls (Go), pylsp (Python),
//! typescript-language-server (TypeScript), csharp-ls (C#), Eclipse JDT LS (Java), clangd
//! (C++) e pasls (Free Pascal). O `Verificar ambiente` instala os que faltam
//! ([`crate::env`]), na pasta de ferramentas do aylis.
//!
//! Como o depurador ([`crate::dap`]): o servidor é um processo com mensagens JSON (cabeçalho
//! `Content-Length`) no stdin/stdout. Os pedidos não bloqueiam: [`Client::request`] devolve
//! o número, e as respostas e os avisos chegam em [`Client::poll`].

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_project::Language;
use serde_json::{Value, json};

/// Um servidor de linguagem: como abrir e o que mandar no `initialize`.
#[derive(Clone, Debug)]
pub struct Server {
    pub name: &'static str,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    /// O `languageId` dos documentos (`rust`, `pascal`, `typescriptreact`...).
    pub language_id: &'static str,
    pub init_options: Value,
}

fn tools() -> Option<PathBuf> {
    crate::dap::tools_dir()
}

fn exe(name: &str) -> String {
    crate::env::exe(name)
}

/// O primeiro arquivo que existe.
fn first(paths: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    paths.into_iter().find(|p| p.is_file())
}

/// Um arquivo `name` em alguma subpasta de `dir` (até dois níveis: `clangd_23.1.0/bin`).
fn find_below(dir: &Path, name: &str, depth: usize) -> Option<PathBuf> {
    let direct = dir.join(name);
    if direct.is_file() {
        return Some(direct);
    }
    if depth == 0 {
        return None;
    }
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    entries.sort();
    entries.into_iter().rev().find_map(|d| find_below(&d, name, depth - 1))
}

/// O servidor da linguagem, se estiver instalado. `code_dir` é a pasta do código do projeto.
pub fn server(language: Language, code_dir: &Path) -> Option<Server> {
    let t = tools();
    let s = match language {
        Language::Rust => {
            // O do rustup (rust-analyzer do toolchain), o da pasta de ferramentas ou o do PATH.
            let from_rustup = crate::env::output("rustup", &["which", "rust-analyzer"]).map(|p| PathBuf::from(p.trim())).filter(|p| p.is_file());
            let program = from_rustup.or_else(|| first(t.iter().map(|d| d.join("rust-analyzer").join(exe("rust-analyzer")))))?;
            Server { name: "rust-analyzer", program, args: Vec::new(), env: Vec::new(), language_id: "rust", init_options: json!({}) }
        }
        Language::Go => {
            let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from);
            let program = first(t.iter().map(|d| d.join("gopls").join(exe("gopls"))).chain(home.map(|h| h.join("go").join("bin").join(exe("gopls")))))
                .or_else(|| crate::hostlang::which(&exe("gopls")))?;
            Server { name: "gopls", program, args: Vec::new(), env: Vec::new(), language_id: "go", init_options: json!({}) }
        }
        Language::Python => {
            let dir = t.as_ref()?.join("pylsp");
            if !dir.join("pylsp").is_dir() {
                return None;
            }
            let (python, _) = crate::env::python()?;
            // `python -m` põe a pasta atual na frente da busca de módulos: um `pylsp.py` ou
            // `json.py` do projeto rodaria no lugar do verdadeiro. O PYTHONSAFEPATH (3.11+)
            // tira essa pasta, e o servidor roda na pasta dele (veja `Client::start`).
            Server {
                name: "pylsp",
                program: PathBuf::from(python),
                args: vec!["-m".into(), "pylsp".into()],
                env: vec![("PYTHONPATH".into(), dir.display().to_string()), ("PYTHONSAFEPATH".into(), "1".into())],
                language_id: "python",
                init_options: json!({}),
            }
        }
        Language::TypeScript => {
            let cli = t.as_ref()?.join("tsls").join("node_modules").join("typescript-language-server").join("lib").join("cli.mjs");
            if !cli.is_file() {
                return None;
            }
            let (node, _) = crate::hostlang::node().ok()?;
            let tsdk = t.as_ref()?.join("tsls").join("node_modules").join("typescript").join("lib").join("tsserver.js");
            Server {
                name: "typescript-language-server",
                program: node,
                args: vec![cli.display().to_string(), "--stdio".into()],
                env: Vec::new(),
                language_id: "typescript",
                init_options: json!({ "tsserver": { "path": tsdk.display().to_string() } }),
            }
        }
        Language::CSharp => {
            let program = first(t.iter().map(|d| d.join("csharp-ls").join(exe("csharp-ls")))).or_else(|| crate::hostlang::which(&exe("csharp-ls")))?;
            Server { name: "csharp-ls", program, args: Vec::new(), env: Vec::new(), language_id: "csharp", init_options: json!({}) }
        }
        Language::Java => {
            let dir = t.as_ref()?.join("jdtls");
            let launcher = std::fs::read_dir(dir.join("plugins"))
                .ok()?
                .flatten()
                .map(|e| e.path())
                .find(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("org.eclipse.equinox.launcher_")))?;
            let java = crate::env::java_bin()?.join(exe("java"));
            let config = dir.join(if cfg!(windows) { "config_win" } else if cfg!(target_os = "macos") { "config_mac" } else { "config_linux" });
            // Os dados do Eclipse por projeto (um índice por pasta).
            let data = t.as_ref()?.join("jdtls-data").join(format!("{:016x}", hash(&code_dir.display().to_string())));
            let args = [
                "-Declipse.application=org.eclipse.jdt.ls.core.id1",
                "-Dosgi.bundles.defaultStartLevel=4",
                "-Declipse.product=org.eclipse.jdt.ls.core.product",
                "-Xmx1G",
                "--add-modules=ALL-SYSTEM",
                "--add-opens",
                "java.base/java.util=ALL-UNNAMED",
                "--add-opens",
                "java.base/java.lang=ALL-UNNAMED",
                "-jar",
            ]
            .iter()
            .map(|s| s.to_string())
            .chain([launcher.display().to_string(), "-configuration".into(), config.display().to_string(), "-data".into(), data.display().to_string()])
            .collect();
            Server { name: "Eclipse JDT LS", program: java, args, env: Vec::new(), language_id: "java", init_options: json!({ "settings": { "java": { "signatureHelp": { "enabled": true } } } }) }
        }
        Language::Cpp => {
            let program = t
                .as_ref()
                .and_then(|d| find_below(&d.join("clangd"), &exe("clangd"), 2))
                .or_else(|| crate::hostlang::which(&exe("clangd")))
                .or_else(|| first([PathBuf::from("C:/Program Files/LLVM/bin/clangd.exe")]))?;
            Server {
                name: "clangd",
                program,
                args: vec!["--background-index".into(), "--header-insertion=never".into(), "--log=error".into()],
                env: Vec::new(),
                language_id: "cpp",
                init_options: json!({}),
            }
        }
        Language::Pascal => {
            let program = first(t.iter().map(|d| d.join("pasls").join(exe("pasls"))))?;
            let lazarus = lazarus_dir();
            // O FPC do Lazarus (64 bits, junto dos fontes do RTL que o pasls lê); senão, o do
            // projeto (o de 32 bits não responde pelo alvo win64 e o RTL não é achado).
            let fpc = lazarus.as_ref().and_then(|l| find_below(&l.join("fpc"), &exe("fpc"), 3)).or_else(|| crate::hostlang::find(Language::Pascal).ok().map(|tc| tc.program));
            let mut env = vec![("FPCTARGET".to_string(), if cfg!(windows) { "win64".to_string() } else { std::env::consts::OS.to_string() }), ("FPCTARGETCPU".into(), "x86_64".into())];
            if let Some(fpc) = &fpc {
                env.push(("PP".into(), fpc.display().to_string()));
            }
            if let Some(src) = fpc_source_dir() {
                env.push(("FPCDIR".into(), src.display().to_string()));
            }
            if let Some(l) = &lazarus {
                env.push(("LAZARUSDIR".into(), l.display().to_string()));
            }
            // As units do projeto (relativas à pasta do código, uma por linha): `src` e as
            // pastas de dentro (o pasls da Castle Engine lê o `projectSearchPaths`).
            let mut dirs = vec!["src".to_string()];
            if let Ok(entries) = std::fs::read_dir(code_dir.join("src")) {
                let mut subs: Vec<String> = entries.flatten().filter(|e| e.path().is_dir()).map(|e| format!("src/{}", e.file_name().to_string_lossy())).collect();
                subs.sort();
                dirs.extend(subs);
            }
            // O PP, o FPCDIR e o LAZARUSDIR vão só no ambiente: no `initializationOptions` o
            // pasls os junta com a pasta do projeto.
            let options = json!({ "projectSearchPaths": dirs.join("\n"), "syntaxErrorReportingMode": 2 });
            Server { name: "pasls", program, args: Vec::new(), env, language_id: "pascal", init_options: options }
        }
    };
    Some(s)
}

/// A pasta do Lazarus (onde o instalador põe), para o pasls achar o CodeTools e o FPC.
pub fn lazarus_dir() -> Option<PathBuf> {
    std::env::var_os("LAZARUSDIR").map(PathBuf::from).filter(|p| p.is_dir()).or_else(|| first_dir(["C:/lazarus", "C:/Program Files/Lazarus", "/usr/lib/lazarus", "/usr/share/lazarus"]))
}

/// Os fontes do Free Pascal (o pasls lê as units do RTL): os do Lazarus ou os do FPC.
pub fn fpc_source_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("FPCDIR").map(PathBuf::from).filter(|p| p.is_dir()) {
        return Some(dir);
    }
    let lazarus = lazarus_dir()?;
    let fpc = lazarus.join("fpc");
    let mut versions: Vec<PathBuf> = std::fs::read_dir(&fpc).ok()?.flatten().map(|e| e.path().join("source")).filter(|p| p.is_dir()).collect();
    versions.sort();
    versions.pop()
}

fn first_dir<const N: usize>(paths: [&str; N]) -> Option<PathBuf> {
    paths.into_iter().map(PathBuf::from).find(|p| p.is_dir())
}

fn hash(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut h);
    h.finish()
}

/// O que cada linguagem precisa na pasta do código antes do servidor abrir: o
/// `compile_commands.json` do clangd (os fontes e a pasta das units); o `.project` e o
/// `.classpath` do jdtls (a pasta `src` é a dos fontes) quando não há Maven nem Gradle.
pub fn prepare(language: Language, code_dir: &Path) {
    if language == Language::Java {
        let build = ["pom.xml", "build.gradle", "build.gradle.kts", ".classpath"].iter().any(|f| code_dir.join(f).exists());
        if !build {
            let name = code_dir.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "app".into());
            let project = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<projectDescription>\n\t<name>{name}</name>\n\t<buildSpec>\n\t\t<buildCommand><name>org.eclipse.jdt.core.javabuilder</name></buildCommand>\n\t</buildSpec>\n\t<natures><nature>org.eclipse.jdt.core.javanature</nature></natures>\n</projectDescription>\n"
            );
            let classpath = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<classpath>\n\t<classpathentry kind=\"src\" path=\"src\"/>\n\t<classpathentry kind=\"con\" path=\"org.eclipse.jdt.launching.JRE_CONTAINER\"/>\n\t<classpathentry kind=\"output\" path=\"out\"/>\n</classpath>\n";
            let _ = std::fs::write(code_dir.join(".project"), project);
            let _ = std::fs::write(code_dir.join(".classpath"), classpath);
        }
        return;
    }
    if language != Language::Cpp {
        return;
    }
    let src = code_dir.join("src");
    let mut files = Vec::new();
    collect_ext(&src, &["cpp", "cc", "cxx"], &mut files);
    let entries: Vec<Value> = files
        .iter()
        .map(|f| {
            json!({
                "directory": code_dir.display().to_string(),
                "file": f.display().to_string(),
                "arguments": ["clang++", "-std=c++20", "-I", src.display().to_string(), "-I", src.join("forms").display().to_string(), "-c", f.display().to_string()],
            })
        })
        .collect();
    let _ = std::fs::write(code_dir.join("compile_commands.json"), serde_json::to_string_pretty(&entries).unwrap_or_default());
}

fn collect_ext(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect_ext(&p, exts, out);
        } else if p.extension().and_then(|x| x.to_str()).is_some_and(|x| exts.contains(&x)) {
            out.push(p);
        }
    }
}

// ---------------------------------------------------------------------------
// Caminhos e posições

/// `C:\pasta\a b.rs` → `file:///C:/pasta/a%20b.rs`.
pub fn uri(path: &Path) -> String {
    let p = path.display().to_string().replace('\\', "/");
    let p = p.trim_start_matches("//?/");
    let mut out = String::from(if p.starts_with('/') { "file://" } else { "file:///" });
    for b in p.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' | b':' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// O caminho de um `file://` (`None` se não é arquivo).
pub fn path_of(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let bytes = rest.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    let s = String::from_utf8_lossy(&out).to_string();
    // Windows: "/C:/x" → "C:/x".
    let s = if s.len() > 2 && s.as_bytes()[0] == b'/' && s.as_bytes()[2] == b':' { s[1..].to_string() } else { s };
    Some(PathBuf::from(if cfg!(windows) { s.replace('/', "\\") } else { s }))
}

/// Uma posição do LSP: linha (a partir de 0) e coluna em unidades UTF-16.
pub type Pos = (usize, usize);

fn pos(v: &Value) -> Pos {
    (v["line"].as_u64().unwrap_or(0) as usize, v["character"].as_u64().unwrap_or(0) as usize)
}

fn range(v: &Value) -> (Pos, Pos) {
    (pos(&v["start"]), pos(&v["end"]))
}

/// A posição de um índice (em caracteres) no texto.
pub fn position_in(text: &str, index: usize) -> Pos {
    let (mut line, mut col) = (0, 0);
    for (i, c) in text.chars().enumerate() {
        if i == index {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += c.len_utf16();
        }
    }
    (line, col)
}

/// O índice (em caracteres) de uma posição no texto.
pub fn index_in(text: &str, (line, column): Pos) -> usize {
    let mut index = 0;
    for (i, l) in text.split('\n').enumerate() {
        if i == line {
            let mut units = 0;
            for (n, c) in l.chars().enumerate() {
                if units >= column {
                    return index + n;
                }
                units += c.len_utf16();
            }
            return index + l.chars().count();
        }
        index += l.chars().count() + 1;
    }
    text.chars().count()
}

fn position_json((line, character): Pos) -> Value {
    json!({ "line": line, "character": character })
}

/// Os parâmetros de um pedido num ponto do documento.
pub fn at(file: &Path, p: Pos) -> Value {
    json!({ "textDocument": { "uri": uri(file) }, "position": position_json(p) })
}

// ---------------------------------------------------------------------------
// O cliente

/// O que chegou do servidor.
#[derive(Clone, Debug, PartialEq)]
pub enum Incoming {
    /// A resposta de um pedido (o número que [`Client::request`] devolveu).
    Response { id: i64, result: Result<Value, String> },
    /// Os erros e avisos de um arquivo (substituem os anteriores).
    Diagnostics { file: PathBuf, items: Vec<Diag> },
    /// Uma mensagem para o usuário (`window/showMessage`).
    Message(String),
    /// O trabalho em andamento do servidor mudou (o `$/progress`: indexar, carregar o
    /// projeto); veja [`Client::work`].
    Progress,
}

/// Quem acorda a IDE quando chega alguma coisa.
pub type Wake = Arc<dyn Fn() + Send + Sync>;

pub struct Client {
    pub server: Server,
    child: Child,
    writer: Arc<Mutex<ChildStdin>>,
    incoming: Receiver<Value>,
    next_id: i64,
    versions: HashMap<PathBuf, i32>,
    /// O que o servidor sabe fazer (o `capabilities` do `initialize`).
    pub capabilities: Value,
    /// A versão que o servidor disse no `initialize` (o `serverInfo.version`; vazia se não disse).
    pub version: String,
    ended: bool,
    /// O trabalho em andamento que o servidor avisou pelo `$/progress` (rust-analyzer
    /// indexando, o clangd lendo o projeto...), por token, na ordem em que começou.
    work: Vec<(String, Work)>,
}

/// O que mostrar do servidor `name` com o trabalho em andamento `work`: o resumo curto (o
/// primeiro trabalho, com a porcentagem) e o detalhe de todos (a dica). Sem trabalho, só o
/// nome (pronto).
pub fn work_text(name: &str, work: &[Work]) -> (String, String) {
    let Some(first) = work.first() else { return (name.to_string(), String::new()) };
    let line = |w: &Work| {
        let mut s = if w.title.is_empty() { "carregando".to_string() } else { w.title.clone() };
        if let Some(p) = w.percentage {
            s.push_str(&format!(" {p}%"));
        }
        s
    };
    let short = format!("{name}: {}…", line(first));
    let detail = work.iter().map(|w| if w.message.is_empty() { line(w) } else { format!("{} — {}", line(w), w.message) }).collect::<Vec<_>>().join("\n");
    (short, format!("{name} ainda está preparando o projeto; ir para a declaração, as dicas e as sugestões ficam completas quando terminar.\n{detail}"))
}

/// Um trabalho em andamento do servidor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Work {
    pub title: String,
    pub message: String,
    /// De 0 a 100, quando o servidor diz.
    pub percentage: Option<u32>,
}

fn write_message(writer: &mut impl Write, value: &Value) -> std::io::Result<()> {
    let body = value.to_string();
    if trace() {
        eprintln!("lsp → {}", body.chars().take(400).collect::<String>());
    }
    write!(writer, "Content-Length: {}\r\n\r\n{body}", body.len())?;
    writer.flush()
}

/// `AYLIS_LSP_TRACE=1`: cada mensagem vai para o stderr.
fn trace() -> bool {
    std::env::var_os("AYLIS_LSP_TRACE").is_some()
}

fn read_message(reader: &mut impl BufRead) -> Option<Value> {
    let mut length = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(v) = line.strip_prefix("Content-Length:") {
            length = v.trim().parse::<usize>().ok();
        }
    }
    let mut body = vec![0; length?];
    reader.read_exact(&mut body).ok()?;
    if trace() {
        eprintln!("lsp ← {}", String::from_utf8_lossy(&body).chars().take(400).collect::<String>());
    }
    serde_json::from_slice(&body).ok()
}

impl Client {
    /// Abre o servidor na pasta `root` e faz o `initialize` (espera até 90 s: o Java e o
    /// Rust indexam o projeto). Rode numa thread: bloqueia.
    pub fn start(server: &Server, root: &Path, wake: Option<Wake>) -> Result<Client, String> {
        let mut command = Command::new(&server.program);
        // O pylsp roda na pasta dele, não na do projeto (o Python antes do 3.11 não tem o
        // PYTHONSAFEPATH); o projeto vai pelo `rootUri` do protocolo.
        let cwd = match server.name {
            "pylsp" => server.env.iter().find(|(k, _)| k == "PYTHONPATH").map(|(_, v)| PathBuf::from(v)).unwrap_or_else(|| root.to_path_buf()),
            _ => root.to_path_buf(),
        };
        command.args(&server.args).current_dir(cwd).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(if trace() { Stdio::inherit() } else { Stdio::null() });
        for (k, v) in &server.env {
            command.env(k, v);
        }
        crate::process::quiet(&mut command);
        let mut child = command.spawn().map_err(|e| format!("não consegui abrir o {}: {e}", server.name))?;
        crate::process::kill_with_parent(&child);
        let stdin = child.stdin.take().ok_or("sem stdin")?;
        let stdout = child.stdout.take().ok_or("sem stdout")?;
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new()
            .name(format!("lsp-{}", server.name))
            .spawn(move || {
                let mut reader = BufReader::new(stdout);
                while let Some(msg) = read_message(&mut reader) {
                    if tx.send(msg).is_err() {
                        break;
                    }
                    if let Some(w) = &wake {
                        w();
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        let mut client = Client {
            server: server.clone(),
            child,
            writer: Arc::new(Mutex::new(stdin)),
            incoming: rx,
            next_id: 1,
            versions: HashMap::new(),
            capabilities: Value::Null,
            version: String::new(),
            ended: false,
            work: Vec::new(),
        };
        let id = client.request("initialize", initialize_params(server, root))?;
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let msg = match client.incoming.recv_timeout(left) {
                Ok(m) => m,
                Err(RecvTimeoutError::Timeout) => return Err(format!("o {} não respondeu ao initialize", server.name)),
                Err(RecvTimeoutError::Disconnected) => return Err(format!("o {} fechou ao abrir", server.name)),
            };
            if msg.get("id").and_then(Value::as_i64) == Some(id) && msg.get("method").is_none() {
                if let Some(e) = msg.get("error") {
                    return Err(format!("{}: {}", server.name, e["message"].as_str().unwrap_or("erro no initialize")));
                }
                client.capabilities = msg["result"]["capabilities"].clone();
                client.version = msg["result"]["serverInfo"]["version"].as_str().unwrap_or_default().to_string();
                break;
            }
            if msg.get("method").is_some() && msg.get("id").is_some() {
                client.answer(&msg);
            }
        }
        client.notify("initialized", json!({}))?;
        Ok(client)
    }

    fn send(&mut self, value: &Value) -> Result<(), String> {
        let mut w = self.writer.lock().map_err(|_| "servidor ocupado")?;
        write_message(&mut *w, value).map_err(|e| {
            self.ended = true;
            format!("o {} fechou: {e}", self.server.name)
        })
    }

    /// Manda um pedido; a resposta chega em [`Client::poll`] com este número.
    pub fn request(&mut self, method: &str, params: Value) -> Result<i64, String> {
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))?;
        Ok(id)
    }

    pub fn notify(&mut self, method: &str, params: Value) -> Result<(), String> {
        self.send(&json!({ "jsonrpc": "2.0", "method": method, "params": params }))
    }

    /// Responde a um pedido do servidor (configuração, registro de recursos, progresso).
    fn answer(&mut self, msg: &Value) {
        let id = msg["id"].clone();
        let result = match msg["method"].as_str().unwrap_or_default() {
            "workspace/configuration" => Value::Array(msg["params"]["items"].as_array().map_or(Vec::new(), |items| items.iter().map(|_| Value::Null).collect())),
            "workspace/workspaceFolders" => Value::Null,
            "workspace/applyEdit" => json!({ "applied": false }),
            _ => Value::Null,
        };
        let _ = self.send(&json!({ "jsonrpc": "2.0", "id": id, "result": result }));
    }

    /// O que chegou desde a última vez (não bloqueia).
    pub fn poll(&mut self) -> Vec<Incoming> {
        let mut out = Vec::new();
        loop {
            let msg = match self.incoming.try_recv() {
                Ok(m) => m,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.ended = true;
                    break;
                }
            };
            match (msg.get("method").and_then(Value::as_str), msg.get("id")) {
                (Some(_), Some(_)) => self.answer(&msg),
                (Some("textDocument/publishDiagnostics"), None) => {
                    if let Some(file) = msg["params"]["uri"].as_str().and_then(path_of) {
                        out.push(Incoming::Diagnostics { file, items: diagnostics(&msg["params"]["diagnostics"]) });
                    }
                }
                (Some("window/showMessage"), None) => {
                    if let Some(text) = msg["params"]["message"].as_str() {
                        out.push(Incoming::Message(text.to_string()));
                    }
                }
                (Some("$/progress"), None) => {
                    if self.progress(&msg["params"]) && !out.contains(&Incoming::Progress) {
                        out.push(Incoming::Progress);
                    }
                }
                (None, Some(id)) => {
                    let id = id.as_i64().unwrap_or(-1);
                    let result = match msg.get("error") {
                        Some(e) => Err(e["message"].as_str().unwrap_or("erro").to_string()),
                        None => Ok(msg.get("result").cloned().unwrap_or(Value::Null)),
                    };
                    out.push(Incoming::Response { id, result });
                }
                _ => {}
            }
        }
        out
    }

    /// Um `$/progress` chegou (veja [`Client::apply_progress`]).
    fn progress(&mut self, params: &Value) -> bool {
        Self::apply_progress(&mut self.work, params)
    }

    /// Um `$/progress` (`begin`, `report`, `end`) atualiza o trabalho em andamento `work` (por
    /// token). `true` se mudou.
    fn apply_progress(work: &mut Vec<(String, Work)>, params: &Value) -> bool {
        let token = match &params["token"] {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        let value = &params["value"];
        let text = |k: &str| value[k].as_str().map(str::to_string);
        let percentage = value["percentage"].as_f64().map(|p| p.clamp(0.0, 100.0) as u32);
        let at = work.iter().position(|(t, _)| *t == token);
        match (value["kind"].as_str(), at) {
            (Some("begin"), _) => {
                let item = Work { title: text("title").unwrap_or_default(), message: text("message").unwrap_or_default(), percentage };
                match at {
                    Some(i) => work[i].1 = item,
                    None => work.push((token, item)),
                }
            }
            (Some("report"), Some(i)) => {
                let w = &mut work[i].1;
                let before = w.clone();
                if let Some(m) = text("message") {
                    w.message = m;
                }
                if percentage.is_some() {
                    w.percentage = percentage;
                }
                return *w != before;
            }
            (Some("end"), Some(i)) => {
                work.remove(i);
            }
            _ => return false,
        }
        true
    }

    /// O trabalho em andamento do servidor (o mais antigo primeiro); vazio: pronto.
    pub fn work(&self) -> Vec<Work> {
        self.work.iter().map(|(_, w)| w.clone()).collect()
    }

    /// O servidor ainda está de pé?
    pub fn alive(&mut self) -> bool {
        !self.ended && self.child.try_wait().ok().flatten().is_none()
    }

    /// Abriu um arquivo no editor.
    pub fn open(&mut self, file: &Path, text: &str) -> Result<(), String> {
        let key = file.to_path_buf();
        if self.versions.contains_key(&key) {
            return self.change(file, text);
        }
        self.versions.insert(key, 1);
        let id = language_id(file, self.server.language_id);
        self.notify("textDocument/didOpen", json!({ "textDocument": { "uri": uri(file), "languageId": id, "version": 1, "text": text } }))
    }

    /// O texto mudou (manda o texto inteiro).
    pub fn change(&mut self, file: &Path, text: &str) -> Result<(), String> {
        let Some(v) = self.versions.get_mut(file) else { return self.open(file, text) };
        *v += 1;
        let version = *v;
        self.notify("textDocument/didChange", json!({ "textDocument": { "uri": uri(file), "version": version }, "contentChanges": [{ "text": text }] }))
    }

    pub fn save(&mut self, file: &Path, text: &str) -> Result<(), String> {
        self.notify("textDocument/didSave", json!({ "textDocument": { "uri": uri(file) }, "text": text }))
    }

    pub fn close(&mut self, file: &Path) -> Result<(), String> {
        if self.versions.remove(file).is_none() {
            return Ok(());
        }
        self.notify("textDocument/didClose", json!({ "textDocument": { "uri": uri(file) } }))
    }

    /// Está aberto no servidor?
    pub fn is_open(&self, file: &Path) -> bool {
        self.versions.contains_key(file)
    }

    /// O servidor sabe fazer `capability` (`definitionProvider`, `renameProvider`...)?
    pub fn supports(&self, capability: &str) -> bool {
        match self.capabilities.get(capability) {
            None | Some(Value::Null) | Some(Value::Bool(false)) => false,
            Some(_) => true,
        }
    }

    /// Fecha o servidor (`shutdown` e `exit`; se não sair, mata).
    pub fn stop(&mut self) {
        if !self.ended {
            let _ = self.request("shutdown", Value::Null);
            std::thread::sleep(Duration::from_millis(100));
            let _ = self.notify("exit", Value::Null);
        }
        self.ended = true;
        let deadline = Instant::now() + Duration::from_millis(800);
        while Instant::now() < deadline {
            if self.child.try_wait().ok().flatten().is_some() {
                return;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        let _ = self.child.kill();
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}

/// O `languageId` pela extensão (o TypeScript tem `.tsx`, o C++ tem `.c`).
fn language_id(file: &Path, default: &'static str) -> &'static str {
    match file.extension().and_then(|e| e.to_str()).unwrap_or_default() {
        "tsx" => "typescriptreact",
        "js" | "mjs" => "javascript",
        "c" => "c",
        _ => default,
    }
}

fn initialize_params(server: &Server, root: &Path) -> Value {
    json!({
        "processId": std::process::id(),
        "clientInfo": { "name": "Aylis IDE", "version": env!("CARGO_PKG_VERSION") },
        "locale": "pt-BR",
        "rootUri": uri(root),
        "rootPath": root.display().to_string(),
        "workspaceFolders": [{ "uri": uri(root), "name": root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default() }],
        "initializationOptions": server.init_options,
        "capabilities": {
            "general": { "positionEncodings": ["utf-16"] },
            "workspace": {
                "applyEdit": false,
                "workspaceEdit": { "documentChanges": true },
                "configuration": true,
                "workspaceFolders": true,
                "symbol": { "symbolKind": { "valueSet": (1..=26).collect::<Vec<u32>>() } },
                "didChangeConfiguration": { "dynamicRegistration": false },
            },
            "textDocument": {
                "synchronization": { "didSave": true, "dynamicRegistration": false },
                "completion": {
                    "completionItem": { "snippetSupport": false, "documentationFormat": ["plaintext", "markdown"], "insertReplaceSupport": false, "labelDetailsSupport": true },
                    "completionItemKind": { "valueSet": (1..=25).collect::<Vec<u32>>() },
                    "contextSupport": true
                },
                "hover": { "contentFormat": ["plaintext", "markdown"] },
                "signatureHelp": { "signatureInformation": { "documentationFormat": ["plaintext", "markdown"], "parameterInformation": { "labelOffsetSupport": true } } },
                "definition": { "linkSupport": true },
                "declaration": { "linkSupport": true },
                "typeDefinition": { "linkSupport": true },
                "implementation": { "linkSupport": true },
                "references": {},
                "documentHighlight": {},
                "documentSymbol": { "hierarchicalDocumentSymbolSupport": true, "symbolKind": { "valueSet": (1..=26).collect::<Vec<u32>>() } },
                "formatting": {},
                "rangeFormatting": {},
                "rename": { "prepareSupport": false },
                "codeAction": { "codeActionLiteralSupport": { "codeActionKind": { "valueSet": ["", "quickfix", "refactor", "refactor.extract", "refactor.inline", "refactor.rewrite", "source", "source.organizeImports"] } } },
                "publishDiagnostics": { "relatedInformation": false }
            },
            "window": { "workDoneProgress": true, "showMessage": {} }
        }
    })
}

// ---------------------------------------------------------------------------
// As respostas, em estruturas simples

/// Um lugar num arquivo.
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub file: PathBuf,
    pub start: Pos,
    pub end: Pos,
}

/// `Location`, `Location[]` ou `LocationLink[]` (definição, referências...).
pub fn locations(value: &Value) -> Vec<Location> {
    let one = |v: &Value| -> Option<Location> {
        let uri = v.get("uri").or_else(|| v.get("targetUri"))?.as_str()?;
        let r = v.get("range").or_else(|| v.get("targetSelectionRange")).or_else(|| v.get("targetRange"))?;
        let (start, end) = range(r);
        Some(Location { file: path_of(uri)?, start, end })
    };
    match value {
        Value::Array(list) => list.iter().filter_map(one).collect(),
        Value::Object(_) => one(value).into_iter().collect(),
        _ => Vec::new(),
    }
}

/// Uma sugestão do autocompletar.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Completion {
    pub label: String,
    pub detail: String,
    pub insert: String,
    pub kind: &'static str,
    /// O trecho que a sugestão troca (do `textEdit`), se veio.
    pub range: Option<(Pos, Pos)>,
}

/// As sugestões (`CompletionItem[]` ou `CompletionList`), sem os marcadores dos snippets.
pub fn completions(value: &Value) -> Vec<Completion> {
    let items = value.get("items").unwrap_or(value).as_array().cloned().unwrap_or_default();
    let mut out: Vec<(String, Completion)> = items
        .iter()
        .map(|i| {
            let label = i["label"].as_str().unwrap_or_default().to_string();
            let edit = i.get("textEdit");
            let text = edit.and_then(|e| e["newText"].as_str()).or_else(|| i["insertText"].as_str()).unwrap_or(&label).to_string();
            let snippet = i["insertTextFormat"].as_u64() == Some(2);
            let insert = if snippet { strip_snippet(&text) } else { text };
            let detail = i["labelDetails"]["detail"].as_str().or_else(|| i["detail"].as_str()).unwrap_or_default().trim().to_string();
            let range = edit.and_then(|e| e.get("range").or_else(|| e.get("insert"))).map(range);
            let sort = i["sortText"].as_str().unwrap_or(&label).to_string();
            (sort, Completion { label, detail, insert, kind: completion_kind(i["kind"].as_u64().unwrap_or(1)), range })
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.into_iter().map(|(_, c)| c).collect()
}

/// `foo(${1:x}, $2)$0` → `foo(`: o texto sem os lugares do snippet (o cursor fica no fim).
fn strip_snippet(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            }
            '$' => {
                if chars.peek() == Some(&'{') {
                    // ${1:padrão} → para no primeiro lugar: o resto é o usuário que escreve.
                    return out;
                }
                while chars.peek().is_some_and(char::is_ascii_digit) {
                    chars.next();
                }
                // $1 no meio (entre parênteses): o cursor ia para lá; o que vem depois sai.
                if chars.peek().is_some() && !out.ends_with('(') {
                    continue;
                }
                return out;
            }
            c => out.push(c),
        }
    }
    out
}

fn completion_kind(k: u64) -> &'static str {
    match k {
        2 => "method",
        3 => "function",
        4 => "constructor",
        5 => "field",
        6 => "variable",
        7 => "class",
        8 => "interface",
        9 => "module",
        10 => "property",
        11 => "unit",
        13 => "enum",
        14 => "keyword",
        15 => "snippet",
        17 => "file",
        20 => "enum",
        21 => "constant",
        22 => "struct",
        25 => "type",
        _ => "text",
    }
}

/// A dica (`Hover`) em Markdown, como o servidor mandou: o `MarkupContent` em markdown fica
/// como está; o de texto puro e o `MarkedString` com linguagem viram blocos de código (a
/// linguagem `text`, para o texto puro, fica sem cor); numa lista, cada parte é separada por
/// uma linha (`---`), como no VS Code.
pub fn hover_markdown(value: &Value) -> String {
    fn fence(lang: &str, code: &str) -> String {
        // A cerca mais longa que as crases de dentro (um código com ``` não fecha antes).
        let mut ticks = "```".to_string();
        while code.contains(ticks.as_str()) {
            ticks.push('`');
        }
        format!("{ticks}{lang}\n{}\n{ticks}", code.trim_end_matches('\n'))
    }
    fn part(v: &Value) -> String {
        match v {
            Value::String(s) => s.clone(),
            Value::Object(o) => {
                let value = o.get("value").and_then(Value::as_str).unwrap_or_default();
                match (o.get("language").and_then(Value::as_str), o.get("kind").and_then(Value::as_str)) {
                    (Some(lang), _) => fence(lang, value),
                    (None, Some("plaintext")) => fence("text", value),
                    _ => value.to_string(),
                }
            }
            Value::Array(a) => a.iter().map(part).filter(|s| !s.trim().is_empty()).collect::<Vec<_>>().join("\n\n---\n\n"),
            _ => String::new(),
        }
    }
    part(&value["contents"]).trim().to_string()
}

/// O texto da dica (`Hover`): `MarkupContent`, `MarkedString` ou uma lista deles, sem as
/// cercas de código do markdown.
pub fn hover_text(value: &Value) -> String {
    fn part(v: &Value) -> String {
        match v {
            Value::String(s) => s.clone(),
            Value::Object(o) => o.get("value").and_then(Value::as_str).unwrap_or_default().to_string(),
            Value::Array(a) => a.iter().map(part).filter(|s| !s.trim().is_empty()).collect::<Vec<_>>().join("\n\n"),
            _ => String::new(),
        }
    }
    let text = part(&value["contents"]);
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim_end();
        if t.starts_with("```") {
            continue;
        }
        out.push(t.replace("\\_", "_").replace("&nbsp;", " "));
    }
    let joined = out.join("\n");
    let mut clean = joined.trim().to_string();
    while clean.contains("\n\n\n") {
        clean = clean.replace("\n\n\n", "\n\n");
    }
    clean
}

/// A ajuda dos parâmetros: a assinatura ativa, com o parâmetro atual entre ‹ ›.
pub fn signature_text(value: &Value) -> String {
    let Some(sigs) = value["signatures"].as_array().filter(|s| !s.is_empty()) else { return String::new() };
    let active = value["activeSignature"].as_u64().unwrap_or(0) as usize;
    let sig = &sigs[active.min(sigs.len() - 1)];
    let label = sig["label"].as_str().unwrap_or_default().to_string();
    let param = sig["activeParameter"].as_u64().or_else(|| value["activeParameter"].as_u64()).unwrap_or(0) as usize;
    let mut text = label.clone();
    if let Some(p) = sig["parameters"].as_array().and_then(|ps| ps.get(param)) {
        match &p["label"] {
            Value::String(s) if !s.is_empty() => {
                if let Some(i) = label.find(s.as_str()) {
                    text = format!("{}‹{}›{}", &label[..i], s, &label[i + s.len()..]);
                }
            }
            Value::Array(off) if off.len() == 2 => {
                // Posições em UTF-16 dentro do rótulo.
                let (a, b) = (off[0].as_u64().unwrap_or(0) as usize, off[1].as_u64().unwrap_or(0) as usize);
                let (mut ia, mut ib) = (index_in(&label, (0, a)), index_in(&label, (0, b)));
                let chars: Vec<char> = label.chars().collect();
                // Servidor que conta uma posição a mais (o pasls): o trecho começaria no meio
                // de um nome; volta um.
                let word = |c: char| c.is_alphanumeric() || c == '_';
                if ia > 0 && ia <= ib && ib <= chars.len() && ia < chars.len() && word(chars[ia - 1]) && word(chars[ia]) {
                    ia -= 1;
                    ib -= 1;
                }
                // Sem o separador do fim (`, ` ou `; `).
                while ib > ia && matches!(chars.get(ib - 1), Some(',' | ';' | ' ')) {
                    ib -= 1;
                }
                if ia <= ib && ib <= chars.len() {
                    text = format!("{}‹{}›{}", chars[..ia].iter().collect::<String>(), chars[ia..ib].iter().collect::<String>(), chars[ib..].iter().collect::<String>());
                }
            }
            _ => {}
        }
    }
    let doc = match &sig["documentation"] {
        Value::String(s) => s.clone(),
        Value::Object(o) => o.get("value").and_then(Value::as_str).unwrap_or_default().to_string(),
        _ => String::new(),
    };
    if doc.trim().is_empty() { text } else { format!("{text}\n{}", doc.trim()) }
}

/// Um erro ou aviso.
#[derive(Clone, Debug, PartialEq)]
pub struct Diag {
    pub start: Pos,
    pub end: Pos,
    /// 1 erro, 2 aviso, 3 informação, 4 dica.
    pub severity: u8,
    pub message: String,
    pub source: String,
}

pub fn diagnostics(value: &Value) -> Vec<Diag> {
    value
        .as_array()
        .map(|list| {
            list.iter()
                .map(|d| {
                    let (start, end) = range(&d["range"]);
                    Diag {
                        start,
                        end,
                        severity: d["severity"].as_u64().unwrap_or(1) as u8,
                        message: d["message"].as_str().unwrap_or_default().to_string(),
                        source: d["source"].as_str().unwrap_or_default().to_string(),
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Uma troca de texto.
#[derive(Clone, Debug, PartialEq)]
pub struct TextEdit {
    pub start: Pos,
    pub end: Pos,
    pub text: String,
}

pub fn text_edits(value: &Value) -> Vec<TextEdit> {
    value
        .as_array()
        .map(|list| {
            list.iter()
                .map(|e| {
                    let (start, end) = range(&e["range"]);
                    TextEdit { start, end, text: e["newText"].as_str().unwrap_or_default().to_string() }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// As trocas de um `WorkspaceEdit`, por arquivo (`changes` ou `documentChanges`).
pub fn workspace_edit(value: &Value) -> Vec<(PathBuf, Vec<TextEdit>)> {
    let mut out: Vec<(PathBuf, Vec<TextEdit>)> = Vec::new();
    if let Some(changes) = value["changes"].as_object() {
        for (uri, edits) in changes {
            if let Some(file) = path_of(uri) {
                out.push((file, text_edits(edits)));
            }
        }
    }
    if let Some(list) = value["documentChanges"].as_array() {
        for change in list {
            if let Some(file) = change["textDocument"]["uri"].as_str().and_then(path_of) {
                out.push((file, text_edits(&change["edits"])));
            }
        }
    }
    out
}

/// Um símbolo (do documento ou do projeto).
#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: &'static str,
    pub detail: String,
    /// O arquivo (no do documento, vazio: o próprio).
    pub file: Option<PathBuf>,
    pub start: Pos,
    /// Profundidade (os de dentro de uma classe ficam com 1...).
    pub depth: usize,
}

/// `DocumentSymbol[]` (com os filhos) ou `SymbolInformation[]`.
pub fn symbols(value: &Value) -> Vec<Symbol> {
    fn walk(list: &[Value], depth: usize, out: &mut Vec<Symbol>) {
        for s in list {
            let (location_file, start) = match s.get("location") {
                Some(l) => (l["uri"].as_str().and_then(path_of), pos(&l["range"]["start"])),
                None => (None, pos(&s.get("selectionRange").unwrap_or(&s["range"])["start"])),
            };
            out.push(Symbol {
                name: s["name"].as_str().unwrap_or_default().to_string(),
                kind: symbol_kind(s["kind"].as_u64().unwrap_or(0)),
                detail: s["detail"].as_str().or_else(|| s["containerName"].as_str()).unwrap_or_default().to_string(),
                file: location_file,
                start,
                depth,
            });
            if let Some(children) = s["children"].as_array() {
                walk(children, depth + 1, out);
            }
        }
    }
    let mut out = Vec::new();
    if let Some(list) = value.as_array() {
        walk(list, 0, &mut out);
    }
    out
}

fn symbol_kind(k: u64) -> &'static str {
    match k {
        1 => "file",
        2 => "module",
        3 => "namespace",
        4 => "package",
        5 => "class",
        6 => "method",
        7 => "property",
        8 => "field",
        9 => "constructor",
        10 => "enum",
        11 => "interface",
        12 => "function",
        13 => "variable",
        14 => "constant",
        22 => "enum",
        23 => "struct",
        26 => "type",
        _ => "symbol",
    }
}

/// Uma correção (`CodeAction`): o título e as trocas.
#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub title: String,
    pub edits: Vec<(PathBuf, Vec<TextEdit>)>,
    /// Um comando do servidor (`workspace/executeCommand`), quando não vem a troca pronta.
    pub command: Option<Value>,
}

pub fn actions(value: &Value) -> Vec<Action> {
    value
        .as_array()
        .map(|list| {
            list.iter()
                .map(|a| {
                    // Um Command solto (sem edit) ou um CodeAction.
                    let command = match (&a["command"], a.get("arguments")) {
                        (Value::String(name), args) => Some(json!({ "command": name, "arguments": args.cloned().unwrap_or(Value::Null) })),
                        (Value::Object(_), _) => Some(a["command"].clone()),
                        _ => None,
                    };
                    Action { title: a["title"].as_str().unwrap_or_default().to_string(), edits: workspace_edit(&a["edit"]), command }
                })
                .filter(|a| !a.title.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O `$/progress` do servidor (o rust-analyzer indexando): começa, avança com a
    /// porcentagem e termina; o rótulo diz o quê e quanto, e some quando acaba (07/11: "se ele
    /// [é] lento tem que ter um progresso ou informar na IDE que ele ainda não está pronto").
    #[test]
    fn server_progress_shows_what_is_running_and_how_much() {
        let mut work = Vec::new();
        let p = |kind: &str, extra: Value| {
            let mut value = json!({ "kind": kind });
            value.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            json!({ "token": "rustAnalyzer/Indexing", "value": value })
        };
        assert!(Client::apply_progress(&mut work, &p("begin", json!({ "title": "Indexing", "percentage": 0 }))));
        assert!(Client::apply_progress(&mut work, &p("report", json!({ "message": "12/120 (core)", "percentage": 10 }))));
        assert!(!Client::apply_progress(&mut work, &p("report", json!({ "message": "12/120 (core)", "percentage": 10 }))), "nada mudou");
        let list: Vec<Work> = work.iter().map(|(_, w)| w.clone()).collect();
        let (short, hint) = work_text("rust-analyzer", &list);
        assert_eq!(short, "rust-analyzer: Indexing 10%…");
        assert!(hint.contains("12/120 (core)") && hint.contains("ainda está preparando"), "{hint}");
        // Um token numérico, sem porcentagem, ao mesmo tempo.
        let numeric = json!({ "token": 7, "value": { "kind": "begin", "title": "Loading" } });
        assert!(Client::apply_progress(&mut work, &numeric));
        assert_eq!(work.len(), 2);
        assert!(Client::apply_progress(&mut work, &p("end", json!({}))));
        assert!(Client::apply_progress(&mut work, &json!({ "token": 7, "value": { "kind": "end" } })));
        assert!(work.is_empty());
        assert_eq!(work_text("rust-analyzer", &[]), ("rust-analyzer".to_string(), String::new()), "pronto: só o nome");
        assert!(!Client::apply_progress(&mut work, &p("report", json!({ "percentage": 50 }))), "relato de token que não começou");
    }

    #[test]
    fn uris_go_and_come_back() {
        let p = if cfg!(windows) { PathBuf::from(r"C:\Users\Ana Souza\calc.pas") } else { PathBuf::from("/home/ana souza/calc.pas") };
        let u = uri(&p);
        assert!(u.starts_with("file:///") && u.contains("%20"), "{u}");
        assert_eq!(path_of(&u), Some(p));
        assert_eq!(path_of("file:///c%3A/x/y.rs").map(|p| p.display().to_string().to_lowercase().replace('\\', "/")), Some("c:/x/y.rs".into()));
    }

    #[test]
    fn positions_count_utf16() {
        let text = "é\n😀ab";
        assert_eq!(position_in(text, 4), (1, 3));
        assert_eq!(index_in(text, (1, 2)), 3);
    }

    #[test]
    fn responses_become_simple_structures() {
        let c = completions(&json!({ "items": [
            { "label": "println!", "kind": 3, "insertText": "println!(${1:fmt})", "insertTextFormat": 2, "sortText": "b" },
            { "label": "print", "kind": 3, "detail": "fn print()", "sortText": "a" }
        ]}));
        assert_eq!(c[0].label, "print");
        assert_eq!(c[1].insert, "println!(");
        assert_eq!(hover_text(&json!({ "contents": { "kind": "markdown", "value": "```rust\nfn apertar(texto: &str)\n```\nAperta." } })), "fn apertar(texto: &str)\nAperta.");
        assert_eq!(hover_markdown(&json!({ "contents": { "kind": "markdown", "value": "```rust\nfn apertar()\n```\nAperta." } })), "```rust\nfn apertar()\n```\nAperta.");
        assert_eq!(hover_markdown(&json!({ "contents": [{ "language": "go", "value": "func A()" }, "Faz **A**."] })), "```go\nfunc A()\n```\n\n---\n\nFaz **A**.");
        assert_eq!(hover_markdown(&json!({ "contents": { "kind": "plaintext", "value": "int x" } })), "```text\nint x\n```");
        let s = signature_text(&json!({ "signatures": [{ "label": "apertar(aylis, texto)", "parameters": [{ "label": "aylis" }, { "label": "texto" }] }], "activeParameter": 1 }));
        assert_eq!(s, "apertar(aylis, ‹texto›)");
        let edits = workspace_edit(&json!({ "changes": { "file:///C:/a.rs": [{ "range": { "start": { "line": 0, "character": 1 }, "end": { "line": 0, "character": 2 } }, "newText": "x" }] } }));
        assert_eq!(edits[0].1[0].text, "x");
        let syms = symbols(&json!([{ "name": "Calc", "kind": 5, "range": { "start": { "line": 2, "character": 0 } }, "selectionRange": { "start": { "line": 2, "character": 2 } }, "children": [{ "name": "usar", "kind": 6, "selectionRange": { "start": { "line": 3, "character": 4 } } }] }]));
        assert_eq!((syms[1].name.as_str(), syms[1].depth, syms[1].start), ("usar", 1, (3, 4)));
    }
}
