//! Item 16 (04/11): o delta separado por tipo e o mestre-detalhe gravado junto: o delta do
//! mestre leva os detalhes na ordem de gravar, o resultado acha o dataset pelo `ref` e a chave
//! gerada do mestre passa aos detalhes.

use aylis_core::platform::LocaleInfo;
use aylis_core::{ComponentId, Ui};
use aylis_data::*;

fn setup() -> Ui {
    let mut ui = Ui::new();
    register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    ui
}

/// Pedidos (id gerado pelo banco) e itens ligados por `pedido_id=id`, com um pedido 1 e dois
/// itens dele já gravados.
fn orders(ui: &mut Ui) -> (ComponentId, ComponentId) {
    let orders = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.set_name(orders, "cdsPedidos").unwrap();
    ui.add(Field::new("id", FieldType::AutoInc), orders);
    ui.add(Field::new("cliente", FieldType::String), orders);
    let source = ui.add_root(DataSource::new(orders));
    let items = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.set_name(items, "cdsItens").unwrap();
    ui.add(Field::new("id", FieldType::AutoInc), items);
    ui.add(Field::new("pedido_id", FieldType::Integer), items);
    ui.add(Field::new("produto", FieldType::String), items);
    ui.set_prop(items, "master_source", source).unwrap();
    ui.set_prop(items, "master_fields", "pedido_id=id").unwrap();
    data_set(ui, orders).load_rows(vec![vec![1.into(), "Maria".into()]]).unwrap();
    data_set(ui, items)
        .load_rows(vec![vec![10.into(), 1.into(), "Caneta".into()], vec![11.into(), 1.into(), "Lápis".into()]])
        .unwrap();
    (orders, items)
}

#[test]
fn the_master_delta_carries_the_details_in_saving_order() {
    let mut ui = setup();
    let (orders, items) = orders(&mut ui);
    // No pedido 1: um item sai e outro muda.
    let mut ds = data_set(&mut ui, items);
    ds.first().unwrap();
    ds.delete().unwrap();
    ds.edit().unwrap();
    ds.set_value("produto", "Lápis 2B").unwrap();
    ds.post().unwrap();
    // Um pedido novo com dois itens.
    let mut ds = data_set(&mut ui, orders);
    ds.append().unwrap();
    ds.set_value("cliente", "João").unwrap();
    ds.post().unwrap();
    for produto in ["Borracha", "Régua"] {
        let mut ds = data_set(&mut ui, items);
        ds.append().unwrap();
        ds.set_value("produto", produto).unwrap();
        ds.post().unwrap();
    }
    // E o pedido 1 é excluído (o banco decide o que fazer com os itens dele).
    let mut ds = data_set(&mut ui, orders);
    ds.first().unwrap();
    ds.delete().unwrap();

    let changes = data_set(&mut ui, orders).delta_with_details().unwrap();
    let order: Vec<(&str, &str)> = changes.iter().map(|c| (c.data_set.as_str(), c.kind.name())).collect();
    assert_eq!(
        order,
        [
            ("cdsPedidos", "insert"),
            ("cdsItens", "update"),
            ("cdsItens", "insert"),
            ("cdsItens", "insert"),
            ("cdsItens", "delete"),
            ("cdsPedidos", "delete"),
        ],
        "o mestre entra antes dos itens e sai depois deles"
    );
    let new_order = changes[0].reference;
    let link = changes[2].master.as_ref().expect("o item novo depende do pedido novo");
    assert_eq!((link.data_set.as_str(), link.reference), ("cdsPedidos", new_order));
    assert_eq!(link.fields, [("pedido_id".to_string(), "id".to_string())]);
    assert!(changes[1].master.is_none(), "o item alterado é de um pedido já gravado");
    // O próprio delta do detalhe continua só dele.
    assert_eq!(data_set(&mut ui, items).delta().unwrap().len(), 4);

    // O filtro por tipo.
    let delta = Delta::new(changes.clone());
    assert_eq!(delta.inserted().count(), 3);
    assert_eq!(delta.updated().count(), 1);
    assert_eq!(delta.deleted().count(), 2);
    assert_eq!(delta.of_data_set("cdsItens").count(), 4);

    // No JSON, o dataset de cada alteração e o mestre de que depende.
    let json: serde_json::Value = serde_json::from_str(&json::delta_to_json("cdsPedidos", &changes)).unwrap();
    assert_eq!(json["changes"][2]["data_set"], "cdsItens");
    assert_eq!(json["changes"][2]["master"]["ref"], new_order);
    assert_eq!(json["changes"][2]["master"]["fields"]["pedido_id"], "id");
}

