//! Desenho na GPU: contexto OpenGL criado com `glutin`, desenho com Skia (Ganesh/GL).

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
    // A ordem importa no drop: o Skia antes do contexto GL.
    skia: GlBackend,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

pub(crate) fn create(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Result<(Rc<Window>, GpuBackend), Error> {
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);
    let (window, config) = DisplayBuilder::new()
        .with_window_attributes(Some(attributes))
        .build(event_loop, template, pick_config)?;
    let window = window.ok_or("OpenGL: não foi possível criar a janela")?;

    let raw_handle = window.window_handle()?.as_raw();
    let display = config.display();
    let attributes = ContextAttributesBuilder::new().build(Some(raw_handle));
    let fallback = ContextAttributesBuilder::new().with_context_api(ContextApi::Gles(None)).build(Some(raw_handle));
    // SAFETY: a janela (dona do handle) vive mais que o contexto, que fica no mesmo GpuBackend.
    let not_current = unsafe {
        display
            .create_context(&config, &attributes)
            .or_else(|_| display.create_context(&config, &fallback))?
    };
    let surface_attributes = window.build_surface_attributes(SurfaceAttributesBuilder::new())?;
    // SAFETY: idem.
    let gl_surface = unsafe { display.create_window_surface(&config, &surface_attributes)? };
    let gl_context = not_current.make_current(&gl_surface)?;
    // Medições (AYLIS_BENCH): apresenta sem esperar o monitor.
    if crate::diag::diagnostic_setting("AYLIS_BENCH").is_some() {
        let _ = gl_surface.set_swap_interval(&gl_context, glutin::surface::SwapInterval::DontWait);
    }
    if crate::diag::diagnostic_setting("AYLIS_DEBUG_GL").is_some() {
        print_gl_info(&display);
    }
    if gl_description().is_none() {
        let text = read_gl_strings(&display).join(" · ");
        GL_DESCRIPTION.with(|d| *d.borrow_mut() = Some(text));
    }

    let mut skia = GlBackend::new(
        |name| {
            // Em EGL o Skia tentaria usar esta função; o glutin já cuida do display.
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            CString::new(name).map_or(std::ptr::null(), |n| display.get_proc_address(&n))
        },
        config.num_samples() as usize,
        config.stencil_size() as usize,
    )
    .ok_or("Skia: falha ao criar o contexto OpenGL")?;
    DISPLAY.with(|d| *d.borrow_mut() = Some(display.clone()));
    skia.set_gl_loader(gl_loader as *const () as usize);

    Ok((Rc::new(window), GpuBackend { skia, gl_surface, gl_context }))
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
        let _ = self.gl_context.make_current(&self.gl_surface);
        // Os campos (o Skia primeiro) caem depois daqui, com o contexto certo.
    }
}

impl GpuBackend {
    /// Com várias janelas, cada uma tem o seu contexto OpenGL: antes de mexer numa, o
    /// contexto dela precisa ser o atual.
    fn make_current(&self) -> Result<(), Error> {
        if !self.gl_context.is_current() {
            self.gl_context.make_current(&self.gl_surface)?;
        }
        Ok(())
    }

    pub(crate) fn resize(&self, window: &Window) {
        if self.make_current().is_ok() {
            window.resize_surface(&self.gl_surface, &self.gl_context);
        }
    }

    /// A imagem do quadro anterior está guardada neste tamanho: o quadro pode repintar só
    /// uma área.
    pub(crate) fn keeps_image(&self, (width, height): (NonZeroU32, NonZeroU32)) -> bool {
        self.skia.keeps_image(width.get(), height.get())
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
        if self.skia.render_parts(renderer, width.get(), height.get(), scale, background, parts, paint).is_none() {
            return Err("Skia: falha ao criar a superfície OpenGL".into());
        }
        // Depois do swap o buffer de trás fica indefinido: a captura vem antes.
        let frame = if capture { self.skia.read_frame() } else { None };
        self.gl_surface.swap_buffers(&self.gl_context)?;
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
            self.result = Some(create(event_loop, attributes).map(|(_window, mut backend)| f(&mut backend.skia)));
            event_loop.exit();
        }

        fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
    }

    let mut app = OneShot { f: Some(f), result: None };
    EventLoop::new()?.run_app(&mut app)?;
    app.result.unwrap_or_else(|| Err("o sistema não iniciou o aplicativo".into()))
}
