//! A escala de um controle e de tudo dentro dele: ele ocupa o
//! mesmo retângulo no pai, e o desenho e o layout dos filhos acontecem num espaço do tamanho
//! do retângulo dividido pela escala. O clique, o lugar no form e a pintura acompanham.

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Um form 400x300 com um painel em (50, 40) de 200x100 e escala 2: dentro dele, 100x50.
fn scaled_panel() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Escala", 400.0, 300.0));
    let panel = ui.add(Panel::default(), form);
    ui.control_mut(panel).unwrap().set_position(50.0, 40.0).set_size(200.0, 100.0);
    ui.set_prop(panel, "scale", 2.0).unwrap();
    ui.show(form).unwrap();
    (ui, form, panel)
}

fn click_at(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, click_count: 1, ..Default::default() };
    ui.pointer_move(form, e);
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
    ui.process_events();
}

#[test]
fn children_are_laid_out_inside_and_shown_scaled() {
    let (mut ui, _form, panel) = scaled_panel();
    let child = ui.add(Panel::default(), panel);
    ui.control_mut(child).unwrap().set_align(Align::Client);
    ui.update_layout();
    assert_eq!(ui.control(panel).unwrap().inner_size(), SizeF::new(100.0, 50.0));
    assert_eq!(ui.bounds(child), Some(RectF::new(0.0, 0.0, 100.0, 50.0)), "arrumado no espaço de dentro");
    assert_eq!(ui.absolute_rect(child), Some(RectF::new(50.0, 40.0, 200.0, 100.0)), "na tela, o dobro");
    assert_eq!(ui.get_prop(panel, "scale").unwrap(), PropValue::Float(2.0));
    // A escala volta ao normal: os filhos ocupam o retângulo inteiro.
    ui.set_prop(panel, "scale", 1.0).unwrap();
    ui.update_layout();
    assert_eq!(ui.bounds(child), Some(RectF::new(0.0, 0.0, 200.0, 100.0)));
}

#[test]
fn the_click_reaches_the_right_child_in_the_scaled_space() {
    let (mut ui, form, panel) = scaled_panel();
    let clicked = Rc::new(Cell::new(""));
    for (name, align) in [("esquerdo", Align::Left), ("direito", Align::Right)] {
        let b = ui.add(Button::new(name), panel);
        ui.control_mut(b).unwrap().set_align(align).set_width(40.0);
        let c = clicked.clone();
        ui.on(b, "on_click", move |_, _, _| c.set(name));
    }
    ui.update_layout();
    // Dentro: o direito ocupa x de 60 a 100; no form, de 50 + 120 a 50 + 200.
    click_at(&mut ui, form, 50.0 + 180.0, 40.0 + 50.0);
    assert_eq!(clicked.get(), "direito");
    click_at(&mut ui, form, 50.0 + 20.0, 40.0 + 50.0);
    assert_eq!(clicked.get(), "esquerdo");
    // No meio (dentro, x = 50): entre os dois, nenhum.
    clicked.set("");
    click_at(&mut ui, form, 50.0 + 100.0, 40.0 + 50.0);
    assert_eq!(clicked.get(), "");
    // O ponto do form nas coordenadas do botão.
    let right = ui.find("direito").or_else(|| ui.children(panel).last().copied()).unwrap();
    let local = ui.local_point(right, PointF::new(50.0 + 140.0, 40.0 + 20.0));
    assert!((local.x - 10.0).abs() < 0.01 && (local.y - 10.0).abs() < 0.01, "{local:?}");
}

#[test]
fn nested_scales_multiply() {
    let (mut ui, _form, panel) = scaled_panel();
    let inner = ui.add(Panel::default(), panel);
    ui.control_mut(inner).unwrap().set_position(10.0, 5.0).set_size(40.0, 20.0);
    ui.set_prop(inner, "scale", 0.5).unwrap();
    let leaf = ui.add(Panel::default(), inner);
    ui.control_mut(leaf).unwrap().set_align(Align::Client);
    ui.update_layout();
    // Dentro do de dentro: 80x40; na tela, 2 x 0,5 = o tamanho de antes da escala.
    assert_eq!(ui.bounds(leaf), Some(RectF::new(0.0, 0.0, 80.0, 40.0)));
    assert_eq!(ui.absolute_rect(leaf), Some(RectF::new(50.0 + 20.0, 40.0 + 10.0, 80.0, 40.0)));
    assert!((ui.inner_scale(leaf) - 1.0).abs() < 1e-6);
    assert!((ui.inner_scale(inner) - 1.0).abs() < 1e-6);
    assert!((ui.inner_scale(panel) - 2.0).abs() < 1e-6);
}

#[test]
fn the_auto_height_counts_the_scale() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Escala", 400.0, 300.0));
    let panel = ui.add(Panel::default(), form);
    ui.control_mut(panel).unwrap().set_align(Align::Top).set_auto_size(true).set_padding(0.0);
    let child = ui.add(Panel::default(), panel);
    ui.control_mut(child).unwrap().set_align(Align::Top).set_height(30.0);
    ui.show(form).unwrap();
    ui.update_layout();
    let plain = ui.bounds(panel).unwrap().height;
    ui.set_prop(panel, "scale", 2.0).unwrap();
    ui.update_layout();
    assert_eq!(ui.bounds(panel).unwrap().height, plain * 2.0, "o conteúdo medido dentro, vezes a escala");
}

/// O desenho do filho sai ampliado e no lugar certo: um quadrado de 10x10 em (10, 10) dentro
/// do painel com escala 2 ocupa 20x20 em (50 + 20, 40 + 20) no form.
#[test]
fn the_child_is_painted_scaled_in_place() {
    let (mut ui, form, panel) = scaled_panel();
    ui.set_prop(panel, "fill", "#FFFFFF").unwrap();
    let square = ui.add(Rectangle::default(), panel);
    ui.control_mut(square).unwrap().set_position(10.0, 10.0).set_size(10.0, 10.0);
    ui.set_prop(square, "fill", "#000000").unwrap();
    ui.set_prop(square, "border_width", 0.0).ok();
    ui.update_layout();
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let (w, h) = (400u32, 300u32);
    let mut pixels = vec![0u32; (w * h) as usize];
    renderer.render_area_to_pixels(&mut pixels, w, h, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
    let dark = |x: u32, y: u32| (pixels[(y * w + x) as usize] & 0xFF) < 64;
    assert!(dark(75, 65), "dentro do quadrado ampliado");
    assert!(dark(88, 78), "perto do canto de baixo, ainda dentro (até 90, 80)");
    assert!(!dark(93, 83), "fora do quadrado ampliado");
    assert!(!dark(66, 56), "antes do começo (70, 60)");
}
