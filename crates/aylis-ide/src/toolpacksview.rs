//! Ferramentas → Pacotes de ferramentas (docs/pacotes.md): a lista dos pacotes do manifesto
//! para esta plataforma (o Rust, o JDK, o SDK do Android, o Node...), com a versão, o tamanho,
//! as dependências e a situação de cada um; marcar e instalar ou remover, com o andamento e o
//! "Cancelar". Instalar e remover rodam fora da thread da interface (a vaga do ambiente dos
//! `jobs`) e, no fim, a verificação de ambiente das plataformas afetadas é reconferida (o
//! registro dos pacotes entra nas marcas dela). Na IDE lite o item não aparece (`lite::needs`).

use std::path::PathBuf;

use aylis_controls::{Button, CheckBox, DlgKind, Form, Label, Layout};
use aylis_core::{Align, ComponentId, ModalResult, Ui};
use aylis_project::Language;
use aylis_tools::Target;
use aylis_tools::env;
use aylis_tools::toolpacks::{Installer, Manifest};

use crate::app::{Ids, refresh};
use crate::jobs::{self, Msg, Slot};
use crate::state::Shared;
use crate::widgets::{label, muted};

/// Os títulos dos trabalhos desta tela (a vaga do ambiente: um de cada vez).
pub const CHECKING: &str = "Conferindo os pacotes";
pub const INSTALLING: &str = "Instalando pacotes";
pub const REMOVING: &str = "Removendo pacotes";

/// Um pacote na lista.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub name: String,
    pub title: String,
    pub description: String,
    pub version: String,
    /// Os bytes dos arquivos a obter.
    pub size: u64,
    pub depends: Vec<String>,
    /// A versão instalada (pelo Aylis) e onde.
    pub installed: Option<(String, PathBuf)>,
    /// Onde a máquina já tem a ferramenta, fora dos pacotes (a verificação de ambiente aceita).
    pub system: Option<String>,
}

/// A situação de um pacote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowState {
    Installed,
    Outdated,
    OnSystem,
    NotInstalled,
}

impl Row {
    pub fn state(&self) -> RowState {
        match (&self.installed, &self.system) {
            (Some((v, _)), _) if *v == self.version => RowState::Installed,
            (Some(_), _) => RowState::Outdated,
            (None, Some(_)) => RowState::OnSystem,
            (None, None) => RowState::NotInstalled,
        }
    }
}

/// Os pacotes desta plataforma, com o que está instalado. Com `system`, também onde a máquina
/// já tem cada ferramenta ([`env::on_system`]: pode rodar processos, só fora da thread da
/// interface).
pub fn rows(system: bool) -> Vec<Row> {
    let Ok(installer) = Installer::new() else { return Vec::new() };
    installer
        .status()
        .into_iter()
        .filter(|s| s.package.available())
        .map(|s| {
            let p = s.package;
            Row {
                system: if system && s.installed.is_none() { env::on_system(&p.name) } else { None },
                size: p.size(),
                installed: s.installed.map(|i| (i.version, i.dir)),
                name: p.name,
                title: p.title,
                description: p.description,
                version: p.version,
                depends: p.depends,
            }
        })
        .collect()
}

/// As plataformas que um pacote muda (a verificação delas é reconferida).
pub fn affected(name: &str) -> Vec<Target> {
    match name {
        "rust" => vec![Target::Desktop, Target::Android, Target::Web],
        "jdk" => vec![Target::Desktop, Target::Android],
        "rust-android" | "android-sdk" | "android-ndk" | "cargo-ndk" | "bundletool" => vec![Target::Android],
        "rust-wasm" | "wasm-bindgen" => vec![Target::Web],
        _ => vec![Target::Desktop],
    }
}

/// A tela aberta.
#[derive(Clone)]
pub struct View {
    pub form: ComponentId,
    pub list: ComponentId,
    pub progress: ComponentId,
    pub install: ComponentId,
    pub remove: ComponentId,
    pub cancel: ComponentId,
    /// A caixa de cada pacote, pelo nome.
    pub checks: Vec<(String, ComponentId)>,
}

fn busy_here(state: &Shared) -> bool {
    state.borrow().env_job.as_ref().is_some_and(|j| [CHECKING, INSTALLING, REMOVING].contains(&j.title.as_str()))
}

