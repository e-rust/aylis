use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::platform::ClipboardService;
use aylis_core::*;

/// Canvas que não desenha nada; mede como o `Ui` sem serviço de texto (estimativa).
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

/// Clipboard em memória.
#[derive(Clone, Default)]
struct FakeClipboard(Rc<RefCell<String>>);

impl ClipboardService for FakeClipboard {
    fn get_text(&mut self) -> Option<String> {
        Some(self.0.borrow().clone())
    }
    fn set_text(&mut self, text: &str) {
        *self.0.borrow_mut() = text.to_string();
    }
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

fn ctrl(c: char) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() }
}

fn shift(k: Key) -> KeyEvent {
    KeyEvent { key: k, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() }
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

fn form(ui: &mut Ui) -> ComponentId {
    aylis_controls::register(ui);
    ui.add_root(Form::new("teste", 400.0, 300.0))
}

fn text(ui: &Ui, id: ComponentId) -> String {
    ui.get::<Edit>(id).unwrap().text.clone()
}

// ---------------------------------------------------------------------------
// Edit

fn edit_setup(initial: &str) -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let e = ui.add(Edit::new(initial), f);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 300.0, 32.0);
    ui.set_focus(e);
    ui.paint(f, &mut NullCanvas);
    (ui, f, e)
}

#[test]
fn edit_typing_replaces_the_auto_selection() {
    let (mut ui, f, e) = edit_setup("antigo");
    // ao receber foco, seleciona tudo; digitar substitui
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (0, 6));
    let changes = Rc::new(RefCell::new(Vec::new()));
    let c = changes.clone();
    ui.on(e, "on_change", move |_, _, args| {
        if let EventArgs::Value(PropValue::String(s)) = args {
            c.borrow_mut().push(s.clone());
        }
    });
    ui.text_input(f, "Olá");
    ui.text_input(f, " mundo");
    assert_eq!(text(&ui, e), "Olá mundo");
    assert_eq!(*changes.borrow(), vec!["Olá", "Olá mundo"]);
}

#[test]
fn edit_keyboard_navigation_and_deletion() {
    let (mut ui, f, e) = edit_setup("um dois tres");
    ui.key_down(f, key(Key::End));
    ui.key_down(f, key(Key::Backspace));
    assert_eq!(text(&ui, e), "um dois tre");

    let ctrl_back = KeyEvent { key: Key::Backspace, modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() };
    ui.key_down(f, ctrl_back);
    assert_eq!(text(&ui, e), "um dois ", "Ctrl+Backspace apaga a palavra");

    ui.key_down(f, key(Key::Home));
    ui.key_down(f, shift(Key::Right));
    ui.key_down(f, shift(Key::Right));
    assert_eq!(ui.get::<Edit>(e).unwrap().selected_text(), "um");
    ui.key_down(f, key(Key::Delete));
    assert_eq!(text(&ui, e), " dois ");

    // Tab não é tratado pelo Edit: continua movendo o foco
    let other = ui.add(Button::new("ok"), f);
    ui.key_down(f, key(Key::Tab));
    assert_eq!(ui.focused(f), Some(other));
}

#[test]
fn edit_unicode_is_handled_by_character() {
    let (mut ui, f, e) = edit_setup("ação");
    ui.key_down(f, key(Key::End));
    ui.key_down(f, key(Key::Left));
    ui.key_down(f, key(Key::Backspace));
    assert_eq!(text(&ui, e), "aço");
    ui.text_input(f, "ã");
    assert_eq!(text(&ui, e), "ação");
}

#[test]
fn edit_clipboard_copy_cut_paste() {
    let (mut ui, f, e) = edit_setup("copiar isto");
    let clipboard = FakeClipboard::default();
    ui.services_mut().clipboard = Some(Box::new(clipboard.clone()));

    ui.key_down(f, ctrl('a'));
    ui.key_down(f, ctrl('c'));
    assert_eq!(*clipboard.0.borrow(), "copiar isto");

    ui.key_down(f, ctrl('x'));
    assert_eq!(text(&ui, e), "");
    ui.key_down(f, ctrl('v'));
    ui.key_down(f, ctrl('v'));
    assert_eq!(text(&ui, e), "copiar istocopiar isto");

    // senha não deixa copiar
    ui.set_prop(e, "password", true).unwrap();
    *clipboard.0.borrow_mut() = String::new();
    ui.key_down(f, ctrl('a'));
    ui.key_down(f, ctrl('c'));
    assert_eq!(*clipboard.0.borrow(), "");
}

#[test]
fn edit_max_length_and_read_only() {
    let (mut ui, f, e) = edit_setup("");
    ui.set_prop(e, "max_length", 5).unwrap();
    ui.text_input(f, "1234567");
    assert_eq!(text(&ui, e), "12345");

    ui.set_prop(e, "read_only", true).unwrap();
    ui.text_input(f, "x");
    ui.key_down(f, key(Key::Backspace));
    assert_eq!(text(&ui, e), "12345");
}

#[test]
fn edit_mouse_places_caret_and_double_click_selects_word() {
    let (mut ui, f, e) = edit_setup("abc defgh");
    // fonte 14 → cada caractere ~7,7 de largura na estimativa; texto começa em x=10+8
    let x_of = |chars: f32| 10.0 + 8.0 + chars * 7.7;
    ui.pointer_down(f, at(x_of(2.0) + 1.0, 20.0));
    ui.pointer_up(f, at(x_of(2.0) + 1.0, 20.0));
    assert_eq!(ui.get::<Edit>(e).unwrap().caret(), 2);

    // arrastar seleciona
    ui.set_time(10.0);
    ui.pointer_down(f, at(x_of(4.0), 20.0));
    ui.pointer_move(f, at(x_of(7.0), 20.0));
    ui.pointer_up(f, at(x_of(7.0), 20.0));
    assert_eq!(ui.get::<Edit>(e).unwrap().selected_text(), "def");

    // duplo clique seleciona a palavra
    ui.set_time(20.0);
    ui.pointer_down(f, at(x_of(5.5), 20.0));
    ui.pointer_up(f, at(x_of(5.5), 20.0));
    ui.pointer_down(f, at(x_of(5.5), 20.0));
    ui.pointer_up(f, at(x_of(5.5), 20.0));
    assert_eq!(ui.get::<Edit>(e).unwrap().selected_text(), "defgh");
}

