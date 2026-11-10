//! Os itens desenhados pelos controles (abas, linhas, itens de menu) para leitores de tela,
//! automação e testes: listar e clicar sem evento de mouse.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::automation::{ItemAction, ItemRole};
use aylis_core::*;

fn form(ui: &mut Ui) -> ComponentId {
    aylis_controls::register(ui);
    let f = ui.add_root(Form::new("teste", 600.0, 400.0));
    ui.show(f).unwrap();
    f
}

/// Os eventos disparados por `id` (nome e texto).
fn record(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.on(id, event, move |_, _, args| s.borrow_mut().push(args.text().map(str::to_string).unwrap_or_else(|| format!("{args:?}"))));
    seen
}

#[test]
fn tabs_are_items_and_clicking_one_switches_the_page() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let tabs = ui.add(TabControl::default(), f);
    let a = TabControl::add_tab(&mut ui, tabs, "Dados");
    let b = TabControl::add_tab(&mut ui, tabs, "Clientes");
    let changes = record(&mut ui, tabs, "on_change");
    let items = ui.access_items(tabs);
    assert_eq!(items.iter().map(|i| (i.role, i.name.as_str(), i.selected)).collect::<Vec<_>>(), [(ItemRole::Tab, "Dados", true), (ItemRole::Tab, "Clientes", false)]);
    assert!(ui.click_item(tabs, "clientes"), "sem diferenciar maiúsculas");
    assert!(!ui.control(a).unwrap().visible() && ui.control(b).unwrap().visible());
    assert!(ui.access_items(tabs)[1].selected);
    assert_eq!(changes.borrow().len(), 1);
    assert!(!ui.click_item(tabs, "Pedidos"), "não existe");
}

#[test]
fn list_and_combo_items_select_and_fire_on_change() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let list = ui.add(ListBox::new(&["Recife", "São Paulo", "Curitiba"]), f);
    let changes = record(&mut ui, list, "on_change");
    assert_eq!(ui.access_items(list).len(), 3);
    assert!(ui.click_item(list, "São Paulo"));
    assert_eq!(ui.get::<ListBox>(list).unwrap().item_index, 1);
    assert_eq!(*changes.borrow(), ["São Paulo"]);
    assert!(ui.access_items(list)[1].selected);

    let combo = ui.add(ComboBox::new(&["Aberto", "Faturado"]), f);
    assert!(ui.access_action(combo, 1, ItemAction::Select));
    assert_eq!(ui.get::<ComboBox>(combo).unwrap().text(), "Faturado");
    assert!(!ui.get::<ComboBox>(combo).unwrap().is_open(), "sem abrir a lista");
}

#[test]
fn menus_open_submenus_and_activate_items_by_name() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let bar = ui.add(MainMenu::default(), f);
    let file = ui.add(MenuItem::new("Arquivo"), bar);
    let save = ui.add(MenuItem::new("Salvar"), file);
    ui.add(MenuItem::new("-"), file);
    let export = ui.add(MenuItem::new("Exportar"), file);
    let pdf = ui.add(MenuItem::new("PDF"), export);
    let saved = record(&mut ui, save, "on_click");
    let exported = record(&mut ui, pdf, "on_click");
    ui.layout();

    let top = ui.access_items(bar);
    assert_eq!((top[0].role, top[0].name.as_str(), top[0].has_popup), (ItemRole::MenuItem, "Arquivo", true));
    // Clicar no item da barra abre o menu (a camada sobre o form) com os itens dele.
    assert!(ui.click_item(bar, "Arquivo"));
    let overlay = *ui.children(f).iter().find(|&&c| ui.class_name(c) == Some("MenuOverlay")).expect("o menu abriu");
    let names: Vec<String> = ui.access_items(overlay).into_iter().map(|i| i.name).collect();
    assert_eq!(names, ["Salvar", "Exportar"], "sem a divisória");
    // Exportar tem submenu: abre; o PDF passa a aparecer e ativa.
    assert!(ui.click_item(overlay, "Exportar"));
    assert!(ui.access_items(overlay).iter().any(|i| i.name == "PDF"));
    assert!(ui.click_item(overlay, "PDF"));
    assert_eq!(exported.borrow().len(), 1);
    assert!(!ui.contains(overlay), "o menu fecha ao ativar");
    // De novo pela barra, agora o Salvar.
    ui.click_item(bar, "Arquivo");
    let overlay = *ui.children(f).iter().find(|&&c| ui.class_name(c) == Some("MenuOverlay")).unwrap();
    assert!(ui.click_item(overlay, "Salvar"));
    assert_eq!(saved.borrow().len(), 1);
}

