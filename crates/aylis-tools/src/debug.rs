//! Depuração no VS Code: gera `.vscode/launch.json` e `.vscode/tasks.json` do projeto.
//!
//! - **Rust**: "aylis: depurar" compila (com o `target/` do aylis) e abre o executável no
//!   depurador (`cppvsdbg` no Windows, extensão C/C++; `lldb` nos outros, CodeLLDB). F5 no
//!   VS Code, pontos de parada nos handlers de `src/forms/*.rs`.
//! - **Python**: "aylis: anexar (Python)" conecta no `debugpy` (porta 5678); a IDE executa
//!   o projeto com `python -m debugpy --listen 5678 --wait-for-client src/main.py` e o
//!   programa espera o VS Code anexar.

use std::path::PathBuf;

use aylis_project::{Language, Project};
use serde_json::json;

/// Porta do `debugpy` (Python).
pub const DEBUGPY_PORT: u16 = 5678;
/// A porta do inspetor do Node (TypeScript) e a do JDWP (Java) ao depurar na IDE.
pub const NODE_PORT: u16 = 9229;
pub const JDWP_PORT: u16 = 5005;

/// Escreve (ou atualiza) as configurações do VS Code do projeto. Configurações do usuário
/// com outros nomes são mantidas. Devolve a pasta `.vscode`.
pub fn write_vscode(project: &Project) -> Result<PathBuf, String> {
    let dir = project.dir().join(".vscode");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let target = crate::shared_target_dir();
    let (launch, task) = match project.language {
        Language::Rust => {
            let exe = target.join("debug").join(crate::env::exe(&project.package_name()));
            let kind = if cfg!(windows) { "cppvsdbg" } else { "lldb" };
            let launch = json!({
                "name": "aylis: depurar",
                "type": kind,
                "request": "launch",
                "program": exe.display().to_string(),
                "args": [],
                "cwd": "${workspaceFolder}",
                "preLaunchTask": "aylis: compilar",
            });
            let task = json!({
                "label": "aylis: compilar",
                "type": "shell",
                "command": "cargo",
                "args": ["build", "--bin", project.package_name()],
                "options": { "cwd": format!("${{workspaceFolder}}/{}", project.language.folder()), "env": { "CARGO_TARGET_DIR": target.display().to_string() } },
                "problemMatcher": ["$rustc"],
                "group": "build",
            });
            (launch, Some(task))
        }
        Language::Python => (
            json!({
                "name": "aylis: anexar (Python)",
                "type": "debugpy",
                "request": "attach",
                "connect": { "host": "127.0.0.1", "port": DEBUGPY_PORT },
                "justMyCode": true,
            }),
            None,
        ),
        // As outras rodam pelo aylis-host: o VS Code anexa ao backend em execução.
        other => {
            let name = format!("aylis: anexar ({})", other.name());
            let launch = match other {
                Language::TypeScript => json!({ "name": name, "type": "node", "request": "attach", "port": 9229 }),
                Language::Java => json!({ "name": name, "type": "java", "request": "attach", "hostName": "127.0.0.1", "port": 5005 }),
                Language::Go => json!({ "name": name, "type": "go", "request": "attach", "mode": "local", "processId": "${command:pickProcess}" }),
                Language::CSharp => json!({ "name": name, "type": "coreclr", "request": "attach", "processId": "${command:pickProcess}" }),
                Language::Cpp if cfg!(windows) => json!({ "name": name, "type": "cppvsdbg", "request": "attach", "processId": "${command:pickProcess}" }),
                _ => json!({ "name": name, "type": "cppdbg", "request": "attach", "program": format!("${{workspaceFolder}}/.aylis/build/{}/app", other.folder()), "processId": "${command:pickProcess}", "MIMode": "gdb" }),
            };
            (launch, None)
        }
    };
    merge(&dir.join("launch.json"), "configurations", launch, json!({ "version": "0.2.0" }))?;
    if let Some(task) = task {
        merge(&dir.join("tasks.json"), "tasks", task, json!({ "version": "2.0.0" }))?;
    }
    Ok(dir)
}

