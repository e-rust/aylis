//! Depuração dentro da IDE, como no Delphi: pontos de parada na calha da janela de código
//! (F5 ou clique), Ctrl+F5 depura o projeto, e quando o programa para a linha fica em
//! amarelo e a janela "Depuração" mostra a pilha, as variáveis e o "Avaliar". F8 vai para
//! a próxima linha, F7 entra na função, Shift+F8 sai dela, F9 continua e Ctrl+F2 para.
//!
//! Por baixo, o protocolo de depuração do VS Code (`aylis_tools::dap`): o CodeLLDB para
//! Rust e o `debugpy` para Python. A sessão roda numa thread própria ([`Worker`]): a IDE
//! manda os pedidos e mostra as respostas quando chegam, sem nunca esperar o depurador. Os
//! campos de uma variável só são buscados quando ela é aberta.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::Arc;
use std::time::Duration;

use aylis_controls::{Button, Edit, Label, Layout, Splitter, TreeNode, TreeView};
use aylis_core::{Align, ComponentId, EventArgs, Key, Ui};
use aylis_project::Language;
use aylis_tools::dap::{Event, Frame, Go, Job, Reply, Session, Variable, Worker};
use aylis_tools::{Target, run};

use crate::jobs::{Msg, OutKind, OutputLine, Slot};
use crate::state::Shared;
use crate::widgets::muted;

/// Um comando de depuração (das teclas, dos botões e dos menus).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd {
    Continue,
    StepOver,
    StepIn,
    StepOut,
    Pause,
    Stop,
}

impl Cmd {
    /// Os comandos com o texto e o atalho (os mesmos em todas as janelas).
    pub const ALL: [(Cmd, &'static str, &'static str); 6] = [
        (Cmd::Continue, "Continuar", "F9"),
        (Cmd::StepOver, "Próxima linha", "F8"),
        (Cmd::StepIn, "Entrar na função", "F7"),
        (Cmd::StepOut, "Sair da função", "Shift+F8"),
        (Cmd::Pause, "Pausar", ""),
        (Cmd::Stop, "Parar", "Ctrl+F2"),
    ];
}

/// O painel de depuração (à direita, na janela de código).
#[derive(Clone, Copy)]
struct Window {
    panel: ComponentId,
    stack: ComponentId,
    vars: ComponentId,
    status: ComponentId,
    result: ComponentId,
}

/// A depuração em andamento (fica no estado da IDE).
#[derive(Default)]
pub struct Debugger {
    session: Option<Worker>,
    /// A sessão sendo aberta noutra thread (o Python espera o programa subir).
    pending: Option<Receiver<Result<Session, String>>>,
    /// Parado (chegou o `stopped`; a pilha pode ainda estar a caminho).
    paused: bool,
    frames: Vec<Frame>,
    /// O quadro escolhido na pilha.
    frame: usize,
    /// As variáveis do quadro escolhido (sem os campos).
    locals: Vec<Variable>,
    /// Variáveis abertas esperando os campos: a referência e o caminho do nó na árvore.
    expanding: HashMap<i64, Vec<usize>>,
    window: Option<Window>,
    /// Onde a linha amarela está.
    exec: Option<(PathBuf, usize)>,
    /// "Executar até o cursor" (F4): o ponto de parada temporário.
    run_to: Option<(PathBuf, usize)>,
    /// O "Pausar" foi pedido: no Windows a pausa chega como a exceção 0x80000003 (o
    /// DebugBreakProcess), que é mostrada como "pausado".
    pausing: bool,
    /// As linguagens do aylis-host: o que anexar quando o processo da lógica subir.
    attach: Option<Attach>,
    /// O processo da lógica (o aylis-host informa ao subir).
    backend_pid: Option<u32>,
    /// O que o último passo gastou (o texto no fim da linha amarela).
    timing: Option<aylis_tools::dap::LineTiming>,
}

/// Uma anexação esperando o processo da lógica subir.
struct Attach {
    project: aylis_project::Project,
    breakpoints: BTreeMap<PathBuf, Vec<aylis_tools::dap::Breakpoint>>,
}

impl Debugger {
    /// O aylis-host informou o processo da lógica (`AYLIS_BACKEND_PID=`).
    pub fn backend_started(&mut self, pid: u32) {
        if self.attach.is_some() {
            self.backend_pid = Some(pid);
        }
    }

    /// A execução terminou antes do processo subir (erro de compilação): desiste.
    pub fn cancel_attach(&mut self) {
        self.attach = None;
        self.backend_pid = None;
    }

    /// Há uma depuração em andamento (ou começando).
    pub fn active(&self) -> bool {
        self.session.is_some() || self.pending.is_some() || self.attach.is_some()
    }

    /// O programa está parado (num ponto de parada ou num passo).
    pub fn stopped(&self) -> bool {
        self.session.is_some() && self.paused
    }

    /// Parado e com a pilha já lida (ela chega do depurador logo depois do `stopped`).
    pub fn has_stack(&self) -> bool {
        self.stopped() && !self.frames.is_empty()
    }
}

