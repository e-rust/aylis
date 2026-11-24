//! Os recursos de janela e de sistema nos componentes: o `place` do form, o
//! `border_color: none` do `WindowSettings`, o `on_second_instance` do `ApplicationEvents`, os
//! métodos e eventos novos do `Screen` e o `on_context`/`open_menu` do `TrayIcon`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::*;

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

#[test]
fn place_asks_the_window_for_a_physical_rectangle() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    ui.show(form).unwrap();
    ui.take_window_commands();
    ui.invoke(form, "place", &[1.into(), 10.into(), 20.into(), 800.into(), 300.into(), false.into()]).unwrap();
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::Place { monitor: 1, x: 10, y: 20, width: 800, height: 300, activate: false })]);
    assert!(ui.invoke(form, "place", &[0.into(), 0.into(), 0.into(), 0.into(), 300.into()]).is_err(), "largura zero");
    // O evento existe no form (só o Windows dispara).
    assert!(ui.component(form).unwrap().props().iter().any(|p| p.name == "on_topmost_lost" && p.kind == PropKind::Event));
}

#[test]
fn the_window_border_can_be_none() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let settings = ui.create("WindowSettings", Some(form)).unwrap();
    assert_eq!(ui.window_options(form).border_color, BorderColor::System);
    ui.set_prop(settings, "border_color", "none").unwrap();
    assert_eq!(ui.window_options(form).border_color, BorderColor::None);
    assert_eq!(ui.get_prop(settings, "border_color").unwrap(), PropValue::String("none".into()));
    ui.set_prop(settings, "border_color", Color::rgb(1, 2, 3)).unwrap();
    assert_eq!(ui.window_options(form).border_color, BorderColor::Color(Color::rgb(1, 2, 3)));
    ui.set_prop(settings, "border_color", PropValue::Null).unwrap();
    assert_eq!(ui.window_options(form).border_color, BorderColor::System);
}

#[test]
fn a_second_instance_brings_the_window_forward_and_gives_the_arguments() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let events = ui.create("ApplicationEvents", Some(form)).unwrap();
    ui.show(form).unwrap();
    ui.set_main_form(form);
    let got: Rc<RefCell<Vec<String>>> = Rc::default();
    let sink = got.clone();
    ui.on(events, "on_second_instance", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            sink.borrow_mut().push(t.clone());
        }
    });
    ui.take_window_commands();
    ui.fire_second_instance(vec!["--abrir".into(), "a;b.txt".into()]);
    assert_eq!(*got.borrow(), vec!["--abrir;a;b.txt".to_string()]);
    assert_eq!(ui.second_instance_args(), ["--abrir".to_string(), "a;b.txt".to_string()]);
    assert_eq!(ui.take_window_commands(), vec![(form, WindowCommand::Show)]);
}

#[test]
fn the_screen_answers_outside_the_windows() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let screen = ui.create("Screen", Some(form)).unwrap();
    ui.show(form).unwrap();
    // Sem plataforma: o mouse não se sabe, nenhum form no ponto, a transparência ligada.
    assert_eq!(ui.invoke(screen, "cursor_position", &[]).unwrap(), PropValue::String("null".into()));
    assert_eq!(ui.invoke(screen, "form_at", &[0.into(), 0.into()]).unwrap(), PropValue::Component(None));
    assert_eq!(ui.invoke(screen, "system_transparency", &[]).unwrap(), PropValue::Bool(true));
    // A mudança da transparência sai no `on_transparency_change` (o timer do `Screen`).
    let got: Rc<RefCell<Vec<String>>> = Rc::default();
    let sink = got.clone();
    ui.on(screen, "on_transparency_change", move |_, _, args| sink.borrow_mut().push(format!("{args:?}")));
    ui.set_system_transparency(true);
    ui.advance(1.1);
    ui.run_timers();
    ui.set_system_transparency(false);
    ui.advance(1.1);
    ui.run_timers();
    assert_eq!(*got.borrow(), vec![format!("{:?}", EventArgs::Value(PropValue::Bool(false)))]);
    // A captura sem o `allow_screen_capture`: o erro no `on_capture`.
    let sink = got.clone();
    ui.on(screen, "on_capture", move |_, _, args| sink.borrow_mut().push(format!("{args:?}")));
    ui.set_monitors(vec![aylis_core::monitors::Monitor { width: 100, height: 100, ..Default::default() }]);
    ui.invoke(screen, "capture_monitor", &[0.into(), 50.into()]).unwrap();
    ui.run_timers();
    let last = got.borrow().last().cloned().unwrap_or_default();
    assert!(last.contains("monitor") && last.contains("allow_screen_capture"), "{last}");
}

