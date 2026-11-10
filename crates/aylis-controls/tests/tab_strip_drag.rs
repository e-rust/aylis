//! TabStrip: a aba puxada para fora vira arrasto (com a prévia), passa para outra faixa (em
//! outra janela também), volta para a própria, vira janela nova ou leva a janela inteira.

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

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), ..Default::default() }
}

/// Um form aberto com uma faixa no topo e, embaixo, um Layout com as páginas `titles`.
fn window(ui: &mut Ui, titles: &[&str]) -> (ComponentId, ComponentId, Vec<ComponentId>) {
    let form = ui.create("Form", None).unwrap();
    ui.control_mut(form).unwrap().set_size(800.0, 500.0);
    let strip = ui.add(TabStrip::new(), form);
    ui.control_mut(strip).unwrap().set_align(Align::Top).set_height(38.0);
    let host = ui.add(Layout::default(), form);
    ui.control_mut(host).unwrap().set_align(Align::Client);
    ui.get_mut::<TabStrip>(strip).unwrap().page_host = Some(host);
    let pages = titles
        .iter()
        .map(|t| {
            let page = ui.add(Panel::default(), host);
            TabStrip::add_page(ui, strip, t, "description", page);
            page
        })
        .collect();
    ui.show(form).unwrap();
    ui.layout();
    ui.paint(form, &mut NullCanvas);
    (form, strip, pages)
}

fn titles(ui: &Ui, strip: ComponentId) -> Vec<String> {
    ui.get::<TabStrip>(strip).unwrap().tabs().iter().map(|t| t.title.clone()).collect()
}

fn visible(ui: &Ui, id: ComponentId) -> bool {
    ui.control(id).is_some_and(|c| c.visible())
}

/// Aperta a aba em `x` e puxa para baixo, para fora da faixa: o arrasto começa.
fn lift(ui: &mut Ui, form: ComponentId, x: f32) {
    ui.pointer_down(form, at(x, 19.0));
    ui.pointer_move(form, at(x + 20.0, 19.0));
    ui.pointer_move(form, at(x + 20.0, 300.0));
    assert!(ui.dragging(form), "a aba puxada para fora vira arrasto");
}

#[test]
fn a_tab_goes_to_the_strip_of_another_window_with_its_page() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, pa) = window(&mut ui, &["a1", "a2"]);
    let (fb, sb, pb) = window(&mut ui, &["b1"]);
    let dropped = Rc::new(RefCell::new(None));
    let d = dropped.clone();
    ui.on(sb, "on_tab_drop", move |ui, sender, _| *d.borrow_mut() = ui.get::<TabStrip>(sender).unwrap().last_drop());

    // a2 (a escolhida) sai da janela A e é solta no fim da faixa da B.
    lift(&mut ui, fa, 300.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(700.0, 19.0));
    assert_eq!(ui.get::<TabStrip>(sb).unwrap().tab_index, 0, "ainda não soltou");
    ui.pointer_up(fa, at(700.0, 300.0));

    assert_eq!(titles(&ui, sa), ["a1"]);
    assert_eq!(titles(&ui, sb), ["b1", "a2"]);
    assert_eq!(ui.get::<TabStrip>(sb).unwrap().tab_index, 1, "a aba que chegou fica escolhida");
    assert_eq!(*dropped.borrow(), Some(TabDrop { source: sa, from: 1, to: 1 }));
    // A página foi junto e é a que aparece; na A, aparece a que ficou.
    let host_b = ui.get::<TabStrip>(sb).unwrap().page_host.unwrap();
    assert_eq!(ui.parent(pa[1]), Some(host_b));
    assert!(visible(&ui, pa[1]) && !visible(&ui, pb[0]));
    assert!(visible(&ui, pa[0]));
    assert!(ui.is_open(fa), "a janela A ainda tem aba");
}

#[test]
fn back_on_its_own_strip_the_tab_only_moves() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, _) = window(&mut ui, &["a1", "a2", "a3"]);
    lift(&mut ui, fa, 40.0);
    // Volta para a faixa, depois da última aba.
    ui.pointer_move(fa, at(790.0, 19.0));
    ui.pointer_up(fa, at(790.0, 19.0));
    assert_eq!(titles(&ui, sa), ["a2", "a3", "a1"]);
    assert_eq!(ui.open_forms().len(), 1, "nenhuma janela nova");
}

#[test]
fn dropped_far_away_the_tab_becomes_a_window() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, pa) = window(&mut ui, &["a1", "a2"]);
    let changes = Rc::new(RefCell::new(0));
    let c = changes.clone();
    ui.on(sa, "on_change", move |_, _, _| *c.borrow_mut() += 1);
    let created = Rc::new(RefCell::new(None));
    let cr = created.clone();
    ui.on(sa, "on_new_window", move |_, _, args| {
        if let EventArgs::Value(PropValue::Component(s)) = args {
            *cr.borrow_mut() = *s;
        }
    });

    lift(&mut ui, fa, 40.0);
    // A plataforma diz onde o mouse está na tela; fora de todas as janelas, sem destino.
    ui.set_screen_pointer(Some(ScreenPointer { x: 1500, y: 600, scale: 1.0 }));
    ui.drag_over_form(fa, None, PointF::default());
    ui.pointer_up(fa, at(900.0, 700.0));

    assert_eq!(titles(&ui, sa), ["a2"]);
    let new_strip = created.borrow().expect("on_new_window com a faixa nova");
    let new_form = ui.root_of(new_strip);
    assert!(ui.is_open(new_form));
    assert_eq!(ui.form_owner(new_form), None, "janela independente, como as do navegador");
    assert_eq!(titles(&ui, new_strip), ["a1"]);
    assert_eq!(ui.root_of(pa[0]), new_form, "a página foi para a janela nova");
    assert!(visible(&ui, pa[0]) && visible(&ui, pa[1]));
    // Onde o mouse soltou: o ponto pego da aba fica sob ele.
    let p = ui.window_placement(new_form).expect("posição da janela nova");
    assert!(p.x < 1500 && p.x > 1400 && p.y < 600 && p.y > 540, "{p:?}");
    // A faixa nova responde com os mesmos handlers.
    let before = *changes.borrow();
    ui.get_mut::<TabStrip>(new_strip).unwrap().add("x", "");
    ui.layout();
    ui.paint(new_form, &mut NullCanvas);
    ui.pointer_down(new_form, at(300.0, 19.0));
    assert_eq!(*changes.borrow(), before + 1);
}

