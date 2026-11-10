//! Pinta o form de cadastro na GPU (OpenGL, fora da tela) e na CPU e compara os pixels.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example compare -- [pasta-de-saída]
//! ```
//!
//! Grava `cpu.png`, `gpu.png` e `diff.png` (diferenças ampliadas 8x) na pasta indicada (sem ela,
//! em `target/compare`).

#[path = "shared/registration_form.rs"]
mod registration_form;

use aylis_skia::SkiaRenderer;

const W: u32 = 900;
const H: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sem pasta: `target/compare` (gravava na pasta atual, e as imagens foram parar na raiz do
    // repositório).
    let out = std::path::PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| "target/compare".into()));
    std::fs::create_dir_all(&out)?;

    let render = |gpu: Option<&mut aylis_skia::gl::GlBackend>| -> Option<Vec<u8>> {
        let (mut ui, form) = registration_form::build(false).ok()?;
        let background = ui.theme().color("background");
        let renderer = SkiaRenderer::new();
        let paint = |canvas: &mut aylis_skia::SkiaCanvas<'_>| ui.paint(form, canvas);
        match gpu {
            Some(gl) => gl.render_rgba(&renderer, W, H, 1.0, background, paint),
            None => renderer.render_rgba(W, H, 1.0, background, paint),
        }
    };

    let cpu = render(None).ok_or("falha ao renderizar na CPU")?;
    let gpu = aylis_platform_winit::with_offscreen_gl(move |gl| render(Some(gl)))?.ok_or("falha ao renderizar na GPU")?;

    let mut diff = vec![0u8; cpu.len()];
    let (mut differing, mut over_16, mut max) = (0usize, 0usize, 0u8);
    for (i, (a, b)) in cpu.chunks_exact(4).zip(gpu.chunks_exact(4)).enumerate() {
        let d = (0..3).map(|c| a[c].abs_diff(b[c])).max().unwrap_or(0);
        if d > 0 {
            differing += 1;
        }
        if d > 16 {
            over_16 += 1;
        }
        max = max.max(d);
        let v = d.saturating_mul(8);
        diff[i * 4..i * 4 + 4].copy_from_slice(&[v, v, v, 255]);
    }

    let total = (W * H) as f64;
    println!("pixels: {}", W * H);
    println!("iguais: {:.2}%", 100.0 * (total - differing as f64) / total);
    println!("diferentes: {differing} ({:.2}%)", 100.0 * differing as f64 / total);
    println!("diferença > 16 (de 255): {over_16} ({:.3}%)", 100.0 * over_16 as f64 / total);
    println!("maior diferença num canal: {max}");

    for (name, pixels) in [("cpu.png", &cpu), ("gpu.png", &gpu), ("diff.png", &diff)] {
        let png = aylis_skia::encode_png(pixels, W, H).ok_or("falha ao gerar PNG")?;
        std::fs::write(out.join(name), png)?;
    }
    println!("imagens gravadas em {}", out.display());
    Ok(())
}
