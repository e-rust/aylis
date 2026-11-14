//! Executar (F9): compila e roda o projeto na plataforma escolhida, com a saída do
//! programa nas mensagens. Termina quando o programa termina ou com [`Control::cancel`].

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::{Language, Project};
use serde_json::Value;

use crate::env;
use crate::process::{Control, Event, LineKind, Sink, cargo, run_tool};
use crate::publish::{self, device_abi};
use crate::{Target, serve};

#[derive(Clone, Debug)]
pub struct Options {
    pub target: Target,
    /// O `aylis-host` (projetos Python).
    pub aylis_host: Option<PathBuf>,
    /// Android: número de série do aparelho (vazio = o primeiro conectado).
    pub device: Option<String>,
    /// Roteiro de teste (desktop): os passos e onde gravar o estado. Vai na linha do stdin
    /// do app (veja `aylis_remote::script`), nunca em variável de ambiente.
    pub script: Option<(Value, PathBuf)>,
    /// Python: espera o depurador do VS Code (`debugpy`) antes de começar.
    pub debug: bool,
    /// Depurar dentro da IDE (as linguagens do `aylis-host`): o app sobe sem esperar, com o
    /// inspetor do Node ou o JDWP do Java abertos, e o `aylis-host` informa o processo da
    /// lógica (`AYLIS_BACKEND_PID=`), onde o depurador da linguagem se anexa.
    pub ide_debug: bool,
    /// Desktop em Rust: compila otimizado (`cargo build --release`), ainda com o canal da IDE.
    pub release: bool,
    /// Desktop: abre no tamanho de um aparelho (`aylis_core::devices`, "Nome" ou "Nome;deitado"),
    /// com a área segura e a barra de status dele (`AYLIS_DEVICE`).
    pub frame: Option<String>,
    /// A porta em que o canal de desenvolvimento da IDE ouve, quando não é a do projeto (a
    /// padrão estava ocupada): o app no desktop recebe em `AYLIS_DEV_PORT`, o do Android pelo
    /// `adb reverse`.
    pub dev_port: Option<u16>,
}

impl Options {
    pub fn new(target: Target) -> Self {
        Self { target, aylis_host: None, device: None, script: None, debug: false, ide_debug: false, release: false, frame: None, dev_port: None }
    }
}

/// O projeto tem a feature `dev` (o canal com a IDE só entra ao executar pela IDE).
fn has_dev(project: &Project) -> bool {
    std::fs::read_to_string(project.code_dir().join("Cargo.toml")).is_ok_and(|c| c.contains("\ndev ="))
}

/// A linha que vai no stdin do app de desenvolvimento: o alvo do canal (`dev`, só no
/// aylis-host; o app em Rust já leva o dele embutido) e o roteiro de teste, se houver.
fn start_line(dev: Option<Value>, options: &Options) -> Option<String> {
    let mut line = serde_json::Map::new();
    if let Some(dev) = dev {
        line.insert("dev".into(), dev);
    }
    if let Some((script, output)) = &options.script {
        line.insert("script".into(), script.clone());
        line.insert("output".into(), Value::String(output.display().to_string()));
    }
    (!line.is_empty()).then(|| Value::Object(line).to_string())
}

/// O canal de desenvolvimento no aylis-host (as outras linguagens): o alvo vai pelo stdin
/// (`--dev`), nunca num arquivo ou variável de ambiente.
fn dev_target(project: &Project, sink: Sink) -> Option<Value> {
    match crate::devkeys::target(project) {
        Ok(target) => Some(target.to_json()),
        Err(e) => {
            sink(Event::Line(format!("canal de desenvolvimento desligado: {e}"), LineKind::Tool));
            None
        }
    }
}

