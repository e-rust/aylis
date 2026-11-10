//! `AnimatedText` pintado em instantes certos: cada letra com a opacidade e a posição do
//! efeito, os efeitos encadeados (`after`) e o `on_finish` no fim.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{AnimatedText, Form};
use aylis_core::*;

/// Canvas que guarda cada texto desenhado: (texto, alfa, y da posição atual).
#[derive(Default)]
struct Probe {
    texts: Vec<(String, u8, f32)>,
    ty: f32,
    stack: Vec<f32>,
}

impl Canvas for Probe {
    fn save(&mut self) {
        self.stack.push(self.ty);
    }
    fn restore(&mut self) {
        self.ty = self.stack.pop().unwrap_or(0.0);
    }
    fn translate(&mut self, _: f32, dy: f32) {
        self.ty += dy;
    }
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, style: &TextStyle) {
        self.texts.push((text.to_string(), style.color.a, self.ty));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.5, style.font.size * 1.2)
    }
}

fn paint(ui: &mut Ui, form: ComponentId, t: f64) -> Vec<(String, u8, f32)> {
    ui.set_time(t);
    let mut p = Probe::default();
    ui.paint(form, &mut p);
    p.texts
}

#[test]
fn letters_follow_the_effects_over_time() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 400.0, 200.0));
    // Por letra, 100 ms entre elas: "ab" -> a de 0 a 1 s, b de 0,1 a 1,1 s. Depois, um slide.
    let text = ui.add(AnimatedText::new("ab", "fade duration=1000 stagger=100 easing=linear; slide dy=40 by=all duration=500 easing=linear after"), form);
    ui.control_mut(text).unwrap().set_rect(0.0, 0.0, 400.0, 100.0);
    let finished = Rc::new(RefCell::new(0));
    let f = finished.clone();
    ui.on(text, "on_finish", move |_, _, _| *f.borrow_mut() += 1);
    ui.layout();

    let start = paint(&mut ui, form, 10.0);
    assert!(start.iter().all(|(_, a, _)| *a == 0) || start.is_empty(), "no começo nada aparece: {start:?}");
    let half = paint(&mut ui, form, 10.5);
    let alpha = |texts: &[(String, u8, f32)], s: &str| texts.iter().find(|t| t.0 == s).map_or(0, |t| t.1);
    assert!((120..=135).contains(&alpha(&half, "a")), "a pela metade: {half:?}");
    assert!(alpha(&half, "b") < alpha(&half, "a"), "b começa depois");
    // O slide (after) começa em 1,1 s: em 1,35 s está pela metade (20 dos 40 pontos).
    let mid = paint(&mut ui, form, 11.35);
    let y = mid.iter().find(|t| t.0 == "a").unwrap().2;
    let base = paint(&mut ui, form, 12.0).iter().find(|t| t.0 == "a").unwrap().2;
    assert!(((y - base) - 20.0).abs() < 1.0, "slide pela metade: {} pontos", y - base);
    // Fim: tudo visível e o on_finish uma vez.
    let end = paint(&mut ui, form, 13.0);
    assert!(end.iter().all(|t| t.1 == 255), "{end:?}");
    ui.process_posted();
    paint(&mut ui, form, 14.0);
    ui.process_posted();
    assert_eq!(*finished.borrow(), 1);
    assert!(!AnimatedText::is_playing(&ui, text));
    // De novo, pelo invoke (o caminho das outras linguagens).
    ui.invoke(text, "play", &[]).unwrap();
    assert!(AnimatedText::is_playing(&ui, text));
}
