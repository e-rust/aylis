//! Depuração pela IDE com o protocolo de depuração do VS Code (DAP): a IDE é o cliente e
//! um adaptador por linguagem faz o trabalho — o CodeLLDB para Rust (Windows, Linux, macOS)
//! e o `debugpy` para Python.
//!
//! [`Session`] cuida da conversa: inicia (ou conecta), manda os pontos de parada, continua,
//! anda passo a passo e lê a pilha e as variáveis. Os pedidos esperam a resposta; os eventos
//! (`stopped`, `output`, `terminated`) ficam numa fila lida com [`Session::events`].
//!
//! A IDE não conversa direto: [`Session::into_worker`] leva a sessão para uma thread
//! ([`Worker`]), que recebe os pedidos ([`Job`]) e devolve as respostas e os eventos
//! ([`Reply`]). O depurador pode demorar (montar a pilha, formatar variáveis grandes) sem
//! travar a tela.
//!
//! Mensagens: `Content-Length: N\r\n\r\n` + JSON, nos dois sentidos.

use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

/// Quanto um pedido espera a resposta.
const TIMEOUT: Duration = Duration::from_secs(15);

/// Um ponto de parada: a linha (a partir de 1) e, opcionalmente, a condição (só para quando
/// a expressão é verdadeira, como `i == 3`) e a contagem de passagens (`5` para na quinta;
/// `>= 5` da quinta em diante).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Breakpoint {
    pub line: usize,
    pub condition: String,
    pub hit_condition: String,
}

impl Breakpoint {
    pub fn at(line: usize) -> Self {
        Self { line, ..Default::default() }
    }
}

impl From<usize> for Breakpoint {
    fn from(line: usize) -> Self {
        Self::at(line)
    }
}

/// Um quadro da pilha.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub id: i64,
    pub name: String,
    /// O arquivo (sem ele, código sem fonte: a biblioteca padrão, o sistema).
    pub file: Option<PathBuf>,
    /// Linha a partir de 1.
    pub line: usize,
}

/// Uma variável (com `children` > 0, tem campos: [`Session::variables`] com esse número).
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub kind: String,
    pub children: i64,
}

/// O que o programa depurado avisa.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Parou (ponto de parada, passo, pausa, exceção) na thread dada.
    Stopped { thread: i64, reason: String, text: Option<String> },
    Continued,
    /// Saída do programa ou do adaptador.
    Output { category: String, text: String },
    /// Terminou (o código, se o adaptador informou).
    Exited(Option<i64>),
    Terminated,
}

/// Onde mandar as mensagens (stdin do adaptador ou o socket).
type Writer = Arc<Mutex<Box<dyn Write + Send>>>;

/// Acorda quem usa a sessão quando chega uma mensagem (a IDE repinta e lê os eventos).
pub type WakeFn = Arc<dyn Fn() + Send + Sync>;
type WakeSlot = Arc<Mutex<Option<WakeFn>>>;

pub struct Session {
    writer: Writer,
    incoming: Receiver<Value>,
    seq: i64,
    events: VecDeque<Event>,
    child: Option<Child>,
    /// Pontos de parada mandados (arquivo → linhas), para mandar de novo ao mudar.
    breakpoints: BTreeMap<PathBuf, Vec<Breakpoint>>,
    wake: WakeSlot,
    /// O processo depurado (o CodeLLDB no Windows às vezes não avisa que ele terminou:
    /// uma thread vigia e a sessão fecha sozinha).
    debuggee: Option<u32>,
    gone_since: Option<Instant>,
    /// A thread que parou por último (os passos valem para ela).
    pub thread: Option<i64>,
    ended: bool,
    /// A sessão-mãe que abriu esta (o js-debug abre uma sessão por alvo): fica viva junto.
    parent: Option<Box<Session>>,
    /// O adaptador (`lldb`, `go`, `coreclr`...): o CodeLLDB precisa de ajuda com os textos do
    /// Free Pascal.
    adapter: String,
    /// A medição do passo em andamento (veja [`LineTiming`]).
    measure: Option<Measure>,
    /// Quando chegou o último `stopped` (o fim do tempo de relógio do passo).
    stopped_at: Option<Instant>,
    /// A expressão que dá os bytes alocados no programa (o contador do `aylis` com `dev`, no
    /// Rust); `None` se a linguagem não tem ou a leitura falhou.
    pub memory_expression: Option<String>,
}

/// O CodeLLDB que a IDE usa: o da pasta de ferramentas do aylis
/// (`%LOCALAPPDATA%/aylis/tools/codelldb`, `~/.local/share/aylis/tools/codelldb`) ou o da
/// extensão do VS Code.
pub fn codelldb() -> Option<PathBuf> {
    let exe = if cfg!(windows) { "codelldb.exe" } else { "codelldb" };
    let mut dirs = Vec::new();
    if let Some(d) = tools_dir() {
        dirs.push(d.join("codelldb/extension"));
    }
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        let ext = PathBuf::from(home).join(".vscode/extensions");
        if let Ok(list) = std::fs::read_dir(&ext) {
            let mut found: Vec<PathBuf> = list.flatten().map(|e| e.path()).filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("vadimcn.vscode-lldb"))).collect();
            found.sort();
            dirs.extend(found.into_iter().rev());
        }
    }
    dirs.into_iter().map(|d| d.join("adapter").join(exe)).find(|p| p.is_file())
}

/// A pasta de ferramentas baixadas pela IDE.
pub fn tools_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        // Sem o LOCALAPPDATA (um processo aberto com o ambiente reduzido): a pasta padrão dele.
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join("AppData").join("Local")))
            .map(|d| d.join("aylis").join("tools"))
    } else {
        std::env::var_os("HOME").map(|d| PathBuf::from(d).join(".local/share/aylis/tools"))
    }
}

/// `AYLIS_DAP_TRACE=1`: cada mensagem com o depurador vai para o stderr.
fn trace() -> bool {
    std::env::var_os("AYLIS_DAP_TRACE").is_some()
}

/// Lê mensagens `Content-Length` e entrega na fila (numa thread).
fn spawn_reader(input: impl Read + Send + 'static, wake: WakeSlot) -> Receiver<Value> {
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("aylis-dap".into())
        .spawn(move || {
            let mut reader = BufReader::new(input);
            loop {
                let mut length = None;
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) | Err(_) => return,
                        Ok(_) => {}
                    }
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    if let Some(v) = line.strip_prefix("Content-Length:") {
                        length = v.trim().parse::<usize>().ok();
                    }
                }
                let Some(n) = length else { continue };
                let mut body = vec![0; n];
                if reader.read_exact(&mut body).is_err() {
                    return;
                }
                if let Ok(v) = serde_json::from_slice::<Value>(&body) {
                    if trace() {
                        eprintln!("dap ← {}", String::from_utf8_lossy(&body).chars().take(400).collect::<String>());
                    }
                    if tx.send(v).is_err() {
                        return;
                    }
                    if let Some(w) = wake.lock().ok().and_then(|w| w.clone()) {
                        w();
                    }
                }
            }
        })
        .expect("thread do DAP");
    rx
}