/// Liga o canal de desenvolvimento no app que vai ser compilado: grava o alvo (o endereço da
/// IDE, a chave pública e o segredo do projeto) onde o `build.rs` lê. Devolve se o projeto
/// tem a feature `dev`.
fn prepare_dev(project: &Project, sink: Sink) -> bool {
    if !has_dev(project) {
        return false;
    }
    if let Err(e) = crate::devkeys::write_build_target(project, &project.code_dir()) {
        sink(Event::Line(format!("canal de desenvolvimento desligado: {e}"), LineKind::Tool));
        return false;
    }
    true
}

/// Compila e executa. Devolve a mensagem final ("Programa terminou.").
pub fn run(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    if project.language != Language::Python {
        aylis_project::codegen::generate(project)?;
    }
    match (options.target, project.language) {
        (Target::Desktop, Language::Rust) => desktop_rust(project, options, control, sink),
        (Target::Desktop, Language::Python) => desktop_python(project, options, control, sink),
        (Target::Desktop, _) => desktop_host(project, options, control, sink),
        (Target::Linux, Language::Rust) => linux_wsl(project, options, control, sink),
        (_, l) if l.uses_host() => Err(format!("projetos em {} só rodam no desktop (para {}, troque a linguagem para Rust)", l.name(), options.target.name())),
        (Target::Android, _) => android(project, options, control, sink),
        (Target::Web, _) => web(project, options, control, sink),
        (Target::Ios, _) => ios(project, options, control, sink),
        (Target::Linux, _) => Err("o Linux pelo WSL só roda projetos em Rust".into()),
    }
}

/// Linux pelo WSL: compila dentro da distribuição e abre a janela pelo WSLg.
fn linux_wsl(project: &Project, _options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let distro = crate::wsl::distro().ok_or("nenhuma distribuição Linux no WSL (Ferramentas → Verificar ambiente instala o Ubuntu)")?;
    let script = crate::wsl::run_script(project);
    match crate::wsl::run(&distro, &script, control, sink, true)? {
        Some(0) | None => Ok("Programa terminou.".into()),
        Some(3) => Err("o Rust não está instalado no Linux do WSL (Ferramentas → Verificar ambiente instala)".into()),
        Some(c) => Err(format!("terminou com código {c} (a compilação ou o programa; veja as mensagens)")),
    }
}

/// Roda o programa até ele terminar, com a saída nas mensagens.
fn watch(command: &mut Command, control: &Control, sink: Sink) -> Result<String, String> {
    watch_with(command, None, control, sink)
}

/// Como [`watch`], escrevendo `input` no stdin do programa (o alvo do canal para o aylis-host).
fn watch_with(command: &mut Command, input: Option<&str>, control: &Control, sink: Sink) -> Result<String, String> {
    let code = crate::process::run_lines_with_input(command, control, input, |line, is_err| {
        sink(Event::Line(line, if is_err { LineKind::ProgramError } else { LineKind::Program }))
    })?;
    Ok(match code {
        Some(0) | None => "Programa terminou.".to_string(),
        Some(c) => format!("Programa terminou com código {c}."),
    })
}

/// Compila o executável do desktop e devolve o caminho, com o canal da IDE (a feature `dev`,
/// se o projeto tiver) e, com `release`, otimizado. Usado pelo Executar, pelo Compilar e pela
/// depuração na IDE; a publicação compila sem `dev`.
pub fn build_desktop(project: &Project, release: bool, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    if project.language == Language::Rust {
        aylis_project::codegen::generate(project)?;
    }
    compile_desktop(project, release, control, sink)
}

/// O [`build_desktop`] sem gerar o código: para quem já gerou e verificou antes (o
/// `aylis-tool build`, que não gera duas vezes).
pub fn compile_desktop(project: &Project, release: bool, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    close_or_refuse_running(project, release, sink)?;
    build_commands(project, &project.output.before_build, None, control, sink)?;
    let exe = build_desktop_exe(project, release, control, sink)?;
    build_commands(project, &project.output.after_build, Some(&exe), control, sink)?;
    Ok(exe)
}