#[test]
fn edit_caret_blink_schedules_a_wake_up_instead_of_continuous_frames() {
    let (mut ui, f, _e) = edit_setup("x");
    ui.set_time(3.0);
    ui.paint(f, &mut NullCanvas);
    assert!(!ui.is_animating(), "não pede frames sem parar");
    let wake = ui.next_wake().expect("agenda a próxima troca do cursor");
    assert!(wake > 3.0 && wake <= 3.5 + 1e-9, "{wake}");
}

// ---------------------------------------------------------------------------
// CheckBox, RadioButton, Switch, ProgressBar

#[test]
fn checkbox_toggles_by_click_and_space() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let cb = ui.add(CheckBox::new("Ativo"), f);
    let changes = Rc::new(RefCell::new(0));
    let c = changes.clone();
    ui.on(cb, "on_change", move |_, _, _| *c.borrow_mut() += 1);
    ui.layout();

    ui.pointer_down(f, at(5.0, 5.0));
    ui.pointer_up(f, at(5.0, 5.0));
    assert!(ui.get::<CheckBox>(cb).unwrap().checked);
    assert!(ui.control(cb).unwrap().state().checked, "estado sincronizado (tema/animações)");

    ui.key_down(f, key(Key::Space));
    assert!(!ui.get::<CheckBox>(cb).unwrap().checked);
    assert_eq!(*changes.borrow(), 2);

    ui.set_prop(cb, "checked", true).unwrap();
    assert!(ui.control(cb).unwrap().state().checked, "set_prop também sincroniza");
    assert!(ui.prop_info(cb, "on_change").is_some());
}

#[test]
fn checked_trigger_runs_animations() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let cb = ui.add(CheckBox::new("x"), f);
    ui.add(PropertyAnimation::float("opacity", 0.5).with_trigger(Trigger::Checked).with_duration(0.0), cb);
    ui.click(cb);
    ui.tick();
    assert_eq!(ui.get_prop(cb, "opacity").unwrap(), PropValue::Float(0.5));
    ui.click(cb);
    ui.tick();
    assert_eq!(ui.get_prop(cb, "opacity").unwrap(), PropValue::Float(1.0));
}

#[test]
fn radio_buttons_are_exclusive_within_a_group() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let radios: Vec<_> = ["Física", "Jurídica", "Outra"].iter().map(|t| ui.add(RadioButton::new(*t), f)).collect();
    let separate = ui.add(RadioButton::new("Outro grupo"), f);
    ui.set_prop(separate, "group", "b").unwrap();
    ui.click(separate);

    ui.click(radios[0]);
    ui.click(radios[1]);
    let checked: Vec<bool> = radios.iter().map(|&r| ui.get::<RadioButton>(r).unwrap().checked).collect();
    assert_eq!(checked, vec![false, true, false]);
    assert!(ui.get::<RadioButton>(separate).unwrap().checked, "outro grupo não é afetado");

    ui.click(radios[1]);
    assert!(ui.get::<RadioButton>(radios[1]).unwrap().checked, "clicar de novo não desmarca");
}

#[test]
fn switch_toggles_and_slides() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let sw = ui.add(Switch::new("Notificações"), f);
    ui.set_time(1.0);
    ui.click(sw);
    assert!(ui.get::<Switch>(sw).unwrap().checked);
    ui.set_time(1.05);
    ui.paint(f, &mut NullCanvas);
    assert!(ui.is_animating(), "a bolinha está deslizando");
    ui.set_time(2.0);
    ui.paint(f, &mut NullCanvas);
    assert!(!ui.is_animating());
}

#[test]
fn progress_bar_fraction_and_props() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let p = ui.create("ProgressBar", Some(f)).unwrap();
    ui.set_prop(p, "value", 25).unwrap();
    assert_eq!(ui.get::<ProgressBar>(p).unwrap().fraction(), 0.25);
    ui.set_prop(p, "max", 50).unwrap();
    assert_eq!(ui.get::<ProgressBar>(p).unwrap().fraction(), 0.5);
    ui.set_prop(p, "value", 500).unwrap();
    assert_eq!(ui.get::<ProgressBar>(p).unwrap().fraction(), 1.0);
}

#[test]
fn every_control_can_be_created_by_name_and_painted() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    for class in ["Label", "Button", "Edit", "CheckBox", "RadioButton", "Switch", "ProgressBar", "Panel", "Layout", "Rectangle"] {
        let id = ui.create(class, Some(f)).unwrap_or_else(|e| panic!("{class}: {e}"));
        assert_eq!(ui.class_name(id), Some(class));
    }
    ui.paint(f, &mut NullCanvas);
    let names: Vec<_> = ui.registry().iter().map(|c| c.name).collect();
    assert!(names.contains(&"FloatAnimation") && names.contains(&"ColorAnimation"));
}

// ---------------------------------------------------------------------------
// ScrollBox, Splitter, ComboBox

