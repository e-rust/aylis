//! O Reveal Highlight (a luz do Fluent que segue o mouse): uma grade de botões no estilo
//! `reveal` (borda e fundo) e um painel com a borda acesa (`Panel.reveal`), no tema escuro.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example reveal --features d2d
//! ```

use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::dark());
    let form = ui.create("Form", None)?;
    ui.set_prop(form, "caption", "Aylis — Reveal Highlight")?;
    ui.set_prop(form, "width", 420.0)?;
    ui.set_prop(form, "height", 460.0)?;
    let panel = ui.create("Panel", Some(form))?;
    ui.set_prop(panel, "align", "client")?;
    ui.set_prop(panel, "margins", "16")?;
    ui.set_prop(panel, "padding", "12")?;
    ui.set_prop(panel, "style", "reveal")?;
    let grid = ui.create("GridLayout", Some(panel))?;
    ui.set_prop(grid, "align", "client")?;
    ui.set_prop(grid, "columns", 4)?;
    ui.set_prop(grid, "gap", 2.0)?;
    ui.set_prop(grid, "row_height", 64.0)?;
    for text in ["%", "CE", "C", "⌫", "7", "8", "9", "×", "4", "5", "6", "−", "1", "2", "3", "+", "±", "0", ",", "="] {
        let button = ui.create("Button", Some(grid))?;
        ui.set_prop(button, "text", text)?;
        ui.set_prop(button, "style", "reveal")?;
    }
    aylis_platform_winit::run(ui, form)
}