/// Abre a tela: a lista na hora (o que está instalado) e, em segundo plano, o que a máquina
/// já tem no sistema.
pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    if state.borrow().toolpacks_view.is_some() {
        return;
    }
    let form = ui.add_root(Form::new("Pacotes de ferramentas", 900.0, 640.0));
    let _ = ui.set_name(form, "frmPacotesFerramentas");
    let _ = ui.set_prop(form, "border_style", "single");
    ui.control_mut(form).unwrap().set_padding(16.0);

    let top = ui.add(Layout::default(), form);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_height(48.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let sources = ui.add(Button::new("Origens…"), top);
    ui.control_mut(sources).unwrap().set_align(Align::Right).set_width(120.0).set_margins([8.0, 0.0, 0.0, 14.0]).set_style("flat");
    let place = aylis_tools::toolpacks::Paths::user().map(|p| p.tools.display().to_string()).unwrap_or_default();
    let intro = label(
        ui,
        top,
        &format!(
            "{} {place}. {}",
            ui.tr("Ferramentas que o Aylis instala sem administrador, em"),
            ui.tr("O que a máquina já tem fica como está: o pacote só entra onde falta.")
        ),
        Align::Client,
        0.0,
    );
    let _ = ui.set_prop(intro, "wrap", true);
    muted(ui, intro);

    let footer = ui.add(Layout::default(), form);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let close = ui.add(Button::new("Fechar"), footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let _ = ui.set_prop(close, "modal_result", "ok");
    let cancel = ui.add(Button::new("Cancelar"), footer);
    ui.control_mut(cancel).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let remove = ui.add(Button::new("Remover"), footer);
    ui.control_mut(remove).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]).set_style("flat");
    let install = ui.add(Button::new("Instalar"), footer);
    ui.control_mut(install).unwrap().set_align(Align::Right).set_width(110.0);
    let progress = label(ui, footer, "", Align::Client, 0.0);
    muted(ui, progress);

    let list = crate::widgets::framed_list(ui, form);
    state.borrow_mut().toolpacks_view = Some(View { form, list, progress, install, remove, cancel, checks: Vec::new() });
    if state.borrow().toolpacks.is_empty() {
        state.borrow_mut().toolpacks = rows(false);
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(sources, move |ui, _| crate::options::open_at(ui, &state, &ids, crate::options::PACKAGES_TAB));
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(install, move |ui, _| install_checked(ui, &state, &ids));
    }
    {
        let (state, ids) = (state.clone(), *ids);
        ui.on_click(remove, move |ui, _| remove_checked(ui, &state, &ids));
    }
    {
        let state = state.clone();
        ui.on_click(cancel, move |ui, _| {
            if busy_here(&state)
                && let Some(job) = state.borrow().env_job.as_ref()
            {
                job.control.cancel();
            }
            status(ui, &state);
        });
    }
    let (state2, ids2) = (state.clone(), *ids);
    fill(ui, state);
    let _ = ui.show_modal(form, move |ui, _| {
        state2.borrow_mut().toolpacks_view = None;
        ui.free(form);
        refresh(ui, &state2, &ids2);
    });
    check(state);
}

/// Confere em segundo plano o que a máquina já tem (se a vaga do ambiente estiver livre;
/// senão, quando ela ficar).
pub fn check(state: &Shared) {
    let started = jobs::start(state, Slot::Environment, CHECKING, |_, sender| {
        aylis_tools::setup::refresh_path();
        sender.send(Msg::ToolPacks(rows(true)));
        Ok(String::new())
    });
    state.borrow_mut().toolpacks_pending = !started;
}

/// Os nomes marcados.
fn checked(ui: &Ui, state: &Shared) -> Vec<String> {
    let Some(view) = state.borrow().toolpacks_view.clone() else { return Vec::new() };
    view.checks.iter().filter(|(_, c)| ui.get_prop(*c, "checked").ok().and_then(|v| v.to_bool().ok()).unwrap_or(false)).map(|(n, _)| n.clone()).collect()
}

/// Sem origem e sem os arquivos no cache: diz e oferece as Opções (a aba Pacotes). `true` se
/// pode seguir.
pub fn ensure_source(ui: &mut Ui, state: &Shared, ids: &Ids, names: &[String]) -> bool {
    if names.iter().all(|n| env::package_has_source(n)) {
        return true;
    }
    let text = format!("{}\n\n{}", ui.tr(env::NO_PACKAGE_SOURCE), ui.tr("Abrir as Opções agora?"));
    let (state, ids) = (state.clone(), *ids);
    aylis_controls::message_dlg(ui, &text, DlgKind::Warning, &[ModalResult::Yes, ModalResult::No], move |ui, r| {
        if r == ModalResult::Yes {
            crate::options::open_at(ui, &state, &ids, crate::options::PACKAGES_TAB);
        }
    });
    false
}