#[test]
fn a_popup_menu_check_item_toggles_through_automation() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let menu = ui.add(PopupMenu::default(), f);
    let grid = ui.add(MenuItem::new("Mostrar grade"), menu);
    ui.set_prop(grid, "auto_check", true).unwrap();
    PopupMenu::popup(&mut ui, menu, PointF::new(20.0, 20.0));
    let overlay = *ui.children(f).iter().find(|&&c| ui.class_name(c) == Some("MenuOverlay")).unwrap();
    let item = &ui.access_items(overlay)[0];
    assert_eq!((item.role, item.checked), (ItemRole::MenuItemCheck, Some(false)));
    assert!(ui.click_item(overlay, "Mostrar grade"));
    assert_eq!(ui.get_prop(grid, "checked").unwrap(), PropValue::Bool(true));
}

#[test]
fn tree_items_have_levels_and_open_and_close() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let tree = ui.add(TreeView::default(), f);
    ui.set_prop(tree, "items", "Brasil;>São Paulo;>Recife;Portugal;>Lisboa").unwrap();
    let changes = record(&mut ui, tree, "on_change");
    let items = ui.access_items(tree);
    // Fechados: só as raízes, com filhos (expanded = false).
    assert_eq!(items.iter().map(|i| (i.name.as_str(), i.level, i.expanded)).collect::<Vec<_>>(), [("Brasil", Some(0), Some(false)), ("Portugal", Some(0), Some(false))]);
    assert!(ui.access_action(tree, 0, ItemAction::Expand));
    let items = ui.access_items(tree);
    assert_eq!(items.iter().map(|i| i.name.as_str()).collect::<Vec<_>>(), ["Brasil", "São Paulo", "Recife", "Portugal"]);
    assert_eq!((items[1].level, items[1].expanded), (Some(1), None), "folha, sem abrir/fechar");
    assert!(ui.click_item(tree, "Recife"));
    assert_eq!(*changes.borrow(), ["Brasil/Recife"]);
    assert!(ui.access_items(tree)[2].selected);
    assert!(ui.access_action(tree, 0, ItemAction::Collapse));
    assert_eq!(ui.access_items(tree).len(), 2);
}

#[test]
fn list_view_radio_chips_buttons_and_segments_are_items() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let lv = ui.add(ListView::default(), f);
    ui.set_prop(lv, "items", "Ana|Recife;Bruno|Curitiba").unwrap();
    let clicks = record(&mut ui, lv, "on_item_click");
    assert_eq!(ui.access_items(lv)[1].name, "Bruno, Curitiba");
    assert!(ui.click_item(lv, "Bruno, Curitiba"));
    assert_eq!(ui.get_prop(lv, "item_index").unwrap(), PropValue::Int(1));
    assert_eq!(clicks.borrow().len(), 1, "o clique dispara on_item_click");

    let radio = ui.add(RadioGroup::new("Frete", &["PAC", "Sedex"]), f);
    assert!(ui.click_item(radio, "Sedex"));
    let items = ui.access_items(radio);
    assert_eq!((items[1].role, items[1].checked), (ItemRole::Radio, Some(true)));

    let chips = ui.add(ChipGroup::default(), f);
    ui.set_prop(chips, "multiple", true).unwrap();
    assert!(ui.click_item(chips, "Abertos"));
    let on: Vec<String> = ui.access_items(chips).into_iter().filter(|i| i.checked == Some(true)).map(|i| i.name).collect();
    assert_eq!(on, ["Todos", "Abertos"]);

    let group = ui.add(ButtonGroup::default(), f);
    let changed = record(&mut ui, group, "on_change");
    assert!(ui.click_item(group, "Mês"));
    assert_eq!(*changed.borrow(), ["Mês"]);
    assert_eq!(ui.access_items(group)[2].checked, Some(true));

    let seg = ui.add(SegmentedControl::new("Um;Dois;Três"), f);
    assert!(ui.click_item(seg, "Três"));
    assert_eq!(ui.get_prop(seg, "item_index").unwrap(), PropValue::Int(2));
}

#[test]
fn calendar_days_and_month_buttons_are_items() {
    let mut ui = Ui::new();
    let f = form(&mut ui);
    let cal = ui.add(Calendar::new("2026-10-15"), f);
    ui.layout();
    let changes = record(&mut ui, cal, "on_change");
    let items = ui.access_items(cal);
    assert_eq!(items[0].role, ItemRole::Button);
    assert_eq!(items.len(), 2 + 42);
    assert!(items.iter().any(|i| i.name == "2026-10-15" && i.selected));
    assert!(ui.click_item(cal, "2026-10-27"));
    assert_eq!(*changes.borrow(), ["2026-10-27"]);
    // O botão de próximo mês muda a grade.
    assert!(ui.access_action(cal, 1, ItemAction::Click));
    assert!(ui.access_items(cal).iter().any(|i| i.name == "2026-11-30"));
}
