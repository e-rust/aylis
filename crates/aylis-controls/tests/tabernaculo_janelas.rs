//! O relatório do Tabernáculo, lote 2 (janelas e plataforma): o que dá para conferir sem janela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn count(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<usize>> {
    let n = Rc::new(RefCell::new(0));
    let c = n.clone();
    ui.on(id, event, move |_, _, _| *c.borrow_mut() += 1);
    n
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 400.0, 300.0));
    ui.show(form).unwrap();
    ui.layout();
    (ui, form)
}

/// P10 e P08: o `one_shot` dispara uma vez; o `restart` conta de novo; o `advance` anda.
#[test]
fn a_one_shot_timer_fires_once_and_restarts() {
    let (mut ui, form) = setup();
    let timer = ui.add(Timer::new(100), form);
    ui.set_prop(timer, "one_shot", true).unwrap();
    let fired = count(&mut ui, timer, "on_timer");
    for _ in 0..5 {
        ui.advance(0.1);
    }
    ui.process_events();
    assert_eq!(*fired.borrow(), 1);
    assert_eq!(ui.get_prop(timer, "enabled").unwrap(), PropValue::Bool(false));
    Timer::restart(&mut ui, timer);
    ui.advance(0.05);
    ui.advance(0.06);
    ui.process_events();
    assert_eq!(*fired.borrow(), 2);
}

/// J23: o Enter que ninguém usou clica o botão padrão; o Esc, o de cancelar.
#[test]
fn enter_and_escape_click_the_default_and_cancel_buttons() {
    let (mut ui, form) = setup();
    let ok = ui.add(Button::new("OK"), form);
    let cancel = ui.add(Button::new("Cancelar"), form);
    let label = ui.add(Label::new("x"), form);
    ui.set_prop(ok, "default_button", true).unwrap();
    ui.set_prop(cancel, "cancel_button", true).unwrap();
    let (oks, cancels) = (count(&mut ui, ok, "on_click"), count(&mut ui, cancel, "on_click"));
    let _ = label;
    ui.key_down(form, KeyEvent { key: Key::Enter, ..Default::default() });
    ui.key_down(form, KeyEvent { key: Key::Escape, ..Default::default() });
    ui.process_events();
    assert_eq!((*oks.borrow(), *cancels.borrow()), (1, 1));
}

/// J21/J22: `close_action = free` libera o form depois de fechar.
#[test]
fn close_action_free_releases_the_form() {
    let (mut ui, _) = setup();
    let other = ui.add_root(Form::new("y", 200.0, 100.0));
    ui.set_prop(other, "close_action", PropValue::Enum("free".into())).unwrap();
    ui.show(other).unwrap();
    ui.close(other);
    ui.advance(0.01);
    assert!(!ui.contains(other));
}

/// P05: o `FrameTimer` só dispara ligado, a cada quadro.
#[test]
fn the_frame_timer_fires_each_frame_while_enabled() {
    let (mut ui, form) = setup();
    let frames = ui.add(FrameTimer::default(), form);
    let fired = count(&mut ui, frames, "on_frame");
    ui.tick();
    ui.process_events();
    assert_eq!(*fired.borrow(), 0, "começa desligado");
    FrameTimer::set_enabled(&mut ui, frames, true);
    for i in 0..3 {
        ui.set_time(1.0 + i as f64 / 60.0);
        ui.tick();
    }
    ui.process_events();
    assert_eq!(*fired.borrow(), 3);
}

/// L13: o tamanho de um controle alinhado é o que o layout deu; P07: o `tag_string`.
#[test]
fn an_aligned_width_is_the_laid_out_one_and_tag_string_keeps_text() {
    let (mut ui, form) = setup();
    let top = ui.add(Panel::new(100.0, 40.0), form);
    ui.control_mut(top).unwrap().set_align(Align::Top);
    ui.layout();
    assert_eq!(ui.get_prop(top, "width").unwrap(), PropValue::Float(400.0));
    ui.set_prop(top, "tag_string", "cliente:42").unwrap();
    assert_eq!(ui.get_prop(top, "tag_string").unwrap(), PropValue::String("cliente:42".into()));
}

/// J13: o atalho escrito para mostrar.
#[test]
fn shortcut_text_writes_the_shortcut() {
    assert_eq!(shortcut_text("ctrl+shift+s"), "Ctrl+Shift+S");
    assert_eq!(shortcut_text("del"), "Del");
    assert_eq!(shortcut_text("alt+f4"), "Alt+F4");
}
