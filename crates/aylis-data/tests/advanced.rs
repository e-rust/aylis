//! Fase 17: expressões, máscaras, campos calculados e lookup, filtro, índice e
//! master-detail.

use aylis_core::platform::LocaleInfo;
use aylis_core::{ComponentId, EventArgs, Ui};
use aylis_data::expr::Expr;
use aylis_data::mask::{self, Mask};
use aylis_data::*;

fn get<'a>(pairs: &'a [(&'a str, Value)]) -> impl Fn(&str) -> Option<Value> + 'a {
    move |name| pairs.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone())
}

#[test]
fn expressions() {
    let row = [
        ("nome", Value::from("Maria Silva")),
        ("cidade", Value::from("São Paulo")),
        ("limite", Value::Decimal(Decimal::new(150000, 2))),
        ("qtd", Value::Int(3)),
        ("ativo", Value::Bool(true)),
        ("email", Value::Null),
        ("nascimento", Value::Date(Date::new(1990, 4, 12).unwrap())),
    ];
    let g = get(&row);
    let t = |e: &str| Expr::parse(e).unwrap_or_else(|err| panic!("{e}: {err}")).eval(&g).unwrap();
    assert_eq!(t("cidade = 'são paulo' and limite >= 1000"), Value::Bool(true), "texto sem diferenciar maiúsculas");
    assert_eq!(t("nome like 'mar%' and not (qtd > 5)"), Value::Bool(true));
    assert_eq!(t("cidade in ('Rio', 'São Paulo')"), Value::Bool(true));
    assert_eq!(t("email is null and nome is not null"), Value::Bool(true));
    assert_eq!(t("nascimento >= '1990-01-01'"), Value::Bool(true));
    assert_eq!(t("limite * qtd"), Value::Decimal(Decimal::new(450000, 2)), "decimal exato");
    assert_eq!(t("qtd * 2 + 1"), Value::Int(7));
    assert_eq!(t("upper(substr(nome, 1, 5)) + '!'"), Value::from("MARIA!"));
    assert_eq!(t("iif(limite > 5000, 'VIP', 'Normal')"), Value::from("Normal"));
    assert_eq!(t("coalesce(email, 'sem e-mail')"), Value::from("sem e-mail"));
    assert_eq!(t("year(nascimento)"), Value::Int(1990));
    assert_eq!(t("[nome] <> 'x' && ativo"), Value::Bool(true));
    assert!(Expr::parse("cidade = ").is_err());
    assert!(Expr::parse("x = 'aberto").is_err());
    assert_eq!(Expr::parse("a = 1 and b > c").unwrap().fields(), ["a", "b", "c"]);
}

#[test]
fn absurd_expressions_are_refused_instead_of_overflowing_the_stack() {
    // Parênteses, sinais, `not` e cadeias longas: todos estourariam a pilha (o processo
    // fecha sem mensagem); agora viram erro.
    let deep = |open: &str, middle: &str, close: &str, n: usize| format!("{}{middle}{}", open.repeat(n), close.repeat(n));
    for text in [
        deep("(", "a", ")", 100_000),
        deep("-", "a", "", 100_000),
        deep("not ", "a", "", 50_000),
        deep("abs(", "a", ")", 50_000),
        vec!["a"; 20_000].join(" + "),
        vec!["a = 1"; 10_000].join(" or "),
    ] {
        let err = Expr::parse(&text).unwrap_err();
        assert!(err.contains("aninhada demais") || err.contains("longa demais"), "{}…: {err}", &text[..20]);
    }
    assert!(Expr::parse(&"a".repeat(70_000)).unwrap_err().contains("longa demais"));
    // O que é legítimo continua valendo: 200 termos e 100 parênteses.
    let row = [("a", Value::Int(1))];
    let sum = vec!["a"; 200].join(" + ");
    assert_eq!(Expr::parse(&sum).unwrap().eval(&get(&row)).unwrap(), Value::Int(200));
    assert_eq!(Expr::parse(&deep("(", "a", ")", 100)).unwrap().eval(&get(&row)).unwrap(), Value::Int(1));
}

#[test]
fn masks() {
    let cpf = Mask::parse(mask::CPF).unwrap();
    assert_eq!(cpf.apply("12345678901"), "123.456.789-01");
    assert_eq!(cpf.apply("1234"), "123.4", "os literais aparecem conforme digita");
    assert_eq!(cpf.stored("123.456.789-01"), "12345678901", ";0 grava sem literais");
    assert!(cpf.is_complete("123.456.789-01") && !cpf.is_complete("123.456"));
    let phone = Mask::parse(mask::PHONE).unwrap();
    assert_eq!(phone.apply("11987654321"), "(11) 98765-4321");
    let plate = Mask::parse(">LLL-0A00;1").unwrap();
    assert_eq!(plate.stored("abc1d23"), "ABC-1D23", ";1 grava com os literais; > deixa maiúsculo");
}

fn setup() -> Ui {
    let mut ui = Ui::new();
    register(&mut ui);
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    ui
}

/// Clientes (id, nome, cidade, limite) com 5 registros.
fn customers(ui: &mut Ui) -> ComponentId {
    let cds = ui.add_root(ClientDataSet::new().with_key("id"));
    let _ = ui.set_name(cds, "cdsClientes");
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("nome", FieldType::String), cds);
    ui.add(Field::new("cidade", FieldType::String), cds);
    ui.add(Field::new("limite", FieldType::Currency), cds);
    let rows = [(1, "Maria", "São Paulo", 1500), (2, "João", "Rio", 300), (3, "Ana", "São Paulo", 9000), (4, "Carlos", "Recife", 50), (5, "Bia", "Rio", 2500)];
    data_set(ui, cds)
        .load_rows(rows.iter().map(|(id, n, c, l)| vec![Value::Int(*id), (*n).into(), (*c).into(), Value::Int(*l)]).collect())
        .unwrap();
    cds
}

