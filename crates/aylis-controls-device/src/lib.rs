//! # aylis-controls-device
//!
//! Os componentes do aparelho: sensores, bateria, rede, permissões, biometria, [`AppLink`], [`UrlScheme`],
//! [`Nfc`], [`Bluetooth`], [`PushNotifications`] e [`SystemBars`]. Os apps usam pelo `aylis-controls`, que
//! registra as classes.

mod actions;
mod bluetooth;
mod device;
mod nfc;
mod push;
mod sensors;
mod system_bars;
mod url_scheme;

pub use actions::{NotifyAction, PushTrigger, Ringtone, WakeAppAction};
pub use bluetooth::{Bluetooth, parse_hex};
pub use device::{AppLink, Biometrics, Permissions};
pub use nfc::Nfc;
pub use aylis_core::platform::json_field;
pub use push::PushNotifications;
pub use system_bars::SystemBars;
pub use url_scheme::UrlScheme;
pub use sensors::{BackgroundService, Battery, Compass, Gamepad, LightSensor, Location, LockScreen, MotionSensor, Network, Phone, ProximitySensor, Screen, Vibration};
