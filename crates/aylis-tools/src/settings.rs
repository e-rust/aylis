//! Onde estão os compiladores e as ferramentas que não ficam no PATH: um arquivo só
//! (`%LOCALAPPDATA%\aylis\tools.json`, `~/.local/share/aylis/tools.json`), gravado pela aba de
//! cada linguagem nas Opções da IDE e lido pelo `aylis-tool`, pela IDE e pelos testes. No lugar
//! das antigas variáveis `AYLIS_GO`, `AYLIS_DOTNET`, `AYLIS_FPC`...
//!
//! As chaves: `bun`, `node`, `dotnet`, `javac`, `java`, `go`, `vcvars` (o `vcvars64.bat` do
//! Visual C++), `gxx`, `fpc`, `fpc_units` (a pasta das units), `makensis`, `bundletool`,
//! `wix` e `vscode`. E a compilação dos projetos em Rust: `target_dir` (a pasta de
//! compilação de todos os projetos), `cleanup_project_days` e `cleanup_shared_days` (os dias
//! da limpeza, veja [`crate::cleanup`]).

use std::path::PathBuf;

use serde_json::{Map, Value};

/// As ferramentas que a tela de opções mostra, por linguagem: (chave, descrição).
pub const TOOLS: &[(&str, &str)] = &[
    ("bun", "Bun"),
    ("node", "Node.js"),
    ("dotnet", "SDK do .NET (dotnet)"),
    ("javac", "Compilador Java (javac)"),
    ("java", "Java (java)"),
    ("go", "Go"),
    ("vcvars", "Visual C++ (vcvars64.bat)"),
    ("gxx", "g++ ou clang++"),
    ("fpc", "Free Pascal (fpc)"),
    ("fpc_units", "Units do Free Pascal (pasta)"),
    ("makensis", "NSIS (makensis)"),
    ("bundletool", "bundletool (Android)"),
    ("wix", "WiX (wix)"),
    ("vscode", "VS Code (code)"),
];

/// O arquivo das ferramentas.
pub fn path() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join("tools.json"))
}

fn read() -> Map<String, Value> {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
}

/// O caminho escolhido para a ferramenta `key`, se existir no disco.
pub fn tool(key: &str) -> Option<PathBuf> {
    read().get(key).and_then(Value::as_str).filter(|s| !s.is_empty()).map(PathBuf::from).filter(|p| p.exists())
}

/// O texto gravado para `key` (mesmo que o caminho não exista mais), para a tela mostrar.
pub fn tool_text(key: &str) -> String {
    read().get(key).and_then(Value::as_str).unwrap_or_default().to_string()
}

/// Grava (ou apaga, com `None` ou vazio) o caminho de `key`, mantendo os outros.
pub fn set_tool(key: &str, value: Option<&str>) -> Result<(), String> {
    let path = path().ok_or("sem pasta de dados do usuário")?;
    let mut map = read();
    match value.map(str::trim).filter(|v| !v.is_empty()) {
        Some(v) => {
            map.insert(key.to_string(), Value::String(v.to_string()));
        }
        None => {
            map.remove(key);
        }
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// A pasta de compilação escolhida nas Opções (`target_dir`), se houver.
pub fn target_dir() -> Option<PathBuf> {
    Some(tool_text("target_dir")).filter(|s| !s.trim().is_empty()).map(|s| PathBuf::from(s.trim()))
}

/// Um número gravado (`cleanup_project_days`, `cleanup_shared_days`), se for válido.
pub fn number(key: &str) -> Option<u64> {
    match read().get(key)? {
        Value::Number(n) => n.as_u64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}
