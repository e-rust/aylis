//! Projeto → Pendências entre linguagens: o que falta portar para a linguagem ativa (a
//! lógica que mudou em outra, handlers que saíram do form, componentes citados que não
//! existem mais). Fica aberta ao lado: "Lado a lado" abre o código das duas linguagens e
//! "Portado" tira o item da lista.

use aylis_controls::{Button, Label, Layout, ScrollBox};
use aylis_core::{Align, ComponentId, Ui};
use aylis_project::sync::{self, Pending};
use aylis_project::{Language, Project};

use crate::dialogs::new_dialog;
use crate::state::Shared;
use crate::widgets::{clear, muted};

/// Abre a lista (só se houver pendências, com `only_if_any`). Devolve se abriu.
pub fn open(ui: &mut Ui, state: &Shared, only_if_any: bool) -> bool {
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto primeiro".into();
        return false;
    };
    let items = match sync::pending(&project, project.language) {
        Ok(items) => items,
        Err(e) => {
            state.borrow_mut().status = format!("⚠ {e}");
            return false;
        }
    };
    if items.is_empty() {
        state.borrow_mut().status = format!("Nada a portar para {}: a linguagem está em dia com as outras", project.language.name());
        if only_if_any {
            return false;
        }
    }
    let d = new_dialog(ui, &format!("Pendências entre linguagens · {}", project.language.name()), 820.0, 520.0);
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let form = d.form;
    ui.on_click(close, move |ui, _| {
        ui.free(form);
    });
    let others: Vec<&str> = project.code_languages().into_iter().filter(|l| *l != project.language).map(Language::name).collect();
    let intro = ui.add(Label::new(format!(
        "O visual é o mesmo para todas as linguagens; a lógica não é convertida. Aqui está o que mudou em {} e ainda falta em {}.",
        if others.is_empty() { "outra linguagem".into() } else { others.join(", ") },
        project.language.name()
    )), d.body);
    ui.control_mut(intro).unwrap().set_align(Align::Top).set_height(44.0);
    let _ = ui.set_prop(intro, "wrap", true);
    muted(ui, intro);
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);
    fill(ui, state, list);
    crate::lang::translate(ui);
    let _ = ui.show(form);
    true
}

/// As linhas da lista (refeita depois de "Portado").
fn fill(ui: &mut Ui, state: &Shared, list: ComponentId) {
    clear(ui, list);
    let Some(project) = state.borrow().project.clone() else { return };
    let items = sync::pending(&project, project.language).unwrap_or_default();
    if items.is_empty() {
        let l = ui.add(Label::new("Nada a portar: esta linguagem está em dia."), list);
        ui.control_mut(l).unwrap().set_align(Align::Top).set_height(32.0);
    }
    for item in items {
        let row = ui.add(Layout::default(), list);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_height(34.0).set_margins([0.0, 0.0, 0.0, 6.0]);
        let mut buttons: Vec<(&str, Box<dyn Fn(&mut Ui)>)> = Vec::new();
        match &item {
            Pending::Changed { form, handler, by, .. } => {
                let (p, f, h, from) = (project.clone(), form.clone(), handler.clone(), by.first().copied());
                buttons.push(("Lado a lado", Box::new(move |ui| side_by_side(ui, &p, &f, &h, from))));
                let (state2, f, h) = (state.clone(), form.clone(), handler.clone());
                buttons.push((
                    "Portado",
                    Box::new(move |ui| {
                        let Some(p) = state2.borrow().project.clone() else { return };
                        state2.borrow_mut().status = match sync::mark_ported(&p, p.language, &f, &h) {
                            Ok(()) => format!("{f}.{h} marcado como portado para {}", p.language.name()),
                            Err(e) => format!("⚠ {e}"),
                        };
                        fill(ui, &state2, list);
                        ui.layout();
                    }),
                ));
            }
            Pending::Unused { form, handler } => {
                let location = sync::handler_location(&project, project.language, form, handler);
                buttons.push(("Abrir", Box::new(move |ui| open_at(ui, location.clone()))));
            }
            Pending::MissingComponent { file, line, .. } => {
                let location = Some((file.clone(), *line));
                buttons.push(("Abrir", Box::new(move |ui| open_at(ui, location.clone()))));
            }
        }
        for (text, action) in buttons.into_iter().rev() {
            let b = ui.add(Button::new(text), row);
            ui.control_mut(b).unwrap().set_align(Align::Right).set_width(110.0).set_margins([6.0, 0.0, 0.0, 0.0]);
            ui.on_click(b, move |ui, _| action(ui));
        }
        let text = ui.add(Label::new(item.describe()), row);
        ui.control_mut(text).unwrap().set_align(Align::Client);
    }
}

/// O código do handler nas duas linguagens: a ativa e a de onde vem a mudança.
fn side_by_side(ui: &mut Ui, project: &Project, form: &str, handler: &str, from: Option<Language>) {
    if let Some(from) = from {
        open_at(ui, sync::handler_location(project, from, form, handler));
    }
    open_at(ui, sync::handler_location(project, project.language, form, handler));
}

fn open_at(ui: &mut Ui, location: Option<(std::path::PathBuf, usize)>) {
    if let Some((file, line)) = location {
        let _ = crate::codewindow::open(ui, &file, line);
    }
}
