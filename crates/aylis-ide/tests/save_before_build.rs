//! Antes de compilar, os fontes alterados nos editores vão para o disco (a compilação lê o
//! disco: o app rodava com o código velho; conferência de 08/11).

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{app, codewindow};

#[test]
fn edited_code_is_saved_before_building() {
    let dir = std::env::temp_dir().join(format!("aylis-salvar-antes-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("principal.py");
    std::fs::write(&file, "def ok_click(aylis, event):\n    pass\n").unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    codewindow::open(&mut ide.ui, &file, 1).unwrap();
    codewindow::edit_text(&mut ide.ui, &file, "def ok_click(aylis, event):\n    pass\n", "", false).unwrap();
    assert!(std::fs::read_to_string(&file).unwrap().contains("ok_click"), "ainda não salvo");
    assert_eq!(codewindow::save_all(&mut ide.ui), 1);
    assert!(!std::fs::read_to_string(&file).unwrap().contains("ok_click"), "salvo antes de compilar");
    assert_eq!(codewindow::save_all(&mut ide.ui), 0, "nada mais a salvar");
    let _ = std::fs::remove_dir_all(&dir);
}
