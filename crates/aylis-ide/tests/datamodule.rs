//! DataModule na IDE: criar, só não visuais, o form escolhe os que usa e enxerga os
//! componentes deles no Object Inspector; o `uses` vai para o layout.

use std::path::PathBuf;

use aylis_core::PropValue;
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_form_uses_the_components_of_a_data_module() {
    let base = temp("datamodule");
    let mut project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    project.add_data_module("dmDados").unwrap();
    project.save().unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);

    // O DataModule: só não visuais.
    actions::open_project_form(&state, "dmDados");
    {
        let mut st = state.borrow_mut();
        assert!(st.doc.is_data_module());
        assert!(st.doc.add("Button", None).is_err(), "visual não entra");
        let menu = st.doc.add("PopupMenu", None).unwrap();
        st.doc.ui.set_name(menu, "pmAcoes").unwrap();
        let path = st.doc.path.clone().unwrap();
        st.doc.save_to(&path).unwrap();
    }

    // O form principal passa a usar o DataModule e vê o menu dele.
    actions::open_project_form(&state, "frmPrincipal");
    let (modules, current) = actions::uses_choices(&state).unwrap();
    assert_eq!(modules, ["dmDados"]);
    assert!(current.is_empty());
    actions::set_uses(&state, vec!["dmDados".into()]);
    {
        let mut st = state.borrow_mut();
        let button = st.doc.add("Button", None).unwrap();
        let candidates = st.doc.reference_candidates(button);
        assert!(candidates.contains(&"dmDados.pmAcoes".to_string()), "{candidates:?}");
        st.doc.ui.set_prop(button, "popup_menu", "dmDados.pmAcoes").unwrap();
        let json = st.doc.to_json().unwrap();
        assert!(json.contains("\"uses\"") && json.contains("dmDados.pmAcoes"), "{json}");
        let path = st.doc.path.clone().unwrap();
        st.doc.save_to(&path).unwrap();
        // O DataModule não vai junto no arquivo do form.
        assert!(!std::fs::read_to_string(&path).unwrap().contains("\"DataModule\""));
    }
    let reopened = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(reopened.uses_of("frmPrincipal"), ["dmDados"]);
    assert_eq!(reopened.used_by("dmDados"), ["frmPrincipal"]);

    // Reabrir o form: a referência continua resolvida.
    let fresh = shared(Document::new());
    let mut ide2 = app::build(fresh.clone());
    actions::open_project(&mut ide2.ui, &fresh, &ide2.ids, &project.path);
    actions::open_project_form(&fresh, "frmPrincipal");
    let st = fresh.borrow();
    assert!(st.status.is_empty() || !st.status.starts_with('⚠'), "{}", st.status);
    let button = st.doc.ui.find_in(st.doc.root, "Button1").unwrap();
    let menu = st.doc.ui.find_path("dmDados.pmAcoes").unwrap();
    assert_eq!(st.doc.ui.get_prop(button, "popup_menu").unwrap(), PropValue::Component(Some(menu)));
}
