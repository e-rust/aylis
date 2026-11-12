//! O `config.toml` do cargo desta máquina (`%USERPROFILE%\.cargo\config.toml`, ou o do
//! `CARGO_HOME`): por padrão, todo projeto Rust da máquina (do Aylis ou não, no terminal, no
//! editor) compila na pasta de compilação única e sem informação de depuração nas
//! dependências. Sem isso, cada projeto tem o próprio `target/`, e o cargo de fora do Aylis
//! compila as dependências com outro perfil, guardando duas cópias delas (medido no OlaNome:
//! 1,3 GB viraram 4 GB; veja `medicoes/debug_dependencias.json`).
//!
//! Cada chave gravada leva o comentário [`MARK`] logo acima: só essas o Aylis atualiza ou
//! tira (ao desmarcar nas Opções); o que o autor escreveu no arquivo fica como está, e uma
//! chave dele com o mesmo nome vence.

use std::path::{Path, PathBuf};

use toml_edit::{DocumentMut, Item, Table, value};

/// O comentário acima de cada chave gravada pelo Aylis.
pub const MARK: &str = "# Gravado pelo Aylis (Opções → Rust e Python; desmarque lá para tirar)";

/// A chave das Opções (`settings`): `off` desliga; vazia ou qualquer outra, ligada.
pub const SETTING: &str = "global_cargo_config";

/// Se o Aylis cuida do `config.toml` global (o padrão é sim).
pub fn enabled() -> bool {
    crate::settings::tool_text(SETTING).trim() != "off"
}

/// Liga ou desliga (grava nas Opções e já aplica).
pub fn set_enabled(on: bool) -> Result<(), String> {
    crate::settings::set_tool(SETTING, (!on).then_some("off"))?;
    apply()
}

/// O arquivo que o cargo lê: o `config` sem extensão, se existir (o cargo prefere ele), senão
/// o `config.toml`, no `CARGO_HOME` ou em `~/.cargo`.
pub fn global_path() -> Option<PathBuf> {
    let home = std::env::var_os("CARGO_HOME").filter(|h| !h.is_empty()).map(PathBuf::from).or_else(|| {
        let user = if cfg!(windows) { std::env::var_os("USERPROFILE") } else { std::env::var_os("HOME") };
        user.filter(|u| !u.is_empty()).map(|u| PathBuf::from(u).join(".cargo"))
    })?;
    let old = home.join("config");
    Some(if old.is_file() { old } else { home.join("config.toml") })
}

/// Aplica a escolha das Opções ao arquivo global, com a pasta de compilação de agora.
pub fn apply() -> Result<(), String> {
    if cfg!(target_arch = "wasm32") {
        return Ok(());
    }
    let path = global_path().ok_or("sem pasta do usuário para o config.toml do cargo")?;
    apply_to(&path, enabled(), &crate::shared_target_dir())
}

/// As chaves que o Aylis cuida: o caminho das tabelas, a chave e o valor.
fn entries(target: &Path) -> [(&'static [&'static str], &'static str, Item); 2] {
    [(&["build"], "target-dir", value(target.display().to_string())), (&["profile", "dev", "package", "*"], "debug", value(false))]
}

/// Grava (`on`) ou tira as chaves do Aylis de `path`. Um arquivo que não é TOML válido fica
/// como está (o erro diz por quê); o arquivo que fica vazio é apagado.
pub fn apply_to(path: &Path, on: bool, target: &Path) -> Result<(), String> {
    let current = std::fs::read_to_string(path).ok();
    let mut doc: DocumentMut = current.as_deref().unwrap_or_default().parse().map_err(|e| format!("{}: {e}", path.display()))?;
    for (tables, key, item) in entries(target) {
        let ours = table_at(doc.as_table(), tables).and_then(|t| t.key(key)).is_some_and(is_ours);
        let exists = table_at(doc.as_table(), tables).is_some_and(|t| t.contains_key(key));
        if on && (ours || !exists) {
            let table = table_mut(doc.as_table_mut(), tables);
            table.insert(key, item);
            if let Some(mut k) = table.key_mut(key) {
                k.leaf_decor_mut().set_prefix(format!("{MARK}\n"));
            }
        } else if !on && ours {
            if let Some(table) = table_at_mut(doc.as_table_mut(), tables) {
                table.remove(key);
            }
            prune(doc.as_table_mut(), tables);
        }
    }
    let text = doc.to_string();
    if current.as_deref() == Some(text.as_str()) {
        return Ok(());
    }
    if text.trim().is_empty() {
        return if current.is_some() { std::fs::remove_file(path).map_err(|e| format!("{}: {e}", path.display())) } else { Ok(()) };
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn is_ours(key: &toml_edit::Key) -> bool {
    key.leaf_decor().prefix().and_then(|p| p.as_str()).is_some_and(|p| p.contains(MARK))
}

fn table_at<'a>(mut table: &'a Table, path: &[&str]) -> Option<&'a Table> {
    for name in path {
        table = table.get(name)?.as_table()?;
    }
    Some(table)
}