fn names(ui: &mut Ui, cds: ComponentId) -> Vec<String> {
    let v = view(ui, cds);
    (0..v.record_count()).map(|i| v.value_at(i, "nome").unwrap().to_string()).collect()
}

#[test]
fn filter_and_index() {
    let mut ui = setup();
    let cds = customers(&mut ui);
    let mut ds = data_set(&mut ui, cds);
    ds.set_filter("limite >= 1000").unwrap();
    assert_eq!(ds.record_count(), 3);
    ds.set_index("cidade;limite desc").unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Ana", "Maria"], "Rio, depois São Paulo pelo maior limite");

    // editar um registro para fora do filtro: ele some da visão mas continua no delta
    let mut ds = data_set(&mut ui, cds);
    ds.first().unwrap();
    ds.edit().unwrap();
    ds.set_text("limite", "10").unwrap();
    ds.post().unwrap();
    assert_eq!(names(&mut ui, cds), ["Ana", "Maria"]);
    assert_eq!(data_set(&mut ui, cds).delta().unwrap().len(), 1);

    // desligar o filtro pela propriedade (como no Object Inspector)
    ui.set_prop(cds, "filtered", false).unwrap();
    ui.process_posted();
    assert_eq!(view(&ui, cds).record_count(), 5);
    // filtro em Rust somado ao índice
    data_set(&mut ui, cds).on_filter_record(|get| get("nome").to_string().len() <= 3).unwrap();
    assert_eq!(names(&mut ui, cds), ["Bia", "Ana"]);
    // filtro inválido: erro, sem perder os dados
    ui.set_prop(cds, "filter", "cidade = ").unwrap();
    ui.set_prop(cds, "filtered", true).unwrap();
    assert!(data_set(&mut ui, cds).refilter().is_err());
}

