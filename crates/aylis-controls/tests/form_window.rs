//! O que a janela informa ao form: a escala (`form_scale`, `on_scale_change`), o fim do
//! redimensionamento (`on_resize_end`), o mouse sobre a janela (`pointer_over`) e o trazer para
//! a frente (`bring_to_front`), pelo Rust e por nome.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::form_window::RESIZE_END_DELAY_MS;
use aylis_core::*;

fn ui() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None).unwrap();
    ui.show(form).unwrap();
    (ui, form)
}

/// Os eventos que o form recebeu, com o argumento.
fn record(ui: &mut Ui, form: ComponentId, event: &'static str) -> Rc<RefCell<Vec<EventArgs>>> {
    let got: Rc<RefCell<Vec<EventArgs>>> = Rc::default();
    let sink = got.clone();
    ui.on(form, event, move |_, _, args| sink.borrow_mut().push(args.clone()));
    got
}

#[test]
fn the_window_scale_is_known_and_its_change_is_an_event() {
    let (mut ui, form) = ui();
    let child = ui.create("Button", Some(form)).unwrap();
    assert_eq!(ui.form_scale(form), 1.0, "sem janela, 1");
    let got = record(&mut ui, form, "on_scale_change");
    ui.set_form_scale(form, 1.25);
    assert!(got.borrow().is_empty(), "a primeira vez é a escala de abrir, não uma mudança");
    assert_eq!(ui.form_scale(child), 1.25, "o controle dá o form dele");
    ui.set_form_scale(form, 1.25);
    ui.set_form_scale(form, 1.5);
    assert_eq!(*got.borrow(), vec![EventArgs::Value(PropValue::Float(1.5))]);
    assert_eq!(ui.invoke(form, "window_scale", &[]).unwrap(), PropValue::Float(1.5));
    ui.forget_form_window(form);
    assert_eq!(ui.form_scale(form), 1.0, "a janela fechou");
}

#[test]
fn the_resize_ends_after_a_quiet_moment() {
    let (mut ui, form) = ui();
    let got = record(&mut ui, form, "on_resize_end");
    ui.set_time(0.0);
    ui.form_resized(form, 800, 600, false);
    assert!(!ui.is_resizing(form), "o tamanho de abrir não é redimensionar");
    let delay = RESIZE_END_DELAY_MS as f64 / 1000.0;
    for (i, width) in [820u32, 840, 860].into_iter().enumerate() {
        ui.set_time(i as f64 * delay / 2.0);
        ui.form_resized(form, width, 600, false);
        ui.run_timers();
    }
    assert!(ui.is_resizing(form) && got.borrow().is_empty(), "cada tamanho novo adia o fim");
    ui.set_time(delay + delay * 1.01);
    ui.run_timers();
    assert_eq!(got.borrow().len(), 1);
    assert!(!ui.is_resizing(form));
    // O mesmo tamanho (a janela que volta de minimizada) e o 0 × 0 não contam.
    ui.form_resized(form, 0, 0, false);
    ui.form_resized(form, 860, 600, false);
    assert!(!ui.is_resizing(form));
    // Parado, nenhum timer fica esperando.
    assert_eq!(ui.next_wake(), None);
}

#[test]
fn holding_the_border_waits_for_the_release() {
    let (mut ui, form) = ui();
    let got = record(&mut ui, form, "on_resize_end");
    ui.set_time(0.0);
    ui.form_resized(form, 800, 600, false);
    ui.form_resized(form, 900, 600, true);
    ui.set_time(10.0);
    ui.run_timers();
    assert!(got.borrow().is_empty(), "com a borda segura, o tempo não encerra");
    ui.form_resize_released(form);
    assert_eq!(got.borrow().len(), 1);
    // Soltar sem ter mudado o tamanho (só arrastou a janela) não é fim de redimensionamento.
    ui.form_resize_released(form);
    assert_eq!(got.borrow().len(), 1);
}

/// O mouse num ponto e o form sob ele, como a plataforma diria.
struct FakeDesktop {
    cursor: Option<(i32, i32)>,
    form: Option<ComponentId>,
}

impl DesktopService for FakeDesktop {
    fn cursor_position(&self) -> Option<(i32, i32)> {
        self.cursor
    }
    fn form_at(&self, _x: i32, _y: i32) -> Option<ComponentId> {
        self.form
    }
}

#[test]
fn the_pointer_over_the_window_needs_the_system_to_tell() {
    let (mut ui, form) = ui();
    let other = ui.create("Form", None).unwrap();
    ui.show(other).unwrap();
    let button = ui.create("Button", Some(form)).unwrap();
    assert!(!ui.pointer_over(form), "sem o serviço (Wayland, celular, navegador): false");
    ui.services_mut().desktop = Some(Box::new(FakeDesktop { cursor: Some((5, 5)), form: Some(form) }));
    assert!(ui.pointer_over(form) && ui.pointer_over(button));
    assert!(!ui.pointer_over(other));
    assert_eq!(ui.invoke(form, "pointer_over", &[]).unwrap(), PropValue::Bool(true));
    ui.services_mut().desktop = Some(Box::new(FakeDesktop { cursor: None, form: Some(form) }));
    assert!(!ui.pointer_over(form), "sem a posição do mouse, false");
}

#[test]
fn bring_to_front_is_a_window_command_and_keeps_the_token_once() {
    let (mut ui, form) = ui();
    ui.take_window_commands();
    ui.invoke(form, "set_activation_token", &["  tok-123 ".into()]).unwrap();
    ui.invoke(form, "bring_to_front", &[]).unwrap();
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::BringToFront)]);
    assert_eq!(ui.take_activation_token().as_deref(), Some("tok-123"));
    assert_eq!(ui.take_activation_token(), None, "o compositor só aceita cada token uma vez");
    ui.set_activation_token("");
    assert_eq!(ui.take_activation_token(), None);
    // Os eventos existem no form, para o JSON e as outras linguagens.
    let props = ui.component(form).unwrap().props();
    for event in ["on_scale_change", "on_resize_end"] {
        assert!(props.iter().any(|p| p.name == event && p.kind == PropKind::Event), "{event}");
    }
}