#[test]
fn the_generated_master_key_goes_to_the_details() {
    let mut ui = setup();
    let (orders, items) = orders(&mut ui);
    let mut ds = data_set(&mut ui, orders);
    ds.append().unwrap();
    ds.set_value("cliente", "João").unwrap();
    ds.post().unwrap();
    let temp = ds.value("id").unwrap();
    assert!(matches!(temp, Value::Int(n) if n < 0), "a chave provisória é negativa");
    let mut ds = data_set(&mut ui, items);
    ds.append().unwrap();
    ds.set_value("produto", "Borracha").unwrap();
    ds.post().unwrap();
    assert_eq!(ds.value("pedido_id").unwrap(), temp);

    // O backend: o `Delta` troca a chave do pedido no item antes de o item ser gravado.
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let s = seen.clone();
    data_set(&mut ui, orders).on_apply(move |changes| {
        let mut delta = Delta::new(changes.to_vec());
        let mut next_id = 500;
        for i in 0..delta.len() {
            let change = delta.change(i).clone();
            s.borrow_mut().push(change.new.iter().find(|(n, _)| n == "pedido_id").map(|(_, v)| v.clone()));
            next_id += 1;
            delta.accept(change.reference, vec![("id".into(), Value::Int(next_id))]);
        }
        delta.into_results()
    });
    data_set(&mut ui, orders).apply_updates().unwrap();
    assert_eq!(*seen.borrow(), [None, Some(Value::Int(501))], "o item foi gravado com o id do pedido");

    // Na tela: o pedido com o id do banco, o item ligado a ele e nada pendente.
    assert_eq!(view(&ui, orders).value("id").unwrap(), Value::Int(501));
    assert_eq!(view(&ui, items).value("pedido_id").unwrap(), Value::Int(501));
    assert_eq!(view(&ui, items).value("id").unwrap(), Value::Int(502));
    assert_eq!(view(&ui, items).record_count(), 1, "o detalhe continua mostrando o item do pedido novo");
    assert_eq!(view(&ui, orders).change_count() + view(&ui, items).change_count(), 0);
}

#[test]
fn a_detail_error_stays_in_the_detail() {
    let mut ui = setup();
    let (orders, items) = orders(&mut ui);
    let mut ds = data_set(&mut ui, items);
    ds.first().unwrap();
    ds.edit().unwrap();
    ds.set_value("produto", "").unwrap();
    ds.post().unwrap();
    data_set(&mut ui, orders).on_apply(|changes| changes.iter().map(|c| ApplyResult::error(c.reference, "informe o produto")).collect());
    let err = data_set(&mut ui, orders).apply_updates().unwrap_err();
    assert!(err.message.contains("informe o produto"));
    assert_eq!(view(&ui, items).change_count(), 1, "o item continua pendente");
    assert_eq!(data_set(&mut ui, orders).apply_errors(), 1);
}

#[test]
fn a_separate_detail_does_not_go_with_the_master() {
    let mut ui = setup();
    let (orders, items) = orders(&mut ui);
    ui.set_prop(items, "apply_separately", true).unwrap();
    let mut ds = data_set(&mut ui, items);
    ds.first().unwrap();
    ds.delete().unwrap();
    assert!(data_set(&mut ui, orders).delta_with_details().unwrap().is_empty());
    assert_eq!(data_set(&mut ui, items).delta_with_details().unwrap().len(), 1);
}
