//! O `SeekBar` com `show_labels`: o tempo atual e o total ao lado da barra, e o clique na
//! barra encurtada continua certo (o relatório do player de música, 21/11).

use aylis_controls::{Form, SeekBar};
use aylis_core::*;

#[derive(Default)]
struct Probe {
    texts: Vec<String>,
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
    fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.5, style.font.size * 1.2)
    }
}

#[test]
fn the_labels_show_the_position_and_the_duration() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 400.0, 100.0));
    let bar = ui.add(SeekBar::default(), form);
    ui.set_prop(bar, "duration", 180.0).unwrap();
    ui.set_prop(bar, "position", 65.0).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    let mut p = Probe::default();
    ui.paint(form, &mut p);
    assert!(!p.texts.iter().any(|t| t == "3:00"), "desligado: sem rótulos");
    ui.set_prop(bar, "show_labels", true).unwrap();
    let mut p = Probe::default();
    ui.paint(form, &mut p);
    assert!(p.texts.iter().any(|t| t == "1:05") && p.texts.iter().any(|t| t == "3:00"), "{:?}", p.texts);
    // O meio da barra (entre os rótulos) ainda é a metade da música.
    let r = ui.control(bar).unwrap().bounds();
    let ev = PointerEvent { position: PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0), ..Default::default() };
    ui.pointer_down(form, ev);
    ui.pointer_up(form, ev);
    let pos = ui.get_prop(bar, "position").unwrap().to_f64().unwrap();
    assert!((pos - 90.0).abs() < 1.0, "{pos}");
}
