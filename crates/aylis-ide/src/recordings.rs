//! Gravações de uso: a IDE grava o próprio uso (`EventRecorder` + `Analytics` no form
//! dela, em `%LOCALAPPDATA%\aylis`) e abre gravações `.aylisrec` — da IDE ou de um app
//! feito com o aylis — para ver o que o usuário fez: o resumo e a linha do tempo, com
//! busca, filtro e os detalhes de cada acontecimento.

use std::path::{Path, PathBuf};

use aylis_controls::{Analytics, Button, ComboBox, Edit, EventRecorder, Label, Layout, Recording, ScrollBox, recorder};
use aylis_core::{Align, ComponentId, PropValue, Ui};
use serde_json::Value;

use crate::app::Ids;
use crate::dialogs::new_dialog;
use crate::state::Shared;
use crate::widgets::{caption, clear, muted, row};

/// Nomes dos componentes de gravação no form da IDE.
pub const RECORDER: &str = "ideRecorder";
pub const METRICS: &str = "ideMetrics";
/// Quantas linhas a lista mostra de uma vez (a busca refina o resto).
const MAX_ROWS: usize = 1500;

/// A pasta das gravações da IDE.
pub fn folder() -> PathBuf {
    let base = recorder::app_data_dir("aylis");
    let folder = base.join("recordings");
    // A pasta de antes do nome em inglês: as gravações dela continuam na lista.
    let old = base.join("gravacoes");
    if !folder.exists() && old.is_dir() {
        let _ = std::fs::rename(&old, &folder);
    }
    folder
}

/// Liga a gravação do uso no form da IDE (se não foi desligada no menu Ajuda).
pub fn install(ui: &mut Ui, form: ComponentId) {
    // "gravar_uso": o nome antigo da opção, de antes do config em inglês.
    let on = crate::actions::config_flag("record_usage", crate::actions::config_flag("gravar_uso", true));
    let rec = ui.add(EventRecorder::default(), form);
    let _ = ui.set_name(rec, RECORDER);
    let _ = ui.set_prop(rec, "folder", folder().display().to_string());
    let _ = ui.set_prop(rec, "app_name", "Aylis IDE");
    let _ = ui.set_prop(rec, "keep", 30_i64);
    let _ = ui.set_prop(rec, "active", on);
    let metrics = ui.add(Analytics::default(), form);
    let _ = ui.set_name(metrics, METRICS);
    let _ = ui.set_prop(metrics, "file", recorder::app_data_dir("aylis").join("analytics-ide.json").display().to_string());
    let _ = ui.set_prop(metrics, "app_name", "Aylis IDE");
    let _ = ui.set_prop(metrics, "active", on);
}

/// Está gravando o uso da IDE?
pub fn recording(ui: &Ui, ids: &Ids) -> bool {
    ui.find_in(ids.form, RECORDER).is_some_and(|r| matches!(ui.get_prop(r, "active"), Ok(PropValue::Bool(true))))
}

/// Liga/desliga a gravação do uso da IDE e guarda a escolha.
pub fn set_recording(ui: &mut Ui, state: &Shared, ids: &Ids, on: bool) {
    for name in [RECORDER, METRICS] {
        if let Some(id) = ui.find_in(ids.form, name) {
            let _ = ui.set_prop(id, "active", on);
        }
    }
    crate::actions::update_config("record_usage", Value::from(on));
    state.borrow_mut().status = if on {
        format!("Gravando o uso da IDE em {}", folder().display())
    } else {
        "Gravação do uso da IDE desligada".into()
    };
}

/// Os avisos da IDE ("⚠ ...") também vão para a gravação, como erros.
pub fn report_status(ui: &mut Ui, status: &str) {
    thread_local! {
        static LAST: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
    }
    let message = status.trim_start_matches('⚠').trim();
    if !status.starts_with('⚠') || LAST.with(|l| *l.borrow() == message) {
        return;
    }
    LAST.with(|l| *l.borrow_mut() = message.to_string());
    ui.report(&aylis_core::activity::Activity::Error { message });
}

/// Arquivo → Abrir gravação…
pub fn open_dialog(state: &Shared) {
    let state2 = state.clone();
    crate::actions::spawn(state, async move {
        let Some(file) = crate::files::AsyncFileDialog::new()
            .add_filter("Gravação do Aylis", &[aylis_controls::RECORDING_EXTENSION])
            .set_directory(folder())
            .pick_file()
            .await
        else {
            return;
        };
        let path = file.path().to_path_buf();
        state2.borrow_mut().after_task = Some(Box::new(move |ui: &mut Ui| {
            show(ui, &path);
        }));
    });
}

