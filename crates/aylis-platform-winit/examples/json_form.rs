//! Abre uma tela descrita em JSON e liga os eventos por nome — como um .fmx com os
//! métodos do form.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example json_form
//! cargo run -p aylis-platform-winit --example json_form -- caminho/tela.json --theme schema/aylis-dark.theme.json
//! ```

use std::path::PathBuf;

use aylis_core::{ComponentId, EventArgs, PropValue, Ui};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let layout_path = args
        .iter()
        .find(|a| a.ends_with(".json") && !a.ends_with(".theme.json"))
        .map(PathBuf::from)
        .unwrap_or_else(|| root_dir.join("crates/aylis-platform-winit/examples/forms/registration.json"));

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    if let Some(i) = args.iter().position(|a| a == "--theme") {
        let path = args.get(i + 1).ok_or("informe o arquivo depois de --theme")?;
        ui.set_theme(aylis_json::load_theme(&std::fs::read_to_string(path)?)?);
    }

    // Os "métodos do form": handlers registrados pelo nome usado no JSON.
    ui.register_handler("btn_novo_click", |ui, _, _| say(ui, "Novo cliente"));
    ui.register_handler("btn_salvar_click", |ui, _, _| {
        let name = text_of(ui, "edtNome");
        say(ui, &format!("Salvo: {name}"));
    });
    ui.register_handler("btn_excluir_click", |ui, _, _| say(ui, "Excluir clicado"));
    ui.register_handler("ok_click", |ui, _, _| say(ui, "OK!"));
    ui.register_handler("menu_click", |ui, sender, _| {
        let name = ui.name(sender).unwrap_or_default().trim_start_matches("mnu").to_string();
        say(ui, &format!("Menu: {name}"));
    });
    ui.register_handler("pesquisa_change", |ui, _, args| {
        if let EventArgs::Value(PropValue::String(text)) = args {
            say(ui, &format!("Pesquisando: {text}"));
        }
    });
    ui.register_handler("ativo_change", |ui, _, args| {
        let on = matches!(args, EventArgs::Value(PropValue::Bool(true)));
        say(ui, if on { "Cliente ativo" } else { "Cliente inativo" });
    });

    let json = std::fs::read_to_string(&layout_path)
        .map_err(|e| format!("não consegui ler {}: {e}", layout_path.display()))?;
    let form: ComponentId = aylis_json::load(&mut ui, &json, None)?;

    // `--png arquivo.png`: renderiza num arquivo (CPU) em vez de abrir a janela.
    if let Some(i) = args.iter().position(|a| a == "--png") {
        let path = args.get(i + 1).ok_or("informe o arquivo depois de --png")?;
        let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
        let background = ui.theme().color("background");
        let png = aylis_skia::SkiaRenderer::new()
            .render_png(w as u32, h as u32, 1.0, background, |canvas| ui.paint(form, canvas))
            .ok_or("falha ao gerar o PNG")?;
        std::fs::write(path, png)?;
        return Ok(());
    }

    aylis_platform_winit::run(ui, form)
}

fn say(ui: &mut Ui, text: &str) {
    if let Some(label) = ui.find("lblStatus") {
        let _ = ui.set_prop(label, "text", text);
    }
}

fn text_of(ui: &Ui, name: &str) -> String {
    ui.find(name)
        .and_then(|id| ui.get_prop(id, "text").ok())
        .and_then(|v| v.to_str().ok().map(str::to_string))
        .unwrap_or_default()
}
