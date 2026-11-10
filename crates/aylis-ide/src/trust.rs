//! Projetos de fora em modo restrito: abrir um projeto que não foi criado nesta máquina (nem
//! confiado antes) pergunta antes de rodar qualquer coisa dele. No modo restrito dá para ver e
//! editar os forms, ler o código e gerar o código; o servidor de linguagem, compilar,
//! executar, depurar e publicar ficam desligados (todos rodam código do projeto: o
//! `build.rs`, as macros, as tarefas do MSBuild, os scripts do Gradle...).
//!
//! A confirmação é só do usuário: o MCP não clica nesse diálogo (veja `mcp.rs`).

use std::path::Path;

use aylis_core::Ui;

use crate::app::Ids;
use crate::state::Shared;

/// O nome do form da pergunta (o MCP recusa os `dialog_*` nele).
pub const DIALOG: &str = "frmConfiarProjeto";

/// O projeto em `dir` pode rodar? Pergunta se não estiver na lista; enquanto não confiar,
/// fica restrito.
pub fn check(ui: &mut Ui, state: &Shared, ids: &Ids, dir: &Path) {
    let trusted = aylis_tools::trust::is_trusted(dir);
    state.borrow_mut().restricted = !trusted;
    if trusted {
        return;
    }
    let mut text = format!(
        "{}\n\n{}",
        ui.tr("Este projeto não foi criado nesta máquina. Compilar, executar e o servidor de linguagem rodam código dele (o build.rs, as macros, os scripts de compilação)."),
        ui.tr("Confie só em projetos de quem você conhece. Sem confiar, dá para ver e editar os forms e ler o código (modo restrito).")
    );
    // Os comandos que o projeto roda ao compilar (cmd e PowerShell): confiar libera estes,
    // então eles aparecem aqui, um por linha (decidido pelo autor em 08/11).
    let commands = state.borrow().project.as_ref().map(|p| p.build_commands()).unwrap_or_default();
    if !commands.is_empty() {
        text.push_str(&format!("\n\n{}\n", ui.tr("Ao compilar, este projeto roda estes comandos:")));
        for c in commands {
            text.push_str(&format!("\n• {c}"));
        }
    }
    let (state, ids) = (state.clone(), *ids);
    let dir = dir.to_path_buf();
    let title = ui.tr("Confiar neste projeto?");
    let ok = ui.tr("Confiar");
    let id = aylis_controls::confirm(ui, &title, &text, &ok, false, move |ui, yes| {
        if yes {
            trust_dir(&state, &dir);
        } else {
            state.borrow_mut().status = "Modo restrito: compilar, executar e o servidor de linguagem ficam desligados (Projeto → Opções do projeto → Confiar neste projeto)".into();
        }
        crate::app::refresh(ui, &state, &ids);
    });
    let form = ui.root_of(id);
    let _ = ui.set_name(form, DIALOG);
}

fn trust_dir(state: &Shared, dir: &Path) {
    let mut st = state.borrow_mut();
    match aylis_tools::trust::trust(dir) {
        Ok(()) => {
            st.restricted = false;
            st.status = "Projeto confiável: tudo liberado".into();
        }
        Err(e) => st.status = format!("⚠ {e}"),
    }
}

/// "Projeto → Opções do projeto → Confiar neste projeto".
pub fn trust_open_project(state: &Shared) {
    let Some(dir) = state.borrow().project.as_ref().map(|p| p.dir()) else { return };
    trust_dir(state, &dir);
}

/// Pode rodar código do projeto? No modo restrito, avisa na barra de status e diz que não.
pub fn allowed(state: &Shared) -> bool {
    if !state.borrow().restricted {
        return true;
    }
    state.borrow_mut().status = "⚠ Modo restrito: confie no projeto antes (Projeto → Opções do projeto → Confiar neste projeto)".into();
    false
}
