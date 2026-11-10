//! Formas geométricas: passe o mouse e clique; só a figura responde (o canto de um círculo
//! não é o círculo).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example shapes --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — formas", 820.0, 420.0));
    let info = ui.add(Label::new("Clique numa forma"), form);
    ui.control_mut(info).unwrap().set_position(24.0, 16.0).set_size(600.0, 28.0);
    let shapes = [
        ("Rectangle", "#0EA5E9"),
        ("Ellipse", "#22C55E"),
        ("Circle", "#EAB308"),
        ("Polygon", "#A855F7"),
        ("Star", "#F97316"),
        ("Pie", "#EF4444"),
        ("Arc", "#14B8A6"),
        ("Line", "#64748B"),
        ("Path", "#EC4899"),
    ];
    for (i, (class, color)) in shapes.iter().enumerate() {
        let id = ui.create(class, Some(form))?;
        let (col, row) = (i % 5, i / 5);
        ui.control_mut(id).unwrap().set_rect(24.0 + col as f32 * 156.0, 64.0 + row as f32 * 170.0, 130.0, 130.0);
        let c = Color::from_hex(color).unwrap_or(Color::BLACK);
        let prop = if matches!(*class, "Arc" | "Line") { "stroke" } else { "fill" };
        ui.set_prop(id, prop, c)?;
        // Hover: a figura clareia (o canto vazio não dispara).
        ui.add(PropertyAnimation::color(prop, c.with_opacity(0.6)).with_trigger(Trigger::Hover).with_duration(120.0), id);
        let name = class.to_string();
        ui.on_click(id, move |ui, _| {
            let _ = ui.set_prop(info, "text", format!("Clicou: {name}"));
        });
    }
    aylis_platform_winit::run(ui, form)
}
