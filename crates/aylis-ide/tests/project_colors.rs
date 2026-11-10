//! Um form que usa as cores do projeto pelo nome (`"fill": "green"`) abre no designer, e a
//! versão clara/escura do form continua com essas cores.

use aylis_core::{Color, PropValue};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};
use serde_json::{Value, json};

#[test]
fn a_form_with_project_colors_opens_in_the_designer() {
    let dir = std::env::temp_dir().join(format!("aylis_ide_project_colors_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let project = aylis_project::create(&dir, "Cores", aylis_project::Language::Rust).unwrap();

    // As cores no .aylisproj e um painel do form principal pintado com elas.
    let mut proj: Value = serde_json::from_str(&std::fs::read_to_string(&project.path).unwrap()).unwrap();
    proj["colors"] = json!({ "green": "#10B981", "painel": { "dark": "#111111", "light": "#EEEEEE" } });
    std::fs::write(&project.path, serde_json::to_string_pretty(&proj).unwrap()).unwrap();
    let form_file = project.file(&project.form(&project.main_form).unwrap().file);
    let mut form: Value = serde_json::from_str(&std::fs::read_to_string(&form_file).unwrap()).unwrap();
    let children = form["root"].as_object_mut().unwrap().entry("children").or_insert(json!([]));
    children.as_array_mut().unwrap().push(json!({ "class": "Panel", "name": "pnlVerde", "fill": "green" }));
    children.as_array_mut().unwrap().push(json!({ "class": "Panel", "name": "pnlModo", "fill": "painel" }));
    std::fs::write(&form_file, serde_json::to_string_pretty(&form).unwrap()).unwrap();

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let main = ide.form;
    ide.ui.show(main).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let status = state.borrow().status.clone();
    assert!(!status.contains('⚠'), "{status}");
    let st = state.borrow();
    let doc = &st.doc;
    let panel = doc.ui.find("pnlVerde").expect("o form abriu com o painel");
    // No designer a propriedade guarda o nome (é ele que vai para o arquivo).
    assert_eq!(doc.ui.get_prop(panel, "fill").unwrap(), PropValue::String("green".into()));
    drop(st);

    // "Form escuro"/"Form claro": as cores do projeto continuam, no modo certo.
    let mut st = state.borrow_mut();
    st.doc.set_dark(true);
    assert_eq!(st.doc.theme.resolve_color("green"), Some(Color::rgb(0x10, 0xB9, 0x81)));
    assert_eq!(st.doc.theme.resolve_color("painel"), Some(Color::rgb(0x11, 0x11, 0x11)));
    st.doc.set_dark(false);
    assert_eq!(st.doc.theme.resolve_color("painel"), Some(Color::rgb(0xEE, 0xEE, 0xEE)));
    drop(st);
    let _ = std::fs::remove_dir_all(&dir);
}
