//! O `Label` quebrado em linhas não passa da própria altura: com `line_height`, com
//! `max_lines` ou com os dois, as linhas param e o `trimming` põe "…" na última (o relatório
//! do player de música, 21/11: o título longo descia por cima dos controles de baixo).

use aylis_controls::{Form, Label};
use aylis_core::*;

/// Canvas que guarda cada linha desenhada e onde.
#[derive(Default)]
struct Probe {
    lines: Vec<(String, RectF)>,
}

impl Canvas for Probe {
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
    fn draw_text(&mut self, text: &str, rect: RectF, _: &TextStyle) {
        self.lines.push((text.to_string(), rect));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.5, style.font.size * 1.2)
    }
}

fn lines(props: &[(&str, PropValue)]) -> (Vec<(String, RectF)>, RectF) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 400.0, 400.0));
    let label = ui.add(Label::new("uma música com um nome bem comprido que não cabe em duas linhas de jeito nenhum"), form);
    ui.control_mut(label).unwrap().set_width(200.0).set_height(50.0);
    ui.set_prop(label, "wrap", true).unwrap();
    ui.set_prop(label, "trimming", "word").unwrap();
    ui.set_prop(label, "font_size", 20.0).unwrap();
    for (name, value) in props {
        ui.set_prop(label, name, value.clone()).unwrap();
    }
    ui.show(form).unwrap();
    ui.layout();
    let mut p = Probe::default();
    ui.paint(form, &mut p);
    (p.lines, ui.control(label).unwrap().bounds())
}

#[test]
fn the_wrapped_lines_stop_at_the_height_with_an_ellipsis() {
    // 50 de altura, linhas de 21 (20 × 1,05): duas linhas, a segunda com "…".
    let (drawn, rect) = lines(&[("line_height", PropValue::Float(1.05))]);
    assert_eq!(drawn.len(), 2, "{drawn:?}");
    assert!(drawn.iter().all(|(_, r)| r.bottom() <= rect.bottom() + 0.5), "{drawn:?}");
    assert!(drawn.last().unwrap().0.ends_with('…'), "{drawn:?}");
}

#[test]
fn max_lines_clamps_with_or_without_line_height() {
    let (one, _) = lines(&[("max_lines", PropValue::Int(1))]);
    assert_eq!(one.len(), 1, "{one:?}");
    assert!(one[0].0.ends_with('…'));
    let (both, _) = lines(&[("max_lines", PropValue::Int(1)), ("line_height", PropValue::Float(1.05))]);
    assert_eq!(both.len(), 1, "{both:?}");
}