#[test]
fn scroll_box_scrolls_its_children_and_clamps() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let sb = ui.add(ScrollBox::default(), f);
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    let rows: Vec<_> = (0..10).map(|i| {
        let r = ui.add(Label::new(format!("linha {i}")), sb);
        ui.control_mut(r).unwrap().set_align(Align::Top).set_height(30.0);
        r
    }).collect();
    ui.layout();
    assert_eq!(ui.control(sb).unwrap().content_size().height, 300.0);
    assert_eq!(ui.get::<ScrollBox>(sb).unwrap().max_scroll(), 200.0);

    let wheel = |dy| WheelEvent { position: PointF::new(50.0, 50.0), delta: PointF::new(0.0, dy), ..Default::default() };
    ui.wheel(f, wheel(-40.0));
    ui.layout();
    assert_eq!(ui.bounds(rows[0]).unwrap().y, -40.0, "conteúdo subiu 40");
    ui.wheel(f, wheel(-4000.0));
    ui.layout();
    assert_eq!(ui.get::<ScrollBox>(sb).unwrap().scroll(), 200.0, "não passa do fim");
    assert_eq!(ui.hit_test(f, PointF::new(20.0, 95.0)), Some(sb), "o último rótulo (hit_test=false) deixa passar para o ScrollBox");

    ScrollBox::scroll_into_view(&mut ui, sb, rows[1]);
    ui.layout();
    assert_eq!(ui.bounds(rows[1]).unwrap().y, 0.0);
}

#[test]
fn splitter_resizes_the_previous_docked_control() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let left = ui.add(Panel::new(120.0, 0.0), f);
    ui.control_mut(left).unwrap().set_align(Align::Left);
    let split = ui.add(Splitter::new(Align::Left), f);
    let rest = ui.add(Panel::new(0.0, 0.0), f);
    ui.control_mut(rest).unwrap().set_align(Align::Client);
    ui.layout();
    assert_eq!(ui.bounds(split).unwrap().x, 120.0);

    ui.pointer_move(f, at(122.0, 50.0));
    assert_eq!(ui.cursor(f), CursorShape::ResizeHorizontal);
    ui.pointer_down(f, at(122.0, 50.0));
    ui.pointer_move(f, at(182.0, 50.0));
    ui.pointer_up(f, at(182.0, 50.0));
    ui.layout();
    assert_eq!(ui.control(left).unwrap().width(), 180.0);
    assert_eq!(ui.bounds(rest).unwrap().x, 186.0);
    let _ = split;
}

#[test]
fn combo_box_opens_picks_and_fires_on_change() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let combo = ui.add(ComboBox::new(&["Aberto", "Faturado", "Cancelado"]), f);
    ui.control_mut(combo).unwrap().set_rect(10.0, 10.0, 150.0, 32.0);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let c = changes.clone();
    ui.on(combo, "on_change", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            c.borrow_mut().push(t.clone());
        }
    });
    ui.layout();

    ui.pointer_move(f, at(50.0, 20.0));
    ui.pointer_down(f, at(50.0, 20.0));
    ui.pointer_up(f, at(50.0, 20.0));
    assert!(ui.get::<ComboBox>(combo).unwrap().is_open());
    ui.layout();
    // segunda opção: lista começa em y = 10 + 32 + 2, linhas de 28 (+2 de borda)
    let y = 44.0 + 2.0 + 28.0 + 10.0;
    ui.pointer_move(f, at(50.0, y));
    ui.pointer_down(f, at(50.0, y));
    ui.pointer_up(f, at(50.0, y));
    let c = ui.get::<ComboBox>(combo).unwrap();
    assert!(!c.is_open());
    assert_eq!(c.text(), "Faturado");

    ui.set_focus(combo);
    ui.key_down(f, key(Key::Down));
    assert_eq!(ui.get::<ComboBox>(combo).unwrap().text(), "Cancelado");
    ui.key_down(f, key(Key::Down));
    assert_eq!(*changes.borrow(), vec!["Faturado", "Cancelado"], "não passa da última nem repete");
    assert_eq!(ui.get_prop(combo, "item_index").unwrap(), PropValue::Int(2));
}

// ---------------------------------------------------------------------------
// Toque

fn finger(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), kind: PointerKind::Touch, pointer_id: 7, ..Default::default() }
}

/// Um ScrollBox de 200x100 com 10 botões de 30 de altura (conteúdo de 300).
fn touch_setup() -> (Ui, ComponentId, ComponentId, Rc<RefCell<u32>>) {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let sb = ui.add(ScrollBox::default(), f);
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    let clicks = Rc::new(RefCell::new(0));
    for i in 0..10 {
        let b = ui.add(Button::new(format!("botão {i}")), sb);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(30.0);
        let clicks = clicks.clone();
        ui.on_click(b, move |_, _| *clicks.borrow_mut() += 1);
    }
    ui.layout();
    (ui, f, sb, clicks)
}

#[test]
fn a_tap_still_clicks() {
    let (mut ui, f, _, clicks) = touch_setup();
    ui.pointer_down(f, finger(50.0, 40.0));
    ui.pointer_move(f, finger(52.0, 43.0)); // tremida do dedo: abaixo do limite
    ui.pointer_up(f, finger(52.0, 43.0));
    assert_eq!(*clicks.borrow(), 1);
}

#[test]
fn dragging_a_finger_scrolls_without_clicking() {
    let (mut ui, f, sb, clicks) = touch_setup();
    ui.set_time(1.0);
    ui.pointer_down(f, finger(50.0, 80.0));
    ui.set_time(1.2);
    ui.pointer_move(f, finger(50.0, 60.0));
    ui.set_time(1.4);
    ui.pointer_move(f, finger(50.0, 30.0));
    assert_eq!(ui.get::<ScrollBox>(sb).unwrap().scroll(), 50.0, "o conteúdo acompanha o dedo 1:1");
    // parou o dedo e soltou depois: sem inércia, e o botão tocado não recebe clique
    ui.set_time(2.0);
    ui.pointer_up(f, finger(50.0, 30.0));
    assert_eq!(*clicks.borrow(), 0);
    assert!(!ui.is_animating());
}

