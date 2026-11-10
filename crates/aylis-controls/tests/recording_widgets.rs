//! Controles de gravadores: o botão de gravar, as telas para escolher, os avisos
//! empilhados e o medidor estéreo.

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

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 800.0, 600.0));
    (ui, form)
}

fn click_at(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_move(form, e);
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
}

fn clicks(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
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
fn record_button_clicks_unless_finalizing() {
    let (mut ui, form) = setup();
    let b = ui.create("RecordButton", Some(form)).unwrap();
    ui.control_mut(b).unwrap().set_rect(10.0, 10.0, 72.0, 72.0);
    ui.layout();
    let log = clicks(&mut ui, b, "on_click");
    click_at(&mut ui, form, 46.0, 46.0);
    assert_eq!(log.borrow().len(), 1);
    ui.set_prop(b, "finalizing", true).unwrap();
    click_at(&mut ui, form, 46.0, 46.0);
    assert_eq!(log.borrow().len(), 1, "fechando o arquivo: o clique não vale");
    // Gravando, com contagem: desenha sem erro.
    for (name, value) in [("finalizing", PropValue::Bool(false)), ("recording", PropValue::Bool(true)), ("countdown", PropValue::Int(3))] {
        ui.set_prop(b, name, value).unwrap();
    }
    ui.paint(form, &mut NullCanvas);
}

#[test]
fn screen_layout_toggles_screens_unless_locked() {
    let (mut ui, form) = setup();
    let mut layout = ScreenLayout::default();
    layout.base.set_rect(0.0, 0.0, 416.0, 250.0);
    // Dois monitores lado a lado de 1920×1080: escala 400/3840, 8 px de folga.
    layout.set_screens(vec![
        ScreenItem { id: "a".into(), name: "A".into(), width: 1920, height: 1080, enabled: true, ..Default::default() },
        ScreenItem { id: "b".into(), name: "B".into(), x: 1920, width: 1920, height: 1080, ..Default::default() },
    ]);
    let id = ui.add(layout, form);
    ui.layout();
    let log = clicks(&mut ui, id, "on_change");
    click_at(&mut ui, form, 300.0, 125.0);
    assert_eq!(*log.borrow(), ["b"]);
    assert!(ui.get::<ScreenLayout>(id).unwrap().is_enabled("b"));
    click_at(&mut ui, form, 100.0, 125.0);
    assert!(!ui.get::<ScreenLayout>(id).unwrap().is_enabled("a"));
    ui.set_prop(id, "locked", true).unwrap();
    click_at(&mut ui, form, 100.0, 125.0);
    assert!(!ui.get::<ScreenLayout>(id).unwrap().is_enabled("a"), "travado (gravando)");
    ui.paint(form, &mut NullCanvas);
}

#[test]
fn notify_stacks_renews_by_title_and_expires() {
    let (mut ui, form) = setup();
    let stack = notify(&mut ui, "Dispositivos atualizados", "Microfone adicionado", ToastKind::Warning, 1000).unwrap();
    notify(&mut ui, "Dispositivos atualizados", "Saída removida", ToastKind::Warning, 1000);
    notify(&mut ui, "Erro", "Falhou", ToastKind::Error, 5000);
    assert_eq!(ui.get::<ToastStack>(stack).unwrap().count(), 2, "o mesmo título só renova");
    ui.paint(form, &mut NullCanvas);
    ui.set_time(1.5);
    ui.paint(form, &mut NullCanvas);
    assert_eq!(ui.get::<ToastStack>(stack).unwrap().count(), 1, "o primeiro venceu");
    // Depois da saída do vencido, o clique no cartão que sobrou (o de cima) fecha.
    ui.set_time(2.0);
    ui.paint(form, &mut NullCanvas);
    let card = ui.bounds(stack).unwrap();
    assert!(!ui.component(stack).unwrap().contains_point(PointF::new(10.0, card.height - 10.0), card.size()), "fora dos cartões o clique passa");
    click_at(&mut ui, form, card.x + card.width - 20.0, card.y + 20.0);
    assert_eq!(ui.get::<ToastStack>(stack).unwrap().count(), 0);
}

#[test]
fn level_meter_has_stereo_gradient_and_mute() {
    let (mut ui, form) = setup();
    let m = ui.create("LevelMeter", Some(form)).unwrap();
    for (name, value) in [("stereo", true), ("gradient", true), ("muted", true), ("dim", true)] {
        ui.set_prop(m, name, value).unwrap();
        assert_eq!(ui.get_prop(m, name).unwrap(), PropValue::Bool(value));
    }
    ui.set_prop(m, "value_right", 0.5).unwrap();
    ui.paint(form, &mut NullCanvas);
}

#[test]
fn label_spaces_letters_and_changes_case() {
    struct Texts(Vec<String>);
    impl Canvas for Texts {
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
        fn draw_text(&mut self, text: &str, _: RectF, _: &TextStyle) {
            self.0.push(text.to_string());
        }
        fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
            SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
        }
    }
    let (mut ui, form) = setup();
    let l = ui.add(Label::new("Microfones"), form);
    ui.control_mut(l).unwrap().set_rect(0.0, 0.0, 300.0, 20.0);
    ui.set_prop(l, "text_case", "upper").unwrap();
    let mut canvas = Texts(Vec::new());
    ui.paint(form, &mut canvas);
    assert_eq!(canvas.0, ["MICROFONES"]);
    ui.set_prop(l, "letter_spacing", 1.5).unwrap();
    let mut canvas = Texts(Vec::new());
    ui.paint(form, &mut canvas);
    assert_eq!(canvas.0.concat(), "MICROFONES", "letra a letra");
    assert_eq!(canvas.0.len(), 10);
    assert_eq!(ui.get_prop(l, "text").unwrap(), PropValue::String("Microfones".into()), "o texto guardado não muda");
}

