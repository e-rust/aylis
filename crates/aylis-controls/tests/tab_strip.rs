//! TabStrip: as abas do navegador (clicar escolhe, o × e o clique do meio fecham, arrastar
//! troca a ordem, o "+" pede uma nova).

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

fn setup() -> (Ui, ComponentId, ComponentId, Rc<RefCell<Vec<String>>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 900.0, 300.0));
    let id = ui.create("TabStrip", Some(form)).unwrap();
    ui.set_prop(id, "tabs", "Início|home;Relatório;Notas").unwrap();
    ui.control_mut(id).unwrap().set_align(Align::Top).set_height(40.0);
    let log = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_change", "on_close", "on_add", "on_reorder"] {
        let log = log.clone();
        ui.on(id, event, move |_, _, args| log.borrow_mut().push(format!("{event} {args:?}")));
    }
    ui.layout();
    ui.paint(form, &mut NullCanvas);
    (ui, form, id, log)
}

fn at(x: f32, y: f32, button: PointerButton) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), button, click_count: 1, ..Default::default() }
}

fn strip(ui: &Ui, id: ComponentId) -> &TabStrip {
    ui.get::<TabStrip>(id).unwrap()
}

#[test]
fn tabs_from_text_and_api() {
    let mut t = TabStrip::new();
    t.set_tabs_text("A|home;B;C".into());
    assert_eq!(t.tabs().len(), 3);
    assert_eq!(t.tabs()[0].icon, "home");
    assert_eq!(t.tabs_text(), "A|home;B;C");
    t.tab_index = 2;
    t.move_tab(2, 0);
    assert_eq!(t.tabs_text(), "C;A|home;B");
    assert_eq!(t.tab_index, 0, "a ativa acompanha");
    t.remove(0);
    assert_eq!(t.tab_index, 0);
    assert_eq!(t.tabs()[0].title, "A");
}

