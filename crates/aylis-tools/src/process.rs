//! Processos externos (cargo, adb, o programa do usuário) com a saída linha a linha,
//! cancelamento e as mensagens do compilador estruturadas.

use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};

use serde_json::Value;

use crate::Location;

/// De onde vem uma linha.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineKind {
    /// Progresso de uma ferramenta (cargo, aapt2...).
    Tool,
    /// Saída normal do programa do usuário.
    Program,
    /// Saída de erro do programa do usuário (`eprintln!`, exceções do Python).
    ProgramError,
}

/// Gravidade de uma mensagem do compilador.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
    Note,
}

/// Uma mensagem do compilador com o local (a lista de erros clicável).
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub level: Level,
    /// A mensagem curta (`cannot find value `x` in this scope`).
    pub message: String,
    /// Código do erro (`E0425`), se houver.
    pub code: Option<String>,
    pub location: Option<Location>,
    /// O texto completo como o compilador mostra (com o trecho do código).
    pub rendered: String,
}

/// Andamento de um trabalho.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Uma etapa começou ("Compilando...", "Instalando no aparelho...").
    Step(String),
    /// Uma linha de saída.
    Line(String, LineKind),
    Diagnostic(Diagnostic),
}

/// Recebe o andamento (a IDE manda para a thread da interface).
pub type Sink<'a> = &'a mut dyn FnMut(Event);

/// Interrompe um trabalho: mata o processo em andamento e as etapas seguintes não começam.
#[derive(Default)]
pub struct Control {
    cancelled: AtomicBool,
    child: Mutex<Option<Child>>,
}

impl Control {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        if let Ok(mut child) = self.child.lock() {
            if let Some(c) = child.as_mut() {
                let _ = c.kill();
            }
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Erro de "interrompido" se o trabalho foi cancelado.
    pub fn check(&self) -> Result<(), String> {
        if self.is_cancelled() { Err(CANCELLED.into()) } else { Ok(()) }
    }
}

/// Mensagem de erro de um trabalho interrompido.
pub const CANCELLED: &str = "interrompido";

/// Sem janela de console para cada ferramenta (a IDE é um programa de janela).
pub fn quiet(command: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

/// Roda o comando até o fim, mandando cada linha (stdout e stderr) para `on_line`.
/// Devolve o código de saída (`None` se o processo foi morto).
pub fn run_lines(command: &mut Command, control: &Control, on_line: impl FnMut(String, bool)) -> Result<Option<i32>, String> {
    run_lines_with_input(command, control, None, on_line)
}

/// Como [`run_lines`], escrevendo `input` (uma linha) no stdin do processo logo ao iniciar.
pub fn run_lines_with_input(
    command: &mut Command,
    control: &Control,
    input: Option<&str>,
    mut on_line: impl FnMut(String, bool),
) -> Result<Option<i32>, String> {
    control.check()?;
    let program = command.get_program().to_string_lossy().to_string();
    quiet(command).stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|e| format!("não consegui iniciar {program}: {e}"))?;
    if let (Some(text), Some(mut stdin)) = (input, child.stdin.take()) {
        use std::io::Write;
        let _ = writeln!(stdin, "{text}");
    }
    let (tx, rx) = mpsc::channel::<(String, bool)>();
    let readers: Vec<(Box<dyn Read + Send>, bool)> = [
        child.stdout.take().map(|s| (Box::new(s) as Box<dyn Read + Send>, false)),
        child.stderr.take().map(|s| (Box::new(s) as Box<dyn Read + Send>, true)),
    ]
    .into_iter()
    .flatten()
    .collect();
    for (reader, is_err) in readers {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(reader);
            let mut buf = Vec::new();
            // Linha a linha, aceitando texto que não é UTF-8 (ferramentas do Windows).
            while reader.read_until(b'\n', &mut buf).is_ok_and(|n| n > 0) {
                let line = String::from_utf8_lossy(&buf).trim_end_matches(['\r', '\n']).to_string();
                buf.clear();
                if tx.send((line, is_err)).is_err() {
                    break;
                }
            }
        });
    }
    drop(tx);
    *control.child.lock().map_err(|_| "controle")? = Some(child);
    if control.is_cancelled() {
        control.cancel();
    }
    for (line, is_err) in rx {
        on_line(line, is_err);
    }
    let child = control.child.lock().map_err(|_| "controle")?.take();
    let status = child.map(|mut c| c.wait()).transpose().map_err(|e| e.to_string())?;
    control.check()?;
    Ok(status.and_then(|s| s.code()))
}