#[test]
fn a_quick_flick_keeps_scrolling_until_it_stops() {
    let (mut ui, f, sb, _) = touch_setup();
    ui.set_time(1.0);
    ui.pointer_down(f, finger(50.0, 90.0));
    ui.set_time(1.016);
    ui.pointer_move(f, finger(50.0, 80.0));
    ui.set_time(1.032);
    ui.pointer_move(f, finger(50.0, 70.0));
    ui.pointer_up(f, finger(50.0, 70.0));
    let after_finger = ui.get::<ScrollBox>(sb).unwrap().scroll();
    assert_eq!(after_finger, 20.0);
    assert!(ui.is_animating(), "inércia em andamento");
    // O primeiro frame chega logo depois de soltar: um passo menor que um ponto não
    // pode ser confundido com "chegou ao fim".
    ui.set_time(1.0322);
    ui.tick();
    assert!(ui.is_animating(), "um passo pequeno não para a inércia");

    let mut t = 1.0322;
    for _ in 0..200 {
        t += 0.016;
        ui.set_time(t);
        ui.tick();
        if !ui.is_animating() {
            break;
        }
    }
    let end = ui.get::<ScrollBox>(sb).unwrap().scroll();
    assert!(end > after_finger + 30.0, "continuou rolando ({end})");
    assert!(!ui.is_animating(), "a inércia termina");
    assert!(end <= 200.0);
}

#[test]
fn splitter_takes_the_finger_drag() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let left = ui.add(Panel::new(120.0, 0.0), f);
    ui.control_mut(left).unwrap().set_align(Align::Left);
    ui.add(Splitter::new(Align::Left), f);
    ui.layout();
    ui.pointer_down(f, finger(122.0, 50.0));
    ui.pointer_move(f, finger(162.0, 50.0));
    ui.pointer_up(f, finger(162.0, 50.0));
    ui.layout();
    assert_eq!(ui.bounds(left).unwrap().width, 160.0);
}

#[test]
fn focusing_an_edit_asks_for_the_keyboard() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let e = ui.add(Edit::new(""), f);
    let b = ui.add(Button::new("ok"), f);
    assert!(!ui.wants_text_input(f));
    ui.set_focus(e);
    assert!(ui.wants_text_input(f));
    ui.set_focus(b);
    assert!(!ui.wants_text_input(f));
}

#[test]
fn the_drag_keeps_its_target_when_other_content_passes_under_the_finger() {
    // ScrollBox com botões e, mais abaixo, outro ScrollBox (como a grade dentro da tela).
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let outer = ui.add(ScrollBox::default(), f);
    ui.control_mut(outer).unwrap().set_rect(0.0, 0.0, 200.0, 200.0);
    for _ in 0..3 {
        let b = ui.add(Button::new("b"), outer);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(50.0);
    }
    let inner = ui.add(ScrollBox::default(), outer);
    ui.control_mut(inner).unwrap().set_align(Align::Top).set_height(150.0);
    for _ in 0..10 {
        let b = ui.add(Button::new("i"), inner);
        ui.control_mut(b).unwrap().set_align(Align::Top).set_height(50.0);
    }
    ui.layout();

    ui.set_time(1.0);
    ui.pointer_down(f, finger(50.0, 140.0)); // no 3º botão do ScrollBox de fora
    for (i, y) in [120.0, 90.0, 60.0, 30.0].into_iter().enumerate() {
        ui.set_time(1.1 + i as f64 * 0.1);
        ui.pointer_move(f, finger(50.0, y)); // o ScrollBox de dentro passa sob o dedo
    }
    assert_eq!(ui.get::<ScrollBox>(outer).unwrap().scroll(), 100.0, "o de fora rola até o fim");
    assert_eq!(ui.get::<ScrollBox>(inner).unwrap().scroll(), 0.0, "o de dentro não foi tocado");
}

#[test]
fn touch_focuses_on_tap_not_when_a_scroll_starts() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let sb = ui.add(ScrollBox::default(), f);
    ui.control_mut(sb).unwrap().set_rect(0.0, 0.0, 200.0, 100.0);
    let mut edits = Vec::new();
    for _ in 0..6 {
        let e = ui.add(Edit::new(""), sb);
        ui.control_mut(e).unwrap().set_align(Align::Top).set_height(40.0);
        edits.push(e);
    }
    ui.layout();

    ui.set_time(1.0);
    ui.pointer_down(f, finger(50.0, 20.0));
    assert_eq!(ui.focused(f), None, "encostar não dá foco");
    ui.set_time(1.1);
    ui.pointer_move(f, finger(50.0, 60.0 - 50.0));
    ui.set_time(1.5);
    ui.pointer_up(f, finger(50.0, 10.0));
    assert_eq!(ui.focused(f), None, "rolar em cima do campo não abre o teclado");

    ui.pointer_down(f, finger(50.0, 50.0));
    ui.pointer_up(f, finger(50.0, 50.0));
    assert!(ui.focused(f).is_some_and(|e| edits.contains(&e)), "um toque dá o foco");
    assert!(ui.wants_text_input(f));
}

#[test]
fn the_safe_area_keeps_content_out_of_the_system_bars() {
    let mut ui = Ui::new();
    let f = form(&mut ui); // 400 x 300
    let header = ui.add(Panel::new(0.0, 50.0), f);
    ui.control_mut(header).unwrap().set_align(Align::Top);
    let body = ui.add(Panel::new(0.0, 0.0), f);
    ui.control_mut(body).unwrap().set_align(Align::Client);
    ui.set_safe_area(f, Bounds::new(0.0, 24.0, 0.0, 40.0)); // barra de status + teclado
    ui.layout();
    assert_eq!(ui.bounds(header).unwrap().y, 24.0);
    let b = ui.bounds(body).unwrap();
    assert_eq!((b.y, b.bottom()), (74.0, 260.0));
    assert_eq!(ui.bounds(f).unwrap().height, 300.0, "o form (o fundo) continua na tela toda");
}

#[test]
fn a_shrinking_scroll_box_keeps_the_focused_field_visible() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let sb = ui.add(ScrollBox::default(), f);
    ui.control_mut(sb).unwrap().set_align(Align::Client);
    let mut edits = Vec::new();
    for _ in 0..6 {
        let e = ui.add(Edit::new(""), sb);
        ui.control_mut(e).unwrap().set_align(Align::Top).set_height(40.0);
        edits.push(e);
    }
    ui.layout();
    ui.set_focus(edits[5]); // o último campo (200..240)
    ui.set_safe_area(f, Bounds::new(0.0, 0.0, 0.0, 150.0)); // o teclado cobre 150
    ui.layout();
    ui.layout();
    let field = ui.absolute_rect(edits[5]).unwrap();
    assert!(field.bottom() <= 150.0, "o campo com foco ficou acima do teclado ({field:?})");
}

