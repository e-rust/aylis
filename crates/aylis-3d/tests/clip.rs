//! O evento de fim de clipe do `Model3D`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_3d::{Model, Model3D, sample};
use aylis_core::*;

/// Canvas que não desenha nada: o tempo da animação anda na pintura.
struct NullCanvas;

impl Canvas for NullCanvas {
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
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

/// Um `Model3D` pequeno com o tubo de exemplo; devolve o id, o primeiro clipe e a duração.
fn setup(ui: &mut Ui, looping: bool) -> (ComponentId, String, f64, Rc<RefCell<Vec<String>>>) {
    aylis_3d::register(ui);
    let model = Model::from_glb(&sample::tube_glb()).unwrap();
    let clip = model.clips[0].clone();
    let id = ui.add_root(Model3D::default().with_model(model));
    ui.control_mut(id).unwrap().set_rect(0.0, 0.0, 8.0, 8.0);
    ui.set_prop(id, "looping", looping).unwrap();
    ui.set_prop(id, "animation", clip.name.clone()).unwrap();
    let ended = Rc::new(RefCell::new(Vec::new()));
    let log = ended.clone();
    ui.on(id, "on_clip_end", move |_, _, args| {
        if let EventArgs::Text(name) = args {
            log.borrow_mut().push(name.clone());
        }
    });
    (id, clip.name, clip.duration as f64, ended)
}

fn frame(ui: &mut Ui, id: ComponentId, time: f64) {
    ui.set_time(time);
    ui.paint(id, &mut NullCanvas);
    ui.process_posted();
}

#[test]
fn a_clip_without_looping_warns_once_at_the_end_and_asking_again_plays_it_again() {
    let mut ui = Ui::new();
    let (id, name, d, ended) = setup(&mut ui, false);
    frame(&mut ui, id, 0.0);
    frame(&mut ui, id, d * 0.5);
    assert!(ended.borrow().is_empty());
    frame(&mut ui, id, d + 0.1);
    assert_eq!(*ended.borrow(), [name.clone()]);
    // Parado na última pose: não avisa de novo.
    frame(&mut ui, id, d + 1.0);
    frame(&mut ui, id, d + 2.0);
    assert_eq!(ended.borrow().len(), 1);
    let (_, t) = ui.get::<Model3D>(id).unwrap().clip_time().unwrap();
    assert!((t as f64 - d).abs() < 1e-4, "{t}");

    // Pedir o mesmo clipe de novo toca do começo.
    ui.set_prop(id, "animation", name.clone()).unwrap();
    frame(&mut ui, id, d + 2.0 + d * 0.5);
    assert_eq!(ended.borrow().len(), 1);
    frame(&mut ui, id, d + 2.0 + d + 0.1);
    assert_eq!(*ended.borrow(), [name.clone(), name]);
}

#[test]
fn a_looping_clip_warns_at_every_lap() {
    let mut ui = Ui::new();
    let (id, name, d, ended) = setup(&mut ui, true);
    frame(&mut ui, id, 0.0);
    for lap in 1..=3 {
        frame(&mut ui, id, d * (lap as f64 - 0.4));
        frame(&mut ui, id, d * lap as f64 + 0.05);
        assert_eq!(ended.borrow().len(), lap, "volta {lap}");
    }
    assert!(ended.borrow().iter().all(|n| *n == name));
}