/// Os executáveis que a compilação vai gravar: o do cargo (no `target`) e, com pasta de saída
/// (Projeto → Opções), a cópia de lá.
fn written_exes(project: &Project, release: bool) -> Vec<PathBuf> {
    let built = crate::shared_target_dir().join(if release { "release" } else { "debug" }).join(crate::env::exe(&project.bin_name()));
    let mut out = vec![built.clone()];
    out.extend(project.output_exe(&built));
    out
}

/// O arquivo existe e está em uso (um executável rodando não abre para escrita): a compilação
/// não conseguiria substituí-lo.
fn in_use(path: &Path) -> bool {
    path.is_file() && std::fs::OpenOptions::new().write(true).open(path).is_err()
}

/// Antes de compilar: algum executável que a compilação vai gravar está em uso? Com
/// `close_running` no projeto, encerra só os processos daquele arquivo; senão, para logo. Não
/// olha o nome do processo: outra cópia do app (a instalada, a de outra pasta) não atrapalha a
/// compilação, e recusar por ela estava errado (pedido do autor em 11/11: "o Aylis deve verificar
/// se o exe gerado já está em uso e não pode ser substituído").
fn close_or_refuse_running(project: &Project, release: bool, sink: Sink) -> Result<(), String> {
    let busy: Vec<PathBuf> = written_exes(project, release).into_iter().filter(|p| in_use(p)).collect();
    if busy.is_empty() {
        return Ok(());
    }
    let list = busy.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ");
    if !project.output.close_running {
        return Err(format!(
            "{list} está aberto e não pode ser substituído: feche esse app antes de compilar, ou ligue \"Encerrar o app aberto antes de executar\" em Projeto → Opções do projeto (Geral)"
        ));
    }
    sink(Event::Line(format!("Encerrando o app aberto de {list} (Opções do projeto)"), LineKind::Tool));
    for path in &busy {
        close_processes_of(path);
    }
    // O sistema leva um instante para soltar o arquivo.
    for _ in 0..30 {
        if !busy.iter().any(|p| in_use(p)) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Err(format!("não consegui encerrar o app aberto de {list}"))
}

/// Encerra o app do projeto que roda de um executável que a compilação grava (o `target`, em
/// depuração e em release, e a pasta de saída), mesmo aberto por outra IDE ou por uma sessão
/// anterior desta: o Parar não o alcançava depois de reabrir a IDE (relatório do InsightDB,
/// 11/11). Devolve se havia algum.
pub fn stop_project_app(project: &Project) -> bool {
    let busy: Vec<PathBuf> = written_exes(project, false).into_iter().chain(written_exes(project, true)).filter(|p| in_use(p)).collect();
    for path in &busy {
        close_processes_of(path);
    }
    !busy.is_empty()
}

/// Encerra os processos cujo executável é `path` (só eles: outra cópia do app, de outra pasta,
/// continua aberta).
fn close_processes_of(path: &Path) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};
        let system = System::new_with_specifics(RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_exe(UpdateKind::Always)));
        let target = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        for process in system.processes().values() {
            let same = process.exe().is_some_and(|exe| exe.canonicalize().unwrap_or_else(|_| exe.to_path_buf()) == target);
            if same {
                process.kill();
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = path;
}

/// Os comandos antes ou depois da compilação, um de cada vez, na pasta do projeto. Só em
/// projeto confiado (é a exceção à diretriz "texto não vira comando", decidida pelo autor em
/// 08/11): o texto vai para um `.bat` ou `.ps1` em `.aylis/` (o `cmd /C` não entende aspas
/// escapadas nos argumentos), rodado sem shell no meio. Um que falha para a compilação.
fn build_commands(project: &Project, commands: &[aylis_project::BuildCommand], exe: Option<&Path>, control: &Control, sink: Sink) -> Result<(), String> {
    if commands.is_empty() {
        return Ok(());
    }
    if !crate::trust::is_trusted(&project.dir()) {
        return Err("o projeto tem comandos antes ou depois da compilação, e só roda comandos de projeto confiado (Projeto → Opções do projeto → Confiar neste projeto)".into());
    }
    let dir = project.dir().join(".aylis");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for (i, c) in commands.iter().enumerate() {
        sink(Event::Step(format!("{} ({}): {}", if exe.is_some() { "Depois de compilar" } else { "Antes de compilar" }, c.shell.name(), c.command)));
        let (file, mut command) = match c.shell {
            aylis_project::BuildShell::Cmd => {
                if !cfg!(windows) {
                    sink(Event::Line(format!("o cmd só existe no Windows: pulado ({})", c.command), LineKind::ProgramError));
                    continue;
                }
                let file = dir.join(format!("build-command-{i}.bat"));
                // lint-allow: cmd-shell (o comando antes ou depois da compilação, só em projeto confiado)
                let mut command = Command::new("cmd");
                command.args(["/D", "/C"]).arg(&file);
                (file, command)
            }
            aylis_project::BuildShell::PowerShell => {
                let file = dir.join(format!("build-command-{i}.ps1"));
                let mut command = Command::new(if cfg!(windows) { "powershell" } else { "pwsh" });
                command.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-File"]).arg(&file);
                (file, command)
            }
        };
        let text = if c.shell == aylis_project::BuildShell::Cmd { format!("@echo off\r\n{}\r\n", c.command) } else { format!("{}\n", c.command) };
        std::fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))?;
        command.current_dir(project.dir()).env("AYLIS_PROJECT_DIR", project.dir());
        if let Some(exe) = exe {
            command.env("AYLIS_OUTPUT_EXE", exe);
        }
        let result = run_tool(&mut command, control, sink, LineKind::Program);
        let _ = std::fs::remove_file(&file);
        result.map_err(|e| format!("o comando \"{}\" falhou: {e}", c.command))?;
    }
    Ok(())
}

