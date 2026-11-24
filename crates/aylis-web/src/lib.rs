//! # aylis-web
//!
//! Renderizador do aylis para navegadores: implementa o [`Canvas`] e o [`TextService`]
//! sobre o Canvas 2D do HTML.
//!
//! O Skia nativo (`skia-safe`) não compila para `wasm32-unknown-unknown`. No Chrome, no
//! Edge e no Firefox o próprio Canvas 2D é desenhado pelo Skia (no Safari, pelo Core
//! Graphics), então o resultado fica muito próximo do desktop sem baixar nada extra.
//!
//! Diferenças conhecidas em relação ao `aylis-skia`:
//! - `save_layer` aplica a opacidade a cada desenho (não compõe a camada inteira de uma
//!   vez), o que só muda o resultado quando filhos semitransparentes se sobrepõem;
//! - as fontes são as do navegador;
//! - imagens codificadas (PNG, JPEG, WebP, SVG) são decodificadas pelo navegador, de
//!   forma assíncrona: aparecem na repintura seguinte ao carregamento ([`on_image_loaded`]).

use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;

use aylis_core::{
    Brush, Canvas, Color, FontSpec, FontWeight, GradientStop, Image, ImageSource, PointF, RectF,
    SizeF, Stroke, TextAlign, TextService, TextStyle,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasGradient, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData};

// ---------------------------------------------------------------------------
// Imagens

/// Uma imagem pronta para o `drawImage` (ou ainda carregando).
#[derive(Clone)]
enum WebImage {
    /// O navegador está decodificando (o `<img>` avisa quando terminar).
    Loading,
    Ready(HtmlImageElement),
    /// Pixels (RGBA/BGRA) copiados para um canvas fora da tela.
    Pixels(HtmlCanvasElement),
    Failed,
}

/// As imagens prontas, nos limites do projeto: as de arquivo pela quantidade (`files`), as
/// de pixels pelo tamanho (`pixels_mb`).
struct ImageCache {
    files: aylis_core::ImageCache<WebImage>,
    pixels: aylis_core::ImageCache<WebImage>,
    on_loaded: Option<std::rc::Rc<dyn Fn()>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        Self {
            files: aylis_core::ImageCache::named("Navegador · arquivos"),
            pixels: aylis_core::ImageCache::named("Navegador · pixels"),
            on_loaded: None,
        }
    }
}

impl ImageCache {
    /// A imagem pronta (marcada como usada), se já estiver guardada.
    fn get(&mut self, image: &Image) -> Option<WebImage> {
        match image.source() {
            ImageSource::Encoded(_) => self.files.get(image).cloned(),
            _ => self.pixels.get(image).cloned(),
        }
    }

    fn insert(&mut self, image: &Image, loaded: WebImage) {
        let limits = aylis_core::image_cache::limits();
        match image.source() {
            ImageSource::Encoded(_) => self.files.insert(image, loaded, 0, limits.files as usize, usize::MAX),
            ImageSource::Rgba { width, height, .. } | ImageSource::Bgra { width, height, .. } => {
                self.pixels.insert(image, loaded, *width as usize * *height as usize * 4, usize::MAX, limits.pixels_bytes())
            }
        }
    }
}

thread_local! {
    static IMAGES: RefCell<ImageCache> = RefCell::new(ImageCache::default());
}

/// Chamado quando uma imagem termina de carregar (a plataforma pede uma repintura).
pub fn on_image_loaded(f: impl Fn() + 'static) {
    IMAGES.with(|c| c.borrow_mut().on_loaded = Some(std::rc::Rc::new(f)));
}

thread_local! {
    static FONT_GENERATION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Fontes de arquivo registradas (`aylis_core::fonts`) viram `FontFace` no documento. Elas
/// carregam de forma assíncrona: quando terminam, a plataforma repinta (o mesmo aviso das
/// imagens, [`on_image_loaded`]).
fn sync_fonts() {
    let generation = aylis_core::fonts::generation();
    if FONT_GENERATION.with(|g| g.replace(generation)) == generation {
        return;
    }
    forget_widths();
    let add = || -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("sem window")?;
        let document = window.document().ok_or("sem document")?;
        let set = js_sys::Reflect::get(&document, &"fonts".into())?;
        let add: js_sys::Function = js_sys::Reflect::get(&set, &"add".into())?.dyn_into()?;
        let ctor: js_sys::Function = js_sys::Reflect::get(&window, &"FontFace".into())?.dyn_into()?;
        for font in aylis_core::fonts::registered() {
            let data = js_sys::Uint8Array::from(&font.data[..]);
            let mut faces = vec![js_sys::Array::of2(&font.family.as_str().into(), &data.buffer())];
            // A fonte de emoji sem cor (`color_glyphs: false`) vale também com outro nome, só nos
            // trechos de emoji (`unicode-range`): na lista de fontes ela vem antes da pedida, e o
            // resto do texto (dígitos, `#`) passa direto para a pedida.
            if font.family == aylis_core::canvas::MONO_EMOJI_FAMILY {
                let descriptors = js_sys::Object::new();
                js_sys::Reflect::set(&descriptors, &"unicodeRange".into(), &mono_emoji_unicode_range().into())?;
                faces.push(js_sys::Array::of3(&MONO_EMOJI_FACE.into(), &data.buffer(), &descriptors));
            }
            for args in faces {
                let face = js_sys::Reflect::construct(&ctor, &args)?;
                add.call1(&set, &face)?;
                let load: js_sys::Function = js_sys::Reflect::get(&face, &"load".into())?.dyn_into()?;
                let promise: js_sys::Promise = load.call0(&face)?.dyn_into()?;
                let done = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
                    // A fonte chegou: as larguras medidas com a substituta não valem mais.
                    forget_widths();
                    if let Some(f) = IMAGES.with(|c| c.borrow().on_loaded.clone()) {
                        f();
                    }
                });
                let _ = promise.then(&done);
                done.forget();
            }
        }
        Ok(())
    };
    let _ = add();
}

/// `color_glyphs: false`: o texto sem o U+FE0F (o pedido de emoji colorido). Com ele, o
/// navegador escolhe uma fonte colorida para o emoji mesmo com a sem cor antes na lista; o
/// Direct2D e o Skia já mandam o trecho para a fonte sem cor.
fn plain_emoji<'t>(text: &'t str, style: &TextStyle) -> std::borrow::Cow<'t, str> {
    if style.color_glyphs || !text.contains('\u{FE0F}') {
        return text.into();
    }
    text.replace('\u{FE0F}', "").into()
}

/// O nome da Noto Emoji só nos trechos de emoji (veja [`sync_fonts`]).
const MONO_EMOJI_FACE: &str = "Aylis Mono Emoji";

/// O `unicode-range` dos trechos de emoji (`aylis_core::canvas::MONO_EMOJI_RANGES`, os mesmos
/// do Direct2D e do Skia).
fn mono_emoji_unicode_range() -> String {
    aylis_core::canvas::MONO_EMOJI_RANGES
        .iter()
        .map(|&(a, b)| if a == b { format!("U+{a:X}") } else { format!("U+{a:X}-{b:X}") })
        .collect::<Vec<_>>()
        .join(", ")
}

