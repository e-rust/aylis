//! O que mudou no disco por fora da IDE (o relatório do Tabernáculo, M13: a IDE guardava o
//! que estava aberto e perdia, ao salvar, o que se gravou por fora).
//!
//! Cada form e cada fonte aberto guarda a marca do arquivo ([`FileStamp`]) de quando foi lido
//! ou salvo. A conferência ([`check`]) só olha os abertos, pela marca (sem ler o projeto
//! inteiro), quando a IDE volta a ter o foco e antes de cada chamada do MCP:
//!
//! - mudou e a IDE não tem alteração: recarrega do disco;
//! - mudou e a IDE também tem alteração: conflito. Com o usuário, a IDE pergunta; pelo MCP,
//!   a resposta leva o diff, e o agente resolve (`resolve_conflict`), como decidido pelo autor
//!   em 11/11.

use std::path::{Path, PathBuf};

use aylis_core::Ui;
use aylis_project::FileStamp;

use crate::state::Shared;

/// Um arquivo aberto que mudou dos dois lados.
pub struct Conflict {
    pub file: PathBuf,
    /// O texto na IDE (não salvo) e o do disco.
    pub ide: String,
    pub disk: String,
}

impl Conflict {
    /// As linhas que diferem, para o agente decidir (`-` só na IDE, `+` só no disco).
    pub fn diff(&self) -> String {
        line_diff(&self.ide, &self.disk)
    }
}

/// Confere os forms e os fontes abertos. Recarrega os que mudaram no disco sem alteração na
/// IDE (devolve quantos) e devolve os conflitos.
pub fn check(ui: &mut Ui, state: &Shared) -> (usize, Vec<Conflict>) {
    let mut reloaded = 0;
    let mut conflicts = Vec::new();
    // Os forms: a aba ativa e as outras.
    {
        let mut st = state.borrow_mut();
        let active = st.active;
        let count = st.tabs.len();
        for i in 0..count {
            let doc = if i == active { Some(&mut st.doc) } else { st.tabs.get_mut(i).and_then(Option::as_mut) };
            let Some(doc) = doc else { continue };
            let Some(path) = doc.path.clone() else { continue };
            let now = FileStamp::read(&path);
            if now.is_none() || now == doc.disk {
                continue;
            }
            let Ok(disk) = std::fs::read_to_string(&path) else { continue };
            let ide = doc.to_json().unwrap_or_default();
            if disk == ide {
                doc.disk = now;
                continue;
            }
            if !doc.modified {
                if doc.replace_json(&disk).is_ok() {
                    doc.modified = false;
                    doc.disk = now;
                    reloaded += 1;
                }
                continue;
            }
            conflicts.push(Conflict { file: path, ide, disk });
        }
    }
    // Os fontes.
    for (file, modified, ide) in crate::codewindow::disk_changes(ui) {
        let Ok(disk) = std::fs::read_to_string(&file) else { continue };
        if disk == ide {
            crate::codewindow::keep_editor(ui, &file);
            continue;
        }
        if !modified {
            crate::codewindow::reload_from_disk(ui, &file, &disk);
            reloaded += 1;
            continue;
        }
        conflicts.push(Conflict { file, ide, disk });
    }
    if reloaded > 0 {
        state.borrow_mut().status = format!("{reloaded} arquivo(s) aberto(s) mudaram fora da IDE e foram recarregados");
    }
    (reloaded, conflicts)
}

/// Resolve um conflito: fica a versão do disco (`keep_disk`) ou a da IDE (que continua não
/// salva; salvar grava por cima do disco).
pub fn resolve(ui: &mut Ui, state: &Shared, file: &Path, keep_disk: bool) -> Result<(), String> {
    let disk = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
    let stamp = FileStamp::read(file);
    {
        let mut st = state.borrow_mut();
        let active = st.active;
        let count = st.tabs.len();
        for i in 0..count {
            let doc = if i == active { Some(&mut st.doc) } else { st.tabs.get_mut(i).and_then(Option::as_mut) };
            let Some(doc) = doc else { continue };
            if doc.path.as_deref() != Some(file) {
                continue;
            }
            if keep_disk {
                doc.replace_json(&disk)?;
                doc.modified = false;
            }
            doc.disk = stamp;
            return Ok(());
        }
    }
    if crate::codewindow::is_open(ui, file) {
        if keep_disk {
            crate::codewindow::reload_from_disk(ui, file, &disk);
        } else {
            crate::codewindow::keep_editor(ui, file);
        }
        return Ok(());
    }
    Err(format!("{} não está aberto na IDE", file.display()))
}

/// Pergunta ao usuário, um conflito por vez: "recarregar do disco?". Não: fica a da IDE (e
/// não pergunta de novo até o disco mudar outra vez).
pub fn ask(ui: &mut Ui, state: &Shared, ids: &crate::app::Ids, conflicts: Vec<Conflict>) {
    for c in conflicts {
        // Marca já como resolvido para a IDE: o "Não" não precisa fazer nada.
        let _ = resolve(ui, state, &c.file, false);
        let name = c.file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let question = format!("{name} mudou fora da IDE, e há alterações não salvas aqui. Recarregar a versão do disco (as alterações daqui se perdem)?");
        let (state2, file) = (state.clone(), c.file.clone());
        crate::actions::confirm(ui, state, ids, &question, move |ui| {
            let _ = resolve(ui, &state2, &file, true);
        });
    }
}

/// As linhas diferentes entre `a` e `b`: o começo e o fim iguais ficam de fora, o meio sai
/// como `-` (de `a`) e `+` (de `b`), com a linha onde começa.
pub fn line_diff(a: &str, b: &str) -> String {
    let (x, y): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    let start = x.iter().zip(&y).take_while(|(p, q)| p == q).count();
    let end = x[start..].iter().rev().zip(y[start..].iter().rev()).take_while(|(p, q)| p == q).count();
    let mut out = format!("@@ linha {} @@\n", start + 1);
    for line in &x[start..x.len() - end] {
        out.push_str(&format!("-{line}\n"));
    }
    for line in &y[start..y.len() - end] {
        out.push_str(&format!("+{line}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_diff_shows_only_what_changed() {
        let d = super::line_diff("a\nb\nc\nd", "a\nB\nc\nd");
        assert_eq!(d, "@@ linha 2 @@\n-b\n+B\n");
        assert_eq!(super::line_diff("a", "a\nb"), "@@ linha 2 @@\n+b\n");
    }
}
