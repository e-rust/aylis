//! `aylis-ide --headless --mcp-stdio` (item 38, 1d): a IDE sem janela responde o MCP pelo
//! stdin e pelo stdout, cria um projeto, põe um componente e termina quando o stdin fecha.

#![cfg(feature = "desktop")]

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

#[test]
fn the_headless_ide_answers_mcp_on_stdio() {
    let dir = std::env::temp_dir().join(format!("aylis_headless_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_aylis-ide"))
        .args(["--headless", "--mcp-stdio"])
        // O servidor de sempre fica desligado mesmo se estiver ligado nas Opções de quem roda.
        .env("AYLIS_IDE_NO_MCP", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut next = 0;
    let mut ask = |method: &str, params: Value| -> Value {
        next += 1;
        writeln!(stdin, "{}", json!({ "jsonrpc": "2.0", "id": next, "method": method, "params": params })).unwrap();
        stdin.flush().unwrap();
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        let reply: Value = serde_json::from_str(&line).unwrap_or_else(|e| panic!("{e}: {line}"));
        assert_eq!(reply["id"], next, "{reply}");
        reply
    };
    let text = |reply: &Value| reply["result"]["content"][0]["text"].as_str().unwrap_or_default().to_string();

    let init = ask("initialize", json!({ "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "teste" } }));
    assert_eq!(init["result"]["serverInfo"]["name"], "aylis-ide", "{init}");
    let tools = ask("tools/list", json!({}));
    assert!(tools["result"]["tools"].as_array().unwrap().iter().any(|t| t["name"] == "new_project"));

    let made = ask("tools/call", json!({ "name": "new_project", "arguments": { "dir": dir.display().to_string(), "name": "SemJanela" } }));
    assert_ne!(made["result"]["isError"], true, "{made}");
    let added = ask("tools/call", json!({ "name": "add_component", "arguments": { "class": "Button", "name": "btnOk" } }));
    assert_ne!(added["result"]["isError"], true, "{added}");
    let list = ask("tools/call", json!({ "name": "list_components", "arguments": {} }));
    assert!(text(&list).contains("btnOk"), "{list}");

    // O stdin fechou: a IDE termina.
    drop(stdin);
    let start = std::time::Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(status.success(), "{status}");
            break;
        }
        assert!(start.elapsed().as_secs() < 30, "a IDE sem janela não terminou com o stdin fechado");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// O `aylis-tool ide-script --headless` (2c) com esta IDE: o roteiro cria o projeto, põe um
/// botão e salva; um passo com erro marcado segue, e o resultado diz quantos falharam.
#[test]
fn an_ide_script_runs_in_the_headless_ide() {
    let dir = std::env::temp_dir().join(format!("aylis_headless_script_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let script = dir.join("roteiro.json");
    let text = json!({
        "title": "Teste",
        "steps": [
            {"tool": "new_project", "arguments": {"dir": dir.display().to_string(), "name": "Roteiro"}},
            {"tool": "add_component", "arguments": {"class": "Button", "name": "btnRoteiro"}},
            {"tool": "add_component", "arguments": {"class": "ClasseQueNaoExiste"}, "continue_on_error": true},
            {"tool": "save", "arguments": {"all": true}}
        ]
    });
    std::fs::write(&script, text.to_string()).unwrap();
    let mut ide = Command::new(env!("CARGO_BIN_EXE_aylis-ide"));
    ide.args(["--headless", "--mcp-stdio"]).env("AYLIS_IDE_NO_MCP", "1");
    let mut lines = Vec::new();
    let result = aylis_tools::ide_script::run(&script, &mut ide, true, &mut |l| lines.push(l.to_string()));
    let all = lines.join("\n");
    assert_eq!(result, Err("1 passo(s) com erro".into()), "{all}");
    assert!(all.contains("[2/4] add_component: ok") && all.contains("[3/4] add_component: ERRO") && all.contains("[4/4] save: ok"), "{all}");
    let form = std::fs::read_dir(dir.join("Roteiro").join("forms")).unwrap().flatten().map(|e| std::fs::read_to_string(e.path()).unwrap()).collect::<String>();
    assert!(form.contains("btnRoteiro"), "o save gravou o form com o botão: {form}");
    let _ = std::fs::remove_dir_all(&dir);
}
