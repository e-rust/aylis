//! Arrastar e soltar entre controles: começa só depois de alguns pontos e só se o
//! `on_drag_start` pedir, destaca o destino que aceita, entrega ao soltar e Esc cancela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::*;

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn panel(ui: &mut Ui, form: ComponentId, x: f32) -> ComponentId {
    let p = ui.create("Panel", Some(form)).unwrap();
    ui.control_mut(p).unwrap().set_rect(x, 10.0, 100.0, 100.0);
    p
}

fn log(ui: &mut Ui, id: ComponentId, event: &'static str, out: &Rc<RefCell<Vec<String>>>) {
    let out = out.clone();
    ui.on(id, event, move |_, _, args| {
        let text = match args {
            EventArgs::Text(t) => t.clone(),
            EventArgs::Value(v) => format!("{v:?}"),
            _ => String::new(),
        };
        out.borrow_mut().push(format!("{event} {text}"));
    });
}

#[test]
fn drags_between_controls() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 600.0, 300.0));
    let source = panel(&mut ui, form, 10.0);
    let target = panel(&mut ui, form, 200.0);
    let refuses = panel(&mut ui, form, 400.0);
    ui.control_mut(source).unwrap().set_draggable(true);
    ui.control_mut(target).unwrap().set_accepts_drops(true);
    ui.control_mut(refuses).unwrap().set_accepts_drops(true);
    ui.layout();
    let events = Rc::new(RefCell::new(Vec::new()));
    ui.on(source, "on_drag_start", |ui, _, _| ui.begin_drag("rec-1", "Gravação 1"));
    log(&mut ui, source, "on_drag_end", &events);
    log(&mut ui, source, "on_click", &events);
    log(&mut ui, target, "on_drag_over", &events);
    log(&mut ui, target, "on_drag_drop", &events);
    ui.on(refuses, "on_drag_over", |ui, _, _| ui.reject_drop());
    log(&mut ui, refuses, "on_drag_drop", &events);

    // Um tremido de 3 pontos ainda é clique.
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(53.0, 50.0));
    assert!(ui.drag_view(form).is_none());
    ui.pointer_up(form, at(53.0, 50.0));
    assert_eq!(events.borrow().as_slice(), ["on_click "]);
    events.borrow_mut().clear();

    // Arrasto até o destino: destacado e entregue; a origem não recebe clique.
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(120.0, 50.0));
    assert_eq!(ui.drag_data(form), Some("rec-1"));
    assert_eq!(ui.drag_view(form).map(|v| v.2), Some(None), "fora de um destino");
    ui.pointer_move(form, at(250.0, 50.0));
    assert_eq!(ui.drag_view(form).map(|v| v.2), Some(Some(target)));
    ui.pointer_up(form, at(250.0, 50.0));
    assert_eq!(events.borrow().as_slice(), ["on_drag_over rec-1", "on_drag_drop rec-1", "on_drag_end Bool(true)"]);
    assert!(ui.drag_view(form).is_none());
    events.borrow_mut().clear();

    // O destino que recusa não recebe nada.
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(450.0, 50.0));
    assert_eq!(ui.drag_view(form).map(|v| v.2), Some(None));
    ui.pointer_up(form, at(450.0, 50.0));
    assert_eq!(events.borrow().as_slice(), ["on_drag_end Bool(false)"]);
    events.borrow_mut().clear();

    // Esc cancela no meio.
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(250.0, 50.0));
    ui.key_down(form, KeyEvent { key: Key::Escape, ..Default::default() });
    ui.pointer_up(form, at(250.0, 50.0));
    assert_eq!(events.borrow().as_slice(), ["on_drag_over rec-1", "on_drag_end Bool(false)"]);
    events.borrow_mut().clear();

    // Sem `begin_drag` no `on_drag_start`, nada de arrasto.
    ui.on(source, "on_drag_start", |_, _, _| {});
    let plain = panel(&mut ui, form, 10.0);
    ui.control_mut(plain).unwrap().set_rect(10.0, 150.0, 100.0, 100.0);
    ui.layout();
    ui.pointer_down(form, at(50.0, 200.0));
    ui.pointer_move(form, at(250.0, 50.0));
    assert!(ui.drag_view(form).is_none(), "não é arrastável");
}

#[test]
fn declared_drag_needs_no_handler_and_drop_types_decide() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 600.0, 300.0));
    let source = panel(&mut ui, form, 10.0);
    let clients = panel(&mut ui, form, 200.0);
    let products = panel(&mut ui, form, 400.0);
    ui.control_mut(source).unwrap().set_draggable(true);
    ui.set_prop(source, "drag_data", "cliente:123").unwrap();
    ui.set_prop(source, "drag_label", "Maria").unwrap();
    ui.set_prop(clients, "drop_types", "cliente").unwrap();
    ui.set_prop(products, "drop_types", "produto").unwrap();
    ui.layout();
    let events = Rc::new(RefCell::new(Vec::new()));
    log(&mut ui, clients, "on_drag_drop", &events);
    log(&mut ui, products, "on_drag_drop", &events);

    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(450.0, 50.0));
    assert_eq!(ui.drag_view(form).map(|v| (v.0, v.2)), Some(("Maria".to_string(), None)), "produto recusa sem perguntar");
    ui.pointer_move(form, at(250.0, 50.0));
    assert_eq!(ui.drag_view(form).and_then(|v| v.2), Some(clients));
    ui.pointer_up(form, at(250.0, 50.0));
    assert_eq!(events.borrow().as_slice(), ["on_drag_drop cliente:123"]);
}

