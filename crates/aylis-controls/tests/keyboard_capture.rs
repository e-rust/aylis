//! A janela de vídeo igual no Windows e no Linux (24/11): o `KeyboardCapture`, o cursor a
//! partir de uma imagem e as travas do teclado fora de um evento.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Form, KeyboardCapture, Panel, Screen};
use aylis_core::*;

fn ui_with_form() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Vídeo", 400.0, 300.0));
    ui.set_name(form, "frmVideo").unwrap();
    (ui, form)
}

/// Desligado por padrão; ligado, o form pede a captura (e o Alt+F4 só com `alt_f4`); desligado
/// ou liberado, deixa de pedir.
#[test]
fn the_keyboard_capture_is_asked_only_while_active() {
    let (mut ui, form) = ui_with_form();
    let other = ui.add_root(Form::new("frmOutro", 200.0, 100.0));
    let capture = ui.add(KeyboardCapture::default(), form);
    assert_eq!(ui.get_prop(capture, "active").unwrap(), PropValue::Bool(false), "nasce desligado (mexe em segurança)");
    assert_eq!(ui.keyboard_capture(form), None);

    ui.set_prop(capture, "active", PropValue::Bool(true)).unwrap();
    assert_eq!(ui.keyboard_capture(form), Some(KeyboardCaptureRequest { alt_f4: false }));
    assert_eq!(ui.keyboard_capture(other), None, "só o form do componente");
    ui.set_prop(capture, "alt_f4", PropValue::Bool(true)).unwrap();
    assert_eq!(ui.keyboard_capture(form), Some(KeyboardCaptureRequest { alt_f4: true }));

    ui.set_prop(capture, "active", PropValue::Bool(false)).unwrap();
    assert_eq!(ui.keyboard_capture(form), None);

    ui.set_prop(capture, "active", PropValue::Bool(true)).unwrap();
    ui.free(capture);
    assert_eq!(ui.keyboard_capture(form), None, "liberado, solta");
}

/// No designer, o componente nunca pede (a IDE ficaria com o teclado preso).
#[test]
fn the_designer_never_captures_the_keyboard() {
    let (mut ui, form) = ui_with_form();
    ui.set_design_mode(true);
    let capture = ui.add(KeyboardCapture::default(), form);
    ui.set_prop(capture, "active", PropValue::Bool(true)).unwrap();
    assert_eq!(ui.keyboard_capture(form), None);
}

/// A plataforma avisa quando prendeu e soltou: o `on_capture_change` leva o estado, e o
/// `is_capturing()` diz o de agora.
#[test]
fn the_capture_change_reaches_the_app() {
    let (mut ui, form) = ui_with_form();
    let capture = ui.add(KeyboardCapture::default(), form);
    ui.set_name(capture, "kbcCaptura").unwrap();
    ui.set_prop(capture, "active", PropValue::Bool(true)).unwrap();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    ui.on(capture, "on_capture_change", move |_ui, _id, args| {
        if let EventArgs::Value(PropValue::Bool(on)) = args {
            log.borrow_mut().push(*on);
        }
    });
    assert_eq!(ui.invoke(capture, "is_capturing", &[]), Ok(PropValue::Bool(false)));
    ui.set_keyboard_captured(Some(form));
    assert_eq!(ui.invoke(capture, "is_capturing", &[]), Ok(PropValue::Bool(true)));
    ui.set_keyboard_captured(Some(form));
    ui.set_keyboard_captured(None);
    assert_eq!(*seen.borrow(), [true, false], "uma vez por mudança");
    assert_eq!(ui.invoke(capture, "is_capturing", &[]), Ok(PropValue::Bool(false)));
}

fn pixels(width: u32, height: u32, rgba: [u8; 4]) -> Vec<u8> {
    rgba.repeat((width * height) as usize)
}

/// A imagem posta num controle vale sob o ponteiro, e os filhos sem cursor próprio herdam; um
/// filho com `cursor` próprio fica com o dele.
#[test]
fn the_cursor_image_follows_the_control_under_the_pointer() {
    let (mut ui, form) = ui_with_form();
    let video = ui.add(Panel::new(200.0, 200.0), form);
    ui.control_mut(video).unwrap().set_rect(0.0, 0.0, 200.0, 200.0);
    let inner = ui.add(Panel::new(50.0, 50.0), video);
    ui.control_mut(inner).unwrap().set_rect(10.0, 10.0, 50.0, 50.0);
    let text = ui.add(Panel::new(50.0, 50.0), video);
    ui.control_mut(text).unwrap().set_rect(100.0, 100.0, 50.0, 50.0).set_cursor(CursorShape::Text);
    ui.layout();

    let image = Image::rgba(4, 3, pixels(4, 3, [255, 0, 0, 255]));
    ui.set_cursor_image(video, &image, 1, 2).unwrap();
    let at = |ui: &mut Ui, x: f32, y: f32| {
        ui.pointer_move(form, PointerEvent { position: PointF::new(x, y), ..Default::default() });
        ui.cursor_image(form)
    };
    let shown = at(&mut ui, 150.0, 20.0).expect("sobre o vídeo");
    assert_eq!((shown.width(), shown.height(), shown.hotspot()), (4, 3, (1, 2)));
    assert_eq!(&shown.rgba()[..4], &[255, 0, 0, 255]);
    assert_eq!(at(&mut ui, 20.0, 20.0).map(|c| c.id()), Some(shown.id()), "o filho sem cursor herda");
    assert!(at(&mut ui, 120.0, 120.0).is_none(), "o filho com cursor próprio fica com o dele");
    assert_eq!(ui.cursor(form), CursorShape::Text);
    assert!(at(&mut ui, 300.0, 250.0).is_none(), "fora do vídeo");

    ui.clear_cursor_image(video);
    assert!(at(&mut ui, 150.0, 20.0).is_none(), "tirada, volta o `cursor`");
}

