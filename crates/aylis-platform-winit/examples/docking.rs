//! Forms que entram e saem de outro, como as abas do navegador: a janela principal tem uma
//! faixa de abas que aceita forms (`accept_forms`) e dois forms com `dockable` (Clientes e
//! Pedidos), que começam nela como abas. Puxe a aba de um deles para fora: o form vira a
//! janela dele. Arraste essa janela pela barra de título até a faixa: ele volta a ser aba. O
//! botão alterna o Clientes pelo código (`host`). Cada entrada e saída escreve uma linha
//! (`entrou: <form>` / `saiu: <form>`), para os testes que simulam o mouse.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example docking
//! ```

use std::io::Write;

use aylis_core::*;

fn child(ui: &mut Ui, name: &str, caption: &str, text: &str) -> Result<ComponentId, Error> {
    let form = ui.create("Form", None)?;
    ui.set_prop(form, "name", name)?;
    ui.set_prop(form, "caption", caption)?;
    ui.set_prop(form, "width", 520.0)?;
    ui.set_prop(form, "height", 340.0)?;
    ui.set_prop(form, "dockable", true)?;
    ui.set_prop(form, "padding", 16.0)?;
    let label = ui.create("Label", Some(form))?;
    ui.set_prop(label, "align", "top")?;
    ui.set_prop(label, "height", 40.0)?;
    ui.set_prop(label, "font_size", 18.0)?;
    ui.set_prop(label, "text", text)?;
    let edit = ui.create("Edit", Some(form))?;
    ui.set_prop(edit, "align", "top")?;
    ui.set_prop(edit, "text", format!("o mesmo {caption}, embutido ou na janela"))?;
    let say = |what: &'static str, caption: String| {
        move |_: &mut Ui, _: ComponentId, _: &EventArgs| {
            println!("{what}: {caption}");
            let _ = std::io::stdout().flush();
        }
    };
    ui.on(form, "on_dock", say("entrou", caption.to_string()));
    ui.on(form, "on_undock", say("saiu", caption.to_string()));
    Ok(form)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.create("Form", None)?;
    ui.set_prop(main, "name", "frmPrincipal")?;
    ui.set_prop(main, "caption", "Aylis — forms em abas")?;
    ui.set_prop(main, "width", 900.0)?;
    ui.set_prop(main, "height", 560.0)?;
    let bar = ui.create("Layout", Some(main))?;
    ui.set_prop(bar, "align", "top")?;
    ui.set_prop(bar, "height", 44.0)?;
    ui.set_prop(bar, "padding", 6.0)?;
    let toggle = ui.create("Button", Some(bar))?;
    ui.set_prop(toggle, "align", "left")?;
    ui.set_prop(toggle, "width", 260.0)?;
    ui.set_prop(toggle, "text", "Clientes: entrar / sair (host)")?;
    let strip = ui.create("TabStrip", Some(main))?;
    ui.set_prop(strip, "name", "tabForms")?;
    ui.set_prop(strip, "align", "top")?;
    ui.set_prop(strip, "height", 40.0)?;
    ui.set_prop(strip, "accept_forms", true)?;
    ui.set_prop(strip, "tear_off", true)?;
    let pages = ui.create("Layout", Some(main))?;
    ui.set_prop(pages, "align", "client")?;
    ui.set_prop(strip, "page_host", PropValue::Component(Some(pages)))?;

    let customers = child(&mut ui, "frmClientes", "Clientes", "Cadastro de clientes")?;
    let orders = child(&mut ui, "frmPedidos", "Pedidos", "Pedidos do dia")?;
    if std::env::args().all(|a| a != "--loose") {
        ui.set_prop(customers, "host", PropValue::Component(Some(strip)))?;
        ui.set_prop(orders, "host", PropValue::Component(Some(strip)))?;
    }
    ui.on_click(toggle, move |ui, _| {
        let docked = matches!(ui.get_prop(customers, "host"), Ok(PropValue::Component(Some(_))));
        let _ = ui.set_prop(customers, "host", PropValue::Component(if docked { None } else { Some(strip) }));
    });
    aylis_platform_winit::run(ui, main)?;
    Ok(())
}
