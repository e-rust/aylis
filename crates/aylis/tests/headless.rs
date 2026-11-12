//! O modo sem plataforma (`aylis::headless`): monta o form, simula o usuário e tira a foto sem
//! janela nem `platform::run` (o relatório do Tabernáculo, B04).

use aylis::core::{PropValue, Ui};
use aylis::headless::Headless;

const FORM: &str = r#"{"aylis": 1, "root": {"class": "Form", "name": "frmTeste", "width": 320, "height": 200, "children": [
    {"class": "Edit", "name": "edtNome", "left": 16, "top": 16, "width": 200, "height": 28},
    {"class": "Button", "name": "btnOk", "text": "OK", "left": 16, "top": 56, "width": 120, "height": 32},
    {"class": "Label", "name": "lblStatus", "text": "", "left": 16, "top": 100, "width": 280, "height": 24},
    {"class": "Timer", "name": "tmrPasso", "interval": 500, "enabled": true}
]}}"#;

fn text(h: &Headless, name: &str) -> String {
    match h.ui.get_prop(h.find(name).expect(name), "text") {
        Ok(PropValue::String(s)) => s.to_string(),
        other => panic!("{name}.text: {other:?}"),
    }
}

/// O tamanho gravado no cabeçalho do PNG (o IHDR).
fn png_size(png: &[u8]) -> (u32, u32) {
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n", "não é PNG");
    let be = |i: usize| u32::from_be_bytes([png[i], png[i + 1], png[i + 2], png[i + 3]]);
    (be(16), be(20))
}

#[test]
fn a_form_is_clicked_typed_and_photographed_without_a_window() {
    let mut h = Headless::new();
    let form = h.load_form(FORM).unwrap();
    let ok = h.find("btnOk").unwrap();
    let edit = h.find("edtNome").unwrap();
    let status = h.find("lblStatus").unwrap();
    // O clique de verdade (o mouse simulado no meio do botão) dispara o handler.
    h.ui.on(ok, "on_click", move |ui, _, _| {
        let nome = match ui.get_prop(edit, "text") {
            Ok(PropValue::String(s)) => s.to_string(),
            _ => String::new(),
        };
        let _ = ui.set_prop(status, "text", PropValue::String(format!("Olá, {nome}")));
    });
    let ticks = std::rc::Rc::new(std::cell::Cell::new(0));
    let counter = ticks.clone();
    let timer = h.find("tmrPasso").unwrap();
    h.ui.on(timer, "on_timer", move |_, _, _| counter.set(counter.get() + 1));

    let before = h.render_png().expect("o desenho fora da tela sem a plataforma");
    assert_eq!(png_size(&before), (320, 200));

    h.click("edtNome").unwrap();
    assert_eq!(h.ui.focused(form), Some(edit), "o clique no campo dá o foco");
    h.type_text("Ana");
    assert_eq!(text(&h, "edtNome"), "Ana");
    h.click("btnOk").unwrap();
    assert_eq!(text(&h, "lblStatus"), "Olá, Ana");

    // O relógio falso: o Timer de 500 ms dispara duas vezes em 1,1 s, sem esperar.
    h.advance(0.55);
    h.advance(0.55);
    assert_eq!(ticks.get(), 2);

    h.leave();
    let after = h.render_png().unwrap();
    assert_eq!(png_size(&after), (320, 200));
    assert_ne!(before, after, "a foto mostra o texto digitado e o rótulo");
    // Em alta densidade, o dobro de pixels.
    assert_eq!(png_size(&h.render_png_scaled(2.0).unwrap()), (640, 400));

    // A janela redimensionada: a foto sai no tamanho novo.
    h.resize(400.0, 240.0);
    assert_eq!(png_size(&h.render_png().unwrap()), (400, 240));
}

/// Desabilitado não clica (o mouse simulado passa pelo `enabled`, como na tela).
#[test]
fn a_disabled_button_is_not_clicked() {
    let mut h = Headless::new();
    h.load_form(FORM).unwrap();
    let ok = h.find("btnOk").unwrap();
    let clicked = std::rc::Rc::new(std::cell::Cell::new(false));
    let flag = clicked.clone();
    h.ui.on(ok, "on_click", move |_, _, _| flag.set(true));
    h.ui.set_prop(ok, "enabled", PropValue::Bool(false)).unwrap();
    h.click("btnOk").unwrap();
    assert!(!clicked.get());
    assert!(h.click("btnNaoExiste").is_err());
}

/// No Windows com o Direct2D (`--features d2d`): a mesma foto sai pelo D2D, com o texto medido
/// pelo DirectWrite (o padrão do app que compila só com o D2D).
#[cfg(all(windows, feature = "d2d"))]
#[test]
fn the_photo_also_comes_out_of_direct2d() {
    let mut ui = Ui::new();
    aylis::register_all(&mut ui);
    let mut h = Headless::from_ui(ui, aylis::platform::Renderer::D2d);
    h.load_form(FORM).unwrap();
    h.click("edtNome").unwrap();
    h.type_text("Ana");
    assert_eq!(png_size(&h.render_png().unwrap()), (320, 200));
    assert_eq!(png_size(&h.render_png_scaled(2.0).unwrap()), (640, 400));
}