/// Abre o visualizador de uma gravação; devolve o form dele.
pub fn show(ui: &mut Ui, path: &Path) -> Option<ComponentId> {
    let recording = match Recording::open(path) {
        Ok(r) => r,
        Err(e) => {
            aylis_controls::message_dlg(ui, &e, aylis_controls::DlgKind::Error, &[aylis_core::ModalResult::Ok], |_, _| {});
            return None;
        }
    };
    let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let d = new_dialog(ui, &format!("Gravação · {file_name}"), 1180.0, 780.0);
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "cancel");

    let h = &recording.header;
    let summary = format!(
        "{} · {} · {} · {} acontecimentos · {} erro(s) · {} {}",
        h["app"].as_str().unwrap_or("?"),
        h["start_ms"].as_u64().map(local_time).unwrap_or_default(),
        duration_text(recording.duration()),
        recording.entries.len(),
        recording.errors(),
        h["os"].as_str().unwrap_or(""),
        h["arch"].as_str().unwrap_or(""),
    );
    let head = ui.add(Label::new(&summary), d.body);
    ui.control_mut(head).unwrap().set_align(Align::Top).set_height(26.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let _ = ui.set_prop(head, "bold", true);

    let bar = ui.add(Layout::default(), d.body);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    let filter = ui.add(ComboBox::new(&FILTERS.map(|(name, _)| name)), bar);
    ui.control_mut(filter).unwrap().set_align(Align::Right).set_width(170.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(filter, "item_index", 0_i64);
    let search = ui.add(Edit::new(""), bar);
    ui.control_mut(search).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(search, "placeholder", "Buscar (componente, tecla, texto, evento...)");

    // Direita: o resumo e os detalhes do acontecimento escolhido.
    let side = ui.add(Layout::default(), d.body);
    ui.control_mut(side).unwrap().set_align(Align::Right).set_width(400.0).set_margins([12.0, 0.0, 0.0, 0.0]);
    let side_title = caption(ui, side, "RESUMO");
    let details = ui.add(ScrollBox::default(), side);
    ui.control_mut(details).unwrap().set_align(Align::Client);
    let details_text = ui.add(Label::new(&overview(&recording)), details);
    ui.control_mut(details_text).unwrap().set_align(Align::Top).set_height(1600.0);
    let _ = ui.set_prop(details_text, "wrap", true);
    let _ = ui.set_prop(details_text, "v_align", "leading");

    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    let recording = std::rc::Rc::new(recording);
    let fill = {
        let recording = recording.clone();
        std::rc::Rc::new(move |ui: &mut Ui| {
            let text = ui.get::<Edit>(search).map(|e| e.text.to_lowercase()).unwrap_or_default();
            let kind = ui.get::<ComboBox>(filter).map_or(0, |c| c.item_index.max(0) as usize);
            fill_list(ui, list, &recording, &text, kind, side_title, details_text);
        })
    };
    fill(ui);
    for (control, event) in [(search, "on_change"), (filter, "on_change")] {
        let fill = fill.clone();
        ui.on(control, event, move |ui, _, _| fill(ui));
    }
    crate::lang::translate(ui);
    let form = d.form;
    crate::dialogs::close_on_escape(ui, form);
    let _ = ui.show_modal(form, move |ui, _| {
        ui.free(form);
    });
    Some(form)
}

/// Filtros da lista: nome e os tipos que entram (vazio: todos).
const FILTERS: [(&str, &[&str]); 6] = [
    ("Tudo", &[]),
    ("Cliques e roda", &["pointer_down", "pointer_up", "drag", "wheel"]),
    ("Teclado", &["key", "text"]),
    ("Eventos", &["event", "metric"]),
    ("Forms", &["form_shown", "form_closed"]),
    ("Erros", &["error"]),
];

fn fill_list(ui: &mut Ui, list: ComponentId, rec: &std::rc::Rc<Recording>, text: &str, kind: usize, side_title: ComponentId, details: ComponentId) {
    clear(ui, list);
    let types = FILTERS.get(kind).map_or(&[][..], |(_, t)| *t);
    let matching: Vec<usize> = rec
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| types.is_empty() || types.contains(&e["type"].as_str().unwrap_or_default()))
        .filter(|(_, e)| text.is_empty() || e.to_string().to_lowercase().contains(text))
        .map(|(i, _)| i)
        .collect();
    if matching.len() > MAX_ROWS {
        let l = ui.add(Label::new(&format!("Mostrando {MAX_ROWS} de {}: refine a busca ou o filtro.", matching.len())), list);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(24.0);
        muted(ui, l);
    }
    let danger = ui.theme().color("danger");
    let warning = ui.theme().color("warning");
    for &i in matching.iter().take(MAX_ROWS) {
        let e = &rec.entries[i];
        let line = format!("{:>9}   {}", time_text(e["t"].as_f64().unwrap_or(0.0)), Recording::describe(e));
        let r = row(ui, list, &line, 0.0, false, 24.0);
        if let Some(&label) = ui.children(r).first() {
            if e["type"] == "error" {
                let _ = ui.set_prop(label, "color", danger);
            } else if e["blocked"] == true {
                let _ = ui.set_prop(label, "color", warning);
            }
        }
        let rec = rec.clone();
        ui.on_click(r, move |ui, _| {
            let _ = ui.set_prop(side_title, "text", format!("ACONTECIMENTO {} DE {}", i + 1, rec.entries.len()));
            let _ = ui.set_prop(details, "text", details_text(&rec, i));
        });
    }
}

