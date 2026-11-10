//! A câmera ao vivo (`CameraView`) e o nível do microfone (`Microphone`), com foto em PNG.
//! `AYLIS_AUTO_CLOSE=1`: tira uma foto depois de 3 s (num arquivo temporário, apagado em
//! seguida), imprime o tamanho e fecha.
//!
//! ```text
//! cargo run -p aylis-media --example camera
//! ```

use aylis_controls::{Button, Form, Label, Layout};
use aylis_core::*;
use aylis_media::{CameraView, Microphone, cameras, register};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    ui.set_theme(Theme::dark());
    let form = ui.add_root(Form::new("aylis — câmera e microfone", 680.0, 560.0));
    ui.control_mut(form).unwrap().set_padding(16.0);

    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let photo = ui.add(Button::new("Foto"), bar);
    ui.control_mut(photo).unwrap().set_align(Align::Left).set_width(100.0);
    let status = ui.add(Label::new(""), bar);
    ui.control_mut(status).unwrap().set_align(Align::Client).set_margins([12.0, 0.0, 0.0, 0.0]);

    let mic = ui.add(Microphone::default(), form);
    ui.control_mut(mic).unwrap().set_align(Align::Bottom).set_height(20.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    ui.set_prop(mic, "active", true)?;

    let names: Vec<String> = cameras().into_iter().map(|c| c.name).collect();
    ui.set_prop(status, "text", if names.is_empty() { "nenhuma câmera".to_string() } else { names.join(", ") })?;

    let view = ui.add(CameraView::default(), form);
    ui.control_mut(view).unwrap().set_align(Align::Client);
    ui.set_prop(view, "radius", 16.0f32)?;
    ui.set_prop(view, "mirror", true)?;
    ui.set_prop(view, "active", true)?;
    ui.on(view, "on_error", move |ui, _, args| {
        if let EventArgs::Text(e) = args {
            let _ = ui.set_prop(status, "text", format!("⚠ {e}"));
        }
    });

    let take = move |ui: &mut Ui| -> Result<std::path::PathBuf, String> {
        let path = std::env::temp_dir().join(format!("aylis-foto-{}.png", std::process::id()));
        CameraView::photo(ui, view, &path)?;
        Ok(path)
    };
    ui.on(photo, "on_click", move |ui, _, _| {
        let text = match take(ui) {
            Ok(p) => format!("foto: {}", p.display()),
            Err(e) => format!("⚠ {e}"),
        };
        let _ = ui.set_prop(status, "text", text);
    });

    if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
        let timer = ui.add(aylis_controls::Timer::new(3000), form);
        ui.on(timer, "on_timer", move |ui, _, _| {
            match take(ui) {
                Ok(p) => {
                    let size = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    println!("foto: {size} bytes; nível do microfone: {:.3}", Microphone::level(ui, mic));
                    let _ = std::fs::remove_file(&p);
                }
                Err(e) => println!("sem foto: {e}"),
            }
            ui.request_close();
        });
    }
    aylis_platform_winit::run(ui, form)
}
