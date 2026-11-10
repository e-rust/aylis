//! Os tipos de propriedade dos editores visuais: ângulo, duração, sombra e curva.

mod common;

use common::boxy;
use aylis_core::*;

#[test]
fn easing_parses_names_css_and_cubic_bezier() {
    assert_eq!(Easing::parse("out_quad"), Some(Easing::OutQuad));
    assert_eq!(Easing::parse("OutQuad"), Some(Easing::OutQuad));
    assert_eq!(Easing::parse("in-out-cubic"), Some(Easing::InOutCubic));
    assert_eq!(Easing::parse("ease-in"), Some(Easing::CubicBezier(0.42, 0.0, 1.0, 1.0)));
    assert_eq!(Easing::parse("cubic-bezier(0.2, 0, 0, 1)"), Some(Easing::CubicBezier(0.2, 0.0, 0.0, 1.0)));
    assert_eq!(Easing::parse("cubic-bezier(1, 2)"), None);
    assert_eq!(Easing::parse("rapido"), None);
    // Volta ao mesmo texto (as do CSS pelo nome).
    for spec in ["linear", "out_back", "ease", "ease-in-out", "cubic-bezier(0.2, 0, 0, 1)"] {
        assert_eq!(Easing::parse(spec).unwrap().to_spec(), spec);
    }
}

#[test]
fn cubic_bezier_follows_the_curve() {
    let linear = Easing::CubicBezier(0.0, 0.0, 1.0, 1.0);
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        assert!((linear.apply(t) - t).abs() < 1e-3, "{t}");
    }
    // ease-in começa devagar; ease-out, rápido; a "mola" passa de 1.
    let ease_in = Easing::parse("ease-in").unwrap();
    let ease_out = Easing::parse("ease-out").unwrap();
    assert!(ease_in.apply(0.3) < 0.15);
    assert!(ease_out.apply(0.3) > 0.4);
    assert!(Easing::parse("cubic-bezier(0.34, 1.56, 0.64, 1)").unwrap().apply(0.6) > 1.0);
    // As prontas, convertidas em Bézier, ficam perto da função original.
    for e in Easing::ALL {
        let [a, b, c, d] = e.to_bezier();
        let approx = Easing::CubicBezier(a, b, c, d);
        for i in 0..=20 {
            let t = i as f32 / 20.0;
            assert!((approx.apply(t) - e.apply(t)).abs() < 0.06, "{e:?} em {t}");
        }
    }
}

#[test]
fn shadow_spec_round_trip() {
    let s = Shadow::parse("0 4px 12 #00000040").unwrap();
    assert_eq!((s.offset.x, s.offset.y, s.blur, s.spread), (0.0, 4.0, 12.0, 0.0));
    assert_eq!(s.color, Color::rgba(0, 0, 0, 0x40));
    assert_eq!(s.to_spec(), "0 4 12 #00000040");
    let s = Shadow::parse("-2 3 6 1.5 #2563EB").unwrap();
    assert_eq!(s.to_spec(), "-2 3 6 1.5 #2563EB");
    assert_eq!(Shadow::parse("2 2").unwrap().blur, 0.0, "sem desfoque");
    assert!(Shadow::parse("2").is_none());
    assert!(Shadow::parse("1 2 3 #zz").is_none());
}

#[test]
fn every_control_has_rotation_and_shadow() {
    let mut ui = Ui::new();
    let f = ui.add_root(boxy(200.0, 200.0));
    let b = ui.add(boxy(40.0, 20.0), f);
    assert_eq!(ui.prop_info(b, "rotation").unwrap().kind, PropKind::Angle);
    assert_eq!(ui.prop_info(b, "shadow").unwrap().kind, PropKind::Shadow);
    ui.set_prop(b, "rotation", 450.0).unwrap();
    assert_eq!(ui.get_prop(b, "rotation").unwrap(), PropValue::Float(90.0));
    assert_eq!(ui.get_prop(b, "shadow").unwrap(), PropValue::Null);
    ui.set_prop(b, "shadow", "0 2 6 #00000033").unwrap();
    assert_eq!(ui.get_prop(b, "shadow").unwrap(), PropValue::String("0 2 6 #00000033".into()));
    assert!(ui.set_prop(b, "shadow", "grande").is_err());
    ui.set_prop(b, "shadow", "").unwrap();
    assert_eq!(ui.control(b).unwrap().shadow(), None);
}

/// Canvas que guarda os giros e as sombras.
#[derive(Default)]
struct Probe {
    rotations: Vec<f32>,
    shadows: Vec<(RectF, Shadow)>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, degrees: f32) {
        self.rotations.push(degrees);
    }
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.len() as f32 * 7.0, 14.0)
    }
    fn draw_shadow(&mut self, rect: RectF, _: f32, shadow: &Shadow) {
        self.shadows.push((rect, *shadow));
    }
}

#[test]
fn rotation_and_shadow_are_painted() {
    let mut ui = Ui::new();
    let f = ui.add_root(boxy(200.0, 200.0));
    let b = ui.add(boxy(40.0, 20.0), f);
    ui.set_prop(b, "rotation", 30.0).unwrap();
    ui.set_prop(b, "shadow", "1 2 3 #000000").unwrap();
    ui.layout();
    let mut p = Probe::default();
    ui.paint(f, &mut p);
    assert_eq!(p.rotations, vec![30.0]);
    assert_eq!(p.shadows.len(), 1);
    assert_eq!(p.shadows[0].0, RectF::new(0.0, 0.0, 40.0, 20.0));
    assert_eq!(p.shadows[0].1.blur, 3.0);
}

