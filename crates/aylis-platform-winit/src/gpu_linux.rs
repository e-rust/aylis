//! Linux (Wayland): a placa de vídeo de cada janela.
//!
//! O glutin abre o OpenGL na placa padrão do processo. Para a janela desenhar em outra (a do
//! monitor em que está, com `gpu = monitor`, ou a pedida pelo nome), aqui se abre um
//! `EGLDisplay` do Wayland naquela placa (`EGL_EXT_explicit_device`), com o contexto e a
//! superfície da janela nele. Quem diz a placa do monitor é o compositor, pelo feedback do
//! `linux-dmabuf` v4: a placa preferida da superfície muda quando ela passa para um monitor
//! ligado a outra placa. No X11 não há o que fazer (o servidor desenha a tela inteira numa
//! placa só): nada daqui liga.

use std::ffi::{CStr, c_void};
use std::os::unix::fs::MetadataExt;

use glutin::api::egl::device::Device;
use glutin::display::GlDisplay;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use wayland_client::backend::{Backend, ObjectId};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use wayland_protocols::wp::linux_dmabuf::zv1::client::zwp_linux_dmabuf_feedback_v1::{self, ZwpLinuxDmabufFeedbackV1};
use wayland_protocols::wp::linux_dmabuf::zv1::client::zwp_linux_dmabuf_v1::{self, ZwpLinuxDmabufV1};
use winit::window::Window;

use crate::Error;

type EglDisplay = *mut c_void;
type EglConfig = *mut c_void;
type EglContext = *mut c_void;
type EglSurface = *mut c_void;

const EGL_NONE: i32 = 0x3038;
const EGL_PLATFORM_WAYLAND_KHR: u32 = 0x31D8;
const EGL_DEVICE_EXT: isize = 0x322C;
const EGL_SURFACE_TYPE: i32 = 0x3033;
const EGL_WINDOW_BIT: i32 = 0x0004;
const EGL_RENDERABLE_TYPE: i32 = 0x3040;
const EGL_OPENGL_BIT: i32 = 0x0008;
const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
const EGL_RED_SIZE: i32 = 0x3024;
const EGL_GREEN_SIZE: i32 = 0x3023;
const EGL_BLUE_SIZE: i32 = 0x3022;
const EGL_ALPHA_SIZE: i32 = 0x3021;
const EGL_STENCIL_SIZE: i32 = 0x3026;
const EGL_SAMPLES: i32 = 0x3031;
const EGL_OPENGL_API: u32 = 0x30A2;
const EGL_OPENGL_ES_API: u32 = 0x30A0;
const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;

/// As funções do EGL usadas aqui, pedidas ao `eglGetProcAddress` (o mesmo `libEGL` do glutin).
struct Egl {
    get_platform_display: unsafe extern "C" fn(u32, *mut c_void, *const isize) -> EglDisplay,
    initialize: unsafe extern "C" fn(EglDisplay, *mut i32, *mut i32) -> u32,
    bind_api: unsafe extern "C" fn(u32) -> u32,
    choose_config: unsafe extern "C" fn(EglDisplay, *const i32, *mut EglConfig, i32, *mut i32) -> u32,
    get_config_attrib: unsafe extern "C" fn(EglDisplay, EglConfig, i32, *mut i32) -> u32,
    create_context: unsafe extern "C" fn(EglDisplay, EglConfig, EglContext, *const i32) -> EglContext,
    create_window_surface: unsafe extern "C" fn(EglDisplay, EglConfig, *mut c_void, *const isize) -> EglSurface,
    make_current: unsafe extern "C" fn(EglDisplay, EglSurface, EglSurface, EglContext) -> u32,
    swap_buffers: unsafe extern "C" fn(EglDisplay, EglSurface) -> u32,
    swap_interval: unsafe extern "C" fn(EglDisplay, i32) -> u32,
    destroy_surface: unsafe extern "C" fn(EglDisplay, EglSurface) -> u32,
    destroy_context: unsafe extern "C" fn(EglDisplay, EglContext) -> u32,
    get_current_context: unsafe extern "C" fn() -> EglContext,
    get_error: unsafe extern "C" fn() -> i32,
}

