//! Pelo protocolo: o `prompt` (a pergunta com campo), o ícone escolhido no `message_dlg` e o
//! `show_modal` com `container` (o modal de uma parte da tela).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::Ui;
use aylis_remote::*;
use serde_json::{Value, json};

fn ui() -> Ui {
    let mut ui = Ui::new();
    register_all(&mut ui);
    ui
}

fn run(ui: &mut Ui, command: Value) -> Value {
    execute(ui, &command).unwrap_or_else(|e| panic!("{command}: {e}"))
}

fn capture(ui: &mut Ui) -> Rc<RefCell<Vec<Value>>> {
    let events = Rc::new(RefCell::new(Vec::new()));
    let e = events.clone();
    forward_events(ui, move |_, event| e.borrow_mut().push(event));
    events
}

fn main_form(ui: &mut Ui) {
    run(ui, json!({ "cmd": "load", "layout": { "class": "Form", "name": "frmPrincipal", "children": [
        { "class": "Panel", "name": "pnlPagina", "align": "left", "width": 300 },
        { "class": "Edit", "name": "edtFora", "align": "client" } ] } }));
    run(ui, json!({ "cmd": "show", "target": "frmPrincipal" }));
}

#[test]
fn prompt_by_protocol_sends_the_text_or_null() {
    let mut ui = ui();
    let events = capture(&mut ui);
    main_form(&mut ui);
    let dlg = run(&mut ui, json!({ "cmd": "prompt", "title": "Novo perfil", "label": "Nome:", "value": "Padrão", "required": true, "icon": "edit", "icon_color": "success", "handler": "nome_digitado" }));
    let path = dlg.as_str().unwrap().to_string();
    let edit = ui.find_path(&path).and_then(|d| {
        let mut stack = ui.children(d).to_vec();
        while let Some(c) = stack.pop() {
            if ui.class_name(c) == Some("Edit") {
                return Some(c);
            }
            stack.extend(ui.children(c).iter().copied());
        }
        None
    });
    ui.set_prop(edit.expect("o campo"), "text", "Noite").unwrap();
    run(&mut ui, json!({ "cmd": "modal_result", "target": path, "result": "ok" }));
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!((e["handler"].as_str(), e["args"].as_str()), (Some("nome_digitado"), Some("Noite")));
    let dlg = run(&mut ui, json!({ "cmd": "prompt", "title": "Novo perfil", "label": "Nome:", "handler": "nome_digitado" }));
    run(&mut ui, json!({ "cmd": "modal_result", "target": dlg, "result": "cancel" }));
    let e = events.borrow_mut().pop().unwrap();
    assert!(e["args"].is_null(), "o Cancelar manda null: {e}");
}

#[test]
fn message_dlg_with_icon_and_show_modal_in_a_container() {
    let mut ui = ui();
    let events = capture(&mut ui);
    main_form(&mut ui);
    let dlg = run(&mut ui, json!({ "cmd": "message_dlg", "text": "Excluir?", "kind": "warning", "buttons": ["yes", "no"], "icon": "delete", "icon_color": "#c62828", "handler": "respondeu" }));
    run(&mut ui, json!({ "cmd": "modal_result", "target": dlg, "result": "yes" }));
    assert_eq!(events.borrow_mut().pop().unwrap()["args"], "yes");
    // O modal só na página: o campo de fora continua respondendo.
    run(&mut ui, json!({ "cmd": "create", "class": "Form", "name": "frmPergunta" }));
    run(&mut ui, json!({ "cmd": "show_modal", "target": "frmPergunta", "container": "pnlPagina", "handler": "pergunta_fechou" }));
    let main = ui.find("frmPrincipal").unwrap();
    let outside = ui.find_path("frmPrincipal.edtFora").unwrap();
    assert!(!ui.input_blocked(main));
    assert!(ui.set_focus(outside));
    let question = ui.find("frmPergunta").unwrap();
    assert_eq!(ui.root_of(question), main);
    run(&mut ui, json!({ "cmd": "modal_result", "target": "frmPergunta", "result": "cancel" }));
    let e = events.borrow_mut().pop().unwrap();
    assert_eq!((e["handler"].as_str(), e["args"].as_str()), (Some("pergunta_fechou"), Some("cancel")));
}
