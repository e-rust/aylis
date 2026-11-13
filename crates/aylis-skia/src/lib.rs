//! # aylis-skia
//!
//! Implementa o [`Canvas`](aylis_core::Canvas) e o [`TextService`](aylis_core::TextService)
//! do aylis com o Skia.
//!
//! Hoje desenha na CPU (superfície raster). A GPU entra na fase 3 trocando só a
//! superfície: o [`SkiaCanvas`] é o mesmo nos dois casos, então o resultado é idêntico.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use aylis_core::{
    Brush, Canvas, Color, FontSpec, FontWeight, Image, ImageSource, LineCap, LineJoin, PointF, RectF, SizeF, Stroke, TextAlign,
    TextService, TextStyle,
};
use skia_safe as sk;
use skia_safe::textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TypefaceFontProvider};

/// Dono dos recursos compartilhados do Skia (fontes, imagens decodificadas). Crie um por
/// aplicativo.
#[derive(Clone)]
pub struct SkiaRenderer {
    fonts: FontCollection,
    images: Rc<RefCell<ImageCache>>,
    font_sync: Rc<FontSync>,
    /// Os desenhos guardados como imagem (`Canvas::begin_cached`: o `cache_layer` de um
    /// controle, um mapa parado), pela chave.
    layers: Rc<RefCell<aylis_core::LayerCache<sk::Image>>>,
}

/// Até que geração do registro de fontes (`aylis_core::fonts`) a coleção está em dia.
#[derive(Default)]
struct FontSync {
    generation: std::cell::Cell<u64>,
}

/// Carrega na coleção as fontes de arquivo registradas desde a última vez (as do projeto,
/// `assets/fonts`). Elas são procuradas antes das do sistema.
fn sync_fonts(fonts: &FontCollection, sync: &FontSync) {
    let generation = aylis_core::fonts::generation();
    if sync.generation.get() == generation {
        return;
    }
    sync.generation.set(generation);
    let manager = sk::FontMgr::new();
    let mut provider = TypefaceFontProvider::new();
    for font in aylis_core::fonts::registered() {
        if let Some(typeface) = manager.new_from_data(sk::Data::new_copy(&font.data), None) {
            provider.register_typeface(typeface, Some(font.family.as_str()));
        }
    }
    // A coleção é compartilhada (o serviço de medição usa a mesma).
    let mut shared = fonts.clone();
    shared.set_asset_font_manager(Some(provider.into()));
    shared.clear_caches();
}

/// Imagens de arquivo já decodificadas, até o limite `files` do projeto (as RGBA, como
/// quadros de vídeo, não entram: o Skia as usa direto dos pixels).
struct ImageCache {
    decoded: aylis_core::ImageCache<Option<sk::Image>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        Self { decoded: aylis_core::ImageCache::named("Skia · arquivos decodificados") }
    }
}

impl ImageCache {
    fn get(&mut self, image: &Image) -> Option<sk::Image> {
        match image.source() {
            ImageSource::Rgba { width, height, pixels } => {
                let info = rgba_info(*width, *height).with_alpha_type(sk::AlphaType::Unpremul);
                sk::images::raster_from_data(&info, sk::Data::new_copy(pixels), *width as usize * 4)
            }
            ImageSource::Bgra { width, height, pixels } => {
                // O contrato do tipo: opaco ou já pré-multiplicado, como o Direct2D usa (antes o
                // Skia tratava como não pré-multiplicado, e o semitransparente saía diferente; o
                // relatório do Tabernáculo, T08).
                let info = sk::ImageInfo::new((*width as i32, *height as i32), sk::ColorType::BGRA8888, sk::AlphaType::Premul, None);
                sk::images::raster_from_data(&info, sk::Data::new_copy(pixels), *width as usize * 4)
            }
            ImageSource::Encoded(bytes) => {
                if let Some(cached) = self.decoded.get(image) {
                    return cached.clone();
                }
                let _profile = aylis_core::profile::scope(aylis_core::profile::Lane::Resource, || format!("imagem ({} KB)", bytes.len() / 1024));
                // O tamanho vem do cabeçalho, antes de decodificar: uma bomba de imagem não chega
                // a reservar memória.
                let decoded = sk::Image::from_encoded(sk::Data::new_copy(bytes))
                    .filter(|i| aylis_core::image::checked_size(i.width() as u32, i.height() as u32).is_ok())
                    .and_then(|i| i.make_raster_image(None, None));
                let files = aylis_core::image_cache::limits().files as usize;
                let bytes = decoded.as_ref().map_or(0, |i| i.width() as usize * i.height() as usize * 4);
                self.decoded.insert(image, decoded.clone(), bytes, files, usize::MAX);
                decoded
            }
        }
    }
}

impl Default for SkiaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl SkiaRenderer {
    pub fn new() -> Self {
        let mut fonts = FontCollection::new();
        fonts.set_default_font_manager(sk::FontMgr::new(), None);
        Self { fonts, images: Rc::default(), font_sync: Rc::default(), layers: Rc::new(RefCell::new(aylis_core::LayerCache::named("Skia · camadas guardadas"))) }
    }

    /// Embrulha um canvas do Skia como um canvas do aylis.
    pub fn canvas<'a>(&'a self, canvas: &'a sk::Canvas) -> SkiaCanvas<'a> {
        sync_fonts(&self.fonts, &self.font_sync);
        SkiaCanvas { canvas, fonts: &self.fonts, images: &self.images, masks: Vec::new(), layers: &self.layers, cached: Vec::new() }
    }

    /// Serviço de medição de texto com as mesmas fontes usadas na pintura.
    pub fn text_service(&self) -> SkiaTextService {
        SkiaTextService { fonts: self.fonts.clone(), font_sync: self.font_sync.clone() }
    }

