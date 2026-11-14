//! Eventos que perguntam (item 3): o handler em Rust responde na hora; um handler que adia (o
//! backend remoto) segura a ação até a resposta. E o caminho da tecla: `char_map`,
//! `char_case`, `key_preview`, `ApplicationEvents` e a fila de teclas.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{ApplicationEvents, Edit, Form};
use aylis_core::*;

fn key(c: char) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers::default(), repeat: false }
}

/// Um "backend" que adia toda pergunta e guarda o número dela (como o `aylis-remote`).
fn deferring_sink(ui: &mut Ui) -> Rc<RefCell<Vec<(String, u64)>>> {
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log = asked.clone();
    ui.set_event_sink(move |ui, _sender, event, _handler, _args| {
        if ui.asking() {
            let token = ui.defer_question();
            log.borrow_mut().push((event.to_string(), token));
        }
    });
    asked
}

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmTeste", 400.0, 300.0));
    let edit = ui.add(Edit::new(""), form);
    ui.show(form).unwrap();
    ui.set_focus(edit);
    (ui, form, edit)
}

fn text(ui: &Ui, id: ComponentId) -> String {
    ui.get_prop(id, "text").unwrap().to_str().unwrap().to_string()
}

#[test]
fn close_query_in_rust_answers_now_and_remote_waits_for_the_answer() {
    let (mut ui, form, _) = setup();
    ui.on(form, "on_close_query", |ui, _, _| ui.reply("can_close", false));
    assert!(!ui.close(form) && ui.is_open(form), "can_close falso mantém aberto");

    let (mut ui, form, _) = setup();
    let asked = deferring_sink(&mut ui);
    ui.set_prop(form, "on_close_query", PropValue::Event(Some("fechar".into()))).unwrap();
    assert!(!ui.close(form) && ui.is_open(form), "espera o backend");
    let token = asked.borrow()[0].1;
    // O segundo X durante a espera é a saída de emergência.
    let emergency = Rc::new(RefCell::new(0));
    let e = emergency.clone();
    ui.set_unresponsive_handler(move |_, _| *e.borrow_mut() += 1);
    ui.close(form);
    assert_eq!(*emergency.borrow(), 1);
    // Recusado: continua aberto; aceito: fecha sem perguntar de novo.
    ui.answer(token, Answer::Refused(String::new()));
    assert!(ui.is_open(form));
    ui.close(form);
    let token = asked.borrow()[1].1;
    ui.answer(token, Answer::Accepted(vec![]));
    assert!(!ui.is_open(form), "aceito, fecha");
    assert_eq!(asked.borrow().len(), 2, "não perguntou outra vez");
}

#[test]
fn a_remote_key_down_holds_the_next_keys_in_order() {
    let (mut ui, form, edit) = setup();
    let asked = deferring_sink(&mut ui);
    ui.set_prop(edit, "on_key_down", PropValue::Event(Some("tecla".into()))).unwrap();
    ui.key_down(form, key('a'));
    ui.text_input(form, "a");
    ui.key_down(form, key('b'));
    ui.text_input(form, "b");
    assert_eq!(text(&ui, edit), "", "nada entra antes da resposta");
    assert_eq!(asked.borrow().len(), 1, "a segunda tecla espera na fila");
    let first = asked.borrow()[0].1;
    ui.answer(first, Answer::Accepted(vec![]));
    assert_eq!(text(&ui, edit), "a");
    let second = asked.borrow()[1].1;
    // Engolida: o `on_key_down` não deixa passar (o caractere vem do `text_input`, que entra).
    ui.answer(second, Answer::Accepted(vec![("key".into(), PropValue::String(String::new()))]));
    assert_eq!(text(&ui, edit), "ab");
}

#[test]
fn key_press_replaces_or_swallows_the_char() {
    let (mut ui, form, edit) = setup();
    ui.on(edit, "on_key_press", |ui, _, args| {
        if let EventArgs::Text(t) = args {
            match t.as_str() {
                "x" => ui.suppress_key(),
                "a" => ui.replace_key_char('A'),
                _ => {}
            }
        }
    });
    ui.text_input(form, "axb");
    assert_eq!(text(&ui, edit), "Ab");
}

#[test]
fn char_map_and_char_case_change_what_is_typed() {
    let (mut ui, form, edit) = setup();
    ui.set_prop(form, "char_map", ",.").unwrap();
    ui.set_prop(edit, "char_case", PropValue::Enum("upper".into())).unwrap();
    ui.text_input(form, "a,ß");
    assert_eq!(text(&ui, edit), "A.SS");
    // O código que grava o texto também passa pelo `char_case`.
    ui.set_prop(edit, "text", "abc").unwrap();
    assert_eq!(text(&ui, edit), "ABC");
}

