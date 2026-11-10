//! Ajuda → Relatar problema…: o que aconteceu, o que esperava, como reproduzir e quem relata
//! (nome e contato, lembrados para a próxima vez); caixas dos anexos (a captura da tela, a
//! gravação dos últimos minutos, os arquivos do form e do projeto — este desmarcado). A
//! máquina e o resumo do projeto vão sempre. "Ver o que será enviado" mostra tudo antes. O
//! relato vira um `.aylisreport` na pasta de relatos, que abre em seguida; o usuário manda o
//! arquivo ao autor como preferir. Dois cliques num `.aylisreport` abrem o visualizador.

use std::path::{Path, PathBuf};

use aylis_controls::{Button, CheckBox, DlgKind, Memo, message_dlg, recorder};
use aylis_core::{Align, ComponentId, Color, ModalResult, PropValue, Ui};
use aylis_tools::report::{self, Problem, Report, Reporter};
use serde_json::{Map, Value, json};

use crate::app::{Ids, refresh};
use crate::dialogs::{self, edit, field, text_of};
use crate::state::Shared;

fn check(ui: &mut Ui, parent: ComponentId, text: &str, on: bool) -> ComponentId {
    let c = ui.add(CheckBox::new(text), parent);
    ui.control_mut(c).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(c, "checked", on);
    c
}

fn checked(ui: &Ui, id: ComponentId) -> bool {
    matches!(ui.get_prop(id, "checked"), Ok(PropValue::Bool(true)))
}

/// Os anexos escolhidos.
#[derive(Clone, Copy)]
struct Choices {
    screenshot: bool,
    recording: bool,
    project: bool,
}

/// Os campos do diálogo.
#[derive(Clone, Copy)]
struct Fields {
    what: ComponentId,
    expected: ComponentId,
    steps: ComponentId,
    name: ComponentId,
    contact: ComponentId,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

/// O que o projeto aberto tem (sem os arquivos; esses são anexo à parte).
fn project_info(state: &Shared) -> Option<Map<String, Value>> {
    let st = state.borrow();
    let p = st.project.as_ref()?;
    let mut m = Map::new();
    m.insert("name".into(), json!(p.name));
    m.insert("language".into(), serde_json::to_value(p.language).unwrap_or_default());
    m.insert("main_form".into(), json!(p.main_form));
    m.insert("forms".into(), json!(p.forms.iter().map(|f| f.name.as_str()).collect::<Vec<_>>()));
    m.insert("open_form".into(), json!(st.doc.form_name()));
    if !p.theme.is_empty() {
        m.insert("theme".into(), json!(p.theme));
    }
    if !p.publish.version.is_empty() {
        m.insert("version".into(), json!(p.publish.version));
    }
    // Os pacotes de componente copiados no projeto (`packages/<nome>`).
    let packages: Vec<String> = p
        .path
        .parent()
        .and_then(|dir| std::fs::read_dir(dir.join("packages")).ok())
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    if !packages.is_empty() {
        m.insert("packages".into(), json!(packages));
    }
    Some(m)
}

/// Monta o relato com o que está no diálogo.
fn build(ui: &mut Ui, state: &Shared, ids: &Ids, f: Fields, c: Choices, crashed: Option<&Path>) -> Report {
    let mut machine = report::machine_info();
    machine.insert("ide".into(), json!(env!("CARGO_PKG_VERSION")));
    machine.insert("system_language".into(), json!(crate::lang::system_language()));
    if let Some(cb) = ui.control(ids.form) {
        machine.insert("ide_window".into(), json!(format!("{:.0}×{:.0}", cb.width(), cb.height())));
    }
    let mut files = Vec::new();
    if c.screenshot {
        let form = ids.form;
        let (w, h) = ui.control(form).map_or((1280.0, 800.0), |cb| (cb.width(), cb.height()));
        if let Some(png) = crate::host::render_png(w as u32, h as u32, Color::WHITE, |canvas| ui.paint(form, canvas)) {
            files.push(("tela.png".to_string(), png));
        }
    }
    if c.recording {
        // A gravação da sessão que quebrou ou, sem ela, a mais recente da IDE (os últimos
        // minutos de uso).
        let file = crashed.map(Path::to_path_buf).or_else(|| recordings_newest_first().into_iter().next());
        if let Some(file) = file {
            if let Ok(bytes) = std::fs::read(&file) {
                files.push((file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), bytes));
            }
        }
    }
    if c.project {
        let st = state.borrow();
        if let Ok(json) = st.doc.to_json() {
            files.push((format!("form-{}.json", st.doc.form_name()), json.into_bytes()));
        }
        if let Some(p) = &st.project {
            if let Ok(bytes) = std::fs::read(&p.path) {
                files.push(("projeto.aylisproj".into(), bytes));
            }
        }
    }
    Report {
        problem: Problem { what: text_of(ui, f.what), expected: text_of(ui, f.expected), steps: text_of(ui, f.steps) },
        reporter: Reporter { name: text_of(ui, f.name), contact: text_of(ui, f.contact) },
        created_ms: now_ms(),
        machine,
        project: project_info(state),
        files,
    }
}

