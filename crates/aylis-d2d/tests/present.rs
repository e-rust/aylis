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

/// A pintura por parte copia para o buffer de trás só o que mudou (a união das áreas desde que
/// aquele buffer foi mostrado) e apresenta com os retângulos sujos: cada buffer continua com o
/// quadro inteiro que ele mostrou, nos três modos de apresentar (a swap chain da janela e a da
/// composição giram dois buffers; a do modelo de cópia, um).
#[cfg(windows)]
#[test]
fn the_partial_copy_keeps_every_buffer_whole() {
    use aylis_core::gpu::GpuChoice;
    use aylis_core::{Canvas, Color, RectF};
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, WINDOW_EX_STYLE, WS_POPUP, WS_VISIBLE};
    use windows::core::w;

    let (w, h) = (64u32, 48u32);
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    // As áreas de cada quadro (vazio: o inteiro) e a cor que pintam.
    let frames: [(&[RectF], Color); 6] = [
        (&[], Color::WHITE),
        (&[RectF::new(0.0, 0.0, 20.0, 20.0)], Color::rgb(255, 0, 0)),
        (&[RectF::new(30.0, 20.0, 20.0, 20.0)], Color::rgb(0, 0, 255)),
        (&[RectF::new(10.0, 30.0, 10.0, 10.0), RectF::new(50.0, 0.0, 14.0, 10.0)], Color::rgb(0, 200, 0)),
        (&[RectF::new(5.0, 5.0, 10.0, 10.0)], Color::rgb(0, 0, 0)),
        (&[RectF::new(40.0, 25.0, 4.0, 4.0)], Color::rgb(255, 255, 0)),
    ];
    // O quadro inteiro depois de cada um.
    let mut expected: Vec<Vec<u8>> = Vec::new();
    let mut now = vec![0u8; (w * h * 4) as usize];
    for (areas, color) in &frames {
        for y in 0..h {
            for x in 0..w {
                let hit = areas.is_empty() || areas.iter().any(|a| (x as f32) >= a.x && (x as f32) < a.right() && (y as f32) >= a.y && (y as f32) < a.bottom());
                if hit {
                    let i = ((y * w + x) * 4) as usize;
                    now[i..i + 4].copy_from_slice(&[color.r, color.g, color.b, 255]);
                }
            }
        }
        expected.push(now.clone());
    }
    for (present, lag) in [(PresentMode::Window, 1), (PresentMode::Composition, 1), (PresentMode::Composed, 0)] {
        // SAFETY: uma janela da classe pronta "STATIC", fora da tela (visível: a encoberta não
        // gira os buffers), destruída no fim.
        let hwnd = unsafe { CreateWindowExW(WINDOW_EX_STYLE(0), w!("STATIC"), w!(""), WS_POPUP | WS_VISIBLE, -2000, -2000, w as i32, h as i32, None, None, None, None) }.expect("janela");
        let mut target = renderer.hwnd_target_with(hwnd.0 as isize, w, h, false, GpuChoice::Auto, present).expect("alvo");
        let partial = aylis_d2d::partial_presents();
        for (k, (areas, color)) in frames.iter().enumerate() {
            let parts = (!areas.is_empty()).then_some((&[][..], *areas));
            target
                .draw_parts(&renderer, w, h, 1.0, Color::WHITE, parts, |c, _| c.fill_rect(RectF::new(0.0, 0.0, w as f32, h as f32), 0.0, &(*color).into()))
                .expect("quadro");
            // O buffer de trás de agora mostrou o quadro de `lag` apresentações atrás.
            if k > lag {
                let back = target.back_buffer_rgba().expect("o buffer de trás");
                assert!(back == expected[k - lag], "{present:?}, depois do quadro {k}: o buffer de trás não é o quadro {}", k - lag);
            }
        }
        let partial = aylis_d2d::partial_presents() - partial;
        eprintln!("{present:?}: {partial} apresentações por parte");
        assert!(partial >= 3, "{present:?}: só {partial} apresentações por parte (a janela encoberta não gira?)");
        drop(target);
        // SAFETY: a janela criada acima.
        unsafe {
            let _ = DestroyWindow(hwnd);
        }
    }
}
