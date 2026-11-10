//! O teclado do celular cobre parte do form: o campo com foco fica à vista (`reveal`), o
//! form rola com o dedo quando o conteúdo não cabe e volta quando o teclado fecha.

mod common;

use common::boxy;
use aylis_core::*;

fn form_with_fields(n: usize) -> (Ui, ComponentId, Vec<ComponentId>) {
    let mut ui = Ui::new();
    let root = ui.add_root(boxy(400.0, 400.0));
    let fields: Vec<_> = (0..n)
        .map(|_| {
            let f = ui.add(boxy(400.0, 60.0), root);
            ui.control_mut(f).unwrap().set_align(Align::Top);
            f
        })
        .collect();
    ui.update_layout();
    (ui, root, fields)
}

#[test]
fn the_focused_field_shows_above_the_keyboard_and_the_form_comes_back() {
    // 6 campos de 60 = 360: cabe na janela de 400.
    let (mut ui, root, fields) = form_with_fields(6);
    let last = fields[5];
    assert_eq!(ui.absolute_rect(last).unwrap().y, 300.0);

    // O teclado cobre os 200 de baixo: o último campo (300..360) ficaria embaixo dele.
    ui.set_safe_area(root, Bounds::new(0.0, 0.0, 0.0, 200.0));
    ui.reveal(last);
    let r = ui.absolute_rect(last).unwrap();
    assert!(r.bottom() <= 200.0 - 7.9, "o campo ficou sob o teclado: {r:?}");
    assert!(r.y >= 0.0);

    // O dedo rola o form de volta para cima (até o começo).
    let wheel = WheelEvent { position: PointF::new(200.0, 100.0), delta: PointF::new(0.0, 500.0), touch: true, ..Default::default() };
    ui.wheel(root, wheel);
    ui.update_layout();
    assert_eq!(ui.absolute_rect(fields[0]).unwrap().y, 0.0);

    // Rolado de novo e o teclado fecha: tudo cabe, o form volta ao lugar.
    ui.reveal(last);
    ui.set_safe_area(root, Bounds::default());
    ui.update_layout();
    assert_eq!(ui.absolute_rect(fields[0]).unwrap().y, 0.0);
    assert_eq!(ui.absolute_rect(last).unwrap().y, 300.0);
}

#[test]
fn a_form_that_fits_does_not_scroll() {
    let (mut ui, root, fields) = form_with_fields(3);
    let wheel = WheelEvent { position: PointF::new(200.0, 100.0), delta: PointF::new(0.0, -100.0), ..Default::default() };
    ui.wheel(root, wheel);
    ui.update_layout();
    assert_eq!(ui.absolute_rect(fields[0]).unwrap().y, 0.0);
}
