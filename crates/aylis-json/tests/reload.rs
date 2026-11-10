//! Hot reload: aplicar um layout novo num form aberto.

use aylis_controls::{Button, Edit};
use aylis_core::*;

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

const V1: &str = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frm", "caption": "Clientes", "width": 400, "height": 300,
  "children": [
    { "class": "Edit", "name": "edtNome", "placeholder": "Nome", "align": "top", "height": 32 },
    { "class": "Button", "name": "btnSalvar", "text": "Salvar", "align": "bottom", "on_click": "salvar" },
    { "class": "Label", "name": "lblVelho", "text": "sai" }
  ] } }"#;

const V2: &str = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frm", "caption": "Clientes (novo)", "width": 400, "height": 300,
  "children": [
    { "class": "Label", "name": "lblTitulo", "text": "Cadastro", "align": "top" },
    { "class": "Edit", "name": "edtNome", "placeholder": "Nome completo", "align": "top", "height": 32 },
    { "class": "Button", "name": "btnSalvar", "text": "Gravar", "align": "bottom", "on_click": "salvar" }
  ] } }"#;

#[test]
fn reload_applies_only_what_changed_in_the_layout() {
    let mut ui = ui();
    let form = aylis_json::load(&mut ui, V1, None).unwrap();
    let edit = ui.find_in(form, "edtNome").unwrap();
    let button = ui.find_in(form, "btnSalvar").unwrap();
    // o usuário digitou no app e um handler foi ligado por código
    ui.get_mut::<Edit>(edit).unwrap().text = "Maria".into();
    let clicks = std::rc::Rc::new(std::cell::Cell::new(0));
    let c = clicks.clone();
    ui.on(button, "on_pointer_enter", move |_, _, _| c.set(c.get() + 1));
    // um controle criado pelo código do app (não está no layout)
    let extra = ui.add(Button::new("do código"), form);

    let report = aylis_json::reload(&mut ui, form, V2, Some(V1)).unwrap();
    assert_eq!((report.created, report.removed), (1, 1));
    assert_eq!(ui.get_prop(form, "caption").unwrap(), PropValue::String("Clientes (novo)".into()));
    assert_eq!(ui.find_in(form, "edtNome"), Some(edit), "o mesmo componente (não recriado)");
    assert_eq!(ui.get::<Edit>(edit).unwrap().text, "Maria", "o que foi digitado fica");
    assert_eq!(ui.get::<Edit>(edit).unwrap().placeholder, "Nome completo");
    assert_eq!(ui.get::<Button>(button).unwrap().text, "Gravar");
    assert!(ui.find_in(form, "lblVelho").is_none(), "saiu do layout");
    assert!(ui.contains(extra), "o criado pelo código fica");
    // a ordem segue o layout (o título novo vem antes do Edit)
    let title = ui.find_in(form, "lblTitulo").unwrap();
    let order = ui.children(form).to_vec();
    assert!(order.iter().position(|&c| c == title) < order.iter().position(|&c| c == edit));
    // o handler ligado por código continua
    ui.fire(button, "on_pointer_enter", &EventArgs::None);
    assert_eq!(clicks.get(), 1);
}

#[test]
fn reload_without_previous_applies_everything_and_rejects_another_class() {
    let mut ui = ui();
    let form = aylis_json::load(&mut ui, V1, None).unwrap();
    let edit = ui.find_in(form, "edtNome").unwrap();
    ui.get_mut::<Edit>(edit).unwrap().placeholder = "mudado no código".into();
    aylis_json::reload(&mut ui, form, V1, None).unwrap();
    assert_eq!(ui.get::<Edit>(edit).unwrap().placeholder, "Nome", "sem o anterior, o layout manda");
    let panel = r#"{ "aylis": 1, "root": { "class": "Panel" } }"#;
    assert!(aylis_json::reload(&mut ui, form, panel, None).is_err());
    assert!(aylis_json::reload(&mut ui, form, "{ quebrado", None).is_err());
}
