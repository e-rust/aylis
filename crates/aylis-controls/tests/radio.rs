//! `Nfc` e `Bluetooth` com serviços de plataforma falsos: os pedidos chegam à plataforma, o
//! que ela conta vira evento e propriedade, e sem serviço `available` fica falso.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_controls::Form;
use aylis_core::ndef::{self, NdefRecord};
use aylis_core::platform::{BleCharacteristic, BleDevice, BluetoothEvent, BluetoothService, NfcEvent, NfcService, NfcTag, ble_uuid};
use aylis_core::*;

struct Shared<E> {
    calls: Vec<String>,
    events: Vec<E>,
}

impl<E> Default for Shared<E> {
    fn default() -> Self {
        Self { calls: Vec::new(), events: Vec::new() }
    }
}

type Link<E> = Arc<Mutex<Shared<E>>>;

struct FakeNfc(Link<NfcEvent>);

impl NfcService for FakeNfc {
    fn available(&self) -> bool {
        true
    }
    fn set_reading(&mut self, on: bool) {
        self.0.lock().unwrap().calls.push(format!("reading {on}"));
    }
    fn write(&mut self, message: Vec<u8>) {
        self.0.lock().unwrap().calls.push(format!("write {}", ndef::records_to_text(&ndef::decode(&message))));
    }
    fn poll(&mut self) -> Vec<NfcEvent> {
        std::mem::take(&mut self.0.lock().unwrap().events)
    }
}

struct FakeBluetooth(Link<BluetoothEvent>);

impl BluetoothService for FakeBluetooth {
    fn available(&self) -> bool {
        true
    }
    fn scan(&mut self, on: bool, services: Vec<String>, name: String) {
        self.0.lock().unwrap().calls.push(format!("scan {on} {} {name}", services.join(",")));
    }
    fn connect(&mut self, device: &str) {
        self.0.lock().unwrap().calls.push(format!("connect {device}"));
    }
    fn disconnect(&mut self, device: &str) {
        self.0.lock().unwrap().calls.push(format!("disconnect {device}"));
    }
    fn read(&mut self, device: &str, service: &str, characteristic: &str) {
        self.0.lock().unwrap().calls.push(format!("read {device} {service} {characteristic}"));
    }
    fn write(&mut self, device: &str, _: &str, characteristic: &str, data: Vec<u8>, with_response: bool) {
        self.0.lock().unwrap().calls.push(format!("write {device} {characteristic} {data:?} {with_response}"));
    }
    fn subscribe(&mut self, device: &str, _: &str, characteristic: &str, on: bool) {
        self.0.lock().unwrap().calls.push(format!("subscribe {device} {characteristic} {on}"));
    }
    fn poll(&mut self) -> Vec<BluetoothEvent> {
        std::mem::take(&mut self.0.lock().unwrap().events)
    }
}

fn record(ui: &mut Ui, id: ComponentId, events: &[&'static str]) -> Rc<RefCell<Vec<String>>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    for &event in events {
        let log = log.clone();
        ui.on(id, event, move |_, _, args| log.borrow_mut().push(format!("{event}:{}", args.text().unwrap_or_default())));
    }
    log
}

fn step(ui: &mut Ui, t: &mut f64) {
    *t += 1.5;
    ui.set_time(*t);
    ui.run_timers();
}

