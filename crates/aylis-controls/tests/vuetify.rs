//! Os componentes equivalentes aos do Vuetify: comportamento com mouse, teclado e tempo.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

/// Canvas que só mede (8 por caractere) e guarda os textos.
#[derive(Default)]
struct Probe {
    texts: Vec<String>,
}

impl Canvas for Probe {
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
        self.texts.push(text.to_string());
    }
    fn measure_text(&mut self, text: &str, _: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * 8.0, 16.0)
    }
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.add_root(Form::new("teste", 800.0, 600.0));
    (ui, f)
}

fn place(ui: &mut Ui, id: ComponentId, x: f32, y: f32, w: f32, h: f32) {
    ui.control_mut(id).unwrap().set_rect(x, y, w, h);
}

fn paint(ui: &mut Ui, f: ComponentId) -> Probe {
    let mut p = Probe::default();
    ui.layout();
    ui.paint(f, &mut p);
    p
}

fn click(ui: &mut Ui, f: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_move(f, e);
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
}

fn key(ui: &mut Ui, f: ComponentId, key: Key) {
    ui.key_down(f, KeyEvent { key, ..Default::default() });
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            EventArgs::Value(v) => format!("{v:?}"),
            other => format!("{other:?}"),
        })
    });
    l
}

#[test]
fn grid_and_flow_layouts_arrange_children() {
    let (mut ui, f) = setup();
    let grid = ui.add(GridLayout::default(), f);
    place(&mut ui, grid, 0.0, 0.0, 640.0, 400.0);
    let _ = ui.set_prop(grid, "min_column_width", 200.0);
    let _ = ui.set_prop(grid, "gap", 10.0);
    let cards: Vec<ComponentId> = (0..5).map(|_| ui.add(Panel::default(), grid)).collect();
    for c in &cards {
        ui.control_mut(*c).unwrap().set_size(50.0, 80.0);
    }
    ui.layout();
    // 640 de largura, colunas de pelo menos 200 + 10 de espaço: 3 colunas.
    let b: Vec<RectF> = cards.iter().map(|c| ui.bounds(*c).unwrap()).collect();
    assert!((b[0].width - (640.0 - 20.0) / 3.0).abs() < 0.01 && b[0].height == 80.0, "{:?}", b[0]);
    assert_eq!(b[3].y, 90.0, "4º vai para a 2ª linha");
    assert!((b[1].x - (b[0].width + 10.0)).abs() < 0.01);
    // Mais estreito: 2 colunas.
    place(&mut ui, grid, 0.0, 0.0, 420.0, 400.0);
    ui.layout();
    assert_eq!(ui.bounds(cards[2]).unwrap().y, 90.0);

    let flow = ui.add(FlowLayout::default(), f);
    place(&mut ui, flow, 0.0, 0.0, 200.0, 200.0);
    let items: Vec<ComponentId> = (0..3).map(|_| ui.add(Chip::default(), flow)).collect();
    for c in &items {
        ui.control_mut(*c).unwrap().set_size(90.0, 30.0);
    }
    ui.layout();
    let r: Vec<RectF> = items.iter().map(|c| ui.bounds(*c).unwrap()).collect();
    assert_eq!((r[0].x, r[1].x, r[1].y), (0.0, 98.0, 0.0));
    assert_eq!((r[2].x, r[2].y), (0.0, 38.0), "não coube: quebrou a linha");
    // Justificado: a linha cheia vai de ponta a ponta; a última fica à esquerda.
    ui.set_prop(flow, "h_align", PropValue::Enum("justify".into())).unwrap();
    ui.layout();
    let r: Vec<RectF> = items.iter().map(|c| ui.bounds(*c).unwrap()).collect();
    assert_eq!((r[0].x, r[1].x, r[2].x), (0.0, 110.0, 0.0));
}