/// A mesma imagem de novo reaproveita o id (o cursor do sistema não é recriado); o BGRA vira
/// RGBA; os erros dizem o que veio errado.
#[test]
fn the_cursor_image_is_reused_and_checked() {
    let (mut ui, form) = ui_with_form();
    let a = ui.add(Panel::new(10.0, 10.0), form);
    let b = ui.add(Panel::new(10.0, 10.0), form);
    let image = Image::rgba(2, 2, pixels(2, 2, [1, 2, 3, 4]));
    ui.set_cursor_image(a, &image, 0, 0).unwrap();
    ui.set_cursor_image(b, &Image::rgba(2, 2, pixels(2, 2, [1, 2, 3, 4])), 0, 0).unwrap();
    assert_eq!(ui.cursor_image_of(a).unwrap().id(), ui.cursor_image_of(b).unwrap().id(), "os mesmos pixels, o mesmo cursor");
    ui.set_cursor_image(b, &image, 1, 1).unwrap();
    assert_ne!(ui.cursor_image_of(a).unwrap().id(), ui.cursor_image_of(b).unwrap().id(), "outro ponto de clique, outro cursor");

    ui.set_cursor_image(a, &Image::bgra(1, 1, vec![10, 20, 30, 255]), 0, 0).unwrap();
    assert_eq!(ui.cursor_image_of(a).unwrap().rgba(), &[30, 20, 10, 255], "o BGRA convertido");

    assert!(ui.set_cursor_image(a, &image, 2, 0).is_err(), "o ponto fora da imagem");
    assert!(ui.set_cursor_image(a, &Image::rgba(2000, 1, pixels(2000, 1, [0; 4])), 0, 0).is_err(), "grande demais");
    assert!(ui.set_cursor_image(a, &Image::encoded(vec![0x89, b'P']), 0, 0).is_err(), "codificada");
}

/// Por nome (as outras linguagens): o `Screen.set_cursor_image` com os pixels em base64 e o
/// componente pelo caminho, e o `clear_cursor_image`.
#[test]
fn the_cursor_image_by_name() {
    let (mut ui, form) = ui_with_form();
    let video = ui.add(Panel::new(100.0, 100.0), form);
    ui.set_name(video, "pnlVideo").unwrap();
    let screen = ui.add(Screen::default(), form);
    let rgba = image::encode_base64(&pixels(2, 2, [9, 8, 7, 255]));
    let args = [PropValue::String("frmVideo.pnlVideo".into()), PropValue::String(rgba), PropValue::Int(2), PropValue::Int(2), PropValue::Int(1), PropValue::Int(0)];
    ui.invoke(screen, "set_cursor_image", &args).unwrap();
    let cursor = ui.cursor_image_of(video).expect("o cursor do painel");
    assert_eq!((cursor.width(), cursor.hotspot(), &cursor.rgba()[..4]), (2, (1, 0), &[9u8, 8, 7, 255][..]));
    // O tamanho que não bate com os pixels.
    let mut wrong = args.clone();
    wrong[2] = PropValue::Int(3);
    assert!(ui.invoke(screen, "set_cursor_image", &wrong).is_err());
    ui.invoke(screen, "clear_cursor_image", &[PropValue::String("frmVideo.pnlVideo".into())]).unwrap();
    assert!(ui.cursor_image_of(video).is_none());
}

/// As travas fora de um evento: sem plataforma, desligadas; com ela, o que ela diz, também
/// pelo `Screen.lock_keys` (o registro com os três).
#[test]
fn the_lock_keys_are_read_outside_a_key_event() {
    struct Locks;
    impl platform::DesktopService for Locks {
        fn lock_keys(&self) -> Option<LockKeys> {
            Some(LockKeys { caps_lock: false, num_lock: true, scroll_lock: true })
        }
    }
    let (mut ui, form) = ui_with_form();
    assert_eq!(ui.lock_keys(), LockKeys::default());
    ui.services_mut().desktop = Some(Box::new(Locks));
    assert_eq!(ui.lock_keys(), LockKeys { caps_lock: false, num_lock: true, scroll_lock: true });
    let screen = ui.add(Screen::default(), form);
    let json = ui.invoke(screen, "lock_keys", &[]).unwrap();
    assert_eq!(json, PropValue::String(r#"{"caps_lock":false,"num_lock":true,"scroll_lock":true}"#.into()));
    assert!(!Modifiers::default().scroll_lock);
}