#[test]
fn nfc_reads_tags_and_writes_on_the_next_one() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let nfc = ui.create("Nfc", Some(form)).unwrap();
    let mut t = 0.0;
    // Sem serviço: indisponível e o gravar avisa.
    step(&mut ui, &mut t);
    assert_eq!(ui.get_prop(nfc, "available").unwrap(), PropValue::Bool(false));
    assert!(ui.invoke(nfc, "write", &[PropValue::from("text:oi")]).is_err());

    let link: Link<NfcEvent> = Default::default();
    ui.services_mut().nfc = Some(Box::new(FakeNfc(link.clone())));
    let log = record(&mut ui, nfc, &["on_tag", "on_written", "on_error"]);
    ui.set_prop(nfc, "active", true).unwrap();
    step(&mut ui, &mut t);
    assert_eq!(ui.get_prop(nfc, "available").unwrap(), PropValue::Bool(true));
    assert_eq!(link.lock().unwrap().calls, ["reading true"]);
    link.lock().unwrap().events.push(NfcEvent::Tag(NfcTag {
        id: "04:A2:1B".into(),
        tech: "MifareUltralight".into(),
        records: vec![NdefRecord::text("Olá"), NdefRecord::Uri("https://aylis.dev".into())],
        writable: true,
    }));
    step(&mut ui, &mut t);
    assert_eq!(*log.borrow(), ["on_tag:04:A2:1B"]);
    assert_eq!(ui.get_prop(nfc, "records").unwrap(), PropValue::from("text:Olá\nuri:https://aylis.dev"));
    assert_eq!(ui.get_prop(nfc, "tag_type").unwrap(), PropValue::from("MifareUltralight"));
    assert_eq!(ui.get_prop(nfc, "writable").unwrap(), PropValue::Bool(true));

    // Gravar: a mensagem montada vai para a plataforma; o resultado chega depois.
    assert!(ui.invoke(nfc, "write", &[PropValue::from("")]).is_err(), "nada para gravar");
    ui.invoke(nfc, "write", &[PropValue::from("text:Pedido 42\nuri:tel:+5511999999999")]).unwrap();
    assert_eq!(link.lock().unwrap().calls.last().unwrap(), "write text:Pedido 42\nuri:tel:+5511999999999");
    assert_eq!(ui.get_prop(nfc, "writing").unwrap(), PropValue::Bool(true));
    link.lock().unwrap().events.extend([NfcEvent::Written, NfcEvent::Error("a tag é só de leitura".into())]);
    step(&mut ui, &mut t);
    assert_eq!(log.borrow()[1..], ["on_written:".to_string(), "on_error:a tag é só de leitura".to_string()]);
    assert_eq!(ui.get_prop(nfc, "writing").unwrap(), PropValue::Bool(false));

    // Desligado, para de ler.
    ui.set_prop(nfc, "active", false).unwrap();
    step(&mut ui, &mut t);
    assert_eq!(link.lock().unwrap().calls.last().unwrap(), "reading false");
}