thread_local! {
    /// Se o Canvas 2D tem `letterSpacing` (o Chrome desde a 99, o Firefox desde a 115, o
    /// Safari desde a 18.4): lido uma vez.
    static HAS_LETTER_SPACING: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

fn has_letter_spacing() -> bool {
    HAS_LETTER_SPACING.with(|cell| {
        if let Some(v) = cell.get() {
            return v;
        }
        let global = js_sys::global();
        let proto = js_sys::Reflect::get(&global, &"CanvasRenderingContext2D".into()).ok().and_then(|c| js_sys::Reflect::get(&c, &"prototype".into()).ok());
        let v = proto.is_some_and(|p| js_sys::Reflect::has(&p, &"letterSpacing".into()).unwrap_or(false));
        cell.set(Some(v));
        v
    })
}

/// Põe o `letterSpacing` do contexto (em pixels lógicos; `0` volta ao normal).
fn set_letter_spacing(ctx: &CanvasRenderingContext2d, spacing: f32) {
    let _ = js_sys::Reflect::set(ctx, &"letterSpacing".into(), &format!("{spacing}px").into());
}

/// Desenha uma linha com a ponta esquerda em `x` e o meio em `y`, com o espaço entre letras:
/// o `letterSpacing` do canvas, ou letra a letra no navegador que não o tem.
fn fill_spaced(ctx: &CanvasRenderingContext2d, font: &str, spacing: f32, line: &str, x: f32, y: f32) {
    if spacing == 0.0 {
        let _ = ctx.fill_text(line, x as f64, y as f64);
    } else if has_letter_spacing() {
        set_letter_spacing(ctx, spacing);
        let _ = ctx.fill_text(line, x as f64, y as f64);
        set_letter_spacing(ctx, 0.0);
    } else {
        let mut at = x;
        let mut buf = [0u8; 4];
        for c in line.chars() {
            let s = c.encode_utf8(&mut buf);
            let _ = ctx.fill_text(s, at as f64, y as f64);
            at += text_width(ctx, font, 0.0, s) + spacing;
        }
    }
}

/// Começa a decodificar uma imagem codificada (um `<img>` com um blob).
fn load_encoded(id: u64, bytes: &[u8]) -> WebImage {
    let start = || -> Result<(), JsValue> {
        let array = js_sys::Uint8Array::from(bytes);
        let parts = js_sys::Array::of1(&array);
        let blob = web_sys::Blob::new_with_u8_array_sequence(&parts)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        let img = HtmlImageElement::new()?;
        let done_img = img.clone();
        let done_url = url.clone();
        let onload = Closure::once_into_js(move || {
            let _ = web_sys::Url::revoke_object_url(&done_url);
            let notify = IMAGES.with(|c| {
                let mut cache = c.borrow_mut();
                if let Some(slot) = cache.files.get_by_id_mut(id) {
                    *slot = WebImage::Ready(done_img);
                }
                cache.on_loaded.clone()
            });
            // Fora do empréstimo: a repintura pode desenhar imagens na hora.
            if let Some(f) = notify {
                f();
            }
        });
        let onerror = Closure::once_into_js(move || {
            IMAGES.with(|c| {
                if let Some(slot) = c.borrow_mut().files.get_by_id_mut(id) {
                    *slot = WebImage::Failed;
                }
            });
        });
        img.set_onload(Some(onload.unchecked_ref()));
        img.set_onerror(Some(onerror.unchecked_ref()));
        img.set_src(&url);
        Ok(())
    };
    if start().is_ok() { WebImage::Loading } else { WebImage::Failed }
}

/// Copia pixels para um canvas fora da tela (o `drawImage` aceita canvas).
fn load_pixels(width: u32, height: u32, rgba: &[u8], bgra: bool) -> WebImage {
    let make = || -> Result<HtmlCanvasElement, JsValue> {
        let document = web_sys::window().and_then(|w| w.document()).ok_or("sem document")?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        canvas.set_width(width);
        canvas.set_height(height);
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d")?.ok_or("sem 2d")?.dyn_into()?;
        let mut data = rgba.to_vec();
        if bgra {
            // O BGRA vem pré-multiplicado (o contrato do tipo); o `ImageData` quer sem.
            for px in data.chunks_exact_mut(4) {
                px.swap(0, 2);
                let a = px[3] as u32;
                if a > 0 && a < 255 {
                    for c in &mut px[..3] {
                        *c = ((*c as u32 * 255 + a / 2) / a).min(255) as u8;
                    }
                }
            }
        }
        let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
        ctx.put_image_data(&image, 0.0, 0.0)?;
        Ok(canvas)
    };
    make().map_or(WebImage::Failed, WebImage::Pixels)
}

/// Garante a imagem no cache e roda `f` com ela.
fn with_image<R>(image: &Image, f: impl FnOnce(&WebImage) -> R) -> R {
    let ready = IMAGES.with(|c| {
        if let Some(found) = c.borrow_mut().get(image) {
            return found;
        }
        let loaded = match image.source() {
            ImageSource::Encoded(bytes) => load_encoded(image.id(), bytes),
            ImageSource::Rgba { width, height, pixels } => load_pixels(*width, *height, pixels, false),
            ImageSource::Bgra { width, height, pixels } => load_pixels(*width, *height, pixels, true),
        };
        c.borrow_mut().insert(image, loaded.clone());
        loaded
    });
    // Fora do empréstimo do cache (o desenho pode pedir outra imagem).
    f(&ready)
}

thread_local! {
    /// Os `<canvas>` de quem desenha por fora (o 3D), pelo número.
    static GPU_CANVASES: RefCell<HashMap<u32, HtmlCanvasElement>> = RefCell::new(HashMap::new());
}

/// O `<canvas id="aylis-3d-<n>">` da página (guardado depois da primeira busca).
fn gpu_canvas(n: u32) -> Option<HtmlCanvasElement> {
    if let Some(c) = GPU_CANVASES.with(|m| m.borrow().get(&n).cloned()) {
        return Some(c);
    }
    let document = web_sys::window()?.document()?;
    let canvas: HtmlCanvasElement = document.get_element_by_id(&format!("aylis-3d-{n}"))?.dyn_into().ok()?;
    GPU_CANVASES.with(|m| m.borrow_mut().insert(n, canvas.clone()));
    Some(canvas)
}

thread_local! {
    /// Se o Canvas 2D tem `filter` (o Safari não tem): lido uma vez.
    static HAS_FILTER: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    /// Canvas fora da tela do desfoque sem `filter`.
    static BLUR_SCRATCH: RefCell<Option<(HtmlCanvasElement, CanvasRenderingContext2d)>> = const { RefCell::new(None) };
}

/// O Canvas 2D aceita `filter: blur()`? Diagnóstico: `?aylis_blur=reduzir` na URL força o
/// caminho sem filtro (o do Safari) em qualquer navegador.
fn has_canvas_filter() -> bool {
    HAS_FILTER.with(|cell| {
        if let Some(v) = cell.get() {
            return v;
        }
        let global = js_sys::global();
        let get = |o: &JsValue, k: &str| js_sys::Reflect::get(o, &k.into()).ok();
        let forced = get(&global, "location").and_then(|l| get(&l, "search")).and_then(|s| s.as_string()).is_some_and(|q| q.contains("aylis_blur=reduzir"));
        let proto = get(&global, "CanvasRenderingContext2D").and_then(|c| get(&c, "prototype"));
        let v = !forced && proto.is_some_and(|p| js_sys::Reflect::has(&p, &"filter".into()).unwrap_or(false));
        cell.set(Some(v));
        v
    })
}

/// Desfoque sem `filter`: a região é reduzida em dois passos e ampliada de volta com
/// suavização, o que fica perto de um gaussiano de desvio `sigma` (em pixels).
fn blur_by_scaling(ctx: &CanvasRenderingContext2d, source: &HtmlCanvasElement, [x, y, w, h]: [f64; 4], sigma: f64) {
    // Reduzir por k e ampliar com interpolação linear espalha cerca de k/2 de desvio; o
    // passo intermediário (k/2) tira os "degraus" da ampliação direta.
    let k = (sigma * 1.5).max(2.0);
    let (sw, sh) = ((w / k).ceil().max(1.0), (h / k).ceil().max(1.0));
    let (mw, mh) = ((sw * 2.0).min(w).max(1.0), (sh * 2.0).min(h).max(1.0));
    BLUR_SCRATCH.with(|cell| {
        let mut cell = cell.borrow_mut();
        if cell.is_none() {
            let canvas = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.create_element("canvas").ok())
                .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok());
            let ctx = canvas.as_ref().and_then(|c| c.get_context("2d").ok().flatten()).and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok());
            if let (Some(canvas), Some(ctx)) = (canvas, ctx) {
                *cell = Some((canvas, ctx));
            }
        }
        let Some((scratch, sctx)) = cell.as_ref() else { return };
        let (cw, ch) = ((sw + mw).ceil() as u32, mh.ceil() as u32);
        if scratch.width() < cw || scratch.height() < ch {
            scratch.set_width(cw.max(scratch.width()));
            scratch.set_height(ch.max(scratch.height()));
        }
        let _ = js_sys::Reflect::set(sctx, &"imageSmoothingQuality".into(), &"high".into());
        let _ = js_sys::Reflect::set(ctx, &"imageSmoothingQuality".into(), &"high".into());
        sctx.set_image_smoothing_enabled(true);
        ctx.set_image_smoothing_enabled(true);
        sctx.clear_rect(0.0, 0.0, cw as f64, ch as f64);
        // Fonte → meio (mw×mh, em x = sw) → pequeno (sw×sh, em x = 0) → meio → destino.
        let _ = sctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(source, x, y, w, h, sw, 0.0, mw, mh);
        let _ = sctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(scratch, sw, 0.0, mw, mh, 0.0, 0.0, sw, sh);
        let _ = sctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(scratch, 0.0, 0.0, sw, sh, sw, 0.0, mw, mh);
        let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(scratch, sw, 0.0, mw, mh, x, y, w, h);
    });
}

