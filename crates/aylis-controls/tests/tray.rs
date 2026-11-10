//! TrayIcon: a descrição para a plataforma, os cliques e o menu próprio.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

#[test]
fn spec_clicks_and_menu() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("app", 400.0, 300.0));
    ui.show(form).unwrap();
    let mut tray = TrayIcon::default();
    tray.hint = "Meu app".into();
    tray.badge = "3".into();
    tray.menu_items = "Abrir;-;Sair".into();
    tray.menu_theme = "dark".into();
    let tray = ui.add(tray, form);

    let specs = ui.trays();
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].1.tooltip, "Meu app");
    assert_eq!(specs[0].1.badge, "3");

    let log: Rc<RefCell<Vec<String>>> = Rc::default();
    for event in ["on_click", "on_menu"] {
        let log = log.clone();
        ui.on(tray, event, move |_, _, args| {
            log.borrow_mut().push(match args {
                EventArgs::Text(t) => format!("{event}:{t}"),
                _ => event.to_string(),
            })
        });
    }
    ui.send_message(tray, &TrayEvent { kind: TrayEventKind::Click, screen: PointF::new(1800.0, 1050.0), scale: 1.5 });
    ui.process_posted();
    assert_eq!(log.borrow().as_slice(), ["on_click"]);

    // Clique direito: o menu abre acima do ponto, por cima de tudo e fora da barra de tarefas.
    ui.send_message(tray, &TrayEvent { kind: TrayEventKind::RightClick, screen: PointF::new(1800.0, 1050.0), scale: 1.5 });
    ui.process_posted();
    let menu = *ui.open_forms().last().unwrap();
    assert_ne!(menu, form);
    assert_eq!(ui.get_prop(menu, "topmost").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(menu, "show_in_taskbar").unwrap(), PropValue::Bool(false));
    let p = ui.window_placement(menu).unwrap();
    assert!(p.y + p.height as i32 <= 1050 && p.x + p.width as i32 <= 1800, "{p:?}");

    // Escolher "Sair" (o terceiro item, depois da linha).
    ui.update_layout();
    let at = PointF::new(40.0, 4.0 + 32.0 + 9.0 + 16.0);
    let e = PointerEvent { position: at, button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(menu, e);
    ui.pointer_up(menu, e);
    ui.process_posted();
    assert_eq!(log.borrow().last().unwrap(), "on_menu:Sair");
    assert!(!ui.is_open(menu), "o menu fecha ao escolher");
}
