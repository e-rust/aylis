//! A galeria monta um cartão por componente visual e a busca filtra.

use aylis::core::EventArgs;

#[test]
fn a_galeria_monta_os_cartoes_e_a_busca_filtra() {
    let mut app = galeria::app().expect("o app abre");
    let ui = &mut app.ui;
    let form = ui.find("frmPrincipal").unwrap();
    ui.fire(form, "on_show", &EventArgs::None);
    ui.layout();
    let grade = ui.find_path("frmPrincipal.fluCartoes").unwrap();
    let total = ui.children(grade).len();
    assert!(total > 120, "cartões: {total}");
    assert!(ui.find_path("frmPrincipal.crdButton").is_some() && ui.find_path("frmPrincipal.crdCalendar").is_some());
    assert!(ui.find_path("frmPrincipal.crdTimer").is_none(), "o não visual sai");
    // A grade cresce com os cartões (a caixa rolada rola até o fim).
    assert!(ui.bounds(grade).unwrap().height > 3000.0, "{:?}", ui.bounds(grade));
    let busca = ui.find_path("frmPrincipal.edtBusca").unwrap();
    ui.set_prop(busca, "text", "chart").unwrap();
    ui.fire(busca, "on_change", &EventArgs::None);
    let visiveis = ui.children(grade).iter().filter(|&&c| ui.control(c).is_some_and(|cb| cb.visible())).count();
    assert!((1..10).contains(&visiveis), "com 'chart': {visiveis}");
}
