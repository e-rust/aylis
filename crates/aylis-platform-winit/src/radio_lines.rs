//! Os eventos de NFC e Bluetooth que o Java do Android entrega em texto (uma linha por
//! evento, campos separados por tab; os bytes em hexadecimal). Separado do JNI para ser
//! testado no desktop.

use aylis_core::platform::{BleCharacteristic, BleDevice, BluetoothEvent, NfcEvent, NfcTag};

pub(crate) fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).filter_map(|i| u8::from_str_radix(text.get(i * 2..i * 2 + 2)?, 16).ok()).collect()
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// `tag id tecnologia gravável ndef`, `written`, `error mensagem`.
pub(crate) fn nfc_events(text: &str) -> Vec<NfcEvent> {
    text.lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            Some(match f[0] {
                "tag" => NfcEvent::Tag(NfcTag {
                    id: aylis_core::ndef::id_text(&unhex(f.get(1)?)),
                    tech: f.get(2).unwrap_or(&"").to_string(),
                    writable: f.get(3) == Some(&"1"),
                    records: aylis_core::ndef::decode(&unhex(f.get(4).unwrap_or(&""))),
                }),
                "written" => NfcEvent::Written,
                "error" => NfcEvent::Error(f.get(1).unwrap_or(&"").to_string()),
                _ => return None,
            })
        })
        .collect()
}

/// As linhas do Bluetooth (veja a `AylisActivity`: `found`, `scanstop`, `connected`,
/// `disconnected`, `value`, `written`, `received`, `error`).
pub(crate) fn bt_events(text: &str) -> Vec<BluetoothEvent> {
    text.lines()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let at = |i: usize| f.get(i).copied().unwrap_or("").to_string();
            Some(match f[0] {
                "found" => BluetoothEvent::Found(BleDevice { id: at(1), name: at(2), rssi: at(3).parse().unwrap_or(0) }),
                "scanstop" => BluetoothEvent::ScanStopped,
                "connected" => BluetoothEvent::Connected {
                    device: at(1),
                    services: at(2)
                        .split(';')
                        .filter_map(|c| {
                            let (path, props) = c.split_once(' ').unwrap_or((c, ""));
                            let (service, characteristic) = path.split_once('/')?;
                            Some(BleCharacteristic {
                                service: service.to_string(),
                                characteristic: characteristic.to_string(),
                                properties: props.split(',').filter(|p| !p.is_empty()).map(str::to_string).collect(),
                            })
                        })
                        .collect(),
                },
                "disconnected" => BluetoothEvent::Disconnected(at(1)),
                "value" => BluetoothEvent::Value { device: at(1), characteristic: at(2), data: unhex(&at(3)) },
                "written" => BluetoothEvent::Written { device: at(1), characteristic: at(2) },
                "received" => BluetoothEvent::Received { device: at(1), data: unhex(&at(2)) },
                "error" => BluetoothEvent::Error(at(1)),
                _ => return None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aylis_core::ndef::NdefRecord;

    #[test]
    fn the_java_lines_become_events() {
        let ndef = hex(&aylis_core::ndef::encode(&[NdefRecord::text("oi")]));
        let nfc = nfc_events(&format!("tag\t04A21B\tNfcA\t1\t{ndef}\nwritten\nerror\tsó leitura\nlixo\n"));
        assert_eq!(
            nfc,
            [
                NfcEvent::Tag(NfcTag { id: "04:A2:1B".into(), tech: "NfcA".into(), records: vec![NdefRecord::text("oi")], writable: true }),
                NfcEvent::Written,
                NfcEvent::Error("só leitura".into()),
            ]
        );
        let bt = bt_events("found\tAA:BB\tH10\t-60\nconnected\tAA:BB\ts1/c1 read,notify;s1/c2 write;\nvalue\tAA:BB\tc1\t0048\nreceived\tAA:BB\t6F6B\n");
        assert_eq!(bt[0], BluetoothEvent::Found(BleDevice { id: "AA:BB".into(), name: "H10".into(), rssi: -60 }));
        let BluetoothEvent::Connected { services, .. } = &bt[1] else { panic!("{bt:?}") };
        assert_eq!(services.len(), 2);
        assert_eq!(services[0].properties, ["read", "notify"]);
        assert_eq!(bt[2], BluetoothEvent::Value { device: "AA:BB".into(), characteristic: "c1".into(), data: vec![0, 0x48] });
        assert_eq!(bt[3], BluetoothEvent::Received { device: "AA:BB".into(), data: b"ok".to_vec() });
        // O clássico conecta sem características.
        assert_eq!(bt_events("connected\tAA:BB\t\n"), [BluetoothEvent::Connected { device: "AA:BB".into(), services: vec![] }]);
    }
}
