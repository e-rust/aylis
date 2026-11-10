//! O Memo como editor de código: desfazer, localizar, as operações de linha, os pares, o
//! Ctrl+D e a lista de sugestões.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn with(key: Key, ctrl: bool, shift: bool, alt: bool) -> KeyEvent {
    KeyEvent { key, modifiers: Modifiers { ctrl, shift, alt, ..Default::default() }, ..Default::default() }
}

fn setup(text: &str) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 600.0, 400.0));
    let mut memo = Memo::new(text);
    memo.syntax = "rust".into();
    memo.want_tabs = true;
    memo.code_editing = true;
    let m = ui.add(memo, form);
    ui.control_mut(m).unwrap().set_align(Align::Client);
    ui.layout();
    ui.set_focus(m);
    (ui, form, m)
}

fn text(ui: &Ui, m: ComponentId) -> String {
    ui.get::<Memo>(m).unwrap().text.clone()
}

fn caret_to(ui: &mut Ui, m: ComponentId, line: usize, col: usize) {
    ui.get_mut::<Memo>(m).unwrap().go_to(line, col);
}

#[test]
fn typing_undoes_in_one_step_and_redoes() {
    let (mut ui, form, m) = setup("");
    ui.text_input(form, "a");
    ui.text_input(form, "b");
    ui.text_input(form, "c");
    assert_eq!(text(&ui, m), "abc");
    ui.key_down(form, with(Key::Char('z'), true, false, false));
    assert_eq!(text(&ui, m), "", "a digitação seguida volta inteira");
    ui.key_down(form, with(Key::Char('y'), true, false, false));
    assert_eq!(text(&ui, m), "abc");
}

#[test]
fn pairs_close_and_skip() {
    let (mut ui, form, m) = setup("");
    ui.text_input(form, "f");
    ui.text_input(form, "(");
    assert_eq!(text(&ui, m), "f()");
    ui.text_input(form, "x");
    ui.text_input(form, ")");
    assert_eq!(text(&ui, m), "f(x)", "o ) que já existe só é pulado");
    ui.key_down(form, key(Key::End));
    ui.text_input(form, "[");
    ui.key_down(form, key(Key::Backspace));
    assert_eq!(text(&ui, m), "f(x)", "Backspace num par vazio apaga os dois");
    // O apóstrofo no meio de uma palavra não abre par.
    ui.text_input(form, " d");
    ui.text_input(form, "'");
    assert_eq!(text(&ui, m), "f(x) d'");
}

#[test]
fn line_operations_like_vscode() {
    let (mut ui, form, m) = setup("um\ndois\ntres");
    caret_to(&mut ui, m, 1, 1);
    ui.key_down(form, with(Key::Down, false, false, true));
    assert_eq!(text(&ui, m), "dois\num\ntres", "Alt+Down desce a linha");
    ui.key_down(form, with(Key::Down, false, true, true));
    assert_eq!(text(&ui, m), "dois\num\num\ntres", "Shift+Alt+Down copia");
    ui.key_down(form, with(Key::Char('k'), true, true, false));
    assert_eq!(text(&ui, m), "dois\num\ntres", "Ctrl+Shift+K apaga");
    caret_to(&mut ui, m, 1, 1);
    ui.key_down(form, with(Key::Char('/'), true, false, false));
    assert_eq!(text(&ui, m), "// dois\num\ntres", "Ctrl+/ comenta");
    ui.key_down(form, with(Key::Char('/'), true, false, false));
    assert_eq!(text(&ui, m), "dois\num\ntres", "de novo descomenta");
    ui.key_down(form, with(Key::Enter, true, false, false));
    assert_eq!(text(&ui, m), "dois\n\num\ntres", "Ctrl+Enter abre linha embaixo");
    // Tab com várias linhas selecionadas recua todas.
    ui.get_mut::<Memo>(m).unwrap().select(0, 13);
    ui.key_down(form, key(Key::Tab));
    assert_eq!(text(&ui, m), "    dois\n\n    um\n    tres");
    ui.key_down(form, with(Key::Tab, false, true, false));
    assert_eq!(text(&ui, m), "dois\n\num\ntres");
}

#[test]
fn find_replace_and_ctrl_d() {
    let (mut ui, form, m) = setup("let a = 1;\nlet ab = a + a;\n");
    {
        let memo = ui.get_mut::<Memo>(m).unwrap();
        assert_eq!(memo.find_all("a", true, true).len(), 3, "palavra inteira: não pega o 'ab'");
        assert_eq!(memo.find_all("LET", false, false).len(), 2);
        assert_eq!(memo.replace_all("a", "x", true, true, 0.0), 3);
        assert_eq!(memo.text, "let x = 1;\nlet ab = x + x;\n");
        assert!(memo.undo());
        assert_eq!(memo.text, "let a = 1;\nlet ab = a + a;\n");
    }
    caret_to(&mut ui, m, 1, 6);
    ui.key_down(form, with(Key::Char('d'), true, false, false));
    assert_eq!(ui.get::<Memo>(m).unwrap().sel_text(), "a", "Ctrl+D pega a palavra");
    ui.key_down(form, with(Key::Char('d'), true, false, false));
    assert_eq!(ui.get::<Memo>(m).unwrap().selection(), (20, 21), "de novo, a próxima palavra inteira (não o 'a' do 'ab')");
}

#[test]
fn suggestions_filter_and_accept() {
    let (mut ui, form, m) = setup("");
    let asked = Rc::new(RefCell::new(Vec::new()));
    {
        let asked = asked.clone();
        ui.on(m, "on_complete_request", move |_, _, args| asked.borrow_mut().push(format!("{args:?}")));
    }
    ui.text_input(form, "pr");
    ui.process_posted();
    assert!(!asked.borrow().is_empty(), "digitar pede sugestões");
    let items = ["print", "println", "process"].map(|l| Suggestion { label: l.into(), kind: "function".into(), ..Default::default() }).to_vec();
    ui.get_mut::<Memo>(m).unwrap().show_suggestions(items, 0);
    ui.text_input(form, "i");
    assert!(ui.get::<Memo>(m).unwrap().suggestions_visible());
    ui.key_down(form, key(Key::Down));
    ui.key_down(form, key(Key::Enter));
    assert_eq!(text(&ui, m), "println", "a segunda das que casam com 'pri'");
    assert!(!ui.get::<Memo>(m).unwrap().suggestions_visible());
}

#[test]
fn lsp_positions_count_utf16() {
    let (ui, _, m) = setup("é\n😀ab");
    let memo = ui.get::<Memo>(m).unwrap();
    // "😀" vale 2 unidades UTF-16 e 1 caractere.
    assert_eq!(memo.index_of_position(1, 2), 3);
    assert_eq!(memo.position_of_index(4), (1, 3));
    assert_eq!(memo.index_of_position(0, 1), 1);
}
