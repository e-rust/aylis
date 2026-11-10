//! Fontes nomeadas do projeto: o componente usa o nome (`font_family: "titulo"`) e desenha
//! e mede com a família dele; trocar a família do nome troca em todos.

use aylis_controls::{Form, Label};
use aylis_core::*;

#[derive(Default)]
struct Families(Vec<String>);

impl Canvas for Families {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, style: &TextStyle) {
        self.0.push(style.font.family.clone());
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.len() as f32 * style.font.size * 0.5, style.font.size)
    }
}

#[test]
fn components_follow_the_named_font() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 300.0, 100.0));
    let a = ui.add(Label::new("A"), form);
    let b = ui.add(Label::new("B"), form);
    ui.control_mut(a).unwrap().set_rect(0.0, 0.0, 100.0, 30.0);
    ui.control_mut(b).unwrap().set_rect(0.0, 40.0, 100.0, 30.0);
    ui.set_prop(a, "font_family", "titulo").unwrap();
    ui.set_prop(b, "font_family", "Arial").unwrap();
    ui.set_font_names(vec![("titulo".into(), "Montserrat".into())]);
    assert_eq!(ui.font_of(a).family, "Montserrat");
    assert_eq!(ui.font_of(b).family, "Arial", "uma família comum fica como está");
    let mut c = Families::default();
    ui.paint(form, &mut c);
    assert!(c.0.contains(&"Montserrat".to_string()) && c.0.contains(&"Arial".to_string()), "{:?}", c.0);
    // Trocar a família do nome troca em todos os que usam o nome.
    ui.set_font_names(vec![("titulo".into(), "Inter".into())]);
    assert_eq!(ui.font_of(a).family, "Inter");
    // O valor da propriedade continua o nome (é ele que vai para o form).
    assert_eq!(ui.get_prop(a, "font_family").unwrap(), PropValue::from("titulo"));
}
