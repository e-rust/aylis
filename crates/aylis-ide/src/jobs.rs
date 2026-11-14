//! Trabalhos em segundo plano: Executar (F9), Publicar, Verificar ambiente e as
//! correções. Cada um roda numa thread (com o `aylis-tools`); o andamento chega pela
//! bomba de mensagens ([`drain`]) e vira linhas no painel de Mensagens.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::Arc;

use aylis_core::Ui;
use aylis_project::{Language, Project};
use aylis_tools::env::{self, Check, Fix};
use aylis_tools::process::{CANCELLED, Diagnostic, Level};
use aylis_tools::{Control, Event, LineKind, Location, Target, publish, run};

use crate::app::Ids;
use crate::state::Shared;

/// Uma linha do painel de Mensagens.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputLine {
    pub text: String,
    pub kind: OutKind,
    /// O que um clique faz (abrir o código no erro, abrir a pasta publicada).
    pub action: Option<OutAction>,
    /// O texto completo de um erro do compilador (com o trecho do código).
    pub detail: Option<String>,
    /// Do programa em execução (depois da compilação), não da compilação: o `output` do MCP
    /// separa os dois (M20 do Tabernáculo).
    pub program: bool,
}

/// Uma linha do histórico das mensagens (o `output` do MCP).
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryLine {
    pub text: String,
    pub kind: OutKind,
    pub program: bool,
    /// A execução em que apareceu ([`crate::state::State::run_number`]).
    pub run: u64,
    /// O projeto aberto quando apareceu (vazio: nenhum).
    pub project: String,
}

impl HistoryLine {
    pub fn of(line: &OutputLine, run: u64, project: String) -> Self {
        Self { text: line.text.clone(), kind: line.kind, program: line.program || line.kind == OutKind::Program, run, project }
    }

    /// `build` (a compilação, a geração, a verificação) ou `program` (o app e o rastro).
    pub fn source(&self) -> &'static str {
        if self.program { "program" } else { "build" }
    }
}

