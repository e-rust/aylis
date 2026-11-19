//! O editor de coleção das listas de registros: os itens à esquerda, os campos do escolhido à
//! direita; os filhos de um nó abrem o mesmo editor por cima.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, Edit, ListBox};
use aylis_core::{ComponentId, ModalResult, Ui};
use serde_json::{Value, json};

fn all(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.add_root(aylis_controls::Form::new("Principal", 400.0, 300.0));
    ui.show(main).unwrap();
    ui
}

#[test]
fn edit_the_fields_and_the_children_of_a_tree_node() {
    let mut ui = ui();
    let result: Rc<RefCell<Option<Value>>> = Rc::default();
    let out = result.clone();
    let tree = r#"[{"text": "Brasil", "icon": "flag", "children": [{"text": "SP"}]}, {"text": "Chile"}]"#;
    aylis_ide::records_editor::records_editor(&mut ui, "Itens", &aylis_controls::TREE_NODE, tree, false, move |_, v| *out.borrow_mut() = Some(v));
    let dialog = *ui.open_forms().last().unwrap();
    assert!(ui.is_modal(dialog));
    ui.layout();
    let ids = all(&ui, dialog);
    let list = ids.iter().copied().find(|&id| ui.get::<ListBox>(id).is_some()).unwrap();
    assert_eq!(ui.get::<ListBox>(list).unwrap().items, "Brasil;Chile", "os itens pelo texto");

    // O primeiro: o texto num Edit, o `expanded` numa caixa, os filhos num botão.
    let edits: Vec<ComponentId> = ids.iter().copied().filter(|&id| ui.get::<Edit>(id).is_some()).collect();
    let text = edits.iter().copied().find(|&e| ui.get::<Edit>(e).unwrap().text == "Brasil").expect("o Edit do texto");
    let _ = ui.set_prop(text, "text", "Brasil (país)");
    let expanded = ids.iter().copied().find(|&id| ui.get::<CheckBox>(id).is_some()).expect("a caixa do expanded");
    let _ = ui.set_prop(expanded, "checked", true);
    let children = ids.iter().copied().find(|&id| ui.get::<Button>(id).is_some_and(|b| b.text.contains("(1)"))).expect("o botão dos filhos, com 1");

    // Os filhos: acrescenta um.
    ui.click(children);
    let inner = *ui.open_forms().last().unwrap();
    assert_ne!(inner, dialog, "o editor dos filhos por cima");
    ui.layout();
    let add = all(&ui, inner).into_iter().find(|&id| ui.get::<Button>(id).is_some_and(|b| b.text == "+ Item")).unwrap();
    ui.click(add);
    ui.layout();
    let new_text = all(&ui, inner).into_iter().filter(|&id| ui.get::<Edit>(id).is_some()).next().expect("o Edit do item novo");
    let _ = ui.set_prop(new_text, "text", "RJ");
    ui.set_modal_result(inner, ModalResult::Ok);
    ui.set_modal_result(dialog, ModalResult::Ok);

    let value = result.borrow().clone().expect("o OK devolve a lista");
    assert_eq!(value[0]["text"], json!("Brasil (país)"));
    assert_eq!(value[0]["expanded"], json!(true));
    assert_eq!(value[0]["icon"], json!("flag"));
    assert_eq!(value[0]["children"], json!([{"text": "SP"}, {"text": "RJ"}]));
    assert_eq!(value[1]["text"], json!("Chile"));
}

#[test]
fn the_numbers_and_the_choices_go_back_in_their_type() {
    let mut ui = ui();
    let result: Rc<RefCell<Option<Value>>> = Rc::default();
    let out = result.clone();
    let series = r#"[{"name": "Vendas", "values": [1, 2.5], "kind": "line"}]"#;
    aylis_ide::records_editor::records_editor(&mut ui, "Séries", &aylis_controls::CHART_SERIES, series, false, move |_, v| *out.borrow_mut() = Some(v));
    let dialog = *ui.open_forms().last().unwrap();
    ui.layout();
    let values = all(&ui, dialog).into_iter().find(|&id| ui.get::<Edit>(id).is_some_and(|e| e.text == "1; 2.5")).expect("os números separados por ;");
    let _ = ui.set_prop(values, "text", "3; 4,5; 6");
    ui.set_modal_result(dialog, ModalResult::Ok);
    let value = result.borrow().clone().unwrap();
    assert_eq!(value[0]["values"], json!([3.0, 4.5, 6.0]));
    assert_eq!(value[0]["kind"], json!("line"), "a escolha que já estava fica");
}