/// Canvas do aylis sobre um contexto 2D.
pub struct Canvas2d<'a> {
    ctx: CanvasRenderingContext2d,
    /// Os `save` abertos por este canvas: um `restore` a mais (o Skia ignora) não pode
    /// fechar o `save` de quem o criou, que guarda o recorte da área repintada.
    depth: usize,
    /// Os `begin_cached` abertos (`None`: não deu para guardar, desenha direto).
    cached: Vec<Option<WebCache>>,
    /// As camadas de `save_layer` abertas (a opacidade de um contêiner com os filhos).
    layers: Vec<WebLayer>,
    /// As máscaras abertas (`begin_masked`).
    masks: Vec<WebMask>,
    _life: std::marker::PhantomData<&'a CanvasRenderingContext2d>,
}

/// Uma máscara aberta ([`Canvas::begin_masked`]): o conteúdo e depois a máscara num canvas à
/// parte cada um, do tamanho do de baixo; no `end_mask`, a máscara recorta o conteúdo
/// (`destination-in`) e ele vai para o canvas de baixo.
struct WebMask {
    main: CanvasRenderingContext2d,
    depth: usize,
    content: HtmlCanvasElement,
    content_ctx: CanvasRenderingContext2d,
    mask: Option<HtmlCanvasElement>,
    inverse: bool,
    /// A máscara é de luminância (o `<mask>` do SVG): o brilho vira o alfa antes de recortar.
    luminance: bool,
}

/// Uma camada de `save_layer`: o grupo é desenhado num canvas à parte, do tamanho do de
/// baixo, e composto uma vez só com a opacidade no `restore`. Com o `globalAlpha` em cada
/// desenho, o que os filhos cobrem uns dos outros aparecia por baixo (o fundo do painel através
/// do botão), diferente do Skia e do Direct2D (o relatório do Tabernáculo, C24).
struct WebLayer {
    main: CanvasRenderingContext2d,
    /// Os `save` de `main` abertos antes da camada.
    depth: usize,
    /// Quantos `begin_cached` estavam abertos: o `restore` só fecha a camada no mesmo nível.
    cached: usize,
    canvas: HtmlCanvasElement,
    opacity: f64,
    /// Como a camada se junta ao canvas de baixo (o `globalCompositeOperation`).
    blend: aylis_core::BlendMode,
}

/// Um desenho sendo guardado: o contexto de antes e a imagem onde ele vai.
struct WebCache {
    main: CanvasRenderingContext2d,
    depth: usize,
    canvas: HtmlCanvasElement,
    key: u64,
    /// A chave do núcleo (para soltar pela `forget_cached`).
    owner: u64,
    at: (f64, f64),
}

thread_local! {
    /// Os desenhos guardados (`Canvas::begin_cached`: o `cache_layer` de um controle, um
    /// mapa parado), pela chave.
    static LAYERS: RefCell<aylis_core::LayerCache<HtmlCanvasElement>> = RefCell::new(aylis_core::LayerCache::named("Canvas 2D · camadas guardadas"));
    /// Os canvas das camadas de `save_layer` já usados, para o próximo quadro.
    static SPARE_LAYERS: RefCell<Vec<HtmlCanvasElement>> = const { RefCell::new(Vec::new()) };
    /// As sombras prontas ([`shadow_tile`]), pelo raio, desfoque e cor em pixels.
    static SHADOW_TILES: RefCell<std::collections::HashMap<(u32, u32, u32), ShadowTile>> = RefCell::new(std::collections::HashMap::new());
}

/// Quantas sombras prontas ficam guardadas (cada combinação de raio, desfoque e cor; as de
/// uma tela são poucas).
const SHADOW_TILES_MAX: usize = 64;

/// A sombra de um retângulo arredondado pronta, para montar em nove pedaços (os cantos
/// inteiros, as bordas e o meio esticados, como o `box-shadow` dos navegadores): o
/// `shadowBlur` refaz o desfoque a cada desenho, e na GPU de um celular as sombras dos cards
/// seguravam a rolagem em 12 quadros por segundo.
#[derive(Clone)]
struct ShadowTile {
    canvas: HtmlCanvasElement,
    /// A borda em volta da forma (até onde o desfoque vai) e o canto (o raio mais o
    /// desfoque para dentro), em pixels: o pedaço de canto tem `pad + corner`.
    pad: f64,
    corner: f64,
}

/// A sombra pronta para o raio `radius`, o desfoque `blur` (o `shadowBlur`) e a cor, tudo em
/// pixels da tela.
fn shadow_tile(radius: f64, blur: f64, color: Color) -> Option<ShadowTile> {
    let key = ((radius * 4.0).round() as u32, (blur * 4.0).round() as u32, u32::from_be_bytes([color.r, color.g, color.b, color.a]));
    if let Some(tile) = SHADOW_TILES.with(|t| t.borrow().get(&key).cloned()) {
        return Some(tile);
    }
    // O desfoque do canvas é gaussiano com sigma = blur / 2: some em uns 3 sigmas.
    let pad = (blur * 1.5).ceil() + 2.0;
    let corner = (radius + blur * 1.5).ceil() + 1.0;
    let shape = 2.0 * corner + 1.0;
    let size = (shape + 2.0 * pad) as u32;
    let canvas = web_sys::window()?.document()?.create_element("canvas").ok()?.dyn_into::<HtmlCanvasElement>().ok()?;
    canvas.set_width(size);
    canvas.set_height(size);
    let ctx = canvas.get_context("2d").ok().flatten()?.dyn_into::<CanvasRenderingContext2d>().ok()?;
    // A forma fica longe e só a sombra, deslocada de volta, cai no canvas.
    const FAR: f64 = 100_000.0;
    ctx.set_shadow_color(&css_color(color));
    ctx.set_shadow_blur(blur);
    ctx.set_shadow_offset_x(FAR);
    ctx.set_fill_style_str("#000");
    let (x, y, r) = (pad - FAR, pad, radius.min(shape / 2.0));
    ctx.begin_path();
    if r > 0.0 {
        ctx.move_to(x + r, y);
        let _ = ctx.arc_to(x + shape, y, x + shape, y + shape, r);
        let _ = ctx.arc_to(x + shape, y + shape, x, y + shape, r);
        let _ = ctx.arc_to(x, y + shape, x, y, r);
        let _ = ctx.arc_to(x, y, x + shape, y, r);
        ctx.close_path();
    } else {
        ctx.rect(x, y, shape, shape);
    }
    ctx.fill();
    let tile = ShadowTile { canvas, pad, corner };
    SHADOW_TILES.with(|t| {
        let mut t = t.borrow_mut();
        if t.len() >= SHADOW_TILES_MAX {
            t.clear();
        }
        t.insert(key, tile.clone());
    });
    Some(tile)
}