#[test]
fn media_card_selects_by_its_box_and_clicks_elsewhere() {
    let (mut ui, form) = setup();
    let card = ui.create("MediaCard", Some(form)).unwrap();
    ui.control_mut(card).unwrap().set_rect(0.0, 0.0, 170.0, 148.0);
    ui.set_prop(card, "title", "Gravação").unwrap();
    ui.set_prop(card, "badges", "*AV1;60fps;icon:list").unwrap();
    ui.layout();
    let clicks = clicks(&mut ui, card, "on_click");
    let selects = clicks_of(&mut ui, card);
    ui.set_prop(card, "selection_mode", true).unwrap();
    click_at(&mut ui, form, 18.0, 18.0);
    assert_eq!(selects.borrow().len(), 1, "a caixa do canto seleciona");
    click_at(&mut ui, form, 90.0, 60.0);
    assert_eq!(clicks.borrow().len(), 1, "o resto abre");
    ui.set_prop(card, "selected", true).unwrap();
    ui.set_prop(card, "order", 2i64).unwrap();
    ui.set_prop(card, "progress", 0.4).unwrap();
    ui.paint(form, &mut NullCanvas);
}

fn clicks_of(ui: &mut Ui, id: ComponentId) -> Rc<RefCell<Vec<String>>> {
    clicks(ui, id, "on_select")
}

#[test]
fn label_markup_splits_bold_and_code_runs() {
    let runs = markup_runs("Use <code>{AAAA}</code> e <b>negrito</b> aqui");
    assert_eq!(
        runs,
        [
            ("Use ".to_string(), false, false),
            ("{AAAA}".to_string(), false, true),
            (" e ".to_string(), false, false),
            ("negrito".to_string(), true, false),
            (" aqui".to_string(), false, false),
        ]
    );
    assert_eq!(markup_plain("a <b>b</b><br>c"), "a b\nc");
    let (mut ui, form) = setup();
    let l = ui.create("Label", Some(form)).unwrap();
    ui.control_mut(l).unwrap().set_rect(0.0, 0.0, 60.0, 80.0);
    ui.set_prop(l, "markup", true).unwrap();
    ui.set_prop(l, "text", "um <b>dois tres</b> quatro cinco").unwrap();
    ui.layout();
    ui.paint(form, &mut NullCanvas);
}

/// Sem o pico à vista (`show_peak: false`), o medidor parado não pede quadros: a queda do
/// pico pedia um por atualização do monitor, e a janela parada desenhava sem parar.
#[test]
fn level_meter_without_peak_does_not_ask_for_frames() {
    let asks = |show_peak: bool| {
        let (mut ui, form) = setup();
        let m = ui.create("LevelMeter", Some(form)).unwrap();
        ui.set_prop(m, "show_peak", show_peak).unwrap();
        ui.set_prop(m, "smoothing", 0.0).unwrap();
        ui.set_prop(m, "value", 0.8).unwrap();
        ui.paint(form, &mut NullCanvas);
        // O nível cai e passa o tempo de segurar o pico.
        ui.set_prop(m, "value", 0.1).unwrap();
        ui.set_time(3.0);
        ui.paint(form, &mut NullCanvas);
        ui.is_animating() || ui.next_paint_wake().is_some()
    };
    assert!(asks(true), "com o pico, ele cai aos poucos (pede quadros)");
    assert!(!asks(false), "sem o pico, nada a animar");
}

/// O que fica inteiro atrás de um controle opaco (uma tela por cima da principal) não é pintado
/// nem pede quadros: o medidor animando embaixo não deixa a janela parada desenhando sem parar.
/// Com a cobertura escondida, ele volta a pintar e a animar.
#[test]
fn a_control_covered_by_an_opaque_sibling_does_not_ask_for_frames() {
    let (mut ui, form) = setup();
    let meter = ui.create("LevelMeter", Some(form)).unwrap();
    ui.control_mut(meter).unwrap().set_rect(10.0, 10.0, 200.0, 8.0);
    ui.set_prop(meter, "show_peak", false).unwrap();
    ui.set_prop(meter, "smoothing", 500.0).unwrap();
    // A tela de cima: um retângulo de cor sólida do tamanho do form.
    let cover = ui.create("Rectangle", Some(form)).unwrap();
    ui.control_mut(cover).unwrap().set_rect(0.0, 0.0, 800.0, 600.0);
    ui.set_prop(cover, "fill", "#202020").unwrap();
    ui.set_prop(cover, "radius", 0.0).unwrap();
    let animating = |ui: &mut Ui, value: f64, time: f64| {
        ui.set_prop(meter, "value", value).unwrap();
        ui.set_time(time);
        ui.paint(form, &mut NullCanvas);
        ui.is_animating()
    };
    // A primeira pintura descobre quem cobre; daí em diante o medidor nem é pintado.
    animating(&mut ui, 0.5, 0.0);
    assert!(!animating(&mut ui, 0.9, 0.1), "coberto, o medidor não pede quadros");
    // Translúcida, a cobertura deixa ver: o medidor pinta e anima.
    ui.set_prop(cover, "fill", "#20202080").unwrap();
    animating(&mut ui, 0.2, 0.2);
    assert!(animating(&mut ui, 0.8, 0.3), "à vista, o medidor desliza");
    // Opaca de novo e depois escondida.
    ui.set_prop(cover, "fill", "#202020").unwrap();
    animating(&mut ui, 0.1, 0.4);
    assert!(!animating(&mut ui, 0.7, 0.5));
    ui.set_prop(cover, "visible", false).unwrap();
    assert!(animating(&mut ui, 0.2, 0.6), "sem a cobertura, volta a animar");
}
