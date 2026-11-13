//! O `OleObject` com um programa falso no lugar do serviço do Windows: os caminhos do VBA, a
//! lista de programas permitidos, a matriz numa chamada só e largar os objetos.

use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::platform::{OleReleaser, OleService, OleValue, allow_ole, ole_allowed, ole_date, ole_date_parts};
use aylis_core::*;
use aylis_data::*;
use aylis_providers::ole::{Segment, parse_path};

/// Um "Excel" de mentira: cada nome que termina em objeto devolve um objeto novo, e o resto
/// fica anotado (o que o teste confere).
#[derive(Default)]
struct Log {
    calls: Vec<String>,
    released: Vec<u64>,
    put: Option<OleValue>,
    next: u64,
}

struct FakeOle(Arc<Mutex<Log>>);

const OBJECTS: &[&str] = &["Workbooks", "Add", "ActiveSheet", "Range", "Cells", "Resize"];

impl OleService for FakeOle {
    fn create(&mut self, prog_id: &str, running: bool) -> Result<u64, String> {
        let mut log = self.0.lock().unwrap();
        log.calls.push(format!("create {prog_id} {running}"));
        log.next += 1;
        Ok(log.next)
    }
    fn invoke(&mut self, object: u64, name: &str, args: Vec<OleValue>) -> Result<OleValue, String> {
        let mut log = self.0.lock().unwrap();
        log.calls.push(format!("{object}.{name}{args:?}"));
        if OBJECTS.contains(&name) {
            log.next += 1;
            return Ok(OleValue::Object(log.next));
        }
        match name {
            "Value" => Ok(OleValue::Array(vec![vec![OleValue::Int(1), OleValue::String("a".into())], vec![OleValue::Date(ole_date(2026, 11, 12, 8, 30, 0)), OleValue::Null]])),
            "Count" => Ok(OleValue::Int(3)),
            "Name" => Ok(OleValue::String("Plan1".into())),
            _ => Err(format!("o objeto não tem '{name}'")),
        }
    }
    fn put(&mut self, object: u64, name: &str, args: Vec<OleValue>, value: OleValue) -> Result<(), String> {
        let mut log = self.0.lock().unwrap();
        log.calls.push(format!("{object}.{name}{args:?} = put"));
        log.put = Some(value);
        Ok(())
    }
    fn release(&mut self, object: u64) {
        self.0.lock().unwrap().released.push(object);
    }
    fn releaser(&self) -> Option<OleReleaser> {
        let log = self.0.clone();
        Some(Arc::new(move |o| log.lock().unwrap().released.push(o)))
    }
}

fn setup() -> (Ui, ComponentId, ComponentId, Arc<Mutex<Log>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_providers::register(&mut ui);
    let log = Arc::new(Mutex::new(Log::default()));
    ui.services_mut().ole = Some(Box::new(FakeOle(log.clone())));
    let form = ui.add_root(Form::new("x", 400.0, 300.0));
    let ole = ui.create("OleObject", Some(form)).unwrap();
    (ui, form, ole, log)
}

