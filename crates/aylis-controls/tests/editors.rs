//! Os editores visuais (relógio de ângulo, sombra, curva e linha do tempo) com o mouse.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

#[derive(Default)]
struct Probe {
    texts: Vec<String>,
    brushes: Vec<Brush>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, brush: &Brush) {
        self.brushes.push(brush.clone());
    }
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * 7.0, 14.0)
    }
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 600.0, 500.0));
    (ui, f)
}

fn place(ui: &mut Ui, id: ComponentId, w: f32, h: f32) {
    ui.control_mut(id).unwrap().set_rect(0.0, 0.0, w, h);
    ui.layout();
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn drag(ui: &mut Ui, f: ComponentId, from: (f32, f32), to: (f32, f32)) {
    ui.pointer_move(f, at(from.0, from.1));
    ui.pointer_down(f, at(from.0, from.1));
    ui.pointer_move(f, at(to.0, to.1));
    ui.pointer_up(f, at(to.0, to.1));
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<EventArgs>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| c.borrow_mut().push(args.clone()));
    l
}

#[test]
fn angle_dial_follows_the_pointer_and_snaps_with_shift() {
    let (mut ui, f) = setup();
    let dial = ui.add(AngleDial::default(), f);
    place(&mut ui, dial, 100.0, 100.0);
    let commits = log(&mut ui, dial, "on_commit");
    // À direita do centro = 90°; embaixo = 180°.
    drag(&mut ui, f, (50.0, 10.0), (95.0, 50.0));
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 90.0);
    assert_eq!(commits.borrow().len(), 1);
    assert!(matches!(&commits.borrow()[0], EventArgs::Value(PropValue::Float(v)) if *v == 90.0));
    // Com Shift, de 15 em 15: 37° vira 30°... a 40° do topo dá 45°.
    let p = |deg: f32| {
        let a = deg.to_radians();
        (50.0 + a.sin() * 40.0, 50.0 - a.cos() * 40.0)
    };
    let (x, y) = p(40.0);
    let mut e = at(x, y);
    e.modifiers.shift = true;
    ui.pointer_move(f, e);
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 45.0);
    let (x, y) = p(37.0);
    drag(&mut ui, f, (x, y), (x, y));
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 37.0, "sem Shift, grau a grau");
    // Teclado: seta para a esquerda volta 1°.
    ui.set_focus(dial);
    ui.key_down(f, KeyEvent { key: Key::Left, ..Default::default() });
    assert_eq!(ui.get::<AngleDial>(dial).unwrap().angle, 36.0);
    assert_eq!(ui.prop_info(dial, "angle").unwrap().kind, PropKind::Angle);
}

