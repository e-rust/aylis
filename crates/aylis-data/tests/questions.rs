//! `before_post`, `before_delete` e `on_validate` como eventos-pergunta (item 3.5): o handler
//! em Rust recusa na hora; o backend remoto (aqui, um receptor que adia) segura a gravação
//! até a resposta.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::{Answer, ComponentId, PropValue, Ui};
use aylis_data::*;

fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    register(&mut ui);
    let cds = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String), cds);
    data_set(&mut ui, cds).load_rows(vec![vec![Value::Int(1), "Ana".into()], vec![Value::Int(2), "Bia".into()]]).unwrap();
    (ui, cds)
}

fn deferring_sink(ui: &mut Ui) -> Rc<RefCell<Vec<u64>>> {
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log = asked.clone();
    ui.set_event_sink(move |ui, _, _, _, _| {
        if ui.asking() {
            log.borrow_mut().push(ui.defer_question());
        }
    });
    asked
}

fn errors(ui: &mut Ui) -> Rc<RefCell<Vec<String>>> {
    let shown = Rc::new(RefCell::new(Vec::new()));
    let s = shown.clone();
    ui.set_exception_display(move |_, e| s.borrow_mut().push(e.message.clone()));
    shown
}

#[test]
fn before_post_in_rust_refuses_with_the_message() {
    let (mut ui, cds) = setup();
    let shown = errors(&mut ui);
    ui.on(cds, "before_post", |ui, _, _| ui.refuse("Informe o CPF"));
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_value("nome", "Carla").unwrap();
    assert!(ds.post().is_err());
    assert_eq!(ds.state(), DataSetState::Edit, "recusado, continua em edição");
    assert_eq!(*shown.borrow(), vec!["Informe o CPF".to_string()]);
}

#[test]
fn a_remote_before_post_waits_for_the_answer() {
    let (mut ui, cds) = setup();
    let asked = deferring_sink(&mut ui);
    ui.set_prop(cds, "before_post", PropValue::Event(Some("gravar".into()))).unwrap();
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_value("nome", "Carla").unwrap();
    ds.post().unwrap();
    assert_eq!(ds.state(), DataSetState::Edit, "espera a resposta");
    assert!(ds.is_asking());
    ds.post().unwrap();
    assert_eq!(asked.borrow().len(), 1, "o segundo post não pergunta de novo");
    let token = asked.borrow()[0];
    ui.answer(token, Answer::Accepted(vec![]));
    let ds = data_set(&mut ui, cds);
    assert_eq!(ds.state(), DataSetState::Browse, "gravou com a resposta");
    assert_eq!(ds.text("nome").unwrap(), "Carla");
}

#[test]
fn a_remote_before_delete_refused_keeps_the_record() {
    let (mut ui, cds) = setup();
    let shown = errors(&mut ui);
    let asked = deferring_sink(&mut ui);
    ui.set_prop(cds, "before_delete", PropValue::Event(Some("excluir".into()))).unwrap();
    data_set(&mut ui, cds).delete().unwrap();
    assert_eq!(data_set(&mut ui, cds).record_count(), 2);
    let token = asked.borrow()[0];
    ui.answer(token, Answer::Refused("Tem pedidos".into()));
    assert_eq!(data_set(&mut ui, cds).record_count(), 2);
    assert_eq!(*shown.borrow(), vec!["Tem pedidos".to_string()]);
    // Aceito: sai.
    data_set(&mut ui, cds).delete().unwrap();
    let token = asked.borrow()[1];
    ui.answer(token, Answer::Accepted(vec![]));
    assert_eq!(data_set(&mut ui, cds).record_count(), 1);
}

#[test]
fn on_validate_refuses_the_value_now_in_rust_and_marks_the_field_when_remote() {
    let (mut ui, cds) = setup();
    let field = ui.children(cds)[1];
    ui.on(field, "on_validate", |ui, _, _| ui.refuse("nome curto"));
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    assert!(ds.set_value("nome", "X").is_err());
    assert_eq!(ds.text("nome").unwrap(), "Ana", "o valor recusado não entra");

    let (mut ui, cds) = setup();
    let field = ui.children(cds)[1];
    let asked = deferring_sink(&mut ui);
    ui.set_prop(field, "on_validate", PropValue::Event(Some("validar".into()))).unwrap();
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_value("nome", "X").unwrap();
    ds.post().unwrap();
    assert_eq!(ds.state(), DataSetState::Edit, "o post espera a validação");
    let token = asked.borrow()[0];
    ui.answer(token, Answer::Refused("nome curto".into()));
    let mut ds = data_set(&mut ui, cds);
    assert_eq!(ds.state(), DataSetState::Edit);
    assert!(ds.post().is_err(), "o campo ficou com o erro");
}

#[test]
fn a_remote_apply_waits_for_the_results() {
    let (mut ui, cds) = setup();
    let asked = deferring_sink(&mut ui);
    ui.set_prop(cds, "on_apply_updates", PropValue::Event(Some("gravar".into()))).unwrap();
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_value("nome", "Carla").unwrap();
    ds.post().unwrap();
    ds.apply_updates().unwrap();
    assert!(ds.is_applying(), "aplicando até a resposta");
    assert!(aylis_data::view(&ui, cds).is_waiting());
    // A resposta traz os resultados por registro (aqui, todos gravados).
    let delta = data_set(&mut ui, cds).delta().unwrap();
    let results: Vec<serde_json::Value> = delta.iter().map(|c| serde_json::json!({ "ref": c.reference })).collect();
    let token = asked.borrow()[0];
    ui.answer(token, Answer::Accepted(vec![("results".into(), PropValue::String(serde_json::Value::Array(results).to_string()))]));
    let ds = data_set(&mut ui, cds);
    assert!(!ds.is_applying());
    assert_eq!(ds.apply_errors(), 0);
    assert_eq!(ds.delta().unwrap().len(), 0, "gravou");

    // Recusado: a gravação inteira fica pendente.
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_value("nome", "Dora").unwrap();
    ds.post().unwrap();
    ds.apply_updates().unwrap();
    let token = asked.borrow()[1];
    ui.answer(token, Answer::Refused(String::new()));
    let ds = data_set(&mut ui, cds);
    assert_eq!(ds.apply_errors(), usize::MAX);
    assert_eq!(ds.delta().unwrap().len(), 1);
}
