//! Bluetooth BLE no desktop pelo `btleplug` (WinRT no Windows, Core Bluetooth no macOS, o
//! BlueZ no Linux). O `btleplug` é assíncrono: uma thread com um runtime do tokio recebe os
//! pedidos do componente por um canal e devolve os eventos numa fila que o `poll` esvazia.
//! O clássico (SPP) fica de fora: no desktop o sistema o mostra como uma porta serial.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use btleplug::api::{Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::StreamExt;
use aylis_core::platform::{BleCharacteristic, BleDevice, BluetoothEvent, BluetoothService};
use tokio::sync::mpsc;
use uuid::Uuid;

enum Command {
    Scan { on: bool, services: Vec<String>, name: String },
    Connect(String),
    Disconnect(String),
    Read { device: String, characteristic: String },
    Write { device: String, characteristic: String, data: Vec<u8>, with_response: bool },
    Subscribe { device: String, characteristic: String, on: bool },
}

type Events = Arc<Mutex<Vec<BluetoothEvent>>>;

/// O `available`: 0 enquanto procura o adaptador; achou; não há.
const YES: u8 = 1;
const NO: u8 = 2;

pub(crate) struct DesktopBluetooth {
    events: Events,
    available: Arc<AtomicU8>,
    /// A thread só nasce no primeiro uso (o `available` do componente já conta).
    commands: OnceLock<mpsc::UnboundedSender<Command>>,
}

impl DesktopBluetooth {
    pub(crate) fn new() -> Self {
        Self { events: Arc::default(), available: Arc::default(), commands: OnceLock::new() }
    }

    fn sender(&self) -> &mpsc::UnboundedSender<Command> {
        self.commands.get_or_init(|| {
            let (tx, rx) = mpsc::unbounded_channel();
            let (events, available) = (self.events.clone(), self.available.clone());
            let spawned = std::thread::Builder::new().name("aylis-bluetooth".into()).spawn(move || {
                match tokio::runtime::Builder::new_multi_thread().worker_threads(1).enable_all().build() {
                    Ok(rt) => rt.block_on(run(rx, events, available)),
                    Err(_) => available.store(NO, Ordering::SeqCst),
                }
            });
            if spawned.is_err() {
                self.available.store(NO, Ordering::SeqCst);
            }
            tx
        })
    }

    fn send(&self, command: Command) {
        let _ = self.sender().send(command);
    }
}

impl BluetoothService for DesktopBluetooth {
    fn available(&self) -> bool {
        self.sender();
        self.available.load(Ordering::SeqCst) == YES
    }

    fn scan(&mut self, on: bool, services: Vec<String>, name: String) {
        self.send(Command::Scan { on, services, name });
    }

    fn connect(&mut self, device: &str) {
        self.send(Command::Connect(device.to_string()));
    }

    fn disconnect(&mut self, device: &str) {
        self.send(Command::Disconnect(device.to_string()));
    }

    fn read(&mut self, device: &str, _service: &str, characteristic: &str) {
        self.send(Command::Read { device: device.to_string(), characteristic: characteristic.to_string() });
    }

    fn write(&mut self, device: &str, _service: &str, characteristic: &str, data: Vec<u8>, with_response: bool) {
        self.send(Command::Write { device: device.to_string(), characteristic: characteristic.to_string(), data, with_response });
    }

    fn subscribe(&mut self, device: &str, _service: &str, characteristic: &str, on: bool) {
        self.send(Command::Subscribe { device: device.to_string(), characteristic: characteristic.to_string(), on });
    }

    fn poll(&mut self) -> Vec<BluetoothEvent> {
        self.events.lock().map(|mut e| std::mem::take(&mut *e)).unwrap_or_default()
    }
}

fn push(events: &Events, event: BluetoothEvent) {
    if let Ok(mut e) = events.lock() {
        e.push(event);
    }
}

fn props(flags: CharPropFlags) -> Vec<String> {
    [
        (CharPropFlags::READ, "read"),
        (CharPropFlags::WRITE, "write"),
        (CharPropFlags::WRITE_WITHOUT_RESPONSE, "write_without_response"),
        (CharPropFlags::NOTIFY, "notify"),
        (CharPropFlags::INDICATE, "indicate"),
    ]
    .into_iter()
    .filter(|(f, _)| flags.contains(*f))
    .map(|(_, n)| n.to_string())
    .collect()
}

/// O laço da thread: acha o adaptador, atende os pedidos e repassa os eventos do sistema.
async fn run(mut commands: mpsc::UnboundedReceiver<Command>, events: Events, available: Arc<AtomicU8>) {
    let adapter = match Manager::new().await {
        Ok(m) => m.adapters().await.ok().and_then(|a| a.into_iter().next()),
        Err(_) => None,
    };
    let Some(adapter) = adapter else {
        available.store(NO, Ordering::SeqCst);
        return;
    };
    available.store(YES, Ordering::SeqCst);
    let mut central = match adapter.events().await {
        Ok(s) => s,
        Err(e) => return push(&events, BluetoothEvent::Error(e.to_string())),
    };
    let name_filter: Arc<Mutex<String>> = Arc::default();
    loop {
        tokio::select! {
            command = commands.recv() => {
                let Some(command) = command else { return };
                let (adapter, events, name_filter) = (adapter.clone(), events.clone(), name_filter.clone());
                // Cada pedido numa tarefa: conectar demora, e os outros não esperam.
                tokio::spawn(async move {
                    if let Err(e) = handle(&adapter, &events, &name_filter, command).await {
                        push(&events, BluetoothEvent::Error(e));
                    }
                });
            }
            event = central.next() => {
                let Some(event) = event else { return };
                match event {
                    CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
                        let Ok(p) = adapter.peripheral(&id).await else { continue };
                        let Ok(Some(info)) = p.properties().await else { continue };
                        let name = info.local_name.or(info.advertisement_name).unwrap_or_default();
                        let wanted = name_filter.lock().map(|n| n.clone()).unwrap_or_default();
                        if !wanted.is_empty() && !name.to_lowercase().contains(&wanted.to_lowercase()) {
                            continue;
                        }
                        push(&events, BluetoothEvent::Found(BleDevice { id: id.to_string(), name, rssi: info.rssi.unwrap_or(0) as i32 }));
                    }
                    CentralEvent::DeviceDisconnected(id) => push(&events, BluetoothEvent::Disconnected(id.to_string())),
                    _ => {}
                }
            }
        }
    }
}

