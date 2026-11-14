//! O deslizar do dedo (`on_swipe`, decidido em 14/11): um arrasto rápido que não rolou nada no
//! mesmo eixo vai ao controle tocado e sobe; o `TabControl` com `swipe` troca de página, o
//! `Carousel` de item, e a `NavigationDrawer` abre pela borda do form.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn touch(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), kind: PointerKind::Touch, pointer_id: 5, ..Default::default() }
}

/// Um deslizar de `from` a `to` em 4 passos de 10 ms (rápido), soltando no fim.
fn swipe(ui: &mut Ui, form: ComponentId, from: (f32, f32), to: (f32, f32)) {
    ui.pointer_down(form, touch(from.0, from.1));
    for i in 1..=4 {
        let t = ui.time() + 0.01;
        ui.set_time(t);
        let k = i as f32 / 4.0;
        ui.pointer_move(form, touch(from.0 + (to.0 - from.0) * k, from.1 + (to.1 - from.1) * k));
    }
    ui.pointer_up(form, touch(to.0, to.1));
    ui.process_events();
}

fn form(ui: &mut Ui) -> ComponentId {
    let form = ui.add_root(Form::new("frmDedo", 400.0, 600.0));
    ui.show(form).unwrap();
    form
}

fn setup() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_time(100.0);
    ui
}

#[test]
fn a_fast_drag_is_a_swipe_with_the_direction() {
    let mut ui = setup();
    let f = form(&mut ui);
    let panel = ui.create("Panel", Some(f)).unwrap();
    ui.set_prop(panel, "align", "client").unwrap();
    ui.layout();
    let got: Rc<RefCell<Vec<String>>> = Rc::default();
    let g = got.clone();
    ui.on(panel, "on_swipe", move |_, _, args| {
        if let EventArgs::Swipe(s) = args {
            g.borrow_mut().push(s.direction.name().to_string());
        }
    });
    swipe(&mut ui, f, (300.0, 300.0), (100.0, 310.0));
    swipe(&mut ui, f, (200.0, 100.0), (210.0, 400.0));
    assert_eq!(*got.borrow(), ["left", "down"]);
    // Devagar não é deslizar.
    ui.pointer_down(f, touch(300.0, 300.0));
    for i in 1..=4 {
        let t = ui.time() + 0.5;
        ui.set_time(t);
        ui.pointer_move(f, touch(300.0 - 50.0 * i as f32, 300.0));
    }
    let t = ui.time() + 0.5;
    ui.set_time(t);
    ui.pointer_up(f, touch(100.0, 300.0));
    assert_eq!(got.borrow().len(), 2, "devagar não desliza");
}

#[test]
fn a_swipe_that_scrolled_is_not_a_swipe() {
    let mut ui = setup();
    let f = form(&mut ui);
    let scroll = ui.create("ScrollBox", Some(f)).unwrap();
    ui.set_prop(scroll, "align", "client").unwrap();
    for i in 0..40 {
        let b = ui.create("Button", Some(scroll)).unwrap();
        ui.set_prop(b, "align", "top").unwrap();
        ui.set_prop(b, "text", format!("Linha {i}")).unwrap();
    }
    ui.layout();
    let got = Rc::new(RefCell::new(0));
    let g = got.clone();
    ui.on(scroll, "on_swipe", move |_, _, _| *g.borrow_mut() += 1);
    // Para cima numa lista que rola: é rolagem, não deslizar.
    swipe(&mut ui, f, (200.0, 500.0), (200.0, 200.0));
    assert_eq!(*got.borrow(), 0);
    // Para o lado (a lista não rola para os lados): desliza.
    swipe(&mut ui, f, (300.0, 300.0), (80.0, 300.0));
    assert_eq!(*got.borrow(), 1);
}