/// Um canvas `width` × `height` limpo, para uma camada (um guardado, se houver).
fn layer_canvas(width: u32, height: u32) -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let canvas = match SPARE_LAYERS.with(|s| s.borrow_mut().pop()) {
        Some(c) => c,
        None => web_sys::window()?.document()?.create_element("canvas").ok()?.dyn_into::<HtmlCanvasElement>().ok()?,
    };
    if canvas.width() != width || canvas.height() != height {
        canvas.set_width(width);
        canvas.set_height(height);
    }
    let ctx = canvas.get_context("2d").ok().flatten()?.dyn_into::<CanvasRenderingContext2d>().ok()?;
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    ctx.set_global_alpha(1.0);
    ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
    Some((canvas, ctx))
}

/// Desenha `canvas` com o canto em `at`, em pixels (sem a transformação atual; o recorte vale).
fn draw_at_device(ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement, at: (f64, f64)) {
    ctx.save();
    let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let _ = ctx.draw_image_with_html_canvas_element(canvas, at.0, at.1);
    ctx.restore();
}

impl<'a> Canvas2d<'a> {
    pub fn new(ctx: &'a CanvasRenderingContext2d) -> Self {
        Self { ctx: ctx.clone(), depth: 0, cached: Vec::new(), layers: Vec::new(), masks: Vec::new(), _life: std::marker::PhantomData }
    }

    /// O contexto 2D por baixo, para desenhos que o trait não cobre.
    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }

    fn rect_path(&self, r: RectF, radius: f32) {
        let ctx = &self.ctx.clone();
        ctx.begin_path();
        let (x, y, w, h) = (r.x as f64, r.y as f64, r.width as f64, r.height as f64);
        let rad = (radius as f64).min(w / 2.0).min(h / 2.0).max(0.0);
        if rad <= 0.0 {
            ctx.rect(x, y, w, h);
            return;
        }
        ctx.move_to(x + rad, y);
        let _ = ctx.arc_to(x + w, y, x + w, y + h, rad);
        let _ = ctx.arc_to(x + w, y + h, x, y + h, rad);
        let _ = ctx.arc_to(x, y + h, x, y, rad);
        let _ = ctx.arc_to(x, y, x + w, y, rad);
        ctx.close_path();
    }

    /// A sombra montada da pronta ([`shadow_tile`]), em nove pedaços. `false` quando não dá
    /// (giro, escala diferente nos dois eixos, retângulo menor que os cantos): a sombra é
    /// desenhada pelo `shadowBlur`.
    fn draw_shadow_tiled(&self, rect: RectF, radius: f32, shadow: &aylis_core::Shadow) -> bool {
        let Ok(m) = self.ctx.get_transform() else { return false };
        let (a, d) = (m.a(), m.d());
        if m.b() != 0.0 || m.c() != 0.0 || a <= 0.0 || (a - d).abs() > 1e-6 {
            return false;
        }
        let r = rect.inflate(shadow.spread);
        // Em pixels da tela: o deslocamento da sombra não segue a transformação (como no
        // `shadowOffset` do canvas), só a escala.
        let (x, y) = ((r.x as f64) * a + m.e() + shadow.offset.x as f64 * a, (r.y as f64) * d + m.f() + shadow.offset.y as f64 * d);
        let (w, h) = (r.width as f64 * a, r.height as f64 * d);
        let rad = (radius + shadow.spread).max(0.0) as f64 * a;
        let Some(tile) = shadow_tile(rad.min(w / 2.0).min(h / 2.0), shadow.blur as f64 * a, shadow.color) else { return false };
        let c = tile.corner;
        if w < 2.0 * c || h < 2.0 * c {
            return false;
        }
        // A caixa da sombra em pixels inteiros (meio pixel de uma sombra desfocada não se vê,
        // e os pedaços se encontram sem fresta).
        let (l, t) = ((x - tile.pad).round(), (y - tile.pad).round());
        let (rr, b) = ((x + w + tile.pad).round(), (y + h + tile.pad).round());
        let size = tile.canvas.width() as f64;
        let k = tile.pad + c;
        // Origem e destino de cada faixa: o canto, o meio (1 pixel esticado) e o outro canto.
        let cols = [(0.0, k, l, l + k), (k, 1.0, l + k, rr - k), (k + 1.0, size - k - 1.0, rr - k, rr)];
        let rows = [(0.0, k, t, t + k), (k, 1.0, t + k, b - k), (k + 1.0, size - k - 1.0, b - k, b)];
        let ctx = &self.ctx;
        ctx.save();
        let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        for (sy, sh, dy0, dy1) in rows {
            for (sx, sw, dx0, dx1) in cols {
                if dx1 > dx0 && dy1 > dy0 {
                    let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&tile.canvas, sx, sy, sw, sh, dx0, dy0, dx1 - dx0, dy1 - dy0);
                }
            }
        }
        ctx.restore();
        true
    }

    fn ellipse_path(&self, r: RectF) {
        self.ctx.begin_path();
        let _ = self.ctx.ellipse(
            (r.x + r.width / 2.0) as f64,
            (r.y + r.height / 2.0) as f64,
            (r.width / 2.0).max(0.0) as f64,
            (r.height / 2.0).max(0.0) as f64,
            0.0,
            0.0,
            2.0 * PI,
        );
    }

    fn set_fill(&self, brush: &Brush) {
        match paint_of(&self.ctx, brush) {
            Paint::Color(c) => self.ctx.set_fill_style_str(&c),
            Paint::Gradient(g) => self.ctx.set_fill_style_canvas_gradient(&g),
        }
    }

    fn set_stroke(&self, stroke: &Stroke) {
        match paint_of(&self.ctx, &stroke.brush) {
            Paint::Color(c) => self.ctx.set_stroke_style_str(&c),
            Paint::Gradient(g) => self.ctx.set_stroke_style_canvas_gradient(&g),
        }
        self.ctx.set_line_width(stroke.width as f64);
        let dash = js_sys::Array::new();
        for d in stroke.dash.iter().flatten() {
            dash.push(&JsValue::from_f64(*d as f64));
        }
        let _ = self.ctx.set_line_dash(&dash);
        self.ctx.set_line_cap(match stroke.cap {
            aylis_core::LineCap::Butt => "butt",
            aylis_core::LineCap::Round => "round",
            aylis_core::LineCap::Square => "square",
        });
        self.ctx.set_line_join(match stroke.join {
            aylis_core::LineJoin::Miter => "miter",
            aylis_core::LineJoin::Round => "round",
            aylis_core::LineJoin::Bevel => "bevel",
        });
    }

    fn build_path(&self, path: &aylis_core::Path) {
        use aylis_core::PathCmd;
        let ctx = &self.ctx.clone();
        ctx.begin_path();
        for c in &path.cmds {
            match *c {
                PathCmd::MoveTo(p) => ctx.move_to(p.x as f64, p.y as f64),
                PathCmd::LineTo(p) => ctx.line_to(p.x as f64, p.y as f64),
                PathCmd::QuadTo(c, p) => ctx.quadratic_curve_to(c.x as f64, c.y as f64, p.x as f64, p.y as f64),
                PathCmd::CubicTo(a, b, p) => {
                    ctx.bezier_curve_to(a.x as f64, a.y as f64, b.x as f64, b.y as f64, p.x as f64, p.y as f64)
                }
                PathCmd::Close => ctx.close_path(),
            }
        }
    }
}

impl Canvas2d<'_> {
    /// Passa a desenhar a máscara da [`WebMask`] aberta, num canvas à parte com a mesma
    /// transformação do conteúdo.
    fn start_mask(&mut self, luminance: bool) {
        let Some(state) = self.masks.last_mut() else { return };
        let Some(main) = state.content_ctx.canvas() else { return };
        let Some((mask, mask_ctx)) = layer_canvas(main.width(), main.height()) else { return };
        if let Ok(m) = state.content_ctx.get_transform() {
            let _ = mask_ctx.set_transform(m.a(), m.b(), m.c(), m.d(), m.e(), m.f());
        }
        // Os `save` do conteúdo que ficaram abertos não passam para a máscara.
        for _ in 0..self.depth {
            state.content_ctx.restore();
        }
        self.depth = 0;
        state.mask = Some(mask);
        state.luminance = luminance;
        self.ctx = mask_ctx;
    }
}