impl Egl {
    fn load(display: &glutin::display::Display) -> Option<Self> {
        macro_rules! f {
            ($name:literal) => {{
                let p = display.get_proc_address(CStr::from_bytes_with_nul(concat!($name, "\0").as_bytes()).ok()?);
                if p.is_null() {
                    return None;
                }
                // SAFETY: o ponteiro é a função do EGL com este nome, na assinatura da
                // especificação (EGL 1.5).
                unsafe { std::mem::transmute::<*const c_void, _>(p) }
            }};
        }
        Some(Self {
            get_platform_display: f!("eglGetPlatformDisplay"),
            initialize: f!("eglInitialize"),
            bind_api: f!("eglBindAPI"),
            choose_config: f!("eglChooseConfig"),
            get_config_attrib: f!("eglGetConfigAttrib"),
            create_context: f!("eglCreateContext"),
            create_window_surface: f!("eglCreatePlatformWindowSurface"),
            make_current: f!("eglMakeCurrent"),
            swap_buffers: f!("eglSwapBuffers"),
            swap_interval: f!("eglSwapInterval"),
            destroy_surface: f!("eglDestroySurface"),
            destroy_context: f!("eglDestroyContext"),
            get_current_context: f!("eglGetCurrentContext"),
            get_error: f!("eglGetError"),
        })
    }

    fn error(&self, what: &str) -> Error {
        // SAFETY: só lê o último erro da thread.
        format!("EGL: {what} (erro 0x{:X})", unsafe { (self.get_error)() }).into()
    }
}

/// O `wl_display` e o `wl_surface` da janela (`None`: não é Wayland).
fn wayland_handles(window: &Window) -> Option<(*mut c_void, *mut c_void)> {
    let RawDisplayHandle::Wayland(display) = window.display_handle().ok()?.as_raw() else { return None };
    let RawWindowHandle::Wayland(surface) = window.window_handle().ok()?.as_raw() else { return None };
    Some((display.display.as_ptr(), surface.surface.as_ptr()))
}

/// A janela está no Wayland (só lá a placa muda por janela).
pub(crate) fn is_wayland(window: &Window) -> bool {
    wayland_handles(window).is_some()
}

/// O contexto OpenGL de uma janela numa placa escolhida.
pub(crate) struct DeviceGl {
    egl: Egl,
    display: EglDisplay,
    context: EglContext,
    surface: EglSurface,
    native: *mut wayland_sys::egl::wl_egl_window,
    /// A placa (o `EGLDeviceEXT`), para saber se já está nela.
    pub(crate) device: *mut c_void,
    pub(crate) samples: usize,
    pub(crate) stencil: usize,
}