/// O acontecimento em JSON e os cinco anteriores (o que levou até ele).
fn details_text(rec: &Recording, i: usize) -> String {
    let mut out = serde_json::to_string_pretty(&rec.entries[i]).unwrap_or_default();
    out.push_str("\n\nANTES:\n");
    for e in rec.entries[i.saturating_sub(5)..i].iter() {
        out.push_str(&format!("{}  {}\n", time_text(e["t"].as_f64().unwrap_or(0.0)), Recording::describe(e)));
    }
    out
}

/// O resumo: contagem por tipo, os componentes mais clicados, os atalhos e os erros.
fn overview(rec: &Recording) -> String {
    use std::collections::BTreeMap;
    let mut by_type: BTreeMap<&str, usize> = BTreeMap::new();
    let mut clicks: BTreeMap<String, usize> = BTreeMap::new();
    let mut keys: BTreeMap<String, usize> = BTreeMap::new();
    for e in &rec.entries {
        *by_type.entry(e["type"].as_str().unwrap_or("?")).or_default() += 1;
        if e["type"] == "pointer_down" {
            let target = e["target_text"].as_str().map(|t| format!("\"{t}\" ")).unwrap_or_default() + e["target"].as_str().unwrap_or("?");
            *clicks.entry(target).or_default() += 1;
        }
        if e["type"] == "key" && e["key"].as_str().is_some_and(|k| k.contains('+') || k.starts_with('F')) {
            *keys.entry(e["key"].as_str().unwrap_or_default().to_string()).or_default() += 1;
        }
    }
    let top = |m: BTreeMap<String, usize>| {
        let mut v: Vec<_> = m.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.into_iter().take(10).map(|(k, n)| format!("  {n:>4}  {k}")).collect::<Vec<_>>().join("\n")
    };
    let mut out = String::from("POR TIPO\n");
    for (t, n) in &by_type {
        out.push_str(&format!("  {n:>4}  {t}\n"));
    }
    out.push_str("\nMAIS CLICADOS\n");
    out.push_str(&top(clicks));
    out.push_str("\n\nATALHOS\n");
    out.push_str(&top(keys));
    let errors: Vec<String> = rec.entries.iter().filter(|e| e["type"] == "error").map(|e| format!("  {}  {}", time_text(e["t"].as_f64().unwrap_or(0.0)), e["message"].as_str().unwrap_or(""))).collect();
    if !errors.is_empty() {
        out.push_str("\n\nERROS\n");
        out.push_str(&errors.join("\n"));
    }
    out.push_str("\n\nClique numa linha para ver os detalhes.");
    out
}

/// `1:02.345` (minutos:segundos).
fn time_text(t: f64) -> String {
    let m = (t / 60.0).floor();
    format!("{}:{:06.3}", m as u64, t - m * 60.0)
}

fn duration_text(t: f64) -> String {
    let s = t.round() as u64;
    if s >= 3600 { format!("{} h {} min", s / 3600, s % 3600 / 60) } else if s >= 60 { format!("{} min {} s", s / 60, s % 60) } else { format!("{s} s") }
}

/// Data e hora (UTC) do início.
fn local_time(ms: u64) -> String {
    recorder::utc_text(ms)
}