#[test]
fn calculated_and_lookup_fields() {
    let mut ui = setup();
    let customers = customers(&mut ui);
    let items = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.add(Field::new("id", FieldType::Integer), items);
    ui.add(Field::new("cliente_id", FieldType::Integer), items);
    ui.add(Field::new("qtd", FieldType::Integer), items);
    ui.add(Field::new("preco", FieldType::Currency), items);
    ui.add(Field::new("total", FieldType::Currency).calculated("qtd * preco").format("#,##0.00"), items);
    ui.add(Field::new("cliente", FieldType::String).lookup(customers, "cliente_id", "id", "nome"), items);
    ui.add(Field::new("faixa", FieldType::String).on_calculate(|get| if get("total").as_f64().unwrap_or(0.0) > 100.0 { "alta".into() } else { "baixa".into() }), items);
    let mut ds = data_set(&mut ui, items);
    // só os campos de dados (calculados e lookup ficam de fora)
    ds.load_rows(vec![
        vec![Value::Int(1), Value::Int(3), Value::Int(2), Value::Decimal(Decimal::new(1999, 2))],
        vec![Value::Int(2), Value::Int(1), Value::Int(10), Value::Decimal(Decimal::new(2500, 2))],
    ])
    .unwrap();
    assert_eq!(ds.text("total").unwrap(), "39,98");
    assert_eq!(ds.text("cliente").unwrap(), "Ana");
    assert_eq!(ds.text("faixa").unwrap(), "baixa");
    ds.next().unwrap();
    assert_eq!(ds.text("cliente").unwrap(), "Maria");
    assert_eq!(ds.text("faixa").unwrap(), "alta");
    // calculado recalcula durante a edição; não aceita escrita; não vai no delta
    ds.edit().unwrap();
    ds.set_value("qtd", 1).unwrap();
    assert_eq!(ds.text("total").unwrap(), "25,00");
    assert!(ds.set_value("total", 5).is_err());
    ds.post().unwrap();
    let delta = data_set(&mut ui, items).delta().unwrap();
    assert_eq!(delta[0].new, vec![("qtd".to_string(), Value::Int(1))]);
    // ordenar por campo calculado
    data_set(&mut ui, items).set_index("total desc").unwrap();
    assert_eq!(view(&ui, items).value_at(0, "id").unwrap(), Value::Int(1));
}

