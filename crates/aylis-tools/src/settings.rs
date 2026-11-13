//! Onde estão os compiladores e as ferramentas que não ficam no PATH: um arquivo só
//! (`%USERPROFILE%\.aylis\tools.json`, `~/.aylis/tools.json`), gravado pelas Opções da IDE e
//! lido pelo `aylis-tool`, pela IDE, pelo instalador e pelos testes. No lugar das antigas
//! variáveis `AYLIS_GO`, `AYLIS_DOTNET`, `AYLIS_FPC`...
//!
//! Fora do `AppData` de propósito: o Claude Desktop (um pacote MSIX) dá aos processos que abre
//! uma cópia privada do `AppData`, e a IDE aberta por ele e o instalador veriam arquivos
//! diferentes. O de antes (`%LOCALAPPDATA%\aylis\tools.json`) é copiado uma vez ([`migrate`]).
//!
//! As chaves: `bun`, `node`, `dotnet`, `javac`, `java`, `go`, `vcvars` (o `vcvars64.bat` do
//! Visual C++), `gxx`, `fpc`, `fpc_units` (a pasta das units), `makensis`, `bundletool`,
//! `wix` e `vscode`. E a compilação dos projetos em Rust: `target_dir` (a pasta de
//! compilação de todos os projetos), `cleanup_project_days` e `cleanup_shared_days` (os dias
//! da limpeza, veja [`crate::cleanup`]). E as origens dos pacotes de ferramentas
//! ([`crate::toolpacks`]): `package_dirs` e `package_mirrors`, listas.

use std::path::{Path, PathBuf};

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

/// O arquivo das ferramentas: `%USERPROFILE%\.aylis\tools.json` (`~/.aylis` fora do Windows,
/// como os pacotes de ferramentas). Na primeira vez no processo, traz o do lugar antigo, se
/// só ele existir.
pub fn path() -> Option<PathBuf> {
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })?;
    let file = PathBuf::from(home).join(".aylis").join("tools.json");
    static MIGRATED: std::sync::Once = std::sync::Once::new();
    MIGRATED.call_once(|| {
        if let Some(old) = old_path() {
            migrate(&old, &file);
        }
    });
    Some(file)
}

/// Onde o arquivo ficava até a 0.2.0-beta.5 (no `AppData`, que o MSIX do Claude Desktop
/// desvia).
fn old_path() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join("tools.json"))
}

