//! Windows: notificações push pelo WNS (Windows Push Notification Services). O token é o
//! endereço do canal (`https://*.notify.windows.com/...`), para onde o servidor do app envia.
//! O Windows só dá o canal a um app com identidade de pacote (MSIX); fora dele, o registro
//! responde no `on_error`. Com o app aberto, as mensagens chegam pelo evento do canal: a do
//! tipo raw (o texto em [`push_message_from_payload`]) e a toast; o componente mostra as
//! duas com o `show_in_foreground`.

use std::sync::{Arc, Mutex};

use aylis_core::platform::{PushConfig, PushEvent, PushMessage, PushService, push_event, push_message_from_payload};
use windows::Foundation::TypedEventHandler;
use windows::Networking::PushNotifications::{PushNotificationChannel, PushNotificationChannelManager, PushNotificationReceivedEventArgs, PushNotificationType};

#[derive(Default)]
pub(crate) struct WnsPush {
    channel: Arc<Mutex<Option<PushNotificationChannel>>>,
}

/// O app roda com identidade de pacote (MSIX)?
fn packaged() -> bool {
    let mut len = 0u32;
    // Sem pacote: APPMODEL_ERROR_NO_PACKAGE (15700); com pacote e sem buffer: ERROR_INSUFFICIENT_BUFFER.
    let r = unsafe { windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName(&mut len, None) };
    r.0 != 15700
}

/// O título e o texto de uma toast (os dois primeiros `<text>` do XML).
fn toast_message(args: &PushNotificationReceivedEventArgs) -> Option<PushMessage> {
    let toast = args.ToastNotification().ok()?;
    let texts = toast.Content().ok()?.GetElementsByTagName(&"text".into()).ok()?;
    let text = |i: u32| texts.Item(i).ok().and_then(|n| n.InnerText().ok()).map(|s| s.to_string()).unwrap_or_default();
    let launch = toast.Content().ok()?.DocumentElement().ok().and_then(|e| e.GetAttribute(&"launch".into()).ok()).map(|s| s.to_string()).unwrap_or_default();
    // O `launch` da toast, quando é JSON, são os dados.
    let data = if launch.trim_start().starts_with('{') { launch } else { "{}".into() };
    Some(PushMessage { id: String::new(), title: text(0), body: text(1), data })
}

fn received(args: &PushNotificationReceivedEventArgs) {
    let message = match args.NotificationType() {
        Ok(PushNotificationType::Raw) => args.RawNotification().and_then(|r| r.Content()).ok().map(|c| push_message_from_payload(&c.to_string())),
        Ok(PushNotificationType::Toast) => {
            // Com o app aberto, quem mostra é o componente (com o `show_in_foreground`).
            let _ = args.SetCancel(true);
            toast_message(args)
        }
        _ => None,
    };
    if let Some(m) = message {
        push_event(PushEvent::Message(m));
    }
}

impl PushService for WnsPush {
    fn available(&self) -> bool {
        packaged()
    }

    fn provider(&self) -> &'static str {
        "wns"
    }

    fn register(&mut self, _config: &PushConfig) {
        if !packaged() {
            push_event(PushEvent::Error(aylis_core::messages::t("o WNS só registra o app empacotado (MSIX)")));
            return;
        }
        let slot = self.channel.clone();
        std::thread::spawn(move || {
            let result = (|| -> windows::core::Result<String> {
                let channel = PushNotificationChannelManager::CreatePushNotificationChannelForApplicationAsync()?.join()?;
                channel.PushNotificationReceived(&TypedEventHandler::<PushNotificationChannel, PushNotificationReceivedEventArgs>::new(|_, args| {
                    if let Some(args) = args.as_ref() {
                        received(args);
                    }
                    Ok(())
                }))?;
                let uri = channel.Uri()?.to_string();
                if let Ok(mut s) = slot.lock() {
                    *s = Some(channel);
                }
                Ok(uri)
            })();
            push_event(match result {
                Ok(uri) => PushEvent::Token(uri),
                Err(e) => PushEvent::Error(format!("WNS: {}", e.message())),
            });
        });
    }

    fn unregister(&mut self) {
        if let Some(channel) = self.channel.lock().ok().and_then(|mut c| c.take()) {
            let _ = channel.Close();
        }
    }

    // Com o app aberto, nenhuma aparece sozinha: o componente mostra.
    fn shows_notifications(&self) -> bool {
        false
    }
}
