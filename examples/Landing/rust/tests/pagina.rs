//! A landing page carrega inteira (sem propriedade errada no form), vira a vista de celular
//! numa tela estreita.

use aylis::core::PropValue;

#[test]
fn a_pagina_carrega_e_vira_celular() {
    let mut app = landing::app().expect("o app abre");
    let ui = &mut app.ui;
    let form = ui.find("frmPrincipal").expect("o form principal");
    ui.layout();
    assert_eq!(ui.active_view(form), None, "1200 pontos: o desktop");
    for path in ["frmPrincipal.layLingote", "frmPrincipal.grdLinguagens", "frmPrincipal.grdComponentes", "frmPrincipal.grdIdiomas", "frmPrincipal.svgLinux"] {
        assert!(ui.find_path(path).is_some(), "{path}");
    }

    // Celular: o lingote some, o título diminui e as linguagens ficam numa coluna.
    ui.control_mut(form).unwrap().set_width(390.0);
    ui.layout();
    assert_eq!(ui.active_view(form), Some("phone"));
    let get = |ui: &aylis::core::Ui, path: &str, prop: &str| ui.get_prop(ui.find_path(path).unwrap(), prop).unwrap();
    assert_eq!(get(ui, "frmPrincipal.layLingote", "visible"), PropValue::Bool(false));
    assert_eq!(get(ui, "frmPrincipal.lblTitulo", "font_size"), PropValue::Float(30.0));
    assert_eq!(get(ui, "frmPrincipal.grdLinguagens", "columns"), PropValue::Int(1));
    // As seções crescem com o conteúdo: numa coluna, os 8 cartões de linguagem cabem.
    let section = ui.find_path("frmPrincipal.secLinguagens").unwrap();
    let grid = ui.find_path("frmPrincipal.grdLinguagens").unwrap();
    assert!(ui.bounds(grid).unwrap().height >= 8.0 * 112.0, "{:?}", ui.bounds(grid));
    assert!(ui.bounds(section).unwrap().height > ui.bounds(grid).unwrap().height);
}
