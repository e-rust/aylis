//! Vários forms: `show` (outra janela), `show_modal` (resultado numa função) e um form
//! embutido num painel de outro (`Form2.Parent := Panel1`), com nomes repetidos entre os
//! forms (os dois têm um `btnSalvar`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example windows
//! ```

use aylis_controls::{Button, Edit, Form, Label, Layout, Panel};
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);

    // ----- principal -----
    let main_form = ui.add_root(Form::new("aylis — vários forms", 640.0, 420.0));
    ui.set_name(main_form, "frmPrincipal")?;
    ui.control_mut(main_form).unwrap().set_padding(12.0);
    let bar = ui.add(Layout::default(), main_form);
    ui.control_mut(bar).unwrap().set_align(Align::Top).set_height(36.0).set_margins([0.0, 0.0, 0.0, 10.0]);
    let button = |ui: &mut Ui, text: &str, name: &str| {
        let b = ui.add(Button::new(text), bar);
        ui.control_mut(b).unwrap().set_align(Align::Left).set_width(150.0).set_margins([0.0, 0.0, 8.0, 0.0]);
        ui.set_name(b, name).unwrap();
        b
    };
    let b_show = button(&mut ui, "Show", "btnShow");
    let b_modal = button(&mut ui, "ShowModal", "btnModal");
    let b_embed = button(&mut ui, "Embutir", "btnEmbutir");
    let b_save = button(&mut ui, "Salvar (principal)", "btnSalvar");
    let status = ui.add(Label::new("Escolha uma opção"), main_form);
    ui.control_mut(status).unwrap().set_align(Align::Bottom).set_height(28.0);
    ui.set_name(status, "lblStatus")?;
    let panel = ui.add(Panel::new(0.0, 0.0), main_form);
    ui.control_mut(panel).unwrap().set_align(Align::Client).set_padding(8.0);
    ui.set_name(panel, "pnlConteudo")?;

    // ----- um form comum (Show) -----
    let list = ui.add_root(Form::new("Lista (Show)", 360.0, 240.0));
    ui.set_name(list, "frmLista")?;
    let l = ui.add(Label::new("Janela comum: o principal continua respondendo."), list);
    ui.control_mut(l).unwrap().set_align(Align::Client).set_margins(16.0);

    // ----- um diálogo (ShowModal) -----
    let dialog = ui.add_root(Form::new("Novo cliente (ShowModal)", 380.0, 170.0));
    ui.set_name(dialog, "frmDialogo")?;
    ui.set_prop(dialog, "border_style", "single")?;
    ui.control_mut(dialog).unwrap().set_padding(16.0);
    let name = ui.add(Edit::new(""), dialog);
    ui.control_mut(name).unwrap().set_align(Align::Top).set_height(36.0);
    ui.set_prop(name, "placeholder", "Nome do cliente")?;
    ui.set_name(name, "edtNome")?;
    let footer = ui.add(Layout::default(), dialog);
    ui.control_mut(footer).unwrap().set_align(Align::Bottom).set_height(36.0);
    for (text, result) in [("Cancelar", "cancel"), ("OK", "ok")] {
        let b = ui.add(Button::new(text), footer);
        ui.control_mut(b).unwrap().set_align(Align::Right).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]);
        ui.set_prop(b, "modal_result", result)?;
    }
    // Exige um nome para o OK (CanClose := False se estiver vazio).
    ui.on(dialog, "on_close_query", move |ui, form, _| {
        let empty = ui.get::<Edit>(name).is_some_and(|e| e.text.trim().is_empty());
        if ui.modal_result(form) == ModalResult::Ok && empty {
            ui.cancel_close();
            let _ = ui.set_prop(name, "placeholder", "Informe o nome (obrigatório)");
        }
    });

    // ----- um form para embutir (também tem um btnSalvar) -----
    let customers = ui.add_root(Form::new("Clientes", 300.0, 200.0));
    ui.set_name(customers, "frmClientes")?;
    ui.control_mut(customers).unwrap().set_padding(12.0);
    let title = ui.add(Label::new("Form frmClientes embutido no painel"), customers);
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(28.0);
    let save_customers = ui.add(Button::new("Salvar (clientes)"), customers);
    ui.control_mut(save_customers).unwrap().set_align(Align::Top).set_height(36.0);
    ui.set_name(save_customers, "btnSalvar")?;

    // ----- handlers -----
    let say = |ui: &mut Ui, text: String| {
        if let Some(l) = ui.find_path("frmPrincipal.lblStatus") {
            let _ = ui.set_prop(l, "text", text);
        }
    };
    ui.on_click(b_show, move |ui, _| {
        let _ = ui.show(list);
        say(ui, "Show: frmLista aberto em outra janela".into());
    });
    ui.on_click(b_modal, move |ui, _| {
        let _ = ui.set_prop(name, "text", "");
        let _ = ui.show_modal(dialog, move |ui, result| {
            let text = ui.get::<Edit>(name).map(|e| e.text.clone()).unwrap_or_default();
            say(ui, format!("ShowModal terminou: {} ({text})", result.name()));
        });
    });
    ui.on_click(b_embed, move |ui, _| {
        if ui.parent(customers).is_none() {
            let _ = ui.set_parent(customers, Some(panel));
            ui.control_mut(customers).unwrap().set_align(Align::Client);
            say(ui, "frmClientes embutido no painel (dois btnSalvar, um em cada form)".into());
        }
    });
    ui.on_click(b_save, move |ui, sender| say(ui, format!("clicou em {}", ui.path_of(sender))));
    ui.on_click(save_customers, move |ui, sender| say(ui, format!("clicou em {}", ui.path_of(sender))));

    aylis_platform_winit::run(ui, main_form)
}