/// Põe (ou troca, pelo nome) `item` na lista `key` do arquivo JSON.
fn merge(path: &std::path::Path, key: &str, item: serde_json::Value, base: serde_json::Value) -> Result<(), String> {
    let mut doc: serde_json::Value = std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str(&strip_comments(&t)).ok())
        .unwrap_or(base);
    let name_key = if key == "tasks" { "label" } else { "name" };
    let name = item[name_key].clone();
    let list = doc[key].as_array().cloned().unwrap_or_default();
    let mut list: Vec<serde_json::Value> = list.into_iter().filter(|c| c[name_key] != name).collect();
    list.insert(0, item);
    doc[key] = serde_json::Value::Array(list);
    let text = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// O VS Code aceita comentários `//` nos seus JSON.
fn strip_comments(text: &str) -> String {
    text.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n")
}

/// Abre a pasta do projeto no VS Code.
pub fn open_vscode(project: &Project) -> Result<(), String> {
    crate::shell::run_vscode([project.dir()])
}

/// A depuração de um app no Android, pronta para anexar.
pub struct AndroidDebug {
    /// O processo do app no aparelho.
    pub pid: u32,
    /// Os comandos do `lldb` que conectam no aparelho (antes do `attach`).
    pub commands: Vec<String>,
    /// O `lldb` do NDK (para depurar sem o VS Code).
    pub lldb: Option<PathBuf>,
}

/// Prepara a depuração do app (compilado com `--debug` ou `dev`, que o marcam como depurável) já
/// aberto no aparelho: copia o `lldb-server` do NDK para a pasta do app (`run-as`), sobe o
/// servidor num socket abstrato só dele e grava "aylis: anexar (Android)" no
/// `.vscode/launch.json` (CodeLLDB). As bibliotecas com símbolos vêm do `target` do aylis.
pub fn android_debug(project: &Project, serial: Option<&str>) -> Result<AndroidDebug, String> {
    let sdk = crate::env::android_sdk().ok_or("Android SDK não encontrado (Ferramentas → Verificar ambiente)")?;
    let ndk = crate::env::android_ndk(&sdk).ok_or("NDK não encontrado (Ferramentas → Verificar ambiente)")?;
    let adb = crate::env::adb(&sdk).display().to_string();
    let id = project.identifier();
    let adb_run = |args: &[&str]| -> Result<String, String> {
        let mut all: Vec<&str> = Vec::new();
        if let Some(s) = serial {
            all.extend(["-s", s]);
        }
        all.extend(args);
        let mut command = std::process::Command::new(&adb);
        command.args(&all);
        let out = crate::process::quiet(&mut command).output().map_err(|e| format!("adb: {e}"))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
        if out.status.success() { Ok(text) } else { Err(text.trim().to_string()) }
    };
    let serial = match serial {
        Some(s) => s.to_string(),
        None => adb_run(&["get-serialno"])?.trim().to_string(),
    };
    let abi = crate::publish::device_abi(Some(&serial)).ok_or("nenhum aparelho conectado")?;
    let (arch, triple) = match abi.as_str() {
        "arm64-v8a" => ("aarch64", "aarch64-linux-android"),
        "x86_64" => ("x86_64", "x86_64-linux-android"),
        other => return Err(format!("arquitetura sem depuração: {other}")),
    };
    let prebuilt = ndk.join("toolchains/llvm/prebuilt").join(if cfg!(windows) { "windows-x86_64" } else if cfg!(target_os = "macos") { "darwin-x86_64" } else { "linux-x86_64" });
    let server = std::fs::read_dir(prebuilt.join("lib/clang"))
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|d| d.path().join("lib/linux").join(arch).join("lldb-server"))
        .find(|p| p.is_file())
        .ok_or("o NDK não tem o lldb-server")?;
    // O app precisa ser depurável (o `run-as` recusa os outros).
    adb_run(&["shell", "run-as", &id, "id"]).map_err(|e| format!("{e}: compile com --debug ou execute pela IDE (o `dev` também marca o app como depurável)"))?;
    let pid: u32 = adb_run(&["shell", "pidof", &id])?.trim().split_whitespace().next().and_then(|p| p.parse().ok()).ok_or("o app não está aberto no aparelho")?;
    adb_run(&["push", &server.display().to_string(), "/data/local/tmp/lldb-server"])?;
    adb_run(&["shell", "run-as", &id, "sh", "-c", "'pkill -x lldb-server; cp /data/local/tmp/lldb-server ./lldb-server && chmod 700 ./lldb-server'"])?;
    let socket = format!("/{id}/lldb.sock");
    // Em segundo plano no aparelho: continua depois que o adb volta.
    adb_run(&["shell", "run-as", &id, "sh", "-c", &format!("'./lldb-server platform --server --listen unix-abstract://{socket} > /dev/null 2>&1 &'")])?;
    // `--debug` compila em debug; o Executar, em release com o `dev`.
    let symbols = crate::shared_target_dir().join(triple);
    let commands = vec![
        "platform select remote-android".to_string(),
        format!("platform connect unix-abstract-connect://{serial}{socket}"),
        format!("settings append target.exec-search-paths \"{}\" \"{}\"", symbols.join("debug").display(), symbols.join("release").display()),
    ];
    let dir = project.dir().join(".vscode");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let launch = json!({
        "name": "aylis: anexar (Android)",
        "type": "lldb",
        "request": "attach",
        "pid": pid,
        "initCommands": commands,
    });
    merge(&dir.join("launch.json"), "configurations", launch, json!({ "version": "0.2.0" }))?;
    let lldb = prebuilt.join("bin").join(crate::env::exe("lldb"));
    Ok(AndroidDebug { pid, commands, lldb: lldb.is_file().then_some(lldb) })
}

