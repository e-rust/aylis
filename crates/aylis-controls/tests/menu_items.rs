//! O item de menu com submenu também recebe o clique, e o `content` põe um controle do form
//! (botões, um frame) dentro do item enquanto o menu está aberto (pedido do autor em 17/11).

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::{Button, Form, MenuItem, PopupMenu};
use aylis_core::{ComponentId, PointF, PointerEvent, PropValue, Ui};

fn form(ui: &mut Ui) -> ComponentId {
    aylis_controls::register(ui);
    ui.add_root(Form::new("teste", 600.0, 400.0))
}

fn overlay(ui: &Ui, f: ComponentId) -> Option<ComponentId> {
    ui.children(f).iter().copied().find(|&c| ui.class_name(c) == Some("MenuOverlay"))
}

fn press(ui: &mut Ui, f: ComponentId, p: PointF) {
    let e = PointerEvent { position: p, ..Default::default() };
    ui.pointer_down(f, e);
    ui.pointer_up(f, e);
}

fn counter(ui: &mut Ui, id: ComponentId) -> Rc<Cell<u32>> {
    let n = Rc::new(Cell::new(0));
    let c = n.clone();
    ui.on(id, "on_click", move |_, _, _| c.set(c.get() + 1));
    n
}

#[test]
fn the_parent_of_a_submenu_receives_the_click_and_the_menu_stays_open() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let menu = ui.add(PopupMenu::default(), f);
    let parent = ui.add(MenuItem::new("Exportar"), menu);
    ui.add(MenuItem::new("PDF"), parent);
    let clicks = counter(&mut ui, parent);
    PopupMenu::popup(&mut ui, menu, PointF::new(20.0, 20.0));
    let o = overlay(&ui, f).expect("o menu abriu");
    let row = ui.access_items(o)[0].rect.unwrap();
    press(&mut ui, f, PointF::new(row.x + 10.0, row.y + row.height / 2.0));
    assert_eq!(clicks.get(), 1, "o clique no item pai chega ao app");
    assert!(overlay(&ui, f).is_some(), "o menu continua aberto, com o submenu");
    assert_eq!(ui.access_items(o).len(), 2, "o submenu está à vista");

    // close() fecha pelo código.
    ui.invoke(menu, "close", &[]).unwrap();
    ui.process_posted();
    assert!(overlay(&ui, f).is_none());
}

#[test]
fn the_item_content_is_shown_in_the_item_and_given_back_on_close() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let menu = ui.add(PopupMenu::default(), f);
    let item = ui.add(MenuItem::new("Microfone"), menu);
    // Um botão do form, invisível, mostrado no item.
    let toggle = ui.add(Button::new("Ativar"), f);
    ui.control_mut(toggle).unwrap().set_rect(5.0, 300.0, 70.0, 40.0).set_visible(false);
    ui.set_prop(item, "content", PropValue::Component(Some(toggle))).unwrap();
    let toggled = counter(&mut ui, toggle);
    let item_clicks = counter(&mut ui, item);

    PopupMenu::popup(&mut ui, menu, PointF::new(20.0, 20.0));
    let o = overlay(&ui, f).unwrap();
    assert_eq!(ui.parent(toggle), Some(o), "emprestado ao menu aberto");
    let row = ui.access_items(o)[0].rect.unwrap();
    assert!(row.height >= 46.0, "a linha cresce até a altura do conteúdo: {row:?}");
    ui.update_layout();
    let cb = ui.control(toggle).unwrap();
    let r = cb.bounds();
    assert!(cb.visible() && r.right() <= row.right() && r.x > row.x + 60.0, "à direita, à frente do texto: {r:?} em {row:?}");

    // Clicar no botão é do botão: o item não dispara e o menu fica aberto.
    press(&mut ui, f, PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0));
    assert_eq!((toggled.get(), item_clicks.get()), (1, 0));
    assert!(overlay(&ui, f).is_some(), "o menu fica aberto");

    // Fechou: o botão volta como estava.
    ui.invoke(menu, "close", &[]).unwrap();
    ui.process_posted();
    assert_eq!(ui.parent(toggle), Some(f));
    ui.update_layout();
    let cb = ui.control(toggle).unwrap();
    assert!(!cb.visible());
    assert_eq!((cb.left(), cb.top(), cb.width(), cb.height()), (5.0, 300.0, 70.0, 40.0), "a posição de antes");
}

#[test]
fn an_item_without_text_gives_the_whole_row_to_the_content() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let menu = ui.add(PopupMenu::default(), f);
    let item = ui.add(MenuItem::new(""), menu);
    let panel = ui.add(Button::new("x"), f);
    ui.control_mut(panel).unwrap().set_rect(0.0, 0.0, 120.0, 50.0).set_visible(false);
    ui.set_prop(item, "content", PropValue::Component(Some(panel))).unwrap();
    PopupMenu::popup(&mut ui, menu, PointF::new(20.0, 20.0));
    let o = overlay(&ui, f).unwrap();
    let row = ui.access_items(o)[0].rect.unwrap();
    ui.update_layout();
    let r = ui.control(panel).unwrap().bounds();
    assert!((r.width - (row.width - 8.0)).abs() < 0.5 && r.height == 50.0, "{r:?} em {row:?}");
}

/// O `item_height` e o `separator_height` do estilo `PopupMenu` valem também no menu do form.
#[test]
fn the_popup_menu_rows_follow_the_theme_sizes() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let mut theme = ui.theme().clone();
    let props = aylis_core::StyleProps { item_height: Some(40.0), separator_height: Some(15.0), ..Default::default() };
    theme.styles.insert("PopupMenu".into(), aylis_core::Style::new(props));
    ui.set_theme(theme);
    let menu = ui.add(PopupMenu::default(), f);
    ui.add(MenuItem::new("Abrir"), menu);
    ui.add(MenuItem::new("-"), menu);
    ui.add(MenuItem::new("Sair"), menu);
    PopupMenu::popup(&mut ui, menu, PointF::new(20.0, 20.0));
    let o = overlay(&ui, f).expect("o menu abriu");
    let rows: Vec<_> = ui.access_items(o).iter().filter_map(|i| i.rect).collect();
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_eq!(rows[0].height, 40.0);
    assert_eq!(rows[1].y - rows[0].bottom(), 15.0, "a divisória do tema");
}
