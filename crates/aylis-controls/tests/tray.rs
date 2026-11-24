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
    tray.menu_items = r#"[{"text":"Abrir"},{"separator":true},{"text":"Sair"}]"#.into();
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

/// Abre o menu de `items` no ponto (1800, 1050) com escala 1 e devolve o form do menu.
fn open_tray_menu(ui: &mut Ui, items: &str) -> ComponentId {
    let form = ui.add_root(Form::new("app", 400.0, 300.0));
    ui.show(form).unwrap();
    let mut tray = TrayIcon::default();
    tray.menu_items = items.into();
    let tray = ui.add(tray, form);
    ui.send_message(tray, &TrayEvent { kind: TrayEventKind::RightClick, screen: PointF::new(1800.0, 1050.0), scale: 1.0 });
    ui.process_posted();
    *ui.open_forms().last().unwrap()
}

const ITEMS: &str = r#"[{"text":"Abrir"},{"separator":true},{"text":"Sair"}]"#;

/// Sem medidas no tema, o menu de sempre: 220 de largura, item de 32 e divisória de 9.
#[test]
fn the_tray_menu_keeps_its_sizes_without_theme_tokens() {
    let mut ui = Ui::new();
    let menu = open_tray_menu(&mut ui, ITEMS);
    let p = ui.window_placement(menu).unwrap();
    assert_eq!((p.width, p.height), (220, 32 + 9 + 32 + 8));
}

/// O estilo `PopupMenu` dá a largura, a altura do item e a da divisória; os cliques seguem as
/// linhas novas.
#[test]
fn the_tray_menu_sizes_come_from_the_popup_menu_style() {
    let mut ui = Ui::new();
    let mut theme = ui.theme().clone();
    let props = StyleProps { width: Some(300.0), item_height: Some(40.0), separator_height: Some(13.0), ..Default::default() };
    theme.styles.insert("PopupMenu".into(), Style::new(props));
    ui.set_theme(theme);
    let menu = open_tray_menu(&mut ui, ITEMS);
    let p = ui.window_placement(menu).unwrap();
    assert_eq!((p.width, p.height), (300, 40 + 13 + 40 + 8));
    assert_eq!(ui.theme().menu_metrics().item_height, Some(40.0));

    let chosen: Rc<RefCell<Vec<String>>> = Rc::default();
    let tray = *ui.children(ui.open_forms()[0]).last().unwrap();
    let log = chosen.clone();
    ui.on(tray, "on_menu", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            log.borrow_mut().push(t.clone());
        }
    });
    ui.update_layout();
    // No item de 32 de antes já é a divisória (4 + 40 + 13): "Sair" começa em 57.
    let e = PointerEvent { position: PointF::new(40.0, 4.0 + 40.0 + 13.0 + 20.0), button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(menu, e);
    ui.pointer_up(menu, e);
    ui.process_posted();
    assert_eq!(chosen.borrow().as_slice(), ["Sair"]);
}

/// Sem largura no estilo, o texto mais longo alarga o menu (e o curto não o estreita).
#[test]
fn the_tray_menu_grows_for_the_longest_text() {
    let mut ui = Ui::new();
    let long = "Abrir a janela principal do aplicativo com todas as opções";
    let menu = open_tray_menu(&mut ui, &format!(r#"[{{"text":"{long}"}},{{"text":"Sair"}}]"#));
    let width = ui.window_placement(menu).unwrap().width as f32;
    let font = ui.theme().resolve(&["TrayMenuList"], "", ControlState::default()).font;
    let text = ui.measure_text(long, &TextStyle { font, ..Default::default() }).width;
    assert!(width > 220.0 && width >= 4.0 + 30.0 + text + 4.0, "o texto não cabe: {width} para {text}");
}
