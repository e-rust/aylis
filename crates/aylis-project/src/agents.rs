//! O guia para agentes de IA de um projeto novo (a opção "Guia para agentes de IA" do Novo
//! projeto, marcada por padrão; decidido pelo autor em 08/11): o `AGENTS.md` com as regras de
//! quem escreve o app, um `CLAUDE.md` de uma linha que aponta para ele e o `.mcp.json` que liga o
//! agente aberto na pasta do projeto à IDE (`aylis-tool mcp`).
//!
//! O texto vem dos modelos em `templates/agents/` (atualizados junto com o Aylis): uma parte
//! comum e uma por linguagem. Nenhum arquivo que já existe é trocado: o projeto criado dentro de
//! um repositório (`create_at`) guarda o `AGENTS.md` e o `CLAUDE.md` dele, e o `.mcp.json` só
//! ganha o servidor `aylis` se ainda não o tiver.

use std::path::{Path, PathBuf};

use crate::{Error, Language, Project};

/// Os arquivos do guia, na pasta do projeto.
pub const FILES: [&str; 3] = ["AGENTS.md", "CLAUDE.md", ".mcp.json"];

/// A parte comum do guia.
const COMMON: &str = include_str!("../templates/agents/AGENTS.md");

/// A parte de cada linguagem: o que é gerado, o handler, o acesso tipado, a mensagem, a thread
/// e os testes.
fn language_section(language: Language) -> &'static str {
    match language {
        Language::Rust => include_str!("../templates/agents/rust.md"),
        Language::Python => include_str!("../templates/agents/python.md"),
        Language::TypeScript => include_str!("../templates/agents/typescript.md"),
        Language::CSharp => include_str!("../templates/agents/csharp.md"),
        Language::Java => include_str!("../templates/agents/java.md"),
        Language::Go => include_str!("../templates/agents/go.md"),
        Language::Cpp => include_str!("../templates/agents/cpp.md"),
        Language::Pascal => include_str!("../templates/agents/pascal.md"),
    }
}

/// O texto do `AGENTS.md` do projeto.
pub fn guide_text(project: &Project) -> String {
    let file = project.path.file_name().map_or_else(|| format!("{}.{}", project.name, crate::EXTENSION), |n| n.to_string_lossy().to_string());
    COMMON
        .replace("{language_section}", language_section(project.language).trim_end())
        .replace("{name}", &project.name)
        .replace("{language}", project.language.name())
        .replace("{folder}", project.language.folder())
        .replace("{project_file}", &file)
}

/// O `CLAUDE.md`: uma linha que importa o `AGENTS.md` (o Claude Code lê o `@arquivo`).
pub const CLAUDE_LINE: &str = "As regras deste projeto para agentes de IA estão em @AGENTS.md\n";

/// O `.mcp.json` com o servidor `aylis` (a ponte `aylis-tool mcp`), mantendo os outros
/// servidores de um arquivo que já existia. `aylis_tool`: o caminho do executável (sem ele, o
/// nome, achado pelo PATH).
pub fn mcp_config(existing: Option<&str>, aylis_tool: Option<&Path>) -> Option<String> {
    let mut config: serde_json::Value = match existing {
        Some(text) => serde_json::from_str(text.trim_start_matches('\u{feff}')).ok()?,
        None => serde_json::json!({}),
    };
    let servers = config.as_object_mut()?.entry("mcpServers").or_insert_with(|| serde_json::json!({}));
    let servers = servers.as_object_mut()?;
    if servers.contains_key("aylis") {
        return None;
    }
    let command = aylis_tool.map_or_else(|| "aylis-tool".to_string(), |p| p.display().to_string());
    servers.insert("aylis".into(), serde_json::json!({ "command": command, "args": ["mcp"] }));
    serde_json::to_string_pretty(&config).ok().map(|t| t + "\n")
}

/// Grava o guia na pasta do projeto: `AGENTS.md`, `CLAUDE.md` e `.mcp.json` (os que ainda não
/// existem; o `.mcp.json` existente só ganha o `aylis`). Devolve os arquivos gravados.
pub fn write_guide(project: &Project, aylis_tool: Option<&Path>) -> Result<Vec<PathBuf>, Error> {
    let dir = project.dir();
    let mut written = Vec::new();
    let write = |path: &Path, text: &str, written: &mut Vec<PathBuf>| -> Result<(), Error> {
        std::fs::write(path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
        written.push(path.to_path_buf());
        Ok(())
    };
    let agents = dir.join("AGENTS.md");
    if !agents.exists() {
        write(&agents, &guide_text(project), &mut written)?;
    }
    let claude = dir.join("CLAUDE.md");
    if !claude.exists() {
        write(&claude, CLAUDE_LINE, &mut written)?;
    }
    let mcp = dir.join(".mcp.json");
    let existing = std::fs::read_to_string(&mcp).ok();
    // Um `.mcp.json` que não é JSON fica como está.
    if existing.is_none() || existing.as_deref().is_some_and(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}')).is_ok()) {
        if let Some(text) = mcp_config(existing.as_deref(), aylis_tool) {
            write(&mcp, &text, &mut written)?;
        }
    }
    Ok(written)
}