#[test]
fn the_tab_control_changes_page_only_with_swipe_on() {
    let mut ui = setup();
    let f = form(&mut ui);
    let tabs = ui.add(TabControl::default(), f);
    ui.set_prop(tabs, "align", "client").unwrap();
    for name in ["A", "B", "C"] {
        let page = ui.create("TabItem", Some(tabs)).unwrap();
        ui.set_prop(page, "caption", name).unwrap();
    }
    ui.layout();
    let index = |ui: &Ui| ui.get_prop(tabs, "tab_index").unwrap();
    swipe(&mut ui, f, (350.0, 300.0), (50.0, 300.0));
    assert_eq!(index(&ui), PropValue::Int(0), "desligado: não troca");
    ui.set_prop(tabs, "swipe", true).unwrap();
    swipe(&mut ui, f, (350.0, 300.0), (50.0, 300.0));
    assert_eq!(index(&ui), PropValue::Int(1));
    swipe(&mut ui, f, (350.0, 300.0), (50.0, 300.0));
    swipe(&mut ui, f, (350.0, 300.0), (50.0, 300.0));
    assert_eq!(index(&ui), PropValue::Int(2), "na última, não dá a volta");
    swipe(&mut ui, f, (50.0, 300.0), (350.0, 300.0));
    assert_eq!(index(&ui), PropValue::Int(1));
}

#[test]
fn the_carousel_changes_item_and_the_drawer_opens_from_the_edge() {
    let mut ui = setup();
    let f = form(&mut ui);
    let drawer = ui.add(NavigationDrawer::default(), f);
    ui.set_prop(drawer, "open", false).unwrap();
    let carousel = ui.create("Carousel", Some(f)).unwrap();
    ui.set_prop(carousel, "align", "client").unwrap();
    for i in 0..3 {
        let item = ui.create("Panel", Some(carousel)).unwrap();
        ui.set_prop(item, "tag", i as i64).unwrap();
    }
    ui.layout();
    let open = |ui: &Ui| ui.get_prop(drawer, "open").unwrap();
    let left = ui.absolute_rect(carousel).unwrap().x;
    // No meio do carrossel: troca de item.
    swipe(&mut ui, f, (left + 300.0, 300.0), (left + 60.0, 300.0));
    assert_eq!(ui.get_prop(carousel, "index").unwrap(), PropValue::Int(1));
    assert_eq!(open(&ui), PropValue::Bool(false));
    // Da borda esquerda do form para a direita: a gaveta abre, mesmo com o carrossel embaixo
    // do dedo (a borda vem antes, como no Android).
    ui.set_prop(carousel, "index", 0i64).unwrap();
    swipe(&mut ui, f, (4.0, 300.0), (250.0, 300.0));
    assert_eq!(open(&ui), PropValue::Bool(true), "abriu pela borda");
    // Para a esquerda em cima dela: fecha.
    swipe(&mut ui, f, (200.0, 300.0), (10.0, 300.0));
    assert_eq!(open(&ui), PropValue::Bool(false), "fechou");
}

/// A pinça no `Image` (`zoomable`, decidido em 14/11): desligada, o gesto sobe para os pais;
/// ligada, a imagem amplia e fica com ele, e ampliada o dedo arrasta a imagem em vez de rolar
/// a lista de fora.
#[test]
fn a_zoomable_image_takes_the_pinch_and_the_finger() {
    let mut ui = setup();
    let f = form(&mut ui);
    let scroll = ui.create("ScrollBox", Some(f)).unwrap();
    ui.set_prop(scroll, "align", "client").unwrap();
    let image = ui.create("Image", Some(scroll)).unwrap();
    for (prop, value) in [("align", PropValue::from("top")), ("height", 300.0.into())] {
        ui.set_prop(image, prop, value).unwrap();
    }
    for i in 0..20 {
        let b = ui.create("Button", Some(scroll)).unwrap();
        ui.set_prop(b, "align", "top").unwrap();
        ui.set_prop(b, "text", format!("Linha {i}")).unwrap();
    }
    ui.layout();
    let outer = Rc::new(RefCell::new(0));
    let o = outer.clone();
    ui.on(scroll, "on_gesture", move |_, _, _| *o.borrow_mut() += 1);
    let pinch = |ui: &mut Ui| {
        for (phase, scale) in [(GesturePhase::Begin, 1.0), (GesturePhase::Update, 2.0), (GesturePhase::End, 2.0)] {
            ui.gesture(f, GestureEvent { phase, center: PointF::new(200.0, 150.0), scale, delta_scale: if phase == GesturePhase::Update { 2.0 } else { 1.0 }, ..Default::default() });
        }
    };
    pinch(&mut ui);
    assert_eq!(*outer.borrow(), 3, "desligada: o gesto sobe");
    ui.set_prop(image, "zoomable", true).unwrap();
    pinch(&mut ui);
    assert_eq!(*outer.borrow(), 3, "ligada: a imagem fica com o gesto");
    // Ampliada: o dedo arrasta a imagem, e a lista não rola.
    let before = ui.get_prop(scroll, "scroll_position").unwrap();
    ui.pointer_down(f, touch(200.0, 250.0));
    for i in 1..=4 {
        let t = ui.time() + 0.05;
        ui.set_time(t);
        ui.pointer_move(f, touch(200.0, 250.0 - 30.0 * i as f32));
    }
    ui.pointer_up(f, touch(200.0, 130.0));
    assert_eq!(ui.get_prop(scroll, "scroll_position").unwrap(), before, "a lista não rolou");
}