/// O Android pelo Wi-Fi: com o aparelho no cabo (ou o escolhido), liga o `adb` pela rede
/// (`adb tcpip 5555`), acha o IP do Wi-Fi dele e conecta (`adb connect`). Devolve o novo
/// endereço do aparelho (`ip:5555`), que o Executar e o `debug-android` usam como `--serial`.
pub fn android_wifi(serial: Option<&str>) -> Result<String, String> {
    let sdk = crate::env::android_sdk().ok_or("Android SDK não encontrado (Ferramentas → Verificar ambiente)")?;
    let adb = crate::env::adb(&sdk).display().to_string();
    let run = |args: &[&str]| -> Result<String, String> {
        let mut command = std::process::Command::new(&adb);
        command.args(args);
        let out = crate::process::quiet(&mut command).output().map_err(|e| format!("adb: {e}"))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
        if out.status.success() { Ok(text) } else { Err(text.trim().to_string()) }
    };
    let serial = match serial {
        Some(s) => s.to_string(),
        None => run(&["get-serialno"])?.trim().to_string(),
    };
    // O IP do Wi-Fi (`wlan0`), antes de trocar o adb para a rede.
    let route = run(&["-s", &serial, "shell", "ip", "-f", "inet", "addr", "show", "wlan0"])?;
    let ip = route
        .split_whitespace()
        .skip_while(|w| *w != "inet")
        .nth(1)
        .and_then(|cidr| cidr.split('/').next())
        .map(str::to_string)
        .ok_or("o aparelho não está num Wi-Fi (sem IP em wlan0)")?;
    run(&["-s", &serial, "tcpip", "5555"])?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    let address = format!("{ip}:5555");
    let answer = run(&["connect", &address])?;
    if !answer.contains("connected") {
        return Err(format!("adb connect {address}: {}", answer.trim()));
    }
    Ok(address)
}
