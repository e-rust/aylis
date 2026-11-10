//! PDF pelo `hayro` (Rust puro): os tamanhos das páginas e cada página em RGBA, na escala
//! pedida. Fora do navegador, uma thread por documento rasteriza as páginas (é a parte
//! demorada); no navegador, sem threads, rasteriza na hora.

use std::sync::Arc;

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::{PixmapSettings, RenderCache, RenderSettings};

/// Tamanho de cada página, em pontos (1/72 de polegada).
pub fn page_sizes(bytes: &[u8]) -> Result<Vec<(f32, f32)>, String> {
    let pdf = Pdf::new(bytes.to_vec()).map_err(|e| format!("PDF: {e:?}"))?;
    Ok(pdf.pages().iter().map(|p| p.render_dimensions()).collect())
}

/// O padrão da maior página rasterizada (64 megapixels).
pub const MAX_PIXELS: f32 = 64_000_000.0;

/// Rasteriza a página `index` com `scale` pixels por ponto: (largura, altura, RGBA).
pub fn render_page(pdf: &Pdf, index: usize, scale: f32) -> Option<(u32, u32, Vec<u8>)> {
    render_page_with(pdf, index, scale, MAX_PIXELS)
}

/// Como [`render_page`], com no máximo `max_pixels`: o tamanho da página vem do arquivo
/// (até 14.400 × 14.400 pontos), e uma página enorme com zoom viraria bilhões de pixels.
/// Acima do limite a escala diminui (a página aparece menos nítida).
pub fn render_page_with(pdf: &Pdf, index: usize, scale: f32, max_pixels: f32) -> Option<(u32, u32, Vec<u8>)> {
    let page = pdf.pages().get(index)?;
    let (w, h) = page.render_dimensions();
    let side = aylis_core::image::MAX_IMAGE_SIDE as f32;
    let fit_area = (max_pixels / (w * h * scale * scale).max(1.0)).sqrt();
    let fit_side = side / (w.max(h) * scale).max(1.0);
    let scale = scale * fit_area.min(fit_side).min(1.0);
    let cache = RenderCache::new();
    let settings = PixmapSettings { x_scale: scale, y_scale: scale, bg_color: WHITE };
    let pixmap = hayro::render(page, &cache, &InterpreterSettings::default(), &RenderSettings::default(), &settings);
    let (w, h) = (pixmap.width() as u32, pixmap.height() as u32);
    // Fundo branco opaco: o RGBA pré-multiplicado é o mesmo que o comum.
    Some((w, h, pixmap.data_as_u8_slice().to_vec()))
}

/// Uma página pronta: (página, escala em centésimos, largura, altura, RGBA).
pub type Rendered = (usize, u32, u32, u32, Vec<u8>);

/// Quem rasteriza as páginas de um documento.
pub struct Renderer {
    #[cfg(not(target_arch = "wasm32"))]
    jobs: std::sync::mpsc::Sender<(usize, u32)>,
    #[cfg(target_arch = "wasm32")]
    pdf: Pdf,
    #[cfg(target_arch = "wasm32")]
    max_pixels: f32,
}

impl Renderer {
    /// `done` recebe cada página pronta (de outra thread, fora do navegador). Cada página tem
    /// no máximo `max_pixels`.
    pub fn new(bytes: Arc<Vec<u8>>, max_pixels: f32, done: impl Fn(Rendered) + Send + 'static) -> Result<Self, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (tx, rx) = std::sync::mpsc::channel::<(usize, u32)>();
            std::thread::Builder::new()
                .name("aylis-pdf".into())
                .spawn(move || {
                    let Ok(pdf) = Pdf::new(bytes.as_ref().clone()) else { return };
                    // O canal fecha quando o visualizador some: a thread sai.
                    while let Ok((page, scale)) = rx.recv() {
                        if let Some((w, h, rgba)) = render_page_with(&pdf, page, scale as f32 / 100.0, max_pixels) {
                            done((page, scale, w, h, rgba));
                        }
                    }
                })
                .map_err(|e| e.to_string())?;
            Ok(Self { jobs: tx })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = done;
            Ok(Self { pdf: Pdf::new(bytes.as_ref().clone()).map_err(|e| format!("PDF: {e:?}"))?, max_pixels })
        }
    }

    /// Pede a página `page` na escala `scale` (centésimos de pixel por ponto).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn request(&self, page: usize, scale: u32) -> Option<Rendered> {
        let _ = self.jobs.send((page, scale));
        None
    }

    /// No navegador a página sai na hora.
    #[cfg(target_arch = "wasm32")]
    pub fn request(&self, page: usize, scale: u32) -> Option<Rendered> {
        render_page_with(&self.pdf, page, scale as f32 / 100.0, self.max_pixels).map(|(w, h, rgba)| (page, scale, w, h, rgba))
    }
}