fn build_desktop_exe(project: &Project, release: bool, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    sink(Event::Step(format!("Compilando {}{}…", project.name, if release { " (release)" } else { "" })));
    // O executável pelo `[[bin]]` do Cargo.toml, que pode não ter o nome do pacote (relatório do
    // RTSPlayer, 10/11: "no bin target named 'rtsplayer-app'").
    let bin = project.bin_name();
    // O canal com a IDE (inspetor, hot reload, rastro): só no que a IDE compila para rodar.
    let dev = prepare_dev(project, sink);
    let mut args = vec!["build", "--bin", bin.as_str()];
    if release {
        args.push("--release");
    }
    if dev {
        args.extend(["--features", "dev"]);
    }
    let built = cargo("cargo", &args, &project.code_dir(), &[], control, sink)?;
    if built.warnings > 0 {
        sink(Event::Line(format!("{} aviso(s)", built.warnings), LineKind::Tool));
    }
    let exe = built
        .artifacts
        .iter()
        .filter(|a| a.kinds.iter().any(|k| k == "bin"))
        .find_map(|a| a.executable.clone())
        .ok_or_else(|| "o cargo não informou o executável".to_string())?;
    // Com pasta de saída (Projeto → Opções), roda de lá, ao lado do que o app precisa.
    let Some(target) = project.output_exe(&exe) else { return Ok(exe) };
    if let Some(dir) = target.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("não consegui criar a pasta de saída {}: {e}", dir.display()))?;
    }
    std::fs::copy(&exe, &target).map_err(|e| format!("não consegui copiar o executável para {} (ele está aberto?): {e}", target.display()))?;
    sink(Event::Line(format!("Executável em {}", target.display()), LineKind::Tool));
    Ok(target)
}

fn desktop_rust(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let exe = build_desktop(project, options.release, control, sink)?;
    sink(Event::Step(format!("Executando {}…", project.name)));
    let mut command = Command::new(exe);
    command.current_dir(project.dir()).env("RUST_BACKTRACE", "1");
    frame_env(&mut command, options);
    watch_with(&mut command, start_line(None, options).as_deref(), control, sink)
}

