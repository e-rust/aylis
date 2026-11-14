//! O teclado do form: as teclas no lugar do Tab, os caracteres de cada campo (com o form e
//! os painéis) e o Alt+letra do `&` nas legendas.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::platform::ClipboardService;
use aylis_core::*;

#[derive(Clone, Default)]
struct FakeClipboard(Rc<RefCell<String>>);

impl ClipboardService for FakeClipboard {
    fn get_text(&mut self) -> Option<String> {
        Some(self.0.borrow().clone())
    }
    fn set_text(&mut self, text: &str) {
        *self.0.borrow_mut() = text.to_string();
    }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn with(key: Key, modifiers: Modifiers) -> KeyEvent {
    KeyEvent { key, modifiers, ..Default::default() }
}

fn text(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

/// Um form com três campos (o do meio é um Memo) e um painel com um campo.
fn setup() -> (Ui, ComponentId, [ComponentId; 4], ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    let a = ui.add(Edit::new(""), form);
    let memo = ui.add(Memo::new(""), form);
    let b = ui.add(Edit::new(""), form);
    let panel = ui.add(Panel::new(200.0, 100.0), form);
    let c = ui.add(Edit::new(""), panel);
    for (id, name) in [(a, "edtA"), (memo, "memo"), (b, "edtB"), (c, "edtC"), (panel, "pnl")] {
        ui.set_name(id, name).unwrap();
    }
    ui.layout();
    (ui, form, [a, memo, b, c], panel)
}

#[test]
fn enter_works_as_tab_but_the_memo_keeps_it() {
    let (mut ui, form, [a, memo, b, _], _) = setup();
    ui.set_prop(form, "tab_keys", "Enter").unwrap();
    ui.set_prop(form, "back_tab_keys", "Shift+Enter;Up").unwrap();
    assert!(ui.set_focus(a));
    ui.key_down(form, key(Key::Enter));
    assert_eq!(ui.focused(form), Some(memo), "Enter pula do campo como o Tab");
    ui.key_down(form, key(Key::Enter));
    assert_eq!(ui.focused(form), Some(memo), "no Memo o Enter é nova linha");
    assert_eq!(text(&ui, memo), "\n");
    ui.set_focus(b);
    ui.key_down(form, with(Key::Enter, Modifiers { shift: true, ..Default::default() }));
    assert_eq!(ui.focused(form), Some(memo), "Shift+Enter volta");
    // Pelo código.
    ui.set_focus(a);
    assert!(ui.focus_next(form, true));
    assert_eq!(ui.focused(form), Some(memo));
}

#[test]
fn the_field_decides_first_and_the_parent_can_override() {
    let (mut ui, form, [a, _, _, c], panel) = setup();
    // O form bloqueia "@"; o campo A libera.
    ui.set_prop(form, "blocked_chars", "@").unwrap();
    ui.set_prop(a, "allowed_chars", "@").unwrap();
    ui.set_focus(a);
    ui.text_input(form, "a@b");
    assert_eq!(text(&ui, a), "a@b", "o filho libera");
    ui.set_focus(c);
    ui.text_input(form, "a@b");
    assert_eq!(text(&ui, c), "ab", "o campo C segue o form");
    // O painel manda nos filhos: só números, mesmo que o campo libere letras.
    ui.set_prop(panel, "allowed_chars", "0-9").unwrap();
    ui.set_prop(panel, "only_allowed", true).unwrap();
    ui.set_prop(panel, "chars_override", true).unwrap();
    ui.set_prop(c, "allowed_chars", "a-z").unwrap();
    ui.set_prop(c, "text", "").unwrap();
    ui.text_input(form, "x1y2");
    assert_eq!(text(&ui, c), "12", "o painel com prioridade manda");
    // O colado passa pelo mesmo filtro.
    let clipboard = FakeClipboard::default();
    *clipboard.0.borrow_mut() = "3a4".into();
    ui.services_mut().clipboard = Some(Box::new(clipboard));
    ui.key_down(form, with(Key::Char('v'), Modifiers { ctrl: true, ..Default::default() }));
    assert_eq!(text(&ui, c), "1234");
}

#[test]
fn alt_letter_clicks_the_ampersand_control() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    let save = ui.add(Button::new("&Salvar"), form);
    let label = ui.add(Label::new("&Nome"), form);
    let name = ui.add(Edit::new(""), form);
    let check = ui.add(CheckBox::new("&Ativo"), form);
    ui.set_prop(label, "focus_control", PropValue::Component(Some(name))).unwrap();
    ui.layout();
    let clicks = Rc::new(Cell::new(0));
    {
        let clicks = clicks.clone();
        ui.on_click(save, move |_, _| clicks.set(clicks.get() + 1));
    }
    let alt = Modifiers { alt: true, ..Default::default() };
    ui.key_down(form, with(Key::Char('s'), alt));
    ui.process_posted();
    assert_eq!(clicks.get(), 1, "Alt+S clica no &Salvar");
    ui.key_down(form, with(Key::Char('N'), alt));
    assert_eq!(ui.focused(form), Some(name), "o rótulo leva ao FocusControl");
    ui.key_down(form, with(Key::Char('a'), alt));
    assert_eq!(ui.get_prop(check, "checked").ok(), Some(PropValue::Bool(true)));
    // A IDE avisa as letras repetidas.
    let other = ui.add(Button::new("&Sair"), form);
    let conflicts = ui.mnemonic_conflicts(form);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].0, 's');
    assert!(conflicts[0].1.contains(&save) && conflicts[0].1.contains(&other));
}

