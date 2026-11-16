//! Como o quadro de uma janela em tela cheia sem borda chega ao monitor em cada modo de
//! apresentar (`window`, `composition`, `composed`), pelo DXGI (`HwndTarget::presentation`):
//! `composed` (o DWM compõe), `overlay` (um plano de overlay, o MPO) ou `none` (direto ao
//! monitor, o independent flip). Feito para o relato do NoOBS de 15/11: numa placa com MPO, o
//! cursor congelava sobre um form assim, e o modo `composed` deve ficar sempre no DWM.
//!
//! `cargo run --release -p aylis-d2d --example present_modes [quadros]`
//!
//! Abre uma janela por cima da tela inteira por alguns segundos em cada modo.

#[cfg(windows)]
fn main() {
    use aylis_core::gpu::{GpuChoice, PresentMode};
    use aylis_core::{Canvas, Color, RectF};
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DestroyWindow, DispatchMessageW, GetSystemMetrics, MSG, PM_REMOVE, PeekMessageW, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, ShowWindow, TranslateMessage, WS_EX_TOPMOST, WS_POPUP,
    };
    use windows::core::w;

    let frames: u32 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(180);
    // SAFETY: chamadas sem ponteiros.
    let (width, height) = unsafe { (GetSystemMetrics(SM_CXSCREEN) as u32, GetSystemMetrics(SM_CYSCREEN) as u32) };
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    for (name, mode) in [("window", PresentMode::Window), ("composition", PresentMode::Composition), ("composed", PresentMode::Composed)] {
        // SAFETY: uma janela da classe pronta "STATIC", do tamanho da tela, destruída no fim.
        let hwnd = unsafe { CreateWindowExW(WS_EX_TOPMOST, w!("STATIC"), w!("present_modes"), WS_POPUP, 0, 0, width as i32, height as i32, None, None, None, None) }.expect("janela");
        // SAFETY: a janela criada acima.
        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
        }
        let mut target = renderer.hwnd_target_with(hwnd.0 as isize, width, height, true, GpuChoice::Auto, mode).expect("alvo da janela");
        let mut seen = std::collections::BTreeMap::<&str, u32>::new();
        for f in 0..frames {
            // As mensagens da janela, para o sistema não a dar como travada.
            let mut msg = MSG::default();
            // SAFETY: a fila desta thread; `msg` vive durante as chamadas.
            unsafe {
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
            target
                .draw(&renderer, width, height, 1.0, Color::rgb(20, 24, 32), None, |canvas, _| {
                    let x = (f * 8 % width.max(1)) as f32;
                    canvas.fill_rect(RectF::new(x, height as f32 / 2.0 - 60.0, 120.0, 120.0), 12.0, &Color::rgb(60, 140, 240).into());
                })
                .expect("quadro");
            // Os primeiros quadros não contam: o sistema decide o caminho depois de alguns.
            if f >= 30 {
                *seen.entry(target.presentation().unwrap_or("?")).or_default() += 1;
            }
        }
        let summary: Vec<String> = seen.iter().map(|(k, v)| format!("{k} {v}")).collect();
        println!("{name:12} {}", summary.join(", "));
        println!("AYLIS_PRESENT mode={name} {}", seen.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" "));
        drop(target);
        // SAFETY: a janela criada acima.
        unsafe {
            let _ = DestroyWindow(hwnd);
        }
    }
}

#[cfg(not(windows))]
fn main() {
    println!("só no Windows (Direct2D)");
}
