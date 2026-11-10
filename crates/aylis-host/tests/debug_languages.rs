//! Depurar na IDE as linguagens do aylis-host: o app roda pelo aylis-host no modo "depurar na
//! IDE", o aylis-host informa o processo da lógica (`AYLIS_BACKEND_PID=`), o depurador da
//! linguagem se anexa (CodeLLDB, Delve, netcoredbg, js-debug, jdb) e para num ponto de
//! parada dentro de um handler (um Timer de 300 ms chama o handler sem parar).
//!
//! Abre a janela do app e precisa dos compiladores e dos depuradores (Executar → Verificar
//! ambiente): `cargo test -p aylis-host --test debug_languages -- --ignored`. Uma
//! linguagem sem compilador ou sem depurador é pulada (com aviso).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use aylis_project::{Language, codegen};
use aylis_tools::dap::{self, Breakpoint, Event, Session};
use aylis_tools::{Control, Target, hostlang, run};

/// O corpo vazio que a IDE gera para o handler `tique` e o que o teste põe (uma linha
/// marcada, onde fica o ponto de parada).
fn body(lang: Language) -> (&'static str, &'static str, &'static str) {
    match lang {
        Language::TypeScript => ("void aylis, ev;", "void aylis, ev; const marca = 1; void marca;", "const marca"),
        Language::CSharp => (
            "public static void tique(Aylis aylis, AylisEvent ev)\n    {\n",
            "public static void tique(Aylis aylis, AylisEvent ev)\n    {\n        int marca = 1; _ = marca;\n",
            "int marca",
        ),
        Language::Java => ("public static void tique(Aylis aylis, Aylis.Event ev) {\n", "public static void tique(Aylis aylis, Aylis.Event ev) {\n        int marca = 1;\n", "int marca"),
        Language::Go => ("tique(app *aylis.Aylis, ev aylis.Event) {\n", "tique(app *aylis.Aylis, ev aylis.Event) {\n\tmarca := 1\n\t_ = marca\n", "marca := 1"),
        Language::Cpp => ("    (void)ev;\n", "    (void)ev;\n    int marca = 1; (void)marca;\n", "int marca"),
        Language::Pascal => ("Event: TJSONObject);\nbegin\n", "Event: TJSONObject);\nbegin\n  if Event = nil then Exit;\n", "if Event = nil"),
        _ => unreachable!(),
    }
}

fn attach(lang: Language, pid: u32, project: &aylis_project::Project, bps: &BTreeMap<PathBuf, Vec<Breakpoint>>) -> Option<Result<Session, String>> {
    Some(match lang {
        Language::Pascal | Language::Cpp => Session::attach_native(&dap::codelldb()?, pid, bps),
        Language::Go => Session::attach_go(&dap::delve()?, pid, bps),
        Language::CSharp => Session::attach_dotnet(&dap::netcoredbg()?, pid, bps),
        Language::TypeScript => {
            let node = aylis_tools::settings::tool("node").or_else(|| which("node"))?;
            wait_port(aylis_tools::debug::NODE_PORT);
            Session::attach_node(&dap::js_debug()?, &node, aylis_tools::debug::NODE_PORT, bps)
        }
        Language::Java => {
            wait_port(aylis_tools::debug::JDWP_PORT);
            let tool = PathBuf::from(env!("CARGO_BIN_EXE_aylis-host")).with_file_name(if cfg!(windows) { "aylis-tool.exe" } else { "aylis-tool" });
            if !tool.is_file() {
                eprintln!("Java: sem o aylis-tool em {} (cargo build -p aylis-tools)", tool.display());
                return None;
            }
            Session::attach_java(&tool, &dap::jdb()?, aylis_tools::debug::JDWP_PORT, &project.code_dir().join("src"), bps)
        }
        _ => return None,
    })
}

fn which(exe: &str) -> Option<PathBuf> {
    let exe = if cfg!(windows) { format!("{exe}.exe") } else { exe.to_string() };
    std::env::var_os("PATH").and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(&exe)).find(|p| p.is_file()))
}