impl DeviceGl {
    /// Abre o EGL do Wayland na placa `device` e o contexto e a superfície da janela nele
    /// (ainda sem torná-lo o atual).
    pub(crate) fn new(glutin_display: &glutin::display::Display, device: &Device, window: &Window) -> Result<Self, Error> {
        let egl = Egl::load(glutin_display).ok_or("EGL: funções não encontradas")?;
        let (wl_display, wl_surface) = wayland_handles(window).ok_or("a janela não é do Wayland")?;
        let wegl = wayland_sys::egl::wayland_egl_option().ok_or("libwayland-egl não encontrada")?;
        let size = window.inner_size();
        // SAFETY: chamadas do EGL e do libwayland-egl com o display e a superfície da janela,
        // que vive mais que este contexto (os dois ficam no mesmo backend).
        unsafe {
            let attrs = [EGL_DEVICE_EXT, device.raw_device() as isize, EGL_NONE as isize];
            let display = (egl.get_platform_display)(EGL_PLATFORM_WAYLAND_KHR, wl_display, attrs.as_ptr());
            if display.is_null() || (egl.initialize)(display, std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
                return Err(egl.error("o display da placa não abriu"));
            }
            // OpenGL; sem ele, OpenGL ES (como o glutin faz).
            let mut found = None;
            for (api, bit, attrs) in [(EGL_OPENGL_API, EGL_OPENGL_BIT, vec![EGL_NONE]), (EGL_OPENGL_ES_API, EGL_OPENGL_ES2_BIT, vec![EGL_CONTEXT_CLIENT_VERSION, 3, EGL_NONE])] {
                if (egl.bind_api)(api) == 0 {
                    continue;
                }
                let Some(config) = pick_config(&egl, display, bit) else { continue };
                let context = (egl.create_context)(display, config, std::ptr::null_mut(), attrs.as_ptr());
                if !context.is_null() {
                    found = Some((config, context));
                    break;
                }
            }
            let (config, context) = found.ok_or_else(|| egl.error("nenhum contexto OpenGL na placa"))?;
            let native = (wegl.wl_egl_window_create)(wl_surface.cast(), size.width.max(1) as i32, size.height.max(1) as i32);
            let surface = (egl.create_window_surface)(display, config, native.cast(), [EGL_NONE as isize].as_ptr());
            if native.is_null() || surface.is_null() {
                let error = egl.error("a superfície da janela não abriu");
                (egl.destroy_context)(display, context);
                if !native.is_null() {
                    (wegl.wl_egl_window_destroy)(native);
                }
                return Err(error);
            }
            let attrib = |name| {
                let mut v = 0;
                (egl.get_config_attrib)(display, config, name, &mut v);
                v.max(0) as usize
            };
            let (samples, stencil) = (attrib(EGL_SAMPLES), attrib(EGL_STENCIL_SIZE));
            Ok(Self { egl, display, context, surface, native, device: device.raw_device() as *mut c_void, samples, stencil })
        }
    }

    pub(crate) fn is_current(&self) -> bool {
        // SAFETY: só lê o contexto atual da thread.
        unsafe { (self.egl.get_current_context)() == self.context }
    }

    pub(crate) fn make_current(&self) -> Result<(), Error> {
        // SAFETY: o display, a superfície e o contexto são deste objeto e estão vivos.
        if self.is_current() || unsafe { (self.egl.make_current)(self.display, self.surface, self.surface, self.context) } != 0 {
            Ok(())
        } else {
            Err(self.egl.error("eglMakeCurrent falhou"))
        }
    }

    pub(crate) fn swap(&self) -> Result<(), Error> {
        // SAFETY: idem.
        if unsafe { (self.egl.swap_buffers)(self.display, self.surface) } != 0 { Ok(()) } else { Err(self.egl.error("eglSwapBuffers falhou")) }
    }

    /// Sem esperar o monitor (as medições).
    pub(crate) fn dont_wait(&self) {
        // SAFETY: com o contexto atual.
        unsafe { (self.egl.swap_interval)(self.display, 0) };
    }

    pub(crate) fn resize(&self, width: u32, height: u32) {
        if let Some(wegl) = wayland_sys::egl::wayland_egl_option() {
            // SAFETY: a janela EGL é deste objeto.
            unsafe { (wegl.wl_egl_window_resize)(self.native, width.max(1) as i32, height.max(1) as i32, 0, 0) };
        }
    }
}

