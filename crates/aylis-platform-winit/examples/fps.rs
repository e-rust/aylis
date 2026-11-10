//! Limite de quadros e medida do FPS: um quadrado girando sem parar e o `FpsMeter`.
//! `AYLIS_MAX_FPS=30` limita; `AYLIS_AUTO_CLOSE=1` imprime o FPS a cada segundo e fecha em 4 s.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example fps --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    if let Some(fps) = std::env::var("AYLIS_MAX_FPS").ok().and_then(|v| v.parse().ok()) {
        ui.set_max_fps(fps);
    }
    let form = ui.add_root(Form::new("aylis — FPS", 400.0, 300.0));
    let meter = ui.add(FpsMeter::default(), form);
    ui.control_mut(meter).unwrap().set_position(12.0, 12.0).set_size(120.0, 24.0);
    let square = ui.add(Rectangle::default(), form);
    ui.control_mut(square).unwrap().set_position(150.0, 100.0).set_size(100.0, 100.0);
    let spin = AnimationOptions { duration_ms: 2000.0, easing: Easing::Linear, looping: true, ..Default::default() };
    ui.animate(square, "rotation", 359.0, spin)?;
    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(Timer::new(1000), form);
        let elapsed = std::cell::Cell::new(0);
        ui.on(timer, "on_timer", move |ui, _, _| {
            elapsed.set(elapsed.get() + 1);
            let seconds = elapsed.get();
            println!("{seconds} s: {:.0} fps", ui.fps());
            if seconds >= 4 {
                ui.request_close();
            }
        });
    }
    aylis_platform_winit::run(ui, form)
}