#[test]
fn edit_mask_rules_and_validate_form() {
    let (mut ui, f) = setup();
    let phone = ui.add(Edit::new(""), f);
    let _ = ui.set_prop(phone, "mask", "(##) #####-####");
    let _ = ui.set_prop(phone, "text", "11987654321");
    assert_eq!(ui.get::<Edit>(phone).unwrap().text, "(11) 98765-4321");
    // Digitando: a máscara põe os separadores sozinha.
    let _ = ui.set_prop(phone, "text", "");
    place(&mut ui, phone, 10.0, 10.0, 200.0, 50.0);
    ui.layout();
    ui.set_focus(phone);
    ui.text_input(f, "1198a7");
    assert_eq!(ui.get::<Edit>(phone).unwrap().text, "(11) 987");

    let email = ui.add(Edit::new("joao@"), f);
    let _ = ui.set_prop(email, "rules", "required;email");
    let name = ui.add(Edit::new(""), f);
    let _ = ui.set_prop(name, "rules", "required;min:3");
    assert!(!validate_form(&mut ui, f));
    assert_eq!(ui.get::<Edit>(email).unwrap().error, "E-mail inválido");
    assert_eq!(ui.get::<Edit>(name).unwrap().error, "Obrigatório");
    let _ = ui.set_prop(email, "text", "joao@aylis.dev");
    let _ = ui.set_prop(name, "text", "João");
    assert!(validate_form(&mut ui, f));
    assert!(ui.get::<Edit>(email).unwrap().error.is_empty());
    // Rótulo, ajuda e contador aparecem.
    let _ = ui.set_prop(name, "label", "Nome");
    let _ = ui.set_prop(name, "counter", true);
    let _ = ui.set_prop(name, "max_length", 20i64);
    place(&mut ui, name, 10.0, 80.0, 200.0, 56.0);
    let p = paint(&mut ui, f);
    assert!(p.texts.contains(&"Nome".to_string()) && p.texts.contains(&"4/20".to_string()), "{:?}", p.texts);
}

#[test]
fn chips_rating_range_and_otp() {
    let (mut ui, f) = setup();
    let chip = ui.add(Chip::new("Rust"), f);
    place(&mut ui, chip, 0.0, 0.0, 100.0, 32.0);
    let _ = ui.set_prop(chip, "closable", true);
    let closed = log(&mut ui, chip, "on_close");
    paint(&mut ui, f);
    click(&mut ui, f, 100.0 - 14.0, 16.0);
    assert_eq!(closed.borrow().len(), 1);

    let group = ui.add(ChipGroup::default(), f);
    place(&mut ui, group, 0.0, 50.0, 400.0, 40.0);
    let _ = ui.set_prop(group, "multiple", true);
    let changes = log(&mut ui, group, "on_change");
    paint(&mut ui, f);
    // "Todos;Abertos;Fechados": clique no segundo.
    let x = 28.0 + 5.0 * 8.0 + 18.0 + 8.0 + 20.0;
    click(&mut ui, f, x, 66.0);
    assert_eq!(changes.borrow().last().map(String::as_str), Some("Todos;Abertos"));

    let rating = ui.add(Rating::default(), f);
    place(&mut ui, rating, 0.0, 100.0, 150.0, 30.0);
    let rated = log(&mut ui, rating, "on_change");
    ui.layout();
    click(&mut ui, f, 125.0, 115.0); // 5ª estrela
    assert_eq!(ui.get::<Rating>(rating).unwrap().value, 5.0);
    assert_eq!(rated.borrow().len(), 1);

    let range = ui.add(RangeSlider::default(), f);
    place(&mut ui, range, 0.0, 140.0, 224.0, 32.0);
    ui.layout();
    // Trilho de 12 a 212: clicar perto do controle de cima o move.
    click(&mut ui, f, 12.0 + 200.0 * 0.9, 156.0);
    let r = ui.get::<RangeSlider>(range).unwrap();
    assert_eq!((r.low, r.high), (20.0, 90.0));

    let otp = ui.add(OtpInput::default(), f);
    place(&mut ui, otp, 0.0, 180.0, 280.0, 48.0);
    let _ = ui.set_prop(otp, "length", 4i64);
    let done = log(&mut ui, otp, "on_complete");
    ui.layout();
    ui.set_focus(otp);
    ui.text_input(f, "12a3");
    key(&mut ui, f, Key::Backspace);
    ui.text_input(f, "45");
    assert_eq!(ui.get::<OtpInput>(otp).unwrap().text, "1245");
    assert_eq!(done.borrow().as_slice(), ["1245"]);
}

