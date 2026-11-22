//! Projeto → Opções do projeto → Linux: o formato do pacote do Publicar (`.deb`, `.rpm`,
//! AppImage ou Flatpak) e o que vai nele: o nome do pacote, as categorias do menu, a descrição,
//! o mantenedor, as dependências a mais, os tipos de arquivo que o app abre, as permissões do
//! Flatpak e o atalho no menu ao publicar sem instalador (o `publish.linux`, decidido em 21/11).

use aylis_controls::CheckBox;
use aylis_core::{Align, PropValue, Ui};
use aylis_project::{LinuxInstaller, LinuxOptions};

use crate::app::{Ids, refresh};
use crate::dialogs::{self, combo, edit, field, text_of};
use crate::state::Shared;
use crate::widgets::{label, muted};

/// Os formatos (o valor gravado é o que vem antes do espaço).
const INSTALLERS: [&str; 4] = ["deb (Debian, Ubuntu, Mint)", "rpm (Fedora, RHEL, openSUSE)", "appimage (qualquer distribuição)", "flatpak (Flathub)"];

/// A lista separada por `;` (sem vazios).
fn list(text: &str) -> Vec<String> {
    text.split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect()
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let Some(project) = state.borrow().project.clone() else { return };
    let l = project.publish.linux.clone();
    let d = dialogs::new_dialog(ui, "Linux", 680.0, 700.0);
    let line = field(ui, d.body, "Pacote do instalador");
    let current = match l.kind() {
        LinuxInstaller::Deb => INSTALLERS[0],
        LinuxInstaller::Rpm => INSTALLERS[1],
        LinuxInstaller::AppImage => INSTALLERS[2],
        LinuxInstaller::Flatpak => INSTALLERS[3],
    };
    let installer = combo(ui, line, &INSTALLERS, current);
    let line = field(ui, d.body, "");
    let note = label(ui, line, "O .deb sai no Windows e no Linux; o .rpm, o AppImage e o Flatpak são montados no Linux (do Windows, pelo WSL).", Align::Client, 0.0);
    let _ = ui.set_prop(note, "wrap", true);
    muted(ui, note);
    ui.control_mut(line).unwrap().set_height(36.0);
    let line = field(ui, d.body, "Nome do pacote");
    let package = edit(ui, line, &l.package);
    let _ = ui.set_prop(package, "placeholder", project.linux_package_id());
    let line = field(ui, d.body, "Categorias do menu");
    let categories = edit(ui, line, &l.categories.join("; "));
    let _ = ui.set_prop(categories, "placeholder", "Utility   ex.: AudioVideo; Audio; Player");
    let line = field(ui, d.body, "Descrição");
    let description = edit(ui, line, &l.description);
    let _ = ui.set_prop(description, "placeholder", "Feito com o Aylis.");
    let line = field(ui, d.body, "Mantenedor");
    let maintainer = edit(ui, line, &l.maintainer);
    let _ = ui.set_prop(maintainer, "placeholder", "Nome <email@exemplo.com>");
    let line = field(ui, d.body, "Dependências a mais (.deb)");
    let depends = edit(ui, line, &l.depends.join("; "));
    let _ = ui.set_prop(depends, "placeholder", "ffmpeg; libssl3");
    let line = field(ui, d.body, "Dependências a mais (.rpm)");
    let rpm_depends = edit(ui, line, &l.rpm_depends.join("; "));
    let _ = ui.set_prop(rpm_depends, "placeholder", "ffmpeg-free; openssl-libs");
    let line = field(ui, d.body, "Tipos de arquivo que o app abre");
    let mime_types = edit(ui, line, &l.mime_types.join("; "));
    let _ = ui.set_prop(mime_types, "placeholder", "audio/mpeg; audio/flac");
    let line = field(ui, d.body, "Permissões a mais (Flatpak)");
    let permissions = edit(ui, line, &l.flatpak_permissions.join("; "));
    let _ = ui.set_prop(permissions, "placeholder", "--share=network; --filesystem=xdg-music");
    let line = field(ui, d.body, "");
    let shortcut = ui.add(CheckBox::new("Sem instalador: criar o atalho no menu deste usuário"), line);
    ui.control_mut(shortcut).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(shortcut, "checked", l.menu_shortcut);
    let _ = ui.set_prop(shortcut, "hint", "Grava o atalho em ~/.local/share/applications apontando para a pasta publicada. Quem recebe a pasta roda o app com --install-shortcut.");

    let read = move |ui: &Ui| -> LinuxOptions {
        let kind = dialogs::combo_text(ui, installer);
        LinuxOptions {
            installer: if kind == INSTALLERS[0] { String::new() } else { kind.split(' ').next().unwrap_or_default().to_string() },
            package: text_of(ui, package).trim().to_string(),
            categories: list(&text_of(ui, categories)),
            description: text_of(ui, description).trim().to_string(),
            maintainer: text_of(ui, maintainer).trim().to_string(),
            depends: list(&text_of(ui, depends)),
            rpm_depends: list(&text_of(ui, rpm_depends)),
            mime_types: list(&text_of(ui, mime_types)),
            flatpak_permissions: list(&text_of(ui, permissions)),
            menu_shortcut: matches!(ui.get_prop(shortcut, "checked"), Ok(PropValue::Bool(true))),
        }
    };
    let (state, ids) = (state.clone(), *ids);
    dialogs::run(
        ui,
        d,
        move |ui| {
            // O nome do pacote: minúsculas, números, `.`, `+` e `-` (o Debian e o rpm recusam o resto).
            let name = text_of(ui, package);
            let ok = name.trim().chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '+' | '-'));
            if ok { Ok(()) } else { Err("o nome do pacote vai em minúsculas, números e -".into()) }
        },
        move |ui| {
            let l = read(ui);
            let mut st = state.borrow_mut();
            let Some(mut p) = st.project.clone() else { return };
            p.publish.linux = l;
            st.status = match p.save() {
                Ok(()) => "Opções do Linux salvas".into(),
                Err(e) => format!("⚠ {e}"),
            };
            st.project = Some(p);
            drop(st);
            refresh(ui, &state, &ids);
        },
    );
}
