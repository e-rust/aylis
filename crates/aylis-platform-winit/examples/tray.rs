//! TrayIcon: ícone na bandeja com indicador (o número de mensagens) e menu próprio escuro.
//! `AYLIS_AUTO_CLOSE=1` fecha em 3 s (teste).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example tray --features cpu,tray
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — bandeja", 420.0, 200.0));
    let status = ui.add(Label::new("Clique direito no ícone da bandeja (perto do relógio)."), form);
    ui.control_mut(status).unwrap().set_align(Align::Client).set_margins([16.0, 16.0, 16.0, 16.0]);
    let mut tray = TrayIcon::default();
    tray.hint = "aylis — 3 mensagens".into();
    tray.badge = "3".into();
    tray.menu_items = "Abrir;Não perturbe;-;Sair".into();
    tray.menu_theme = "dark".into();
    let tray = ui.add(tray, form);
    ui.on(tray, "on_menu", move |ui, _, args| {
        if let EventArgs::Text(item) = args {
            if item == "Sair" {
                ui.request_close();
            }
            let _ = ui.set_prop(status, "text", format!("Menu: {item}"));
        }
    });
    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(3000), form);
        ui.on(timer, "on_timer", move |ui, _, _| {
            println!("bandeja: {} ícone(s)", ui.trays().len());
            ui.request_close();
        });
    }
    aylis_platform_winit::run(ui, form)
}
