//! O guarda de edição: um agente de IA (o Claude Code) não grava direto nos arquivos de um
//! projeto aberto na IDE. A IDE guarda os forms e o código abertos na memória e não vê a
//! mudança no disco: ao salvar, a edição do agente some. Com o MCP da IDE ligado, o agente
//! edita por ele (`edit_code`, `set_property`...), e este guarda recusa o resto.
//!
//! Como: ao abrir um projeto com o MCP ligado, a IDE grava no `.claude/settings.local.json`
//! do projeto (o arquivo local, fora do git) um gancho `PreToolUse` do Claude Code que roda
//! `aylis-ide --guard-edit` antes de cada `Write`, `Edit`, `MultiEdit` e `NotebookEdit`. O
//! guarda lê o pedido (JSON na entrada), olha as IDEs abertas ([`aylis_tools::instances`]) e,
//! se o arquivo é de um projeto aberto numa delas com o MCP ligado, recusa (código de saída
//! 2, o motivo na saída de erro, que o agente lê). Fora isso, deixa.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

/// O argumento da linha de comando que roda o guarda.
pub const FLAG: &str = "--guard-edit";

/// As ferramentas de arquivo do Claude Code que gravam.
const WRITE_TOOLS: &str = "Write|Edit|MultiEdit|NotebookEdit";

/// Um projeto aberto numa IDE: a pasta e a porta do MCP dela.
pub struct OpenProject {
    pub dir: PathBuf,
    pub mcp_port: u16,
    /// As pastas de código (`rust`, `python`...), relativas à do projeto.
    pub code_dirs: Vec<String>,
}

/// Os projetos abertos nas IDEs com o MCP ligado.
pub fn open_projects() -> Vec<OpenProject> {
    aylis_tools::instances::list()
        .into_iter()
        .filter_map(|i| {
            let port = i.mcp_port?;
            let path = i.project?;
            let project = aylis_project::Project::open(&path).ok()?;
            let code_dirs = project.code_languages().into_iter().map(|l| l.folder().to_string()).collect();
            Some(OpenProject { dir: project.dir(), mcp_port: port, code_dirs })
        })
        .collect()
}

/// O que o guarda decide para o pedido do gancho (`input`, o JSON que o Claude Code manda):
/// `Some(motivo)` recusa.
pub fn decide(input: &Value, open: &[OpenProject]) -> Option<String> {
    let tool = input["tool_name"].as_str().unwrap_or_default();
    if !WRITE_TOOLS.split('|').any(|t| t == tool) {
        return None;
    }
    let file = input["tool_input"]["file_path"].as_str().or_else(|| input["tool_input"]["notebook_path"].as_str())?;
    let mut path = PathBuf::from(file);
    if !path.has_root() {
        path = PathBuf::from(input["cwd"].as_str().unwrap_or_default()).join(path);
    }
    let path = normalize(&path);
    for p in open {
        let dir = normalize(&p.dir);
        let Some(rel) = relative(&path, &dir) else { continue };
        if !held_by_ide(&rel, &p.code_dirs) {
            continue;
        }
        return Some(format!(
            "{} é de um projeto aberto na IDE do Aylis ({}), que guarda os forms e o código abertos na memória: \
             gravando direto, a IDE não vê a mudança e a perde ao salvar. Use o MCP da IDE (porta {}): \
             read_code e edit_code para o código, set_property, add_component e set_layout para os forms, save \
             para salvar. Para editar o arquivo direto, feche o projeto na IDE.",
            path.display(),
            dir.display(),
            p.mcp_port
        ));
    }
    None
}

/// Os arquivos que a IDE abre e guarda (relativos à pasta do projeto, com `/`): os forms, os
/// temas, o `.aylisproj` e o código de cada linguagem. O resto (assets, docs, `target`) fica livre.
fn held_by_ide(rel: &str, code_dirs: &[String]) -> bool {
    let first = rel.split('/').next().unwrap_or_default();
    if ["target", ".git", "node_modules", "dist", ".claude"].contains(&first) {
        return false;
    }
    rel.starts_with("forms/") || rel.starts_with("themes/") || rel.ends_with(".aylisproj") || first == "src" || code_dirs.iter().any(|d| d == first)
}