    /// Pinta direto num buffer de pixels `0x00RRGGBB` (o formato do `softbuffer`), sem
    /// cópia. `scale` é a escala de DPI: o aylis trabalha em unidades lógicas.
    ///
    /// Devolve `false` se o buffer não tiver o tamanho `width * height`.
    pub fn render_to_pixels(
        &self,
        pixels: &mut [u32],
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        paint: impl FnOnce(&mut SkiaCanvas<'_>),
    ) -> bool {
        self.render_area_to_pixels(pixels, width, height, scale, background, None, paint)
    }

    /// Como [`render_to_pixels`](Self::render_to_pixels), repintando só `area` (unidades
    /// lógicas) por cima do que já está em `pixels` (o quadro anterior); `None` é tudo.
    #[allow(clippy::too_many_arguments)]
    pub fn render_area_to_pixels(
        &self,
        pixels: &mut [u32],
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        area: Option<aylis_core::RectF>,
        paint: impl FnOnce(&mut SkiaCanvas<'_>),
    ) -> bool {
        if width == 0 || height == 0 || pixels.len() != (width as usize) * (height as usize) {
            return false;
        }
        // BGRA em little-endian lido como u32 é 0xAARRGGBB.
        let info = sk::ImageInfo::new(
            (width as i32, height as i32),
            sk::ColorType::BGRA8888,
            sk::AlphaType::Premul,
            None,
        );
        // SAFETY: u8 não tem requisito de alinhamento e o slice cobre exatamente a mesma
        // memória dos u32, que fica emprestada só durante este bloco.
        let bytes = unsafe { std::slice::from_raw_parts_mut(pixels.as_mut_ptr().cast::<u8>(), pixels.len() * 4) };
        let Some(mut surface) = sk::surfaces::wrap_pixels(&info, bytes, Some(width as usize * 4), None) else {
            return false;
        };
        self.draw_area(surface.canvas(), scale, background, area, paint);
        drop(surface);
        // Alguns backends do softbuffer exigem o byte alto zerado (só onde pintou: numa tela
        // gigante, passar em todos os pixels a cada quadro custava mais que a pintura).
        clear_alpha(pixels, width, height, scale, area.as_slice());
        true
    }

    /// Um quadro por parte em `pixels` (o quadro anterior): primeiro as cópias (os pixels que
    /// andaram, a rolagem), depois as áreas, pintadas de uma vez com o recorte nelas (`paint`
    /// recebe as áreas, para pintar só o que toca alguma, `Ui::paint_regions`). Unidades lógicas, como em
    /// [`render_area_to_pixels`](Self::render_area_to_pixels).
    #[allow(clippy::too_many_arguments)]
    pub fn render_parts_to_pixels(
        &self,
        pixels: &mut [u32],
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        moves: &[aylis_core::ScrollMove],
        areas: &[aylis_core::RectF],
        paint: impl FnOnce(&mut SkiaCanvas<'_>, &[aylis_core::RectF]),
    ) -> bool {
        if width == 0 || height == 0 || pixels.len() != (width as usize) * (height as usize) {
            return false;
        }
        let info = sk::ImageInfo::new((width as i32, height as i32), sk::ColorType::BGRA8888, sk::AlphaType::Premul, None);
        // SAFETY: como em `render_area_to_pixels`.
        let bytes = unsafe { std::slice::from_raw_parts_mut(pixels.as_mut_ptr().cast::<u8>(), pixels.len() * 4) };
        let Some(mut surface) = sk::surfaces::wrap_pixels(&info, bytes, Some(width as usize * 4), None) else {
            return false;
        };
        apply_moves(&mut surface, scale, moves);
        self.draw_areas(surface.canvas(), scale, background, areas, paint);
        drop(surface);
        let moved: Vec<aylis_core::RectF> = moves.iter().map(|m| m.rect.offset(m.delta.x, m.delta.y)).collect();
        clear_alpha(pixels, width, height, scale, areas);
        if !moved.is_empty() {
            clear_alpha(pixels, width, height, scale, &moved);
        }
        true
    }

    /// Pinta numa imagem e devolve o PNG (útil para testes e capturas de tela).
    pub fn render_png(
        &self,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        paint: impl FnOnce(&mut SkiaCanvas<'_>),
    ) -> Option<Vec<u8>> {
        aylis_core::image::checked_size(width, height).ok()?;
        let mut surface = sk::surfaces::raster_n32_premul((width as i32, height as i32))?;
        self.draw(surface.canvas(), scale, background, paint);
        let image = surface.image_snapshot();
        let data = image.encode(None, sk::EncodedImageFormat::PNG, 100)?;
        Some(data.as_bytes().to_vec())
    }

    /// Pinta numa imagem e devolve os pixels RGBA (4 bytes por pixel, linha a linha).
    pub fn render_rgba(
        &self,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        paint: impl FnOnce(&mut SkiaCanvas<'_>),
    ) -> Option<Vec<u8>> {
        aylis_core::image::checked_size(width, height).ok()?;
        let info = rgba_info(width, height);
        let mut surface = sk::surfaces::raster(&info, None, None)?;
        self.draw(surface.canvas(), scale, background, paint);
        read_rgba(&mut surface, width, height)
    }

    pub(crate) fn draw(&self, canvas: &sk::Canvas, scale: f32, background: Color, paint: impl FnOnce(&mut SkiaCanvas<'_>)) {
        self.draw_area(canvas, scale, background, None, paint);
    }

    /// Como [`draw_area`](Self::draw_area), com várias áreas pintadas de uma vez: o recorte é
    /// a união delas (nos pixels inteiros) e `paint` recebe as áreas (um controle caro que
    /// aparece em duas áreas é pintado uma vez só; um que não toca nenhuma, nem é pintado).
    pub(crate) fn draw_areas(&self, canvas: &sk::Canvas, scale: f32, background: Color, areas: &[aylis_core::RectF], paint: impl FnOnce(&mut SkiaCanvas<'_>, &[aylis_core::RectF])) {
        if areas.is_empty() {
            return;
        }
        let mut region = sk::Region::new();
        let mut pixels = 0i64;
        for a in areas {
            let r = pixel_rect(*a, scale);
            pixels += r.width() as i64 * r.height() as i64;
            region.op_rect(r, sk::region::RegionOp::Union);
        }
        self.draw_region(canvas, scale, background, &region, pixels, |c| paint(c, areas));
    }

    /// Como [`draw`](Self::draw), só na área `area` (unidades lógicas): o resto do canvas
    /// fica como estava (a imagem do quadro anterior). O recorte é sem suavização, nos
    /// pixels inteiros, para não aparecer emenda na borda.
    pub(crate) fn draw_area(&self, canvas: &sk::Canvas, scale: f32, background: Color, area: Option<aylis_core::RectF>, paint: impl FnOnce(&mut SkiaCanvas<'_>)) {
        match area {
            Some(a) => {
                let r = pixel_rect(a, scale);
                self.draw_region(canvas, scale, background, &sk::Region::from_rect(r), r.width() as i64 * r.height() as i64, paint)
            }
            None => {
                canvas.save();
                canvas.clear(to_sk(background));
                canvas.scale((scale, scale));
                paint(&mut self.canvas(canvas));
                canvas.restore();
            }
        }
    }

    /// Pinta só em `region` (pixels do canvas). Na CPU, a pintura vai para uma superfície à
    /// parte, [`REGION_MARGIN`] pixels maior de cada lado, e só a região volta ao canvas: o
    /// rasterizador do Skia corta as curvas na borda do recorte e as refaz em retas de novo,
    /// e um círculo cortado pela área (o polegar de um TrackBar ao lado do controle que mudou)
    /// saía com a beirada até uns 100 tons diferente da pintura inteira, até 4 pixels para
    /// dentro. Com a margem, o corte fica fora do que volta. Pintam direto: a GPU (que recorta
    /// sem refazer as curvas), as áreas espalhadas (o retângulo que as junta passa do dobro
    /// delas: a tela gigante, deslizando, ia de 3 para 8 ms) e as grandes (mais de
    /// [`REGION_ASIDE_MAX`] pixels), em que copiar custa mais que a beirada vale.
    fn draw_region(&self, canvas: &sk::Canvas, scale: f32, background: Color, region: &sk::Region, pixels: i64, paint: impl FnOnce(&mut SkiaCanvas<'_>)) {
        let size = canvas.base_layer_size();
        let device = sk::IRect::from_wh(size.width, size.height);
        let bounds = region.bounds();
        let outer = sk::IRect::from_ltrb(bounds.left - REGION_MARGIN, bounds.top - REGION_MARGIN, bounds.right + REGION_MARGIN, bounds.bottom + REGION_MARGIN);
        let outer = sk::IRect::intersect(&outer, &device).unwrap_or_default();
        let together = bounds.width() as i64 * bounds.height() as i64 <= pixels * 2;
        let small = together && (outer.width() as i64 * outer.height() as i64) <= REGION_ASIDE_MAX;
        #[cfg(feature = "gl")]
        let gpu = canvas.direct_context().is_some();
        #[cfg(not(feature = "gl"))]
        let gpu = false;
        let info = canvas.image_info().with_dimensions(outer.size());
        let aside = if small && !gpu && !outer.is_empty() { canvas.new_surface(&info, None) } else { None };
        canvas.save();
        canvas.clip_region(region, sk::ClipOp::Intersect);
        match aside {
            Some(mut surface) => {
                let target = surface.canvas();
                target.clear(to_sk(background));
                target.translate((-outer.left as f32, -outer.top as f32));
                target.scale((scale, scale));
                paint(&mut self.canvas(target));
                let mut copy = sk::Paint::default();
                copy.set_blend_mode(sk::BlendMode::Src);
                canvas.draw_image(surface.image_snapshot(), (outer.left as f32, outer.top as f32), Some(&copy));
            }
            None => {
                canvas.clear(to_sk(background));
                canvas.scale((scale, scale));
                paint(&mut self.canvas(canvas));
            }
        }
        canvas.restore();
    }
}

/// Quanto a superfície da pintura por parte passa da área, em pixels (veja
/// [`SkiaRenderer::draw_region`]): com 16, a beirada de um círculo cortado muda no máximo uns
/// poucos tons (com 4, ainda uns 45).
const REGION_MARGIN: i32 = 16;

/// Até quantos pixels a pintura por parte vai para a superfície à parte (uns 2 MB).
const REGION_ASIDE_MAX: i64 = 512 * 1024;

/// A área (unidades lógicas) alargada até os pixels inteiros.
fn pixel_rect(a: aylis_core::RectF, scale: f32) -> sk::IRect {
    sk::IRect::from_ltrb((a.x * scale).floor() as i32, (a.y * scale).floor() as i32, (a.right() * scale).ceil() as i32, (a.bottom() * scale).ceil() as i32)
}

#[cfg(feature = "gl")]
thread_local! {
    /// O contexto OpenGL do quadro sendo pintado (função das funções e identificação),
    /// para quem desenha por fora, como o 3D, usar o mesmo.
    static GL_FRAME: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) };
}

/// Canvas do aylis sobre um canvas do Skia.
pub struct SkiaCanvas<'a> {
    canvas: &'a sk::Canvas,
    fonts: &'a FontCollection,
    images: &'a RefCell<ImageCache>,
    /// Máscaras abertas: com `inverse`.
    masks: Vec<bool>,
    layers: &'a RefCell<aylis_core::LayerCache<sk::Image>>,
    /// Os `begin_cached` abertos (`None`: não deu para guardar, desenha direto).
    cached: Vec<Option<CacheState<'a>>>,
}

/// Um desenho sendo guardado: o canvas de antes e a superfície onde ele vai.
struct CacheState<'a> {
    main: &'a sk::Canvas,
    surface: sk::Surface,
    key: u64,
    /// A chave do núcleo (para soltar pela `forget_cached`).
    owner: u64,
    at: (f32, f32),
}

impl SkiaCanvas<'_> {
    /// O canvas do Skia por baixo, para quem quiser desenhar algo que o trait não cobre.
    pub fn skia(&self) -> &sk::Canvas {
        self.canvas
    }
}

impl SkiaCanvas<'_> {
    /// Desenha um parágrafo de uma linha no retângulo, com os alinhamentos do estilo.
    fn paint_line(&mut self, paragraph: &Paragraph, rect: RectF, style: &TextStyle) {
        let width = paragraph.max_intrinsic_width();
        let x = match style.h_align {
            TextAlign::Leading | TextAlign::Justify => rect.x,
            TextAlign::Center => rect.x + (rect.width - width) / 2.0,
            TextAlign::Trailing => rect.right() - width,
        };
        let free = rect.height - paragraph.height();
        let y = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => rect.y,
            TextAlign::Center => rect.y + free / 2.0,
            TextAlign::Trailing => rect.y + free,
        };
        paragraph.paint(self.canvas, (x, y));
    }
}

impl<'a> Canvas for SkiaCanvas<'a> {
    fn device_scale(&self) -> f32 {
        let m = self.canvas.local_to_device_as_3x3();
        (m.scale_x() * m.scale_y() - m.skew_x() * m.skew_y()).abs().sqrt()
    }

    #[cfg(feature = "gl")]
    fn gpu_device(&self) -> Option<aylis_core::GpuDevice> {
        let (loader, context) = GL_FRAME.try_with(|g| g.get()).ok().flatten()?;
        self.canvas.direct_context()?;
        Some(aylis_core::GpuDevice::OpenGl { loader, context })
    }

    #[cfg(feature = "gl")]
    fn draw_gpu_surface(&mut self, dest: RectF, surface: &aylis_core::GpuSurface) -> bool {
        use skia_safe::gpu;
        let aylis_core::GpuSurface::GlTexture { texture, width, height, .. } = *surface else { return false };
        let Some(mut context) = self.canvas.direct_context() else { return false };
        // Quem desenhou mexeu no estado do OpenGL: o Skia não pode confiar no que guardou.
        context.reset(None);
        let info = gpu::gl::TextureInfo { target: 0x0DE1, id: texture, format: 0x8058, ..Default::default() };
        let backend = unsafe { gpu::backend_textures::make_gl((width as i32, height as i32), gpu::Mipmapped::No, info, "aylis-3d") };
        let Some(image) = gpu::images::borrow_texture_from(
            &mut context,
            &backend,
            gpu::SurfaceOrigin::TopLeft,
            sk::ColorType::RGBA8888,
            sk::AlphaType::Premul,
            None,
        ) else {
            return false;
        };
        let paint = sk::Paint::default();
        self.canvas.draw_image_rect_with_sampling_options(
            image,
            None,
            sk::Rect::from_xywh(dest.x, dest.y, dest.width, dest.height),
            sk::SamplingOptions::new(sk::FilterMode::Linear, sk::MipmapMode::None),
            &paint,
        );
        true
    }

    #[cfg(feature = "gl")]
    fn gpu_state_changed(&mut self) {
        if let Some(mut context) = self.canvas.direct_context() {
            context.reset(None);
        }
    }

    fn save(&mut self) {
        self.canvas.save();
    }

    fn restore(&mut self) {
        self.canvas.restore();
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas.translate((dx, dy));
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas.scale((sx, sy));
    }

    fn rotate(&mut self, degrees: f32) {
        self.canvas.rotate(degrees, None);
    }

    fn clip_rect(&mut self, rect: RectF) {
        self.canvas.clip_rect(to_rect(rect), sk::ClipOp::Intersect, true);
    }

    fn save_layer(&mut self, opacity: f32) {
        self.canvas.save_layer_alpha_f(None, opacity);
    }

    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        let Some(filter) = sk::image_filters::blur((blur, blur), sk::TileMode::Clamp, None, None) else { return false };
        self.canvas.save();
        self.canvas.clip_rrect(sk::RRect::new_rect_xy(to_rect(rect), radius, radius), sk::ClipOp::Intersect, true);
        let bounds = to_rect(rect);
        self.canvas.save_layer(&sk::canvas::SaveLayerRec::default().bounds(&bounds).backdrop(&filter));
        self.canvas.restore();
        self.canvas.restore();
        true
    }

    fn begin_cached(&mut self, key: u64, r: RectF) -> bool {
        use std::hash::{Hash, Hasher};
        let m = self.canvas.local_to_device_as_3x3();
        // Só sem giro: a imagem vai para a tela pixel a pixel.
        if m.skew_x() != 0.0 || m.skew_y() != 0.0 || m.scale_x() <= 0.0 || m.scale_y() <= 0.0 || m.has_perspective() {
            self.cached.push(None);
            return false;
        }
        let x0 = (r.x * m.scale_x() + m.translate_x()).round();
        let y0 = (r.y * m.scale_y() + m.translate_y()).round();
        let (w, h) = ((r.width * m.scale_x()).ceil(), (r.height * m.scale_y()).ceil());
        if w < 1.0 || h < 1.0 || w > 8192.0 || h > 8192.0 {
            self.cached.push(None);
            return false;
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (key, w as u32, h as u32, m.scale_x().to_bits(), m.scale_y().to_bits()).hash(&mut hasher);
        // Na GPU, a imagem guardada é uma textura do contexto da janela que a desenhou, e o
        // renderizador é um só para todas as janelas: a chave leva o contexto (a aba de código
        // arrastada para uma janela nova achava a imagem da outra e o minimapa ficava vazio;
        // relatado pelo autor em 07/11).
        #[cfg(feature = "gl")]
        if let Some(context) = self.canvas.direct_context() {
            format!("{:?}", context.id()).hash(&mut hasher);
        }
        let full = hasher.finish();
        if let Some(image) = self.layers.borrow_mut().get(full) {
            draw_at_device(self.canvas, image, (x0, y0));
            return true;
        }
        // Sem lugar no limite das camadas (`image_cache.layers_mb`): desenha direto.
        if !aylis_core::LayerCache::<sk::Image>::accepts(w as usize * h as usize * 4) {
            self.cached.push(None);
            return false;
        }
        let info = sk::ImageInfo::new_n32_premul((w as i32, h as i32), None);
        let Some(mut surface) = self.canvas.new_surface(&info, None) else {
            self.cached.push(None);
            return false;
        };
        // SAFETY: o canvas é da superfície, que fica guardada em `cached` (no heap do Skia,
        // não anda com o `Surface`) até o `end_cached`, que volta para `main` antes de soltá-la.
        let target: &'a sk::Canvas = unsafe { &*std::ptr::from_ref(surface.canvas()) };
        target.clear(sk::Color::TRANSPARENT);
        // A mesma escala; a origem da imagem é o canto dela na tela.
        target.translate((-x0, -y0));
        target.concat(&m);
        let main = std::mem::replace(&mut self.canvas, target);
        self.cached.push(Some(CacheState { main, surface, key: full, owner: key, at: (x0, y0) }));
        false
    }

    fn end_cached(&mut self) {
        let Some(mut state) = self.cached.pop().flatten() else { return };
        self.canvas = state.main;
        let image = state.surface.image_snapshot();
        drop(state.surface);
        draw_at_device(self.canvas, &image, state.at);
        // O limite é em bytes (`image_cache.layers_mb`): cada destaque novo de um mapa é uma
        // imagem nova, e sai a usada há mais tempo.
        let bytes = image.width().max(0) as usize * image.height().max(0) as usize * 4;
        self.layers.borrow_mut().insert(state.owner, state.key, image, bytes);
    }

    fn forget_cached(&mut self, key: u64) {
        self.layers.borrow_mut().forget(key);
    }

    fn begin_masked(&mut self, inverse: bool) -> bool {
        self.canvas.save_layer(&sk::canvas::SaveLayerRec::default());
        self.masks.push(inverse);
        true
    }

    fn begin_mask(&mut self) {
        let inverse = self.masks.last().copied().unwrap_or(false);
        let mut paint = sk::Paint::default();
        paint.set_blend_mode(if inverse { sk::BlendMode::DstOut } else { sk::BlendMode::DstIn });
        self.canvas.save_layer(&sk::canvas::SaveLayerRec::default().paint(&paint));
    }

    fn end_mask(&mut self) {
        if self.masks.pop().is_some() {
            self.canvas.restore();
            self.canvas.restore();
        }
    }

    fn clip_path(&mut self, path: &aylis_core::Path, inverse: bool) {
        let op = if inverse { sk::ClipOp::Difference } else { sk::ClipOp::Intersect };
        self.canvas.clip_path(&to_path(path), op, true);
    }

    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        let paint = fill_paint(brush);
        if radius > 0.0 {
            self.canvas.draw_rrect(sk::RRect::new_rect_xy(to_rect(rect), radius, radius), &paint);
        } else {
            self.canvas.draw_rect(to_rect(rect), &paint);
        }
    }

    /// O traço fica do lado de dentro do retângulo, para não ser cortado pelo clip do pai.
    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        let inset = rect.inflate(-stroke.width / 2.0);
        let paint = stroke_paint(stroke);
        if radius > 0.0 {
            let r = (radius - stroke.width / 2.0).max(0.0);
            self.canvas.draw_rrect(sk::RRect::new_rect_xy(to_rect(inset), r, r), &paint);
        } else {
            self.canvas.draw_rect(to_rect(inset), &paint);
        }
    }

    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        self.canvas.draw_oval(to_rect(rect), &fill_paint(brush));
    }

    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke) {
        if stroke.width > 0.0 {
            self.canvas.draw_oval(to_rect(rect.inflate(-stroke.width / 2.0)), &stroke_paint(stroke));
        }
    }

    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        self.canvas.draw_line((from.x, from.y), (to.x, to.y), &stroke_paint(stroke));
    }

    fn fill_path(&mut self, path: &aylis_core::Path, brush: &Brush) {
        self.canvas.draw_path(&to_path(path), &fill_paint(brush));
    }

    fn stroke_path(&mut self, path: &aylis_core::Path, stroke: &Stroke) {
        if stroke.width > 0.0 {
            self.canvas.draw_path(&to_path(path), &stroke_paint(stroke));
        }
    }

    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &aylis_core::Shadow) {
        if shadow.color.a == 0 {
            return;
        }
        let r = rect.inflate(shadow.spread);
        let r = RectF::new(r.x + shadow.offset.x, r.y + shadow.offset.y, r.width, r.height);
        let mut paint = sk::Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(to_sk(shadow.color));
        if shadow.blur > 0.0 {
            // O "raio" do CSS equivale a ~2 sigmas.
            paint.set_mask_filter(sk::MaskFilter::blur(sk::BlurStyle::Normal, shadow.blur / 2.0, false));
        }
        let radius = (radius + shadow.spread).max(0.0);
        self.canvas.draw_rrect(sk::RRect::new_rect_xy(to_rect(r), radius, radius), &paint);
    }

    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        if text.is_empty() || rect.width <= 0.0 {
            return;
        }
        // Linhas pela regra comum (quebra e reticências iguais em todos os renderizadores).
        let fonts = self.fonts;
        let mut plain = style.clone();
        plain.h_align = TextAlign::Leading;
        plain.wrap = true; // sem o "…" do próprio Skia: o corte já vem feito
        // Caso comum (uma linha que cabe, ou sem corte): um parágrafo só.
        if !style.wrap && !text.contains('\n') {
            let mut paragraph = line_paragraph(fonts, text, &plain);
            let width = paragraph.max_intrinsic_width();
            if width > rect.width && style.trimming != aylis_core::Trimming::None {
                let mut measure = |s: &str| measure_line(fonts, s, &plain).width;
                let cut = aylis_core::text::trim_line(text, rect.width, style.trimming, &mut measure);
                paragraph = line_paragraph(fonts, &cut, &plain);
            }
            self.paint_line(&paragraph, rect, style);
            return;
        }
        let line_height = measure_line(fonts, "Ag", &plain).height.max(1.0);
        let max_lines = ((rect.height + 1.0) / line_height).floor().max(1.0) as usize;
        let mut measure = |s: &str| measure_line(fonts, s, &plain).width;
        let lines = aylis_core::text::layout_lines_marked(text, rect.width, max_lines, style.wrap, style.trimming, &mut measure);
        let total = line_height * lines.len() as f32;
        let top = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => rect.y,
            TextAlign::Center => rect.y + (rect.height - total) / 2.0,
            TextAlign::Trailing => rect.bottom() - total,
        };
        for (i, (line, end)) in lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }
            // Justificado: cada palavra no lugar dela (a última linha do parágrafo fica à
            // esquerda).
            let words = (style.h_align == TextAlign::Justify && !end).then(|| aylis_core::text::justify_line(line, rect.width, &mut measure)).flatten();
            if let Some(words) = words {
                for (x, word) in words {
                    line_paragraph(fonts, &word, &plain).paint(self.canvas, (rect.x + x, top + line_height * i as f32));
                }
                continue;
            }
            let paragraph = line_paragraph(fonts, line, &plain);
            let width = paragraph.max_intrinsic_width();
            let x = match style.h_align {
                TextAlign::Leading | TextAlign::Justify => rect.x,
                TextAlign::Center => rect.x + (rect.width - width) / 2.0,
                TextAlign::Trailing => rect.right() - width,
            };
            paragraph.paint(self.canvas, (x, top + line_height * i as f32));
        }
    }

    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        measure_line(self.fonts, text, style)
    }

    fn draw_image(&mut self, image: &Image, src: RectF, dest: RectF, opacity: f32) {
        let Some(sk_image) = self.images.borrow_mut().get(image) else { return };
        let mut paint = sk::Paint::default();
        paint.set_anti_alias(true);
        paint.set_alpha_f(opacity.clamp(0.0, 1.0));
        let sampling = sk::SamplingOptions::new(sk::FilterMode::Linear, sk::MipmapMode::None);
        self.canvas.draw_image_rect_with_sampling_options(
            &sk_image,
            Some((&to_rect(src), sk::canvas::SrcRectConstraint::Fast)),
            to_rect(dest),
            sampling,
            &paint,
        );
    }

    fn image_size(&mut self, image: &Image) -> Option<SizeF> {
        if let Some(size) = image.known_size() {
            return Some(size);
        }
        let decoded = self.images.borrow_mut().get(image)?;
        Some(SizeF::new(decoded.width() as f32, decoded.height() as f32))
    }

    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        let rrect = sk::RRect::new_rect_xy(to_rect(rect), radius, radius);
        self.canvas.clip_rrect(rrect, sk::ClipOp::Intersect, true);
    }
}

