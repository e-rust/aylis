//! O `WmiQuery` com um WMI falso no lugar do serviço do Windows: o resultado no
//! `ClientDataSet` (com e sem campos), em JSON, o modo assíncrono, os eventos da assinatura e
//! as recusas (namespace de outra máquina, consulta vazia, sem WMI).

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::platform::{WmiColumn, WmiDone, WmiListener, WmiService, WmiTable, WmiType, WmiValue, WmiWatch, wmi_datetime, wmi_namespace, wmi_select_list};
use aylis_core::*;
use aylis_data::*;

#[derive(Default)]
struct Log {
    queries: Vec<String>,
    listener: Option<WmiListener>,
    stopped: bool,
    /// Guarda o fim em vez de responder na hora (o assíncrono).
    hold: bool,
    held: Vec<WmiDone>,
}

struct FakeWmi(Arc<Mutex<Log>>);

fn disks() -> WmiTable {
    WmiTable {
        columns: vec![
            WmiColumn { name: "Name".into(), kind: WmiType::String },
            WmiColumn { name: "Size".into(), kind: WmiType::Integer },
            WmiColumn { name: "Compressed".into(), kind: WmiType::Boolean },
            WmiColumn { name: "InstallDate".into(), kind: WmiType::DateTime },
            WmiColumn { name: "Flags".into(), kind: WmiType::Array },
        ],
        rows: vec![
            vec![
                WmiValue::String("C:".into()),
                WmiValue::Int(512_000_000_000),
                WmiValue::Bool(false),
                WmiValue::DateTime("2026-11-12T08:30:00".into()),
                WmiValue::Array(vec![WmiValue::Int(1), WmiValue::Int(2)]),
            ],
            vec![WmiValue::String("D:".into()), WmiValue::Int(1_000), WmiValue::Bool(true), WmiValue::Null, WmiValue::Null],
        ],
    }
}

impl WmiService for FakeWmi {
    fn query(&mut self, namespace: &str, wql: &str, done: WmiDone) {
        let mut log = self.0.lock().unwrap();
        log.queries.push(format!("{namespace}|{wql}"));
        if let Err(e) = wmi_namespace(namespace) {
            drop(log);
            return done(Err(e));
        }
        if wql.contains("Erro") {
            drop(log);
            return done(Err("consulta inválida".into()));
        }
        if log.hold {
            log.held.push(done);
            return;
        }
        drop(log);
        done(Ok(disks()));
    }

    fn watch(&mut self, namespace: &str, wql: &str, listener: WmiListener) -> Result<WmiWatch, String> {
        wmi_namespace(namespace)?;
        let mut log = self.0.lock().unwrap();
        log.queries.push(format!("watch {wql}"));
        log.listener = Some(listener);
        log.stopped = false;
        let shared = self.0.clone();
        Ok(WmiWatch::new(move || shared.lock().unwrap().stopped = true))
    }
}

fn setup() -> (Ui, ComponentId, ComponentId, Arc<Mutex<Log>>) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    aylis_data::register(&mut ui);
    aylis_providers::register(&mut ui);
    let log = Arc::new(Mutex::new(Log::default()));
    ui.services_mut().wmi = Some(Box::new(FakeWmi(log.clone())));
    let form = ui.add_root(Form::new("x", 400.0, 300.0));
    let wmi = ui.create("WmiQuery", Some(form)).unwrap();
    (ui, form, wmi, log)
}

#[test]
fn the_cim_helpers() {
    assert_eq!(wmi_datetime("20261112083000.000000-180").as_deref(), Some("2026-11-12T08:30:00"));
    assert_eq!(wmi_datetime("00000001132312.000000:000"), None, "intervalo não é data");
    assert_eq!(wmi_datetime("abc"), None);
    assert_eq!(wmi_namespace("root/cimv2").unwrap(), "root\\cimv2");
    assert_eq!(wmi_namespace("").unwrap(), "root\\cimv2");
    for bad in ["\\\\servidor\\root\\cimv2", "//servidor/root/cimv2", "cimv2", "root\\..\\x", "root\\a b"] {
        assert!(wmi_namespace(bad).is_err(), "aceitou {bad:?}");
    }
    assert_eq!(wmi_select_list("SELECT Name, Size FROM Win32_LogicalDisk"), ["Name", "Size"]);
    assert_eq!(wmi_select_list("select  Caption from Win32_OperatingSystem where x = 1"), ["Caption"]);
    assert!(wmi_select_list("SELECT * FROM Win32_Process").is_empty());
}

