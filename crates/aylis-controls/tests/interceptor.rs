//! O `EventInterceptor` (item 18, 04/11): os eventos de qualquer componente num lugar só, antes
//! do handler, com o form, o componente, o evento e os argumentos; cancelar faz o handler não
//! rodar (e recusa um evento que é pergunta).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, Edit, EventInterceptor, Form};
use aylis_core::*;

type Log = Rc<RefCell<Vec<String>>>;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmPedido", 400.0, 300.0));
    ui.set_name(form, "frmPedido").unwrap();
    let button = ui.add(Button::new("Excluir"), form);
    ui.set_name(button, "btnExcluir").unwrap();
    (ui, form, button)
}

/// Um interceptador que anota cada evento ("form.componente.evento") e cancela os que `cancel`
/// diz.
fn interceptor(ui: &mut Ui, parent: ComponentId, scope: &str, log: &Log, cancel: impl Fn(&Intercepted) -> bool + 'static) -> ComponentId {
    let id = ui.add(EventInterceptor::default(), parent);
    ui.set_prop(id, "scope", scope).unwrap();
    let l = log.clone();
    ui.on(id, "on_intercept", move |ui, _, args| {
        let EventArgs::Intercept(i) = args else { panic!("argumentos: {args:?}") };
        let form = i.form.and_then(|f| ui.name(f)).unwrap_or_default().to_string();
        let sender = i.sender.and_then(|s| ui.name(s)).unwrap_or_default().to_string();
        l.borrow_mut().push(format!("{form}.{sender}.{}", i.event));
        if cancel(i) {
            ui.cancel_event();
        }
    });
    id
}

fn handler(ui: &mut Ui, id: ComponentId, event: &str, log: &Log) {
    let l = log.clone();
    let name = event.to_string();
    ui.on(id, event, move |_, _, _| l.borrow_mut().push(format!("handler {name}")));
}

#[test]
fn the_interceptor_sees_the_event_before_the_handler_and_can_cancel_it() {
    let (mut ui, form, button) = setup();
    let log: Log = Rc::default();
    let blocked = Rc::new(RefCell::new(true));
    let b = blocked.clone();
    interceptor(&mut ui, form, "form", &log, move |i| *b.borrow() && i.event == "on_click");
    handler(&mut ui, button, "on_click", &log);

    ui.fire(button, "on_click", &EventArgs::None);
    assert_eq!(*log.borrow(), ["frmPedido.btnExcluir.on_click"], "cancelado: o handler não rodou");

    log.borrow_mut().clear();
    *blocked.borrow_mut() = false;
    ui.fire(button, "on_click", &EventArgs::None);
    assert_eq!(*log.borrow(), ["frmPedido.btnExcluir.on_click", "handler on_click"]);
}

#[test]
fn events_without_a_handler_also_pass_and_after_intercept_runs_after() {
    let (mut ui, form, button) = setup();
    let log: Log = Rc::default();
    let i = interceptor(&mut ui, form, "form", &log, |_| false);
    let l = log.clone();
    ui.on(i, "after_intercept", move |_, _, args| {
        if let EventArgs::Intercept(i) = args {
            l.borrow_mut().push(format!("depois {}", i.event));
        }
    });
    ui.fire(button, "on_click", &EventArgs::None);
    assert_eq!(*log.borrow(), ["frmPedido.btnExcluir.on_click", "depois on_click"]);
}

#[test]
fn application_scope_reaches_every_form_and_runs_before_the_form_scope() {
    let (mut ui, form, button) = setup();
    let other = ui.add_root(Form::new("frmCliente", 400.0, 300.0));
    ui.set_name(other, "frmCliente").unwrap();
    let edit = ui.add(Edit::new(""), other);
    ui.set_name(edit, "edtNome").unwrap();
    let log: Log = Rc::default();
    let form_log: Log = Rc::default();
    interceptor(&mut ui, form, "form", &form_log, |_| false);
    let app = ui.add_root(Form::new("dmPrincipal", 10.0, 10.0));
    interceptor(&mut ui, app, "application", &log, |_| false);

    ui.fire(edit, "on_change", &EventArgs::Text("Ana".into()));
    ui.fire(button, "on_click", &EventArgs::None);
    assert_eq!(*log.borrow(), ["frmCliente.edtNome.on_change", "frmPedido.btnExcluir.on_click"]);
    assert_eq!(*form_log.borrow(), ["frmPedido.btnExcluir.on_click"], "o do form só vê o próprio form");
}

