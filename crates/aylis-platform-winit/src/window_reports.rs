//! O que a janela informa ao `Ui` (a escala e o tamanho, para o `on_scale_change` e o
//! `on_resize_end`) e o trazer para a frente ([`WindowCommand::BringToFront`]), com o token de
//! ativação do Wayland.
//!
//! O fim do redimensionamento: no Windows, o `WM_EXITSIZEMOVE` (o usuário soltou a borda);
//! fora dele, e no Windows fora do arrasto (maximizar, encaixar), o núcleo espera
//! [`aylis_core::form_window::RESIZE_END_DELAY_MS`] sem tamanho novo, num timer que só existe
//! enquanto a janela muda de tamanho.

use super::*;

impl App {
    /// A janela `index` mudou de escala ou de tamanho (o `ScaleFactorChanged` e o `Resized`):
    /// o `Ui` fica sabendo. Os pedaços extras de um form em vários monitores não contam (a
    /// janela do form é a principal).
    pub(crate) fn report_window_change(&mut self, index: usize, event: &WindowEvent) {
        let Some(w) = self.windows.get(index).filter(|w| !w.is_extra()) else { return };
        let (form, window) = (w.form, w.window.clone());
        match event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => self.ui.set_form_scale(form, *scale_factor as f32),
            WindowEvent::Resized(size) => {
                // No Windows, com o usuário segurando a borda, o fim é o `WM_EXITSIZEMOVE`.
                #[cfg(windows)]
                let holding = win32::in_size_move(form);
                #[cfg(not(windows))]
                let holding = false;
                // Minimizada (o tamanho 0) não é redimensionar.
                if window.is_minimized() != Some(true) {
                    self.ui.form_resized(form, size.width, size.height, holding);
                }
            }
            _ => {}
        }
    }

    /// A janela nova do form: a escala de agora e, no X11, a janela para o `form_at_screen`.
    pub(crate) fn report_new_window(&mut self, window: &Window, form: ComponentId) {
        self.ui.forget_form_window(form);
        self.ui.set_form_scale(form, window.scale_factor() as f32);
        let size = window.inner_size();
        self.ui.form_resized(form, size.width, size.height, false);
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        crate::desktop::track_x11_window(window, form);
    }

    /// Traz a janela do form para a frente com o foco ([`WindowCommand::BringToFront`]).
    pub(crate) fn bring_window_to_front(&mut self, form: ComponentId) {
        let token = self.ui.take_activation_token().or_else(startup_token);
        let Some(window) = self.window_of(form).map(|w| w.window.clone()) else { return };
        #[cfg(not(any(web, mobile)))]
        if window.is_minimized() == Some(true) {
            window.set_minimized(false);
        }
        #[cfg(windows)]
        {
            let _ = token;
            win32::force_foreground(&window);
        }
        #[cfg(all(target_os = "linux", not(target_os = "android")))]
        if wayland::is_wayland(&window) {
            // Sem token (ou recusado), o pedido de atenção: o winit pede um token da própria
            // janela e ativa com ele, e o compositor decide (a maioria marca a janela).
            if !token.is_some_and(|t| wayland::activate(&window, &t)) {
                window.request_user_attention(Some(winit::window::UserAttentionType::Informational));
            }
            return;
        }
        #[cfg(not(windows))]
        {
            let _ = token;
            window.focus_window();
        }
    }
}

/// O `XDG_ACTIVATION_TOKEN` com que o app abriu (no Wayland, quem abriu o app deu o direito de
/// ativar uma vez), entregue uma vez só.
fn startup_token() -> Option<String> {
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    {
        static TAKEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if TAKEN.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return None;
        }
        return std::env::var("XDG_ACTIVATION_TOKEN").ok().filter(|t| !t.trim().is_empty());
    }
    #[allow(unreachable_code)]
    None
}

/// A ativação do Wayland (`xdg_activation_v1.activate`): o winit só usa um token ao criar a
/// janela, e aqui ele vale para a janela que já existe.
#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod wayland {
    use std::cell::RefCell;
    use std::ffi::c_void;

    use wayland_client::backend::{Backend, ObjectId};
    use wayland_client::protocol::wl_registry::{self, WlRegistry};
    use wayland_client::protocol::wl_surface::WlSurface;
    use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
    use wayland_protocols::xdg::activation::v1::client::xdg_activation_v1::{self, XdgActivationV1};
    use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use winit::window::Window;

    /// O `wl_display` e o `wl_surface` da janela (`None`: não é Wayland).
    fn handles(window: &Window) -> Option<(*mut c_void, *mut c_void)> {
        let RawDisplayHandle::Wayland(display) = window.display_handle().ok()?.as_raw() else { return None };
        let RawWindowHandle::Wayland(surface) = window.window_handle().ok()?.as_raw() else { return None };
        Some((display.display.as_ptr(), surface.surface.as_ptr()))
    }

    pub(super) fn is_wayland(window: &Window) -> bool {
        handles(window).is_some()
    }

    #[derive(Default)]
    struct State {
        activation: Option<XdgActivationV1>,
    }

    /// A conexão sobre o `wl_display` do winit e a fila própria, abertas uma vez (a fila fica
    /// viva enquanto o registro existir: os avisos dele chegam nela).
    struct Link {
        conn: Connection,
        queue: EventQueue<State>,
        state: State,
    }

    thread_local! {
        static LINK: RefCell<Option<Option<Link>>> = const { RefCell::new(None) };
    }

    fn open(display: *mut c_void) -> Option<Link> {
        // SAFETY: o `wl_display` é o do winit e vive enquanto o app roda.
        let conn = Connection::from_backend(unsafe { Backend::from_foreign_display(display.cast()) });
        let mut queue = conn.new_event_queue();
        let qh = queue.handle();
        let mut state = State::default();
        conn.display().get_registry(&qh, ());
        queue.roundtrip(&mut state).ok()?;
        state.activation.as_ref()?;
        Some(Link { conn, queue, state })
    }

    /// Ativa a janela com o token. `false`: o compositor não tem o `xdg-activation`.
    pub(super) fn activate(window: &Window, token: &str) -> bool {
        let Some((display, surface)) = handles(window) else { return false };
        LINK.with(|link| {
            let mut link = link.borrow_mut();
            let Some(link) = link.get_or_insert_with(|| open(display)).as_mut() else { return false };
            let _ = link.queue.dispatch_pending(&mut link.state);
            let Some(activation) = link.state.activation.as_ref() else { return false };
            // SAFETY: o `wl_surface` é o da janela, viva enquanto o winit a guarda (e o pedido é
            // feito agora, com ela aberta).
            let Ok(id) = (unsafe { ObjectId::from_ptr(WlSurface::interface(), surface.cast()) }) else { return false };
            let Ok(surface) = WlSurface::from_id(&link.conn, id) else { return false };
            activation.activate(token.to_string(), &surface);
            link.conn.flush().is_ok()
        })
    }

    impl Dispatch<WlRegistry, ()> for State {
        fn event(state: &mut Self, registry: &WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
            if let wl_registry::Event::Global { name, interface, .. } = event
                && interface == XdgActivationV1::interface().name
            {
                state.activation = Some(registry.bind(name, 1, qh, ()));
            }
        }
    }

    impl Dispatch<XdgActivationV1, ()> for State {
        fn event(_: &mut Self, _: &XdgActivationV1, _: xdg_activation_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
    }
}
