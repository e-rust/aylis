//! `aylis-tool ide-script roteiro.json`: uma lista de ferramentas do MCP da IDE tocada pela linha
//! de comando, sem um agente no meio (item 38, 2c): tarefas repetidas, como atualizar todos os
//! projetos de exemplo ou gerar a documentação. Fala com a IDE pela mesma ponte do `aylis-tool
//! mcp` (um processo filho; ela acha ou abre a IDE e lê o token), então vale tudo o que vale para
//! um agente: o `project` escolhe a IDE e a IDE confere os argumentos. Com `--headless`, fala
//! direto com a IDE sem janela (`aylis-ide --headless --mcp-stdio`, ao lado do `aylis-tool`), que
//! abre só para o roteiro e fecha no fim: o jeito da CI.
//!
//! ```json
//! {
//!   "title": "Regerar os exemplos",
//!   "projects": ["examples/Paint/Paint.aylisproj", "examples/IBGE/IBGE.aylisproj"],
//!   "steps": [
//!     {"tool": "open_project", "arguments": {"path": "{project}"}},
//!     {"tool": "generate_code"},
//!     {"tool": "check", "continue_on_error": true},
//!     {"tool": "save", "arguments": {"all": true}}
//!   ]
//! }
//! ```
//!
//! Com `projects`, os passos rodam uma vez por projeto, com `{project}` trocado pelo caminho
//! absoluto dele, que também vai como `project` em cada chamada; sem, uma vez só (e o `project`
//! de cima, se houver, vai em todas). O primeiro erro para o roteiro (a não ser no passo com
//! `continue_on_error`).

use serde_json::{Value, json};
use std::path::Path;

/// Uma chamada do roteiro: a ferramenta, os argumentos e se o erro dela para tudo.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub tool: String,
    pub arguments: Value,
    pub continue_on_error: bool,
}

/// O resultado de uma chamada: o texto da resposta e se foi erro.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub text: String,
    pub error: bool,
}

/// Troca `{project}` nos textos dos argumentos.
fn substitute(value: &Value, project: &str) -> Value {
    match value {
        Value::String(s) => Value::String(s.replace("{project}", project)),
        Value::Array(a) => Value::Array(a.iter().map(|v| substitute(v, project)).collect()),
        Value::Object(o) => Value::Object(o.iter().map(|(k, v)| (k.clone(), substitute(v, project))).collect()),
        other => other.clone(),
    }
}

/// As chamadas do roteiro, em ordem (os caminhos relativos dos projetos, a partir de `base`).
pub fn plan(script: &Value, base: &Path) -> Result<Vec<Call>, String> {
    let steps = script["steps"].as_array().ok_or("o roteiro não tem \"steps\"")?;
    let absolute = |p: &str| {
        let path = Path::new(p);
        if path.is_absolute() { p.to_string() } else { base.join(path).to_string_lossy().into_owned() }
    };
    let projects: Vec<Option<String>> = match script["projects"].as_array() {
        Some(list) => list.iter().map(|p| p.as_str().map(absolute).ok_or("\"projects\" é uma lista de caminhos".to_string())).map(|r| r.map(Some)).collect::<Result<_, _>>()?,
        None => vec![script["project"].as_str().map(absolute)],
    };
    let mut out = Vec::new();
    if let Some(title) = script["title"].as_str() {
        let mut arguments = json!({ "title": title });
        if let Some(Some(p)) = projects.first() {
            arguments["project"] = json!(p);
        }
        out.push(Call { tool: "session_title".into(), arguments, continue_on_error: true });
    }
    for project in &projects {
        for (n, step) in steps.iter().enumerate() {
            let tool = step["tool"].as_str().filter(|t| !t.is_empty()).ok_or(format!("passo {}: falta \"tool\"", n + 1))?;
            let mut arguments = step.get("arguments").cloned().unwrap_or_else(|| json!({}));
            if !arguments.is_object() {
                return Err(format!("passo {}: \"arguments\" é um objeto", n + 1));
            }
            if let Some(p) = project {
                arguments = substitute(&arguments, p);
                if arguments.get("project").is_none() {
                    arguments["project"] = json!(p);
                }
            }
            out.push(Call { tool: tool.to_string(), arguments, continue_on_error: step["continue_on_error"].as_bool().unwrap_or(false) });
        }
    }
    Ok(out)
}

