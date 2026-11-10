//! A roda nas caixas de rolagem: o passo por dente vem do app (ou do controle) e o
//! deslize anda até o destino a cada quadro (`Ui::tick`).

use aylis_controls::{Form, ScrollBox};
use aylis_core::*;

fn setup(options: ScrollOptions) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_scroll_options(options);
    let form = ui.add_root(Form::new("t", 300.0, 300.0));
    let sb = ui.create("ScrollBox", Some(form)).unwrap();
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 300.0, 300.0);
    let inner = ui.create("Panel", Some(sb)).unwrap();
    ui.control_mut(inner).unwrap().set_align(Align::Top).set_height(2000.0);
    ui.layout();
    (ui, form, sb)
}

/// Um dente para baixo, como a plataforma manda (já com o passo do app).
fn notch(ui: &mut Ui, form: ComponentId) {
    let step = ui.scroll_options().wheel_step;
    ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -step), lines: true, ..Default::default() });
}

fn scroll(ui: &Ui, sb: ComponentId) -> f32 {
    ui.get::<ScrollBox>(sb).unwrap().scroll()
}

#[test]
fn notch_moves_the_configured_step() {
    let (mut ui, form, sb) = setup(ScrollOptions { wheel_step: 100.0, smooth: false, ..Default::default() });
    notch(&mut ui, form);
    assert_eq!(scroll(&ui, sb), 100.0);
    // O controle pode ter o próprio passo.
    ui.set_prop(sb, "wheel_step", 30.0).unwrap();
    notch(&mut ui, form);
    assert_eq!(scroll(&ui, sb), 130.0);
}

#[test]
fn smooth_glides_to_the_target() {
    let (mut ui, form, sb) = setup(ScrollOptions { wheel_step: 100.0, smooth: true, smooth_ms: 160.0 });
    ui.set_time(1.0);
    notch(&mut ui, form);
    notch(&mut ui, form);
    assert_eq!(scroll(&ui, sb), 0.0, "o deslize começa no quadro, não salta");
    ui.set_time(1.05);
    ui.tick();
    let mid = scroll(&ui, sb);
    assert!(mid > 0.0 && mid < 200.0, "no meio do caminho: {mid}");
    ui.set_time(1.3);
    ui.tick();
    assert_eq!(scroll(&ui, sb), 200.0, "dois dentes somam");
    // Desligado no controle, salta.
    ui.set_prop(sb, "smooth", "off").unwrap();
    notch(&mut ui, form);
    assert_eq!(scroll(&ui, sb), 300.0);
}

#[test]
fn the_default_is_a_browser_step_with_a_glide() {
    let options = ScrollOptions::default();
    assert_eq!(options.wheel_step, 100.0);
    assert!(options.smooth);
}

#[test]
fn the_glide_moves_in_fractions_and_lands_on_a_whole_point() {
    let (mut ui, form, sb) = setup(ScrollOptions { wheel_step: 100.0, smooth: true, smooth_ms: 160.0 });
    ui.set_time(1.0);
    notch(&mut ui, form);
    assert!(ui.is_animating(), "o deslize pede os quadros");
    ui.set_time(1.013);
    ui.tick();
    let mid = scroll(&ui, sb);
    assert!(mid > 0.0 && mid.fract() != 0.0, "no meio, sem arredondar: {mid}");
    ui.set_time(1.5);
    ui.tick();
    assert_eq!(scroll(&ui, sb), 100.0);
    assert!(!ui.is_animating(), "parado, não pede mais quadros");
}