fn desktop_python(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let host = options.aylis_host.clone().filter(|h| h.is_file()).ok_or("aylis-host não encontrado ao lado da IDE (cargo build -p aylis-host)")?;
    let (python, _) = env::python().ok_or("Python não encontrado (Ferramentas → Verificar ambiente)")?;
    let mut args = vec!["--".to_string(), python, "-u".to_string()];
    if options.debug {
        let port = crate::debug::DEBUGPY_PORT.to_string();
        // Sem os módulos congelados do Python 3.11+, o depurador não perde pontos de parada.
        args.extend(["-Xfrozen_modules=off", "-m", "debugpy", "--listen", port.as_str(), "--wait-for-client"].map(str::to_string));
        sink(Event::Step(format!("Esperando o depurador do VS Code (debugpy na porta {port}): F5 em \"aylis: anexar (Python)\"")));
    } else {
        sink(Event::Step(format!("Executando {} (Python)…", project.name)));
    }
    args.push("src/main.py".into());
    let mut command = Command::new(host);
    let line = start_line(dev_target(project, sink), options);
    if line.is_some() {
        command.arg("--dev");
    }
    command.arg("--project").arg(&project.path).args(&args).current_dir(project.code_dir());
    frame_env(&mut command, options);
    watch_with(&mut command, line.as_deref(), control, sink)
}

/// TypeScript, C#, Java, Go, C++ e Free Pascal: compila o backend em `.aylis/build` e
/// executa pelo aylis-host com o projeto.
fn desktop_host(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let host = options.aylis_host.clone().filter(|h| h.is_file()).ok_or("aylis-host não encontrado ao lado da IDE (cargo build -p aylis-host)")?;
    let tc = crate::hostlang::find(project.language)?;
    let out = project.dir().join(".aylis").join("build").join(project.language.folder());
    let mut backend = crate::hostlang::build(project, &tc, &out, false, control, sink)?;
    if (options.ide_debug || options.debug) && project.language == Language::TypeScript && !backend.iter().any(|a| a == "--experimental-strip-types") {
        // O inspetor do Bun não é o do Node: para depurar, o TypeScript roda no Node.
        let (node, _) = crate::hostlang::node()?;
        backend.splice(0..1, [node.display().to_string(), "--experimental-strip-types".into(), "--no-warnings".into()]);
    }
    if options.ide_debug {
        // Depurar na IDE: o depurador se anexa ao processo que já roda.
        match project.language {
            Language::TypeScript => backend.insert(1, format!("--inspect=127.0.0.1:{}", crate::debug::NODE_PORT)),
            Language::Java => backend.insert(1, format!("-agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=127.0.0.1:{}", crate::debug::JDWP_PORT)),
            _ => {}
        }
        sink(Event::Step(format!("Executando {} ({}) para depurar na IDE…", project.name, project.language.name())));
    } else if options.debug {
        // Depurar: o processo espera o depurador do VS Code anexar (onde a linguagem deixa).
        match project.language {
            Language::TypeScript => backend.insert(1, "--inspect-brk=9229".into()),
            Language::Java => backend.insert(1, "-agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=5005".into()),
            _ => {}
        }
        sink(Event::Step(format!("Depurar {}: no VS Code, F5 em \"aylis: anexar ({})\"", project.name, project.language.name())));
    } else {
        sink(Event::Step(format!("Executando {} ({})…", project.name, project.language.name())));
    }
    let mut command = Command::new(host);
    let line = start_line(dev_target(project, sink), options);
    if line.is_some() {
        command.arg("--dev");
    }
    command.arg("--project").arg(&project.path).arg("--").args(&backend).current_dir(project.code_dir());
    if options.ide_debug {
        command.env("AYLIS_DEBUG_PID", "1");
    }
    frame_env(&mut command, options);
    watch_with(&mut command, line.as_deref(), control, sink)
}

/// O aparelho do designer: a janela abre do tamanho dele (só com o `dev`, o canal da IDE).
/// As variáveis do app de desktop: o aparelho do designer e a porta do canal da IDE.
fn frame_env(command: &mut Command, options: &Options) {
    if let Some(frame) = options.frame.as_deref().filter(|f| !f.is_empty()) {
        command.env("AYLIS_DEVICE", frame);
    }
    if let Some(port) = options.dev_port {
        command.env("AYLIS_DEV_PORT", port.to_string());
    }
}

