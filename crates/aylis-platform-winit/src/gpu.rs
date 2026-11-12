//! Desenho na GPU: contexto OpenGL criado com `glutin`, desenho com Skia (Ganesh/GL).
//!
//! No Linux com Wayland, a janela pode desenhar noutra placa que não a do processo (a do
//! monitor, `gpu = monitor`, ou a pedida pelo nome): o contexto passa a ser o de
//! [`crate::gpu_linux::DeviceGl`], e o Skia é refeito nele.

use std::ffi::CString;
use std::num::NonZeroU32;
use std::rc::Rc;

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::prelude::{GlSurface, NotCurrentGlContext, PossiblyCurrentGlContext};
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use aylis_core::Color;
use aylis_skia::gl::GlBackend;
use aylis_skia::{SkiaCanvas, SkiaRenderer};
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::Error;

pub(crate) struct GpuBackend {
    // A ordem importa no drop: o Skia antes do contexto GL. `None` só durante a troca de placa.
    skia: Option<GlBackend>,
    gl: Gl,
    /// O display do glutin (a placa do processo) e a configuração: para voltar a ela.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    display: glutin::display::Display,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    config: Config,
    /// A regra de placa da janela e o nome da placa em que desenha agora.
    choice: aylis_core::gpu::GpuChoice,
    adapter: String,
    /// O `GL_RENDERER` da placa do processo (o display do glutin).
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    default_adapter: String,
    #[cfg(target_os = "linux")]
    feedback: Option<crate::gpu_linux::Feedback>,
}

/// O contexto e a superfície da janela: os do glutin, ou os de uma placa escolhida.
enum Gl {
    Default { surface: Surface<WindowSurface>, context: PossiblyCurrentContext },
    #[cfg(target_os = "linux")]
    Device(crate::gpu_linux::DeviceGl),
}

/// Cria o Skia no contexto atual.
fn new_skia(display: &glutin::display::Display, samples: usize, stencil: usize) -> Result<GlBackend, Error> {
    let mut skia = GlBackend::new(
        |name| {
            // Em EGL o Skia tentaria usar esta função; o glutin já cuida do display.
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            CString::new(name).map_or(std::ptr::null(), |n| display.get_proc_address(&n))
        },
        samples,
        stencil,
    )
    .ok_or("Skia: falha ao criar o contexto OpenGL")?;
    skia.set_gl_loader(gl_loader as *const () as usize);
    Ok(skia)
}

/// O contexto e a superfície do glutin na janela (a placa do processo), já atuais.
fn default_gl(display: &glutin::display::Display, config: &Config, window: &Window) -> Result<Gl, Error> {
    let raw_handle = window.window_handle()?.as_raw();
    let attributes = ContextAttributesBuilder::new().build(Some(raw_handle));
    let fallback = ContextAttributesBuilder::new().with_context_api(ContextApi::Gles(None)).build(Some(raw_handle));
    // SAFETY: a janela (dona do handle) vive mais que o contexto, que fica no mesmo GpuBackend.
    let not_current = unsafe { display.create_context(config, &attributes).or_else(|_| display.create_context(config, &fallback))? };
    let surface_attributes = window.build_surface_attributes(SurfaceAttributesBuilder::new())?;
    // SAFETY: idem.
    let surface = unsafe { display.create_window_surface(config, &surface_attributes)? };
    let context = not_current.make_current(&surface)?;
    // Medições (AYLIS_BENCH): apresenta sem esperar o monitor.
    if crate::diag::diagnostic_setting("AYLIS_BENCH").is_some() {
        let _ = surface.set_swap_interval(&context, glutin::surface::SwapInterval::DontWait);
    }
    Ok(Gl::Default { surface, context })
}

