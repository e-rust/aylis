//! Texto com efeitos (`AnimatedText`), como num programa de apresentação: entradas por
//! letra, palavra e linha, encadeadas (`after`) ou juntas, e efeitos contínuos.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example text_effects --features d2d
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::dark());
    let form = ui.add_root(Form::new("aylis — texto com efeitos", 900.0, 640.0));
    ui.control_mut(form).unwrap().set_padding(24.0);

    let slides: [(&str, &str, f32); 5] = [
        ("Bem-vindo ao aylis", "fade by=letter duration=500 stagger=45; slide dy=40 by=letter duration=700 stagger=45 easing=back", 44.0),
        ("Palavra por palavra, de baixo para cima", "fade by=word duration=400 stagger=180; scale from=0.3 by=word duration=600 stagger=180 easing=elastic", 26.0),
        ("Máquina de escrever, com cursor…", "typewriter speed=55 cursor", 24.0),
        ("Depois de entrar, fica ondulando", "fade by=letter duration=300 stagger=25 order=center; wave amplitude=6 period=1400 after", 26.0),
        ("Cor e espaçamento chegando juntos", "color from=#22c55e duration=1500; tracking from=12 duration=1200 easing=out; rotate from=-25 by=word duration=800 stagger=150", 26.0),
    ];
    let mut texts = Vec::new();
    for (text, effects, size) in slides {
        let t = ui.add(AnimatedText::new(text, effects), form);
        ui.control_mut(t).unwrap().set_align(Align::Top).set_height(size * 2.4).set_margins([0.0, 0.0, 0.0, 12.0]);
        ui.set_prop(t, "font_size", size)?;
        texts.push(t);
    }
    let again = ui.add(Button::new("Repetir"), form);
    ui.control_mut(again).unwrap().set_align(Align::Bottom).set_height(40.0);
    ui.on_click(again, move |ui, _| {
        for &t in &texts {
            AnimatedText::play(ui, t);
        }
    });
    aylis_platform_winit::run(ui, form)
}