/// Troca as cores da máscara pelo brilho delas como alfa (o brilho vezes o alfa, com os pesos
/// do SVG e do `LumaColorFilter` do Skia), pixel a pixel.
fn luminance_to_alpha(mask: &HtmlCanvasElement) {
    let Some(ctx) = mask.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) else { return };
    let (w, h) = (mask.width(), mask.height());
    if w == 0 || h == 0 {
        return;
    }
    let Ok(image) = ctx.get_image_data(0.0, 0.0, w as f64, h as f64) else { return };
    let mut data = image.data().0;
    for px in data.chunks_exact_mut(4) {
        let (r, g, b, a) = (px[0] as f32, px[1] as f32, px[2] as f32, px[3] as f32);
        let lum = (0.2125 * r + 0.7154 * g + 0.0721 * b) / 255.0;
        px[..3].fill(0);
        px[3] = (lum * a).round().clamp(0.0, 255.0) as u8;
    }
    if let Ok(out) = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), w, h) {
        let _ = ctx.put_image_data(&out, 0.0, 0.0);
    }
}

impl Canvas for Canvas2d<'_> {
    fn save(&mut self) {
        self.depth += 1;
        self.ctx.save();
    }

    fn restore(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
            self.ctx.restore();
        } else if self.layers.last().is_some_and(|l| l.cached == self.cached.len()) {
            // Fecha a camada: o grupo vai para o canvas de baixo com a opacidade, no recorte
            // de lá.
            let Some(layer) = self.layers.pop() else { return };
            self.ctx = layer.main;
            self.depth = layer.depth;
            self.ctx.save();
            let _ = self.ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            self.ctx.set_global_alpha(self.ctx.global_alpha() * layer.opacity);
            if layer.blend != aylis_core::BlendMode::Normal {
                let _ = self.ctx.set_global_composite_operation(layer.blend.css());
            }
            let _ = self.ctx.draw_image_with_html_canvas_element(&layer.canvas, 0.0, 0.0);
            self.ctx.restore();
            SPARE_LAYERS.with(|s| {
                let mut spare = s.borrow_mut();
                if spare.len() < 4 {
                    spare.push(layer.canvas);
                }
            });
        }
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        let _ = self.ctx.translate(dx as f64, dy as f64);
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        let _ = self.ctx.scale(sx as f64, sy as f64);
    }

    fn rotate(&mut self, degrees: f32) {
        let _ = self.ctx.rotate((degrees as f64).to_radians());
    }

    fn clip_rect(&mut self, rect: RectF) {
        self.rect_path(rect, 0.0);
        self.ctx.clip();
    }

    fn save_layer(&mut self, opacity: f32) {
        self.save_layer_blend(opacity, aylis_core::BlendMode::Normal, RectF::default());
    }

    // A camada já é um canvas à parte: ao fechar, ela se junta ao de baixo pelo
    // `globalCompositeOperation` do modo (os mesmos nomes do CSS). Sem o canvas à parte, sem
    // mistura.
    fn save_layer_blend(&mut self, opacity: f32, blend: aylis_core::BlendMode, _bounds: RectF) {
        // O grupo num canvas à parte, com a mesma transformação; o recorte de baixo vale ao
        // compor. Sem como criar o canvas, o `globalAlpha` em cada desenho (o de antes).
        let target = self.ctx.canvas().zip(self.ctx.get_transform().ok()).and_then(|(main, m)| Some((layer_canvas(main.width(), main.height())?, m)));
        let Some(((canvas, target), m)) = target else {
            self.depth += 1;
            self.ctx.save();
            self.ctx.set_global_alpha(self.ctx.global_alpha() * opacity as f64);
            return;
        };
        let _ = target.set_transform(m.a(), m.b(), m.c(), m.d(), m.e(), m.f());
        let main = std::mem::replace(&mut self.ctx, target);
        let depth = std::mem::take(&mut self.depth);
        self.layers.push(WebLayer { main, depth, cached: self.cached.len(), canvas, opacity: opacity as f64, blend });
    }

    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        self.set_fill(brush);
        self.rect_path(rect, radius);
        self.ctx.fill();
    }

    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        self.set_stroke(stroke);
        self.rect_path(rect.inflate(-stroke.width / 2.0), (radius - stroke.width / 2.0).max(0.0));
        self.ctx.stroke();
    }

    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        self.set_fill(brush);
        self.ellipse_path(rect);
        self.ctx.fill();
    }

    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        self.set_stroke(stroke);
        self.ellipse_path(rect.inflate(-stroke.width / 2.0));
        self.ctx.stroke();
    }

    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        self.set_stroke(stroke);
        self.ctx.begin_path();
        self.ctx.move_to(from.x as f64, from.y as f64);
        self.ctx.line_to(to.x as f64, to.y as f64);
        self.ctx.stroke();
    }

    fn fill_path(&mut self, path: &aylis_core::Path, brush: &Brush) {
        self.set_fill(brush);
        self.build_path(path);
        if path.even_odd {
            self.ctx.fill_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
        } else {
            self.ctx.fill();
        }
    }

    fn begin_masked(&mut self, inverse: bool) -> bool {
        let made = self.ctx.canvas().zip(self.ctx.get_transform().ok()).and_then(|(main, m)| Some((layer_canvas(main.width(), main.height())?, m)));
        let Some(((content, content_ctx), m)) = made else { return false };
        let _ = content_ctx.set_transform(m.a(), m.b(), m.c(), m.d(), m.e(), m.f());
        let main = std::mem::replace(&mut self.ctx, content_ctx.clone());
        let depth = std::mem::take(&mut self.depth);
        self.masks.push(WebMask { main, depth, content, content_ctx, mask: None, inverse, luminance: false });
        true
    }

    fn begin_mask(&mut self) {
        self.start_mask(false);
    }

    fn begin_luminance_mask(&mut self) {
        self.start_mask(true);
    }

    fn end_mask(&mut self) {
        let Some(state) = self.masks.pop() else { return };
        self.ctx = state.main;
        self.depth = state.depth;
        let content_ctx = state.content_ctx;
        if let Some(mask) = &state.mask {
            if state.luminance {
                luminance_to_alpha(mask);
            }
            content_ctx.save();
            let _ = content_ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            content_ctx.set_global_alpha(1.0);
            let _ = content_ctx.set_global_composite_operation(if state.inverse { "destination-out" } else { "destination-in" });
            let _ = content_ctx.draw_image_with_html_canvas_element(mask, 0.0, 0.0);
            content_ctx.restore();
        }
        draw_at_device(&self.ctx, &state.content, (0.0, 0.0));
        SPARE_LAYERS.with(|s| {
            let mut spare = s.borrow_mut();
            for canvas in std::iter::once(state.content).chain(state.mask) {
                if spare.len() < 4 {
                    spare.push(canvas);
                }
            }
        });
    }

    fn begin_cached(&mut self, key: u64, r: RectF) -> bool {
        use std::hash::{Hash, Hasher};
        let Ok(m) = self.ctx.get_transform() else {
            self.cached.push(None);
            return false;
        };
        // Só sem giro: a imagem vai para a tela pixel a pixel.
        let (a, d) = (m.a(), m.d());
        if m.b() != 0.0 || m.c() != 0.0 || a <= 0.0 || d <= 0.0 {
            self.cached.push(None);
            return false;
        }
        let x0 = (r.x as f64 * a + m.e()).round();
        let y0 = (r.y as f64 * d + m.f()).round();
        let (w, h) = ((r.width as f64 * a).ceil(), (r.height as f64 * d).ceil());
        if w < 1.0 || h < 1.0 || w > 8192.0 || h > 8192.0 {
            self.cached.push(None);
            return false;
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (key, w as u32, h as u32, a.to_bits(), d.to_bits()).hash(&mut hasher);
        let full = hasher.finish();
        if let Some(canvas) = LAYERS.with(|l| l.borrow_mut().get(full).cloned()) {
            draw_at_device(&self.ctx, &canvas, (x0, y0));
            return true;
        }
        // Sem lugar no limite das camadas (`image_cache.layers_mb`): desenha direto.
        if !aylis_core::LayerCache::<HtmlCanvasElement>::accepts(w as usize * h as usize * 4) {
            self.cached.push(None);
            return false;
        }
        let made = web_sys::window().and_then(|w| w.document()).and_then(|d| d.create_element("canvas").ok()).and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok());
        let Some(canvas) = made else {
            self.cached.push(None);
            return false;
        };
        canvas.set_width(w as u32);
        canvas.set_height(h as u32);
        let Some(target) = canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) else {
            self.cached.push(None);
            return false;
        };
        // A mesma escala; a origem da imagem é o canto dela na tela.
        let _ = target.set_transform(a, 0.0, 0.0, d, m.e() - x0, m.f() - y0);
        let main = std::mem::replace(&mut self.ctx, target);
        let depth = std::mem::take(&mut self.depth);
        self.cached.push(Some(WebCache { main, depth, canvas, key: full, owner: key, at: (x0, y0) }));
        false
    }

    fn end_cached(&mut self) {
        let Some(state) = self.cached.pop().flatten() else { return };
        self.ctx = state.main;
        self.depth = state.depth;
        draw_at_device(&self.ctx, &state.canvas, state.at);
        // O limite é em bytes (`image_cache.layers_mb`): cada destaque novo de um mapa é uma
        // imagem nova, e sai a usada há mais tempo.
        let bytes = state.canvas.width() as usize * state.canvas.height() as usize * 4;
        LAYERS.with(|l| l.borrow_mut().insert(state.owner, state.key, state.canvas, bytes));
    }

    fn forget_cached(&mut self, key: u64) {
        LAYERS.with(|l| l.borrow_mut().forget(key));
    }

    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        // O próprio canvas desenhado sobre si, desfocado (`filter: blur()`), recortado
        // nos cantos. Em pixels: lê a transformação atual.
        let ctx = &self.ctx.clone();
        let read = || -> Option<[f64; 6]> {
            let f: js_sys::Function = js_sys::Reflect::get(ctx, &"getTransform".into()).ok()?.dyn_into().ok()?;
            let m = f.call0(ctx).ok()?;
            let g = |k: &str| js_sys::Reflect::get(&m, &k.into()).ok().and_then(|v| v.as_f64());
            Some([g("a")?, g("b")?, g("c")?, g("d")?, g("e")?, g("f")?])
        };
        let (Some([a, _b, _c, d, e, f]), Some(canvas)) = (read(), ctx.canvas()) else { return false };
        let pad = (blur as f64) * a.abs() * 3.0;
        let x0 = (rect.x as f64 * a + e - pad).max(0.0);
        let y0 = (rect.y as f64 * d + f - pad).max(0.0);
        let x1 = ((rect.right() as f64) * a + e + pad).min(canvas.width() as f64);
        let y1 = ((rect.bottom() as f64) * d + f + pad).min(canvas.height() as f64);
        if x1 <= x0 || y1 <= y0 {
            return false;
        }
        ctx.save();
        self.rect_path(rect, radius);
        ctx.clip();
        let _ = ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let sigma = blur as f64 * a.abs();
        if has_canvas_filter() {
            ctx.set_filter(&format!("blur({sigma}px)"));
            let _ = ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&canvas, x0, y0, x1 - x0, y1 - y0, x0, y0, x1 - x0, y1 - y0);
        } else {
            blur_by_scaling(ctx, &canvas, [x0, y0, x1 - x0, y1 - y0], sigma);
        }
        ctx.restore();
        true
    }

    fn clip_path(&mut self, path: &aylis_core::Path, inverse: bool) {
        if inverse {
            let mut p = aylis_core::Path::new();
            p.rect(RectF::new(-1.0e6, -1.0e6, 2.0e6, 2.0e6), 0.0);
            p.append(path);
            self.build_path(&p);
            self.ctx.clip_with_canvas_winding_rule(web_sys::CanvasWindingRule::Evenodd);
        } else {
            self.build_path(path);
            let rule = if path.even_odd { web_sys::CanvasWindingRule::Evenodd } else { web_sys::CanvasWindingRule::Nonzero };
            self.ctx.clip_with_canvas_winding_rule(rule);
        }
    }

    fn stroke_path(&mut self, path: &aylis_core::Path, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        self.set_stroke(stroke);
        self.build_path(path);
        self.ctx.stroke();
    }

    /// A sombra do próprio canvas 2D: desenha a forma longe da área visível e só a
    /// sombra (deslocada de volta) aparece.
    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &aylis_core::Shadow) {
        if shadow.color.a == 0 {
            return;
        }
        if self.draw_shadow_tiled(rect, radius, shadow) {
            return;
        }
        const FAR: f32 = 100_000.0;
        let ctx = &self.ctx.clone();
        ctx.save();
        // O deslocamento da sombra não segue a transformação: converte para pixels.
        let (sx, sy) = canvas_scale(ctx);
        ctx.set_shadow_color(&css_color(shadow.color));
        ctx.set_shadow_blur((shadow.blur * sx) as f64);
        ctx.set_shadow_offset_x(((shadow.offset.x + FAR) * sx) as f64);
        ctx.set_shadow_offset_y((shadow.offset.y * sy) as f64);
        ctx.set_fill_style_str("#000");
        let r = rect.inflate(shadow.spread);
        self.rect_path(RectF::new(r.x - FAR, r.y, r.width, r.height), (radius + shadow.spread).max(0.0));
        ctx.fill();
        ctx.restore();
    }

    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        if text.is_empty() || rect.width <= 0.0 {
            return;
        }
        let text = &*plain_emoji(text, style);
        sync_fonts();
        let ctx = &self.ctx.clone();
        let font = font_css(&style.font, !style.color_glyphs);
        let spacing = style.letter_spacing;
        ctx.set_font(&font);
        ctx.set_fill_style_str(&css_color(style.color));
        ctx.set_text_baseline("middle");
        // Quebra e reticências pela regra comum (iguais às do Skia).
        let line_height = line_height(&style.font);
        let max_lines = ((rect.height + 1.0) / line_height).floor().max(1.0) as usize;
        let mut measure = |s: &str| text_width(ctx, &font, spacing, s);
        let lines = aylis_core::text::layout_lines_marked(text, rect.width, max_lines, style.wrap, style.trimming, &mut measure);
        let total = line_height * lines.len() as f32;
        let top = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => rect.y,
            TextAlign::Center => rect.y + (rect.height - total) / 2.0,
            TextAlign::Trailing => rect.bottom() - total,
        };
        let (align, x) = match style.h_align {
            TextAlign::Leading | TextAlign::Justify => ("left", rect.x),
            TextAlign::Center => ("center", rect.x + rect.width / 2.0),
            TextAlign::Trailing => ("right", rect.right()),
        };
        ctx.set_text_align(align);
        for (i, (line, end)) in lines.iter().enumerate() {
            let y = top + line_height * (i as f32 + 0.5);
            // Justificado: cada palavra no lugar dela (a última linha do parágrafo fica à
            // esquerda); o sublinhado vai de ponta a ponta.
            let words = (style.h_align == TextAlign::Justify && !end).then(|| aylis_core::text::justify_line(line, rect.width, &mut measure)).flatten();
            if let Some(words) = words {
                ctx.set_text_align("left");
                for (wx, word) in &words {
                    ctx.set_font(&font);
                    fill_spaced(ctx, &font, spacing, word, rect.x + wx, y);
                }
                ctx.set_text_align(align);
                let right = words.last().map_or(0.0, |(wx, w)| wx + text_width(ctx, &font, spacing, w));
                aylis_core::decorate_line(self, &style.font, style.color, rect.x, right, top + line_height * i as f32, line_height);
                continue;
            }
            if spacing == 0.0 {
                let _ = ctx.fill_text(line, x as f64, y as f64);
            } else {
                // Com espaço entre letras, a ponta esquerda pela largura medida (sem o espaço
                // depois da última letra, que o `letterSpacing` do canvas conta).
                let w = text_width(ctx, &font, spacing, line);
                let left = match style.h_align {
                    TextAlign::Leading | TextAlign::Justify => x,
                    TextAlign::Center => x - w / 2.0,
                    TextAlign::Trailing => x - w,
                };
                ctx.set_font(&font);
                ctx.set_text_align("left");
                fill_spaced(ctx, &font, spacing, line, left, y);
                ctx.set_text_align(align);
            }
            // Sublinhado e tachado: o canvas 2D não faz sozinho.
            if style.font.underline || style.font.strikeout {
                let w = text_width(ctx, &font, spacing, line);
                let left = match style.h_align {
                    TextAlign::Leading | TextAlign::Justify => x,
                    TextAlign::Center => x - w / 2.0,
                    TextAlign::Trailing => x - w,
                };
                aylis_core::decorate_line(self, &style.font, style.color, left, w, top + line_height * i as f32, line_height);
            }
        }
    }

    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        measure_line(&self.ctx, text, style)
    }

    fn measure_text_wrapped(&mut self, text: &str, style: &TextStyle, max_width: f32) -> SizeF {
        let text = &*plain_emoji(text, style);
        sync_fonts();
        let ctx = &self.ctx.clone();
        let font = font_css(&style.font, !style.color_glyphs);
        WRAPPED.with(|w| {
            w.borrow_mut().get_or(text, style, max_width, || {
                aylis_core::canvas::wrapped_size(text, max_width, line_height(&style.font), &mut |s| text_width(ctx, &font, style.letter_spacing, s))
            })
        })
    }

    fn device_scale(&self) -> f32 {
        let (sx, sy) = canvas_scale(&self.ctx);
        (sx * sy).abs().sqrt()
    }

    fn gpu_device(&self) -> Option<aylis_core::GpuDevice> {
        Some(aylis_core::GpuDevice::Web)
    }

    fn draw_gpu_surface(&mut self, dest: RectF, surface: &aylis_core::GpuSurface) -> bool {
        let aylis_core::GpuSurface::WebCanvas { element, .. } = *surface else { return false };
        let Some(canvas) = gpu_canvas(element) else { return false };
        // O navegador copia o canvas do WebGPU dentro da GPU.
        self.ctx
            .draw_image_with_html_canvas_element_and_dw_and_dh(&canvas, dest.x as f64, dest.y as f64, dest.width as f64, dest.height as f64)
            .is_ok()
    }

    fn draw_image(&mut self, image: &Image, src: RectF, dest: RectF, opacity: f32) {
        let ctx = &self.ctx.clone();
        let (sx, sy, sw, sh) = (src.x as f64, src.y as f64, src.width as f64, src.height as f64);
        let (dx, dy, dw, dh) = (dest.x as f64, dest.y as f64, dest.width as f64, dest.height as f64);
        with_image(image, |img| {
            ctx.save();
            ctx.set_global_alpha(ctx.global_alpha() * opacity as f64);
            ctx.set_image_smoothing_enabled(true);
            let _ = match img {
                WebImage::Ready(el) => ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(el, sx, sy, sw, sh, dx, dy, dw, dh),
                WebImage::Pixels(el) => ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(el, sx, sy, sw, sh, dx, dy, dw, dh),
                WebImage::Loading | WebImage::Failed => Ok(()),
            };
            ctx.restore();
        });
    }

    fn image_size(&mut self, image: &Image) -> Option<SizeF> {
        if let Some(size) = image.known_size() {
            return Some(size);
        }
        with_image(image, |img| match img {
            WebImage::Ready(el) if el.natural_width() > 0 => Some(SizeF::new(el.natural_width() as f32, el.natural_height() as f32)),
            _ => None,
        })
    }

    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        self.rect_path(rect, radius);
        self.ctx.clip();
    }
}

