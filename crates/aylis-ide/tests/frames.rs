//! Frames no designer: o conteúdo do form do frame aparece, mas clicar seleciona o frame,
//! a estrutura não lista o que é do frame e salvar não grava o conteúdo dele.

use aylis_core::PointF;
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};
use serde_json::{Value, json};

#[test]
fn the_designer_shows_the_frame_content_but_keeps_it_out_of_the_form() {
    let dir = std::env::temp_dir().join(format!("aylis_ide_frames_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut project = aylis_project::create(&dir, "Telas", aylis_project::Language::Rust).unwrap();
    project.add_form("fraTopo", "").unwrap();
    std::fs::write(
        project.file(&project.form("fraTopo").unwrap().file),
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "fraTopo", "children": [
            { "class": "Button", "name": "btnSalvar", "align": "client" } ] } }"#,
    )
    .unwrap();
    project.save().unwrap();
    let form_file = project.file(&project.form(&project.main_form).unwrap().file);
    let mut form: Value = serde_json::from_str(&std::fs::read_to_string(&form_file).unwrap()).unwrap();
    form["root"]["children"] = json!([{ "class": "Frame", "name": "frmTopo", "form": "fraTopo", "left": 0, "top": 0, "width": 200, "height": 60 }]);
    std::fs::write(&form_file, serde_json::to_string_pretty(&form).unwrap()).unwrap();

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let main = ide.form;
    ide.ui.show(main).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let mut st = state.borrow_mut();
    let doc = &mut st.doc;
    let frame = doc.ui.find_in(doc.root, "frmTopo").unwrap();
    assert!(doc.ui.find_path("fraTopo.btnSalvar").is_some(), "o conteúdo do frame aparece");
    // Clicar no botão do frame seleciona o frame.
    assert_eq!(doc.component_at(PointF::new(100.0, 30.0)), Some(frame));
    // A estrutura e o arquivo não levam o conteúdo do frame.
    assert!(doc.tree().iter().all(|(id, _)| doc.ui.name(*id) != Some("btnSalvar")));
    let saved = aylis_json::save_value(&doc.ui, doc.root).unwrap();
    let frame_json = &saved["root"]["children"][0];
    assert_eq!(frame_json["form"], "fraTopo");
    assert!(frame_json.get("children").is_none(), "{frame_json}");
    // Trocar o form do frame troca o conteúdo.
    doc.set_prop(frame, "form", &json!("")).unwrap();
    assert!(doc.ui.find_path("fraTopo.btnSalvar").is_none());
    doc.set_prop(frame, "form", &json!("fraTopo")).unwrap();
    assert!(doc.ui.find_path("fraTopo.btnSalvar").is_some());
    // Desfazer mantém o frame mostrando o conteúdo.
    assert!(doc.undo());
    assert!(doc.ui.find_path("fraTopo.btnSalvar").is_none());
    drop(st);
    let _ = std::fs::remove_dir_all(&dir);
}