#[test]
fn pagination_stepper_and_snackbar() {
    let (mut ui, f) = setup();
    let mut p = Pagination::default();
    (p.page, p.length) = (6, 20);
    assert_eq!(p.visible_pages(), vec![1, 0, 5, 6, 7, 0, 20]);
    let mut p = Pagination::default();
    (p.page, p.length) = (2, 20);
    assert_eq!(p.visible_pages(), vec![1, 2, 3, 4, 5, 0, 20]);
    let mut p = Pagination::default();
    (p.page, p.length) = (1, 4);
    assert_eq!(p.visible_pages(), vec![1, 2, 3, 4]);
    let mut p = Pagination::default();
    (p.page, p.length) = (19, 20);
    assert_eq!(p.visible_pages(), vec![1, 0, 16, 17, 18, 19, 20]);

    let pg = ui.add(Pagination::default(), f);
    place(&mut ui, pg, 0.0, 0.0, 400.0, 36.0);
    let pages = log(&mut ui, pg, "on_change");
    ui.layout();
    ui.set_focus(pg);
    key(&mut ui, f, Key::End);
    assert_eq!(ui.get::<Pagination>(pg).unwrap().page, 10);
    assert_eq!(pages.borrow().len(), 1);

    let mut s = Stepper::default();
    s.next();
    s.next();
    assert_eq!(s.step, 3);
    s.next();
    s.next();
    assert_eq!(s.step, 4, "não passa do fim");

    // O snackbar some sozinho depois do tempo.
    let bar = ui.add(Snackbar::default(), f);
    place(&mut ui, bar, 0.0, 100.0, 300.0, 48.0);
    let closed = log(&mut ui, bar, "on_close");
    let _ = ui.set_prop(bar, "timeout", 1000i64);
    let _ = ui.set_prop(bar, "active", true);
    ui.set_time(0.0);
    ui.run_timers();
    ui.set_time(1.2);
    ui.run_timers();
    assert!(!ui.get::<Snackbar>(bar).unwrap().active);
    assert_eq!(closed.borrow().len(), 1);
    // A função cria um no form e ele sai ao fechar.
    let before = ui.children(f).len();
    let id = snackbar(&mut ui, "Salvo", "Desfazer", |_| {}).unwrap();
    assert_eq!(ui.children(f).len(), before + 1);
    ui.set_time(2.0);
    ui.run_timers();
    ui.set_time(7.0);
    ui.run_timers();
    ui.process_posted();
    assert!(!ui.contains(id), "saiu do form");
}

#[test]
fn autocomplete_suggests_and_picks() {
    let (mut ui, f) = setup();
    let a = ui.add(AutoComplete::default(), f);
    place(&mut ui, a, 10.0, 10.0, 240.0, 34.0);
    let picked = log(&mut ui, a, "on_select");
    ui.layout();
    ui.set_focus(a);
    ui.text_input(f, "sao");
    ui.layout();
    // A lista aparece no form, abaixo do campo.
    let list = *ui.children(f).last().unwrap();
    assert_eq!(ui.class_name(list), Some("Suggestions"));
    assert_eq!(ui.bounds(list).unwrap().y, 46.0);
    key(&mut ui, f, Key::Down);
    key(&mut ui, f, Key::Enter);
    assert_eq!(ui.get::<AutoComplete>(a).unwrap().base.text, "São Paulo");
    assert_eq!(picked.borrow().as_slice(), ["São Paulo"]);
    assert!(!ui.contains(list), "fechou");
}

#[test]
fn color_edit_opens_picker_and_files_drop() {
    let (mut ui, f) = setup();
    let edit = ui.add(ColorEdit::default(), f);
    place(&mut ui, edit, 10.0, 10.0, 180.0, 34.0);
    let changed = log(&mut ui, edit, "on_change");
    ui.layout();
    click(&mut ui, f, 50.0, 20.0);
    let picker = *ui.children(f).last().unwrap();
    assert_eq!(ui.class_name(picker), Some("ColorPicker"));
    ui.layout();
    // Uma amostra (o vermelho, a primeira) na linha de baixo do seletor.
    let pr = ui.bounds(picker).unwrap();
    let swatch_y = pr.y + 10.0 + 150.0 + 12.0 + 12.0 + 10.0 + 0.0 + 36.0 + 12.0;
    click(&mut ui, f, pr.x + 14.0, swatch_y);
    assert_eq!(ui.get::<ColorEdit>(edit).unwrap().color, Color::rgb(0xF4, 0x43, 0x36));
    assert_eq!(changed.borrow().len(), 1);

    let zone = ui.add(FileDropZone::default(), f);
    place(&mut ui, zone, 300.0, 10.0, 300.0, 200.0);
    let _ = ui.set_prop(zone, "filter", "Imagens|*.png;*.jpg");
    let files = log(&mut ui, zone, "on_change");
    ui.layout();
    assert!(!ui.drop_files(f, PointF::new(400.0, 50.0), vec!["c:/a.txt".into()]), "fora do filtro");
    assert!(ui.drop_files(f, PointF::new(400.0, 50.0), vec!["c:/foto.PNG".into()]));
    assert_eq!(files.borrow().as_slice(), ["c:/foto.PNG"]);
    // Sem posição útil: vai para o primeiro que aceita.
    assert!(ui.drop_files(f, PointF::new(-5.0, -5.0), vec!["c:/b.jpg".into()]));
    assert_eq!(ui.get::<FileDropZone>(zone).unwrap().files, "c:/foto.PNG;c:/b.jpg");
}