/// A janela e o OpenGL dela, na placa que `choice` pede (no Linux com Wayland; nas outras
/// plataformas, a do processo).
pub(crate) fn create(event_loop: &ActiveEventLoop, attributes: WindowAttributes, choice: aylis_core::gpu::GpuChoice) -> Result<(Rc<Window>, GpuBackend), Error> {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);
    let (window, config) = DisplayBuilder::new()
        .with_window_attributes(Some(attributes))
        .build(event_loop, template, pick_config)?;
    let window = window.ok_or("OpenGL: não foi possível criar a janela")?;
    let display = config.display();
    let gl = default_gl(&display, &config, &window)?;
    if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
        print_gl_info(&display);
    }
    let strings = read_gl_strings(&display);
    if gl_description().is_none() {
        GL_DESCRIPTION.with(|d| *d.borrow_mut() = Some(strings.join(" · ")));
    }
    let skia = new_skia(&display, config.num_samples() as usize, config.stencil_size() as usize)?;
    DISPLAY.with(|d| *d.borrow_mut() = Some(display.clone()));
    let mut backend = GpuBackend {
        skia: Some(skia),
        gl,
        display,
        config,
        choice: aylis_core::gpu::GpuChoice::Auto,
        adapter: strings.get(1).cloned().unwrap_or_default(),
        default_adapter: strings.get(1).cloned().unwrap_or_default(),
        #[cfg(target_os = "linux")]
        feedback: None,
    };
    // A placa do form, se não é a do processo (o primeiro quadro já sai nela).
    if let Err(e) = backend.set_choice(&window, choice) {
        aylis_core::stderr_line!("aylis: a janela fica na placa do processo ({e})");
    }
    Ok((Rc::new(window), backend))
}

thread_local! {
    /// O display do OpenGL, para [`gl_loader`] achar as funções.
    static DISPLAY: std::cell::RefCell<Option<glutin::display::Display>> = const { std::cell::RefCell::new(None) };
}

/// Acha uma função do OpenGL pelo nome (entregue a quem desenha por fora, como o 3D).
unsafe extern "C" fn gl_loader(name: *const std::ffi::c_char) -> *const std::ffi::c_void {
    if name.is_null() {
        return std::ptr::null();
    }
    // SAFETY: quem chama passa uma string C terminada em zero.
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    // `try_with`: o 3D pode pedir uma função quando a thread já está acabando (o app
    // fechando); aí não há mais display, e a resposta é "não tem".
    DISPLAY
        .try_with(|d| d.borrow().as_ref().map_or(std::ptr::null(), |d| d.get_proc_address(name)))
        .unwrap_or(std::ptr::null())
}

thread_local! {
    /// Fabricante, placa e versão do OpenGL da primeira janela (para as gravações de uso).
    static GL_DESCRIPTION: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// `"NVIDIA Corporation · NVIDIA GeForce ... · 4.6.0"`, depois da primeira janela OpenGL.
pub(crate) fn gl_description() -> Option<String> {
    GL_DESCRIPTION.with(|d| d.borrow().clone())
}

/// Lê fabricante, placa e versão com o contexto atual.
fn read_gl_strings(display: &glutin::display::Display) -> Vec<String> {
    let get_string = display.get_proc_address(c"glGetString");
    if get_string.is_null() {
        return Vec::new();
    }
    // SAFETY: glGetString é `const GLubyte* glGetString(GLenum)`, chamado com o contexto atual.
    let f: extern "system" fn(u32) -> *const std::ffi::c_char = unsafe { std::mem::transmute(get_string) };
    [0x1F00, 0x1F01, 0x1F02]
        .into_iter()
        .map(|id| f(id))
        .filter(|p| !p.is_null())
        // SAFETY: o driver devolve uma string terminada em zero.
        .map(|p| unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy().into_owned())
        .collect()
}

/// Diagnóstico (`AYLIS_DEBUG_GL=1`): fabricante, placa e versão do OpenGL em uso.
fn print_gl_info(display: &glutin::display::Display) {
    let get_string = display.get_proc_address(c"glGetString");
    if get_string.is_null() {
        return;
    }
    // SAFETY: glGetString é `const GLubyte* glGetString(GLenum)`, chamado com o contexto atual.
    let f: extern "system" fn(u32) -> *const std::ffi::c_char = unsafe { std::mem::transmute(get_string) };
    for (name, id) in [("fabricante", 0x1F00), ("placa", 0x1F01), ("versão", 0x1F02)] {
        let p = f(id);
        if !p.is_null() {
            // SAFETY: o driver devolve uma string terminada em zero.
            aylis_core::stderr_line!("aylis: OpenGL {name}: {}", unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy());
        }
    }
}

/// Prefere a configuração com menos amostras de MSAA (o Skia já faz antialiasing).
fn pick_config(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|best, c| if c.num_samples() < best.num_samples() { c } else { best })
        .expect("nenhuma configuração OpenGL disponível")
}

