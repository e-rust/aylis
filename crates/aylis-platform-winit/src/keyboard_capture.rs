//! A captura das teclas do sistema (o `KeyboardCapture`, decidido em 24/11): enquanto a janela
//! de um form que pede está em primeiro plano, o Windows/Super, o Alt+Tab, o Ctrl+Esc e os
//! outros atalhos do sistema vão para ela. Ao perder o foco, ao desligar o pedido ou ao sair,
//! solta tudo.
//!
//! - **Windows:** o gancho `WH_KEYBOARD_LL`, instalado só enquanto a janela está em foco. Ele
//!   engole a tecla do sistema e a entrega à janela como `WM_KEYDOWN`/`WM_KEYUP` comuns (não os
//!   `WM_SYS*`, que o `DefWindowProc` transformaria no menu da janela ou no fechar). O
//!   Ctrl+Alt+Del e o Win+L ficam com o sistema (ele não deixa).
//! - **X11:** o `XGrabKeyboard` na conexão do winit (a da janela: um grab é do cliente que o
//!   pede), com `owner_events`, para as teclas chegarem pelo caminho de sempre.
//! - **Wayland:** o `zwp_keyboard_shortcuts_inhibit_manager_v1` (o compositor pode perguntar ao
//!   usuário, como o GNOME, ou recusar), numa fila própria na conexão do winit. O GNOME só
//!   lembra a resposta de um app com o `.desktop` instalado com o nome do `app_id` da janela
//!   (sem ele, pergunta a cada abertura; o Aylis avisa uma vez no stderr).
//! - **macOS, celular e navegador:** nada (o `available` do componente é falso).

use aylis_core::ComponentId;
use winit::window::{Window, WindowId};

use crate::App;

/// Linux: o identificador do app (o `app_id` do Wayland e o `WM_CLASS` do X11): o
/// `Options::app_id` ou, vazio, o nome do executável (o mesmo nome do `.desktop`).
#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub(crate) fn linux_app_id(app_id: &str) -> Option<String> {
    Some(app_id.to_string()).filter(|id| !id.is_empty()).or_else(|| std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string())))
}

/// O teclado preso agora: a janela, o form e o que foi pedido.
pub(crate) struct Held {
    form: ComponentId,
    window: WindowId,
    alt_f4: bool,
    grab: Grab,
}

impl App {
    /// Prende o teclado na janela em foco, se o form dela pede, e solta quando não pede mais
    /// (ou perdeu o foco). Chamado a cada `about_to_wait`: barato (só o foco das janelas).
    pub(crate) fn sync_keyboard_capture(&mut self) {
        let wanted = self
            .windows
            .iter()
            .filter(|w| !w.is_extra())
            .find(|w| w.window.has_focus())
            .and_then(|w| self.ui.keyboard_capture(w.form).map(|r| (w.form, w.window.id(), r.alt_f4)));
        let same = match (&self.keyboard_held, wanted) {
            (Some(h), Some((form, window, alt_f4))) => h.form == form && h.window == window && h.alt_f4 == alt_f4,
            (None, None) => true,
            _ => false,
        };
        if same {
            return;
        }
        self.release_keyboard();
        if let Some((form, id, alt_f4)) = wanted {
            let app_id = &self.options.app_id;
            let grab = self.windows.iter().find(|w| w.window.id() == id).and_then(|w| Grab::take(&w.window, alt_f4, app_id));
            // Sem o grab (outro programa prendeu, o compositor recusou), tenta de novo no
            // próximo `about_to_wait`.
            if let Some(grab) = grab {
                self.keyboard_held = Some(Held { form, window: id, alt_f4, grab });
                self.ui.set_keyboard_captured(Some(form));
            }
        }
    }

    /// Solta o teclado (ao sair, ao perder o foco, ao desligar o pedido).
    pub(crate) fn release_keyboard(&mut self) {
        if let Some(held) = self.keyboard_held.take() {
            held.grab.release();
            self.ui.set_keyboard_captured(None);
        }
    }
}

