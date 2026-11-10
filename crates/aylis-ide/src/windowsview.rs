//! Projeto → Opções do projeto → Windows: o tipo de instalador do Publicar (`.msi` do WiX,
//! `.exe` do NSIS ou do Inno Setup) e o que vai nele: os arquivos ao lado do executável, a
//! pasta de instalação, iniciar com o Windows, atalho na área de trabalho, fechar o app aberto,
//! abrir no fim e a pasta de dados que o desinstalar oferece apagar.

use aylis_controls::CheckBox;
use aylis_core::{Align, ComponentId, PropValue, Ui};
use aylis_project::WindowsOptions;

use crate::app::{Ids, refresh};
use crate::dialogs::{self, combo, edit, field, text_of};
use crate::state::Shared;
use crate::widgets::{label, muted};

/// Os tipos de instalador (o valor gravado é o que vem antes do espaço).
const INSTALLERS: [&str; 3] = ["msi (WiX)", "nsis (NSIS)", "inno (Inno Setup)"];
/// Como oferecer uma opção na instalação (`autostart`, `desktop_shortcut`): vazio, `on`, `off`.
const OFFERS: [&str; 3] = ["Não oferecer", "Oferecer marcado", "Oferecer desmarcado"];

fn check(ui: &mut Ui, parent: ComponentId, text: &str, on: bool) -> ComponentId {
    let c = ui.add(CheckBox::new(text), parent);
    ui.control_mut(c).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(c, "checked", on);
    c
}

fn checked(ui: &Ui, id: ComponentId) -> bool {
    matches!(ui.get_prop(id, "checked"), Ok(PropValue::Bool(true)))
}

/// A lista separada por `;` (sem vazios).
fn list(text: &str) -> Vec<String> {
    text.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

fn offer_label(value: &str) -> &'static str {
    match value.to_ascii_lowercase().as_str() {
        "on" | "true" | "yes" => OFFERS[1],
        "off" | "false" | "no" => OFFERS[2],
        _ => OFFERS[0],
    }
}

fn offer_value(label: &str) -> String {
    if label == OFFERS[1] {
        "on".into()
    } else if label == OFFERS[2] {
        "off".into()
    } else {
        String::new()
    }
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else { return };
    let w = project.publish.windows.clone();
    let d = dialogs::new_dialog(ui, "Windows", 680.0, 640.0);
    let line = field(ui, d.body, "Instalador");
    let current = match w.kind() {
        aylis_project::WindowsInstaller::Msi => INSTALLERS[0],
        aylis_project::WindowsInstaller::Nsis => INSTALLERS[1],
        aylis_project::WindowsInstaller::Inno => INSTALLERS[2],
    };
    let installer = combo(ui, line, &INSTALLERS, current);
    let line = field(ui, d.body, "");
    let note = label(ui, line, "O .msi instala em Arquivos de Programas; NSIS e Inno Setup instalam por usuário, com as opções abaixo.", Align::Client, 0.0);
    let _ = ui.set_prop(note, "wrap", true);
    muted(ui, note);
    ui.control_mut(line).unwrap().set_height(36.0);
    let line = field(ui, d.body, "Arquivos ao lado do executável");
    let files = edit(ui, line, &w.files.join("; "));
    let _ = ui.set_prop(files, "placeholder", "runtime/*.dll; licencas/LEIAME.txt");
    let line = field(ui, d.body, "Pasta de instalação");
    let install_dir = edit(ui, line, &w.install_dir);
    let _ = ui.set_prop(install_dir, "placeholder", format!("{{localappdata}}\\Programs\\{}", project.app_name()));
    let line = field(ui, d.body, "Subpasta do executável");
    let exe_dir = edit(ui, line, &w.exe_dir);
    let _ = ui.set_prop(exe_dir, "placeholder", "bin");
    let line = field(ui, d.body, "Iniciar com o Windows");
    let autostart = combo(ui, line, &OFFERS, offer_label(&w.autostart));
    let line = field(ui, d.body, "Argumentos ao iniciar com o Windows");
    let autostart_args = edit(ui, line, &w.autostart_args);
    let _ = ui.set_prop(autostart_args, "placeholder", "/autostart");
    let line = field(ui, d.body, "Atalho na área de trabalho");
    let desktop = combo(ui, line, &OFFERS, offer_label(&w.desktop_shortcut));
    let line = field(ui, d.body, "");
    let close_running = check(ui, line, "Fechar o app aberto antes de instalar e de desinstalar", w.close_running);
    let line = field(ui, d.body, "");
    let run_after = check(ui, line, "Abrir o app no fim da instalação", w.run_after_install);
    let line = field(ui, d.body, "Pasta de dados (desinstalar pergunta se apaga)");
    let data_dir = edit(ui, line, &w.data_dir);
    let _ = ui.set_prop(data_dir, "placeholder", format!("{{localappdata}}\\{}", project.app_name()));
    let line = field(ui, d.body, "");
    let marks = label(ui, line, "Nas pastas: {localappdata}, {appdata}, {programfiles} e {documents}.", Align::Client, 0.0);
    muted(ui, marks);

    let read = move |ui: &Ui| -> WindowsOptions {
        let kind = dialogs::combo_text(ui, installer);
        WindowsOptions {
            installer: if kind == INSTALLERS[0] { String::new() } else { kind.split(' ').next().unwrap_or_default().to_string() },
            files: list(&text_of(ui, files)),
            install_dir: text_of(ui, install_dir).trim().to_string(),
            exe_dir: text_of(ui, exe_dir).trim().to_string(),
            autostart: offer_value(&dialogs::combo_text(ui, autostart)),
            autostart_args: text_of(ui, autostart_args).trim().to_string(),
            desktop_shortcut: offer_value(&dialogs::combo_text(ui, desktop)),
            close_running: checked(ui, close_running),
            run_after_install: checked(ui, run_after),
            data_dir: text_of(ui, data_dir).trim().to_string(),
        }
    };
    let (state, ids) = (state.clone(), *ids);
    dialogs::run(
        ui,
        d,
        |_| Ok(()),
        move |ui| {
            let w = read(ui);
            let mut st = state.borrow_mut();
            let Some(mut p) = st.project.clone() else { return };
            p.publish.windows = w;
            st.status = match p.save() {
                Ok(()) => "Opções do Windows salvas".into(),
                Err(e) => format!("⚠ {e}"),
            };
            st.project = Some(p);
            drop(st);
            refresh(ui, &state, &ids);
        },
    );
}