/// O texto de uma variável na árvore.
fn var_text(v: &Variable) -> String {
    if v.kind.is_empty() { format!("{} = {}", v.name, v.value) } else { format!("{} = {}   ({})", v.name, v.value, v.kind) }
}

/// O texto do filho provisório de uma variável com campos (até eles chegarem).
const LOADING: &str = "…";

/// Os nós das variáveis: as que têm campos ganham um filho provisório e a referência em
/// `data`, e os campos vêm quando o nó é aberto.
fn var_nodes(vars: &[Variable]) -> Vec<TreeNode> {
    vars.iter()
        .take(200)
        .map(|v| {
            let mut n = TreeNode::new(var_text(v));
            if v.children > 0 {
                n.children = vec![TreeNode::new(LOADING)];
                n.data = v.children.to_string();
            }
            n
        })
        .collect()
}

/// A depuração para quem comanda a IDE de fora (o MCP): a situação, onde parou, a pilha,
/// as variáveis do quadro escolhido e, com `expression`, o "Avaliar".
pub fn snapshot(state: &Shared, expression: Option<&str>) -> serde_json::Value {
    use serde_json::json;
    let mut st = state.borrow_mut();
    let status = st.status.clone();
    let points: Vec<serde_json::Value> =
        st.breakpoints.iter().map(|(f, lines)| json!({ "file": f.display().to_string(), "lines": lines.iter().collect::<Vec<_>>() })).collect();
    let d = &mut st.debugger;
    let (active, stopped) = (d.active(), d.stopped());
    let exec = d.exec.as_ref().map(|(f, l)| json!({ "file": f.display().to_string(), "line": l }));
    let frames: Vec<serde_json::Value> = d
        .frames
        .iter()
        .enumerate()
        .map(|(i, f)| json!({ "index": i, "selected": i == d.frame, "name": f.name, "file": f.file.as_ref().map(|p| p.display().to_string()), "line": f.line }))
        .collect();
    let frame = d.frames.get(d.frame).map(|f| f.id);
    let variables: Vec<serde_json::Value> = d
        .locals
        .iter()
        .take(200)
        .map(|v| json!({ "name": v.name, "value": v.value, "type": v.kind, "reference": v.children }))
        .collect();
    let mut evaluated = serde_json::Value::Null;
    if let (Some(s), Some(frame), Some(e)) = (d.session.as_ref(), frame, expression.filter(|e| !e.trim().is_empty())) {
        // O MCP espera a resposta (as outras ficam guardadas para a tela).
        let job = Job::Evaluate { frame, expression: e.to_string() };
        let reply = s.call(job, Duration::from_secs(15), |r| matches!(r, Reply::Evaluated { expression, .. } if expression == e));
        evaluated = match reply {
            Some(Reply::Evaluated { result: Ok(v), .. }) => json!({ "expression": e, "value": v }),
            Some(Reply::Evaluated { result: Err(err), .. }) => json!({ "expression": e, "error": err }),
            _ => json!({ "expression": e, "error": "o depurador não respondeu" }),
        };
    }
    json!({ "status": status, "active": active, "stopped": stopped, "exec": exec, "stack": frames,
            "variables": variables, "evaluate": evaluated, "breakpoints": points })
}

/// Para quem comanda a IDE de fora (o MCP): espera a resposta do depurador a `job` (até
/// 15 s). `Err` com a falha, sem sessão ou sem resposta.
fn wait_reply(state: &Shared, job: Job, matches: impl Fn(&Reply) -> bool) -> Result<Reply, String> {
    let st = state.borrow();
    let s = st.debugger.session.as_ref().ok_or("nenhuma depuração em andamento")?;
    match s.call(job, Duration::from_secs(15), |r| matches(r) || matches!(r, Reply::Failed(_))) {
        Some(Reply::Failed(e)) => Err(e),
        Some(r) => Ok(r),
        None => Err("o depurador não respondeu".into()),
    }
}

/// Os campos de uma variável (a `reference` que o `snapshot` dá), para o MCP.
pub fn children(state: &Shared, reference: i64) -> Result<Vec<Variable>, String> {
    match wait_reply(state, Job::Children { reference }, |r| matches!(r, Reply::Children { reference: got, .. } if *got == reference))? {
        Reply::Children { vars, .. } => Ok(vars),
        _ => unreachable!(),
    }
}

/// Escolhe o quadro `index` da pilha (como clicar nele): a linha no código e as variáveis
/// dele. Espera as variáveis (para o MCP).
pub fn select_frame(ui: &mut Ui, state: &Shared, index: usize) -> Result<(), String> {
    let frame = state.borrow().debugger.frames.get(index).map(|f| f.id).ok_or("o programa não está parado ou o quadro não existe")?;
    let locals = match wait_reply(state, Job::Locals { frame }, |r| matches!(r, Reply::Locals { frame: got, .. } if *got == frame))? {
        Reply::Locals { locals, .. } => locals,
        _ => unreachable!(),
    };
    {
        let mut st = state.borrow_mut();
        st.debugger.frame = index;
        st.debugger.locals = locals;
    }
    show_frame(ui, state);
    Ok(())
}

