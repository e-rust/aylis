//! O modo de apresentar `composed` (relato do NoOBS, 15/11: o cursor congelava sobre um form em
//! tela cheia numa placa com MPO): a janela desenha na swap chain do modelo de cópia, que o DWM
//! sempre compõe, e a propriedade `present` do form troca o modo com a janela aberta. Medido em
//! 16/11 nesta placa AMD com o exemplo `present_modes`: no modo `window`, todo quadro ia a um
//! plano de overlay.

use aylis_core::gpu::{PresentMode, present_for, set_present};

#[test]
fn the_mode_is_read_from_the_form_or_the_project() {
    assert_eq!(PresentMode::parse("composed"), PresentMode::Composed);
    assert_eq!(PresentMode::parse("Composto"), PresentMode::Composed);
    assert_eq!(PresentMode::parse(PresentMode::Composed.name()), PresentMode::Composed);
    assert_eq!(PresentMode::parse("qualquer"), PresentMode::Window);
    set_present(PresentMode::Composition);
    assert_eq!(present_for(""), PresentMode::Composition, "vazio: o do projeto");
    assert_eq!(present_for("composed"), PresentMode::Composed);
    set_present(PresentMode::Window);
}

#[cfg(windows)]
#[test]
fn a_composed_window_draws_and_the_mode_changes_while_open() {
    use aylis_core::gpu::GpuChoice;
    use aylis_core::{Canvas, Color, RectF};
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, WINDOW_EX_STYLE, WS_POPUP};
    use windows::core::w;

    // SAFETY: uma janela escondida da classe pronta "STATIC", destruída no fim.
    let hwnd = unsafe { CreateWindowExW(WINDOW_EX_STYLE(0), w!("STATIC"), w!(""), WS_POPUP, 0, 0, 320, 200, None, None, None, None) }.expect("janela");
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    let mut target = renderer.hwnd_target_with(hwnd.0 as isize, 320, 200, false, GpuChoice::Auto, PresentMode::Composed).expect("alvo no modelo de cópia");
    let mut frame = |target: &mut aylis_d2d::HwndTarget| {
        target
            .draw(&renderer, 320, 200, 1.0, Color::WHITE, None, |canvas, _| canvas.fill_rect(RectF::new(10.0, 10.0, 50.0, 50.0), 0.0, &Color::BLACK.into()))
            .expect("quadro")
    };
    for _ in 0..3 {
        frame(&mut target);
    }
    assert!(!target.set_present(&renderer, PresentMode::Composed).unwrap(), "o mesmo modo não recria");
    assert!(target.set_present(&renderer, PresentMode::Window).unwrap(), "outro modo recria o alvo");
    frame(&mut target);
    assert!(target.set_present(&renderer, PresentMode::Composed).unwrap());
    frame(&mut target);
    drop(target);
    // SAFETY: a janela criada acima.
    unsafe {
        let _ = DestroyWindow(hwnd);
    }
}
