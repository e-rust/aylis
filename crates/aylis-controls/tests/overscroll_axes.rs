//! O fim da rolagem só no eixo em que o contêiner rola (relatado pelo RTSPlayer em 15/11): na
//! grade de câmeras, que só rola na vertical, o arrasto de lado esticava o conteúdo na
//! horizontal. Como no Android, a lista vertical estica só em cima e embaixo.

use aylis_controls::*;
use aylis_core::*;

fn touch(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), kind: PointerKind::Touch, pointer_id: 7, ..Default::default() }
}

/// Arrasta de `from` a `to` em 4 passos devagar e diz se abriu um puxão (pede quadros com o
/// dedo parado); solta e deixa voltar.
fn pulls(ui: &mut Ui, form: ComponentId, from: (f32, f32), to: (f32, f32)) -> bool {
    ui.pointer_down(form, touch(from.0, from.1));
    for i in 1..=4 {
        let t = ui.time() + 0.1;
        ui.set_time(t);
        let k = i as f32 / 4.0;
        ui.pointer_move(form, touch(from.0 + (to.0 - from.0) * k, from.1 + (to.1 - from.1) * k));
    }
    let pulled = ui.is_animating();
    ui.pointer_up(form, touch(to.0, to.1));
    for _ in 0..200 {
        if !ui.is_animating() {
            break;
        }
        let t = ui.time() + 1.0 / 60.0;
        ui.set_time(t);
        ui.tick();
    }
    pulled
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_overscroll_style(Overscroll::Stretch);
    ui.set_time(100.0);
    let form = ui.add_root(Form::new("frmCameras", 400.0, 600.0));
    ui.show(form).unwrap();
    (ui, form)
}

#[test]
fn a_vertical_grid_stretches_only_up_and_down() {
    let (mut ui, form) = setup();
    let grid = ui.create("VirtualList", Some(form)).unwrap();
    ui.set_prop(grid, "align", "client").unwrap();
    ui.set_prop(grid, "columns", 2i64).unwrap();
    ui.set_prop(grid, "item_height", 120.0).unwrap();
    ui.set_prop(grid, "item_count", 20i64).unwrap();
    ui.layout();
    assert!(!pulls(&mut ui, form, (300.0, 300.0), (100.0, 305.0)), "de lado não estica");
    assert!(!pulls(&mut ui, form, (100.0, 300.0), (300.0, 295.0)), "de lado não estica");
    assert!(pulls(&mut ui, form, (200.0, 100.0), (205.0, 300.0)), "para baixo, no topo, estica");
}

#[test]
fn a_horizontal_scroll_box_stretches_only_sideways() {
    let (mut ui, form) = setup();
    let scroll = ui.create("ScrollBox", Some(form)).unwrap();
    ui.set_prop(scroll, "align", "client").unwrap();
    ui.set_prop(scroll, "horizontal", true).unwrap();
    for i in 0..20 {
        let b = ui.create("Button", Some(scroll)).unwrap();
        ui.set_prop(b, "align", "left").unwrap();
        ui.set_prop(b, "width", 120.0).unwrap();
        ui.set_prop(b, "text", format!("Coluna {i}")).unwrap();
    }
    ui.layout();
    assert!(!pulls(&mut ui, form, (200.0, 100.0), (205.0, 300.0)), "na vertical não estica");
    assert!(pulls(&mut ui, form, (100.0, 300.0), (300.0, 305.0)), "para a direita, no começo, estica");
}
