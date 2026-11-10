//! # aylis-controls-device
//!
//! Os componentes do aparelho: sensores, bateria, rede, permissões, biometria, [`AppLink`],
//! [`Nfc`], [`Bluetooth`] e [`PushNotifications`]. Os apps usam pelo `aylis-controls`, que
//! registra as classes.

mod actions;
mod bluetooth;
mod device;
mod nfc;
mod push;
mod sensors;

pub use actions::{NotifyAction, PushTrigger, Ringtone, WakeAppAction};
pub use bluetooth::{Bluetooth, parse_hex};
pub use device::{AppLink, Biometrics, Permissions};
pub use nfc::Nfc;
pub use aylis_core::platform::json_field;
pub use push::PushNotifications;
pub use sensors::{BackgroundService, Battery, Compass, Gamepad, LightSensor, Location, LockScreen, MotionSensor, Network, Phone, ProximitySensor, Screen, Vibration};
