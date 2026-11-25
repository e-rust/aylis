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

/// Os cantos da janela pedidos ao DWM do Windows 11 (`DWM_WINDOW_CORNER_PREFERENCE`: 0 o do
/// sistema, 1 retos, 2 arredondados, 3 o raio pequeno), pelas propriedades `window_corners` e
/// `border_style` do form. O `default` com a barra própria (`custom`) arredonda como uma janela
/// comum (sem moldura, o sistema deixaria reto); com as outras bordas, fica com o sistema.
pub fn corner_preference(corners: &str, border_style: &str) -> i32 {
    match corners {
        "square" => 1,
        "round" => 2,
        "small" => 3,
        _ if border_style == "custom" => 2,
        _ => 0,
    }
}

#[cfg(test)]
mod corner_tests {
    use super::corner_preference;

    #[test]
    fn the_custom_title_bar_gets_rounded_corners_by_default() {
        assert_eq!(corner_preference("default", "custom"), 2);
        assert_eq!(corner_preference("default", "none"), 0, "a tela sem moldura fica como o sistema deixa");
        assert_eq!(corner_preference("default", "sizeable"), 0);
        assert_eq!(corner_preference("square", "custom"), 1);
        assert_eq!(corner_preference("small", "none"), 3);
        assert_eq!(corner_preference("round", "single"), 2);
    }
}
#[cfg(native_gpu)]
mod gpu;
#[cfg(all(native_gpu, target_os = "linux"))]
mod gpu_linux;
mod access;
#[cfg(native_clipboard)]
mod clipboard;
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
#[cfg(any(test, not(any(web, mobile))))]
mod window_icon;
#[cfg(windows)]
mod notify;
#[cfg(windows)]
mod url_scheme;
#[cfg(windows)]
mod share_windows;
#[cfg(all(target_os = "linux", not(mobile)))]
mod url_scheme_linux;
mod input;
mod cursor_images;
mod keyboard_capture;
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
#[cfg(windows)]
mod windows_media;
#[cfg(all(target_os = "linux", not(target_os = "android")))]
mod linux_media;
#[cfg(all(feature = "nfc", not(any(web, mobile))))]
mod desktop_nfc;
#[cfg(windows)]
mod win_ole;
#[cfg(windows)]
mod win_wmi;
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
mod web_media;
#[cfg(web)]
mod web_radio;
#[cfg(web)]
mod web_push;
#[cfg(any(native_webview, web_webview, android_webview))]
mod native;
#[cfg(native_tray)]
mod tray;
mod desktop;
#[cfg(not(any(web, mobile)))]
mod single_instance;
#[cfg(not(any(web, mobile)))]
pub use single_instance::{claim_single_instance, claim_single_instance_with};
mod login;
pub use login::{set_app_identity, set_start_with_login, start_with_login};

/// Solta a trava da instância única antes de o app relançar a si mesmo (veja
/// [`aylis_core::Ui::release_single_instance`]). No celular e no navegador, não há trava.
pub fn release_single_instance() {
    #[cfg(not(any(web, mobile)))]
    single_instance::release_single_instance();
}
#[cfg(any(native_tray, windows))]
mod badge;
#[cfg(windows)]
mod win32;
// Um form em vários monitores (uma janela por monitor).
mod span;
mod services;
pub use services::{install_offscreen_service, install_text_service};
use services::*;
mod window_commands;
mod window_input;
mod window_lifecycle;
mod window_monitors;
mod window_reports;
mod system_bars;
#[cfg(windows)]
mod drag_window;
#[cfg(windows)]
mod hand_cursors;
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