/// Direto na IDE sem janela (sem a ponte): o `session_title` e o `project` são da ponte (o
/// título na barra da janela, a escolha da IDE) e saem.
pub fn without_bridge(calls: Vec<Call>) -> Vec<Call> {
    calls
        .into_iter()
        .filter(|c| c.tool != "session_title")
        .map(|mut c| {
            if let Some(o) = c.arguments.as_object_mut() {
                o.remove("project");
            }
            c
        })
        .collect()
}

/// Toca as chamadas com `call` (a ponte, ou um falso nos testes), dizendo cada uma em `say`.
/// Devolve quantas falharam; para no primeiro erro de um passo sem `continue_on_error`.
pub fn play(calls: &[Call], call: &mut dyn FnMut(&Call) -> Result<Outcome, String>, say: &mut dyn FnMut(&str)) -> usize {
    let mut failed = 0;
    for (n, c) in calls.iter().enumerate() {
        let outcome = call(c).unwrap_or_else(|e| Outcome { text: e, error: true });
        say(&format!("[{}/{}] {}: {}", n + 1, calls.len(), c.tool, if outcome.error { "ERRO" } else { "ok" }));
        for line in outcome.text.lines().take(20) {
            say(&format!("    {line}"));
        }
        if outcome.error {
            failed += 1;
            if !c.continue_on_error {
                say("parado no primeiro erro (marque o passo com \"continue_on_error\" para seguir)");
                break;
            }
        }
    }
    failed
}

/// O texto de uma resposta `tools/call` do MCP (o `content` de texto) e se é erro.
pub fn outcome(reply: &Value) -> Outcome {
    if let Some(e) = reply.get("error") {
        return Outcome { text: e["message"].as_str().unwrap_or("erro").to_string(), error: true };
    }
    let result = &reply["result"];
    let text = result["content"].as_array().map(|c| c.iter().filter_map(|b| b["text"].as_str()).collect::<Vec<_>>().join("\n")).unwrap_or_default();
    Outcome { text, error: result["isError"].as_bool().unwrap_or(false) }
}

