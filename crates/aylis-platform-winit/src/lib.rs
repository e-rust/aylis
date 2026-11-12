//! # aylis-platform-winit
//!
//! Plataforma do aylis sobre o `winit`: abre a janela, traduz mouse/teclado/toque e pinta
//! o form.
//!
//! **Desktop** (Windows; Linux e macOS pelo mesmo caminho), com `aylis_skia`:
//! - na **GPU** com OpenGL (feature `gpu`), ou
//! - na **CPU**, mostrando os pixels com `softbuffer` (feature `cpu`).
//!
//! As duas vêm ligadas por padrão. Com [`Renderer::Auto`] (o padrão) o aylis tenta a GPU
//! e, se o OpenGL não estiver disponível, cai para a CPU.
//!
//! **Windows sem Skia** (feature `d2d`): Direct2D + DirectWrite, que já vêm no sistema.
//! Com `default-features = false, features = ["d2d"]` o Skia nem entra no executável.
//! Com as duas, `Renderer::D2d` (ou `AYLIS_RENDERER=d2d`) escolhe o Direct2D.
//!
//! **Medição** (`AYLIS_BENCH=N`): redesenha N quadros sem parar, sem esperar o monitor, e
//! escreve o tempo de cada quadro (pintura + apresentação: média, p50, p95) e a memória
//! do processo; depois fecha.
//!
//! **Navegador** (`wasm32-unknown-unknown`): o form é desenhado num `<canvas>` com o
//! Canvas 2D (`aylis_web`); [`run`] retorna na hora e o navegador conduz os eventos.
//!
//! **Celular:** no Android o app é uma `NativeActivity` e começa em `android_main`, que
//! chama [`android::run`]; o form ocupa a tela, o OpenGL ES (ou a CPU) desenha, e o
//! teclado virtual abre quando um campo de texto ganha o foco. No iOS o [`run`] normal
//! funciona (desenhando na CPU por enquanto).
//!
//! O código de pintura dos controles é o mesmo em todos os casos.
//!
//! ```no_run
//! use aylis_core::Ui;
//!
//! let mut ui = Ui::new();
//! # #[derive(Default)] struct F { base: aylis_core::ControlBase }
//! # impl aylis_core::Component for F {
//! #   fn class_name(&self) -> &'static str { "Form" }
//! #   fn control(&self) -> Option<&aylis_core::ControlBase> { Some(&self.base) }
//! #   fn control_mut(&mut self) -> Option<&mut aylis_core::ControlBase> { Some(&mut self.base) }
//! # }
//! let form = ui.add_root(F::default());
//! aylis_platform_winit::run(ui, form).unwrap();
//! ```

#[cfg(not(any(web, native_cpu, native_gpu, native_d2d)))]
compile_error!("aylis-platform-winit: no desktop, ligue ao menos uma das features `cpu`, `gpu` ou `d2d` (Windows)");

#[cfg(native_cpu)]
mod cpu;
#[cfg(native_d2d)]
mod d2d;
pub(crate) mod diag;

/// Uma configuração de diagnóstico (testes e medições do próprio aylis): a variável de
/// ambiente `name` e, no Android (onde o app não recebe variáveis), a propriedade
/// `debug.aylis.<nome>` (`AYLIS_BENCH` → `debug.aylis.bench`). Só com a feature `diagnostics`
/// (a `dev` do app liga): no app publicado é sempre `None`.
pub fn setting(name: &str) -> Option<String> {
    diag::diagnostic_setting(name)
}
#[cfg(native_gpu)]
mod gpu;
#[cfg(all(native_gpu, target_os = "linux"))]
mod gpu_linux;
mod access;
mod http;
#[cfg(not(mobile))]
mod links;
/// A amostragem da pilha da thread da interface (o profiler da IDE).
#[cfg(all(windows, target_arch = "x86_64"))]
pub mod sampler;
#[cfg(any(windows, all(target_os = "linux", not(target_os = "android"))))]
mod sensors;
#[cfg(not(mobile))]
mod gamepads;
#[cfg(web)]
mod web_windows;
/// Os monitores (lista, nome do aparelho, área útil) e a posição guardada dos forms.
mod monitors;
#[cfg(not(any(web, mobile)))]
mod placements;
#[cfg(target_os = "android")]
mod android_gamepad;
#[cfg(not(any(web, mobile)))]
mod print;
#[cfg(windows)]
mod notify;
mod input;
#[cfg(target_os = "android")]
mod android_jni;
#[cfg(target_os = "android")]
mod android_device;
#[cfg(target_os = "android")]
mod android_radio;
#[cfg(target_os = "android")]
mod android_push;
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod apple_push;
#[cfg(windows)]
mod windows_push;
#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod linux_push;
#[cfg(all(feature = "nfc", not(any(web, mobile))))]
mod desktop_nfc;
#[cfg(all(feature = "bluetooth", not(any(web, mobile))))]
mod desktop_bluetooth;
#[cfg(any(target_os = "android", test))]
mod radio_lines;
#[cfg(target_os = "ios")]
mod ios_keyboard;
#[cfg(target_os = "ios")]
mod ios_device;
#[cfg(web)]
mod web;
#[cfg(web)]
mod web_access;
#[cfg(web)]
mod web_device;
#[cfg(web)]
mod web_radio;
#[cfg(web)]
mod web_push;
#[cfg(any(native_webview, web_webview, android_webview))]
mod native;
#[cfg(native_tray)]
mod tray;
#[cfg(any(native_tray, windows))]
mod badge;
#[cfg(windows)]
mod win32;
#[cfg(windows)]
mod drag_window;
#[cfg(target_os = "windows")]
mod textassist_win;
#[cfg(target_os = "windows")]
mod hello;
#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod textassist_linux;

/// Windows: interceptar as mensagens da janela de um form (impedir ou trocar).
#[cfg(windows)]
pub mod win {
    pub use crate::win32::{MessageReply, WindowMessage, clear_message_hooks, hwnd, msg, on_message, remove_message_hook};
}

use std::fmt;
use std::num::NonZeroU32;
use std::rc::Rc;
#[cfg(native_gpu)]
pub use gpu::with_offscreen_gl;
use aylis_core::{
    Bounds, ComponentId, CursorShape, Modifiers, PointF, PointerEvent, PointerKind, PropValue, Ui, WheelEvent,
};
#[cfg(native_skia)]
use aylis_skia::SkiaRenderer;
use web_time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, Ime, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::event::StartCause;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowButtons, WindowId};

pub type Error = Box<dyn std::error::Error>;

/// Onde desenhar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Renderer {
    /// GPU se disponível, senão CPU.
    #[default]
    Auto,
    /// Skia na CPU.
    Cpu,
    /// Skia na GPU (OpenGL).
    Gpu,
    /// Direct2D (Windows, feature `d2d`), sem Skia.
    D2d,
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Renderer::Auto => "automático",
            Renderer::Cpu => "Skia CPU",
            Renderer::Gpu => "Skia GPU (OpenGL)",
            Renderer::D2d => "Direct2D",
        })
    }
}

/// Os renderizadores compilados (pelas features).
struct Painters {
    #[cfg(native_skia)]
    skia: SkiaRenderer,
    #[cfg(native_d2d)]
    d2d: aylis_d2d::D2dRenderer,
}

impl Painters {
    #[cfg(not(web))]
    fn new() -> Result<Self, Error> {
        Ok(Self {
            #[cfg(native_skia)]
            skia: SkiaRenderer::new(),
            #[cfg(native_d2d)]
            d2d: aylis_d2d::D2dRenderer::new()?,
        })
    }

    /// O renderizador de verdade para a escolha (o automático, resolvido).
    fn resolve(renderer: Renderer) -> Renderer {
        match renderer {
            Renderer::Auto if cfg!(not(native_skia)) && cfg!(native_d2d) => Renderer::D2d,
            r => r,
        }
    }
}

/// Medição de quadros (`AYLIS_BENCH=N`).
struct Bench {
    frames: usize,
    times: Vec<f64>,
}

impl Bench {
    const WARMUP: usize = 10;

    /// `AYLIS_BENCH=N` no desktop; no navegador, `?bench=N` na URL.
    fn from_env() -> Option<Self> {
        #[cfg(web)]
        let frames = web_sys::window()?
            .location()
            .search()
            .ok()?
            .trim_start_matches('?')
            .split('&')
            .find_map(|p| p.strip_prefix("bench="))?
            .parse::<usize>()
            .ok()?
            .max(1);
        #[cfg(not(web))]
        let frames = diag::diagnostic_setting("AYLIS_BENCH")?.parse::<usize>().ok()?.max(1);
        Some(Self { frames, times: Vec::with_capacity(frames + Self::WARMUP) })
    }

    /// Guarda um quadro; `true` quando terminou.
    fn record(&mut self, ms: f64) -> bool {
        self.times.push(ms);
        self.times.len() >= self.frames + Self::WARMUP
    }

    /// `used`: o renderizador que de fato entrou (e a placa, na GPU).
    fn report(&self, renderer: Renderer, used: &str) {
        let mut t: Vec<f64> = self.times[Self::WARMUP.min(self.times.len())..].to_vec();
        t.sort_by(|a, b| a.total_cmp(b));
        let n = t.len().max(1);
        let avg = t.iter().sum::<f64>() / n as f64;
        let at = |q: f64| t.get(((n as f64 - 1.0) * q).round() as usize).copied().unwrap_or(0.0);
        let (private, working) = process_memory();
        let line = format!(
            "AYLIS_BENCH renderizador={renderer} em_uso=\"{used}\" quadros={n} media_ms={avg:.3} p50_ms={:.3} p95_ms={:.3} fps={:.0} memoria_privada_mb={:.1} working_set_mb={:.1}",
            at(0.5),
            at(0.95),
            1000.0 / avg.max(1e-9),
            private as f64 / 1048576.0,
            working as f64 / 1048576.0
        );
        #[cfg(web)]
        web_sys::console::log_1(&line.into());
        #[cfg(all(not(web), target_os = "android"))]
        diag::diagnostic(&line);
        #[cfg(all(not(web), not(target_os = "android")))]
        println!("{line}");
    }
}

/// (memória privada, working set) do processo, em bytes. Só no Windows.
#[cfg(windows)]
#[doc(hidden)]
pub fn process_memory() -> (usize, usize) {
    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set: usize,
        working_set: usize,
        quota_peak_paged: usize,
        quota_paged: usize,
        quota_peak_non_paged: usize,
        quota_non_paged: usize,
        pagefile: usize,
        peak_pagefile: usize,
        private: usize,
    }
    unsafe extern "system" {
        fn GetCurrentProcess() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut Counters, cb: u32) -> i32;
    }
    let mut c = Counters { cb: std::mem::size_of::<Counters>() as u32, ..Default::default() };
    // SAFETY: estrutura do tamanho declarado em `cb`; a função só escreve nela.
    let ok = unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut c, c.cb) };
    if ok == 0 { (0, 0) } else { (c.private, c.working_set) }
}

/// No Linux e no Android: a memória anônima residente (`RssAnon`) e a residente toda
/// (`VmRSS`), de `/proc/self/status`.
#[cfg(all(not(windows), not(web), any(target_os = "linux", target_os = "android")))]
pub fn process_memory() -> (usize, usize) {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let kb = |key: &str| {
        status.lines().find_map(|l| l.strip_prefix(key)).and_then(|v| v.trim().trim_end_matches("kB").trim().parse::<usize>().ok()).unwrap_or(0) * 1024
    };
    (kb("RssAnon:"), kb("VmRSS:"))
}

#[cfg(not(any(windows, all(not(web), any(target_os = "linux", target_os = "android")))))]
pub fn process_memory() -> (usize, usize) {
    (0, 0)
}

/// A memória do processo para o painel de memória (`None` onde não se sabe medir).
fn panel_memory() -> Option<aylis_core::memory_panel::ProcessMemory> {
    let (private, working_set) = process_memory();
    (working_set > 0).then_some(aylis_core::memory_panel::ProcessMemory { private, working_set })
}

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub renderer: Renderer,
    /// Ícone das janelas (PNG, JPEG...): barra de título e barra de tarefas.
    pub icon: Option<Vec<u8>>,
    /// A placa de vídeo, quando há mais de uma (`AYLIS_GPU` troca, para testes).
    pub gpu: aylis_core::gpu::GpuChoice,
    /// Como o quadro chega à janela no Windows (`AYLIS_PRESENT` troca, para testes).
    pub present: aylis_core::gpu::PresentMode,
    /// A pasta do `placements.json` (onde ficam as janelas com `remember_placement`). `None`: a
    /// pasta de dados do usuário com o nome do executável (o relatório do Tabernáculo, P09).
    pub placements_dir: Option<std::path::PathBuf>,
}

/// Abre o form principal numa janela e roda até ele ser fechado (como `Application.Run`).
///
/// O título vem da propriedade `caption` do form (ou do nome dele). O tamanho inicial
/// é o `width`/`height` do form; quando o usuário redimensiona a janela, o form
/// acompanha e o layout é recalculado.
///
/// Outros forms abrem com [`Ui::show`] e [`Ui::show_modal`], cada um na sua janela (no
/// celular e no navegador, a tela mostra o form do topo); fechar o principal encerra.
pub fn run(ui: Ui, form: ComponentId) -> Result<(), Error> {
    run_with(ui, form, Options::default())
}

/// Acorda a interface a partir de qualquer thread (para ela rodar a [`Pump`]).
#[derive(Clone)]
pub struct Waker {
    proxy: EventLoopProxy<()>,
}

impl Waker {
    /// Pede para a interface rodar a bomba de mensagens assim que puder.
    pub fn wake(&self) {
        let _ = self.proxy.send_event(());
    }
}

/// Executada na thread da interface sempre que um [`Waker`] acorda (e uma vez no início):
/// é onde mensagens vindas de outras threads (um backend, a C ABI) mexem no `Ui`.
pub type Pump = Box<dyn FnMut(&mut Ui)>;

/// Como [`run`], escolhendo opções (por exemplo forçar CPU ou GPU).
///
/// No navegador, retorna logo depois de criar o `<canvas>`; o resto acontece nos eventos
/// do navegador (e as opções de renderizador não se aplicam).
pub fn run_with(ui: Ui, form: ComponentId, options: Options) -> Result<(), Error> {
    run_with_pump(ui, form, options, |_| Box::new(|_| {}))
}

/// Como [`run_with`], com uma bomba de mensagens: `make_pump` recebe o [`Waker`] (para
/// entregar a outras threads) e devolve a função que processa o que chegou.
///
/// O `Ui` pede para fechar com [`Ui::request_close`].
pub fn run_with_pump(
    ui: Ui,
    form: ComponentId,
    options: Options,
    make_pump: impl FnOnce(Waker) -> Pump,
) -> Result<(), Error> {
    run_on(EventLoop::new()?, ui, form, options, make_pump)
}

/// Android: o app começa em `android_main(app: AndroidApp)` e entrega o `AndroidApp` aqui.
///
/// ```ignore
/// #[unsafe(no_mangle)]
/// fn android_main(app: aylis_platform_winit::android::AndroidApp) {
///     let (ui, form) = montar_tela();
///     aylis_platform_winit::android::run(app, ui, form).unwrap();
/// }
/// ```
#[cfg(target_os = "android")]
pub mod android {
    pub use winit::platform::android::activity::AndroidApp;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    use super::*;

    /// O que o processo precisa antes do código do app: o `log` no logcat e a pasta
    /// temporária. Sem o `TMPDIR`, o `std::env::temp_dir()` devolve `/data/local/tmp`, onde
    /// um app não grava ("Permission denied"); passa a ser a pasta de cache do app. Chamar
    /// mais de uma vez não faz mal.
    pub fn prepare(app: &AndroidApp) {
        crate::diag::android_logger();
        if std::env::var_os("TMPDIR").is_none()
            && let Some(cache) = app.internal_data_path().and_then(|files| files.parent().map(|dir| dir.join("cache")))
            && std::fs::create_dir_all(&cache).is_ok()
        {
            // SAFETY: no começo do `android_main`, antes do código do app e das threads do aylis.
            unsafe { std::env::set_var("TMPDIR", &cache) };
        }
    }

    pub fn run(app: AndroidApp, ui: Ui, form: ComponentId) -> Result<(), Error> {
        run_with(app, ui, form, Options::default())
    }

    pub fn run_with(app: AndroidApp, ui: Ui, form: ComponentId, options: Options) -> Result<(), Error> {
        run_with_pump(app, ui, form, options, |_| Box::new(|_| {}))
    }

