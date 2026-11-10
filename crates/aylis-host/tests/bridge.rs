//! A ponte (`aylis-host --bridge`): a tela conecta por WebSocket, como o app publicado no
//! celular ou no navegador, e ganha um backend próprio (aqui, a demonstração TypeScript).
//! Pulado sem o Bun.

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn a_screen_connects_to_the_bridge_and_gets_its_own_backend() {
    if Command::new("bun").arg("--version").output().is_err() {
        eprintln!("bun não encontrado: pulado");
        return;
    }
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let mut bridge = Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .current_dir(root)
        .args(["--bridge", &format!("127.0.0.1:{port}"), "--", "bun", "clients/typescript/demo.ts", "--selftest"])
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Espera a ponte abrir a porta.
    let deadline = Instant::now() + Duration::from_secs(20);
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(Instant::now() < deadline, "a ponte não abriu");
        std::thread::sleep(Duration::from_millis(50));
    }
    // A tela: vazia, como um app recém-aberto; o backend manda o layout.
    let mut ui = aylis::core::Ui::new();
    aylis::register_all(&mut ui);
    aylis::connect_backend(&mut ui, &format!("ws://127.0.0.1:{port}")).unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut label = String::new();
    while Instant::now() < deadline && !ui.close_requested() {
        ui.process_posted();
        ui.process_events();
        if let Some(id) = ui.find_path("frmDemo.lblMsg") {
            if let Ok(aylis::core::PropValue::String(s)) = ui.get_prop(id, "text") {
                label = s;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(ui.close_requested(), "o backend fecha a tela no fim do roteiro");
    assert_eq!(label, "Olá, Ana! (TypeScript/bun)");
    let _ = bridge.kill();
    let mut stderr = String::new();
    bridge.stderr.take().unwrap().read_to_string(&mut stderr).unwrap();
    assert!(stderr.contains("SELFTEST typescript-bun Olá, Ana!"), "{stderr}");
}

/// Uma tela de teste ligada à ponte, como o app publicado.
fn screen(port: u16) -> aylis::core::Ui {
    let mut ui = aylis::core::Ui::new();
    aylis::register_all(&mut ui);
    aylis::connect_backend(&mut ui, &format!("ws://127.0.0.1:{port}")).unwrap();
    ui
}

/// Roda a fila das telas até `done` ou o prazo.
fn pump(screens: &mut [&mut aylis::core::Ui], secs: u64, done: impl Fn(&[&mut aylis::core::Ui]) -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(secs);
    while Instant::now() < deadline {
        for ui in screens.iter_mut() {
            ui.process_posted();
            ui.process_events();
        }
        if done(screens) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}

fn text(ui: &aylis::core::Ui, path: &str) -> String {
    match ui.find_path(path).map(|id| ui.get_prop(id, "text")) {
        Some(Ok(aylis::core::PropValue::String(s))) => s,
        _ => String::new(),
    }
}

fn click(ui: &mut aylis::core::Ui, path: &str) {
    let id = ui.find_path(path).unwrap();
    ui.fire(id, "on_click", &aylis::core::EventArgs::None);
}

/// O roteiro das sessões, igual em todas as linguagens: um processo para as telas, um
/// contador por tela, uma exceção e um handler lento que ficam só na tela deles, e duas telas
/// do mesmo usuário (`login`) atualizadas juntas pelo `broadcast`. `backend`: o comando do
/// backend de teste da linguagem (`tests/data/sessions_backend.*`).
fn sessions_scenario(lang: &str, backend: &[&str]) {
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let mut bridge = Bridge(Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .current_dir(root)
        .args(["--bridge", &format!("127.0.0.1:{port}"), "--sessions", "--"])
        .args(backend)
        .env("PYTHONIOENCODING", "utf-8")
        .stderr(Stdio::piped())
        .spawn()
        .unwrap());
    let deadline = Instant::now() + Duration::from_secs(20);
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(Instant::now() < deadline, "{lang}: a ponte não abriu");
        std::thread::sleep(Duration::from_millis(50));
    }
    let (mut a, mut b) = (screen(port), screen(port));
    assert!(pump(&mut [&mut a, &mut b], 60, |s| s.iter().all(|ui| ui.find_path("frm.btn").is_some())), "{lang}: as duas telas abriram");

    // Cada tela com o seu contador.
    for _ in 0..3 {
        click(&mut a, "frm.btn");
    }
    click(&mut b, "frm.btn");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[0], "frm.lbl").ends_with(":3") && text(s[1], "frm.lbl").ends_with(":1")), "{lang}: a={} b={}", text(&a, "frm.lbl"), text(&b, "frm.lbl"));
    let (sa, sb) = (text(&a, "frm.lbl"), text(&b, "frm.lbl"));
    assert_ne!(sa.split(':').next(), sb.split(':').next(), "{lang}: sessões diferentes: {sa} {sb}");

    // Uma exceção e um handler lento na tela A não atrapalham a B.
    click(&mut a, "frm.boom");
    click(&mut a, "frm.lento");
    let started = Instant::now();
    click(&mut b, "frm.btn");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[1], "frm.lbl").ends_with(":2")), "{lang}: b={}", text(&b, "frm.lbl"));
    assert!(started.elapsed() < Duration::from_secs(2), "{lang}: a tela B esperou a A: {:?}", started.elapsed());
    click(&mut a, "frm.btn");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[0], "frm.lbl").ends_with(":4")), "{lang}: a={}", text(&a, "frm.lbl"));

    // O mesmo usuário nas duas telas: o que uma muda aparece nas duas.
    click(&mut a, "frm.entrar");
    click(&mut b, "frm.entrar");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[0], "frm.lblu") == "ana" && text(s[1], "frm.lblu") == "ana"), "{lang}: login a={} b={}", text(&a, "frm.lblu"), text(&b, "frm.lblu"));
    click(&mut a, "frm.comum");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[0], "frm.lblu") == "u:1" && text(s[1], "frm.lblu") == "u:1"), "{lang}: a={} b={}", text(&a, "frm.lblu"), text(&b, "frm.lblu"));
    click(&mut b, "frm.comum");
    assert!(pump(&mut [&mut a, &mut b], 10, |s| text(s[0], "frm.lblu") == "u:2" && text(s[1], "frm.lblu") == "u:2"), "{lang}: a={} b={}", text(&a, "frm.lblu"), text(&b, "frm.lblu"));

    // A tela que sai avisa o backend.
    drop(a);
    std::thread::sleep(Duration::from_millis(800));
    let _ = bridge.0.kill();
    let mut bytes = Vec::new();
    bridge.0.stderr.take().unwrap().read_to_end(&mut bytes).unwrap();
    let stderr = String::from_utf8_lossy(&bytes);
    assert_eq!(stderr.matches("backend das sessões iniciado").count(), 1, "{lang}: um processo só: {stderr}");
    assert_eq!(stderr.matches("abriu (pid").count(), 2, "{lang}: {stderr}");
    assert!(stderr.contains("erro de propósito"), "{lang}: a exceção vai para o log: {stderr}");
    assert!(stderr.contains("saiu"), "{lang}: {stderr}");
    drop(b);
}

