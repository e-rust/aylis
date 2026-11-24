//! O `load_json` sem campos declarados cria os campos pelos dados.

use aylis_core::Ui;
use aylis_data::*;
use serde_json::json;

fn field_types(ui: &Ui, cds: aylis_core::ComponentId) -> Vec<(String, FieldType)> {
    let v = view(ui, cds);
    v.field_names().into_iter().map(|n| (n.clone(), v.field(&n).unwrap().field_type)).collect()
}

#[test]
fn load_json_without_fields_creates_them_from_the_data() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    let rows = json!([
        {"id": 1, "nome": "Ana", "valor": 10, "ativo": true, "dia": "2026-11-23", "hora": "13:45", "quando": "2026-11-23T13:45:00", "utc": "2026-11-23T13:45:00Z", "preco": "1500.00", "misto": 1, "vazio": null},
        {"id": 2, "nome": "Bruno", "valor": 2.5, "ativo": false, "dia": "2026-11-24", "hora": "08:00:00", "quando": "2026-11-24", "utc": "2026-11-24T08:00:00-03:00", "preco": "12.5", "misto": "x", "vazio": null, "extra": "só aqui"}
    ]);
    data_set(&mut ui, cds).load_json(&rows).unwrap();
    use FieldType::*;
    assert_eq!(
        field_types(&ui, cds),
        [
            ("id", Integer),
            ("nome", String),
            ("valor", Float),
            ("ativo", Boolean),
            ("dia", Date),
            ("hora", Time),
            ("quando", DateTime),
            ("utc", Timestamp),
            // Decimal em texto, tipos misturados e só nulos: texto, na dúvida.
            ("preco", String),
            ("misto", String),
            ("vazio", String),
            ("extra", String),
        ]
        .map(|(n, t)| (n.to_string(), t))
    );
    let v = view(&ui, cds);
    assert_eq!(v.record_count(), 2);
    assert_eq!(v.value_at(1, "valor").unwrap(), Value::Float(2.5));
    assert_eq!(v.value_at(0, "misto").unwrap(), Value::String("1".into()));
    assert_eq!(v.value_at(1, "extra").unwrap(), Value::String("só aqui".into()));
    assert!(v.value_at(0, "extra").unwrap().is_null());
}

#[test]
fn a_new_load_with_other_columns_replaces_the_inferred_fields() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    data_set(&mut ui, cds).load_json(&json!([{"a": 1, "b": "x"}])).unwrap();
    let first: Vec<_> = ui.children(cds).to_vec();
    // As mesmas colunas: os campos ficam (a grade não refaz as colunas a cada carga).
    data_set(&mut ui, cds).load_json(&json!([{"a": 2, "b": "y"}, {"a": 3, "b": "z"}])).unwrap();
    assert_eq!(ui.children(cds).to_vec(), first);
    assert_eq!(view(&ui, cds).record_count(), 2);
    // Outra consulta: outros campos.
    data_set(&mut ui, cds).load_json(&json!([{"cidade": "Recife", "pop": 1_500_000}])).unwrap();
    assert_eq!(field_types(&ui, cds), [("cidade".to_string(), FieldType::String), ("pop".to_string(), FieldType::Integer)]);
    assert_eq!(ui.children(cds).len(), 2);
    assert_eq!(view(&ui, cds).text("cidade").unwrap(), "Recife");
}

#[test]
fn declared_fields_are_never_inferred() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    ui.add(Field::new("a", FieldType::String), cds);
    data_set(&mut ui, cds).load_json(&json!([{"a": "x", "b": 2}])).unwrap();
    assert_eq!(field_types(&ui, cds), [("a".to_string(), FieldType::String)]);
}

#[test]
fn many_rows_type_by_a_sample_spread_over_the_list() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    // Inteiros no começo e um número com casas no fim: o tipo vê o fim também.
    let mut rows: Vec<serde_json::Value> = (0..5000).map(|i| json!({"n": i})).collect();
    rows.push(json!({"n": 0.5}));
    data_set(&mut ui, cds).load_json(&serde_json::Value::Array(rows)).unwrap();
    assert_eq!(field_types(&ui, cds), [("n".to_string(), FieldType::Float)]);
    assert_eq!(view(&ui, cds).record_count(), 5001);
}