/// Sem campos, o dataset ganha um por coluna, no tipo do CIM; a lista vai como JSON.
#[test]
fn execute_fills_a_dataset_without_fields() {
    let (mut ui, form, wmi, log) = setup();
    let cds = ui.create("ClientDataSet", Some(form)).unwrap();
    ui.set_name(cds, "cdsDiscos").unwrap();
    ui.set_prop(wmi, "data_set", PropValue::Component(Some(cds))).unwrap();
    ui.set_prop(wmi, "query", "SELECT Name, Size FROM Win32_LogicalDisk").unwrap();
    assert_eq!(ui.invoke(wmi, "execute", &[]).unwrap(), PropValue::Int(2));
    assert_eq!(log.lock().unwrap().queries, ["root\\cimv2|SELECT Name, Size FROM Win32_LogicalDisk"]);
    let fields: Vec<ComponentId> = ui.children(cds).to_vec();
    assert_eq!(fields.len(), 5);
    assert_eq!(ui.name(fields[0]), Some("cdsDiscosName"));
    let types: Vec<String> = fields.iter().map(|&f| ui.get_prop(f, "type").unwrap().to_str().unwrap().to_string()).collect();
    assert_eq!(types, ["string", "integer", "boolean", "datetime", "memo"]);
    let v = view(&ui, cds);
    assert_eq!(v.record_count(), 2);
    assert_eq!(v.value_at(0, "Size").unwrap(), Value::Int(512_000_000_000));
    assert_eq!(v.value_at(0, "Flags").unwrap(), Value::String("[1,2]".into()));
    assert_eq!(v.value_at(1, "Compressed").unwrap(), Value::Bool(true));
    assert!(matches!(v.value_at(0, "InstallDate").unwrap(), Value::DateTime(_)));
    assert_eq!(ui.get_prop(wmi, "row_count").unwrap(), PropValue::Int(2));
    let json: serde_json::Value = serde_json::from_str(ui.get_prop(wmi, "result").unwrap().to_str().unwrap()).unwrap();
    assert_eq!(json[0]["Name"], "C:");
    assert_eq!(json[0]["Flags"], serde_json::json!([1, 2]));
    // De novo: os campos já existem e não são recriados.
    ui.invoke(wmi, "execute", &[]).unwrap();
    assert_eq!(ui.children(cds).len(), 5);
}

/// Com campos, cada um recebe a coluna do mesmo nome (sem diferenciar maiúsculas).
#[test]
fn execute_fills_the_fields_the_dataset_has() {
    let (mut ui, form, wmi, _) = setup();
    let cds = ui.create("ClientDataSet", Some(form)).unwrap();
    ui.add(Field::new("name", FieldType::String), cds);
    ui.add(Field::new("Nada", FieldType::String), cds);
    ui.set_prop(wmi, "data_set", PropValue::Component(Some(cds))).unwrap();
    ui.invoke(wmi, "execute", &[]).unwrap();
    assert_eq!(ui.children(cds).len(), 2, "não criou outros");
    let v = view(&ui, cds);
    assert_eq!(v.value_at(1, "name").unwrap(), Value::String("D:".into()));
    assert_eq!(v.value_at(1, "Nada").unwrap(), Value::Null);
}

#[test]
fn get_value_and_get_json() {
    let (mut ui, _, wmi, _) = setup();
    let args = [PropValue::from("SELECT Size FROM Win32_LogicalDisk"), PropValue::from("size")];
    assert_eq!(ui.invoke(wmi, "get_value", &args).unwrap(), PropValue::String("512000000000".into()));
    let args = [PropValue::from("SELECT * FROM Win32_LogicalDisk"), PropValue::from("Flags")];
    assert_eq!(ui.invoke(wmi, "get_value", &args).unwrap(), PropValue::String("[1,2]".into()));
    let json = ui.invoke(wmi, "get_json", &[PropValue::from("SELECT * FROM Win32_LogicalDisk")]).unwrap();
    let json: serde_json::Value = serde_json::from_str(json.to_str().unwrap()).unwrap();
    assert_eq!(json.as_array().unwrap().len(), 2);
    assert_eq!(json[1]["InstallDate"], serde_json::Value::Null);
}