impl Drop for DeviceGl {
    fn drop(&mut self) {
        // SAFETY: solta o que este objeto criou; o Skia já caiu (o backend solta antes). O
        // display fica: o EGL devolve o mesmo para a mesma placa, e outra janela pode usá-lo.
        unsafe {
            if self.is_current() {
                (self.egl.make_current)(self.display, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
            }
            (self.egl.destroy_surface)(self.display, self.surface);
            (self.egl.destroy_context)(self.display, self.context);
            if let Some(wegl) = wayland_sys::egl::wayland_egl_option() {
                (wegl.wl_egl_window_destroy)(self.native);
            }
        }
    }
}

/// Uma configuração RGBA8 de janela, com menos amostras de MSAA (o Skia já faz antialiasing).
///
/// # Safety
/// `display` inicializado.
unsafe fn pick_config(egl: &Egl, display: EglDisplay, renderable: i32) -> Option<EglConfig> {
    let attrs = [
        EGL_SURFACE_TYPE, EGL_WINDOW_BIT, EGL_RENDERABLE_TYPE, renderable, EGL_RED_SIZE, 8, EGL_GREEN_SIZE, 8, EGL_BLUE_SIZE, 8, EGL_ALPHA_SIZE, 8, EGL_NONE,
    ];
    let mut configs = [std::ptr::null_mut(); 64];
    let mut count = 0;
    // SAFETY: a lista cabe no vetor (o tamanho vai junto).
    if unsafe { (egl.choose_config)(display, attrs.as_ptr(), configs.as_mut_ptr(), configs.len() as i32, &mut count) } == 0 {
        return None;
    }
    configs[..count.max(0) as usize].iter().copied().min_by_key(|&c| {
        let mut samples = 0;
        // SAFETY: a configuração veio do próprio display.
        unsafe { (egl.get_config_attrib)(display, c, EGL_SAMPLES, &mut samples) };
        samples
    })
}

/// As placas que o EGL enxerga (`EGL_EXT_device_enumeration`).
pub(crate) fn devices() -> Vec<Device> {
    Device::query_devices().map(|d| d.collect()).unwrap_or_default()
}

/// O renderizador por software do Mesa (o llvmpipe).
fn is_software(device: &Device) -> bool {
    device.extensions().contains("EGL_MESA_device_software")
}

/// O nome da placa para escolher e mostrar.
pub(crate) fn describe(device: &Device) -> String {
    if is_software(device) {
        return "llvmpipe (software)".into();
    }
    let path = device.drm_render_device_node_path().or(device.drm_device_node_path()).map(|p| p.display().to_string());
    [device.vendor().map(str::to_string), device.name().map(str::to_string), path].into_iter().flatten().collect::<Vec<_>>().join(" ")
}

/// A placa do nome pedido (parte do nome, do fabricante ou do nó, como `renderD129`).
pub(crate) fn device_named(choice: &aylis_core::gpu::GpuChoice) -> Option<Device> {
    devices().into_iter().find(|d| choice.matches_name(&describe(d)))
}

/// A placa do `dev_t` que o compositor manda (o nó principal ou o de renderização).
pub(crate) fn device_of(dev: u64) -> Option<Device> {
    devices().into_iter().find(|d| {
        [d.drm_device_node_path(), d.drm_render_device_node_path()].into_iter().flatten().any(|p| std::fs::metadata(p).is_ok_and(|m| m.rdev() == dev))
    })
}

/// A placa do display padrão do glutin (a do processo), quando dá para saber. `renderer`: o
/// `GL_RENDERER` dele. Um driver sem nó DRM (o D3D12 do WSL) diz que a placa dele é a de
/// software; aí ela só vale se o OpenGL for mesmo o llvmpipe.
pub(crate) fn default_device(display: &glutin::display::Display, renderer: &str) -> Option<Device> {
    let device = match display {
        glutin::display::Display::Egl(d) => d.device().ok()?,
        #[allow(unreachable_patterns)]
        _ => return None,
    };
    (!is_software(&device) || renderer.contains("llvmpipe")).then_some(device)
}

pub(crate) fn same(a: &Device, b: &Device) -> bool {
    a.raw_device() == b.raw_device()
}

/// O feedback do `linux-dmabuf` v4 da superfície da janela: a placa que o compositor prefere
/// para ela (a do monitor em que está). Uma fila própria na conexão do winit: os eventos
/// chegam quando o winit lê o socket, e [`Feedback::poll`] só os entrega.
pub(crate) struct Feedback {
    queue: EventQueue<FeedbackState>,
    state: FeedbackState,
    _feedback: ZwpLinuxDmabufFeedbackV1,
}

#[derive(Default)]
pub(crate) struct FeedbackState {
    dmabuf: Option<ZwpLinuxDmabufV1>,
    main: Option<u64>,
    /// A placa da primeira faixa (a de maior preferência) do lote em andamento.
    first_target: Option<u64>,
    preferred: Option<u64>,
    changed: bool,
}

impl Feedback {
    /// `None`: não é Wayland, ou o compositor não tem o `linux-dmabuf` v4.
    pub(crate) fn new(window: &Window) -> Option<Self> {
        let (display, surface) = wayland_handles(window)?;
        // SAFETY: o `wl_display` é o do winit e vive enquanto o app roda.
        let conn = Connection::from_backend(unsafe { Backend::from_foreign_display(display.cast()) });
        let mut queue = conn.new_event_queue();
        let qh = queue.handle();
        let mut state = FeedbackState::default();
        conn.display().get_registry(&qh, ());
        queue.roundtrip(&mut state).ok()?;
        let dmabuf = state.dmabuf.clone()?;
        // SAFETY: o `wl_surface` é o da janela, vivo enquanto ela existe (e este objeto é dela).
        let id = unsafe { ObjectId::from_ptr(WlSurface::interface(), surface.cast()) }.ok()?;
        let surface = WlSurface::from_id(&conn, id).ok()?;
        let feedback = dmabuf.get_surface_feedback(&surface, &qh, ());
        let _ = conn.flush();
        Some(Self { queue, state, _feedback: feedback })
    }

