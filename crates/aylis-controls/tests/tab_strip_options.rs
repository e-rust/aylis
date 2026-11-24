//! TabStrip: as cores da aba ativa e das outras pelo estilo `TabStrip.tab` do tema, a prévia
//! da aba sob o mouse (`hover_preview`), o mouse só sobre as abas (`hit_tabs_only`) e o
//! `receive_drag` público.

use aylis_controls::*;
use aylis_core::pages::{Op, RecordingCanvas};
use aylis_core::theme::{ColorRef, Style, StyleProps};
use aylis_core::*;

/// Um desenho fora da tela que não desenha nada (o `ui.snapshot` das miniaturas).
struct FakeOffscreen;

impl platform::OffscreenService for FakeOffscreen {
    fn render_png(&self, _w: u32, _h: u32, _scale: f32, _bg: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        let mut canvas = RecordingCanvas::new(None);
        paint(&mut canvas);
        Some(vec![1, 2, 3])
    }
}

/// Um form com a faixa no topo e duas páginas (a primeira escolhida no fim).
fn window(ui: &mut Ui) -> (ComponentId, ComponentId, Vec<ComponentId>) {
    let form = ui.add_root(Form::new("Teste", 800.0, 500.0));
    let strip = ui.add(TabStrip::new(), form);
    ui.control_mut(strip).unwrap().set_align(Align::Top).set_height(40.0);
    let host = ui.add(Layout::default(), form);
    ui.control_mut(host).unwrap().set_align(Align::Client);
    ui.get_mut::<TabStrip>(strip).unwrap().page_host = Some(host);
    let pages = ["Clientes", "Pedidos"]
        .iter()
        .map(|t| {
            let page = ui.add(Panel::default(), host);
            TabStrip::add_page(ui, strip, t, "", page);
            page
        })
        .collect();
    ui.show(form).unwrap();
    ui.layout();
    paint(ui, form);
    (form, strip, pages)
}

fn paint(ui: &mut Ui, form: ComponentId) -> Vec<Op> {
    let mut canvas = RecordingCanvas::new(None);
    ui.request_repaint();
    ui.paint(form, &mut canvas);
    canvas.ops
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, click_count: 1, ..Default::default() }
}

fn setup() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

#[test]
fn the_theme_style_paints_the_active_and_the_inactive_tabs() {
    let mut ui = setup();
    let (form, _, _) = window(&mut ui);
    let before = paint(&mut ui, form);
    let (green, blue, white) = (Color::rgb(0, 160, 0), Color::rgb(0, 0, 200), Color::rgb(250, 250, 250));
    assert!(!before.iter().any(|op| matches!(op, Op::FillPath { color, .. } if *color == green)), "sem o estilo, o visual de sempre");
    let mut theme = ui.theme().clone();
    let mut tab = Style::new(StyleProps { fill: Some(ColorRef::Value(blue)), text: Some(ColorRef::Value(white)), ..Default::default() });
    tab.checked = StyleProps { fill: Some(ColorRef::Value(green)), ..Default::default() };
    theme.set_style("TabStrip.tab", tab);
    ui.set_theme(theme);
    let ops = paint(&mut ui, form);
    assert!(ops.iter().any(|op| matches!(op, Op::FillPath { color, .. } if *color == green)), "a ativa com o fundo do `checked`");
    assert!(ops.iter().any(|op| matches!(op, Op::FillRect { color, .. } if *color == blue)), "a inativa com o fundo do estilo");
    assert!(ops.iter().any(|op| matches!(op, Op::Text { text, style, .. } if text == "Clientes" && style.color == white)), "o texto da inativa pelo estilo");
}

#[test]
fn hover_preview_shows_the_thumbnail_of_a_tab_already_shown() {
    let mut ui = setup();
    ui.services_mut().offscreen = Some(Box::new(FakeOffscreen));
    let (form, strip, _) = window(&mut ui);
    ui.get_mut::<TabStrip>(strip).unwrap().hover_preview = true;
    let preview_count = |ui: &Ui| ui.children(form).iter().filter(|&&c| ui.class_name(c) == Some("TabPreview")).count();
    // "Pedidos" (a segunda) está à vista; "Clientes" nunca foi mostrada depois de ligar a prévia.
    ui.set_time(1.0);
    ui.pointer_move(form, at(60.0, 20.0));
    ui.set_time(2.0);
    ui.run_timers();
    assert_eq!(preview_count(&ui), 0, "a aba sem miniatura não mostra prévia");
    // Escolher "Clientes" guarda a miniatura de "Pedidos" (que sai da vista).
    ui.pointer_down(form, at(60.0, 20.0));
    ui.pointer_up(form, at(60.0, 20.0));
    assert_eq!(ui.get::<TabStrip>(strip).unwrap().tab_index, 0);
    paint(&mut ui, form);
    ui.pointer_move(form, at(8.0 + 240.0 + 60.0, 20.0));
    ui.set_time(2.1);
    ui.run_timers();
    assert_eq!(preview_count(&ui), 0, "espera o mouse parar");
    ui.set_time(3.0);
    ui.run_timers();
    assert_eq!(preview_count(&ui), 1, "a prévia de Pedidos");
    ui.pointer_move(form, at(700.0, 20.0));
    assert_eq!(preview_count(&ui), 0, "o mouse saiu da aba: a prévia some");
}

#[test]
fn hit_tabs_only_lets_the_empty_part_through() {
    let mut ui = setup();
    let (form, strip, _) = window(&mut ui);
    // A parte vazia, à direita das abas (e do "+").
    let empty = PointF::new(760.0, 20.0);
    assert_eq!(ui.hit_test(form, empty), Some(strip), "desligado: a faixa inteira pega o mouse");
    ui.get_mut::<TabStrip>(strip).unwrap().hit_tabs_only = true;
    assert_ne!(ui.hit_test(form, empty), Some(strip), "ligado: a parte vazia deixa passar");
    assert_eq!(ui.hit_test(form, PointF::new(60.0, 20.0)), Some(strip), "a aba continua pegando");
}

#[test]
fn receive_drag_brings_a_tab_from_another_strip() {
    let mut ui = setup();
    let (_, a, pages) = window(&mut ui);
    let (b_form, b, _) = window(&mut ui);
    let data = format!("aylis-tab::{}:0", a.to_u64());
    assert!(TabStrip::receive_drag(&mut ui, b, &data, None));
    let titles: Vec<String> = ui.get::<TabStrip>(b).unwrap().tabs().iter().map(|t| t.title.clone()).collect();
    assert_eq!(titles, ["Clientes", "Pedidos", "Clientes"], "no fim");
    assert_eq!(ui.root_of(pages[0]), b_form, "a página foi junto");
    assert_eq!(ui.get::<TabStrip>(a).unwrap().tabs().len(), 1);
    // De outro grupo: recusada.
    ui.get_mut::<TabStrip>(b).unwrap().drop_group = "outro".into();
    let data = format!("aylis-tab::{}:0", a.to_u64());
    assert!(!TabStrip::receive_drag(&mut ui, b, &data, Some(0)));
    // Pelo nome, nas outras linguagens.
    ui.get_mut::<TabStrip>(b).unwrap().drop_group = String::new();
    assert_eq!(ui.invoke(b, "receive_drag", &[PropValue::from(data.as_str()), PropValue::Int(0)]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get::<TabStrip>(b).unwrap().tabs()[0].title, "Pedidos");
}
