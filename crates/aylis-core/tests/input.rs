mod common;

use std::cell::RefCell;
use std::rc::Rc;

use common::{Boxy, boxy};
use aylis_core::*;

/// Botão de teste: recebe foco, Enter clica, roda é tratada se `scrolls`.
struct Btn {
    base: ControlBase,
    scrolls: bool,
}

fn btn(w: f32, h: f32) -> Btn {
    let mut base = ControlBase::new(w, h);
    base.set_can_focus(true).set_cursor(CursorShape::Pointer);
    Btn { base, scrolls: false }
}

impl Component for Btn {
    fn class_name(&self) -> &'static str {
        "Btn"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Enter {
            self.click(ctx);
            ctx.set_handled();
        }
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, _event: &WheelEvent) {
        if self.scrolls {
            ctx.set_handled();
        }
    }
}

type Log = Rc<RefCell<Vec<String>>>;

fn log_events(ui: &mut Ui, id: ComponentId, name: &'static str, log: &Log) {
    for event in [
        "on_click",
        "on_dbl_click",
        "on_pointer_down",
        "on_pointer_up",
        "on_pointer_enter",
        "on_pointer_leave",
        "on_enter",
        "on_exit",
        "on_key_down",
        "on_wheel",
    ] {
        let log = log.clone();
        ui.on(id, event, move |_, _, _| log.borrow_mut().push(format!("{name}.{event}")));
    }
}

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

/// Form 200x100 com dois botões lado a lado: a em (0,0,100,100) e b em (100,0,100,100).
fn setup() -> (Ui, ComponentId, ComponentId, ComponentId, Log) {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(200.0, 100.0));
    let a = ui.add(btn(100.0, 0.0), form);
    ui.control_mut(a).unwrap().set_align(Align::Left);
    let b = ui.add(btn(0.0, 0.0), form);
    ui.control_mut(b).unwrap().set_align(Align::Client);
    let log: Log = Rc::default();
    log_events(&mut ui, a, "a", &log);
    log_events(&mut ui, b, "b", &log);
    ui.layout();
    (ui, form, a, b, log)
}

fn take(log: &Log) -> Vec<String> {
    std::mem::take(&mut *log.borrow_mut())
}

#[test]
fn hit_test_picks_topmost_and_respects_hit_test_flag() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let under = ui.add(boxy(100.0, 100.0), form);
    let over = ui.add(boxy(50.0, 50.0), form);
    assert_eq!(ui.hit_test(form, PointF::new(10.0, 10.0)), Some(over));
    assert_eq!(ui.hit_test(form, PointF::new(80.0, 80.0)), Some(under));

    // hit_test = false: o ponteiro passa para o que está atrás, mas os filhos ainda recebem
    ui.control_mut(over).unwrap().set_hit_test(false);
    let child = ui.add(boxy(10.0, 10.0), over);
    ui.layout();
    assert_eq!(ui.hit_test(form, PointF::new(20.0, 20.0)), Some(under));
    assert_eq!(ui.hit_test(form, PointF::new(5.0, 5.0)), Some(child));
    assert_eq!(ui.hit_test(form, PointF::new(500.0, 5.0)), None);
}

#[test]
fn hover_enter_leave_and_state() {
    let (mut ui, form, a, b, log) = setup();
    ui.pointer_move(form, at(10.0, 10.0));
    assert!(ui.control(a).unwrap().state().hover);
    ui.pointer_move(form, at(20.0, 10.0));
    ui.pointer_move(form, at(150.0, 10.0));
    assert!(!ui.control(a).unwrap().state().hover);
    assert!(ui.control(b).unwrap().state().hover);
    ui.pointer_leave(form);
    assert_eq!(ui.hovered(form), None);
    assert_eq!(take(&log), vec![
        "a.on_pointer_enter",
        "a.on_pointer_leave",
        "b.on_pointer_enter",
        "b.on_pointer_leave",
    ]);
}

#[test]
fn click_only_when_released_over_the_same_control() {
    let (mut ui, form, _a, _b, log) = setup();
    ui.pointer_down(form, at(10.0, 10.0));
    ui.pointer_up(form, at(12.0, 10.0));
    let events = take(&log);
    assert!(events.contains(&"a.on_click".to_string()), "{events:?}");

    // pressiona em a, solta em b: sem clique, e o soltar vai para a (captura)
    ui.pointer_down(form, at(10.0, 10.0));
    ui.pointer_move(form, at(150.0, 10.0));
    ui.pointer_up(form, at(150.0, 10.0));
    let events = take(&log);
    assert!(events.contains(&"a.on_pointer_up".to_string()), "{events:?}");
    assert!(!events.iter().any(|e| e.ends_with("on_click")), "{events:?}");
}

