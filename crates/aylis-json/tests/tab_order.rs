//! A ordem do Tab do form (`tab_order`, entre níveis) e o primeiro foco (`active_control`).

use aylis_core::*;

const DOC: &str = r#"{
  "aylis": 1,
  "root": {
    "class": "Form", "name": "frmCliente", "width": 600, "height": 400,
    "active_control": "edtEmail",
    "tab_order": "edtCidade; edtNome; naoExiste",
    "children": [
      { "class": "Panel", "name": "pnlDados", "left": 10, "top": 10, "width": 280, "height": 200, "children": [
        { "class": "Edit", "name": "edtNome", "left": 10, "top": 10, "width": 200, "height": 30 },
        { "class": "Edit", "name": "edtEmail", "left": 10, "top": 50, "width": 200, "height": 30 }
      ] },
      { "class": "GroupBox", "name": "grpEndereco", "left": 300, "top": 10, "width": 280, "height": 200, "children": [
        { "class": "Edit", "name": "edtCidade", "left": 10, "top": 30, "width": 200, "height": 30 }
      ] },
      { "class": "Button", "name": "btnSalvar", "text": "Salvar", "left": 10, "top": 300, "width": 100, "height": 32 }
    ]
  }
}"#;

fn name(ui: &Ui, id: Option<ComponentId>) -> String {
    id.and_then(|i| ui.name(i)).unwrap_or_default().to_string()
}

#[test]
fn tab_follows_the_form_list_across_levels_and_starts_at_active_control() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, DOC, None).unwrap();
    ui.layout();
    ui.show(form).unwrap();
    // Abriu com o foco no active_control (um campo de dentro do painel).
    assert_eq!(name(&ui, ui.focused(form)), "edtEmail");
    let names = |ui: &Ui| ui.tab_order(form).into_iter().map(|i| ui.name(i).unwrap_or_default().to_string()).collect::<Vec<_>>();
    // A lista primeiro (do grupo para o painel; o nome que não existe é ignorado), depois
    // os que faltam, na ordem da árvore.
    assert_eq!(names(&ui), ["edtCidade", "edtNome", "edtEmail", "btnSalvar"]);
    let tab = KeyEvent { key: Key::Tab, ..Default::default() };
    let back = KeyEvent { key: Key::Tab, modifiers: Modifiers { shift: true, ..Default::default() }, repeat: false };
    ui.key_down(form, tab);
    assert_eq!(name(&ui, ui.focused(form)), "btnSalvar");
    ui.key_down(form, tab);
    assert_eq!(name(&ui, ui.focused(form)), "edtCidade", "volta ao começo da lista (não sai do form)");
    ui.key_down(form, tab);
    assert_eq!(name(&ui, ui.focused(form)), "edtNome", "pula do grupo para o painel");
    ui.key_down(form, back);
    assert_eq!(name(&ui, ui.focused(form)), "edtCidade");
    // Salvar e carregar mantém as duas propriedades.
    let saved = aylis_json::save(&ui, form).unwrap();
    assert!(saved.contains("\"active_control\": \"edtEmail\""), "{saved}");
    assert!(saved.contains("edtCidade; edtNome"), "{saved}");
}

#[test]
fn runtime_properties_are_not_saved() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, r#"{ "aylis": 1, "root": { "class": "Form", "name": "f", "children": [
        { "class": "Edit", "name": "e", "text": "abc", "caret_style": "underline" } ] } }"#, None).unwrap();
    let e = ui.find_in(form, "e").unwrap();
    ui.set_prop(e, "sel_start", 1_i64).unwrap();
    let saved = aylis_json::save(&ui, form).unwrap();
    assert!(!saved.contains("sel_start") && !saved.contains("sel_text"), "{saved}");
    assert!(saved.contains("\"caret_style\": \"underline\""), "{saved}");
}