#[test]
fn refusals_come_back_as_errors() {
    let (mut ui, _, wmi, log) = setup();
    ui.set_prop(wmi, "namespace", "\\\\servidor\\root\\cimv2").unwrap();
    let e = ui.invoke(wmi, "execute", &[]).unwrap_err();
    assert!(e.contains("servidor"), "{e}");
    ui.set_prop(wmi, "namespace", "root\\cimv2").unwrap();
    ui.set_prop(wmi, "query", "  ").unwrap();
    assert!(ui.invoke(wmi, "execute", &[]).is_err());
    ui.set_prop(wmi, "query", "SELECT * FROM Erro").unwrap();
    assert!(ui.invoke(wmi, "execute", &[]).unwrap_err().contains("inválida"));
    assert_eq!(log.lock().unwrap().queries.len(), 2, "a consulta vazia nem chega ao serviço");
    // Sem o serviço (outra plataforma): o erro diz por quê.
    ui.services_mut().wmi = None;
    assert!(ui.invoke(wmi, "execute", &[]).unwrap_err().contains("Windows"));
}

/// O assíncrono: `busy` até a resposta chegar pela fila da tela, e o fim no `on_done`.
#[test]
fn execute_async_ends_in_on_done() {
    let (mut ui, form, wmi, log) = setup();
    let cds = ui.create("ClientDataSet", Some(form)).unwrap();
    ui.set_prop(wmi, "data_set", PropValue::Component(Some(cds))).unwrap();
    let done = Rc::new(RefCell::new(Vec::new()));
    let d = done.clone();
    ui.on(wmi, "on_done", move |_, _, args| d.borrow_mut().push(format!("{args:?}")));
    log.lock().unwrap().hold = true;
    ui.invoke(wmi, "execute_async", &[]).unwrap();
    assert_eq!(ui.get_prop(wmi, "busy").unwrap(), PropValue::Bool(true));
    // A resposta vem de outra thread.
    let held = std::mem::take(&mut log.lock().unwrap().held);
    std::thread::spawn(move || {
        for f in held {
            f(Ok(disks()));
        }
    })
    .join()
    .unwrap();
    ui.process_posted();
    ui.process_events();
    assert_eq!(ui.get_prop(wmi, "busy").unwrap(), PropValue::Bool(false));
    assert_eq!(done.borrow().len(), 1);
    assert!(done.borrow()[0].contains("Int(2)"), "{:?}", done.borrow());
    assert_eq!(view(&ui, cds).record_count(), 2);
}

/// O `watch`: a assinatura liga só com o app rodando, cada evento chega no `on_event` em
/// JSON, e desligar (ou liberar o componente) larga a assinatura.
#[test]
fn watch_delivers_events_and_stops() {
    let (mut ui, _, wmi, log) = setup();
    let events = Rc::new(RefCell::new(Vec::new()));
    let e = events.clone();
    ui.on(wmi, "on_event", move |_, _, args| e.borrow_mut().push(args.text().unwrap_or_default().to_string()));
    ui.set_prop(wmi, "query", "SELECT * FROM __InstanceCreationEvent WITHIN 1 WHERE TargetInstance ISA 'Win32_Process'").unwrap();
    ui.set_prop(wmi, "watch", true).unwrap();
    ui.advance(0.05);
    let listener = log.lock().unwrap().listener.clone().expect("assinou");
    std::thread::spawn(move || {
        listener(Ok(vec![
            ("__CLASS".into(), WmiValue::String("__InstanceCreationEvent".into())),
            ("TargetInstance".into(), WmiValue::Object(vec![("Name".into(), WmiValue::String("notepad.exe".into())), ("ProcessId".into(), WmiValue::Int(42))])),
        ]));
    })
    .join()
    .unwrap();
    ui.process_posted();
    ui.process_events();
    assert_eq!(events.borrow().len(), 1);
    let json: serde_json::Value = serde_json::from_str(&events.borrow()[0]).unwrap();
    assert_eq!(json["TargetInstance"]["Name"], "notepad.exe");
    assert_eq!(json["TargetInstance"]["ProcessId"], 42);
    assert_eq!(ui.get_prop(wmi, "last_event").unwrap().to_str().unwrap(), events.borrow()[0]);
    // Parado, sem timer.
    assert!(ui.component(wmi).unwrap().timer_interval().is_none());
    ui.set_prop(wmi, "watch", false).unwrap();
    ui.advance(0.05);
    assert!(log.lock().unwrap().stopped);
    // Liberar o componente também larga.
    ui.set_prop(wmi, "watch", true).unwrap();
    ui.advance(0.05);
    assert!(!log.lock().unwrap().stopped);
    ui.free(wmi);
    assert!(log.lock().unwrap().stopped);
}

/// No designer, a assinatura não liga.
#[test]
fn watch_waits_for_the_running_app() {
    let (mut ui, _, wmi, log) = setup();
    ui.set_design_mode(true);
    ui.set_prop(wmi, "watch", true).unwrap();
    ui.advance(0.05);
    assert!(log.lock().unwrap().listener.is_none());
}