/// Roda uma ferramenta: cada linha vira [`Event::Line`] do tipo `kind`. Erro se o código
/// de saída não for 0.
pub fn run_tool(command: &mut Command, control: &Control, sink: Sink, kind: LineKind) -> Result<(), String> {
    let program = Path::new(command.get_program()).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    match run_lines(command, control, |line, _| sink(Event::Line(line, kind)))? {
        Some(0) => Ok(()),
        Some(code) => Err(format!("{program} terminou com erro (código {code})")),
        None => Err(format!("{program} foi interrompido")),
    }
}

/// O que o cargo produziu.
#[derive(Clone, Debug, Default)]
pub struct Artifact {
    /// `bin`, `lib`, `cdylib`...
    pub kinds: Vec<String>,
    pub name: String,
    pub executable: Option<PathBuf>,
    pub filenames: Vec<PathBuf>,
}

/// Resultado de uma compilação.
#[derive(Clone, Debug, Default)]
pub struct CargoOutput {
    pub artifacts: Vec<Artifact>,
    pub errors: usize,
    pub warnings: usize,
}

/// Roda o cargo com `--message-format=json` em `dir`: as mensagens do compilador viram
/// [`Event::Diagnostic`] (só as do projeto: as dos crates do aylis não interessam aqui), o
/// resto vira [`Event::Line`]. `args` é o subcomando e as opções (`["build", "--release"]`).
pub fn cargo(
    program: &str,
    args: &[&str],
    dir: &Path,
    envs: &[(&str, &Path)],
    control: &Control,
    sink: Sink,
) -> Result<CargoOutput, String> {
    // `envs` pode trocar a pasta (os testes usam uma só deles).
    let target = envs.iter().find(|(k, _)| *k == "CARGO_TARGET_DIR").map_or_else(crate::shared_target_dir, |(_, v)| v.to_path_buf());
    // No máximo uma vez por hora, o que nenhum projeto usa mais sai da pasta de compilação.
    if let Some(r) = crate::cleanup::automatic(&target).filter(|r| r.bytes > 0) {
        sink(Event::Line(format!("limpeza do target: {} unidades sem uso, {} MB", r.units, r.bytes / (1024 * 1024)), LineKind::Tool));
    }
    let mut command = Command::new(program);
    command.args(args).args(["--message-format=json", "--color", "never"]).current_dir(dir);
    // Dependências sem informação de depuração (os .pdb delas passam de gigabytes); o
    // código do projeto continua depurável.
    command.args(["--config", "profile.dev.package.\"*\".debug=false"]);
    command.env("CARGO_TARGET_DIR", &target);
    for (k, v) in envs {
        command.env(k, v);
    }
    write_cargo_config(dir, &target);
    let mut out = CargoOutput::default();
    let mut success = None;
    // O que esta compilação usou, para o índice da limpeza (mesmo se falhar: o que compilou
    // está no disco).
    let mut used = crate::build_index::Collector::default();
    let code = run_lines(&mut command, control, |line, _| {
        let Some(json) = line.starts_with('{').then(|| serde_json::from_str::<Value>(&line).ok()).flatten() else {
            sink(Event::Line(line, LineKind::Tool));
            return;
        };
        used.message(&json, &target, dir);
        match json["reason"].as_str() {
            Some("compiler-message") => {
                let manifest = json["manifest_path"].as_str().map(PathBuf::from);
                let own = manifest.as_deref().and_then(Path::parent).is_some_and(|m| same_dir(m, dir));
                if let Some(d) = diagnostic(&json["message"], dir) {
                    match d.level {
                        Level::Error => out.errors += 1,
                        Level::Warning => out.warnings += 1,
                        Level::Note => {}
                    }
                    if own || d.level == Level::Error {
                        sink(Event::Diagnostic(d));
                    }
                }
            }
            Some("compiler-artifact") => {
                let strings = |v: &Value| -> Vec<String> {
                    v.as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default()
                };
                out.artifacts.push(Artifact {
                    kinds: strings(&json["target"]["kind"]),
                    name: json["target"]["name"].as_str().unwrap_or_default().to_string(),
                    executable: json["executable"].as_str().map(PathBuf::from),
                    filenames: strings(&json["filenames"]).into_iter().map(PathBuf::from).collect(),
                });
            }
            Some("build-finished") => success = json["success"].as_bool(),
            _ => {}
        }
    });
    used.finish(&target, dir);
    let code = code?;
    if code == Some(0) && success != Some(false) {
        Ok(out)
    } else if out.errors > 0 {
        Err(format!("a compilação falhou: {} erro(s)", out.errors))
    } else {
        Err(format!("{program} falhou (código {})", code.map_or("?".into(), |c| c.to_string())))
    }
}

