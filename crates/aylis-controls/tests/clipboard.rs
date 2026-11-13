//! A área de transferência do Aylis: o que a plataforma não guarda fica no processo até outro
//! app copiar por cima; o `Clipboard` faz o mesmo pelo nome (os métodos do protocolo); o
//! PaintBox e o `Image` copiam e colam a imagem por ela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Form, ImageView, PaintCanvas};
use aylis_core::clipboard::{self, ClipboardContent};
use aylis_core::platform::ClipboardService;
use aylis_core::*;

/// Uma plataforma que só guarda texto (como o navegador sem permissão): o resto fica no
/// processo.
#[derive(Clone, Default)]
struct TextOnly(Rc<RefCell<String>>);

impl ClipboardService for TextOnly {
    fn get_text(&mut self) -> Option<String> {
        Some(self.0.borrow().clone())
    }
    fn set_text(&mut self, text: &str) {
        *self.0.borrow_mut() = text.to_string();
    }
}

fn scene() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("c", 400.0, 300.0));
    (ui, form)
}

fn red(ui: &Ui, pc: ComponentId) -> Color {
    ui.get::<PaintCanvas>(pc).unwrap().bitmap().get(0, 0).unwrap()
}

#[test]
fn without_a_platform_everything_stays_in_the_process() {
    let mut ui = Ui::new();
    let image = Image::rgba(1, 1, vec![1, 2, 3, 255]);
    clipboard::set(&mut ui, ClipboardContent::text("olá").with_html("<b>olá</b>").with_image(image.clone()).with_data("application/x-teste", b"123".to_vec()));
    assert_eq!(clipboard::text(&mut ui).as_deref(), Some("olá"));
    assert_eq!(clipboard::html(&mut ui).as_deref(), Some("<b>olá</b>"));
    assert_eq!(clipboard::image(&mut ui), Some(image), "a mesma imagem, sem conversão");
    assert_eq!(clipboard::data(&mut ui, "application/x-teste").as_deref(), Some(&b"123"[..]));
    assert!(clipboard::has(&mut ui, clipboard::IMAGE));
    assert!(!clipboard::has(&mut ui, "application/x-outro"));
    clipboard::clear(&mut ui);
    assert_eq!(clipboard::text(&mut ui), None);
    assert!(!clipboard::has(&mut ui, "application/x-teste"));
}

#[test]
fn what_the_platform_does_not_keep_is_lost_when_another_app_copies() {
    let mut ui = Ui::new();
    let board = TextOnly::default();
    ui.services_mut().clipboard = Some(Box::new(board.clone()));
    clipboard::set(&mut ui, ClipboardContent::text("registro 7").with_data("application/x-registro", b"{\"id\":7}".to_vec()));
    assert_eq!(*board.0.borrow(), "registro 7", "o texto foi para a plataforma");
    assert_eq!(clipboard::data(&mut ui, "application/x-registro").as_deref(), Some(&b"{\"id\":7}"[..]));
    // Outro app copiou: o formato do app não vale mais (colaria um registro velho).
    *board.0.borrow_mut() = "outra coisa".into();
    assert_eq!(clipboard::data(&mut ui, "application/x-registro"), None);
    assert_eq!(clipboard::text(&mut ui).as_deref(), Some("outra coisa"));
}

