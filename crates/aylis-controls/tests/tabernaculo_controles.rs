//! O relatório do Tabernáculo, lote 3 (controles e tema): o que dá para conferir sem janela.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

fn count(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<usize>> {
    let n = Rc::new(RefCell::new(0));
    let c = n.clone();
    ui.on(id, event, move |_, _, _| *c.borrow_mut() += 1);
    n
}

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 600.0, 400.0));
    ui.show(form).unwrap();
    ui.layout();
    (ui, form)
}

fn key(key: Key) -> KeyEvent {
    KeyEvent { key, ..Default::default() }
}

/// C03: no grupo, marcar um botão desmarca os outros, e clicar no marcado não desmarca.
#[test]
fn buttons_in_a_group_are_exclusive() {
    let (mut ui, form) = setup();
    let ids: Vec<ComponentId> = (0..3).map(|i| ui.add(Button::new(&format!("Aba {i}")), form)).collect();
    for &b in &ids {
        ui.set_prop(b, "checkable", true).unwrap();
        ui.set_prop(b, "group_index", 1).unwrap();
    }
    ui.click(ids[0]);
    ui.click(ids[1]);
    ui.click(ids[1]);
    let checked: Vec<bool> = ids.iter().map(|&b| ui.get_prop(b, "checked").unwrap() == PropValue::Bool(true)).collect();
    assert_eq!(checked, [false, true, false]);
}

/// C14 e C21: o `auto_size` mede o texto (o botão, a caixa de marcar e a lista).
#[test]
fn auto_size_measures_buttons_checkboxes_and_combos() {
    let (mut ui, form) = setup();
    let short = ui.add(Button::new("OK"), form);
    let long = ui.add(Button::new("Salvar e continuar depois"), form);
    let check = ui.add(CheckBox::new("Lembrar de mim neste computador"), form);
    let combo = ui.add(ComboBox::new(&["A", "Uma opção bem mais comprida"]), form);
    for id in [short, long, check, combo] {
        ui.set_prop(id, "auto_size", true).unwrap();
    }
    ui.layout();
    let w = |ui: &Ui, id| ui.control(id).unwrap().bounds().width;
    assert!(w(&ui, short) < w(&ui, long), "{} × {}", w(&ui, short), w(&ui, long));
    assert!(w(&ui, check) > 180.0, "a caixa e o texto: {}", w(&ui, check));
    assert!(w(&ui, combo) > 150.0, "a opção mais larga e a seta: {}", w(&ui, combo));
}

/// C36: sem `auto_select`, entrar no campo não seleciona o texto.
#[test]
fn auto_select_off_keeps_the_caret() {
    let (mut ui, form) = setup();
    let on = ui.add(Edit::new("abc"), form);
    let off = ui.add(Edit::new("abc"), form);
    ui.set_prop(off, "auto_select", false).unwrap();
    ui.set_focus(on);
    assert_eq!(ui.get::<Edit>(on).unwrap().selected_text(), "abc");
    ui.set_focus(off);
    assert_eq!(ui.get::<Edit>(off).unwrap().selected_text(), "");
}

/// C16 e C15: o menu avisa ao fechar; o item de título não é escolhido.
#[test]
fn the_popup_menu_reports_its_close_and_skips_headers() {
    let (mut ui, form) = setup();
    let menu = ui.add(PopupMenu::default(), form);
    let header = ui.add(MenuItem::new("Câmera 2"), menu);
    ui.set_prop(header, "header", true).unwrap();
    let open = ui.add(MenuItem::new("Abrir"), menu);
    let closed = count(&mut ui, menu, "on_close");
    let clicked = count(&mut ui, open, "on_click");
    PopupMenu::popup(&mut ui, menu, PointF::new(10.0, 10.0));
    ui.layout();
    // ↓ pula a cabeça e vai para "Abrir"; Enter escolhe e fecha.
    ui.key_down(form, key(Key::Down));
    ui.key_down(form, key(Key::Enter));
    ui.process_events();
    assert_eq!((*clicked.borrow(), *closed.borrow()), (1, 1));
}

/// C18: a página que aparece avisa.
#[test]
fn a_tab_page_reports_when_shown() {
    let (mut ui, form) = setup();
    let tabs = ui.add(TabControl::default(), form);
    let _a = ui.add(TabItem::new("A"), tabs);
    let b = ui.add(TabItem::new("B"), tabs);
    let shown = count(&mut ui, b, "on_show");
    ui.layout();
    ui.set_prop(tabs, "tab_index", 1).unwrap();
    ui.process_events();
    assert_eq!(*shown.borrow(), 1);
}

