//! O `on_view_change` do form: a vista que entrou pelo tamanho (vazio: o desenho base), para o
//! código fazer o que não é propriedade (decidido em 21/11, no lugar dos forms por faixa).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::*;

#[test]
fn the_form_tells_which_view_came_in() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmPlayer", 900.0, 600.0));
    ui.set_views(form, vec![FormView::new("compact", 699.0)]);
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    ui.on(form, "on_view_change", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            log.borrow_mut().push(t.clone());
        }
    });
    ui.show(form).unwrap();
    ui.layout();
    assert!(seen.borrow().is_empty(), "largo: o base, sem mudança");
    ui.control_mut(form).unwrap().set_width(500.0);
    ui.layout();
    ui.control_mut(form).unwrap().set_width(480.0);
    ui.layout();
    ui.control_mut(form).unwrap().set_width(1000.0);
    ui.layout();
    assert_eq!(*seen.borrow(), vec!["compact".to_string(), String::new()], "uma vez ao entrar e uma ao sair");
}