async fn find(adapter: &Adapter, device: &str) -> Result<Peripheral, String> {
    let list = adapter.peripherals().await.map_err(|e| e.to_string())?;
    list.into_iter().find(|p| p.id().to_string().eq_ignore_ascii_case(device)).ok_or_else(|| format!("aparelho {device} não encontrado (procure antes)"))
}

fn characteristic(p: &Peripheral, uuid: &str) -> Result<Characteristic, String> {
    let uuid = Uuid::parse_str(uuid).map_err(|_| format!("uuid inválido: {uuid}"))?;
    p.characteristics().into_iter().find(|c| c.uuid == uuid).ok_or_else(|| format!("característica {uuid} não encontrada"))
}

/// Os avisos de cada aparelho já ligados (o fluxo de notificações é um por aparelho).
static LISTENING: Mutex<Option<HashMap<String, ()>>> = Mutex::new(None);

async fn handle(adapter: &Adapter, events: &Events, name_filter: &Mutex<String>, command: Command) -> Result<(), String> {
    match command {
        Command::Scan { on, services, name } => {
            if let Ok(mut n) = name_filter.lock() {
                *n = name;
            }
            if on {
                let services = services.iter().filter_map(|s| Uuid::parse_str(s).ok()).collect();
                adapter.start_scan(ScanFilter { services }).await.map_err(|e| e.to_string())?;
            } else {
                adapter.stop_scan().await.map_err(|e| e.to_string())?;
                push(events, BluetoothEvent::ScanStopped);
            }
        }
        Command::Connect(device) => {
            let p = find(adapter, &device).await?;
            if !p.is_connected().await.unwrap_or(false) {
                p.connect().await.map_err(|e| e.to_string())?;
            }
            p.discover_services().await.map_err(|e| e.to_string())?;
            let services = p
                .characteristics()
                .into_iter()
                .map(|c| BleCharacteristic { service: c.service_uuid.to_string(), characteristic: c.uuid.to_string(), properties: props(c.properties) })
                .collect();
            let first = LISTENING.lock().map(|mut l| l.get_or_insert_with(HashMap::new).insert(device.clone(), ()).is_none()).unwrap_or(false);
            if first {
                let mut stream = p.notifications().await.map_err(|e| e.to_string())?;
                let (events, device) = (events.clone(), device.clone());
                tokio::spawn(async move {
                    while let Some(n) = stream.next().await {
                        push(&events, BluetoothEvent::Value { device: device.clone(), characteristic: n.uuid.to_string(), data: n.value });
                    }
                    if let Ok(mut l) = LISTENING.lock()
                        && let Some(l) = l.as_mut()
                    {
                        l.remove(&device);
                    }
                });
            }
            push(events, BluetoothEvent::Connected { device, services });
        }
        Command::Disconnect(device) => {
            find(adapter, &device).await?.disconnect().await.map_err(|e| e.to_string())?;
        }
        Command::Read { device, characteristic: uuid } => {
            let p = find(adapter, &device).await?;
            let c = characteristic(&p, &uuid)?;
            let data = p.read(&c).await.map_err(|e| e.to_string())?;
            push(events, BluetoothEvent::Value { device, characteristic: c.uuid.to_string(), data });
        }
        Command::Write { device, characteristic: uuid, data, with_response } => {
            let p = find(adapter, &device).await?;
            let c = characteristic(&p, &uuid)?;
            let kind = if with_response { WriteType::WithResponse } else { WriteType::WithoutResponse };
            p.write(&c, &data, kind).await.map_err(|e| e.to_string())?;
            push(events, BluetoothEvent::Written { device, characteristic: c.uuid.to_string() });
        }
        Command::Subscribe { device, characteristic: uuid, on } => {
            let p = find(adapter, &device).await?;
            let c = characteristic(&p, &uuid)?;
            if on { p.subscribe(&c).await } else { p.unsubscribe(&c).await }.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Com o Bluetooth da máquina: procura por 8 s e mostra o que achou (`--ignored`).
    #[test]
    #[ignore]
    fn scans_the_real_adapter() {
        let mut bt = DesktopBluetooth::new();
        let start = std::time::Instant::now();
        while bt.available.load(Ordering::SeqCst) == 0 && start.elapsed().as_secs() < 5 {
            bt.available();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(bt.available(), "sem adaptador Bluetooth");
        bt.scan(true, Vec::new(), String::new());
        std::thread::sleep(std::time::Duration::from_secs(8));
        bt.scan(false, Vec::new(), String::new());
        std::thread::sleep(std::time::Duration::from_millis(500));
        let events = bt.poll();
        let found: Vec<&BleDevice> = events.iter().filter_map(|e| if let BluetoothEvent::Found(d) = e { Some(d) } else { None }).collect();
        aylis_core::stderr_line!("achados: {}", found.len());
        for d in found.iter().take(10) {
            aylis_core::stderr_line!("  {} '{}' {} dBm", d.id, d.name, d.rssi);
        }
        assert!(events.iter().all(|e| !matches!(e, BluetoothEvent::Error(_))), "{events:?}");
    }
}
