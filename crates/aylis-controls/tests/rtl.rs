//! Interface espelhada (árabe): posições, encaixes, o desenho de cada controle, o mouse e
//! o que fica da esquerda para a direita.

use aylis_controls::{CheckBox, Edit, Form, Label, Panel};
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::*;

fn scene() -> (Ui, ComponentId, ComponentId, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("rtl", 400.0, 300.0));
    let side = ui.add(Panel::new(100.0, 100.0), form);
    ui.control_mut(side).unwrap().set_align(Align::Left);
    let label = ui.add(Label::new("مرحبا"), form);
    ui.control_mut(label).unwrap().set_rect(110.0, 10.0, 80.0, 20.0);
    let check = ui.add(CheckBox::new("موافق"), form);
    ui.control_mut(check).unwrap().set_rect(110.0, 40.0, 200.0, 24.0);
    ui.layout();
    (ui, form, side, label, check)
}

#[test]
fn positions_and_docking_are_mirrored() {
    let (mut ui, _, side, label, check) = scene();
    assert_eq!(ui.control(side).unwrap().bounds().x, 0.0);
    ui.set_right_to_left(true);
    ui.layout();
    assert_eq!(ui.control(side).unwrap().bounds(), RectF::new(300.0, 0.0, 100.0, 300.0), "Left vai para a direita");
    assert_eq!(ui.control(label).unwrap().bounds().x, 400.0 - 190.0);
    assert_eq!(ui.control(check).unwrap().bounds().x, 400.0 - 310.0);
    assert!(aylis_core::is_right_to_left_language("ar-SA") && aylis_core::is_right_to_left_language("he") && !aylis_core::is_right_to_left_language("pt-BR"));
}

#[test]
fn each_control_draws_mirrored_and_receives_mirrored_input() {
    let (mut ui, form, _, _, check) = scene();
    ui.set_right_to_left(true);
    ui.layout();
    let mut canvas = RecordingCanvas::new(None);
    ui.paint(check, &mut canvas);
    let b = ui.control(check).unwrap().bounds();
    // A caixa (o primeiro retângulo pequeno) fica na ponta direita do controle; o texto
    // continua com o alinhamento do fim (à direita).
    let boxes: Vec<RectF> = canvas.ops.iter().filter_map(|op| match op {
        Op::FillRect { rect, .. } | Op::StrokeRect { rect, .. } if rect.width < 30.0 && rect.height < 30.0 => Some(*rect),
        _ => None,
    }).collect();
    let first = boxes.first().copied().expect("a caixa");
    assert!(first.x > b.x + b.width - 40.0, "a caixa à direita: {first:?} em {b:?}");
    let text = canvas.ops.iter().find_map(|op| if let Op::Text { rect, style, .. } = op { Some((*rect, style.h_align)) } else { None }).unwrap();
    assert!(text.0.right() < first.x + 1.0, "o texto à esquerda da caixa: {text:?}");
    // O clique na caixa (à direita) chega ao controle como clique na caixa.
    let p = PointF::new(first.center().x, first.center().y);
    ui.pointer_down(form, PointerEvent { position: p, ..Default::default() });
    ui.pointer_up(form, PointerEvent { position: p, ..Default::default() });
    assert_eq!(ui.get_prop(check, "checked").unwrap(), PropValue::Bool(true));
}

#[test]
fn text_fields_and_locked_controls_keep_their_direction() {
    let (mut ui, form, side, _, _) = scene();
    let edit = ui.add(Edit::default(), form);
    ui.control_mut(edit).unwrap().set_rect(10.0, 100.0, 200.0, 32.0);
    let inner = ui.add(Label::new("x"), side);
    ui.control_mut(inner).unwrap().set_rect(5.0, 5.0, 20.0, 20.0);
    ui.control_mut(side).unwrap().set_keep_left_to_right(true);
    ui.set_right_to_left(true);
    ui.layout();
    // O painel vai para a direita, mas dentro dele nada vira.
    assert_eq!(ui.control(side).unwrap().bounds().x, 300.0);
    assert_eq!(ui.control(inner).unwrap().bounds().x, 5.0);
    assert!(!ui.is_mirrored(inner));
    // O campo de texto só muda de lugar.
    assert_eq!(ui.control(edit).unwrap().bounds().x, 400.0 - 210.0);
    let local = ui.local_point(edit, PointF::new(400.0 - 210.0 + 5.0, 110.0));
    assert_eq!(local.x, 5.0, "o mouse no campo de texto não vira");
}
