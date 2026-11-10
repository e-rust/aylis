//! Android: NFC (o `NfcAdapter` em modo leitor) e Bluetooth (BLE pelo `BluetoothGatt` e o
//! clássico pelo `BluetoothSocket`), pela `AylisActivity`. O Java guarda os eventos numa fila
//! e o aylis consulta (`nfcPoll`, `btPoll`) a cada timer do componente.

use aylis_core::platform::{BluetoothEvent, BluetoothService, NfcEvent, NfcService};
use jni::objects::{JObject, JString, JValue, JValueOwned};

use super::android_jni::with_env;
use super::radio_lines::{bt_events, hex, nfc_events};

pub(crate) fn flag(method: &str) -> bool {
    with_env(|env, a| env.call_method(a, method, "()Z", &[])?.z()).unwrap_or(false)
}

pub(crate) fn text(method: &str) -> String {
    with_env(|env, a| {
        let s: JString = env.call_method(a, method, "()Ljava/lang/String;", &[])?.l()?.into();
        Ok(env.get_string(&s).map(String::from).unwrap_or_default())
    })
    .unwrap_or_default()
}

/// Chama um método `(String...)V` ou `(String..., boolean)V` da `AylisActivity`.
pub(crate) fn call(method: &str, args: &[&str], last: Option<bool>) {
    with_env(|env, a| {
        let mut values: Vec<JValueOwned> = Vec::new();
        let mut sig = String::from("(");
        for arg in args {
            values.push(JValueOwned::Object(JObject::from(env.new_string(arg)?)));
            sig.push_str("Ljava/lang/String;");
        }
        if let Some(b) = last {
            values.push(JValueOwned::Bool(b.into()));
            sig.push('Z');
        }
        sig.push_str(")V");
        let borrowed: Vec<JValue> = values.iter().map(|v| v.borrow()).collect();
        env.call_method(a, method, &sig, &borrowed).map(|_| ())
    });
}

pub(crate) struct AndroidNfc;

impl NfcService for AndroidNfc {
    fn available(&self) -> bool {
        flag("nfcAvailable")
    }

    fn enabled(&self) -> bool {
        flag("nfcEnabled")
    }

    fn set_reading(&mut self, on: bool) {
        with_env(|env, a| env.call_method(a, "nfcRead", "(Z)V", &[JValue::Bool(on.into())]).map(|_| ()));
    }

    fn write(&mut self, message: Vec<u8>) {
        call("nfcWrite", &[&hex(&message)], None);
    }

    fn cancel_write(&mut self) {
        call("nfcWrite", &[""], None);
    }

    fn poll(&mut self) -> Vec<NfcEvent> {
        nfc_events(&text("nfcPoll"))
    }
}

pub(crate) struct AndroidBluetooth;

impl BluetoothService for AndroidBluetooth {
    fn available(&self) -> bool {
        flag("btAvailable")
    }

    fn enabled(&self) -> bool {
        flag("btEnabled")
    }

    fn scan(&mut self, on: bool, services: Vec<String>, name: String) {
        with_env(|env, a| {
            let s = env.new_string(services.join(";"))?;
            let n = env.new_string(name)?;
            env.call_method(a, "btScan", "(ZLjava/lang/String;Ljava/lang/String;)V", &[JValue::Bool(on.into()), (&s).into(), (&n).into()]).map(|_| ())
        });
    }

    fn connect(&mut self, device: &str) {
        call("btConnect", &[device], None);
    }

    fn disconnect(&mut self, device: &str) {
        call("btDisconnect", &[device], None);
    }

    fn read(&mut self, device: &str, service: &str, characteristic: &str) {
        call("btRead", &[device, service, characteristic], None);
    }

    fn write(&mut self, device: &str, service: &str, characteristic: &str, data: Vec<u8>, with_response: bool) {
        call("btWrite", &[device, service, characteristic, &hex(&data)], Some(with_response));
    }

    fn subscribe(&mut self, device: &str, service: &str, characteristic: &str, on: bool) {
        call("btSubscribe", &[device, service, characteristic], Some(on));
    }

    fn paired(&mut self) -> Vec<(String, String)> {
        text("btPaired").lines().filter_map(|l| l.split_once(';')).map(|(id, name)| (id.to_string(), name.to_string())).collect()
    }

    fn open_serial(&mut self, device: &str) -> Result<(), String> {
        call("btOpen", &[device], None);
        Ok(())
    }

    fn send_serial(&mut self, device: &str, data: Vec<u8>) -> Result<(), String> {
        let sent = with_env(|env, a| {
            let d = env.new_string(device)?;
            let h = env.new_string(hex(&data))?;
            env.call_method(a, "btSend", "(Ljava/lang/String;Ljava/lang/String;)Z", &[(&d).into(), (&h).into()])?.z()
        });
        if sent == Some(true) { Ok(()) } else { Err(format!("{device} não está aberto (open)")) }
    }

    fn close_serial(&mut self, device: &str) {
        call("btClose", &[device], None);
    }

    fn poll(&mut self) -> Vec<BluetoothEvent> {
        bt_events(&text("btPoll"))
    }
}
