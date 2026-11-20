//! Os recursos da janela do sistema no form: chamar a atenção (`request_attention`), manter a
//! tela acesa (`keep_awake`), ficar sempre por baixo (`bottommost`), prender o mouse
//! (`confine_cursor`), a transparência da janela inteira (`window_opacity`) e a tela cheia
//! exclusiva (`full_screen_mode`, com o modo de vídeo).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example window_features -- [opções]
//!   --attention [critical]   chama a atenção 1 s depois de abrir (troque de janela antes)
//!   --awake system|display   mantém o computador (e a tela) acordado
//!   --bottom                 sempre por baixo das outras
//!   --top                    sempre por cima
//!   --confine window|monitor prende o mouse
//!   --opacity 0.6            a janela inteira translúcida
//!   --exclusive 1920x1080@60 tela cheia exclusiva com esse modo de vídeo
//!   --caption texto          o título (para achar a janela)
//!   --close N                fecha depois de N ms
//!   --confirm                abre uma pergunta modal antes da janela
//!   --splash                 abre antes uma tela de abertura, que dá lugar à janela em 1 s
//! ```

use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let value = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let flag = |name: &str| args.iter().any(|a| a == name);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None)?;
    ui.set_prop(form, "name", "frmRecursos")?;
    ui.set_prop(form, "caption", value("--caption").unwrap_or_else(|| "Aylis — recursos da janela".into()))?;
    ui.set_prop(form, "width", 640.0)?;
    ui.set_prop(form, "height", 400.0)?;
    let mut text = Vec::new();
    if let Some(mode) = value("--awake") {
        ui.set_prop(form, "keep_awake", mode.as_str())?;
        text.push(format!("keep_awake = {mode}"));
    }
    if flag("--bottom") {
        ui.set_prop(form, "bottommost", true)?;
        text.push("bottommost".into());
    }
    if flag("--top") {
        ui.set_prop(form, "topmost", true)?;
        text.push("topmost".into());
    }
    if let Some(mode) = value("--confine") {
        ui.set_prop(form, "confine_cursor", mode.as_str())?;
        text.push(format!("confine_cursor = {mode}"));
    }
    if let Some(opacity) = value("--opacity").and_then(|v| v.parse::<f64>().ok()) {
        ui.set_prop(form, "window_opacity", opacity)?;
        text.push(format!("window_opacity = {opacity}"));
    }
    if let Some(mode) = value("--exclusive") {
        // 1920x1080@60
        let (size, hz) = mode.split_once('@').unwrap_or((mode.as_str(), "0"));
        let (w, h) = size.split_once('x').unwrap_or(("0", "0"));
        ui.set_prop(form, "full_screen_mode", "exclusive")?;
        ui.set_prop(form, "video_width", w.parse::<i64>().unwrap_or(0))?;
        ui.set_prop(form, "video_height", h.parse::<i64>().unwrap_or(0))?;
        ui.set_prop(form, "video_refresh", hz.parse::<f64>().unwrap_or(0.0))?;
        ui.set_prop(form, "window_state", "full_screen")?;
        text.push(format!("tela cheia exclusiva {mode}"));
    }
    if let Some(kind) = args.iter().position(|a| a == "--attention") {
        let critical = args.get(kind + 1).is_some_and(|a| a == "critical");
        ui.set_timeout(1000, move |ui| {
            let _ = ui.invoke(form, "request_attention", &[PropValue::Bool(critical)]);
            println!("request_attention({critical})");
        });
        text.push("request_attention em 1 s".into());
    }
    let label = ui.create("Label", Some(form))?;
    ui.set_prop(label, "align", "client")?;
    ui.set_prop(label, "h_align", "center")?;
    ui.set_prop(label, "font_size", 20.0)?;
    ui.set_prop(label, "text", if text.is_empty() { "sem opções".to_string() } else { text.join("\n") })?;
    // `--confirm`: uma pergunta modal aberta antes do `run` (como a de confiar no projeto, que a
    // IDE faz ao abrir com um projeto): ela tem de ficar presa por cima da janela.
    if flag("--confirm") {
        aylis_controls::confirm(&mut ui, "Confirmar?", "A pergunta aberta antes da janela.", "Sim", false, |_, yes| println!("respondeu: {yes}"));
    }
    // `--splash`: como a IDE, abre antes uma tela de abertura (o principal enquanto aparece);
    // a pergunta do `--confirm` nasce durante ela, e 1 s depois a abertura dá lugar à janela.
    if flag("--splash") {
        let splash = ui.create("Form", None)?;
        ui.set_prop(splash, "caption", "Abertura")?;
        ui.set_prop(splash, "width", 400.0)?;
        ui.set_prop(splash, "height", 240.0)?;
        ui.show(splash)?;
        if flag("--confirm") {
            aylis_controls::confirm(&mut ui, "Confirmar?", "A pergunta aberta durante a abertura.", "Sim", false, |_, yes| println!("respondeu: {yes}"));
        }
        ui.set_timeout(1000, move |ui| {
            ui.set_main_form(form);
            ui.close(splash);
            let _ = ui.show(form);
        });
        if let Some(ms) = value("--close").and_then(|v| v.parse::<u64>().ok()) {
            ui.set_timeout(ms, |ui| ui.request_close());
        }
        aylis_platform_winit::run(ui, splash)?;
        return Ok(());
    }
    if let Some(ms) = value("--close").and_then(|v| v.parse::<u64>().ok()) {
        ui.set_timeout(ms, |ui| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