    /// A placa preferida (`dev_t`), se mudou desde a última vez. Barato: só entrega o que o
    /// winit já leu.
    pub(crate) fn poll(&mut self) -> Option<u64> {
        let _ = self.queue.dispatch_pending(&mut self.state);
        std::mem::take(&mut self.state.changed).then_some(self.state.preferred).flatten()
    }
}

fn dev_of(bytes: &[u8]) -> Option<u64> {
    Some(u64::from_ne_bytes(bytes.try_into().ok()?))
}

impl Dispatch<WlRegistry, ()> for FeedbackState {
    fn event(state: &mut Self, registry: &WlRegistry, event: wl_registry::Event, _: &(), _: &Connection, qh: &QueueHandle<Self>) {
        if let wl_registry::Event::Global { name, interface, version } = event
            && interface == ZwpLinuxDmabufV1::interface().name
            && version >= 4
        {
            state.dmabuf = Some(registry.bind(name, 4, qh, ()));
        }
    }
}

impl Dispatch<ZwpLinuxDmabufV1, ()> for FeedbackState {
    fn event(_: &mut Self, _: &ZwpLinuxDmabufV1, _: zwp_linux_dmabuf_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<ZwpLinuxDmabufFeedbackV1, ()> for FeedbackState {
    fn event(state: &mut Self, _: &ZwpLinuxDmabufFeedbackV1, event: zwp_linux_dmabuf_feedback_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        use zwp_linux_dmabuf_feedback_v1::Event;
        match event {
            Event::MainDevice { device } => state.main = dev_of(&device),
            Event::TrancheTargetDevice { device } => {
                if state.first_target.is_none() {
                    state.first_target = dev_of(&device);
                }
            }
            Event::Done => {
                let preferred = state.first_target.take().or(state.main);
                if preferred != state.preferred {
                    state.preferred = preferred;
                    state.changed = true;
                }
            }
            // A tabela de formatos (um arquivo aberto): fechada ao sair daqui.
            _ => {}
        }
    }
}