/// Copia `old` para `new` se `new` não existe e `old` existe (uma vez: depois o novo existe).
/// O antigo fica (uma versão anterior do Aylis ainda o lê). `true` se copiou.
pub fn migrate(old: &Path, new: &Path) -> bool {
    if new.exists() || !old.is_file() {
        return false;
    }
    if let Some(dir) = new.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::copy(old, new).is_ok()
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

/// As pastas de pacotes de ferramentas (uma lista de caminhos: uma pasta de rede, a do Google
/// Drive para desktop), procuradas depois do cache ([`crate::toolpacks::Sources`]).
pub const PACKAGE_DIRS: &str = "package_dirs";
/// Os espelhos dos pacotes de ferramentas: endereços HTTPS de um `pacotes.json` ou o link de
/// uma pasta compartilhada do Google Drive.
pub const PACKAGE_MIRRORS: &str = "package_mirrors";

/// O espelho de quem distribui o Aylis (a pasta do Google Drive com os pacotes de
/// ferramentas): a origem que vale enquanto ninguém escolheu outra nas Opções. Apagado nas
/// Opções, fica apagado (a lista vazia é gravada).
pub const DEFAULT_PACKAGE_MIRROR: &str = "https://drive.google.com/drive/folders/128gDbpqvVzI_nXRe9e2J2T17v7x90zfe";

/// Uma lista gravada (`package_dirs`, `package_mirrors`); um texto só vale como lista de um.
/// Os espelhos nunca gravados são o [`DEFAULT_PACKAGE_MIRROR`].
pub fn list(key: &str) -> Vec<String> {
    list_in(&read(), key)
}

fn list_in(map: &Map<String, Value>, key: &str) -> Vec<String> {
    match map.get(key) {
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect(),
        Some(Value::String(s)) if !s.trim().is_empty() => vec![s.trim().to_string()],
        None if key == PACKAGE_MIRRORS => vec![DEFAULT_PACKAGE_MIRROR.to_string()],
        _ => Vec::new(),
    }
}

/// Grava (ou apaga, se vazia) a lista de `key`, mantendo o resto. A dos espelhos vazia fica
/// gravada vazia: senão voltaria a ser o espelho padrão.
pub fn set_list(key: &str, values: &[String]) -> Result<(), String> {
    let path = path().ok_or("sem pasta de dados do usuário")?;
    let mut map = read();
    let values: Vec<Value> = values.iter().map(|v| v.trim()).filter(|v| !v.is_empty()).map(|v| Value::String(v.to_string())).collect();
    if values.is_empty() && key == PACKAGE_MIRRORS {
        map.insert(key.to_string(), Value::Array(Vec::new()));
    } else if values.is_empty() {
        map.remove(key);
    } else {
        map.insert(key.to_string(), Value::Array(values));
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// Um número gravado (`cleanup_project_days`, `cleanup_shared_days`), se for válido.
pub fn number(key: &str) -> Option<u64> {
    match read().get(key)? {
        Value::Number(n) => n.as_u64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sem nada gravado, o espelho é o Drive de quem distribui o Aylis; a lista gravada (até
    /// vazia, quando o usuário apagou) vale no lugar dele. As pastas não têm padrão.
    #[test]
    fn the_default_mirror_holds_until_the_list_is_saved() {
        let empty = Map::new();
        assert_eq!(list_in(&empty, PACKAGE_MIRRORS), [DEFAULT_PACKAGE_MIRROR]);
        assert!(list_in(&empty, PACKAGE_DIRS).is_empty());
        assert!(crate::toolpacks::fetch::drive_folder_id(DEFAULT_PACKAGE_MIRROR).is_some(), "o link é de uma pasta do Drive");
        let mut saved = Map::new();
        saved.insert(PACKAGE_MIRRORS.into(), Value::Array(Vec::new()));
        assert!(list_in(&saved, PACKAGE_MIRRORS).is_empty(), "apagado nas Opções, fica apagado");
        saved.insert(PACKAGE_MIRRORS.into(), Value::Array(vec![Value::String("https://outro/pacotes.json".into())]));
        assert_eq!(list_in(&saved, PACKAGE_MIRRORS), ["https://outro/pacotes.json"]);
    }

    /// O `tools.json` sai do `AppData` (que o MSIX do Claude Desktop desvia): o antigo é
    /// copiado uma vez, e o novo, depois de existir, não é sobrescrito.
    #[test]
    fn the_old_tools_json_is_copied_once() {
        let dir = std::env::temp_dir().join(format!("aylis_settings_migrate_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (old, new) = (dir.join("AppData/aylis/tools.json"), dir.join(".aylis/tools.json"));
        assert!(!migrate(&old, &new), "sem o antigo, nada");
        std::fs::create_dir_all(old.parent().unwrap()).unwrap();
        std::fs::write(&old, r#"{"go": "C:\go\bin\go.exe"}"#).unwrap();
        assert!(migrate(&old, &new));
        assert_eq!(std::fs::read_to_string(&new).unwrap(), std::fs::read_to_string(&old).unwrap());
        assert!(old.is_file(), "o antigo fica (uma versão anterior ainda o lê)");
        std::fs::write(&new, "{}").unwrap();
        assert!(!migrate(&old, &new), "o novo já existe: fica como está");
        assert_eq!(std::fs::read_to_string(&new).unwrap(), "{}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_settings_file_is_outside_appdata() {
        let path = path().expect("a pasta do usuário");
        assert!(path.ends_with(Path::new(".aylis").join("tools.json")), "{}", path.display());
        assert!(!path.display().to_string().contains("AppData"), "{}", path.display());
    }
}