fn wait_port(port: u16) {
    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline && std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn try_language(lang: Language) {
    if hostlang::find(lang).is_err() {
        eprintln!("{}: sem compilador, pulado", lang.name());
        return;
    }
    let base = std::env::temp_dir().join(format!("aylis_dbg_{}", lang.name().replace(['#', '+', ' '], "x")));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Loja", lang).unwrap();
    // Um Timer chama o handler `tique` a cada 300 ms.
    let form_path = project.file("forms/principal.json");
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&form_path).unwrap()).unwrap();
    let timer = serde_json::json!({ "class": "Timer", "name": "tmrTique", "interval": 300, "on_timer": "tique" });
    match layout["root"]["children"].as_array_mut() {
        Some(children) => children.push(timer),
        None => layout["root"]["children"] = serde_json::json!([timer]),
    }
    std::fs::write(&form_path, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    codegen::generate(&project).unwrap();
    let source = codegen::form_source(&project, "frmPrincipal").unwrap();
    let text = std::fs::read_to_string(&source).unwrap();
    let (empty, marked, marker) = body(lang);
    assert!(text.contains(empty), "{}: o stub gerado\n{text}", lang.name());
    let text = text.replacen(empty, marked, 1);
    std::fs::write(&source, &text).unwrap();
    let line = text.lines().position(|l| l.contains(marker)).unwrap() + 1;

    // Executa no modo "depurar na IDE" (numa thread: termina quando o app fecha).
    let (tx, rx) = mpsc::channel::<String>();
    let control = Control::new();
    let run_control = std::sync::Arc::clone(&control);
    let run_project = project.clone();
    let runner = std::thread::spawn(move || {
        let mut options = run::Options::new(Target::Desktop);
        options.aylis_host = Some(PathBuf::from(env!("CARGO_BIN_EXE_aylis-host")));
        options.ide_debug = true;
        let mut sink = |e: aylis_tools::Event| {
            if let aylis_tools::Event::Line(t, _) | aylis_tools::Event::Step(t) = e {
                let _ = tx.send(t);
            }
        };
        run::run(&run_project, &options, &run_control, &mut sink)
    });
    let deadline = Instant::now() + Duration::from_secs(300);
    let mut log = Vec::new();
    let pid = loop {
        let line = rx.recv_timeout(deadline.saturating_duration_since(Instant::now())).unwrap_or_else(|_| panic!("{}: o processo não subiu\n{}", lang.name(), log.join("\n")));
        if let Some(pid) = line.trim().strip_prefix("AYLIS_BACKEND_PID=") {
            break pid.parse::<u32>().unwrap();
        }
        log.push(line);
    };
    let mut bps = BTreeMap::new();
    bps.insert(source.clone(), vec![Breakpoint::at(line)]);
    let Some(session) = attach(lang, pid, &project, &bps) else {
        eprintln!("{}: sem o depurador, pulado", lang.name());
        control.cancel();
        let _ = runner.join();
        return;
    };
    let mut session = session.unwrap_or_else(|e| panic!("{}: anexar: {e}", lang.name()));
    // O Timer chama o handler: para no ponto de parada.
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut seen = Vec::new();
    let stopped = loop {
        assert!(Instant::now() < deadline, "{}: não parou no ponto de parada ({:?})", lang.name(), seen);
        let events = session.events();
        if let Some(e) = events.iter().find(|e| matches!(e, Event::Stopped { .. })) {
            break e.clone();
        }
        seen.extend(events);
        std::thread::sleep(Duration::from_millis(100));
    };
    eprintln!("{}: parou: {stopped:?}", lang.name());
    let frames = session.stack().unwrap();
    let top = &frames[0];
    eprintln!("{}: quadro {}:{} {:?}", lang.name(), top.name, top.line, top.file);
    assert_eq!(top.line, line, "{}: a linha do ponto de parada", lang.name());
    let locals = session.locals(top.id).unwrap_or_default();
    eprintln!("{}: variáveis {:?}", lang.name(), locals.iter().map(|v| format!("{}={}", v.name, v.value)).collect::<Vec<_>>());
    session.stop();
    control.cancel();
    let _ = runner.join();
}

#[test]
#[ignore]
fn go_debugs_in_the_ide() {
    try_language(Language::Go);
}

#[test]
#[ignore]
fn csharp_debugs_in_the_ide() {
    try_language(Language::CSharp);
}

#[test]
#[ignore]
fn typescript_debugs_in_the_ide() {
    try_language(Language::TypeScript);
}

#[test]
#[ignore]
fn java_debugs_in_the_ide() {
    try_language(Language::Java);
}

#[test]
#[ignore]
fn pascal_debugs_in_the_ide() {
    try_language(Language::Pascal);
}

#[test]
#[ignore]
fn cpp_debugs_in_the_ide() {
    try_language(Language::Cpp);
}
