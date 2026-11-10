//! VirtualList: só as linhas visíveis existem, reaproveitadas ao rolar; on_bind,
//! on_load_more (lista infinita) e fonte de valores com bindings.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Form, Label, Panel, RowSource, VirtualList};
use aylis_core::{ComponentId, EventArgs, PointF, PropValue, Ui, WheelEvent};

fn text(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

/// Form 400 x 400 com a lista (300 de altura, linhas de 50) e o modelo: um painel com o
/// rótulo `lblTexto`.
fn scene(ui: &mut Ui, count: i64) -> (ComponentId, ComponentId) {
    scene_with(ui, count, true)
}

fn scene_with(ui: &mut Ui, count: i64, on_bind: bool) -> (ComponentId, ComponentId) {
    aylis_controls::register(ui);
    let form = ui.add_root(Form::new("lista", 400.0, 400.0));
    let list = ui.add(VirtualList::default(), form);
    ui.control_mut(list).unwrap().set_rect(0.0, 0.0, 400.0, 300.0);
    let template = ui.add(Panel::new(400.0, 50.0), list);
    let label = ui.add(Label::new("modelo"), template);
    ui.set_name(label, "lblTexto").unwrap();
    ui.set_prop(list, "item_height", 50.0).unwrap();
    ui.set_prop(list, "item_count", count).unwrap();
    // Preenche a linha pelo índice, como o onBindViewHolder.
    if on_bind {
    ui.on(list, "on_bind", |ui, list, args| {
        let EventArgs::Value(PropValue::Int(i)) = args else { return };
        let row = VirtualList::item(ui, list, *i as usize).unwrap();
        let label = ui.find_in(row, "lblTexto").unwrap();
        let _ = ui.set_prop(label, "text", format!("Item {i}"));
    });
    }
    ui.layout();
    (form, list)
}

fn shown(ui: &Ui, list: ComponentId) -> Vec<String> {
    VirtualList::visible(ui, list)
        .into_iter()
        .map(|i| {
            let row = VirtualList::item(ui, list, i).unwrap();
            text(ui, ui.find_in(row, "lblTexto").unwrap())
        })
        .collect()
}

#[test]
fn a_million_items_cost_only_the_visible_rows() {
    let mut ui = Ui::new();
    let (form, list) = scene(&mut ui, 1_000_000);
    // 300 / 50 = 6 visíveis, mais 2 entrando.
    assert_eq!(VirtualList::visible(&ui, list), (0..8).collect::<Vec<_>>());
    assert_eq!(shown(&ui, list)[0], "Item 0");
    assert_eq!(ui.children(list).len(), 1 + 8, "o modelo e as 8 linhas");

    // Rolar 500: começa no item 10, com as mesmas 8 linhas.
    ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -500.0), ..Default::default() });
    ui.layout();
    assert_eq!(VirtualList::visible(&ui, list), (10..18).collect::<Vec<_>>());
    assert_eq!(shown(&ui, list)[0], "Item 10");
    assert_eq!(ui.children(list).len(), 1 + 8, "nenhuma linha nova: reaproveitadas");
    // A linha do item 10 está no topo da lista.
    let row = VirtualList::item(&ui, list, 10).unwrap();
    assert_eq!(ui.bounds(row).unwrap().y, 0.0);

    // Pular para perto do fim.
    VirtualList::scroll_to(&mut ui, list, 999_990);
    ui.layout();
    assert_eq!(VirtualList::visible(&ui, list).first(), Some(&999_990), "o item pedido fica no topo");
}

#[test]
fn near_the_end_it_asks_for_more_once() {
    let mut ui = Ui::new();
    let (form, list) = scene(&mut ui, 20);
    let asked = Rc::new(RefCell::new(Vec::new()));
    let seen = asked.clone();
    ui.on(list, "on_load_more", move |ui, list, args| {
        if let EventArgs::Value(PropValue::Int(n)) = args {
            seen.borrow_mut().push(*n);
            // Busca mais 20 (a "próxima página").
            let _ = ui.set_prop(list, "item_count", n + 20);
        }
    });
    let scroll = |ui: &mut Ui, dy: f32| {
        ui.wheel(form, WheelEvent { position: PointF::new(100.0, 100.0), delta: PointF::new(0.0, -dy), ..Default::default() });
        ui.layout();
    };
    scroll(&mut ui, 300.0);
    assert!(asked.borrow().is_empty(), "longe do fim");
    scroll(&mut ui, 400.0); // itens 14 a 21: perto do fim dos 20
    assert_eq!(*asked.borrow(), [20]);
    scroll(&mut ui, 10.0);
    assert_eq!(*asked.borrow(), [20], "uma vez por tamanho");
    scroll(&mut ui, 700.0); // itens 28 a 35: perto do fim dos 40
    assert_eq!(*asked.borrow(), [20, 40]);
}