#[test]
fn the_window_with_one_tab_moves_or_joins_another() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, _) = window(&mut ui, &["a1"]);
    let (fb, sb, pb) = window(&mut ui, &["b1"]);
    let _ = ui.take_window_commands();

    // Solta longe: a janela inteira vai para lá.
    lift(&mut ui, fb, 40.0);
    ui.set_screen_pointer(Some(ScreenPointer { x: 900, y: 400, scale: 1.0 }));
    ui.drag_over_form(fb, None, PointF::default());
    ui.pointer_up(fb, at(900.0, 700.0));
    let moved = ui.take_window_commands();
    assert!(matches!(moved.as_slice(), [(f, WindowCommand::MoveTo(x, y))] if *f == fb && *x < 900 && *y < 400), "{moved:?}");
    assert_eq!(ui.open_forms().len(), 2);

    // Solta na faixa da outra: junta-se a ela e a janela vazia fecha (mesmo sendo a principal).
    assert_eq!(ui.main_form(), Some(fa));
    lift(&mut ui, fa, 40.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(20.0, 19.0));
    ui.pointer_up(fa, at(20.0, 300.0));
    assert_eq!(titles(&ui, sb), ["a1", "b1"]);
    assert!(titles(&ui, sa).is_empty());
    assert!(!ui.is_open(fa), "a janela que ficou sem abas fecha");
    assert_eq!(ui.main_form(), Some(fb));
    assert!(!visible(&ui, pb[0]));
}

#[test]
fn esc_and_other_groups_keep_the_tab() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, _) = window(&mut ui, &["a1", "a2"]);
    let (fb, sb, _) = window(&mut ui, &["b1"]);
    let torn = Rc::new(RefCell::new(0));
    let t = torn.clone();
    // Recusar o on_tear_off: sem janela nova.
    ui.on(sa, "on_tear_off", move |ui, _, _| {
        *t.borrow_mut() += 1;
        ui.refuse("");
    });

    lift(&mut ui, fa, 40.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(700.0, 19.0));
    assert!(ui.cancel_drag(fa));
    assert_eq!(titles(&ui, sa), ["a1", "a2"]);
    assert_eq!(titles(&ui, sb), ["b1"]);

    // Outro grupo não recebe: soltar ali é soltar em lugar nenhum.
    ui.get_mut::<TabStrip>(sb).unwrap().drop_group = "outro".into();
    lift(&mut ui, fa, 40.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(700.0, 19.0));
    ui.pointer_up(fa, at(700.0, 300.0));
    assert_eq!(titles(&ui, sb), ["b1"]);
    assert_eq!(*torn.borrow(), 1, "virou on_tear_off");
    assert_eq!(titles(&ui, sa), ["a1", "a2"], "recusado: a aba fica");
    assert_eq!(ui.open_forms().len(), 2);
}

/// Faixas sem páginas (como as da IDE): uma aba de outra faixa só entra onde há
/// `on_tab_drop` (quem usa leva o conteúdo); sem ele, soltar ali é soltar em lugar nenhum.
#[test]
fn strips_without_pages_receive_only_with_on_tab_drop() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let (fa, sa, _) = window(&mut ui, &[]);
    let (fb, sb, _) = window(&mut ui, &[]);
    for (s, f) in [(sa, fa), (sb, fb)] {
        ui.get_mut::<TabStrip>(s).unwrap().set_tabs_text("x;y".into());
        ui.layout();
        ui.paint(f, &mut NullCanvas);
    }
    let torn = Rc::new(RefCell::new(0));
    let t = torn.clone();
    ui.on(sa, "on_tear_off", move |_, _, _| *t.borrow_mut() += 1);

    lift(&mut ui, fa, 40.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(700.0, 19.0));
    ui.pointer_up(fa, at(700.0, 300.0));
    assert_eq!(titles(&ui, sb), ["x", "y"]);
    assert_eq!(*torn.borrow(), 1);

    ui.on(sb, "on_tab_drop", |_, _, _| {});
    lift(&mut ui, fa, 40.0);
    ui.drag_over_form(fa, Some(fb), PointF::new(700.0, 19.0));
    ui.pointer_up(fa, at(700.0, 300.0));
    assert_eq!(titles(&ui, sa), ["y"]);
    assert_eq!(titles(&ui, sb), ["x", "y", "x"]);
    assert!(ui.is_open(fa), "sem páginas, a janela não fecha sozinha");
}
