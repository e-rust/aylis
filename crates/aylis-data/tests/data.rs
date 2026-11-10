use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::platform::LocaleInfo;
use aylis_core::{ComponentId, Ui};
use aylis_data::*;

/// Clientes: id (autoinc), nome (obrigatório), email, nascimento (data), limite (moeda),
/// ativo (lógico).
fn setup() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    ui.services_mut().locale = Some(Box::new(LocaleInfo::pt_br()));
    let cds = ui.add_root(ClientDataSet::new().with_key("id"));
    ui.set_name(cds, "cdsClientes").unwrap();
    ui.add(Field::new("id", FieldType::AutoInc), cds);
    ui.add(Field::new("nome", FieldType::String).label("Nome").size(20).required(), cds);
    ui.add(Field::new("email", FieldType::String), cds);
    ui.add(Field::new("nascimento", FieldType::Date).label("Nascimento"), cds);
    ui.add(Field::new("limite", FieldType::Currency).label("Limite").format("R$ #,##0.00").min(0.0), cds);
    ui.add(Field::new("ativo", FieldType::Boolean).default_value("true"), cds);
    let rows = vec![
        row(1, "Ana", "ana@x.com", (1990, 4, 12), "1500.5", true),
        row(2, "Bruno", "bruno@x.com", (1985, 1, 30), "0", false),
        row(3, "Carla", "", (2000, 12, 31), "12345678.9", true),
    ];
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    (ui, cds)
}

fn row(id: i64, name: &str, email: &str, (y, m, d): (i32, u8, u8), limit: &str, active: bool) -> Vec<Value> {
    vec![
        id.into(),
        name.into(),
        if email.is_empty() { Value::Null } else { email.into() },
        Date::new(y, m, d).unwrap().into(),
        Decimal::parse(limit).unwrap().into(),
        active.into(),
    ]
}

#[test]
fn navigation_follows_delphi_bof_eof() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    let mut names = Vec::new();
    ds.first().unwrap();
    while !ds.eof() {
        names.push(ds.text("nome").unwrap());
        ds.next().unwrap();
    }
    assert_eq!(names, ["Ana", "Bruno", "Carla"], "while !eof passa por todos");

    ds.last().unwrap();
    assert!(ds.eof() && !ds.bof());
    ds.prior().unwrap();
    assert_eq!(ds.text("nome").unwrap(), "Bruno");
    assert!(!ds.eof());
    assert!(ds.locate("nome", "Carla").unwrap());
    assert_eq!(ds.rec_no(), Some(2));
    assert!(!ds.locate("nome", "Zé").unwrap());
}

#[test]
fn pt_br_display_and_edit_texts() {
    let (mut ui, cds) = setup();
    let ds = data_set(&mut ui, cds);
    assert_eq!(ds.text("limite").unwrap(), "R$ 1.500,50");
    assert_eq!(ds.view().edit_text("limite").unwrap(), "1500,50");
    assert_eq!(ds.text("nascimento").unwrap(), "12/04/1990");
    assert_eq!(ds.text("ativo").unwrap(), "Sim");
    assert_eq!(ds.text("email").unwrap(), "ana@x.com");
    let mut ds = ds;
    ds.last().unwrap();
    assert_eq!(ds.text("limite").unwrap(), "R$ 12.345.678,90");
    assert_eq!(ds.text("email").unwrap(), "", "nulo vira texto vazio");
}