/// A pasta dos relatos gerados.
fn folder() -> Result<PathBuf, String> {
    aylis_tools::dap::tools_dir().and_then(|d| d.parent().map(|p| p.join("reports"))).ok_or_else(|| "sem pasta de dados".into())
}

/// Um campo de várias linhas.
fn memo(ui: &mut Ui, body: ComponentId, text: &str, height: f32) -> ComponentId {
    let line = field(ui, body, text);
    ui.control_mut(line).unwrap().set_height(height);
    let m = ui.add(Memo::new(""), line);
    ui.control_mut(m).unwrap().set_align(Align::Client);
    m
}

/// As gravações da IDE, da mais nova para a mais velha.
fn recordings_newest_first() -> Vec<PathBuf> {
    recordings_in(&crate::recordings::folder())
}

fn recordings_in(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == aylis_controls::RECORDING_EXTENSION))
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    files.sort_by(|a, b| b.0.cmp(&a.0));
    files.into_iter().map(|(_, p)| p).collect()
}

/// A sessão anterior da IDE quebrou: a gravação dela e a mensagem do erro.
pub struct Crash {
    pub recording: PathBuf,
    pub message: String,
}

/// A última gravação de antes de `started` (o início desta sessão), se ela termina num erro
/// fatal: o gancho de pânico do gravador escreve essa linha quando a IDE quebra.
pub fn previous_crash(started: std::time::SystemTime) -> Option<Crash> {
    previous_crash_in(&crate::recordings::folder(), started)
}

/// Como [`previous_crash`], numa pasta de gravações.
pub fn previous_crash_in(dir: &Path, started: std::time::SystemTime) -> Option<Crash> {
    let file = recordings_in(dir).into_iter().find(|p| std::fs::metadata(p).and_then(|m| m.modified()).is_ok_and(|t| t < started))?;
    let text = std::fs::read_to_string(&file).ok()?;
    let last: serde_json::Value = serde_json::from_str(text.lines().rev().find(|l| !l.trim().is_empty())?).ok()?;
    if last["type"] != "error" || last["fatal"] != true {
        return None;
    }
    let message = format!("{} ({})", last["message"].as_str().unwrap_or("erro"), last["location"].as_str().unwrap_or("?"));
    Some(Crash { recording: file, message })
}

/// A IDE abriu depois de quebrar: oferece relatar (com a gravação da sessão que quebrou).
pub fn offer_after_crash(ui: &mut Ui, state: &Shared, ids: &Ids, crash: Crash) {
    let text = format!("{}\n\n{}", ui.tr("A IDE fechou por um erro na última vez. Relatar o problema ao autor?"), crash.message);
    let (title, ok) = (ui.tr("Relatar problema"), ui.tr("Relatar…"));
    let (state, ids) = (state.clone(), *ids);
    aylis_controls::confirm(ui, &title, &text, &ok, false, move |ui, yes| {
        if yes {
            open_with(ui, &state, &ids, Some(crash));
        }
    });
}

pub fn open(ui: &mut Ui, state: &Shared, ids: &Ids) {
    open_with(ui, state, ids, None);
}