/// O caminho como a IDE compara (o depurador pode devolver com outra caixa ou barra).
pub fn norm(path: &Path) -> PathBuf {
    // Guardado: as abas pedem o de cada form a cada refresh, e o `canonicalize` vai ao disco
    // (no ERP com os 1009 forms abertos, 180 ms por troca de aba numa pasta do OneDrive;
    // `erp_all_forms_in_tabs`). Só o que existe entra (o que ainda não existe muda quando for
    // criado).
    static CACHE: std::sync::Mutex<Option<std::collections::HashMap<PathBuf, PathBuf>>> = std::sync::Mutex::new(None);
    if let Some(hit) = CACHE.lock().ok().and_then(|c| c.as_ref().and_then(|c| c.get(path).cloned())) {
        return hit;
    }
    match std::fs::canonicalize(path) {
        Ok(p) => {
            let p = PathBuf::from(p.to_string_lossy().trim_start_matches(r"\\?\").to_string());
            if let Ok(mut cache) = CACHE.lock() {
                cache.get_or_insert_with(Default::default).insert(path.to_path_buf(), p.clone());
            }
            p
        }
        Err(_) => PathBuf::from(path.to_string_lossy().trim_start_matches(r"\\?\").to_string()),
    }
}

/// Os pontos de parada de um arquivo, com as condições (e o temporário do F4).
fn points_of(state: &Shared, file: &Path) -> Vec<aylis_tools::dap::Breakpoint> {
    let st = state.borrow();
    let mut points: Vec<aylis_tools::dap::Breakpoint> = st
        .breakpoints
        .get(file)
        .into_iter()
        .flatten()
        .map(|&line| {
            let (condition, hit_condition) = st.bp_conditions.get(&(file.to_path_buf(), line)).cloned().unwrap_or_default();
            aylis_tools::dap::Breakpoint { line, condition, hit_condition }
        })
        .collect();
    if let Some((f, line)) = &st.debugger.run_to {
        if f == file && !points.iter().any(|p| p.line == *line) {
            points.push(aylis_tools::dap::Breakpoint::at(*line));
        }
    }
    points
}

fn breakpoint_map(state: &Shared) -> BTreeMap<PathBuf, Vec<aylis_tools::dap::Breakpoint>> {
    let mut files: Vec<PathBuf> = state.borrow().breakpoints.keys().cloned().collect();
    if let Some((f, _)) = state.borrow().debugger.run_to.clone() {
        files.push(f);
    }
    files.into_iter().map(|f| (f.clone(), points_of(state, &f))).collect()
}

/// Manda ao depurador (se houver sessão) os pontos de parada do arquivo.
fn send_points(state: &Shared, file: &Path) {
    let points = points_of(state, file);
    if let Some(s) = state.borrow().debugger.session.as_ref() {
        s.send(Job::SetBreakpoints { file: file.to_path_buf(), points });
    }
}

/// Condição e contagem de passagens do ponto de parada da linha (cria o ponto se não
/// houver). Vazias: um ponto comum.
pub fn set_condition(ui: &mut Ui, state: &Shared, file: &Path, line: usize, condition: &str, hits: &str) {
    let file = norm(file);
    let lines: Vec<usize> = {
        let mut st = state.borrow_mut();
        let set = st.breakpoints.entry(file.clone()).or_default();
        set.insert(line);
        let lines = set.iter().copied().collect();
        if condition.trim().is_empty() && hits.trim().is_empty() {
            st.bp_conditions.remove(&(file.clone(), line));
        } else {
            st.bp_conditions.insert((file.clone(), line), (condition.trim().to_string(), hits.trim().to_string()));
        }
        lines
    };
    crate::codewindow::update(ui, &file, |m| m.set_breakpoints(&lines));
    send_points(state, &file);
}

/// A condição de um ponto de parada (vazia se não tiver).
pub fn condition_of(state: &Shared, file: &Path, line: usize) -> (String, String) {
    state.borrow().bp_conditions.get(&(norm(file), line)).cloned().unwrap_or_default()
}

/// "Executar até o cursor" (F4, como no Delphi): roda até a linha, com um ponto de parada
/// temporário (sem depuração em andamento, começa uma).
pub fn run_to_cursor(ui: &mut Ui, state: &Shared, file: &Path, line: usize) {
    let file = norm(file);
    state.borrow_mut().debugger.run_to = Some((file.clone(), line));
    if !state.borrow().debugger.active() {
        start(ui, state);
        return;
    }
    send_points(state, &file);
    command(ui, state, Cmd::Continue);
}

/// Liga ou desliga o ponto de parada da linha (a partir de 1). Com o programa rodando,
/// vale na hora.
pub fn toggle_breakpoint(ui: &mut Ui, state: &Shared, file: &Path, line: usize) {
    let file = norm(file);
    let lines: Vec<usize> = {
        let mut st = state.borrow_mut();
        let set = st.breakpoints.entry(file.clone()).or_default();
        if !set.remove(&line) {
            set.insert(line);
        }
        set.iter().copied().collect()
    };
    if !lines.contains(&line) {
        state.borrow_mut().bp_conditions.remove(&(file.clone(), line));
    }
    crate::codewindow::update(ui, &file, |m| m.set_breakpoints(&lines));
    send_points(state, &file);
}

/// As linhas com ponto de parada de um arquivo.
pub fn breakpoints_of(state: &Shared, file: &Path) -> Vec<usize> {
    state.borrow().breakpoints.get(&norm(file)).map(|s| s.iter().copied().collect()).unwrap_or_default()
}

/// Executar (F9), como no Delphi: com uma depuração em andamento, continua; com algum ponto
/// de parada no desktop, executa no depurador; senão, executa sem ele (sobe mais rápido).
pub fn run(ui: &mut Ui, state: &Shared) {
    let (active, has_points, desktop) = {
        let st = state.borrow();
        (st.debugger.active(), st.breakpoints.values().any(|s| !s.is_empty()), st.run_target == Target::Desktop)
    };
    if active {
        command(ui, state, Cmd::Continue);
    } else if has_points && desktop && state.borrow().project.is_some() {
        start(ui, state);
    } else {
        crate::actions::run_project(ui, state);
    }
}

/// Ctrl+F5: compila e depura o projeto no desktop, parando nos pontos de parada.
pub fn start(ui: &mut Ui, state: &Shared) {
    // O código alterado no editor entra na compilação (ela lê o disco).
    crate::codewindow::save_all(ui);
    if !crate::trust::allowed(state) {
        return;
    }
    if state.borrow().debugger.active() {
        state.borrow_mut().status = "Já há uma depuração em andamento (F9 continua, Ctrl+F2 para)".into();
        return;
    }
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para depurar".into();
        return;
    };
    crate::actions::save_all(state);
    let breakpoints = breakpoint_map(state);
    match project.language {
        Language::Rust => {
            let Some(adapter) = aylis_tools::dap::codelldb() else {
                state.borrow_mut().status = "⚠ Falta o CodeLLDB para depurar Rust (Ferramentas → Verificar ambiente)".into();
                return;
            };
            let title = format!("Depurando {}", project.name);
            // O canal com a IDE (o "Inspetor do app" e o MCP), como no F9 sem depurador.
            let trace = state.borrow().trace_events;
            if !crate::jobs::open_channel(state, &project, trace) {
                return;
            }
            let env = vec![("RUST_BACKTRACE".to_string(), "1".to_string())];
            let key = crate::jobs::build_key(&project, Target::Desktop, "dev");
            crate::jobs::start(state, Slot::Main, title, move |control, sender| {
                let mut sink = |e| sender.send(Msg::Event(e));
                let exe = run::build_desktop(&project, false, control, &mut sink)?;
                sender.send(Msg::Event(aylis_tools::Event::Step("Depurando (pontos de parada: F5 na janela de código)…".into())));
                let session = Session::launch_rust(&adapter, &exe, &project.dir(), &env, &breakpoints)?;
                sender.send(Msg::Debug(Box::new(session)));
                Ok("O programa está rodando no depurador.".into())
            });
            crate::jobs::track_build(state, key);
        }
        Language::Python => {
            // O aylis-host roda o backend com o debugpy esperando; a IDE conecta nele.
            state.borrow_mut().run_target = Target::Desktop;
            crate::jobs::run_debug(state);
            let (tx, rx) = mpsc::channel();
            let waker = state.borrow().output_waker.clone();
            std::thread::spawn(move || {
                let result = Session::attach_python(aylis_tools::debug::DEBUGPY_PORT, &breakpoints);
                let _ = tx.send(result);
                if let Some(w) = waker {
                    w.wake();
                }
            });
            state.borrow_mut().debugger.pending = Some(rx);
        }
        // As outras: o app sobe pelo aylis-host e o depurador da linguagem se anexa ao
        // processo da lógica.
        other => {
            if let Err(missing) = attach_tools(other) {
                let looked = aylis_tools::dap::tools_dir().map(|d| d.display().to_string()).unwrap_or_else(|| "(sem LOCALAPPDATA)".into());
                state.borrow_mut().status =
                    format!("⚠ Para depurar {} na IDE falta {missing} (procurado em {looked}; Ferramentas → Verificar ambiente instala)", other.name());
                return;
            }
            let mut st = state.borrow_mut();
            st.debugger.attach = Some(Attach { project, breakpoints });
            st.debugger.backend_pid = None;
            st.run_target = Target::Desktop;
            drop(st);
            crate::jobs::run_debug_attach(state);
        }
    }
}

/// O depurador de cada linguagem do aylis-host. `Err` com o que falta.
fn attach_tools(language: Language) -> Result<(), &'static str> {
    use aylis_tools::dap;
    match language {
        Language::Pascal | Language::Cpp => dap::codelldb().map(|_| ()).ok_or("o CodeLLDB"),
        Language::Go => dap::delve().map(|_| ()).ok_or("o Delve (dlv)"),
        Language::CSharp => dap::netcoredbg().map(|_| ()).ok_or("o netcoredbg"),
        Language::TypeScript => {
            dap::js_debug().ok_or("o js-debug")?;
            aylis_tools::hostlang::node().map(|_| ()).map_err(|_| "o Node 22.6+ (o js-debug roda nele)")
        }
        Language::Java => {
            dap::jdb().ok_or("o jdb (vem com o JDK)")?;
            crate::jobs::aylis_tool().map(|_| ()).ok_or("o aylis-tool ao lado da IDE")
        }
        _ => Ok(()),
    }
}

/// Espera alguém ouvir na porta (o inspetor do Node, o JDWP do Java) por até `secs`.
fn wait_port(port: u16, secs: u64) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    while std::time::Instant::now() < deadline {
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Anexa o depurador da linguagem ao processo `pid` (numa thread: demora).
fn attach_session(attach: Attach, pid: u32) -> Result<Session, String> {
    use aylis_tools::dap;
    let Attach { project, breakpoints } = attach;
    match project.language {
        Language::Pascal | Language::Cpp => Session::attach_native(&dap::codelldb().ok_or("sem o CodeLLDB")?, pid, &breakpoints),
        Language::Go => Session::attach_go(&dap::delve().ok_or("sem o Delve")?, pid, &breakpoints),
        Language::CSharp => Session::attach_dotnet(&dap::netcoredbg().ok_or("sem o netcoredbg")?, pid, &breakpoints),
        Language::TypeScript => {
            let port = aylis_tools::debug::NODE_PORT;
            wait_port(port, 15);
            let (node, _) = aylis_tools::hostlang::node()?;
            Session::attach_node(&dap::js_debug().ok_or("sem o js-debug")?, &node, port, &breakpoints)
        }
        Language::Java => {
            let port = aylis_tools::debug::JDWP_PORT;
            wait_port(port, 15);
            let tool = crate::jobs::aylis_tool().ok_or("sem o aylis-tool")?;
            Session::attach_java(&tool, &dap::jdb().ok_or("sem o jdb")?, port, &project.code_dir().join("src"), &breakpoints)
        }
        other => Err(format!("{} não depura por anexação", other.name())),
    }
}

/// A sessão aberta pela thread de trabalho chega aqui.
pub fn attach(state: &Shared, session: Session) {
    let wake = state.borrow().output_waker.clone().map(|w| Arc::new(move || w.wake()) as aylis_tools::dap::WakeFn);
    let mut st = state.borrow_mut();
    st.debugger.session = Some(session.into_worker(wake));
    st.status = "Depurando: o programa está rodando e para quando chegar num ponto de parada".into();
}

/// Na bomba de mensagens: a sessão que chegou e os eventos do programa. `true` se algo
/// mudou.
pub fn poll(ui: &mut Ui, state: &Shared) -> bool {
    let mut changed = false;
    // O processo da lógica subiu: anexa o depurador da linguagem (numa thread).
    let ready = {
        let mut st = state.borrow_mut();
        match st.debugger.backend_pid.take() {
            Some(pid) => st.debugger.attach.take().map(|a| (a, pid)),
            None => None,
        }
    };
    if let Some((attach, pid)) = ready {
        let (tx, rx) = mpsc::channel();
        let waker = state.borrow().output_waker.clone();
        std::thread::spawn(move || {
            let _ = tx.send(attach_session(attach, pid));
            if let Some(w) = waker {
                w.wake();
            }
        });
        let mut st = state.borrow_mut();
        st.debugger.pending = Some(rx);
        st.status = "Anexando o depurador…".into();
        changed = true;
    }
    let pending = state.borrow_mut().debugger.pending.take();
    if let Some(rx) = pending {
        match rx.try_recv() {
            Ok(Ok(session)) => {
                attach(state, session);
                changed = true;
            }
            Ok(Err(e)) => {
                state.borrow_mut().status = format!("⚠ depurador: {e}");
                changed = true;
            }
            Err(TryRecvError::Empty) => state.borrow_mut().debugger.pending = Some(rx),
            Err(TryRecvError::Disconnected) => {}
        }
    }
    let replies = state.borrow().debugger.session.as_ref().map(Worker::replies).unwrap_or_default();
    for reply in replies {
        changed = true;
        // Terminou no meio: o resto não interessa mais.
        if state.borrow().debugger.session.is_none() {
            break;
        }
        match reply {
            Reply::Events(events) => on_events(ui, state, events),
            Reply::Stopped { frames, frame, locals, timing } => {
                {
                    let mut st = state.borrow_mut();
                    if !st.debugger.paused {
                        continue; // já voltou a rodar
                    }
                    st.debugger.frames = frames;
                    st.debugger.frame = frame;
                    st.debugger.locals = locals;
                    // O relógio do passo também no status (a dica da linha).
                    if let Some(t) = &timing {
                        st.status = format!("{} — relógio {}", st.status, aylis_tools::cputime::short_time(t.wall));
                    }
                    st.debugger.timing = timing;
                }
                open_window(ui, state);
                show_frame(ui, state);
            }
            Reply::Locals { frame, locals } => {
                let current = state.borrow().debugger.frames.get(state.borrow().debugger.frame).map(|f| f.id);
                if current == Some(frame) {
                    state.borrow_mut().debugger.locals = locals;
                    show_locals(ui, state);
                }
            }
            Reply::Children { reference, vars } => {
                let (path, window) = {
                    let mut st = state.borrow_mut();
                    (st.debugger.expanding.remove(&reference), st.debugger.window)
                };
                if let (Some(path), Some(w)) = (path, window) {
                    if let Some(t) = ui.get_mut::<TreeView>(w.vars) {
                        // Só se o nó ainda é o mesmo (a árvore pode ter sido refeita).
                        if t.node(&path).is_some_and(|n| n.data == reference.to_string()) {
                            t.set_children(&path, var_nodes(&vars));
                            ui.request_repaint();
                        }
                    }
                }
            }
            Reply::Evaluated { result, .. } => {
                let answer = result.unwrap_or_else(|e| format!("⚠ {e}"));
                if let Some(w) = state.borrow().debugger.window.filter(|w| ui.contains(w.result)) {
                    let _ = ui.set_prop(w.result, "text", format!("= {answer}"));
                }
            }
            Reply::Breakpoints { result: Err(e), .. } => state.borrow_mut().status = format!("⚠ ponto de parada: {e}"),
            Reply::Breakpoints { .. } => {}
            Reply::Failed(e) => state.borrow_mut().status = format!("⚠ {e}"),
        }
    }
    changed
}

/// Os eventos do programa (parou, voltou a rodar, saída, terminou).
fn on_events(ui: &mut Ui, state: &Shared, events: Vec<Event>) {
    for event in events {
        match event {
            Event::Stopped { reason, text, .. } => on_stopped(ui, state, &reason, text.as_deref()),
            Event::Continued => running(ui, state),
            Event::Output { category, text } => {
                let kind = if category == "stderr" { OutKind::Error } else { OutKind::Program };
                let mut st = state.borrow_mut();
                for line in text.lines().filter(|l| !l.trim().is_empty()) {
                    st.push(OutputLine::new(line.to_string(), kind));
                }
            }
            Event::Exited(code) => {
                state.borrow_mut().push(OutputLine::new(format!("Programa terminou{}.", code.map(|c| format!(" com código {c}")).unwrap_or_default()), OutKind::Info));
            }
            Event::Terminated => {
                end(ui, state, "O programa terminou");
                break;
            }
        }
    }
}

/// Parou: a pilha, as variáveis e a linha amarela no código.
fn on_stopped(ui: &mut Ui, state: &Shared, reason: &str, text: Option<&str>) {
    // Chegou (ou parou antes): o ponto temporário do F4 sai.
    let run_to = state.borrow_mut().debugger.run_to.take();
    if let Some((file, _)) = run_to {
        send_points(state, &file);
    }
    {
        let mut st = state.borrow_mut();
        let Some(s) = st.debugger.session.as_ref() else { return };
        // A pilha e as variáveis vêm depois, pelo `poll` (a tela não espera o depurador).
        s.send(Job::Stopped);
        st.debugger.paused = true;
        let pausing = std::mem::take(&mut st.debugger.pausing);
        let reason = if pausing && reason == "exception" { "pause" } else { reason };
        let why = match reason {
            "breakpoint" => "no ponto de parada".to_string(),
            "step" => "depois do passo".to_string(),
            "pause" => "pausado".to_string(),
            "exception" => format!("numa exceção{}", text.map(|t| format!(": {t}")).unwrap_or_default()),
            other => other.to_string(),
        };
        st.status = if reason == "pause" {
            "Pausado (F9 continua, F8 próxima linha, F7 entra, Shift+F8 sai)".to_string()
        } else {
            format!("Parado {why} (F9 continua, F8 próxima linha, F7 entra, Shift+F8 sai)")
        };
    }
    if let Some(w) = state.borrow().debugger.window.filter(|w| ui.contains(w.status)) {
        let _ = ui.set_prop(w.status, "text", state.borrow().status.clone());
    }
}

/// Mostra o quadro escolhido: a linha no código e as variáveis.
fn show_frame(ui: &mut Ui, state: &Shared) {
    let (frame, window) = {
        let st = state.borrow();
        (st.debugger.frames.get(st.debugger.frame).cloned(), st.debugger.window)
    };
    let Some(frame) = frame else { return };
    clear_exec(ui, state);
    if let Some(file) = frame.file.as_ref().filter(|f| f.is_file()) {
        let file = norm(file);
        let marks = breakpoints_of(state, &file);
        // O tempo e a memória do passo, só no quadro de cima (onde o passo parou).
        let note = {
            let st = state.borrow();
            let top = st.debugger.frames.iter().position(|f| f.file.as_ref().is_some_and(|p| p.is_file()));
            st.debugger.timing.as_ref().filter(|_| top == Some(st.debugger.frame)).map(aylis_tools::dap::LineTiming::note).unwrap_or_default()
        };
        if crate::codewindow::open(ui, &file, frame.line).is_ok() {
            crate::codewindow::update(ui, &file, |m| {
                m.set_breakpoints(&marks);
                m.set_exec_line(Some(frame.line));
                m.set_exec_note(note.clone());
            });
            state.borrow_mut().debugger.exec = Some((file, frame.line));
        }
    }
    let Some(w) = window else { return };
    // A pilha.
    let (frames, selected) = {
        let st = state.borrow();
        (st.debugger.frames.clone(), st.debugger.frame)
    };
    let nodes: Vec<TreeNode> = frames
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let place = f.file.as_ref().map(|p| format!("  {}:{}", p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), f.line)).unwrap_or_default();
            let mut n = TreeNode::new(format!("{}{}{}", if i == selected { "▶ " } else { "" }, f.name, place));
            n.data = i.to_string();
            n
        })
        .collect();
    if let Some(t) = ui.get_mut::<TreeView>(w.stack) {
        t.set_nodes(nodes);
    }
    show_locals(ui, state);
    let status = state.borrow().status.clone();
    let _ = ui.set_prop(w.status, "text", status);
    ui.request_repaint();
}