#[test]
fn a_remote_drag_over_keeps_the_target_neutral_until_the_answer() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 600.0, 300.0));
    let source = panel(&mut ui, form, 10.0);
    let target = panel(&mut ui, form, 200.0);
    ui.control_mut(source).unwrap().set_draggable(true);
    ui.control_mut(target).unwrap().set_accepts_drops(true);
    ui.set_prop(source, "drag_data", "rec-1").unwrap();
    ui.set_prop(target, "on_drag_over", PropValue::Event(Some("sobre".into()))).unwrap();
    ui.layout();
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log2 = asked.clone();
    ui.set_event_sink(move |ui, _, _, _, _| {
        if ui.asking() {
            log2.borrow_mut().push(ui.defer_question());
        }
    });
    let events = Rc::new(RefCell::new(Vec::new()));
    log(&mut ui, target, "on_drag_drop", &events);
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(250.0, 50.0));
    assert_eq!(ui.drag_view(form).and_then(|v| v.2), None, "neutro até a resposta");
    // Soltou antes da resposta: solta quando ela chegar.
    ui.pointer_up(form, at(250.0, 50.0));
    assert!(events.borrow().is_empty());
    let token = asked.borrow()[0];
    ui.answer(token, Answer::Accepted(vec![]));
    assert_eq!(events.borrow().as_slice(), ["on_drag_drop rec-1"]);
}

/// O que acompanha o cursor: o `DragImage` que o controle aponta em `drag_image` (pelo
/// nome, como no form) ou o que o `on_drag_start` pede com `set_drag_image`.
#[test]
fn the_drag_image_follows_the_cursor() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 600.0, 300.0));
    let source = panel(&mut ui, form, 10.0);
    let image = ui.create("DragImage", Some(form)).unwrap();
    let _ = ui.set_name(image, "dimVideo");
    let thumb = Image::rgba(4, 4, vec![255; 64]);
    ui.get_mut::<aylis_controls::DragImage>(image).unwrap().set_picture(Some(thumb.clone()));
    let _ = ui.set_prop(image, "width", 120.0);
    let _ = ui.set_prop(image, "height", 68.0);
    let _ = ui.set_prop(source, "draggable", true);
    let _ = ui.set_prop(source, "drag_data", "rec-1");
    let _ = ui.set_prop(source, "drag_image", "dimVideo");
    assert_eq!(ui.get_prop(source, "drag_image").unwrap(), PropValue::Component(Some(image)));
    ui.layout();

    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(120.0, 60.0));
    let (visual, position) = ui.drag_visual(form).expect("a imagem vai junto");
    assert_eq!(visual.image, thumb);
    assert_eq!(visual.size, SizeF::new(120.0, 68.0));
    assert_eq!(position, PointF::new(120.0, 60.0), "no cursor");
    ui.pointer_up(form, at(120.0, 60.0));
    assert!(ui.drag_visual(form).is_none(), "acabou com o arrasto");

    // O handler troca a imagem só deste arrasto.
    let other = Image::rgba(2, 2, vec![0; 16]);
    let other2 = other.clone();
    ui.on(source, "on_drag_start", move |ui, _, _| {
        ui.begin_drag("rec-1", "Gravação 1");
        ui.set_drag_image(Some(DragVisual::new(other2.clone(), SizeF::new(80.0, 45.0))));
    });
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(120.0, 60.0));
    assert_eq!(ui.drag_visual(form).map(|(v, _)| v.image), Some(other));
    ui.pointer_up(form, at(120.0, 60.0));

    // Liberado o DragImage, a referência sai (o arrasto segue sem imagem).
    ui.free(image);
    assert_eq!(ui.get_prop(source, "drag_image").unwrap(), PropValue::Component(None));
}

/// Desenho fora da tela de mentira: devolve uns bytes fixos.
struct FakeOffscreen;

impl platform::OffscreenService for FakeOffscreen {
    fn render_png(&self, _w: u32, _h: u32, _scale: f32, _bg: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        struct Nothing;
        impl Canvas for Nothing {
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
            fn measure_text(&mut self, _: &str, _: &TextStyle) -> SizeF {
                SizeF::new(0.0, 0.0)
            }
            fn fill_path(&mut self, _: &Path, _: &Brush) {}
            fn stroke_path(&mut self, _: &Path, _: &Stroke) {}
        }
        paint(&mut Nothing);
        Some(vec![7, 7, 7])
    }
}

/// Sem imagem, o `DragImage` leva o desenho do próprio controle arrastado (uma pasta, uma
/// linha), do tamanho dele; com `control`, o desse controle.
#[test]
fn without_an_image_the_drag_carries_the_control_itself() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.services_mut().offscreen = Some(Box::new(FakeOffscreen));
    let form = ui.add_root(Form::new("t", 600.0, 300.0));
    let folder = panel(&mut ui, form, 10.0);
    let image = ui.create("DragImage", Some(form)).unwrap();
    let _ = ui.set_name(image, "dimFolder");
    let _ = ui.set_prop(folder, "draggable", true);
    let _ = ui.set_prop(folder, "drag_data", "pasta:1");
    let _ = ui.set_prop(folder, "drag_image", "dimFolder");
    ui.layout();
    ui.pointer_down(form, at(50.0, 50.0));
    ui.pointer_move(form, at(120.0, 60.0));
    let (visual, _) = ui.drag_visual(form).expect("o desenho vai junto");
    assert_eq!(visual.image.source(), &ImageSource::Encoded(vec![7, 7, 7]));
    assert_eq!(visual.size, SizeF::new(100.0, 100.0), "do tamanho do controle");
    ui.pointer_up(form, at(120.0, 60.0));
}
