//! O menu de texto padrão dos campos (Recortar, Copiar, Colar, Selecionar tudo) no botão
//! direito e no toque longo, a palavra selecionada pelo toque e as alças da seleção.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::platform::ClipboardService;
use aylis_core::*;

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

/// Um canvas que só guarda as bolinhas pintadas (as alças), no form.
#[derive(Default)]
struct Ellipses {
    found: Vec<(RectF, Color)>,
    at: PointF,
    saved: Vec<PointF>,
}

impl Canvas for Ellipses {
    fn save(&mut self) {
        self.saved.push(self.at);
    }
    fn restore(&mut self) {
        self.at = self.saved.pop().unwrap_or_default();
    }
    fn translate(&mut self, x: f32, y: f32) {
        self.at = PointF::new(self.at.x + x, self.at.y + y);
    }
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        if let Brush::Solid(c) = brush {
            self.found.push((rect.offset(self.at.x, self.at.y), *c));
        }
    }
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3)
    }
}

/// Um form com um campo em (10, 10, 300 × 32) e a área de transferência falsa.
fn setup(field: impl Component) -> (Ui, ComponentId, ComponentId, FakeClipboard) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let clipboard = FakeClipboard::default();
    ui.services_mut().clipboard = Some(Box::new(clipboard.clone()));
    let f = ui.add_root(Form::new("teste", 500.0, 400.0));
    ui.show(f).unwrap();
    let e = ui.add(field, f);
    ui.control_mut(e).unwrap().set_rect(10.0, 10.0, 300.0, 32.0);
    paint(&mut ui, f);
    (ui, f, e, clipboard)
}

fn paint(ui: &mut Ui, f: ComponentId) -> Vec<(RectF, Color)> {
    ui.layout();
    let mut canvas = Ellipses::default();
    ui.paint(f, &mut canvas);
    canvas.found
}

/// O x (no form) do começo do caractere `n` do Edit (a fonte de 14 da medida falsa: 7,7 por
/// caractere, depois da borda e do recuo do campo).
fn x_of(n: usize) -> f32 {
    10.0 + 8.0 + n as f32 * 7.7
}

fn touch(x: f32, y: f32) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), kind: PointerKind::Touch, click_count: 1, ..Default::default() }
}

fn mouse(x: f32, y: f32, button: PointerButton) -> PointerEvent {
    PointerEvent { position: PointF::new(x, y), button, click_count: 1, ..Default::default() }
}

/// Segura o dedo em `(x, y)` até o toque longo vencer e solta.
fn long_press(ui: &mut Ui, f: ComponentId, x: f32, y: f32) {
    ui.set_time(1.0);
    ui.pointer_down(f, touch(x, y));
    ui.set_time(1.6);
    ui.run_timers();
    ui.pointer_up(f, touch(x, y));
}

fn right_click(ui: &mut Ui, f: ComponentId, x: f32, y: f32) {
    ui.pointer_down(f, mouse(x, y, PointerButton::Secondary));
    ui.pointer_up(f, mouse(x, y, PointerButton::Secondary));
}

/// O menu aberto agora (a camada do form).
fn overlay(ui: &Ui, f: ComponentId) -> Option<ComponentId> {
    ui.children(f).iter().copied().find(|&c| ui.class_name(c) == Some("MenuOverlay"))
}

/// Os itens (texto, ligado) do menu de texto do campo.
fn items(ui: &Ui, field: ComponentId) -> Vec<(String, bool)> {
    let menu = ui.children(field).iter().copied().find(|&c| ui.class_name(c) == Some("PopupMenu")).expect("o menu do campo");
    ui.children(menu).iter().filter_map(|&c| ui.get::<MenuItem>(c)).map(|m| (m.text.clone(), m.enabled)).collect()
}

/// Escolhe o item pelo teclado do menu aberto (setas e Enter): o caminho do usuário.
fn choose(ui: &mut Ui, f: ComponentId, field: ComponentId, label: &str) {
    assert!(overlay(ui, f).is_some(), "o menu está aberto");
    let list = items(ui, field);
    let selectable: Vec<&(String, bool)> = list.iter().filter(|(t, on)| *on && t != "-").collect();
    let downs = selectable.iter().position(|(t, _)| t == label).unwrap_or_else(|| panic!("{label} ligado em {list:?}")) + 1;
    for _ in 0..downs {
        ui.key_down(f, KeyEvent { key: Key::Down, ..Default::default() });
    }
    ui.key_down(f, KeyEvent { key: Key::Enter, ..Default::default() });
    ui.process_events();
    assert!(overlay(ui, f).is_none(), "o menu fechou");
}

fn all_four() -> Vec<(String, bool)> {
    ["Recortar", "Copiar", "Colar", "Selecionar tudo"].iter().map(|t| (t.to_string(), true)).collect()
}