fn measure_line(fonts: &FontCollection, text: &str, style: &TextStyle) -> SizeF {
    if text.is_empty() {
        return SizeF::new(0.0, style.font.size * 1.3);
    }
    let paragraph = line_paragraph(fonts, text, style);
    SizeF::new(paragraph.max_intrinsic_width(), paragraph.height())
}

/// A chave de um parágrafo de uma linha já montado: o texto e tudo do estilo que muda a forma
/// ou a cor das letras.
#[derive(Clone, PartialEq, Eq, Hash)]
struct LineKey {
    text: String,
    family: String,
    size: u32,
    weight: u16,
    flags: u8,
    color: u32,
    align: u8,
    /// A coleção de fontes (a do renderizador e a do serviço de medição são outras) e a
    /// geração das fontes do projeto.
    fonts: usize,
    generation: u64,
}

thread_local! {
    /// Os parágrafos de uma linha já montados e medidos (o HarfBuzz e a busca das fontes
    /// custam caro): a tela repintada a cada quadro (a rolagem, uma animação) desenha e mede
    /// os mesmos textos de novo. Cheio, esvazia.
    static LINES: RefCell<HashMap<LineKey, Rc<Paragraph>>> = RefCell::new(HashMap::new());
}

/// Quantos parágrafos o cache guarda antes de esvaziar.
const MAX_LINES: usize = 8192;

