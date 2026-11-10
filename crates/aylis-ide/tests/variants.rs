//! Variantes de form na IDE: cada uma na sua aba, salva no próprio arquivo.

use std::path::PathBuf;

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
fn each_variant_opens_in_its_own_tab_and_saves_to_its_file() {
    let base = temp("variantes");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    project.add_variant("frmPrincipal", "android").unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);

    actions::open_project_form(&state, "frmPrincipal");
    actions::open_project_variant(&state, "frmPrincipal", Some("android"));
    {
        let mut st = state.borrow_mut();
        assert_eq!(st.tab_count(), 2, "padrão e android em abas separadas");
        assert!(st.doc.path.as_ref().unwrap().ends_with("principal.android.json"));
        let b = st.doc.add("Button", None).unwrap();
        st.doc.ui.set_name(b, "btnSoNoAndroid").unwrap();
        let path = st.doc.path.clone().unwrap();
        st.doc.save_to(&path).unwrap();
    }
    // Voltar ao padrão troca de aba (não abre outra).
    actions::open_project_form(&state, "frmPrincipal");
    assert_eq!(state.borrow().tab_count(), 2);
    assert!(state.borrow().doc.path.as_ref().unwrap().ends_with("principal.json"));
    let default = std::fs::read_to_string(project.file("forms/principal.json")).unwrap();
    let android = std::fs::read_to_string(project.file("forms/principal.android.json")).unwrap();
    assert!(!default.contains("btnSoNoAndroid") && android.contains("btnSoNoAndroid"));
}