/// As variáveis do quadro escolhido (os campos vêm ao abrir cada uma).
fn show_locals(ui: &mut Ui, state: &Shared) {
    let (nodes, window) = {
        let mut st = state.borrow_mut();
        st.debugger.expanding.clear();
        (var_nodes(&st.debugger.locals), st.debugger.window)
    };
    if let Some(t) = window.and_then(|w| ui.get_mut::<TreeView>(w.vars)) {
        t.set_nodes(nodes);
    }
    ui.request_repaint();
}

/// Tira a linha amarela.
fn clear_exec(ui: &mut Ui, state: &Shared) {
    let exec = state.borrow_mut().debugger.exec.take();
    if let Some((file, _)) = exec {
        crate::codewindow::update(ui, &file, |m| m.set_exec_line(None));
    }
}

/// O programa voltou a rodar: sem linha amarela e sem pilha.
fn running(ui: &mut Ui, state: &Shared) {
    clear_exec(ui, state);
    let mut st = state.borrow_mut();
    st.debugger.paused = false;
    st.debugger.frames.clear();
    st.debugger.locals.clear();
    st.debugger.expanding.clear();
}

/// Terminou (o programa ou o "Parar").
fn end(ui: &mut Ui, state: &Shared, message: &str) {
    running(ui, state);
    let window = {
        let mut st = state.borrow_mut();
        // A thread para o programa e termina sozinha.
        if let Some(s) = st.debugger.session.take() {
            s.send(Job::Stop);
        }
        st.debugger.pending = None;
        st.status = message.to_string();
        st.debugger.window
    };
    if let Some(w) = window.filter(|w| ui.contains(w.panel)) {
        let _ = ui.set_prop(w.status, "text", message);
        let _ = ui.set_prop(w.result, "text", "");
        for t in [w.stack, w.vars] {
            if let Some(t) = ui.get_mut::<TreeView>(t) {
                t.set_nodes(Vec::new());
            }
        }
        // Fora da depuração, o código ocupa a janela toda.
        crate::codewindow::debug_panel(ui, false);
    }
}

