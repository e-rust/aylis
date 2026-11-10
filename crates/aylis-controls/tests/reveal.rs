//! Animação ao aparecer na tela (`trigger: appear`, o "scroll reveal") e sequência de passos
//! (`keyframes`).

use aylis_controls::*;
use aylis_core::*;

fn float(ui: &Ui, id: ComponentId, prop: &str) -> f64 {
    match ui.get_prop(id, prop).unwrap() {
        PropValue::Float(v) => v,
        PropValue::Int(v) => v as f64,
        other => panic!("{other:?}"),
    }
}

#[test]
fn an_appear_animation_runs_once_when_the_control_scrolls_into_view() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let scroll = ui.add(ScrollBox::default(), form);
    ui.control_mut(scroll).unwrap().set_align(Align::Client);
    let card = ui.add(Panel::default(), scroll);
    ui.control_mut(card).unwrap().set_rect(10.0, 900.0, 200.0, 100.0).set_opacity(0.0);
    let reveal = ui.add(FloatAnimation(PropertyAnimation::float("opacity", 1.0).with_trigger(Trigger::Appear).with_duration(100.0)), card);
    ui.show(form).unwrap();
    ui.layout();
    ui.set_time(0.0);
    ui.tick();
    ui.set_time(1.0);
    ui.tick();
    assert_eq!(float(&ui, card, "opacity"), 0.0, "fora da tela, não roda");
    // Rola até o cartão.
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 800.0));
    ui.layout();
    ui.set_time(2.0);
    ui.tick();
    ui.set_time(2.2);
    ui.tick();
    assert_eq!(float(&ui, card, "opacity"), 1.0, "apareceu: animou");
    // Uma vez só: voltar e rolar de novo não repete.
    ui.set_prop(card, "opacity", 0.5).unwrap();
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 0.0));
    ui.layout();
    ui.set_time(3.0);
    ui.tick();
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 800.0));
    ui.layout();
    ui.set_time(4.0);
    ui.tick();
    ui.set_time(4.5);
    ui.tick();
    assert_eq!(float(&ui, card, "opacity"), 0.5);
    let _ = reveal;
}

#[test]
fn keyframes_go_through_each_step() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 400.0, 300.0));
    let b = ui.add(Button::new("ok"), form);
    let mut a = PropertyAnimation::float("opacity", 1.0).with_duration(1000.0).with_easing(Easing::Linear);
    a.keyframes = "0:0; 0.5:1; 1:0.2".into();
    let anim = ui.add(FloatAnimation(a), b);
    assert!(ui.set_prop(anim, "keyframes", "torto").is_err());
    ui.set_time(0.0);
    ui.start_animation(anim).unwrap();
    for (t, want) in [(0.25, 0.5), (0.5, 1.0), (0.75, 0.6), (1.5, 0.2)] {
        ui.set_time(t);
        ui.tick();
        assert!((float(&ui, b, "opacity") - want).abs() < 0.01, "em {t}s: {} (esperado {want})", float(&ui, b, "opacity"));
    }
}
