//! Os gestos do `Model3D` no celular, como nos visualizadores 3D: um dedo gira a câmera,
//! pinçar aproxima e girar os dois dedos não gira o modelo.

use aylis_3d::Model3D;
use aylis_core::*;

fn touch(id: u64, x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), kind: PointerKind::Touch, pointer_id: id, ..Default::default() }
}

fn float(ui: &Ui, id: ComponentId, prop: &str) -> f32 {
    match ui.get_prop(id, prop).unwrap() {
        PropValue::Float(v) => v as f32,
        other => panic!("{prop}: {other:?}"),
    }
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_3d::register(&mut ui);
    let id = ui.add_root(Model3D::default());
    ui.control_mut(id).unwrap().set_rect(0.0, 0.0, 400.0, 400.0);
    ui.layout();
    (ui, id)
}

#[test]
fn one_finger_orbits_and_does_not_zoom() {
    let (mut ui, id) = setup();
    let (yaw, pitch) = (float(&ui, id, "yaw"), float(&ui, id, "pitch"));
    ui.pointer_down(id, touch(1, 200.0, 200.0));
    for i in 1..=10 {
        ui.pointer_move(id, touch(1, 200.0 + i as f32 * 10.0, 200.0 + i as f32 * 5.0));
    }
    ui.pointer_up(id, touch(1, 300.0, 250.0));
    assert!((float(&ui, id, "zoom") - 1.0).abs() < 1e-6, "o arrasto do dedo não muda o zoom");
    assert!((float(&ui, id, "yaw") - (yaw - 40.0)).abs() < 1.0, "girou: {}", float(&ui, id, "yaw"));
    assert!(float(&ui, id, "pitch") > pitch, "inclinou: {}", float(&ui, id, "pitch"));
}

#[test]
fn pinching_zooms_and_twisting_does_not_spin() {
    let (mut ui, id) = setup();
    let yaw = float(&ui, id, "yaw");
    ui.pointer_down(id, touch(1, 150.0, 200.0));
    ui.pointer_down(id, touch(2, 250.0, 200.0));
    // Abre os dedos: aproxima.
    ui.pointer_move(id, touch(2, 350.0, 200.0));
    assert!(float(&ui, id, "zoom") > 1.5, "zoom {}", float(&ui, id, "zoom"));
    // Gira os dedos (mesma distância): o modelo não gira.
    ui.pointer_move(id, touch(2, 150.0, 400.0));
    assert!((float(&ui, id, "yaw") - yaw).abs() < 1e-3);
    ui.pointer_up(id, touch(2, 150.0, 400.0));
    ui.pointer_up(id, touch(1, 150.0, 200.0));
    // Sem a órbita, o arrasto volta a ser da tela (rolagem).
    ui.set_prop(id, "orbit", false).unwrap();
    assert!(ui.control(id).unwrap().pan_on_touch());
}