// ---------------------------------------------------------------------------
// Forms modais

#[test]
fn a_button_with_modal_result_closes_the_modal_form_after_on_click() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main_form = ui.add_root(Form::new("principal", 400.0, 300.0));
    let dialog = ui.add_root(Form::new("dialogo", 300.0, 150.0));
    let ok = ui.add(Button::new("OK"), dialog);
    ui.set_prop(ok, "modal_result", "ok").unwrap();
    ui.show(main_form).unwrap();

    let result = Rc::new(RefCell::new(None));
    let r = result.clone();
    ui.show_modal(dialog, move |_, res| *r.borrow_mut() = Some(res)).unwrap();

    // 1º clique: o on_click recusa (como "ModalResult := mrNone" no Delphi)
    let refuse = Rc::new(RefCell::new(true));
    let flag = refuse.clone();
    ui.on_click(ok, move |ui, sender| {
        if *flag.borrow() {
            let form = ui.form_of(sender);
            ui.set_modal_result(form, ModalResult::None);
        }
    });
    ui.click(ok);
    assert!(ui.is_open(dialog), "o handler cancelou");
    assert_eq!(*result.borrow(), None);

    // 2º clique: fecha com Ok
    *refuse.borrow_mut() = false;
    ui.click(ok);
    assert!(!ui.is_open(dialog));
    assert_eq!(*result.borrow(), Some(ModalResult::Ok));
}

// ---------------------------------------------------------------------------
// Menus

fn menu_key(k: Key, ctrl: bool) -> KeyEvent {
    KeyEvent { key: k, modifiers: Modifiers { ctrl, ..Default::default() }, ..Default::default() }
}

/// Form com MainMenu: Arquivo (Novo Ctrl+N, -, Recentes > [a.json]), Editar (desabilitado).
fn menu_setup() -> (Ui, ComponentId, ComponentId, Rc<RefCell<Vec<String>>>) {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let bar = ui.add(MainMenu::default(), f);
    let log = Rc::new(RefCell::new(Vec::new()));
    let file = ui.add(MenuItem::new("Arquivo"), bar);
    let new_item = ui.add(MenuItem::new("Novo").with_shortcut("Ctrl+N"), file);
    ui.add(MenuItem::new("-"), file);
    let recent = ui.add(MenuItem::new("Recentes"), file);
    let a = ui.add(MenuItem::new("a.json"), recent);
    let edit = ui.add(MenuItem::new("Editar"), bar);
    ui.set_prop(edit, "enabled", false).unwrap();
    for (id, name) in [(new_item, "novo"), (a, "a.json")] {
        let log = log.clone();
        ui.on_click(id, move |_, _| log.borrow_mut().push(name.to_string()));
    }
    ui.layout();
    // a barra precisa ser pintada uma vez para saber onde estão os itens
    ui.paint(f, &mut NullCanvas);
    (ui, f, bar, log)
}

fn overlay(ui: &Ui, f: ComponentId) -> Option<ComponentId> {
    ui.children(f).iter().copied().find(|&c| ui.class_name(c) == Some("MenuOverlay"))
}

#[test]
fn a_shortcut_works_with_the_menu_closed() {
    let (mut ui, f, _, log) = menu_setup();
    ui.key_down(f, menu_key(Key::Char('n'), true));
    assert_eq!(*log.borrow(), ["novo"]);
    ui.key_down(f, menu_key(Key::Char('n'), false));
    assert_eq!(log.borrow().len(), 1, "sem o Ctrl não é o atalho");
}

#[test]
fn the_main_menu_opens_and_navigates_with_the_keyboard() {
    let (mut ui, f, _, log) = menu_setup();
    // clica em "Arquivo" (primeiro item da barra, perto da esquerda)
    let p = PointerEvent { position: PointF::new(20.0, 15.0), ..Default::default() };
    ui.pointer_move(f, p);
    ui.pointer_down(f, p);
    ui.pointer_up(f, p);
    let o = overlay(&ui, f).expect("menu aberto");
    assert_eq!(ui.focused(f), Some(o));
    // ↓ Novo, ↓ (pula o separador) Recentes, → abre o submenu e vai para a.json, Enter
    for k in [Key::Down, Key::Down, Key::Right, Key::Enter] {
        ui.key_down(f, menu_key(k, false));
    }
    assert_eq!(*log.borrow(), ["a.json"]);
    assert!(overlay(&ui, f).is_none(), "o menu fecha ao escolher");
}

#[test]
fn escape_and_a_click_outside_close_the_menu() {
    let (mut ui, f, _, _) = menu_setup();
    let p = PointerEvent { position: PointF::new(20.0, 15.0), ..Default::default() };
    ui.pointer_down(f, p);
    ui.pointer_up(f, p);
    ui.key_down(f, menu_key(Key::Escape, false));
    assert!(overlay(&ui, f).is_none());

    ui.pointer_down(f, p);
    ui.pointer_up(f, p);
    assert!(overlay(&ui, f).is_some());
    let outside = PointerEvent { position: PointF::new(350.0, 280.0), ..Default::default() };
    ui.pointer_down(f, outside);
    ui.pointer_up(f, outside);
    assert!(overlay(&ui, f).is_none());
}

