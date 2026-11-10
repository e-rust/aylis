//! Barra de título própria: `border_style = custom` no form e uma `TitleBar` com um botão
//! a mais. Arraste pela barra, duplo clique maximiza, as bordas redimensionam.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example titlebar --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::dark());
    let form = ui.add_root(Form::new("Meu aplicativo", 720.0, 440.0));
    let _ = ui.set_prop(form, "border_style", PropValue::Enum("custom".into()));
    let bar = ui.add(TitleBar::default(), form);
    let menu = ui.add(Button::new("☰"), bar);
    ui.control_mut(menu).unwrap().set_align(Align::Left).set_width(44.0).set_margins([4.0, 4.0, 0.0, 4.0]);
    let _ = ui.set_prop(menu, "style", "flat");
    let body = ui.add(Label::new("Conteúdo do app. A barra de cima é do aylis."), form);
    ui.control_mut(body).unwrap().set_align(Align::Client).set_margins([16.0, 16.0, 16.0, 16.0]);
    aylis_platform_winit::run(ui, form)
}
