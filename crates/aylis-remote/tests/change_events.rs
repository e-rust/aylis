//! Pelo protocolo (conferência de 08/11): o `set` de um valor dispara o `on_change` (como
//! na VCL), o `load` e o `create` não; e a segunda janela do mesmo form ganha outro nome.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::{ComponentId, Ui};
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

fn changes(events: &Rc<RefCell<Vec<Value>>>) -> Vec<(String, Value)> {
    events
        .borrow()
        .iter()
        .filter(|e| e["event"] == "on_change")
        .map(|e| (e["sender"]["name"].as_str().unwrap_or_default().to_string(), e["args"].clone()))
        .collect()
}

#[test]
fn set_fires_on_change_but_load_and_create_do_not() {
    let mut ui = ui();
    let events = capture(&mut ui);
    run(&mut ui, json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "Frm", "children": [
            { "class": "Edit", "name": "edtNome", "text": "Ana", "on_change": "nome_change" },
            { "class": "CheckBox", "name": "chkAtivo", "checked": true, "on_change": "ativo_change" },
            { "class": "ComboBox", "name": "cbCor", "items": "Azul;Verde", "item_index": 0, "on_change": "cor_change" }
        ]
    }}));
    run(&mut ui, json!({ "cmd": "create", "class": "TrackBar", "parent": "Frm", "name": "tbVolume", "props": { "on_change": "volume_change", "value": 30 } }));
    assert!(changes(&events).is_empty(), "carregar e criar não disparam: {:?}", changes(&events));

    run(&mut ui, json!({ "cmd": "set", "target": "edtNome", "prop": "text", "value": "Bia" }));
    run(&mut ui, json!({ "cmd": "set", "target": "chkAtivo", "props": { "checked": false } }));
    run(&mut ui, json!({ "cmd": "set", "target": "cbCor", "prop": "item_index", "value": 1 }));
    run(&mut ui, json!({ "cmd": "set", "target": "tbVolume", "prop": "value", "value": 60 }));
    // O mesmo valor de novo: nada.
    run(&mut ui, json!({ "cmd": "set", "target": "edtNome", "prop": "text", "value": "Bia" }));
    assert_eq!(
        changes(&events),
        vec![
            ("edtNome".to_string(), json!("Bia")),
            ("chkAtivo".to_string(), json!(false)),
            ("cbCor".to_string(), json!("Verde")),
            ("tbVolume".to_string(), json!(60)),
        ]
    );
    let e = events.borrow();
    let first = e.iter().find(|e| e["event"] == "on_change").unwrap();
    assert_eq!(first["handler"], "nome_change");
}

/// A fábrica de forms do teste: o `frmCliente`, de um layout.
struct Forms;

impl aylis_core::platform::FormFactory for Forms {
    fn create_form(&mut self, ui: &mut Ui, name: &str) -> Result<ComponentId, String> {
        let layout = json!({ "aylis": 1, "root": {
            "class": "Form", "name": name, "children": [{ "class": "Button", "name": "btnSalvar", "on_click": "salvar" }]
        }});
        aylis_json::load_value(ui, &layout, None).map_err(|e| e.to_string())
    }

    fn form_names(&self) -> Vec<String> {
        vec!["frmCliente".into()]
    }
}

#[test]
fn two_windows_of_the_same_form_have_different_names() {
    let mut ui = ui();
    ui.services_mut().forms = Some(Box::new(Forms));
    let events = capture(&mut ui);
    let a = run(&mut ui, json!({ "cmd": "create_form", "name": "frmCliente", "new": true }));
    let b = run(&mut ui, json!({ "cmd": "create_form", "name": "frmCliente", "new": true }));
    assert_eq!((a["name"].as_str(), b["name"].as_str()), (Some("frmCliente"), Some("frmCliente2")));
    run(&mut ui, json!({ "cmd": "click", "target": "frmCliente2.btnSalvar" }));
    let e = events.borrow().last().cloned().unwrap();
    // O caminho e o `form_id` dizem qual janela; o `form`, de que form são os handlers.
    assert_eq!(e["sender"]["path"], "frmCliente2.btnSalvar");
    assert_eq!(e["form_id"], b["id"]);
    assert_eq!(e["form"], "frmCliente");
    assert_eq!(e["handler"], "salvar");
}