/// Depurar na IDE (o app em Rust no Windows): com `AYLIS_WAIT_DEBUGGER=<segundos>`, espera o
/// depurador se anexar (a IDE recebe o número do processo do `aylis-tool`), para os pontos de
/// parada valerem desde o começo. No Windows, o CodeLLDB que lança o app trava antes da
/// janela abrir (22/11); anexado ao app já aberto, não. Nas outras plataformas, e sem a
/// variável, não faz nada.
pub fn wait_for_debugger() {
    #[cfg(windows)]
    {
        let Some(secs) = std::env::var("AYLIS_WAIT_DEBUGGER").ok().and_then(|s| s.parse::<u64>().ok()) else { return };
        unsafe extern "system" {
            fn IsDebuggerPresent() -> i32;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
        // SAFETY: sem argumentos; só lê o estado do próprio processo.
        while unsafe { IsDebuggerPresent() } == 0 {
            if std::time::Instant::now() >= deadline {
                eprintln!("aylis: o depurador não se anexou em {secs} s; seguindo sem ele");
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        // O depurador manda os pontos de parada logo depois de se anexar.
        std::thread::sleep(std::time::Duration::from_millis(700));
    }
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
    /// Linux: o identificador do app (o `app_id` do Wayland e o `WM_CLASS` do X11), o nome do
    /// `.desktop` e o `StartupWMClass` do pacote. Vazio: o nome do executável.
    pub app_id: String,
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
    #[allow(unused_mut)]
    let mut builder = EventLoop::builder();
    // Linux: vai pelo X11 quando o Wayland não serve (veja `prefer_x11`).
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    if prefer_x11() {
        use winit::platform::x11::EventLoopBuilderExtX11;
        builder.with_x11();
    }
    run_on(builder.build()?, ui, form, options, make_pump)
}

/// Há um X11 (`DISPLAY`) e o Wayland pedido não serve:
/// - no WSL: o compositor do WSLg (o weston) cai com uma janela desenhada pelo EGL no Wayland
///   ao lado de outra (testado em 19/11: o app com a IDE aberta, ou com o `glxgears`), e todos
///   os apps do Linux caem junto. Pelo X11 (o Xwayland do WSLg) não cai, com o d3d12 ou o
///   llvmpipe; o Executar da IDE do Windows no WSL já ia por ele (`aylis_tools::wsl`);
/// - o `WAYLAND_DISPLAY` aponta para um soquete que não existe, nem no `XDG_RUNTIME_DIR` nem
///   como caminho absoluto (o WSL rodando como root: o WSLg liga o soquete só no diretório do
///   usuário dele). Sem o X11, o app não abriria.
#[cfg(all(target_os = "linux", not(target_os = "android")))]
fn prefer_x11() -> bool {
    let Some(display) = std::env::var_os("WAYLAND_DISPLAY").filter(|d| !d.is_empty()) else { return false };
    if std::env::var_os("DISPLAY").is_none_or(|d| d.is_empty()) {
        return false;
    }
    if is_wsl() {
        return true;
    }
    let display = std::path::PathBuf::from(display);
    let socket = if display.is_absolute() { display } else { std::env::var_os("XDG_RUNTIME_DIR").map_or(display.clone(), |r| std::path::PathBuf::from(r).join(&display)) };
    !socket.exists()
}

/// O ponto (pixels físicos da tela) cai no monitor.
fn contains(monitor: &winit::monitor::MonitorHandle, x: i32, y: i32) -> bool {
    let (at, size) = (monitor.position(), monitors::size(monitor));
    x >= at.x && y >= at.y && x < at.x + size.width as i32 && y < at.y + size.height as i32
}

/// Roda no WSL (o núcleo do Linux da Microsoft).
#[cfg(all(target_os = "linux", not(target_os = "android")))]
fn is_wsl() -> bool {
    std::env::var_os("WSL_DISTRO_NAME").is_some()
        || std::fs::read_to_string("/proc/sys/kernel/osrelease").is_ok_and(|r| r.to_ascii_lowercase().contains("microsoft"))
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
    // Os argumentos das outras aberturas (o `single_instance` do projeto) chegam por aqui.
    #[cfg(not(any(web, mobile)))]
    single_instance::attach(ui.poster());
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
    // A área de trabalho (o mouse fora das janelas, a janela sob um ponto, a transparência, a
    // captura de um monitor) e os "Efeitos de transparência" de agora.
    #[cfg(not(any(web, mobile)))]
    if ui.services().desktop.is_none() {
        ui.services_mut().desktop = Some(Box::new(desktop::Desktop));
    }
    if let Some(on) = ui.services().desktop.as_ref().and_then(|d| d.transparency()) {
        ui.set_system_transparency(on);
    }
    // A sessão de agora (bloqueada, conectada): o `on_session_change` conta a partir daqui.
    if let Some((locked, connected)) = ui.services().desktop.as_ref().and_then(|d| d.session_state()) {
        ui.set_system_session(locked, connected);
    }
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
    // O form passado é o principal.
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
        // O quadrado do meio, como o `.ico` (uma imagem larga não sai achatada).
        .map(|(pixels, w, h)| window_icon::square(&pixels, w, h))
        .and_then(|(pixels, side)| winit::window::Icon::from_rgba(pixels, side, side).ok());
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
        #[cfg(any(target_os = "ios", web))]
        system_bars_applied: 0,
        #[cfg(mobile)]
        orientation_applied: None,
        tapped: false,
        started_once: false,
        #[cfg(windows)]
        drag_window: Default::default(),
        #[cfg(windows)]
        label_window: Default::default(),
        #[cfg(windows)]
        hand_cursors: Vec::new(),
        cursor_images: Default::default(),
        keyboard_held: None,
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
        keep_awake: 0,
        hovered_files: Vec::new(),
        spans: Default::default(),
        span_checks: Vec::new(),
        deactivations: Vec::new(),
        recorded_placements: Default::default(),
        pointer_down: false,
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

/// Traz a janela do form para a frente com o foco: pelo caminho forçado no Windows quando o
/// form pede (`force_foreground`, [`Ui::forces_foreground`]: o processo sem a última entrada,
/// como o form aberto por um atalho global); senão, e nas outras plataformas, o pedido do winit.
fn bring_to_front(ui: &Ui, form: ComponentId, window: &Window) {
    #[cfg(windows)]
    if ui.forces_foreground(form) {
        win32::force_foreground(window);
        return;
    }
    let _ = (ui, form);
    window.focus_window();
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
    /// O id do cursor de imagem mostrado (`CursorImage::id`; 0: o da forma).
    cursor_image: u64,
    /// Aceita entrada (fica desligada debaixo de um form modal).
    enabled: bool,
    /// `border_style = custom`: a borda sob o mouse (redimensiona ao apertar).
    edge: Option<winit::window::ResizeDirection>,
    /// O título mostrado (o `caption` do form muda e a janela acompanha).
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
    /// O form em vários monitores (`span_monitors`): o pedaço que esta janela mostra.
    span: Option<span::SpanPiece>,
    /// A janela tem o foco (os pedaços de um form: o `on_deactivate` só sai quando nenhum tem).
    focused: bool,
    /// O mouse está sobre ela (os pedaços de um form: o `pointer_leave` só sai quando o mouse
    /// sai de todos).
    hovered: bool,
    /// Windows: mostrada encoberta (`DWMWA_CLOAK`) e descoberta depois do próximo quadro
    /// pintado (o form que nasceu escondido aparecendo sem o branco).
    #[cfg_attr(not(windows), allow(dead_code))]
    uncloak_after_draw: bool,
    /// Windows: o mouse está preso no monitor por esta janela (`confine_cursor = monitor`).
    #[cfg_attr(not(windows), allow(dead_code))]
    clipped_monitor: bool,
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
    /// Android: o que já foi mandado às barras do sistema (as cores, os ícones escuros e o que
    /// aparece); iOS e navegador: a versão do pedido (`SystemBars`) já aplicada.
    #[cfg(target_os = "android")]
    system_bars: Option<(aylis_core::Color, aylis_core::Color, bool, bool, bool, bool)>,
    #[cfg(any(target_os = "ios", web))]
    system_bars_applied: u64,
    /// Celular: a orientação já pedida ao sistema (`Ui::requested_orientation`).
    #[cfg(mobile)]
    orientation_applied: Option<aylis_core::ScreenOrientation>,
    /// Houve um toque: se o foco está num campo, pede o teclado de novo (o usuário pode
    /// tê-lo fechado com o "voltar" sem o campo perder o foco).
    tapped: bool,
    /// Já abriu a janela alguma vez (no Android ela é recriada a cada volta ao app).
    started_once: bool,
    /// A imagem do arrasto numa janela própria (segue o mouse pela tela inteira).
    #[cfg(windows)]
    drag_window: drag_window::DragWindow,
    /// O rótulo do arrasto numa janela própria, como a imagem (segue o mouse fora da janela de
    /// origem também), e o texto e a escala que ela mostra.
    #[cfg(windows)]
    label_window: drag_window::DragWindow,
    /// Os cursores de mão (`Grab`, `Grabbing`) desenhados, por escala inteira: o winit dá a seta
    /// de mover no Windows, que não tem mão aberta nem fechada no sistema.
    #[cfg(windows)]
    hand_cursors: Vec<(u32, winit::window::CustomCursor, winit::window::CustomCursor)>,
    /// Os cursores de imagem (`Ui::set_cursor_image`) já criados no sistema.
    cursor_images: cursor_images::CursorImages,
    /// O teclado preso para um `KeyboardCapture` (veja `keyboard_capture.rs`).
    keyboard_held: Option<keyboard_capture::Held>,
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
    /// O `keep_awake` pedido ao sistema agora (o maior dos forms abertos: 0, 1 ou 2).
    keep_awake: u8,
    /// Os arquivos do sistema passando por cima de uma janela agora.
    hovered_files: Vec<String>,
    pump: Pump,
    /// `AYLIS_WAKE_TRACE`: por que o laço acordou, contado por segundo (para achar o que
    /// gasta CPU com o app parado).
    wakes: Option<WakeTrace>,
    /// A taxa do monitor e quando foi perguntada (pedir ao sistema leva milissegundos, e o
    /// arraste com um mouse rápido pergunta a cada evento).
    display_hz: std::cell::Cell<Option<(Instant, f64)>>,
    /// Os forms em vários monitores (`span.rs`).
    spans: std::collections::HashMap<ComponentId, span::SpanForm>,
    /// Os forms que conferem, na próxima volta do laço, se viram janelas por monitor.
    span_checks: Vec<ComponentId>,
    /// Os forms com um pedaço que perdeu o foco (o `on_deactivate` sai se nenhum ganhou).
    deactivations: Vec<ComponentId>,
    /// O último lugar de cada janela que a própria plataforma anotou (ao mover, ao mudar de
    /// tamanho): ao reabrir, só ele não vence o `position` do form, salvo com
    /// `remember_placement`. O que o app pôs com `set_window_placement` vale sempre.
    recorded_placements: std::collections::HashMap<ComponentId, aylis_core::WindowPlacement>,
    /// Um botão do mouse está apertado numa janela do app (a borda de redimensionar não é
    /// procurada no meio de um arrasto).
    pointer_down: bool,
}

impl App {
    fn index_of(&self, id: WindowId) -> Option<usize> {
        self.windows.iter().position(|w| w.window.id() == id)
    }

    /// Quando o laço tem de acordar pelos timers (o `exact` um pouco antes, e termina esperando
    /// ativo no `about_to_wait`).
    fn next_wake(&self) -> Option<Instant> {
        let wake = self.ui.next_wake().map(|at| self.started + Duration::from_secs_f64(at.max(0.0)));
        let exact = self.ui.exact_timer_due().map(|at| self.started + Duration::from_secs_f64(at.max(0.0)));
        wake.into_iter().chain(exact.map(|at| at.checked_sub(aylis_core::timing::SPIN).unwrap_or(at))).min()
    }

    fn window_of(&self, form: ComponentId) -> Option<&FormWindow> {
        self.windows.iter().find(|w| w.form == form)
    }

    /// A janela recém-criada do form `form` passa a ser a dona das janelas abertas que o têm
    /// como dono no `Ui` e estão sem dona no sistema (a dona delas fechou antes de esta abrir).
    #[cfg(windows)]
    fn adopt_owned_windows(&self, index: usize) {
        let Some(owner) = self.windows.get(index) else { return };
        for w in self.windows.iter().filter(|w| w.form != owner.form && !w.is_extra()) {
            if self.ui.form_owner(w.form) == Some(owner.form) && win32::owner_of(&w.window).is_none() {
                win32::set_owner(&w.window, Some(&owner.window));
            }
        }
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
                // A dona passa a ser o principal mesmo sem a janela dele ainda (a abertura que
                // fecha logo antes de a janela principal abrir): ela assume esta ao abrir
                // (`adopt_owned_windows`). Sem isso, a pergunta aberta durante a abertura ficava
                // sem dona e ia para trás da janela que ela bloqueia (relatado em 20/11).
                let main_form = self.ui.main_form().filter(|m| !closing.contains(m) && *m != w.form);
                self.ui.set_form_owner(w.form, main_form);
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

    fn redraw(&mut self, index: usize) -> Result<(), Error> {
        let Some(w) = self.windows.get(index) else { return Ok(()) };
        // Um form em vários monitores pinta os pedaços juntos (`span.rs`).
        if w.span.is_some() && self.spans.contains_key(&w.form) {
            return self.redraw_span(index);
        }
        // O `frame_rate` do form: antes da vez dele, o quadro fica para depois (o que mudou
        // continua marcado, e o laço volta a pedir).
        if let Ok(PropValue::Float(rate)) = self.ui.get_prop(w.form, "frame_rate")
            && rate > 0.0
            && self.started.elapsed().as_secs_f64() - w.last_draw < 1.0 / rate as f64 - 0.0005
        {
            return Ok(());
        }
        let now = self.started.elapsed().as_secs_f64();
        // O `max_paint_rate` do form: o que mudou sem entrada do usuário espera a vez (continua
        // marcado, e o `next_wake` do `Ui` acorda o laço nela). Todo pedido de quadro passa por
        // aqui, inclusive os que não perguntaram ao `forms_to_paint`.
        self.ui.set_time(now);
        if self.ui.paint_held_until(w.form).is_some() {
            return Ok(());
        }
        if let Some(w) = self.windows.get_mut(index) {
            w.last_draw = now;
        }
        let Some(w) = self.windows.get(index) else { return Ok(()) };
        let size = surface_size(&w.window);
        let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return Ok(()); // janela minimizada
        };
        let size = (width, height);
        let scale = self.system_scale(index) as f32 * self.ui.form_zoom(w.form);
        self.update_safe_area(index);
        self.ui.set_time(self.started.elapsed().as_secs_f64());
        self.ui.tick();
        // Arrastando a borda (Windows), o laço modal do sistema só manda Resized e pintura, sem
        // passar pelo about_to_wait: o `set_timeout` que o on_resize agendou (o app refazendo a
        // tela pela largura nova) roda aqui, antes deste quadro, e não só ao soltar o mouse.
        self.ui.update_layout();
        self.ui.run_timers();
        // O que os relógios mudaram (o `set_timeout` do on_resize) assenta antes do quadro
        // (L02 do Tabernáculo): sem isso, um quadro saía com a largura de antes.
        self.ui.update_layout();
        // A janela transparente começa de nada: o que o form não pinta deixa ver o que está atrás.
        let background = if self.see_through(self.windows[index].form) { aylis_core::Color::TRANSPARENT } else { self.ui.theme().color("background") };
        // A cor do apagar do Windows acompanha o form (o tema pode ter mudado).
        #[cfg(windows)]
        win32::set_background(self.windows[index].form, self.erase_color(self.windows[index].form));
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
            Backend::D2d(d2d) => {
                // O `frame_reuse_kb` do form desta janela (ou o do projeto), lido a cada quadro.
                d2d.set_frame_reuse_kb(ui.form_frame_reuse_kb(form));
                d2d.set_blend_compositor(ui.form_blend_compositor(form));
                d2d.draw(&self.renderer.d2d, size, scale, background, parts, |c, area| paint_stack(ui, under, form, c, area), capture)?
            }
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
        // A janela mostrada encoberta aparece já com este quadro.
        #[cfg(windows)]
        if let Some(w) = self.windows.get_mut(index).filter(|w| w.uncloak_after_draw) {
            w.uncloak_after_draw = false;
            win32::set_cloaked(&w.window, false);
        }
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
        let forms: Vec<ComponentId> = self.windows.iter().map(|w| w.form).collect();
        let paint = self.ui.forms_to_paint(&forms);
        for index in 0..self.windows.len() {
            if !paint.contains(&self.windows[index].form) || self.is_extra(index) {
                continue;
            }
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

/// Aplica numa janela as opções do `WindowSettings` ([`aylis_core::WindowOptions`]). O que o
/// sistema não tem não faz nada (o winit ignora; o resto é só do Windows).
#[cfg(not(any(web, mobile)))]
pub(crate) fn apply_window_options(window: &Window, o: &aylis_core::WindowOptions) {
    use winit::window::WindowButtons;
    window.set_content_protected(o.capture_excluded);
    let _ = window.set_cursor_hittest(!o.click_through);
    let mut buttons = WindowButtons::CLOSE;
    if o.minimize_button {
        buttons |= WindowButtons::MINIMIZE;
    }
    if o.maximize_button {
        buttons |= WindowButtons::MAXIMIZE;
    }
    window.set_enabled_buttons(buttons);
    window.set_blur(o.blur);
    #[cfg(windows)]
    {
        use aylis_core::WindowBackdrop;
        use winit::platform::windows::{BackdropType, Color as WinColor, WindowExtWindows};
        let color = |c: aylis_core::Color| WinColor::from_rgb(c.r, c.g, c.b);
        window.set_system_backdrop(match o.backdrop {
            WindowBackdrop::System => BackdropType::Auto,
            WindowBackdrop::None => BackdropType::None,
            WindowBackdrop::Mica => BackdropType::MainWindow,
            WindowBackdrop::Acrylic => BackdropType::TransientWindow,
            WindowBackdrop::Tabbed => BackdropType::TabbedWindow,
        });
        // Vazio é a cor do sistema: o `None` do winit tiraria a cor (a borda sumiria).
        window.set_title_background_color(Some(o.title_color.map_or(WinColor::SYSTEM_DEFAULT, color)));
        window.set_title_text_color(o.title_text_color.map_or(WinColor::SYSTEM_DEFAULT, color));
        // A borda: a do sistema, nenhuma (o `None` do winit é o `DWMWA_COLOR_NONE`: some também a
        // linha de 1 px em volta da tela cheia) ou a cor.
        window.set_border_color(match o.border_color {
            aylis_core::BorderColor::System => Some(WinColor::SYSTEM_DEFAULT),
            aylis_core::BorderColor::None => None,
            aylis_core::BorderColor::Color(c) => Some(color(c)),
        });
        window.set_undecorated_shadow(o.shadow);
        win32::set_window_animation(window, o.animation);
        win32::set_window_ex_styles(window, o.no_activate, !o.alt_tab);
    }
}

/// O modo automático achou o OpenGL por software numa janela: as outras vão direto para a CPU.
#[cfg(all(native_gpu, native_cpu))]
static SOFTWARE_GL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// O `GL_RENDERER` é de um desenho por software: o llvmpipe, o softpipe e o swrast do Mesa, e
/// os do Windows sem driver de vídeo (o "GDI Generic" e o "Basic Render").
#[cfg_attr(not(all(native_gpu, native_cpu)), allow(dead_code))]
fn software_gl(renderer: &str) -> bool {
    let r = renderer.to_ascii_lowercase();
    ["llvmpipe", "softpipe", "swrast", "software rasterizer", "gdi generic", "basic render"].iter().any(|s| r.contains(s))
}

/// Cria a janela já com o backend pedido; no modo automático, cai para a CPU se a GPU
/// falhar ou for por software.
#[cfg(not(web))]
fn create_backend(
    event_loop: &ActiveEventLoop,
    attributes: WindowAttributes,
    renderer: Renderer,
    painters: &Painters,
    gpu: aylis_core::gpu::GpuChoice,
    present: aylis_core::gpu::PresentMode,
) -> Result<(Rc<Window>, Backend), Error> {
    let _ = (painters, &gpu, present);
    match renderer {
        #[cfg(native_d2d)]
        Renderer::D2d => {
            // `AYLIS_VSYNC=0`: sem esperar o monitor fora do `AYLIS_BENCH` também (o exemplo
            // `transitions`, que mede o custo de cada quadro de uma animação).
            let vsync = diag::diagnostic_setting("AYLIS_BENCH").is_none() && diag::diagnostic_setting("AYLIS_VSYNC").as_deref() != Some("0");
            d2d::create(event_loop, attributes, &painters.d2d, vsync, gpu, present).map(|(w, b)| (w, Backend::D2d(b)))
        }
        #[cfg(native_gpu)]
        Renderer::Gpu => gpu::create(event_loop, attributes, gpu).map(|(w, b)| (w, Backend::Gpu(b))),
        #[cfg(native_cpu)]
        Renderer::Cpu => cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b))),
        // O OpenGL por software (o llvmpipe de uma VM sem placa, de um servidor): desenhar nele
        // é bem mais lento que o Skia direto na CPU, e a IDE ficava segundos sem responder
        // (testado numa VM do Hyper-V com o Fedora em 20/11). A primeira janela que o achar
        // decide pelas outras.
        #[cfg(all(native_gpu, native_cpu))]
        Renderer::Auto if SOFTWARE_GL.load(std::sync::atomic::Ordering::Relaxed) => cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b))),
        #[cfg(all(native_gpu, native_cpu))]
        Renderer::Auto => match gpu::create(event_loop, attributes.clone(), gpu.clone()) {
            Ok((w, b)) if b.adapter().is_some_and(|a| software_gl(&a)) => {
                aylis_core::stderr_line!("aylis: o OpenGL é por software ({}); desenhando na CPU", b.adapter().unwrap_or_default());
                SOFTWARE_GL.store(true, std::sync::atomic::Ordering::Relaxed);
                drop(b);
                drop(w);
                cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b)))
            }
            Ok((w, b)) => Ok((w, Backend::Gpu(b))),
            Err(e) => {
                aylis_core::stderr_line!("aylis: OpenGL indisponível ({e}); desenhando na CPU");
                cpu::create(event_loop, attributes).map(|(w, b)| (w, Backend::Cpu(b)))
            }
        },
        #[cfg(all(native_skia, not(all(native_gpu, native_cpu))))]
        Renderer::Auto => {
            #[cfg(native_gpu)]
            return create_backend(event_loop, attributes, Renderer::Gpu, painters, gpu, present);
            #[cfg(native_cpu)]
            return create_backend(event_loop, attributes, Renderer::Cpu, painters, gpu, present);
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
        #[cfg(windows)]
        if self.hand_cursors.is_empty() {
            self.hand_cursors = hand_cursors::create(event_loop);
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
        // A escala e o tamanho novos: o `on_scale_change` e o `on_resize_end` do form.
        if matches!(event, WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }) {
            self.report_window_change(index, &event);
            // Arrastar a borda é entrada: pinta na hora, sem o `max_paint_rate` do form.
            self.ui.note_input();
        }
        match event {
            WindowEvent::CloseRequested => {
                // O X passa pelo on_close_query; fechar o principal encerra.
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
            // Um pedaço de um form em vários monitores fica do tamanho do pedaço, mesmo num
            // monitor com outra escala (o sistema sugere outro tamanho).
            WindowEvent::ScaleFactorChanged { mut inner_size_writer, .. } if self.windows[index].span.is_some() => {
                if let Some(s) = self.windows[index].span {
                    let _ = inner_size_writer.request_inner_size(PhysicalSize::new(s.piece.screen.width, s.piece.screen.height));
                }
                self.follow_monitor(index, false);
                self.windows[index].window.request_redraw();
            }
            // Um form em pixels físicos (`physical_pixels`) num monitor de outra escala: a janela
            // fica com os mesmos pixels (o form não muda de tamanho), em vez do que o sistema
            // sugere.
            WindowEvent::ScaleFactorChanged { mut inner_size_writer, .. } if self.ui.physical_pixels(self.windows[index].form) => {
                let _ = inner_size_writer.request_inner_size(self.windows[index].window.inner_size());
                self.sync_form_size(index);
                self.follow_monitor(index, false);
                self.windows[index].window.request_redraw();
            }
            WindowEvent::Resized(_) if self.is_extra(index) => {
                self.follow_monitor(index, false);
                self.windows[index].window.request_redraw();
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => self.window_resized(index),
            WindowEvent::Moved(_) => {
                self.store_placement(index);
                self.follow_monitor(index, false);
                // A janela de um form `dockable` arrastada pela barra de título: o destino sob o
                // mouse (a faixa de abas de outra janela do app) acende.
                #[cfg(windows)]
                self.window_drag_over(index);
                #[cfg(not(any(web, mobile)))]
                if self.windows[index].focused {
                    self.apply_cursor_confine(index);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(w) = self.wakes.as_mut() {
                    w.count("quadro");
                }
                if let Err(e) = self.redraw(index) {
                    self.fail(event_loop, e);
                }
                #[cfg(any(target_os = "android", target_os = "ios", web))]
                self.sync_system_bars();
                #[cfg(mobile)]
                self.sync_orientation();
                // A tela mudou: o leitor de tela recebe a árvore nova (só se estiver ativo).
                #[cfg(not(web))]
                if let Some(w) = self.windows.get_mut(index) {
                    let system = if self.ui.physical_pixels(w.form) { 1.0 } else { w.window.scale_factor() };
                    let (form, scale) = (w.form, system * self.ui.form_zoom(w.form) as f64);
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
            let now = self.started.elapsed().as_secs_f64() + 0.001;
            let due = self.ui.next_paint_wake().is_some_and(|at| at <= now);
            if due {
                // Quem pediu o quadro desta hora (o vídeo, o cursor) é marcado, e só a janela
                // dele pinta, só a área dele. Sem dono (um pedido de fora de um controle),
                // todas, como antes.
                let forms: Vec<ComponentId> = self.windows.iter().map(|w| w.form).collect();
                let paint = if self.ui.take_due_frames(now) { self.ui.forms_to_paint(&forms) } else { forms };
                for w in self.windows.iter().filter(|w| paint.contains(&w.form) && !w.is_extra()) {
                    w.window.request_redraw();
                }
            }
        }
    }

    /// Só pede um novo frame quando algo mudou ou há animação em andamento. Se algum
    /// controle agendou um frame para depois, dorme até lá. Parado, não consome CPU.
    /// O laço vai acabar (o X do principal, `request_close`): grava as posições guardadas.
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.release_keyboard();
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
        #[cfg(windows)]
        self.caption_buttons();
        // O teclado preso (o `KeyboardCapture`) segue a janela em foco; os cursores de imagem
        // pedidos no `after_input` são criados aqui (o winit pede o `ActiveEventLoop`).
        self.sync_keyboard_capture();
        if self.cursor_images.create_missing(event_loop) {
            self.after_input();
        }
        self.settle_deactivations();
        self.apply_window_commands();
        self.refresh_monitors(event_loop, false);
        // Os forms em vários monitores: viram (ou deixam de ser) uma janela por monitor.
        if !self.span_checks.is_empty() {
            self.check_spans(event_loop);
        }
        #[cfg(not(any(web, mobile)))]
        self.placements.flush_if_due();
        if self.ui.take_zoom_changed() {
            for index in 0..self.windows.len() {
                self.sync_form_size(index);
                self.windows[index].window.request_redraw();
            }
            // Com outro zoom, os pedaços mudam de lugar no form.
            let spanned: Vec<ComponentId> = self.spans.keys().copied().collect();
            self.span_checks.extend(spanned);
        }
        #[cfg(windows)]
        {
            let windows: Vec<(ComponentId, &Window)> = self.windows.iter().filter(|w| !w.is_extra()).map(|w| (w.form, &*w.window)).collect();
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
        // A janela de um form `dockable` foi solta: entra no destino sob o mouse, se havia.
        #[cfg(windows)]
        for form in win32::take_move_ends() {
            self.ui.window_drag_end(form);
            // Soltou a borda depois de redimensionar: o `on_resize_end`.
            self.ui.form_resize_released(form);
        }
        // O mouse foi tomado no meio de um gesto (outro programa capturou, a troca de janela com
        // o botão apertado): o soltar não vai chegar, e o arrasto acaba como no Esc.
        #[cfg(windows)]
        for form in win32::take_capture_losses() {
            self.gesture_lost(form);
        }
        // A composição do sistema mudou: o desfoque (`WindowSettings.blur`) é refeito com um
        // compositor novo.
        // A sessão mudou (bloqueio, desbloqueio, a sessão remota conectando ou saindo): o
        // `on_session_change` do `Screen`.
        #[cfg(windows)]
        if win32::take_session_changed() {
            if let Some((locked, connected)) = win32::session_state() {
                self.ui.set_system_session(locked, connected);
            }
        }
        #[cfg(windows)]
        if win32::take_composition_changed() {
            // Os "Efeitos de transparência" podem ter mudado (o `on_transparency_change` do `Screen`).
            if let Some(on) = win32::transparency_effects() {
                self.ui.set_system_transparency(on);
            }
            if win32::reset_blurs() {
                for index in 0..self.windows.len() {
                    self.apply_shape(index);
                }
            }
        }
        // Outra janela sempre por cima passou à frente de um form `topmost`: o evento leva quem
        // cobriu (o registro `TopmostLost`, em JSON).
        #[cfg(windows)]
        for (form, by) in win32::take_topmost_lost() {
            if self.ui.contains(form) {
                self.ui.fire(form, "on_topmost_lost", &aylis_core::EventArgs::Text(by));
                self.ui.process_events();
            }
        }
        #[cfg(native_d2d)]
        for (form, force) in win32::take_gpu_checks() {
            // Todas as janelas do form (os pedaços de um form em vários monitores também).
            let indices: Vec<usize> = (0..self.windows.len()).filter(|&i| self.windows[i].form == form).collect();
            for index in indices {
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
            // Num form em vários monitores, os navegadores embutidos ficam na janela principal
            // (com a origem dela no form).
            let mut windows: Vec<(ComponentId, Rc<Window>, aylis_core::PointF, bool)> = self
                .windows
                .iter()
                .filter(|w| !w.is_extra())
                .map(|w| {
                    let origin = match (w.span, self.spans.contains_key(&w.form)) {
                        (Some(s), true) => {
                            let scale = self.input_scale(self.index_of(w.window.id()).unwrap_or(0));
                            aylis_core::PointF::new((-s.piece.origin_x as f64 / scale) as f32, (-s.piece.origin_y as f64 / scale) as f32)
                        }
                        _ => aylis_core::PointF::new(0.0, 0.0),
                    };
                    (w.form, w.window.clone(), origin, true)
                })
                .collect();
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
        // Os pedidos à janela feitos pelos timers (o `Show` do form escondido) valem já: com as
        // janelas escondidas, nada mais acordaria o laço para a próxima volta.
        self.apply_window_commands();
        if let Some(at) = self.frame_due().filter(|_| self.ui.needs_repaint()) {
            // Limite de FPS: espera a vez do próximo quadro.
            event_loop.set_control_flow(ControlFlow::WaitUntil(at));
        } else if self.ui.needs_repaint() {
            // Só as janelas com o que pintar (um quadro de câmera não repinta as outras).
            let forms: Vec<ComponentId> = self.windows.iter().map(|w| w.form).collect();
            let paint = self.ui.forms_to_paint(&forms);
            for w in self.windows.iter().filter(|w| paint.contains(&w.form) && !w.is_extra()) {
                w.window.request_redraw();
            }
            // A janela escondida não pinta (o pedido de quadro não chega): os timers continuam
            // acordando o laço, e não só o quadro.
            match self.next_wake() {
                Some(at) => event_loop.set_control_flow(ControlFlow::WaitUntil(at)),
                None => event_loop.set_control_flow(ControlFlow::Wait),
            }
        } else {
            let wake = self.next_wake();
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
    // No WSL a placa do Windows chega pelo `/dev/dxg`, e o Mesa só a usa com o driver d3d12
    // (sem ele, o zink falha e o desenho vai para a CPU, no llvmpipe). Como no Executar da IDE
    // do Windows no WSL (`aylis_tools::wsl`); a biblioteca vem de `/usr/lib/wsl/lib`.
    if is_wsl()
        && std::env::var_os("GALLIUM_DRIVER").is_none()
        && std::path::Path::new("/dev/dxg").exists()
        && std::path::Path::new("/usr/lib/wsl/lib/libd3d12.so").exists()
    {
        // SAFETY: na abertura do app, antes de o OpenGL e as threads de desenho existirem.
        unsafe { std::env::set_var("GALLIUM_DRIVER", "d3d12") };
    }
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