#[test]
fn bluetooth_scans_connects_reads_and_is_notified() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("t", 100.0, 100.0));
    let bt = ui.create("Bluetooth", Some(form)).unwrap();
    let mut t = 0.0;
    step(&mut ui, &mut t);
    assert_eq!(ui.get_prop(bt, "available").unwrap(), PropValue::Bool(false));
    assert!(ui.invoke(bt, "start_scan", &[]).is_err(), "sem Bluetooth");

    let link: Link<BluetoothEvent> = Default::default();
    ui.services_mut().bluetooth = Some(Box::new(FakeBluetooth(link.clone())));
    let log = record(&mut ui, bt, &["on_found", "on_connected", "on_disconnected", "on_value", "on_written", "on_error"]);
    ui.set_prop(bt, "service_filter", "180D; 0x180F").unwrap();
    ui.set_prop(bt, "name_filter", " Polar ").unwrap();
    ui.invoke(bt, "start_scan", &[]).unwrap();
    assert_eq!(link.lock().unwrap().calls, [format!("scan true {},{} Polar", ble_uuid("180d"), ble_uuid("180f"))]);
    assert_eq!(ui.get_prop(bt, "scanning").unwrap(), PropValue::Bool(true));
    let polar = BleDevice { id: "A0:9E:1A:00:11:22".into(), name: "Polar H10".into(), rssi: -60 };
    link.lock().unwrap().events.extend([
        BluetoothEvent::Found(polar.clone()),
        // O mesmo de novo, com outro sinal: atualiza sem avisar.
        BluetoothEvent::Found(BleDevice { rssi: -52, ..polar.clone() }),
        BluetoothEvent::ScanStopped,
    ]);
    step(&mut ui, &mut t);
    assert_eq!(*log.borrow(), ["on_found:A0:9E:1A:00:11:22;Polar H10;-60"]);
    assert_eq!(ui.get_prop(bt, "devices").unwrap(), PropValue::from("A0:9E:1A:00:11:22;Polar H10;-52"));
    assert_eq!(ui.get_prop(bt, "scanning").unwrap(), PropValue::Bool(false));

    // Conectar, as características, ler, gravar e o aviso de mudança.
    let id = PropValue::from(polar.id.as_str());
    ui.invoke(bt, "connect", std::slice::from_ref(&id)).unwrap();
    let hr = ble_uuid("2A37");
    link.lock().unwrap().events.push(BluetoothEvent::Connected {
        device: polar.id.clone(),
        services: vec![BleCharacteristic { service: ble_uuid("180D"), characteristic: hr.clone(), properties: vec!["notify".into()] }],
    });
    step(&mut ui, &mut t);
    assert_eq!(ui.get_prop(bt, "connected").unwrap(), PropValue::from(polar.id.as_str()));
    assert_eq!(ui.get_prop(bt, "services").unwrap(), PropValue::from(format!("{}/{hr} notify", ble_uuid("180D")).as_str()));
    let args = |c: &str, data: &str| vec![id.clone(), PropValue::from("180D"), PropValue::from(c), PropValue::from(data), PropValue::Bool(false)];
    ui.invoke(bt, "subscribe", &args("2A37", "")[..3]).unwrap();
    ui.invoke(bt, "read", &args("2A38", "")[..3]).unwrap();
    ui.invoke(bt, "write_hex", &args("2A39", "01 0A")).unwrap();
    ui.invoke(bt, "write", &args("2A39", "ok")).unwrap();
    assert!(ui.invoke(bt, "write_hex", &args("2A39", "1")).is_err(), "hexadecimal ímpar");
    let calls = link.lock().unwrap().calls.clone();
    assert_eq!(calls[2], format!("subscribe {} {hr} true", polar.id));
    assert_eq!(calls[3], format!("read {} {} {}", polar.id, ble_uuid("180D"), ble_uuid("2A38")));
    assert_eq!(calls[4], format!("write {} {} [1, 10] false", polar.id, ble_uuid("2A39")));
    assert_eq!(calls[5], format!("write {} {} [111, 107] false", polar.id, ble_uuid("2A39")));
    link.lock().unwrap().events.extend([
        BluetoothEvent::Value { device: polar.id.clone(), characteristic: hr.clone(), data: vec![0x00, 72] },
        BluetoothEvent::Written { device: polar.id.clone(), characteristic: ble_uuid("2A39") },
        BluetoothEvent::Disconnected(polar.id.clone()),
    ]);
    step(&mut ui, &mut t);
    assert_eq!(log.borrow()[2..], [format!("on_value:{hr};0048"), format!("on_written:{}", ble_uuid("2A39")), format!("on_disconnected:{}", polar.id)]);
    assert_eq!(ui.get_prop(bt, "last_value").unwrap(), PropValue::from("0048"));
    assert_eq!(ui.get_prop(bt, "last_characteristic").unwrap(), PropValue::from(hr.as_str()));
    assert_eq!(ui.get_prop(bt, "connected").unwrap(), PropValue::from(""));
    // O clássico, onde a plataforma não tem: o erro diz.
    assert!(ui.invoke(bt, "open", std::slice::from_ref(&id)).is_err());
    assert_eq!(ui.invoke(bt, "paired", &[]).unwrap(), PropValue::from(""));
}

#[test]
fn short_uuids_become_full_ones() {
    assert_eq!(ble_uuid("180D"), "0000180d-0000-1000-8000-00805f9b34fb");
    assert_eq!(ble_uuid("0x2A37"), "00002a37-0000-1000-8000-00805f9b34fb");
    assert_eq!(ble_uuid("6E400001-B5A3-F393-E0A9-E50E24DCCA9E"), "6e400001-b5a3-f393-e0a9-e50e24dcca9e");
    assert_eq!(aylis_controls::parse_hex("0A:1b ff").unwrap(), [10, 27, 255]);
}
