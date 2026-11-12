//! Renomear embutido no MediaCard (`editable_title`, `on_rename`, o método `rename`): o
//! clique no nome abre o campo, a miniatura continua no `on_click` (pedido do NoOBS, 11/11).

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Form, MediaCard};
use aylis_core::*;

fn setup() -> (Ui, ComponentId, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmX", 400.0, 300.0));
    let mut c = MediaCard::default();
    c.title = "Gravação 1".into();
    c.editable_title = true;
    let card = ui.add(c, form);
    ui.control_mut(card).unwrap().set_rect(10.0, 10.0, 170.0, 148.0);
    ui.show(form).unwrap();
    ui.layout();
    (ui, form, card)
}

fn click(ui: &mut Ui, form: ComponentId, x: f32, y: f32) {
    let e = PointerEvent { position: PointF::new(x, y), ..Default::default() };
    ui.pointer_move(form, e);
    ui.pointer_down(form, e);
    ui.pointer_up(form, e);
    ui.process_events();
}

fn log(ui: &mut Ui, id: ComponentId, event: &str) -> Rc<RefCell<Vec<String>>> {
    let l = Rc::new(RefCell::new(Vec::new()));
    let c = l.clone();
    ui.on(id, event, move |_, _, args| {
        c.borrow_mut().push(match args {
            EventArgs::Text(t) => t.clone(),
            other => format!("{other:?}"),
        })
    });
    l
}

fn key(ui: &mut Ui, form: ComponentId, key: Key) {
    ui.key_down(form, KeyEvent { key, ..Default::default() });
    ui.process_events();
}

/// O nome do card fica a 104,5 px do topo dele (a miniatura 16:9 e a folga), aqui em y = 10.
const TITLE_Y: f32 = 10.0 + 104.5 + 8.0;

#[test]
fn clicking_the_title_opens_the_editor_and_enter_fires_on_rename() {
    let (mut ui, form, card) = setup();
    let clicks = log(&mut ui, card, "on_click");
    let renames = log(&mut ui, card, "on_rename");
    click(&mut ui, form, 60.0, TITLE_Y);
    assert!(clicks.borrow().is_empty(), "o clique no nome não abre o item");
    let editor = ui.focused(form).expect("o campo de renomear recebe o foco");
    assert_ne!(editor, card);
    assert_eq!(ui.get_prop(editor, "text").unwrap(), PropValue::String("Gravação 1".into()));
    // O campo fica sobre o nome do card.
    let r = ui.bounds(editor).unwrap();
    assert!(r.y < TITLE_Y && r.bottom() > TITLE_Y && r.x < 20.0, "{r:?}");
    // O nome todo vem selecionado: digitar troca.
    ui.text_input(form, "Abertura");
    key(&mut ui, form, Key::Enter);
    assert_eq!(*renames.borrow(), vec!["Abertura".to_string()]);
    assert_eq!(ui.get_prop(card, "title").unwrap(), PropValue::String("Abertura".into()));
    assert!(!ui.contains(editor), "o campo some depois de confirmar");
    // A miniatura continua abrindo o item.
    click(&mut ui, form, 60.0, 50.0);
    assert_eq!(clicks.borrow().len(), 1);
}

#[test]
fn escape_cancels_and_the_rename_method_opens_the_editor() {
    let (mut ui, form, card) = setup();
    let renames = log(&mut ui, card, "on_rename");
    ui.invoke(card, "rename", &[]).unwrap();
    ui.process_events();
    let editor = ui.focused(form).expect("o rename abre o campo");
    ui.text_input(form, "Outro");
    key(&mut ui, form, Key::Escape);
    assert!(renames.borrow().is_empty(), "Esc não renomeia");
    assert_eq!(ui.get_prop(card, "title").unwrap(), PropValue::String("Gravação 1".into()));
    assert!(!ui.contains(editor));
}

#[test]
fn without_editable_title_the_title_click_is_an_ordinary_click() {
    let (mut ui, form, card) = setup();
    ui.set_prop(card, "editable_title", false).unwrap();
    let clicks = log(&mut ui, card, "on_click");
    click(&mut ui, form, 60.0, TITLE_Y);
    assert_eq!(clicks.borrow().len(), 1);
    assert_ne!(ui.focused(form).map(|f| f != card), Some(true), "nenhum campo abriu");
}

#[test]
fn clicking_outside_the_editor_keeps_the_new_name() {
    let (mut ui, form, card) = setup();
    let renames = log(&mut ui, card, "on_rename");
    click(&mut ui, form, 60.0, TITLE_Y);
    let editor = ui.focused(form).unwrap();
    ui.text_input(form, "Fora");
    // O fundo do form, que não recebe foco.
    click(&mut ui, form, 300.0, 250.0);
    assert_eq!(*renames.borrow(), vec!["Fora".to_string()]);
    assert!(!ui.contains(editor));
}

#[test]
fn clicking_the_card_itself_keeps_the_new_name_without_opening_the_item() {
    let (mut ui, form, card) = setup();
    let renames = log(&mut ui, card, "on_rename");
    let clicks = log(&mut ui, card, "on_click");
    click(&mut ui, form, 60.0, TITLE_Y);
    ui.text_input(form, "Miniatura");
    click(&mut ui, form, 60.0, 50.0);
    assert_eq!(*renames.borrow(), vec!["Miniatura".to_string()]);
    assert!(clicks.borrow().is_empty(), "o clique que fecha o campo não abre o item");
    click(&mut ui, form, 60.0, 50.0);
    assert_eq!(clicks.borrow().len(), 1);
}
