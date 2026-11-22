//! O `VirtualList` de 22/11 (o player de música): a seleção marcada pelo estado `checked`, os
//! eventos com o índice, o teclado, vários modelos de linha e a barra de rolagem, sem impor
//! colunas à linha.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Form, Label, Panel, VirtualList};
use aylis_core::*;

fn at(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), button: PointerButton::Primary, ..Default::default() }
}

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    ui.pointer_move(form, at(x, y));
    ui.pointer_down(form, at(x, y));
    ui.pointer_up(form, at(x, y));
}

/// Uma lista de 100 itens com um modelo de 40 de altura (um painel com um texto) e o registro
/// dos eventos com o índice.
fn setup() -> (Ui, ComponentId, ComponentId, Rc<RefCell<Vec<String>>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmPlayer", 300.0, 400.0));
    let list = ui.add(VirtualList::default(), form);
    ui.control_mut(list).unwrap().set_width(300.0).set_height(400.0);
    let row = ui.add(Panel::new(300.0, 40.0), list);
    ui.set_name(row, "pnlMusica").unwrap();
    ui.set_prop(row, "interactive", true).unwrap();
    let text = ui.add(Label::new(""), row);
    ui.set_name(text, "lblTitulo").unwrap();
    ui.set_prop(list, "item_height", 40.0).unwrap();
    ui.set_prop(list, "selection", "single").unwrap();
    let log = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_item_click", "on_item_dbl_click", "on_item_activate", "on_change"] {
        let log = log.clone();
        ui.on(list, event, move |_, _, args| {
            if let EventArgs::Value(PropValue::Int(i)) = args {
                log.borrow_mut().push(format!("{event} {i}"));
            }
        });
    }
    ui.set_prop(list, "item_count", 100).unwrap();
    ui.show(form).unwrap();
    ui.layout();
    VirtualList::refresh(&mut ui, list);
    ui.layout();
    (ui, form, list, log)
}

#[test]
fn clicking_a_row_selects_it_and_tells_the_index() {
    let (mut ui, form, list, log) = setup();
    // A segunda linha (40 a 80): o clique cai no painel da cópia.
    click(&mut ui, form, 100.0, 60.0);
    assert_eq!(ui.get_prop(list, "item_index").unwrap(), PropValue::Int(1));
    assert_eq!(*log.borrow(), vec!["on_change 1".to_string(), "on_item_click 1".to_string()]);
    let row = VirtualList::item(&ui, list, 1).unwrap();
    assert!(ui.control(row).unwrap().state().checked, "a linha escolhida fica marcada");
    assert!(!ui.control(VirtualList::item(&ui, list, 0).unwrap()).unwrap().state().checked);
    // O duplo clique: o item e o "ativar" (tocar a música).
    log.borrow_mut().clear();
    ui.pointer_down(form, at(100.0, 60.0));
    assert!(log.borrow().contains(&"on_item_dbl_click 1".to_string()) && log.borrow().contains(&"on_item_activate 1".to_string()), "{:?}", log.borrow());
    ui.pointer_up(form, at(100.0, 60.0));
}

#[test]
fn the_keyboard_moves_the_selection_and_scrolls_to_it() {
    let (mut ui, form, list, log) = setup();
    click(&mut ui, form, 100.0, 20.0);
    let key = |ui: &mut Ui, k: Key| ui.key_down(form, KeyEvent { key: k, ..Default::default() });
    key(&mut ui, Key::Down);
    assert_eq!(ui.get_prop(list, "item_index").unwrap(), PropValue::Int(1));
    key(&mut ui, Key::End);
    ui.layout();
    assert_eq!(ui.get_prop(list, "item_index").unwrap(), PropValue::Int(99));
    assert!(VirtualList::item(&ui, list, 99).is_some(), "rolou até o último");
    log.borrow_mut().clear();
    key(&mut ui, Key::Enter);
    assert_eq!(*log.borrow(), vec!["on_item_activate 99".to_string()]);
}

