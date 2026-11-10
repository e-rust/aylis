//! Projetos confiáveis: abrir um projeto de fora não pode executar nada dele (servidor de
//! linguagem, compilar, executar). A lista fica em `%LOCALAPPDATA%\aylis\trusted.json`; uma
//! pasta confiável vale para tudo dentro dela (dá para confiar numa pasta-mãe inteira). Os
//! projetos criados na IDE entram sozinhos, e os exemplos que vêm com o aylis também.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn file() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.join("trusted.json"))
}

/// O caminho comparável: absoluto, sem o `\\?\` do Windows, barras normais e (no Windows)
/// em minúsculas.
fn key(dir: &Path) -> String {
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let text = dir.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").trim_end_matches('/').to_string();
    if cfg!(windows) { text.to_lowercase() } else { text }
}

fn read() -> Vec<String> {
    file()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()))
        .unwrap_or_default()
}

fn inside(dir: &str, parent: &str) -> bool {
    dir == parent || dir.strip_prefix(parent).is_some_and(|rest| rest.starts_with('/'))
}

/// A pasta (de um projeto) é confiável: está na lista (ou dentro de uma pasta da lista) ou é
/// um exemplo do próprio aylis.
pub fn is_trusted(dir: &Path) -> bool {
    let k = key(dir);
    let root = key(Path::new(&aylis_project::codegen::aylis_root()));
    inside(&k, &format!("{root}/examples")) || read().iter().any(|t| inside(&k, t))
}

/// Põe a pasta na lista.
pub fn trust(dir: &Path) -> Result<(), String> {
    let mut list = read();
    let k = key(dir);
    if list.iter().any(|t| inside(&k, t)) {
        return Ok(());
    }
    list.push(k);
    let path = file().ok_or("sem pasta de dados do usuário")?;
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// As pastas confiáveis (para a tela de opções).
pub fn list() -> Vec<String> {
    read()
}

/// Tira da lista as pastas que não existem mais (projetos apagados, pastas temporárias).
pub fn prune() {
    let list = read();
    let kept: Vec<String> = list.iter().filter(|t| Path::new(t.as_str()).is_dir()).cloned().collect();
    if kept.len() != list.len() {
        if let (Some(path), Ok(text)) = (file(), serde_json::to_string_pretty(&kept)) {
            let _ = std::fs::write(path, text + "\n");
        }
    }
}

/// Tira da lista a pasta `dir` (o caminho como no disco).
pub fn forget_dir(dir: &Path) {
    let _ = forget(&key(dir));
}

/// Tira uma pasta da lista.
pub fn forget(dir: &str) -> Result<(), String> {
    let list: Vec<String> = read().into_iter().filter(|t| t != dir).collect();
    let path = file().ok_or("sem pasta de dados do usuário")?;
    let text = serde_json::to_string_pretty(&list).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folders_contain_their_subfolders_only() {
        assert!(inside("c:/proj/loja", "c:/proj"));
        assert!(inside("c:/proj", "c:/proj"));
        assert!(!inside("c:/projetos", "c:/proj"));
        // Os exemplos do aylis são confiáveis sem estar na lista.
        let example = Path::new(&aylis_project::codegen::aylis_root()).join("examples").join("Paint");
        assert!(is_trusted(&example));
    }
}