#[test]
fn a_popup_menu_opens_on_right_click() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let menu = ui.add(PopupMenu::default(), f);
    let copy = ui.add(MenuItem::new("Copiar"), menu);
    let target = Rc::new(RefCell::new(None));
    let a = target.clone();
    ui.on(menu, "on_popup", move |_, _, args| {
        if let EventArgs::Value(PropValue::Component(source)) = args {
            *a.borrow_mut() = *source;
        }
    });
    let panel = ui.add(Panel::new(200.0, 100.0), f);
    ui.set_prop(panel, "popup_menu", PropValue::Component(Some(menu))).unwrap();
    ui.layout();
    let p = PointerEvent { position: PointF::new(50.0, 50.0), button: PointerButton::Secondary, ..Default::default() };
    ui.pointer_down(f, p);
    ui.pointer_up(f, p);
    assert!(overlay(&ui, f).is_some(), "abre no clique direito");
    assert_eq!(*target.borrow(), Some(panel), "on_popup diz em qual controle");
    let clicked = Rc::new(RefCell::new(false));
    let c = clicked.clone();
    ui.on_click(copy, move |_, _| *c.borrow_mut() = true);
    ui.key_down(f, menu_key(Key::Down, false));
    ui.key_down(f, menu_key(Key::Enter, false));
    assert!(*clicked.borrow());
}

#[test]
fn combo_list_follows_the_scroll_and_hides_when_the_combo_leaves_the_view() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let scroll = ui.add(ScrollBox::default(), f);
    ui.control_mut(scroll).unwrap().set_rect(0.0, 0.0, 300.0, 200.0);
    let tall = ui.add(Panel::new(280.0, 1000.0), scroll);
    ui.control_mut(tall).unwrap().set_rect(0.0, 0.0, 280.0, 1000.0);
    let combo = ui.add(ComboBox::new(&["a", "b", "c"]), scroll);
    ui.control_mut(combo).unwrap().set_rect(10.0, 50.0, 150.0, 32.0);
    ui.layout();
    ui.pointer_move(f, at(50.0, 60.0));
    ui.pointer_down(f, at(50.0, 60.0));
    ui.pointer_up(f, at(50.0, 60.0));
    assert!(ui.get::<ComboBox>(combo).unwrap().is_open());
    ui.layout();
    let list = *ui.children(f).last().unwrap();
    assert_ne!(list, scroll);
    assert_eq!(ui.bounds(list).unwrap().y, 84.0);

    // Rola 30: a lista sobe junto com o combo.
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 30.0));
    ui.invalidate();
    ui.layout();
    assert_eq!(ui.bounds(list).unwrap().y, 54.0);
    assert!(ui.control(list).unwrap().visible());

    // O combo saiu da parte visível: a lista some até ele voltar.
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::new(0.0, 300.0));
    ui.invalidate();
    ui.layout();
    assert!(!ui.control(list).unwrap().visible());
    ui.control_mut(scroll).unwrap().set_scroll_offset(PointF::ZERO);
    ui.invalidate();
    ui.layout();
    assert!(ui.control(list).unwrap().visible());
    assert_eq!(ui.bounds(list).unwrap().y, 84.0);
}

#[test]
fn the_designer_shows_a_disabled_button_right_away() {
    // No designer o relógio do Ui não anda: o estilo de desabilitado não pode depender da
    // transição (senão fica parado no de habilitado).
    for design in [true, false] {
        let mut ui = Ui::new();
        ui.set_design_mode(design);
        let f = form(&mut ui);
        let button = ui.add(Button::new("OK"), f);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let s = seen.clone();
        ui.on_paint(button, move |canvas, ctx, paint| {
            s.borrow_mut().push((ctx.state.disabled, ctx.style.opacity));
            paint(canvas, ctx);
        });
        ui.paint(f, &mut NullCanvas);
        ui.set_prop(button, "enabled", false).unwrap();
        ui.paint(f, &mut NullCanvas);
        let seen = seen.borrow();
        let (first, last) = (seen[0], *seen.last().unwrap());
        assert!(last.0);
        if design {
            assert_ne!(first.1, last.1, "no designer, o desabilitado aparece já na primeira pintura");
        } else {
            assert_eq!(first.1, last.1, "em execução, a transição começa do estilo anterior");
        }
    }
}

#[test]
fn an_animation_in_one_window_keeps_running_when_another_window_paints_after() {
    // O demonstrador da suavização (num diálogo) pede quadros; a janela principal, pintada
    // depois, não pode apagar esse pedido.
    let mut ui = Ui::new();
    let main = form(&mut ui);
    let dialog = ui.add_root(Form::new("Suavização", 400.0, 300.0));
    let editor = ui.add(EasingEditor::new(Easing::OutCubic), dialog);
    ui.control_mut(editor).unwrap().set_rect(0.0, 0.0, 320.0, 300.0);
    ui.paint(dialog, &mut NullCanvas);
    assert!(ui.is_animating());
    ui.paint(main, &mut NullCanvas);
    assert!(ui.is_animating(), "a pintura da principal apagou o pedido do diálogo");
    assert!(ui.needs_repaint());

    // Fechado o diálogo, a animação dele não segura mais os quadros.
    ui.free(dialog);
    ui.paint(main, &mut NullCanvas);
    assert!(!ui.is_animating());
}

/// Canvas que guarda as cores usadas (fundo, borda, texto).
#[derive(Default)]
struct ColorCanvas {
    fills: Vec<Color>,
    strokes: Vec<Color>,
    texts: Vec<Color>,
    fonts: Vec<(String, FontSpec)>,
}