/// A primeira linha do `.cargo/config.toml` que o aylis grava (o que não começa com ela é do
/// autor e fica como está).
const CARGO_CONFIG_MARK: &str = "# Gravado pelo aylis";

/// Grava em `<projeto>/.cargo/config.toml` a pasta de compilação compartilhada: o cargo de
/// fora do aylis (no terminal, o servidor de linguagem do editor) usa a mesma pasta, em vez de
/// compilar o aylis inteiro de novo num `target/` do projeto. Fica de fora o que está dentro do
/// repositório do aylis (os exemplos de lá cuidam disso no próprio repositório).
pub fn write_cargo_config(dir: &Path, target: &Path) {
    if !dir.join("Cargo.toml").is_file() {
        return;
    }
    let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    if canon(dir).starts_with(canon(&crate::aylis_root())) {
        return;
    }
    let file = dir.join(".cargo").join("config.toml");
    let current = std::fs::read_to_string(&file).ok();
    if current.as_deref().is_some_and(|c| !c.starts_with(CARGO_CONFIG_MARK)) {
        return;
    }
    // Uma string literal do TOML (entre aspas simples): as barras invertidas valem como estão.
    let text = format!(
        "{CARGO_CONFIG_MARK}: o cargo fora do aylis (terminal, editor) compila na mesma pasta\n# que a IDE (Opções → Rust e Python). Pode apagar: o aylis grava de novo.\n[build]\ntarget-dir = '{}'\n",
        target.display()
    );
    if current.as_deref() != Some(text.as_str()) {
        let _ = std::fs::create_dir_all(file.parent().unwrap()).and_then(|_| std::fs::write(&file, text));
    }
    // A pasta de compilação é desta máquina (o caminho tem o nome do usuário): o arquivo não vai
    // para o git do projeto. O `.gitignore` ao lado dele vale em qualquer repositório.
    let ignore = dir.join(".cargo").join(".gitignore");
    if !ignore.exists() {
        let _ = std::fs::write(&ignore, format!("{CARGO_CONFIG_MARK}: o config.toml é desta máquina.\nconfig.toml\n.gitignore\n"));
    }
}

