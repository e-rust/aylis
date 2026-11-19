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
mod records;
mod sensors;
mod system_bars;
mod url_scheme;

pub use actions::{NOTIFICATION_BUTTON, NotificationButton, NotifyAction, PushTrigger, Ringtone, WakeAppAction};
pub use bluetooth::{
    BLUETOOTH_DEVICE, BLUETOOTH_VALUE, Bluetooth, BluetoothDevice, BluetoothDeviceList, BluetoothValue, GATT_CHARACTERISTIC, GattCharacteristic, GattCharacteristicList, PAIRED_DEVICE,
    PairedDevice, parse_hex,
};
pub use device::{
    AppLink, Biometrics, PERMISSION_ANSWER, PERMISSION_REASON, PERMISSION_RESULT, PermissionAnswer, PermissionReason, PermissionResult, Permissions, SHARED_CONTENT, SharedContent,
};
pub use nfc::{NFC_RECORD, Nfc, NfcRecord, NfcRecordList};
pub use aylis_core::platform::json_field;
pub use push::PushNotifications;
pub use system_bars::SystemBars;
pub use url_scheme::UrlScheme;
pub use sensors::{CALL_RECORD, CallRecord, BackgroundService, Battery, Compass, Gamepad, LightSensor, Location, LockScreen, MotionSensor, Network, Phone, ProximitySensor, Screen, Vibration};