#[test]
fn edit_and_post_records_original_values_in_the_delta() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    assert_eq!(ds.state(), DataSetState::Edit);
    ds.set_text("limite", "2.000,00").unwrap();
    ds.set_text("nascimento", "1/5/1991").unwrap();
    ds.post().unwrap();
    assert_eq!(ds.state(), DataSetState::Browse);
    assert_eq!(ds.text("limite").unwrap(), "R$ 2.000,00");
    assert_eq!(ds.change_count(), 1);

    let delta = ds.delta().unwrap();
    assert_eq!(delta.len(), 1);
    let change = &delta[0];
    assert_eq!(change.kind, ChangeKind::Update);
    assert_eq!(change.key, vec![("id".to_string(), Value::Int(1))]);
    assert_eq!(change.old.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>(), ["nascimento", "limite"]);
    assert_eq!(change.new[1].1, Value::Decimal(Decimal::new(200000, 2)));

    // editar de volta ao valor original: nada a enviar
    ds.edit().unwrap();
    ds.set_text("limite", "1500,50").unwrap();
    ds.set_text("nascimento", "12/04/1990").unwrap();
    ds.post().unwrap();
    assert!(ds.delta().unwrap().is_empty());
}

#[test]
fn insert_append_cancel_and_temporary_keys() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    ds.append().unwrap();
    assert_eq!(ds.state(), DataSetState::Insert);
    assert_eq!(ds.value("id").unwrap(), Value::Int(-1), "chave provisória negativa");
    assert_eq!(ds.value("ativo").unwrap(), Value::Bool(true), "default_value");
    ds.cancel().unwrap();
    assert_eq!(ds.record_count(), 3);

    ds.first().unwrap();
    ds.insert().unwrap();
    ds.set_text("nome", "Daniel").unwrap();
    ds.post().unwrap();
    assert_eq!(ds.record_count(), 4);
    assert_eq!(ds.rec_no(), Some(0), "insert entra na posição atual");
    assert_eq!(ds.value("id").unwrap(), Value::Int(-2));
    let delta = ds.delta().unwrap();
    assert_eq!(delta[0].kind, ChangeKind::Insert);
    assert!(delta[0].new.iter().any(|(k, v)| k == "nome" && *v == Value::from("Daniel")));
}

#[test]
fn required_size_min_and_types_are_validated() {
    let (mut ui, cds) = setup();
    let errors = Rc::new(RefCell::new(Vec::new()));
    let e = errors.clone();
    ui.on(cds, "on_error", move |_, _, args| {
        if let aylis_core::EventArgs::Text(t) = args {
            e.borrow_mut().push(t.clone());
        }
    });
    let mut ds = data_set(&mut ui, cds);
    ds.append().unwrap();
    let err = ds.post().unwrap_err();
    assert_eq!(err.message, "Nome é obrigatório");
    assert_eq!(err.field.as_deref(), Some("nome"));
    assert_eq!(ds.state(), DataSetState::Insert, "continua em inclusão");

    assert!(ds.set_text("nome", "um nome com mais de vinte letras").unwrap_err().message.contains("20"));
    assert!(ds.set_text("limite", "-5").unwrap_err().message.contains("mínimo"));
    assert!(ds.set_text("limite", "abc").unwrap_err().message.contains("não é um valor válido"));
    assert!(ds.set_text("nascimento", "31/02/2020").unwrap_err().message.contains("data válida"));
    assert!(ds.set_value("id", 5).unwrap_err().message.contains("somente leitura"));
    assert!(ds.set_value("ativo", "talvez").unwrap_err().message.contains("não aceita"));
    assert_eq!(errors.borrow().len(), 7, "cada erro também dispara on_error");
}

#[test]
fn delete_and_navigation_auto_post() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_text("email", "nova@x.com").unwrap();
    ds.next().unwrap(); // grava antes de navegar, como na VCL
    assert_eq!(ds.state(), DataSetState::Browse);
    assert_eq!(ds.change_count(), 1);

    ds.delete().unwrap();
    assert_eq!(ds.record_count(), 2);
    assert_eq!(ds.text("nome").unwrap(), "Carla");
    let delta = ds.delta().unwrap();
    assert_eq!(delta.iter().map(|c| c.kind).collect::<Vec<_>>(), [ChangeKind::Update, ChangeKind::Delete]);
    assert_eq!(delta[1].key, vec![("id".to_string(), Value::Int(2))]);

    // excluir um registro incluído e ainda não enviado não vai para o delta
    ds.append().unwrap();
    ds.set_text("nome", "Temp").unwrap();
    ds.post().unwrap();
    ds.delete().unwrap();
    assert_eq!(ds.delta().unwrap().len(), 2);
}