/// Um form com um botão em foco que conta os cliques.
fn button_setup() -> (Ui, ComponentId, ComponentId, Rc<Cell<u32>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    let button = ui.add(Button::new("Somar"), form);
    let clicks = Rc::new(Cell::new(0));
    let c = clicks.clone();
    ui.on(button, "on_click", move |_, _, _| c.set(c.get() + 1));
    ui.layout();
    assert!(ui.set_focus(button));
    (ui, form, button, clicks)
}

/// O Enter clica ao apertar e, seguro, repete o clique (a
/// repetição do teclado); com `repeat_enter` desligado, só o primeiro.
#[test]
fn holding_enter_on_a_button_repeats_the_click() {
    let (mut ui, form, button, clicks) = button_setup();
    assert_eq!(ui.get_prop(button, "repeat_enter").unwrap(), PropValue::Bool(true), "ligado por padrão");
    let repeat = KeyEvent { key: Key::Enter, repeat: true, ..Default::default() };
    ui.key_down(form, key(Key::Enter));
    assert_eq!(clicks.get(), 1, "clica ao apertar");
    ui.key_down(form, repeat);
    ui.key_down(form, repeat);
    ui.key_up(form, key(Key::Enter));
    assert_eq!(clicks.get(), 3, "seguro, repete");

    ui.set_prop(button, "repeat_enter", false).unwrap();
    ui.key_down(form, key(Key::Enter));
    ui.key_down(form, repeat);
    ui.key_down(form, repeat);
    ui.key_up(form, key(Key::Enter));
    assert_eq!(clicks.get(), 4, "desligado, só o primeiro");
}

/// O Espaço afunda o botão e clica ao soltar (e não repete); soltar depois de o foco sair não
/// clica, nem o Espaço que só é solto no botão (apertado em outro lugar).
#[test]
fn space_clicks_a_button_on_release() {
    let (mut ui, form, button, clicks) = button_setup();
    ui.key_down(form, key(Key::Space));
    assert_eq!(clicks.get(), 0, "apertar não clica");
    assert!(ui.control(button).unwrap().state().pressed, "o botão afunda");
    ui.key_down(form, KeyEvent { key: Key::Space, repeat: true, ..Default::default() });
    ui.key_up(form, key(Key::Space));
    assert_eq!(clicks.get(), 1, "clica ao soltar, uma vez");
    assert!(!ui.control(button).unwrap().state().pressed);

    // Solto sem ter sido apertado aqui: nada.
    ui.key_up(form, key(Key::Space));
    assert_eq!(clicks.get(), 1);

    // O foco sai com o Espaço apertado: solta sem clicar.
    let other = ui.add(Edit::new(""), form);
    ui.layout();
    ui.key_down(form, key(Key::Space));
    assert!(ui.set_focus(other));
    assert!(!ui.control(button).unwrap().state().pressed);
    assert!(ui.set_focus(button));
    ui.key_up(form, key(Key::Space));
    assert_eq!(clicks.get(), 1, "o foco saiu: não clica");
}

/// Ctrl+C na caixa de mensagem copia o texto dela, como no Windows (o título, o texto e os
/// botões).
#[test]
fn ctrl_c_copies_the_message_box() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let clipboard = FakeClipboard::default();
    ui.services_mut().clipboard = Some(Box::new(clipboard.clone()));
    let form = ui.add_root(Form::new("Teste", 400.0, 300.0));
    ui.show(form).unwrap();
    let dlg = message_dlg(&mut ui, "O arquivo não foi salvo.", DlgKind::Error, &[ModalResult::Ok], |_, _| {});
    ui.layout();
    ui.key_down(dlg, with(Key::Char('c'), Modifiers { ctrl: true, ..Default::default() }));
    let copied = clipboard.0.borrow().clone();
    assert!(copied.contains("O arquivo não foi salvo."), "{copied}");
    assert!(copied.contains("Erro") && copied.contains("OK"), "o título e o botão, como no Windows: {copied}");
    assert!(ui.is_modal(dlg), "copiar não fecha a caixa");
}
