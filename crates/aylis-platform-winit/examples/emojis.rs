//! EmojiPicker: o seletor de emoji (busca em português, categorias, recentes).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example emojis --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — emojis", 420.0, 520.0));
    ui.control_mut(form).unwrap().set_padding(12.0);
    let chosen = ui.add(Label::new("Escolha um emoji"), form);
    ui.control_mut(chosen).unwrap().set_align(Align::Top).set_height(36.0);
    let mut picker = EmojiPicker::default();
    picker.recent = "👍 😂 ❤️ 🎉".into();
    let picker = ui.add(picker, form);
    ui.control_mut(picker).unwrap().set_align(Align::Client);
    ui.on(picker, "on_select", move |ui, _, args| {
        if let EventArgs::Text(e) = args {
            let _ = ui.set_prop(chosen, "text", format!("Escolhido: {e}"));
        }
    });
    aylis_platform_winit::run(ui, form)
}