#[test]
fn drawer_carousel_calendar_and_scroll_events() {
    let (mut ui, f) = setup();
    let drawer = ui.add(NavigationDrawer::default(), f);
    ui.layout();
    assert_eq!(ui.bounds(drawer).unwrap().width, 256.0);
    let _ = ui.set_prop(drawer, "rail", true);
    let _ = ui.set_prop(drawer, "open", false);
    ui.layout();
    assert_eq!(ui.bounds(drawer).unwrap().width, 64.0);
    let _ = ui.set_prop(drawer, "rail", false);
    ui.layout();
    assert!(!ui.control(drawer).unwrap().visible());
    let _ = ui.set_prop(drawer, "open", true);

    let car = ui.add(Carousel::default(), f);
    place(&mut ui, car, 300.0, 0.0, 300.0, 200.0);
    let slides: Vec<ComponentId> = (0..3).map(|_| ui.add(Panel::default(), car)).collect();
    for s in &slides {
        ui.control_mut(*s).unwrap().set_align(Align::Client);
    }
    ui.layout();
    let shown = |ui: &Ui| slides.iter().map(|s| ui.control(*s).unwrap().visible()).collect::<Vec<bool>>();
    assert_eq!(shown(&ui), [true, false, false]);
    Carousel::next(&mut ui, car);
    assert_eq!(shown(&ui), [false, true, false]);
    let _ = ui.set_prop(car, "index", -1i64);
    assert_eq!(shown(&ui), [false, false, true], "dá a volta");

    let cal = ui.add(Calendar::default(), f);
    place(&mut ui, cal, 0.0, 300.0, 280.0, 280.0);
    let _ = ui.set_prop(cal, "range", true);
    let _ = ui.set_prop(cal, "date", "2026-10-16");
    let ranges = log(&mut ui, cal, "on_change");
    ui.layout();
    ui.set_focus(cal);
    // Teclado: Esquerda escolhe o dia anterior (que vira o início do intervalo).
    key(&mut ui, f, Key::Left);
    assert_eq!(ranges.borrow().last().map(String::as_str), Some("2026-10-15;2026-10-16"));

    let scroll = ui.add(ScrollBox::default(), f);
    place(&mut ui, scroll, 600.0, 300.0, 150.0, 100.0);
    let inner = ui.add(Panel::default(), scroll);
    place(&mut ui, inner, 0.0, 0.0, 140.0, 400.0);
    let ends = log(&mut ui, scroll, "on_end_reached");
    ui.layout();
    for _ in 0..10 {
        ui.wheel(f, WheelEvent { position: PointF::new(650.0, 350.0), delta: PointF::new(0.0, -40.0), ..Default::default() });
    }
    assert_eq!(ends.borrow().len(), 1, "avisa uma vez");
}

