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
