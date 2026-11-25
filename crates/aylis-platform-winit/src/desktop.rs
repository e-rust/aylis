//! A área de trabalho fora das janelas do app ([`aylis_core::DesktopService`]): o mouse na tela,
//! o form sob um ponto, os "Efeitos de transparência", a imagem de um monitor, o Caps Lock e
//! o Num Lock, a sessão do usuário (bloqueada, conectada; só o Windows), o início com o sistema
//! (`login.rs`) e a soltura da instância única.
//!
//! - **Windows:** tudo (`GetCursorPos`, a ordem real das janelas, o registro do tema, o GDI).
//! - **macOS:** o mouse e o Caps Lock pelo `NSEvent`; a captura ainda não (pede a permissão de
//!   gravação da tela: `docs/testes-em-dispositivos.md`).
//! - **Linux:** o X11 pela `libX11` carregada na hora (o mouse, o form sob um ponto, o
//!   Caps/Num Lock e a imagem do monitor); numa sessão Wayland o sistema não diz onde está o
//!   mouse (o XWayland só sabe dele sobre as janelas X) nem deixa capturar sem o portal: `None`
//!   e erro.
//! - **Celular e navegador:** o padrão (`None`; o Caps Lock da web vem da tecla, `web.rs`).

#[cfg(not(any(web, mobile)))]
use aylis_core::{ComponentId, DesktopService, Image};

/// O serviço da área de trabalho deste sistema.
#[cfg(not(any(web, mobile)))]
pub(crate) struct Desktop;

#[cfg(not(any(web, mobile)))]
impl DesktopService for Desktop {
    fn lock_keys(&self) -> Option<aylis_core::LockKeys> {
        Some(lock_keys())
    }

    fn cursor_position(&self) -> Option<(i32, i32)> {
        #[cfg(windows)]
        return crate::win32::cursor_pos();
        #[cfg(target_os = "macos")]
        return mac::cursor_position();
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        return x11::cursor_position();
        #[allow(unreachable_code)]
        None
    }

    fn form_at(&self, x: i32, y: i32) -> Option<ComponentId> {
        #[cfg(windows)]
        return crate::win32::form_at(x, y);
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        return x11::form_at(x, y);
        #[allow(unreachable_code)]
        {
            let _ = (x, y);
            None
        }
    }

    fn transparency(&self) -> Option<bool> {
        #[cfg(windows)]
        return crate::win32::transparency_effects();
        #[allow(unreachable_code)]
        None
    }

    fn capture_monitor(&mut self, area: (i32, i32, u32, u32), max_width: u32, done: Box<dyn FnOnce(Result<Image, String>) + Send>) {
        // Numa thread: o GDI copia a tela inteira (4K leva dezenas de milissegundos).
        let spawned = std::thread::Builder::new().name("aylis-capture".into()).spawn(move || {
            #[cfg(windows)]
            let result = crate::win32::capture_screen(area, max_width);
            #[cfg(all(target_os = "linux", not(target_os = "android")))]
            let result = x11::capture(area, max_width);
            #[cfg(not(any(windows, all(target_os = "linux", not(target_os = "android")))))]
            let result: Result<Image, String> = {
                let _ = (area, max_width);
                Err(aylis_core::messages::t("A captura da tela não existe neste sistema."))
            };
            done(result);
        });
        if let Err(e) = spawned {
            aylis_core::stderr_line!("aylis: a captura da tela não começou ({e})");
        }
    }

    fn session_state(&self) -> Option<(bool, bool)> {
        #[cfg(windows)]
        return crate::win32::session_state();
        #[allow(unreachable_code)]
        None
    }

    fn start_with_login(&self) -> bool {
        crate::login::start_with_login()
    }

    fn set_start_with_login(&mut self, on: bool) -> Result<(), String> {
        crate::login::set_start_with_login(on)
    }

    fn release_single_instance(&mut self) {
        crate::single_instance::release_single_instance();
    }
}

