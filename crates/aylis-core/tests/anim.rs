mod common;

use common::{RecordingCanvas, boxy};
use aylis_core::*;

fn float(ui: &Ui, id: ComponentId, prop: &str) -> f64 {
    ui.get_prop(id, prop).unwrap().to_f64().unwrap()
}

#[test]
fn easing_curves_start_at_0_and_end_at_1() {
    for e in Easing::ALL {
        assert!(e.apply(0.0).abs() < 1e-6, "{e:?}");
        assert!((e.apply(1.0) - 1.0).abs() < 1e-6, "{e:?}");
    }
    assert!(Easing::OutBack.apply(0.7) > 1.0, "OutBack passa do destino");
}

#[test]
fn animate_a_property_over_time() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    ui.set_time(10.0);
    ui.animate(form, "opacity", 0.0, AnimationOptions::new(1000.0, Easing::Linear)).unwrap();
    assert!(ui.is_animating());

    ui.set_time(10.5);
    ui.tick();
    assert!((float(&ui, form, "opacity") - 0.5).abs() < 1e-6);

    ui.set_time(11.2);
    ui.tick();
    assert_eq!(float(&ui, form, "opacity"), 0.0);
    assert!(!ui.is_animating(), "terminou");
}

#[test]
fn auto_reverse_and_looping() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let options = AnimationOptions { duration_ms: 1000.0, easing: Easing::Linear, auto_reverse: true, looping: true, ..Default::default() };
    ui.animate(form, "left", 100, options).unwrap();
    for (time, expected) in [(0.5, 50.0), (1.0, 100.0), (1.5, 50.0), (2.5, 50.0)] {
        ui.set_time(time);
        ui.tick();
        assert!((float(&ui, form, "left") - expected).abs() < 1e-3, "t={time}");
    }
    ui.stop_animations(form);
    assert!(!ui.is_animating());
}

#[test]
fn colors_are_animated_too() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let anim = ui.add(PropertyAnimation::color("tint", Color::WHITE), form);
    // Boxy não tem "tint": a animação falha com erro, sem quebrar nada
    assert!(ui.start_animation(anim).is_err());

    let err = ui.animate(form, "visible", false, AnimationOptions::default()).unwrap_err();
    assert!(err.to_string().contains("numéricas ou de cor"), "{err}");
}

#[test]
fn trigger_animation_goes_and_comes_back() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(200.0, 100.0));
    let target = ui.add(boxy(100.0, 100.0), form);
    let anim = ui.add(
        PropertyAnimation::float("opacity", 0.5).with_trigger(Trigger::Hover).with_duration(100.0).with_easing(Easing::Linear),
        target,
    );
    ui.layout();

    ui.set_time(0.0);
    ui.pointer_move(form, PointerEvent { position: PointF::new(10.0, 10.0), ..Default::default() });
    ui.set_time(0.05);
    ui.tick();
    assert!((float(&ui, target, "opacity") - 0.75).abs() < 1e-3);
    ui.set_time(0.2);
    ui.tick();
    assert_eq!(float(&ui, target, "opacity"), 0.5);

    // saiu: volta ao valor original
    ui.pointer_move(form, PointerEvent { position: PointF::new(150.0, 10.0), ..Default::default() });
    ui.set_time(0.4);
    ui.tick();
    assert_eq!(float(&ui, target, "opacity"), 1.0);

    // desligada não dispara
    ui.set_prop(anim, "enabled", false).unwrap();
    ui.pointer_move(form, PointerEvent { position: PointF::new(10.0, 10.0), ..Default::default() });
    assert!(!ui.is_animating());
}

#[test]
fn animation_components_have_properties() {
    let mut ui = Ui::new();
    ui.register_class::<FloatAnimation>("Animations");
    ui.register_class::<ColorAnimation>("Animations");
    let form = ui.add_root(boxy(100.0, 100.0));
    let anim = ui.create("ColorAnimation", Some(form)).unwrap();
    ui.set_prop(anim, "stop_value", "#FF0000").unwrap();
    ui.set_prop(anim, "trigger", "pressed").unwrap();
    ui.set_prop(anim, "easing", "out_back").unwrap();
    assert_eq!(ui.get_prop(anim, "stop_value").unwrap(), PropValue::Color(Color::rgb(255, 0, 0)));
    assert_eq!(ui.get_prop(anim, "trigger").unwrap(), PropValue::Enum("pressed".into()));
    assert_eq!(ui.class_name(anim), Some("ColorAnimation"));
}

#[test]
fn theme_transitions_interpolate_between_states() {
    let mut ui = Ui::new();
    ui.theme_mut().set_style(
        "Boxy",
        Style::new(StyleProps::new().fill("#000000").transition(100.0)).hover(StyleProps::new().fill("#FFFFFF")),
    );
    let form = ui.add_root(boxy(100.0, 100.0));
    let mut canvas = RecordingCanvas::default();

    ui.set_time(0.0);
    ui.paint(form, &mut canvas);
    assert!(!ui.is_animating());

    ui.control_mut(form).unwrap().state_mut().hover = true;
    ui.set_time(1.0);
    canvas.ops.clear();
    ui.paint(form, &mut canvas); // começa a transição
    assert!(ui.is_animating());

    ui.set_time(1.05);
    canvas.ops.clear();
    ui.paint(form, &mut canvas);
    let mid = match &canvas.ops[0] {
        common::Op::Fill { color, .. } => *color,
        other => panic!("{other:?}"),
    };
    assert!(mid.r > 100 && mid.r < 255, "cor intermediária: {mid}");

    ui.set_time(1.5);
    canvas.ops.clear();
    ui.paint(form, &mut canvas);
    assert_eq!(canvas.ops[0], common::Op::Fill { rect: RectF::new(0.0, 0.0, 100.0, 100.0), color: Color::WHITE });
    assert!(!ui.is_animating(), "transição terminou");
}