/// O parágrafo de uma linha (`layout` sem limite de largura), montado ou do cache.
fn line_paragraph(fonts: &FontCollection, text: &str, style: &TextStyle) -> Rc<Paragraph> {
    let font = &style.font;
    let key = LineKey {
        text: text.to_string(),
        family: font.family.clone(),
        size: font.size.to_bits(),
        weight: font.weight as u16,
        flags: u8::from(font.italic) | u8::from(font.underline) << 1 | u8::from(font.strikeout) << 2 | u8::from(style.wrap) << 3,
        color: u32::from_be_bytes([style.color.r, style.color.g, style.color.b, style.color.a]),
        align: style.h_align as u8,
        fonts: fonts as *const FontCollection as usize,
        generation: aylis_core::fonts::generation(),
    };
    if let Some(hit) = LINES.with(|c| c.borrow().get(&key).cloned()) {
        return hit;
    }
    let mut paragraph = build_paragraph(fonts, text, style);
    paragraph.layout(1.0e6);
    let paragraph = Rc::new(paragraph);
    LINES.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() >= MAX_LINES {
            c.clear();
        }
        c.insert(key, paragraph.clone());
    });
    paragraph
}

/// Medição de texto com Skia.
pub struct SkiaTextService {
    fonts: FontCollection,
    font_sync: Rc<FontSync>,
}