#[test]
fn pressed_state_follows_the_button() {
    let (mut ui, form, a, _b, _log) = setup();
    ui.pointer_down(form, at(10.0, 10.0));
    assert!(ui.control(a).unwrap().state().pressed);
    ui.pointer_up(form, at(10.0, 10.0));
    assert!(!ui.control(a).unwrap().state().pressed);
}

#[test]
fn pointer_positions_are_local_to_the_control() {
    let (mut ui, form, _a, b, _log) = setup();
    let seen = Rc::new(RefCell::new(None));
    let s = seen.clone();
    ui.on(b, "on_pointer_down", move |_, _, args| {
        if let EventArgs::Pointer(e) = args {
            *s.borrow_mut() = Some(e.position);
        }
    });
    ui.pointer_down(form, at(130.0, 40.0));
    assert_eq!(*seen.borrow(), Some(PointF::new(30.0, 40.0)));
}

#[test]
fn double_click_depends_on_time() {
    let (mut ui, form, _a, _b, log) = setup();
    ui.set_time(1.0);
    ui.pointer_down(form, at(10.0, 10.0));
    ui.pointer_up(form, at(10.0, 10.0));
    ui.set_time(1.2);
    ui.pointer_down(form, at(11.0, 10.0));
    ui.pointer_up(form, at(11.0, 10.0));
    assert!(take(&log).contains(&"a.on_dbl_click".to_string()));

    ui.set_time(5.0);
    ui.pointer_down(form, at(10.0, 10.0));
    ui.set_time(6.0);
    ui.pointer_down(form, at(10.0, 10.0));
    assert!(!take(&log).contains(&"a.on_dbl_click".to_string()));
}

#[test]
fn disabled_controls_receive_nothing() {
    let (mut ui, form, a, _b, log) = setup();
    ui.control_mut(a).unwrap().set_enabled(false);
    ui.pointer_move(form, at(10.0, 10.0));
    ui.pointer_down(form, at(10.0, 10.0));
    ui.pointer_up(form, at(10.0, 10.0));
    assert!(take(&log).is_empty());
    assert!(!ui.set_focus(a));
}

#[test]
fn click_gives_focus_and_tab_cycles() {
    let (mut ui, form, a, b, log) = setup();
    ui.pointer_down(form, at(150.0, 10.0));
    assert_eq!(ui.focused(form), Some(b));
    assert!(ui.control(b).unwrap().state().focused);
    take(&log);

    let tab = KeyEvent { key: Key::Tab, ..Default::default() };
    ui.key_down(form, tab);
    assert_eq!(ui.focused(form), Some(a), "Tab volta ao primeiro");
    let shift_tab = KeyEvent { key: Key::Tab, modifiers: Modifiers { shift: true, ..Default::default() }, ..Default::default() };
    ui.key_down(form, shift_tab);
    assert_eq!(ui.focused(form), Some(b));

    let events = take(&log);
    assert!(events.contains(&"b.on_exit".to_string()) && events.contains(&"a.on_enter".to_string()), "{events:?}");

    // tab_stop = false fica fora do Tab, mas ainda recebe foco por clique
    ui.control_mut(a).unwrap().set_tab_stop(false);
    ui.key_down(form, tab);
    assert_eq!(ui.focused(form), Some(b));
}

#[test]
fn keys_go_to_the_focused_control_and_bubble() {
    let (mut ui, form, a, _b, log) = setup();
    let form_keys = Rc::new(RefCell::new(0));
    let k = form_keys.clone();
    ui.on(form, "on_key_down", move |_, _, _| *k.borrow_mut() += 1);
    ui.set_focus(a);
    take(&log);

    // Enter é tratado pelo botão: clica e não sobe
    ui.key_down(form, KeyEvent { key: Key::Enter, ..Default::default() });
    assert_eq!(take(&log), vec!["a.on_key_down", "a.on_click"]);
    assert_eq!(*form_keys.borrow(), 0);

    // outra tecla não é tratada: sobe até o form
    ui.key_down(form, KeyEvent { key: Key::Char('x'), ..Default::default() });
    assert_eq!(*form_keys.borrow(), 1);
}

