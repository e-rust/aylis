//! Âncoras de controles que medem a própria altura (o `auto_size` do Label): acompanham o pai
//! que muda de tamanho, na janela e na célula de um GridLayout (relatório do InsightDB, 11/11:
//! o `auto_size` regravava a altura a cada arrumação, a referência das âncoras era refeita e o
//! controle ficava parado).

use aylis_controls::{Form, GridLayout, Label, Panel};
use aylis_core::*;

fn label(ui: &mut Ui, parent: ComponentId, x: f32, y: f32) -> ComponentId {
    let l = ui.add(Label::new("Total"), parent);
    ui.control_mut(l).unwrap().set_position(x, y).set_width(60.0).set_anchors(Anchors::RIGHT | Anchors::BOTTOM);
    let _ = ui.set_prop(l, "auto_size", true);
    l
}

#[test]
fn anchored_auto_size_labels_follow_the_window_and_the_grid_cell() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 400.0, 300.0));
    let corner = label(&mut ui, form, 300.0, 250.0);
    let grid = ui.add(GridLayout::default(), form);
    ui.control_mut(grid).unwrap().set_align(Align::Client);
    let _ = ui.set_prop(grid, "columns", 2_i64);
    let cell = ui.add(Panel::new(100.0, 100.0), grid);
    let _other = ui.add(Panel::new(100.0, 100.0), grid);
    let in_cell = label(&mut ui, cell, 20.0, 60.0);
    ui.layout();
    let before = (ui.bounds(corner).unwrap(), ui.bounds(in_cell).unwrap(), ui.bounds(cell).unwrap());

    // A janela em tela cheia: o rótulo do canto anda junto, e a célula (que cresce com a grade)
    // leva o dela.
    ui.control_mut(form).unwrap().set_size(800.0, 600.0);
    ui.layout();
    let after = (ui.bounds(corner).unwrap(), ui.bounds(in_cell).unwrap(), ui.bounds(cell).unwrap());
    assert_eq!(after.0.x - before.0.x, 400.0, "o rótulo da janela acompanhou a largura");
    assert_eq!(after.0.y - before.0.y, 300.0, "e a altura");
    let grew = (after.2.width - before.2.width, after.2.height - before.2.height);
    assert!(grew.0 > 0.0, "a célula cresceu");
    assert_eq!(after.1.x - before.1.x, grew.0, "o rótulo da célula acompanhou a célula");
    assert_eq!(after.1.y - before.1.y, grew.1);
}