#[test]
fn horizontal_scroll_and_slide_group() {
    let (mut ui, f) = setup();
    let sb = ui.add(ScrollBox::default(), f);
    place(&mut ui, sb, 0.0, 0.0, 200.0, 60.0);
    let _ = ui.set_prop(sb, "horizontal", true);
    for _ in 0..6 {
        let c = ui.add(Chip::default(), sb);
        ui.control_mut(c).unwrap().set_align(Align::Left).set_width(90.0).set_margins([0.0, 0.0, 8.0, 0.0]);
    }
    ui.layout();
    let wheel = |ui: &mut Ui, dy: f32| ui.wheel(f, WheelEvent { position: PointF::new(100.0, 30.0), delta: PointF::new(0.0, dy), ..Default::default() });
    wheel(&mut ui, -100.0);
    let s = ui.get::<ScrollBox>(sb).unwrap();
    assert_eq!(s.scroll(), 100.0, "a roda comum rola para os lados");
    assert_eq!(ui.control(sb).unwrap().scroll_offset(), PointF::new(100.0, 0.0));
    wheel(&mut ui, -10_000.0);
    assert_eq!(ui.get::<ScrollBox>(sb).unwrap().scroll(), 6.0 * 98.0 - 200.0);

    let g = ui.add(ChipGroup::default(), f);
    place(&mut ui, g, 0.0, 100.0, 120.0, 40.0);
    let _ = ui.set_prop(g, "wrap", false);
    let _ = ui.set_prop(g, "items", "Um;Dois;Três;Quatro;Cinco");
    let changes = log(&mut ui, g, "on_change");
    paint(&mut ui, f);
    // A seta da direita rola; o clique seguinte no chip visível escolhe ele.
    click(&mut ui, f, 120.0 - 10.0, 120.0);
    paint(&mut ui, f);
    assert!(changes.borrow().is_empty(), "a seta não escolhe");
    click(&mut ui, f, 60.0, 120.0);
    assert_eq!(changes.borrow().len(), 1);
    assert_ne!(changes.borrow()[0], "Um", "rolou: o chip sob o clique é outro");
}

#[test]
fn label_highlight_marks_matches() {
    assert_eq!(find_matches("São Paulo e são josé", "sao"), vec![(0, 3), (12, 15)]);
    assert!(find_matches("abc", "").is_empty());
    let (mut ui, f) = setup();
    let l = ui.add(Label::new("Busque por São Paulo"), f);
    place(&mut ui, l, 0.0, 0.0, 300.0, 24.0);
    let _ = ui.set_prop(l, "highlight", "sao");
    let p = paint(&mut ui, f);
    assert_eq!(p.texts, ["Busque por São Paulo"], "o texto sai inteiro, com o fundo marcado atrás");
}

#[test]
fn badge_sticks_to_the_corner_of_its_target() {
    let (mut ui, f) = setup();
    let bell = ui.add(Button::with_icon("bell"), f);
    place(&mut ui, bell, 100.0, 100.0, 40.0, 40.0);
    let badge = ui.add(Badge::default(), f);
    let _ = ui.set_prop(badge, "target", PropValue::Component(Some(bell)));
    let _ = ui.set_prop(badge, "text", "3");
    ui.layout();
    let b = ui.bounds(badge).unwrap();
    assert_eq!((b.width, b.height), (18.0, 18.0));
    let c = b.center();
    assert!((c.x - (140.0 - 40.0 * 0.12)).abs() < 0.01 && (c.y - (100.0 + 40.0 * 0.12)).abs() < 0.01, "{b:?}");
    // O alvo mudou de lugar: o selo vai junto.
    place(&mut ui, bell, 300.0, 50.0, 40.0, 40.0);
    ui.layout();
    assert!(ui.bounds(badge).unwrap().x > 300.0);
    let _ = ui.set_prop(badge, "anchor", "bottom_left");
    ui.layout();
    assert!(ui.bounds(badge).unwrap().center().x < 310.0);
    ui.free(bell);
    assert_eq!(ui.get::<Badge>(badge).unwrap().target, None, "o alvo saiu");
}

#[test]
fn snackbar_queue_shows_one_after_another() {
    let (mut ui, f) = setup();
    let done = Rc::new(RefCell::new(Vec::new()));
    let (d1, d2) = (done.clone(), done.clone());
    let id = snackbar(&mut ui, "Primeiro", "Desfazer", move |_| d1.borrow_mut().push(1)).unwrap();
    let same = snackbar(&mut ui, "Segundo", "Ver", move |_| d2.borrow_mut().push(2)).unwrap();
    assert_eq!(id, same, "o segundo entra na fila do mesmo snackbar");
    assert_eq!(ui.get::<Snackbar>(id).unwrap().pending(), 1);
    ui.layout();
    paint(&mut ui, f);
    // A ação do primeiro roda e o segundo entra.
    let r = ui.bounds(id).unwrap();
    click(&mut ui, f, r.right() - 40.0, r.y + 24.0);
    assert_eq!(done.borrow().as_slice(), [1]);
    let bar = ui.get::<Snackbar>(id).unwrap();
    assert_eq!((bar.text.as_str(), bar.active, bar.pending()), ("Segundo", true, 0));
    // O tempo acaba: sai do form.
    ui.set_time(10.0);
    ui.run_timers();
    ui.set_time(20.0);
    ui.run_timers();
    ui.process_posted();
    assert!(!ui.contains(id));
}