/// As plataformas a reconferir: as do projeto e as que os pacotes mudam.
fn targets_after(state: &Shared, names: &[String]) -> (Language, Vec<Target>) {
    let st = state.borrow();
    let mut targets = crate::tooldialogs::project_targets(&st);
    for t in names.iter().flat_map(|n| affected(n)) {
        if !targets.contains(&t) && Target::available(st.project.as_ref().map_or(Language::Rust, |p| p.language)).contains(&t) {
            targets.push(t);
        }
    }
    (st.project.as_ref().map_or(Language::Rust, |p| p.language), targets)
}

/// Depois de instalar ou remover: a lista nova e a verificação de ambiente das plataformas
/// afetadas (pelas marcas: o registro dos pacotes mudou, então elas rodam de novo).
fn after(language: Language, targets: &[Target], sender: &jobs::Sender) {
    aylis_tools::setup::refresh_path();
    sender.send(Msg::ToolPacks(rows(true)));
    let host = jobs::aylis_host();
    let request = env::Request::new(language, targets, host.as_deref());
    let shared = std::sync::Mutex::new(sender.duplicate());
    env::check_with(&request, &|t, checks| {
        if let Ok(s) = shared.lock() {
            s.send(Msg::Checks(vec![t], checks.to_vec()));
        }
    });
}

fn install_checked(ui: &mut Ui, state: &Shared, ids: &Ids) {
    let names = checked(ui, state);
    if names.is_empty() {
        state.borrow_mut().toolpacks_log = "Marque pelo menos um pacote.".into();
        status(ui, state);
        return;
    }
    if !ensure_source(ui, state, ids, &names) {
        return;
    }
    let (language, targets) = targets_after(state, &names);
    let started = jobs::start(state, Slot::Environment, INSTALLING, move |control, sender| {
        let mut failed = Vec::new();
        for name in &names {
            sender.send(Msg::Event(aylis_tools::Event::Step(format!("Instalando {name}"))));
            if let Err(e) = env::install_package(name, control, &mut |e| sender.send(Msg::Event(e))) {
                if control.is_cancelled() {
                    after(language, &targets, sender);
                    return Err(aylis_tools::process::CANCELLED.into());
                }
                failed.push(format!("{name}: {e}"));
            }
        }
        after(language, &targets, sender);
        if failed.is_empty() { Ok(format!("Instalado: {}", names.join(", "))) } else { Err(failed.join("\n")) }
    });
    if started {
        state.borrow_mut().toolpacks_log.clear();
    }
    status(ui, state);
}

fn remove_checked(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    let names = checked(ui, state);
    let installed: Vec<String> = {
        let st = state.borrow();
        names.into_iter().filter(|n| st.toolpacks.iter().any(|r| &r.name == n && r.installed.is_some())).collect()
    };
    if installed.is_empty() {
        state.borrow_mut().toolpacks_log = "Marque pelo menos um pacote instalado.".into();
        status(ui, state);
        return;
    }
    let title = ui.tr("Remover os pacotes?");
    let text = ui.tr("Sai do disco a pasta de {0} (o arquivo baixado fica no cache).").replace("{0}", &installed.join(", "));
    let ok = ui.tr("Remover");
    let state = state.clone();
    aylis_controls::confirm(ui, &title, &text, &ok, true, move |ui, yes| {
        if !yes {
            return;
        }
        let (language, targets) = targets_after(&state, &installed);
        // Quem depende sai antes (o bundletool antes do JDK).
        let order: Vec<String> = Manifest::embedded().with_dependencies(&installed.iter().map(String::as_str).collect::<Vec<_>>()).unwrap_or_default().into_iter().rev().filter(|n| installed.contains(n)).collect();
        let started = jobs::start(&state, Slot::Environment, REMOVING, move |_, sender| {
            let installer = Installer::new()?;
            let mut failed = Vec::new();
            for name in &order {
                let result = installer.remove(name, &mut |p| {
                    if let aylis_tools::toolpacks::Progress::Step(s) = p {
                        sender.send(Msg::Event(aylis_tools::Event::Line(s, aylis_tools::LineKind::Tool)));
                    }
                });
                if let Err(e) = result {
                    failed.push(e);
                }
            }
            after(language, &targets, sender);
            if failed.is_empty() { Ok(format!("Removido: {}", order.join(", "))) } else { Err(failed.join("\n")) }
        });
        if started {
            state.borrow_mut().toolpacks_log.clear();
        }
        status(ui, &state);
    });
}