/// O formatador mais rápido de HashMap e HashSet (veja `lldb/rust_hashmap.py`).
const RUST_HASHMAP: &str = include_str!("../lldb/rust_hashmap.py");

/// Os comandos que o LLDB roda antes de iniciar o programa: carrega os scripts do aylis
/// (gravados na pasta temporária; sem ela, nada).
fn lldb_scripts() -> Vec<String> {
    let dir = std::env::temp_dir().join("aylis-lldb");
    let file = dir.join("rust_hashmap.py");
    let same = std::fs::read_to_string(&file).is_ok_and(|s| s == RUST_HASHMAP);
    if !same && (std::fs::create_dir_all(&dir).is_err() || std::fs::write(&file, RUST_HASHMAP).is_err()) {
        return Vec::new();
    }
    vec![format!("command script import '{}'", file.display().to_string().replace('\\', "/"))]
}

/// O ambiente do programa depurado: o pedido mais `_NO_DEBUG_HEAP=1`. Iniciado por um
/// depurador, o Windows liga o heap de depuração (confere cada alocação e liberação): um app
/// que aloca o tempo todo fica centenas de vezes mais lento (medido: 200 mil alocações, de
/// 30 ms para 12 s). O Visual Studio desliga do mesmo jeito; quem quiser o heap de depuração
/// põe a variável nas próprias.
fn launch_env(env: &[(String, String)]) -> serde_json::Map<String, Value> {
    let mut map: serde_json::Map<String, Value> = env.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect();
    map.entry("_NO_DEBUG_HEAP").or_insert_with(|| Value::String("1".into()));
    map
}

impl Session {
    fn new(writer: Box<dyn Write + Send>, incoming: Receiver<Value>, child: Option<Child>, wake: WakeSlot) -> Session {
        Session {
            writer: Arc::new(Mutex::new(writer)),
            incoming,
            seq: 1,
            events: VecDeque::new(),
            child,
            breakpoints: BTreeMap::new(),
            wake,
            debuggee: None,
            gone_since: None,
            thread: None,
            ended: false,
            parent: None,
            adapter: String::new(),
            measure: None,
            stopped_at: None,
            memory_expression: None,
        }
    }

    /// Chamada (de outra thread) a cada mensagem do depurador.
    pub fn set_wake(&self, wake: WakeFn) {
        if let Ok(mut w) = self.wake.lock() {
            *w = Some(wake);
        }
    }

    /// Os pontos de parada mandados ao depurador (arquivo → linhas).
    pub fn sent_breakpoints(&self) -> &BTreeMap<PathBuf, Vec<Breakpoint>> {
        &self.breakpoints
    }

    /// Um adaptador que fala por stdin/stdout.
    pub fn spawn(mut command: Command) -> Result<Session, String> {
        let mut child = crate::process::quiet(&mut command).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().map_err(|e| format!("não consegui iniciar o depurador: {e}"))?;
        crate::process::kill_with_parent(&child);
        let stdin = child.stdin.take().ok_or("sem stdin")?;
        let stdout = child.stdout.take().ok_or("sem stdout")?;
        let wake = WakeSlot::default();
        Ok(Session::new(Box::new(stdin), spawn_reader(stdout, wake.clone()), Some(child), wake))
    }

