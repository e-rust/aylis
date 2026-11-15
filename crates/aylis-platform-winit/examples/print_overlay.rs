//! O form do print do NoOBS: sem borda, sempre na frente, fora da barra de tarefas, cobrindo a
//! área de trabalho virtual inteira (todos os monitores, `window_state = full_screen` com
//! `span_monitors`), com 1 unidade = 1 pixel físico (`physical_pixels`) e a imagem de fundo do
//! tamanho da tela. Criado de um JSON, mostrado, fechado e liberado a cada vez (como a cada
//! Print Screen), `--cycles` vezes (padrão 20).
//!
//! Mede, a cada abertura, o tempo do `show()` até a pintura de todos os pedaços (cada pedaço da
//! tela é coberto por células com um `on_paint` que anota a primeira vez), e confere que fechar
//! e liberar não deixa janelas nem memória para trás (as janelas do processo e a memória
//! privada depois de cada ciclo). Com `--auto`, na primeira abertura manda um clique (pelas
//! mensagens da janela sob o ponto, sem mexer no mouse) num pixel da tela e confere que ele
//! chega ao form como o mesmo pixel do retângulo virtual. A última linha, `AYLIS_PRINT_OVERLAY`,
//! é o JSON que vai para `medicoes/print_overlay.json`.
//!
//! ```text
//! set AYLIS_SPAN_SPLIT=2
//! cargo run --release -p aylis-platform-winit --example print_overlay --features d2d -- --auto
//! ```
//!
//! `--logical`: sem o `physical_pixels` (o form na escala do monitor principal); `--no-image`: sem a
//! imagem de fundo (para separar o custo dela); `--rgba`: a imagem em RGBA (convertida para a placa
//! uma vez) em vez de BGRA.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use aylis_controls::*;
use aylis_core::span::ScreenRect;
use aylis_core::*;
use aylis_platform_winit::{Options, Renderer};

/// As células que cobrem o form (colunas × linhas): cada uma anota a primeira pintura.
const COLUMNS: usize = 16;
const ROWS: usize = 9;

/// O que um ciclo mediu.
#[derive(Clone, Debug, Default)]
struct Cycle {
    /// Do `show()` até a pintura de todas as células, em ms.
    open_ms: f64,
    /// Células pintadas (as que caem num vão entre monitores não pintam).
    painted: usize,
    /// Janelas do processo e memória privada (MB) depois de fechar e liberar.
    windows: usize,
    private_mb: f64,
}