/// Um comando (teclas, botões, menus).
pub fn command(ui: &mut Ui, state: &Shared, cmd: Cmd) {
    if !state.borrow().debugger.active() {
        if cmd == Cmd::Continue {
            start(ui, state);
        } else if cmd == Cmd::Stop && state.borrow().job.is_some() {
            // Ainda compilando (o Rust anexa só depois): para a compilação.
            crate::jobs::stop(state);
        } else {
            state.borrow_mut().status = "Nenhuma depuração em andamento (Ctrl+F5 depura o projeto)".into();
        }
        return;
    }
    if cmd == Cmd::Stop {
        crate::jobs::stop(state);
        end(ui, state, "Depuração parada");
        return;
    }
    // Ainda compilando ou anexando: não há o que continuar.
    if state.borrow().debugger.session.is_none() {
        state.borrow_mut().status = "A depuração está começando (compilando e anexando o depurador)…".into();
        return;
    }
    if cmd != Cmd::Pause {
        running(ui, state);
    }
    let go = match cmd {
        Cmd::Continue => Go::Resume,
        Cmd::StepOver => Go::StepOver,
        Cmd::StepIn => Go::StepIn,
        Cmd::StepOut => Go::StepOut,
        Cmd::Pause => Go::Pause,
        Cmd::Stop => return,
    };
    let mut st = state.borrow_mut();
    st.debugger.pausing = cmd == Cmd::Pause;
    if let Some(s) = st.debugger.session.as_ref() {
        // Um erro volta como `Reply::Failed` no `poll`.
        s.send(Job::Go(go));
    }
    st.status = "Rodando…".into();
}

