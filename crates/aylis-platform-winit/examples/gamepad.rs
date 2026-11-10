//! Gamepad e Joystick: mostra o controle de jogo conectado (nome, botões apertados, os
//! analógicos e os gatilhos), vibra no botão "Vibrar" e tem a alavanca da tela ao lado.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example gamepad --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Gamepad", 560.0, 420.0));
    let _ = ui.set_prop(form, "padding", 16);

    let line = |ui: &mut Ui, text: &str| {
        let id = ui.add(Label::new(text), form);
        ui.control_mut(id).unwrap().set_align(Align::Top).set_height(28.0);
        id
    };
    let status = line(&mut ui, "Nenhum controle: ligue um (USB ou Bluetooth).");
    let buttons = line(&mut ui, "Botões: -");
    let last = line(&mut ui, "Último evento: -");
    let left = line(&mut ui, "Analógico esquerdo: 0.00  0.00");
    let right = line(&mut ui, "Analógico direito: 0.00  0.00");
    let triggers = line(&mut ui, "Gatilhos: 0.00  0.00");
    let rumble = ui.add(Button::new("Vibrar"), form);
    ui.control_mut(rumble).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 8.0, 380.0, 8.0]);
    let screen = line(&mut ui, "Alavanca da tela: 0.00  0.00");
    let joystick = ui.add(Joystick::default(), form);
    ui.control_mut(joystick).unwrap().set_align(Align::Left).set_width(140.0);

    let pad = ui.add(Gamepad::default(), form);
    let show = move |ui: &mut Ui| {
        let g = ui.get::<Gamepad>(pad).unwrap();
        let text = if g.connected() {
            format!("Conectado: {} ({} controle(s))", g.device_name(), g.count())
        } else {
            "Nenhum controle: ligue um (USB ou Bluetooth).".into()
        };
        let down: Vec<&str> = aylis_core::platform::GAMEPAD_BUTTONS.iter().copied().filter(|b| g.pressed(b)).collect();
        let down = if down.is_empty() { "-".into() } else { down.join(" ") };
        let (l, r, t) = (
            format!("Analógico esquerdo: {:.2}  {:.2}", g.left_x(), g.left_y()),
            format!("Analógico direito: {:.2}  {:.2}", g.right_x(), g.right_y()),
            format!("Gatilhos: {:.2}  {:.2}", g.left_trigger(), g.right_trigger()),
        );
        let _ = ui.set_prop(status, "text", text);
        let _ = ui.set_prop(buttons, "text", format!("Botões: {down}"));
        let _ = ui.set_prop(left, "text", l);
        let _ = ui.set_prop(right, "text", r);
        let _ = ui.set_prop(triggers, "text", t);
    };
    for event in ["on_connect", "on_disconnect", "on_change", "on_button_down", "on_button_up"] {
        ui.on(pad, event, move |ui, _, args| {
            if event.starts_with("on_button") || !event.ends_with("change") {
                let name = args.text().map(|b| format!(" {b}")).unwrap_or_default();
                let _ = ui.set_prop(last, "text", format!("Último evento: {event}{name}"));
            }
            show(ui);
        });
    }
    ui.on_click(rumble, move |ui, _| {
        let ok = ui.invoke(pad, "rumble", &[PropValue::Float(0.8), PropValue::from(400i64)]).ok() == Some(PropValue::Bool(true));
        let _ = ui.set_prop(last, "text", if ok { "Último evento: vibrou" } else { "Último evento: este controle não vibra (ou nenhum conectado)" });
    });
    ui.on(joystick, "on_change", move |ui, sender, _| {
        let j = ui.get::<Joystick>(sender).unwrap();
        let text = format!("Alavanca da tela: {:.2}  {:.2}", j.x(), j.y());
        let _ = ui.set_prop(screen, "text", text);
    });
    aylis_platform_winit::run(ui, form)
}