/// Fechar uma janela (um diálogo) solta os recursos do Skia dela (texturas, buffers): o
/// contexto dela precisa ser o atual nessa hora. Senão o Skia apaga os objetos por número
/// no contexto da janela que desenhou por último (a principal), e ela passa a desenhar com
/// texturas trocadas ou apagadas: tela vazia ou triângulos de lixo.
impl Drop for GpuBackend {
    fn drop(&mut self) {
        let _ = self.make_current();
        // Os campos (o Skia primeiro) caem depois daqui, com o contexto certo.
    }
}

impl Gl {
    fn make_current(&self) -> Result<(), Error> {
        match self {
            Gl::Default { surface, context } => {
                if !context.is_current() {
                    context.make_current(surface)?;
                }
                Ok(())
            }
            #[cfg(target_os = "linux")]
            Gl::Device(d) => d.make_current(),
        }
    }
}

impl GpuBackend {
    /// Com várias janelas, cada uma tem o seu contexto OpenGL: antes de mexer numa, o
    /// contexto dela precisa ser o atual.
    fn make_current(&self) -> Result<(), Error> {
        self.gl.make_current()
    }

    fn skia(&mut self) -> &mut GlBackend {
        self.skia.as_mut().expect("o Skia só falta durante a troca de placa")
    }

    pub(crate) fn resize(&self, window: &Window) {
        if self.make_current().is_ok() {
            match &self.gl {
                Gl::Default { surface, context } => window.resize_surface(surface, context),
                #[cfg(target_os = "linux")]
                Gl::Device(d) => {
                    let size = window.inner_size();
                    d.resize(size.width, size.height);
                }
            }
        }
    }

    /// A placa em que a janela desenha (o `GL_RENDERER`).
    pub(crate) fn adapter(&self) -> Option<String> {
        (!self.adapter.is_empty()).then(|| self.adapter.clone())
    }

