//! Zoom: o do app inteiro (`Ui::set_zoom`, Ctrl + `+`/`-`/`0` e Ctrl + roda) e o de um form
//! (`zoom` do `Form`), que multiplica o do app.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example zoom --features d2d
//! AYLIS_ZOOM=1.5 cargo run ...        começa com 150%
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::light());
    ui.set_zoom_keys(true);
    if let Some(z) = std::env::var("AYLIS_ZOOM").ok().and_then(|v| v.parse().ok()) {
        ui.set_zoom(z);
    }
    let form = ui.add_root(Form::new("aylis — zoom", 520.0, 360.0));
    ui.control_mut(form).unwrap().set_padding(16.0);

    let info = ui.add(Label::new(""), form);
    ui.control_mut(info).unwrap().set_align(Align::Top).set_height(32.0);
    let show = move |ui: &mut Ui| {
        let text = format!("Zoom do app: {:.0}%  (Ctrl + roda, Ctrl + / - / 0)", ui.zoom() * 100.0);
        let _ = ui.set_prop(info, "text", text);
    };
    show(&mut ui);

    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 8.0, 0.0, 8.0]);
    for (text, delta) in [("Menor", -0.25f32), ("Normal", 0.0), ("Maior", 0.25)] {
        let b = ui.add(Button::new(text), bar);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(110.0).set_margins([0.0, 0.0, 8.0, 0.0]);
        ui.on_click(b, move |ui, _| {
            let z = if delta == 0.0 { 1.0 } else { ui.zoom() + delta };
            ui.set_zoom(z);
            show(ui);
        });
    }
    let edit = ui.add(Edit::new(""), form);
    ui.control_mut(edit).unwrap().set_align(Align::Top).set_height(36.0);
    ui.set_prop(edit, "placeholder", "Digite aqui: o campo, o cursor e a seleção acompanham o zoom")?;
    let check = ui.add(CheckBox::new("Uma caixa de marcar"), form);
    ui.control_mut(check).unwrap().set_align(Align::Top).set_height(32.0).set_margins([0.0, 8.0, 0.0, 0.0]);

    // Um segundo form com zoom próprio (200% do app).
    let other = ui.add_root(Form::new("aylis — form com zoom 2", 300.0, 160.0));
    ui.set_prop(other, "zoom", 2.0)?;
    ui.control_mut(other).unwrap().set_padding(12.0);
    let label = ui.add(Label::new("Este form tem zoom 2"), other);
    ui.control_mut(label).unwrap().set_align(Align::Client);
    let open = ui.add(Button::new("Abrir o form com zoom 2"), form);
    ui.control_mut(open).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 8.0, 0.0, 0.0]);
    ui.on_click(open, move |ui, _| {
        let _ = ui.show(other);
    });
    aylis_platform_winit::run(ui, form)
}
