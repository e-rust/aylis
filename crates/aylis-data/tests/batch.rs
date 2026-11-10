//! Carga em massa: lotes montados em threads (`RowBatch`), juntados e carregados de uma vez.

use aylis_core::Ui;
use aylis_data::*;

#[test]
fn batches_built_in_threads_load_at_once() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("cidade", FieldType::String), cds);
    ui.add(Field::new("valor", FieldType::Float), cds);
    // Um campo calculado no meio: o lote só tem os de dados.
    ui.add(Field::new("dobro", FieldType::Float).calculated("valor * 2"), cds);
    let cidades = ["Recife", "Belém", "Curitiba"];
    // Dois lotes em paralelo, com os textos em ordens diferentes (dicionários diferentes).
    let lotes: Vec<RowBatch> = std::thread::scope(|s| {
        let feitos: Vec<_> = (0..2)
            .map(|t| {
                s.spawn(move || {
                    let mut lote = RowBatch::new(3);
                    for i in 0..1000i64 {
                        let id = t * 1000 + i;
                        let cidade = cidades[((id + t) % 3) as usize];
                        lote.int(id).text(cidade);
                        if id % 10 == 0 { lote.null() } else { lote.float(id as f64 / 2.0) };
                        lote.end_row();
                    }
                    lote
                })
            })
            .collect();
        feitos.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut todos = RowBatch::default();
    for (i, lote) in lotes.into_iter().enumerate() {
        if i == 0 { todos = lote } else { todos.append(lote) }
    }
    assert_eq!(todos.len(), 2000);
    let mut ds = data_set(&mut ui, cds);
    ds.set_index("id desc").unwrap();
    ds.load_batch(todos).unwrap();
    assert_eq!(ds.record_count(), 2000);
    // A ordem escolhida vale: o maior id primeiro.
    ds.first().unwrap();
    assert_eq!(ds.value("id").unwrap(), Value::Int(1999));
    assert_eq!(ds.value("cidade").unwrap(), Value::String(cidades[((1999 + 1) % 3) as usize].into()));
    assert_eq!(ds.value("valor").unwrap(), Value::Float(999.5));
    // Um nulo e um texto do segundo lote (que foi juntado ao dicionário do primeiro).
    ds.set_index("id").unwrap();
    ds.first().unwrap();
    assert_eq!(ds.value("valor").unwrap(), Value::Null);
    for _ in 0..1500 {
        ds.next().unwrap();
    }
    assert_eq!(ds.value("id").unwrap(), Value::Int(1500));
    assert_eq!(ds.value("cidade").unwrap(), Value::String(cidades[((1500 + 1) % 3) as usize].into()));
    // Número de colunas errado: erro, os dados de antes continuam.
    assert!(ds.load_batch(RowBatch::new(5)).is_err());
    assert_eq!(ds.record_count(), 2000);
}

#[test]
fn aligned_batches_join_without_copying_and_keep_every_value() {
    // Lotes com RowBatch::PART registros: os blocos passam de um para o outro.
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String), cds);
    ui.add(Field::new("valor", FieldType::Float), cds);
    let part = RowBatch::PART;
    let lote = |first: usize, n: usize, nomes: &[&str]| {
        let mut l = RowBatch::new(3);
        for r in first..first + n {
            l.int(r as i64).text(nomes[r % nomes.len()]);
            if r % 7 == 0 { l.null() } else { l.float(r as f64) };
            l.end_row();
        }
        l
    };
    let mut todos = lote(0, part, &["a", "b"]);
    todos.append(lote(part, part, &["c", "a"]));
    todos.append(lote(2 * part, 100, &["b", "d"]));
    let n = 2 * part + 100;
    assert_eq!(todos.len(), n);
    let mut ds = data_set(&mut ui, cds);
    ds.load_batch(todos).unwrap();
    let nomes = |r: usize| if r < part { ["a", "b"][r % 2] } else if r < 2 * part { ["c", "a"][r % 2] } else { ["b", "d"][r % 2] };
    for r in [0, 1, 6, 7, part - 1, part, part + 7, 2 * part - 1, 2 * part, n - 1] {
        ds.move_to(r).unwrap();
        assert_eq!(ds.value("id").unwrap(), Value::Int(r as i64), "id {r}");
        assert_eq!(ds.value("nome").unwrap(), Value::String(nomes(r).into()), "nome {r}");
        let valor = if r % 7 == 0 { Value::Null } else { Value::Float(r as f64) };
        assert_eq!(ds.value("valor").unwrap(), valor, "valor {r}");
    }
}