#[test]
fn time_edit_opens_the_clock() {
    let (mut ui, f) = setup();
    let t = ui.add(TimeEdit::new("09:15"), f);
    place(&mut ui, t, 10.0, 10.0, 140.0, 34.0);
    let changes = log(&mut ui, t, "on_change");
    ui.layout();
    ui.set_focus(t);
    click(&mut ui, f, 140.0, 27.0); // o botão do relógio
    let picker = *ui.children(f).last().unwrap();
    assert_eq!(ui.class_name(picker), Some("TimePicker"));
    ui.layout();
    let p = ui.bounds(picker).unwrap();
    // O mostrador: centro e raio como o TimePicker calcula.
    let side = (p.width - 24.0).min(p.height - 64.0 - 16.0);
    let (cx, cy, r) = (p.x + p.width / 2.0, p.y + 72.0 + side / 2.0, side / 2.0);
    click(&mut ui, f, cx + r * 0.4, cy); // 15 h (anel de dentro, às 3)
    assert_eq!(ui.get::<TimeEdit>(t).unwrap().time, "15:15");
    assert!(ui.contains(picker), "agora os minutos");
    click(&mut ui, f, cx, cy + r - 12.0); // 30 min
    assert_eq!(ui.get::<TimeEdit>(t).unwrap().time, "15:30");
    assert!(!ui.contains(picker), "fechou ao escolher os minutos");
    assert_eq!(changes.borrow().last().map(String::as_str), Some("15:30"));
}

#[test]
fn event_calendar_week_and_day_views() {
    let events = parse_events("2026-10-21 14:00-15:30|Reunião|success;2026-10-21|Feriado;2026-10-21 14:30-16:00|Revisão");
    assert_eq!(events[0].time, Some((840, 930)));
    assert_eq!(events[1].time, None);
    let (mut ui, f) = setup();
    let c = ui.add(EventCalendar::default(), f);
    place(&mut ui, c, 0.0, 0.0, 700.0, 500.0);
    let _ = ui.set_prop(c, "view", "week");
    let _ = ui.set_prop(c, "date", "2026-10-21");
    let _ = ui.set_prop(c, "events", "2026-10-21 14:00-15:30|Reunião|success;2026-10-21|Feriado;2026-10-21 14:30-16:00|Revisão");
    let picked = log(&mut ui, c, "on_event");
    let slots = log(&mut ui, c, "on_slot");
    let p = paint(&mut ui, f);
    assert!(p.texts.iter().any(|t| t == "Reunião") && p.texts.iter().any(|t| t == "Feriado"), "{:?}", p.texts);
    // Grade: margem 44, 7 colunas; qua (21) é a 4ª coluna com a semana começando no domingo.
    let cw = (700.0 - 44.0) / 7.0;
    let grid_y = 40.0 + 40.0 + 24.0;
    let hour_h = (500.0 - grid_y - 4.0) / 13.0;
    let x = 44.0 + 3.0 * cw + 8.0; // a faixa da esquerda (Reunião)
    click(&mut ui, f, x, grid_y + (14.5 - 7.0) * hour_h);
    assert_eq!(picked.borrow().as_slice(), ["2026-10-21|Reunião"]);
    click(&mut ui, f, x, grid_y + 0.5 * hour_h);
    assert_eq!(slots.borrow().as_slice(), ["2026-10-21 07:00"]);
    // A seta anda uma semana.
    click(&mut ui, f, 20.0, 20.0);
    assert_eq!(ui.get::<EventCalendar>(c).unwrap().date, "2026-10-14");
    let _ = ui.set_prop(c, "view", "day");
    click(&mut ui, f, 680.0, 20.0);
    assert_eq!(ui.get::<EventCalendar>(c).unwrap().date, "2026-10-15");
}
