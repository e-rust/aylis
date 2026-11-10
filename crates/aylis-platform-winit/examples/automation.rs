//! Automação e leitores de tela: os controles chegam ao sistema (UI Automation no Windows)
//! com o nome do componente como AutomationId, e as abas, as linhas da lista e os itens de
//! menu como itens próprios. Uma ferramenta de automação (FlaUI, pywinauto, o Narrador)
//! acha e clica em tudo sem mandar evento de mouse para a janela.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example automation
//! ```

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("aylis — automação", 520.0, 360.0));
    ui.control_mut(form).unwrap().set_padding(16.0);

    let status = ui.add(Label::new("Nada ainda"), form);
    ui.set_name(status, "lblStatus")?;
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(28.0);

    let save = ui.add(Button::new("Salvar"), form);
    ui.set_name(save, "btnSalvar")?;
    ui.control_mut(save).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    ui.on_click(save, move |ui, _| {
        let _ = ui.set_prop(status, "text", "Salvo");
    });

    let tabs = ui.add(TabControl::default(), form);
    ui.set_name(tabs, "tabPedido")?;
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    TabControl::add_tab(&mut ui, tabs, "Dados");
    let customers = TabControl::add_tab(&mut ui, tabs, "Clientes");
    ui.on(tabs, "on_change", move |ui, _, _| {
        let _ = ui.set_prop(status, "text", "Aba trocada");
    });
    let cities = ui.add(ListBox::new(&["Recife", "São Paulo", "Curitiba"]), customers);
    ui.set_name(cities, "lstCidades")?;
    ui.control_mut(cities).unwrap().set_align(Align::Client);
    ui.on(cities, "on_change", move |ui, _, args| {
        let _ = ui.set_prop(status, "text", format!("Cidade: {}", args.text().unwrap_or_default()));
    });
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
