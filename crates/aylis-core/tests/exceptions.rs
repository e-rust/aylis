//! Um erro num handler ou num componente não derruba o app: vira uma exceção tratada (o
//! `Application.HandleException` do Delphi), e o resto continua.

mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::{RecordingCanvas, boxy};
use aylis_core::*;

/// Entra em pânico ao ser clicado ou pintado (conforme o campo).
#[derive(Default)]
struct Fragile {
    base: ControlBase,
    on_click: bool,
    on_paint: bool,
}

impl Component for Fragile {
    fn class_name(&self) -> &'static str {
        "Fragile"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }
    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.on_click {
            panic!("o componente quebrou");
        }
        ctx.fire("on_click", EventArgs::None);
    }
    fn paint(&self, _canvas: &mut dyn Canvas, _ctx: &PaintCtx<'_>) {
        if self.on_paint {
            panic!("a pintura quebrou");
        }
    }
}

fn setup() -> (Ui, ComponentId, Rc<RefCell<Vec<Exception>>>) {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    ui.set_name(form, "frmPedido").unwrap();
    ui.show(form).unwrap();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    ui.on_exception(move |_, e| {
        log.borrow_mut().push(e.clone());
        true
    });
    (ui, form, seen)
}

#[test]
fn a_panicking_handler_becomes_an_exception_and_the_app_goes_on() {
    let (mut ui, form, seen) = setup();
    let button = ui.add(Fragile::default(), form);
    ui.set_name(button, "btnSalvar").unwrap();
    let clicks = Rc::new(RefCell::new(0));
    let count = clicks.clone();
    ui.on(button, "on_click", move |_, _, _| {
        *count.borrow_mut() += 1;
        let empty: Option<i32> = None;
        empty.unwrap();
    });

    ui.click(button);
    ui.click(button);

    assert_eq!(*clicks.borrow(), 2, "o segundo clique ainda chega");
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].path, "frmPedido.btnSalvar");
    assert_eq!(seen[0].event, "on_click");
    assert!(seen[0].message.contains("None"), "{}", seen[0].message);
}

#[test]
fn a_panicking_component_method_keeps_the_component_in_place() {
    let (mut ui, form, seen) = setup();
    let button = ui.add(Fragile { base: ControlBase::new(100.0, 30.0), on_click: true, on_paint: false }, form);
    ui.set_name(button, "btnTotal").unwrap();

    assert!(!ui.click(button), "o clique falhou");

    assert_eq!(seen.borrow()[0].message, "o componente quebrou");
    assert_eq!(seen.borrow()[0].path, "frmPedido.btnTotal");
    // Voltou à árvore: não ficou "ocupado" para sempre.
    assert_eq!(ui.get_prop(button, "width"), Ok(PropValue::Float(100.0)));
}

#[test]
fn the_default_display_shows_what_the_app_did_not_handle() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(400.0, 300.0));
    let shown = Rc::new(RefCell::new(Vec::new()));
    let log = shown.clone();
    ui.set_exception_display(move |_, e| log.borrow_mut().push(e.message.clone()));
    ui.on_exception(|_, e| e.message == "tratado pelo app");

    ui.raise_exception(Exception { message: "tratado pelo app".into(), path: String::new(), event: String::new() });
    ui.raise_exception(Exception { message: "não tratado".into(), path: String::new(), event: String::new() });

    assert_eq!(*shown.borrow(), vec!["não tratado".to_string()]);
    let _ = form;
}

#[test]
fn an_error_while_handling_an_error_does_not_loop() {
    let mut ui = Ui::new();
    let calls = Rc::new(RefCell::new(0));
    let count = calls.clone();
    ui.on_exception(move |ui, _| {
        *count.borrow_mut() += 1;
        ui.raise_exception(Exception { message: "de novo".into(), path: String::new(), event: String::new() });
        panic!("o tratamento quebrou");
    });
    ui.raise_exception(Exception { message: "primeiro".into(), path: String::new(), event: String::new() });
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn a_paint_panic_marks_the_component_and_is_reported_once() {
    let (mut ui, form, _) = setup();
    let chart = ui.add(Fragile { base: ControlBase::new(100.0, 50.0), on_click: false, on_paint: true }, form);
    let errors = Rc::new(RefCell::new(0));
    let count = errors.clone();
    ui.add_observer(move |_, a| {
        if matches!(a, activity::Activity::Error { .. }) {
            *count.borrow_mut() += 1;
        }
    });
    ui.layout();

    for _ in 0..3 {
        let mut canvas = RecordingCanvas::default();
        ui.paint(form, &mut canvas);
    }

    assert!(ui.paint_failed(chart));
    assert_eq!(*errors.borrow(), 1, "uma vez, não uma por quadro");
}