/// Quem fala o MCP pelo stdin e pelo stdout, como processo filho: a ponte (`aylis-tool mcp`, que
/// acha ou abre a IDE com janela) ou a IDE sem janela (`aylis-ide --headless --mcp-stdio`).
#[cfg(not(target_arch = "wasm32"))]
pub struct Bridge {
    child: std::process::Child,
    /// Fechar (soltar) encerra a IDE sem janela; a ponte também sai.
    stdin: Option<std::process::ChildStdin>,
    stdout: std::io::BufReader<std::process::ChildStdout>,
    next_id: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl Bridge {
    /// Sobe o processo (`command`, com o stdin e o stdout ligados aqui) e faz o aperto de mão.
    pub fn start(command: &mut std::process::Command) -> Result<Bridge, String> {
        use std::process::Stdio;
        let program = command.get_program().to_string_lossy().into_owned();
        let mut child = command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().map_err(|e| format!("{program}: {e}"))?;
        let stdin = child.stdin.take().ok_or("sem stdin da ponte")?;
        let stdout = std::io::BufReader::new(child.stdout.take().ok_or("sem stdout da ponte")?);
        let mut bridge = Bridge { child, stdin: Some(stdin), stdout, next_id: 1 };
        bridge.request("initialize", json!({ "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": { "name": "aylis-tool ide-script", "version": env!("CARGO_PKG_VERSION") } }))?;
        bridge.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }))?;
        Ok(bridge)
    }

    fn send(&mut self, message: &Value) -> Result<(), String> {
        use std::io::Write;
        let stdin = self.stdin.as_mut().ok_or("a ponte já fechou")?;
        writeln!(stdin, "{message}").and_then(|_| stdin.flush()).map_err(|e| format!("a ponte fechou: {e}"))
    }

    /// Um pedido e a resposta dele (os avisos no meio, como o andamento, ficam de fora).
    pub fn request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        use std::io::BufRead;
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }))?;
        loop {
            let mut line = String::new();
            if self.stdout.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
                return Err("a ponte fechou sem responder".into());
            }
            let Ok(reply) = serde_json::from_str::<Value>(&line) else { continue };
            if reply["id"] == json!(id) {
                return Ok(reply);
            }
        }
    }

    pub fn call(&mut self, c: &Call) -> Result<Outcome, String> {
        self.request("tools/call", json!({ "name": c.tool, "arguments": c.arguments })).map(|r| outcome(&r))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Bridge {
    fn drop(&mut self) {
        // Fecha o stdin e espera o processo sair (a IDE sem janela termina o que estava fazendo);
        // se ele não sair em 10 s, encerra. A IDE com janela da ponte continua aberta.
        drop(self.stdin.take());
        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < 10 {
            if matches!(self.child.try_wait(), Ok(Some(_))) {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// O `ide-script`: lê o roteiro, sobe `server` (a ponte ou a IDE sem janela; veja [`Bridge`]) e
/// toca. `Err` se algum passo falhou.
#[cfg(not(target_arch = "wasm32"))]
pub fn run(script_path: &Path, server: &mut std::process::Command, direct: bool, say: &mut dyn FnMut(&str)) -> Result<(), String> {
    let text = std::fs::read_to_string(script_path).map_err(|e| format!("{}: {e}", script_path.display()))?;
    let script: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", script_path.display()))?;
    let base = std::path::absolute(script_path).ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_default();
    let mut calls = plan(&script, &base)?;
    if direct {
        calls = without_bridge(calls);
    }
    let mut bridge = Bridge::start(server)?;
    let failed = play(&calls, &mut |c| bridge.call(c), say);
    if failed == 0 { Ok(()) } else { Err(format!("{failed} passo(s) com erro")) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_steps_repeat_for_each_project() {
        let abs = std::env::temp_dir().join("B.aylisproj").to_string_lossy().into_owned();
        let script = json!({
            "title": "Regerar",
            "projects": ["a/A.aylisproj", abs],
            "steps": [
                {"tool": "open_project", "arguments": {"path": "{project}"}},
                {"tool": "check", "continue_on_error": true}
            ]
        });
        let base = Path::new("/base");
        let calls = plan(&script, base).unwrap();
        let a = base.join("a/A.aylisproj").to_string_lossy().into_owned();
        assert_eq!(calls.len(), 5);
        assert_eq!(calls[0].tool, "session_title");
        assert_eq!(calls[1].arguments, json!({"path": a, "project": a}));
        assert!(calls[2].continue_on_error);
        assert_eq!(calls[4].arguments["project"], abs);
        assert!(plan(&json!({"steps": [{"arguments": {}}]}), base).unwrap_err().contains("falta \"tool\""));
        // Na IDE sem janela, sem o que é da ponte.
        let direct = without_bridge(calls);
        assert_eq!(direct.len(), 4);
        assert_eq!(direct[0].arguments, json!({"path": a}));
    }

    #[test]
    fn the_first_error_stops_unless_marked() {
        let calls = plan(&json!({"steps": [{"tool": "a"}, {"tool": "b", "continue_on_error": true}, {"tool": "c"}, {"tool": "d"}, {"tool": "e"}]}), Path::new(".")).unwrap();
        let mut called = Vec::new();
        let mut lines = Vec::new();
        let failed = play(
            &calls,
            &mut |c| {
                called.push(c.tool.clone());
                Ok(Outcome { text: format!("feito {}", c.tool), error: c.tool == "b" || c.tool == "d" })
            },
            &mut |l| lines.push(l.to_string()),
        );
        assert_eq!(failed, 2);
        assert_eq!(called, ["a", "b", "c", "d"]);
        assert!(lines.iter().any(|l| l == "[2/5] b: ERRO"));
    }

    #[test]
    fn the_reply_text_and_the_error_flag() {
        let ok = json!({"jsonrpc": "2.0", "id": 1, "result": {"content": [{"type": "text", "text": "um"}, {"type": "text", "text": "dois"}]}});
        assert_eq!(outcome(&ok), Outcome { text: "um\ndois".into(), error: false });
        let bad = json!({"jsonrpc": "2.0", "id": 1, "result": {"content": [{"type": "text", "text": "não"}], "isError": true}});
        assert!(outcome(&bad).error);
        assert!(outcome(&json!({"id": 1, "error": {"message": "x"}})).error);
    }
}
