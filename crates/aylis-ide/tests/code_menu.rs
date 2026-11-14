//! Recortar, Copiar, Colar e Selecionar tudo no menu do editor de código (13/11): no topo do
//! menu do botão direito (o editor embutido na IDE) e no Editar da janela própria, pelo mesmo
//! caminho do teclado.

use aylis_controls::{Memo, MenuItem, PopupMenu};
use aylis_core::{ComponentId, Ui};

fn item(ui: &Ui, form: ComponentId, text: &str) -> ComponentId {
    ui.components()
        .find(|&c| ui.get::<MenuItem>(c).is_some_and(|m| m.text == text) && ui.root_of(c) == ui.root_of(form))
        .unwrap_or_else(|| panic!("o item {text} não está no menu"))
}

fn text(ui: &Ui, memo: ComponentId) -> String {
    ui.get::<Memo>(memo).unwrap().text.clone()
}

fn file(name: &str, content: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_code_menu_{name}_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("a.rs");
    std::fs::write(&path, content).unwrap();
    path
}

#[test]
fn the_code_window_cuts_copies_and_pastes_from_the_edit_menu() {
    let mut ui = Ui::new();
    let path = file("window", "fn um() {}\n");
    let code = aylis_ide::codewindow::open(&mut ui, &path, 1).unwrap();
    let (form, memo) = (code.form, code.memo);

    ui.get_mut::<Memo>(memo).unwrap().select(3, 5);
    ui.click(item(&ui, form, "Copiar"));
    assert_eq!(aylis_core::clipboard::text(&mut ui).as_deref(), Some("um"));

    ui.click(item(&ui, form, "Recortar"));
    assert_eq!(text(&ui, memo), "fn () {}\n");

    ui.click(item(&ui, form, "Colar"));
    assert_eq!(text(&ui, memo), "fn um() {}\n");

    ui.click(item(&ui, form, "Selecionar tudo"));
    assert_eq!(ui.get::<Memo>(memo).unwrap().sel_text(), "fn um() {}\n");
    let _ = std::fs::remove_dir_all(path.parent().unwrap());
}

#[test]
fn the_embedded_editor_has_the_clipboard_items_at_the_top_of_the_right_click_menu() {
    let mut ui = Ui::new();
    let host = ui.add_root(aylis_controls::Form::new("IDE", 800.0, 600.0));
    aylis_ide::codewindow::set_host(Some(host));
    let path = file("embedded", "abc\n");
    let code = aylis_ide::codewindow::open(&mut ui, &path, 1).unwrap();
    let popup = ui.components().find(|&c| ui.get::<PopupMenu>(c).is_some() && ui.root_of(c) == ui.root_of(code.form)).expect("o menu do botão direito");
    let top: Vec<String> = ui.children(popup).iter().filter_map(|&c| ui.get::<MenuItem>(c).map(|m| m.text.clone())).collect();
    assert_eq!(&top[..6], ["Recortar", "Copiar", "Colar", "-", "Selecionar tudo", "-"]);

    ui.get_mut::<Memo>(code.memo).unwrap().select(0, 3);
    ui.click(item(&ui, code.form, "Recortar"));
    assert_eq!(text(&ui, code.memo), "\n");
    ui.click(item(&ui, code.form, "Colar"));
    assert_eq!(text(&ui, code.memo), "abc\n");
    aylis_ide::codewindow::set_host(None);
    let _ = std::fs::remove_dir_all(path.parent().unwrap());
}
