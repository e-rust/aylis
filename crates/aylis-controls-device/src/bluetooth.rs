//! O componente `Bluetooth`: BLE (procurar, conectar, serviços e características, ler,
//! gravar e ser avisado das mudanças) e o clássico (SPP/RFCOMM: impressoras, leitores
//! serial). A plataforma faz o trabalho ([`aylis_core::platform::BluetoothService`]); o
//! componente consulta num timer.

use aylis_core::platform::{BluetoothEvent, BluetoothService, ble_uuid};
use aylis_core::prop::PropKind;
use aylis_core::{Component, EventArgs, EventCtx, MethodInfo, PropValue, Ui, properties};

/// Bluetooth: BLE e o clássico num componente só.
///
/// - **Procurar:** `start_scan` (com `service_filter`, uuids separados por `;`, e
///   `name_filter`, parte do nome) chama o `on_found` a cada aparelho (`id;nome;rssi`);
///   `devices` lista os achados, um por linha.
/// - **Conectar:** `connect(id)`; o `on_connected` traz o id e a lista de serviços e
///   características fica em `services` (`serviço/característica propriedades` por linha,
///   as propriedades `read`, `write`, `write_without_response`, `notify`, `indicate`).
/// - **Ler e gravar:** `read(id, serviço, característica)` e `write`/`write_hex` (com ou sem
///   resposta); `subscribe` liga o `notify`/`indicate`. Os valores chegam no `on_value`
///   (`característica;hex`), com `last_value` (hex) e `last_text` (UTF-8).
/// - **Clássico:** `paired` lista os pareados; `open(id)`, `send(id, texto)`, `close(id)`; o
///   que chega vem no `on_received`.
///
/// Os uuids aceitam a forma curta do Bluetooth SIG (`180D` = frequência cardíaca).
/// Android pelo Java (as permissões `BLUETOOTH_SCAN` e `BLUETOOTH_CONNECT`, pedidas na
/// primeira vez); Windows, Linux e macOS pelo `btleplug` (só BLE); navegador pela Web
/// Bluetooth (só o Chrome; o navegador mostra o seletor de aparelhos). Onde não houver,
/// `available` é falso.
#[derive(Default)]
pub struct Bluetooth {
    /// A procura só acha os aparelhos com estes serviços (uuids separados por `;`, aceita a
    /// forma curta como `180D`); vazio: todos.
    pub service_filter: String,
    /// A procura só acha os aparelhos com este trecho no nome; vazio: todos.
    pub name_filter: String,
    available: bool,
    scanning: bool,
    devices: Vec<(String, String, i32)>,
    connected: Vec<String>,
    services: String,
    last_device: String,
    last_characteristic: String,
    last_value: Vec<u8>,
}

impl Bluetooth {
    fn service(ui: &mut Ui) -> Result<&mut Box<dyn BluetoothService>, String> {
        ui.services_mut().bluetooth.as_mut().ok_or_else(|| aylis_core::messages::t("este aparelho não tem Bluetooth"))
    }

    pub fn available(&self) -> bool {
        self.available
    }
    pub fn scanning(&self) -> bool {
        self.scanning
    }
    /// Os aparelhos achados, `id;nome;rssi` por linha.
    pub fn devices(&self) -> String {
        self.devices.iter().map(|(id, name, rssi)| format!("{id};{name};{rssi}")).collect::<Vec<_>>().join("\n")
    }
    /// Os ids conectados, separados por `;`.
    pub fn connected(&self) -> String {
        self.connected.join(";")
    }
    pub fn services(&self) -> String {
        self.services.clone()
    }
    pub fn last_device(&self) -> String {
        self.last_device.clone()
    }
    pub fn last_characteristic(&self) -> String {
        self.last_characteristic.clone()
    }
    pub fn last_value(&self) -> String {
        hex(&self.last_value)
    }
    pub fn last_text(&self) -> String {
        String::from_utf8_lossy(&self.last_value).into_owned()
    }
    /// O último valor recebido, em bytes.
    pub fn last_bytes(&self) -> &[u8] {
        &self.last_value
    }
    fn set_s(&mut self, _: String) {}
    fn set_b(&mut self, _: bool) {}
}

