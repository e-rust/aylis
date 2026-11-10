//! Notificações do Windows (toast), pela API do próprio Windows (WinRT
//! `ToastNotificationManager`).
//!
//! Um app fora da loja só aparece na notificação com o nome dele se o AppUserModelID dele
//! estiver registrado para o usuário (`HKCU\Software\Classes\AppUserModelId\<id>`, com
//! `DisplayName`): é o que um instalador faz; aqui o aylis registra na primeira notificação.

use aylis_core::platform::{NotificationService, SystemNotification};
use aylis_core::{ComponentId, EventArgs, Poster};
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::UI::Notifications::{ToastActivatedEventArgs, ToastNotification, ToastNotificationManager};
use windows::core::{IInspectable, Interface};
use windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey, RegCreateKeyExW, RegSetValueExW};
use windows::core::{HSTRING, PCWSTR};

pub(crate) struct WindowsNotifications {
    registered: std::collections::HashSet<String>,
    /// Os toques (no corpo ou num botão) voltam para a thread da interface por aqui.
    poster: Poster,
    /// As notificações mostradas (para atualizar e tirar pelo número).
    shown: std::collections::HashMap<u32, ToastNotification>,
    app_id: String,
}

impl WindowsNotifications {
    pub(crate) fn new(poster: Poster) -> Self {
        Self { registered: Default::default(), poster, shown: Default::default(), app_id: String::new() }
    }
}

/// Repassa o toque para o componente: `on_click` no corpo, `on_action` (ou `on_button`) num botão.
fn deliver(poster: &Poster, component: ComponentId, arguments: String) {
    poster.post(move |ui| {
        if arguments.is_empty() || arguments == "aylis:click" {
            ui.fire(component, "on_click", &EventArgs::None);
        } else {
            // O `Notification` chama `on_action`; as ações (`NotifyAction`, `WakeAppAction`), `on_button`.
            if !ui.fire(component, "on_action", &EventArgs::Text(arguments.clone())) {
                ui.fire(component, "on_button", &EventArgs::Text(arguments));
            }
        }
        ui.process_events();
    });
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Registra o AppUserModelID (nome que aparece na notificação) para o usuário atual.
fn register(app_id: &str, app_name: &str) -> Result<(), String> {
    let path = HSTRING::from(format!("Software\\Classes\\AppUserModelId\\{app_id}"));
    let mut key = HKEY::default();
    // SAFETY: chamadas do registro com ponteiros válidos; a chave é fechada no fim.
    unsafe {
        RegCreateKeyExW(HKEY_CURRENT_USER, &path, None, PCWSTR::null(), REG_OPTION_NON_VOLATILE, KEY_WRITE, None, &mut key, None)
            .ok()
            .map_err(|e| e.message())?;
        let name: Vec<u16> = app_name.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes = std::slice::from_raw_parts(name.as_ptr().cast::<u8>(), name.len() * 2);
        let result = RegSetValueExW(key, &HSTRING::from("DisplayName"), None, REG_SZ, Some(bytes)).ok().map_err(|e| e.message());
        let _ = RegCloseKey(key);
        result
    }
}

impl NotificationService for WindowsNotifications {
    fn notify(&mut self, n: &SystemNotification) -> Result<(), String> {
        if !self.registered.contains(&n.app_id) {
            register(&n.app_id, &n.app_name)?;
            self.registered.insert(n.app_id.clone());
        }
        // A imagem já lida (asset, form desenhado) vai para um arquivo temporário: o toast
        // só lê imagem de arquivo.
        let file = if n.image_png.is_empty() {
            n.image.clone()
        } else {
            let path = std::env::temp_dir().join(format!("aylis-notificacao-{}-{}.png", std::process::id(), n.id));
            std::fs::write(&path, &n.image_png).map_err(|e| format!("notificação: {e}"))?;
            path.display().to_string()
        };
        let image = if file.is_empty() {
            String::new()
        } else {
            let path = std::fs::canonicalize(&file).map(|p| p.display().to_string()).unwrap_or(file);
            let placement = if n.big_image { "hero" } else { "appLogoOverride" };
            format!(r#"<image placement="{placement}" src="file:///{}"/>"#, xml_escape(&path.trim_start_matches(r"\\?\").replace('\\', "/")))
        };
        let progress = n
            .progress
            .map(|p| format!(r#"<progress value="{p:.3}" status="{}"/>"#, xml_escape(&format!("{:.0}%", p * 100.0))))
            .unwrap_or_default();
        let actions: String = n
            .actions
            .iter()
            .map(|(id, text)| format!(r#"<action content="{}" arguments="{}" activationType="foreground"/>"#, xml_escape(text), xml_escape(id)))
            .collect();
        let actions = if actions.is_empty() { actions } else { format!("<actions>{actions}</actions>") };
        let scenario = if n.ongoing { r#" scenario="reminder""# } else { "" };
        let xml = format!(
            r#"<toast launch="aylis:click"{scenario}><visual><binding template="ToastGeneric"><text>{}</text><text>{}</text>{image}{progress}</binding></visual>{actions}</toast>"#,
            xml_escape(&n.title),
            xml_escape(&n.body)
        );
        let err = |e: windows::core::Error| format!("notificação: {}", e.message());
        let doc = XmlDocument::new().map_err(err)?;
        doc.LoadXml(&HSTRING::from(xml)).map_err(err)?;
        let toast = ToastNotification::CreateToastNotification(&doc).map_err(err)?;
        // Mesmo número: substitui a anterior.
        toast.SetTag(&HSTRING::from(n.id.to_string())).map_err(err)?;
        if let Some(component) = n.component {
            let poster = self.poster.clone();
            let handler = TypedEventHandler::<ToastNotification, IInspectable>::new(move |_, args| {
                let arguments = args.as_ref().and_then(|a| a.cast::<ToastActivatedEventArgs>().ok()).and_then(|a| a.Arguments().ok()).map(|s| s.to_string()).unwrap_or_default();
                deliver(&poster, component, arguments);
                Ok(())
            });
            toast.Activated(&handler).map_err(err)?;
        }
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&n.app_id)).map_err(err)?;
        if let Some(old) = self.shown.remove(&n.id) {
            let _ = notifier.Hide(&old);
        }
        notifier.Show(&toast).map_err(err)?;
        self.app_id = n.app_id.clone();
        self.shown.insert(n.id, toast);
        Ok(())
    }

    fn cancel(&mut self, id: u32) -> Result<(), String> {
        let Some(toast) = self.shown.remove(&id) else { return Ok(()) };
        let err = |e: windows::core::Error| format!("notificação: {}", e.message());
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&self.app_id)).map_err(err)?;
        notifier.Hide(&toast).map_err(err)
    }
}
