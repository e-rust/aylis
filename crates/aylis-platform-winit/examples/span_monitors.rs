//! Um form em vários monitores (`span_monitors`): com o form maior que um monitor, uma janela
//! por monitor, cada uma na placa do seu (veja `docs/gpu-por-monitor.md`). O form é um só:
//! um cartão e um botão à esquerda, outro botão à direita (contam os cliques) e um ComboBox
//! embaixo.
//!
//! - `--full`: em tela cheia em todos os monitores (um telão).
//! - `--auto`: confere sozinho e fecha: imprime os pedaços, no Windows manda um clique à
//!   janela que está sob cada botão (pelas mensagens dela, sem mexer no mouse de verdade) e
//!   desliga e religa a propriedade (volta a uma janela e vira pedaços de novo).
//!
//! Numa máquina com um monitor só, `AYLIS_SPAN_SPLIT=2` conta o monitor como dois, lado a
//! lado (o form no meio da tela passa da divisa). Com `AYLIS_SNAPSHOT=pasta`, cada janela grava
//! o seu pedaço em `pasta/frmTelao.<n>.png`.
//!
//! ```text
//! set AYLIS_SPAN_SPLIT=2
//! cargo run -p aylis-platform-winit --example span_monitors --features d2d -- --auto
//! ```

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;
use aylis_platform_winit::{Options, Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let auto = args.iter().any(|a| a == "--auto");
    let full = args.iter().any(|a| a == "--full");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Form em vários monitores", 1000.0, 520.0));
    let _ = ui.set_name(form, "frmTelao");
    let _ = ui.set_prop(form, "span_monitors", true);
    let _ = ui.set_prop(form, "position", "center");
    if full {
        let _ = ui.set_prop(form, "window_state", "full_screen");
    }

    let card = ui.add(Rectangle::default(), form);
    ui.control_mut(card).unwrap().set_rect(16.0, 16.0, 360.0, 200.0);
    let _ = ui.set_prop(card, "fill", "primary");
    let _ = ui.set_prop(card, "fill2", "accent");
    let _ = ui.set_prop(card, "radius", 16.0);
    let _ = ui.set_prop(card, "shadow", "lg");
    let info = ui.add(Label::new(""), form);
    ui.control_mut(info).unwrap().set_rect(16.0, 240.0, 600.0, 120.0);
    let _ = ui.set_prop(info, "wrap", true);
    let left = ui.add(Button::new("Esquerda"), form);
    ui.control_mut(left).unwrap().set_rect(40.0, 400.0, 200.0, 48.0);
    let button = ui.add(Button::new("Direita"), form);
    ui.control_mut(button).unwrap().set_rect(760.0, 100.0, 200.0, 48.0);
    let combo = ui.add(ComboBox::new(&["Um", "Dois", "Três", "Quatro", "Cinco", "Seis"]), form);
    ui.control_mut(combo).unwrap().set_rect(700.0, 460.0, 260.0, 32.0);

    let clicks = Rc::new(Cell::new(0u32));
    for b in [left, button] {
        let counter = clicks.clone();
        ui.on_click(b, move |ui, _| {
            counter.set(counter.get() + 1);
            let name = ui.get_prop(b, "text").ok().and_then(|v| v.to_str().ok().map(str::to_string)).unwrap_or_default();
            println!("clique no botão {name} ({} no total)", counter.get());
        });
    }
    ui.on(form, "on_activate", |_, _, _| println!("on_activate"));
    ui.on(form, "on_deactivate", |_, _, _| println!("on_deactivate"));

    let timer = ui.add(Timer::new(800), form);
    let step = Cell::new(0u32);
    ui.on(timer, "on_timer", move |ui, _, _| {
        step.set(step.get() + 1);
        let pieces = ui.form_pieces(form).to_vec();
        let size = ui.control(form).map(|cb| (cb.width(), cb.height())).unwrap_or_default();
        let text = format!("Pedaços: {}\nForm: {:.0} × {:.0}\nCliques: {}", pieces.len().max(1), size.0, size.1, clicks.get());
        let _ = ui.set_prop(info, "text", text);
        if !auto {
            return;
        }
        match step.get() {
            2 => {
                println!("form {:.0} × {:.0}, {} pedaços:", size.0, size.1, pieces.len());
                for p in &pieces {
                    println!("  {p:?}");
                }
            }
            3 | 4 => {
                #[cfg(windows)]
                if let Some(r) = ui.absolute_rect(if step.get() == 3 { left } else { button }) {
                    click::at(form, ui, PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0));
                }
            }
            // Desligar a propriedade com o app aberto: volta a uma janela.
            5 => {
                println!("{} clique(s) nos botões; span_monitors desligado", clicks.get());
                let _ = ui.set_prop(form, "span_monitors", false);
            }
            6 => {
                println!("desligado: {} pedaços, form {:.0} × {:.0}; religando", pieces.len(), size.0, size.1);
                let _ = ui.set_prop(form, "span_monitors", true);
            }
            7 => {
                println!("fim: {} pedaços depois de religar, {} clique(s) nos botões", pieces.len(), clicks.get());
                ui.request_close();
            }
            _ => {}
        }
    });
    // No Windows o Direct2D (compile com a feature `d2d`); nos outros, o Skia na GPU. O
    // `AYLIS_RENDERER` (cpu, gpu, d2d) escolhe outro.
    let renderer = match std::env::var_os("AYLIS_RENDERER") {
        Some(_) => Renderer::Auto,
        None if cfg!(windows) => Renderer::D2d,
        None => Renderer::Gpu,
    };
    aylis_platform_winit::run_with(ui, form, Options { renderer, ..Default::default() })?;
    Ok(())
}