#[test]
fn long_press_selects_the_word_and_opens_the_text_menu() {
    let (mut ui, f, e, clipboard) = setup(Edit::new("olá mundo bonito"));
    long_press(&mut ui, f, x_of(6), 26.0);
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (4, 9), "a palavra sob o dedo");
    assert_eq!(items(&ui, e), all_four());
    assert!(overlay(&ui, f).is_some(), "soltar o dedo não fecha o menu");
    // As alças, na cor `primary` do tema, embaixo das pontas da seleção.
    let primary = ui.theme().color("primary");
    let handles: Vec<RectF> = paint(&mut ui, f).into_iter().filter(|(_, c)| *c == primary).map(|(r, _)| r).collect();
    assert_eq!(handles.len(), 2, "as duas alças");
    assert!((handles[0].center().x - x_of(4)).abs() < 1.0 && (handles[1].center().x - x_of(9)).abs() < 1.0, "{handles:?}");
    assert!(handles[0].y >= 26.0, "embaixo do texto");
    choose(&mut ui, f, e, "Copiar");
    assert_eq!(*clipboard.0.borrow(), "mundo");
    assert_eq!(ui.focused(f), Some(e), "o foco volta ao campo");
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (4, 9), "a seleção fica");
}

#[test]
fn right_click_opens_the_same_menu_and_each_item_goes_the_keyboard_way() {
    let (mut ui, f, e, clipboard) = setup(Edit::new("olá mundo bonito"));
    ui.set_prop(e, "auto_select", false).unwrap();
    let changes = Rc::new(RefCell::new(Vec::new()));
    for event in ["on_change", "on_paste"] {
        let c = changes.clone();
        ui.on(e, event, move |_, _, _| c.borrow_mut().push(event));
    }
    ui.set_focus(e);
    ui.set_prop(e, "sel_start", 4_i64).unwrap();
    ui.set_prop(e, "sel_length", 5_i64).unwrap();
    right_click(&mut ui, f, x_of(6), 26.0);
    assert_eq!(items(&ui, e), all_four());

    // Recortar: some do texto e vai para a área de transferência.
    ui.set_time(1.0);
    choose(&mut ui, f, e, "Recortar");
    assert_eq!(ui.get::<Edit>(e).unwrap().text, "olá  bonito");
    assert_eq!(*clipboard.0.borrow(), "mundo");
    // Sem seleção, Recortar e Copiar ficam desligados.
    right_click(&mut ui, f, x_of(4), 26.0);
    let list = items(&ui, e);
    assert_eq!(list[0], ("Recortar".to_string(), false));
    assert_eq!(list[1], ("Copiar".to_string(), false));
    // Colar: no cursor, com `on_change` e `on_paste`.
    *clipboard.0.borrow_mut() = "mundo".into();
    changes.borrow_mut().clear();
    ui.set_time(3.0);
    choose(&mut ui, f, e, "Colar");
    assert_eq!(ui.get::<Edit>(e).unwrap().text, "olá mundo bonito");
    assert_eq!(*changes.borrow(), ["on_paste", "on_change"]);
    // O desfazer pega a colagem.
    ui.key_down(f, KeyEvent { key: Key::Char('z'), modifiers: Modifiers { ctrl: true, ..Default::default() }, ..Default::default() });
    assert_eq!(ui.get::<Edit>(e).unwrap().text, "olá  bonito");
    // Colar respeita o `max_length` e os caracteres do campo.
    ui.set_prop(e, "allowed_chars", "a-z").unwrap();
    ui.set_prop(e, "only_allowed", true).unwrap();
    ui.set_prop(e, "text", "").unwrap();
    *clipboard.0.borrow_mut() = "a1b2".into();
    right_click(&mut ui, f, x_of(0), 26.0);
    choose(&mut ui, f, e, "Colar");
    assert_eq!(ui.get::<Edit>(e).unwrap().text, "ab");
    // Selecionar tudo.
    right_click(&mut ui, f, x_of(0), 26.0);
    choose(&mut ui, f, e, "Selecionar tudo");
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (0, 2));
}

#[test]
fn read_only_and_password_fields_show_only_what_they_allow() {
    let (mut ui, f, e, _) = setup(Edit::new("segredo"));
    ui.set_prop(e, "read_only", true).unwrap();
    right_click(&mut ui, f, x_of(2), 26.0);
    let names: Vec<String> = items(&ui, e).into_iter().map(|(t, _)| t).collect();
    assert_eq!(names, ["Copiar", "Selecionar tudo"]);
    ui.key_down(f, KeyEvent { key: Key::Escape, ..Default::default() });
    assert!(overlay(&ui, f).is_none());

    ui.set_prop(e, "read_only", false).unwrap();
    ui.set_prop(e, "password", true).unwrap();
    right_click(&mut ui, f, x_of(2), 26.0);
    let names: Vec<String> = items(&ui, e).into_iter().map(|(t, _)| t).collect();
    assert_eq!(names, ["Colar", "Selecionar tudo"], "a senha não sai do campo");
}

