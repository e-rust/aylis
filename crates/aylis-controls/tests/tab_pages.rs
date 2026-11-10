//! TabControl sem a barra de abas (`show_tabs = false`): só as páginas, até as bordas,
//! trocadas pelo `tab_index`.

use aylis_controls::{Form, TabControl, TabItem};
use aylis_core::*;

#[test]
fn pages_without_the_tab_bar() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let _ = ui.set_prop(form, "padding", 0i64);
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    let a = TabControl::add_tab(&mut ui, tabs, "A");
    let b = TabControl::add_tab(&mut ui, tabs, "B");
    ui.set_prop(tabs, "show_tabs", false).unwrap();
    for t in [a, b] {
        ui.set_prop(t, "padding", 0i64).unwrap();
    }
    ui.layout();
    // A página ocupa o controle inteiro (sem a barra de 38 px).
    assert_eq!(ui.bounds(a).unwrap(), RectF::new(0.0, 0.0, 400.0, 300.0));
    assert!(ui.control(a).unwrap().visible() && !ui.control(b).unwrap().visible());
    // Clicar onde seria a barra não troca a página; o código troca pelo tab_index.
    ui.pointer_down(form, PointerEvent { position: PointF::new(60.0, 10.0), button: PointerButton::Primary, ..Default::default() });
    assert!(ui.control(a).unwrap().visible());
    TabControl::select(&mut ui, tabs, 1);
    assert!(!ui.control(a).unwrap().visible() && ui.control(b).unwrap().visible());
}