/// O Caps Lock, o Num Lock e o Scroll Lock ligados agora (o `Modifiers` de cada tecla e do
/// mouse, e o `Ui::lock_keys`). No celular, desligados; no navegador, o que a última tecla disse
/// (`web.rs`); no Wayland, o do XWayland quando há `DISPLAY` (o winit não dá as travas do
/// `wl_keyboard`), e sem ele, desligados.
pub(crate) fn lock_keys() -> aylis_core::LockKeys {
    #[cfg(windows)]
    return crate::win32::lock_keys();
    #[cfg(target_os = "macos")]
    return aylis_core::LockKeys { caps_lock: mac::caps_lock(), ..Default::default() };
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    return x11::lock_keys().unwrap_or_default();
    #[cfg(web)]
    return crate::web::lock_keys();
    #[allow(unreachable_code)]
    aylis_core::LockKeys::default()
}

/// A janela nova do form no X11 (no Wayland, nada): o `form_at_screen` acha o form por ela.
#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub(crate) fn track_x11_window(window: &winit::window::Window, form: ComponentId) {
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    let id = match window.window_handle().map(|h| h.as_raw()) {
        Ok(RawWindowHandle::Xlib(h)) => h.window as u64,
        Ok(RawWindowHandle::Xcb(h)) => u64::from(h.window.get()),
        _ => return,
    };
    x11::track(form, id);
}

/// A redução de uma área `width × height` para no máximo `max_width` de largura (0: inteira),
/// mantendo a proporção.
#[cfg_attr(not(any(windows, all(target_os = "linux", not(target_os = "android")))), allow(dead_code))]
pub(crate) fn scaled(width: u32, height: u32, max_width: u32) -> (u32, u32) {
    if max_width == 0 || width <= max_width || width == 0 {
        return (width.max(1), height.max(1));
    }
    let h = (u64::from(height) * u64::from(max_width) / u64::from(width)).max(1) as u32;
    (max_width, h)
}

#[cfg(target_os = "macos")]
mod mac {
    use objc2::runtime::AnyClass;
    use objc2::{msg_send, sel};
    use objc2_foundation::{NSPoint, NSRect};

    /// O mouse em pixels físicos, com a origem no canto de cima à esquerda do monitor principal
    /// (o `mouseLocation` é em pontos, com a origem embaixo).
    pub(super) fn cursor_position() -> Option<(i32, i32)> {
        let _ = sel!(mouseLocation);
        let event = AnyClass::get("NSEvent")?;
        let screen = AnyClass::get("NSScreen")?;
        // SAFETY: métodos de classe do AppKit sem argumentos, com os tipos de retorno da documentação;
        // o `mainScreen` pode ser nulo (sem monitor) e é conferido.
        unsafe {
            let p: NSPoint = msg_send![event, mouseLocation];
            let screens: *mut objc2::runtime::AnyObject = msg_send![screen, screens];
            if screens.is_null() {
                return None;
            }
            let first: *mut objc2::runtime::AnyObject = msg_send![screens, firstObject];
            if first.is_null() {
                return None;
            }
            let frame: NSRect = msg_send![first, frame];
            let scale: f64 = msg_send![first, backingScaleFactor];
            Some(((p.x * scale).round() as i32, ((frame.size.height - p.y) * scale).round() as i32))
        }
    }

    /// O Caps Lock ligado (`NSEventModifierFlagCapsLock`).
    pub(super) fn caps_lock() -> bool {
        let Some(event) = AnyClass::get("NSEvent") else { return false };
        // SAFETY: método de classe do AppKit, sem argumentos, que devolve um inteiro.
        let flags: usize = unsafe { msg_send![event, modifierFlags] };
        flags & (1 << 16) != 0
    }
}

