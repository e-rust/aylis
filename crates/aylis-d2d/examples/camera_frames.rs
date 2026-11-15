//! Mede o custo de mostrar quadros de câmera ou de vídeo (pixels BGRA novos a cada quadro) no
//! Direct2D, numa janela escondida de verdade (a swap chain da placa, como no app).
//!
//! `cargo run --release -p aylis-d2d --example camera_frames [quadros] [largura] [altura] [câmeras] [limite_kb] [limite_do_form_kb]`
//!
//! `limite_kb` troca o limite do reaproveitamento em execução, como o app faria
//! (`aylis_core::image_cache::set_frame_reuse_kb`, o `RenderSettings`; `0`: sempre cria);
//! `limite_do_form_kb`, o do form desta janela (`HwndTarget::set_frame_reuse_kb`, o
//! `frame_reuse_kb` do Form), que ganha do outro. `aylis-tool bench-frames`
//! roda os tamanhos com e sem e sugere o limite.
//!
//! Cada quadro é uma imagem nova (o app solta a anterior, como o `VideoPlayer` e o
//! `CameraView`). Escreve o tempo médio por quadro e quantas imagens o renderizador guardou.

#[cfg(windows)]
fn main() {
    use aylis_core::{Canvas, Color, Image, RectF};
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, WINDOW_EX_STYLE, WS_POPUP};
    use windows::core::w;

    let args: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let frames = args.first().copied().unwrap_or(300);
    let (width, height) = (args.get(1).copied().unwrap_or(1920), args.get(2).copied().unwrap_or(1080));
    let cameras = args.get(3).copied().unwrap_or(1).max(1);
    if let Some(&kb) = args.get(4) {
        aylis_core::image_cache::set_frame_reuse_kb(kb);
    }
    // SAFETY: uma janela escondida da classe pronta "STATIC", destruída no fim.
    let hwnd = unsafe { CreateWindowExW(WINDOW_EX_STYLE(0), w!("STATIC"), w!(""), WS_POPUP, 0, 0, width as i32, height as i32, None, None, None, None) }.expect("janela");
    let renderer = aylis_d2d::D2dRenderer::new().expect("Direct2D");
    let mut target = renderer.hwnd_target(hwnd.0 as isize, width, height, false).expect("alvo da janela");
    target.set_frame_reuse_kb(args.get(5).copied());
    let cols = (cameras as f32).sqrt().ceil() as u32;
    let rows = cameras.div_ceil(cols);
    let (cw, ch) = (width / cols, height / rows);
    // Um conteúdo diferente por quadro, sem pagar a geração dentro da medida.
    let base: Vec<u8> = (0..cw * ch * 4).map(|i| (i % 251) as u8).collect();
    let mut total = std::time::Duration::ZERO;
    for f in 0..frames {
        let images: Vec<Image> = (0..cameras)
            .map(|c| {
                let mut pixels = base.clone();
                pixels[0] = (f + c) as u8;
                Image::bgra(cw, ch, pixels)
            })
            .collect();
        let start = std::time::Instant::now();
        target
            .draw(&renderer, width, height, 1.0, Color::BLACK, None, |canvas, _| {
                for (i, image) in images.iter().enumerate() {
                    let (x, y) = ((i as u32 % cols) * cw, (i as u32 / cols) * ch);
                    let src = RectF::new(0.0, 0.0, cw as f32, ch as f32);
                    canvas.draw_image(image, src, RectF::new(x as f32, y as f32, cw as f32, ch as f32), 1.0);
                }
            })
            .expect("quadro");
        // O primeiro quadro paga a criação da janela e dos bitmaps: fica fora da média.
        if f > 0 {
            total += start.elapsed();
        }
    }
    let per = total.as_secs_f64() * 1000.0 / (frames.max(2) - 1) as f64;
    println!(
        "{frames} quadros {width}x{height}, {cameras} câmera(s) de {cw}x{ch}: {per:.3} ms por quadro, {} bitmaps criados na placa",
        aylis_d2d::pixel_bitmaps_created()
    );
    // A linha que o `aylis-tool bench-frames` lê.
    println!("AYLIS_FRAMES ms={per:.4} bitmaps={}", aylis_d2d::pixel_bitmaps_created());
    // SAFETY: a janela criada acima.
    unsafe {
        let _ = DestroyWindow(hwnd);
    }
}

#[cfg(not(windows))]
fn main() {
    println!("só no Windows (Direct2D)");
}