impl Canvas for ColorCanvas {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, brush: &Brush) {
        if let Brush::Solid(c) = brush {
            self.fills.push(*c);
        }
    }
    fn stroke_rect(&mut self, _: RectF, _: f32, stroke: &Stroke) {
        if let Brush::Solid(c) = &stroke.brush {
            self.strokes.push(*c);
        }
    }
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, text: &str, _: RectF, style: &TextStyle) {
        self.texts.push(style.color);
        self.fonts.push((text.to_string(), style.font.clone()));
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

#[test]
fn a_button_can_have_its_own_fill_border_and_text_colors() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let button = ui.add(Button::new("Salvar"), f);
    ui.control_mut(button).unwrap().set_rect(10.0, 10.0, 120.0, 36.0);
    ui.set_prop(button, "fill", "#16a34a").unwrap();
    ui.set_prop(button, "border_color", "#14532d").unwrap();
    ui.set_prop(button, "text_color", "#ffffff").unwrap();
    let green = Color::from_hex("#16a34a").unwrap();
    let mut canvas = ColorCanvas::default();
    ui.paint(f, &mut canvas);
    assert!(canvas.fills.contains(&green));
    assert!(canvas.strokes.contains(&Color::from_hex("#14532d").unwrap()));
    assert!(canvas.texts.contains(&Color::WHITE));

    // O mouse em cima: o verde (escuro) clareia, mas continua verde.
    ui.pointer_move(f, at(50.0, 20.0));
    let mut canvas = ColorCanvas::default();
    ui.set_time(10.0);
    ui.paint(f, &mut canvas);
    assert!(!canvas.fills.contains(&green));
    assert!(canvas.fills.iter().any(|c| c.g > green.g && c.g > c.r));

    // Vazio volta para o tema.
    ui.set_prop(button, "fill", PropValue::Null).unwrap();
    assert_eq!(ui.get_prop(button, "fill").unwrap(), PropValue::Null);
}

#[test]
fn the_font_comes_from_the_parent_like_delphi_parent_font() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    ui.set_prop(f, "font_family", "Georgia").unwrap();
    ui.set_prop(f, "font_size", 20.0).unwrap();
    let panel = ui.add(Panel::new(300.0, 200.0), f);
    let inherited = ui.add(Label::new("herdada"), panel);
    let own = ui.add(Label::new("própria"), panel);
    ui.set_prop(own, "font_weight", "bold").unwrap();
    ui.set_prop(own, "font_italic", true).unwrap();
    ui.set_prop(own, "font_underline", true).unwrap();
    ui.set_prop(own, "font_strikeout", true).unwrap();
    let button = ui.add(Button::new("botão"), f);
    ui.set_prop(button, "font_size", 11.0).unwrap();
    let mut canvas = ColorCanvas::default();
    ui.paint(f, &mut canvas);
    let font = |text: &str| canvas.fonts.iter().find(|(t, _)| t == text).map(|(_, f)| f.clone()).unwrap();
    let a = font("herdada");
    assert_eq!(ui.font_of(inherited), a, "fora da pintura (medir texto) a fonte é a mesma");
    assert_eq!((a.family.as_str(), a.size, a.weight, a.italic), ("Georgia", 20.0, FontWeight::Normal, false));
    let b = font("própria");
    assert_eq!((b.family.as_str(), b.size, b.weight), ("Georgia", 20.0, FontWeight::Bold), "o resto vem do form");
    assert!(b.italic && b.underline && b.strikeout);
    let c = font("botão");
    assert_eq!((c.family.as_str(), c.size), ("Georgia", 11.0), "vale para qualquer controle, não só o Label");

    // Gravado e lido de volta pelo nome; o padrão volta para o herdado.
    assert_eq!(ui.get_prop(own, "font_weight").unwrap(), PropValue::Enum("bold".into()));
    ui.set_prop(own, "font_weight", "default").unwrap();
    assert_eq!(ui.get_prop(own, "font_weight").unwrap(), PropValue::Enum("default".into()));
    assert!(ui.set_prop(own, "font_weight", "gordo").is_err());
}

#[test]
fn text_measured_outside_painting_uses_the_controls_own_font() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    ui.set_prop(f, "font_size", 30.0).unwrap();
    let panel = ui.add(Panel::new(200.0, 100.0), f);
    let label = ui.add(Label::new("x"), panel);
    ui.set_prop(label, "font_family", "Georgia").unwrap();
    let font = ui.font_of(label);
    assert_eq!((font.family.as_str(), font.size), ("Georgia", 30.0), "o próprio e o do form");

    // O menu mede a largura com a fonte do form: com letra grande, o painel é mais largo.
    let width_with = |ui: &mut Ui, size: f32| {
        ui.set_prop(f, "font_size", size).unwrap();
        let menu = ui.add(PopupMenu::default(), f);
        ui.add(MenuItem::new("Um item de menu com um texto bem comprido"), menu);
        PopupMenu::popup(ui, menu, PointF::new(10.0, 10.0));
        let overlay = *ui.children(f).iter().find(|&&c| ui.class_name(c) == Some("MenuOverlay")).unwrap();
        let w = ui.access_items(overlay)[0].rect.unwrap().width;
        ui.free(menu);
        w
    };
    let small = width_with(&mut ui, 12.0);
    let big = width_with(&mut ui, 30.0);
    assert!(big > small * 1.8, "{small} → {big}");
}

/// O `window_state` do form: ao mudar com o app rodando, pede à janela (tela cheia,
/// maximizar...); no designer, não.
#[test]
fn form_window_state_drives_the_window() {
    use aylis_core::WindowCommand as W;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.create("Form", None).unwrap();
    ui.set_prop(f, "window_state", aylis_core::PropValue::Enum("full_screen".into())).unwrap();
    assert!(ui.take_window_commands().contains(&(f, W::SetFullscreen(true))));
    ui.set_prop(f, "window_state", aylis_core::PropValue::Enum("normal".into())).unwrap();
    let commands = ui.take_window_commands();
    assert!(commands.contains(&(f, W::SetFullscreen(false))) && commands.contains(&(f, W::Restore)), "{commands:?}");
    ui.set_design_mode(true);
    ui.set_prop(f, "window_state", aylis_core::PropValue::Enum("maximized".into())).unwrap();
    assert!(ui.take_window_commands().is_empty(), "no designer não há janela");
}

