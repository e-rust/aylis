use std::path::Path;

use aylis_tools::env::{lock_version, parse_version};
use aylis_tools::process::{Level, diagnostic};
use aylis_tools::*;

fn temp_project() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_tools_{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src/forms")).unwrap();
    std::fs::write(dir.join("src/forms/principal.rs"), "fn x() {}\n").unwrap();
    std::fs::write(dir.join("src/main.py"), "print(1)\n").unwrap();
    dir
}

#[test]
fn finds_locations_in_output_lines() {
    let dir = temp_project();
    let rust = location_in("thread 'main' panicked at src/forms/principal.rs:12:5:", &dir).unwrap();
    assert_eq!((rust.file, rust.line, rust.column), (dir.join("src/forms/principal.rs"), 12, 5));
    let py = location_in(&format!("  File \"{}\", line 7, in <module>", dir.join("src/main.py").display()), &dir).unwrap();
    assert_eq!((py.line, py.column), (7, 1));
    assert!(location_in("   Compiling aylis v0.1.0", &dir).is_none());
    assert!(location_in("src/nao_existe.rs:1:1", &dir).is_none(), "só arquivos que existem");
}

#[test]
fn parses_compiler_messages() {
    let dir = Path::new("C:/proj");
    let json: serde_json::Value = serde_json::from_str(
        r#"{"level":"error","message":"mismatched types","code":{"code":"E0308"},
            "spans":[{"file_name":"src/forms/principal.rs","line_start":15,"column_start":22,"is_primary":true}],
            "rendered":"error[E0308]: mismatched types\n"}"#,
    )
    .unwrap();
    let d = diagnostic(&json, dir).unwrap();
    assert_eq!(d.level, Level::Error);
    assert_eq!(d.code.as_deref(), Some("E0308"));
    let l = d.location.unwrap();
    assert_eq!((l.file, l.line, l.column), (dir.join("src/forms/principal.rs"), 15, 22));
    let summary: serde_json::Value = serde_json::from_str(r#"{"level":"error","message":"aborting due to 1 previous error","spans":[]}"#).unwrap();
    assert!(diagnostic(&summary, dir).is_none(), "resumos ficam de fora");
}

#[test]
fn versions_and_names() {
    assert_eq!(parse_version("rustc 1.99.0 (abc 2026-08-27)"), Some((1, 99, 0)));
    assert_eq!(parse_version("Python 3.11.9"), Some((3, 11, 9)));
    assert_eq!(parse_version("sem versão"), None);
    let lock = std::env::temp_dir().join(format!("aylis_tools_lock_{}", std::process::id()));
    std::fs::write(&lock, "[[package]]\nname = \"wasm-bindgen\"\nversion = \"0.2.129\"\n").unwrap();
    assert_eq!(lock_version(&lock, "wasm-bindgen").as_deref(), Some("0.2.129"));
    assert_eq!(file_name("Vendas: 2024?"), "Vendas 2024");
    assert_eq!(Target::from_id("apk"), Some(Target::Android));
    assert_eq!(Target::from_id(Target::Web.id()), Some(Target::Web));
}

#[test]
fn a_cancelled_control_stops_the_process() {
    let control = Control::new();
    let c2 = control.clone();
    let started = std::time::Instant::now();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        c2.cancel();
    });
    let mut command = if cfg!(windows) {
        let mut c = std::process::Command::new("powershell");
        c.args(["-NoProfile", "-Command", "Start-Sleep 20"]);
        c
    } else {
        let mut c = std::process::Command::new("sleep");
        c.arg("20");
        c
    };
    let result = process::run_lines(&mut command, &control, |_, _| {});
    assert_eq!(result, Err(process::CANCELLED.to_string()));
    assert!(started.elapsed().as_secs() < 10);
}

#[test]
fn vscode_debug_configuration() {
    let base = std::env::temp_dir().join(format!("aylis_tools_dbg_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let vscode = project.dir().join(".vscode");
    std::fs::create_dir_all(&vscode).unwrap();
    // uma configuração do usuário que deve continuar
    std::fs::write(vscode.join("launch.json"), "// comentário\n{ \"version\": \"0.2.0\", \"configurations\": [ { \"name\": \"minha\", \"type\": \"node\" } ] }").unwrap();
    debug::write_vscode(&project).unwrap();
    debug::write_vscode(&project).unwrap(); // de novo: não duplica
    let launch: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(vscode.join("launch.json")).unwrap()).unwrap();
    let names: Vec<&str> = launch["configurations"].as_array().unwrap().iter().filter_map(|c| c["name"].as_str()).collect();
    assert_eq!(names, ["aylis: depurar", "minha"]);
    let program = launch["configurations"][0]["program"].as_str().unwrap();
    assert!(program.contains("loja"), "{program}");
    let tasks: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(vscode.join("tasks.json")).unwrap()).unwrap();
    assert_eq!(tasks["tasks"][0]["label"], "aylis: compilar");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn each_host_language_checks_its_debugger() {
    use aylis_project::Language;
    let expected = [
        (Language::Go, "Delve"),
        (Language::CSharp, "netcoredbg"),
        (Language::TypeScript, "js-debug"),
        (Language::Java, "jdb"),
        (Language::Pascal, "CodeLLDB"),
        (Language::Cpp, "CodeLLDB"),
    ];
    for (lang, debugger) in expected {
        let checks = env::check(lang, &[Target::Desktop], None);
        let check = checks.iter().find(|c| c.name.starts_with(debugger)).unwrap_or_else(|| panic!("{}: sem a verificação do {debugger}", lang.name()));
        // Instalado, ou com o instalador (ou a instrução, no jdb).
        assert!(check.status == env::Status::Ok || check.fix.is_some() || check.help.is_some(), "{}: {:?}", lang.name(), check.detail);
        if let Some(fix) = &check.fix {
            assert!(fix.description.contains(debugger.split(' ').next().unwrap()), "{}", fix.description);
        }
    }
}

#[test]
fn ray_tracing_forms_take_the_dxc_compiler_along() {
    let base = std::env::temp_dir().join(format!("aylis_raios_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Raios", aylis_project::Language::Rust).unwrap();
    assert!(!publish::uses_ray_tracing(&project));
    let form = project.file("forms/principal.json");
    let text = std::fs::read_to_string(&form).unwrap();
    let mut layout: serde_json::Value = serde_json::from_str(&text).unwrap();
    let model = serde_json::json!({ "class": "Model3D", "name": "m3d", "ray_traced": true });
    match layout["root"]["children"].as_array_mut() {
        Some(children) => children.push(model),
        None => layout["root"]["children"] = serde_json::json!([model]),
    }
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    assert!(publish::uses_ray_tracing(&project));
    // O DXC, quando a máquina tem (Vulkan SDK, Windows SDK): o dxcompiler.dll primeiro.
    let files = publish::dxc_files();
    if let Some(first) = files.first() {
        assert_eq!(first.file_name().unwrap(), "dxcompiler.dll");
    }
    let _ = std::fs::remove_dir_all(&base);
}