/// O desenho fora da tela no navegador: um `<canvas>` solto, desenhado pelo [`Canvas2d`] e lido
/// pelo `getImageData` (o texto do `PaintCanvas` que vira pixel, o `Ui::render_rgba`, o
/// `Ui::render_png`). As imagens e as fontes que ainda carregam (o navegador decodifica depois)
/// ficam de fora desse desenho, como na tela antes do carregamento.
pub struct WebOffscreen;

/// Um `<canvas>` de `width` × `height` pixels com o fundo, e o contexto dele.
fn loose_canvas(width: u32, height: u32, background: Color) -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let canvas: HtmlCanvasElement = web_sys::window()?.document()?.create_element("canvas").ok()?.dyn_into().ok()?;
    canvas.set_width(width.max(1));
    canvas.set_height(height.max(1));
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").ok()??.dyn_into().ok()?;
    if background.a > 0 {
        ctx.set_fill_style_str(&css_color(background));
        ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
    }
    Some((canvas, ctx))
}

/// Desenha `paint` com `scale` pixels por unidade e devolve os pixels RGBA (sem pré-multiplicar,
/// como o `getImageData` dá).
fn draw_loose(ctx: &CanvasRenderingContext2d, width: u32, height: u32, scale: f32, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
    {
        let mut canvas = Canvas2d::new(ctx);
        canvas.save();
        canvas.scale(scale, scale);
        paint(&mut canvas);
        canvas.restore();
    }
    Some(ctx.get_image_data(0.0, 0.0, width.max(1) as f64, height.max(1) as f64).ok()?.data().0)
}

