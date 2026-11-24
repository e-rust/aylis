//! A área de trabalho fora das janelas do app ([`aylis_core::DesktopService`]): o mouse na tela,
//! o form sob um ponto, os "Efeitos de transparência", a imagem de um monitor, o Caps Lock e
//! o Num Lock, a sessão do usuário (bloqueada, conectada; só o Windows), o início com o sistema
//! (`login.rs`) e a soltura da instância única.
//!
//! - **Windows:** tudo (`GetCursorPos`, a ordem real das janelas, o registro do tema, o GDI).
//! - **macOS:** o mouse e o Caps Lock pelo `NSEvent`; a captura ainda não (pede a permissão de
//!   gravação da tela: `docs/testes-em-dispositivos.md`).
//! - **Linux:** o X11 pela `libX11` carregada na hora (o mouse, o Caps/Num Lock e a imagem do
//!   monitor); no Wayland o sistema não diz onde está o mouse nem deixa capturar sem o portal:
//!   `None` e erro.
//! - **Celular e navegador:** o padrão (`None`; o Caps Lock da web vem da tecla, `web.rs`).

#[cfg(not(any(web, mobile)))]
use aylis_core::{ComponentId, DesktopService, Image};

/// O serviço da área de trabalho deste sistema.
#[cfg(not(any(web, mobile)))]
pub(crate) struct Desktop;

#[cfg(not(any(web, mobile)))]
impl DesktopService for Desktop {
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

/// O Caps Lock e o Num Lock ligados agora (o `Modifiers` de cada tecla e do mouse). No celular,
/// `false`; no navegador, o que a última tecla disse (`web.rs`).
pub(crate) fn lock_keys() -> (bool, bool) {
    #[cfg(windows)]
    return crate::win32::lock_keys();
    #[cfg(target_os = "macos")]
    return (mac::caps_lock(), false);
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    return x11::lock_keys().unwrap_or((false, false));
    #[cfg(web)]
    return crate::web::lock_keys();
    #[allow(unreachable_code)]
    (false, false)
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

    pub(super) fn cursor_position() -> Option<(i32, i32)> {
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

    /// O Caps Lock e o Num Lock (os bits do `LockMask` e do `Mod2Mask` do ponteiro).
    pub(super) fn lock_keys() -> Option<(bool, bool)> {
        with(|lib, display| {
            let (mut root, mut child) = (0, 0);
            let (mut rx, mut ry, mut wx, mut wy, mut mask) = (0, 0, 0, 0, 0u32);
            // SAFETY: idem.
            let ok = unsafe {
                let window = (lib.XDefaultRootWindow)(display);
                (lib.XQueryPointer)(display, window, &mut root, &mut child, &mut rx, &mut ry, &mut wx, &mut wy, &mut mask)
            };
            (ok != 0).then_some((mask & x11_dl::xlib::LockMask != 0, mask & x11_dl::xlib::Mod2Mask != 0))
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