/// O estado do vaivém: o print aberto (quando foi mostrado e as primeiras pinturas) ou não.
#[derive(Default)]
struct State {
    overlay: Option<ComponentId>,
    shown: Option<Instant>,
    painted: Vec<Option<Instant>>,
    cycles: Vec<Cycle>,
    clicked: Option<(PointF, PointF)>,
    click_sent: Option<(f32, f32)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let auto = args.iter().any(|a| a == "--auto");
    let physical = !args.iter().any(|a| a == "--logical");
    let image = !args.iter().any(|a| a == "--no-image");
    let rgba = args.iter().any(|a| a == "--rgba");
    let cycles = args.iter().position(|a| a == "--cycles").and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok()).unwrap_or(20usize);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.add_root(Form::new("Aylis — o print abre e fecha sozinho", 460.0, 140.0));
    let status = ui.create("Label", Some(main))?;
    ui.set_prop(status, "align", "client")?;
    ui.set_prop(status, "wrap", true)?;
    let state = Rc::new(RefCell::new(State::default()));
    let windows_before = Rc::new(Cell::new(0usize));
    let memory_before = Rc::new(Cell::new(0f64));

    let timer = ui.add(Timer::new(40), main);
    let st = state.clone();
    let (wb, mb) = (windows_before.clone(), memory_before.clone());
    let started = Cell::new(false);
    let idle_until = Cell::new(Instant::now());
    ui.on(timer, "on_timer", move |ui, _, _| {
        if !started.replace(true) {
            wb.set(process_windows());
            mb.set(private_mb());
            idle_until.set(Instant::now() + std::time::Duration::from_millis(500));
            return;
        }
        let mut s = st.borrow_mut();
        match s.overlay {
            None if Instant::now() >= idle_until.get() => {
                if s.cycles.len() >= cycles {
                    drop(s);
                    report(&st.borrow(), wb.get(), mb.get(), physical);
                    ui.request_close();
                    return;
                }
                drop(s);
                match open_overlay(ui, &st, physical, image, rgba) {
                    Ok(form) => {
                        let mut s = st.borrow_mut();
                        s.overlay = Some(form);
                        s.shown = Some(Instant::now());
                        drop(s);
                        let _ = ui.show(form);
                    }
                    Err(e) => {
                        println!("não abriu o print: {e}");
                        ui.request_close();
                    }
                }
            }
            None => {}
            Some(form) => {
                let shown = s.shown.unwrap_or_else(Instant::now);
                let done = s.painted.iter().all(Option::is_some);
                let waited = shown.elapsed().as_secs_f64();
                // Todas as células pintadas (ou 1,5 s: as dos vãos entre monitores não pintam).
                if !(done || waited > 1.5) {
                    return;
                }
                // O clique, na primeira abertura: num pixel do pedaço da direita.
                if auto && s.cycles.is_empty() && s.click_sent.is_none() {
                    let size = ui.control(form).map(|cb| (cb.width(), cb.height())).unwrap_or_default();
                    let virt = virtual_rect(ui);
                    let at = ((size.0 * 0.75).round(), (size.1 * 0.4).round());
                    let scale = if physical { 1.0 } else { ui.monitors().first().map_or(1.0, |m| m.scale) as f32 };
                    s.click_sent = Some(at);
                    #[cfg(windows)]
                    click::at_screen(virt.x + (at.0 * scale) as i32, virt.y + (at.1 * scale) as i32);
                    let _ = virt;
                    return;
                }
                if s.click_sent.is_some() && s.clicked.is_none() && waited < 3.0 {
                    return; // o clique ainda não chegou
                }
                let last = s.painted.iter().flatten().max().copied();
                let painted = s.painted.iter().filter(|p| p.is_some()).count();
                let open_ms = last.map_or(f64::NAN, |t| t.duration_since(shown).as_secs_f64() * 1000.0);
                s.cycles.push(Cycle { open_ms, painted, ..Default::default() });
                s.overlay = None;
                drop(s);
                ui.close(form);
                idle_until.set(Instant::now() + std::time::Duration::from_millis(250));
                // Depois de fechar (e liberar): as janelas e a memória, na volta seguinte.
                let st2 = st.clone();
                ui.set_timeout(200, move |ui| {
                    let mut s = st2.borrow_mut();
                    let n = s.cycles.len();
                    if let Some(c) = s.cycles.last_mut() {
                        c.windows = process_windows();
                        c.private_mb = private_mb();
                        let text = format!("Ciclo {n}: {:.1} ms até pintar {} células; {} janela(s), {:.1} MB; o form existe: {}", c.open_ms, c.painted, c.windows, c.private_mb, ui.contains(form));
                        println!("{text}");
                    }
                });
            }
        }
    });
    let renderer = match std::env::var_os("AYLIS_RENDERER") {
        Some(_) => Renderer::Auto,
        None if cfg!(windows) => Renderer::D2d,
        None => Renderer::Gpu,
    };
    aylis_platform_winit::run_with(ui, main, Options { renderer, ..Default::default() })?;
    Ok(())
}

/// O retângulo de todos os monitores (pixels físicos).
fn virtual_rect(ui: &Ui) -> ScreenRect {
    let monitors: Vec<ScreenRect> = ui.monitors().iter().map(|m| ScreenRect::new(m.x, m.y, m.width, m.height)).collect();
    ScreenRect::bounding(&monitors).unwrap_or(ScreenRect::new(0, 0, 1920, 1080))
}

/// O JSON do form do print (o que o NoOBS grava) e, por código, a imagem de fundo e as
/// células de medir.
fn overlay_json(physical: bool) -> String {
    let physical = if physical { "\n    \"physical_pixels\": true," } else { "" };
    format!(
        r#"{{
  "aylis": 1,
  "root": {{
    "class": "Form",
    "name": "frmPrint",
    "caption": "Print",
    "border_style": "none",
    "window_state": "full_screen",
    "span_monitors": true,{physical}
    "topmost": true,
    "show_in_taskbar": false,
    "force_foreground": true,
    "key_preview": true,
    "close_action": "free",
    "children": [
      {{ "class": "Image", "name": "imgFundo", "align": "client", "fit": "stretch" }}
    ]
  }}
}}"#
    )
}