/// Arrastar uma aba para longe da faixa e soltar dispara `on_tear_off` com o índice.
#[test]
fn tab_strip_tear_off() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let f = ui.create("Form", None).unwrap();
    let mut strip = TabStrip::new();
    strip.add("a.rs".to_string(), "file");
    strip.add("b.rs".to_string(), "file");
    let s = ui.add(strip, f);
    ui.control_mut(s).unwrap().set_align(Align::Top).set_height(38.0);
    ui.layout();
    ui.paint(f, &mut NullCanvas);
    let torn = Rc::new(RefCell::new(None));
    let t = torn.clone();
    ui.on(s, "on_tear_off", move |_, _, args| {
        if let EventArgs::Value(v) = args {
            *t.borrow_mut() = v.to_i64().ok().map(|i| i.to_string());
        }
    });
    let at = |x: f32, y: f32| PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_down(f, at(40.0, 19.0));
    ui.pointer_move(f, at(60.0, 19.0));
    ui.pointer_move(f, at(60.0, 300.0));
    ui.pointer_up(f, at(60.0, 300.0));
    assert_eq!(torn.borrow().as_deref(), Some("0"));
}

/// Um form embutido em outro (o editor de código na IDE): com o foco dentro dele, o atalho
/// do menu dele vale mais que o do form de fora; com o foco fora, vale o de fora.
#[test]
fn shortcuts_prefer_the_embedded_form_with_focus() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let outer = ui.create("Form", None).unwrap();
    let hits = Rc::new(RefCell::new(Vec::new()));
    let menu_with = |ui: &mut Ui, form: ComponentId, who: &'static str| {
        let menu = ui.add(MainMenu::default(), form);
        let top = ui.add(MenuItem::new("Arquivo"), menu);
        let item = ui.add(MenuItem::new("Salvar").with_shortcut("Ctrl+S"), top);
        let hits = hits.clone();
        ui.on_click(item, move |_, _| hits.borrow_mut().push(who));
    };
    menu_with(&mut ui, outer, "fora");
    let outside_edit = ui.add(Edit::new(""), outer);
    let inner = ui.create("Form", None).unwrap();
    ui.set_parent(inner, Some(outer)).unwrap();
    menu_with(&mut ui, inner, "dentro");
    let inside_edit = ui.add(Edit::new(""), inner);
    ui.layout();
    let (modifiers, key) = parse_shortcut("Ctrl+S").unwrap();
    let ctrl_s = KeyEvent { key, modifiers, repeat: false };
    ui.set_focus(inside_edit);
    ui.key_down(outer, ctrl_s);
    ui.set_focus(outside_edit);
    ui.key_down(outer, ctrl_s);
    assert_eq!(*hits.borrow(), vec!["dentro", "fora"]);
}

/// O Edit desfaz e refaz o que o usuário digitou (Ctrl+Z, Ctrl+Y, Ctrl+Shift+Z) e marca a tecla
/// como tratada: o Ctrl+Z num campo não chega ao atalho do menu (o desfazer do form).
#[test]
fn edit_undoes_and_redoes_what_was_typed() {
    let key = |key: Key, ctrl: bool, shift: bool| KeyEvent { key, modifiers: Modifiers { ctrl, shift, ..Default::default() }, repeat: false };
    let mut ui = Ui::new();
    register(&mut ui);
    let form = ui.add_root(Form::new("Teste", 400.0, 200.0));
    let edit = ui.add(Edit::new("abc"), form);
    ui.layout();
    ui.set_focus(edit);
    let text = |ui: &Ui| ui.get::<Edit>(edit).unwrap().text.clone();
    // O foco seleciona tudo: o cursor vai para o fim antes de digitar.
    ui.key_down(form, key(Key::End, false, false));
    ui.text_input(form, "d");
    ui.text_input(form, "e");
    assert_eq!(text(&ui), "abcde");
    // As duas teclas seguidas são um passo só.
    ui.key_down(form, key(Key::Char('z'), true, false));
    assert_eq!(text(&ui), "abc");
    ui.key_down(form, key(Key::Char('y'), true, false));
    assert_eq!(text(&ui), "abcde");
    ui.key_down(form, key(Key::Char('z'), true, false));
    ui.key_down(form, key(Key::Char('Z'), true, true));
    assert_eq!(text(&ui), "abcde");
    // O texto posto pelo código recomeça o histórico.
    ui.set_prop(edit, "text", "novo").unwrap();
    ui.key_down(form, key(Key::Char('z'), true, false));
    assert_eq!(text(&ui), "novo");
}

/// A lista aberta fecha com o clique fora, mesmo onde nada recebe foco (o fundo do form, um
/// rótulo): antes só a perda de foco do ComboBox fechava. Clicar na lista ou no próprio
/// ComboBox não conta como fora.
#[test]
fn combo_list_closes_on_a_press_outside() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let combo = ui.add(ComboBox::new(&["Aberto", "Faturado", "Cancelado"]), f);
    ui.control_mut(combo).unwrap().set_rect(10.0, 10.0, 150.0, 32.0);
    let label = ui.add(Label::new("Sem foco"), f);
    ui.control_mut(label).unwrap().set_rect(250.0, 200.0, 100.0, 20.0);
    ui.layout();
    let click = |ui: &mut Ui, x: f32, y: f32| {
        ui.pointer_move(f, at(x, y));
        ui.pointer_down(f, at(x, y));
        ui.pointer_up(f, at(x, y));
        ui.layout();
    };
    let open = |ui: &Ui| ui.get::<ComboBox>(combo).unwrap().is_open();
    click(&mut ui, 50.0, 20.0);
    assert!(open(&ui));
    // No fundo do form (nada recebe foco ali).
    click(&mut ui, 300.0, 280.0);
    assert!(!open(&ui), "o clique no fundo fecha");
    click(&mut ui, 50.0, 20.0);
    assert!(open(&ui));
    // Num rótulo.
    click(&mut ui, 280.0, 210.0);
    assert!(!open(&ui), "o clique no rótulo fecha");
    // Clicar no ComboBox aberto continua fechando e abrindo por ele mesmo.
    click(&mut ui, 50.0, 20.0);
    click(&mut ui, 50.0, 20.0);
    assert!(!open(&ui));
}