#[test]
fn apply_updates_and_reconcile_with_a_rust_backend() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    let received = Rc::new(RefCell::new(Vec::new()));
    let r = received.clone();
    ds.on_apply(move |changes| {
        r.borrow_mut().extend(changes.iter().cloned());
        changes
            .iter()
            .map(|c| match c.kind {
                ChangeKind::Insert => ApplyResult::ok(c.reference).with_value("id", 1043),
                ChangeKind::Delete => ApplyResult::error(c.reference, "Cliente possui pedidos"),
                ChangeKind::Update => ApplyResult::ok(c.reference),
            })
            .collect()
    });
    ds.edit().unwrap();
    ds.set_text("email", "x@y.com").unwrap();
    ds.post().unwrap();
    ds.append().unwrap();
    ds.set_text("nome", "Novo").unwrap();
    ds.post().unwrap();
    ds.first().unwrap();
    ds.next().unwrap();
    ds.delete().unwrap();

    let err = ds.apply_updates().unwrap_err();
    assert_eq!(err.message, "Cliente possui pedidos");
    assert_eq!(received.borrow().len(), 3);
    assert_eq!(ds.change_count(), 1, "só a exclusão recusada continua pendente");
    ds.last().unwrap();
    assert_eq!(ds.value("id").unwrap(), Value::Int(1043), "id definitivo veio do backend");
}