    pub fn run_with_pump(
        app: AndroidApp,
        ui: Ui,
        form: ComponentId,
        options: Options,
        make_pump: impl FnOnce(Waker) -> Pump,
    ) -> Result<(), Error> {
        prepare(&app);
        // O winit só cria o laço de eventos uma vez por processo. Se o Android destruiu a
        // Activity e abriu outra com o processo ainda vivo, `android_main` roda de novo e
        // o laço não nasce: a Activity é reaberta num processo novo (como o
        // ProcessPhoenix), que começa do zero como numa abertura normal.
        let event_loop = match EventLoop::builder().with_android_app(app.clone()).build() {
            Err(winit::error::EventLoopError::RecreationAttempt) => {
                if let Err(e) = super::android_jni::restart_activity(&app) {
                    aylis_core::stderr_line!("aylis: não consegui reabrir a Activity: {e}");
                }
                std::process::exit(0);
            }
            other => other?,
        };
        super::android_jni::init(&app, event_loop.create_proxy());
        run_on(event_loop, ui, form, options, make_pump)
    }
}

fn run_on(
    event_loop: EventLoop<()>,
    mut ui: Ui,
    form: ComponentId,
    options: Options,
    make_pump: impl FnOnce(Waker) -> Pump,
) -> Result<(), Error> {
    let pump = make_pump(Waker { proxy: event_loop.create_proxy() });
    // iOS: o teclado mudou de tamanho, a área segura muda junto.
    #[cfg(target_os = "ios")]
    {
        let proxy = event_loop.create_proxy();
        ios_keyboard::watch(move || {
            let _ = proxy.send_event(());
        });
    }
    // O fuso do aparelho, para "hoje" e horários locais.
    aylis_core::platform::set_local_offset_fn(diag::local_offset);
    // O relógio fino do sistema para os timers `high` e `exact`.
    #[cfg(windows)]
    aylis_core::timing::set_resolution_hook(|on| {
        // SAFETY: só ajusta a resolução do relógio do sistema; cada begin tem o seu end.
        let _ = unsafe { if on { windows::Win32::Media::timeBeginPeriod(1) } else { windows::Win32::Media::timeEndPeriod(1) } };
    });
    // `ui.poster()`: outras threads agendam funções e acordam a interface.
    let proxy = std::sync::Mutex::new(event_loop.create_proxy());
    ui.set_wake(std::sync::Arc::new(move || {
        if let Ok(p) = proxy.lock() {
            let _ = p.send_event(());
        }
    }));
    // Diagnóstico: AYLIS_RENDERER=cpu, gpu ou d2d força o renderizador do modo automático.
    let mut options = options;
    if options.renderer == Renderer::Auto {
        match diag::diagnostic_setting("AYLIS_RENDERER").as_deref() {
            Some("cpu") => options.renderer = Renderer::Cpu,
            Some("gpu") => options.renderer = Renderer::Gpu,
            Some("d2d") => options.renderer = Renderer::D2d,
            _ => {}
        }
    }
    // Diagnóstico: AYLIS_GPU troca a placa de vídeo do projeto.
    if let Some(gpu) = diag::diagnostic_setting("AYLIS_GPU") {
        options.gpu = aylis_core::gpu::GpuChoice::parse(&gpu);
    }
    aylis_core::gpu::set_choice(options.gpu.clone());
    // Diagnóstico: AYLIS_PRESENT=window ou composition troca o do projeto.
    if let Some(present) = diag::diagnostic_setting("AYLIS_PRESENT") {
        options.present = aylis_core::gpu::PresentMode::parse(&present);
    }
    aylis_core::gpu::set_present(options.present);
    #[cfg(target_os = "linux")]
    mesa_gpu(&aylis_core::gpu::choice());
    options.renderer = Painters::resolve(options.renderer);
    #[cfg(not(web))]
    let renderer = Painters::new()?;
    #[cfg(not(web))]
    install_services(&mut ui, &renderer, options.renderer);
    // Campos que já tinham texto antes do motor de texto existir (o form carregado do
    // JSON, o texto posto antes do `run`): conferem agora.
    if ui.services().text_assist.is_some() {
        let checked: Vec<_> = ui.components().filter(|&c| matches!(ui.get_prop(c, "spellcheck"), Ok(PropValue::Bool(true)))).collect();
        for c in checked {
            let _ = ui.set_prop(c, "spellcheck", true);
        }
    }
    #[cfg(web)]
    install_web_services(&mut ui);
    // O aparelho simulado: o form do tamanho da tela dele.
    if let Some((device, landscape)) = simulated_device() {
        let size = device.size(landscape);
        let _ = ui.set_prop(form, "width", size.width);
        let _ = ui.set_prop(form, "height", size.height);
        let _ = ui.set_prop(form, "caption", format!("{} — {}", ui.get_prop(form, "caption").ok().and_then(|c| c.to_str().ok().map(str::to_string)).unwrap_or_default(), device.name));
    }
    // O form passado é o principal (como o `Application.CreateForm` do primeiro form).
    ui.set_main_form(form);
    // Celular e navegador: os forms se empilham numa superfície (abrem e fecham com a
    // `transition` deles).
    ui.set_stacked_forms(SINGLE_SURFACE);
    // No Windows a imagem do arrasto vai numa janela própria (`drag_window`): o form não a pinta.
    ui.set_native_drag_image(cfg!(windows));
    if !ui.is_open(form) {
        ui.show(form)?;
    }
    #[cfg(not(any(web, mobile)))]
    let icon = options
        .icon
        .as_deref()
        .and_then(decode_rgba)
        .and_then(|(pixels, w, h)| winit::window::Icon::from_rgba(pixels, w, h).ok());
    #[cfg(not(any(web, mobile)))]
    let options_placements = options.placements_dir.clone();
    let app = App {
        ui,
        options,
        #[cfg(not(any(web, mobile)))]
        icon,
        #[cfg(not(web))]
        renderer,
        windows: Vec::new(),
        snapshot_dir: diag::diagnostic_setting("AYLIS_SNAPSHOT").map(std::path::PathBuf::from),
        bench: Bench::from_env(),
        error: None,
        started: Instant::now(),
        modifiers: Modifiers::default(),
        keyboard: false,
        #[cfg(target_os = "android")]
        system_bars: None,
        tapped: false,
        started_once: false,
        #[cfg(windows)]
        drag_window: Default::default(),
        watch_insets_until: None,
        #[cfg(any(native_webview, web_webview, android_webview))]
        native: Default::default(),
        #[cfg(web)]
        web_access: None,
        #[cfg(windows)]
        taskbars: Default::default(),
        #[cfg(windows)]
        win_proxy: event_loop.create_proxy(),
        #[cfg(native_tray)]
        trays: Default::default(),
        #[cfg(native_tray)]
        tray_proxy: event_loop.create_proxy(),
        #[cfg(native_tray)]
        tray_next: None,
        pump,
        wakes: setting("AYLIS_WAKE_TRACE").filter(|v| !v.is_empty() && v != "0").map(|_| WakeTrace::default()),
        display_hz: std::cell::Cell::new(None),
        #[cfg(web)]
        floating: Default::default(),
        monitors_checked: None,
        #[cfg(not(any(web, mobile)))]
        placements: match &options_placements {
            Some(dir) => placements::Placements::at(Some(dir.join("placements.json"))),
            None => placements::Placements::load(),
        },
        hovered_files: Vec::new(),
    };
    #[cfg(web)]
    {
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(app);
        Ok(())
    }
    #[cfg(not(web))]
    {
        let mut app = app;
        event_loop.run_app(&mut app)?;
        match app.error {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

/// O aparelho simulado no desktop (`AYLIS_DEVICE`, que a IDE passa no Executar com um aparelho
/// escolhido no designer): a janela do tamanho dele, a área segura e a barra de status.
static DEVICE: std::sync::OnceLock<Option<(&'static aylis_core::devices::Device, bool)>> = std::sync::OnceLock::new();

fn simulated_device() -> Option<(&'static aylis_core::devices::Device, bool)> {
    *DEVICE.get_or_init(|| {
        if cfg!(any(web, mobile)) {
            return None;
        }
        diag::diagnostic_setting("AYLIS_DEVICE").and_then(|spec| aylis_core::devices::parse(&spec))
    })
}

thread_local! {
    /// Quanto o último quadro levou pintando (gravando os comandos), para a faixa da GPU.
    static PAINT_SECS: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
}

/// Pinta o form (e, durante uma transição, o de baixo antes); com um aparelho simulado, o que
/// o sistema dele desenha por cima.
/// A área a repintar (unidades lógicas) alargada até os pixels inteiros e cortada no form:
/// o recorte do renderizador e os controles pintados usam a mesma área. Vazia: `None`
/// (nada a mostrar de novo; o quadro sai inteiro, como antes).
#[cfg_attr(not(native_gpu), allow(dead_code))]
fn pixel_area(area: aylis_core::RectF, scale: f32, form: Option<aylis_core::RectF>) -> Option<aylis_core::RectF> {
    let scale = scale.max(0.01);
    let (l, t) = ((area.x * scale).floor() / scale, (area.y * scale).floor() / scale);
    let (r, b) = ((area.right() * scale).ceil() / scale, (area.bottom() * scale).ceil() / scale);
    let mut rect = aylis_core::RectF::new(l, t, r - l, b - t);
    if let Some(f) = form {
        let (x, y) = (rect.x.max(f.x), rect.y.max(f.y));
        let (right, bottom) = (rect.right().min(f.right()), rect.bottom().min(f.bottom()));
        rect = aylis_core::RectF::new(x, y, (right - x).max(0.0), (bottom - y).max(0.0));
    }
    (rect.width > 0.0 && rect.height > 0.0).then_some(rect)
}

fn paint_stack(ui: &mut Ui, under: Option<ComponentId>, form: ComponentId, canvas: &mut dyn aylis_core::Canvas, areas: Option<&[aylis_core::RectF]>) {
    let painting = Instant::now();
    if let Some(u) = under {
        ui.paint(u, canvas);
    }
    ui.paint_regions(form, canvas, areas.unwrap_or(&[]));
    if let Some((device, landscape)) = simulated_device() {
        let size = ui.control(form).map_or(device.size(landscape), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
        let dark = ui.theme().dark;
        aylis_core::devices::paint_overlay(canvas, device, landscape, aylis_core::RectF::new(0.0, 0.0, size.width, size.height), dark);
    }
    // O painel de memória do app em depuração, só no form principal.
    if ui.memory_panel_visible() && ui.open_forms().first() == Some(&form) {
        let size = ui.control(form).map_or(aylis_core::SizeF::new(0.0, 0.0), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
        let frames = aylis_core::memory_panel::Frames { fps: ui.fps(), max_fps: ui.max_fps() };
        aylis_core::memory_panel::paint(canvas, aylis_core::RectF::new(0.0, 0.0, size.width, size.height), Some(frames), panel_memory());
    }
    PAINT_SECS.with(|p| p.set(painting.elapsed().as_secs_f64()));
}

fn gesture_phase(phase: TouchPhase) -> aylis_core::GesturePhase {
    match phase {
        TouchPhase::Started => aylis_core::GesturePhase::Begin,
        TouchPhase::Moved => aylis_core::GesturePhase::Update,
        TouchPhase::Ended | TouchPhase::Cancelled => aylis_core::GesturePhase::End,
    }
}

/// Decodifica uma imagem (o ícone) com o renderizador que estiver presente.
#[cfg(not(any(web, mobile)))]
fn decode_rgba(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    #[cfg(native_skia)]
    return aylis_skia::decode_rgba(bytes);
    #[cfg(all(native_d2d, not(native_skia)))]
    return aylis_d2d::decode_rgba(bytes);
}

/// Entrega ao `Ui` os serviços desta plataforma que ele ainda não tiver. A medição de
/// texto é a do renderizador em uso (as larguras batem com o desenho).
/// Põe já o serviço de texto do renderizador `renderer` (o que o `run` usaria), para que o
/// `measure_text` valha na montagem dos forms, antes da janela: sem ele, a largura do texto
/// era estimada e um rótulo com `auto_size` nascia cortado (o relatório do Tabernáculo, T01).
/// O `run` mantém o que já estiver posto.
#[cfg(not(web))]
pub fn install_text_service(ui: &mut Ui, renderer: Renderer) {
    if ui.services().text.is_some() {
        return;
    }
    let chosen = Painters::resolve(match setting("AYLIS_RENDERER").as_deref() {
        Some("cpu") if renderer == Renderer::Auto => Renderer::Cpu,
        Some("gpu") if renderer == Renderer::Auto => Renderer::Gpu,
        Some("d2d") if renderer == Renderer::Auto => Renderer::D2d,
        _ => renderer,
    });
    let Ok(painters) = Painters::new() else { return };
    let services = ui.services_mut();
    let _ = (&painters, chosen);
    #[cfg(native_d2d)]
    if chosen == Renderer::D2d {
        services.text = Some(Box::new(painters.d2d.text_service()));
    }
    #[cfg(native_skia)]
    if services.text.is_none() {
        services.text = Some(Box::new(painters.skia.text_service()));
    }
}

/// No navegador o serviço de texto nasce com o `run` (precisa do documento).
#[cfg(web)]
pub fn install_text_service(_ui: &mut Ui, _renderer: Renderer) {}

#[cfg(not(web))]
fn install_services(ui: &mut Ui, renderer: &Painters, chosen: Renderer) {
    let poster = ui.poster();
    let services = ui.services_mut();
    #[cfg(native_skia)]
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(SkiaOffscreen));
    }
    // Só com o Direct2D (o padrão no Windows): a captura também sai por ele.
    #[cfg(all(native_d2d, not(native_skia)))]
    if services.offscreen.is_none() {
        services.offscreen = Some(Box::new(D2dOffscreen));
    }
    if services.text.is_none() {
        let _ = (renderer, chosen);
        #[cfg(native_d2d)]
        if chosen == Renderer::D2d {
            services.text = Some(Box::new(renderer.d2d.text_service()));
        }
        #[cfg(native_skia)]
        if services.text.is_none() {
            services.text = Some(Box::new(renderer.skia.text_service()));
        }
    }
    // A foto da janela com as vistas nativas (W10).
    #[cfg(target_os = "windows")]
    if services.window_capture.is_none() {
        services.window_capture = Some(Box::new(win32::WindowCapture));
    }
    // O motor de texto nativo (correção e previsão), se o app não pôs outro.
    #[cfg(target_os = "windows")]
    if services.text_assist.is_none() {
        services.text_assist = Some(Box::new(textassist_win::WindowsTextAssist::new()));
    }
    #[cfg(target_os = "windows")]
    if services.biometrics.is_none() {
        services.biometrics = Some(Box::new(hello::WindowsHello));
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if services.text_assist.is_none() {
        services.text_assist = Some(Box::new(textassist_linux::LinuxTextAssist::new()));
    }
    #[cfg(not(mobile))]
    if services.file_dialogs.is_none() {
        services.file_dialogs = Some(Box::new(SystemFileDialogs));
    }
    #[cfg(not(any(web, mobile)))]
    if services.printer.is_none() {
        services.printer = Some(Box::new(print::SystemPrinter));
    }
    #[cfg(native_clipboard)]
    if services.clipboard.is_none() {
        if let Ok(clipboard) = arboard::Clipboard::new() {
            services.clipboard = Some(Box::new(SystemClipboard(clipboard)));
        }
    }
    #[cfg(target_os = "android")]
    if services.clipboard.is_none() {
        services.clipboard = Some(Box::new(android_jni::Clipboard));
    }
    if services.http.is_none() {
        services.http = Some(Box::new(http::NativeHttp::default()));
    }
    // Abrir endereços e apps (o `AppLink`); no desktop, o link que abriu o app chega pela
    // linha de comando.
    #[cfg(not(any(web, mobile)))]
    if services.links.is_none() {
        services.links = Some(Box::new(links::DesktopLinks));
        links::command_line_links();
    }
    // NFC por leitor USB e Bluetooth BLE no desktop (as features `nfc` e `bluetooth`).
    #[cfg(all(feature = "nfc", not(any(web, mobile))))]
    if services.nfc.is_none() {
        services.nfc = Some(Box::new(desktop_nfc::DesktopNfc::new()));
    }
    #[cfg(all(feature = "bluetooth", not(any(web, mobile))))]
    if services.bluetooth.is_none() {
        services.bluetooth = Some(Box::new(desktop_bluetooth::DesktopBluetooth::new()));
    }
    #[cfg(any(windows, all(target_os = "linux", not(target_os = "android"))))]
    if services.sensors.is_none() {
        services.sensors = Some(Box::new(sensors::DesktopSensors::new()));
    }
    #[cfg(not(mobile))]
    if services.gamepads.is_none() {
        services.gamepads = gamepads::GilrsPads::new().map(|p| Box::new(p) as Box<dyn aylis_core::platform::GamepadService>);
    }
    #[cfg(target_os = "android")]
    if services.gamepads.is_none() {
        services.gamepads = Some(Box::new(android_gamepad::AndroidGamepads));
    }
    #[cfg(windows)]
    if services.notifications.is_none() {
        services.notifications = Some(Box::new(notify::WindowsNotifications::new(poster.clone())));
    }
    // Notificações push: o WNS (Windows), o APNs (iOS e macOS), o UnifiedPush (Linux); o FCM
    // do Android vem com a AylisActivity, abaixo.
    #[cfg(windows)]
    if services.push.is_none() {
        services.push = Some(Box::new(windows_push::WnsPush::default()));
    }
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if services.push.is_none() {
        services.push = Some(Box::new(apple_push::ApplePush::new()));
    }
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if services.push.is_none() {
        services.push = Some(Box::new(linux_push::UnifiedPush::default()));
    }
    #[cfg(target_os = "android")]
    if services.notifications.is_none() && android_jni::has_webview() {
        services.notifications = Some(Box::new(android_jni::Notifications::new(poster.clone())));
    }
    // Arquivos, permissões e biometria do iOS.
    #[cfg(target_os = "ios")]
    {
        if services.file_dialogs.is_none() {
            services.file_dialogs = Some(Box::new(ios_device::IosFileDialogs));
        }
        if services.permissions.is_none() {
            services.permissions = Some(Box::new(ios_device::IosPermissions::new()));
        }
        if services.biometrics.is_none() {
            services.biometrics = Some(Box::new(ios_device::IosBiometrics));
        }
    }
    // Arquivos, permissões e biometria pela AylisActivity.
    #[cfg(target_os = "android")]
    if android_jni::has_webview() {
        if services.links.is_none() {
            services.links = Some(Box::new(android_device::AndroidLinks));
        }
        if services.sensors.is_none() {
            services.sensors = Some(Box::new(android_device::AndroidSensors));
        }
        if services.background.is_none() {
            services.background = Some(Box::new(android_device::AndroidBackground));
        }
        if services.phone.is_none() {
            services.phone = Some(Box::new(android_device::AndroidPhone));
        }
        if services.nfc.is_none() {
            services.nfc = Some(Box::new(android_radio::AndroidNfc));
        }
        if services.bluetooth.is_none() {
            services.bluetooth = Some(Box::new(android_radio::AndroidBluetooth));
        }
        if services.push.is_none() {
            services.push = Some(Box::new(android_push::AndroidPush));
        }
        if services.actions.is_none() {
            services.actions = Some(Box::new(android_push::AndroidActions));
        }
        if services.file_dialogs.is_none() {
            services.file_dialogs = Some(Box::new(android_device::AndroidFileDialogs));
        }
        if services.permissions.is_none() {
            services.permissions = Some(Box::new(android_device::AndroidPermissions));
        }
        if services.biometrics.is_none() {
            services.biometrics = Some(Box::new(android_device::AndroidBiometrics));
        }
        android_device::install_files();
    }
    let _ = poster;
}

/// Desenho fora da tela com o Skia na CPU (a imagem de um form na notificação).
#[cfg(native_skia)]
struct SkiaOffscreen;

#[cfg(native_skia)]
impl aylis_core::platform::OffscreenService for SkiaOffscreen {
    fn render_png(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        let (w, h) = ((width as f32 * scale).ceil() as u32, (height as f32 * scale).ceil() as u32);
        aylis_skia::SkiaRenderer::new().render_png(w, h, scale, background, |canvas| paint(canvas))
    }
}

#[cfg(all(native_d2d, not(native_skia)))]
struct D2dOffscreen;

#[cfg(all(native_d2d, not(native_skia)))]
impl aylis_core::platform::OffscreenService for D2dOffscreen {
    fn render_png(&self, width: u32, height: u32, scale: f32, background: aylis_core::Color, paint: &mut dyn FnMut(&mut dyn aylis_core::Canvas)) -> Option<Vec<u8>> {
        let (w, h) = ((width as f32 * scale).ceil() as u32, (height as f32 * scale).ceil() as u32);
        let rgba = aylis_d2d::D2dRenderer::new().ok()?.render_rgba(w, h, scale, background, |canvas| paint(canvas))?;
        aylis_d2d::encode_png(&rgba, w, h)
    }
}

#[cfg(web)]
fn install_web_services(ui: &mut Ui) {
    // Imagens decodificadas pelo navegador chegam depois: repinta quando carregam.
    let poster = ui.poster();
    let paste = poster.clone();
    aylis_web::on_image_loaded(move || poster.post(|ui| ui.request_repaint()));
    let services = ui.services_mut();
    // Copiar e colar pela área de transferência do sistema.
    if services.clipboard.is_none() {
        services.clipboard = Some(Box::new(web::WebClipboard));
        web::watch_paste(paste);
        web::keep_mouse_navigation();
    }
    if services.text.is_none() {
        if let Ok(text) = aylis_web::WebTextService::new() {
            services.text = Some(Box::new(text));
        }
    }
    if services.http.is_none() {
        services.http = Some(Box::new(http::WebHttp));
    }
    if services.links.is_none() {
        services.links = Some(Box::new(links::WebLinks));
    }
    // Controles de jogo pela Gamepad API.
    if services.gamepads.is_none() {
        services.gamepads = gamepads::GilrsPads::new().map(|p| Box::new(p) as Box<dyn aylis_core::platform::GamepadService>);
    }
    // Arquivos, permissões e biometria do navegador.
    if services.file_dialogs.is_none() {
        services.file_dialogs = Some(Box::new(web_device::WebFileDialogs));
    }
    if services.permissions.is_none() {
        services.permissions = Some(Box::new(web_device::WebPermissions::new()));
    }
    if services.biometrics.is_none() {
        services.biometrics = Some(Box::new(web_device::WebBiometrics::new()));
    }
    // Web NFC e Web Bluetooth (só no Chrome; onde não houver, `available` é falso).
    if services.nfc.is_none() {
        services.nfc = Some(Box::new(web_radio::WebNfc::default()));
    }
    if services.bluetooth.is_none() {
        services.bluetooth = Some(Box::new(web_radio::WebBluetooth::default()));
    }
    // Web Push pelo service worker da publicação.
    if services.push.is_none() {
        services.push = Some(Box::new(web_push::WebPush::new()));
    }
    web_device::install_files();
}

/// Diálogos de arquivo do sistema (rfd), numa thread própria: a interface não trava.
#[cfg(not(any(web, mobile)))]
struct SystemFileDialogs;

#[cfg(not(any(web, mobile)))]
impl aylis_core::platform::FileDialogService for SystemFileDialogs {
    fn show(&mut self, options: aylis_core::platform::FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>) {
        // A janela do form dono (no Windows, pelo HWND): o diálogo fica por cima dela.
        #[cfg(windows)]
        let parent = options.owner.and_then(win32::hwnd);
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new();
            #[cfg(windows)]
            if let Some(hwnd) = parent {
                dialog = dialog.set_parent(&win32::ParentWindow(hwnd));
            }
            if !options.title.is_empty() {
                dialog = dialog.set_title(&options.title);
            }
            for (name, exts) in &options.filters {
                // No macOS o painel não entende o "*": sem esse filtro, ele já mostra todos.
                if cfg!(target_os = "macos") && exts.iter().any(|e| e == "*") {
                    continue;
                }
                dialog = dialog.add_filter(name, exts);
            }
            if !options.directory.is_empty() {
                dialog = dialog.set_directory(&options.directory);
            }
            if !options.file_name.is_empty() {
                dialog = dialog.set_file_name(&options.file_name);
            }
            let to_string = |p: std::path::PathBuf| p.display().to_string();
            let paths = if options.save {
                dialog.save_file().map(to_string).into_iter().collect()
            } else if options.multiple {
                dialog.pick_files().unwrap_or_default().into_iter().map(to_string).collect()
            } else {
                dialog.pick_file().map(to_string).into_iter().collect()
            };
            #[cfg(windows)]
            if parent.is_some() {
                win32::dialog_closed();
            }
            done(paths);
        });
    }
}

#[cfg(native_clipboard)]
struct SystemClipboard(arboard::Clipboard);

#[cfg(native_clipboard)]
impl aylis_core::platform::ClipboardService for SystemClipboard {
    fn get_text(&mut self) -> Option<String> {
        self.0.get_text().ok()
    }

    fn set_text(&mut self, text: &str) {
        let _ = self.0.set_text(text.to_string());
    }
}

enum Backend {
    #[cfg(native_cpu)]
    Cpu(cpu::CpuBackend),
    #[cfg(native_gpu)]
    Gpu(gpu::GpuBackend),
    #[cfg(native_d2d)]
    D2d(d2d::D2dBackend),
    #[cfg(web)]
    Web(web::WebBackend),
}

impl Backend {
    /// O nome do renderizador desta janela (para as gravações de uso).
    fn name(&self) -> &'static str {
        match self {
            #[cfg(native_cpu)]
            Backend::Cpu(_) => "Skia CPU",
            #[cfg(native_gpu)]
            Backend::Gpu(_) => "Skia GPU (OpenGL)",
            #[cfg(native_d2d)]
            Backend::D2d(_) => "Direct2D",
            #[cfg(web)]
            Backend::Web(_) => "Canvas 2D",
        }
    }
}

/// Celular e navegador têm uma tela só: ela mostra o form do topo (o último aberto).
/// No desktop, cada form aberto tem a sua janela.
const SINGLE_SURFACE: bool = cfg!(any(web, mobile));

/// Uma janela e o form que ela mostra.
struct FormWindow {
    /// Acessibilidade (leitores de tela) desta janela.
    #[cfg(not(web))]
    access: Option<access::Adapter>,
    // O backend guarda recursos ligados à janela: precisa ser liberado antes dela.
    backend: Backend,
    window: Rc<Window>,
    form: ComponentId,
    /// Última posição do ponteiro, em unidades lógicas.
    pointer: PointF,
    cursor: CursorShape,
    /// Aceita entrada (fica desligada debaixo de um form modal).
    enabled: bool,
    /// `border_style = custom`: a borda sob o mouse (redimensiona ao apertar).
    edge: Option<winit::window::ResizeDirection>,
    /// O título mostrado (o `caption` do form muda e a janela acompanha, como no Delphi).
    title: String,
    /// O tamanho mínimo e o máximo aplicados à janela (largura e altura, lógicos com o zoom;
    /// 0 = sem limite): `min_width`/`min_height`/`max_width`/`max_height` do form.
    limits: [f64; 4],
    /// Os pontos do mouse entre dois eventos (o Windows junta os movimentos).
    #[cfg(windows)]
    trail: win32::MouseTrail,
    /// Quando desenhou pela última vez (segundos do app): o `frame_rate` do form pula os quadros
    /// antes da vez dele.
    last_draw: f64,
    /// Estava minimizada (o `on_minimize` e o `on_restore` saem da mudança).
    minimized: bool,
    /// A barra de título escura (acompanha o tema).
    dark_title: bool,
}

struct App {
    ui: Ui,
    options: Options,
    /// Ícone das janelas, já decodificado.
    #[cfg(not(any(web, mobile)))]
    icon: Option<winit::window::Icon>,
    #[cfg(not(web))]
    renderer: Painters,
    windows: Vec<FormWindow>,
    /// `AYLIS_SNAPSHOT`: pasta onde gravar o último quadro de cada janela.
    snapshot_dir: Option<std::path::PathBuf>,
    /// `AYLIS_BENCH`: medição dos quadros.
    bench: Option<Bench>,
    error: Option<Error>,
    started: Instant,
    modifiers: Modifiers,
    /// Teclado virtual aberto (celular).
    keyboard: bool,
    /// Android: a cor já mandada para as barras do sistema.
    #[cfg(target_os = "android")]
    system_bars: Option<aylis_core::Color>,
    /// Houve um toque: se o foco está num campo, pede o teclado de novo (o usuário pode
    /// tê-lo fechado com o "voltar" sem o campo perder o foco).
    tapped: bool,
    /// Já abriu a janela alguma vez (no Android ela é recriada a cada volta ao app).
    started_once: bool,
    /// A imagem do arrasto numa janela própria (segue o mouse pela tela inteira).
    #[cfg(windows)]
    drag_window: drag_window::DragWindow,
    /// Até quando conferir a área segura (o sistema não avisa quando o teclado virtual
    /// termina de abrir ou fechar).
    watch_insets_until: Option<Instant>,
    /// Navegadores embutidos (WebView) sobre os controles.
    #[cfg(any(native_webview, web_webview, android_webview))]
    native: native::NativeViews,
    /// Windows: o botão na barra de tarefas (Taskbar) e quem acorda o laço com as mensagens
    /// da janela.
    #[cfg(windows)]
    taskbars: win32::Taskbars,
    #[cfg(windows)]
    win_proxy: EventLoopProxy<()>,
    /// Ícones na bandeja (TrayIcon) e quem acorda o laço com os cliques neles.
    #[cfg(native_tray)]
    trays: tray::Trays,
    #[cfg(native_tray)]
    tray_proxy: EventLoopProxy<()>,
    /// Quando a animação de um ícone da bandeja pede o próximo quadro.
    #[cfg(native_tray)]
    tray_next: Option<f64>,
    /// Navegador: a árvore de acessibilidade em elementos ARIA sobre o canvas.
    #[cfg(web)]
    web_access: Option<web_access::WebAccess>,
    /// Navegador: os forms por cima do principal como janelas dentro do canvas.
    #[cfg(web)]
    floating: web_windows::Floating,
    /// Quando os monitores foram conferidos (a cada 2 s).
    monitors_checked: Option<Instant>,
    /// As posições guardadas dos forms com `remember_placement`.
    #[cfg(not(any(web, mobile)))]
    placements: placements::Placements,
    /// Os arquivos do sistema passando por cima de uma janela agora.
    hovered_files: Vec<String>,
    pump: Pump,
    /// `AYLIS_WAKE_TRACE`: por que o laço acordou, contado por segundo (para achar o que
    /// gasta CPU com o app parado).
    wakes: Option<WakeTrace>,
    /// A taxa do monitor e quando foi perguntada (pedir ao sistema leva milissegundos, e o
    /// arraste com um mouse rápido pergunta a cada evento).
    display_hz: std::cell::Cell<Option<(Instant, f64)>>,
}

impl App {
    fn index_of(&self, id: WindowId) -> Option<usize> {
        self.windows.iter().position(|w| w.window.id() == id)
    }

    fn window_of(&self, form: ComponentId) -> Option<&FormWindow> {
        self.windows.iter().find(|w| w.form == form)
    }

    /// As janelas que continuam abertas e eram possuídas por uma das que vão fechar passam
    /// para a janela do form principal (ou ficam sem dona): o Windows destruiria elas junto.
    #[cfg(windows)]
    fn reown_windows_of(&mut self, closing: &[ComponentId]) {
        let main = self.ui.main_form().filter(|f| !closing.contains(f)).and_then(|f| self.window_of(f)).map(|w| w.window.clone());
        let closing_hwnds: Vec<_> = self.windows.iter().filter(|w| closing.contains(&w.form)).filter_map(|w| win32::hwnd_of(&w.window)).collect();
        for w in self.windows.iter().filter(|w| !closing.contains(&w.form)) {
            let Some(hwnd) = win32::hwnd_of(&w.window) else { continue };
            // SAFETY: a janela é deste processo e está viva (o winit a guarda); GetWindow com
            // GW_OWNER só lê a dona.
            let owner = unsafe { windows::Win32::UI::WindowsAndMessaging::GetWindow(hwnd, windows::Win32::UI::WindowsAndMessaging::GW_OWNER) };
            let Ok(owner) = owner else { continue };
            if closing_hwnds.contains(&owner) {
                let new_owner = main.as_deref().filter(|m| win32::hwnd_of(m) != Some(hwnd));
                win32::set_owner(&w.window, new_owner);
                self.ui.set_form_owner(w.form, self.ui.main_form().filter(|_| new_owner.is_some()));
            }
        }
    }

    /// A borda de redimensionar sob `p`, num form `custom` não maximizado.
    fn resize_edge(&self, index: usize, p: PointF) -> Option<winit::window::ResizeDirection> {
        use winit::window::ResizeDirection as D;
        let w = self.windows.get(index)?;
        if SINGLE_SURFACE || w.window.is_maximized() || self.enum_prop(w.form, "border_style") != "custom" {
            return None;
        }
        let size = w.window.inner_size().to_logical::<f32>(w.window.scale_factor() * self.ui.form_zoom(w.form) as f64);
        const EDGE: f32 = 6.0;
        let (left, right) = (p.x < EDGE, p.x > size.width - EDGE);
        let (top, bottom) = (p.y < EDGE, p.y > size.height - EDGE);
        Some(match (left, right, top, bottom) {
            (true, _, true, _) => D::NorthWest,
            (_, true, true, _) => D::NorthEast,
            (true, _, _, true) => D::SouthWest,
            (_, true, _, true) => D::SouthEast,
            (true, ..) => D::West,
            (_, true, ..) => D::East,
            (_, _, true, _) => D::North,
            (_, _, _, true) => D::South,
            _ => return None,
        })
    }

    /// Executa os pedidos à janela (`Ui::window_command`): a barra de título própria.
    fn apply_window_commands(&mut self) {
        use aylis_core::{ResizeEdge, WindowCommand};
        use winit::window::ResizeDirection as D;
        for (form, command) in self.ui.take_window_commands() {
            // A propriedade `gpu` do form mudou: a janela confere a placa (precisa do backend
            // mutável, então fica fora do `match` abaixo).
            if command == WindowCommand::UpdateGpu {
                self.update_gpu(form);
                continue;
            }
            let Some(w) = self.window_of(form) else { continue };
            let window = &w.window;
            match command {
                WindowCommand::Minimize => window.set_minimized(true),
                WindowCommand::Maximize => window.set_maximized(true),
                WindowCommand::Restore => {
                    window.set_minimized(false);
                    window.set_maximized(false);
                }
                WindowCommand::ToggleMaximize => window.set_maximized(!window.is_maximized()),
                WindowCommand::StartDrag => {
                    let _ = window.drag_window();
                }
                WindowCommand::StartResize(edge) => {
                    let _ = window.drag_resize_window(match edge {
                        ResizeEdge::Left => D::West,
                        ResizeEdge::Right => D::East,
                        ResizeEdge::Top => D::North,
                        ResizeEdge::Bottom => D::South,
                        ResizeEdge::TopLeft => D::NorthWest,
                        ResizeEdge::TopRight => D::NorthEast,
                        ResizeEdge::BottomLeft => D::SouthWest,
                        ResizeEdge::BottomRight => D::SouthEast,
                    });
                }
                WindowCommand::SetTopmost(on) => {
                    window.set_window_level(if on { winit::window::WindowLevel::AlwaysOnTop } else { winit::window::WindowLevel::Normal })
                }
                WindowCommand::Hide => {
                    window.set_visible(false);
                    // O app pode ter mostrado a janela pela API do Windows: o winit acha
                    // que ela já está escondida e não faz nada. Escondida de verdade.
                    #[cfg(windows)]
                    win32::set_shown(window, false);
                }
                WindowCommand::Show => {
                    window.set_visible(true);
                    window.set_minimized(false);
                    #[cfg(windows)]
                    win32::set_shown(window, true);
                    window.focus_window();
                }
                WindowCommand::SetFullscreen(on) => {
                    #[cfg(windows)]
                    win32::sync_shown(window);
                    let monitor = match self.ui.get_prop(form, "monitor") {
                        Ok(PropValue::Int(i)) if i >= 0 => window.available_monitors().nth(i as usize),
                        _ => None,
                    };
                    window.set_fullscreen(on.then_some(winit::window::Fullscreen::Borderless(monitor)))
                }
                // No meio da área útil do monitor; maximizada, volta, muda e maximiza de novo.
                WindowCommand::MoveToMonitor(index) => {
                    let Some(target) = window.available_monitors().nth(index) else { continue };
                    let maximized = window.is_maximized();
                    if maximized {
                        window.set_maximized(false);
                    }
                    let (origin, area) = self.work_rect(&target);
                    let size = window.outer_size();
                    let x = origin.x + (area.width as i32 - size.width as i32).max(0) / 2;
                    let y = origin.y + (area.height as i32 - size.height as i32).max(0) / 2;
                    window.set_outer_position(PhysicalPosition::new(x, y));
                    if maximized {
                        window.set_maximized(true);
                    }
                }
                WindowCommand::MoveTo(x, y) => {
                    if window.is_maximized() {
                        window.set_maximized(false);
                    }
                    window.set_outer_position(PhysicalPosition::new(x, y));
                    window.focus_window();
                }
                WindowCommand::Close | WindowCommand::UpdateGpu => {}
            }
        }
    }

    /// Trata um evento de entrada da janela `index`. Devolve `false` se não for de entrada.
    fn handle_input(&mut self, index: usize, event: &WindowEvent) -> bool {
        let form = self.windows[index].form;
        let scale = self.windows[index].window.scale_factor() * self.ui.form_zoom(form) as f64;
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        // Navegador com janelas: o ponteiro vai ao form sob ele; o teclado, ao de cima.
        #[cfg(web)]
        if self.floating_mode() {
            if self.floating_input(index, event, scale) {
                // O cursor também segue o ponteiro aqui (sem isso ficava no último valor,
                // a mãozinha depois de um clique).
                self.after_input();
                return true;
            }
            if let Some(&top) = self.ui.open_forms().last() {
                return self.handle_input_for(index, event, top, scale);
            }
        }
        self.handle_input_for(index, event, form, scale)
    }

    /// Durante um arrasto que começou em `form`, o mouse fora da janela dele: acha a outra
    /// janela do app sob ele (pela posição na tela) e passa ao núcleo o form e o ponto nele.
    #[cfg(not(any(web, mobile)))]
    fn drag_between_windows(&mut self, index: usize, form: ComponentId, position: winit::dpi::PhysicalPosition<f64>) {
        let w = &self.windows[index];
        let size = w.window.inner_size();
        let inside = position.x >= 0.0 && position.y >= 0.0 && position.x < size.width as f64 && position.y < size.height as f64;
        let Ok(origin) = w.window.inner_position() else { return };
        let screen = (origin.x as f64 + position.x, origin.y as f64 + position.y);
        // Quem solta fora das janelas (a aba que vira janela) precisa saber onde.
        self.ui.set_screen_pointer(Some(aylis_core::ScreenPointer { x: screen.0 as i32, y: screen.1 as i32, scale: w.window.scale_factor() as f32 }));
        let mut over = None;
        if !inside {
            for (i, other) in self.windows.iter().enumerate() {
                if i == index || other.window.is_minimized() == Some(true) {
                    continue;
                }
                let (Ok(at), size) = (other.window.inner_position(), other.window.inner_size()) else { continue };
                let local = (screen.0 - at.x as f64, screen.1 - at.y as f64);
                if local.0 >= 0.0 && local.1 >= 0.0 && local.0 < size.width as f64 && local.1 < size.height as f64 {
                    let scale = other.window.scale_factor() * self.ui.form_zoom(other.form) as f64;
                    over = Some((other.form, PointF::new((local.0 / scale) as f32, (local.1 / scale) as f32)));
                    break;
                }
            }
        }
        match over {
            Some((other, at)) => self.ui.drag_over_form(form, Some(other), at),
            // Dentro da própria janela o `pointer_move` já cuidou; fora de todas, sem destino.
            None if !inside => self.ui.drag_over_form(form, None, PointF::default()),
            None => {}
        }
    }

    /// O resto da entrada, para o form `form`.
    fn handle_input_for(&mut self, index: usize, event: &WindowEvent, form: ComponentId, scale: f64) -> bool {
        let modifiers = self.modifiers;
        let pointer_event = |position: PointF, button, kind| PointerEvent { position, button, kind, modifiers, ..Default::default() };
        match event {
            WindowEvent::ModifiersChanged(m) => self.modifiers = input::modifiers(m.state()),
            WindowEvent::CursorMoved { position, .. } => {
                let p = position.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                // Janela sem a barra do sistema (`border_style = custom`): as bordas
                // redimensionam.
                let edge = self.resize_edge(index, p);
                if edge != self.windows[index].edge {
                    self.windows[index].edge = edge;
                    let w = &self.windows[index];
                    w.window.set_cursor(match edge {
                        Some(d) => winit::window::CursorIcon::from(d),
                        None => input::cursor(w.cursor),
                    });
                }
                if edge.is_none() {
                    // Os pontos por onde o mouse passou desde o último evento, antes deste.
                    #[cfg(windows)]
                    {
                        let w = &mut self.windows[index];
                        for (x, y) in w.trail.points_before(&w.window, position.x, position.y) {
                            let q = winit::dpi::PhysicalPosition::new(x, y).to_logical::<f32>(scale);
                            self.ui.pointer_move(form, pointer_event(PointF::new(q.x, q.y), Default::default(), PointerKind::Mouse));
                        }
                    }
                    self.ui.pointer_move(form, pointer_event(p, Default::default(), PointerKind::Mouse));
                    // O arrasto saiu desta janela: o destino pode estar em outra janela do app.
                    #[cfg(not(any(web, mobile)))]
                    if self.ui.dragging(form) {
                        self.drag_between_windows(index, form, *position);
                    }
                }
            }
            WindowEvent::CursorLeft { .. } => {
                #[cfg(windows)]
                self.windows[index].trail.reset();
                self.ui.pointer_leave(form)
            }
            // Arquivos arrastados do sistema (um evento por arquivo).
            WindowEvent::DroppedFile(path) => {
                let p = self.windows[index].pointer;
                self.hovered_files.clear();
                self.ui.files_hover(form, p, None);
                self.ui.drop_files(form, p, vec![path.to_string_lossy().to_string()]);
            }
            // Arquivos do sistema passando por cima (um evento por arquivo): a zona sob o mouse
            // se destaca (F03 do Tabernáculo).
            WindowEvent::HoveredFile(path) => {
                self.hovered_files.push(path.to_string_lossy().to_string());
                let p = self.windows[index].pointer;
                self.ui.files_hover(form, p, Some(self.hovered_files.clone()));
            }
            WindowEvent::HoveredFileCancelled => {
                self.hovered_files.clear();
                let p = self.windows[index].pointer;
                self.ui.files_hover(form, p, None);
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: winit::event::MouseButton::Left, .. }
                if self.windows[index].edge.is_some() =>
            {
                if let Some(d) = self.windows[index].edge {
                    let _ = self.windows[index].window.drag_resize_window(d);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(button) = input::button(*button) else { return true };
                let e = pointer_event(self.windows[index].pointer, button, PointerKind::Mouse);
                match state {
                    ElementState::Pressed => self.ui.pointer_down(form, e),
                    ElementState::Released => self.ui.pointer_up(form, e),
                }
            }
            WindowEvent::MouseWheel { delta: delta_kind, .. } if self.ui.zoom_keys() && modifiers.ctrl => {
                // Ctrl + roda: o zoom do app, de 10 em 10%.
                let up = match delta_kind {
                    MouseScrollDelta::LineDelta(_, y) => *y > 0.0,
                    MouseScrollDelta::PixelDelta(p) => p.y > 0.0,
                };
                let zoom = self.ui.zoom();
                self.ui.set_zoom(if up { zoom + 0.1 } else { zoom - 0.1 });
            }
            WindowEvent::MouseWheel { delta: delta_kind, .. } => {
                // Positivo = rolar para cima / para a esquerda, como no winit.
                let (delta, lines) = self.wheel_delta(delta_kind, scale);
                // No navegador, a posição do próprio evento (sem movimento antes, seria o canto).
                #[cfg(web)]
                if let Some(p) = web::wheel_position() {
                    self.windows[index].pointer = p;
                }
                let e = WheelEvent { position: self.windows[index].pointer, delta, modifiers, lines, ..Default::default() };
                self.ui.wheel(form, e);
            }
            // Android: o analógico e os botões de um controle de jogo chegam como toque e
            // tecla; viram o estado do `Gamepad` e não chegam à tela.
            #[cfg(target_os = "android")]
            WindowEvent::Touch(touch) if android_gamepad::touch(&touch.device_id, touch.phase, touch.location.x, touch.location.y) => {}
            #[cfg(target_os = "android")]
            WindowEvent::KeyboardInput { device_id, event, .. } if android_gamepad::key(device_id, event) => {}
            WindowEvent::Touch(touch) => {
                let p = touch.location.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                let e = PointerEvent { pointer_id: touch.id, ..pointer_event(p, Default::default(), PointerKind::Touch) };
                match touch.phase {
                    TouchPhase::Started => self.ui.pointer_down(form, e),
                    TouchPhase::Moved => self.ui.pointer_move(form, e),
                    TouchPhase::Ended => {
                        self.ui.pointer_up(form, e);
                        self.ui.pointer_leave(form);
                        self.tapped = true;
                    }
                    TouchPhase::Cancelled => self.ui.pointer_cancel(form),
                }
            }
            // Celular: o "voltar" fecha o form de cima (um modal, uma tela aberta com show);
            // no principal, encerra (o Android volta para a tela anterior).
            WindowEvent::KeyboardInput { event, .. }
                if cfg!(mobile)
                    && input::is_pressed(event)
                    && matches!(event.logical_key, winit::keyboard::Key::Named(winit::keyboard::NamedKey::BrowserBack | winit::keyboard::NamedKey::GoBack)) =>
            {
                let top = self.ui.open_forms().last().copied();
                match top {
                    Some(f) if self.ui.is_modal(f) => self.ui.set_modal_result(f, aylis_core::ModalResult::Cancel),
                    Some(f) if Some(f) != self.ui.main_form() => {
                        self.ui.close(f);
                    }
                    // Android: o app vai para segundo plano e volta como estava (como os
                    // apps do Android 12 em diante); no iOS não há "voltar" do sistema.
                    #[cfg(target_os = "android")]
                    _ if android_jni::move_task_to_back() => {}
                    _ => self.ui.request_close(),
                }
            }
            WindowEvent::KeyboardInput { event, .. }
                if self.ui.zoom_keys()
                    && (modifiers.ctrl || modifiers.meta)
                    && input::is_pressed(event)
                    && matches!(event.logical_key.to_text(), Some("+" | "=" | "-" | "0")) =>
            {
                let zoom = self.ui.zoom();
                match event.logical_key.to_text() {
                    Some("+" | "=") => self.ui.set_zoom(zoom + 0.1),
                    Some("-") => self.ui.set_zoom(zoom - 0.1),
                    _ => self.ui.set_zoom(1.0),
                }
            }
            // Ctrl+Shift+M: o painel de memória (só no app em depuração, que o liga).
            WindowEvent::KeyboardInput { event, .. }
                if modifiers.ctrl
                    && modifiers.shift
                    && input::is_pressed(event)
                    && event.logical_key.to_text().is_some_and(|t| t.eq_ignore_ascii_case("m"))
                    && self.ui.toggle_memory_panel() => {}
            WindowEvent::KeyboardInput { event, .. } => {
                let key = input::key_event(event, modifiers);
                if input::is_pressed(event) {
                    self.ui.key_down(form, key);
                    // Ctrl+A, Alt+F... são atalhos, não texto (o winit manda o "a" junto). Com
                    // AltGr (Ctrl+Alt no Windows) a tecla produz texto, como o @.
                    let shortcut = (modifiers.ctrl != modifiers.alt) || modifiers.meta;
                    if let Some(text) = event.text.as_ref().filter(|t| !shortcut && t.chars().all(|c| !c.is_control())) {
                        self.ui.text_input(form, text);
                    }
                } else {
                    self.ui.key_up(form, key);
                }
            }
            // Pinça e giro do touchpad (macOS): o mesmo gesto dos dois dedos na tela.
            WindowEvent::PinchGesture { delta, phase, .. } => {
                let delta = if delta.is_finite() { *delta as f32 } else { 0.0 };
                let e = aylis_core::GestureEvent { phase: gesture_phase(*phase), center: self.windows[index].pointer, scale: 1.0 + delta, delta_scale: 1.0 + delta, ..Default::default() };
                self.ui.gesture(form, e);
            }
            WindowEvent::RotationGesture { delta, phase, .. } => {
                let r = -(*delta).to_radians();
                let e = aylis_core::GestureEvent { phase: gesture_phase(*phase), center: self.windows[index].pointer, scale: 1.0, delta_scale: 1.0, rotation: r, delta_rotation: r, ..Default::default() };
                self.ui.gesture(form, e);
            }
            // Texto composto pelo IME (teclado do iOS, acentos e idiomas asiáticos).
            WindowEvent::Ime(Ime::Commit(text)) => self.ui.text_input(form, text),
            WindowEvent::Focused(focused) => {
                if !focused {
                    self.ui.pointer_cancel(form);
                }
                // `on_activate`/`on_deactivate` do form (um menu fecha ao perder o foco).
                self.ui.fire(form, if *focused { "on_activate" } else { "on_deactivate" }, &aylis_core::EventArgs::None);
            }
            _ => return false,
        }
        self.after_input();
        true
    }

    /// Android: aplica no controle com foco o que o teclado do sistema mandou (texto,
    /// composição, correção, emoji, apagar, Enter).
    #[cfg(target_os = "android")]
    fn apply_keyboard_edits(&mut self) {
        let Some(form) = self.windows.first().map(|w| w.form) else { return };
        use aylis_core::{Key, KeyEvent};
        let press = |ui: &mut Ui, key: Key| {
            let e = KeyEvent { key, ..Default::default() };
            ui.key_down(form, e);
            ui.key_up(form, e);
        };
        for edit in android_jni::take() {
            match edit {
                android_jni::Edit::Text { before, after, text } => {
                    for _ in 0..before {
                        press(&mut self.ui, Key::Backspace);
                    }
                    for _ in 0..after {
                        press(&mut self.ui, Key::Delete);
                    }
                    if !text.is_empty() {
                        self.ui.text_input(form, &text);
                    }
                }
                android_jni::Edit::Key(code) => {
                    let key = match code {
                        66 => Key::Enter,
                        67 => Key::Backspace,
                        112 => Key::Delete,
                        61 => Key::Tab,
                        19 => Key::Up,
                        20 => Key::Down,
                        21 => Key::Left,
                        22 => Key::Right,
                        122 => Key::Home,
                        123 => Key::End,
                        _ => continue,
                    };
                    press(&mut self.ui, key);
                }
            }
        }
        self.after_input();
    }

    /// Android: as barras de status e de navegação com a cor de fundo do form principal
    /// (muda junto com o tema); ícones escuros sobre fundo claro.
    #[cfg(target_os = "android")]
    fn sync_system_bars(&mut self) {
        let Some(form) = self.ui.main_form() else { return };
        let mut color = self.ui.background_color(form);
        if color.a == 0 {
            color = self.ui.theme().color("background");
        }
        if self.system_bars == Some(color) {
            return;
        }
        let luminance = 0.299 * color.r as f32 + 0.587 * color.g as f32 + 0.114 * color.b as f32;
        let argb = 0xFF00_0000 | (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32;
        if android_jni::set_system_bars(argb, luminance > 140.0) {
            self.system_bars = Some(color);
        }
    }

    /// O tipo de teclado do controle com foco: 0 texto, 1 senha, 2 número, 3 e-mail,
    /// 4 texto sem sugestões nem correção (`keyboard_suggestions` desligado).
    #[cfg(target_os = "android")]
    fn keyboard_kind(&self, form: ComponentId) -> i32 {
        let Some(f) = self.ui.focused(form) else { return 0 };
        if matches!(self.ui.get_prop(f, "password"), Ok(PropValue::Bool(true))) {
            return 1;
        }
        let rules = match self.ui.get_prop(f, "rules") {
            Ok(PropValue::String(r)) => r,
            _ => String::new(),
        };
        if rules.contains("number") || self.ui.component(f).map(|c| c.class_name()) == Some("NumberBox") {
            2
        } else if rules.contains("email") {
            3
        } else if matches!(self.ui.get_prop(f, "keyboard_suggestions"), Ok(PropValue::Bool(false))) {
            4
        } else {
            0
        }
    }

    /// Atualiza cursores (e, no celular, o teclado virtual) e pede pintura se algo mudou.
    fn after_input(&mut self) {
        // A imagem do arrasto em andamento acompanha o cursor (ou some, se acabou).
        #[cfg(windows)]
        {
            let visual = self.windows.iter().find_map(|w| self.ui.drag_visual(w.form).map(|(v, _)| (v, w.window.scale_factor() as f32)));
            self.drag_window.sync(visual.as_ref().map(|(v, scale)| (v, *scale)));
        }
        let floating_mode = self.floating_mode();
        for w in &mut self.windows {
            // Com janelas no navegador, o cursor é o do form sob o ponteiro.
            #[cfg(web)]
            let cursor = if floating_mode { self.floating.cursor(&self.ui) } else { self.ui.cursor(w.form) };
            #[cfg(not(web))]
            let cursor = {
                let _ = floating_mode;
                self.ui.cursor(w.form)
            };
            if cursor != w.cursor {
                w.cursor = cursor;
                w.window.set_cursor(input::cursor(cursor));
                w.window.set_cursor_visible(cursor != CursorShape::None);
            }
        }
        // No desktop o teclado é físico; ligar o IME lá mudaria a entrega das teclas.
        if cfg!(mobile) {
            if let Some(w) = self.windows.first() {
                let wanted = self.ui.wants_text_input(w.form);
                let tapped = std::mem::take(&mut self.tapped);
                if wanted != self.keyboard || (wanted && tapped) {
                    self.keyboard = wanted;
                    #[cfg(target_os = "android")]
                    let full = android_jni::show_keyboard(wanted, self.keyboard_kind(w.form));
                    #[cfg(not(target_os = "android"))]
                    let full = false;
                    if !full {
                        w.window.set_ime_allowed(wanted);
                    }
                    self.watch_insets_until = Some(Instant::now() + Duration::from_millis(1500));
                }
            }
        }
        if self.ui.needs_repaint() && self.frame_due().is_none() {
            for w in &self.windows {
                w.window.request_redraw();
            }
        }
    }

    /// Com o limite de FPS (`Ui::set_max_fps`; sem ele, a taxa do monitor, porque nem
    /// todo renderizador espera o vsync): ainda é cedo para o próximo quadro, que pode
    /// sair neste instante.
    fn frame_due(&self) -> Option<Instant> {
        // No navegador o `requestAnimationFrame` já dá um quadro por atualização do monitor (e
        // o navegador não diz a taxa: seria 60 num monitor de 160 Hz); só vale o limite do app.
        let monitor = || if cfg!(web) { None } else { self.ui.last_frame().map(|last| last + 1.0 / self.display_hz()) };
        let at = self.ui.next_frame_at().or_else(monitor)?;
        let now = self.started.elapsed().as_secs_f64();
        (at > now + 0.0005).then(|| self.started + Duration::from_secs_f64(at))
    }

    /// A taxa de quadros do laço: a da janela mais rápida (cada uma com o `frame_rate` do form
    /// ou, sem ele, a do monitor em que está; 60 se o sistema não diz), perguntada ao sistema no
    /// máximo uma vez por segundo. Antes era a do monitor da primeira janela para todas (o
    /// relatório do Tabernáculo, P13).
    fn display_hz(&self) -> f64 {
        if let Some((_, hz)) = self.display_hz.get().filter(|(at, _)| at.elapsed() < Duration::from_secs(1)) {
            return hz;
        }
        let hz = self.windows.iter().map(|w| self.window_hz(w)).fold(0.0, f64::max);
        let hz = if hz > 0.0 { hz } else { 60.0 };
        self.display_hz.set(Some((Instant::now(), hz)));
        hz
    }

    /// A taxa de uma janela: o `frame_rate` do form ou a do monitor dela.
    fn window_hz(&self, w: &FormWindow) -> f64 {
        match self.ui.get_prop(w.form, "frame_rate") {
            Ok(PropValue::Float(rate)) if rate > 0.0 => rate.clamp(1.0, 500.0),
            _ => w.window.current_monitor().and_then(|m| m.refresh_rate_millihertz()).map_or(60.0, |mhz| (mhz as f64 / 1000.0).clamp(24.0, 500.0)),
        }
    }

    fn title(&self, form: ComponentId) -> String {
        match self.ui.get_prop(form, "caption") {
            Ok(PropValue::String(caption)) if !caption.is_empty() => caption,
            _ => self.ui.name(form).unwrap_or("aylis").to_string(),
        }
    }

    fn enum_prop(&self, form: ComponentId, name: &str) -> String {
        match self.ui.get_prop(form, name) {
            Ok(PropValue::Enum(v)) => v,
            _ => String::new(),
        }
    }

    /// O tamanho mínimo e o máximo da janela de `form` (largura e altura, lógicos com o zoom;
    /// 0 = sem limite).
    fn size_limits(&self, form: ComponentId) -> [f64; 4] {
        let zoom = self.ui.form_zoom(form) as f64;
        self.ui.control(form).map_or([0.0; 4], |cb| [cb.min_width(), cb.min_height(), cb.max_width(), cb.max_height()].map(|v| v as f64 * zoom))
    }

    /// Atributos da janela de um form: título, tamanho, borda e posição.
    fn attributes(&self, event_loop: &ActiveEventLoop, form: ComponentId) -> WindowAttributes {
        let (width, height) = self.ui.control(form).map_or((640.0, 480.0), |cb| (cb.width(), cb.height()));
        // Com zoom, a janela cresce junto: o form aparece com o tamanho de projeto.
        let zoom = self.ui.form_zoom(form) as f64;
        let mut attributes = Window::default_attributes()
            .with_title(self.title(form))
            .with_inner_size(LogicalSize::new(width as f64 * zoom, height as f64 * zoom))
            // A barra de título do sistema clara ou escura como o tema (no Windows, pelo DWM;
            // o relatório do Tabernáculo, J04).
            .with_theme(Some(if self.ui.theme().dark { winit::window::Theme::Dark } else { winit::window::Theme::Light }));
        // Mostrar sem tomar o foco (`activate_on_show` desligado; J05).
        if matches!(self.ui.get_prop(form, "activate_on_show"), Ok(PropValue::Bool(false))) {
            attributes = attributes.with_active(false);
        }
        match self.enum_prop(form, "border_style").as_str() {
            "single" => {
                attributes = attributes
                    .with_resizable(false)
                    .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
            }
            "none" => attributes = attributes.with_decorations(false),
            // Barra de título própria (TitleBar): sem a do sistema, redimensionável pelas
            // bordas.
            "custom" => attributes = attributes.with_decorations(false).with_resizable(true),
            _ => {}
        }
        if SINGLE_SURFACE {
            return attributes;
        }
        #[cfg(not(any(web, mobile)))]
        {
            attributes = attributes.with_window_icon(self.icon.clone());
        }
        let owner = self.ui.form_owner(form).and_then(|o| self.window_of(o)).map(|w| w.window.clone());
        // Onde a janela estava (o app guardou): vale se ainda cair numa tela ligada.
        let placement = self.ui.window_placement(form);
        let saved = placement.filter(|p| {
            event_loop.available_monitors().any(|m| {
                let (at, size) = (m.position(), monitors::size(&m));
                p.x + 40 >= at.x && p.y >= at.y && p.x + 40 < at.x + size.width as i32 && p.y + 20 < at.y + size.height as i32
            })
        });
        if let Some(p) = saved.filter(|p| p.width > 0 && p.height > 0) {
            attributes = attributes.with_inner_size(PhysicalSize::new(p.width, p.height));
        }
        if let Some(p) = placement {
            attributes = attributes.with_maximized(p.maximized);
        }
        // O monitor pedido no form (`monitor`, o índice; -1 = o do dono ou o principal).
        let chosen = match self.ui.get_prop(form, "monitor") {
            Ok(PropValue::Int(i)) if i >= 0 => event_loop.available_monitors().nth(i as usize),
            _ => None,
        };
        // O `window_state` do form (o desenhado vale mais que o guardado). A tela cheia vai para
        // o monitor pedido (J07 do Tabernáculo); sem ele, o da janela.
        match self.enum_prop(form, "window_state").as_str() {
            "maximized" => attributes = attributes.with_maximized(true),
            "full_screen" => attributes = attributes.with_fullscreen(Some(winit::window::Fullscreen::Borderless(chosen.clone()))),
            _ => {}
        }
        let position = match self.enum_prop(form, "position").as_str() {
            _ if saved.is_some() => saved.map(|p| PhysicalPosition::new(p.x, p.y)),
            // `left`/`top` a partir do canto do monitor pedido (sem ele, da área de trabalho).
            "designed" => self.ui.control(form).map(|cb| {
                let origin = chosen.as_ref().map_or(PhysicalPosition::new(0, 0), |m| m.position());
                PhysicalPosition::new(origin.x + cb.left() as i32, origin.y + cb.top() as i32)
            }),
            "default" if chosen.is_none() => None,
            // Num canto da área útil (a do monitor pedido ou do principal; J10 do Tabernáculo).
            corner @ ("top_left" | "top_right" | "bottom_left" | "bottom_right") => {
                let monitor = chosen.clone().or_else(|| event_loop.primary_monitor());
                monitor.map(|m| {
                    let (origin, size) = self.work_rect(&m);
                    let scale = m.scale_factor();
                    let (w, h) = ((width as f64 * scale) as i32, (height as f64 * scale) as i32);
                    let x = if corner.ends_with("left") { origin.x } else { origin.x + size.width as i32 - w };
                    let y = if corner.starts_with("top") { origin.y } else { origin.y + size.height as i32 - h };
                    PhysicalPosition::new(x, y)
                })
            }
            choice => {
                // Centro do dono (se houver, a escolha for "center" e nenhum monitor foi
                // pedido) ou da área útil do monitor (o pedido ou o principal).
                let area = match owner.as_ref().filter(|_| choice == "center" && chosen.is_none()) {
                    // A dona minimizada: o lugar dela quando aberta (senão o diálogo abria em
                    // ≈ -21000, fora da tela).
                    #[cfg(windows)]
                    Some(o) if o.is_minimized() == Some(true) => {
                        win32::normal_rect(o).or_else(|| event_loop.primary_monitor().map(|m| self.work_rect(&m)))
                    }
                    #[cfg(not(windows))]
                    Some(o) if o.is_minimized() == Some(true) => event_loop.primary_monitor().map(|m| self.work_rect(&m)),
                    Some(o) => o.outer_position().ok().map(|p| (p, o.outer_size())),
                    None => chosen.clone().or_else(|| event_loop.primary_monitor()).map(|m| self.work_rect(&m)),
                };
                let scale = chosen.clone().or_else(|| event_loop.primary_monitor()).map_or(1.0, |m| m.scale_factor());
                area.map(|(origin, size)| {
                    let w = (width as f64 * scale) as i32;
                    let h = (height as f64 * scale) as i32;
                    PhysicalPosition::new(origin.x + (size.width as i32 - w) / 2, origin.y + (size.height as i32 - h) / 2)
                })
            }
        };
        if let Some(p) = position {
            attributes = attributes.with_position(p);
        }
        if matches!(self.ui.get_prop(form, "topmost"), Ok(PropValue::Bool(true))) {
            attributes = attributes.with_window_level(winit::window::WindowLevel::AlwaysOnTop);
        }
        #[cfg(target_os = "windows")]
        if matches!(self.ui.get_prop(form, "show_in_taskbar"), Ok(PropValue::Bool(false))) {
            use winit::platform::windows::WindowAttributesExtWindows;
            attributes = attributes.with_skip_taskbar(true);
        }
        // Windows: a janela de um form aberto por outro é "possuída" por ele (fica por
        // cima dele e minimiza junto), como as janelas do Delphi.
        #[cfg(target_os = "windows")]
        if let Some(owner) = &owner {
            use winit::platform::windows::WindowAttributesExtWindows;
            use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
            if let Ok(RawWindowHandle::Win32(h)) = owner.window_handle().map(|h| h.as_raw()) {
                attributes = attributes.with_owner_window(h.hwnd.get());
            }
        }
        attributes
    }

    /// A área útil de um monitor (sem a barra de tarefas), em pixels físicos.
    fn work_rect(&self, handle: &winit::monitor::MonitorHandle) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
        match monitors::index_of(self.ui.monitors(), handle).map(|i| self.ui.monitors()[i].work_area()) {
            Some((x, y, w, h)) if w > 0 && h > 0 => (PhysicalPosition::new(x, y), PhysicalSize::new(w, h)),
            _ => (handle.position(), monitors::size(handle)),
        }
    }

    /// Os monitores mudaram? Pergunta ao sistema no máximo a cada 2 s (os modos de vídeo, só
    /// quando a lista muda) e diz a cada form em que monitor está a janela dele.
    fn refresh_monitors(&mut self, event_loop: &ActiveEventLoop, force: bool) {
        if !force && self.monitors_checked.is_some_and(|t| t.elapsed() < Duration::from_secs(2)) {
            return;
        }
        self.monitors_checked = Some(Instant::now());
        let light = monitors::list(event_loop, false);
        let known: Vec<aylis_core::monitors::Monitor> = self.ui.monitors().iter().map(|m| aylis_core::monitors::Monitor { modes: Vec::new(), ..m.clone() }).collect();
        if light != known {
            self.ui.set_monitors(monitors::list(event_loop, true));
        }
        self.sync_form_monitors();
    }

    /// Em que monitor está a janela de cada form.
    fn sync_form_monitors(&mut self) {
        let found: Vec<(ComponentId, Option<usize>)> = self
            .windows
            .iter()
            .map(|w| (w.form, w.window.current_monitor().and_then(|m| monitors::index_of(self.ui.monitors(), &m))))
            .collect();
        for (form, index) in found {
            self.ui.set_form_monitor(form, index);
        }
    }

    /// `remember_placement`: a posição guardada do form vira a dele, se ainda cabe nos
    /// monitores de agora (senão ele abre como se não houvesse).
    #[cfg(not(any(web, mobile)))]
    fn restore_remembered(&mut self, form: ComponentId) {
        if !matches!(self.ui.get_prop(form, "remember_placement"), Ok(PropValue::Bool(true))) || self.ui.window_placement(form).is_some() {
            return;
        }
        let name = self.ui.name(form).unwrap_or_default().to_string();
        if let Some(p) = self.placements.get(&name).and_then(|s| aylis_core::monitors::restore_placement(s, self.ui.monitors())) {
            self.ui.set_window_placement(form, p);
        }
    }

    fn open_window(&mut self, event_loop: &ActiveEventLoop, form: ComponentId) -> Result<(), Error> {
        #[cfg(not(any(web, mobile)))]
        self.restore_remembered(form);
        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut attributes = self.attributes(event_loop, form);
        // Windows: a tela cheia vem depois de a janela e o alvo de desenho existirem. Nascida em
        // tela cheia, sem dona e por cima de tudo, a swap chain é recusada (DXGI_ERROR_INVALID_CALL)
        // e a janela não abre (o relatório do Tabernáculo, R01; na 0.1.0 caía no dxgi.dll).
        #[cfg(windows)]
        let fullscreen_later = attributes.fullscreen.take();
        // Com acessibilidade, a janela nasce invisível: o AccessKit se liga antes de ela
        // aparecer (exigência dele). `AYLIS_ACCESSIBILITY=0` desliga.
        #[cfg(not(web))]
        let accessible = setting("AYLIS_ACCESSIBILITY").as_deref() != Some("0");
        #[cfg(not(web))]
        let hidden = self.ui.take_start_hidden(form);
        #[cfg(not(web))]
        let attributes = if (accessible || hidden) && !cfg!(mobile) { attributes.with_visible(false) } else { attributes };
        // Windows: um form modal tem como dono a janela principal e fica sempre na frente
        // dela, mesmo aberto antes de ela aparecer (a pergunta de confiança ao abrir a IDE).
        // Um modal com a janela principal minimizada: ela volta antes (o modal bloqueia a
        // principal, e minimizada o usuário não vê nem um nem outro).
        #[cfg(not(any(web, mobile)))]
        if self.ui.is_modal(form) {
            if let Some(main) = self.windows.first().filter(|w| w.window.is_minimized() == Some(true)) {
                main.window.set_minimized(false);
            }
        }
        #[cfg(windows)]
        use winit::raw_window_handle::HasWindowHandle;
        #[cfg(windows)]
        let modal_owner = self.ui.form_owner(form).and_then(|o| self.window_of(o)).or_else(|| self.windows.first());
        #[cfg(windows)]
        let attributes = match modal_owner.filter(|_| self.ui.is_modal(form)).map(|w| w.window.window_handle().map(|h| h.as_raw())) {
            Some(Ok(winit::raw_window_handle::RawWindowHandle::Win32(owner))) => {
                use winit::platform::windows::WindowAttributesExtWindows;
                attributes.with_owner_window(owner.hwnd.get())
            }
            _ => attributes,
        };
        #[cfg(not(web))]
        let (window, backend) = create_backend(event_loop, attributes, self.options.renderer, &self.renderer, self.gpu_choice(form))?;
        #[cfg(web)]
        let (window, backend) = create_backend(event_loop, attributes, self.options.renderer)?;
        // Windows: as mensagens da janela passam pelos ganchos do app e pela barra de
        // tarefas.
        #[cfg(windows)]
        win32::attach(&window, form, &self.win_proxy);
        #[cfg(windows)]
        if let Some(fullscreen) = fullscreen_later {
            window.set_fullscreen(Some(fullscreen));
        }
        if self.enum_prop(form, "window_state") == "minimized" {
            window.set_minimized(true);
        }
        #[cfg(not(web))]
        let access = accessible.then(|| {
            let adapter = access::Adapter::new(event_loop, &window, self.ui.poster());
            if !hidden {
                window.set_visible(true);
            }
            adapter
        });
        // Nas gravações de uso: qual renderizador cada janela usa (e a placa, no OpenGL e no
        // Direct2D).
        self.ui.set_platform_info("renderizador", backend.name());
        #[cfg(native_d2d)]
        if let Backend::D2d(d2d) = &backend
            && let Some(gpu) = d2d.adapter()
        {
            self.ui.set_platform_info("gpu", &gpu);
            self.ui.set_form_gpu(form, Some(gpu));
        }
        #[cfg(native_d2d)]
        aylis_core::gpu::set_adapter_list(aylis_d2d::adapters);
        #[cfg(native_gpu)]
        if let Some(gpu) = gpu::gl_description() {
            self.ui.set_platform_info("gpu", &gpu);
        }
        // A placa de cada janela no OpenGL (no Wayland, cada uma pode estar numa).
        #[cfg(native_gpu)]
        if let Backend::Gpu(gl) = &backend {
            self.ui.set_form_gpu(form, gl.adapter());
        }
        #[cfg(all(native_gpu, target_os = "linux"))]
        aylis_core::gpu::set_adapter_list(|| gpu_linux::devices().iter().map(gpu_linux::describe).collect());
        let name = self.ui.name(form).unwrap_or_default().to_string();
        self.ui.report(&aylis_core::activity::Activity::Custom { name: "janela_aberta", value: &format!("{name} ({})", backend.name()) });
        self.windows.push(FormWindow {
            #[cfg(not(web))]
            access,
            backend,
            window,
            form,
            pointer: PointF::ZERO,
            cursor: CursorShape::Default,
            enabled: true,
            edge: None,
            title: self.title(form),
            limits: [0.0; 4],
            #[cfg(windows)]
            trail: Default::default(),
            last_draw: f64::NEG_INFINITY,
            minimized: false,
            dark_title: self.ui.theme().dark,
        });
        self.sync_form_size(self.windows.len() - 1);
        // A janela existe (com o alvo de desenho e, no Windows, o HWND em
        // `aylis::platform::win::hwnd`): o `on_show` vem antes dela, então quem precisa da
        // janela (a barra de título, a posição) usa este (o relatório do Tabernáculo, R03).
        self.ui.fire(form, "on_window_created", &aylis_core::EventArgs::None);
        Ok(())
    }

    /// Abre e fecha janelas para acompanhar os forms abertos no `Ui`.
    /// O deslocamento de um evento da roda (positivo = para cima / para a esquerda) e se veio
    /// em dentes.
    fn wheel_delta(&self, delta_kind: &MouseScrollDelta, scale: f64) -> (PointF, bool) {
        let step = self.ui.scroll_options().wheel_step;
        match delta_kind {
            // Cada dente vale o passo do app (`Ui::scroll_options`).
            MouseScrollDelta::LineDelta(x, y) => (PointF::new(x * step, y * step), true),
            MouseScrollDelta::PixelDelta(p) => {
                let p = p.to_logical::<f32>(scale);
                match input::wheel_notches(p.x, p.y) {
                    // A roda do mouse no navegador (em pixels, cerca de 100 por dente): vira
                    // dentes, com o passo e o deslize do app.
                    Some((x, y)) => (PointF::new(x * step, y * step), true),
                    // Touchpad: os pixels contínuos, direto.
                    None => (PointF::new(p.x, p.y), false),
                }
            }
        }
    }

    /// Navegador numa tela larga: os forms abertos por cima do principal viram janelas dentro
    /// do canvas (`web_windows`); no celular e numa tela estreita, cada um ocupa a tela.
    fn floating_mode(&self) -> bool {
        #[cfg(web)]
        if let Some(w) = self.windows.first() {
            let logical = surface_size(&w.window).to_logical::<f32>(draw_scale(&w.window));
            return logical.width >= web_windows::MIN_WIDTH;
        }
        false
    }

    /// O ponteiro com janelas no navegador: o `web_windows` entrega ao form sob ele (ou arrasta
    /// e fecha pela barra). `false`: não era do ponteiro.
    #[cfg(web)]
    fn floating_input(&mut self, index: usize, event: &WindowEvent, scale: f64) -> bool {
        use web_windows::Pointer;
        let base = self.windows[index].form;
        let canvas = self.ui.control(base).map_or(aylis_core::SizeF::default(), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
        let modifiers = self.modifiers;
        let at = |position: PointF, button, kind| PointerEvent { position, button, kind, modifiers, ..Default::default() };
        let mut leave_after = false;
        let pointer = match event {
            WindowEvent::CursorMoved { position, .. } => {
                let p = position.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                Pointer::Move(at(p, Default::default(), PointerKind::Mouse))
            }
            WindowEvent::CursorLeft { .. } => Pointer::Leave,
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(button) = input::button(*button) else { return true };
                let e = at(self.windows[index].pointer, button, PointerKind::Mouse);
                if *state == ElementState::Pressed { Pointer::Down(e) } else { Pointer::Up(e) }
            }
            // Ctrl + roda: o zoom do app (o caminho de sempre).
            WindowEvent::MouseWheel { .. } if self.ui.zoom_keys() && modifiers.ctrl => return false,
            WindowEvent::MouseWheel { delta: delta_kind, .. } => {
                let (delta, lines) = self.wheel_delta(delta_kind, scale);
                if let Some(p) = web::wheel_position() {
                    self.windows[index].pointer = p;
                }
                Pointer::Wheel(WheelEvent { position: self.windows[index].pointer, delta, modifiers, lines, ..Default::default() })
            }
            WindowEvent::Touch(touch) => {
                let p = touch.location.to_logical::<f32>(scale);
                let p = PointF::new(p.x, p.y);
                self.windows[index].pointer = p;
                let e = PointerEvent { pointer_id: touch.id, ..at(p, Default::default(), PointerKind::Touch) };
                match touch.phase {
                    TouchPhase::Started => Pointer::Down(e),
                    TouchPhase::Moved => Pointer::Move(e),
                    TouchPhase::Ended => {
                        leave_after = true;
                        self.tapped = true;
                        Pointer::Up(e)
                    }
                    TouchPhase::Cancelled => {
                        let top = self.ui.open_forms().last().copied().unwrap_or(base);
                        self.ui.pointer_cancel(top);
                        return true;
                    }
                }
            }
            _ => return false,
        };
        let mut repaint = self.floating.pointer(&mut self.ui, base, canvas, pointer);
        if leave_after {
            repaint |= self.floating.pointer(&mut self.ui, base, canvas, Pointer::Leave);
        }
        if repaint {
            self.windows[index].window.request_redraw();
        }
        true
    }

    fn sync_windows(&mut self, event_loop: &ActiveEventLoop) {
        let open = self.ui.open_forms();
        if SINGLE_SURFACE {
            // Navegador numa tela larga: a tela fica com o principal e os outros viram janelas
            // por cima (`web_windows`).
            let surface = if self.floating_mode() { open.first() } else { open.last() };
            let Some(&top) = surface else { return };
            if self.windows.is_empty() {
                if let Err(e) = self.open_window(event_loop, top) {
                    self.fail(event_loop, e);
                }
            } else if self.windows[0].form != top {
                // Outro form no topo: a tela passa a mostrar ele.
                self.ui.pointer_cancel(self.windows[0].form);
                self.windows[0].form = top;
                self.windows[0].window.set_title(&self.title(top));
                self.sync_form_size(0);
                self.windows[0].window.request_redraw();
            }
        } else {
            // Fechar a janela é soltar o FormWindow.
            let closing: Vec<ComponentId> = self.windows.iter().map(|w| w.form).filter(|f| !open.contains(f)).collect();
            #[cfg(windows)]
            if !closing.is_empty() {
                self.reown_windows_of(&closing);
            }
            self.windows.retain(|w| open.contains(&w.form));
            for form in closing {
                self.ui.set_form_gpu(form, None);
                let name = self.ui.name(form).unwrap_or_default().to_string();
                self.ui.report(&aylis_core::activity::Activity::Custom { name: "janela_fechada", value: &name });
            }
            // Os modais por último: a janela do dono já existe quando eles abrem.
            let mut open = open;
            open.sort_by_key(|&f| self.ui.is_modal(f));
            for form in open {
                if self.window_of(form).is_none() {
                    if let Err(e) = self.open_window(event_loop, form) {
                        self.fail(event_loop, e);
                        return;
                    }
                }
            }
            // O caption mudou depois de aberta: o título acompanha.
            let titles: Vec<String> = self.windows.iter().map(|w| self.title(w.form)).collect();
            for (w, title) in self.windows.iter_mut().zip(titles) {
                if w.title != title {
                    w.window.set_title(&title);
                    w.title = title;
                }
            }
            // O mínimo e o máximo do form (ou o zoom) mudaram: a janela acompanha. Só quando
            // mudam (o Windows reaplica o tamanho a cada chamada, e isso vira outro Resized).
            if !SINGLE_SURFACE {
                let limits: Vec<[f64; 4]> = self.windows.iter().map(|w| self.size_limits(w.form)).collect();
                for (w, limits) in self.windows.iter_mut().zip(limits) {
                    if w.limits != limits {
                        let [min_w, min_h, max_w, max_h] = limits;
                        w.window.set_min_inner_size((min_w > 0.0 || min_h > 0.0).then(|| LogicalSize::new(min_w, min_h)));
                        // Sem máximo num dos lados: grande o bastante para qualquer tela.
                        let open = |v: f64| if v > 0.0 { v } else { 100_000.0 };
                        w.window.set_max_inner_size((max_w > 0.0 || max_h > 0.0).then(|| LogicalSize::new(open(max_w), open(max_h))));
                        w.limits = limits;
                    }
                }
            }
            // Form com `auto_size`: a janela fica da altura do conteúdo (L07 do Tabernáculo). Só
            // quando a diferença passa de meio ponto (cada pedido vira outro Resized).
            if !SINGLE_SURFACE {
                for w in &self.windows {
                    let Some(want) = self.ui.auto_form_height(w.form) else { continue };
                    let zoom = w.window.scale_factor() * self.ui.form_zoom(w.form) as f64;
                    let size = w.window.inner_size().to_logical::<f64>(zoom);
                    if (size.height - want as f64).abs() > 0.5 {
                        let _ = w.window.request_inner_size(LogicalSize::new(size.width * self.ui.form_zoom(w.form) as f64, want as f64 * self.ui.form_zoom(w.form) as f64));
                    }
                }
            }
            // Um diálogo do sistema com dona reabilita a dona ao fechar, mesmo debaixo de um modal
            // do Aylis: a conferência abaixo aplica de novo (o relatório do Tabernáculo, J02).
            #[cfg(windows)]
            let resync = win32::take_dialog_closed();
            #[cfg(not(windows))]
            let resync = false;
            for w in &mut self.windows {
                let enabled = !self.ui.input_blocked(w.form);
                if enabled != w.enabled || (resync && !enabled) {
                    w.enabled = enabled;
                    // Windows: a janela debaixo de um modal fica desabilitada (clicar nela
                    // traz o modal para a frente), como no Delphi.
                    #[cfg(target_os = "windows")]
                    {
                        use winit::platform::windows::WindowExtWindows;
                        w.window.set_enable(enabled);
                    }
                }
            }
        }
        if let Some(form) = self.ui.take_activation() {
            if let Some(w) = self.window_of(form) {
                w.window.focus_window();
            }
        }
    }

    /// Encerra o laço. O `Poll` antes do `exit()`: o winit chama o `about_to_wait` e logo depois
    /// espera por mensagens sem olhar o `exit()`; com o `Wait` da volta anterior, o processo
    /// dormiria até a próxima mensagem do sistema (o relatório do Tabernáculo, R02).
    fn quit(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(not(any(web, mobile)))]
        self.placements.flush();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.exit();
    }

    /// A regra de placa de vídeo do form: a propriedade `gpu` dele ou, vazia, a do projeto.
    #[cfg_attr(web, allow(dead_code))]
    fn gpu_choice(&self, form: ComponentId) -> aylis_core::gpu::GpuChoice {
        match self.ui.get_prop(form, "gpu") {
            Ok(PropValue::String(text)) => aylis_core::gpu::choice_for(&text),
            _ => aylis_core::gpu::choice(),
        }
    }

    /// Com a placa que segue o monitor (Direct2D): se a janela `index` passou para um monitor
    /// de outra placa, o alvo é recriado nela e o form recebe o `on_gpu_change`. Sem a cópia de
    /// cada quadro entre as placas, a janela não trava no monitor da outra.
    ///
    /// No OpenGL quem avisa a troca é o compositor do Wayland (`App::follow_feedback`).
    ///
    /// Arrastando a janela (o laço de mover do Windows), espera soltar: o `WM_EXITSIZEMOVE`
    /// confere de novo (`win32::take_gpu_checks`), e a janela não troca de placa no meio do
    /// arraste nem várias vezes sobre a divisa. `force`: os monitores mudaram (confere mesmo
    /// no monitor de antes).
    fn follow_monitor(&mut self, index: usize, force: bool) {
        #[cfg(native_d2d)]
        {
            let Some(w) = self.windows.get_mut(index) else { return };
            if !force && win32::in_size_move(w.form) {
                return;
            }
            let changed = match &mut w.backend {
                Backend::D2d(d2d) => d2d.follow_monitor(&self.renderer.d2d, force),
                #[allow(unreachable_patterns)]
                _ => return,
            };
            self.after_gpu_change(index, changed);
        }
        #[cfg(not(native_d2d))]
        let _ = (index, force);
    }

    /// Linux (Wayland), `gpu = monitor`: o compositor diz a placa preferida de cada janela
    /// (o feedback do `linux-dmabuf`); a que mudou de placa troca. Barato quando nada mudou.
    #[cfg(all(native_gpu, target_os = "linux"))]
    fn follow_feedback(&mut self) {
        for index in 0..self.windows.len() {
            let w = &mut self.windows[index];
            let Backend::Gpu(gl) = &mut w.backend else { continue };
            let changed = gl.follow_monitor(&w.window);
            if !matches!(changed, Ok(false)) {
                self.after_gpu_change(index, changed);
            }
        }
    }

    /// A propriedade `gpu` do form mudou com o app rodando.
    fn update_gpu(&mut self, form: ComponentId) {
        #[cfg(any(native_d2d, native_gpu))]
        {
            let choice = self.gpu_choice(form);
            let Some(index) = self.windows.iter().position(|w| w.form == form) else { return };
            let w = &mut self.windows[index];
            let changed = match &mut w.backend {
                #[cfg(native_d2d)]
                Backend::D2d(d2d) => d2d.set_choice(&self.renderer.d2d, choice),
                #[cfg(native_gpu)]
                Backend::Gpu(gl) => gl.set_choice(&w.window, choice),
                #[allow(unreachable_patterns)]
                _ => return,
            };
            self.after_gpu_change(index, changed);
        }
        #[cfg(not(any(native_d2d, native_gpu)))]
        let _ = form;
    }

    /// Depois de conferir a placa da janela `index`: trocou, então o quadro sai inteiro na
    /// placa nova e o form fica sabendo.
    #[cfg(any(native_d2d, native_gpu))]
    fn after_gpu_change(&mut self, index: usize, changed: Result<bool, Error>) {
        match changed {
            Ok(true) => {
                let w = &self.windows[index];
                let (form, adapter) = (w.form, match &w.backend {
                    #[cfg(native_d2d)]
                    Backend::D2d(d2d) => d2d.adapter(),
                    #[cfg(native_gpu)]
                    Backend::Gpu(gl) => gl.adapter(),
                    #[allow(unreachable_patterns)]
                    _ => None,
                });
                w.window.request_redraw();
                if let Some(gpu) = &adapter {
                    self.ui.set_platform_info("gpu", gpu);
                }
                self.ui.set_form_gpu(form, adapter);
            }
            Ok(false) => {}
            Err(e) => aylis_core::stderr_line!("aylis: não deu para trocar de placa ({e}); a janela fica na de antes"),
        }
    }

    /// Ajusta o form ao tamanho da janela, em unidades lógicas.
    /// Guarda onde a janela está ([`aylis_core::WindowPlacement`]); maximizada, só marca
    /// (a posição e o tamanho de antes ficam, para voltar a eles).
    fn store_placement(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        if SINGLE_SURFACE || w.window.is_minimized() == Some(true) {
            return;
        }
        let form = w.form;
        let mut p = self.ui.window_placement(form).unwrap_or_default();
        p.maximized = w.window.is_maximized();
        if !p.maximized {
            if let Ok(at) = w.window.outer_position() {
                (p.x, p.y) = (at.x, at.y);
            }
            let size = w.window.inner_size();
            if size.width > 0 && size.height > 0 {
                (p.width, p.height) = (size.width, size.height);
            }
        }
        self.ui.set_window_placement(form, p);
        // O monitor dela agora e, com `remember_placement`, a posição para reabrir igual.
        let monitor = w.window.current_monitor();
        self.ui.set_form_monitor(form, monitor.as_ref().and_then(|m| monitors::index_of(self.ui.monitors(), m)));
        #[cfg(not(any(web, mobile)))]
        if let Some(m) = monitor.filter(|_| matches!(self.ui.get_prop(form, "remember_placement"), Ok(PropValue::Bool(true)))) {
            let name = self.ui.name(form).unwrap_or_default().to_string();
            self.placements.set(&name, aylis_core::monitors::SavedPlacement { monitor: monitors::device(&m), placement: p });
        }
    }

    fn sync_form_size(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        #[cfg(native_gpu)]
        #[allow(irrefutable_let_patterns)] // quando só a feature `gpu` está ligada
        if let Backend::Gpu(gpu) = &w.backend {
            gpu.resize(&w.window);
        }
        let form = w.form;
        let logical = surface_size(&w.window).to_logical::<f64>(draw_scale(&w.window) * self.ui.form_zoom(form) as f64);
        if let Some(cb) = self.ui.control_mut(form) {
            cb.set_size(logical.width as f32, logical.height as f32);
        }
        self.update_safe_area(index);
    }

    /// Passa ao form as faixas da tela cobertas pelo sistema (o layout só refaz se mudou).
    fn update_safe_area(&mut self, index: usize) {
        let Some(w) = self.windows.get(index) else { return };
        // O aparelho simulado: a área segura dele, no form principal.
        if let Some((device, landscape)) = simulated_device().filter(|_| self.ui.main_form() == Some(w.form)) {
            self.ui.set_safe_area(w.form, device.safe_area(landscape));
            return;
        }
        if let Some(insets) = system_insets(&w.window) {
            let form = w.form;
            let z = self.ui.form_zoom(form);
            let insets = Bounds::new(insets.left / z, insets.top / z, insets.right / z, insets.bottom / z);
            let changed = self.ui.safe_area(form) != insets;
            self.ui.set_safe_area(form, insets);
            // O teclado abriu (ou mudou de tamanho): o campo com foco fica à vista.
            if changed && self.keyboard {
                if let Some(f) = self.ui.focused(self.ui.root_of(form)) {
                    self.ui.reveal(f);
                }
            }
        }
    }

    fn redraw(&mut self, index: usize) -> Result<(), Error> {
        let Some(w) = self.windows.get(index) else { return Ok(()) };
        // O `frame_rate` do form: antes da vez dele, o quadro fica para depois (o que mudou
        // continua marcado, e o laço volta a pedir).
        if let Ok(PropValue::Float(rate)) = self.ui.get_prop(w.form, "frame_rate")
            && rate > 0.0
            && self.started.elapsed().as_secs_f64() - w.last_draw < 1.0 / rate as f64 - 0.0005
        {
            return Ok(());
        }
        let now = self.started.elapsed().as_secs_f64();
        if let Some(w) = self.windows.get_mut(index) {
            w.last_draw = now;
        }
        let Some(w) = self.windows.get(index) else { return Ok(()) };
        let size = surface_size(&w.window);
        let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return Ok(()); // janela minimizada
        };
        let size = (width, height);
        let scale = draw_scale(&w.window) as f32 * self.ui.form_zoom(w.form);
        self.update_safe_area(index);
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        self.ui.tick();
        // Arrastando a borda (Windows), o laço modal do sistema só manda Resized e pintura, sem
        // passar pelo about_to_wait: o `set_timeout` que o on_resize agendou (o app refazendo a
        // tela pela largura nova) roda aqui, antes deste quadro, e não só ao soltar o mouse.
        self.ui.update_layout();
        self.ui.run_timers();
        let background = self.ui.theme().color("background");
        let capture = self.snapshot_dir.is_some();
        let floating_mode = self.floating_mode();
        let w = &mut self.windows[index];
        let (ui, form) = (&mut self.ui, w.form);
        // Uma tela entrando ou saindo (transição): a de baixo aparece por trás dela.
        let under = if SINGLE_SURFACE && !floating_mode {
            let open = ui.open_forms();
            (open.len() >= 2 && open.last() == Some(&form) && ui.in_transition(form)).then(|| open[open.len() - 2])
        } else {
            None
        };
        // Só o que mudou: as cópias da rolagem e as áreas (o renderizador guarda a imagem do
        // quadro anterior: o Skia na GPU e na CPU, o Direct2D, o canvas do navegador); com uma
        // transição, a captura ou a imagem nova, a janela inteira. O que é pintado por cima de
        // tudo na janela (o aparelho simulado, o painel de memória) não deixa copiar.
        let bounds = ui.control(form).map(|cb| cb.bounds());
        let damage = ui.take_frame(form);
        let damage = if simulated_device().is_some() || ui.memory_panel_visible() { damage.without_moves() } else { damage.snapped(scale) };
        let (moves, areas): (Vec<aylis_core::ScrollMove>, Vec<aylis_core::RectF>) = match damage {
            aylis_core::FrameDamage::Parts { moves, areas } if !capture && under.is_none() => (moves, areas.into_iter().filter_map(|r| pixel_area(r, scale, bounds)).collect()),
            _ => (Vec::new(), Vec::new()),
        };
        // Para quem decide sozinho se a imagem anterior ainda vale (o Skia na CPU, pela idade
        // do buffer do softbuffer; o Direct2D, pelo quadro guardado dele). Sem área: inteira.
        #[allow(unused_variables)]
        let parts = (!areas.is_empty()).then_some((&moves[..], &areas[..]));
        // Quanto da janela este quadro repinta (o profiler mostra no bloco do quadro; pedido do
        // autor em 08/11, para ver se a pintura do mouse no fundo quase some): sem áreas, inteira.
        let repainted = if areas.is_empty() {
            100.0
        } else {
            let total = (size.0.get() as f32 * size.1.get() as f32).max(1.0);
            (areas.iter().map(|r| r.width * r.height).sum::<f32>() / total * 100.0).min(100.0)
        };
        #[cfg(native_gpu)]
        let gpu_parts = match &w.backend {
            Backend::Gpu(gpu) if gpu.keeps_image(size) => parts,
            _ => None,
        };
        let started = Instant::now();
        let frame: Option<Vec<u8>> = match &mut w.backend {
            #[cfg(native_cpu)]
            Backend::Cpu(cpu) => cpu.draw(&self.renderer.skia, size, scale, background, parts, |c, area| paint_stack(ui, under, form, c, area), capture)?,
            #[cfg(native_gpu)]
            Backend::Gpu(gpu) => gpu.draw(&self.renderer.skia, size, scale, background, gpu_parts, |c, area| paint_stack(ui, under, form, c, area), capture)?,
            #[cfg(native_d2d)]
            Backend::D2d(d2d) => d2d.draw(&self.renderer.d2d, size, scale, background, parts, |c, area| paint_stack(ui, under, form, c, area), capture)?,
            #[cfg(web)]
            Backend::Web(web) => {
                let _ = capture;
                // As janelas dos forms de cima, por cima do principal.
                let mut floating = floating_mode.then_some(&mut self.floating);
                let canvas = ui.control(form).map_or(aylis_core::SizeF::default(), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
                // Por parte só sem janelas por cima (a área é a do form principal).
                let parts = parts.filter(|_| !floating_mode || ui.open_forms().len() <= 1);
                web.draw(size, scale, background, parts, |c, area| {
                    paint_stack(ui, under, form, c, area);
                    if let Some(f) = floating.as_mut() {
                        f.paint(ui, canvas, c);
                    }
                })?;
                None
            }
        };
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        // A GPU no quadro (o profiler): o quadro inteiro menos a pintura, que é o mandar e o
        // apresentar (a espera da placa de vídeo entra aqui).
        if aylis_core::profile::is_on() {
            let gpu = (elapsed / 1000.0 - PAINT_SECS.with(std::cell::Cell::get)).max(0.0);
            let name = format!("quadro · {repainted:.0}% repintado");
            aylis_core::profile::record(aylis_core::profile::Lane::Gpu, &name, (aylis_core::profile::now() - gpu).max(0.0), gpu);
        }
        self.ui.frame_cost_measured(elapsed / 1000.0);
        self.ui.frame_drawn();
        if let (Some(dir), Some(rgba)) = (&self.snapshot_dir, frame) {
            save_snapshot(dir, self.ui.name(form).filter(|n| !n.is_empty()).unwrap_or("form"), &rgba, size);
        }
        if let Some(bench) = self.bench.as_mut() {
            if bench.record(elapsed) {
                // O que de fato desenhou (o pedido pode cair para a CPU, como num WSL sem GPU).
                let info = self.ui.platform_info();
                let get = |k: &str| info.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
                let used = match (get("renderizador"), get("gpu")) {
                    (Some(r), Some(g)) => format!("{r} · {g}"),
                    (Some(r), None) => r,
                    _ => String::new(),
                };
                bench.report(self.options.renderer, &used);
                self.ui.request_close();
            } else if let Some(w) = self.windows.get(index) {
                // Sem esperar mudança: o próximo quadro já.
                self.ui.request_repaint();
                w.window.request_redraw();
            }
        }
        Ok(())
    }

    /// Uma enxurrada de entrada (um mouse de 1000 a 8000 Hz arrastando) nunca deixa a fila
    /// vazia, e o Windows só entrega o `WM_PAINT` (o `request_redraw`) com a fila vazia: a
    /// janela pararia de pintar enquanto o mouse se move (medido: 6 quadros em 2,4 s de
    /// arraste a 4000 Hz). Com algo para pintar e um quadro do monitor sem pintar, pinta aqui
    /// mesmo, no meio da entrada.
    #[cfg(not(web))]
    fn paint_if_starved(&mut self, event_loop: &ActiveEventLoop) {
        if !self.ui.needs_repaint() {
            return;
        }
        let Some(last) = self.ui.last_frame() else { return };
        let now = self.started.elapsed().as_secs_f64();
        // O limite mais curto antes de perguntar a taxa do monitor.
        if now - last < 1.0 / 500.0 || now - last < 1.0 / self.display_hz() {
            return;
        }
        if self.frame_due().is_some() {
            return; // o limite de FPS ainda não deixa
        }
        for index in 0..self.windows.len() {
            if let Err(e) = self.redraw(index) {
                self.fail(event_loop, e);
                return;
            }
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, error: Error) {
        #[cfg(web)]
        web::report(&error);
        self.error = Some(error);
        event_loop.exit();
    }
}

/// Diagnóstico (`AYLIS_SNAPSHOT=pasta`): grava o último quadro de cada janela em
/// `pasta/<nome do form>.png`, lido do próprio buffer da janela (GPU ou CPU). Serve para
/// testes automáticos e para conferir o que foi desenhado quando uma captura de tela não
/// enxerga a janela (OpenGL apresentado direto pela placa).
#[cfg(not(web))]
fn save_snapshot(dir: &std::path::Path, name: &str, rgba: &[u8], (width, height): (NonZeroU32, NonZeroU32)) {
    #[cfg(native_skia)]
    let png = aylis_skia::encode_png(rgba, width.get(), height.get());
    #[cfg(all(native_d2d, not(native_skia)))]
    let png = aylis_d2d::encode_png(rgba, width.get(), height.get());
    if let Some(png) = png {
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(dir.join(format!("{name}.png")), png);
    }
}

#[cfg(web)]
fn save_snapshot(_: &std::path::Path, _: &str, _: &[u8], _: (NonZeroU32, NonZeroU32)) {}

/// Faixas da janela cobertas pelas barras do sistema e pelo teclado virtual, em unidades
/// lógicas. No Android vem da área de conteúdo da `NativeActivity` (que já desconta o
/// teclado, com `adjustResize`).
#[cfg(target_os = "android")]
fn system_insets(window: &Window) -> Option<Bounds> {
    use winit::platform::android::WindowExtAndroid;
    let r = window.content_rect();
    if r.right <= r.left || r.bottom <= r.top {
        return None; // o sistema ainda não informou
    }
    let size = window.inner_size();
    let s = window.scale_factor() as f32;
    let px = |v: i32| v.max(0) as f32 / s;
    Some(Bounds::new(px(r.left), px(r.top), px(size.width as i32 - r.right), px(size.height as i32 - r.bottom)))
}

/// No iOS o winit dá como `inner` só a área segura (sem o entalhe e a barra de início);
/// a diferença para a tela inteira são as faixas.
#[cfg(target_os = "ios")]
fn system_insets(window: &Window) -> Option<Bounds> {
    let s = window.scale_factor();
    let inner = window.inner_position().ok()?.to_logical::<f32>(s);
    let outer = window.outer_position().ok()?.to_logical::<f32>(s);
    let inner_size = window.inner_size().to_logical::<f32>(s);
    let outer_size = window.outer_size().to_logical::<f32>(s);
    let (left, top) = (inner.x - outer.x, inner.y - outer.y);
    let right = outer_size.width - inner_size.width - left;
    // O teclado cobre a barra de início: a faixa de baixo é o maior dos dois.
    let bottom = (outer_size.height - inner_size.height - top).max(ios_keyboard::covered());
    Some(Bounds::new(left.max(0.0), top.max(0.0), right.max(0.0), bottom.max(0.0)))
}

/// No desktop e no navegador, a janela inteira é do app.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn system_insets(_window: &Window) -> Option<Bounds> {
    None
}

/// Tamanho da superfície de desenho, em pixels. No iOS a view ocupa a tela toda (o
/// `inner_size` do winit lá é só a área segura); nos outros, é a área interna da janela.
fn surface_size(window: &Window) -> winit::dpi::PhysicalSize<u32> {
    #[cfg(web)]
    return web::physical_size(window);
    #[cfg(not(web))]
    if cfg!(target_os = "ios") { window.outer_size() } else { window.inner_size() }
}

/// A escala do desenho (pixels por ponto). No navegador vem da página na hora: o winit
/// demora a ver a troca (zoom, outro monitor) e o desenho saía com a escala velha.
fn draw_scale(window: &Window) -> f64 {
    #[cfg(web)]
    {
        let _ = window;
        return web::device_scale();
    }
    #[cfg(not(web))]
    window.scale_factor()
}

#[cfg(web)]
fn create_backend(
    event_loop: &ActiveEventLoop,
    attributes: WindowAttributes,
    _renderer: Renderer,
) -> Result<(Rc<Window>, Backend), Error> {
    web::create(event_loop, attributes).map(|(w, b)| (w, Backend::Web(b)))
}

/// Cria a janela já com o backend pedido; no modo automático, cai para a CPU se a GPU
/// falhar.
#[cfg(not(web))]
fn create_backend(
    event_loop: &ActiveEventLoop,
    attributes: WindowAttributes,
    renderer: Renderer,
    painters: &Painters,
    gpu: aylis_core::gpu::GpuChoice,
) -> Result<(Rc<Window>, Backend), Error> {
    let _ = (painters, &gpu);
    match renderer {
        #[cfg(native_d2d)]
        Renderer::D2d => {
            let vsync = diag::diagnostic_setting("AYLIS_BENCH").is_none();
            d2d::create(event_loop, attributes, &painters.d2d, vsync, gpu).map(|(w, b)| (w, Backend::D2d(b)))
        }
        #[cfg(native_gpu)]
        Renderer::Gpu => gpu::create(event_loop, attributes, gpu).map(|(w, b)| (w, Backend::Gpu(b))),
        #[cfg(native_cpu)]
        Renderer::Cpu => cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b))),
        #[cfg(all(native_gpu, native_cpu))]
        Renderer::Auto => match gpu::create(event_loop, attributes.clone(), gpu.clone()) {
            Ok((w, b)) => Ok((w, Backend::Gpu(b))),
            Err(e) => {
                aylis_core::stderr_line!("aylis: OpenGL indisponível ({e}); desenhando na CPU");
                cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b)))
            }
        },
        #[cfg(all(native_skia, not(all(native_gpu, native_cpu))))]
        Renderer::Auto => {
            #[cfg(native_gpu)]
            return create_backend(event_loop, attributes, Renderer::Gpu, painters, gpu);
            #[cfg(native_cpu)]
            return create_backend(event_loop, attributes, Renderer::Cpu, painters, gpu);
        }
        #[allow(unreachable_patterns)]
        other => Err(format!("renderizador {other} não incluído (veja as features do crate)").into()),
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.windows.is_empty() {
            return;
        }
        // Os monitores antes da primeira janela (ela pode abrir num monitor pedido).
        self.refresh_monitors(event_loop, true);
        if !self.started_once {
            // Primeiro layout no tamanho de design: é a referência das âncoras, mesmo
            // que o sistema abra a janela com outro tamanho.
            self.ui.layout();
        }
        self.sync_windows(event_loop);
        if self.windows.is_empty() {
            return; // falhou (o erro já foi guardado)
        }
        self.keyboard = false;
        self.watch_insets_until = Some(Instant::now() + Duration::from_millis(1500));
        self.after_input();
        if !self.started_once {
            self.started_once = true;
            (self.pump)(&mut self.ui);
        }
    }

    /// Android: o app foi para segundo plano e a superfície nativa vai ser destruída.
    /// Solta a janela e o contexto de desenho; `resumed` cria de novo na volta. O `Ui`
    /// (e tudo que está nele) continua intacto.
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if cfg!(target_os = "android") {
            for w in &self.windows {
                self.ui.pointer_cancel(w.form);
            }
            self.windows.clear();
        }
    }

    /// Um `Waker` acordou a interface: processa as mensagens pendentes.
    fn user_event(&mut self, event_loop: &ActiveEventLoop, _event: ()) {
        if let Some(w) = self.wakes.as_mut() {
            w.count("waker (bomba)");
        }
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        #[cfg(target_os = "android")]
        self.apply_keyboard_edits();
        #[cfg(target_os = "ios")]
        for index in 0..self.windows.len() {
            self.update_safe_area(index);
        }
        self.ui.process_posted();
        (self.pump)(&mut self.ui);
        self.after_input();
        if self.ui.close_requested() {
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(index) = self.index_of(id) else { return };
        #[cfg(not(web))]
        {
            let w = &mut self.windows[index];
            if let Some(access) = w.access.as_mut() {
                access.process_event(&w.window, &event);
            }
        }
        match event {
            WindowEvent::CloseRequested => {
                // Como o X do Delphi: passa pelo on_close_query; fechar o principal encerra.
                let form = self.windows[index].form;
                if SINGLE_SURFACE && self.ui.open_forms().len() <= 1 {
                    self.ui.request_close();
                } else {
                    self.ui.close(form);
                }
                if self.ui.close_requested() {
                    event_loop.exit();
                }
            }
            // A janela foi destruída por fora (a dona fechou antes de o Aylis passar a janela
            // para outra, ou outro processo): o form fecha, sem perguntar ao winit pelo tamanho
            // de uma janela que já não existe (o `inner_size` dela entra em pânico).
            #[cfg(not(any(web, mobile)))]
            WindowEvent::Destroyed => {
                let form = self.windows[index].form;
                self.windows.remove(index);
                if self.ui.is_open(form) {
                    self.ui.close(form);
                }
                if self.ui.close_requested() || self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                // Minimizou ou voltou: o form fica sabendo (o relatório do Tabernáculo, P06).
                let minimized = self.windows[index].window.is_minimized() == Some(true);
                if minimized != self.windows[index].minimized {
                    self.windows[index].minimized = minimized;
                    let form = self.windows[index].form;
                    self.ui.set_form_minimized(form, minimized);
                    self.ui.fire(form, if minimized { "on_minimize" } else { "on_restore" }, &aylis_core::EventArgs::None);
                }
                self.sync_form_size(index);
                self.store_placement(index);
                // Maximizar ou encaixar (Win+Shift+seta) também leva a outro monitor.
                self.follow_monitor(index, false);
                self.windows[index].window.request_redraw();
            }
            WindowEvent::Moved(_) => {
                self.store_placement(index);
                self.follow_monitor(index, false);
            }
            WindowEvent::RedrawRequested => {
                if let Some(w) = self.wakes.as_mut() {
                    w.count("quadro");
                }
                if let Err(e) = self.redraw(index) {
                    self.fail(event_loop, e);
                }
                #[cfg(target_os = "android")]
                self.sync_system_bars();
                // A tela mudou: o leitor de tela recebe a árvore nova (só se estiver ativo).
                #[cfg(not(web))]
                if let Some(w) = self.windows.get_mut(index) {
                    let (form, scale) = (w.form, w.window.scale_factor() * self.ui.form_zoom(w.form) as f64);
                    let ui = &self.ui;
                    if let Some(access) = w.access.as_mut() {
                        access.update_if_active(|| access::tree(ui, form, scale));
                    }
                }
            }
            other => {
                self.handle_input(index, &other);
                #[cfg(not(web))]
                self.paint_if_starved(event_loop);
            }
        }
    }

    /// O horário agendado por algum controle (ex.: cursor piscando) chegou.
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if let Some(w) = self.wakes.as_mut() {
            w.count(match cause {
                StartCause::ResumeTimeReached { .. } => "horário pedido",
                StartCause::WaitCancelled { .. } => "evento",
                StartCause::Poll => "poll",
                StartCause::Init => "início",
            });
        }
        if let StartCause::ResumeTimeReached { .. } = cause {
            // Se a área segura mudou, o layout fica pendente e o about_to_wait pede o frame.
            if self.watch_insets_until.is_some_and(|until| Instant::now() >= until) {
                self.watch_insets_until = None;
            }
            for index in 0..self.windows.len() {
                self.update_safe_area(index);
            }
            // Só um quadro pedido na pintura (o cursor piscando) repinta; um timer vencido roda no
            // about_to_wait e pede repintura se mudar algo (repintar a cada tique de um timer
            // deixava o app parado gastando CPU).
            let due = self.ui.next_paint_wake().is_some_and(|at| at <= self.started.elapsed().as_secs_f64() + 0.001);
            if due {
                for w in &self.windows {
                    w.window.request_redraw();
                }
            }
        }
    }

    /// Só pede um novo frame quando algo mudou ou há animação em andamento. Se algum
    /// controle agendou um frame para depois, dorme até lá. Parado, não consome CPU.
    /// O laço vai acabar (o X do principal, `request_close`): grava as posições guardadas.
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        #[cfg(not(any(web, mobile)))]
        self.placements.flush();
        self.ui.fire_terminate();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(w) = self.wakes.as_mut() {
            w.report(&self.ui);
        }
        // Sem form aberto, o app acaba, a não ser que viva na bandeja (P12).
        if self.ui.close_requested() || (self.started_once && self.ui.open_forms().is_empty() && !self.ui.has_tray_icon()) {
            self.quit(event_loop);
            return;
        }
        if self.started_once && !(cfg!(target_os = "android") && self.windows.is_empty()) {
            // Forms abertos ou fechados por código (show, show_modal, close).
            self.sync_windows(event_loop);
        }
        if self.windows.is_empty() {
            return;
        }
        self.apply_window_commands();
        self.refresh_monitors(event_loop, false);
        #[cfg(not(any(web, mobile)))]
        self.placements.flush_if_due();
        if self.ui.take_zoom_changed() {
            for index in 0..self.windows.len() {
                self.sync_form_size(index);
                self.windows[index].window.request_redraw();
            }
        }
        #[cfg(windows)]
        {
            let windows: Vec<(ComponentId, &Window)> = self.windows.iter().map(|w| (w.form, &*w.window)).collect();
            self.taskbars.sync(&mut self.ui, &windows);
        }
        #[cfg(all(native_gpu, target_os = "linux"))]
        self.follow_feedback();
        // O tema mudou entre claro e escuro: a barra de título do sistema acompanha (J04).
        let dark = self.ui.theme().dark;
        for w in &mut self.windows {
            if w.dark_title != dark {
                w.dark_title = dark;
                w.window.set_theme(Some(if dark { winit::window::Theme::Dark } else { winit::window::Theme::Light }));
            }
        }
        #[cfg(native_d2d)]
        for (form, force) in win32::take_gpu_checks() {
            if let Some(index) = self.windows.iter().position(|w| w.form == form) {
                self.follow_monitor(index, force);
            }
        }
        #[cfg(native_tray)]
        {
            let scale = self.windows.first().map_or(1.0, |w| w.window.scale_factor() as f32);
            let now = self.started.elapsed().as_secs_f64();
            self.tray_next = self.trays.sync(&mut self.ui, &self.tray_proxy, now, scale);
        }
        #[cfg(any(native_webview, web_webview, android_webview))]
        {
            #[cfg_attr(not(web), allow(unused_mut))]
            let mut windows: Vec<(ComponentId, Rc<Window>, aylis_core::PointF, bool)> = self.windows.iter().map(|w| (w.form, w.window.clone(), aylis_core::PointF::new(0.0, 0.0), true)).collect();
            // Navegador com janelas: os forms abertos por cima ficam no canvas do principal.
            #[cfg(web)]
            if self.floating_mode() {
                if let Some(w) = self.windows.first() {
                    let (base, window) = (w.form, w.window.clone());
                    let canvas = self.ui.control(base).map_or(aylis_core::SizeF::default(), |cb| aylis_core::SizeF::new(cb.width(), cb.height()));
                    windows = self.floating.origins(&mut self.ui, base, canvas).into_iter().map(|(form, at, shown)| (form, window.clone(), at, shown)).collect();
                }
            }
            self.native.sync(&mut self.ui, &windows);
        }
        // Navegador: a descrição da tela para o leitor de tela (elementos ARIA). Aqui, e não
        // na pintura, para valer também com a aba em segundo plano (sem quadros).
        #[cfg(web)]
        if let Some(w) = self.windows.first() {
            if self.web_access.is_none() {
                use winit::platform::web::WindowExtWebSys;
                if let Some(canvas) = w.window.canvas() {
                    self.web_access = web_access::WebAccess::new(canvas, self.ui.poster());
                }
            }
            let (form, ui, now) = (w.form, &self.ui, self.started.elapsed().as_secs_f64());
            if let Some(a) = self.web_access.as_mut() {
                a.update(now, || access::tree(ui, form, 1.0));
            }
        }
        // Timers (on_timer) e toques longos vencidos.
        // Timer `exact` a menos de `SPIN` do prazo: espera ativa até ele.
        if let Some(due) = self.ui.exact_timer_due() {
            let at = self.started + Duration::from_secs_f64(due.max(0.0));
            if at.saturating_duration_since(Instant::now()) <= aylis_core::timing::SPIN {
                while Instant::now() < at {
                    std::hint::spin_loop();
                }
            }
        }
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        self.ui.run_timers();
        // Um `on_timer` pode ter pedido para fechar: a conferência do começo já passou, e
        // nada acordaria o laço para a próxima (o relatório do Tabernáculo, R02).
        if self.ui.close_requested() {
            self.quit(event_loop);
            return;
        }
        if let Some(at) = self.frame_due().filter(|_| self.ui.needs_repaint()) {
            // Limite de FPS: espera a vez do próximo quadro.
            event_loop.set_control_flow(ControlFlow::WaitUntil(at));
        } else if self.ui.needs_repaint() {
            for w in &self.windows {
                w.window.request_redraw();
            }
            event_loop.set_control_flow(ControlFlow::Wait);
        } else {
            let wake = self.ui.next_wake().map(|at| self.started + Duration::from_secs_f64(at.max(0.0)));
            // O `exact` acorda um pouco antes e termina esperando ativo (acima).
            let exact = self.ui.exact_timer_due().map(|at| self.started + Duration::from_secs_f64(at.max(0.0)));
            let wake = wake.into_iter().chain(exact.map(|at| at.checked_sub(aylis_core::timing::SPIN).unwrap_or(at))).min();
            // Conferindo a área segura: acorda a cada 50 ms até o prazo.
            let watch = self.watch_insets_until.map(|_| Instant::now() + Duration::from_millis(50));
            // Ícone animado na bandeja: o próximo quadro.
            #[cfg(native_tray)]
            let watch = watch.or_else(|| self.tray_next.map(|at| self.started + Duration::from_secs_f64(at)));
            // Linux com WebView: o laço do GTK anda junto (a cada 16 ms).
            #[cfg(any(native_webview, web_webview, android_webview))]
            let watch = watch.or_else(|| self.native.needs_pump().then(|| Instant::now() + Duration::from_millis(16)));
            match wake.into_iter().chain(watch).min() {
                Some(at) => event_loop.set_control_flow(ControlFlow::WaitUntil(at)),
                None => event_loop.set_control_flow(ControlFlow::Wait),
            }
        }
    }
}