/// O andamento e os botões (a cada linha que chega).
pub fn status(ui: &mut Ui, state: &Shared) {
    let Some(view) = state.borrow().toolpacks_view.clone() else { return };
    let busy = busy_here(state);
    let other = state.borrow().env_job.as_ref().filter(|_| !busy).map(|j| j.title.clone());
    let text = {
        let st = state.borrow();
        match (&st.env_job, other) {
            (Some(job), None) if job.title == CHECKING => ui.tr("Conferindo o que a máquina já tem…"),
            (Some(job), None) if st.env_log.is_empty() => format!("{}…", ui.tr(&job.title)),
            (Some(job), None) => format!("{}… {}", ui.tr(&job.title), st.env_log),
            (_, Some(title)) => format!("{} {}…", ui.tr("Aguardando:"), ui.tr(&title)),
            (None, _) => ui.tr(&st.toolpacks_log),
        }
    };
    let _ = ui.set_prop(view.progress, "text", text);
    let working = state.borrow().env_job.as_ref().is_some_and(|j| j.title != CHECKING);
    let _ = ui.set_prop(view.install, "enabled", !working);
    let _ = ui.set_prop(view.remove, "enabled", !working);
    let _ = ui.set_prop(view.cancel, "enabled", busy && working);
    ui.request_repaint();
}

/// Refaz a lista (as marcas ficam).
pub fn fill(ui: &mut Ui, state: &Shared) {
    let Some(view) = state.borrow().toolpacks_view.clone() else { return };
    let marked = checked(ui, state);
    let rows = state.borrow().toolpacks.clone();
    let children = ui.children(view.list).to_vec();
    ui.free_many(&children);
    let (ok, warn, muted_color) = (ui.theme().color("success"), ui.theme().color("warning"), ui.theme().color("text_muted"));
    let mut checks = Vec::new();
    for row in &rows {
        let line = ui.add(Layout::default(), view.list);
        ui.control_mut(line).unwrap().set_align(Align::Top).set_height(28.0).set_margins([0.0, 4.0, 0.0, 0.0]);
        let check = ui.add(CheckBox::new(row.title.as_str()), line);
        ui.control_mut(check).unwrap().set_align(Align::Left).set_width(280.0);
        let _ = ui.set_prop(check, "checked", marked.contains(&row.name));
        // O MCP (`dialog_check`) acha cada caixa pelo nome do pacote.
        let _ = ui.set_prop(check, "hint", row.name.as_str());
        checks.push((row.name.clone(), check));
        let version = label(ui, line, &row.version, Align::Left, 130.0);
        muted(ui, version);
        let size = label(ui, line, &format!("{} MB", row.size.div_ceil(1024 * 1024)), Align::Left, 80.0);
        muted(ui, size);
        let (text, color) = match row.state() {
            RowState::Installed => (ui.tr("instalado"), ok),
            RowState::Outdated => (ui.tr("desatualizado (instalada a {0})").replace("{0}", row.installed.as_ref().map_or("", |(v, _)| v.as_str())), warn),
            RowState::OnSystem => (format!("{} ({})", ui.tr("já existe no sistema"), row.system.as_deref().unwrap_or_default()), ok),
            RowState::NotInstalled => (ui.tr("não instalado"), muted_color),
        };
        let s = ui.add(Label::new(text.as_str()), line);
        ui.control_mut(s).unwrap().set_align(Align::Client);
        let _ = ui.set_prop(s, "color", color);
        let mut about = row.description.clone();
        if !row.depends.is_empty() {
            if !about.is_empty() {
                about.push_str(" · ");
            }
            about.push_str(&format!("{} {}", ui.tr("precisa de"), row.depends.join(", ")));
        }
        if !about.is_empty() {
            let d = label(ui, view.list, &about, Align::Top, 20.0);
            ui.control_mut(d).unwrap().set_margins([26.0, 0.0, 0.0, 2.0]);
            let _ = ui.set_prop(d, "font_size", 12);
            muted(ui, d);
        }
    }
    if let Some(v) = state.borrow_mut().toolpacks_view.as_mut() {
        v.checks = checks;
    }
    crate::lang::translate(ui);
    ui.layout();
    status(ui, state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_state_of_a_row() {
        let mut row = Row { name: "jdk".into(), title: "JDK".into(), description: String::new(), version: "17".into(), size: 0, depends: Vec::new(), installed: None, system: None };
        assert_eq!(row.state(), RowState::NotInstalled);
        row.system = Some("C:/jdk".into());
        assert_eq!(row.state(), RowState::OnSystem);
        row.installed = Some(("16".into(), PathBuf::new()));
        assert_eq!(row.state(), RowState::Outdated);
        row.installed = Some(("17".into(), PathBuf::new()));
        assert_eq!(row.state(), RowState::Installed);
    }

    #[test]
    fn the_android_packages_recheck_android() {
        assert_eq!(affected("android-ndk"), [Target::Android]);
        assert!(affected("rust").contains(&Target::Web));
        assert_eq!(affected("node"), [Target::Desktop]);
    }
}
