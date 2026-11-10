//! EmojiPicker: busca em português e inglês, categorias e recentes.

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

#[test]
fn the_list_has_the_groups_and_names_in_portuguese() {
    assert_eq!(emoji_groups().len(), 9);
    let list = emoji_list();
    assert!(list.len() > 1500);
    let smile = list.iter().find(|e| e.emoji == "😀").unwrap();
    assert_eq!(smile.name, "rosto risonho");
    assert_eq!(smile.group, 0);
}

#[test]
fn search_and_pick_go_to_recent() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("e", 400.0, 400.0));
    let picker = ui.add(EmojiPicker::default(), form);
    ui.control_mut(picker).unwrap().set_align(Align::Client);
    let picked = Rc::new(RefCell::new(String::new()));
    {
        let p = picked.clone();
        ui.on(picker, "on_select", move |_, _, args| {
            if let EventArgs::Text(t) = args {
                *p.borrow_mut() = t.clone();
            }
        });
    }
    // Busca sem acento e em inglês acham o mesmo.
    let _ = ui.set_prop(picker, "filter", "coracao vermelho");
    let found = ui.get::<EmojiPicker>(picker).unwrap().sections();
    assert!(found[0].1.contains(&"❤️"), "{:?}", &found[0].1[..found[0].1.len().min(10)]);
    let _ = ui.set_prop(picker, "filter", "pizza");
    let found = ui.get::<EmojiPicker>(picker).unwrap().sections();
    assert_eq!(found[0].1, vec!["🍕"]);

    // Pintar monta as células; tocar na primeira escolhe o emoji.
    ui.update_layout();
    let mut canvas = NullCanvas;
    ui.paint(form, &mut canvas);
    let at = ui.absolute_rect(picker).unwrap();
    // A primeira célula: abaixo da busca (40) e da barra (40) e do título (26).
    let p = PointF::new(at.x + 4.0 + 22.0, at.y + 40.0 + 40.0 + 26.0 + 22.0);
    let e = PointerEvent { position: p, button: PointerButton::Primary, ..Default::default() };
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
    ui.process_posted();
    assert_eq!(*picked.borrow(), "🍕");
    assert_eq!(ui.get::<EmojiPicker>(picker).unwrap().recent, "🍕");
}

#[test]
fn map_projection_round_trips_and_parses_markers() {
    for (lat, lon) in [(-23.5505, -46.6333), (51.5, -0.12), (0.0, 0.0), (60.0, 150.0)] {
        let (x, y) = project(lat, lon, 12.0);
        let (la, lo) = unproject(x, y, 12.0);
        assert!((la - lat).abs() < 1e-9 && (lo - lon).abs() < 1e-9);
    }
    // Zoom 0: o mundo num bloco de 256; (0,0) no meio.
    assert_eq!(project(0.0, 0.0, 0.0), (128.0, 128.0));
    let mut m = MapView::default();
    m.markers = "-23.56,-46.65,MASP;bad;1,2".into();
    assert_eq!(m.marker_list(), vec![(-23.56, -46.65, "MASP".to_string()), (1.0, 2.0, String::new())]);
}
