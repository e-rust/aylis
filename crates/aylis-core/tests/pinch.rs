//! Pinça e giro com dois dedos: o gesto vai ao controle tocado e nenhum dedo vira clique.

mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::boxy;
use aylis_core::*;

fn finger(id: u64, x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), pointer_id: id, kind: PointerKind::Touch, button: PointerButton::Primary, ..Default::default() }
}

#[test]
fn two_fingers_make_a_gesture_and_no_click() {
    let mut ui = Ui::new();
    let root = ui.add_root(boxy(400.0, 400.0));
    let child = ui.add(boxy(400.0, 400.0), root);
    ui.control_mut(child).unwrap().set_align(Align::Client);
    let gestures: Rc<RefCell<Vec<GestureEvent>>> = Rc::default();
    let clicks = Rc::new(RefCell::new(0));
    {
        let g = gestures.clone();
        ui.on(child, "on_gesture", move |_, _, args| {
            if let EventArgs::Gesture(e) = args {
                g.borrow_mut().push(*e);
            }
        });
        let c = clicks.clone();
        ui.on(child, "on_click", move |_, _, _| *c.borrow_mut() += 1);
    }

    // Dedos a 100 de distância, na horizontal; depois a 200, na vertical (giro de 90°).
    ui.pointer_down(root, finger(1, 150.0, 200.0));
    ui.pointer_down(root, finger(2, 250.0, 200.0));
    ui.pointer_move(root, finger(2, 350.0, 200.0));
    ui.pointer_move(root, finger(1, 250.0, 100.0));
    ui.pointer_move(root, finger(2, 250.0, 300.0));
    ui.pointer_up(root, finger(2, 250.0, 300.0));
    // O dedo que sobrou anda e sai: não rola nem clica.
    ui.pointer_move(root, finger(1, 260.0, 110.0));
    ui.pointer_up(root, finger(1, 260.0, 110.0));
    ui.process_posted();

    let g = gestures.borrow();
    assert_eq!(g.first().map(|e| e.phase), Some(GesturePhase::Begin));
    assert_eq!(g.last().map(|e| e.phase), Some(GesturePhase::End));
    let updates: Vec<_> = g.iter().filter(|e| e.phase == GesturePhase::Update).collect();
    assert_eq!(updates.len(), 3);
    // 100 → 200: escala 2 (o produto das escalas parciais também).
    let last = updates.last().unwrap();
    assert!((last.scale - 2.0).abs() < 1e-3, "{last:?}");
    let product: f32 = updates.iter().map(|e| e.delta_scale).product();
    assert!((product - 2.0).abs() < 1e-3);
    // Horizontal → vertical para baixo: +90°.
    assert!((last.rotation - std::f32::consts::FRAC_PI_2).abs() < 1e-3, "{last:?}");
    assert!((last.center.x - 250.0).abs() < 1e-3 && (last.center.y - 200.0).abs() < 1e-3);
    assert!((g.last().unwrap().scale - 2.0).abs() < 1e-3);
    assert_eq!(*clicks.borrow(), 0, "nenhum dedo vira clique");

    // Depois do gesto, um toque comum volta a clicar.
    ui.pointer_down(root, finger(3, 100.0, 100.0));
    ui.pointer_up(root, finger(3, 100.0, 100.0));
    ui.process_posted();
    assert_eq!(*clicks.borrow(), 1);
}