    /// Troca a regra de placa da janela (a propriedade `gpu` do form) e, no Wayland, passa
    /// para a placa pedida. `Ok(true)`: trocou de placa.
    pub(crate) fn set_choice(&mut self, window: &Window, choice: aylis_core::gpu::GpuChoice) -> Result<bool, Error> {
        if choice == self.choice {
            return Ok(false);
        }
        self.choice = choice;
        #[cfg(target_os = "linux")]
        {
            use aylis_core::gpu::GpuChoice;
            if !crate::gpu_linux::is_wayland(window) {
                return Ok(false);
            }
            let wanted = match &self.choice {
                GpuChoice::FollowMonitor => {
                    if self.feedback.is_none() {
                        self.feedback = crate::gpu_linux::Feedback::new(window);
                        if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
                            let state = if self.feedback.is_some() { "ligado" } else { "o compositor não tem o linux-dmabuf v4" };
                            aylis_core::stderr_line!("aylis: feedback da placa preferida: {state}");
                        }
                    }
                    // A placa do monitor chega pelo feedback (`follow_monitor`).
                    return self.follow_monitor(window);
                }
                GpuChoice::Named(_) => crate::gpu_linux::device_named(&self.choice),
                // As outras valem para o processo: a placa padrão.
                _ => None,
            };
            return self.switch_to(window, wanted);
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = window;
            Ok(false)
        }
    }

    /// Com `gpu = monitor` no Wayland: se o compositor passou a preferir outra placa (a
    /// janela foi para um monitor dela), troca. Barato: só olha os eventos já lidos.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn follow_monitor(&mut self, window: &Window) -> Result<bool, Error> {
        #[cfg(target_os = "linux")]
        if self.choice == aylis_core::gpu::GpuChoice::FollowMonitor
            && let Some(dev) = self.feedback.as_mut().and_then(|f| f.poll())
        {
            if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
                aylis_core::stderr_line!("aylis: o compositor prefere a placa {}:{}", dev >> 8 & 0xFFF, dev & 0xFF);
            }
            // Placa que o EGL não enxerga: fica onde está.
            let Some(device) = crate::gpu_linux::device_of(dev) else { return Ok(false) };
            return self.switch_to(window, Some(device));
        }
        let _ = window;
        Ok(false)
    }

    /// Passa a janela para a placa `device` (`None`: a do processo). `Ok(false)`: já estava.
    #[cfg(target_os = "linux")]
    fn switch_to(&mut self, window: &Window, device: Option<glutin::api::egl::device::Device>) -> Result<bool, Error> {
        use crate::gpu_linux::{DeviceGl, default_device, same};
        // A placa padrão também é "a do processo".
        let default = default_device(&self.display, &self.default_adapter);
        if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
            let name = |d: &Option<glutin::api::egl::device::Device>| d.as_ref().map_or("(nenhuma)".to_string(), crate::gpu_linux::describe);
            aylis_core::stderr_line!("aylis: placa pedida {}, a do processo {}", name(&device), name(&default));
        }
        let device = device.filter(|d| default.as_ref().is_none_or(|default| !same(d, default)));
        match (&self.gl, &device) {
            (Gl::Default { .. }, None) => return Ok(false),
            (Gl::Device(current), Some(d)) if current.device == d.raw_device() as *mut std::ffi::c_void => return Ok(false),
            _ => {}
        }
        // O novo primeiro: se não abrir, a janela continua como estava.
        let gl = match &device {
            Some(d) => Gl::Device(DeviceGl::new(&self.display, d, window)?),
            None => default_gl(&self.display, &self.config, window)?,
        };
        // O Skia de antes cai com o contexto dele; depois o contexto, e o novo assume.
        let _ = self.make_current();
        self.skia = None;
        self.gl = gl;
        self.make_current()?;
        let (samples, stencil) = match &self.gl {
            Gl::Device(d) => {
                if crate::diag::diagnostic_setting("AYLIS_BENCH").is_some() {
                    d.dont_wait();
                }
                (d.samples, d.stencil)
            }
            Gl::Default { .. } => (self.config.num_samples() as usize, self.config.stencil_size() as usize),
        };
        self.skia = Some(new_skia(&self.display, samples, stencil)?);
        self.adapter = read_gl_strings(&self.display).get(1).cloned().unwrap_or_default();
        if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
            aylis_core::stderr_line!("aylis: a janela passou a desenhar em {}", self.adapter);
        }
        Ok(true)
    }

    /// A imagem do quadro anterior está guardada neste tamanho: o quadro pode repintar só
    /// uma área.
    pub(crate) fn keeps_image(&self, (width, height): (NonZeroU32, NonZeroU32)) -> bool {
        self.skia.as_ref().is_some_and(|s| s.keeps_image(width.get(), height.get()))
    }

    /// Pinta o quadro; com `parts` (as cópias da rolagem e as áreas, unidades lógicas alinhadas
    /// ao pixel), só elas, por cima da imagem guardada do quadro anterior.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw(
        &mut self,
        renderer: &SkiaRenderer,
        (width, height): (NonZeroU32, NonZeroU32),
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[aylis_core::RectF])>,
        paint: impl FnOnce(&mut SkiaCanvas<'_>, Option<&[aylis_core::RectF]>),
        capture: bool,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.make_current()?;
        if self.skia().render_parts(renderer, width.get(), height.get(), scale, background, parts, paint).is_none() {
            return Err("Skia: falha ao criar a superfície OpenGL".into());
        }
        // Depois do swap o buffer de trás fica indefinido: a captura vem antes.
        let frame = if capture { self.skia().read_frame() } else { None };
        match &self.gl {
            Gl::Default { surface, context } => surface.swap_buffers(context)?,
            #[cfg(target_os = "linux")]
            Gl::Device(d) => d.swap()?,
        }
        Ok(frame)
    }
}

/// Cria um contexto OpenGL numa janela invisível e entrega o [`GlBackend`] para `f`.
/// Serve para renderizar na GPU fora da tela (testes, capturas, comparação com a CPU).
///
/// Usa um `EventLoop` próprio; o winit só permite um por processo, então não chame
/// junto com [`run`](crate::run).
pub fn with_offscreen_gl<R: 'static>(f: impl FnOnce(&mut GlBackend) -> R + 'static) -> Result<R, Error> {
    struct OneShot<F, R> {
        f: Option<F>,
        result: Option<Result<R, Error>>,
    }

    impl<F: FnOnce(&mut GlBackend) -> R, R> ApplicationHandler for OneShot<F, R> {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let Some(f) = self.f.take() else { return };
            let attributes = Window::default_attributes().with_visible(false);
            self.result = Some(create(event_loop, attributes, aylis_core::gpu::GpuChoice::Auto).map(|(_window, mut backend)| f(backend.skia())));
            event_loop.exit();
        }

        fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
    }

    let mut app = OneShot { f: Some(f), result: None };
    EventLoop::new()?.run_app(&mut app)?;
    app.result.unwrap_or_else(|| Err("o sistema não iniciou o aplicativo".into()))
}