#[test]
fn animation_delay_waits_before_starting() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    ui.set_time(1.0);
    let options = AnimationOptions { delay_ms: 500.0, ..AnimationOptions::new(1000.0, Easing::Linear) };
    ui.animate(form, "opacity", 0.0, options).unwrap();
    ui.set_time(1.4);
    ui.tick();
    assert_eq!(ui.get_prop(form, "opacity").unwrap().to_f64().unwrap(), 1.0, "ainda esperando");
    ui.set_time(2.0);
    ui.tick();
    assert!((ui.get_prop(form, "opacity").unwrap().to_f64().unwrap() - 0.5).abs() < 1e-6);
}

#[test]
fn property_animation_publishes_timing_kinds() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let a = ui.add(PropertyAnimation::float("opacity", 0.0), form);
    assert_eq!(ui.prop_info(a, "duration").unwrap().kind, PropKind::Duration);
    assert_eq!(ui.prop_info(a, "delay").unwrap().kind, PropKind::Duration);
    assert_eq!(ui.prop_info(a, "easing").unwrap().kind, PropKind::Easing);
    ui.set_prop(a, "easing", "cubic-bezier(0.2, 0, 0, 1)").unwrap();
    assert_eq!(ui.get_prop(a, "easing").unwrap(), PropValue::String("cubic-bezier(0.2, 0, 0, 1)".into()));
    ui.set_prop(a, "easing", "in_out_quad").unwrap();
    assert_eq!(ui.get::<PropertyAnimation>(a).unwrap().easing, Easing::InOutQuad);
    assert!(ui.set_prop(a, "easing", "devagar").is_err());
    ui.set_prop(a, "delay", 120).unwrap();
    assert_eq!(ui.get::<PropertyAnimation>(a).unwrap().delay, 120.0);
}

#[derive(Default)]
struct Spin {
    base: ControlBase,
    spin: f32,
    time: u32,
}

impl Component for Spin {
    fn class_name(&self) -> &'static str {
        "Spin"
    }
    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        spin: f32 as Angle => "Appearance",
        time: u32 as Duration => "Behavior",
    }
}

#[test]
fn properties_macro_can_pick_the_editor_with_as() {
    let mut ui = Ui::new();
    let g = ui.add_root(Spin::default());
    assert_eq!(ui.prop_info(g, "spin").unwrap().kind, PropKind::Angle);
    assert_eq!(ui.prop_info(g, "time").unwrap().kind, PropKind::Duration);
    ui.set_prop(g, "time", 250.0).unwrap();
    assert_eq!(ui.get::<Spin>(g).unwrap().time, 250);
    ui.set_prop(g, "spin", 45).unwrap();
    assert_eq!(ui.get::<Spin>(g).unwrap().spin, 45.0);
}

#[test]
fn clicks_follow_the_rotation() {
    let mut ui = Ui::new();
    let f = ui.add_root(boxy(300.0, 300.0));
    // 100 × 20 em (50, 50); girado 90°, vira uma faixa em pé em volta do centro (100, 60).
    let b = ui.add(boxy(100.0, 20.0), f);
    ui.control_mut(b).unwrap().set_rect(50.0, 50.0, 100.0, 20.0);
    ui.set_prop(b, "rotation", 90.0).unwrap();
    ui.layout();
    assert_eq!(ui.hit_test(f, PointF::new(100.0, 20.0)), Some(b), "em cima do centro, dentro da faixa girada");
    assert_eq!(ui.hit_test(f, PointF::new(60.0, 60.0)), Some(f), "no retângulo sem giro, mas fora da faixa");
    // O ponto chega ao controle nas coordenadas dele: (100, 20) no form é a ponta esquerda.
    let local = ui.local_point(b, PointF::new(100.0, 20.0));
    assert!((local.x - 10.0).abs() < 1e-3 && (local.y - 10.0).abs() < 1e-3, "{local:?}");
}

#[test]
fn aspect_ratio_keeps_the_proportion() {
    let mut ui = Ui::new();
    let f = ui.add_root(boxy(400.0, 300.0));
    // Top: 16:9 pela largura (400 → 225).
    let top = ui.add(boxy(10.0, 10.0), f);
    ui.control_mut(top).unwrap().set_align(Align::Top).set_aspect_ratio(16.0 / 9.0);
    // Left: 1:2 pela altura que sobra (75 → 37,5).
    let left = ui.add(boxy(10.0, 10.0), f);
    ui.control_mut(left).unwrap().set_align(Align::Left).set_aspect_ratio(0.5);
    // Client: cabe no resto (362,5 × 75) mantendo 1:1, no meio.
    let client = ui.add(boxy(10.0, 10.0), f);
    ui.control_mut(client).unwrap().set_align(Align::Client).set_aspect_ratio(1.0);
    ui.layout();
    assert_eq!(ui.bounds(top).unwrap().size(), SizeF::new(400.0, 225.0));
    assert_eq!(ui.bounds(left).unwrap().size(), SizeF::new(37.5, 75.0));
    let c = ui.bounds(client).unwrap();
    assert_eq!(c.size(), SizeF::new(75.0, 75.0));
    assert!((c.x - (37.5 + (362.5 - 75.0) / 2.0)).abs() < 1e-3);
    assert_eq!(ui.prop_info(top, "aspect_ratio").unwrap().category, "Layout");
}
