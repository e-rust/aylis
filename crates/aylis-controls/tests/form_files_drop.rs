//! Arquivos do sistema soltos no form: com handler no `on_files_drop`, o form recebe os
//! caminhos e o ponto, sem precisar de uma `FileDropZone` à vista (o relatório do player de
//! música, 21/11); um controle que aceita arquivos sob o ponto continua na frente.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{FileDropZone, Form};
use aylis_core::*;

#[test]
fn the_form_gets_the_dropped_files_with_the_point() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmX", 400.0, 300.0));
    let zone = ui.add(FileDropZone::default(), form);
    ui.control_mut(zone).unwrap().set_width(100.0).set_height(50.0);
    let got = Rc::new(RefCell::new(String::new()));
    let seen = got.clone();
    ui.on(form, "on_files_drop", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            *seen.borrow_mut() = t.clone();
        }
    });
    ui.show(form).unwrap();
    ui.layout();
    // Longe da zona: o form.
    assert!(ui.drop_files(form, PointF::new(300.0, 200.0), vec!["/m/a.mp3".into(), "/m/b.flac".into()]));
    assert_eq!(*got.borrow(), "/m/a.mp3;/m/b.flac");
    assert_eq!(ui.files_drop_position(form), Some(PointF::new(300.0, 200.0)));
    assert_eq!(ui.invoke(form, "drop_x", &[]), Ok(PropValue::Float(300.0)));
    // Na zona: ela, como antes.
    got.borrow_mut().clear();
    assert!(ui.drop_files(form, PointF::new(10.0, 10.0), vec!["/m/c.mp3".into()]));
    assert!(got.borrow().is_empty(), "a zona sob o ponto recebeu");
}