#[test]
fn shadow_editor_drags_offset_blur_and_spread() {
    let (mut ui, f) = setup();
    let ed = ui.add(ShadowEditor::default(), f);
    place(&mut ui, ed, 340.0, 196.0);
    let changes = log(&mut ui, ed, "on_change");
    // O quadro (153 de lado em (4, 4), 45% da largura): o canto de baixo à direita é o
    // máximo (24, 24).
    drag(&mut ui, f, (80.0, 80.0), (190.0, 190.0));
    let s = ui.get::<ShadowEditor>(ed).unwrap().shadow();
    assert_eq!((s.offset.x, s.offset.y), (24.0, 24.0));
    // O centro do quadro volta ao zero.
    drag(&mut ui, f, (80.5, 80.5), (80.5, 80.5));
    assert_eq!(ui.get::<ShadowEditor>(ed).unwrap().offset_x, 0.0);
    // A faixa do desfoque, no fim, vai a 48 (o dobro do alcance).
    let blur_y = 196.0 - 4.0 - 38.0 * 2.0 + 27.0;
    drag(&mut ui, f, (300.0, blur_y), (339.0, blur_y));
    assert_eq!(ui.get::<ShadowEditor>(ed).unwrap().blur, 48.0);
    // O espalhamento começa no meio (zero) e vai de -24 a 24.
    let spread_y = blur_y + 38.0;
    drag(&mut ui, f, (220.0, spread_y), (0.0, spread_y));
    assert_eq!(ui.get::<ShadowEditor>(ed).unwrap().spread, -24.0);
    match changes.borrow().last().unwrap() {
        EventArgs::Text(t) => assert!(t.starts_with("0 0 48 -24 "), "{t}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn easing_editor_drags_the_nearest_handle() {
    let (mut ui, f) = setup();
    let ed = ui.add(EasingEditor::new(Easing::Linear), f);
    place(&mut ui, ed, 300.0, 360.0);
    let commits = log(&mut ui, ed, "on_commit");
    // Quadrado: lado = (360 - 16 - 34) / 1.6 = 193.75, x = 53.125, y = 8 + 58.125.
    let side = (360.0 - 16.0 - 34.0) / 1.6;
    let (ux, uy) = ((300.0 - side) / 2.0, 8.0 + side * 0.3);
    let screen = |x: f32, y: f32| (ux + x * side, uy + side - y * side);
    // O ponto 1 do linear está em (0, 0): leva para (0.2, 0.9).
    drag(&mut ui, f, screen(0.02, 0.02), screen(0.2, 0.9));
    let e = ui.get::<EasingEditor>(ed).unwrap();
    assert_eq!((e.x1, e.y1), (0.2, 0.9));
    assert_eq!((e.x2, e.y2), (1.0, 1.0), "o outro ponto fica");
    // O y pode passar de 1 (até a folga) e o x fica entre 0 e 1.
    drag(&mut ui, f, screen(0.98, 0.98), screen(1.4, 1.6));
    let e = ui.get::<EasingEditor>(ed).unwrap();
    assert_eq!((e.x2, e.y2), (1.0, 1.3));
    assert_eq!(e.easing(), Easing::CubicBezier(0.2, 0.9, 1.0, 1.3));
    assert!(matches!(commits.borrow().last(), Some(EventArgs::Text(t)) if t == "cubic-bezier(0.2, 0.9, 1, 1.3)"));
}

#[test]
fn timing_editor_drags_delay_duration_and_the_bar() {
    let (mut ui, f) = setup();
    let ed = ui.add(TimingEditor::default(), f);
    place(&mut ui, ed, 480.0, 170.0);
    let commits = log(&mut ui, ed, "on_commit");
    let x_of = |ui: &Ui, t: f32| {
        let e = ui.get::<TimingEditor>(ed).unwrap();
        12.0 + 456.0 * t / e.scale()
    };
    // Escala de 300 ms (sem atraso): 375 → passo 100 → 400.
    assert_eq!(ui.get::<TimingEditor>(ed).unwrap().scale(), 400.0);
    // Puxa o fim de 300 para 200 ms.
    let (from, to) = (x_of(&ui, 300.0), x_of(&ui, 200.0));
    drag(&mut ui, f, (from, 40.0), (to, 40.0));
    assert_eq!(ui.get::<TimingEditor>(ed).unwrap().duration, 200.0);
    // Puxa o começo para 50 ms: o fim fica (atraso 50, duração 150).
    let (from, to) = (x_of(&ui, 0.0), x_of(&ui, 50.0));
    drag(&mut ui, f, (from, 40.0), (to, 40.0));
    let e = ui.get::<TimingEditor>(ed).unwrap();
    assert_eq!((e.delay, e.duration), (50.0, 150.0));
    // Arrasta a barra 100 ms para a direita.
    let (from, to) = (x_of(&ui, 120.0), x_of(&ui, 220.0));
    drag(&mut ui, f, (from, 40.0), (to, 40.0));
    let e = ui.get::<TimingEditor>(ed).unwrap();
    assert_eq!((e.delay, e.duration), (150.0, 150.0));
    assert_eq!(commits.borrow().len(), 3);
    // O valor ao longo do tempo: nada antes do atraso, 1 no fim, 0 na volta.
    assert_eq!(e.value_at(100.0), None);
    assert_eq!(e.value_at(300.0), Some(1.0));
    let _ = ui.set_prop(ed, "auto_reverse", true);
    let e = ui.get::<TimingEditor>(ed).unwrap();
    assert!(e.value_at(450.0).unwrap().abs() < 1e-6);
    assert!((e.value_at(375.0).unwrap() - 0.75).abs() < 0.01, "na volta, a meio caminho");
}

#[test]
fn rectangle_gradient_follows_the_angle() {
    let (mut ui, f) = setup();
    let r = ui.add(Rectangle::new(100.0, 50.0), f);
    let _ = ui.set_prop(r, "fill", Color::rgb(255, 0, 0));
    let _ = ui.set_prop(r, "fill2", Color::rgb(0, 0, 255));
    let _ = ui.set_prop(r, "gradient_angle", 90.0);
    assert_eq!(ui.prop_info(r, "gradient_angle").unwrap().kind, PropKind::Angle);
    ui.layout();
    let mut p = Probe::default();
    ui.paint(f, &mut p);
    let gradient = p.brushes.iter().find_map(|b| match b {
        Brush::Linear { start, end, .. } => Some((*start, *end)),
        _ => None,
    });
    let (start, end) = gradient.expect("o retângulo pinta um gradiente");
    // 90° = da esquerda para a direita, na altura do meio.
    assert!((start.x - 0.0).abs() < 0.01 && (end.x - 100.0).abs() < 0.01, "{start:?} {end:?}");
    assert!((start.y - 25.0).abs() < 0.01 && (end.y - 25.0).abs() < 0.01);
}

#[test]
fn editors_paint_in_every_theme() {
    for theme in ["Windows 11 Light", "Material 3 Dark", "iOS 26 Light"] {
        let (mut ui, f) = setup();
        if let Some(t) = aylis_core::themes::builtin(theme) {
            ui.set_theme(t.clone());
        }
        for c in ["AngleDial", "ShadowEditor", "EasingEditor", "TimingEditor"] {
            ui.create(c, Some(f)).unwrap_or_else(|e| panic!("{c}: {e:?}"));
        }
        ui.layout();
        let mut p = Probe::default();
        ui.paint(f, &mut p);
        assert!(p.texts.iter().any(|t| t == "0°"), "{theme}: o valor do relógio");
        assert!(p.texts.iter().any(|t| t == "Desfoque"), "{theme}");
        assert!(p.texts.iter().any(|t| t == "out_quad"), "{theme}: a curva");
        assert!(p.texts.iter().any(|t| t.contains("duração 300 ms")), "{theme}: {:?}", p.texts);
    }
}
