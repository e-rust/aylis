//! Componentes industriais: o cano preso segue os componentes e os comandos mudam de estado.

use aylis_controls::*;
use aylis_core::*;

#[test]
fn a_pipe_follows_the_components_it_connects() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 800.0, 600.0));
    let tank = ui.add(TankLevel::default(), form);
    ui.control_mut(tank).unwrap().set_rect(10.0, 10.0, 100.0, 100.0);
    let pump = ui.add(Pump::default(), form);
    ui.control_mut(pump).unwrap().set_rect(300.0, 200.0, 80.0, 80.0);
    let pipe = ui.add(Pipe::default(), form);
    ui.set_prop(pipe, "from", PropValue::Component(Some(tank))).unwrap();
    ui.set_prop(pipe, "to", PropValue::Component(Some(pump))).unwrap();
    ui.set_prop(pipe, "to_port", "left").unwrap();
    ui.layout();
    let b = ui.control(pipe).unwrap().bounds();
    // Do lado direito do tanque (110, 60) ao esquerdo da bomba (300, 240), com folga.
    assert!(b.x <= 110.0 && b.right() >= 300.0 && b.y <= 60.0 && b.bottom() >= 240.0, "{b:?}");
    ui.control_mut(pump).unwrap().set_rect(500.0, 400.0, 80.0, 80.0);
    ui.layout();
    let b2 = ui.control(pipe).unwrap().bounds();
    assert!(b2.right() >= 500.0 && b2.bottom() >= 440.0, "segue a bomba: {b2:?}");
    // Apagar a bomba solta a ponta.
    ui.free(pump);
    assert_eq!(ui.get_prop(pipe, "to").unwrap(), PropValue::Component(None));
    // A rota tem só ângulos retos.
    let c = Connection { from: tank, from_at: (1.0, 0.5), to: tank, to_at: (0.0, 0.5) };
    let pts = c.route(RectF::new(0.0, 0.0, 10.0, 10.0), RectF::new(100.0, 50.0, 10.0, 10.0));
    assert!(pts.windows(2).all(|w| (w[0].x - w[1].x).abs() < 0.01 || (w[0].y - w[1].y).abs() < 0.01), "{pts:?}");
}

#[test]
fn commands_toggle_and_cycle() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let stop = ui.add(EmergencyStop::default(), form);
    let sel = ui.add(SelectorSwitch::default(), form);
    ui.layout();
    ui.click(stop);
    ui.click(sel);
    ui.click(sel);
    assert_eq!(ui.get_prop(stop, "pressed").unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(sel, "item_index").unwrap(), PropValue::Int(0), "Man;0;Auto: de 0 vai a Auto e volta a Man");
    assert!(ui.registry().variant("ElectricalSymbol", "relay").is_some());
}