impl OutKind {
    /// O nível da linha, pelo nome (o `output` do MCP).
    pub fn level(self) -> &'static str {
        match self {
            OutKind::Error => "error",
            OutKind::Warning => "warning",
            OutKind::Success => "success",
            OutKind::Step => "step",
            OutKind::Info | OutKind::Program => "info",
            OutKind::Muted => "note",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutKind {
    Info,
    Step,
    Program,
    Error,
    Warning,
    Success,
    Muted,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutAction {
    Code(Location),
    Folder(PathBuf),
    /// Abrir o form (o arquivo) e selecionar o componente (vazio: só o form).
    Component(PathBuf, String),
}

impl OutputLine {
    pub fn new(text: impl Into<String>, kind: OutKind) -> Self {
        Self { text: text.into(), kind, action: None, detail: None, program: false }
    }
}

/// O que a thread de trabalho manda para a interface.
pub enum Msg {
    Event(Event),
    /// Resultado da verificação do ambiente destas plataformas (as outras ficam como estão).
    Checks(Vec<Target>, Vec<Check>),
    /// O ambiente não está pronto: abrir o "Verificar ambiente".
    NeedsEnvironment,
    /// Cada linguagem, se o compilador está instalado e o que falta (o assistente
    /// "Configurar linguagens").
    Languages(Vec<(Language, bool, Vec<String>)>),
    /// Publicado: (arquivo principal, pasta).
    Published(PathBuf, PathBuf),
    /// A depuração começou: a sessão vai para o estado da IDE.
    Debug(Box<aylis_tools::dap::Session>),
    /// Os problemas da verificação do projeto (antes de compilar), para as mensagens.
    Issues(Vec<aylis_project::check::Issue>),
    /// A geração de código mexeu no disco: o texto dos arquivos abertos antes dela (o editor
    /// acompanha, `codewindow::follow_generation`).
    Generated(Vec<(PathBuf, String)>),
    /// Os pacotes de ferramentas, com o que está instalado e o que a máquina já tem.
    ToolPacks(Vec<crate::toolpacksview::Row>),
    /// O que cada origem de pacotes tem (o "Testar as origens" das Opções).
    Sources(Vec<aylis_tools::toolpacks::Probe>),
    /// Terminou: a mensagem final ou o erro.
    Done(Result<String, String>),
}

/// Um trabalho em andamento.
pub struct Job {
    pub title: String,
    pub control: Arc<Control>,
    rx: Receiver<Msg>,
    /// Quando começou (o tempo decorrido aparece na barra de status).
    started: std::time::Instant,
    /// Quanto levou a compilação, quando o programa já começou a rodar.
    built: Option<std::time::Duration>,
    /// O modo da compilação (projeto, alvo, perfil): a chave do tempo guardado.
    build_key: Option<String>,
    /// Quanto a compilação levou da última vez neste modo (a barra de progresso).
    expected: Option<std::time::Duration>,
    /// O cargo compilou algo (sem isso foi só conferir; o tempo não serve de estimativa).
    compiled: bool,
}

impl Job {
    /// A compilação terminou e o programa começou a rodar (o MCP espera até aqui no `run`).
    pub fn program_started(&self) -> bool {
        self.built.is_some()
    }
}

/// Os tempos de compilação guardados na configuração da IDE, por modo.
const BUILD_TIMES: &str = "build_times";

/// A chave do modo de compilação: o projeto, o alvo e o perfil (`dev`, `release`,
/// `publicar`).
pub fn build_key(project: &Project, target: Target, profile: &str) -> String {
    format!("{}|{}|{profile}", project.dir().display(), target.name())
}

/// Marca o trabalho principal como uma compilação do modo `key`: com o tempo da última vez,
/// a barra de status mostra o progresso estimado; no fim, o tempo de agora é guardado.
pub fn track_build(state: &Shared, key: String) {
    let expected = crate::actions::config_value(BUILD_TIMES)[&key].as_f64().filter(|s| *s > 0.0).map(std::time::Duration::from_secs_f64);
    if let Some(job) = state.borrow_mut().job.as_mut() {
        job.build_key = Some(key);
        job.expected = expected;
    }
}

/// A compilação terminou bem: guarda quanto levou, se compilou algo de fato (o cargo diz
/// "Compiling"; as outras linguagens não dizem, e aí vale ter levado mais que uns segundos: sem
/// nada para compilar, a volta é rápida).
fn record_build(job: &Job, took: std::time::Duration) {
    let worked = job.compiled || took >= std::time::Duration::from_secs(3);
    let Some(key) = job.build_key.as_ref().filter(|_| worked) else { return };
    let mut times = crate::actions::config_value(BUILD_TIMES);
    if !times.is_object() {
        times = serde_json::json!({});
    }
    times[key.as_str()] = serde_json::Value::from((took.as_secs_f64() * 10.0).round() / 10.0);
    crate::actions::update_config(BUILD_TIMES, times);
}

/// Envia mensagens para a interface e a acorda.
pub struct Sender {
    tx: mpsc::Sender<Msg>,
    waker: Option<aylis_platform_winit::Waker>,
}

impl Sender {
    /// Outro igual (para mandar de outra thread: a verificação de ambiente avisa de cada
    /// plataforma, que roda na dela).
    pub fn duplicate(&self) -> Sender {
        Sender { tx: self.tx.clone(), waker: self.waker.clone() }
    }

    pub fn send(&self, msg: Msg) {
        let _ = self.tx.send(msg);
        if let Some(w) = &self.waker {
            w.wake();
        }
    }
}

/// Qual vaga: o Executar/Publicar, ou a verificação do ambiente (que pode rodar com o
/// programa aberto).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Main,
    Environment,
}

/// Começa um trabalho numa thread. `work` devolve a mensagem final.
pub fn start(
    state: &Shared,
    slot: Slot,
    title: impl Into<String>,
    work: impl FnOnce(&Control, &Sender) -> Result<String, String> + Send + 'static,
) -> bool {
    let title = title.into();
    let mut st = state.borrow_mut();
    let busy = match slot {
        Slot::Main => st.job.is_some(),
        Slot::Environment => st.env_job.is_some(),
    };
    if busy {
        st.status = "Já há um trabalho em andamento (Parar: Ctrl+F2)".into();
        return false;
    }
    let (tx, rx) = mpsc::channel();
    let control = Control::new();
    let sender = Sender { tx, waker: st.output_waker.clone() };
    let thread_control = control.clone();
    std::thread::Builder::new()
        .name("aylis-ide-job".into())
        .spawn(move || {
            let result = work(&thread_control, &sender);
            sender.send(Msg::Done(result));
        })
        .expect("thread de trabalho");
    let job = Job { title: title.clone(), control, rx, started: std::time::Instant::now(), built: None, build_key: None, expected: None, compiled: false };
    match slot {
        Slot::Main => {
            st.job = Some(job);
            st.run_number += 1;
            st.output.clear();
            st.show_messages = true;
            st.push(OutputLine::new(format!("▶ {title}"), OutKind::Step));
        }
        Slot::Environment => st.env_job = Some(job),
    }
    st.status = format!("{title}…");
    true
}

/// Para o trabalho principal (o programa em execução, a compilação).
pub fn stop(state: &Shared) {
    let st = state.borrow();
    match &st.job {
        Some(job) => job.control.cancel(),
        // Nada desta IDE rodando: o app do projeto aberto por uma sessão anterior (ou por outra
        // IDE) também para, pelo executável dele (relatório do InsightDB, 11/11: depois de
        // reabrir a IDE, o Parar não o alcançava e o Compilar recusava).
        None => {
            #[cfg(not(target_arch = "wasm32"))]
            let stopped = st.project.as_ref().is_some_and(aylis_tools::run::stop_project_app);
            #[cfg(target_arch = "wasm32")]
            let stopped = false;
            drop(st);
            state.borrow_mut().status = if stopped { "App do projeto encerrado".into() } else { "Nada em execução".into() };
        }
    }
}

/// Para tudo (ao fechar a IDE): não deixa o programa do usuário aberto sozinho.
pub fn stop_all(state: &Shared) {
    let st = state.borrow();
    for job in [&st.job, &st.env_job].into_iter().flatten() {
        job.control.cancel();
    }
}

/// Recebe o andamento dos trabalhos (na bomba de mensagens). `true` se algo mudou.
pub fn drain(ui: &mut Ui, state: &Shared, ids: &Ids) -> bool {
    let mut changed = false;
    let lines_before = state.borrow().output.len();
    let mut open_env = false;
    let mut env_changed = false;
    let mut languages_done = false;
    let mut languages_checked = false;
    let mut toolpacks_changed = false;
    let mut toolpacks_done = false;
    let mut sources: Option<Vec<aylis_tools::toolpacks::Probe>> = None;
    for slot in [Slot::Main, Slot::Environment] {
        let Some(mut job) = (match slot {
            Slot::Main => state.borrow_mut().job.take(),
            Slot::Environment => state.borrow_mut().env_job.take(),
        }) else {
            continue;
        };
        let mut finished = false;
        loop {
            let msg = match job.rx.try_recv() {
                Ok(m) => m,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    finished = true;
                    break;
                }
            };
            changed = true;
            let mut st = state.borrow_mut();
            match msg {
                Msg::Event(event) => {
                    // O aylis-host informou o processo da lógica: o depurador da IDE se anexa.
                    if let Event::Line(text, _) = &event {
                        if let Some(pid) = text.trim().strip_prefix("AYLIS_BACKEND_PID=").and_then(|p| p.parse::<u32>().ok()) {
                            st.debugger.backend_started(pid);
                            continue;
                        }
                        if text.trim_start().starts_with("Compiling ") {
                            job.compiled = true;
                        }
                    }
                    // O programa começou: a compilação terminou, com o tempo dela nas mensagens.
                    if slot == Slot::Main && job.built.is_none() && matches!(&event, Event::Step(s) if starts_program(s)) {
                        let took = job.started.elapsed();
                        job.built = Some(took);
                        record_build(&job, took);
                        st.push(OutputLine::new(format!("✔ Compilado em {}", elapsed_text(took)), OutKind::Success));
                    }
                    let base = st.project.as_ref().map(Project::dir).unwrap_or_default();
                    let mut line = output_line(event, &base);
                    // Depois de compilar, o que chega é do programa.
                    line.program = slot == Slot::Main && job.built.is_some();
                    if slot == Slot::Environment {
                        st.env_log = line.text.clone();
                        if st.languages_view.is_some() {
                            st.setup_log.push(line.text.clone());
                            let extra = st.setup_log.len().saturating_sub(400);
                            st.setup_log.drain(..extra);
                        }
                        env_changed = true;
                    }
                    if slot == Slot::Main || line.kind == OutKind::Error {
                        st.push(line);
                    }
                }
                Msg::Generated(before) => {
                    drop(st);
                    let failed = crate::codewindow::follow_generation(ui, before);
                    st = state.borrow_mut();
                    for file in failed {
                        st.push(OutputLine::new(format!("⚠ A geração acrescentou código em {}, que tem alterações não salvas no editor: salve e gere de novo (Projeto → Gerar código)", file.display()), OutKind::Warning));
                    }
                }
                Msg::Issues(issues) => {
                    if let Some(project) = st.project.clone() {
                        for issue in &issues {
                            st.push(issue_line(&project, issue));
                        }
                    }
                }
                Msg::Checks(targets, checks) => {
                    st.checks.retain(|c| !targets.contains(&c.target));
                    st.checks.extend(checks);
                    let now = aylis_tools::env_cache::now();
                    for t in targets {
                        st.env_pending.retain(|p| *p != t);
                        if !st.env_fresh.contains(&t) {
                            st.env_fresh.push(t);
                        }
                        st.env_saved.retain(|(p, _)| *p != t);
                        st.env_saved.push((t, now));
                    }
                    env_changed = true;
                }
                Msg::NeedsEnvironment => open_env = true,
                Msg::ToolPacks(rows) => {
                    st.toolpacks = rows;
                    toolpacks_changed = true;
                }
                Msg::Sources(list) => sources = Some(list),
                Msg::Languages(list) => {
                    st.language_status = list;
                    languages_checked = true;
                }
                Msg::Debug(session) => {
                    drop(st);
                    crate::debugger::attach(state, *session);
                    continue;
                }
                Msg::Published(main, dir) => {
                    let mut line = OutputLine::new(format!("✔ Publicado (clique para abrir a pasta): {}", main.display()), OutKind::Success);
                    line.action = Some(OutAction::Folder(dir));
                    st.push(line);
                }
                Msg::Done(result) => {
                    finished = true;
                    if slot == Slot::Main {
                        st.debugger.cancel_attach();
                        // Publicar: não há programa rodando, o trabalho todo é a compilação.
                        if job.built.is_none() && result.is_ok() {
                            record_build(&job, job.started.elapsed());
                        }
                    }
                    let (text, kind) = match result {
                        Ok(message) => (format!("■ {message}"), OutKind::Success),
                        Err(e) if e == CANCELLED => ("■ Interrompido.".to_string(), OutKind::Muted),
                        Err(e) => (format!("✖ {e}"), OutKind::Error),
                    };
                    if slot == Slot::Main {
                        st.push(OutputLine::new(text.clone(), kind));
                        // Sem programa rodando (Publicar, erro de compilação): o tempo total.
                        st.status = match job.built {
                            None => format!("{text} ({})", elapsed_text(job.started.elapsed())),
                            Some(_) => text,
                        };
                    } else {
                        // O fim da instalação vai para o log do assistente, que confere de novo.
                        if st.languages_view.is_some() && job.title.starts_with("Configurando") {
                            st.setup_log.push(text.clone());
                            languages_done = true;
                        }
                        if job.title == DEVICE_DEBUG {
                            st.status = text.clone();
                        }
                        if [crate::toolpacksview::INSTALLING, crate::toolpacksview::REMOVING].contains(&job.title.as_str()) {
                            st.toolpacks_log = text.clone();
                        }
                        toolpacks_done = true;
                        st.env_log = if kind == OutKind::Error { text } else { String::new() };
                        // O que não chegou (interrompido, uma correção que falhou) deixa de
                        // aparecer como "conferindo…".
                        st.env_pending.clear();
                        env_changed = true;
                    }
                }
            }
        }
        if !finished {
            let mut st = state.borrow_mut();
            match slot {
                Slot::Main => st.job = Some(job),
                Slot::Environment => st.env_job = Some(job),
            }
        }
    }
    if open_env {
        crate::tooldialogs::environment(ui, state, ids, false);
    } else if env_changed {
        crate::tooldialogs::fill_environment(ui, state, ids);
        crate::tooldialogs::fill_languages(ui, state);
    }
    if languages_checked {
        // A IDE acabou de abrir e nenhuma linguagem está instalada: o assistente.
        let open = {
            let mut st = state.borrow_mut();
            let first = std::mem::take(&mut st.wizard_on_start);
            first && st.language_status.iter().all(|(_, ready, _)| !ready)
        };
        if open {
            crate::tooldialogs::languages(ui, state, ids);
        }
        crate::tooldialogs::fill_languages(ui, state);
    }
    if languages_done {
        check_languages(state);
    }
    // Pacotes de ferramentas: a lista nova, o andamento e, com a vaga livre, a conferência
    // que esperava por ela.
    if toolpacks_changed {
        crate::toolpacksview::fill(ui, state);
    } else if env_changed || toolpacks_done {
        crate::toolpacksview::status(ui, state);
    }
    if toolpacks_done && !languages_done && state.borrow().toolpacks_pending && state.borrow().toolpacks_view.is_some() && state.borrow().env_job.is_none() {
        crate::toolpacksview::check(state);
    }
    if let Some(list) = sources {
        crate::options::show_probes(ui, state, &list);
    }
    // Os erros e avisos das MENSAGENS vão também para a gravação de uso: o que o autor relata
    // ("veja o log") aparece nela depois de a próxima execução limpar o painel (conferência de
    // 08/11). As linhas do programa ficam de fora (podem ter dados do usuário).
    let new_lines: Vec<String> = {
        let st = state.borrow();
        st.output.get(lines_before..).unwrap_or_default().iter().filter(|l| matches!(l.kind, OutKind::Error | OutKind::Warning) && (l.action.is_some() || l.text.starts_with(['✖', '⚠'])))
            .map(|l| l.text.clone())
            .collect()
    };
    for text in new_lines {
        ui.report(&aylis_core::activity::Activity::Custom { name: "mensagem", value: &text });
    }
    changed
}

/// A cor de uma linha do erro padrão do app. As notas do próprio Aylis (`aylis: ...`: o form
/// recarregado, o clique cancelado pelo interceptador, o evento adiado) vão para o erro padrão
/// mas não são erro: saíam em vermelho (relatado pelo autor em 08/11). Vermelho só para o que
/// diz que falhou.
fn aylis_note_kind(text: &str) -> OutKind {
    let Some(note) = text.trim_start().strip_prefix("aylis: ") else { return OutKind::Error };
    let lower = note.to_lowercase();
    let failed = ["erro", "error", "pânico", "panic", "não consegui", "falhou", "falha", "desconhecid", "indisponível", "não está registrada", "não tem"].iter().any(|w| lower.contains(w));
    if failed { OutKind::Error } else { OutKind::Muted }
}

fn output_line(event: Event, base: &Path) -> OutputLine {
    match event {
        Event::Step(text) => OutputLine::new(format!("▶ {text}"), OutKind::Step),
        Event::Line(text, kind) => {
            let out_kind = match kind {
                LineKind::Tool => OutKind::Muted,
                LineKind::Program => OutKind::Program,
                LineKind::ProgramError => aylis_note_kind(&text),
            };
            let lower = text.to_lowercase();
            let out_kind = if out_kind == OutKind::Muted && (lower.starts_with("error") || lower.contains("erro:")) { OutKind::Error } else { out_kind };
            let mut line = OutputLine::new(text, out_kind);
            line.action = aylis_tools::location_in(&line.text, base).map(OutAction::Code);
            line
        }
        Event::Diagnostic(d) => diagnostic_line(d, base),
    }
}

fn diagnostic_line(d: Diagnostic, base: &Path) -> OutputLine {
    let (mark, kind) = match d.level {
        Level::Error => ("✖ erro", OutKind::Error),
        Level::Warning => ("⚠ aviso", OutKind::Warning),
        Level::Note => ("ℹ nota", OutKind::Muted),
    };
    let code = d.code.as_deref().map(|c| format!("[{c}]")).unwrap_or_default();
    let place = d
        .location
        .as_ref()
        .map(|l| {
            let file = l.file.strip_prefix(base).unwrap_or(&l.file);
            format!("{}:{}:{}  ", file.display(), l.line, l.column)
        })
        .unwrap_or_default();
    OutputLine {
        text: format!("{mark}{code}: {place}{}", d.message),
        kind,
        action: d.location.map(OutAction::Code),
        detail: Some(d.rendered).filter(|r| !r.is_empty()),
        program: false,
    }
}

// ---------------------------------------------------------------------------
// Comandos

/// O `aylis-host` ao lado da IDE (projetos Python e das outras linguagens).
pub fn aylis_host() -> Option<PathBuf> {
    beside_ide("aylis-host")
}

/// O `aylis-tool` ao lado da IDE (o adaptador do jdb, para depurar Java).
pub fn aylis_tool() -> Option<PathBuf> {
    beside_ide("aylis-tool")
}

/// Converte o projeto para o formato deste aylis com o `aylis-migrate` (ao lado da IDE).
/// Devolve o relatório.
pub fn aylis_migrate(project: &std::path::Path) -> Result<String, String> {
    let exe = beside_ide("aylis-migrate").ok_or("o aylis-migrate não está ao lado da IDE")?;
    let out = std::process::Command::new(exe).arg(project).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// Um executável na pasta da IDE (ou na de cima, para os testes, que rodam em `deps`).
fn beside_ide(name: &str) -> Option<PathBuf> {
    let dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let up = dir.parent().filter(|_| dir.ends_with("deps")).map(|d| d.join(env::exe(name)));
    Some(dir.join(env::exe(name))).filter(|p| p.is_file()).or(up.filter(|p| p.is_file()))
}

/// Confere o ambiente da plataforma antes de compilar. Erro (e pede para abrir o
/// "Verificar ambiente") se faltar algo. Usa o resultado guardado quando as marcas dele não
/// mudaram (`env_cache`: ~50 ms em vez de ~300 ms a cada F9); guardado com falta, confere de
/// novo (a pessoa pode já ter resolvido).
fn require(language: Language, target: Target, sender: &Sender) -> Result<(), String> {
    let host = aylis_host();
    let targets = [target];
    let mut request = env::Request::new(language, &targets, host.as_deref());
    request.live = false;
    request.recheck_missing = true;
    let checks = env::check_with(&request, &|_, _| {});
    if env::ready(&checks, target) {
        return Ok(());
    }
    let missing: Vec<String> = env::missing(&checks, target).iter().map(|c| c.name.clone()).collect();
    sender.send(Msg::Checks(vec![target], checks));
    sender.send(Msg::NeedsEnvironment);
    Err(format!("o ambiente não está pronto para {}: falta {}", target.name(), missing.join(", ")))
}

/// Executar (F9) na plataforma escolhida.
pub fn run_project(state: &Shared) {
    if !crate::trust::allowed(state) {
        return;
    }
    if state.borrow().job.is_some() {
        state.borrow_mut().status = "O programa já está em execução (Parar: Ctrl+F2)".into();
        return;
    }
    run_with(state, false, false, false);
}

/// Compila otimizado (release) e executa.
pub fn run_release(state: &Shared) {
    if !crate::trust::allowed(state) {
        return;
    }
    if state.borrow().job.is_some() {
        state.borrow_mut().status = "O programa já está em execução (Parar: Ctrl+F2)".into();
        return;
    }
    run_with(state, false, false, true);
}

/// Compilar (Ctrl+F9): só compila o executável do desktop, sem executar (com `release`,
/// otimizado). Projetos Rust; as outras linguagens compilam ao executar.
pub fn build(state: &Shared, release: bool) {
    if !crate::trust::allowed(state) {
        return;
    }
    // Com o app executando (ou outra compilação), nem salva: o `start` recusaria depois.
    if state.borrow().job.is_some() {
        state.borrow_mut().status = "Já há um trabalho em andamento (Parar: Ctrl+F2)".into();
        return;
    }
    crate::actions::save_all(state);
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para compilar".into();
        return;
    };
    if project.language != Language::Rust {
        state.borrow_mut().status = format!("Compilar vale para projetos Rust ({} compila ao executar)", project.language.name());
        return;
    }
    // Um valor só no modelo (dois `{}` colados não casam com a tradução).
    let title = format!("Compilando {}", format!("{}{}", project.name, if release { " (release)" } else { "" }));
    let key = build_key(&project, Target::Desktop, if release { "release" } else { "dev" });
    // Os arquivos abertos no editor: a geração pode acrescentar handlers neles.
    let open_files = crate::codewindow::open_paths();
    let started = start(state, Slot::Main, title, move |control, sender| {
        let mut options = aylis_tools::build::Options::new(Target::Desktop);
        options.release = release;
        compile(&project, &options, Some(&open_files), control, sender).map(|(message, _)| message)
    });
    if started {
        track_build(state, key);
    }
}

/// Executa um projeto Python com o `debugpy` esperando o depurador (o da IDE ou o do VS
/// Code).
pub fn run_debug(state: &Shared) {
    run_with(state, true, false, false);
}

/// Depurar na IDE as linguagens do `aylis-host`: executa sem esperar, e o depurador da
/// linguagem se anexa ao processo da lógica (o `aylis-host` informa o número dele).
pub fn run_debug_attach(state: &Shared) {
    run_with(state, false, true, false);
}

/// Executa; com `debug`, um projeto Python espera o depurador do VS Code; com
/// `ide_debug`, as outras linguagens se preparam para o depurador da IDE; com `release`,
/// compila otimizado.
fn run_with(state: &Shared, debug: bool, ide_debug: bool, release: bool) {
    crate::actions::save_all(state);
    let (project, target, trace) = {
        let st = state.borrow();
        (st.project.clone(), st.run_target, st.trace_events)
    };
    let Some(project) = project else { return };
    if !open_channel(state, &project, trace) {
        return;
    }
    let title = format!("{} {}{} — {}", if debug { "Depurando" } else { "Executando" }, project.name, if release { " (release)" } else { "" }, target.name());
    let key = build_key(&project, target, if release { "release" } else { "dev" });
    // O aparelho escolhido na lista ao lado da saída (sem escolha: o primeiro do adb).
    let device = (target == aylis_tools::Target::Android).then(|| state.borrow().run_device.clone()).flatten();
    // No desktop, o aparelho do designer: a janela abre do tamanho dele, com a moldura.
    let frame = state.borrow().doc.device.map(|(d, landscape)| if landscape { format!("{};deitado", d.name) } else { d.name.to_string() });
    // Os arquivos abertos no editor: a geração pode acrescentar handlers neles.
    let open_files = crate::codewindow::open_paths();
    // A porta que o canal abriu, quando não é a do projeto (a padrão estava ocupada).
    let dev_port = state.borrow().dev.as_ref().map(|d| d.port()).filter(|&p| aylis_tools::devkeys::address(&project).rsplit(':').next() != Some(&p.to_string()));
    let started = start(state, Slot::Main, title, move |control, sender| {
        prepare(&project, target, Some(&open_files), control, sender)?;
        require(project.language, target, sender)?;
        let mut options = run::Options::new(target);
        options.dev_port = dev_port;
        options.aylis_host = aylis_host();
        options.device = device;
        options.debug = debug;
        options.ide_debug = ide_debug;
        options.release = release;
        options.frame = frame.filter(|_| target == aylis_tools::Target::Desktop);
        // O que o cargo mexer (o `Cargo.lock`) vai para as mensagens quando a compilação acaba, ou
        // no fim, se ela falhar (I06 do Tabernáculo).
        let before = std::cell::RefCell::new(Some(aylis_tools::touched::before(&project)));
        let report = || {
            if let Some(b) = before.borrow_mut().take() {
                b.report(&mut |e| sender.send(Msg::Event(e)));
            }
        };
        let mut sink = |e: Event| {
            if matches!(&e, Event::Step(s) if starts_program(s)) {
                report();
            }
            sender.send(Msg::Event(e));
        };
        let result = run::run(&project, &options, control, &mut sink);
        report();
        result
    });
    if started {
        track_build(state, key);
    }
}

/// Abre (ou reaproveita) o canal de desenvolvimento do projeto: o app que a IDE compila
/// conecta nele, em qualquer plataforma (inspetor, hot reload, rastro). `false` se não deu
/// (a mensagem fica na barra de status).
pub fn open_channel(state: &Shared, project: &aylis_project::Project, trace: bool) -> bool {
    let existing = state.borrow().dev.clone();
    match crate::devlink::for_project(existing, project) {
        Ok(server) => {
            server.set_trace(trace);
            state.borrow_mut().dev = Some(server);
            true
        }
        Err(e) => {
            state.borrow_mut().status = format!("⚠ {e}");
            false
        }
    }
}

/// O título do trabalho de "Depurar no aparelho" (o fim dele vai para a barra de status).
const DEVICE_DEBUG: &str = "Preparando a depuração no aparelho";

/// "Depurar no aparelho": põe o `lldb-server` no app aberto no Android (o Executar já o
/// deixa depurável), grava "aylis: anexar (Android)" e abre o VS Code (CodeLLDB).
pub fn debug_device(state: &Shared) {
    if !crate::trust::allowed(state) {
        return;
    }
    let (project, device) = {
        let st = state.borrow();
        (st.project.clone(), st.run_device.clone())
    };
    let Some(project) = project else {
        state.borrow_mut().status = "Abra um projeto para depurar".into();
        return;
    };
    // Nas outras linguagens a lógica não está na biblioteca nativa do app.
    if project.language != Language::Rust {
        state.borrow_mut().status = "Depurar no aparelho: só projetos em Rust".into();
        return;
    }
    start(state, Slot::Environment, DEVICE_DEBUG, move |_, _| {
        let debug = aylis_tools::debug::android_debug(&project, device.as_deref())?;
        aylis_tools::debug::open_vscode(&project)?;
        Ok(format!("App no aparelho (pid {}): no VS Code, F5 em \"aylis: anexar (Android)\"", debug.pid))
    });
}

/// Publicar o projeto (as opções já foram gravadas no `.aylisproj`).
pub fn publish(state: &Shared, target: Target, out_dir: Option<PathBuf>) {
    publish_with(state, target, out_dir, PublishExtras::default());
}

/// O que o diálogo de publicação escolhe além da plataforma.
#[derive(Clone, Debug, Default)]
pub struct PublishExtras {
    /// Android: AAB (Play Store) em vez de APK.
    pub bundle: bool,
    /// Windows: o instalador .msi.
    pub installer: bool,
    /// Android: a senha da chave do desenvolvedor (só em memória).
    pub key_password: Option<String>,
    /// Android: as arquiteturas desta publicação (vazio: as do projeto, `publish.android.abis`).
    pub abis: Vec<String>,
}

pub fn publish_with(state: &Shared, target: Target, out_dir: Option<PathBuf>, extras: PublishExtras) {
    if !crate::trust::allowed(state) {
        return;
    }
    crate::actions::save_all(state);
    let Some(project) = state.borrow().project.clone() else { return };
    let title = format!("Publicando {} — {}", project.app_name(), target.name());
    let key = build_key(&project, target, "publicar");
    let started = start(state, Slot::Main, title, move |control, sender| {
        prepare(&project, target, None, control, sender)?;
        require(project.language, target, sender)?;
        let mut options = publish::Options::new(target);
        options.out_dir = out_dir;
        options.aylis_host = aylis_host();
        options.android_bundle = extras.bundle;
        options.installer = extras.installer;
        options.abis = extras.abis.clone();
        options.signing = match (&extras.key_password, project.publish.keystore.is_empty()) {
            (Some(p), false) => Some(publish::Signing {
                keystore: project.file(&project.publish.keystore),
                alias: project.publish.key_alias.clone(),
                store_password: p.clone(),
                key_password: p.clone(),
            }),
            _ => publish::Signing::from_env(&project),
        };
        let before = aylis_tools::touched::before(&project);
        let mut sink = |e| sender.send(Msg::Event(e));
        let published = publish::publish(&project, &options, control, &mut sink);
        before.report(&mut sink);
        let published = published?;
        sender.send(Msg::Published(published.main, published.dir));
        Ok("Publicação concluída.".into())
    });
    if started {
        track_build(state, key);
    }
}

/// Verifica o ambiente de `targets` (para o diálogo): cada plataforma chega assim que fica
/// pronta. As de `force` rodam tudo de novo; as outras usam o resultado guardado quando as
/// marcas dele não mudaram (`aylis_tools::env_cache`).
pub fn check_environment(state: &Shared, targets: Vec<Target>, force: Vec<Target>) {
    let language = state.borrow().project.as_ref().map_or(Language::Rust, |p| p.language);
    let pending = targets.clone();
    let started = start(state, Slot::Environment, "Verificando o ambiente", move |_, sender| {
        let host = aylis_host();
        let mut request = env::Request::new(language, &targets, host.as_deref());
        request.force = force;
        let shared = std::sync::Mutex::new(sender.duplicate());
        env::check_with(&request, &|t, checks| {
            if let Ok(s) = shared.lock() {
                s.send(Msg::Checks(vec![t], checks.to_vec()));
            }
        });
        Ok(String::new())
    });
    if started {
        state.borrow_mut().env_pending = pending;
    }
}

/// Quais linguagens já estão instaladas (para o assistente; rápido, numa thread).
pub fn check_languages(state: &Shared) {
    start(state, Slot::Environment, "Verificando as linguagens", move |_, sender| {
        aylis_tools::setup::refresh_path();
        // Primeiro o rápido (o compilador), para o assistente já mostrar; depois o que falta.
        let ready: Vec<(Language, bool)> = Language::ALL.into_iter().map(|l| (l, aylis_tools::setup::language_ready(l))).collect();
        sender.send(Msg::Languages(ready.iter().map(|&(l, r)| (l, r, Vec::new())).collect()));
        let host = aylis_host();
        let full = ready
            .into_iter()
            .map(|(l, r)| {
                let missing = if r { aylis_tools::setup::missing(l, host.as_deref()) } else { Vec::new() };
                (l, r, missing)
            })
            .collect();
        sender.send(Msg::Languages(full));
        Ok(String::new())
    });
}

/// Instala e configura as linguagens marcadas no assistente: o compilador, o depurador e o
/// servidor de linguagem de cada uma, sem perguntas (`aylis_tools::setup`).
pub fn setup_languages(state: &Shared, languages: Vec<Language>) {
    let names: Vec<&str> = languages.iter().map(|l| l.name()).collect();
    let title = format!("Configurando {}", names.join(", "));
    start(state, Slot::Environment, title, move |control, sender| {
        let mut pending = Vec::new();
        for language in languages {
            sender.send(Msg::Event(Event::Step(format!("Configurando {}", language.name()))));
            let checks = aylis_tools::setup::setup(language, aylis_host().as_deref(), control, &mut |e| sender.send(Msg::Event(e)));
            if control.is_cancelled() {
                return Err(CANCELLED.into());
            }
            let missing: Vec<&str> = checks.iter().filter(|c| c.status == env::Status::Missing).map(|c| c.name.as_str()).collect();
            let line = if missing.is_empty() && aylis_tools::setup::language_ready(language) {
                format!("✔ {}: pronta", language.name())
            } else {
                pending.push(language.name());
                format!("✖ {}: ainda falta {}", language.name(), missing.join(", "))
            };
            sender.send(Msg::Event(Event::Line(line, LineKind::Tool)));
        }
        if pending.is_empty() { Ok("Linguagens configuradas.".into()) } else { Err(format!("Não ficaram prontas: {}", pending.join(", "))) }
    });
}

/// Roda a correção de um item (o usuário já confirmou) e verifica de novo: tudo de novo nas
/// plataformas que tinham o mesmo item (o Rust que falta aparece em todas), e pelas marcas
/// nas outras do projeto.
pub fn apply_fix(state: &Shared, fix: Fix) {
    let (language, affected, targets) = {
        let st = state.borrow();
        let affected: Vec<Target> = Target::ALL.into_iter().filter(|t| st.checks.iter().any(|c| c.target == *t && c.fix.as_ref() == Some(&fix))).collect();
        let mut targets = crate::tooldialogs::project_targets(&st);
        for t in &affected {
            if !targets.contains(t) {
                targets.push(*t);
            }
        }
        (st.project.as_ref().map_or(Language::Rust, |p| p.language), affected, targets)
    };
    let pending = targets.clone();
    let started = start(state, Slot::Environment, format!("Corrigindo: {}", fix.description), move |control, sender| {
        if fix.interactive {
            let (program, args) = fix.command.split_first().ok_or("comando vazio")?;
            // Instaladores mostram a própria janela: só inicia (sem esconder o console).
            std::process::Command::new(program).args(args).spawn().map_err(|e| format!("não consegui iniciar {program}: {e}"))?;
            sender.send(Msg::Event(Event::Line(
                format!("Iniciado: {}. Quando terminar, clique em \"Verificar de novo\".", fix.command_line()),
                LineKind::Tool,
            )));
            return Ok(String::new());
        }
        fix.run(control, &mut |e| sender.send(Msg::Event(e)))?;
        let host = aylis_host();
        let mut request = env::Request::new(language, &targets, host.as_deref());
        request.force = affected;
        let shared = std::sync::Mutex::new(sender.duplicate());
        env::check_with(&request, &|t, checks| {
            if let Ok(s) = shared.lock() {
                s.send(Msg::Checks(vec![t], checks.to_vec()));
            }
        });
        Ok("Corrigido.".into())
    });
    if started {
        state.borrow_mut().env_pending = pending;
    }
}

/// Clique numa linha das mensagens. O código abre no editor da IDE, na linha (abria no VS Code;
/// conferência de 08/11).
pub fn activate(ui: &mut Ui, state: &Shared, action: &OutAction) {
    let result = match action {
        OutAction::Code(l) => {
            state.borrow_mut().code_view = true;
            crate::codewindow::open(ui, &l.file, (l.line as usize).max(1)).map(|_| format!("Código: {}:{}", l.file.display(), l.line))
        }
        OutAction::Folder(dir) => open_folder(dir).map(|_| format!("Pasta aberta: {}", dir.display())),
        OutAction::Component(form, component) => {
            crate::actions::open_form_path(state, form);
            let mut st = state.borrow_mut();
            let root = st.doc.root;
            match st.doc.ui.find_in(root, component) {
                Some(id) => {
                    st.doc.select(Some(id));
                    Ok(format!("Selecionado: {component}"))
                }
                None => Ok(format!("Form aberto: {}", form.display())),
            }
        }
    };
    state.borrow_mut().status = result.unwrap_or_else(|e| format!("⚠ {e}"));
}

/// Um problema da verificação nas mensagens: o clique abre o form no componente ou o código
/// na linha.
fn issue_line(project: &aylis_project::Project, issue: &aylis_project::check::Issue) -> OutputLine {
    let place = match (issue.form.as_str(), issue.component.as_str()) {
        ("", _) => String::new(),
        (form, "") => format!("{form}: "),
        (form, component) => format!("{form}.{component}: "),
    };
    let mut line = OutputLine::new(format!("{place}{}", issue.message), if issue.error { OutKind::Error } else { OutKind::Warning });
    let handler = issue.message.split(" chama ").nth(1).and_then(|r| r.split(',').next()).map(str::to_string);
    line.action = if let Some(at) = aylis_tools::location_in(&issue.message, &project.dir()) {
        Some(OutAction::Code(at))
    } else if let (Some(handler), false) = (handler, issue.form.is_empty()) {
        // O handler que falta: o arquivo do form, onde ele deveria estar.
        aylis_project::sync::handler_location(project, project.language, &issue.form, &handler).map(|(file, line)| OutAction::Code(Location { file, line: line as u32, column: 1 }))
    } else {
        project.form(&issue.form).map(|f| OutAction::Component(project.dir().join(&f.file), issue.component.clone()))
    };
    line
}

/// O que o Executar e o Publicar fazem antes de compilar: gerar o código (`generate`, com os
/// arquivos abertos no editor, que acompanha o que a geração puser neles) e verificar o projeto
/// para a plataforma `target`, com os problemas indo para as mensagens. Com algum erro, não
/// compila.
fn prepare(project: &aylis_project::Project, target: Target, generate: Option<&[PathBuf]>, control: &Control, sender: &Sender) -> Result<(), String> {
    let mut options = aylis_tools::build::Options::new(target);
    options.generate = generate.is_some();
    options.compile = false;
    compile(project, &options, generate, control, sender).map(|_| ())
}

/// Compila o desktop pelo compilador (gerar, verificar, compilar) e devolve o executável: o
/// Depurar, que roda o programa no depurador da IDE.
pub fn compile_desktop(project: &aylis_project::Project, release: bool, open_files: &[PathBuf], control: &Control, sender: &Sender) -> Result<PathBuf, String> {
    let mut options = aylis_tools::build::Options::new(Target::Desktop);
    options.release = release;
    let (_, exe) = compile(project, &options, Some(open_files), control, sender)?;
    exe.ok_or_else(|| "o compilador não informou o executável".to_string())
}

/// O compilador: o `aylis-tool build` em outro processo (gerar, verificar e compilar), com os
/// eventos dele nas mensagens. A geração e a verificação de um projeto grande (o ERP de 1009
/// forms levava uns 5 s) não pesam mais na IDE, e a linha de comando faz o mesmo que ela.
/// `open_files`: os arquivos abertos no editor antes da geração.
/// Devolve a mensagem final e o executável compilado (quando compilou).
fn compile(project: &aylis_project::Project, options: &aylis_tools::build::Options, open_files: Option<&[PathBuf]>, control: &Control, sender: &Sender) -> Result<(String, Option<PathBuf>), String> {
    use aylis_tools::build::{self, BuildEvent};
    let mut before = open_files.map(crate::codewindow::snapshot);
    let mut built = None;
    let mut sink = |e: BuildEvent| match e {
        BuildEvent::Event(e) => sender.send(Msg::Event(e)),
        BuildEvent::Generated => sender.send(Msg::Generated(before.take().unwrap_or_default())),
        BuildEvent::Issues(issues) => sender.send(Msg::Issues(issues)),
        BuildEvent::Built(exe) => built = Some(exe),
    };
    let message = match aylis_tool() {
        Some(tool) => match build::build_external(&tool, project, options, control, &mut sink) {
            // Um aylis-tool de antes do `build` (o do repositório ainda não recompilado): faz aqui.
            Err(e) if e.starts_with(build::NOT_A_COMPILER) => {
                sender.send(Msg::Event(Event::Line(format!("⚠ {e}: gerando e verificando dentro da IDE (recompile o aylis-tools)"), LineKind::Tool)));
                compile_here(project, options, control, &mut sink)
            }
            other => other,
        },
        None => compile_here(project, options, control, &mut sink),
    }?;
    Ok((message, built))
}

/// O [`compile`] dentro da IDE, sem o `aylis-tool` ao lado dela (os testes, que rodam sem ele).
fn compile_here(project: &aylis_project::Project, options: &aylis_tools::build::Options, control: &Control, sink: &mut dyn FnMut(aylis_tools::build::BuildEvent)) -> Result<String, String> {
    let mut ui = crate::document::new_document_ui(false);
    ui.set_design_mode(false);
    aylis_tools::build::build(project, options, Some(&mut ui), control, sink)
}

/// Abre a pasta no gerenciador de arquivos.
pub fn open_folder(dir: &Path) -> Result<(), String> {
    let program = if cfg!(windows) {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    std::process::Command::new(program).arg(dir).spawn().map(|_| ()).map_err(|e| e.to_string())
}

/// Um passo que já é o programa (ou a instalação dele) rodando: a compilação acabou.
fn starts_program(step: &str) -> bool {
    ["Executando", "Instalando", "Esperando o depurador", "Depurar ", "Depurando"].iter().any(|p| step.starts_with(p))
}

/// `42 s`, `1 min 05 s`.
pub fn elapsed_text(d: std::time::Duration) -> String {
    let s = d.as_secs();
    if s < 60 { format!("{s} s") } else { format!("{} min {:02} s", s / 60, s % 60) }
}

/// A cada quarto de segundo (um `Timer` da IDE): enquanto compila, a barra de status mostra
/// há quanto tempo e, com o tempo da última compilação no mesmo modo, a barra de progresso.
pub fn tick(ui: &mut Ui, state: &Shared, ids: &Ids) {
    // O rastro de eventos que os apps mandaram pelo canal vai para Mensagens.
    let traces = state.borrow().dev.as_ref().map(|d| d.take_traces()).unwrap_or_default();
    if !traces.is_empty() {
        let mut st = state.borrow_mut();
        for line in traces {
            st.push(OutputLine::new(line, OutKind::Program));
        }
        drop(st);
        crate::panels::messages(ui, state, ids);
        ui.request_repaint();
    }
    let progress = {
        let mut st = state.borrow_mut();
        match st.job.as_ref().filter(|j| j.built.is_none()) {
            Some(job) => {
                let elapsed = job.started.elapsed();
                let fraction = job.expected.map(|e| (elapsed.as_secs_f64() / e.as_secs_f64().max(0.1)).min(0.99));
                let text = match (job.expected, fraction) {
                    (Some(e), Some(f)) if elapsed <= e => {
                        format!("{}… {} de ~{} ({}%)", job.title, elapsed_text(elapsed), elapsed_text(e), (f * 100.0).round())
                    }
                    (Some(e), _) => format!("{}… {} (passou dos ~{} da última vez)", job.title, elapsed_text(elapsed), elapsed_text(e)),
                    _ => format!("{}… {}", job.title, elapsed_text(elapsed)),
                };
                st.status = text.clone();
                Some((text, fraction))
            }
            None => None,
        }
    };
    let visible = ui.control(ids.build_progress).is_some_and(|c| c.visible());
    match progress {
        Some((text, fraction)) => {
            let _ = ui.set_prop(ids.status, "text", text);
            if let Some(f) = fraction {
                let _ = ui.set_prop(ids.build_progress, "value", (f * 100.0) as f32);
            }
            if visible != fraction.is_some() {
                ui.control_mut(ids.build_progress).unwrap().set_visible(fraction.is_some());
            }
            ui.request_repaint();
        }
        None if visible => {
            ui.control_mut(ids.build_progress).unwrap().set_visible(false);
            ui.request_repaint();
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// As notas do Aylis no erro padrão do app não saem em vermelho; os erros, sim.
    #[test]
    fn aylis_notes_are_not_errors() {
        assert_eq!(aylis_note_kind("aylis: frmA recarregado (forms/a.json)"), OutKind::Muted);
        assert_eq!(aylis_note_kind("aylis: frmA.btnOk.on_click cancelado por frmA.intercept"), OutKind::Muted);
        assert_eq!(aylis_note_kind("aylis: erro ao tratar um erro: x"), OutKind::Error);
        assert_eq!(aylis_note_kind("thread 'main' panicked at src/main.rs"), OutKind::Error);
    }
}
