//! O tutorial "Seu primeiro projeto": digitar o nome e clicar em "Saudar" escreve a saudação;
//! sem nome, saúda o mundo.

use aylis::core::{EventArgs, PropValue, Ui};

fn text(ui: &Ui, path: &str) -> String {
    match ui.get_prop(ui.find_path(path).unwrap(), "text").unwrap() {
        PropValue::String(s) => s,
        other => format!("{other:?}"),
    }
}

#[test]
fn saudar_escreve_ola_com_o_nome() {
    let mut app = ola_nome::app().expect("o app abre");
    let ui = &mut app.ui;
    let botao = ui.find_path("frmPrincipal.btnSaudar").unwrap();
    ui.fire(botao, "on_click", &EventArgs::None);
    assert_eq!(text(ui, "frmPrincipal.lblResposta"), "Olá, mundo!");
    let nome = ui.find_path("frmPrincipal.edtNome").unwrap();
    ui.set_prop(nome, "text", "Maria").unwrap();
    ui.fire(botao, "on_click", &EventArgs::None);
    assert_eq!(text(ui, "frmPrincipal.lblResposta"), "Olá, Maria!");
}
