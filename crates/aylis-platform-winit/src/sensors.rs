//! Os sensores do desktop (os componentes `Battery`, `Network` e `Location`): no Windows, a
//! energia do sistema, o perfil de rede e a localização do Windows; no Linux, a bateria e a
//! rede pelo `/sys`. O Android tem os dele, pela `AylisActivity` (`android_device`).

use aylis_core::platform::{Sensor, SensorService};

#[cfg(windows)]
pub(crate) struct DesktopSensors {
    /// A última posição, medida numa thread enquanto a localização está ligada.
    location: std::sync::Arc<std::sync::Mutex<Option<Vec<f64>>>>,
    locating: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(windows)]
impl DesktopSensors {
    pub(crate) fn new() -> Self {
        Self { location: Default::default(), locating: Default::default() }
    }

    fn power() -> Option<windows::Win32::System::Power::SYSTEM_POWER_STATUS> {
        let mut status = windows::Win32::System::Power::SYSTEM_POWER_STATUS::default();
        unsafe { windows::Win32::System::Power::GetSystemPowerStatus(&mut status) }.ok()?;
        Some(status)
    }

    fn network() -> Vec<f64> {
        use windows::Networking::Connectivity::{NetworkConnectivityLevel, NetworkInformation};
        let Ok(profile) = NetworkInformation::GetInternetConnectionProfile() else { return vec![0.0, 0.0] };
        let connected = profile.GetNetworkConnectivityLevel().is_ok_and(|l| l == NetworkConnectivityLevel::InternetAccess);
        let kind = if profile.IsWlanConnectionProfile().unwrap_or(false) {
            1.0
        } else if profile.IsWwanConnectionProfile().unwrap_or(false) {
            2.0
        } else if profile.NetworkAdapter().and_then(|a| a.IanaInterfaceType()).is_ok_and(|t| t == 6) {
            3.0
        } else {
            4.0
        };
        vec![f64::from(u8::from(connected)), if connected { kind } else { 0.0 }]
    }

    /// Uma posição da localização do Windows (bloqueia até ela responder).
    fn locate() -> windows::core::Result<Vec<f64>> {
        let locator = windows::Devices::Geolocation::Geolocator::new()?;
        let position = locator.GetGeopositionAsync()?.join()?;
        let coordinate = position.Coordinate()?;
        let point = coordinate.Point()?.Position()?;
        let speed = coordinate.Speed().and_then(|s| s.Value()).unwrap_or(0.0);
        Ok(vec![point.Latitude, point.Longitude, coordinate.Accuracy()?, point.Altitude, if speed.is_nan() { 0.0 } else { speed }])
    }
}

#[cfg(windows)]
impl SensorService for DesktopSensors {
    fn available(&self, sensor: Sensor) -> bool {
        match sensor {
            // 128: o sistema não tem bateria; 255: não sabe.
            Sensor::Battery => Self::power().is_some_and(|s| s.BatteryFlag != 128 && s.BatteryFlag != 255),
            Sensor::Network | Sensor::Location => true,
            _ => false,
        }
    }

    fn set_active(&mut self, sensor: Sensor, on: bool) {
        use std::sync::atomic::Ordering;
        if sensor != Sensor::Location || self.locating.swap(on, Ordering::SeqCst) == on || !on {
            return;
        }
        let (flag, slot) = (self.locating.clone(), self.location.clone());
        std::thread::spawn(move || {
            while flag.load(Ordering::SeqCst) {
                match Self::locate() {
                    Ok(p) => {
                        if let Ok(mut s) = slot.lock() {
                            *s = Some(p);
                        }
                    }
                    // Negada nas configurações de privacidade (ou sem localização): fica sem.
                    Err(_) => std::thread::sleep(std::time::Duration::from_secs(4)),
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    fn read(&mut self, sensor: Sensor) -> Option<Vec<f64>> {
        match sensor {
            Sensor::Battery => Self::power().filter(|s| s.BatteryFlag != 128 && s.BatteryFlag != 255).map(|s| {
                let level = if s.BatteryLifePercent == 255 { 0.0 } else { f64::from(s.BatteryLifePercent) / 100.0 };
                vec![level, f64::from(u8::from(s.BatteryFlag & 8 != 0))]
            }),
            Sensor::Network => Some(Self::network()),
            Sensor::Location => self.location.lock().ok()?.clone(),
            _ => None,
        }
    }
}

/// Linux: a bateria (`/sys/class/power_supply`) e a rede (`/sys/class/net`).
#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub(crate) struct DesktopSensors;

#[cfg(all(target_os = "linux", not(target_os = "android")))]
impl DesktopSensors {
    pub(crate) fn new() -> Self {
        Self
    }

    fn battery() -> Option<Vec<f64>> {
        let dir = std::fs::read_dir("/sys/class/power_supply").ok()?.flatten().map(|e| e.path()).find(|p| {
            std::fs::read_to_string(p.join("type")).is_ok_and(|t| t.trim() == "Battery")
        })?;
        let capacity: f64 = std::fs::read_to_string(dir.join("capacity")).ok()?.trim().parse().ok()?;
        let charging = std::fs::read_to_string(dir.join("status")).is_ok_and(|s| s.trim() == "Charging" || s.trim() == "Full");
        Some(vec![capacity / 100.0, f64::from(u8::from(charging))])
    }

    fn network() -> Vec<f64> {
        let up = std::fs::read_dir("/sys/class/net")
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().is_some_and(|n| n != "lo"))
            .filter(|p| std::fs::read_to_string(p.join("operstate")).is_ok_and(|s| s.trim() == "up"))
            .collect::<Vec<_>>();
        match up.iter().find(|p| p.join("wireless").exists()).or(up.first()) {
            Some(p) if p.join("wireless").exists() => vec![1.0, 1.0],
            Some(_) => vec![1.0, 3.0],
            None => vec![0.0, 0.0],
        }
    }
}

#[cfg(all(target_os = "linux", not(target_os = "android")))]
impl SensorService for DesktopSensors {
    fn available(&self, sensor: Sensor) -> bool {
        match sensor {
            Sensor::Battery => Self::battery().is_some(),
            Sensor::Network => true,
            _ => false,
        }
    }

    fn set_active(&mut self, _: Sensor, _: bool) {}

    fn read(&mut self, sensor: Sensor) -> Option<Vec<f64>> {
        match sensor {
            Sensor::Battery => Self::battery(),
            Sensor::Network => Some(Self::network()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A rede desta máquina responde (conectada ou não, com o tipo); a bateria só existe num
    /// notebook.
    #[test]
    fn the_desktop_reads_the_network_and_the_battery() {
        let mut s = DesktopSensors::new();
        let net = s.read(Sensor::Network).unwrap();
        assert_eq!(net.len(), 2);
        println!("rede: {net:?}; bateria: {} {:?}", s.available(Sensor::Battery), s.read(Sensor::Battery));
        assert_eq!(s.available(Sensor::Battery), s.read(Sensor::Battery).is_some());
    }
}

#[cfg(all(test, windows))]
mod location_tests {
    use super::*;

    /// A localização do Windows responde (se liberada nas configurações de privacidade); não
    /// mostra a posição.
    #[test]
    #[ignore]
    fn windows_location_answers() {
        match DesktopSensors::locate() {
            Ok(p) => println!("localização: respondeu, precisão {:.0} m", p[2]),
            Err(e) => println!("localização: {e}"),
        }
    }
}