/// O que cada sistema guarda enquanto o teclado está preso.
pub(crate) enum Grab {
    #[cfg(windows)]
    Hook(win::Hook),
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    X11(x11::Grab),
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    Wayland(wayland::Inhibitor),
    /// Para o `match` não ficar vazio onde não há captura.
    #[allow(dead_code)]
    None,
}

impl Grab {
    #[allow(unused_variables, unreachable_code)]
    fn take(window: &Window, alt_f4: bool, app_id: &str) -> Option<Self> {
        #[cfg(windows)]
        return win::Hook::install(window, alt_f4).map(Grab::Hook);
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        if let Some(inhibitor) = wayland::Inhibitor::new(window, app_id) {
            return Some(Grab::Wayland(inhibitor));
        }
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        return x11::Grab::take(window).map(Grab::X11);
        None
    }

    fn release(self) {
        match self {
            #[cfg(windows)]
            Grab::Hook(hook) => hook.remove(),
            #[cfg(all(target_os = "linux", not(target_os = "android")))]
            Grab::X11(grab) => grab.release(),
            #[cfg(all(target_os = "linux", not(target_os = "android")))]
            Grab::Wayland(inhibitor) => drop(inhibitor),
            Grab::None => {}
        }
    }
}

#[cfg(windows)]
mod win {
    use std::cell::Cell;

    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_SNAPSHOT, VK_SPACE, VK_TAB,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetForegroundWindow, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, LLKHF_EXTENDED, LLKHF_INJECTED, LLKHF_UP, PostMessageW,
        SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    };
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use winit::window::Window;

    thread_local! {
        /// A janela que recebe as teclas e se o Alt+F4 vai junto (o gancho roda nesta thread,
        /// a da tela, que instalou).
        static TARGET: Cell<(isize, bool)> = const { Cell::new((0, false)) };
    }

    pub(crate) struct Hook(HHOOK);

    impl Hook {
        pub(super) fn install(window: &Window, alt_f4: bool) -> Option<Self> {
            let RawWindowHandle::Win32(handle) = window.window_handle().ok()?.as_raw() else { return None };
            TARGET.set((handle.hwnd.get(), alt_f4));
            // SAFETY: o gancho de teclado de baixo nível é global (thread 0) e roda nesta thread,
            // que tem o laço de mensagens do winit; o módulo é o do executável.
            let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(proc), GetModuleHandleW(None).ok().map(Into::into), 0) }.ok()?;
            Some(Self(hook))
        }

        pub(super) fn remove(self) {
            TARGET.set((0, false));
            // SAFETY: o gancho é o que este objeto instalou.
            unsafe {
                let _ = UnhookWindowsHookEx(self.0);
            }
        }
    }

    /// A tecla é das que o sistema guarda para si (com o Alt e o Ctrl de agora).
    pub(super) fn is_system_key(vk: VIRTUAL_KEY, alt: bool, ctrl: bool, alt_f4: bool) -> bool {
        match vk {
            VK_LWIN | VK_RWIN | VK_SNAPSHOT => true,
            VK_TAB | VK_SPACE => alt,
            VK_ESCAPE => alt || ctrl,
            VK_F4 => alt && alt_f4,
            _ => false,
        }
    }

    /// O `lParam` do `WM_KEYDOWN`/`WM_KEYUP`: uma repetição, o scancode, a tecla estendida e,
    /// no soltar, o estado anterior e a transição.
    pub(super) fn key_lparam(scan: u32, extended: bool, up: bool) -> isize {
        let mut l: u32 = 1 | ((scan & 0xFF) << 16);
        if extended {
            l |= 1 << 24;
        }
        if up {
            l |= (1 << 30) | (1 << 31);
        }
        l as i32 as isize
    }

    unsafe extern "system" fn proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HC_ACTION as i32 {
            // SAFETY: no `HC_ACTION` o `lParam` do gancho de baixo nível é um `KBDLLHOOKSTRUCT`.
            let k = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
            let (hwnd, alt_f4) = TARGET.get();
            // As teclas injetadas (teclados na tela, automação) passam como estão.
            let injected = k.flags.0 & LLKHF_INJECTED.0 != 0;
            // SAFETY: só lê a janela em primeiro plano e o estado do Ctrl.
            let (foreground, ctrl) = unsafe { (GetForegroundWindow().0 as isize, GetAsyncKeyState(VK_CONTROL.0 as i32) < 0) };
            let alt = k.flags.0 & LLKHF_ALTDOWN.0 != 0;
            if hwnd != 0 && !injected && foreground == hwnd && is_system_key(VIRTUAL_KEY(k.vkCode as u16), alt, ctrl, alt_f4) {
                let up = k.flags.0 & LLKHF_UP.0 != 0;
                let message = if up { WM_KEYUP } else { WM_KEYDOWN };
                let l = key_lparam(k.scanCode, k.flags.0 & LLKHF_EXTENDED.0 != 0, up);
                // SAFETY: a janela é a do app; a mensagem vai para a fila dela.
                let _ = unsafe { PostMessageW(Some(HWND(hwnd as *mut _)), message, WPARAM(k.vkCode as usize), LPARAM(l)) };
                return LRESULT(1);
            }
        }
        // SAFETY: repassa ao próximo gancho com os mesmos argumentos.
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn only_the_system_keys_are_taken() {
            assert!(is_system_key(VK_LWIN, false, false, false));
            assert!(is_system_key(VK_SNAPSHOT, false, false, false));
            assert!(is_system_key(VK_TAB, true, false, false), "Alt+Tab");
            assert!(!is_system_key(VK_TAB, false, false, false), "o Tab sozinho é do app, como sempre");
            assert!(is_system_key(VK_ESCAPE, false, true, false), "Ctrl+Esc");
            assert!(!is_system_key(VK_ESCAPE, false, false, false));
            assert!(!is_system_key(VK_F4, true, false, false), "o Alt+F4 fecha, sem o `alt_f4`");
            assert!(is_system_key(VK_F4, true, false, true));
            assert!(!is_system_key(VIRTUAL_KEY(b'R' as u16), false, false, true));
        }

        #[test]
        fn the_lparam_carries_the_scancode_and_the_release() {
            // A tecla Windows esquerda: scancode 0x5B, estendida.
            let down = key_lparam(0x5B, true, false) as u32;
            assert_eq!(down & 0xFFFF, 1);
            assert_eq!((down >> 16) & 0xFF, 0x5B);
            assert_ne!(down & (1 << 24), 0);
            assert_eq!(down & (1 << 31), 0);
            let up = key_lparam(0x5B, true, true) as u32;
            assert_ne!(up & (1 << 30), 0);
            assert_ne!(up & (1 << 31), 0);
        }
    }
}

