//! Imagens para desenhar no [`Canvas`](crate::Canvas): um arquivo codificado (PNG, JPEG,
//! WebP, GIF...) ou pixels RGBA prontos (um quadro de vídeo, uma imagem gerada).
//!
//! O core não decodifica nada: guarda os bytes e um identificador único; cada
//! implementação de `Canvas` decodifica e guarda em cache pelo [`Image::id`]. Uma `Image`
//! é barata de clonar e pode ir para outra thread (um decodificador de vídeo, por exemplo).

use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::geometry::SizeF;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// O maior lado de uma imagem: o tamanho máximo de textura das placas de vídeo (Direct3D 11);
/// acima disso o renderizador não desenha num bitmap só.
pub const MAX_IMAGE_SIDE: u32 = 16_384;
/// A maior área de uma imagem (100 megapixels, cerca de 400 MB em RGBA).
pub const MAX_IMAGE_PIXELS: u64 = 100_000_000;

/// Os bytes RGBA de uma imagem `width × height`, sem estouro e dentro dos limites. Todo
/// lugar que reserva memória para pixels passa por aqui: as dimensões vêm do próprio arquivo,
/// e um PNG de 1 KB pode declarar 40.000 × 40.000.
pub fn checked_size(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 {
        return Err(format!("imagem vazia ({width} × {height})"));
    }
    if width > MAX_IMAGE_SIDE || height > MAX_IMAGE_SIDE || width as u64 * height as u64 > MAX_IMAGE_PIXELS {
        return Err(format!("imagem grande demais: {width} × {height} (máximo {MAX_IMAGE_SIDE} por lado e {} megapixels)", MAX_IMAGE_PIXELS / 1_000_000));
    }
    usize::try_from(width as u64 * height as u64 * 4).map_err(|_| format!("imagem grande demais: {width} × {height}"))
}

/// O conteúdo de uma [`Image`].
#[derive(Debug, PartialEq)]
pub enum ImageSource {
    /// Arquivo de imagem como está no disco (o canvas decodifica).
    Encoded(Vec<u8>),
    /// Pixels RGBA (4 bytes por pixel, linha a linha, sem pré-multiplicar o alfa).
    Rgba { width: u32, height: u32, pixels: Vec<u8> },
    /// Pixels BGRA (a ordem que decodificadores de vídeo e o Windows costumam entregar),
    /// opacos ou já pré-multiplicados: o Direct2D os usa sem converter.
    Bgra { width: u32, height: u32, pixels: Vec<u8> },
}

#[derive(Debug)]
struct ImageData {
    id: u64,
    source: ImageSource,
}

/// Imagem para [`Canvas::draw_image`](crate::Canvas::draw_image).
#[derive(Clone, Debug)]
pub struct Image(Arc<ImageData>);

impl Image {
    /// Um arquivo de imagem (PNG, JPEG, WebP, GIF, BMP, ICO; SVG conforme o canvas).
    pub fn encoded(bytes: Vec<u8>) -> Self {
        Self::new(ImageSource::Encoded(bytes))
    }

    /// Pixels RGBA prontos. `pixels.len()` precisa ser `width * height * 4`.
    pub fn rgba(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        debug_assert_eq!(pixels.len(), width as usize * height as usize * 4);
        Self::new(ImageSource::Rgba { width, height, pixels })
    }

    /// Pixels BGRA prontos (`width * height * 4` bytes), opacos ou pré-multiplicados: o
    /// caminho mais rápido para quadros de vídeo.
    pub fn bgra(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        debug_assert_eq!(pixels.len(), width as usize * height as usize * 4);
        Self::new(ImageSource::Bgra { width, height, pixels })
    }

    fn new(source: ImageSource) -> Self {
        Self(Arc::new(ImageData { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), source }))
    }

    /// Identificador único (cada imagem criada tem um novo): a chave do cache do canvas.
    pub fn id(&self) -> u64 {
        self.0.id
    }

    /// Uma referência que não segura a imagem: o cache do canvas guarda isto ao lado do que
    /// fez dela (decodificada, na GPU) e solta quando o app solta a imagem.
    pub fn downgrade(&self) -> WeakImage {
        WeakImage(Arc::downgrade(&self.0))
    }

    pub fn source(&self) -> &ImageSource {
        &self.0.source
    }

    /// Tamanho em pixels, quando já é conhecido sem decodificar (imagens RGBA).
    pub fn known_size(&self) -> Option<SizeF> {
        match &self.0.source {
            ImageSource::Rgba { width, height, .. } | ImageSource::Bgra { width, height, .. } => {
                Some(SizeF::new(*width as f32, *height as f32))
            }
            ImageSource::Encoded(_) => None,
        }
    }
}

/// Referência a uma [`Image`] que não a mantém viva ([`Image::downgrade`]). Um `id` só volta
/// a ser pedido por quem ainda tem a imagem: morta a imagem, o que o cache guardou dela não
/// serve mais para nada (uma prévia trocada a cada meio segundo, por exemplo).
#[derive(Clone, Debug)]
pub struct WeakImage(Weak<ImageData>);

impl WeakImage {
    /// O app ainda tem a imagem?
    pub fn is_alive(&self) -> bool {
        self.0.strong_count() > 0
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

/// Como uma imagem ocupa o retângulo de destino (como o `WrapMode` do `TImage`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImageFit {
    /// Inteira, mantendo a proporção (pode sobrar espaço).
    #[default]
    Contain,
    /// Cobre tudo, mantendo a proporção (pode cortar as bordas).
    Cover,
    /// Estica para o retângulo (pode distorcer).
    Stretch,
    /// Tamanho original, centralizada.
    Center,
    /// Tamanho original, no canto superior esquerdo.
    Original,
}

impl crate::prop::PropEnum for ImageFit {
    const NAMES: &'static [&'static str] = &["contain", "cover", "stretch", "center", "original"];

    fn from_index(index: usize) -> Option<Self> {
        [Self::Contain, Self::Cover, Self::Stretch, Self::Center, Self::Original].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl ImageFit {
    /// Retângulos de origem (na imagem) e de destino para desenhar uma imagem de tamanho
    /// `image` dentro de `dest`.
    pub fn place(self, image: SizeF, dest: crate::RectF) -> (crate::RectF, crate::RectF) {
        use crate::RectF;
        let full = RectF::from_size(image);
        if image.width <= 0.0 || image.height <= 0.0 {
            return (full, dest);
        }
        let centered = |w: f32, h: f32| RectF::new(dest.x + (dest.width - w) / 2.0, dest.y + (dest.height - h) / 2.0, w, h);
        match self {
            ImageFit::Stretch => (full, dest),
            ImageFit::Contain => {
                let s = (dest.width / image.width).min(dest.height / image.height);
                (full, centered(image.width * s, image.height * s))
            }
            ImageFit::Cover => {
                // Corta a imagem para a proporção do destino.
                let s = (dest.width / image.width).max(dest.height / image.height);
                let (w, h) = (dest.width / s, dest.height / s);
                (RectF::new((image.width - w) / 2.0, (image.height - h) / 2.0, w, h), dest)
            }
            ImageFit::Center => (full, centered(image.width, image.height)),
            ImageFit::Original => (full, RectF::new(dest.x, dest.y, image.width, image.height)),
        }
    }
}