#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod x11 {
    use std::sync::OnceLock;

    use x11_dl::xlib::{Display, Xlib};

    /// A `libX11` e uma conexão própria (o winit não empresta a dele), abertas uma vez. No
    /// Wayland sem `DISPLAY`, nada.
    struct X {
        lib: Xlib,
        display: usize,
    }

    // SAFETY: a conexão só é usada com o `XLockDisplay` das próprias chamadas abaixo, que vêm da
    // thread da tela e da thread da captura (o `XInitThreads` é chamado antes de abrir).
    unsafe impl Send for X {}
    unsafe impl Sync for X {}

    fn x() -> Option<&'static X> {
        static X: OnceLock<Option<X>> = OnceLock::new();
        X.get_or_init(|| {
            if std::env::var_os("DISPLAY").is_none() {
                return None;
            }
            let lib = Xlib::open().ok()?;
            // SAFETY: abre a conexão padrão (`DISPLAY`); nulo é conferido.
            unsafe {
                (lib.XInitThreads)();
                let display = (lib.XOpenDisplay)(std::ptr::null());
                (!display.is_null()).then_some(X { lib, display: display as usize })
            }
        })
        .as_ref()
    }

    fn with<T>(f: impl FnOnce(&Xlib, *mut Display) -> T) -> Option<T> {
        let x = x()?;
        let display = x.display as *mut Display;
        // SAFETY: a conexão vive até o fim do processo; o lock a protege entre as threads.
        unsafe {
            (x.lib.XLockDisplay)(display);
            let out = f(&x.lib, display);
            (x.lib.XUnlockDisplay)(display);
            Some(out)
        }
    }

    /// A sessão é Wayland (`WAYLAND_DISPLAY`, ou o `XDG_SESSION_TYPE`): o XWayland só sabe do
    /// mouse sobre as janelas X, e sobre as nativas (as do app também) a posição dele fica
    /// parada na última que viu. Lá, o mouse e o form sob um ponto ficam sem valor.
    pub(super) fn wayland_session() -> bool {
        std::env::var_os("WAYLAND_DISPLAY").is_some_and(|d| !d.is_empty()) || std::env::var("XDG_SESSION_TYPE").is_ok_and(|t| t.eq_ignore_ascii_case("wayland"))
    }

    pub(super) fn cursor_position() -> Option<(i32, i32)> {
        if wayland_session() {
            return None;
        }
        with(|lib, display| {
            let (mut root, mut child) = (0, 0);
            let (mut rx, mut ry, mut wx, mut wy, mut mask) = (0, 0, 0, 0, 0);
            // SAFETY: a conexão é válida e os ponteiros são das variáveis acima.
            let ok = unsafe {
                let window = (lib.XDefaultRootWindow)(display);
                (lib.XQueryPointer)(display, window, &mut root, &mut child, &mut rx, &mut ry, &mut wx, &mut wy, &mut mask)
            };
            (ok != 0).then_some((rx, ry))
        })
        .flatten()
    }

    thread_local! {
        /// A janela X (o id do servidor) de cada form aberto no X11.
        static WINDOWS: std::cell::RefCell<Vec<(aylis_core::ComponentId, u64)>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    /// Guarda a janela X do form (a do winit; a anterior do mesmo form sai).
    pub(super) fn track(form: aylis_core::ComponentId, window: u64) {
        WINDOWS.with(|w| {
            let mut w = w.borrow_mut();
            w.retain(|(f, _)| *f != form);
            w.push((form, window));
        });
    }

    /// O form cuja janela está à vista no ponto (pixels da tela): desce da janela raiz pela
    /// filha mapeada que contém o ponto (o servidor dá a de cima, na ordem real), passando pela
    /// moldura do gerenciador, até achar uma janela do app. Minimizada não é mapeada, e outro
    /// programa por cima dá a janela dele: `None`.
    pub(super) fn form_at(x: i32, y: i32) -> Option<aylis_core::ComponentId> {
        if wayland_session() {
            return None;
        }
        let ours = WINDOWS.with(|w| w.borrow().clone());
        if ours.is_empty() {
            return None;
        }
        with(|lib, display| {
            // SAFETY: a conexão é válida; as janelas são só ids do servidor (uma que deixou de
            // existir faz o `XTranslateCoordinates` falhar, e a busca para).
            unsafe {
                let root = (lib.XDefaultRootWindow)(display);
                let mut current = root;
                for _ in 0..32 {
                    let (mut dx, mut dy, mut child) = (0, 0, 0);
                    if (lib.XTranslateCoordinates)(display, root, current, x, y, &mut dx, &mut dy, &mut child) == 0 || child == 0 {
                        return None;
                    }
                    if let Some((form, _)) = ours.iter().find(|(_, w)| *w == child as u64) {
                        return Some(*form);
                    }
                    current = child;
                }
                None
            }
        })
        .flatten()
    }

    /// O Caps Lock e o Num Lock (os bits do `LockMask` e do `Mod2Mask` do ponteiro) e o Scroll
    /// Lock (a luz 3 do teclado no `XGetKeyboardControl`: o Scroll Lock não é modificador no X).
    pub(super) fn lock_keys() -> Option<aylis_core::LockKeys> {
        with(|lib, display| {
            let (mut root, mut child) = (0, 0);
            let (mut rx, mut ry, mut wx, mut wy, mut mask) = (0, 0, 0, 0, 0u32);
            // SAFETY: idem; o `XKeyboardState` é preenchido pelo servidor.
            let (ok, leds) = unsafe {
                let window = (lib.XDefaultRootWindow)(display);
                let ok = (lib.XQueryPointer)(display, window, &mut root, &mut child, &mut rx, &mut ry, &mut wx, &mut wy, &mut mask);
                let mut keyboard: x11_dl::xlib::XKeyboardState = std::mem::zeroed();
                (lib.XGetKeyboardControl)(display, &mut keyboard);
                (ok, keyboard.led_mask)
            };
            (ok != 0).then_some(aylis_core::LockKeys {
                caps_lock: mask & x11_dl::xlib::LockMask != 0,
                num_lock: mask & x11_dl::xlib::Mod2Mask != 0,
                scroll_lock: leds & 4 != 0,
            })
        })
        .flatten()
    }

    /// A área da tela pelo `XGetImage` da janela raiz, reduzida pelo ponto mais perto.
    pub(super) fn capture(area: (i32, i32, u32, u32), max_width: u32) -> Result<aylis_core::Image, String> {
        let failed = || aylis_core::messages::t("A captura da tela não existe neste sistema.");
        with(|lib, display| {
            let (x, y, width, height) = area;
            // SAFETY: a imagem é lida pelos campos dela e liberada com o `XDestroyImage`.
            unsafe {
                let window = (lib.XDefaultRootWindow)(display);
                let image = (lib.XGetImage)(display, window, x, y, width, height, !0, x11_dl::xlib::ZPixmap);
                if image.is_null() {
                    return Err(failed());
                }
                let im = &*image;
                // Só o formato comum de 32 bits por pixel (BGRX); outro, o erro.
                let bits = im.bits_per_pixel;
                let (w, h) = super::scaled(width, height, max_width);
                let mut out = vec![0u8; w as usize * h as usize * 4];
                if bits == 32 {
                    for ty in 0..h {
                        let sy = (u64::from(ty) * u64::from(height) / u64::from(h)) as i32;
                        for tx in 0..w {
                            let sx = (u64::from(tx) * u64::from(width) / u64::from(w)) as i32;
                            let p = im.data.add(sy as usize * im.bytes_per_line as usize + sx as usize * 4) as *const u8;
                            let o = (ty as usize * w as usize + tx as usize) * 4;
                            out[o] = *p.add(2);
                            out[o + 1] = *p.add(1);
                            out[o + 2] = *p;
                            out[o + 3] = 255;
                        }
                    }
                }
                (lib.XDestroyImage)(image);
                if bits == 32 { Ok(aylis_core::Image::rgba(w, h, out)) } else { Err(failed()) }
            }
        })
        .unwrap_or_else(|| Err(failed()))
    }
}
