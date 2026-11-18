//! Forms transparentes: um redondo (`window_shape = ellipse`), arrastado pelo fundo; uma estrela
//! (`window_shape = path`, o contorno em `shape_path`) translúcida e com o desfoque; e um com um
//! raio por canto (`rounded`, `shape_radii`), semitransparente (`fill` com alfa) e com o vidro
//! fosco por trás (`WindowSettings.blur`). A borda é suave (antialiasing) e os cliques fora da
//! forma vão para o que está atrás.
//!
//! ```text
//! set AYLIS_RENDERER=d2d
//! cargo run -p aylis-platform-winit --example shaped_forms --features d2d
//! ```
//!
//! No Windows, só com o Direct2D (com o Skia a janela fica opaca e retangular); `--no-blur`: só
//! a semitransparência, sem o desfoque.

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let blur = !args.iter().any(|a| a == "--no-blur");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);

    // O vidro: semitransparente, cantos de 24, o que está atrás desfocado.
    let glass = ui.add_root(Form::new("Aylis — vidro fosco", 420.0, 300.0));
    ui.set_prop(glass, "name", "frmGlass")?;
    ui.set_prop(glass, "window_shape", "rounded")?;
    ui.set_prop(glass, "shape_radii", "32,32,32,4")?;
    ui.set_prop(glass, "position", "designed")?;
    ui.set_prop(glass, "left", 420.0)?;
    ui.set_prop(glass, "top", 120.0)?;
    ui.set_prop(glass, "fill", Color::rgba(255, 255, 255, 70))?;
    if blur {
        let settings = ui.create("WindowSettings", Some(glass))?;
        ui.set_prop(settings, "blur", true)?;
    }
    let bar = ui.create("TitleBar", Some(glass))?;
    ui.set_prop(bar, "caption", "Vidro fosco")?;
    ui.set_prop(bar, "fill", Color::TRANSPARENT)?;
    let text = ui.create("Label", Some(glass))?;
    ui.set_prop(text, "align", "client")?;
    ui.set_prop(text, "margins", "24")?;
    ui.set_prop(text, "wrap", true)?;
    ui.set_prop(text, "font_size", 18.0)?;
    ui.set_prop(text, "text", "Fundo branco com 27% de opacidade e o que está atrás desfocado. Os cantos têm a borda suave.")?;

    // O redondo: um círculo de 220, sem moldura, arrastado pelo fundo.
    let round = ui.create("Form", None)?;
    ui.set_prop(round, "name", "frmRound")?;
    ui.set_prop(round, "caption", "Aylis — form redondo")?;
    ui.set_prop(round, "width", 220.0)?;
    ui.set_prop(round, "height", 220.0)?;
    ui.set_prop(round, "window_shape", "ellipse")?;
    ui.set_prop(round, "fill", Color::rgba(30, 110, 230, 230))?;
    ui.set_prop(round, "topmost", true)?;
    ui.set_prop(round, "position", "designed")?;
    ui.set_prop(round, "left", 120.0)?;
    ui.set_prop(round, "top", 120.0)?;
    let drag = ui.create("TitleBar", Some(round))?;
    ui.set_prop(drag, "align", "client")?;
    ui.set_prop(drag, "caption", "")?;
    ui.set_prop(drag, "fill", Color::TRANSPARENT)?;
    for button in ["show_minimize", "show_maximize", "show_close"] {
        ui.set_prop(drag, button, false)?;
    }
    let close = ui.create("Button", Some(drag))?;
    ui.set_prop(close, "text", "Fechar")?;
    ui.set_prop(close, "left", 70.0)?;
    ui.set_prop(close, "top", 150.0)?;
    ui.set_prop(close, "width", 80.0)?;
    ui.on(close, "on_click", |ui, _, _| {
        ui.request_close();
    });
    let hello = ui.create("Label", Some(drag))?;
    ui.set_prop(hello, "text", "Aylis")?;
    ui.set_prop(hello, "left", 20.0)?;
    ui.set_prop(hello, "top", 70.0)?;
    ui.set_prop(hello, "width", 180.0)?;
    ui.set_prop(hello, "height", 60.0)?;
    ui.set_prop(hello, "font_size", 36.0)?;
    ui.set_prop(hello, "h_align", "center")?;
    ui.set_prop(hello, "text_color", Color::WHITE)?;
    ui.show(round)?;

    // A estrela: um contorno qualquer (`window_shape = path`), translúcida e com o desfoque.
    let star = ui.create("Form", None)?;
    ui.set_prop(star, "name", "frmStar")?;
    ui.set_prop(star, "caption", "Aylis — estrela")?;
    ui.set_prop(star, "width", 240.0)?;
    ui.set_prop(star, "height", 230.0)?;
    ui.set_prop(star, "window_shape", "path")?;
    ui.set_prop(star, "shape_path", "M 50 0 L 61 35 L 98 35 L 68 57 L 79 91 L 50 70 L 21 91 L 32 57 L 2 35 L 39 35 Z")?;
    ui.set_prop(star, "fill", Color::rgba(255, 190, 0, 120))?;
    ui.set_prop(star, "position", "designed")?;
    ui.set_prop(star, "left", 160.0)?;
    ui.set_prop(star, "top", 330.0)?;
    if blur {
        let settings = ui.create("WindowSettings", Some(star))?;
        ui.set_prop(settings, "blur", true)?;
    }
    let grip = ui.create("TitleBar", Some(star))?;
    ui.set_prop(grip, "align", "client")?;
    ui.set_prop(grip, "caption", "")?;
    ui.set_prop(grip, "fill", Color::TRANSPARENT)?;
    for button in ["show_minimize", "show_maximize", "show_close"] {
        ui.set_prop(grip, button, false)?;
    }
    ui.show(star)?;
    aylis_platform_winit::run(ui, glass)
}
