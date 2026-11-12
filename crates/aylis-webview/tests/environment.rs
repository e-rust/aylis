//! O ambiente do WebView (o relatório do Tabernáculo, W07): a cor de fundo, a pasta de dados e
//! os argumentos chegam à vista nativa, sem a depuração remota (abriria uma porta).

use aylis_controls::Form;
use aylis_core::{Align, Color, NativeView, Ui};
use aylis_webview::WebView;

#[test]
fn the_environment_reaches_the_native_view_without_remote_debugging() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("f", 400.0, 300.0));
    ui.show(form).unwrap();
    let web = ui.add(WebView::new("https://aylis.dev"), form);
    ui.control_mut(web).unwrap().set_align(Align::Client);
    ui.set_prop(web, "background_color", Color::rgb(0x12, 0x12, 0x12)).unwrap();
    ui.set_prop(web, "data_dir", "C:/dados/app").unwrap();
    ui.set_prop(web, "browser_args", "--autoplay-policy=no-user-gesture-required --remote-debugging-port=9222").unwrap();
    ui.layout();
    let slot = ui.native_views(form).into_iter().find(|s| s.id == web).unwrap();
    let NativeView::Web { background, data_dir, browser_args, .. } = slot.view;
    assert_eq!(background, Some(Color::rgb(0x12, 0x12, 0x12)));
    assert_eq!(data_dir, "C:/dados/app");
    assert_eq!(browser_args, "--autoplay-policy=no-user-gesture-required");
}
