//! O `WindowSettings` (pedido do autor em 16/11): as configurações de janela que só alguns
//! sistemas têm, num componente. O do form vale para a janela dele; com `scope` = `app`, vale
//! para as janelas dos forms sem um próprio; trocar em execução avisa as janelas.

use aylis_controls::*;
use aylis_core::*;

fn setup() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

#[test]
fn the_form_settings_win_over_the_app_ones() {
    let mut ui = setup();
    let main = ui.add_root(Form::new("frmPrincipal", 400.0, 300.0));
    let print = ui.add_root(Form::new("frmPrint", 400.0, 300.0));
    let other = ui.add_root(Form::new("frmOutro", 400.0, 300.0));
    assert_eq!(ui.window_options(main), WindowOptions::default(), "sem componente: o do sistema");
    assert!(ui.window_settings_of(main).is_none());

    // O do app, no form principal: vale para todas as janelas sem um próprio.
    let app = ui.create("WindowSettings", Some(main)).unwrap();
    ui.set_prop(app, "scope", "app").unwrap();
    ui.set_prop(app, "capture_excluded", true).unwrap();
    // O do print: só a janela dele.
    let own = ui.create("WindowSettings", Some(print)).unwrap();
    ui.set_prop(own, "animation", false).unwrap();
    ui.set_prop(own, "backdrop", "mica").unwrap();
    ui.set_prop(own, "title_color", "#102030").unwrap();
    ui.set_prop(own, "alt_tab", false).unwrap();

    let p = ui.window_options(print);
    assert!(!p.animation && !p.alt_tab && !p.capture_excluded, "o do form ganha do do app: {p:?}");
    assert_eq!(p.backdrop, WindowBackdrop::Mica);
    assert_eq!(p.title_color, Some(Color::rgb(0x10, 0x20, 0x30)));
    assert!(ui.window_options(other).capture_excluded && ui.window_options(other).animation, "o do app nos outros");
    assert!(ui.window_options(main).capture_excluded);
}

#[test]
fn a_change_while_running_tells_the_windows() {
    let mut ui = setup();
    let form = ui.add_root(Form::new("frmHud", 400.0, 300.0));
    let settings = ui.create("WindowSettings", Some(form)).unwrap();
    let _ = ui.take_window_commands();
    ui.set_prop(settings, "click_through", true).unwrap();
    assert!(ui.take_window_commands().contains(&(form, WindowCommand::UpdateOptions)));
    assert!(ui.window_options(form).click_through);
}

/// O form principal nunca é possuído por outro: um app que abre antes uma janela por monitor e
/// uma paleta `topmost` e só depois o principal (o `run` mostra o principal por último) deixava
/// o principal possuído pela paleta, por cima dela e com o `topmost` dela (testado em 20/11).
#[test]
fn the_main_form_is_never_owned() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.create("Form", None).unwrap();
    let screen2 = ui.create("Form", None).unwrap();
    let palette = ui.create("Form", None).unwrap();
    ui.show(screen2).unwrap();
    ui.show(palette).unwrap();
    assert_eq!(ui.form_owner(palette), Some(screen2), "os outros continuam possuídos pelo anterior");
    // Como o `run`: escolhe o principal e o mostra.
    ui.set_main_form(main);
    ui.show(main).unwrap();
    assert_eq!(ui.form_owner(main), None);
    // Já aberto e escolhido depois: deixa de ser possuído.
    ui.set_main_form(palette);
    assert_eq!(ui.form_owner(palette), None);
}

/// O form entra numa faixa de abas pelo `host` (vira uma aba com o título dele, sem a janela,
/// sem fechar) e sai dela (volta a ter janela, com o tamanho de antes), com os avisos.
#[test]
fn a_form_docks_into_a_tab_strip_and_back() {
    use std::cell::RefCell;
    use std::rc::Rc;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.create("Form", None).unwrap();
    ui.set_prop(main, "width", 900.0).unwrap();
    ui.set_prop(main, "height", 600.0).unwrap();
    let strip = ui.create("TabStrip", Some(main)).unwrap();
    ui.set_prop(strip, "align", "top").unwrap();
    let pages = ui.create("Layout", Some(main)).unwrap();
    ui.set_prop(pages, "align", "client").unwrap();
    ui.set_prop(strip, "page_host", PropValue::Component(Some(pages))).unwrap();
    let child = ui.create("Form", None).unwrap();
    ui.set_prop(child, "caption", "Clientes").unwrap();
    ui.set_prop(child, "width", 520.0).unwrap();
    ui.set_prop(child, "height", 340.0).unwrap();
    let events = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_dock", "on_undock", "on_close"] {
        let events = events.clone();
        ui.on(child, event, move |_, _, _| events.borrow_mut().push(event));
    }
    ui.set_main_form(main);
    ui.show(main).unwrap();
    ui.show(child).unwrap();
    assert!(ui.is_open(child));

    ui.set_prop(child, "host", PropValue::Component(Some(strip))).unwrap();
    ui.advance(0.05);
    assert!(!ui.is_open(child), "a janela some");
    assert_eq!(ui.parent(child), Some(pages));
    let titles: Vec<String> = ui.get::<aylis_controls::TabStrip>(strip).unwrap().tabs().iter().map(|t| t.title.clone()).collect();
    assert_eq!(titles, ["Clientes"]);
    assert_eq!(events.borrow().as_slice(), ["on_dock"], "sem o on_close");

    ui.set_prop(child, "host", PropValue::Component(None)).unwrap();
    ui.advance(0.05);
    assert!(ui.is_open(child), "volta a ter janela");
    assert_eq!(ui.parent(child), None);
    assert!(ui.get::<aylis_controls::TabStrip>(strip).unwrap().tabs().is_empty());
    let size = ui.control(child).map(|c| (c.width(), c.height()));
    assert_eq!(size, Some((520.0, 340.0)), "o tamanho de antes");
    assert_eq!(events.borrow().as_slice(), ["on_dock", "on_undock"]);

    // O principal não entra.
    assert!(aylis_controls::docking::dock_form(&mut ui, main, strip, None).is_err());
}

/// A janela de um form `dockable` arrastada (pela plataforma) até uma faixa com `accept_forms`
/// entra como aba no lugar onde foi solta; sem `accept_forms`, não.
#[test]
fn a_dragged_window_docks_where_it_is_dropped() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.create("Form", None).unwrap();
    ui.set_prop(main, "width", 900.0).unwrap();
    ui.set_prop(main, "height", 600.0).unwrap();
    let strip = ui.create("TabStrip", Some(main)).unwrap();
    ui.set_prop(strip, "align", "top").unwrap();
    ui.set_prop(strip, "height", 40.0).unwrap();
    let child = ui.create("Form", None).unwrap();
    ui.set_prop(child, "caption", "Clientes").unwrap();
    ui.set_prop(child, "dockable", true).unwrap();
    ui.set_main_form(main);
    ui.show(main).unwrap();
    ui.show(child).unwrap();
    ui.layout();
    // Sem `accept_forms`: nada acontece.
    ui.window_drag_over(child, Some((main, PointF::new(300.0, 20.0))));
    assert!(!ui.window_drag_end(child));
    assert!(ui.is_open(child));
    ui.set_prop(strip, "accept_forms", true).unwrap();
    ui.window_drag_over(child, Some((main, PointF::new(300.0, 20.0))));
    assert!(ui.window_drag_end(child));
    ui.advance(0.05);
    assert!(!ui.is_open(child));
    assert_eq!(ui.get::<aylis_controls::TabStrip>(strip).unwrap().tabs().len(), 1);
}