#[test]
fn the_app_popup_menu_wins_and_the_property_turns_it_off() {
    let (mut ui, f, e, _) = setup(Edit::new("olá"));
    let menu = ui.add(PopupMenu::default(), f);
    ui.add(MenuItem::new("Do app"), menu);
    ui.set_prop(f, "popup_menu", PropValue::Component(Some(menu))).unwrap();
    right_click(&mut ui, f, x_of(1), 26.0);
    assert!(overlay(&ui, f).is_some());
    assert!(ui.children(e).is_empty(), "o menu de texto nem foi montado");
    ui.key_down(f, KeyEvent { key: Key::Escape, ..Default::default() });

    // Sem o menu do app e com `text_menu` desligado: nada abre.
    ui.set_prop(f, "popup_menu", PropValue::Component(None)).unwrap();
    ui.set_prop(e, "text_menu", false).unwrap();
    right_click(&mut ui, f, x_of(1), 26.0);
    assert!(overlay(&ui, f).is_none());
}

#[test]
fn dragging_a_handle_moves_that_end_of_the_selection() {
    let (mut ui, f, e, _) = setup(Edit::new("olá mundo bonito"));
    long_press(&mut ui, f, x_of(6), 26.0);
    // Fecha o menu (Esc): o foco volta ao campo com a palavra selecionada e as alças.
    ui.key_down(f, KeyEvent { key: Key::Escape, ..Default::default() });
    ui.process_events();
    assert_eq!(ui.focused(f), Some(e));
    let primary = ui.theme().color("primary");
    let handles: Vec<RectF> = paint(&mut ui, f).into_iter().filter(|(_, c)| *c == primary).map(|(r, _)| r).collect();
    assert_eq!(handles.len(), 2);
    // A alça do fim vai até o fim do texto.
    let grab = handles[1].center();
    ui.set_time(5.0);
    ui.pointer_down(f, touch(grab.x, grab.y));
    ui.pointer_move(f, touch(grab.x + 30.0, grab.y));
    ui.pointer_move(f, touch(x_of(16), grab.y));
    ui.pointer_up(f, touch(x_of(16), grab.y));
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (4, 16));
    // A do começo vai até o começo, sem cruzar a outra ponta.
    paint(&mut ui, f);
    let grab = PointF::new(x_of(4), handles[0].center().y);
    ui.set_time(6.0);
    ui.pointer_down(f, touch(grab.x, grab.y));
    ui.pointer_move(f, touch(grab.x - 20.0, grab.y));
    ui.pointer_move(f, touch(x_of(0), grab.y));
    ui.pointer_up(f, touch(x_of(0), grab.y));
    assert_eq!(ui.get::<Edit>(e).unwrap().selection(), (0, 16));
    // O mouse não tem alças: o clique tira a seleção e elas somem.
    ui.pointer_down(f, mouse(x_of(2), 26.0, PointerButton::Primary));
    ui.pointer_up(f, mouse(x_of(2), 26.0, PointerButton::Primary));
    assert!(paint(&mut ui, f).iter().all(|(_, c)| *c != primary));
}

#[test]
fn memo_and_composed_fields_have_the_menu_too() {
    let (mut ui, f, m, clipboard) = setup(Memo::new("primeira linha\nsegunda linha"));
    ui.control_mut(m).unwrap().set_rect(10.0, 10.0, 300.0, 120.0);
    paint(&mut ui, f);
    // A palavra "segunda" (segunda linha) pelo toque longo.
    long_press(&mut ui, f, x_of(3), 10.0 + 8.0 + 20.0 * 1.5 + 2.0);
    let memo = ui.get::<Memo>(m).unwrap();
    assert_eq!(memo.selected_text(), "segunda");
    assert_eq!(items(&ui, m), all_four());
    let primary = ui.theme().color("primary");
    assert_eq!(paint(&mut ui, f).iter().filter(|(_, c)| *c == primary).count(), 2, "as alças no Memo");
    choose(&mut ui, f, m, "Copiar");
    assert_eq!(*clipboard.0.borrow(), "segunda");

    // O NumberBox usa o Edit por dentro: o menu é o dele, e o Colar passa pelo NumberBox.
    let n = ui.add(NumberBox::default(), f);
    ui.control_mut(n).unwrap().set_rect(10.0, 200.0, 200.0, 32.0);
    paint(&mut ui, f);
    ui.set_focus(n);
    *clipboard.0.borrow_mut() = "42".into();
    right_click(&mut ui, f, 30.0, 216.0);
    assert!(items(&ui, n).iter().any(|(t, on)| t == "Colar" && *on));
    choose(&mut ui, f, n, "Colar");
    ui.key_down(f, KeyEvent { key: Key::Enter, ..Default::default() });
    assert_eq!(ui.get_prop(n, "value").ok(), Some(PropValue::Float(42.0)));
}