/// C20: o grupo vira cabeça na lista, e a opção desligada não se escolhe nem pelas setas.
#[test]
fn combo_groups_and_disabled_items() {
    let (mut ui, form) = setup();
    let combo = ui.add(ComboBox::new(&["Monitor 1", "Monitor 2", "Janela"]), form);
    ui.set_prop(combo, "groups", "Monitores;Monitores;Outros").unwrap();
    ui.set_prop(combo, "disabled_items", "1").unwrap();
    ui.set_prop(combo, "item_index", 0).unwrap();
    ui.set_focus(combo);
    ui.key_down(form, key(Key::Down));
    assert_eq!(ui.get_prop(combo, "item_index").unwrap(), PropValue::Int(2), "o Monitor 2 está desligado");
    let c = ui.get_mut::<ComboBox>(combo).unwrap();
    assert!(!c.item_enabled(1));
    c.set_item_enabled(1, true);
    assert!(c.item_enabled(1) && c.disabled_items.is_empty());
}

/// C22: colar avisa com o texto colado.
#[test]
fn paste_fires_on_paste() {
    struct Board(String);
    impl aylis_core::platform::ClipboardService for Board {
        fn get_text(&mut self) -> Option<String> {
            Some(self.0.clone())
        }
        fn set_text(&mut self, text: &str) {
            self.0 = text.to_string();
        }
    }
    let (mut ui, form) = setup();
    ui.services_mut().clipboard = Some(Box::new(Board("linha 1\nlinha 2".into())));
    let memo = ui.add(Memo::default(), form);
    let got = Rc::new(RefCell::new(String::new()));
    let g = got.clone();
    ui.on(memo, "on_paste", move |_, _, args| *g.borrow_mut() = args.text().unwrap_or_default().to_string());
    ui.set_focus(memo);
    ui.key_down(form, KeyEvent { key: Key::Char('v'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    assert_eq!(got.borrow().as_str(), "linha 1\nlinha 2");
}

/// C23: as marcas novas do Label.
#[test]
fn label_markup_has_color_size_and_italic() {
    let spans = markup_spans("Olá <i>mundo</i> <color=text_muted><small>fraco</small></color> <size=20>grande</size><br>fim");
    let find = |t: &str| spans.iter().find(|s| s.text == t).unwrap().clone();
    assert!(find("mundo").italic);
    let weak = find("fraco");
    assert_eq!((weak.color.as_deref(), weak.scale), (Some("text_muted"), 0.85));
    assert_eq!(find("grande").size, Some(20.0));
    assert!(spans.iter().any(|s| s.text == "\n"));
    assert_eq!(markup_plain("<b>a</b> <x>b</x>"), "a <x>b</x>", "o que não é marca fica");
}

/// C26: o menu da bandeja pelos MenuItem de um PopupMenu.
#[test]
fn the_tray_menu_comes_from_a_popup_menu() {
    let (mut ui, form) = setup();
    let menu = ui.add(PopupMenu::default(), form);
    ui.add(MenuItem::new("Abrir"), menu);
    let tray = ui.add(TrayIcon::default(), form);
    ui.set_prop(tray, "popup_menu", PropValue::Component(Some(menu))).unwrap();
    assert_eq!(ui.get_prop(tray, "popup_menu").unwrap(), PropValue::Component(Some(menu)));
    assert!(ui.component(tray).unwrap().methods().iter().any(|m| m.name == "show_balloon"));
}

/// C29 e G19: o nome para o leitor de tela e o `translate` desligado.
#[test]
fn accessible_name_and_translate() {
    let (mut ui, form) = setup();
    let b = ui.add(Button::new("⚙"), form);
    ui.set_prop(b, "accessible_name", "Configurações").unwrap();
    assert_eq!(ui.get_prop(b, "accessible_name").unwrap(), PropValue::String("Configurações".into()));
    ui.set_translations([("Salvar".to_string(), "Save".to_string())].into());
    let fixed = ui.add(Label::new(""), form);
    ui.set_prop(fixed, "translate", false).unwrap();
    ui.set_translated(fixed, "text", "Salvar").unwrap();
    let free = ui.add(Label::new(""), form);
    ui.set_translated(free, "text", "Salvar").unwrap();
    assert_eq!(ui.get_prop(fixed, "text").unwrap(), PropValue::String("Salvar".into()));
    assert_eq!(ui.get_prop(free, "text").unwrap(), PropValue::String("Save".into()));
}

/// C19: o atraso da dica vem do app ou do tema.
#[test]
fn the_hint_delay_is_configurable() {
    let (mut ui, _) = setup();
    assert_eq!(ui.hint_delay(), HINT_DELAY);
    let mut theme = ui.theme().clone();
    theme.durations.insert("hint".into(), 400.0);
    ui.set_theme(theme);
    assert!((ui.hint_delay() - 0.4).abs() < 1e-9);
    ui.set_hint_delay(Some(0.0));
    assert_eq!(ui.hint_delay(), 0.0);
}

/// C30: o dia e a hora juntos; as setas somam o dia ou os minutos.
#[test]
fn date_time_edit_keeps_day_and_time() {
    let (mut ui, form) = setup();
    let dt = ui.add(DateTimeEdit::new("2026-11-12 14:30"), form);
    assert_eq!(ui.get_prop(dt, "date_time").unwrap(), PropValue::String("2026-11-12T14:30".into()));
    ui.set_focus(dt);
    // O cursor no começo (a data): ↑ soma um dia.
    ui.get_mut::<DateTimeEdit>(dt).unwrap().edit.select(0, 0);
    ui.key_down(form, key(Key::Up));
    assert_eq!(ui.get_prop(dt, "date_time").unwrap(), PropValue::String("2026-11-13T14:30".into()));
    // No fim (a hora): ↓ tira um minuto.
    let end = ui.get::<DateTimeEdit>(dt).unwrap().edit.text.chars().count();
    ui.get_mut::<DateTimeEdit>(dt).unwrap().edit.select(end, end);
    ui.key_down(form, key(Key::Down));
    assert_eq!(ui.get_prop(dt, "date_time").unwrap(), PropValue::String("2026-11-13T14:29".into()));
}

/// C15: o Popover abre ao lado do componente, fecha com o clique fora e continua existindo.
#[test]
fn a_popover_opens_by_its_anchor_and_hides_on_outside_press() {
    let (mut ui, form) = setup();
    let button = ui.add(Button::new("Info"), form);
    if let Some(cb) = ui.control_mut(button) {
        cb.set_rect(40.0, 40.0, 80.0, 30.0);
    }
    let panel = ui.add(Panel::new(200.0, 200.0), form);
    let pop = ui.add(Popover::default(), panel);
    let child = ui.add(Label::new("detalhe"), pop);
    let closed = count(&mut ui, pop, "on_close");
    ui.layout();
    Popover::popup(&mut ui, pop, button).unwrap();
    ui.layout();
    assert!(Popover::is_open(&ui, pop));
    assert_eq!(ui.parent(pop), Some(form), "por cima de tudo, na raiz do form");
    let r = ui.absolute_rect(pop).unwrap();
    assert!(r.y >= 70.0 && (r.x - 40.0).abs() < 1.0, "embaixo do botão: {r:?}");
    ui.pointer_down(form, PointerEvent { position: PointF::new(500.0, 350.0), button: PointerButton::Primary, ..Default::default() });
    ui.process_events();
    assert!(!Popover::is_open(&ui, pop));
    assert!(ui.contains(pop) && ui.contains(child), "escondido, não liberado");
    assert_eq!(*closed.borrow(), 1);
}

/// C02 e C25: o tema vê a marca posta direto no campo, e o filho que herda vê a do pai.
#[test]
fn the_theme_sees_checked_fields_and_inherited_state() {
    let (mut ui, form) = setup();
    let check = ui.add(CheckBox::new("x"), form);
    ui.get_mut::<CheckBox>(check).unwrap().checked = true;
    assert_eq!(ui.component(check).unwrap().checked_state(), Some(true));
    let row = ui.add(Button::new(""), form);
    ui.set_prop(row, "checkable", true).unwrap();
    let label = ui.add(Label::new("linha"), row);
    ui.set_prop(label, "inherit_state", true).unwrap();
    ui.set_prop(row, "checked", true).unwrap();
    assert_eq!(ui.component(row).unwrap().checked_state(), Some(true), "o pai marcado, que o filho herda");
    assert!(ui.control(label).unwrap().inherit_state());
}

/// O02: o contexto separa o texto ambíguo; sem tradução própria, aparece só o texto, e as
/// colunas de uma lista não são contexto.
#[test]
fn a_translation_context_disambiguates() {
    let (mut ui, form) = setup();
    ui.set_translations([("arquivo|Abrir".to_string(), "Open".to_string()), ("Abrir".to_string(), "Unlock".to_string())].into());
    let menu = ui.add(Label::new(""), form);
    ui.set_translated(menu, "text", "arquivo|Abrir").unwrap();
    let door = ui.add(Label::new(""), form);
    ui.set_translated(door, "text", "porta|Abrir").unwrap();
    assert_eq!(ui.get_prop(menu, "text").unwrap(), PropValue::String("Open".into()));
    assert_eq!(ui.get_prop(door, "text").unwrap(), PropValue::String("Unlock".into()), "sem a própria: a do texto");
    assert_eq!(aylis_core::i18n::split_context("Ana Souza|Vendas"), None);
    let list = ui.add(ListBox::default(), form);
    ui.set_translated(list, "items", "ana|Vendas;Abrir").unwrap();
    assert_eq!(ui.get_prop(list, "items").unwrap(), PropValue::String("ana|Vendas;Unlock".into()));
}