#[test]
fn master_detail() {
    let mut ui = setup();
    let master = customers(&mut ui);
    let ds_master = ui.add_root(DataSource::new(master));
    let orders = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.add(Field::new("id", FieldType::Integer), orders);
    ui.add(Field::new("cliente_id", FieldType::Integer), orders);
    ui.add(Field::new("valor", FieldType::Currency), orders);
    ui.set_prop(orders, "master_source", ds_master).unwrap();
    ui.set_prop(orders, "master_fields", "cliente_id=id").unwrap();
    let changes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let c = changes.clone();
    ui.on(orders, "on_master_change", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            c.borrow_mut().push(t.clone());
        }
    });
    data_set(&mut ui, orders)
        .load_rows(vec![
            vec![Value::Int(10), Value::Int(1), Value::Int(100)],
            vec![Value::Int(11), Value::Int(1), Value::Int(50)],
            vec![Value::Int(12), Value::Int(3), Value::Int(70)],
        ])
        .unwrap();
    assert_eq!(view(&ui, orders).record_count(), 2, "pedidos da Maria (id 1)");
    data_set(&mut ui, master).locate("id", 3).unwrap();
    assert_eq!(view(&ui, orders).record_count(), 1, "pedidos da Ana");
    assert_eq!(changes.borrow().last().map(String::as_str), Some(r#"{"cliente_id":3}"#));
    // incluir no detalhe já preenche a chave do mestre
    let mut ds = data_set(&mut ui, orders);
    ds.append().unwrap();
    ds.set_value("id", 13).unwrap();
    ds.set_value("valor", 20).unwrap();
    ds.post().unwrap();
    assert_eq!(ds.value("cliente_id").unwrap(), Value::Int(3));
    assert_eq!(ds.record_count(), 2);
    // mestre sem pedidos
    data_set(&mut ui, master).locate("id", 4).unwrap();
    assert_eq!(view(&ui, orders).record_count(), 0);
    // o delta tem o pedido novo mesmo escondido
    assert_eq!(data_set(&mut ui, orders).delta().unwrap().len(), 1);
}

#[test]
fn masked_field_stores_raw_and_shows_formatted() {
    let mut ui = setup();
    let cds = ui.add_root(ClientDataSet::new());
    ui.add(Field::new("cpf", FieldType::String).mask(mask::CPF).label("CPF"), cds);
    let mut ds = data_set(&mut ui, cds);
    ds.open().unwrap();
    ds.append().unwrap();
    assert!(ds.set_text("cpf", "123.456").is_err(), "incompleto");
    ds.set_text("cpf", "123.456.789-01").unwrap();
    assert_eq!(ds.value("cpf").unwrap(), Value::from("12345678901"));
    assert_eq!(ds.text("cpf").unwrap(), "123.456.789-01");
}

#[test]
fn columnar_storage() {
    let mut ui = setup();
    let cds = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.add(Field::new("id", FieldType::Integer), cds);
    ui.add(Field::new("cidade", FieldType::String), cds);
    ui.add(Field::new("valor", FieldType::Currency), cds);
    ui.add(Field::new("obs", FieldType::String), cds);
    let cities = ["São Paulo", "Rio de Janeiro", "Belo Horizonte", "Curitiba"];
    let n = 100_000;
    let rows: Vec<Vec<Value>> = (0..n)
        .map(|i| vec![Value::Int(i as i64), cities[i % 4].into(), Value::Decimal(Decimal::new(i as i128, 2)), if i % 10 == 0 { "nota".into() } else { Value::Null }])
        .collect();
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    let bytes = ui.get::<ClientDataSet>(cds).unwrap().storage_bytes();
    // Em linhas seriam 4 Values por registro (+ o texto de cada cidade); em colunas, a
    // cidade é um número de 4 bytes.
    let as_rows = n * 4 * std::mem::size_of::<Value>() + n * 24;
    assert!(bytes * 2 < as_rows, "colunas: {bytes} bytes; linhas: ~{as_rows}");
    let v = view(&ui, cds);
    assert_eq!(v.value_at(99_999, "cidade").unwrap(), Value::from("Curitiba"));
    assert_eq!(v.value_at(10, "obs").unwrap(), Value::from("nota"));
    assert_eq!(v.value_at(11, "obs").unwrap(), Value::Null);
    // editar não mexe nas colunas: o registro passa a ter os próprios valores
    let mut ds = data_set(&mut ui, cds);
    ds.move_to(5).unwrap();
    ds.edit().unwrap();
    ds.set_value("cidade", "Recife").unwrap();
    ds.post().unwrap();
    assert_eq!(view(&ui, cds).value_at(5, "cidade").unwrap(), Value::from("Recife"));
    assert_eq!(view(&ui, cds).value_at(9, "cidade").unwrap(), Value::from("Rio de Janeiro"));
    let delta = data_set(&mut ui, cds).delta().unwrap();
    assert_eq!(delta.len(), 1);
    assert_eq!(delta[0].old, vec![("cidade".to_string(), Value::from("Rio de Janeiro"))]);
    // filtro e índice sobre as colunas
    let mut ds = data_set(&mut ui, cds);
    ds.set_filter("cidade = 'curitiba'").unwrap();
    assert_eq!(ds.record_count(), n / 4);
    ds.set_index("valor desc").unwrap();
    ds.first().unwrap();
    assert_eq!(ds.value("id").unwrap(), Value::Int(99_999));
}