#[test]
fn filters_by_event_and_class_and_leaves_continuous_events_out_of_all() {
    let (mut ui, form, button) = setup();
    let edit = ui.add(Edit::new(""), form);
    ui.set_name(edit, "edtNome").unwrap();
    let log: Log = Rc::default();
    interceptor(&mut ui, form, "form", &log, |_| false);
    ui.fire(button, "on_pointer_move", &EventArgs::None);
    ui.fire(button, "on_timer", &EventArgs::None);
    assert!(log.borrow().is_empty(), "contínuos ficam fora do \"todos\": {:?}", log.borrow());

    let filtered: Log = Rc::default();
    let i = interceptor(&mut ui, form, "form", &filtered, |_| false);
    ui.set_prop(i, "events", "on_click; on_pointer_move").unwrap();
    ui.set_prop(i, "classes", "Button").unwrap();
    ui.fire(button, "on_pointer_move", &EventArgs::None);
    ui.fire(button, "on_click", &EventArgs::None);
    ui.fire(edit, "on_click", &EventArgs::None);
    ui.fire(button, "on_enter", &EventArgs::None);
    assert_eq!(*filtered.borrow(), ["frmPedido.btnExcluir.on_pointer_move", "frmPedido.btnExcluir.on_click"]);

    ui.set_prop(i, "enabled", false).unwrap();
    ui.fire(button, "on_click", &EventArgs::None);
    assert_eq!(filtered.borrow().len(), 2, "desligado não intercepta");
}

#[test]
fn cancelling_a_question_event_refuses_it() {
    let (mut ui, form, button) = setup();
    let log: Log = Rc::default();
    interceptor(&mut ui, form, "form", &log, |i| i.event == "on_close_query");
    let handled: Log = Rc::default();
    handler(&mut ui, button, "on_close_query", &handled);
    match ui.ask(button, "on_close_query", &EventArgs::None) {
        Asked::Answered(answer) => assert!(answer.is_refused(), "{answer:?}"),
        other => panic!("{other:?}"),
    }
    assert!(handled.borrow().is_empty());
}

#[test]
fn a_remote_interceptor_holds_the_event_until_it_answers() {
    let (mut ui, form, button) = setup();
    let i = ui.add(EventInterceptor::default(), form);
    ui.set_prop(i, "on_intercept", "interceptar").unwrap();
    // O "backend": adia toda pergunta, como o aylis-remote.
    let asked = Rc::new(RefCell::new(Vec::new()));
    let a = asked.clone();
    ui.set_event_sink(move |ui, _, event, _, _| {
        if ui.asking() {
            a.borrow_mut().push((event.to_string(), ui.defer_question()));
        }
    });
    let log: Log = Rc::default();
    handler(&mut ui, button, "on_click", &log);
    handler(&mut ui, button, "before_post", &log);

    // Evento comum: o handler roda quando o backend aceita.
    ui.fire(button, "on_click", &EventArgs::None);
    assert!(log.borrow().is_empty(), "espera a resposta");
    let (_, token) = asked.borrow_mut().remove(0);
    ui.answer(token, Answer::Accepted(Vec::new()));
    assert_eq!(*log.borrow(), ["handler on_click"]);

    // Evento-pergunta: a pergunta dele também espera, e a recusa do backend recusa.
    let outer = match ui.ask(button, "before_post", &EventArgs::None) {
        Asked::Deferred(t) => t,
        other => panic!("a pergunta precisa esperar o interceptador: {other:?}"),
    };
    let result = Rc::new(RefCell::new(None));
    let r = result.clone();
    ui.on_answer(outer, move |_, answer| *r.borrow_mut() = Some(answer));
    let (_, token) = asked.borrow_mut().remove(0);
    ui.answer(token, Answer::Refused("sem permissão".into()));
    assert_eq!(*result.borrow(), Some(Answer::Refused("sem permissão".into())));
    assert_eq!(log.borrow().len(), 1, "o before_post não rodou");
}