fn open_overlay(ui: &mut Ui, state: &Rc<RefCell<State>>, physical: bool, with_image: bool, rgba: bool) -> Result<ComponentId, Box<dyn std::error::Error>> {
    let form = aylis_json::load(ui, &overlay_json(physical), None)?;
    // O tamanho do form: o retângulo virtual em pixels (ou em unidades da escala principal).
    let virt = virtual_rect(ui);
    let scale = if physical { 1.0 } else { ui.monitors().first().map_or(1.0, |m| m.scale) };
    let (w, h) = (virt.width as f32 / scale as f32, virt.height as f32 / scale as f32);
    ui.control_mut(form).ok_or("form")?.set_size(w, h);
    // O "print": um degradê do tamanho da tela, com um quadriculado de 1 px no canto (onde a
    // imagem e a tela batem pixel a pixel, ou não). Em BGRA, como a captura da tela vem (o
    // `Image::bgra` vai à placa como está, sem converter pixel a pixel).
    let (iw, ih) = (virt.width, virt.height);
    let mut pixels = vec![0u8; (iw * ih * 4) as usize];
    for y in 0..ih {
        for x in 0..iw {
            let i = ((y * iw + x) * 4) as usize;
            let checker = x < 64 && y < 64 && (x + y) % 2 == 0;
            let v = if checker { 255 } else { 0 };
            let (blue, red) = if rgba { (2, 0) } else { (0, 2) };
            pixels[i + blue] = if checker { v } else { 96 };
            pixels[i + 1] = if checker { v } else { (y * 255 / ih) as u8 };
            pixels[i + red] = if checker { v } else { (x * 255 / iw) as u8 };
            pixels[i + 3] = 255;
        }
    }
    if let Some(image) = ui.find_in(form, "imgFundo").filter(|_| with_image) {
        if let Some(view) = ui.get_mut::<ImageView>(image) {
            view.set_picture(Some(if rgba { Image::rgba(iw, ih, pixels) } else { Image::bgra(iw, ih, pixels) }));
        }
    }
    let mut s = state.borrow_mut();
    s.painted = vec![None; COLUMNS * ROWS];
    s.click_sent = None;
    drop(s);
    let (cw, ch) = (w / COLUMNS as f32, h / ROWS as f32);
    for i in 0..COLUMNS * ROWS {
        let cell = ui.create("Panel", Some(form))?;
        ui.control_mut(cell).ok_or("célula")?.set_rect((i % COLUMNS) as f32 * cw, (i / COLUMNS) as f32 * ch, cw, ch);
        ui.set_prop(cell, "opacity", 0.2)?;
        let st = state.clone();
        ui.on_paint(cell, move |canvas, ctx, base| {
            base(canvas, ctx);
            if let Ok(mut s) = st.try_borrow_mut() {
                if let Some(slot) = s.painted.get_mut(i).filter(|p| p.is_none()) {
                    *slot = Some(Instant::now());
                }
            }
        });
        let st = state.clone();
        ui.on(cell, "on_pointer_down", move |ui, sender, args| {
            if let EventArgs::Pointer(e) = args {
                let at = ui.absolute_rect(sender).map_or(PointF::ZERO, |r| PointF::new(r.x + e.position.x, r.y + e.position.y));
                let mut s = st.borrow_mut();
                let sent = s.click_sent.map_or(PointF::ZERO, |(x, y)| PointF::new(x, y));
                println!("clique no form em ({:.1}, {:.1}); mandado no pixel ({:.0}, {:.0}) do retângulo virtual", at.x, at.y, sent.x, sent.y);
                s.clicked = Some((sent, at));
            }
        });
    }
    Ok(form)
}

fn report(s: &State, windows_before: usize, memory_before: f64, physical: bool) {
    let mut times: Vec<f64> = s.cycles.iter().map(|c| c.open_ms).filter(|t| t.is_finite()).collect();
    times.sort_by(f64::total_cmp);
    let median = times.get(times.len() / 2).copied().unwrap_or(f64::NAN);
    let first = s.cycles.first().map_or(f64::NAN, |c| c.open_ms);
    let max = times.last().copied().unwrap_or(f64::NAN);
    let windows_after = s.cycles.last().map_or(0, |c| c.windows);
    let mem: Vec<f64> = s.cycles.iter().map(|c| c.private_mb).collect();
    // A memória dos últimos ciclos contra a dos primeiros (depois do primeiro, que carrega o que
    // fica: as fontes, a placa).
    let early = mem.get(1).copied().unwrap_or(f64::NAN);
    let late = mem.last().copied().unwrap_or(f64::NAN);
    let click_ok = s.clicked.map(|(sent, got)| (sent.x - got.x).abs() <= 0.5 && (sent.y - got.y).abs() <= 0.5);
    println!(
        "{} ciclos: abrir {first:.1} ms (o primeiro), mediana {median:.1} ms, máximo {max:.1} ms; janelas antes {windows_before}, depois {windows_after}; memória privada {memory_before:.1} MB antes, {early:.1} MB no 2º ciclo, {late:.1} MB no último; clique no pixel certo: {}",
        s.cycles.len(),
        match click_ok {
            Some(true) => "sim",
            Some(false) => "NÃO",
            None => "não conferido",
        }
    );
    let split = std::env::var("AYLIS_SPAN_SPLIT").unwrap_or_default();
    println!(
        "AYLIS_PRINT_OVERLAY {{\"physical_pixels\": {physical}, \"span_split\": \"{split}\", \"cycles\": {}, \"open_first_ms\": {first:.1}, \"open_median_ms\": {median:.1}, \"open_max_ms\": {max:.1}, \"windows_before\": {windows_before}, \"windows_after\": {windows_after}, \"private_mb_before\": {memory_before:.1}, \"private_mb_cycle2\": {early:.1}, \"private_mb_last\": {late:.1}, \"click_ok\": {}}}",
        s.cycles.len(),
        click_ok.map_or("null".to_string(), |b| b.to_string())
    );
}