#[test]
fn each_item_can_use_another_row_model() {
    let (mut ui, _form, list, _) = setup();
    // Um segundo modelo, mais baixo: o cabeçalho de seção, sem as "colunas" da música.
    let section = ui.add(Panel::new(300.0, 20.0), list);
    ui.set_name(section, "pnlSecao").unwrap();
    let title = ui.add(Label::new(""), section);
    ui.set_name(title, "lblSecao").unwrap();
    ui.set_prop(list, "item_form", "").unwrap();
    ui.set_prop(list, "kinds", "pnlSecao;pnlMusica;pnlMusica").unwrap();
    VirtualList::refresh(&mut ui, list);
    ui.layout();
    let first = VirtualList::item(&ui, list, 0).unwrap();
    let second = VirtualList::item(&ui, list, 1).unwrap();
    assert!(ui.find_in(first, "lblSecao").is_some() && ui.find_in(first, "lblTitulo").is_none(), "o item 0 é uma seção");
    assert!(ui.find_in(second, "lblTitulo").is_some(), "o item 1 é uma música");
    // A seção tem a altura do modelo dela (20); a música seguinte começa logo abaixo.
    assert_eq!(VirtualList::item_span(&ui, list, 1), Some((20.0, 40.0)));
    assert!(VirtualList::part_named(&ui, list, 1, "lblTitulo").is_some());
}

#[test]
fn the_scroll_bar_takes_its_strip_and_scrolls() {
    let (mut ui, form, list, _) = setup();
    ui.set_prop(list, "scroll_bar", "auto").unwrap();
    ui.layout();
    let row = VirtualList::item(&ui, list, 0).unwrap();
    assert_eq!(ui.control(row).unwrap().bounds().width, 290.0, "a linha deixa a faixa da barra");
    // Apertar no fim do trilho leva ao fim da lista.
    ui.pointer_down(form, at(295.0, 398.0));
    ui.pointer_up(form, at(295.0, 398.0));
    ui.layout();
    assert!(VirtualList::item(&ui, list, 99).is_some());
}

/// O `ListHeader`: o título acompanha o componente da linha pelo nome, arrastar a divisa muda a
/// largura dele em todas as linhas, e o clique num título que ordena avisa o `on_sort`.
#[test]
fn the_header_follows_the_row_parts_resizes_and_sorts() {
    let (mut ui, form, list, _) = setup();
    let title = VirtualList::part_named(&ui, list, 0, "lblTitulo").unwrap();
    let _ = title;
    let header = ui.add(aylis_controls::ListHeader::default(), form);
    ui.control_mut(header).unwrap().set_width(300.0).set_height(30.0).set_top(0.0);
    // A lista desce para baixo do cabeçalho.
    ui.control_mut(list).unwrap().set_top(30.0);
    ui.set_prop(header, "list", PropValue::Component(Some(list))).unwrap();
    ui.set_prop(header, "columns", r#"[{"title": "Título", "part": "lblTitulo", "width": 150, "sortable": true, "resizable": true}]"#).unwrap();
    let sorted = Rc::new(RefCell::new(Vec::new()));
    let seen = sorted.clone();
    ui.on(header, "on_sort", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            seen.borrow_mut().push(t.clone());
        }
    });
    ui.process_posted();
    ui.layout();
    let width = |ui: &Ui, i: usize| ui.control(VirtualList::part_named(ui, list, i, "lblTitulo").unwrap()).unwrap().bounds().width;
    assert_eq!(width(&ui, 0), 150.0, "a largura da coluna vai para a peça da linha");
    // Arrasta a divisa (em 150) até 200.
    ui.pointer_move(form, at(150.0, 15.0));
    ui.pointer_down(form, at(150.0, 15.0));
    ui.pointer_move(form, at(200.0, 15.0));
    ui.pointer_up(form, at(200.0, 15.0));
    ui.layout();
    assert_eq!(width(&ui, 0), 200.0);
    assert_eq!(width(&ui, 3), 200.0, "em todas as linhas");
    // O clique no título ordena: a mesma coluna inverte.
    click(&mut ui, form, 60.0, 15.0);
    click(&mut ui, form, 60.0, 15.0);
    assert_eq!(*sorted.borrow(), vec!["lblTitulo".to_string(), "lblTitulo".to_string()]);
    assert_eq!(ui.get_prop(header, "sort_descending").unwrap(), PropValue::Bool(true));
}
