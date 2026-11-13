//! UrlScheme: o prefixo `aylisexemplo://` abre este exemplo (no Windows, registrado ao
//! abrir). Com ele aberto, chame `aylisexemplo://pedido/42` no Executar (Win+R) ou no
//! navegador: o endereço aparece na tela, sem abrir outra janela. `AYLIS_LINK_LOG=arquivo`
//! acrescenta cada link no arquivo.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example url_scheme --features cpu
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Prefixo próprio", 460.0, 160.0));
    let label = ui.add(Label::new("Chame aylisexemplo://pedido/42 de outro programa."), form);
    ui.control_mut(label).unwrap().set_align(Align::Client);
    let scheme = ui.add(UrlScheme::default(), form);
    ui.set_prop(scheme, "scheme", "aylisexemplo")?;
    ui.set_prop(scheme, "description", "Exemplo do Aylis")?;
    ui.set_prop(scheme, "active", true)?;
    ui.on(scheme, "on_link", move |ui, _, args| {
        let EventArgs::Text(url) = args else { return };
        let _ = ui.set_prop(label, "text", format!("Chegou: {url}"));
        if let Ok(path) = std::env::var("AYLIS_LINK_LOG") {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
                let _ = writeln!(f, "{url}");
            }
        }
    });
    ui.on(scheme, "on_error", |_, _, args| eprintln!("UrlScheme: {args:?}"));
    aylis_platform_winit::run(ui, form)
}
