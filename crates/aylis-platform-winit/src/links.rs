//! O `LinkService` do desktop e do navegador (o componente `AppLink`): abrir endereços no
//! programa padrão e outros programas. O Android tem o dele, pela `AylisActivity`
//! (`android_device`).

use aylis_core::platform::LinkService;

/// Windows, macOS e Linux: o programa padrão do sistema para cada endereço.
#[cfg(not(any(web, mobile)))]
pub(crate) struct DesktopLinks;

#[cfg(not(any(web, mobile)))]
impl LinkService for DesktopLinks {
    fn open_url(&mut self, url: &str) -> Result<(), String> {
        open(url, false)
    }

    fn open_app(&mut self, app: &str) -> Result<(), String> {
        open(app, true)
    }
}

/// O link que abriu o app pela linha de comando (o `deep_links` registrado no sistema passa o
/// endereço como argumento): vai para o `on_link` do `AppLink`.
#[cfg(not(any(web, mobile)))]
pub(crate) fn command_line_links() {
    for arg in std::env::args().skip(1).filter(|a| a.contains("://")) {
        aylis_core::platform::push_incoming(aylis_core::platform::Incoming::Link(arg));
    }
}

/// O `ShellExecuteW` abre endereços, documentos e programas registrados (`app`: o nome sem
/// caminho também vale, pelo App Paths do registro).
#[cfg(windows)]
fn open(target: &str, _app: bool) -> Result<(), String> {
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
    use windows::core::{HSTRING, w};
    let result = unsafe { ShellExecuteW(None, w!("open"), &HSTRING::from(target), None, None, SW_SHOWNORMAL) };
    // Acima de 32 deu certo (a convenção do ShellExecute).
    if result.0 as isize > 32 { Ok(()) } else { Err(format!("{target}: não abriu (código {})", result.0 as isize)) }
}

#[cfg(all(not(windows), not(any(web, mobile))))]
fn open(target: &str, app: bool) -> Result<(), String> {
    let mut command = if cfg!(target_os = "macos") {
        let mut c = std::process::Command::new("open");
        if app {
            c.arg("-a");
        }
        c.arg(target);
        c
    } else if app {
        std::process::Command::new(target)
    } else {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(target);
        c
    };
    command.spawn().map(|_| ()).map_err(|e| format!("{target}: {e}"))
}

/// Navegador: o endereço relativo (outra página do mesmo site) abre na mesma aba; o de fora,
/// noutra. Compartilhar usa o `navigator.share` (celular e alguns desktops).
#[cfg(web)]
pub(crate) struct WebLinks;

#[cfg(web)]
impl LinkService for WebLinks {
    fn open_url(&mut self, url: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("sem window")?;
        let outside = url.contains(':');
        let result = if outside { window.open_with_url_and_target(url, "_blank").map(|_| ()) } else { window.location().assign(url) };
        result.map_err(|e| format!("{url}: {e:?}"))
    }

    fn share(&mut self, text: &str, files: &[String]) -> Result<(), String> {
        if !files.is_empty() {
            return Err("o navegador compartilha só texto por aqui".into());
        }
        let navigator = web_sys::window().ok_or("sem window")?.navigator();
        let share = js_sys::Reflect::get(&navigator, &"share".into()).map_err(|_| "sem navigator.share")?;
        let share: js_sys::Function = share.dyn_into().map_err(|_| "este navegador não compartilha (navigator.share)")?;
        let data = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&data, &"text".into(), &text.into());
        share.call1(&navigator, &data).map(|_| ()).map_err(|e| format!("{e:?}"))
    }
}

#[cfg(web)]
use wasm_bindgen::JsCast;