/// O painel de depuração na janela de código: botões, pilha, variáveis e "Avaliar".
fn open_window(ui: &mut Ui, state: &Shared) {
    if let Some(w) = state.borrow().debugger.window.filter(|w| ui.contains(w.panel)) {
        let _ = (w, crate::codewindow::debug_panel(ui, true));
        return;
    }
    let Some(form) = crate::codewindow::debug_panel(ui, true) else { return };
    let bar = crate::codewindow::flow(ui, form);
    for (cmd, text, shortcut) in Cmd::ALL {
        // O atalho vai na dica (o painel é estreito); a largura pelo texto no idioma da IDE,
        // e o FlowLayout quebra a linha quando não cabe.
        let width = ui.tr(text).chars().count() as f32 * 8.0 + 28.0;
        let b = ui.add(Button::new(text), bar);
        ui.control_mut(b).unwrap().set_width(width).set_height(30.0);
        if !shortcut.is_empty() {
            let _ = ui.set_prop(b, "hint", shortcut);
        }
        if cmd != Cmd::Continue {
            ui.control_mut(b).unwrap().set_style("flat");
        }
        let state = state.clone();
        ui.on_click(b, move |ui, _| command(ui, &state, cmd));
    }
    let status = ui.add(Label::new(""), form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(26.0).set_margins([8.0, 0.0, 8.0, 0.0]);
    muted(ui, status);

    // Avaliar: Enter calcula a expressão no quadro escolhido.
    let eval = ui.add(Layout::default(), form);
    ui.control_mut(eval).unwrap().set_align(Align::Bottom).set_height(64.0).set_padding(4.0);
    let expr = ui.add(Edit::new(""), eval);
    ui.control_mut(expr).unwrap().set_align(Align::Top).set_height(30.0);
    let _ = ui.set_prop(expr, "placeholder", "Avaliar expressão (Enter)");
    let result = ui.add(Label::new(""), eval);
    ui.control_mut(result).unwrap().set_align(Align::Client).set_margins([2.0, 4.0, 0.0, 0.0]);
    {
        let state = state.clone();
        ui.on(expr, "on_key_down", move |ui, sender, args| {
            let EventArgs::Key(k) = args else { return };
            if k.key != Key::Enter {
                return;
            }
            let text = match ui.get_prop(sender, "text") {
                Ok(aylis_core::PropValue::String(s)) => s,
                _ => return,
            };
            // A resposta chega pelo `poll` (Reply::Evaluated).
            let answer = {
                let st = state.borrow();
                let frame = st.debugger.frames.get(st.debugger.frame).map(|f| f.id);
                match (st.debugger.session.as_ref(), frame) {
                    (Some(s), Some(frame)) => {
                        s.send(Job::Evaluate { frame, expression: text });
                        "…".to_string()
                    }
                    _ => "o programa precisa estar parado".to_string(),
                }
            };
            let _ = ui.set_prop(result, "text", format!("= {answer}"));
        });
    }

    let stack = ui.add(TreeView::default(), form);
    ui.control_mut(stack).unwrap().set_align(Align::Top).set_height(190.0);
    ui.add(Splitter::new(Align::Top), form);
    let vars = ui.add(TreeView::default(), form);
    ui.control_mut(vars).unwrap().set_align(Align::Client);
    {
        let state = state.clone();
        ui.on(stack, "on_change", move |ui, sender, _| {
            let index = ui.get::<TreeView>(sender).and_then(|t| t.selected()).and_then(|n| n.data.parse::<usize>().ok());
            let Some(i) = index else { return };
            {
                let mut st = state.borrow_mut();
                if st.debugger.frame == i {
                    return;
                }
                st.debugger.frame = i;
                st.debugger.locals.clear();
                // As variáveis do quadro chegam pelo `poll` (Reply::Locals).
                let frame = st.debugger.frames.get(i).map(|f| f.id);
                if let (Some(s), Some(frame)) = (st.debugger.session.as_ref(), frame) {
                    s.send(Job::Locals { frame });
                }
            }
            show_frame(ui, &state);
        });
    }
    {
        // Abriu uma variável: busca os campos (o filho provisório "…" some quando chegam).
        let state = state.clone();
        ui.on(vars, "on_expand", move |ui, sender, _| {
            let Some(t) = ui.get::<TreeView>(sender) else { return };
            let Some(path) = t.last_toggled() else { return };
            let Some(node) = t.node(&path) else { return };
            let waiting = node.children.len() == 1 && node.children[0].text == LOADING;
            let Ok(reference) = node.data.parse::<i64>() else { return };
            if !waiting || reference <= 0 {
                return;
            }
            let mut st = state.borrow_mut();
            if let Some(s) = st.debugger.session.as_ref() {
                s.send(Job::Children { reference });
            }
            st.debugger.expanding.insert(reference, path);
        });
    }
    state.borrow_mut().debugger.window = Some(Window { panel: form, stack, vars, status, result });
    crate::lang::translate(ui);
}