#[test]
fn delta_json_and_reconcile_json_for_other_languages() {
    let (mut ui, cds) = setup();
    let sent = Rc::new(RefCell::new(String::new()));
    let s = sent.clone();
    ui.on(cds, "on_apply_updates", move |_, _, args| {
        if let aylis_core::EventArgs::Text(json) = args {
            *s.borrow_mut() = json.clone();
        }
    });
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_text("limite", "99,90").unwrap();
    ds.post().unwrap();
    ds.apply_updates().unwrap();

    let json: serde_json::Value = serde_json::from_str(&sent.borrow()).unwrap();
    assert_eq!(json["data_set"], "cdsClientes");
    let change = &json["changes"][0];
    assert_eq!(change["op"], "update");
    assert_eq!(change["key"]["id"], 1);
    assert_eq!(change["old"]["limite"], "1500.50");
    assert_eq!(change["new"]["limite"], "99.90");
    assert_eq!(ds.change_count(), 1, "aguardando a resposta do backend");

    let reference = change["ref"].as_u64().unwrap();
    ds.reconcile_json(&format!(r#"{{ "results": [ {{ "ref": {reference} }} ] }}"#)).unwrap();
    assert_eq!(ds.change_count(), 0);
}

#[test]
fn load_json_and_cancel_updates() {
    let (mut ui, cds) = setup();
    let mut ds = data_set(&mut ui, cds);
    let rows = serde_json::json!([
        { "id": 10, "nome": "Zeca", "nascimento": "1970-07-07", "limite": "10.5", "ativo": false },
        { "id": 11, "nome": "Yara" }
    ]);
    ds.load_json(&rows).unwrap();
    assert_eq!(ds.record_count(), 2);
    assert_eq!(ds.text("limite").unwrap(), "R$ 10,50");
    ds.next().unwrap();
    assert_eq!(ds.value("limite").unwrap(), Value::Null);

    ds.edit().unwrap();
    ds.set_text("nome", "Yasmin").unwrap();
    ds.post().unwrap();
    ds.append().unwrap();
    ds.set_text("nome", "W").unwrap();
    ds.post().unwrap();
    ds.first().unwrap();
    ds.delete().unwrap();
    assert_eq!(ds.change_count(), 3);
    ds.cancel_updates().unwrap();
    assert_eq!(ds.change_count(), 0);
    assert_eq!(ds.record_count(), 2);
    let names: Vec<String> = (0..2).map(|i| { ds.move_to(i).unwrap(); ds.text("nome").unwrap() }).collect();
    assert!(names.contains(&"Yara".to_string()) && names.contains(&"Zeca".to_string()));
}

#[test]
fn field_events_get_text_set_text_and_validate() {
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    ui.add(
        Field::new("status", FieldType::Integer)
            .on_get_text(|v, _| match v.as_i64() {
                Some(1) => "Aberto".into(),
                Some(2) => "Faturado".into(),
                _ => String::new(),
            })
            .on_set_text(|t| match t {
                "Aberto" => Ok(Value::Int(1)),
                "Faturado" => Ok(Value::Int(2)),
                _ => Err(format!("status desconhecido: {t}")),
            })
            .on_validate(|v| if v.as_i64() == Some(2) { Err("não pode faturar aqui".into()) } else { Ok(()) }),
        cds,
    );
    let mut ds = data_set(&mut ui, cds);
    ds.load_rows(vec![vec![1.into()]]).unwrap();
    assert_eq!(ds.text("status").unwrap(), "Aberto");
    ds.edit().unwrap();
    assert!(ds.set_text("status", "Faturado").unwrap_err().message.contains("não pode faturar"));
    assert!(ds.set_text("status", "Outro").unwrap_err().message.contains("desconhecido"));
}

#[test]
fn hooks_before_post_on_new_record_and_after_scroll() {
    let (mut ui, cds) = setup();
    let scrolls = Rc::new(RefCell::new(0));
    let s = scrolls.clone();
    let mut ds = data_set(&mut ui, cds);
    ds.on_new_record(|ui, cds| data_set(ui, cds).set_text("email", "@novo").map_err(|e| e.message))
        .on_before_post(|ui, cds| {
            let email = data_set(ui, cds).text("email").unwrap_or_default();
            if email.contains('@') { Ok(()) } else { Err("e-mail inválido".into()) }
        })
        .on_after_scroll(move |_, _| {
            *s.borrow_mut() += 1;
            Ok(())
        });
    ds.next().unwrap();
    ds.next().unwrap();
    assert_eq!(*scrolls.borrow(), 2);

    ds.append().unwrap();
    assert_eq!(ds.text("email").unwrap(), "@novo");
    ds.set_text("nome", "Teste").unwrap();
    ds.set_text("email", "sem arroba").unwrap();
    assert_eq!(ds.post().unwrap_err().message, "e-mail inválido");
    ds.set_text("email", "ok@ok").unwrap();
    ds.post().unwrap();
}

#[test]
fn number_and_date_formats() {
    use aylis_data::format::*;
    let br = LocaleInfo::pt_br();
    let us = LocaleInfo::en_us();
    let d = |s: &str| Decimal::parse(s).unwrap();
    assert_eq!(format_number(d("1234567.891"), "#,##0.00", &br), "1.234.567,89");
    assert_eq!(format_number(d("1234567.891"), "#,##0.00", &us), "1,234,567.89");
    assert_eq!(format_number(d("0.5"), "0.0#", &br), "0,5");
    assert_eq!(format_number(d("-3.456"), "R$ #,##0.00", &br), "-R$ 3,46");
    assert_eq!(format_number(d("-3"), "0.00;(0.00)", &br), "(3,00)");
    assert_eq!(format_number(d("7"), "000", &br), "007");
    assert_eq!(format_number(d("12.5"), "0.0 kg", &br), "12,5 kg");
    assert_eq!(parse_number("R$ 1.234,56", &br), Some(d("1234.56")));
    assert_eq!(parse_number("1,234.56", &us), Some(d("1234.56")));

    let date = Date::new(2024, 3, 5).unwrap();
    let time = Time::new(9, 7, 3).unwrap();
    assert_eq!(format_date_time(Some(date), Some(time), "dd/mm/yyyy hh:nn:ss"), "05/03/2024 09:07:03");
    assert_eq!(format_date_time(Some(date), None, "d/m/yy"), "5/3/24");
    assert_eq!(parse_date_time("5-3-2024", "dd/mm/yyyy"), Some((Some(date), None)));
    assert_eq!(parse_date_time("29/02/2023", "dd/mm/yyyy"), None, "2023 não é bissexto");
    assert!(parse_date_time("29/02/2024", "dd/mm/yyyy").is_some());
}

#[test]
fn decimal_is_exact() {
    let a = Decimal::parse("0.1").unwrap();
    assert_eq!(a.rescale(2), Decimal::parse("0.10").unwrap());
    assert_eq!(Decimal::parse("2.345").unwrap().rescale(2).to_string(), "2.35");
    assert_eq!(Decimal::parse("-2.345").unwrap().rescale(2).to_string(), "-2.35");
    assert_eq!(Decimal::parse("1.50").unwrap(), Decimal::parse("1.5").unwrap());
    assert!(Decimal::parse("1.2.3").is_none());
}

#[test]
fn links_receive_messages_through_a_data_source() {
    use aylis_core::{Component, ControlBase, EventCtx, properties};
    use std::any::Any;

    /// Controle de teste que só registra as mensagens que recebe.
    #[derive(Default)]
    struct Probe {
        base: ControlBase,
        data_source: Option<ComponentId>,
        seen: Rc<RefCell<Vec<DataEvent>>>,
    }
    impl Component for Probe {
        fn class_name(&self) -> &'static str {
            "Probe"
        }
        fn control(&self) -> Option<&ControlBase> {
            Some(&self.base)
        }
        fn control_mut(&mut self) -> Option<&mut ControlBase> {
            Some(&mut self.base)
        }
        properties! { data_source: Option<ComponentId> => "Data" }
        fn message(&mut self, _ctx: &mut EventCtx<'_>, message: &dyn Any) {
            if let Some(m) = message.downcast_ref::<DataMessage>() {
                self.seen.borrow_mut().push(m.event.clone());
            }
        }
    }

    let (mut ui, cds) = setup();
    let source = ui.add_root(DataSource::new(cds));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let form = ui.add_root(Probe::default());
    let probe = ui.add(Probe { data_source: Some(source), seen: seen.clone(), ..Default::default() }, form);
    let _ = probe;
    let data_changes = Rc::new(RefCell::new(0));
    let d = data_changes.clone();
    ui.on(source, "on_data_change", move |_, _, _| *d.borrow_mut() += 1);

    let mut ds = data_set(&mut ui, cds);
    ds.next().unwrap();
    ds.edit().unwrap();
    ds.set_text("nome", "Beto").unwrap();
    ds.post().unwrap();
    assert_eq!(
        *seen.borrow(),
        vec![
            DataEvent::Scrolled,
            DataEvent::StateChanged,
            DataEvent::RecordChanged(Some("nome".into())),
            DataEvent::UpdateData,
            DataEvent::StateChanged,
            DataEvent::DataSetChanged,
        ]
    );
    assert_eq!(*data_changes.borrow(), 3, "scroll, campo e post");

    // disable_controls junta tudo num aviso só
    seen.borrow_mut().clear();
    let mut ds = data_set(&mut ui, cds);
    ds.disable_controls();
    ds.first().unwrap();
    ds.next().unwrap();
    ds.next().unwrap();
    ds.enable_controls();
    assert_eq!(*seen.borrow(), vec![DataEvent::DataSetChanged]);
}

#[test]
fn named_get_text_and_set_text_handlers_answer_with_the_helpers() {
    // Como o código gerado pela IDE: handlers por nome (`ui, sender, args`), respondidos
    // com `aylis_data::get_text` e `aylis_data::set_text`.
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    let status = ui.add(Field::new("status", FieldType::Integer), cds);
    ui.register_handler("status_get_text", |ui, sender, args| {
        get_text(ui, sender, args, |v| match v.as_i64() {
            Some(1) => "Aberto".into(),
            Some(2) => "Faturado".into(),
            _ => v.to_string(),
        });
    });
    ui.register_handler("status_set_text", |ui, sender, args| {
        set_text(ui, sender, args, |t| match t {
            "Aberto" => Ok(Value::Int(1)),
            "Faturado" => Ok(Value::Int(2)),
            _ => Err(format!("situação desconhecida: {t}")),
        });
    });
    ui.set_prop(status, "on_get_text", "status_get_text").unwrap();
    ui.set_prop(status, "on_set_text", "status_set_text").unwrap();
    data_set(&mut ui, cds).load_rows(vec![vec![1.into()], vec![2.into()]]).unwrap();

    // A exibição pede os textos em lote (depois da pintura) e o handler responde.
    let v = view(&ui, cds);
    assert_eq!((v.visible_text(0, "status").unwrap(), v.visible_text(1, "status").unwrap()), ("1".into(), "2".into()), "enquanto não chega, o valor");
    ui.process_posted();
    let v = view(&ui, cds);
    assert_eq!((v.visible_text(0, "status").unwrap(), v.visible_text(1, "status").unwrap()), ("Aberto".into(), "Faturado".into()));

    // O texto digitado vai ao handler, que grava o valor ou recusa.
    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_text("status", "Faturado").unwrap();
    assert_eq!(ds.value("status").unwrap(), Value::Int(2));
    ds.set_text("status", "Outro").unwrap();
    drop(ds);
    assert!(view(&ui, cds).field_errors().iter().any(|(f, m)| f == "status" && m.contains("desconhecida")));
    assert!(data_set(&mut ui, cds).post().is_err(), "com erro no campo, o post recusa");
}

#[test]
fn a_post_waits_for_the_remote_on_set_text_answer() {
    // Backend remoto: o handler só guarda o pedido; a resposta chega depois.
    let mut ui = Ui::new();
    let cds = ui.add_root(ClientDataSet::new());
    let status = ui.add(Field::new("status", FieldType::Integer), cds);
    let asked = Rc::new(RefCell::new(Vec::new()));
    let a = asked.clone();
    ui.register_handler("status_set_text", move |_, _, args| a.borrow_mut().push(args.text().unwrap_or_default().to_string()));
    ui.set_prop(status, "on_set_text", "status_set_text").unwrap();
    data_set(&mut ui, cds).load_rows(vec![vec![1.into()]]).unwrap();

    let mut ds = data_set(&mut ui, cds);
    ds.edit().unwrap();
    ds.set_text("status", "Faturado").unwrap();
    assert!(ds.awaiting_text());
    // O post espera: nada é gravado ainda.
    ds.post().unwrap();
    assert!(ds.state().is_editing(), "o post esperou a resposta");
    // A resposta chega: o valor entra e o post adiado grava.
    ds.set_value("status", 2).unwrap();
    assert!(!ds.awaiting_text());
    assert!(!ds.state().is_editing(), "gravou sozinho ao chegar a resposta");
    assert_eq!(ds.value("status").unwrap(), Value::Int(2));
    assert_eq!(asked.borrow().len(), 1);

    // Resposta com erro: o post adiado não grava, e o campo fica com o erro.
    ds.edit().unwrap();
    ds.set_text("status", "Outro").unwrap();
    ds.post().unwrap();
    assert!(ds.set_field_error("status", Some("situação desconhecida".into())).is_err(), "o post adiado falha com o erro");
    assert!(ds.state().is_editing());
    assert!(!ds.awaiting_text());
    // Cancelar limpa a espera.
    ds.set_text("status", "Outro").unwrap();
    ds.cancel().unwrap();
    assert!(!ds.awaiting_text());
}