/// No Linux o OpenGL vem do Mesa, que escolhe a placa por variáveis de ambiente: o
/// `DRI_PRIME` (a dedicada de um notebook) e, no WSL, o nome do adaptador do D3D12. Só vale
/// se definidas antes de abrir o OpenGL; quem já as definiu manda.
#[cfg(target_os = "linux")]
fn mesa_gpu(choice: &aylis_core::gpu::GpuChoice) {
    use aylis_core::gpu::GpuChoice;
    let (var, value) = match choice {
        GpuChoice::HighPerformance => ("DRI_PRIME", "1".to_string()),
        GpuChoice::LowPower => ("DRI_PRIME", "0".to_string()),
        GpuChoice::Named(name) => ("MESA_D3D12_DEFAULT_ADAPTER_NAME", name.clone()),
        // O processo fica na placa do sistema; no Wayland, cada janela passa para a placa do
        // seu monitor (`gpu_linux.rs`, veja `docs/gpu-por-monitor.md`).
        GpuChoice::Auto | GpuChoice::FollowMonitor => return,
    };
    if std::env::var_os(var).is_none() {
        // SAFETY: na abertura do app, antes de o OpenGL e as threads de desenho existirem.
        unsafe { std::env::set_var(var, value) };
    }
}

/// As contagens do `AYLIS_WAKE_TRACE`: a cada segundo, uma linha no stderr com quantas vezes
/// o laço acordou por cada motivo e o próximo despertar pedido pelo `Ui`.
#[derive(Default)]
struct WakeTrace {
    since: Option<Instant>,
    counts: std::collections::BTreeMap<&'static str, u32>,
}

impl WakeTrace {
    fn count(&mut self, what: &'static str) {
        *self.counts.entry(what).or_default() += 1;
    }

    fn report(&mut self, ui: &Ui) {
        let since = *self.since.get_or_insert_with(Instant::now);
        if since.elapsed() < Duration::from_secs(1) {
            return;
        }
        let now = ui.time();
        let next = ui.next_wake().map(|at| format!("{:.3}s", at - now)).unwrap_or_else(|| "nenhum".into());
        aylis_core::stderr_line!("aylis wake: {:?} | repintar={} | próximo despertar em {next}", self.counts, ui.needs_repaint());
        self.counts.clear();
        self.since = Some(Instant::now());
    }
}
