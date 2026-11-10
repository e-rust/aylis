//! As abas do exemplo: a aba arrancada abre outra instância do frmPrincipal (com os handlers
//! falando com ela), e a aba solta na faixa da outra janela volta, fechando a que ficou vazia.

use aylis::core::*;
use aylis::controls::TabStrip;

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

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn titles(ui: &Ui, strip: ComponentId) -> Vec<String> {
    ui.get::<TabStrip>(strip).unwrap().tabs().iter().map(|t| t.title.clone()).collect()
}

fn caption(ui: &Ui, form: ComponentId) -> String {
    match ui.get_prop(form, "caption").unwrap() {
        PropValue::String(s) => s,
        other => format!("{other:?}"),
    }
}

fn draw(ui: &mut Ui, form: ComponentId) {
    ui.layout();
    ui.paint(form, &mut NullCanvas);
}

#[test]
fn a_aba_vira_outra_janela_do_form_e_volta() {
    let mut app = abas::app().expect("o app abre");
    let ui = &mut app.ui;
    let main = ui.find_path("frmPrincipal").unwrap();
    ui.show(main).unwrap();
    let strip = ui.find_path("frmPrincipal.tbsAbas").unwrap();
    draw(ui, main);
    assert_eq!(titles(ui, strip), ["Aba 1", "Aba 2", "Aba 3"]);
    assert_eq!(caption(ui, main), "Aba 1 — Abas");

    // Puxa a Aba 1 para fora e solta longe de todas as faixas.
    ui.pointer_down(main, at(40.0, 20.0));
    ui.pointer_move(main, at(60.0, 20.0));
    ui.pointer_move(main, at(60.0, 400.0));
    ui.set_screen_pointer(Some(ScreenPointer { x: 1200, y: 500, scale: 1.0 }));
    ui.drag_over_form(main, None, PointF::default());
    ui.pointer_up(main, at(60.0, 400.0));

    assert_eq!(titles(ui, strip), ["Aba 2", "Aba 3"]);
    let other = ui.open_forms().into_iter().find(|&f| f != main).expect("a janela nova está aberta");
    // Outra instância do mesmo form: o nome livre (`frmPrincipal2`, como no protocolo), a classe a mesma.
    assert_eq!(ui.name(other), Some("frmPrincipal2"), "a segunda janela ganha um nome livre");
    assert_eq!(ui.form_class(other), Some("frmPrincipal"), "outra instância do mesmo form");
    let other_strip = ui.find_in(other, "tbsAbas").unwrap();
    assert_eq!(titles(ui, other_strip), ["Aba 1"], "só a aba arrancada (o on_show não criou as iniciais)");
    assert_eq!(caption(ui, other), "Aba 1 — Abas", "o handler fala com a instância nova");

    // O "+" da janela nova cria a aba lá.
    ui.fire(other_strip, "on_add", &EventArgs::None);
    assert_eq!(titles(ui, other_strip), ["Aba 1", "Aba 4"]);
    assert_eq!(caption(ui, other), "Aba 4 — Abas");
    assert_eq!(titles(ui, strip), ["Aba 2", "Aba 3"]);

    // As duas abas da janela nova voltam para a primeira; ela fecha ao ficar vazia.
    for _ in 0..2 {
        draw(ui, other);
        ui.pointer_down(other, at(40.0, 20.0));
        ui.pointer_move(other, at(60.0, 20.0));
        ui.pointer_move(other, at(60.0, 400.0));
        ui.drag_over_form(other, Some(main), PointF::new(990.0, 20.0));
        ui.pointer_up(other, at(60.0, 400.0));
    }
    assert_eq!(titles(ui, strip), ["Aba 2", "Aba 3", "Aba 1", "Aba 4"]);
    assert!(!ui.is_open(other), "a janela que ficou sem abas fecha");
}
