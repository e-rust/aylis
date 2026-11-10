//! Navegador: NFC pela Web NFC (o `NDEFReader`, só no Chrome do Android) e Bluetooth pela
//! Web Bluetooth (só no Chrome; o navegador mostra o seletor de aparelhos e pede um clique
//! do usuário antes). Tudo pelo `js_sys::Reflect`, como no `web_device`: onde a API não
//! existe, `available` é falso.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use aylis_core::ndef::NdefRecord;
use aylis_core::platform::{BleCharacteristic, BleDevice, BluetoothEvent, BluetoothService, NfcEvent, NfcService, NfcTag};
use js_sys::{Array, DataView, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;

use super::web_device::{call, get, object, set, wait, window};

fn navigator() -> JsValue {
    get(&window(), "navigator")
}

/// A mensagem de um erro do JavaScript.
fn error_text(e: &JsValue) -> String {
    get(e, "message").as_string().filter(|m| !m.is_empty()).unwrap_or_else(|| format!("{e:?}"))
}

/// Os bytes de um `DataView` (o `data` dos registros e dos valores do Bluetooth).
fn view_bytes(view: &JsValue) -> Vec<u8> {
    match view.dyn_ref::<DataView>() {
        Some(v) => Uint8Array::new_with_byte_offset_and_length(&v.buffer(), v.byte_offset() as u32, v.byte_length() as u32).to_vec(),
        None => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// NFC

#[derive(Default)]
struct NfcState {
    events: Vec<NfcEvent>,
    reader: Option<JsValue>,
    /// O leitor da página para de ler pelo `AbortController`.
    abort: Option<JsValue>,
}

#[derive(Default)]
pub(crate) struct WebNfc {
    state: Rc<RefCell<NfcState>>,
}

fn has_nfc() -> bool {
    !get(&window(), "NDEFReader").is_undefined()
}

fn new_reader() -> Option<JsValue> {
    let class: js_sys::Function = get(&window(), "NDEFReader").dyn_into().ok()?;
    js_sys::Reflect::construct(&class, &Array::new()).ok()
}

/// Um registro da Web NFC no registro do aylis.
fn record_from_js(r: &JsValue) -> NdefRecord {
    let kind = get(r, "recordType").as_string().unwrap_or_default();
    let data = view_bytes(&get(r, "data"));
    match kind.as_str() {
        "text" => NdefRecord::Text { text: String::from_utf8_lossy(&data).into_owned(), lang: get(r, "lang").as_string().unwrap_or_default() },
        "url" | "absolute-url" => NdefRecord::Uri(String::from_utf8_lossy(&data).into_owned()),
        "mime" => NdefRecord::Mime { mime: get(r, "mediaType").as_string().unwrap_or_default(), data },
        other => NdefRecord::Other { tnf: 4, kind: other.as_bytes().to_vec(), data },
    }
}

/// O registro do aylis no formato que o `write` da Web NFC aceita.
fn record_to_js(r: &NdefRecord) -> JsValue {
    let o = object();
    match r {
        NdefRecord::Text { text, lang } => {
            set(&o, "recordType", "text");
            set(&o, "data", text.as_str());
            set(&o, "lang", lang.as_str());
        }
        NdefRecord::Uri(uri) => {
            set(&o, "recordType", "url");
            set(&o, "data", uri.as_str());
        }
        NdefRecord::Mime { mime, data } => {
            set(&o, "recordType", "mime");
            set(&o, "mediaType", mime.as_str());
            set(&o, "data", Uint8Array::from(data.as_slice()));
        }
        NdefRecord::Other { kind, data, .. } => {
            set(&o, "recordType", String::from_utf8_lossy(kind).as_ref());
            set(&o, "data", Uint8Array::from(data.as_slice()));
        }
    }
    o
}

impl NfcService for WebNfc {
    fn available(&self) -> bool {
        has_nfc()
    }

    fn set_reading(&mut self, on: bool) {
        let state = self.state.clone();
        if let Some(abort) = state.borrow_mut().abort.take() {
            let _ = call(&abort, "abort", &[]);
        }
        if !on {
            return;
        }
        let Some(reader) = new_reader() else { return };
        let Ok(controller) = js_sys::Reflect::construct(&get(&window(), "AbortController").unchecked_into::<js_sys::Function>(), &Array::new()) else { return };
        let options = object();
        set(&options, "signal", get(&controller, "signal"));
        let s = state.clone();
        let on_reading = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
            let serial = get(&event, "serialNumber").as_string().unwrap_or_default().to_uppercase();
            let records = get(&get(&event, "message"), "records");
            let records: Vec<NdefRecord> = Array::from(&records).iter().map(|r| record_from_js(&r)).collect();
            s.borrow_mut().events.push(NfcEvent::Tag(NfcTag { id: serial, tech: "NDEF".into(), records, writable: true }));
        });
        set(&reader, "onreading", on_reading.as_ref().clone());
        on_reading.forget();
        {
            let mut st = state.borrow_mut();
            st.reader = Some(reader.clone());
            st.abort = Some(controller);
        }
        spawn_local(async move {
            if let Err(e) = wait(call(&reader, "scan", &[options])).await {
                state.borrow_mut().events.push(NfcEvent::Error(error_text(&e)));
            }
        });
    }

    fn write(&mut self, message: Vec<u8>) {
        let state = self.state.clone();
        let records = aylis_core::ndef::decode(&message);
        let Some(reader) = state.borrow().reader.clone().or_else(new_reader) else {
            state.borrow_mut().events.push(NfcEvent::Error(aylis_core::messages::t("este aparelho não tem NFC")));
            return;
        };
        let message = object();
        set(&message, "records", records.iter().map(record_to_js).collect::<Array>());
        spawn_local(async move {
            let event = match wait(call(&reader, "write", &[message])).await {
                Ok(_) => NfcEvent::Written,
                Err(e) => NfcEvent::Error(error_text(&e)),
            };
            state.borrow_mut().events.push(event);
        });
    }

    fn poll(&mut self) -> Vec<NfcEvent> {
        std::mem::take(&mut self.state.borrow_mut().events)
    }
}

// ---------------------------------------------------------------------------
// Bluetooth

#[derive(Default)]
struct BtState {
    events: Vec<BluetoothEvent>,
    /// Os aparelhos escolhidos no seletor, pelo id que o navegador dá.
    devices: HashMap<String, JsValue>,
    /// As características dos conectados: (aparelho, uuid) → o objeto.
    characteristics: HashMap<(String, String), JsValue>,
}

#[derive(Default)]
pub(crate) struct WebBluetooth {
    state: Rc<RefCell<BtState>>,
}

fn bluetooth() -> JsValue {
    get(&navigator(), "bluetooth")
}

fn props_of(c: &JsValue) -> Vec<String> {
    let p = get(c, "properties");
    [("read", "read"), ("write", "write"), ("writeWithoutResponse", "write_without_response"), ("notify", "notify"), ("indicate", "indicate")]
        .into_iter()
        .filter(|(js, _)| get(&p, js).as_bool() == Some(true))
        .map(|(_, name)| name.to_string())
        .collect()
}

impl WebBluetooth {
    fn push(&self, event: BluetoothEvent) {
        self.state.borrow_mut().events.push(event);
    }

    fn characteristic(&self, device: &str, uuid: &str) -> Option<JsValue> {
        let c = self.state.borrow().characteristics.get(&(device.to_string(), uuid.to_string())).cloned();
        if c.is_none() {
            self.push(BluetoothEvent::Error(format!("característica {uuid} não encontrada em {device}")));
        }
        c
    }
}

impl BluetoothService for WebBluetooth {
    fn available(&self) -> bool {
        !bluetooth().is_undefined()
    }

    /// A Web Bluetooth não procura em segundo plano: o navegador mostra o seletor (com os
    /// filtros) e o aparelho escolhido chega como "achado".
    fn scan(&mut self, on: bool, services: Vec<String>, name: String) {
        if !on {
            self.push(BluetoothEvent::ScanStopped);
            return;
        }
        let options = object();
        let filters = Array::new();
        for s in &services {
            let f = object();
            set(&f, "services", Array::of1(&JsValue::from_str(s)));
            filters.push(&f);
        }
        if !name.is_empty() {
            let f = object();
            set(&f, "namePrefix", name.as_str());
            filters.push(&f);
        }
        if filters.length() == 0 {
            set(&options, "acceptAllDevices", true);
        } else {
            set(&options, "filters", filters);
        }
        // Sem dizer antes, o navegador não deixa usar os serviços depois de conectar.
        set(&options, "optionalServices", services.iter().map(|s| JsValue::from_str(s)).collect::<Array>());
        let state = self.state.clone();
        spawn_local(async move {
            match wait(call(&bluetooth(), "requestDevice", &[options])).await {
                Ok(device) => {
                    let id = get(&device, "id").as_string().unwrap_or_default();
                    let name = get(&device, "name").as_string().unwrap_or_default();
                    let st = state.clone();
                    let gone = id.clone();
                    let on_disconnected = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| st.borrow_mut().events.push(BluetoothEvent::Disconnected(gone.clone())));
                    let _ = call(&device, "addEventListener", &[JsValue::from_str("gattserverdisconnected"), on_disconnected.as_ref().clone()]);
                    on_disconnected.forget();
                    let mut s = state.borrow_mut();
                    s.devices.insert(id.clone(), device);
                    s.events.push(BluetoothEvent::Found(BleDevice { id, name, rssi: 0 }));
                }
                Err(e) => state.borrow_mut().events.push(BluetoothEvent::Error(error_text(&e))),
            }
            state.borrow_mut().events.push(BluetoothEvent::ScanStopped);
        });
    }

    fn connect(&mut self, device: &str) {
        let Some(d) = self.state.borrow().devices.get(device).cloned() else {
            return self.push(BluetoothEvent::Error(format!("aparelho {device} não encontrado (procure antes)")));
        };
        let (state, id) = (self.state.clone(), device.to_string());
        spawn_local(async move {
            let result: Result<Vec<BleCharacteristic>, JsValue> = async {
                let server = wait(call(&get(&d, "gatt"), "connect", &[])).await?;
                let services = wait(call(&server, "getPrimaryServices", &[])).await?;
                let mut list = Vec::new();
                for service in Array::from(&services).iter() {
                    let service_uuid = get(&service, "uuid").as_string().unwrap_or_default();
                    let Ok(chars) = wait(call(&service, "getCharacteristics", &[])).await else { continue };
                    for c in Array::from(&chars).iter() {
                        let uuid = get(&c, "uuid").as_string().unwrap_or_default();
                        list.push(BleCharacteristic { service: service_uuid.clone(), characteristic: uuid.clone(), properties: props_of(&c) });
                        state.borrow_mut().characteristics.insert((id.clone(), uuid), c);
                    }
                }
                Ok(list)
            }
            .await;
            let event = match result {
                Ok(services) => BluetoothEvent::Connected { device: id, services },
                Err(e) => BluetoothEvent::Error(error_text(&e)),
            };
            state.borrow_mut().events.push(event);
        });
    }

    fn disconnect(&mut self, device: &str) {
        if let Some(d) = self.state.borrow().devices.get(device) {
            let _ = call(&get(d, "gatt"), "disconnect", &[]);
        }
    }

    fn read(&mut self, device: &str, _service: &str, characteristic: &str) {
        let Some(c) = self.characteristic(device, characteristic) else { return };
        let (state, device, characteristic) = (self.state.clone(), device.to_string(), characteristic.to_string());
        spawn_local(async move {
            let event = match wait(call(&c, "readValue", &[])).await {
                Ok(view) => BluetoothEvent::Value { device, characteristic, data: view_bytes(&view) },
                Err(e) => BluetoothEvent::Error(error_text(&e)),
            };
            state.borrow_mut().events.push(event);
        });
    }

    fn write(&mut self, device: &str, _service: &str, characteristic: &str, data: Vec<u8>, with_response: bool) {
        let Some(c) = self.characteristic(device, characteristic) else { return };
        let (state, device, characteristic) = (self.state.clone(), device.to_string(), characteristic.to_string());
        let method = if with_response { "writeValueWithResponse" } else { "writeValueWithoutResponse" };
        spawn_local(async move {
            let event = match wait(call(&c, method, &[Uint8Array::from(data.as_slice()).into()])).await {
                Ok(_) => BluetoothEvent::Written { device, characteristic },
                Err(e) => BluetoothEvent::Error(error_text(&e)),
            };
            state.borrow_mut().events.push(event);
        });
    }

    fn subscribe(&mut self, device: &str, _service: &str, characteristic: &str, on: bool) {
        let Some(c) = self.characteristic(device, characteristic) else { return };
        let (state, device, uuid) = (self.state.clone(), device.to_string(), characteristic.to_string());
        if on {
            let (st, d, u) = (state.clone(), device.clone(), uuid.clone());
            let on_change = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                let value = get(&get(&event, "target"), "value");
                st.borrow_mut().events.push(BluetoothEvent::Value { device: d.clone(), characteristic: u.clone(), data: view_bytes(&value) });
            });
            set(&c, "oncharacteristicvaluechanged", on_change.as_ref().clone());
            on_change.forget();
        }
        spawn_local(async move {
            if let Err(e) = wait(call(&c, if on { "startNotifications" } else { "stopNotifications" }, &[])).await {
                state.borrow_mut().events.push(BluetoothEvent::Error(error_text(&e)));
            }
        });
    }

    fn poll(&mut self) -> Vec<BluetoothEvent> {
        std::mem::take(&mut self.state.borrow_mut().events)
    }
}