fn android(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let sdk = env::android_sdk().ok_or("Android SDK não encontrado (Ferramentas → Verificar ambiente)")?;
    let serial = match &options.device {
        Some(s) => s.clone(),
        None => env::android_devices(&sdk).into_iter().next().ok_or("nenhum aparelho conectado: ligue um emulador ou conecte um celular com a depuração USB")?,
    };
    let abi = device_abi(Some(&serial)).unwrap_or_else(|| "arm64-v8a".into());
    let mut publish = publish::Options::new(Target::Android);
    publish.abis = vec![abi];
    publish.out_dir = Some(run_dir(project, "android"));
    let dev = prepare_dev(project, sink);
    if dev {
        publish.features.push("dev".into());
    }
    let apk = publish::publish(project, &publish, control, sink)?.main;
    let adb = env::adb(&sdk);
    let adb_cmd = |args: &[&str]| {
        let mut c = Command::new(&adb);
        c.args(["-s", serial.as_str()]).args(args);
        c
    };
    sink(Event::Step(format!("Instalando em {serial}…")));
    run_tool(adb_cmd(&["install", "-r"]).arg(&apk), control, sink, LineKind::Tool)?;
    let id = project.identifier();
    let _ = run_tool(&mut adb_cmd(&["shell", "am", "force-stop", &id]), control, sink, LineKind::Tool);
    run_tool(&mut adb_cmd(&["logcat", "-c"]), control, sink, LineKind::Tool)?;
    // O canal com a IDE: o endereço vai embutido no app; num endereço local, a porta do
    // aparelho vem para esta máquina (emulador ou celular pelo cabo).
    let address = crate::devkeys::address(project);
    let reverse = (dev && address.starts_with("127.0.0.1:")).then(|| {
        let device = address.rsplit(':').next().unwrap_or_default().to_string();
        // A porta do aparelho é a embutida; a desta máquina, a que a IDE abriu.
        let host = options.dev_port.map_or_else(|| device.clone(), |p| p.to_string());
        (format!("tcp:{device}"), format!("tcp:{host}"))
    });
    if let Some((device, host)) = &reverse {
        run_tool(&mut adb_cmd(&["reverse", device, host]), control, sink, LineKind::Tool)?;
    }
    run_tool(&mut adb_cmd(&["shell", "am", "start", "-n", &format!("{id}/dev.aylis.AylisActivity")]), control, sink, LineKind::Tool)?;
    sink(Event::Step("Executando no aparelho (a saída do programa aparece aqui; Parar encerra)…".into()));
    // A saída do Rust (println!, eprintln!, panics) vai para o logcat com a etiqueta RustStdoutStderr.
    let mut logcat = vec!["logcat", "-v", "raw", "-s", "RustStdoutStderr:V"];
    loop {
        let result = watch(&mut adb_cmd(&logcat), control, sink);
        // O logcat só acaba sozinho quando o aparelho some (o cabo saiu): espera ele voltar,
        // refaz o túnel do canal (o app reconecta sozinho) e continua; Parar encerra.
        if control.is_cancelled() || env::android_device_state(&sdk, &serial).as_deref() == Some("device") {
            return result;
        }
        sink(Event::Step(format!("O aparelho {serial} saiu: esperando o cabo voltar (Parar encerra)…")));
        while env::android_device_state(&sdk, &serial).as_deref() != Some("device") {
            control.check()?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        if let Some((device, host)) = &reverse {
            run_tool(&mut adb_cmd(&["reverse", device, host]), control, sink, LineKind::Tool)?;
        }
        sink(Event::Step(format!("O aparelho {serial} voltou: canal refeito, acompanhando o app…")));
        // Daqui em diante, só as linhas novas (sem repetir o que já saiu).
        logcat = vec!["logcat", "-T", "1", "-v", "raw", "-s", "RustStdoutStderr:V"];
    }
}

fn web(project: &Project, _options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let mut publish = publish::Options::new(Target::Web);
    publish.out_dir = Some(run_dir(project, "web"));
    if prepare_dev(project, sink) {
        publish.features.push("dev".into());
    }
    let dir = publish::publish(project, &publish, control, sink)?.dir;
    serve::serve(&dir, true, control, sink)?;
    Ok("Servidor parado.".into())
}

fn ios(project: &Project, _options: &Options, control: &Control, sink: Sink) -> Result<String, String> {
    let mut publish = publish::Options::new(Target::Ios);
    publish.debug = true;
    if prepare_dev(project, sink) {
        publish.features.push("dev".into());
    }
    publish.out_dir = Some(run_dir(project, "ios"));
    let app = publish::publish(project, &publish, control, sink)?.main;
    sink(Event::Step("Instalando no simulador…".into()));
    run_tool(Command::new("xcrun").args(["simctl", "install", "booted"]).arg(&app), control, sink, LineKind::Tool)?;
    sink(Event::Step("Executando no simulador…".into()));
    watch(Command::new("xcrun").args(["simctl", "launch", "--console-pty", "--terminate-running-process", "booted", &project.identifier()]), control, sink)
}

/// Pasta de trabalho do Executar (fora de `dist/`, que é a do Publicar).
fn run_dir(project: &Project, target: &str) -> PathBuf {
    crate::shared_target_dir().join("aylis-run").join(project.package_name()).join(target)
}

/// Toca um roteiro de teste no app, sem janela (`aylis_remote::script`), em qualquer
/// linguagem, e devolve o estado final (o JSON gravado em `output`; a tela vai num PNG com o
/// mesmo nome, nas linguagens do aylis-host). O mesmo roteiro tem de dar o mesmo estado em
/// todas as linguagens.
pub fn script(project: &Project, script: &Path, output: &Path, aylis_host: Option<PathBuf>, control: &Control, sink: Sink) -> Result<String, String> {
    let mut options = Options::new(Target::Desktop);
    options.aylis_host = aylis_host;
    let text = std::fs::read_to_string(script).map_err(|e| format!("{}: {e}", script.display()))?;
    let steps: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", script.display()))?;
    options.script = Some((steps, output.to_path_buf()));
    let _ = std::fs::remove_file(output);
    run(project, &options, control, sink)?;
    std::fs::read_to_string(output).map_err(|e| format!("o roteiro não gravou o estado ({}): {e}", output.display()))
}

#[cfg(all(test, windows))]
mod busy_exe_tests {
    use super::*;

    /// Só o executável que a compilação vai gravar conta: uma cópia rodando dele está em uso e é
    /// encerrada pelo caminho; outro arquivo com o mesmo nome, noutra pasta, não (pedido do autor
    /// em 11/11: o NoOBS instalado aberto não pode impedir a compilação).
    #[test]
    fn only_the_written_exe_counts_as_busy() {
        let dir = std::env::temp_dir().join(format!("aylis_exe_em_uso_{}", std::process::id()));
        let other = dir.join("outra");
        std::fs::create_dir_all(&other).unwrap();
        let exe = dir.join("app.exe");
        std::fs::copy(r"C:\Windows\System32\PING.EXE", &exe).unwrap();
        std::fs::copy(r"C:\Windows\System32\PING.EXE", other.join("app.exe")).unwrap();
        let mut child = Command::new(&exe).args(["-n", "30", "127.0.0.1"]).stdout(std::process::Stdio::null()).spawn().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(in_use(&exe), "a cópia rodando está em uso");
        assert!(!in_use(&other.join("app.exe")), "o de outra pasta, com o mesmo nome, não");
        close_processes_of(&exe);
        let _ = child.wait();
        assert!(!in_use(&exe), "encerrado pelo caminho, o arquivo fica livre");
        let _ = std::fs::remove_dir_all(dir);
    }
}