fn same_dir(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

/// Uma mensagem do compilador (o objeto `message` do JSON do cargo), com os caminhos
/// relativos a `dir`.
pub fn diagnostic(message: &Value, dir: &Path) -> Option<Diagnostic> {
    let level = match message["level"].as_str()? {
        "error" | "error: internal compiler error" => Level::Error,
        "warning" => Level::Warning,
        "note" | "help" => Level::Note,
        _ => return None,
    };
    let text = message["message"].as_str()?.to_string();
    // "aborting due to N previous errors" e "N warnings emitted": resumos, não mensagens.
    if message["spans"].as_array().is_some_and(Vec::is_empty) && (text.starts_with("aborting due to") || text.contains("warning emitted") || text.contains("warnings emitted")) {
        return None;
    }
    let location = message["spans"].as_array().and_then(|spans| {
        let span = spans.iter().find(|s| s["is_primary"].as_bool() == Some(true)).or(spans.first())?;
        let file = PathBuf::from(span["file_name"].as_str()?);
        let file = if file.is_absolute() { file } else { dir.join(file) };
        Some(Location {
            file,
            line: span["line_start"].as_u64().unwrap_or(1) as u32,
            column: span["column_start"].as_u64().unwrap_or(1) as u32,
        })
    });
    Some(Diagnostic {
        level,
        message: text,
        code: message["code"]["code"].as_str().map(str::to_string),
        location,
        rendered: message["rendered"].as_str().unwrap_or_default().trim_end().to_string(),
    })
}

/// Faz o processo `child` morrer junto com este (o da IDE): no Windows, ele entra num Job
/// Object com "matar ao fechar", que o sistema fecha quando este processo termina, de
/// qualquer jeito (fechou, caiu, foi morto). Para os servidores que a IDE inicia (de
/// linguagem, de depuração): um teste ou a IDE fechando no meio da partida deixava o
/// `rust-analyzer` órfão, gastando CPU e memória. Nos outros sistemas não faz nada (lá o
/// servidor sai sozinho quando a entrada dele fecha).
pub fn kill_with_parent(child: &Child) {
    #[cfg(windows)]
    win_job::assign(child);
    #[cfg(not(windows))]
    let _ = child;
}

/// Faz os processos que este inicia (o cargo de uma compilação) morrerem junto com ele: o
/// `aylis-tool build` que a IDE interrompe não deixa o cargo compilando sozinho. No Windows,
/// este processo entra num Job Object com "matar ao fechar" (os filhos herdam o Job); nos outros
/// sistemas não faz nada.
pub fn children_die_with_me() {
    #[cfg(windows)]
    win_job::assign_current();
}

#[cfg(windows)]
mod win_job {
    use std::os::windows::io::AsRawHandle;
    use std::process::Child;
    use std::sync::OnceLock;

    type Handle = *mut core::ffi::c_void;

    #[repr(C)]
    #[derive(Default)]
    struct BasicLimits {
        per_process_user_time_limit: i64,
        per_job_user_time_limit: i64,
        limit_flags: u32,
        minimum_working_set_size: usize,
        maximum_working_set_size: usize,
        active_process_limit: u32,
        affinity: usize,
        priority_class: u32,
        scheduling_class: u32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct ExtendedLimits {
        basic: BasicLimits,
        io: [u64; 6],
        process_memory_limit: usize,
        job_memory_limit: usize,
        peak_process_memory_used: usize,
        peak_job_memory_used: usize,
    }

    const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x2000;
    const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION: i32 = 9;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn CreateJobObjectW(attributes: *mut core::ffi::c_void, name: *const u16) -> Handle;
        fn SetInformationJobObject(job: Handle, class: i32, info: *const core::ffi::c_void, length: u32) -> i32;
        fn AssignProcessToJobObject(job: Handle, process: Handle) -> i32;
        fn GetCurrentProcess() -> isize;
    }

    /// O Job do processo (criado na primeira vez; nunca fechado: o sistema fecha ao sair).
    fn job() -> Option<usize> {
        static JOB: OnceLock<Option<usize>> = OnceLock::new();
        *JOB.get_or_init(|| unsafe {
            let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
            if job.is_null() {
                return None;
            }
            let mut limits = ExtendedLimits::default();
            limits.basic.limit_flags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let ok = SetInformationJobObject(job, JOB_OBJECT_EXTENDED_LIMIT_INFORMATION, &limits as *const _ as *const _, std::mem::size_of::<ExtendedLimits>() as u32);
            (ok != 0).then_some(job as usize)
        })
    }

    /// Este processo no Job: quando ele termina (ou é morto), o sistema fecha o Job e mata os
    /// filhos que ficaram.
    pub fn assign_current() {
        if let Some(job) = job() {
            // SAFETY: `job` é um handle válido (criado acima e nunca fechado) e
            // `GetCurrentProcess` devolve o pseudo-handle deste processo, sempre válido.
            unsafe {
                AssignProcessToJobObject(job as Handle, GetCurrentProcess() as Handle);
            }
        }
    }

    pub fn assign(child: &Child) {
        if let Some(job) = job() {
            unsafe {
                AssignProcessToJobObject(job as Handle, child.as_raw_handle() as Handle);
            }
        }
    }
}
