//! Annotation: a anotação de texto sobre um print, como no editor do NoOBS. Cada clique na
//! imagem cria um `Memo` só texto (`plain`) que cresce para a direita e para baixo enquanto se
//! digita (`auto_size`; Shift+Enter quebra a linha), sem fundo, borda nem barras: o print
//! aparece em volta das letras, a seleção é translúcida e o cursor tem contorno.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example annotation --features cpu
//! ```
//!
//! No Direct2D: `--features cpu,d2d` com `AYLIS_RENDERER=d2d`.

use aylis_controls::*;
use aylis_core::*;

/// Um "print" colorido: um lado escuro e um claro, para ver o cursor nos dois.
fn picture(width: u32, height: u32) -> Image {
    let mut px = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let (t, v) = (x as f32 / width as f32, y as f32 / height as f32);
            let (r, g, b) = if t < 0.5 { (20.0 + 140.0 * t, 25.0 + 30.0 * v, 80.0 + 50.0 * v) } else { (255.0 - 120.0 * (t - 0.5), 230.0 - 20.0 * v, 110.0 + 110.0 * v) };
            px.extend([r as u8, g as u8, b as u8, 255]);
        }
    }
    Image::rgba(width, height, px)
}

/// Um Memo só texto em `at` (no form), que cresce com o que se digita, já com o foco.
fn annotate(ui: &mut Ui, form: ComponentId, at: PointF, text: &str) -> ComponentId {
    let memo = ui.add(Memo::new(text), form);
    ui.control_mut(memo).unwrap().set_position(at.x, at.y).set_auto_size(true);
    ui.set_prop(memo, "plain", true).unwrap();
    ui.set_prop(memo, "font_size", 18.0).unwrap();
    ui.set_focus(memo);
    memo
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Anotação sobre o print", 720.0, 420.0));
    let _ = ui.set_prop(form, "padding", 0i64);
    let mut view = ImageView::default();
    view.fit = ImageFit::Stretch;
    view.set_picture(Some(picture(720, 420)));
    let image = ui.add(view, form);
    ui.control_mut(image).unwrap().set_align(Align::Client);
    annotate(&mut ui, form, PointF::new(40.0, 40.0), "Clique no print para anotar");
    ui.on_click(image, move |ui, _| {
        if let Some(p) = ui.click_pointer() {
            annotate(ui, form, p.form_position, "");
        }
    });
    aylis_platform_winit::run(ui, form)
}