/// Os bytes em hexadecimal, sem separador (`0A1B`).
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// `0A 1B`, `0a1b` ou `0A:1B` em bytes.
pub fn parse_hex(text: &str) -> Result<Vec<u8>, String> {
    let digits: String = text.chars().filter(|c| !matches!(c, ' ' | ':' | '-')).collect();
    if digits.len() % 2 != 0 {
        return Err(format!("hexadecimal com número ímpar de dígitos: '{text}'"));
    }
    (0..digits.len()).step_by(2).map(|i| u8::from_str_radix(&digits[i..i + 2], 16).map_err(|_| format!("hexadecimal inválido: '{text}'"))).collect()
}

impl Component for Bluetooth {
    fn class_name(&self) -> &'static str {
        "Bluetooth"
    }

    properties! {
        service_filter: String => "Behavior",
        name_filter: String => "Behavior";
        events: on_found, on_connected, on_disconnected, on_value, on_written, on_received, on_error;
        computed: available: bool => available / set_b, scanning: bool => scanning / set_b, devices: String => devices / set_s,
            connected: String => connected / set_s, services: String => services / set_s, last_device: String => last_device / set_s,
            last_characteristic: String => last_characteristic / set_s, last_value: String => last_value / set_s,
            last_text: String => last_text / set_s
    }

    fn methods(&self) -> &'static [MethodInfo] {
        use PropKind::{Bool, String};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("enabled", &[], Some(Bool)),
            MethodInfo::new("start_scan", &[], None),
            MethodInfo::new("stop_scan", &[], None),
            MethodInfo::new("connect", &[String], None).params(&["device"]),
            MethodInfo::new("disconnect", &[String], None).params(&["device"]),
            MethodInfo::new("read", &[String, String, String], None).params(&["device", "service", "characteristic"]),
            MethodInfo::new("write", &[String, String, String, String, Bool], None).params(&["device", "service", "characteristic", "text", "with_response"]),
            MethodInfo::new("write_hex", &[String, String, String, String, Bool], None).params(&["device", "service", "characteristic", "hex", "with_response"]),
            MethodInfo::new("subscribe", &[String, String, String], None).params(&["device", "service", "characteristic"]),
            MethodInfo::new("unsubscribe", &[String, String, String], None).params(&["device", "service", "characteristic"]),
            MethodInfo::new("paired", &[], Some(String)),
            MethodInfo::new("open", &[String], None).params(&["port"]),
            MethodInfo::new("send", &[String, String], None).params(&["port", "text"]),
            MethodInfo::new("send_hex", &[String, String], None).params(&["port", "hex"]),
            MethodInfo::new("close", &[String], None).params(&["port"]),
        ];
        METHODS
    }

    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let arg = |i: usize| args.get(i).and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
        let flag = |i: usize| args.get(i).and_then(|v| v.to_bool().ok()).unwrap_or(true);
        if method == "enabled" {
            return Ok(PropValue::Bool(ctx.ui.services().bluetooth.as_ref().is_some_and(|s| s.enabled())));
        }
        let s = Self::service(ctx.ui)?;
        match method {
            "start_scan" => {
                let services = self.service_filter.split(';').map(str::trim).filter(|u| !u.is_empty()).map(ble_uuid).collect();
                self.devices.clear();
                s.scan(true, services, self.name_filter.trim().to_string());
                self.scanning = true;
            }
            "stop_scan" => {
                s.scan(false, Vec::new(), String::new());
                self.scanning = false;
            }
            "connect" => s.connect(&arg(0)),
            "disconnect" => s.disconnect(&arg(0)),
            "read" => s.read(&arg(0), &ble_uuid(&arg(1)), &ble_uuid(&arg(2))),
            "write" => s.write(&arg(0), &ble_uuid(&arg(1)), &ble_uuid(&arg(2)), arg(3).into_bytes(), flag(4)),
            "write_hex" => s.write(&arg(0), &ble_uuid(&arg(1)), &ble_uuid(&arg(2)), parse_hex(&arg(3))?, flag(4)),
            "subscribe" => s.subscribe(&arg(0), &ble_uuid(&arg(1)), &ble_uuid(&arg(2)), true),
            "unsubscribe" => s.subscribe(&arg(0), &ble_uuid(&arg(1)), &ble_uuid(&arg(2)), false),
            "paired" => return Ok(PropValue::String(s.paired().into_iter().map(|(id, name)| format!("{id};{name}")).collect::<Vec<_>>().join("\n"))),
            "open" => s.open_serial(&arg(0))?,
            "send" => s.send_serial(&arg(0), arg(1).into_bytes())?,
            "send_hex" => s.send_serial(&arg(0), parse_hex(&arg(1))?)?,
            "close" => s.close_serial(&arg(0)),
            _ => return Err(format!("Bluetooth não tem o método '{method}'")),
        }
        Ok(PropValue::Null)
    }

    fn timer_interval(&self) -> Option<f64> {
        // Sem nada acontecendo, só confere de vez em quando se há Bluetooth (o `available`).
        Some(if self.scanning || !self.connected.is_empty() { 0.05 } else { 0.25 })
    }

    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() {
            return;
        }
        let Some(s) = ctx.ui.services_mut().bluetooth.as_mut() else { return };
        self.available = s.available();
        for event in s.poll() {
            match event {
                BluetoothEvent::Found(d) => {
                    let line = format!("{};{};{}", d.id, d.name, d.rssi);
                    match self.devices.iter_mut().find(|(id, ..)| *id == d.id) {
                        Some(known) => {
                            // O mesmo aparelho de novo (o RSSI muda): atualiza sem avisar outra vez.
                            known.2 = d.rssi;
                            if !d.name.is_empty() {
                                known.1 = d.name;
                            }
                            continue;
                        }
                        None => self.devices.push((d.id, d.name, d.rssi)),
                    }
                    ctx.fire("on_found", EventArgs::Text(line));
                }
                BluetoothEvent::ScanStopped => self.scanning = false,
                BluetoothEvent::Connected { device, services } => {
                    if !self.connected.contains(&device) {
                        self.connected.push(device.clone());
                    }
                    self.services = services
                        .iter()
                        .map(|c| format!("{}/{} {}", c.service, c.characteristic, c.properties.join(",")))
                        .collect::<Vec<_>>()
                        .join("\n");
                    self.last_device = device.clone();
                    ctx.fire("on_connected", EventArgs::Text(device));
                }
                BluetoothEvent::Disconnected(device) => {
                    self.connected.retain(|d| *d != device);
                    ctx.fire("on_disconnected", EventArgs::Text(device));
                }
                BluetoothEvent::Value { device, characteristic, data } => {
                    let text = format!("{characteristic};{}", hex(&data));
                    self.last_device = device;
                    self.last_characteristic = characteristic;
                    self.last_value = data;
                    ctx.fire("on_value", EventArgs::Text(text));
                }
                BluetoothEvent::Written { characteristic, .. } => ctx.fire("on_written", EventArgs::Text(characteristic)),
                BluetoothEvent::Received { device, data } => {
                    self.last_device = device;
                    self.last_characteristic.clear();
                    let text = String::from_utf8_lossy(&data).into_owned();
                    self.last_value = data;
                    ctx.fire("on_received", EventArgs::Text(text));
                }
                BluetoothEvent::Error(e) => ctx.fire("on_error", EventArgs::Text(e)),
            }
        }
    }
}
