//! Transições: a aba nova desliza para dentro enquanto a antiga sai, e os forms empilhados
//! (celular, navegador) entram e saem com a `transition` deles.

use aylis_controls::*;
use aylis_core::*;

fn translate_x(ui: &Ui, id: ComponentId) -> f32 {
    ui.control(id).unwrap().translate().x
}

#[test]
fn tabs_slide_in_the_order_of_the_tabs() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let tabs = ui.add(TabControl::default(), form);
    ui.control_mut(tabs).unwrap().set_align(Align::Client);
    let a = TabControl::add_tab(&mut ui, tabs, "A");
    let b = TabControl::add_tab(&mut ui, tabs, "B");
    let _ = ui.set_prop(tabs, "transition", PropValue::Enum("slide_left".into()));
    ui.update_layout();
    let width = ui.bounds(b).unwrap().width;
    assert!(width > 100.0);

    // Avança para B: B entra pela direita, A sai pela esquerda.
    ui.set_time(10.0);
    TabControl::select(&mut ui, tabs, 1);
    assert!(ui.control(a).unwrap().visible() && ui.control(b).unwrap().visible(), "as duas aparecem");
    assert!((translate_x(&ui, b) - width).abs() < 0.5);
    ui.set_time(10.1);
    ui.tick();
    assert!(translate_x(&ui, b) > 0.0 && translate_x(&ui, b) < width);
    assert!(translate_x(&ui, a) < 0.0);
    assert!(ui.is_animating());
    ui.set_time(11.0);
    ui.tick();
    assert_eq!(translate_x(&ui, b), 0.0);
    assert!(!ui.control(a).unwrap().visible(), "a antiga some no fim");
    assert_eq!(translate_x(&ui, a), 0.0, "e volta ao lugar");

    // Volta para A: A entra pela esquerda.
    ui.set_time(20.0);
    TabControl::select(&mut ui, tabs, 0);
    assert!(translate_x(&ui, a) < 0.0);
    ui.set_time(21.0);
    ui.tick();
    assert!(!ui.control(b).unwrap().visible());
}

#[test]
fn stacked_forms_slide_in_and_out() {
    let mut ui = Ui::new();
    ui.set_stacked_forms(true);
    let main = ui.add_root(Form::new("principal", 400.0, 600.0));
    let second = ui.add_root(Form::new("segunda", 400.0, 600.0));
    let _ = ui.set_prop(second, "transition", PropValue::Enum("slide_left".into()));
    ui.show(main).unwrap();
    ui.update_layout();

    ui.set_time(1.0);
    ui.show(second).unwrap();
    assert!((translate_x(&ui, second) - 400.0).abs() < 0.5, "entra pela direita");
    ui.set_time(2.0);
    ui.tick();
    assert_eq!(translate_x(&ui, second), 0.0);

    // Fechar: sai pela direita (o caminho de volta) e só então fecha.
    ui.set_time(3.0);
    assert!(ui.close(second));
    assert!(ui.is_open(second), "ainda aberto durante a saída");
    ui.set_time(3.1);
    ui.tick();
    assert!(translate_x(&ui, second) > 0.0);
    ui.set_time(4.0);
    ui.tick();
    assert!(!ui.is_open(second), "fechou no fim");
    assert_eq!(translate_x(&ui, second), 0.0);
    assert!(ui.is_open(main));
}

#[test]
fn without_stacking_forms_open_and_close_at_once() {
    let mut ui = Ui::new();
    let main = ui.add_root(Form::new("principal", 400.0, 600.0));
    let second = ui.add_root(Form::new("segunda", 400.0, 600.0));
    let _ = ui.set_prop(second, "transition", PropValue::Enum("fade".into()));
    ui.show(main).unwrap();
    ui.show(second).unwrap();
    assert_eq!(ui.control(second).unwrap().opacity(), 1.0);
    assert!(ui.close(second));
    assert!(!ui.is_open(second));
}