/// O clique de teste: acha a janela do app sob o ponto do form (a principal ou um pedaço) e
/// manda a ela o mouse por mensagens, nas coordenadas dela.
#[cfg(windows)]
mod click {
    use aylis_core::{ComponentId, PointF, Ui};
    use windows::Win32::Foundation::{HWND, LPARAM, POINT, RECT, WPARAM};
    use windows::Win32::Graphics::Gdi::{ClientToScreen, ScreenToClient};
    use windows::Win32::System::Threading::GetCurrentProcessId;
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible, PostMessageW, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
    };
    use windows::core::BOOL;

    pub fn at(form: ComponentId, ui: &Ui, p: PointF) {
        let Some(main) = aylis_platform_winit::win::hwnd(form) else { return };
        let main = HWND(main as *mut _);
        let scale = ui.monitors().first().map_or(1.0, |m| m.scale) * ui.form_zoom(form) as f64;
        // O canto da janela principal na tela, menos a origem do pedaço dela no form: o canto
        // do form na tela.
        let mut corner = POINT::default();
        // SAFETY: a janela é deste processo e está aberta (o form está aberto).
        let _ = unsafe { ClientToScreen(main, &mut corner) };
        let pieces = ui.form_pieces(form);
        let origin = window_origin_in_form(main, pieces, scale);
        let form_x = corner.x as f64 - origin.0;
        let form_y = corner.y as f64 - origin.1;
        let screen = POINT { x: (form_x + p.x as f64 * scale).round() as i32, y: (form_y + p.y as f64 * scale).round() as i32 };
        let Some(target) = window_at(screen) else {
            println!("nenhuma janela do app em {screen:?}");
            return;
        };
        let mut local = screen;
        // SAFETY: a janela é deste processo (achada entre as dele, visível).
        let _ = unsafe { ScreenToClient(target, &mut local) };
        println!("clique em {screen:?} na tela: janela {} em {local:?}{}", target.0 as isize, if target == main { " (a principal)" } else { " (um pedaço)" });
        let lparam = LPARAM(((local.y as u32 & 0xFFFF) << 16 | (local.x as u32 & 0xFFFF)) as isize);
        // SAFETY: mensagens de mouse para uma janela deste processo; o winit as trata como as
        // do sistema.
        unsafe {
            let _ = PostMessageW(Some(target), WM_MOUSEMOVE, WPARAM(0), lparam);
            let _ = PostMessageW(Some(target), WM_LBUTTONDOWN, WPARAM(1), lparam);
            let _ = PostMessageW(Some(target), WM_LBUTTONUP, WPARAM(0), lparam);
        }
    }

    /// Onde a janela principal começa no form, em pixels: a do pedaço cujo tamanho bate com o
    /// dela (sem pedaços, no canto).
    fn window_origin_in_form(main: HWND, pieces: &[aylis_core::RectF], scale: f64) -> (f64, f64) {
        let mut r = RECT::default();
        // SAFETY: a janela é deste processo.
        let _ = unsafe { GetWindowRect(main, &mut r) };
        let (w, h) = ((r.right - r.left) as f64, (r.bottom - r.top) as f64);
        pieces
            .iter()
            .find(|p| ((p.width as f64 * scale) - w).abs() < 2.0 && ((p.height as f64 * scale) - h).abs() < 2.0)
            .map_or((0.0, 0.0), |p| (p.x as f64 * scale, p.y as f64 * scale))
    }

    /// A janela visível deste processo que contém o ponto da tela.
    fn window_at(point: POINT) -> Option<HWND> {
        struct Search {
            point: POINT,
            found: Option<HWND>,
        }
        unsafe extern "system" fn each(hwnd: HWND, data: LPARAM) -> BOOL {
            // SAFETY: `data` é o `Search` passado abaixo, vivo durante o EnumWindows.
            let search = unsafe { &mut *(data.0 as *mut Search) };
            let mut pid = 0;
            // SAFETY: só leem a janela que o sistema passou.
            unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
            let mut r = RECT::default();
            // SAFETY: idem.
            let visible = unsafe { IsWindowVisible(hwnd).as_bool() && GetWindowRect(hwnd, &mut r).is_ok() };
            let p = search.point;
            // SAFETY: sem efeito colateral.
            if pid == unsafe { GetCurrentProcessId() } && visible && p.x >= r.left && p.x < r.right && p.y >= r.top && p.y < r.bottom {
                search.found = Some(hwnd);
                return BOOL(0);
            }
            BOOL(1)
        }
        let mut search = Search { point, found: None };
        // SAFETY: o ponteiro aponta para `search`, que vive até o fim da chamada.
        let _ = unsafe { EnumWindows(Some(each), LPARAM(&mut search as *mut Search as isize)) };
        search.found
    }
}