#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod x11 {
    use std::sync::OnceLock;

    use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use winit::window::Window;
    use x11_dl::xlib::{Display, Xlib};

    fn lib() -> Option<&'static Xlib> {
        static LIB: OnceLock<Option<Xlib>> = OnceLock::new();
        LIB.get_or_init(|| Xlib::open().ok()).as_ref()
    }

    /// O teclado preso na conexão do winit.
    pub(crate) struct Grab {
        display: *mut Display,
    }

    impl Grab {
        pub(super) fn take(window: &Window) -> Option<Self> {
            let RawDisplayHandle::Xlib(display) = window.display_handle().ok()?.as_raw() else { return None };
            let RawWindowHandle::Xlib(handle) = window.window_handle().ok()?.as_raw() else { return None };
            let display = display.display?.as_ptr().cast::<Display>();
            let lib = lib()?;
            // SAFETY: a conexão e a janela são as do winit, vivas enquanto a janela existe, e
            // usadas na thread da tela, a mesma do winit.
            let status = unsafe {
                let status = (lib.XGrabKeyboard)(display, handle.window, x11_dl::xlib::True, x11_dl::xlib::GrabModeAsync, x11_dl::xlib::GrabModeAsync, x11_dl::xlib::CurrentTime);
                (lib.XFlush)(display);
                status
            };
            (status == x11_dl::xlib::GrabSuccess).then_some(Self { display })
        }

        pub(super) fn release(self) {
            let Some(lib) = lib() else { return };
            // SAFETY: a conexão é a do winit, aberta até o fim do app.
            unsafe {
                (lib.XUngrabKeyboard)(self.display, x11_dl::xlib::CurrentTime);
                (lib.XFlush)(self.display);
            }
        }
    }
}