    /// Um adaptador que espera a conexão (o `debugpy --listen`), tentando até `timeout`.
    pub fn connect(port: u16, timeout: Duration) -> Result<Session, String> {
        let deadline = Instant::now() + timeout;
        let stream = loop {
            match TcpStream::connect(("127.0.0.1", port)) {
                Ok(s) => break s,
                Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(100)),
                Err(e) => return Err(format!("o depurador não respondeu na porta {port}: {e}")),
            }
        };
        let reader = stream.try_clone().map_err(|e| e.to_string())?;
        let wake = WakeSlot::default();
        Ok(Session::new(Box::new(stream), spawn_reader(reader, wake.clone()), None, wake))
    }

    fn send(&mut self, message: &Value) -> Result<(), String> {
        let body = message.to_string();
        if trace() {
            eprintln!("dap → {}", body.chars().take(400).collect::<String>());
        }
        let mut w = self.writer.lock().map_err(|_| "depurador ocupado")?;
        write!(w, "Content-Length: {}\r\n\r\n{body}", body.len()).and_then(|_| w.flush()).map_err(|e| format!("depurador: {e}"))
    }

    /// Manda um pedido e espera a resposta dele (os eventos que chegarem antes vão para a
    /// fila). Os pedidos reversos do adaptador (`runInTerminal`) são recusados.
    pub fn request(&mut self, command: &str, arguments: Value) -> Result<Value, String> {
        let seq = self.seq;
        self.seq += 1;
        self.send(&json!({ "seq": seq, "type": "request", "command": command, "arguments": arguments }))?;
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let message = match self.incoming.recv_timeout(left) {
                Ok(m) => m,
                Err(RecvTimeoutError::Timeout) => return Err(format!("o depurador não respondeu ({command})")),
                Err(RecvTimeoutError::Disconnected) => {
                    self.ended = true;
                    return Err("o depurador terminou".into());
                }
            };
            match message.get("type").and_then(Value::as_str) {
                Some("response") if message.get("request_seq").and_then(Value::as_i64) == Some(seq) => {
                    return if message.get("success").and_then(Value::as_bool).unwrap_or(false) {
                        Ok(message.get("body").cloned().unwrap_or(Value::Null))
                    } else {
                        Err(message.get("message").and_then(Value::as_str).unwrap_or("falhou").to_string())
                    };
                }
                Some("event") => self.queue_event(&message),
                Some("request") => self.refuse(&message)?,
                _ => {}
            }
        }
    }

    fn refuse(&mut self, request: &Value) -> Result<(), String> {
        let seq = self.seq;
        self.seq += 1;
        let command = request.get("command").cloned().unwrap_or(Value::Null);
        let reply = json!({ "seq": seq, "type": "response", "request_seq": request.get("seq"), "command": command, "success": false, "message": "não suportado pela IDE do Aylis" });
        self.send(&reply)
    }

    fn queue_event(&mut self, message: &Value) {
        let body = message.get("body").cloned().unwrap_or(Value::Null);
        let text = |k: &str| body.get(k).and_then(Value::as_str).map(str::to_string);
        let event = match message.get("event").and_then(Value::as_str).unwrap_or_default() {
            "stopped" => {
                let thread = body.get("threadId").and_then(Value::as_i64).unwrap_or(0);
                self.thread = Some(thread);
                self.stopped_at = Some(Instant::now());
                Event::Stopped { thread, reason: text("reason").unwrap_or_default(), text: text("text").or_else(|| text("description")) }
            }
            "continued" => Event::Continued,
            "process" => {
                if let Some(pid) = body.get("systemProcessId").and_then(Value::as_u64) {
                    self.watch(pid as u32);
                }
                return;
            }
            "output" => {
                let output = text("output").unwrap_or_default();
                // CodeLLDB: "Launched process 1234 from '...'".
                if let Some(pid) = output.strip_prefix("Launched process ").and_then(|r| r.split_whitespace().next()).and_then(|p| p.parse::<u32>().ok()) {
                    self.watch(pid);
                }
                Event::Output { category: text("category").unwrap_or_else(|| "console".into()), text: output }
            }
            "exited" => Event::Exited(body.get("exitCode").and_then(Value::as_i64)),
            "terminated" => {
                self.ended = true;
                Event::Terminated
            }
            _ => return, // initialized, process, module, thread...: a sessão cuida ou ignora
        };
        self.events.push_back(event);
    }

    /// O que chegou desde a última leitura (sem esperar).
    pub fn events(&mut self) -> Vec<Event> {
        self.events_pending();
        // O programa saiu e o adaptador ficou mudo: a sessão termina por conta própria.
        if let Some(pid) = self.debuggee.filter(|_| !self.ended) {
            if process_alive(pid) {
                self.gone_since = None;
            } else {
                let since = *self.gone_since.get_or_insert_with(Instant::now);
                if since.elapsed() > Duration::from_millis(1500) {
                    self.events.push_back(Event::Exited(None));
                    self.events.push_back(Event::Terminated);
                    self.ended = true;
                    if let Some(mut c) = self.child.take() {
                        let _ = c.kill();
                        let _ = c.wait();
                    }
                }
            }
        }
        if self.child.as_mut().is_some_and(|c| c.try_wait().ok().flatten().is_some()) && !self.ended {
            self.ended = true;
            self.events.push_back(Event::Terminated);
        }
        self.events.drain(..).collect()
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    /// Vigia o processo depurado: quando ele some, acorda quem usa a sessão (que então
    /// confere em [`events`](Self::events)).
    fn watch(&mut self, pid: u32) {
        if self.debuggee.is_some() {
            return;
        }
        self.debuggee = Some(pid);
        let wake = self.wake.clone();
        let _ = std::thread::Builder::new().name("aylis-dap-vigia".into()).spawn(move || {
            while process_alive(pid) {
                std::thread::sleep(Duration::from_millis(300));
            }
            // Dá tempo ao adaptador de avisar; depois acorda duas vezes (antes e depois do prazo).
            for wait in [200, 1600] {
                std::thread::sleep(Duration::from_millis(wait));
                if let Some(w) = wake.lock().ok().and_then(|w| w.clone()) {
                    w();
                }
            }
        });
    }

    /// Espera o evento `initialized` (o adaptador pronto para os pontos de parada).
    fn wait_initialized(&mut self) -> Result<(), String> {
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let message = self.incoming.recv_timeout(left).map_err(|_| "o depurador não ficou pronto")?;
            if message.get("type").and_then(Value::as_str) == Some("event") {
                if message.get("event").and_then(Value::as_str) == Some("initialized") {
                    return Ok(());
                }
                self.queue_event(&message);
            }
        }
    }

    fn initialize(&mut self, adapter: &str) -> Result<(), String> {
        self.adapter = adapter.to_string();
        self.request(
            "initialize",
            json!({ "clientID": "aylis-ide", "clientName": "Aylis IDE", "adapterID": adapter, "linesStartAt1": true, "columnsStartAt1": true,
                    "pathFormat": "path", "locale": "pt-BR" }),
        )?;
        Ok(())
    }

    /// Depois do `launch`/`attach`: os pontos de parada e o "pode começar".
    fn configure(&mut self, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<(), String> {
        self.wait_initialized()?;
        for (file, points) in breakpoints {
            self.set_breakpoints(file, points)?;
        }
        self.request("configurationDone", json!({}))?;
        Ok(())
    }

    /// Rust: o executável (compilado em debug) pelo CodeLLDB.
    pub fn launch_rust(adapter: &Path, exe: &Path, cwd: &Path, env: &[(String, String)], breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let mut command = Command::new(adapter);
        if let Some(lib) = adapter.parent().and_then(Path::parent).map(|e| e.join("lldb/bin").join(if cfg!(windows) { "liblldb.dll" } else { "liblldb.so" })).filter(|p| p.is_file()) {
            command.arg("--liblldb").arg(lib);
        }
        let mut s = Session::spawn(command)?;
        s.initialize("lldb")?;
        let env = launch_env(env);
        // O launch só responde depois do configurationDone: vai sem esperar a resposta.
        let seq = s.seq;
        s.seq += 1;
        s.send(&json!({ "seq": seq, "type": "request", "command": "launch", "arguments": {
            "program": exe, "cwd": cwd, "env": env, "args": [], "stopOnEntry": false, "sourceLanguages": ["rust"], "terminal": "console",
            "preRunCommands": lldb_scripts() } }))?;
        s.configure(breakpoints)?;
        // O app com `dev` conta os bytes alocados (veja `aylis::dev_memory`). Só se o contador
        // está no programa: o CodeLLDB cai ao avaliar um nome que não existe.
        if binary_has_symbol(exe, "AYLIS_DEV_LIVE_BYTES") {
            s.memory_expression = Some("aylis::dev_memory::AYLIS_DEV_LIVE_BYTES".into());
        }
        Ok(s)
    }

    /// Python: conecta no `debugpy --listen <porta> --wait-for-client` do backend.
    pub fn attach_python(port: u16, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let mut s = Session::connect(port, Duration::from_secs(30))?;
        s.initialize("debugpy")?;
        let seq = s.seq;
        s.seq += 1;
        s.send(&json!({ "seq": seq, "type": "request", "command": "attach", "arguments": { "justMyCode": true, "connect": { "host": "127.0.0.1", "port": port } } }))?;
        s.configure(breakpoints)?;
        Ok(s)
    }

    /// Os pontos de parada de um arquivo (troca os anteriores). Devolve as linhas aceitas.
    pub fn set_breakpoints(&mut self, file: &Path, points: &[Breakpoint]) -> Result<Vec<usize>, String> {
        let lines: Vec<usize> = points.iter().map(|b| b.line).collect();
        let json_points: Vec<Value> = points
            .iter()
            .map(|b| {
                let mut p = json!({ "line": b.line });
                if !b.condition.trim().is_empty() {
                    p["condition"] = json!(b.condition.trim());
                }
                if !b.hit_condition.trim().is_empty() {
                    p["hitCondition"] = json!(b.hit_condition.trim());
                }
                p
            })
            .collect();
        // No Windows, só barras invertidas: o netcoredbg compara o caminho com o do PDB.
        let path = if cfg!(windows) { file.to_string_lossy().replace('/', "\\") } else { file.to_string_lossy().into_owned() };
        let body = self.request("setBreakpoints", json!({ "source": { "path": path }, "breakpoints": json_points, "lines": lines }))?;
        self.breakpoints.insert(file.to_path_buf(), points.to_vec());
        Ok(body["breakpoints"].as_array().map(|a| a.iter().filter(|b| b["verified"].as_bool().unwrap_or(false)).filter_map(|b| b["line"].as_u64().map(|l| l as usize)).collect()).unwrap_or_default())
    }

    fn thread_args(&self) -> Value {
        json!({ "threadId": self.thread.unwrap_or(0) })
    }

    /// Manda o programa andar (continuar ou um passo). As paradas que ainda estavam na fila
    /// são de antes (o LLDB às vezes avisa duas vezes a mesma): saem, para a próxima
    /// `Stopped` ser a deste comando.
    ///
    /// Não espera a resposta: o andamento chega pelos eventos (`continued`, `stopped`,
    /// `terminated`), e uma resposta atrasada do adaptador não trava a IDE.
    fn go(&mut self, command: &str) -> Result<(), String> {
        let _ = self.events_pending();
        self.events.retain(|e| !matches!(e, Event::Stopped { .. }));
        let seq = self.seq;
        self.seq += 1;
        self.send(&json!({ "seq": seq, "type": "request", "command": command, "arguments": self.thread_args() }))
    }

    /// Lê o que chegou para a fila, sem esperar.
    fn events_pending(&mut self) -> usize {
        let mut n = 0;
        while let Ok(message) = self.incoming.try_recv() {
            n += 1;
            match message.get("type").and_then(Value::as_str) {
                Some("event") => self.queue_event(&message),
                Some("request") => {
                    let _ = self.refuse(&message);
                }
                _ => {}
            }
        }
        n
    }

    pub fn resume(&mut self) -> Result<(), String> {
        self.go("continue")
    }

    /// Próxima linha (F8 no Delphi).
    pub fn step_over(&mut self) -> Result<(), String> {
        self.go("next")
    }

    /// Entra na função (F7).
    pub fn step_in(&mut self) -> Result<(), String> {
        self.go("stepIn")
    }

    /// Sai da função.
    pub fn step_out(&mut self) -> Result<(), String> {
        self.go("stepOut")
    }

    pub fn pause(&mut self) -> Result<(), String> {
        self.request("pause", self.thread_args()).map(|_| ())
    }

    /// A pilha da thread parada.
    pub fn stack(&mut self) -> Result<Vec<Frame>, String> {
        let body = self.request("stackTrace", json!({ "threadId": self.thread.unwrap_or(0), "levels": 50 }))?;
        Ok(body["stackFrames"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|f| Frame {
                        id: f["id"].as_i64().unwrap_or(0),
                        name: f["name"].as_str().unwrap_or_default().to_string(),
                        file: f["source"]["path"].as_str().map(PathBuf::from),
                        line: f["line"].as_u64().unwrap_or(0) as usize,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// As variáveis locais de um quadro.
    pub fn locals(&mut self, frame: i64) -> Result<Vec<Variable>, String> {
        let body = self.request("scopes", json!({ "frameId": frame }))?;
        let Some(scope) = body["scopes"].as_array().and_then(|s| s.first()).and_then(|s| s["variablesReference"].as_i64()) else { return Ok(Vec::new()) };
        let mut vars = self.variables(scope)?;
        for v in &mut vars {
            if let Some(text) = self.pascal_string(frame, &v.name, &v.kind) {
                v.value = text;
                v.children = 0;
            }
        }
        Ok(vars)
    }

    /// Um texto do Free Pascal (`AnsiString`, `UnicodeString`...) é um ponteiro para os
    /// caracteres, e o CodeLLDB mostra os bytes do ponteiro: lê o texto por ele e devolve
    /// como no Delphi (`'abc'`). `None` se não é um texto do Pascal.
    fn pascal_string(&mut self, frame: i64, expression: &str, kind: &str) -> Option<String> {
        if self.adapter != "lldb" {
            return None;
        }
        let char_type = match kind.to_ascii_lowercase().as_str() {
            "ansistring" | "utf8string" | "rawbytestring" | "string" => "char",
            "unicodestring" | "widestring" => "char16_t",
            _ => return None,
        };
        let native = format!("/nat *({char_type}**)&({expression})");
        let body = self.request("evaluate", json!({ "expression": native, "frameId": frame, "context": "watch" })).ok()?;
        let result = body["result"].as_str().unwrap_or_default();
        Some(pascal_literal(result))
    }

    /// As variáveis (ou os campos) de uma referência.
    pub fn variables(&mut self, reference: i64) -> Result<Vec<Variable>, String> {
        let body = self.request("variables", json!({ "variablesReference": reference }))?;
        Ok(body["variables"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|v| Variable {
                        name: v["name"].as_str().unwrap_or_default().to_string(),
                        value: v["value"].as_str().unwrap_or_default().to_string(),
                        kind: v["type"].as_str().unwrap_or_default().to_string(),
                        children: v["variablesReference"].as_i64().unwrap_or(0),
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// O começo de um passo: o processador da thread (ou do processo, se o id do depurador
    /// não é uma thread do sistema), o relógio e a memória.
    fn begin_measure(&mut self) {
        // Sem o processo (o adaptador não informou), fica só o relógio.
        let pid = self.debuggee.unwrap_or(0);
        let known = self.debuggee.is_some();
        let thread = self.thread.filter(|_| known).and_then(|t| u64::try_from(t).ok()).and_then(|t| crate::cputime::thread_cpu(pid, t).map(|cpu| (t, cpu)));
        let process = if thread.is_none() && known { crate::cputime::process_cpu(pid) } else { None };
        let memory = self.read_memory();
        self.stopped_at = None;
        self.measure = Some(Measure { pid, thread, process, memory, wall: Instant::now() });
    }

    /// O fim do passo (o programa parado não gasta processador: ler agora é o mesmo que no
    /// `stopped`).
    fn finish_measure(&mut self) -> Option<LineTiming> {
        let m = self.measure.take()?;
        let wall = self.stopped_at.unwrap_or_else(Instant::now).saturating_duration_since(m.wall);
        let (cpu, whole_process) = match (m.thread, m.process) {
            // A mesma thread parou de novo; outra thread parou: o tempo da que andou.
            (Some((tid, start)), _) => (crate::cputime::thread_cpu(m.pid, tid).map(|now| now.saturating_sub(start)), false),
            (None, Some(start)) => (crate::cputime::process_cpu(m.pid).map(|now| now.saturating_sub(start)), true),
            (None, None) => (None, false),
        };
        let memory = match (m.memory, self.read_memory()) {
            (Some(a), Some(b)) => Some(b - a),
            _ => None,
        };
        Some(LineTiming { cpu, wall, memory, whole_process })
    }

    /// Os bytes alocados no programa, pela [`memory_expression`](Self::memory_expression).
    /// Falhou uma vez (o app sem o contador): não tenta mais.
    fn read_memory(&mut self) -> Option<i64> {
        // Sem o contador do aylis (as outras linguagens): a memória privada do processo, pelo
        // sistema.
        let Some(expression) = self.memory_expression.clone() else {
            return self.debuggee.and_then(crate::cputime::process_memory).map(|b| b as i64);
        };
        // O CodeLLDB só acha a variável global a partir de um quadro: o de cima basta.
        let frame = self
            .request("stackTrace", json!({ "threadId": self.thread.unwrap_or(0), "levels": 1 }))
            .ok()
            .and_then(|body| body["stackFrames"].get(0).and_then(|f| f["id"].as_i64()));
        let mut args = json!({ "expression": expression, "context": "watch" });
        if let Some(frame) = frame {
            args["frameId"] = frame.into();
        }
        let value = self.request("evaluate", args).ok().and_then(|body| body["result"].as_str().and_then(last_integer));
        if value.is_none() {
            self.memory_expression = None;
        }
        value
    }

    /// Calcula uma expressão no quadro (o "Avaliar/Modificar" do Delphi).
    pub fn evaluate(&mut self, frame: i64, expression: &str) -> Result<String, String> {
        let body = self.request("evaluate", json!({ "expression": expression, "frameId": frame, "context": "watch" }))?;
        if let Some(text) = body["type"].as_str().and_then(|kind| self.pascal_string(frame, expression, kind)) {
            return Ok(text);
        }
        Ok(body["result"].as_str().unwrap_or_default().to_string())
    }

    /// Para o programa e encerra a sessão.
    pub fn stop(&mut self) {
        if !self.ended {
            let _ = self.request("disconnect", json!({ "terminateDebuggee": true }));
        }
        self.ended = true;
        if let Some(mut c) = self.child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

/// Este processo está num *job* do Windows que encerra os processos dele quando o job fecha
/// (`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`): uma IDE aberta assim pela ponte do MCP (o cliente
/// recusou o `CREATE_BREAKAWAY_FROM_JOB`) morre em silêncio, sem erro nem registro do Windows,
/// quando o cliente encerra ou reinicia a ponte (a pista da IDE que sumiu na conferência de 08/11).
#[cfg(windows)]
pub fn in_kill_on_close_job() -> bool {
    unsafe extern "system" {
        fn GetCurrentProcess() -> isize;
        fn IsProcessInJob(process: isize, job: isize, result: *mut i32) -> i32;
        fn QueryInformationJobObject(job: isize, class: u32, info: *mut u8, length: u32, returned: *mut u32) -> i32;
    }
    const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION: u32 = 9;
    const KILL_ON_JOB_CLOSE: u32 = 0x2000;
    // O tamanho da JOBOBJECT_EXTENDED_LIMIT_INFORMATION e onde fica o LimitFlags dentro dela
    // (depois dos dois LARGE_INTEGER do começo).
    #[cfg(target_pointer_width = "64")]
    const SIZE: u32 = 144;
    #[cfg(target_pointer_width = "32")]
    const SIZE: u32 = 112;
    let mut info = [0u8; 160];
    // SAFETY: chamadas do Windows sobre o próprio processo; o buffer tem mais que `SIZE` bytes,
    // e o job nulo é o do processo atual (como a documentação diz).
    unsafe {
        let mut in_job = 0;
        if IsProcessInJob(GetCurrentProcess(), 0, &mut in_job) == 0 || in_job == 0 {
            return false;
        }
        if QueryInformationJobObject(0, JOB_OBJECT_EXTENDED_LIMIT_INFORMATION, info.as_mut_ptr(), SIZE, std::ptr::null_mut()) == 0 {
            return false;
        }
    }
    let flags = u32::from_ne_bytes([info[16], info[17], info[18], info[19]]);
    flags & KILL_ON_JOB_CLOSE != 0
}

#[cfg(not(windows))]
pub fn in_kill_on_close_job() -> bool {
    false
}

/// O processo ainda existe.
#[cfg(windows)]
pub(crate) fn process_alive(pid: u32) -> bool {
    unsafe extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> isize;
        fn WaitForSingleObject(handle: isize, millis: u32) -> u32;
        fn CloseHandle(handle: isize) -> i32;
        fn GetLastError() -> u32;
    }
    const SYNCHRONIZE: u32 = 0x0010_0000;
    const WAIT_TIMEOUT: u32 = 0x102;
    const ERROR_ACCESS_DENIED: u32 = 5;
    // SAFETY: chamadas simples do Windows; o handle é fechado logo em seguida.
    unsafe {
        let h = OpenProcess(SYNCHRONIZE, 0, pid);
        if h == 0 {
            // Acesso negado é de um processo que existe (noutro nível de integridade ou numa
            // sandbox, como pode ser a ponte de outro cliente): contar como morto tirava a sessão
            // viva dele do registro da IDE (conferência de 08/11). Morto mesmo dá parâmetro inválido.
            return GetLastError() == ERROR_ACCESS_DENIED;
        }
        let running = WaitForSingleObject(h, 0) == WAIT_TIMEOUT;
        CloseHandle(h);
        running
    }
}

#[cfg(unix)]
pub(crate) fn process_alive(pid: u32) -> bool {
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    // SAFETY: o sinal 0 só confere se o processo existe.
    unsafe { kill(pid as i32, 0) == 0 }
}

#[cfg(not(any(windows, unix)))]
pub(crate) fn process_alive(_pid: u32) -> bool {
    true
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop();
    }
}

// ---------------------------------------------------------------------------
// As linguagens do aylis-host: o depurador se anexa ao processo da lógica, que já roda
// (o aylis-host informa o número dele com `AYLIS_BACKEND_PID=`).

/// Um arquivo na pasta de ferramentas do aylis ou no PATH.
fn tool(relative: &str, name: &str) -> Option<PathBuf> {
    let exe = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    tools_dir().map(|d| d.join(relative).join(&exe)).filter(|p| p.is_file()).or_else(|| which(&exe))
}

fn which(exe: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(exe)).find(|p| p.is_file()))
}

/// O texto que o LLDB mostra para um `char*` (`"abc"`, `u"abc"`, com os escapes do C) como
/// um literal do Pascal (`'abc'`, o apóstrofo dobrado). O ponteiro nulo (o texto vazio do
/// Pascal) vira `''`.
fn pascal_literal(lldb: &str) -> String {
    let s = lldb.trim();
    let s = s.strip_prefix(['u', 'U', 'L']).filter(|r| r.starts_with('"')).unwrap_or(s);
    let Some(inner) = s.strip_prefix('"').and_then(|r| r.strip_suffix('"')) else { return "''".into() };
    let mut text = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            text.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => text.push('\n'),
            Some('t') => text.push('\t'),
            Some('r') => text.push('\r'),
            Some('0') => text.push('\0'),
            Some(other) => text.push(other),
            None => {}
        }
    }
    format!("'{}'", text.replace('\'', "''"))
}

// ---------------------------------------------------------------------------
// A sessão numa thread
// ---------------------------------------------------------------------------

/// Andar: continuar, um passo ou pausar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Go {
    Resume,
    StepOver,
    StepIn,
    StepOut,
    Pause,
}

/// Um pedido para a sessão que roda no [`Worker`].
#[derive(Debug)]
pub enum Job {
    Go(Go),
    /// Parou: a pilha e as variáveis do primeiro quadro com código-fonte.
    Stopped,
    /// As variáveis de outro quadro (escolhido na pilha).
    Locals { frame: i64 },
    /// Os campos de uma variável (quando ela é aberta).
    Children { reference: i64 },
    Evaluate { frame: i64, expression: String },
    SetBreakpoints { file: PathBuf, points: Vec<Breakpoint> },
    /// Para o programa e termina a thread.
    Stop,
}

/// O começo de um passo (veja `Session::begin_measure`).
struct Measure {
    pid: u32,
    /// A thread do sistema e o processador dela no começo.
    thread: Option<(u64, Duration)>,
    /// O processador do processo todo (quando a thread não pôde ser lida).
    process: Option<Duration>,
    memory: Option<i64>,
    wall: Instant,
}

/// O que um passo gastou (do continuar ou do passo até a próxima parada): o tempo de
/// processador lido no sistema (sem o tempo parado no depurador), o de relógio e a mudança
/// nos bytes alocados (só onde o programa conta: Rust com `dev`).
#[derive(Clone, Debug, PartialEq)]
pub struct LineTiming {
    pub cpu: Option<Duration>,
    pub wall: Duration,
    pub memory: Option<i64>,
    /// O tempo é do processo todo (o depurador não dá a thread do sistema).
    pub whole_process: bool,
}

impl LineTiming {
    /// O texto do fim da linha (veja [`crate::cputime::line_note`]).
    pub fn note(&self) -> String {
        let note = crate::cputime::line_note(self.cpu, self.wall, self.memory);
        if self.whole_process && self.cpu.is_some() { format!("{note}  (processo)") } else { note }
    }
}

/// O nome aparece no executável ou nos símbolos ao lado dele (o `.pdb` do Windows, com `-`
/// virando `_`), lidos aos pedaços.
pub fn binary_has_symbol(exe: &Path, name: &str) -> bool {
    let stem = exe.file_stem().map(|s| s.to_string_lossy().replace('-', "_")).unwrap_or_default();
    let candidates = [exe.to_path_buf(), exe.with_extension("pdb"), exe.with_file_name(format!("{stem}.pdb"))];
    candidates.iter().any(|p| file_contains(p, name.as_bytes()))
}

fn file_contains(path: &Path, needle: &[u8]) -> bool {
    use std::io::Read;
    let Ok(mut file) = std::fs::File::open(path) else { return false };
    let mut buffer = vec![0u8; 1 << 20];
    let mut kept = 0usize;
    loop {
        let Ok(n) = file.read(&mut buffer[kept..]) else { return false };
        if n == 0 {
            return false;
        }
        let filled = kept + n;
        if buffer[..filled].windows(needle.len()).any(|w| w == needle) {
            return true;
        }
        // Guarda o fim do pedaço (o nome pode estar partido entre dois).
        kept = (needle.len() - 1).min(filled);
        buffer.copy_within(filled - kept..filled, 0);
    }
}

/// O último número inteiro do texto (`123`, `-45`, `{v:{value:{0:678}}}` → 678).
fn last_integer(text: &str) -> Option<i64> {
    let end = text.rfind(|c: char| c.is_ascii_digit())? + 1;
    let start = text[..end].rfind(|c: char| !c.is_ascii_digit()).map_or(0, |i| i + 1);
    let value = text[start..end].parse::<i64>().ok()?;
    Some(if text[..start].ends_with('-') { -value } else { value })
}

#[cfg(test)]
mod timing_tests {
    #[test]
    fn reads_the_counter_as_the_debugger_shows_it() {
        assert_eq!(super::last_integer("{v:{value:{0:1048576}}}"), Some(1_048_576));
        assert_eq!(super::last_integer("-45"), Some(-45));
        assert_eq!(super::last_integer("(long) 7"), Some(7));
        assert_eq!(super::last_integer("sem número"), None);
    }
}

/// O que volta do [`Worker`].
#[derive(Debug)]
pub enum Reply {
    Events(Vec<Event>),
    /// A pilha, o quadro escolhido (o primeiro com código-fonte), as variáveis dele e o que
    /// o passo gastou (desde o último continuar ou passo).
    Stopped { frames: Vec<Frame>, frame: usize, locals: Vec<Variable>, timing: Option<LineTiming> },
    Locals { frame: i64, locals: Vec<Variable> },
    Children { reference: i64, vars: Vec<Variable> },
    Evaluated { expression: String, result: Result<String, String> },
    Breakpoints { file: PathBuf, result: Result<Vec<usize>, String> },
    /// Um pedido falhou (o texto já diz qual).
    Failed(String),
}

/// A sessão numa thread: pedidos entram por [`send`](Self::send) sem esperar e as respostas
/// saem por [`replies`](Self::replies) (a função `wake` avisa que chegou algo).
pub struct Worker {
    jobs: mpsc::Sender<Job>,
    replies: Receiver<Reply>,
    /// Respostas que um [`call`](Self::call) tirou da fila antes da que esperava.
    held: std::cell::RefCell<VecDeque<Reply>>,
    ended: Arc<std::sync::atomic::AtomicBool>,
}

impl Worker {
    /// Manda um pedido (não espera).
    pub fn send(&self, job: Job) {
        let _ = self.jobs.send(job);
    }

    /// O que voltou desde a última leitura.
    pub fn replies(&self) -> Vec<Reply> {
        let mut all: Vec<Reply> = self.held.borrow_mut().drain(..).collect();
        all.extend(self.replies.try_iter());
        all
    }

    /// A sessão terminou (o programa saiu ou foi parado).
    pub fn ended(&self) -> bool {
        self.ended.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Pede e espera a resposta (até `timeout`), deixando as outras na fila: para quem não
    /// pode esperar a próxima volta (o MCP). Não use na tela.
    pub fn call(&self, job: Job, timeout: Duration, matches: impl Fn(&Reply) -> bool) -> Option<Reply> {
        self.send(job);
        let deadline = Instant::now() + timeout;
        let found = loop {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.replies.recv_timeout(left) {
                Ok(r) if matches(&r) => break Some(r),
                Ok(r) => self.held.borrow_mut().push_back(r),
                Err(_) => break None,
            }
        };
        found
    }
}

impl Session {
    /// Leva a sessão para uma thread própria (veja [`Worker`]). `wake` é chamada a cada
    /// resposta ou evento que volta.
    pub fn into_worker(mut self, wake: Option<WakeFn>) -> Worker {
        let (jobs, inbox) = mpsc::channel::<Job>();
        let (out, replies) = mpsc::channel::<Reply>();
        let ended = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let done = ended.clone();
        // A sessão acorda a thread (que lê os eventos), não a tela.
        let (nudge, nudged) = mpsc::channel::<()>();
        self.set_wake(Arc::new(move || {
            let _ = nudge.send(());
        }));
        let _ = std::thread::Builder::new().name("aylis-depurador".into()).spawn(move || {
            let send = |r: Reply| {
                let _ = out.send(r);
                if let Some(w) = &wake {
                    w();
                }
            };
            loop {
                // Um pedido, ou um aviso de que chegou mensagem do adaptador, ou a cada 50 ms.
                let job = match inbox.recv_timeout(Duration::from_millis(50)) {
                    Ok(job) => Some(job),
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(RecvTimeoutError::Disconnected) => Some(Job::Stop),
                };
                while nudged.try_recv().is_ok() {}
                if let Some(job) = job {
                    let stop = matches!(job, Job::Stop);
                    if let Some(reply) = self.work(job) {
                        send(reply);
                    }
                    if stop {
                        done.store(true, std::sync::atomic::Ordering::Relaxed);
                        send(Reply::Events(vec![Event::Terminated]));
                        return;
                    }
                }
                let events = self.events();
                if !events.is_empty() {
                    send(Reply::Events(events));
                }
                if self.ended() {
                    // Sempre um `Terminated` no fim (o adaptador pode cair sem mandar).
                    done.store(true, std::sync::atomic::Ordering::Relaxed);
                    send(Reply::Events(vec![Event::Terminated]));
                    return;
                }
            }
        });
        Worker { jobs, replies, held: Default::default(), ended }
    }

    /// Um pedido, na thread do [`Worker`].
    fn work(&mut self, job: Job) -> Option<Reply> {
        let failed = |what: &str, e: String| Some(Reply::Failed(format!("{what}: {e}")));
        match job {
            Job::Go(go) => {
                if go != Go::Pause {
                    self.begin_measure();
                }
                let result = match go {
                    Go::Resume => self.resume(),
                    Go::StepOver => self.step_over(),
                    Go::StepIn => self.step_in(),
                    Go::StepOut => self.step_out(),
                    Go::Pause => self.pause(),
                };
                result.err().and_then(|e| failed("depurador", e))
            }
            Job::Stopped => {
                let frames = match self.stack() {
                    Ok(f) => f,
                    Err(e) => return failed("pilha", e),
                };
                // O primeiro quadro com código-fonte (o resto é da biblioteca ou do sistema).
                let frame = frames.iter().position(|f| f.file.as_ref().is_some_and(|p| p.is_file())).unwrap_or(0);
                let locals = frames.get(frame).map(|f| self.locals(f.id).unwrap_or_default()).unwrap_or_default();
                let timing = self.finish_measure();
                Some(Reply::Stopped { frames, frame, locals, timing })
            }
            Job::Locals { frame } => match self.locals(frame) {
                Ok(locals) => Some(Reply::Locals { frame, locals }),
                Err(e) => failed("variáveis", e),
            },
            Job::Children { reference } => match self.variables(reference) {
                Ok(vars) => Some(Reply::Children { reference, vars }),
                Err(e) => failed("campos", e),
            },
            Job::Evaluate { frame, expression } => {
                let result = self.evaluate(frame, &expression);
                Some(Reply::Evaluated { expression, result })
            }
            Job::SetBreakpoints { file, points } => {
                let result = self.set_breakpoints(&file, &points);
                Some(Reply::Breakpoints { file, result })
            }
            Job::Stop => {
                self.stop();
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{launch_env, pascal_literal};

    #[test]
    fn debugged_programs_run_without_the_windows_debug_heap() {
        let env = launch_env(&[("RUST_BACKTRACE".into(), "1".into())]);
        assert_eq!(env["_NO_DEBUG_HEAP"], "1");
        assert_eq!(env["RUST_BACKTRACE"], "1");
        // Quem pede o heap de depuração fica com ele.
        assert_eq!(launch_env(&[("_NO_DEBUG_HEAP".into(), "0".into())])["_NO_DEBUG_HEAP"], "0");
    }

    #[test]
    fn lldb_strings_become_pascal_literals() {
        assert_eq!(pascal_literal("\"5\""), "'5'");
        assert_eq!(pascal_literal("u\"Olá\""), "'Olá'");
        assert_eq!(pascal_literal("\"d'água \\\"x\\\"\""), "'d''água \"x\"'");
        assert_eq!(pascal_literal("0x0000000000000000"), "''");
        assert_eq!(pascal_literal("<null>"), "''");
    }
}

/// O Delve (Go): o da pasta de ferramentas, o do PATH ou o de `~/go/bin`.
pub fn delve() -> Option<PathBuf> {
    tool("delve", "dlv").or_else(|| {
        let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
        let p = PathBuf::from(home).join("go/bin").join(if cfg!(windows) { "dlv.exe" } else { "dlv" });
        p.is_file().then_some(p)
    })
}

/// O netcoredbg (C#, .NET).
pub fn netcoredbg() -> Option<PathBuf> {
    tool("netcoredbg/netcoredbg", "netcoredbg").or_else(|| tool("netcoredbg", "netcoredbg"))
}

/// O servidor DAP do js-debug (TypeScript e JavaScript, pelo Node).
pub fn js_debug() -> Option<PathBuf> {
    tools_dir().map(|d| d.join("js-debug/js-debug/src/dapDebugServer.js")).filter(|p| p.is_file())
}

/// O `jdb` do JDK (Java): em `JAVA_HOME` ou no PATH.
pub fn jdb() -> Option<PathBuf> {
    let exe = if cfg!(windows) { "jdb.exe" } else { "jdb" };
    std::env::var_os("JAVA_HOME").map(|h| PathBuf::from(h).join("bin").join(exe)).filter(|p| p.is_file()).or_else(|| which(exe))
}

/// Uma porta livre nesta máquina.
fn free_port() -> Result<u16, String> {
    std::net::TcpListener::bind(("127.0.0.1", 0)).and_then(|l| l.local_addr()).map(|a| a.port()).map_err(|e| e.to_string())
}

impl Session {
    /// Manda o `attach` (que alguns adaptadores só respondem depois do `configurationDone`)
    /// e os pontos de parada.
    fn attach_with(mut self, arguments: Value, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let seq = self.seq;
        self.seq += 1;
        self.send(&json!({ "seq": seq, "type": "request", "command": "attach", "arguments": arguments }))?;
        self.configure(breakpoints)?;
        Ok(self)
    }

    /// Pascal e C++: o CodeLLDB anexado ao processo (o executável com informação de
    /// depuração DWARF ou PDB).
    pub fn attach_native(adapter: &Path, pid: u32, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let mut command = Command::new(adapter);
        if let Some(lib) = adapter.parent().and_then(Path::parent).map(|e| e.join("lldb/bin").join(if cfg!(windows) { "liblldb.dll" } else { "liblldb.so" })).filter(|p| p.is_file()) {
            command.arg("--liblldb").arg(lib);
        }
        let mut s = Session::spawn(command)?;
        s.initialize("lldb")?;
        s.watch(pid);
        s.attach_with(json!({ "pid": pid, "stopOnEntry": false }), breakpoints)
    }

    /// Go: o `dlv dap` anexado ao processo.
    pub fn attach_go(dlv: &Path, pid: u32, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let port = free_port()?;
        let mut command = Command::new(dlv);
        command.args(["dap", "--listen", &format!("127.0.0.1:{port}")]);
        let server = crate::process::quiet(&mut command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("não consegui iniciar o Delve: {e}"))?;
        crate::process::kill_with_parent(&server);
        let mut s = Session::connect(port, Duration::from_secs(20))?;
        s.child = Some(server);
        s.initialize("go")?;
        s.watch(pid);
        s.attach_with(json!({ "mode": "local", "processId": pid, "stopOnEntry": false }), breakpoints)
    }

    /// C#: o netcoredbg anexado ao processo do .NET.
    pub fn attach_dotnet(netcoredbg: &Path, pid: u32, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let mut command = Command::new(netcoredbg);
        command.arg("--interpreter=vscode");
        let mut s = Session::spawn(command)?;
        s.initialize("coreclr")?;
        s.watch(pid);
        s.attach_with(json!({ "processId": pid, "justMyCode": true }), breakpoints)
    }

    /// TypeScript e JavaScript: o js-debug anexado ao inspetor do Node (`--inspect`). O
    /// js-debug abre uma sessão por alvo (pedindo `startDebugging` à IDE): a sessão
    /// devolvida é a do alvo, e a primeira fica viva dentro dela.
    pub fn attach_node(server_js: &Path, node: &Path, inspect_port: u16, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let port = free_port()?;
        let mut command = Command::new(node);
        command.arg(server_js).arg(port.to_string()).arg("127.0.0.1");
        let server = crate::process::quiet(&mut command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("não consegui iniciar o js-debug: {e}"))?;
        crate::process::kill_with_parent(&server);
        let mut parent = Session::connect(port, Duration::from_secs(20))?;
        parent.child = Some(server);
        parent.initialize("pwa-node")?;
        let seq = parent.seq;
        parent.seq += 1;
        let arguments = json!({ "type": "pwa-node", "request": "attach", "name": "aylis", "address": "127.0.0.1", "port": inspect_port, "continueOnAttach": true });
        parent.send(&json!({ "seq": seq, "type": "request", "command": "attach", "arguments": arguments }))?;
        // O js-debug pede uma sessão nova para o alvo: a configuração vem no pedido.
        let deadline = Instant::now() + TIMEOUT;
        let configuration = loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let message = parent.incoming.recv_timeout(left).map_err(|_| "o js-debug não encontrou o processo do Node")?;
            let kind = message.get("type").and_then(Value::as_str);
            let command = message.get("command").and_then(Value::as_str);
            if kind == Some("request") && command == Some("startDebugging") {
                let seq = parent.seq;
                parent.seq += 1;
                parent.send(&json!({ "seq": seq, "type": "response", "request_seq": message.get("seq"), "command": "startDebugging", "success": true }))?;
                break message["arguments"]["configuration"].clone();
            } else if kind == Some("request") {
                parent.refuse(&message)?;
            } else if kind == Some("event") && message.get("event").and_then(Value::as_str) == Some("initialized") {
                let seq = parent.seq;
                parent.seq += 1;
                parent.send(&json!({ "seq": seq, "type": "request", "command": "configurationDone", "arguments": {} }))?;
            }
        };
        let mut child = Session::connect(port, Duration::from_secs(10))?;
        child.initialize("pwa-node")?;
        let mut child = child.attach_with(configuration, breakpoints)?;
        child.parent = Some(Box::new(parent));
        Ok(child)
    }

    /// Java: o adaptador do aylis sobre o `jdb` (o `aylis-tool jdb-dap`), ligado ao JDWP do
    /// processo. `sources` é a pasta dos `.java` (para achar o arquivo de cada classe).
    pub fn attach_java(aylis_tool: &Path, jdb: &Path, jdwp_port: u16, sources: &Path, breakpoints: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Result<Session, String> {
        let mut command = Command::new(aylis_tool);
        command.arg("jdb-dap").arg(jdb).arg(sources);
        let mut s = Session::spawn(command)?;
        s.initialize("java")?;
        s.attach_with(json!({ "hostName": "127.0.0.1", "port": jdwp_port }), breakpoints)
    }
}