#[test]
fn wheel_bubbles_until_handled() {
    let mut ui = Ui::new();
    let form = ui.add_root(boxy(100.0, 100.0));
    let outer = ui.add(btn(100.0, 100.0), form);
    let inner = ui.add(boxy(50.0, 50.0), outer);
    let log: Log = Rc::default();
    for (id, name) in [(form, "form"), (outer, "outer"), (inner, "inner")] {
        let l = log.clone();
        ui.on(id, "on_wheel", move |_, _, _| l.borrow_mut().push(name.to_string()));
    }
    ui.get_mut::<Btn>(outer).unwrap().scrolls = true;
    ui.wheel(form, WheelEvent { position: PointF::new(10.0, 10.0), ..Default::default() });
    assert_eq!(take(&log), vec!["inner", "outer"]);
}

#[test]
fn cursor_comes_from_the_hovered_control_or_its_parents() {
    let (mut ui, form, a, _b, _log) = setup();
    let inner = ui.add(boxy(10.0, 10.0), a);
    ui.layout();
    ui.pointer_move(form, at(5.0, 5.0));
    assert_eq!(ui.hovered(form), Some(inner));
    assert_eq!(ui.cursor(form), CursorShape::Pointer, "herdado do botão");
    ui.pointer_leave(form);
    assert_eq!(ui.cursor(form), CursorShape::Default);
}

#[test]
fn freeing_forgets_hover_focus_and_capture() {
    let (mut ui, form, a, _b, _log) = setup();
    ui.pointer_down(form, at(10.0, 10.0));
    assert_eq!(ui.focused(form), Some(a));
    ui.free(a);
    assert_eq!(ui.focused(form), None);
    assert_eq!(ui.hovered(form), None);
    ui.pointer_up(form, at(10.0, 10.0)); // não pode quebrar
}

#[test]
fn hit_test_false_children_route_to_the_parent() {
    // Um texto dentro do botão (hit_test = false) não rouba o clique.
    let (mut ui, form, a, _b, log) = setup();
    let label = ui.add(boxy(50.0, 20.0), a);
    ui.control_mut(label).unwrap().set_hit_test(false);
    ui.layout();
    ui.pointer_down(form, at(5.0, 5.0));
    ui.pointer_up(form, at(5.0, 5.0));
    assert!(take(&log).contains(&"a.on_click".to_string()));
    assert!(ui.get::<Boxy>(label).is_some());
}

/// J03 (relatório do Tabernáculo): o primeiro clique refaz a tela e põe outro controle no
/// mesmo lugar; o segundo clique, rápido, não é duplo para o controle novo (ele recebeu um
/// clique só). Vale para o controle liberado (no handler ou depois, pelo `free_later`) e para
/// o escondido.
#[test]
fn a_double_click_goes_only_to_the_control_that_got_both_clicks() {
    for how in ["free", "free_later", "hide"] {
        let (mut ui, form, a, _b, log) = setup();
        let replacement: Rc<RefCell<Option<ComponentId>>> = Rc::default();
        let (r, l) = (replacement.clone(), log.clone());
        ui.on(a, "on_click", move |ui, sender, _| {
            match how {
                "free" => {
                    ui.free(sender);
                }
                "free_later" => ui.free_later(sender),
                _ => {
                    ui.control_mut(sender).unwrap().set_visible(false);
                }
            }
            let c = ui.add(btn(100.0, 0.0), form);
            ui.control_mut(c).unwrap().set_align(Align::Left);
            log_events(ui, c, "c", &l);
            *r.borrow_mut() = Some(c);
        });
        ui.set_time(1.0);
        ui.pointer_down(form, at(10.0, 10.0));
        ui.pointer_up(form, at(10.0, 10.0));
        ui.process_events();
        // O `free_later` roda na volta dos relógios.
        ui.advance(0.05);
        ui.layout();
        let c = replacement.borrow().expect("o clique refez a tela");
        assert_eq!(ui.hit_test(form, PointF::new(11.0, 10.0)), Some(c), "{how}");
        take(&log);
        let count = Rc::new(RefCell::new(0u8));
        let n = count.clone();
        ui.on(c, "on_pointer_down", move |_, _, args| {
            if let EventArgs::Pointer(e) = args {
                *n.borrow_mut() = e.click_count;
            }
        });
        ui.set_time(1.2);
        ui.pointer_down(form, at(11.0, 10.0));
        ui.pointer_up(form, at(11.0, 10.0));
        let events = take(&log);
        assert!(!events.contains(&"c.on_dbl_click".to_string()), "{how}: {events:?}");
        assert_eq!(*count.borrow(), 1, "{how}: o controle novo recebeu o primeiro clique");
        // O terceiro, rápido, já é o duplo do controle novo.
        ui.set_time(1.4);
        ui.pointer_down(form, at(11.0, 10.0));
        assert!(take(&log).contains(&"c.on_dbl_click".to_string()), "{how}");
    }
}