#[test]
fn paths_read_like_vba() {
    let p = parse_path(r#"Workbooks(1).Worksheets("Dados ""2026"".x").Range("A1:C3").Value"#).unwrap();
    assert_eq!(
        p,
        vec![
            Segment { name: "Workbooks".into(), args: vec![OleValue::Int(1)] },
            Segment { name: "Worksheets".into(), args: vec![OleValue::String("Dados \"2026\".x".into())] },
            Segment { name: "Range".into(), args: vec![OleValue::String("A1:C3".into())] },
            Segment { name: "Value".into(), args: vec![] },
        ]
    );
    let p = parse_path("Cells(2, -3.5).Locked( true ).Add()").unwrap();
    assert_eq!(p[0].args, vec![OleValue::Int(2), OleValue::Float(-3.5)]);
    assert_eq!(p[1].args, vec![OleValue::Bool(true)]);
    assert!(p[2].args.is_empty());
    // Só endereço: nada de contas, variáveis ou outro código.
    for bad in ["", "Range(A1)", "Range(\"A1\"", "Range(\"A1\") + 1", "Value; Quit", "1abc", "Range(\"A1\")..Value"] {
        assert!(parse_path(bad).is_err(), "aceitou {bad:?}");
    }
}

#[test]
fn only_the_allowed_programs_open() {
    assert!(ole_allowed("Excel.Application") && ole_allowed("excel.application.16"));
    assert!(!ole_allowed("WScript.Shell") && !ole_allowed("Shell.Application"));
    let (mut ui, _, ole, log) = setup();
    ui.set_prop(ole, "prog_id", "WScript.Shell").unwrap();
    let e = ui.invoke(ole, "open", &[]).unwrap_err();
    assert!(e.contains("WScript.Shell"), "{e}");
    assert!(log.lock().unwrap().calls.is_empty(), "nem chegou ao serviço");
    // O código do app (não uma propriedade) permite outro programa.
    allow_ole("AylisTeste.Application");
    ui.set_prop(ole, "prog_id", "AylisTeste.Application").unwrap();
    ui.set_prop(ole, "use_running", true).unwrap();
    assert_eq!(ui.invoke(ole, "open", &[]).unwrap(), PropValue::Bool(true));
    assert_eq!(ui.get_prop(ole, "opened").unwrap(), PropValue::Bool(true));
    assert_eq!(log.lock().unwrap().calls, vec!["create AylisTeste.Application true"]);
}

#[test]
fn get_set_and_call_walk_the_path_and_release_what_they_took() {
    let (mut ui, _, ole, log) = setup();
    // `get` abre sozinho; o objeto do meio é largado no fim.
    assert_eq!(ui.invoke(ole, "get_value", &["ActiveSheet.Name".into()]).unwrap(), PropValue::String("Plan1".into()));
    {
        let l = log.lock().unwrap();
        assert_eq!(l.calls, vec!["create Excel.Application false", "1.ActiveSheet[]", "2.Name[]"]);
        assert_eq!(l.released, vec![2]);
    }
    assert_eq!(ui.invoke(ole, "get_number", &["Workbooks.Count".into()]).unwrap(), PropValue::Float(3.0));
    // A matriz em JSON, com a data em ISO.
    let json = ui.invoke(ole, "get_values", &[r#"ActiveSheet.Range("A1:B2").Value"#.into()]).unwrap();
    assert_eq!(json, PropValue::String(r#"[[1,"a"],["2026-11-12 08:30:00",null]]"#.into()));
    // Gravar: o texto vai como texto, o número como número.
    ui.invoke(ole, "set_value", &[r#"ActiveSheet.Range("A1").Value"#.into(), "Olá".into()]).unwrap();
    assert_eq!(log.lock().unwrap().put, Some(OleValue::String("Olá".into())));
    ui.invoke(ole, "set_value", &["Visible".into(), "True".into()]).unwrap();
    assert_eq!(log.lock().unwrap().put, Some(OleValue::Bool(true)), "o booleano não depende do idioma");
    ui.invoke(ole, "set_value", &[r#"ActiveSheet.Range("A2").Value"#.into(), PropValue::Float(2.5)]).unwrap();
    assert_eq!(log.lock().unwrap().put, Some(OleValue::Float(2.5)));
    // Uma matriz inteira numa chamada; a linha curta completa com vazio.
    ui.invoke(ole, "set_values", &[r#"ActiveSheet.Range("A1:B2").Value"#.into(), r#"[[1, "x"], [true]]"#.into()]).unwrap();
    assert_eq!(log.lock().unwrap().put, Some(OleValue::Array(vec![vec![OleValue::Int(1), OleValue::String("x".into())], vec![OleValue::Bool(true), OleValue::Null]])));
    // O erro do programa volta como erro do método.
    let e = ui.invoke(ole, "call", &["ActiveSheet.Explode".into()]).unwrap_err();
    assert!(e.contains("Explode"), "{e}");
    // `close` larga o objeto do programa.
    ui.invoke(ole, "release", &[]).unwrap();
    assert!(log.lock().unwrap().released.contains(&1));
    assert_eq!(ui.get_prop(ole, "opened").unwrap(), PropValue::Bool(false));
}

#[test]
fn put_data_sends_the_whole_dataset_in_one_call() {
    let (mut ui, form, ole, log) = setup();
    let cds = ui.add(ClientDataSet::new(), form);
    ui.add(Field::new("nome", FieldType::String).label("Nome"), cds);
    ui.add(Field::new("preco", FieldType::Currency).label("Preço"), cds);
    ui.add(Field::new("validade", FieldType::Date).label("Validade"), cds);
    let rows = (0..500).map(|i| vec![format!("Item {i}").into(), Decimal::new(1590 + i, 2).into(), Value::Date(Date::new(2027, 1, 31).unwrap())]).collect();
    data_set(&mut ui, cds).load_rows(rows).unwrap();
    let ds = ui.add(DataSource::new(cds), form);
    let n = ui.invoke(ole, "put_data", &[r#"ActiveSheet.Range("B2")"#.into(), PropValue::Component(Some(ds))]).unwrap();
    assert_eq!(n, PropValue::Int(500));
    let l = log.lock().unwrap();
    let puts: Vec<_> = l.calls.iter().filter(|c| c.ends_with("= put")).collect();
    assert_eq!(puts.len(), 1, "uma chamada só, não uma por célula: {:?}", l.calls);
    assert!(l.calls.iter().any(|c| c.ends_with("Resize[Int(501), Int(3)]")), "{:?}", l.calls);
    let Some(OleValue::Array(m)) = &l.put else { panic!("sem matriz") };
    assert_eq!(m.len(), 501);
    assert_eq!(m[0], vec![OleValue::String("Nome".into()), OleValue::String("Preço".into()), OleValue::String("Validade".into())]);
    assert_eq!(m[1][1], OleValue::Float(15.9));
    let OleValue::Date(d) = m[1][2] else { panic!("a data vai como data: {:?}", m[1][2]) };
    assert_eq!(ole_date_parts(d), (2027, 1, 31, 0, 0, 0));
}

#[test]
fn freeing_the_component_releases_the_program() {
    let (mut ui, _, ole, log) = setup();
    ui.invoke(ole, "open", &[]).unwrap();
    ui.free(ole);
    assert_eq!(log.lock().unwrap().released, vec![1]);
}

#[test]
fn without_the_service_it_says_why() {
    let mut ui = Ui::new();
    aylis_providers::register(&mut ui);
    let form = ui.add_root(Form::new("x", 100.0, 100.0));
    let ole = ui.create("OleObject", Some(form)).unwrap();
    let e = ui.invoke(ole, "open", &[]).unwrap_err();
    assert!(e.contains("Windows"), "{e}");
}

#[test]
fn ole_dates_match_excel() {
    // O Excel: 1 = 31/12/1899, 45.000 = 15/03/2023, e a hora na fração.
    assert_eq!(ole_date(1899, 12, 30, 0, 0, 0), 0.0);
    assert_eq!(ole_date(2023, 3, 15, 0, 0, 0), 45_000.0);
    assert_eq!(ole_date_parts(45_000.75), (2023, 3, 15, 18, 0, 0));
    assert_eq!(ole_date_parts(ole_date(1900, 3, 1, 23, 59, 59)), (1900, 3, 1, 23, 59, 59));
}
