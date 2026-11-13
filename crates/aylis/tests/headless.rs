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

/// Um componente do app: uma barra do tamanho de um valor que vem de fora do `Ui` (um dado
/// compartilhado, como o que chega de outra thread). Mudar o valor não avisa ninguém: quem
/// muda tem de marcar o controle (`ui.damage`, ou `ctx.repaint()` dentro do componente).
struct Meter {
    base: aylis::core::ControlBase,
    value: std::rc::Rc<std::cell::Cell<u32>>,
}

impl aylis::core::Component for Meter {
    fn class_name(&self) -> &'static str {
        "Meter"
    }
    fn control(&self) -> Option<&aylis::core::ControlBase> {
        Some(&self.base)
    }
    fn control_mut(&mut self) -> Option<&mut aylis::core::ControlBase> {
        Some(&mut self.base)
    }
    fn paint(&self, canvas: &mut dyn aylis::core::Canvas, ctx: &aylis::core::PaintCtx<'_>) {
        let bar = aylis::core::RectF::new(ctx.rect.x, ctx.rect.y, (self.value.get() as f32 * 10.0).min(ctx.rect.width), ctx.rect.height);
        canvas.fill_rect(bar, 0.0, &aylis::core::Brush::Solid(ctx.theme.color("primary")));
    }
}

fn with_meter() -> (Headless, std::rc::Rc<std::cell::Cell<u32>>) {
    let mut h = Headless::new();
    let form = h.load_form(r#"{"aylis": 1, "root": {"class": "Form", "name": "frmMedidor", "width": 300, "height": 120}}"#).unwrap();
    h.set_form(form);
    let value = std::rc::Rc::new(std::cell::Cell::new(3));
    let meter = h.ui.add(Meter { base: aylis::core::ControlBase::new(200.0, 30.0), value: value.clone() }, form);
    h.ui.set_name(meter, "medidor").unwrap();
    h.settle();
    (h, value)
}

/// A conferência da pintura por parte para o componente do app: quem avisa passa; quem muda o
/// desenho sem avisar é apanhado (na janela, ficaria parado no desenho velho).
#[test]
fn a_component_is_checked_for_partial_paint_and_for_the_missing_repaint() {
    let (mut h, value) = with_meter();
    let report = h
        .check_partial_paint("medidor", |ui: &mut Ui, id| {
            value.set(12);
            ui.damage(id);
        })
        .expect("avisando, passa");
    assert_eq!(report.steps, 5);
    assert!(report.partial >= 1 && report.partial + report.whole <= report.steps, "{report:?}");

    let (mut h, value) = with_meter();
    let err = h
        .check_partial_paint("medidor", |_: &mut Ui, _| {
            value.set(12);
        })
        .expect_err("sem avisar, é apanhado")
        .to_string();
    assert!(err.contains("mudança") && err.contains("ctx.repaint()"), "{err}");
}

/// As classes do Aylis passam na mesma conferência (aqui, o botão com o texto mudado).
#[test]
fn an_aylis_button_passes_the_partial_paint_check() {
    let mut h = Headless::new();
    h.load_form(FORM).unwrap();
    h.check_partial_paint("btnOk", |ui: &mut Ui, id| {
        ui.set_prop(id, "text", "Salvar").unwrap();
    })
    .expect("o botão");
}

/// Dentro de um painel com `scale`, a área a repintar sai ampliada no lugar certo: pintar só a
/// área dá a mesma imagem que pintar tudo.
#[test]
fn a_button_inside_a_scaled_panel_passes_the_partial_paint_check() {
    let mut h = Headless::new();
    h.load_form(
        r#"{"aylis": 1, "root": {"class": "Form", "name": "frmEscala", "width": 400, "height": 240, "children": [
            {"class": "Panel", "name": "pnlZoom", "left": 20, "top": 20, "width": 300, "height": 180, "scale": 1.5, "children": [
                {"class": "Button", "name": "btnDentro", "text": "Dentro", "left": 10, "top": 10, "width": 120, "height": 32},
                {"class": "Edit", "name": "edtDentro", "left": 10, "top": 60, "width": 160, "height": 28}
            ]}
        ]}}"#,
    )
    .unwrap();
    h.check_partial_paint("btnDentro", |ui: &mut Ui, id| {
        ui.set_prop(id, "text", "Salvar").unwrap();
    })
    .expect("o botão ampliado");
    h.check_partial_paint("edtDentro", |ui: &mut Ui, id| {
        ui.set_prop(id, "text", "texto novo").unwrap();
    })
    .expect("o campo ampliado (o cursor de texto marca só uma parte)");
}