#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod wayland {
    use wayland_client::backend::{Backend, ObjectId};
    use wayland_client::protocol::wl_registry::{self, WlRegistry};
    use wayland_client::protocol::wl_seat::{self, WlSeat};
    use wayland_client::protocol::wl_surface::WlSurface;
    use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
    use wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::client::zwp_keyboard_shortcuts_inhibit_manager_v1::{self, ZwpKeyboardShortcutsInhibitManagerV1};
    use wayland_protocols::wp::keyboard_shortcuts_inhibit::zv1::client::zwp_keyboard_shortcuts_inhibitor_v1::{self, ZwpKeyboardShortcutsInhibitorV1};
    use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use winit::window::Window;

    #[derive(Default)]
    pub(crate) struct State {
        manager: Option<ZwpKeyboardShortcutsInhibitManagerV1>,
        seat: Option<WlSeat>,
    }

    /// O pedido ao compositor: vale enquanto este objeto vive.
    pub(crate) struct Inhibitor {
        conn: Connection,
        _queue: EventQueue<State>,
        manager: ZwpKeyboardShortcutsInhibitManagerV1,
        inhibitor: ZwpKeyboardShortcutsInhibitorV1,
    }

    impl Inhibitor {
        /// `None`: não é Wayland, ou o compositor não tem o protocolo.
        pub(crate) fn new(window: &Window, app_id: &str) -> Option<Self> {
            let RawDisplayHandle::Wayland(display) = window.display_handle().ok()?.as_raw() else { return None };
            let RawWindowHandle::Wayland(surface) = window.window_handle().ok()?.as_raw() else { return None };
            let (display, surface) = (display.display.as_ptr(), surface.surface.as_ptr());
            // SAFETY: o `wl_display` é o do winit e vive enquanto o app roda.
            let conn = Connection::from_backend(unsafe { Backend::from_foreign_display(display.cast()) });
            let mut queue = conn.new_event_queue();
            let qh = queue.handle();
            let mut state = State::default();
            conn.display().get_registry(&qh, ());
            queue.roundtrip(&mut state).ok()?;
            let (manager, seat) = (state.manager.clone()?, state.seat.clone()?);
            // SAFETY: o `wl_surface` é o da janela, vivo enquanto ela existe.
            let id = unsafe { ObjectId::from_ptr(WlSurface::interface(), surface.cast()) }.ok()?;
            let surface = WlSurface::from_id(&conn, id).ok()?;
            let inhibitor = manager.inhibit_shortcuts(&surface, &seat, &qh, ());
            let _ = conn.flush();
            warn_without_desktop_file(app_id);
            Some(Self { conn, _queue: queue, manager, inhibitor })
        }
    }

    /// O GNOME pergunta se o app pode inibir os atalhos e só guarda a resposta de um app que
    /// ele reconhece: a janela tem de bater com um `.desktop` instalado com o nome do `app_id`
    /// (o `Options::app_id`; vazio, o nome do executável). Rodando sem ele (o app compilado e
    /// não instalado), a pergunta volta a cada vez: avisa uma vez, com o caminho que resolve.
    /// No Flatpak quem identifica o app é o sandbox (`FLATPAK_ID`), e lá não há o que avisar.
    fn warn_without_desktop_file(app_id: &str) {
        static WARNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if std::env::var_os("FLATPAK_ID").is_some() || WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let Some(id) = super::linux_app_id(app_id) else { return };
        if desktop_file(&id).is_none() {
            aylis_core::stderr_line!(
                "aylis: o KeyboardCapture pediu ao compositor para inibir os atalhos, mas não há um '{id}.desktop' instalado (em ~/.local/share/applications ou /usr/share/applications): o GNOME vai perguntar de novo a cada abertura. Publique com o instalador ou com o atalho no menu (menu_shortcut), ou grave um .desktop com esse nome."
            );
        }
    }

    /// O `.desktop` de `id` nas pastas de dados do XDG (`XDG_DATA_HOME`, depois `XDG_DATA_DIRS`).
    fn desktop_file(id: &str) -> Option<std::path::PathBuf> {
        let home = std::env::var_os("XDG_DATA_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".local/share")));
        let dirs = std::env::var("XDG_DATA_DIRS").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        let dirs: Vec<std::path::PathBuf> = home.into_iter().chain(dirs.split(':').filter(|d| !d.is_empty()).map(std::path::PathBuf::from)).collect();
        desktop_file_in(&dirs, id)
    }

    /// O primeiro `<pasta>/applications/<id>.desktop` que existe.
    fn desktop_file_in(dirs: &[std::path::PathBuf], id: &str) -> Option<std::path::PathBuf> {
        dirs.iter().map(|d| d.join("applications").join(format!("{id}.desktop"))).find(|p| p.is_file())
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn the_desktop_file_is_found_by_the_app_id() {
            let root = std::env::temp_dir().join(format!("aylis-desktop-{}", std::process::id()));
            let (first, second) = (root.join("a"), root.join("b"));
            std::fs::create_dir_all(second.join("applications")).unwrap();
            std::fs::write(second.join("applications/meu-app.desktop"), "[Desktop Entry]\n").unwrap();
            let dirs = [first, second.clone()];
            assert_eq!(super::desktop_file_in(&dirs, "meu-app"), Some(second.join("applications/meu-app.desktop")));
            assert_eq!(super::desktop_file_in(&dirs, "outro"), None);
            let _ = std::fs::remove_dir_all(&root);
        }
    }

    impl Drop for Inhibitor {
        fn drop(&mut self) {
            self.inhibitor.destroy();
            self.manager.destroy();
            let _ = self.conn.flush();
        }
    }

    impl Dispatch<WlRegistry, ()> for State {
        fn event(state: &mut Self, registry: &WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
            if let wl_registry::Event::Global { name, interface, .. } = event {
                if interface == ZwpKeyboardShortcutsInhibitManagerV1::interface().name {
                    state.manager = Some(registry.bind(name, 1, qh, ()));
                } else if interface == WlSeat::interface().name && state.seat.is_none() {
                    state.seat = Some(registry.bind(name, 1, qh, ()));
                }
            }
        }
    }

    impl Dispatch<WlSeat, ()> for State {
        fn event(_: &mut Self, _: &WlSeat, _: wl_seat::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
    }

    impl Dispatch<ZwpKeyboardShortcutsInhibitManagerV1, ()> for State {
        fn event(_: &mut Self, _: &ZwpKeyboardShortcutsInhibitManagerV1, _: zwp_keyboard_shortcuts_inhibit_manager_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
    }

    impl Dispatch<ZwpKeyboardShortcutsInhibitorV1, ()> for State {
        fn event(_: &mut Self, _: &ZwpKeyboardShortcutsInhibitorV1, _: zwp_keyboard_shortcuts_inhibitor_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
    }
}
