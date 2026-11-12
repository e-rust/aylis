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

/// Rolar a página no celular só anda o conteúdo: antes, cada passo da rolagem refazia o layout
/// do form inteiro (os ~100 textos medidos de novo), e no celular a rolagem ficava lenta.
/// Imprime o custo por passo dos dois jeitos (`--nocapture`, em release).
#[test]
fn scrolling_on_the_phone_does_not_lay_out_the_page_again() {
    use aylis::core::{PointF, WheelEvent};
    let mut app = landing::app().expect("o app abre");
    let ui = &mut app.ui;
    let form = ui.find("frmPrincipal").expect("o form principal");
    ui.control_mut(form).unwrap().set_width(375.0).set_height(812.0);
    ui.layout();
    let page = ui.find_path("frmPrincipal.scrPagina").unwrap();
    let wheel = |dy: f32| WheelEvent { position: PointF::new(150.0, 400.0), delta: PointF::new(0.0, dy), ..Default::default() };
    let steps = 120;
    let run = |ui: &mut aylis::core::Ui, old_way: bool| {
        let start = std::time::Instant::now();
        for i in 0..steps {
            ui.wheel(form, wheel(if i < steps / 2 { -40.0 } else { 40.0 }));
            assert!(old_way || !ui.needs_layout(), "rolar pediu o layout do form");
            if old_way {
                // O caminho de antes: o layout do form a cada passo.
                ui.invalidate_form(page);
            }
            ui.update_layout();
        }
        start.elapsed().as_secs_f64() * 1000.0 / steps as f64
    };
    let now = run(ui, false);
    let before = run(ui, true);
    println!("rolagem da Landing no celular, por passo: {now:.3} ms (antes, com o layout: {before:.3} ms)");
}