#[test]
fn sessions_in_python() {
    if Command::new("python").arg("--version").output().is_err() {
        return eprintln!("python não encontrado: pulado");
    }
    sessions_scenario("python", &["python", "crates/aylis-host/tests/data/sessions_backend.py"]);
}

#[test]
fn sessions_in_typescript() {
    let node = Command::new("node").arg("--version").output().ok().map(|o| String::from_utf8_lossy(&o.stdout).trim().trim_start_matches('v').to_string());
    if !node.is_some_and(|v| v.split('.').next().and_then(|m| m.parse::<u32>().ok()).is_some_and(|m| m >= 23)) {
        return eprintln!("Node 23+ não encontrado: pulado");
    }
    sessions_scenario("typescript", &["node", "--experimental-strip-types", "--no-warnings", "crates/aylis-host/tests/data/sessions_backend.ts"]);
}

/// Compila um backend de teste (o comando, na pasta `dir`); `false` se não deu.
fn build_backend(cmd: &mut Command) -> bool {
    match cmd.output() {
        Ok(out) if out.status.success() => true,
        Ok(out) => panic!("compilação: {}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)),
        Err(_) => false,
    }
}

/// A raiz do repositório, sem o prefixo `\?` do Windows (os compiladores não aceitam).
fn root() -> std::path::PathBuf {
    let path = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().unwrap();
    std::path::PathBuf::from(path.to_string_lossy().trim_start_matches(r"\\?\").to_string())
}

#[test]
fn sessions_in_go() {
    let Ok(go) = aylis_tools::hostlang::find(aylis_project::Language::Go).map(|t| t.program) else { return eprintln!("go não encontrado: pulado") };
    let exe = std::env::temp_dir().join(format!("aylis_go_sessions_{}{}", std::process::id(), std::env::consts::EXE_SUFFIX));
    if build_backend(Command::new(&go).current_dir(root().join("clients/go")).args(["build", "-o"]).arg(&exe).arg("./sessions_test_backend")) {
        sessions_scenario("go", &[exe.to_str().unwrap()]);
    }
    let _ = std::fs::remove_file(exe);
}

#[test]
fn sessions_in_csharp() {
    let Ok(dotnet) = aylis_tools::hostlang::find(aylis_project::Language::CSharp).map(|t| t.program) else { return eprintln!("dotnet não encontrado: pulado") };
    let out = std::env::temp_dir().join(format!("aylis_cs_sessions_{}", std::process::id()));
    let project = root().join("crates/aylis-host/tests/data/sessions_csharp");
    let built = build_backend(
        Command::new(&dotnet).arg("build").arg(&project).args(["-c", "Release", "-v", "q", "-o"]).arg(&out).env("DOTNET_CLI_TELEMETRY_OPTOUT", "1").env("DOTNET_NOLOGO", "1"),
    );
    if built {
        let dll = out.join("SessionsBackend.dll");
        sessions_scenario("csharp", &[dotnet.to_str().unwrap(), dll.to_str().unwrap()]);
    }
    let _ = std::fs::remove_dir_all(out);
}

#[test]
fn sessions_in_java() {
    if Command::new("javac").arg("-version").output().is_err() {
        return eprintln!("javac não encontrado: pulado");
    }
    let out = std::env::temp_dir().join(format!("aylis_java_sessions_{}", std::process::id()));
    let client = root().join("clients/java/dev/aylis");
    let built = build_backend(
        Command::new("javac")
            .args(["-encoding", "UTF-8", "-d"])
            .arg(&out)
            .args(["Aylis.java", "Session.java", "User.java", "Server.java"].map(|f| client.join(f)))
            .arg(root().join("crates/aylis-host/tests/data/SessionsBackend.java")),
    );
    if built {
        sessions_scenario("java", &["java", "-cp", out.to_str().unwrap(), "SessionsBackend"]);
    }
    let _ = std::fs::remove_dir_all(out);
}

#[test]
fn sessions_in_cpp() {
    let Ok(cpp) = aylis_tools::hostlang::find(aylis_project::Language::Cpp) else { return eprintln!("C++ não encontrado: pulado") };
    let exe = std::env::temp_dir().join(format!("aylis_cpp_sessions_{}{}", std::process::id(), std::env::consts::EXE_SUFFIX));
    let source = root().join("crates/aylis-host/tests/data/sessions_backend.cpp");
    let built = if cpp.program.extension().is_some_and(|e| e == "bat") {
        // Um .bat: o cmd não entende as aspas escapadas de um argumento.
        let (obj, bat) = (exe.with_extension("obj"), exe.with_extension("bat"));
        let script = format!(
            "@call \"{}\" >nul\r\ncl /nologo /std:c++17 /EHsc /utf-8 /O2 \"{}\" /Fe:\"{}\" /Fo:\"{}\"\r\n",
            cpp.program.display(),
            source.display(),
            exe.display(),
            obj.display()
        );
        std::fs::write(&bat, script).unwrap();
        let ok = build_backend(Command::new("cmd").arg("/C").arg(&bat));
        let _ = std::fs::remove_file(obj);
        let _ = std::fs::remove_file(bat);
        ok
    } else {
        build_backend(Command::new(&cpp.program).args(["-std=c++17", "-O2", "-pthread"]).arg(&source).arg("-o").arg(&exe))
    };
    if built {
        sessions_scenario("cpp", &[exe.to_str().unwrap()]);
    }
    let _ = std::fs::remove_file(exe);
}

/// A ponte do teste: encerrada ao sair, mesmo quando uma conferência falha no meio.
struct Bridge(std::process::Child);

impl Drop for Bridge {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

#[test]
fn sessions_in_free_pascal() {
    let Ok(pascal) = aylis_tools::hostlang::find(aylis_project::Language::Pascal) else { return eprintln!("fpc não encontrado: pulado") };
    let out = std::env::temp_dir().join(format!("aylis_fpc_sessions_{}", std::process::id()));
    std::fs::create_dir_all(&out).unwrap();
    let mut cmd = Command::new(&pascal.program);
    cmd.arg("-Mobjfpc");
    if cfg!(windows) {
        cmd.arg("-Px86_64");
    }
    if let Some(units) = &pascal.extra {
        cmd.arg(format!("-Fu{}", units.join("*").display())).arg(format!("-Fu{}", units.join("rtl").display()));
    }
    cmd.arg(format!("-Fu{}", root().join("clients/pascal").display()))
        .arg(format!("-FU{}", out.display()))
        .arg(format!("-FE{}", out.display()))
        .arg(root().join("crates/aylis-host/tests/data/sessions_backend.lpr"));
    if build_backend(&mut cmd) {
        let exe = out.join(format!("sessions_backend{}", std::env::consts::EXE_SUFFIX));
        sessions_scenario("pascal", &[exe.to_str().unwrap()]);
    }
    let _ = std::fs::remove_dir_all(out);
}