/// O caminho sem `.` e `..`, e com as barras de um jeito só (sem ir ao disco: o arquivo pode
/// ainda não existir).
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// `path` relativo a `dir` (com `/`), se estiver dentro dele (sem diferenciar maiúsculas no
/// Windows e no macOS).
fn relative(path: &Path, dir: &Path) -> Option<String> {
    let fold = |p: &Path| {
        let s = p.to_string_lossy().replace('\\', "/");
        if cfg!(any(windows, target_os = "macos")) { s.to_lowercase() } else { s }
    };
    let (p, d) = (fold(path), fold(dir));
    let d = d.trim_end_matches('/');
    let rest = p.strip_prefix(d)?.strip_prefix('/')?;
    Some(rest.to_string())
}

/// O pedido do gancho. Um que não é JSON (um objeto) é recusado, e não deixado passar: sem
/// ler o pedido, o guarda não sabe se o arquivo é de um projeto aberto (falha fechado; o
/// gancho só roda nas ferramentas que gravam).
pub fn parse(text: &str) -> Result<Value, String> {
    match serde_json::from_str::<Value>(text) {
        Ok(v) if v.is_object() => Ok(v),
        Ok(_) => Err(refuse("não é um objeto".into())),
        Err(_) if text.trim().is_empty() => Err(refuse("entrada vazia".into())),
        Err(e) => Err(refuse(e.to_string())),
    }
}

/// A recusa de um pedido que o guarda não conseguiu ler.
fn refuse(why: String) -> String {
    format!(
        "O guarda de edição do Aylis recebeu um pedido que não é JSON válido ({why}) e recusou a gravação: \
         sem ler o pedido, não dá para saber se o arquivo é de um projeto aberto na IDE. \
         Tente de novo; se repetir, edite pelo MCP da IDE (edit_code, set_property) ou feche o projeto na IDE."
    )
}

/// Anota a recusa de um pedido inválido em `%LOCALAPPDATA%\aylis\edit-guard.log` (o tamanho e
/// o erro; o conteúdo do pedido fica de fora, pode ter o texto de um arquivo).
fn log_invalid(text: &str, reason: &str) {
    let Some(file) = aylis_tools::dap::tools_dir().and_then(|d| d.parent().map(|p| p.join("edit-guard.log"))) else { return };
    let when = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let line = format!("{when} pedido inválido ({} bytes): {reason}\n", text.len());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(file) {
        let _ = std::io::Write::write_all(&mut f, line.as_bytes());
    }
}

/// Roda o guarda: o pedido na entrada, e recusa com o código 2 e o motivo na saída de erro.
pub fn run() -> i32 {
    let mut text = String::new();
    let read = std::io::Read::read_to_string(&mut std::io::stdin(), &mut text);
    let input = match read.map_err(|e| refuse(e.to_string())).and_then(|_| parse(&text)) {
        Ok(input) => input,
        Err(reason) => {
            log_invalid(&text, &reason);
            eprintln!("{reason}");
            return 2;
        }
    };
    match decide(&input, &open_projects()) {
        Some(reason) => {
            eprintln!("{reason}");
            2
        }
        None => 0,
    }
}

/// Põe (ou atualiza) o gancho no `.claude/settings.local.json` da pasta do projeto, com
/// `command` (o caminho desta IDE e o [`FLAG`]); o resto do arquivo fica como está.
pub fn install(project_dir: &Path, command: &str) -> Result<bool, String> {
    let path = project_dir.join(".claude").join("settings.local.json");
    let mut settings: Value = match std::fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?,
        Err(_) => json!({}),
    };
    if !settings.is_object() {
        return Err(format!("{} não é um objeto JSON", path.display()));
    }
    let entry = json!({ "matcher": WRITE_TOOLS, "hooks": [{ "type": "command", "command": command }] });
    let hooks = settings.as_object_mut().unwrap().entry("hooks").or_insert_with(|| json!({}));
    let Some(hooks) = hooks.as_object_mut() else { return Err("`hooks` não é um objeto".into()) };
    let list = hooks.entry("PreToolUse").or_insert_with(|| json!([]));
    let Some(list) = list.as_array_mut() else { return Err("`hooks.PreToolUse` não é uma lista".into()) };
    let ours = |e: &Value| e["hooks"].as_array().is_some_and(|h| h.iter().any(|h| h["command"].as_str().is_some_and(|c| c.contains(FLAG))));
    if list.iter().any(|e| *e == entry) {
        return Ok(false);
    }
    list.retain(|e| !ours(e));
    list.push(entry);
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let text = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(true)
}

