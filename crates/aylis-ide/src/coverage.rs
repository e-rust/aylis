//! Cobertura de handlers (Projeto → Cobertura de handlers…): os handlers ligados nos forms do
//! projeto que nenhum uso ou teste do app em execução disparou. O app avisa pelo canal de
//! desenvolvimento o primeiro disparo de cada handler (`{"handler": "frmX.h"}`); a IDE soma em
//! `.aylis/coverage.json` ([`aylis_project::coverage`]), entre as execuções, até Limpar.
//! Funciona igual nas 8 linguagens. O MCP tem o mesmo relatório (`coverage`).

use aylis_controls::{Button, Label, ScrollBox};
use aylis_core::{Align, Ui};
use aylis_project::coverage::{self, Handler, Report};

use crate::app::Ids;
use crate::dialogs::new_dialog;
use crate::state::Shared;
use crate::widgets::{caption, clear, muted, row};

/// Guarda os handlers que os apps avisaram (a cada quarto de segundo, pelo `jobs::tick`): só
/// escreve no disco quando há novidade, e o app avisa cada handler uma vez por execução.
pub fn collect(state: &Shared) {
    let handlers = state.borrow().dev.as_ref().map(|d| d.take_handlers()).unwrap_or_default();
    if handlers.is_empty() {
        return;
    }
    let Some(project) = state.borrow().project.clone() else { return };
    if let Err(e) = coverage::record(&project, &handlers) {
        state.borrow_mut().status = format!("⚠ {e}");
    }
}

/// A linha de resumo: quantos rodaram, de quantos, e desde quando.
pub fn summary(ui: &Ui, report: &Report) -> String {
    if report.total() == 0 {
        return ui.tr("Os forms do projeto não ligam nenhum handler a eventos.");
    }
    let since = if report.since_ms == 0 { ui.tr("nada rodou ainda") } else { ui.tr("desde {0} (UTC)").replace("{0}", &aylis_controls::recorder::utc_text(report.since_ms)) };
    ui.tr("{0} de {1} handlers rodaram ({2}%), {3}")
        .replace("{0}", &report.covered.len().to_string())
        .replace("{1}", &report.total().to_string())
        .replace("{2}", &format!("{:.0}", report.percent()))
        .replace("{3}", &since)
}

/// O texto de um handler na lista: o form, o nome e os eventos que o chamam.
fn line(h: &Handler) -> String {
    format!("{}.{}   ←  {}", h.form, h.handler, h.events.join(", "))
}

/// Abre o diálogo da cobertura do projeto aberto.
pub fn open(ui: &mut Ui, state: &Shared, _ids: &Ids) {
    collect(state);
    let Some(project) = state.borrow().project.clone() else {
        state.borrow_mut().status = "Abra um projeto para ver a cobertura".into();
        return;
    };
    let d = new_dialog(ui, "Cobertura de handlers", 820.0, 620.0);
    for child in ui.children(d.footer).to_vec() {
        if ui.class_name(child) == Some("Button") {
            ui.free(child);
        }
    }
    let close = ui.add(Button::new("Fechar"), d.footer);
    ui.control_mut(close).unwrap().set_align(Align::Right).set_width(110.0);
    let _ = ui.set_prop(close, "modal_result", "cancel");
    let reset = ui.add(Button::new("Limpar"), d.footer);
    ui.control_mut(reset).unwrap().set_align(Align::Right).set_width(110.0).set_margins([0.0, 0.0, 8.0, 0.0]).set_style("flat");

    let head = ui.add(Label::new(""), d.body);
    ui.control_mut(head).unwrap().set_align(Align::Top).set_height(26.0);
    let _ = ui.set_prop(head, "bold", true);
    let note = ui.add(
        Label::new("Os handlers que os forms ligam a eventos e que nenhum uso ou teste do app em execução pela IDE (F9, o app_command do MCP) disparou, somados entre as execuções até Limpar. Clique num handler para abrir o código."),
        d.body,
    );
    ui.control_mut(note).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 0.0, 0.0, 8.0]);
    let _ = ui.set_prop(note, "wrap", true);
    muted(ui, note);
    let list = ui.add(ScrollBox::default(), d.body);
    ui.control_mut(list).unwrap().set_align(Align::Client);

    let error = d.error;
    let fill = {
        let state = state.clone();
        std::rc::Rc::new(move |ui: &mut Ui| {
            let report = coverage::project_report(&project);
            let text = summary(ui, &report);
            let _ = ui.set_prop(head, "text", text);
            clear(ui, list);
            let sections = [("NÃO RODARAM", &report.missing), ("RODARAM", &report.covered)];
            for (title, handlers) in sections {
                if handlers.is_empty() {
                    continue;
                }
                let c = caption(ui, list, &format!("{} ({})", ui.tr(title), handlers.len()));
                ui.control_mut(c).unwrap().set_margins([0.0, 8.0, 0.0, 4.0]);
                for h in handlers.iter() {
                    let r = row(ui, list, &line(h), 0.0, false, 24.0);
                    let (state, form, handler) = (state.clone(), h.form.clone(), h.handler.clone());
                    ui.on_click(r, move |ui, _| {
                        if let Err(e) = open_handler(ui, &state, &form, &handler) {
                            let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
                        }
                    });
                }
            }
            crate::lang::translate(ui);
        })
    };
    fill(ui);
    {
        let (state, fill) = (state.clone(), fill.clone());
        ui.on_click(reset, move |ui, _| {
            let Some(project) = state.borrow().project.clone() else { return };
            if let Err(e) = coverage::clear(&project) {
                let _ = ui.set_prop(error, "text", format!("⚠ {e}"));
            }
            fill(ui);
        });
    }
    crate::lang::translate(ui);
    let form = d.form;
    crate::dialogs::close_on_escape(ui, form);
    let _ = ui.show_modal(form, move |ui, _| {
        ui.free(form);
    });
}

/// Abre o código de um handler (`frmClientes`, `btn_salvar_click`) no projeto aberto, na linha
/// dele (o Profiler e a cobertura).
pub fn open_handler(ui: &mut Ui, state: &Shared, form: &str, handler: &str) -> Result<(), String> {
    let (file, line) = handler_place(state, form, handler)?;
    crate::codewindow::open(ui, &file, line).map(|_| ())
}

/// O arquivo e a linha do handler no código da linguagem ativa do projeto.
pub fn handler_place(state: &Shared, form: &str, handler: &str) -> Result<(std::path::PathBuf, usize), String> {
    let project = state.borrow().project.clone().ok_or("nenhum projeto aberto")?;
    aylis_project::sync::handler_location(&project, project.language, form, handler).ok_or_else(|| format!("não achei {handler} no código de {form}"))
}