#[test]
fn the_clipboard_component_copies_and_pastes_by_name() {
    let (mut ui, form) = scene();
    let board = TextOnly::default();
    ui.services_mut().clipboard = Some(Box::new(board.clone()));
    let clip = ui.create("Clipboard", Some(form)).unwrap();
    let s = |t: &str| PropValue::String(t.into());

    assert_eq!(ui.invoke(clip, "copy_text", &[s("Olá")]).unwrap(), PropValue::Bool(true));
    assert_eq!(*board.0.borrow(), "Olá");
    assert_eq!(ui.invoke(clip, "paste_text", &[]).unwrap(), s("Olá"));

    ui.invoke(clip, "copy_html", &[s("<i>oi</i>"), s("oi")]).unwrap();
    assert_eq!(ui.invoke(clip, "paste_html", &[]).unwrap(), s("<i>oi</i>"));
    assert_eq!(ui.invoke(clip, "paste_text", &[]).unwrap(), s("oi"));
    assert_eq!(ui.invoke(clip, "has_format", &[s("text/html")]).unwrap(), PropValue::Bool(true));

    ui.invoke(clip, "copy_data", &[s("application/x-pedido+json"), s("{\"n\":3}"), s("Pedido 3")]).unwrap();
    assert_eq!(ui.invoke(clip, "paste_data", &[s("application/x-pedido+json")]).unwrap(), s("{\"n\":3}"));
    assert_eq!(*board.0.borrow(), "Pedido 3", "o texto vai junto para os outros apps");
    assert!(ui.invoke(clip, "copy_data", &[s(""), s("x"), s("")]).is_err(), "sem formato");

    ui.invoke(clip, "clear", &[]).unwrap();
    assert_eq!(ui.invoke(clip, "has_format", &[s("application/x-pedido+json")]).unwrap(), PropValue::Bool(false));
}

#[test]
fn images_go_between_image_views() {
    let (mut ui, form) = scene();
    let clip = ui.create("Clipboard", Some(form)).unwrap();
    let source = ui.add(ImageView::default(), form);
    let target = ui.add(ImageView::default(), form);
    let picture = Image::rgba(2, 1, vec![255, 0, 0, 255, 0, 0, 255, 255]);
    ui.get_mut::<ImageView>(source).unwrap().set_picture(Some(picture.clone()));
    assert_eq!(ui.invoke(clip, "copy_image", &[PropValue::Component(Some(source))]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.invoke(clip, "has_format", &[PropValue::String("image".into())]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.invoke(clip, "paste_image", &[PropValue::Component(Some(target))]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get::<ImageView>(target).unwrap().picture(), Some(&picture));
    // Só num `Image`.
    assert!(ui.invoke(clip, "paste_image", &[PropValue::Component(Some(form))]).is_err());
}

#[test]
fn the_paint_box_copies_to_another_paint_box_and_to_an_image() {
    let (mut ui, form) = scene();
    let add_canvas = |ui: &mut Ui| {
        let pc = ui.add(PaintCanvas::default(), form);
        ui.invoke(pc, "new", &[PropValue::Int(20), PropValue::Int(10)]).unwrap();
        pc
    };
    let (a, b) = (add_canvas(&mut ui), add_canvas(&mut ui));
    // A imagem nova sai na cor 2: vermelha.
    ui.set_prop(a, "back_color", Color::rgb(255, 0, 0)).unwrap();
    ui.invoke(a, "new", &[PropValue::Int(20), PropValue::Int(10)]).unwrap();
    ui.invoke(a, "select_all", &[]).unwrap();
    assert_eq!(ui.invoke(a, "copy", &[]).unwrap(), PropValue::Bool(true));
    let image = clipboard::image(&mut ui).expect("a seleção foi para a área de transferência");
    assert_eq!(image.known_size(), Some(SizeF::new(20.0, 10.0)));
    assert_eq!(ui.invoke(b, "paste", &[]).unwrap(), PropValue::Bool(true));
    ui.invoke(b, "deselect", &[]).unwrap();
    assert_eq!(red(&ui, b), Color::rgb(255, 0, 0), "colou no outro");
    // Uma imagem de fora (um arquivo PNG, como vem de um `Image`) também cola.
    let mut png = Vec::new();
    {
        use ::image::ImageEncoder;
        ::image::codecs::png::PngEncoder::new(&mut png).write_image(&[0, 0, 255, 255], 1, 1, ::image::ExtendedColorType::Rgba8).unwrap();
    }
    clipboard::set(&mut ui, ClipboardContent::image(Image::encoded(png)));
    assert_eq!(ui.invoke(b, "paste", &[]).unwrap(), PropValue::Bool(true));
    ui.invoke(b, "deselect", &[]).unwrap();
    assert_eq!(red(&ui, b), Color::rgb(0, 0, 255));
}
