//! Formas geométricas: o clique acerta a figura, não o retângulo em volta.

use aylis_controls::shapes::{Arc, Circle, Line, PathShape, Pie, Polygon, Star};
use aylis_controls::{Form, Panel};
use aylis_core::{ComponentId, PointF, Ui};

/// Um form com um painel no fundo e a forma por cima, em (100, 100) com 100 x 100.
fn scene(ui: &mut Ui, class: &str) -> (ComponentId, ComponentId, ComponentId) {
    aylis_controls::register(ui);
    let form = ui.add_root(Form::new("formas", 400.0, 400.0));
    let back = ui.add(Panel::new(400.0, 400.0), form);
    let shape = ui.create(class, Some(form)).unwrap();
    ui.control_mut(shape).unwrap().set_rect(100.0, 100.0, 100.0, 100.0);
    ui.layout();
    (form, back, shape)
}

fn at(ui: &Ui, form: ComponentId, x: f32, y: f32) -> Option<ComponentId> {
    ui.hit_test(form, PointF::new(x, y))
}

#[test]
fn a_circle_lets_the_corners_through() {
    let mut ui = Ui::new();
    let (form, back, circle) = scene(&mut ui, "Circle");
    assert_eq!(at(&ui, form, 150.0, 150.0), Some(circle), "centro");
    assert_eq!(at(&ui, form, 103.0, 103.0), Some(back), "canto: passa para o painel");
    assert_eq!(at(&ui, form, 198.0, 150.0), Some(circle), "borda");
    let _ = Circle::default();
}

#[test]
fn a_star_is_hit_on_the_points_not_between_them() {
    let mut ui = Ui::new();
    let (form, back, star) = scene(&mut ui, "Star");
    assert_eq!(at(&ui, form, 150.0, 104.0), Some(star), "a ponta de cima");
    assert_eq!(at(&ui, form, 150.0, 150.0), Some(star), "o miolo");
    // Entre as duas pontas de cima (36° e 72° do centro, longe): fora da estrela.
    assert_eq!(at(&ui, form, 180.0, 112.0), Some(back));
    let _ = Star::default();
}

#[test]
fn a_line_is_hit_close_to_the_stroke() {
    let mut ui = Ui::new();
    let (form, back, line) = scene(&mut ui, "Line");
    assert_eq!(at(&ui, form, 150.0, 150.0), Some(line), "na diagonal");
    assert_eq!(at(&ui, form, 152.0, 149.0), Some(line), "perto");
    assert_eq!(at(&ui, form, 180.0, 120.0), Some(back), "longe");
    let _ = Line::default();
}

#[test]
fn a_pie_misses_the_missing_slice() {
    let mut ui = Ui::new();
    // De 0° a 270°: falta o quarto de cima à esquerda.
    let (form, back, pie) = scene(&mut ui, "Pie");
    assert_eq!(at(&ui, form, 170.0, 130.0), Some(pie), "em cima à direita");
    assert_eq!(at(&ui, form, 130.0, 170.0), Some(pie), "embaixo à esquerda");
    assert_eq!(at(&ui, form, 125.0, 125.0), Some(back), "a fatia que falta");
    let _ = Pie::default();
}

#[test]
fn an_arc_is_only_the_stroke() {
    let mut ui = Ui::new();
    let (form, back, arc) = scene(&mut ui, "Arc");
    assert_eq!(at(&ui, form, 150.0, 150.0), Some(back), "o centro é vazio");
    assert_eq!(at(&ui, form, 150.0, 102.0), Some(arc), "no traço, em cima");
    let _ = Arc::default();
}

#[test]
fn polygons_from_sides_or_points_and_svg_paths() {
    let mut ui = Ui::new();
    let (form, back, poly) = scene(&mut ui, "Polygon");
    // Triângulo pelos pontos: em cima no meio, embaixo nos cantos.
    ui.set_prop(poly, "points", "0.5,0 1,1 0,1").unwrap();
    assert_eq!(at(&ui, form, 150.0, 190.0), Some(poly));
    assert_eq!(at(&ui, form, 105.0, 110.0), Some(back), "fora do triângulo");
    let _ = Polygon::default();

    let mut ui = Ui::new();
    let (form, back, path) = scene(&mut ui, "Path");
    // O coração padrão: o meio acerta, o canto de baixo não.
    assert_eq!(at(&ui, form, 150.0, 140.0), Some(path));
    assert_eq!(at(&ui, form, 104.0, 196.0), Some(back));
    let _ = PathShape::default();
}

#[test]
fn rotation_still_hits_the_figure() {
    let mut ui = Ui::new();
    let (form, back, line) = scene(&mut ui, "Line");
    ui.set_prop(line, "direction", "horizontal").unwrap();
    assert_eq!(at(&ui, form, 120.0, 150.0), Some(line));
    // Girada 90°: vira vertical.
    ui.set_prop(line, "rotation", 90.0).unwrap();
    ui.layout();
    assert_eq!(at(&ui, form, 120.0, 150.0), Some(back));
    assert_eq!(at(&ui, form, 150.0, 120.0), Some(line));
}