fn open_with(ui: &mut Ui, state: &Shared, ids: &Ids, crash: Option<Crash>) {
    let d = dialogs::new_dialog(ui, "Relatar problema", 640.0, 640.0);
    let what = memo(ui, d.body, "O que aconteceu", 70.0);
    let crashed = crash.as_ref().map(|c| c.recording.clone());
    if let Some(c) = &crash {
        let _ = ui.set_prop(what, "text", format!("A IDE fechou sozinha: {}", c.message));
    }
    let expected = memo(ui, d.body, "O que esperava", 52.0);
    let steps = memo(ui, d.body, "Como reproduzir", 70.0);
    let line = field(ui, d.body, "Seu nome");
    let name = edit(ui, line, &crate::actions::config_text("report_name"));
    let line = field(ui, d.body, "Contato");
    let contact = edit(ui, line, &crate::actions::config_text("report_contact"));
    let _ = ui.set_prop(contact, "placeholder", "e-mail ou telefone (opcional)");
    let line = field(ui, d.body, "Vai junto");
    let note = ui.add(aylis_controls::Label::new("Os dados da máquina e o resumo do projeto aberto"), line);
    ui.control_mut(note).unwrap().set_align(Align::Client);
    let line = field(ui, d.body, "");
    let screenshot = check(ui, line, "Captura da tela da IDE", true);
    let line = field(ui, d.body, "");
    let recording = check(ui, line, "Gravação dos últimos minutos", true);
    let line = field(ui, d.body, "");
    let project = check(ui, line, "Os arquivos do form e do projeto abertos", false);
    let line = field(ui, d.body, "");
    let show = ui.add(Button::new("Ver o que será enviado"), line);
    ui.control_mut(show).unwrap().set_align(Align::Left).set_width(220.0);
    let fields = Fields { what, expected, steps, name, contact };
    let choices = move |ui: &Ui| Choices { screenshot: checked(ui, screenshot), recording: checked(ui, recording), project: checked(ui, project) };
    {
        let (state, ids, crashed) = (state.clone(), *ids, crashed.clone());
        ui.on_click(show, move |ui, _| {
            let c = choices(ui);
            let r = build(ui, &state, &ids, fields, c, crashed.as_deref());
            let text = report::summary(&r, &recorder::utc_text(r.created_ms));
            message_dlg(ui, &text, DlgKind::Information, &[ModalResult::Ok], |_, _| {});
        });
    }
    let (state, ids) = (state.clone(), *ids);
    dialogs::run(
        ui,
        d,
        move |ui| if text_of(ui, what).trim().is_empty() { Err("conte o que aconteceu".into()) } else { Ok(()) },
        move |ui| {
            crate::actions::update_config("report_name", json!(text_of(ui, name).trim()));
            crate::actions::update_config("report_contact", json!(text_of(ui, contact).trim()));
            let r = build(ui, &state, &ids, fields, choices(ui), crashed.as_deref());
            let saved = folder().and_then(|dir| {
                let path = dir.join(format!("relato-{}.{}", r.created_ms / 1000, report::EXTENSION));
                report::write(&path, &r).map(|_| path)
            });
            match saved {
                Ok(path) => {
                    if let Some(dir) = path.parent() {
                        let _ = crate::jobs::open_folder(dir);
                    }
                    state.borrow_mut().status = format!("Relato salvo em {} — mande este arquivo ao autor", path.display());
                }
                Err(e) => state.borrow_mut().status = format!("⚠ relato: {e}"),
            }
            refresh(ui, &state, &ids);
        },
    );
}

/// O visualizador de um `.aylisreport` (dois cliques no arquivo): o relato em texto, e os
/// anexos extraídos numa pasta ao lado; a gravação abre no visualizador de gravações.
pub fn show_file(ui: &mut Ui, path: &Path) {
    let r = match report::read(path) {
        Ok(r) => r,
        Err(e) => {
            message_dlg(ui, &e, DlgKind::Error, &[ModalResult::Ok], |_, _| {});
            return;
        }
    };
    let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let d = dialogs::new_dialog(ui, &format!("Relato · {file_name}"), 760.0, 640.0);
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "cancel");
    let text = ui.add(Memo::new(report::summary(&r, &recorder::utc_text(r.created_ms))), d.body);
    ui.control_mut(text).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(text, "read_only", true);
    // Os anexos vão para "<relato>-anexos", ao lado do arquivo.
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let target = path.with_file_name(format!("{stem}-anexos"));
    if !r.files.is_empty() {
        let extract = ui.add(Button::new("Extrair os anexos"), d.footer);
        ui.control_mut(extract).unwrap().set_align(Align::Left).set_width(170.0);
        let (files, dir) = (r.clone(), target.clone());
        ui.on_click(extract, move |ui, _| match report::extract(&files, &dir) {
            Ok(_) => {
                let _ = crate::jobs::open_folder(&dir);
            }
            Err(e) => {
                message_dlg(ui, &e, DlgKind::Error, &[ModalResult::Ok], |_, _| {});
            }
        });
    }
    if let Some((name, bytes)) = r.files.iter().find(|(n, _)| n.ends_with(&format!(".{}", aylis_controls::RECORDING_EXTENSION))).cloned() {
        let watch = ui.add(Button::new("Ver a gravação"), d.footer);
        ui.control_mut(watch).unwrap().set_align(Align::Left).set_width(150.0).set_margins([8.0, 0.0, 0.0, 0.0]);
        let dir = target;
        ui.on_click(watch, move |ui, _| {
            let file = dir.join(&name);
            match std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&file, &bytes)) {
                Ok(()) => {
                    crate::recordings::show(ui, &file);
                }
                Err(e) => {
                    message_dlg(ui, &e.to_string(), DlgKind::Error, &[ModalResult::Ok], |_, _| {});
                }
            }
        });
    }
    dialogs::run(ui, d, |_| Ok(()), |_| {});
}