impl aylis_core::platform::OffscreenService for WebOffscreen {
    fn render_png(&self, width: u32, height: u32, scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        let (w, h) = (((width as f32) * scale).round() as u32, ((height as f32) * scale).round() as u32);
        let (canvas, ctx) = loose_canvas(w, h, background)?;
        draw_loose(&ctx, w, h, scale, paint)?;
        // O PNG pelo próprio navegador (`toDataURL`), em base64.
        let url = canvas.to_data_url().ok()?;
        let encoded = url.split_once(',')?.1;
        let binary = web_sys::window()?.atob(encoded).ok()?;
        Some(binary.chars().map(|c| c as u8).collect())
    }

    fn render_rgba(&self, width: u32, height: u32, scale: f32, background: Color, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        let (_, ctx) = loose_canvas(width, height, background)?;
        draw_loose(&ctx, width, height, scale, paint)
    }

    fn render_area_rgba(&self, width: u32, height: u32, background: Color, over: Option<(&[u8], RectF)>, paint: &mut dyn FnMut(&mut dyn Canvas)) -> Option<Vec<u8>> {
        let Some((previous, area)) = over.filter(|(p, _)| p.len() == (width * height * 4) as usize) else {
            let (_, ctx) = loose_canvas(width, height, background)?;
            return draw_loose(&ctx, width, height, 1.0, paint);
        };
        // A imagem de antes, e só a área repintada por cima (com o fundo nela).
        let (_, ctx) = loose_canvas(width, height, Color::TRANSPARENT)?;
        let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(previous), width, height).ok()?;
        ctx.put_image_data(&image, 0.0, 0.0).ok()?;
        ctx.set_fill_style_str(&css_color(background));
        ctx.fill_rect(area.x as f64, area.y as f64, area.width as f64, area.height as f64);
        draw_loose(&ctx, width, height, 1.0, &mut |c| {
            c.save();
            c.clip_rect(area);
            paint(c);
            c.restore();
        })
    }
}

/// Medição de texto num canvas próprio, fora da tela.
pub struct WebTextService {
    ctx: CanvasRenderingContext2d,
}

impl WebTextService {
    pub fn new() -> Result<Self, JsValue> {
        let document = web_sys::window().and_then(|w| w.document()).ok_or("sem document")?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        let ctx = canvas.get_context("2d")?.ok_or("sem contexto 2d")?.dyn_into()?;
        Ok(Self { ctx })
    }
}