#[test]
fn application_events_and_key_preview_come_before_the_control() {
    let (mut ui, form, edit) = setup();
    let app = ui.add(ApplicationEvents::default(), form);
    let order = Rc::new(RefCell::new(Vec::new()));
    for (id, name) in [(app, "app"), (form, "form"), (edit, "edit")] {
        let order = order.clone();
        ui.on(id, "on_key_down", move |_, _, _| order.borrow_mut().push(name));
    }
    ui.set_prop(form, "key_preview", true).unwrap();
    ui.key_down(form, key('q'));
    assert_eq!(*order.borrow(), vec!["app", "form", "edit"]);
    // A aplicação engole: ninguém mais vê.
    order.borrow_mut().clear();
    ui.on(app, "on_key_down", |ui, _, _| ui.suppress_key());
    ui.key_down(form, key('q'));
    assert!(order.borrow().is_empty());
}

#[test]
fn application_on_exception_replaces_the_error_box() {
    let (mut ui, form, _) = setup();
    let app = ui.add(ApplicationEvents::default(), form);
    let shown = Rc::new(RefCell::new(0));
    let s = shown.clone();
    ui.set_exception_display(move |_, _| *s.borrow_mut() += 1);
    let got = Rc::new(RefCell::new(String::new()));
    let g = got.clone();
    ui.on(app, "on_exception", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            *g.borrow_mut() = t.clone();
        }
    });
    ui.raise_exception(Exception { message: "falhou \"x\"".into(), path: "frmTeste".into(), event: "on_click".into() });
    assert_eq!(*shown.borrow(), 0);
    assert!(got.borrow().contains(r#""message":"falhou \"x\"""#), "{}", got.borrow());
}

#[test]
fn a_busy_backend_refuses_clicks_and_holds_keys_until_done() {
    let (mut ui, form, edit) = setup();
    let button = ui.add(aylis_controls::Button::new("OK"), form);
    ui.control_mut(button).unwrap().set_rect(10.0, 100.0, 100.0, 30.0);
    ui.layout();
    let clicks = Rc::new(RefCell::new(0));
    let c = clicks.clone();
    ui.on(button, "on_click", move |_, _, _| *c.borrow_mut() += 1);
    ui.set_focus(edit);
    let at = PointerEvent { position: PointF::new(50.0, 110.0), ..Default::default() };

    ui.backend_event_sent(7, button);
    assert!(ui.backend_busy());
    ui.pointer_down(form, at);
    ui.pointer_up(form, at);
    assert_eq!(*clicks.borrow(), 0, "o clique é recusado enquanto o backend roda");
    ui.text_input(form, "ab");
    assert_eq!(text(&ui, edit), "", "as teclas esperam");
    // Passou do delay: o cursor de espera; depois de uns 3 s, o indicador.
    ui.set_time(ui.time() + 1.5);
    ui.tick();
    assert_eq!(ui.cursor(form), CursorShape::Wait);
    ui.backend_done(7);
    assert!(!ui.backend_busy());
    assert_eq!(text(&ui, edit), "ab", "e entram depois, em ordem");
    assert_ne!(ui.cursor(form), CursorShape::Wait);
    ui.pointer_down(form, at);
    ui.pointer_up(form, at);
    assert_eq!(*clicks.borrow(), 1);

    // Uma janela aberta pelo pedido espera o usuário: a entrada volta.
    ui.backend_event_sent(8, button);
    ui.backend_waiting_user(true);
    assert!(!ui.backend_busy());
    ui.backend_waiting_user(false);
    assert!(ui.backend_busy());
    ui.backend_done(8);
}

/// A tecla Voltar do celular (`Ui::back_pressed`, decidido em 14/11): sem handler, o de sempre;
/// o `on_back` que responde `handled` (ou recusa) trata a tecla e o de sempre não roda; com o
/// backend em outra linguagem, o de sempre espera a resposta.
#[test]
fn the_back_key_asks_the_top_form_first() {
    let (mut ui, form, _) = setup();
    let ran = Rc::new(RefCell::new(0));
    let fallback = |ran: &Rc<RefCell<i32>>| {
        let r = ran.clone();
        move |_: &mut Ui| *r.borrow_mut() += 1
    };

    // Sem handler: o de sempre.
    ui.back_pressed(fallback(&ran));
    assert_eq!(*ran.borrow(), 1);

    // O app trata (volta a tela de dentro).
    ui.on(form, "on_back", |ui, _, _| ui.reply("handled", true));
    ui.back_pressed(fallback(&ran));
    assert_eq!(*ran.borrow(), 1, "tratado: o de sempre não roda");

    // Respondeu sem tratar: o de sempre.
    ui.on(form, "on_back", |ui, _, _| ui.reply("handled", false));
    ui.back_pressed(fallback(&ran));
    assert_eq!(*ran.borrow(), 2);

    // O backend em outra linguagem: o de sempre espera a resposta.
    let (mut ui2, form2, _) = setup();
    let asked = deferring_sink(&mut ui2);
    ui2.set_prop(form2, "on_back", PropValue::Event(Some("voltar".into()))).unwrap();
    ui2.back_pressed(fallback(&ran));
    assert_eq!(*ran.borrow(), 2, "espera a resposta");
    let token = asked.borrow().iter().find(|(e, _)| e == "on_back").map(|(_, t)| *t).unwrap();
    ui2.answer(token, Answer::Accepted(vec![]));
    assert_eq!(*ran.borrow(), 3, "sem `handled`: o de sempre");
}