/// O comando do gancho para esta IDE (o executável dela, entre aspas).
pub fn command() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    Some(format!("\"{}\" {FLAG}", exe.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open() -> Vec<OpenProject> {
        vec![OpenProject { dir: PathBuf::from("/proj/Loja"), mcp_port: 7350, code_dirs: vec!["rust".into(), "python".into()] }]
    }

    fn ask(tool: &str, file: &str) -> Option<String> {
        decide(&json!({ "tool_name": tool, "tool_input": { "file_path": file }, "cwd": "/proj/Loja" }), &open())
    }

    #[test]
    fn the_files_the_ide_holds_are_refused_and_the_rest_is_free() {
        assert!(ask("Edit", "/proj/Loja/rust/src/forms/principal.rs").is_some_and(|r| r.contains("edit_code") && r.contains("7350")));
        assert!(ask("Write", "/proj/Loja/forms/principal.json").is_some());
        assert!(ask("Edit", "forms/clientes.json").is_some(), "relativo à pasta de trabalho");
        assert!(ask("MultiEdit", "/proj/Loja/Loja.aylisproj").is_some());
        assert!(ask("Write", "/proj/Loja/python/src/forms/principal.py").is_some());
        assert!(ask("Edit", "/proj/Loja/assets/README.md").is_none(), "assets e docs ficam livres");
        assert!(ask("Edit", "/proj/Loja/target/x.rs").is_none());
        assert!(ask("Edit", "/proj/Outro/rust/src/main.rs").is_none(), "outro projeto");
        assert!(ask("Read", "/proj/Loja/rust/src/main.rs").is_none(), "ler pode");
        assert!(ask("Edit", "/proj/Loja/rust/../forms/principal.json").is_some(), "com ..");
        assert!(decide(&json!({ "tool_name": "Edit", "tool_input": { "file_path": "/proj/Loja/forms/a.json" } }), &[]).is_none(), "sem IDE aberta");
    }

    #[test]
    fn a_request_that_is_not_json_is_refused() {
        // Antes passava (código 0): sem ler o pedido, o guarda não sabe o arquivo.
        for text in ["", "   ", "{\"tool_name\": \"Edit\"", "[1, 2]", "nada"] {
            let e = parse(text).unwrap_err();
            assert!(e.contains("não é JSON válido") && e.contains("edit_code"), "{text:?}: {e}");
        }
        assert!(parse(r#"{"tool_name":"Edit","tool_input":{"file_path":"/a"}}"#).is_ok());
    }

    #[test]
    fn the_hook_goes_into_the_local_settings_keeping_the_rest() {
        let dir = std::env::temp_dir().join(format!("aylis-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".claude")).unwrap();
        let file = dir.join(".claude/settings.local.json");
        std::fs::write(&file, r#"{"permissions":{"allow":["Bash(ls)"]},"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"outro"}]}]}}"#).unwrap();
        assert!(install(&dir, "\"C:/aylis/aylis-ide.exe\" --guard-edit").unwrap());
        assert!(!install(&dir, "\"C:/aylis/aylis-ide.exe\" --guard-edit").unwrap(), "já estava: não regrava");
        assert!(install(&dir, "\"D:/novo/aylis-ide.exe\" --guard-edit").unwrap(), "a IDE mudou de lugar");
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
        assert_eq!(v["permissions"]["allow"][0], "Bash(ls)");
        let list = v["hooks"]["PreToolUse"].as_array().unwrap();
        assert_eq!(list.len(), 2, "o outro gancho fica, e o do Aylis é um só: {list:?}");
        assert_eq!(list[1]["hooks"][0]["command"], "\"D:/novo/aylis-ide.exe\" --guard-edit");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