fn private_mb() -> f64 {
    aylis_platform_winit::process_memory().0 as f64 / (1024.0 * 1024.0)
}

/// As janelas de primeiro nível deste processo (visíveis ou não).
#[cfg(windows)]
fn process_windows() -> usize {
    click::count()
}

#[cfg(not(windows))]
fn process_windows() -> usize {
    0
}

/// O clique de teste e a contagem de janelas, pelas mensagens do Windows.
#[cfg(windows)]
mod click {
    use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
    use windows::Win32::Graphics::Gdi::ScreenToClient;
    use windows::Win32::System::Threading::GetCurrentProcessId;
    use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible, PostMessageW, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE};
    use windows::core::BOOL;

    struct Search {
        point: Option<POINT>,
        found: Option<HWND>,
        count: usize,
    }

    unsafe extern "system" fn each(hwnd: HWND, data: LPARAM) -> BOOL {
        // SAFETY: `data` é o `Search` passado em `search`, vivo durante o EnumWindows.
        let search = unsafe { &mut *(data.0 as *mut Search) };
        let mut pid = 0;
        // SAFETY: só lê a janela que o sistema passou.
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
        // SAFETY: sem efeito colateral.
        if pid != unsafe { GetCurrentProcessId() } {
            return BOOL(1);
        }
        search.count += 1;
        if let Some(p) = search.point {
            let mut r = RECT::default();
            // SAFETY: só lê a janela que o sistema passou.
            let visible = unsafe { IsWindowVisible(hwnd).as_bool() && GetWindowRect(hwnd, &mut r).is_ok() };
            if search.found.is_none() && visible && p.x >= r.left && p.x < r.right && p.y >= r.top && p.y < r.bottom {
                search.found = Some(hwnd);
            }
        }
        BOOL(1)
    }

    fn search(point: Option<POINT>) -> Search {
        let mut s = Search { point, found: None, count: 0 };
        // SAFETY: o ponteiro aponta para `s`, que vive até o fim da chamada.
        let _ = unsafe { EnumWindows(Some(each), LPARAM(&mut s as *mut Search as isize)) };
        s
    }

    pub fn count() -> usize {
        search(None).count
    }

    /// Manda um clique à janela do app sob o ponto da tela (pixels físicos), nas coordenadas
    /// dela. A primeira na ordem do sistema (a de cima): o print, sempre na frente.
    pub fn at_screen(x: i32, y: i32) {
        let screen = POINT { x, y };
        let Some(target) = search(Some(screen)).found else {
            println!("nenhuma janela do app em {screen:?}");
            return;
        };
        let mut local = screen;
        // SAFETY: a janela é deste processo (achada entre as dele, visível).
        let _ = unsafe { ScreenToClient(target, &mut local) };
        println!("clique no pixel {screen:?} da tela: janela {} em {local:?}", target.0 as isize);
        let lparam = LPARAM(((local.y as u32 & 0xFFFF) << 16 | (local.x as u32 & 0xFFFF)) as isize);
        // SAFETY: mensagens de mouse para uma janela deste processo; o winit as trata como as
        // do sistema.
        unsafe {
            let _ = PostMessageW(Some(target), WM_MOUSEMOVE, WPARAM(0), lparam);
            let _ = PostMessageW(Some(target), WM_LBUTTONDOWN, WPARAM(1), lparam);
            let _ = PostMessageW(Some(target), WM_LBUTTONUP, WPARAM(0), lparam);
        }
    }
}