impl TextService for SkiaTextService {
    fn measure(&self, text: &str, style: &TextStyle, max_width: Option<f32>) -> SizeF {
        sync_fonts(&self.fonts, &self.font_sync);
        if max_width.is_none() {
            return measure_line(&self.fonts, text, style);
        }
        if text.is_empty() {
            return SizeF::ZERO;
        }
        let mut paragraph = build_paragraph(&self.fonts, text, style);
        paragraph.layout(max_width.unwrap_or(1.0e6));
        let width = match max_width {
            Some(w) => paragraph.longest_line().min(w),
            None => paragraph.max_intrinsic_width(),
        };
        SizeF::new(width.ceil(), paragraph.height().ceil())
    }
}

/// Converte pixels RGBA (como os de [`SkiaRenderer::render_rgba`]) em PNG.
pub fn encode_png(rgba: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let info = rgba_info(width, height);
    let image = sk::images::raster_from_data(&info, sk::Data::new_copy(rgba), width as usize * 4)?;
    let data = image.encode(None, sk::EncodedImageFormat::PNG, 100)?;
    Some(data.as_bytes().to_vec())
}

/// Decodifica uma imagem (PNG, JPEG, WebP...) em pixels RGBA sem pré-multiplicar:
/// `(pixels, largura, altura)`.
pub fn decode_rgba(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let image = sk::Image::from_encoded(sk::Data::new_copy(bytes))?;
    let (w, h) = (image.width() as u32, image.height() as u32);
    let info = rgba_info(w, h).with_alpha_type(sk::AlphaType::Unpremul);
    let mut pixels = vec![0u8; aylis_core::image::checked_size(w, h).ok()?];
    image.read_pixels(&info, &mut pixels, w as usize * 4, (0, 0), sk::image::CachingHint::Allow).then_some((pixels, w, h))
}

