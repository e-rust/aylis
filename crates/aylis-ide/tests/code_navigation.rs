//! Voltar e avançar no editor de código (07/11): os botões extras do mouse e Alt+← / Alt+→
//! levam aos lugares por onde o editor passou (outro arquivo, outra linha), como no VS Code.

use aylis_controls::Memo;
use aylis_core::{Key, KeyEvent, Modifiers, PointerButton, PointerEvent, Ui};

fn caret(ui: &Ui, memo: aylis_core::ComponentId) -> usize {
    ui.get::<Memo>(memo).unwrap().selection().0
}

#[test]
fn the_extra_mouse_buttons_go_back_and_forward_through_the_places_visited() {
    let mut ui = Ui::new();
    let dir = std::env::temp_dir().join(format!("aylis_code_nav_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let a = dir.join("a.rs");
    let b = dir.join("b.rs");
    std::fs::write(&a, "fn a() {}\n\nfn um() {}\nfn dois() {}\n").unwrap();
    std::fs::write(&b, "fn b() {}\n").unwrap();

    let first = aylis_ide::codewindow::open(&mut ui, &a, 3).unwrap();
    let at_line_3 = caret(&ui, first.memo);
    let second = aylis_ide::codewindow::open(&mut ui, &b, 1).unwrap();
    let form = second.form;
    assert_ne!(first.memo, second.memo, "outro arquivo, outra aba");
    ui.set_focus(second.memo);

    let mouse = |button| PointerEvent { button, ..Default::default() };
    ui.pointer_down(form, mouse(PointerButton::Back));
    let now = aylis_ide::codewindow::open_files(&ui);
    assert!(!now.is_empty());
    assert_eq!(ui.focused(form), Some(first.memo), "voltou para a.rs");
    assert_eq!(caret(&ui, first.memo), at_line_3, "na mesma linha");

    let alt = |key| KeyEvent { key, modifiers: Modifiers { alt: true, ..Default::default() }, repeat: false };
    ui.key_down(form, alt(Key::Right));
    assert_eq!(ui.focused(form), Some(second.memo), "avançou para b.rs");
    ui.key_down(form, alt(Key::Left));
    assert_eq!(ui.focused(form), Some(first.memo));
    let _ = std::fs::remove_dir_all(&dir);
}