struct Names(Vec<&'static str>);

impl RowSource for Names {
    fn count(&self, _: &Ui) -> usize {
        self.0.len()
    }
    fn value(&self, _: &Ui, row: usize, field: &str) -> Option<String> {
        (field == "nome").then(|| self.0[row].to_string())
    }
}

#[test]
fn a_source_fills_the_bound_components() {
    let mut ui = Ui::new();
    let (_, list) = scene_with(&mut ui, 0, false);
    ui.set_prop(list, "bindings", "lblTexto=nome").unwrap();
    VirtualList::set_source(&mut ui, list, Box::new(Names(vec!["Ana", "Bruno", "Carla"])));
    ui.layout();
    assert_eq!(shown(&ui, list), ["Ana", "Bruno", "Carla"]);
}

/// O retângulo da linha que mostra o item `i`.
fn rect_of(ui: &Ui, list: ComponentId, i: usize) -> aylis_core::RectF {
    let row = VirtualList::item(ui, list, i).unwrap();
    ui.control(row).unwrap().bounds()
}

#[test]
fn items_can_have_their_own_heights() {
    let mut ui = Ui::new();
    let (form, list) = scene(&mut ui, 20);
    ui.set_prop(list, "sizes", "30;100;40").unwrap();
    ui.layout();
    // 30 + 100 + 40, e o resto com item_height (50).
    let tops: Vec<(f32, f32)> = (0..4).map(|i| (rect_of(&ui, list, i).y, rect_of(&ui, list, i).height)).collect();
    assert_eq!(tops, [(0.0, 30.0), (30.0, 100.0), (130.0, 40.0), (170.0, 50.0)]);
    assert_eq!(VirtualList::item_span(&ui, list, 5), Some((270.0, 50.0)));
    // Pelo código, um milhão de alturas diferentes: rolar até o fim acha o item certo.
    ui.set_prop(list, "item_count", 1_000_000i64).unwrap();
    VirtualList::set_sizer(&mut ui, list, |i| if i % 2 == 0 { 20.0 } else { 40.0 });
    VirtualList::scroll_to(&mut ui, list, 999_998);
    ui.layout();
    let visible = VirtualList::visible(&ui, list);
    assert_eq!(visible.first(), Some(&999_990), "o fim da lista: {visible:?}");
    assert!(visible.contains(&999_999));
    assert_eq!(VirtualList::item_span(&ui, list, 3), Some((80.0, 40.0)), "20 + 40 + 20");
    let _ = form;
}

#[test]
fn horizontal_lists_and_grids() {
    let mut ui = Ui::new();
    let (form, list) = scene(&mut ui, 100);
    // Horizontal: lado a lado, com item_width; a roda comum anda para o lado.
    ui.set_prop(list, "orientation", "horizontal").unwrap();
    ui.set_prop(list, "item_width", 120.0).unwrap();
    ui.layout();
    assert_eq!(VirtualList::visible(&ui, list), (0..6).collect::<Vec<_>>(), "400 / 120: 4 à vista e 2 entrando");
    assert_eq!((rect_of(&ui, list, 1).x, rect_of(&ui, list, 1).y, rect_of(&ui, list, 1).height), (120.0, 0.0, 300.0));
    ui.wheel(form, WheelEvent { position: PointF::new(200.0, 150.0), delta: PointF::new(0.0, -240.0), ..Default::default() });
    ui.layout();
    assert_eq!(VirtualList::visible(&ui, list).first(), Some(&2));
    // Grade: 4 colunas na vertical, de 100 de largura.
    ui.set_prop(list, "orientation", "vertical").unwrap();
    ui.set_prop(list, "columns", 4i64).unwrap();
    VirtualList::scroll_to(&mut ui, list, 0);
    ui.layout();
    let r = rect_of(&ui, list, 5);
    assert_eq!((r.x, r.y, r.width, r.height), (100.0, 50.0, 100.0, 50.0), "o item 5: segunda linha, segunda coluna");
    // 300 / 50 = 6 linhas à vista + 2 entrando, de 4 itens cada.
    assert_eq!(VirtualList::visible(&ui, list).len(), 32);
    VirtualList::scroll_to(&mut ui, list, 41);
    ui.layout();
    assert_eq!(VirtualList::visible(&ui, list).first(), Some(&40), "rola até a linha do item");
}