fn rgba_info(width: u32, height: u32) -> sk::ImageInfo {
    sk::ImageInfo::new((width as i32, height as i32), sk::ColorType::RGBA8888, sk::AlphaType::Premul, None)
}

fn read_rgba(surface: &mut sk::Surface, width: u32, height: u32) -> Option<Vec<u8>> {
    let mut pixels = vec![0u8; width as usize * height as usize * 4];
    surface
        .read_pixels(&rgba_info(width, height), &mut pixels, width as usize * 4, (0, 0))
        .then_some(pixels)
}

/// Desenho na GPU via OpenGL.
#[cfg(feature = "gl")]
pub mod gl {
    use std::ffi::c_void;

    use aylis_core::Color;
    use skia_safe as sk;
    use skia_safe::gpu;

    use crate::{SkiaCanvas, SkiaRenderer};

    /// Contexto do Skia sobre um contexto OpenGL já criado pela plataforma.
    ///
    /// O contexto OpenGL precisa estar ativo ("current") em todas as chamadas.
    pub struct GlBackend {
        context: gpu::DirectContext,
        target: Option<(sk::Surface, (i32, i32))>,
        /// A imagem da janela guardada entre os quadros (na GPU): o quadro repinta só a área
        /// que mudou nela e a copia inteira para a janela (o buffer da janela fica indefinido
        /// depois da troca).
        layer: Option<(sk::Surface, (i32, i32))>,
        samples: usize,
        stencil: usize,
        /// Função que acha as funções do OpenGL (veja [`GlBackend::set_gl_loader`]).
        loader: usize,
    }

    impl GlBackend {
        /// `load` devolve o endereço de uma função OpenGL pelo nome (vem da plataforma).
        /// `samples` e `stencil` descrevem o framebuffer da janela.
        pub fn new(load: impl FnMut(&str) -> *const c_void, samples: usize, stencil: usize) -> Option<Self> {
            let interface = gpu::gl::Interface::new_load_with(load)?;
            let context = gpu::direct_contexts::make_gl(interface, None)?;
            Some(Self { context, target: None, layer: None, samples, stencil, loader: 0 })
        }

        /// Endereço de uma `unsafe extern "C" fn(*const c_char) -> *const c_void` que acha
        /// as funções do OpenGL deste contexto. Com ela, o canvas oferece o contexto a quem
        /// desenha por fora (o 3D), que entrega texturas sem cópia.
        pub fn set_gl_loader(&mut self, loader: usize) {
            self.loader = loader;
        }