fn table_at_mut<'a>(mut table: &'a mut Table, path: &[&str]) -> Option<&'a mut Table> {
    for name in path {
        table = table.get_mut(name)?.as_table_mut()?;
    }
    Some(table)
}

/// A tabela do caminho, criando as que faltam (as do meio implícitas: o arquivo ganha só o
/// `[profile.dev.package."*"]`, sem `[profile]` e `[profile.dev]` vazios).
fn table_mut<'a>(mut table: &'a mut Table, path: &[&str]) -> &'a mut Table {
    for (i, name) in path.iter().enumerate() {
        let item = table.entry(name).or_insert_with(|| {
            let mut t = Table::new();
            t.set_implicit(i + 1 < path.len());
            Item::Table(t)
        });
        if !item.is_table() {
            // Um valor com o nome da tabela (`build = 1`) não é config válida do cargo.
            *item = Item::Table(Table::new());
        }
        table = item.as_table_mut().unwrap();
    }
    table
}

/// Tira as tabelas do caminho que ficaram vazias, da mais funda para cima.
fn prune(root: &mut Table, path: &[&str]) {
    for depth in (1..=path.len()).rev() {
        let (parent, name) = (&path[..depth - 1], path[depth - 1]);
        let Some(parent) = table_at_mut(root, parent) else { return };
        if parent.get(name).and_then(Item::as_table).is_some_and(Table::is_empty) {
            parent.remove(name);
        } else {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("aylis_cargo_config_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn writes_the_shared_folder_and_removes_only_its_own_keys() {
        let d = dir("own");
        let file = d.join("config.toml");
        let target = Path::new(r"C:\Users\x\AppData\Local\aylis\target");
        apply_to(&file, true, target).unwrap();
        let text = std::fs::read_to_string(&file).unwrap();
        let doc: DocumentMut = text.parse().unwrap();
        assert_eq!(doc["build"]["target-dir"].as_str(), Some(target.display().to_string().as_str()), "{text}");
        assert_eq!(doc["profile"]["dev"]["package"]["*"]["debug"].as_bool(), Some(false), "{text}");
        assert!(!text.contains("[profile]\n") && !text.contains("[profile.dev]\n"), "{text}");
        // De novo: nada muda.
        apply_to(&file, true, target).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
        // Outra pasta: só o valor muda.
        apply_to(&file, true, Path::new("D:/target")).unwrap();
        assert!(std::fs::read_to_string(&file).unwrap().contains("D:/target"));
        // Desligado: o arquivo, que era só do Aylis, sai.
        apply_to(&file, false, target).unwrap();
        assert!(!file.exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn keeps_what_the_author_wrote() {
        let d = dir("author");
        let file = d.join("config.toml");
        std::fs::create_dir_all(&d).unwrap();
        let mine = "# meu\n[build]\ntarget-dir = 'E:/meu-target'\njobs = 4\n\n[net]\ngit-fetch-with-cli = true\n";
        std::fs::write(&file, mine).unwrap();
        apply_to(&file, true, Path::new("C:/aylis/target")).unwrap();
        let text = std::fs::read_to_string(&file).unwrap();
        assert!(text.contains("target-dir = 'E:/meu-target'") && !text.contains("C:/aylis/target"), "{text}");
        assert!(text.contains("debug = false") && text.contains("git-fetch-with-cli"), "{text}");
        apply_to(&file, false, Path::new("C:/aylis/target")).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), mine);
        // TOML inválido: fica como está.
        std::fs::write(&file, "[build\n").unwrap();
        assert!(apply_to(&file, true, Path::new("C:/t")).is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "[build\n");
        let _ = std::fs::remove_dir_all(&d);
    }
}