/// Deslizar a linha da lista para mostrar as ações (`swipe_actions`, decidido em 14/11): passou
/// da metade, fica aberta; o toque numa ação dispara `on_swipe_action` com o item e a ação; não
/// passou da metade, volta; rolar continua rolando.
#[test]
fn a_list_row_slides_to_show_its_actions() {
    let mut ui = setup();
    let f = form(&mut ui);
    let list = ui.add(VirtualList::default(), f);
    ui.set_prop(list, "align", "client").unwrap();
    ui.create("Panel", Some(list)).unwrap();
    for (prop, value) in [("item_height", PropValue::from(60.0)), ("item_count", 30i64.into()), ("swipe_actions", "Arquivar;Excluir".into())] {
        ui.set_prop(list, prop, value).unwrap();
    }
    ui.layout();
    ui.process_events();
    let got: Rc<RefCell<Vec<String>>> = Rc::default();
    let g = got.clone();
    ui.on(list, "on_swipe_action", move |_, _, args| g.borrow_mut().push(format!("{args:?}")));
    let drag = |ui: &mut Ui, y: f32, dx: f32| {
        ui.pointer_down(f, touch(350.0, y));
        for i in 1..=6 {
            let t = ui.time() + 0.2;
            ui.set_time(t);
            ui.pointer_move(f, touch(350.0 - dx * i as f32 / 6.0, y));
        }
        let t = ui.time() + 0.2;
        ui.set_time(t);
        ui.pointer_up(f, touch(350.0 - dx, y));
        ui.process_events();
    };
    let row = |ui: &Ui, i: usize| VirtualList::item(ui, list, i).unwrap();
    // A linha 2 (de 120 a 180): pouco, volta.
    drag(&mut ui, 150.0, 40.0);
    assert_eq!(ui.control(row(&ui, 2)).unwrap().translate().x, 0.0, "pouco: voltou");
    // Muito: fica aberta, com as duas ações (160 pontos) à mostra.
    drag(&mut ui, 150.0, 140.0);
    assert_eq!(ui.control(row(&ui, 2)).unwrap().translate().x, -160.0, "abriu");
    // O toque na segunda ação (a faixa das ações começa em 400 - 160).
    ui.pointer_down(f, touch(370.0, 150.0));
    ui.pointer_up(f, touch(370.0, 150.0));
    ui.process_events();
    assert_eq!(got.borrow().len(), 1, "{:?}", got.borrow());
    assert!(got.borrow()[0].contains("\\\"index\\\":2") && got.borrow()[0].contains("Excluir"), "{:?}", got.borrow());
    assert_eq!(ui.control(row(&ui, 2)).unwrap().translate().x, 0.0, "fechou depois da ação");
    // Rolar para cima continua rolando a lista.
    let before = VirtualList::visible(&ui, list);
    swipe(&mut ui, f, (200.0, 500.0), (200.0, 150.0));
    ui.process_events();
    assert_ne!(VirtualList::visible(&ui, list), before, "rolou");
}
