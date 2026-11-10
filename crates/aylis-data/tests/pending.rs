//! Índices nomeados, filtro com parâmetros e a situação de alteração de cada registro.

use aylis_core::{ComponentId, Ui};
use aylis_data::expr::Expr;
use aylis_data::*;

/// Clientes: id, nome, cidade, limite.
fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String), cds);
    ui.add(Field::new("cidade", FieldType::String), cds);
    ui.add(Field::new("limite", FieldType::Integer), cds);
    let rows = [(1, "Bia", "Rio", 500), (2, "Ana", "São Paulo", 3000), (3, "Caio", "São Paulo", 800), (4, "Duda", "Rio", 5000)]
        .into_iter()
        .map(|(id, name, city, limit)| vec![Value::Int(id), name.into(), city.into(), Value::Int(limit)])
        .collect();
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    (ui, cds)
}

fn names(ui: &mut Ui, cds: ComponentId) -> Vec<String> {
    let mut ds = data_set(ui, cds);
    let keep = ds.rec_no();
    let mut out = Vec::new();
    ds.first().unwrap();
    for _ in 0..ds.record_count() {
        out.push(ds.text("nome").unwrap());
        ds.next().unwrap();
    }
    if let Some(i) = keep {
        ds.move_to(i).unwrap();
    }
    out
}

fn go(ui: &mut Ui, cds: ComponentId, name: &str) {
    assert!(data_set(ui, cds).locate("nome", name).unwrap(), "{name}");
}

#[test]
fn named_indexes_switch_the_order_by_name() {
    let (mut ui, cds) = setup();
    ui.set_prop(cds, "indexes", "porNome=nome|porLimite=limite desc|porCidade=cidade;nome").unwrap();
    assert_eq!(ui.get::<ClientDataSet>(cds).unwrap().index_names(), ["porNome", "porLimite", "porCidade"]);
    data_set(&mut ui, cds).set_index_name("porLimite").unwrap();
    assert_eq!(names(&mut ui, cds), ["Duda", "Ana", "Caio", "Bia"]);
    data_set(&mut ui, cds).set_index_name("porcidade").unwrap(); // sem diferenciar maiúsculas
    assert_eq!(names(&mut ui, cds), ["Bia", "Duda", "Ana", "Caio"]);
    // Nome que não existe: erro, e o índice anterior fica.
    assert!(data_set(&mut ui, cds).set_index_name("porIdade").unwrap_err().message.contains("porIdade"));
    assert_eq!(names(&mut ui, cds), ["Bia", "Duda", "Ana", "Caio"]);
    // Ordenar por campos deixa de usar o nome (como IndexFieldNames x IndexName).
    data_set(&mut ui, cds).set_index("nome").unwrap();
    assert_eq!(ui.get::<ClientDataSet>(cds).unwrap().index_name, "");
    assert_eq!(names(&mut ui, cds), ["Ana", "Bia", "Caio", "Duda"]);
    // Pela propriedade (Object Inspector, JSON), refaz sozinho.
    ui.set_prop(cds, "index_name", "porLimite").unwrap();
    ui.process_posted();
    assert_eq!(names(&mut ui, cds), ["Duda", "Ana", "Caio", "Bia"]);
}

#[test]
fn the_filter_takes_parameters() {
    let e = Expr::parse("cidade = :cidade and limite > :minimo").unwrap();
    assert_eq!(e.params(), ["cidade", "minimo"]);
    assert_eq!(e.fields(), ["cidade", "limite"], "parâmetros não são campos");
    let (mut ui, cds) = setup();
    {
        let mut ds = data_set(&mut ui, cds);
        // Sem valor: erro que diz qual falta.
        let err = ds.set_filter("cidade = :cidade and limite > :minimo").unwrap_err();
        assert!(err.message.contains(":cidade"), "{}", err.message);
        ds.set_param("cidade", "Rio").unwrap();
        ds.set_param(":minimo", 1000).unwrap();
    }
    assert_eq!(names(&mut ui, cds), ["Duda"]);
    // Trocar o valor refiltra.
    data_set(&mut ui, cds).set_param("minimo", 100).unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Duda"]);

    // Pela propriedade `params` (texto; número vira número).
    let (mut ui, cds) = setup();
    ui.set_prop(cds, "params", "cidade=São Paulo;minimo=1000").unwrap();
    ui.set_prop(cds, "filter", "cidade = :cidade and limite > :minimo").unwrap();
    ui.set_prop(cds, "filtered", true).unwrap();
    ui.process_posted();
    assert_eq!(names(&mut ui, cds), ["Ana"]);
    // O valor dado por código vale mais que o do texto.
    data_set(&mut ui, cds).set_param("minimo", 500).unwrap();
    assert_eq!(names(&mut ui, cds), ["Ana", "Caio"]);
}

#[test]
fn each_record_knows_its_change_status_and_can_be_reverted() {
    let (mut ui, cds) = setup();
    assert_eq!(view(&ui, cds).update_status(), Some(RowStatus::Unmodified));
    // Altera a Bia, inclui o Eli, exclui a Duda.
    go(&mut ui, cds, "Bia");
    {
        let mut ds = data_set(&mut ui, cds);
        ds.edit().unwrap();
        ds.set_value("limite", 900).unwrap();
        ds.post().unwrap();
        assert_eq!(ds.view().update_status(), Some(RowStatus::Modified));
        ds.append().unwrap();
        assert_eq!(ds.view().update_status(), Some(RowStatus::Inserted), "durante a inclusão");
        ds.set_value("id", 5).unwrap();
        ds.set_value("nome", "Eli").unwrap();
        ds.post().unwrap();
        assert_eq!(ds.view().update_status(), Some(RowStatus::Inserted));
    }
    go(&mut ui, cds, "Duda");
    data_set(&mut ui, cds).delete().unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Ana", "Caio", "Eli"]);

    // Só os alterados e incluídos (StatusFilter).
    data_set(&mut ui, cds).set_status_filter(&[RowStatus::Modified, RowStatus::Inserted]).unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Eli"]);
    ui.set_prop(cds, "status_filter", "unmodified").unwrap();
    ui.process_posted();
    assert_eq!(names(&mut ui, cds), ["Ana", "Caio"]);
    data_set(&mut ui, cds).set_status_filter(&[]).unwrap();

    // Desfazer só um registro (RevertRecord): a Bia volta ao original; o Eli sai.
    go(&mut ui, cds, "Bia");
    data_set(&mut ui, cds).revert_record().unwrap();
    assert_eq!(view(&ui, cds).update_status(), Some(RowStatus::Unmodified));
    assert_eq!(data_set(&mut ui, cds).value("limite").unwrap(), Value::Int(500));
    go(&mut ui, cds, "Eli");
    data_set(&mut ui, cds).revert_record().unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Ana", "Caio"]);
    // A excluída volta pelo id.
    let id = ui.get::<ClientDataSet>(cds).unwrap().deleted_rows()[0].id;
    assert!(data_set(&mut ui, cds).restore_deleted(id).unwrap());
    assert_eq!(data_set(&mut ui, cds).text("nome").unwrap(), "Duda", "o cursor vai até ela");
    assert!(!data_set(&mut ui, cds).restore_deleted(id).unwrap());
    // Nada pendente: o delta está vazio.
    assert_eq!(view(&ui, cds).change_count(), 0);
}
