//! Um form de cadastro numa janela de verdade, pintado com Skia na GPU (OpenGL) ou na CPU.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example registration
//! cargo run -p aylis-platform-winit --example registration -- --dark
//! cargo run -p aylis-platform-winit --example registration -- --cpu
//! cargo run -p aylis-platform-winit --example registration -- --png tela.png 900 600
//! ```
//!
//! Redimensione a janela: toolbar, menu, status e conteúdo se reorganizam (Align) e o
//! botão "OK" acompanha o canto inferior direito (Anchors).

#[path = "shared/registration_form.rs"]
mod registration_form;

use aylis_platform_winit::{Options, Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| args.iter().any(|a| a == name);
    let (mut ui, form) = registration_form::build(flag("--dark"))?;

    // `--png arquivo.png [largura altura]`: renderiza num arquivo (CPU) em vez de abrir a janela.
    if let Some(i) = args.iter().position(|a| a == "--png") {
        let path = args.get(i + 1).ok_or("informe o arquivo depois de --png")?;
        let w: f32 = args.get(i + 2).and_then(|v| v.parse().ok()).unwrap_or(900.0);
        let h: f32 = args.get(i + 3).and_then(|v| v.parse().ok()).unwrap_or(600.0);
        ui.layout(); // primeiro layout no tamanho de design (referência das âncoras)
        ui.control_mut(form).unwrap().set_size(w, h);
        ui.layout();
        // `--hover X Y [--press]`: simula o mouse antes de capturar (animações concluídas).
        if let Some(j) = args.iter().position(|a| a == "--hover") {
            let x: f32 = args.get(j + 1).and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let y: f32 = args.get(j + 2).and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let event = aylis_core::PointerEvent { position: aylis_core::PointF::new(x, y), ..Default::default() };
            ui.pointer_move(form, event);
            if flag("--press") {
                ui.pointer_down(form, event);
            }
            // Pinta uma vez para iniciar as transições do tema e avança o tempo.
            let renderer = aylis_skia::SkiaRenderer::new();
            renderer.render_rgba(w as u32, h as u32, 1.0, aylis_core::Color::BLACK, |c| ui.paint(form, c));
            ui.set_time(1.0);
            ui.tick();
        }
        let background = ui.theme().color("background");
        let png = aylis_skia::SkiaRenderer::new()
            .render_png(w as u32, h as u32, 1.0, background, |canvas| ui.paint(form, canvas))
            .ok_or("falha ao gerar o PNG")?;
        std::fs::write(path, png)?;
        return Ok(());
    }

    let renderer = if flag("--cpu") {
        Renderer::Cpu
    } else if flag("--gpu") {
        Renderer::Gpu
    } else {
        Renderer::Auto
    };
    aylis_platform_winit::run_with(ui, form, Options { renderer, ..Default::default() })
}
