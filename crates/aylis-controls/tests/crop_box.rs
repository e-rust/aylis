//! O recorte por cima da prévia: arrastar canto, mover o miolo e o clique parado.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

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


fn drag(ui: &mut Ui, form: ComponentId, from: (f32, f32), to: (f32, f32)) {
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_move(form, at(from.0, from.1));
    ui.pointer_down(form, at(from.0, from.1));
    ui.pointer_move(form, at(to.0, to.1));
    ui.pointer_up(form, at(to.0, to.1));
}

fn events(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    ui.on(id, event, move |_, _, args| {
        l.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            _ => String::new(),
        })
    });
    log
}

#[test]
fn crop_box_resizes_moves_and_clicks() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 800.0, 600.0));
    // 1920×1080 em 384×216: 1 px de tela = 5 px da origem.
    let mut crop = CropBox::default();
    crop.base.set_rect(10.0, 10.0, 384.0, 216.0);
    crop.set_selection(192.0, 108.0, 960.0, 540.0);
    let id = ui.add(crop, form);
    ui.layout();
    let ends = events(&mut ui, id, "on_end");
    let clicks = events(&mut ui, id, "on_tap");
    // A seleção na tela: (48.4, 31.6) com 192×108; o canto de baixo à direita +20, +10.
    drag(&mut ui, form, (240.4, 139.6), (260.4, 149.6));
    assert_eq!(ui.get::<CropBox>(id).unwrap().selection(), (192.0, 108.0, 1060.0, 590.0));
    assert_eq!(*ends.borrow(), ["192 108 1060 590"]);
    // O miolo move (e para no limite da imagem).
    drag(&mut ui, form, (150.0, 80.0), (140.0, 70.0));
    assert_eq!(ui.get::<CropBox>(id).unwrap().selection(), (142.0, 58.0, 1060.0, 590.0));
    drag(&mut ui, form, (150.0, 80.0), (390.0, 220.0));
    let (x, y, w, h) = ui.get::<CropBox>(id).unwrap().selection();
    assert_eq!((x + w, y + h), (1920.0, 1080.0));
    // Clique parado no miolo: não mexe, avisa.
    let before = ui.get::<CropBox>(id).unwrap().selection();
    let mid = (10.0 + (x + w / 2.0) / 5.0, 10.0 + (y + h / 2.0) / 5.0);
    drag(&mut ui, form, mid, mid);
    assert_eq!(clicks.borrow().len(), 1);
    assert_eq!(ui.get::<CropBox>(id).unwrap().selection(), before);
    // O mínimo e os pares valem também pela propriedade.
    ui.set_prop(id, "sel_width", 7.0).unwrap();
    ui.set_prop(id, "min_size", 32.0).unwrap();
    assert_eq!(ui.get::<CropBox>(id).unwrap().selection().2, 32.0);
    assert!(!ui.get::<CropBox>(id).unwrap().is_full());
    ui.paint(form, &mut NullCanvas);
}

/// No visual `framed`, as alças ficam do lado de fora da seleção: com a seleção inteira, fora
/// do próprio controle. Elas pegam o mouse (o cursor de redimensionar) e arrastam; longe delas,
/// fora do controle, o clique vai para quem está embaixo.
#[test]
fn framed_handles_outside_the_box_resize() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 800.0, 600.0));
    let mut crop = CropBox::default();
    crop.framed = true;
    crop.base.set_rect(100.0, 100.0, 384.0, 216.0);
    let id = ui.add(crop, form);
    ui.layout();
    ui.paint(form, &mut NullCanvas);
    // A alça do canto de baixo à direita: 8 px por fora da seleção (que é o controle inteiro).
    let corner = (100.0 + 384.0 + 4.0, 100.0 + 216.0 + 4.0);
    assert_eq!(ui.hit_test(form, PointF::new(corner.0, corner.1)), Some(id), "a alça de fora pega o mouse");
    ui.pointer_move(form, PointerEvent { position: PointF::new(corner.0, corner.1), ..Default::default() });
    assert_eq!(ui.cursor(form), CursorShape::ResizeNwSe);
    // A da borda de cima, no meio.
    let top = (100.0 + 192.0, 100.0 - 4.0);
    ui.pointer_move(form, PointerEvent { position: PointF::new(top.0, top.1), ..Default::default() });
    assert_eq!(ui.cursor(form), CursorShape::ResizeVertical);
    // Longe das alças, fora do controle: não é dele.
    assert_ne!(ui.hit_test(form, PointF::new(100.0 + 192.0, 100.0 - 9.0 - 20.0)), Some(id));
    // Arrastar a alça do canto para dentro encolhe a seleção.
    drag(&mut ui, form, corner, (corner.0 - 96.0, corner.1 - 54.0));
    let sel = ui.get::<CropBox>(id).unwrap().selection();
    assert!(sel.2 < 1920.0 && sel.3 < 1080.0, "redimensionou: {sel:?}");
}
