//! Notification: o toast do Windows (na central de notificações). `AYLIS_AUTO_CLOSE=1`
//! notifica, imprime o resultado e fecha.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example notification --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Exemplo aylis", 360.0, 160.0));
    let mut n = Notification::default();
    n.title = "Pedido aprovado".into();
    n.text = "O pedido 1234 foi aprovado e já está a caminho.".into();
    n.app_id = "aylis.exemplo".into();
    let n = ui.add(n, form);
    let button = ui.add(Button::new("Notificar"), form);
    ui.control_mut(button).unwrap().set_align(Align::Center).set_size(160.0, 40.0);
    ui.on_click(button, move |ui, _| {
        if let Err(e) = Notification::show(ui, n) {
            eprintln!("{e}");
        }
    });
    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(800), form);
        ui.on(timer, "on_timer", move |ui, _, _| {
            println!("notificação: {:?}", Notification::show(ui, n));
            ui.request_close();
        });
    }
    aylis_platform_winit::run(ui, form)
}
