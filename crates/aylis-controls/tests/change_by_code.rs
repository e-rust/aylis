//! Mudar o valor de um controle pelo código dispara o `on_change` (decidido pelo
//! autor na conferência de 08/11): o `set_prop`, o acesso tipado e o protocolo. Com o mesmo
//! argumento que o controle manda quando o usuário muda; só se o valor mudou; e não no
//! designer nem ao carregar.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{CheckBox, ComboBox, Edit, Form, NumberBox, TrackBar};
use aylis_core::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("teste", 400.0, 300.0));
    (ui, form)
}

/// Guarda os argumentos de cada `on_change` de `id`.
fn record(ui: &mut Ui, id: ComponentId) -> Rc<RefCell<Vec<EventArgs>>> {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let s = seen.clone();
    ui.on(id, "on_change", move |_, _, args| s.borrow_mut().push(args.clone()));
    seen
}

#[test]
fn edit_text_by_code_fires_on_change_once_per_change() {
    let (mut ui, form) = setup();
    let edit = ui.add(Edit::new(""), form);
    let seen = record(&mut ui, edit);
    ui.set_prop(edit, "text", "Ana").unwrap();
    assert_eq!(*seen.borrow(), vec![EventArgs::Value("Ana".into())]);
    // O mesmo valor: não muda, não dispara.
    ui.set_prop(edit, "text", "Ana").unwrap();
    assert_eq!(seen.borrow().len(), 1);
    // O acesso tipado passa pelo mesmo caminho.
    handles::write(&mut ui, edit, "text", "Bia");
    assert_eq!(seen.borrow().last(), Some(&EventArgs::Value("Bia".into())));
    // Outra propriedade (não é o valor) não dispara.
    ui.set_prop(edit, "max_length", 10).unwrap();
    assert_eq!(seen.borrow().len(), 2);
    // Quem muda o próprio valor e dispara ele mesmo grava sem o evento automático.
    ui.set_prop_quiet(edit, "text", "Caio").unwrap();
    assert_eq!(seen.borrow().len(), 2);
}

#[test]
fn checkbox_checked_fires_but_the_caption_does_not() {
    let (mut ui, form) = setup();
    let check = ui.add(CheckBox::new("Ativo"), form);
    let seen = record(&mut ui, check);
    ui.set_prop(check, "checked", true).unwrap();
    ui.set_prop(check, "checked", false).unwrap();
    assert_eq!(*seen.borrow(), vec![EventArgs::Value(true.into()), EventArgs::Value(false.into())]);
    // O `text` do CheckBox é a legenda (aparência), não o valor.
    ui.set_prop(check, "text", "Inativo").unwrap();
    assert_eq!(seen.borrow().len(), 2);
}

#[test]
fn combobox_item_index_sends_the_item_text_like_the_user_choice() {
    let (mut ui, form) = setup();
    let combo = ui.add(ComboBox::new(&["Azul", "Verde", "Vermelho"]), form);
    let seen = record(&mut ui, combo);
    ui.set_prop(combo, "item_index", 1).unwrap();
    assert_eq!(*seen.borrow(), vec![EventArgs::Text("Verde".into())]);
    assert_eq!(seen.borrow()[0].text(), Some("Verde"));
}

#[test]
fn trackbar_and_numberbox_value() {
    let (mut ui, form) = setup();
    let track = ui.add(TrackBar::new(0.0, 0.0, 100.0), form);
    let number = ui.add(NumberBox::new(0.0, 0.0, 100.0), form);
    let t = record(&mut ui, track);
    let n = record(&mut ui, number);
    ui.set_prop(track, "value", 40.0).unwrap();
    assert_eq!(*t.borrow(), vec![EventArgs::Value(PropValue::Float(40.0))]);
    // O valor cortado pelo máximo: o evento leva o que ficou.
    ui.set_prop(track, "value", 500.0).unwrap();
    assert_eq!(ui.get_prop(track, "value").unwrap(), PropValue::Float(100.0));
    assert_eq!(t.borrow().last(), Some(&EventArgs::Value(PropValue::Float(100.0))));
    // O NumberBox manda o número; o `text` sozinho (o que se digita) só vale ao confirmar.
    ui.set_prop(number, "value", 7.0).unwrap();
    ui.set_prop(number, "text", "8").unwrap();
    assert_eq!(*n.borrow(), vec![EventArgs::Value(PropValue::Float(7.0))]);
}

#[test]
fn the_designer_does_not_fire() {
    let (mut ui, form) = setup();
    ui.set_design_mode(true);
    let edit = ui.add(Edit::new(""), form);
    let seen = record(&mut ui, edit);
    ui.set_prop(edit, "text", "Ana").unwrap();
    assert!(seen.borrow().is_empty());
}

#[test]
fn a_handler_that_changes_its_own_value_stops() {
    let (mut ui, form) = setup();
    // O handler que muda o próprio valor dispara de novo: o que converge para.
    let edit = ui.add(Edit::new(""), form);
    let count = Rc::new(RefCell::new(0));
    let c = count.clone();
    ui.on(edit, "on_change", move |ui, sender, _| {
        *c.borrow_mut() += 1;
        let text = ui.get_prop(sender, "text").unwrap().to_str().unwrap().to_uppercase();
        ui.set_prop(sender, "text", text).unwrap();
    });
    ui.set_prop(edit, "text", "ana").unwrap();
    assert_eq!(ui.get_prop(edit, "text").unwrap(), PropValue::String("ANA".into()));
    assert_eq!(*count.borrow(), 2);

    // O que nunca para (inverte sempre) bate no limite de eventos aninhados: vira erro, sem
    // travar nem estourar a pilha.
    let check = ui.add(CheckBox::new("Liga"), form);
    let errors = Rc::new(RefCell::new(Vec::new()));
    let e = errors.clone();
    ui.on_exception(move |_, ex| {
        e.borrow_mut().push(ex.message.clone());
        true
    });
    ui.on(check, "on_change", move |ui, sender, _| {
        let on = ui.get_prop(sender, "checked").unwrap().to_bool().unwrap();
        ui.set_prop(sender, "checked", !on).unwrap();
    });
    ui.set_prop(check, "checked", true).unwrap();
    assert_eq!(errors.borrow().len(), 1, "{:?}", errors.borrow());
    assert!(errors.borrow()[0].contains("recursivo"), "{:?}", errors.borrow());
    // E o `on_change` continua valendo depois.
    ui.set_prop(edit, "text", "bia").unwrap();
    assert_eq!(*count.borrow(), 4);
}
