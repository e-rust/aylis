//! O clique simulado (o `click` do protocolo, que o `app_command` usa) num Panel dispara o
//! `on_click` dele (relatório do InsightDB, 11/11: não disparava).

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::{Form, Panel};
use aylis_core::*;

#[test]
fn a_simulated_click_on_a_panel_fires_its_on_click() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmX", 400.0, 300.0));
    let _ = ui.set_name(form, "frmX");
    let panel = ui.add(Panel::new(100.0, 60.0), form);
    let _ = ui.set_name(panel, "pnlCartao");
    ui.set_prop(panel, "on_click", PropValue::Event(Some("pnlCartao_click".into()))).unwrap();
    let clicks = Rc::new(Cell::new(0));
    let seen = clicks.clone();
    ui.register_handler("frmX.pnlCartao_click", move |_, _, _| seen.set(seen.get() + 1));
    ui.show(form).unwrap();
    ui.layout();
    ui.click(panel);
    ui.process_events();
    assert_eq!(clicks.get(), 1);
}

/// O form dentro de um Frame recebe o `on_show` quando o form de fora aparece (relatório do
/// InsightDB, 11/11).
#[test]
fn the_form_inside_a_frame_gets_on_show() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let outer = ui.add_root(Form::new("frmPrincipal", 400.0, 300.0));
    let _ = ui.set_name(outer, "frmPrincipal");
    let frame = ui.add(aylis_controls::Frame::default(), outer);
    let inner = ui.add(Form::new("fraInicio", 200.0, 100.0), frame);
    let _ = ui.set_name(inner, "fraInicio");
    ui.set_name_scope(inner, true);
    ui.set_prop(inner, "on_show", PropValue::Event(Some("fraInicio_show".into()))).unwrap();
    let shown = Rc::new(Cell::new(0));
    let seen = shown.clone();
    ui.register_handler("fraInicio.fraInicio_show", move |_, _, _| seen.set(seen.get() + 1));
    ui.show(outer).unwrap();
    ui.process_events();
    assert_eq!(shown.get(), 1);
}
/// `auto_size` num Label encaixado à esquerda: a largura acompanha o texto (relatório do
/// InsightDB, 11/11: não encolhia).
#[test]
fn a_left_aligned_auto_size_label_fits_its_text() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().text = Some(Box::new(FixedText));
    let form = ui.add_root(Form::new("x", 400.0, 100.0));
    let label = ui.add(aylis_controls::Label::new("Total"), form);
    ui.control_mut(label).unwrap().set_align(Align::Left).set_width(200.0);
    let _ = ui.set_prop(label, "auto_size", true);
    ui.layout();
    let w = ui.bounds(label).unwrap().width;
    assert!(w < 200.0 && w >= 5.0 * 7.0, "a largura do texto (5 letras de 7 px), não os 200: {w}");
}

/// Medidas fixas de texto (7 px por letra), para o teste não depender de fonte instalada.
struct FixedText;
impl aylis_core::TextService for FixedText {
    fn measure(&self, text: &str, style: &aylis_core::TextStyle, _max_width: Option<f32>) -> aylis_core::SizeF {
        aylis_core::SizeF::new(text.chars().count() as f32 * 7.0, style.font.size * 1.2)
    }
}
/// `radius` com um valor por canto (e negativo, curvando para fora); `radius: 8` continua
/// valendo para os quatro (relatório do InsightDB, 11/11, L-28).
#[test]
fn the_radius_takes_one_value_per_corner() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 300.0, 200.0));
    let tab = ui.add(Panel::new(120.0, 40.0), form);
    ui.set_prop(tab, "radius", 8.0).unwrap();
    assert_eq!(ui.get_prop(tab, "radius").unwrap(), PropValue::Bounds(Bounds::all(8.0)));
    ui.set_prop(tab, "radius", "8, 8, -8, -8").unwrap();
    assert_eq!(ui.get_prop(tab, "radius").unwrap(), PropValue::Bounds(Bounds::new(8.0, 8.0, -8.0, -8.0)));
    // O contorno com os cantos de baixo para fora passa do retângulo dos dois lados.
    let path = aylis_core::Path::corners(RectF::new(10.0, 10.0, 120.0, 40.0), Bounds::new(8.0, 8.0, -8.0, -8.0));
    let b = path.bounds();
    assert!(b.x < 10.0 && b.right() > 130.0, "{b:?}");
    assert!((b.y - 10.0).abs() < 0.5, "em cima, dentro do retângulo: {b:?}");
    ui.set_prop(tab, "radius", PropValue::Null).unwrap();
    assert_eq!(ui.get_prop(tab, "radius").unwrap(), PropValue::Null, "vazio: o do tema");
}