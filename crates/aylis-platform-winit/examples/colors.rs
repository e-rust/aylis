//! Cores e fonte próprias em qualquer componente: à esquerda, como o tema desenha; à
//! direita, os mesmos com `fill`, `border_color`, `text_color`, `accent_color`, as cores de
//! sentido e a fonte (`font_family`, `font_size`, `font_weight`, `font_italic`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example colors --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — cores próprias", 900.0, 760.0));
    let classes: [(&str, &[(&str, PropValue)]); 12] = [
        ("Button", &[("text", PropValue::String("Salvar".into()))]),
        ("Edit", &[("text", "Texto digitado".into())]),
        ("CheckBox", &[("text", "Lembrar".into()), ("checked", true.into())]),
        ("Switch", &[("checked", true.into())]),
        ("ProgressBar", &[("value", 65.0.into())]),
        ("TrackBar", &[("value", 40.0.into())]),
        ("ComboBox", &[("items", "Um;Dois".into()), ("item_index", PropValue::Int(0))]),
        ("Chip", &[("text", "Filtro".into())]),
        ("Badge", &[("text", "12".into())]),
        ("Banner", &[("title", "Aviso".into()), ("text", "Algo aconteceu".into())]),
        ("Rating", &[("value", 3.5.into())]),
        ("Gauge", &[("value", 72.0.into()), ("gradient", true.into())]),
    ];
    for (i, (class, props)) in classes.iter().enumerate() {
        for side in 0..2 {
            let id = ui.create(class, Some(form))?;
            let h: f32 = if *class == "Banner" || *class == "Gauge" { 52.0 } else { 40.0 };
            let y = 20.0 + i as f32 * 58.0;
            ui.control_mut(id).unwrap().set_rect(30.0 + side as f32 * 440.0, y, 380.0, h);
            for (k, v) in props.iter() {
                let _ = ui.set_prop(id, k, v.clone());
            }
            if side == 1 {
                for (k, c) in [("fill", "#1E293B"), ("border_color", "#F59E0B"), ("text_color", "#F8FAFC"), ("accent_color", "#10B981"), ("success_color", "#22D3EE"), ("warning_color", "#A78BFA"), ("danger_color", "#F472B6")] {
                    let _ = ui.set_prop(id, k, Color::from_hex(c).unwrap());
                }
                let _ = ui.set_prop(id, "font_family", "Georgia");
                let _ = ui.set_prop(id, "font_italic", true);
            }
        }
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