/// A sessão pelo nome (as outras linguagens): o estado no `session_locked` e no
/// `session_connected`, e o `on_session_change` a cada mudança (a primeira informação não é
/// mudança).
#[test]
fn the_screen_tells_the_session_state_and_its_changes() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let screen = ui.create("Screen", Some(form)).unwrap();
    ui.show(form).unwrap();
    assert_eq!(ui.invoke(screen, "session_locked", &[]).unwrap(), PropValue::Bool(false));
    assert_eq!(ui.invoke(screen, "session_connected", &[]).unwrap(), PropValue::Bool(true));
    let got: Rc<RefCell<Vec<(bool, bool)>>> = Rc::default();
    let sink = got.clone();
    ui.on(screen, "on_session_change", move |ui, _, _| sink.borrow_mut().push((ui.session_locked(), ui.session_connected())));
    ui.set_system_session(false, true);
    ui.advance(1.1);
    ui.run_timers();
    ui.set_system_session(true, true);
    ui.advance(1.1);
    ui.run_timers();
    ui.set_system_session(true, false);
    ui.advance(1.1);
    ui.run_timers();
    assert_eq!(*got.borrow(), vec![(true, true), (true, false)]);
    assert_eq!(ui.invoke(screen, "session_locked", &[]).unwrap(), PropValue::Bool(true));
}

/// Os métodos do app inteiro no `ApplicationEvents`: sem a plataforma do desktop, iniciar com o
/// sistema é desligado e ligar dá o erro; soltar a instância única não faz nada.
#[test]
fn the_application_events_answer_the_app_wide_methods() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let app = ui.create("ApplicationEvents", Some(form)).unwrap();
    assert_eq!(ui.invoke(app, "start_with_login", &[]).unwrap(), PropValue::Bool(false));
    assert!(ui.invoke(app, "set_start_with_login", &[true.into()]).is_err());
    assert!(ui.invoke(app, "set_start_with_login", &[]).is_err(), "sem o valor");
    assert_eq!(ui.invoke(app, "release_single_instance", &[]).unwrap(), PropValue::Null);
}

/// O clique direito no ícone, como a plataforma entrega.
fn right_click(ui: &mut Ui, tray: ComponentId) {
    let event = TrayEvent { kind: TrayEventKind::RightClick, screen: PointF::new(500.0, 400.0), scale: 1.0 };
    ui.send_message(tray, &event);
    ui.process_posted();
}

fn menu_open(ui: &Ui, before: usize) -> bool {
    ui.open_forms().len() > before
}

#[test]
fn the_tray_menu_asks_on_context_first() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let tray = ui.create("TrayIcon", Some(form)).unwrap();
    ui.set_prop(tray, "menu_items", r#"[{"text": "Abrir"}, {"text": "Sair"}]"#).unwrap();
    ui.show(form).unwrap();
    let before = ui.open_forms().len();
    // O erro do handler não abre o menu.
    let refuse = Rc::new(RefCell::new(true));
    let points: Rc<RefCell<Vec<PointF>>> = Rc::default();
    let (r, p) = (refuse.clone(), points.clone());
    ui.on(tray, "on_context", move |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            p.borrow_mut().push(e.position);
        }
        if *r.borrow() {
            ui.refuse("não agora");
        }
    });
    right_click(&mut ui, tray);
    assert_eq!(*points.borrow(), vec![PointF::new(500.0, 400.0)], "o ponto da tela vem no evento");
    assert!(!menu_open(&ui, before), "recusado: o menu não abre");
    *refuse.borrow_mut() = false;
    right_click(&mut ui, tray);
    assert!(menu_open(&ui, before), "aceito: o menu abre");
}

#[test]
fn the_tray_menu_waits_for_a_deferred_answer_and_opens_by_code() {
    let mut ui = ui();
    let form = ui.create("Form", None).unwrap();
    let tray = ui.create("TrayIcon", Some(form)).unwrap();
    ui.set_prop(tray, "menu_items", r#"[{"text": "Abrir"}]"#).unwrap();
    ui.show(form).unwrap();
    let before = ui.open_forms().len();
    let token: Rc<RefCell<Option<u64>>> = Rc::default();
    let t = token.clone();
    ui.on(tray, "on_context", move |ui, _, _| *t.borrow_mut() = Some(ui.defer_question()));
    right_click(&mut ui, tray);
    assert!(!menu_open(&ui, before), "adiado: ainda não abre");
    let token = token.borrow().unwrap();
    ui.answer(token, Answer::Accepted(Vec::new()));
    assert!(menu_open(&ui, before), "a resposta aceita abre");
    // Por código: o `open_menu` abre de novo (sem passar pelo `on_context`).
    let open = ui.open_forms().to_vec();
    for f in open.into_iter().filter(|f| *f != form) {
        ui.close(f);
    }
    ui.invoke(tray, "open_menu", &[100.into(), 100.into()]).unwrap();
    ui.process_posted();
    assert!(menu_open(&ui, before));
}