#[test]
fn click_selects_middle_click_closes_and_plus_asks_for_a_new_one() {
    let (mut ui, form, id, log) = setup();
    // As abas dividem a largura (até 240 cada): a segunda começa em 8 + 240.
    ui.pointer_down(form, at(8.0 + 240.0 + 60.0, 22.0, PointerButton::Primary));
    ui.pointer_up(form, at(8.0 + 240.0 + 60.0, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tab_index, 1);
    assert!(log.borrow().iter().any(|l| l.starts_with("on_change")));
    // O clique do meio na terceira fecha (com auto_close).
    ui.pointer_down(form, at(8.0 + 480.0 + 60.0, 22.0, PointerButton::Middle));
    assert_eq!(strip(&ui, id).tabs().len(), 2);
    assert!(log.borrow().iter().any(|l| l.starts_with("on_close")));
    // O "+" depois da última.
    ui.paint(form, &mut NullCanvas);
    ui.pointer_down(form, at(8.0 + 480.0 + 6.0 + 15.0, 25.0, PointerButton::Primary));
    assert!(log.borrow().iter().any(|l| l.starts_with("on_add")));
}

#[test]
fn dragging_reorders() {
    let (mut ui, form, id, log) = setup();
    ui.pointer_down(form, at(60.0, 22.0, PointerButton::Primary));
    ui.pointer_move(form, at(200.0, 22.0, PointerButton::Primary));
    ui.pointer_move(form, at(300.0, 22.0, PointerButton::Primary));
    ui.pointer_up(form, at(300.0, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tabs()[1].title, "Início");
    assert_eq!(strip(&ui, id).tab_index, 1);
    assert!(log.borrow().iter().any(|l| l.contains("on_reorder") && l.contains("0,1")), "{:?}", log.borrow());
}

/// Guarda onde cada título foi desenhado (para ver a aba andando).
#[derive(Default)]
struct TextSpots(Vec<(String, f32)>, Vec<(PointF, PointF)>);

impl Canvas for TextSpots {
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
    fn draw_line(&mut self, from: PointF, to: PointF, _: &Stroke) {
        self.1.push((from, to));
    }
    fn draw_text(&mut self, text: &str, rect: RectF, _: &TextStyle) {
        self.0.push((text.to_string(), rect.x));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

fn title_x(ui: &mut Ui, form: ComponentId, title: &str) -> f32 {
    let mut spots = TextSpots::default();
    ui.paint(form, &mut spots);
    spots.0.iter().find(|(t, _)| t == title).map(|s| s.1).unwrap_or_else(|| panic!("{title} não foi desenhado: {:?}", spots.0))
}

/// Como no Edge e no Chrome: a aba arrastada segue o mouse; a vizinha que perde o lugar
/// desliza para o novo (sai de onde estava), e a solta assenta no seu lugar aos poucos.
#[test]
fn dragging_follows_the_mouse_and_the_neighbors_slide() {
    let (mut ui, form, id, _) = setup();
    let rel = title_x(&mut ui, form, "Relatório");
    ui.set_time(1.0);
    ui.pointer_down(form, at(60.0, 22.0, PointerButton::Primary));
    ui.pointer_move(form, at(100.0, 22.0, PointerButton::Primary));
    // Antes de trocar: a aba pega andou junto com o mouse (40 pontos).
    let inicio = title_x(&mut ui, form, "Início");
    ui.pointer_move(form, at(110.0, 22.0, PointerButton::Primary));
    assert!((title_x(&mut ui, form, "Início") - inicio - 10.0).abs() < 0.01, "segue o mouse");
    // O meio dela passou o da vizinha: trocam, e a vizinha ainda está onde estava.
    ui.pointer_move(form, at(250.0, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tabs()[0].title, "Relatório");
    let start = title_x(&mut ui, form, "Relatório");
    assert!((start - rel).abs() < 0.5, "a vizinha começa onde estava: {start} × {rel}");
    // Alguns quadros depois, ela chegou ao lugar novo (o primeiro).
    let mut last = start;
    for k in 1..=20 {
        ui.set_time(1.0 + k as f64 / 60.0);
        ui.tick();
        let x = title_x(&mut ui, form, "Relatório");
        assert!(x <= last + 0.01, "anda sempre para o mesmo lado");
        last = x;
    }
    let first_slot = title_x(&mut ui, form, "Relatório");
    assert!(first_slot < start - 50.0, "deslizou para o primeiro lugar: {first_slot}");
    ui.pointer_up(form, at(250.0, 22.0, PointerButton::Primary));
    // A solta sai de sob o mouse e assenta no lugar dela.
    let solta = title_x(&mut ui, form, "Início");
    for k in 21..=60 {
        ui.set_time(1.0 + k as f64 / 60.0);
        ui.tick();
    }
    let assentada = title_x(&mut ui, form, "Início");
    // O lugar dela é o segundo (248), um pouco à direita de onde o mouse a deixou.
    assert!(assentada > solta + 20.0, "volta para o lugar: {solta} → {assentada}");
    ui.set_time(3.0);
    ui.tick();
    assert_eq!(title_x(&mut ui, form, "Início"), assentada, "já estava no lugar");
    assert!(!ui.is_animating(), "parou");
}

/// Como no Chrome e no Edge: fechar pelo × com o mouse parado mantém a largura das abas (o ×
/// da próxima vem para debaixo do mouse); quando o mouse sai da faixa, elas crescem até a
/// largura natural aos poucos.
#[test]
fn closing_keeps_the_widths_until_the_mouse_leaves() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 900.0, 300.0));
    let id = ui.create("TabStrip", Some(form)).unwrap();
    // Cinco abas numa faixa de 900: 160 cada (sem caber as 240 da largura máxima).
    ui.set_prop(id, "tabs", "A;B;C;D;E").unwrap();
    ui.control_mut(id).unwrap().set_align(Align::Top).set_height(40.0);
    let content = ui.add(Panel::default(), form);
    ui.control_mut(content).unwrap().set_align(Align::Client);
    ui.layout();
    let b = title_x(&mut ui, form, "B");
    let c = title_x(&mut ui, form, "C");
    let w = c - b;

    // Fecha a B pelo × dela, com o mouse parado.
    ui.set_time(1.0);
    let close_x = b - 12.0 + w - 19.0;
    ui.pointer_move(form, at(close_x, 22.0, PointerButton::Primary));
    ui.pointer_down(form, at(close_x, 22.0, PointerButton::Primary));
    ui.pointer_up(form, at(close_x, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tabs().len(), 4);
    assert_eq!(title_x(&mut ui, form, "C"), b, "a C ocupa o lugar da B, com a mesma largura");
    assert_eq!(title_x(&mut ui, form, "D") - title_x(&mut ui, form, "C"), w);
    // Fecha a seguinte sem mexer o mouse: o × da C estava ali.
    ui.pointer_down(form, at(close_x, 22.0, PointerButton::Primary));
    ui.pointer_up(form, at(close_x, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tabs().iter().map(|t| t.title.as_str()).collect::<Vec<_>>(), ["A", "D", "E"]);
    assert_eq!(title_x(&mut ui, form, "D") - title_x(&mut ui, form, "A"), w, "ainda presa");

    // O mouse sai da faixa: crescem até a natural (240), aos poucos e sem voltar.
    ui.pointer_move(form, at(close_x, 200.0, PointerButton::Primary));
    let mut last = w;
    for k in 1..=40 {
        ui.set_time(1.0 + k as f64 / 60.0);
        ui.tick();
        let now = title_x(&mut ui, form, "D") - title_x(&mut ui, form, "A");
        assert!(now >= last - 0.01, "só cresce: {last} → {now}");
        if k == 2 {
            assert!(now > w && now < 239.0, "no meio do caminho: {now}");
        }
        last = now;
    }
    assert_eq!(last, 240.0, "chegou à largura natural");
    assert!(!ui.is_animating(), "parou");
}

/// Onde está o "+": o meio do traço horizontal de 12 pontos dele.
fn plus_x(ui: &mut Ui, form: ComponentId) -> f32 {
    let mut spots = TextSpots::default();
    ui.paint(form, &mut spots);
    spots.1.iter().rev().find(|(a, b)| a.y == b.y && (b.x - a.x - 12.0).abs() < 0.01).map(|(a, _)| a.x + 6.0).expect("o + foi desenhado")
}

/// Como no navegador: a última aba pode ser empurrada para a direita além do lugar dela, até
/// a ponta da faixa, e o "+" vai junto; ao soltar, os dois voltam deslizando.
#[test]
fn pushing_the_last_tab_right_takes_the_plus_along() {
    let (mut ui, form, id, _) = setup();
    let notas = title_x(&mut ui, form, "Notas");
    let plus = plus_x(&mut ui, form);
    ui.set_time(1.0);
    ui.pointer_down(form, at(notas + 20.0, 22.0, PointerButton::Primary));
    ui.pointer_move(form, at(notas + 60.0, 22.0, PointerButton::Primary));
    ui.pointer_move(form, at(2000.0, 22.0, PointerButton::Primary));
    assert_eq!(strip(&ui, id).tabs()[2].title, "Notas", "continua a última");
    let pushed = title_x(&mut ui, form, "Notas");
    let pushed_plus = plus_x(&mut ui, form);
    assert!(pushed > notas + 100.0, "andou para a direita: {notas} → {pushed}");
    assert!((pushed_plus - plus - (pushed - notas)).abs() < 0.01, "o + andou junto: {plus} → {pushed_plus}");
    // Sem passar da ponta da faixa (900 de largura).
    assert!(pushed_plus + 15.0 <= 900.0, "{pushed_plus}");
    ui.pointer_up(form, at(2000.0, 22.0, PointerButton::Primary));
    for k in 1..=60 {
        ui.set_time(1.0 + k as f64 / 60.0);
        ui.tick();
    }
    assert_eq!(title_x(&mut ui, form, "Notas"), notas, "voltou ao lugar");
    assert_eq!(plus_x(&mut ui, form), plus, "o + também");
}
