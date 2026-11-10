//! Correção e sugestão de texto com o motor nativo (no Windows, o corretor e a previsão do
//! sistema): o sublinhado nas palavras erradas (clique direito: as correções), a palavra
//! sugerida enquanto digita (Tab aceita) e o cursor personalizado.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example spellcheck
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::light());
    let form = ui.add_root(Form::new("aylis — correção e sugestão", 640.0, 400.0));
    ui.control_mut(form).unwrap().set_padding(20.0);
    let label = |ui: &mut Ui, text: &str, y: f32| {
        let l = ui.add(Label::new(text), form);
        ui.control_mut(l).unwrap().set_rect(20.0, y, 600.0, 22.0);
    };
    label(&mut ui, "spellcheck: as palavras erradas sublinhadas (clique direito: correções)", 16.0);
    let check = ui.add(Edit::new(""), form);
    ui.control_mut(check).unwrap().set_rect(20.0, 40.0, 600.0, 34.0);
    ui.set_prop(check, "spellcheck", true)?;
    ui.set_prop(check, "text", "Eu gosto do meu cachoro e de programar em Rsut")?;

    label(&mut ui, "autocomplete: a palavra sugerida aparece apagada; Tab aceita", 96.0);
    let complete = ui.add(Edit::new(""), form);
    ui.control_mut(complete).unwrap().set_rect(20.0, 120.0, 600.0, 34.0);
    ui.set_prop(complete, "autocomplete", true)?;
    ui.set_prop(complete, "caret_color", "#e11d48")?;
    ui.set_prop(complete, "caret_width", 2.5)?;

    label(&mut ui, "Memo com os dois e o cursor em bloco, sem piscar", 176.0);
    let memo = ui.add(Memo::new(""), form);
    ui.control_mut(memo).unwrap().set_rect(20.0, 200.0, 600.0, 150.0);
    ui.set_prop(memo, "spellcheck", true)?;
    ui.set_prop(memo, "autocomplete", true)?;
    ui.set_prop(memo, "caret_style", "block")?;
    ui.set_prop(memo, "caret_blink", 0)?;
    ui.set_prop(memo, "text", "Primeira linha sem erros.\nSegunda linha com um eror aqui.")?;

    // O motor nativo entra quando a plataforma começa: a digitação de exemplo vem no
    // primeiro tique (um Timer de uma vez só).
    let once = ui.add(Timer::new(300), form);
    ui.on(once, "on_timer", move |ui, sender, _| {
        let _ = ui.set_prop(sender, "enabled", false);
        ui.set_focus(complete);
        ui.text_input(form, "Estou usando o compu");
    });
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