        /// Pinta no framebuffer padrão da janela (o chamador faz o `swap_buffers`).
        pub fn render(
            &mut self,
            renderer: &SkiaRenderer,
            width: u32,
            height: u32,
            scale: f32,
            background: Color,
            paint: impl FnOnce(&mut SkiaCanvas<'_>),
        ) -> bool {
            self.render_area(renderer, width, height, scale, background, None, paint).is_some()
        }

        /// A imagem do quadro anterior está guardada, neste tamanho: o próximo quadro pode
        /// repintar só uma área ([`render_area`](Self::render_area)).
        pub fn keeps_image(&self, width: u32, height: u32) -> bool {
            self.layer.as_ref().is_some_and(|(_, s)| *s == (width as i32, height as i32))
        }

        /// Como [`render`](Self::render), repintando só `area` (unidades lógicas, alinhada
        /// ao pixel) na imagem guardada do quadro anterior; `None` é a janela inteira.
        /// Devolve se a área foi respeitada (`Some(true)`) ou se a janela foi repintada
        /// inteira (`Some(false)`: a imagem guardada é nova); `None` se falhou.
        pub fn render_area(
            &mut self,
            renderer: &SkiaRenderer,
            width: u32,
            height: u32,
            scale: f32,
            background: Color,
            area: Option<aylis_core::RectF>,
            paint: impl FnOnce(&mut SkiaCanvas<'_>),
        ) -> Option<bool> {
            let list: Vec<aylis_core::RectF> = area.into_iter().collect();
            let parts = area.map(|_| (&[][..], &list[..]));
            self.render_parts(renderer, width, height, scale, background, parts, |c, _| paint(c))
        }

        /// Como [`render_area`](Self::render_area), com as cópias da rolagem (`moves`, feitas
        /// na imagem guardada) e várias áreas pintadas de uma vez (`paint` recebe as áreas;
        /// `None` é a janela inteira).
        #[allow(clippy::type_complexity)]
        pub fn render_parts(
            &mut self,
            renderer: &SkiaRenderer,
            width: u32,
            height: u32,
            scale: f32,
            background: Color,
            parts: Option<(&[aylis_core::ScrollMove], &[aylis_core::RectF])>,
            paint: impl FnOnce(&mut SkiaCanvas<'_>, Option<&[aylis_core::RectF]>),
        ) -> Option<bool> {
            let size = (width as i32, height as i32);
            if self.target.as_ref().is_none_or(|(_, s)| *s != size) {
                let info = gpu::gl::FramebufferInfo {
                    fboid: 0,
                    format: gpu::gl::Format::RGBA8.into(),
                    ..Default::default()
                };
                let target = gpu::backend_render_targets::make_gl(size, self.samples, self.stencil, info);
                self.target = gpu::surfaces::wrap_backend_render_target(
                    &mut self.context,
                    &target,
                    gpu::SurfaceOrigin::BottomLeft,
                    sk::ColorType::RGBA8888,
                    None,
                    None,
                )
                .map(|surface| (surface, size));
            }
            self.target.as_ref()?;
            // A imagem guardada, do tamanho da janela; nova (ou outro tamanho), o quadro é inteiro.
            let mut parts = parts;
            if self.layer.as_ref().is_none_or(|(_, s)| *s != size) {
                let info = sk::ImageInfo::new(size, sk::ColorType::RGBA8888, sk::AlphaType::Premul, None);
                self.layer = gpu::surfaces::render_target(&mut self.context, gpu::Budgeted::Yes, &info, None, gpu::SurfaceOrigin::TopLeft, None, false, None)
                    .map(|surface| (surface, size));
                parts = None;
            }
            let id = &self.context as *const _ as usize;
            if self.loader != 0 {
                super::GL_FRAME.with(|g| g.set(Some((self.loader, id))));
            }
            let (target, _) = self.target.as_mut()?;
            let partial = match self.layer.as_mut() {
                Some((layer, _)) => {
                    match parts {
                        Some((moves, areas)) => {
                            crate::apply_moves(layer, scale, moves);
                            renderer.draw_areas(layer.canvas(), scale, background, areas, |c, a| paint(c, Some(a)));
                        }
                        None => renderer.draw_area(layer.canvas(), scale, background, None, |c| paint(c, None)),
                    }
                    layer.draw(target.canvas(), (0, 0), sk::SamplingOptions::default(), None);
                    parts.is_some()
                }
                // Sem a imagem guardada (a GPU não deu): direto na janela, inteira.
                None => {
                    renderer.draw(target.canvas(), scale, background, |c| paint(c, None));
                    false
                }
            };
            super::GL_FRAME.with(|g| g.set(None));
            // O que o 3D gravou durante a pintura vai antes de o Skia executar os desenhos dele.
            if aylis_core::gpu::run_before_flush() {
                self.context.reset(None);
            }
            self.context.flush_and_submit();
            Some(partial)
        }

        /// Pixels RGBA do último quadro pintado na janela (antes do `swap_buffers`). Serve
        /// para testes: é exatamente o que vai para a tela.
        pub fn read_frame(&mut self) -> Option<Vec<u8>> {
            let (surface, (width, height)) = self.target.as_mut()?;
            let (width, height) = (*width as u32, *height as u32);
            crate::read_rgba(surface, width, height)
        }

        /// Pinta numa textura fora da tela e devolve os pixels RGBA (para comparar com a CPU).
        pub fn render_rgba(
            &mut self,
            renderer: &SkiaRenderer,
            width: u32,
            height: u32,
            scale: f32,
            background: Color,
            paint: impl FnOnce(&mut SkiaCanvas<'_>),
        ) -> Option<Vec<u8>> {
            let info = crate::rgba_info(width, height);
            let mut surface = gpu::surfaces::render_target(
                &mut self.context,
                gpu::Budgeted::Yes,
                &info,
                None,
                gpu::SurfaceOrigin::TopLeft,
                None,
                false,
                None,
            )?;
            if self.loader != 0 {
                super::GL_FRAME.with(|g| g.set(Some((self.loader, &self.context as *const _ as usize))));
            }
            renderer.draw(surface.canvas(), scale, background, paint);
            super::GL_FRAME.with(|g| g.set(None));
            if aylis_core::gpu::run_before_flush() {
                self.context.reset(None);
            }
            self.context.flush_and_submit();
            crate::read_rgba(&mut surface, width, height)
        }

        /// Libera recursos da GPU quando a superfície foi perdida (mobile, troca de driver).
        pub fn abandon(&mut self) {
            self.target = None;
            self.context.abandon();
        }
    }
}

// ---------------------------------------------------------------------------
// conversões
// ---------------------------------------------------------------------------

fn to_sk(c: Color) -> sk::Color {
    sk::Color::from_argb(c.a, c.r, c.g, c.b)
}

fn to_rect(r: RectF) -> sk::Rect {
    sk::Rect::from_xywh(r.x, r.y, r.width, r.height)
}

fn fill_paint(brush: &Brush) -> sk::Paint {
    let mut paint = sk::Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(sk::PaintStyle::Fill);
    apply_brush(&mut paint, brush);
    paint
}

fn stroke_paint(stroke: &Stroke) -> sk::Paint {
    let mut paint = sk::Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(sk::PaintStyle::Stroke);
    paint.set_stroke_width(stroke.width);
    apply_brush(&mut paint, &stroke.brush);
    if let Some(dash) = &stroke.dash {
        paint.set_path_effect(sk::PathEffect::dash(dash, 0.0));
    }
    paint.set_stroke_cap(match stroke.cap {
        LineCap::Butt => sk::paint::Cap::Butt,
        LineCap::Round => sk::paint::Cap::Round,
        LineCap::Square => sk::paint::Cap::Square,
    });
    paint.set_stroke_join(match stroke.join {
        LineJoin::Miter => sk::paint::Join::Miter,
        LineJoin::Round => sk::paint::Join::Round,
        LineJoin::Bevel => sk::paint::Join::Bevel,
    });
    paint
}

fn to_path(path: &aylis_core::Path) -> sk::Path {
    use aylis_core::PathCmd;
    let mut b = sk::PathBuilder::new();
    for c in &path.cmds {
        match *c {
            PathCmd::MoveTo(p) => {
                b.move_to((p.x, p.y));
            }
            PathCmd::LineTo(p) => {
                b.line_to((p.x, p.y));
            }
            PathCmd::QuadTo(c, p) => {
                b.quad_to((c.x, c.y), (p.x, p.y));
            }
            PathCmd::CubicTo(c1, c2, p) => {
                b.cubic_to((c1.x, c1.y), (c2.x, c2.y), (p.x, p.y));
            }
            PathCmd::Close => {
                b.close();
            }
        }
    }
    let mut p = b.detach();
    if path.even_odd {
        p.set_fill_type(sk::PathFillType::EvenOdd);
    }
    p
}

fn apply_brush(paint: &mut sk::Paint, brush: &Brush) {
    use sk::gradient::{Colors, Gradient, Interpolation, shaders};

    let stops = match brush {
        Brush::Solid(c) => {
            paint.set_color(to_sk(*c));
            return;
        }
        Brush::Linear { stops, .. } | Brush::Radial { stops, .. } | Brush::Sweep { stops, .. } => stops,
    };
    let colors: Vec<sk::Color4f> = stops.iter().map(|s| sk::Color4f::from(to_sk(s.color))).collect();
    let pos: Vec<f32> = stops.iter().map(|s| s.offset).collect();
    let gradient = Gradient::new(
        Colors::new(&colors, Some(&pos), sk::TileMode::Clamp, None),
        Interpolation::default(),
    );
    let shader = match brush {
        Brush::Linear { start, end, .. } => {
            shaders::linear_gradient(((start.x, start.y), (end.x, end.y)), &gradient, None)
        }
        Brush::Radial { center, radius, .. } => {
            shaders::radial_gradient(((center.x, center.y), *radius), &gradient, None)
        }
        Brush::Sweep { center, start_angle, .. } => {
            // Gira o gradiente em volta do centro para começar em `start_angle`.
            let matrix = sk::Matrix::rotate_deg_pivot(*start_angle, (center.x, center.y));
            shaders::sweep_gradient((center.x, center.y), (0.0, 360.0), &gradient, &matrix)
        }
        Brush::Solid(_) => unreachable!("tratado acima"),
    };
    paint.set_shader(shader);
}

fn build_paragraph(fonts: &FontCollection, text: &str, style: &TextStyle) -> Paragraph {
    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_text_align(match style.h_align {
        TextAlign::Leading => sk::textlayout::TextAlign::Left,
        TextAlign::Justify => sk::textlayout::TextAlign::Justify,
        TextAlign::Center => sk::textlayout::TextAlign::Center,
        TextAlign::Trailing => sk::textlayout::TextAlign::Right,
    });
    if !style.wrap {
        paragraph_style.set_max_lines(1);
        paragraph_style.set_ellipsis("…");
    }

    let mut text_style = sk::textlayout::TextStyle::new();
    text_style.set_color(to_sk(style.color));
    text_style.set_font_size(style.font.size);
    text_style.set_font_families(&font_families(&style.font));
    text_style.set_font_style(font_style(&style.font));
    // Sublinhado e tachado (o `Font.Style` do Delphi).
    let mut decoration = sk::textlayout::TextDecoration::NO_DECORATION;
    if style.font.underline {
        decoration |= sk::textlayout::TextDecoration::UNDERLINE;
    }
    if style.font.strikeout {
        decoration |= sk::textlayout::TextDecoration::LINE_THROUGH;
    }
    if decoration != sk::textlayout::TextDecoration::NO_DECORATION {
        text_style.set_decoration_type(decoration);
        text_style.set_decoration_color(to_sk(style.color));
    }

    let mut builder = ParagraphBuilder::new(&paragraph_style, fonts);
    builder.push_style(&text_style);
    builder.add_text(text);
    builder.build()
}

/// Família pedida, ou uma lista de fontes comuns em cada sistema para os nomes genéricos.
fn font_families(font: &FontSpec) -> Vec<&str> {
    match font.family.as_str() {
        "" | "sans-serif" => vec!["Segoe UI", "SF Pro Text", "Helvetica Neue", "Roboto", "Noto Sans", "DejaVu Sans", "Arial"],
        "monospace" => vec!["Cascadia Mono", "Consolas", "Menlo", "DejaVu Sans Mono", "Courier New"],
        family => vec![family],
    }
}

fn font_style(font: &FontSpec) -> sk::FontStyle {
    let weight = match font.weight {
        FontWeight::Thin => 100,
        FontWeight::Light => 300,
        FontWeight::Normal => 400,
        FontWeight::Medium => 500,
        FontWeight::SemiBold => 600,
        FontWeight::Bold => 700,
        FontWeight::Black => 900,
    };
    let slant = if font.italic { sk::font_style::Slant::Italic } else { sk::font_style::Slant::Upright };
    sk::FontStyle::new(weight.into(), sk::font_style::Width::NORMAL, slant)
}

/// As famílias de fonte instaladas no sistema, em ordem alfabética (o editor de fonte da IDE).
pub fn system_font_families() -> Vec<String> {
    let mut families: Vec<String> = sk::FontMgr::new().family_names().filter(|f| !f.starts_with('@')).collect();
    families.sort_by_key(|f| f.to_lowercase());
    families.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    families
}

/// Desenha `image` com o canto em `at`, em pixels da tela (sem a transformação atual; o
/// recorte vale).
fn draw_at_device(canvas: &sk::Canvas, image: &sk::Image, at: (f32, f32)) {
    canvas.save();
    canvas.reset_matrix();
    canvas.draw_image(image, at, None);
    canvas.restore();
}

/// Copia, na própria superfície, os pixels que andaram (`moves`, em unidades lógicas; a
/// região e o deslocamento caem em pixels inteiros, veja `FrameDamage::snapped`): a rolagem
/// sem repintar o que já estava na tela. Vale na CPU e na GPU.
pub fn apply_moves(surface: &mut sk::Surface, scale: f32, moves: &[aylis_core::ScrollMove]) {
    for m in moves {
        let r = m.rect;
        let px = |v: f32| (v * scale).round() as i32;
        let src = sk::IRect::from_ltrb(px(r.x), px(r.y), px(r.right()), px(r.bottom()));
        let Some(image) = surface.image_snapshot_with_bounds(src) else { continue };
        let at = ((src.left + px(m.delta.x)) as f32, (src.top + px(m.delta.y)) as f32);
        let canvas = surface.canvas();
        canvas.save();
        canvas.reset_matrix();
        let mut paint = sk::Paint::default();
        paint.set_blend_mode(sk::BlendMode::Src);
        canvas.draw_image(&image, at, Some(&paint));
        canvas.restore();
    }
}

/// Zera o byte alto dos pixels (alguns backends do softbuffer exigem) só em `areas` (unidades
/// lógicas); sem áreas, em todos.
fn clear_alpha(pixels: &mut [u32], width: u32, height: u32, scale: f32, areas: &[aylis_core::RectF]) {
    if areas.is_empty() {
        for p in pixels.iter_mut() {
            *p &= 0x00FF_FFFF;
        }
        return;
    }
    let (w, h) = (width as usize, height as usize);
    for a in areas {
        let l = ((a.x * scale).floor().max(0.0) as usize).min(w);
        let r = ((a.right() * scale).ceil().max(0.0) as usize).min(w);
        let t = ((a.y * scale).floor().max(0.0) as usize).min(h);
        let b = ((a.bottom() * scale).ceil().max(0.0) as usize).min(h);
        for y in t..b {
            for p in &mut pixels[y * w + l..y * w + r] {
                *p &= 0x00FF_FFFF;
            }
        }
    }
}