impl TextService for WebTextService {
    fn measure(&self, text: &str, style: &TextStyle, max_width: Option<f32>) -> SizeF {
        let text = &*plain_emoji(text, style);
        sync_fonts();
        match max_width {
            None => measure_line(&self.ctx, text, style),
            Some(width) => {
                let font = font_css(&style.font, !style.color_glyphs);
                let lines = wrap_lines(&self.ctx, &font, style.letter_spacing, text, width);
                let widest = lines.iter().map(|l| text_width(&self.ctx, &font, style.letter_spacing, l)).fold(0.0, f32::max);
                SizeF::new(widest.ceil(), line_height(&style.font) * lines.len() as f32)
            }
        }
    }
}

// ---------------------------------------------------------------------------

enum Paint {
    Color(String),
    Gradient(CanvasGradient),
}

fn paint_of(ctx: &CanvasRenderingContext2d, brush: &Brush) -> Paint {
    let gradient = match brush {
        Brush::Solid(c) => return Paint::Color(css_color(*c)),
        Brush::Linear { start, end, .. } => {
            Some(ctx.create_linear_gradient(start.x as f64, start.y as f64, end.x as f64, end.y as f64))
        }
        Brush::Radial { center, radius, .. } => ctx
            .create_radial_gradient(
                center.x as f64,
                center.y as f64,
                0.0,
                center.x as f64,
                center.y as f64,
                *radius as f64,
            )
            .ok(),
        Brush::Sweep { center, start_angle, .. } => conic_gradient(ctx, *start_angle, *center),
    };
    let stops: &[GradientStop] = match brush {
        Brush::Linear { stops, .. } | Brush::Radial { stops, .. } | Brush::Sweep { stops, .. } => stops,
        Brush::Solid(_) => &[],
    };
    match gradient {
        Some(g) => {
            for s in stops {
                let _ = g.add_color_stop(s.offset.clamp(0.0, 1.0), &css_color(s.color));
            }
            Paint::Gradient(g)
        }
        // Navegador sem gradiente cônico: usa a primeira cor.
        None => Paint::Color(css_color(stops.first().map_or(Color::TRANSPARENT, |s| s.color))),
    }
}

/// `createConicGradient` ainda não está no web-sys; chama pelo nome.
fn conic_gradient(ctx: &CanvasRenderingContext2d, start_angle: f32, center: PointF) -> Option<CanvasGradient> {
    let function: js_sys::Function = js_sys::Reflect::get(ctx, &"createConicGradient".into()).ok()?.dyn_into().ok()?;
    // No canvas o ângulo 0 aponta para a direita, como no Skia.
    let angle = (start_angle as f64).to_radians();
    function
        .call3(ctx, &angle.into(), &(center.x as f64).into(), &(center.y as f64).into())
        .ok()?
        .dyn_into()
        .ok()
}

/// Escala atual do contexto (`getTransform().a` e `.d`), pelo nome: o web-sys só tem
/// esse método com a feature `DomMatrix`.
fn canvas_scale(ctx: &CanvasRenderingContext2d) -> (f32, f32) {
    let read = || -> Option<(f32, f32)> {
        let f: js_sys::Function = js_sys::Reflect::get(ctx, &"getTransform".into()).ok()?.dyn_into().ok()?;
        let m = f.call0(ctx).ok()?;
        let a = js_sys::Reflect::get(&m, &"a".into()).ok()?.as_f64()?;
        let d = js_sys::Reflect::get(&m, &"d".into()).ok()?.as_f64()?;
        Some((a as f32, d as f32))
    };
    read().unwrap_or((1.0, 1.0))
}

fn css_color(c: Color) -> String {
    format!("rgba({},{},{},{})", c.r, c.g, c.b, c.a as f32 / 255.0)
}

/// O `font` do CSS. `mono_emoji` (o `color_glyphs: false`): a Noto Emoji sem cor antes da
/// fonte pedida, só nos trechos de emoji (veja [`sync_fonts`]).
fn font_css(font: &FontSpec, mono_emoji: bool) -> String {
    let weight = match font.weight {
        FontWeight::Thin => 100,
        FontWeight::Light => 300,
        FontWeight::Normal => 400,
        FontWeight::Medium => 500,
        FontWeight::SemiBold => 600,
        FontWeight::Bold => 700,
        FontWeight::Black => 900,
    };
    let family = match font.family.as_str() {
        "" | "sans-serif" => "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Noto Sans\", sans-serif".to_string(),
        "monospace" => "ui-monospace, \"Cascadia Mono\", Consolas, Menlo, monospace".to_string(),
        family => format!("\"{family}\", sans-serif"),
    };
    let style = if font.italic { "italic " } else { "" };
    let mono = if mono_emoji { format!("\"{MONO_EMOJI_FACE}\", ") } else { String::new() };
    format!("{style}{weight} {}px {mono}{family}", font.size)
}

fn line_height(font: &FontSpec) -> f32 {
    (font.size * 1.3).ceil()
}

thread_local! {
    /// As larguras já medidas, por fonte (o CSS dela) e texto. O `measureText` é uma ida ao
    /// navegador, e o layout e a quebra das linhas medem os mesmos textos a cada quadro (a
    /// Landing no celular: ~300 medidas por quadro de rolagem, cada texto umas 4 vezes).
    static WIDTHS: RefCell<HashMap<String, HashMap<String, f32>>> = RefCell::new(HashMap::new());
    static WIDTH_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Os tamanhos de `measure_text_wrapped` já calculados.
    static WRAPPED: RefCell<aylis_core::canvas::WrapCache> = RefCell::new(aylis_core::canvas::WrapCache::default());
}

/// Acima disto, as larguras guardadas recomeçam (um editor com um texto enorme não cresce
/// sem fim).
const MAX_WIDTHS: usize = 20_000;

fn forget_widths() {
    WIDTHS.with(|w| w.borrow_mut().clear());
    WIDTH_COUNT.with(|c| c.set(0));
}

/// A largura de `text` na fonte `font` (o CSS de [`font_css`]), com `spacing` entre as letras
/// (sem o espaço depois da última). Só mede (e só troca a fonte do contexto) na primeira vez.
fn text_width(ctx: &CanvasRenderingContext2d, font: &str, spacing: f32, text: &str) -> f32 {
    let key: std::borrow::Cow<str> = if spacing == 0.0 { font.into() } else { format!("{font}|{spacing}").into() };
    if let Some(w) = WIDTHS.with(|w| w.borrow().get(key.as_ref()).and_then(|m| m.get(text)).copied()) {
        return w;
    }
    let width = if spacing == 0.0 || text.is_empty() {
        ctx.set_font(font);
        ctx.measure_text(text).map_or(0.0, |m| m.width() as f32)
    } else if has_letter_spacing() {
        ctx.set_font(font);
        set_letter_spacing(ctx, spacing);
        let w = ctx.measure_text(text).map_or(0.0, |m| m.width() as f32);
        set_letter_spacing(ctx, 0.0);
        w - spacing
    } else {
        let mut buf = [0u8; 4];
        let letters: Vec<f32> = text.chars().map(|c| text_width(ctx, font, 0.0, c.encode_utf8(&mut buf))).collect();
        letters.iter().sum::<f32>() + spacing * (letters.len() as f32 - 1.0)
    };
    if WIDTH_COUNT.with(|c| c.replace(c.get() + 1)) >= MAX_WIDTHS {
        forget_widths();
    }
    WIDTHS.with(|w| w.borrow_mut().entry(key.into_owned()).or_default().insert(text.to_string(), width));
    width
}

fn measure_line(ctx: &CanvasRenderingContext2d, text: &str, style: &TextStyle) -> SizeF {
    let text = &*plain_emoji(text, style);
    SizeF::new(text_width(ctx, &font_css(&style.font, !style.color_glyphs), style.letter_spacing, text), line_height(&style.font))
}

/// Quebra por palavras para caber em `width` na fonte `font`.
fn wrap_lines(ctx: &CanvasRenderingContext2d, font: &str, spacing: f32, text: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if !line.is_empty() && text_width(ctx, font, spacing, &candidate) > width {
                lines.push(std::mem::take(&mut line));
                line = word.to_string();
            } else {
                line = candidate;
            }
        }
        lines.push(line);
    }
    lines
}
