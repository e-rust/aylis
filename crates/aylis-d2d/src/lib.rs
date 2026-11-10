//! # aylis-d2d
//!
//! Renderizador do aylis no Windows **sem Skia**: Direct2D (desenho acelerado pela GPU),
//! DirectWrite (texto) e WIC (imagens), que já vêm no Windows. O executável não leva
//! nenhuma biblioteca de desenho.
//!
//! - [`D2dRenderer`]: fábricas e caches (fontes, medidas de texto, imagens decodificadas);
//! - [`HwndTarget`]: desenha numa janela. O padrão é o Direct3D 11 com swap chain "flip"
//!   e `ID2D1DeviceContext`, que compõe imagens deixadas na GPU por outro desenhista (o 3D)
//!   sem passar pela CPU; `AYLIS_D2D_TARGET=hwnd` usa o `ID2D1HwndRenderTarget` antigo;
//! - [`D2dRenderer::render_rgba`]: desenha fora da tela (capturas, testes);
//! - [`D2dCanvas`]: o [`Canvas`] do aylis;
//! - [`D2dTextService`]: medição de texto para o layout.
//!
//! Quebra de linha e reticências usam a mesma regra dos outros renderizadores
//! (`aylis_core::text`). A sombra usa o efeito Gaussian Blur (guardada pronta) e o
//! gradiente cônico, que o Direct2D não tem, é pintado numa imagem.

#![cfg(windows)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use aylis_core::text::{Trimming, layout_lines, layout_lines_marked};
use aylis_core::{
    Brush, Canvas, Color, FontSpec, FontWeight, GpuDevice, GpuSurface, Image, ImageSource, LineCap, LineJoin, Path, PathCmd,
    PointF, RectF, Shadow, SizeF, Stroke, TextAlign, TextService, TextStyle, ImageCache,
};
use windows::Win32::Foundation::{D2DERR_RECREATE_TARGET, HANDLE, HMODULE, HWND};
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::DirectWrite::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Imaging::*;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx};
use windows_core::{Interface, PCWSTR};
use windows_numerics::{Matrix3x2, Vector2};

pub type Error = windows_core::Error;

// ---------------------------------------------------------------------------
// conversões

fn v(p: PointF) -> Vector2 {
    Vector2 { X: p.x, Y: p.y }
}

fn rect(r: RectF) -> D2D_RECT_F {
    D2D_RECT_F { left: r.x, top: r.y, right: r.x + r.width, bottom: r.y + r.height }
}

fn color(c: Color) -> D2D1_COLOR_F {
    D2D1_COLOR_F { r: c.r as f32 / 255.0, g: c.g as f32 / 255.0, b: c.b as f32 / 255.0, a: c.a as f32 / 255.0 }
}

fn mul(a: &Matrix3x2, b: &Matrix3x2) -> Matrix3x2 {
    Matrix3x2 {
        M11: a.M11 * b.M11 + a.M12 * b.M21,
        M12: a.M11 * b.M12 + a.M12 * b.M22,
        M21: a.M21 * b.M11 + a.M22 * b.M21,
        M22: a.M21 * b.M12 + a.M22 * b.M22,
        M31: a.M31 * b.M11 + a.M32 * b.M21 + b.M31,
        M32: a.M31 * b.M12 + a.M32 * b.M22 + b.M32,
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

const INFINITE: D2D_RECT_F = D2D_RECT_F { left: -1.0e9, top: -1.0e9, right: 1.0e9, bottom: 1.0e9 };

// ---------------------------------------------------------------------------
// texto

#[derive(Clone, PartialEq, Eq, Hash)]
struct FontKey {
    family: String,
    /// Tamanho × 100 (chave inteira).
    size: u32,
    weight: i32,
    italic: bool,
}

impl FontKey {
    fn of(font: &FontSpec) -> Self {
        let weight = match font.weight {
            FontWeight::Thin => 100,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::Black => 900,
        };
        Self { family: font.family.clone(), size: (font.size * 100.0).round() as u32, weight, italic: font.italic }
    }

    fn family_name(&self) -> &str {
        match self.family.as_str() {
            "" | "sans-serif" | "system-ui" => "Segoe UI",
            "monospace" => "Consolas",
            "serif" => "Times New Roman",
            f => f,
        }
    }
}

/// Fontes e medidas de texto (compartilhado pelo canvas e pelo serviço de medição).
struct Text {
    dwrite: IDWriteFactory,
    /// Fontes do sistema mais as de arquivo registradas (`aylis_core::fonts`); `None` =
    /// só as do sistema.
    collection: RefCell<Option<IDWriteFontCollection>>,
    font_generation: std::cell::Cell<u64>,
    formats: RefCell<HashMap<FontKey, IDWriteTextFormat>>,
    /// Layouts prontos (texto + fonte): medir e desenhar reaproveitam.
    layouts: RefCell<HashMap<(FontKey, String), (IDWriteTextLayout, f32)>>,
    line_heights: RefCell<HashMap<FontKey, f32>>,
}

const LAYOUT_CACHE: usize = 4096;

impl Text {
    /// Fontes novas no registro: monta a coleção (sistema + registradas) e limpa os caches.
    fn sync_fonts(&self) {
        let generation = aylis_core::fonts::generation();
        if self.font_generation.get() == generation {
            return;
        }
        self.font_generation.set(generation);
        self.formats.borrow_mut().clear();
        self.layouts.borrow_mut().clear();
        self.line_heights.borrow_mut().clear();
        let fonts = aylis_core::fonts::registered();
        if fonts.is_empty() {
            *self.collection.borrow_mut() = None;
            return;
        }
        let built = (|| -> windows_core::Result<IDWriteFontCollection> {
            unsafe {
                // IDWriteFactory5: Windows 10 (1703) em diante.
                let factory: IDWriteFactory5 = self.dwrite.cast()?;
                let loader = factory.CreateInMemoryFontFileLoader()?;
                factory.RegisterFontFileLoader(&loader)?;
                let builder = factory.CreateFontSetBuilder()?;
                builder.AddFontSet(&factory.GetSystemFontSet()?)?;
                for font in &fonts {
                    // Sem dono, o DirectWrite copia os dados.
                    let file = loader.CreateInMemoryFontFileReference(
                        &factory,
                        font.data.as_ptr() as *const _,
                        font.data.len() as u32,
                        None::<&windows_core::IUnknown>,
                    )?;
                    builder.AddFontFile(&file)?;
                }
                let set = builder.CreateFontSet()?;
                factory.CreateFontCollectionFromFontSet(&set)?.cast()
            }
        })();
        *self.collection.borrow_mut() = built.ok();
    }

    fn format(&self, key: &FontKey) -> Option<IDWriteTextFormat> {
        self.sync_fonts();
        if let Some(f) = self.formats.borrow().get(key) {
            return Some(f.clone());
        }
        let family = wide(key.family_name());
        let mut family_z = family.clone();
        family_z.push(0);
        let locale = wide("pt-br\0");
        let style = if key.italic { DWRITE_FONT_STYLE_ITALIC } else { DWRITE_FONT_STYLE_NORMAL };
        let collection = self.collection.borrow().clone();
        let format = unsafe {
            self.dwrite
                .CreateTextFormat(
                    PCWSTR(family_z.as_ptr()),
                    collection.as_ref(),
                    DWRITE_FONT_WEIGHT(key.weight),
                    style,
                    DWRITE_FONT_STRETCH_NORMAL,
                    key.size as f32 / 100.0,
                    PCWSTR(locale.as_ptr()),
                )
                .ok()?
        };
        unsafe {
            let _ = format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP);
        }
        self.formats.borrow_mut().insert(key.clone(), format.clone());
        Some(format)
    }

    fn layout_metrics(&self, text: &str, key: &FontKey) -> Option<DWRITE_TEXT_METRICS> {
        let format = self.format(key)?;
        let w = wide(text);
        let layout = unsafe { self.dwrite.CreateTextLayout(&w, &format, 1.0e6, 1.0e6).ok()? };
        let mut m = DWRITE_TEXT_METRICS::default();
        unsafe { layout.GetMetrics(&mut m).ok()? };
        Some(m)
    }

    /// O layout de uma linha (com cache) e a largura dela.
    fn layout(&self, text: &str, key: &FontKey) -> Option<(IDWriteTextLayout, f32)> {
        self.sync_fonts();
        let cache_key = (key.clone(), text.to_string());
        if let Some(l) = self.layouts.borrow().get(&cache_key) {
            return Some(l.clone());
        }
        let format = self.format(key)?;
        let w = wide(text);
        let layout = unsafe { self.dwrite.CreateTextLayout(&w, &format, 1.0e6, 1.0e6).ok()? };
        let mut m = DWRITE_TEXT_METRICS::default();
        unsafe { layout.GetMetrics(&mut m).ok()? };
        let entry = (layout, m.widthIncludingTrailingWhitespace);
        let mut cache = self.layouts.borrow_mut();
        if cache.len() > LAYOUT_CACHE {
            cache.clear();
        }
        cache.insert(cache_key, entry.clone());
        Some(entry)
    }

    fn width(&self, text: &str, key: &FontKey) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        self.layout(text, key).map_or(0.0, |(_, w)| w)
    }

    fn line_height(&self, key: &FontKey) -> f32 {
        if let Some(h) = self.line_heights.borrow().get(key) {
            return *h;
        }
        let h = self.layout_metrics("Ag", key).map_or(key.size as f32 / 100.0 * 1.33, |m| m.height);
        self.line_heights.borrow_mut().insert(key.clone(), h);
        h
    }
}

/// Medição de texto para o `Ui` (layout, auto-size).
pub struct D2dTextService {
    text: Rc<Text>,
}

impl TextService for D2dTextService {
    fn measure(&self, text: &str, style: &TextStyle, max_width: Option<f32>) -> SizeF {
        let key = FontKey::of(&style.font);
        let lh = self.text.line_height(&key);
        match max_width {
            None => {
                let lines: Vec<&str> = text.split('\n').collect();
                let w = lines.iter().map(|l| self.text.width(l, &key)).fold(0.0f32, f32::max);
                SizeF::new(w, lh * lines.len() as f32)
            }
            Some(max) => {
                let mut measure = |s: &str| self.text.width(s, &key);
                let lines = layout_lines(text, max, usize::MAX, true, Trimming::None, &mut measure);
                let w = lines.iter().map(|l| self.text.width(l, &key)).fold(0.0f32, f32::max);
                SizeF::new(w, lh * lines.len().max(1) as f32)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// renderizador

/// Ponta, junção e tracejado (bits dos `f32`, já divididos pela espessura).
type StrokeKey = (i32, i32, Vec<u32>);

/// Imagem decodificada: largura, altura e pixels BGRA pré-multiplicados.
type Decoded = Rc<(u32, u32, Vec<u8>)>;

/// Fábricas e caches. Um por thread de interface.
pub struct D2dRenderer {
    factory: ID2D1Factory,
    factory1: ID2D1Factory1,
    wic: IWICImagingFactory,
    text: Rc<Text>,
    strokes: RefCell<HashMap<StrokeKey, ID2D1StrokeStyle>>,
    /// Geometrias já montadas, pelo conteúdo do caminho.
    geometries: RefCell<HashMap<u64, ID2D1PathGeometry>>,
    /// Imagens de arquivo decodificadas (`None`: não deu para decodificar).
    decoded: RefCell<ImageCache<Option<Decoded>>>,
}

/// A placa pedida em [`aylis_core::gpu::choice`] (`None`: a padrão do sistema).
fn chosen_adapter() -> Option<IDXGIAdapter> {
    use aylis_core::gpu::GpuChoice;
    let choice = aylis_core::gpu::choice();
    // SAFETY: chamadas COM do DXGI, sem ponteiros guardados.
    unsafe {
        let factory: IDXGIFactory6 = CreateDXGIFactory1().ok()?;
        let adapter: IDXGIAdapter1 = match &choice {
            GpuChoice::Auto => return None,
            GpuChoice::HighPerformance => factory.EnumAdapterByGpuPreference(0, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE).ok()?,
            GpuChoice::LowPower => factory.EnumAdapterByGpuPreference(0, DXGI_GPU_PREFERENCE_MINIMUM_POWER).ok()?,
            GpuChoice::Named(_) => (0..).map_while(|i| factory.EnumAdapters1(i).ok()).find(|a| a.GetDesc1().is_ok_and(|d| choice.matches_name(&adapter_name(&d.Description))))?,
        };
        adapter.cast().ok()
    }
}

/// O nome da placa na descrição do DXGI.
fn adapter_name(description: &[u16]) -> String {
    let end = description.iter().position(|&c| c == 0).unwrap_or(description.len());
    String::from_utf16_lossy(&description[..end])
}

impl D2dRenderer {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            // O winit (ou o próprio app) já pode ter iniciado o COM: tanto faz o resultado.
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            let factory1: ID2D1Factory1 = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
            let factory: ID2D1Factory = factory1.cast()?;
            let dwrite: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
            let wic: IWICImagingFactory = CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)?;
            Ok(Self {
                factory,
                factory1,
                wic,
                text: Rc::new(Text {
                    dwrite,
                    collection: RefCell::new(None),
                    font_generation: std::cell::Cell::new(u64::MAX),
                    formats: RefCell::default(),
                    layouts: RefCell::default(),
                    line_heights: RefCell::default(),
                }),
                strokes: RefCell::default(),
                geometries: RefCell::default(),
                decoded: RefCell::new(ImageCache::named("Direct2D · arquivos decodificados")),
            })
        }
    }

    pub fn text_service(&self) -> D2dTextService {
        D2dTextService { text: self.text.clone() }
    }

    fn stroke_style(&self, stroke: &Stroke) -> Option<ID2D1StrokeStyle> {
        let cap = match stroke.cap {
            LineCap::Butt => D2D1_CAP_STYLE_FLAT,
            LineCap::Round => D2D1_CAP_STYLE_ROUND,
            LineCap::Square => D2D1_CAP_STYLE_SQUARE,
        };
        let join = match stroke.join {
            LineJoin::Miter => D2D1_LINE_JOIN_MITER,
            LineJoin::Round => D2D1_LINE_JOIN_ROUND,
            LineJoin::Bevel => D2D1_LINE_JOIN_BEVEL,
        };
        let dash: Vec<f32> = stroke.dash.clone().unwrap_or_default();
        if cap == D2D1_CAP_STYLE_FLAT && join == D2D1_LINE_JOIN_MITER && dash.is_empty() {
            return None; // o padrão do Direct2D
        }
        // No Direct2D os traços do tracejado são múltiplos da espessura.
        let width = stroke.width.max(0.01);
        let dashes: Vec<f32> = dash.iter().map(|d| d / width).collect();
        let key: StrokeKey = (cap.0, join.0, dashes.iter().map(|d| d.to_bits()).collect());
        if let Some(s) = self.strokes.borrow().get(&key) {
            return Some(s.clone());
        }
        let props = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: cap,
            endCap: cap,
            dashCap: cap,
            lineJoin: join,
            miterLimit: 10.0,
            dashStyle: if dashes.is_empty() { D2D1_DASH_STYLE_SOLID } else { D2D1_DASH_STYLE_CUSTOM },
            dashOffset: 0.0,
        };
        let style = unsafe { self.factory.CreateStrokeStyle(&props, (!dashes.is_empty()).then_some(&dashes[..])).ok()? };
        self.strokes.borrow_mut().insert(key, style.clone());
        Some(style)
    }

    /// Decodifica para BGRA pré-multiplicado. Guarda as imagens de arquivo (PNG, JPEG...) até
    /// o limite do projeto; as que já vêm em pixels (quadros de vídeo, do 3D) não entram:
    /// ficam na placa ([`DeviceCache::frames`]).
    fn decode(&self, image: &Image) -> Option<Decoded> {
        let raw = !matches!(image.source(), ImageSource::Encoded(_));
        if let Some(d) = self.decoded.borrow_mut().get(image) {
            return d.clone();
        }
        let result = match image.source() {
            ImageSource::Encoded(bytes) => {
                let _profile = aylis_core::profile::scope(aylis_core::profile::Lane::Resource, || format!("imagem ({} KB)", bytes.len() / 1024));
                decode_with_wic(&self.wic, bytes).map(|(p, w, h)| (w, h, premultiply_rgba_to_bgra(&p)))
            }
            ImageSource::Rgba { width, height, pixels } => Some((*width, *height, premultiply_rgba_to_bgra(pixels))),
            ImageSource::Bgra { width, height, pixels } => Some((*width, *height, premultiply_bgra(pixels))),
        }
        .map(Rc::new);
        if !raw {
            let files = aylis_core::image_cache::limits().files as usize;
            let bytes = result.as_ref().map_or(0, |d| d.2.len());
            self.decoded.borrow_mut().insert(image, result.clone(), bytes, files, usize::MAX);
        }
        result
    }

    /// Cria o alvo de desenho de uma janela. `vsync = false` apresenta sem esperar o
    /// monitor (medições). Tenta o Direct3D 11 com swap chain; se não der (ou com
    /// `AYLIS_D2D_TARGET=hwnd`), usa o `ID2D1HwndRenderTarget`.
    pub fn hwnd_target(&self, hwnd: isize, width: u32, height: u32, vsync: bool) -> Result<HwndTarget, Error> {
        if diagnostic("AYLIS_D2D_TARGET").is_none_or(|v| v != "hwnd") {
            let swap = self.swap_chain(hwnd, width, height);
            if diagnostic("AYLIS_D2D_DEBUG").is_some() {
                aylis_core::stderr_line!("aylis-d2d: alvo {}", match &swap { Ok(_) => "swap chain".to_string(), Err(e) => format!("da janela ({e})") });
            }
            if let Ok(swap) = swap {
                return Ok(HwndTarget { kind: Kind::Swap(Box::new(swap)), hwnd, size: (width, height), vsync, cache: DeviceCache::default() });
            }
        }
        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
            dpiX: 96.0,
            dpiY: 96.0,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };
        let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: HWND(hwnd as *mut _),
            pixelSize: D2D_SIZE_U { width: width.max(1), height: height.max(1) },
            presentOptions: if vsync { D2D1_PRESENT_OPTIONS_NONE } else { D2D1_PRESENT_OPTIONS_IMMEDIATELY },
        };
        let target = unsafe { self.factory.CreateHwndRenderTarget(&props, &hwnd_props)? };
        Ok(HwndTarget { kind: Kind::Hwnd(target), hwnd, size: (width, height), vsync, cache: DeviceCache::default() })
    }

    /// Direct3D 11 + swap chain "flip" + contexto do Direct2D numa janela.
    fn swap_chain(&self, hwnd: isize, width: u32, height: u32) -> Result<Swap, Error> {
        unsafe {
            let mut device = None;
            let mut context = None;
            let chosen = chosen_adapter();
            D3D11CreateDevice(
                chosen.as_ref(),
                if chosen.is_some() { D3D_DRIVER_TYPE_UNKNOWN } else { D3D_DRIVER_TYPE_HARDWARE },
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&[D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_10_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;
            let device: ID3D11Device = device.ok_or_else(|| Error::from(windows::Win32::Foundation::E_FAIL))?;
            let context: ID3D11DeviceContext = context.ok_or_else(|| Error::from(windows::Win32::Foundation::E_FAIL))?;
            let dxgi: IDXGIDevice = device.cast()?;
            let dc = self.factory1.CreateDevice(&dxgi)?.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?;
            let adapter = dxgi.GetAdapter()?;
            let adapter_desc = adapter.GetDesc()?;
            let luid = adapter_desc.AdapterLuid;
            let factory: IDXGIFactory2 = adapter.GetParent()?;
            let compose = aylis_core::gpu::present() == aylis_core::gpu::PresentMode::Composition;
            // Tearing só na swap chain da janela: a composição sempre espera o DWM.
            let tearing = !compose
                && factory.cast::<IDXGIFactory5>().is_ok_and(|f| {
                    let mut allow = 0i32;
                    f.CheckFeatureSupport(DXGI_FEATURE_PRESENT_ALLOW_TEARING, &mut allow as *mut i32 as *mut _, 4).is_ok() && allow != 0
                });
            let flags = if tearing { DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32 } else { 0 };
            let desc = DXGI_SWAP_CHAIN_DESC1 {
                Width: width.max(1),
                Height: height.max(1),
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                Stereo: false.into(),
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                // Leitura também: o desfoque do fundo (vidro) copia o alvo.
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT | DXGI_USAGE_SHADER_INPUT,
                BufferCount: 2,
                Scaling: DXGI_SCALING_STRETCH,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                AlphaMode: DXGI_ALPHA_MODE_IGNORE,
                Flags: flags,
            };
            // DirectComposition: a swap chain sem janela vira o conteúdo de uma camada que o DWM
            // junta à janela (como os navegadores). Se não der, a swap chain da janela.
            let composed = if compose { compose_into(&factory, &device, &dxgi, hwnd, &desc).ok() } else { None };
            let (swap, composition) = match composed {
                Some((swap, layer)) => (swap, Some(layer)),
                None => {
                    let swap = factory.CreateSwapChainForHwnd(&device, HWND(hwnd as *mut _), &desc, None, None)?;
                    let _ = factory.MakeWindowAssociation(HWND(hwnd as *mut _), DXGI_MWA_NO_ALT_ENTER);
                    (swap, None)
                }
            };
            let mut s = Swap {
                dc,
                swap,
                composition,
                flags,
                target: None,
                frame: None,
                fresh: true,
                link: GpuLink {
                    device: device.cast()?,
                    device5: device.cast().ok(),
                    context4: context.cast().ok(),
                    luid: ((luid.HighPart as i64) << 32) | luid.LowPart as i64,
                    adapter: adapter_name(&adapter_desc.Description),
                    surfaces: HashMap::new(),
                    fences: HashMap::new(),
                    signals: Vec::new(),
                    frame: 0,
                },
            };
            s.attach()?;
            Ok(s)
        }
    }

    /// Desenha fora da tela (na CPU, num bitmap do WIC) e devolve os pixels RGBA.
    pub fn render_rgba(&self, width: u32, height: u32, scale: f32, background: Color, paint: impl FnOnce(&mut D2dCanvas<'_>)) -> Option<Vec<u8>> {
        self.render_area_rgba(width, height, scale, background, None, paint)
    }

    /// Como [`render_rgba`](Self::render_rgba), só na área `area` (unidades lógicas) por cima
    /// da imagem `previous` (RGBA opaco, do mesmo tamanho): o mesmo caminho da janela quando
    /// repinta por parte, para os testes compararem com a pintura inteira.
    pub fn render_area_rgba(
        &self,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        over: Option<(&[u8], RectF)>,
        paint: impl FnOnce(&mut D2dCanvas<'_>),
    ) -> Option<Vec<u8>> {
        unsafe {
            let bitmap = match over {
                Some((previous, _)) if previous.len() == (width * height * 4) as usize => {
                    let bgra: Vec<u8> = previous.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], 255]).collect();
                    self.wic.CreateBitmapFromMemory(width, height, &GUID_WICPixelFormat32bppPBGRA, width * 4, &bgra).ok()?
                }
                _ => self.wic.CreateBitmap(width.max(1), height.max(1), &GUID_WICPixelFormat32bppPBGRA, WICBitmapCacheOnLoad).ok()?,
            };
            let props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                dpiX: 96.0,
                dpiY: 96.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };
            let target = self.factory.CreateWicBitmapRenderTarget(&bitmap, &props).ok()?;
            let mut cache = DeviceCache::default();
            self.draw_on(&target, &mut cache, None, scale, background, over.map(|(_, area)| area), paint);
            target.EndDraw(None, None).ok()?;
            let stride = width * 4;
            let mut bgra = vec![0u8; (stride * height) as usize];
            bitmap.CopyPixels(std::ptr::null(), stride, &mut bgra).ok()?;
            Some(unpremultiply_bgra_to_rgba(&bgra))
        }
    }

    /// Como [`render_area_rgba`](Self::render_area_rgba), com as cópias da rolagem (feitas
    /// aqui na CPU, sobre `previous`) e várias áreas pintadas de uma vez (`paint` recebe as
    /// áreas): para os testes conferirem o recorte de várias áreas.
    #[allow(clippy::too_many_arguments)]
    pub fn render_parts_rgba(
        &self,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        previous: &[u8],
        moves: &[aylis_core::ScrollMove],
        areas: &[RectF],
        paint: impl FnOnce(&mut D2dCanvas<'_>, &[RectF]),
    ) -> Option<Vec<u8>> {
        if previous.len() != (width * height * 4) as usize {
            return None;
        }
        let mut bgra: Vec<u8> = previous.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], 255]).collect();
        let stride = width as usize * 4;
        for m in moves {
            let px = |v: f32| (v * scale).round() as i64;
            let (l, t, r, b) = (px(m.rect.x), px(m.rect.y), px(m.rect.right()), px(m.rect.bottom()));
            let (dx, dy) = (px(m.delta.x), px(m.delta.y));
            let rows: Vec<i64> = if dy > 0 { (t..b).rev().collect() } else { (t..b).collect() };
            for y in rows {
                let (src, dst) = (y as usize * stride + l as usize * 4, (y + dy) as usize * stride + (l + dx) as usize * 4);
                bgra.copy_within(src..src + (r - l) as usize * 4, dst);
            }
        }
        if areas.is_empty() {
            return None;
        }
        unsafe {
            let bitmap = self.wic.CreateBitmapFromMemory(width, height, &GUID_WICPixelFormat32bppPBGRA, width * 4, &bgra).ok()?;
            let props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                dpiX: 96.0,
                dpiY: 96.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };
            let target = self.factory.CreateWicBitmapRenderTarget(&bitmap, &props).ok()?;
            let mut cache = DeviceCache::default();
            self.draw_on_areas(&target, &mut cache, None, scale, background, areas, |c| paint(c, areas));
            target.EndDraw(None, None).ok()?;
            let mut out = vec![0u8; stride * height as usize];
            bitmap.CopyPixels(std::ptr::null(), width * 4, &mut out).ok()?;
            Some(unpremultiply_bgra_to_rgba(&out))
        }
    }

    /// BeginDraw, escala, fundo e a pintura (o EndDraw fica com quem chamou).
    fn draw_on(
        &self,
        target: &ID2D1RenderTarget,
        cache: &mut DeviceCache,
        gpu: Option<&mut GpuLink>,
        scale: f32,
        background: Color,
        area: Option<RectF>,
        paint: impl FnOnce(&mut D2dCanvas<'_>),
    ) {
        let list: Vec<RectF> = area.into_iter().collect();
        self.draw_on_areas(target, cache, gpu, scale, background, &list, paint);
    }

    /// Como `draw_on`, com várias áreas (vazio: o alvo inteiro) pintadas de uma vez: o
    /// recorte é a união delas.
    #[allow(clippy::too_many_arguments)]
    fn draw_on_areas(
        &self,
        target: &ID2D1RenderTarget,
        cache: &mut DeviceCache,
        gpu: Option<&mut GpuLink>,
        scale: f32,
        background: Color,
        areas: &[RectF],
        paint: impl FnOnce(&mut D2dCanvas<'_>),
    ) {
        cache.forget_dropped();
        self.decoded.borrow_mut().forget_dropped();
        unsafe {
            target.BeginDraw();
            target.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
            target.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
            // Só as áreas (em pixels inteiros): o resto do alvo fica com o quadro anterior. O
            // recorte entra como o primeiro da pilha do canvas, que o reabre quando precisa
            // fechar tudo (o desfoque do fundo copia o alvo).
            let mut pushes = Vec::new();
            let pixels = |a: &RectF| D2D_RECT_F {
                left: (a.x * scale).floor(),
                top: (a.y * scale).floor(),
                right: (a.right() * scale).ceil(),
                bottom: (a.bottom() * scale).ceil(),
            };
            target.SetTransform(&Matrix3x2::identity());
            if let [a] = areas {
                let r = pixels(a);
                target.PushAxisAlignedClip(&r, D2D1_ANTIALIAS_MODE_ALIASED);
                pushes.push(Push::Clip(r, Matrix3x2::identity()));
            } else if areas.len() > 1 {
                // Várias: uma camada com a máscara da união (os retângulos caem nos pixels
                // inteiros, então a borda da máscara não suaviza nada).
                let rects: Vec<Option<ID2D1Geometry>> = areas.iter().map(|a| self.factory.CreateRectangleGeometry(&pixels(a)).ok().and_then(|g| g.cast::<ID2D1Geometry>().ok())).collect();
                let group = self.factory.CreateGeometryGroup(D2D1_FILL_MODE_WINDING, &rects).ok().and_then(|g| g.cast::<ID2D1Geometry>().ok());
                let params = D2D1_LAYER_PARAMETERS {
                    contentBounds: INFINITE,
                    geometricMask: std::mem::ManuallyDrop::new(group.clone()),
                    maskAntialiasMode: D2D1_ANTIALIAS_MODE_ALIASED,
                    maskTransform: Matrix3x2::identity(),
                    opacity: 1.0,
                    opacityBrush: std::mem::ManuallyDrop::new(None),
                    layerOptions: D2D1_LAYER_OPTIONS_NONE,
                };
                target.PushLayer(&params, None::<&ID2D1Layer>);
                let mut params = params;
                std::mem::ManuallyDrop::drop(&mut params.geometricMask);
                std::mem::ManuallyDrop::drop(&mut params.opacityBrush);
                pushes.push(Push::Layer(group, 1.0, Matrix3x2::identity()));
            }

            let base = Matrix3x2::scale(scale, scale);
            target.SetTransform(&base);
            // O `Clear` não respeita a máscara de uma camada: com várias áreas, o fundo é
            // pintado em cada uma.
            if areas.len() <= 1 {
                target.Clear(Some(&color(background)));
            }
            let Ok(brush) = target.CreateSolidColorBrush(&color(Color::BLACK), None) else {
                match areas.len() {
                    0 => {}
                    1 => target.PopAxisAlignedClip(),
                    _ => target.PopLayer(),
                }
                return;
            };
            if areas.len() > 1 {
                brush.SetColor(&color(background));
                target.SetTransform(&Matrix3x2::identity());
                for a in areas {
                    target.FillRectangle(&pixels(a), &brush);
                }
                target.SetTransform(&base);
            }
            let mut canvas = D2dCanvas {
                target: target.clone(),
                masks: Vec::new(),
                cached: Vec::new(),
                renderer: self,
                brush,
                transform: base,
                frames: Vec::new(),
                pushes,
                cache,
                gpu,
            };
            paint(&mut canvas);
            canvas.unwind();
        }
    }
}

/// O Direct3D 11 por baixo do contexto do Direct2D: abre as texturas e cercas
/// compartilhadas de quem desenha por fora (o 3D) para compor sem passar pela CPU.
pub struct GpuLink {
    device: ID3D11Device1,
    device5: Option<ID3D11Device5>,
    context4: Option<ID3D11DeviceContext4>,
    luid: i64,
    /// O nome da placa de vídeo.
    adapter: String,
    /// Texturas abertas (pelo id), com o último quadro em que foram usadas.
    surfaces: HashMap<u64, (ID2D1Bitmap1, u64)>,
    fences: HashMap<isize, ID3D11Fence>,
    /// Cercas a marcar quando o quadro terminar (a textura foi lida).
    signals: Vec<(ID3D11Fence, u64)>,
    frame: u64,
}

impl GpuLink {
    fn fence(&mut self, handle: isize) -> Option<ID3D11Fence> {
        if let Some(f) = self.fences.get(&handle) {
            return Some(f.clone());
        }
        let mut f: Option<ID3D11Fence> = None;
        unsafe { self.device5.as_ref()?.OpenSharedFence(HANDLE(handle as *mut _), &mut f).ok()? };
        let f = f?;
        self.fences.insert(handle, f.clone());
        Some(f)
    }

    /// Fim do quadro: marca as cercas das texturas lidas e esquece as que sumiram.
    fn finish(&mut self) {
        if let Some(ctx) = &self.context4 {
            for (fence, value) in self.signals.drain(..) {
                let _ = unsafe { ctx.Signal(&fence, value) };
            }
        }
        let frame = self.frame;
        self.surfaces.retain(|_, (_, used)| *used + 2 >= frame);
        self.frame += 1;
    }
}

/// Alvo por swap chain.
struct Swap {
    dc: ID2D1DeviceContext,
    swap: IDXGISwapChain1,
    /// No modo `composition`, a camada do DirectComposition que mostra a swap chain na janela
    /// (guardada para continuar viva enquanto a janela desenha).
    #[allow(dead_code)]
    composition: Option<Composition>,
    flags: u32,
    /// O buffer de trás da swap chain.
    target: Option<ID2D1Bitmap1>,
    /// O quadro guardado: a pintura vai para ele, por parte quando dá, e ele é copiado
    /// inteiro para o buffer de trás (no modelo flip, o buffer de trás não guarda o quadro
    /// anterior). `fresh`: acabou de ser criado, sem nada pintado.
    frame: Option<ID2D1Bitmap1>,
    fresh: bool,
    link: GpuLink,
}

/// O dispositivo de composição, o alvo ligado à janela e a camada com a swap chain.
struct Composition {
    _device: windows::Win32::Graphics::DirectComposition::IDCompositionDevice,
    _target: windows::Win32::Graphics::DirectComposition::IDCompositionTarget,
    _visual: windows::Win32::Graphics::DirectComposition::IDCompositionVisual,
}

/// Cria a swap chain para composição e a põe como a camada da janela `hwnd`.
unsafe fn compose_into(
    factory: &IDXGIFactory2,
    device: &ID3D11Device,
    dxgi: &IDXGIDevice,
    hwnd: isize,
    desc: &DXGI_SWAP_CHAIN_DESC1,
) -> Result<(IDXGISwapChain1, Composition), Error> {
    use windows::Win32::Graphics::DirectComposition::*;
    // SAFETY: chamadas COM com objetos vivos; a janela existe.
    unsafe {
        let swap = factory.CreateSwapChainForComposition(device, desc, None)?;
        let dcomp: IDCompositionDevice = DCompositionCreateDevice(dxgi)?;
        let target = dcomp.CreateTargetForHwnd(HWND(hwnd as *mut _), true)?;
        let visual = dcomp.CreateVisual()?;
        visual.SetContent(&swap)?;
        target.SetRoot(&visual)?;
        dcomp.Commit()?;
        Ok((swap, Composition { _device: dcomp, _target: target, _visual: visual }))
    }
}

impl Swap {
    /// Liga o buffer de trás da swap chain como alvo do contexto.
    fn attach(&mut self) -> Result<(), Error> {
        unsafe {
            let surface: IDXGISurface = self.swap.GetBuffer(0)?;
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                dpiX: 96.0,
                dpiY: 96.0,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET,
                colorContext: std::mem::ManuallyDrop::new(None),
            };
            let bitmap = self.dc.CreateBitmapFromDxgiSurface(&surface, Some(&props))?;
            let frame = self.dc.CreateBitmap(bitmap.GetPixelSize(), None, 0, &props)?;
            self.dc.SetTarget(&frame);
            self.target = Some(bitmap);
            self.frame = Some(frame);
            self.fresh = true;
        }
        Ok(())
    }

    /// Copia, no quadro guardado, os pixels que andaram (a rolagem; unidades lógicas que caem
    /// em pixels inteiros). Por um bitmap de passagem: origem e destino se sobrepõem. Fora do
    /// `BeginDraw`.
    fn copy_moves(&self, scale: f32, moves: &[aylis_core::ScrollMove]) -> windows::core::Result<()> {
        let Some(frame) = &self.frame else { return Ok(()) };
        let size = unsafe { frame.GetPixelSize() };
        for m in moves {
            let px = |v: f32| (v * scale).round() as i64;
            let r = m.rect;
            let (l, t) = (px(r.x).max(0), px(r.y).max(0));
            let (right, bottom) = (px(r.right()).min(size.width as i64), px(r.bottom()).min(size.height as i64));
            let (dx, dy) = (px(m.delta.x), px(m.delta.y));
            if right <= l || bottom <= t || l + dx < 0 || t + dy < 0 || right + dx > size.width as i64 || bottom + dy > size.height as i64 {
                continue;
            }
            let (w, h) = ((right - l) as u32, (bottom - t) as u32);
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                dpiX: 96.0,
                dpiY: 96.0,
                bitmapOptions: D2D1_BITMAP_OPTIONS_NONE,
                colorContext: std::mem::ManuallyDrop::new(None),
            };
            unsafe {
                let temp = self.dc.CreateBitmap(D2D_SIZE_U { width: w, height: h }, None, 0, &props)?;
                let src = D2D_RECT_U { left: l as u32, top: t as u32, right: right as u32, bottom: bottom as u32 };
                temp.CopyFromBitmap(Some(&D2D_POINT_2U { x: 0, y: 0 }), frame, Some(&src))?;
                let at = D2D_POINT_2U { x: (l + dx) as u32, y: (t + dy) as u32 };
                frame.CopyFromBitmap(Some(&at), &temp, Some(&D2D_RECT_U { left: 0, top: 0, right: w, bottom: h }))?;
            }
        }
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        unsafe {
            self.dc.SetTarget(None);
            self.target = None;
            self.frame = None;
            self.swap.ResizeBuffers(0, width.max(1), height.max(1), DXGI_FORMAT_UNKNOWN, DXGI_SWAP_CHAIN_FLAG(self.flags as i32))?;
        }
        self.attach()
    }
}

enum Kind {
    Hwnd(ID2D1HwndRenderTarget),
    Swap(Box<Swap>),
}

/// Recursos da GPU já criados, que valem enquanto o alvo existir.
pub struct DeviceCache {
    /// Imagens de arquivo já enviadas à GPU (até o limite `files` do projeto).
    bitmaps: ImageCache<ID2D1Bitmap>,
    /// Pixels crus já enviados (quadros de vídeo, prévias, blocos de uma pintura), com o
    /// tamanho: repintar sem imagem nova não reenvia à placa. Até o limite `pixels_mb`.
    frames: ImageCache<(ID2D1Bitmap, u32, u32)>,
    /// Paradas de gradiente (criá-las a cada quadro custa caro, ainda mais no contexto
    /// do Direct2D 1.1), pelas posições e cores.
    gradients: RefCell<HashMap<Vec<u64>, ID2D1GradientStopCollection>>,
    /// Sombras já desfocadas (tamanho, cantos, desfoque e cor em pixels), com a margem.
    shadows: RefCell<HashMap<[u32; 5], (ID2D1Bitmap, f32)>>,
    /// Gradientes cônicos já pintados (o Direct2D não tem esse gradiente).
    sweeps: RefCell<HashMap<Vec<u64>, ID2D1Bitmap>>,
    /// Desenhos guardados como imagem ([`Canvas::begin_cached`]), pela chave e pelo tamanho.
    layers: HashMap<u64, ID2D1Bitmap>,
}

impl Default for DeviceCache {
    fn default() -> Self {
        Self {
            bitmaps: ImageCache::named("Direct2D · arquivos na placa"),
            frames: ImageCache::named("Direct2D · pixels na placa"),
            gradients: RefCell::default(),
            shadows: RefCell::default(),
            sweeps: RefCell::default(),
            layers: HashMap::new(),
        }
    }
}

impl DeviceCache {
    /// Solta os bitmaps das imagens que o app já soltou: ninguém mais pede aquele id (uma
    /// prévia trocada a cada meio segundo enchia a placa até o teto).
    fn forget_dropped(&mut self) {
        self.frames.forget_dropped();
        self.bitmaps.forget_dropped();
    }
}

/// O alvo de desenho de uma janela.
pub struct HwndTarget {
    kind: Kind,
    hwnd: isize,
    size: (u32, u32),
    vsync: bool,
    /// Bitmaps já enviados à GPU (são do alvo: refeitos se ele for recriado).
    cache: DeviceCache,
}

impl HwndTarget {
    /// A placa de vídeo em que desenha (no alvo antigo, `ID2D1HwndRenderTarget`, não se sabe).
    pub fn adapter(&self) -> Option<&str> {
        match &self.kind {
            Kind::Swap(s) => Some(&s.link.adapter),
            _ => None,
        }
    }

    /// Desenha um quadro. Com `area` (unidades lógicas), pode pintar só ela por cima do quadro
    /// anterior: `paint` recebe a área que valeu (`None`: a janela inteira, no alvo antigo ou
    /// sem quadro guardado). Se o dispositivo foi perdido (driver, acesso remoto), recria o
    /// alvo e pede outro quadro (`Ok(false)`).
    pub fn draw(
        &mut self,
        renderer: &D2dRenderer,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        area: Option<RectF>,
        paint: impl FnOnce(&mut D2dCanvas<'_>, Option<RectF>),
    ) -> Result<bool, Error> {
        let list: Vec<RectF> = area.into_iter().collect();
        self.draw_parts(renderer, width, height, scale, background, area.map(|_| (&[][..], &list[..])), |c, a| paint(c, a.and_then(|a| a.first().copied())))
    }

    /// Como [`draw`](Self::draw), com as cópias da rolagem (feitas no quadro guardado) e várias
    /// áreas pintadas de uma vez: `paint` recebe as áreas (`None`: a
    /// janela inteira, no alvo antigo ou sem quadro guardado).
    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    pub fn draw_parts(
        &mut self,
        renderer: &D2dRenderer,
        width: u32,
        height: u32,
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[RectF])>,
        paint: impl FnOnce(&mut D2dCanvas<'_>, Option<&[RectF]>),
    ) -> Result<bool, Error> {
        let resized = self.size != (width, height);
        self.size = (width, height);
        let result = match &mut self.kind {
            Kind::Hwnd(target) => {
                if resized {
                    unsafe { target.Resize(&D2D_SIZE_U { width: width.max(1), height: height.max(1) })? };
                }
                let target: ID2D1RenderTarget = target.cast()?;
                renderer.draw_on(&target, &mut self.cache, None, scale, background, None, |c| paint(c, None));
                unsafe { target.EndDraw(None, None) }
            }
            Kind::Swap(s) => {
                if resized {
                    s.resize(width, height)?;
                }
                let mut parts = parts.filter(|_| !s.fresh);
                // As cópias antes do `BeginDraw`; se uma falhar, o quadro é inteiro.
                if let Some((moves, _)) = parts {
                    if !moves.is_empty() && s.copy_moves(scale, moves).is_err() {
                        parts = None;
                    }
                }
                let areas = parts.map_or(&[][..], |(_, a)| a);
                let given = (!areas.is_empty()).then_some(areas);
                let target: ID2D1RenderTarget = s.dc.cast()?;
                renderer.draw_on_areas(&target, &mut self.cache, Some(&mut s.link), scale, background, areas, |c| paint(c, given));
                let end = unsafe { target.EndDraw(None, None) };
                s.link.finish();
                // O quadro guardado vai inteiro para o buffer de trás (uma cópia na GPU).
                let end = end.and_then(|_| match (&s.target, &s.frame) {
                    (Some(back), Some(frame)) => unsafe { back.CopyFromBitmap(None, frame, None) },
                    _ => Ok(()),
                });
                s.fresh = false;
                end.and_then(|_| {
                    let (interval, flags) = match (self.vsync, s.flags != 0) {
                        (true, _) => (1, DXGI_PRESENT(0)),
                        (false, true) => (0, DXGI_PRESENT_ALLOW_TEARING),
                        (false, false) => (0, DXGI_PRESENT(0)),
                    };
                    unsafe { s.swap.Present(interval, flags) }.ok()
                })
            }
        };
        match result {
            Ok(()) => Ok(true),
            Err(e) if e.code() == D2DERR_RECREATE_TARGET || e.code() == DXGI_ERROR_DEVICE_REMOVED || e.code() == DXGI_ERROR_DEVICE_RESET => {
                *self = renderer.hwnd_target(self.hwnd, width, height, self.vsync)?;
                Ok(false)
            }
            Err(e) => Err(e),
        }
    }
}

// ---------------------------------------------------------------------------
// canvas

/// Um recorte ou camada aberta, com o que é preciso para reabrir (o desfoque do fundo
/// fecha tudo para copiar o alvo e reabre depois).
enum Push {
    Clip(D2D_RECT_F, Matrix3x2),
    Layer(Option<ID2D1Geometry>, f32, Matrix3x2),
}

struct Frame {
    transform: Matrix3x2,
    pushes: usize,
}

/// O [`Canvas`] do aylis sobre um alvo do Direct2D.
/// Uma máscara aberta: o alvo de antes, as camadas dele e o conteúdo já desenhado.
/// Um desenho guardado sendo feito ([`Canvas::begin_cached`]): o alvo de antes e onde
/// compor o resultado (em pixels).
struct CacheState {
    main: ID2D1RenderTarget,
    pushes: Vec<Push>,
    transform: Matrix3x2,
    content: ID2D1BitmapRenderTarget,
    key: u64,
    dest: D2D_RECT_F,
}

struct MaskState {
    main: ID2D1RenderTarget,
    pushes: Vec<Push>,
    content: ID2D1BitmapRenderTarget,
    mask: Option<ID2D1BitmapRenderTarget>,
}

pub struct D2dCanvas<'a> {
    /// O alvo atual (o da janela, ou um fora da tela enquanto desenha uma máscara).
    target: ID2D1RenderTarget,
    masks: Vec<MaskState>,
    /// Os desenhos guardados abertos (`None`: não deu para guardar, desenha direto).
    cached: Vec<Option<CacheState>>,
    renderer: &'a D2dRenderer,
    /// Um pincel sólido reaproveitado (só troca a cor).
    brush: ID2D1SolidColorBrush,
    transform: Matrix3x2,
    frames: Vec<Frame>,
    pushes: Vec<Push>,
    cache: &'a mut DeviceCache,
    /// O Direct3D por baixo (só no alvo por swap chain), para compor texturas de fora.
    gpu: Option<&'a mut GpuLink>,
}

impl D2dCanvas<'_> {
    fn set_transform(&mut self, m: Matrix3x2) {
        self.transform = m;
        unsafe { self.target.SetTransform(&self.transform) };
    }

    /// Aplica uma transformação local (antes da atual, como no Skia).
    fn concat(&mut self, local: Matrix3x2) {
        let m = mul(&local, &self.transform);
        self.set_transform(m);
    }

    fn pop_until(&mut self, count: usize) {
        while self.pushes.len() > count {
            match self.pushes.pop() {
                Some(Push::Clip(..)) => unsafe { self.target.PopAxisAlignedClip() },
                Some(Push::Layer(..)) => unsafe { self.target.PopLayer() },
                None => break,
            }
        }
    }

    /// Compõe a máscara aberta no alvo de antes: o conteúdo com o alfa da máscara.
    fn end_mask_inner(&mut self) {
        let Some(state) = self.masks.pop() else { return };
        self.pop_until(0);
        unsafe {
            match &state.mask {
                Some(mask) => {
                    let _ = mask.EndDraw(None, None);
                }
                None => {
                    let _ = state.content.EndDraw(None, None);
                }
            }
        }
        self.target = state.main;
        self.pushes = state.pushes;
        unsafe {
            let (Ok(content), Some(mask)) = (state.content.GetBitmap(), state.mask.as_ref().and_then(|m| m.GetBitmap().ok())) else {
                return;
            };
            let size = self.target.GetSize();
            let full = D2D_RECT_F { left: 0.0, top: 0.0, right: size.width, bottom: size.height };
            // Os bitmaps estão no espaço da tela: compõe sem transformação.
            self.target.SetTransform(&Matrix3x2::identity());
            if let Ok(brush) = self.target.CreateBitmapBrush(&mask, None, None) {
                let params = D2D1_LAYER_PARAMETERS {
                    contentBounds: full,
                    geometricMask: std::mem::ManuallyDrop::new(None),
                    maskAntialiasMode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                    maskTransform: Matrix3x2::identity(),
                    opacity: 1.0,
                    opacityBrush: std::mem::ManuallyDrop::new(brush.cast::<ID2D1Brush>().ok()),
                    layerOptions: D2D1_LAYER_OPTIONS_NONE,
                };
                self.target.PushLayer(&params, None::<&ID2D1Layer>);
                let mut params = params;
                std::mem::ManuallyDrop::drop(&mut params.opacityBrush);
                self.target.DrawBitmap(&content, Some(&full), 1.0, D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, None);
                self.target.PopLayer();
            }
            self.target.SetTransform(&self.transform);
        }
    }

    /// Um alvo fora da tela do tamanho do atual, limpo e com a mesma transformação.
    fn offscreen(&self) -> Option<ID2D1BitmapRenderTarget> {
        unsafe {
            let rt = self.target.CreateCompatibleRenderTarget(None, None, None, D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE).ok()?;
            rt.BeginDraw();
            rt.Clear(Some(&color(Color::TRANSPARENT)));
            rt.SetTransform(&self.transform);
            Some(rt)
        }
    }

    /// Compõe um desenho guardado em `dest` (pixels do alvo).
    fn draw_layer(&mut self, bitmap: &ID2D1Bitmap, dest: &D2D_RECT_F) {
        unsafe {
            self.target.SetTransform(&Matrix3x2::identity());
            self.target.DrawBitmap(bitmap, Some(dest), 1.0, D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, None);
            self.target.SetTransform(&self.transform);
        }
    }

    fn end_cached_inner(&mut self) {
        let Some(state) = self.cached.pop().flatten() else { return };
        self.pop_until(0);
        unsafe {
            let _ = state.content.EndDraw(None, None);
        }
        self.target = state.main;
        self.pushes = state.pushes;
        self.set_transform(state.transform);
        let Ok(bitmap) = (unsafe { state.content.GetBitmap() }) else { return };
        // Poucos guardados por vez: cada destaque novo de um mapa é uma imagem nova.
        const KEEP: usize = 16;
        if self.cache.layers.len() >= KEEP {
            self.cache.layers.clear();
        }
        self.cache.layers.insert(state.key, bitmap.clone());
        self.draw_layer(&bitmap, &state.dest);
    }

    /// Fecha clips e camadas que a pintura deixou abertos (o EndDraw exige).
    fn unwind(&mut self) {
        while !self.masks.is_empty() {
            self.end_mask_inner();
        }
        while !self.cached.is_empty() {
            self.end_cached_inner();
        }
        self.pop_until(0);
        self.frames.clear();
    }

    fn push_layer(&mut self, mask: Option<ID2D1Geometry>, opacity: f32) {
        self.push_layer_raw(mask.clone(), opacity);
        self.pushes.push(Push::Layer(mask, opacity, self.transform));
    }

    fn push_layer_raw(&self, mask: Option<ID2D1Geometry>, opacity: f32) {
        let params = D2D1_LAYER_PARAMETERS {
            contentBounds: INFINITE,
            geometricMask: std::mem::ManuallyDrop::new(mask),
            maskAntialiasMode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
            maskTransform: Matrix3x2::identity(),
            opacity,
            opacityBrush: std::mem::ManuallyDrop::new(None),
            layerOptions: D2D1_LAYER_OPTIONS_NONE,
        };
        unsafe { self.target.PushLayer(&params, None::<&ID2D1Layer>) };
        let mut params = params;
        unsafe {
            std::mem::ManuallyDrop::drop(&mut params.geometricMask);
            std::mem::ManuallyDrop::drop(&mut params.opacityBrush);
        }
    }

    /// Fecha os recortes e camadas abertos, roda `f` e reabre tudo igual (o Direct2D não
    /// copia o alvo com eles abertos).
    fn without_pushes<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let saved = std::mem::take(&mut self.pushes);
        for p in saved.iter().rev() {
            match p {
                Push::Clip(..) => unsafe { self.target.PopAxisAlignedClip() },
                Push::Layer(..) => unsafe { self.target.PopLayer() },
            }
        }
        let result = f(self);
        for p in &saved {
            match p {
                Push::Clip(r, m) => unsafe {
                    self.target.SetTransform(m);
                    self.target.PushAxisAlignedClip(r, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
                },
                Push::Layer(mask, opacity, m) => {
                    unsafe { self.target.SetTransform(m) };
                    self.push_layer_raw(mask.clone(), *opacity);
                }
            }
        }
        unsafe { self.target.SetTransform(&self.transform) };
        self.pushes = saved;
        result
    }

    /// O pincel de um `Brush` (o sólido é o reaproveitado).
    /// O pincel do Direct2D. `bounds`: a área da forma (o gradiente cônico é pintado nela).
    fn brush(&self, brush: &Brush, bounds: RectF) -> Option<ID2D1Brush> {
        unsafe {
            let stops = match brush {
                Brush::Solid(c) => {
                    self.brush.SetColor(&color(*c));
                    return self.brush.cast().ok();
                }
                Brush::Linear { stops, .. } | Brush::Radial { stops, .. } | Brush::Sweep { stops, .. } => stops,
            };
            if let Brush::Sweep { center, start_angle, .. } = brush {
                // Sem gradiente cônico no Direct2D: pinta a imagem dele na área da forma.
                if let Some(b) = self.sweep_brush(*center, *start_angle, stops, bounds) {
                    return Some(b);
                }
                self.brush.SetColor(&color(stops.first().map_or(Color::TRANSPARENT, |s| s.color)));
                return self.brush.cast().ok();
            }
            let key: Vec<u64> = stops
                .iter()
                .map(|s| ((s.offset.clamp(0.0, 1.0).to_bits() as u64) << 32) | u32::from_be_bytes([s.color.r, s.color.g, s.color.b, s.color.a]) as u64)
                .collect();
            let cached = self.cache.gradients.borrow().get(&key).cloned();
            let collection = match cached {
                Some(c) => c,
                None => {
                    let list: Vec<D2D1_GRADIENT_STOP> =
                        stops.iter().map(|s| D2D1_GRADIENT_STOP { position: s.offset.clamp(0.0, 1.0), color: color(s.color) }).collect();
                    let c = self.target.CreateGradientStopCollection(&list, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP).ok()?;
                    let mut cache = self.cache.gradients.borrow_mut();
                    // Gradientes que mudam sempre (animações de cor) não enchem a memória.
                    if cache.len() >= 512 {
                        cache.clear();
                    }
                    cache.insert(key, c.clone());
                    c
                }
            };
            match brush {
                Brush::Linear { start, end, .. } => {
                    let props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES { startPoint: v(*start), endPoint: v(*end) };
                    self.target.CreateLinearGradientBrush(&props, None, &collection).ok()?.cast().ok()
                }
                Brush::Radial { center, radius, .. } => {
                    let props = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                        center: v(*center),
                        gradientOriginOffset: Vector2 { X: 0.0, Y: 0.0 },
                        radiusX: *radius,
                        radiusY: *radius,
                    };
                    self.target.CreateRadialGradientBrush(&props, None, &collection).ok()?.cast().ok()
                }
                _ => None,
            }
        }
    }

    fn geometry(&self, path: &Path) -> Option<ID2D1PathGeometry> {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for c in &path.cmds {
            let (kind, pts): (u8, [PointF; 3]) = match *c {
                PathCmd::MoveTo(p) => (0, [p, p, p]),
                PathCmd::LineTo(p) => (1, [p, p, p]),
                PathCmd::QuadTo(a, p) => (2, [a, p, p]),
                PathCmd::CubicTo(a, b, p) => (3, [a, b, p]),
                PathCmd::Close => (4, [PointF::new(0.0, 0.0); 3]),
            };
            kind.hash(&mut h);
            path.even_odd.hash(&mut h);
            for p in pts {
                p.x.to_bits().hash(&mut h);
                p.y.to_bits().hash(&mut h);
            }
        }
        let key = h.finish();
        if let Some(g) = self.renderer.geometries.borrow().get(&key) {
            return Some(g.clone());
        }
        let g = self.build_geometry(path)?;
        let mut cache = self.renderer.geometries.borrow_mut();
        if cache.len() > 2048 {
            cache.clear();
        }
        cache.insert(key, g.clone());
        Some(g)
    }

    fn build_geometry(&self, path: &Path) -> Option<ID2D1PathGeometry> {
        unsafe {
            let geometry = self.renderer.factory.CreatePathGeometry().ok()?;
            let sink = geometry.Open().ok()?;
            sink.SetFillMode(if path.even_odd { D2D1_FILL_MODE_ALTERNATE } else { D2D1_FILL_MODE_WINDING });
            let mut open = false;
            let mut current = PointF::new(0.0, 0.0);
            let mut start = current;
            let begin = |sink: &ID2D1GeometrySink, open: &mut bool, at: PointF| {
                if !*open {
                    sink.BeginFigure(v(at), D2D1_FIGURE_BEGIN_FILLED);
                    *open = true;
                }
            };
            for c in &path.cmds {
                match *c {
                    PathCmd::MoveTo(p) => {
                        if open {
                            sink.EndFigure(D2D1_FIGURE_END_OPEN);
                            open = false;
                        }
                        current = p;
                        start = p;
                    }
                    PathCmd::LineTo(p) => {
                        begin(&sink, &mut open, current);
                        sink.AddLine(v(p));
                        current = p;
                    }
                    PathCmd::QuadTo(a, p) => {
                        begin(&sink, &mut open, current);
                        sink.AddQuadraticBezier(&D2D1_QUADRATIC_BEZIER_SEGMENT { point1: v(a), point2: v(p) });
                        current = p;
                    }
                    PathCmd::CubicTo(a, b, p) => {
                        begin(&sink, &mut open, current);
                        sink.AddBezier(&D2D1_BEZIER_SEGMENT { point1: v(a), point2: v(b), point3: v(p) });
                        current = p;
                    }
                    PathCmd::Close => {
                        if open {
                            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
                            open = false;
                        }
                        current = start;
                    }
                }
            }
            if open {
                sink.EndFigure(D2D1_FIGURE_END_OPEN);
            }
            sink.Close().ok()?;
            Some(geometry)
        }
    }

    fn rounded(r: RectF, radius: f32) -> D2D1_ROUNDED_RECT {
        let radius = radius.min(r.width / 2.0).min(r.height / 2.0).max(0.0);
        D2D1_ROUNDED_RECT { rect: rect(r), radiusX: radius, radiusY: radius }
    }

    fn bitmap(&mut self, image: &Image) -> Option<(ID2D1Bitmap, u32, u32)> {
        if let Some(found) = self.cache.frames.get(image) {
            return Some(found.clone());
        }
        // BGRA (vídeo, câmera) vai como está: opaco ou já pré-multiplicado, a ordem do
        // Direct2D — sem passar pixel a pixel.
        if let ImageSource::Bgra { width, height, pixels } = image.source() {
            let (w, h) = (*width, *height);
            let props = D2D1_BITMAP_PROPERTIES {
                pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                dpiX: 96.0,
                dpiY: 96.0,
            };
            let bitmap = unsafe {
                self.target
                    .CreateBitmap(D2D_SIZE_U { width: w, height: h }, Some(pixels.as_ptr() as *const _), w * 4, &props)
                    .ok()?
            };
            self.remember_frame(image, &bitmap, w, h);
            return Some((bitmap, w, h));
        }
        let decoded = self.renderer.decode(image)?;
        let (w, h, pixels) = (decoded.0, decoded.1, &decoded.2);
        if let Some(b) = self.cache.bitmaps.get(image) {
            return Some((b.clone(), w, h));
        }
        let props = D2D1_BITMAP_PROPERTIES {
            pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
            dpiX: 96.0,
            dpiY: 96.0,
        };
        let bitmap = unsafe {
            self.target
                .CreateBitmap(D2D_SIZE_U { width: w, height: h }, Some(pixels.as_ptr() as *const _), w * 4, &props)
                .ok()?
        };
        if matches!(image.source(), ImageSource::Encoded(_)) {
            let files = aylis_core::image_cache::limits().files as usize;
            self.cache.bitmaps.insert(image, bitmap.clone(), w as usize * h as usize * 4, files, usize::MAX);
        } else {
            self.remember_frame(image, &bitmap, w, h);
        }
        Some((bitmap, w, h))
    }

    /// Guarda os pixels crus enviados à placa, até o limite `pixels_mb` do projeto.
    fn remember_frame(&mut self, image: &Image, bitmap: &ID2D1Bitmap, w: u32, h: u32) {
        let max = aylis_core::image_cache::limits().pixels_bytes();
        self.cache.frames.insert(image, (bitmap.clone(), w, h), w as usize * h as usize * 4, usize::MAX, max);
    }

    fn draw_line_text(&self, line: &str, at: PointF, key: &FontKey, c: Color) {
        let Some((layout, _)) = self.renderer.text.layout(line, key) else { return };
        unsafe {
            self.brush.SetColor(&color(c));
            self.target.DrawTextLayout(v(at), &layout, &self.brush, D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT);
        }
    }
}

impl D2dCanvas<'_> {
    fn scale(&self) -> f32 {
        let m = &self.transform;
        (m.M11 * m.M22 - m.M12 * m.M21).abs().sqrt().max(0.01)
    }

    /// A forma desfocada, em pixels da tela: (imagem, margem em pixels, escala).
    fn shadow_image(&self, area: RectF, corner: f32, shadow: &Shadow) -> Option<(ID2D1Bitmap, f32, f32)> {
        let scale = self.scale();
        let (w, h) = ((area.width * scale).round(), (area.height * scale).round());
        if w < 1.0 || h < 1.0 || w > 4096.0 || h > 4096.0 {
            return None;
        }
        let sigma = shadow.blur / 2.0 * scale;
        let margin = (sigma * 3.0).ceil();
        let c = shadow.color;
        let key = [w as u32, h as u32, (corner * scale).to_bits(), sigma.to_bits(), u32::from_be_bytes([c.r, c.g, c.b, c.a])];
        if let Some((b, m)) = self.cache.shadows.borrow().get(&key) {
            return Some((b.clone(), *m, scale));
        }
        unsafe {
            let px = D2D_SIZE_U { width: (w + 2.0 * margin) as u32, height: (h + 2.0 * margin) as u32 };
            let dip = D2D_SIZE_F { width: px.width as f32, height: px.height as f32 };
            let options = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE;
            // A forma, nítida.
            let shape = self.target.CreateCompatibleRenderTarget(Some(&dip), Some(&px), None, options).ok()?;
            shape.BeginDraw();
            shape.SetTransform(&Matrix3x2::identity());
            shape.Clear(Some(&color(Color::TRANSPARENT)));
            let brush = shape.CreateSolidColorBrush(&color(c), None).ok()?;
            let rr = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: margin, top: margin, right: margin + w, bottom: margin + h },
                radiusX: corner * scale,
                radiusY: corner * scale,
            };
            shape.FillRoundedRectangle(&rr, &brush);
            shape.EndDraw(None, None).ok()?;
            let sharp = shape.GetBitmap().ok()?;
            // Desfocada pelo efeito, gravada noutro bitmap.
            let out = self.target.CreateCompatibleRenderTarget(Some(&dip), Some(&px), None, options).ok()?;
            let dc: ID2D1DeviceContext = out.cast().ok()?;
            let effect = dc.CreateEffect(&CLSID_D2D1GaussianBlur).ok()?;
            effect.SetInput(0, &sharp, true);
            effect.SetValue(D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION.0 as u32, D2D1_PROPERTY_TYPE_FLOAT, &sigma.to_le_bytes()).ok()?;
            out.BeginDraw();
            out.SetTransform(&Matrix3x2::identity());
            out.Clear(Some(&color(Color::TRANSPARENT)));
            dc.DrawImage(&effect.GetOutput().ok()?, None, None, D2D1_INTERPOLATION_MODE_LINEAR, D2D1_COMPOSITE_MODE_SOURCE_OVER);
            out.EndDraw(None, None).ok()?;
            let blurred = out.GetBitmap().ok()?;
            let mut cache = self.cache.shadows.borrow_mut();
            if cache.len() >= 256 {
                cache.clear();
            }
            cache.insert(key, (blurred.clone(), margin));
            Some((blurred, margin, scale))
        }
    }

    /// O gradiente cônico pintado na área `bounds` (em pixels da tela, até 1024 de lado) e
    /// usado como pincel. O ângulo começa no eixo x e cresce no sentido horário, como no Skia.
    fn sweep_brush(&self, center: PointF, start_angle: f32, stops: &[aylis_core::GradientStop], bounds: RectF) -> Option<ID2D1Brush> {
        if stops.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return None;
        }
        let scale = self.scale();
        let w = (bounds.width * scale).ceil().clamp(1.0, 1024.0) as u32;
        let h = (bounds.height * scale).ceil().clamp(1.0, 1024.0) as u32;
        let mut key: Vec<u64> = vec![
            center.x.to_bits() as u64,
            center.y.to_bits() as u64,
            start_angle.to_bits() as u64,
            ((bounds.x.to_bits() as u64) << 32) | bounds.y.to_bits() as u64,
            ((w as u64) << 32) | h as u64,
            ((bounds.width.to_bits() as u64) << 32) | bounds.height.to_bits() as u64,
        ];
        key.extend(stops.iter().map(|s| ((s.offset.to_bits() as u64) << 32) | u32::from_be_bytes([s.color.r, s.color.g, s.color.b, s.color.a]) as u64));
        let cached = self.cache.sweeps.borrow().get(&key).cloned();
        let bitmap = match cached {
            Some(b) => b,
            None => {
                let mut pixels = vec![0u8; (w * h * 4) as usize];
                for y in 0..h {
                    for x in 0..w {
                        let px = bounds.x + (x as f32 + 0.5) / w as f32 * bounds.width;
                        let py = bounds.y + (y as f32 + 0.5) / h as f32 * bounds.height;
                        let angle = (py - center.y).atan2(px - center.x).to_degrees();
                        let t = (angle - start_angle).rem_euclid(360.0) / 360.0;
                        let c = color_at(stops, t);
                        let a = c.a as u32;
                        let i = ((y * w + x) * 4) as usize;
                        pixels[i] = (c.b as u32 * a / 255) as u8;
                        pixels[i + 1] = (c.g as u32 * a / 255) as u8;
                        pixels[i + 2] = (c.r as u32 * a / 255) as u8;
                        pixels[i + 3] = c.a;
                    }
                }
                let props = D2D1_BITMAP_PROPERTIES {
                    pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                    dpiX: 96.0,
                    dpiY: 96.0,
                };
                let b = unsafe { self.target.CreateBitmap(D2D_SIZE_U { width: w, height: h }, Some(pixels.as_ptr() as *const _), w * 4, &props).ok()? };
                let mut cache = self.cache.sweeps.borrow_mut();
                if cache.len() >= 64 {
                    cache.clear();
                }
                cache.insert(key, b.clone());
                b
            }
        };
        unsafe {
            let props = D2D1_BITMAP_BRUSH_PROPERTIES {
                extendModeX: D2D1_EXTEND_MODE_CLAMP,
                extendModeY: D2D1_EXTEND_MODE_CLAMP,
                interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
            };
            let brush = self.target.CreateBitmapBrush(&bitmap, Some(&props), None).ok()?;
            let m = Matrix3x2 { M11: bounds.width / w as f32, M12: 0.0, M21: 0.0, M22: bounds.height / h as f32, M31: bounds.x, M32: bounds.y };
            brush.SetTransform(&m);
            brush.cast().ok()
        }
    }

    fn draw_gpu(&mut self, dest: RectF, surface: &GpuSurface) -> Option<()> {
        let GpuSurface::D3D12 { id, texture, ready, ready_value, release, release_value, .. } = *surface else { return None };
        let dc: ID2D1DeviceContext = self.target.cast().ok()?;
        let link = self.gpu.as_deref_mut()?;
        let frame = link.frame;
        let bitmap = match link.surfaces.get_mut(&id) {
            Some((bitmap, used)) => {
                *used = frame;
                bitmap.clone()
            }
            None => unsafe {
                let shared: IDXGISurface = link.device.OpenSharedResource1(HANDLE(texture as *mut _)).ok()?;
                let props = D2D1_BITMAP_PROPERTIES1 {
                    pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                    dpiX: 96.0,
                    dpiY: 96.0,
                    bitmapOptions: D2D1_BITMAP_OPTIONS_NONE,
                    colorContext: std::mem::ManuallyDrop::new(None),
                };
                let bitmap = dc.CreateBitmapFromDxgiSurface(&shared, Some(&props)).ok()?;
                link.surfaces.insert(id, (bitmap.clone(), frame));
                bitmap
            },
        };
        let ready = link.fence(ready)?;
        let release = link.fence(release)?;
        // A GPU espera o 3D terminar o quadro (a CPU não para), desenha, e no fim do quadro
        // a cerca de liberação avisa que a textura pode ser reescrita.
        unsafe {
            link.context4.as_ref()?.Wait(&ready, ready_value).ok()?;
            dc.DrawBitmap(&bitmap, Some(&rect(dest)), 1.0, D2D1_INTERPOLATION_MODE_LINEAR, None, None);
        }
        link.signals.push((release, release_value));
        Some(())
    }
}

impl Canvas for D2dCanvas<'_> {
    fn device_scale(&self) -> f32 {
        let m = &self.transform;
        (m.M11 * m.M22 - m.M12 * m.M21).abs().sqrt()
    }

    fn gpu_device(&self) -> Option<GpuDevice> {
        let link = self.gpu.as_deref()?;
        (link.device5.is_some() && link.context4.is_some()).then_some(GpuDevice::Direct3D11 { adapter_luid: link.luid })
    }

    fn draw_gpu_surface(&mut self, dest: RectF, surface: &GpuSurface) -> bool {
        self.draw_gpu(dest, surface).is_some()
    }

    fn save(&mut self) {
        self.frames.push(Frame { transform: self.transform, pushes: self.pushes.len() });
    }

    fn restore(&mut self) {
        if let Some(f) = self.frames.pop() {
            self.pop_until(f.pushes);
            self.set_transform(f.transform);
        }
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.concat(Matrix3x2::translation(dx, dy));
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        self.concat(Matrix3x2::scale(sx, sy));
    }

    fn rotate(&mut self, degrees: f32) {
        let (s, c) = degrees.to_radians().sin_cos();
        self.concat(Matrix3x2 { M11: c, M12: s, M21: -s, M22: c, M31: 0.0, M32: 0.0 });
    }

    fn clip_rect(&mut self, r: RectF) {
        unsafe { self.target.PushAxisAlignedClip(&rect(r), D2D1_ANTIALIAS_MODE_PER_PRIMITIVE) };
        self.pushes.push(Push::Clip(rect(r), self.transform));
    }

    fn save_layer(&mut self, opacity: f32) {
        self.save();
        self.push_layer(None, opacity);
    }

    fn fill_rect(&mut self, r: RectF, radius: f32, brush: &Brush) {
        let Some(b) = self.brush(brush, r) else { return };
        unsafe {
            if radius > 0.0 {
                self.target.FillRoundedRectangle(&Self::rounded(r, radius), &b);
            } else {
                self.target.FillRectangle(&rect(r), &b);
            }
        }
    }

    /// O traço fica do lado de dentro, como no Skia.
    fn stroke_rect(&mut self, r: RectF, radius: f32, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        let Some(b) = self.brush(&stroke.brush, r) else { return };
        let style = self.renderer.stroke_style(stroke);
        let inset = r.inflate(-stroke.width / 2.0);
        unsafe {
            if radius > 0.0 {
                let rr = Self::rounded(inset, (radius - stroke.width / 2.0).max(0.0));
                self.target.DrawRoundedRectangle(&rr, &b, stroke.width, style.as_ref());
            } else {
                self.target.DrawRectangle(&rect(inset), &b, stroke.width, style.as_ref());
            }
        }
    }

    fn fill_ellipse(&mut self, r: RectF, brush: &Brush) {
        let Some(b) = self.brush(brush, r) else { return };
        let e = D2D1_ELLIPSE { point: v(r.center()), radiusX: r.width / 2.0, radiusY: r.height / 2.0 };
        unsafe { self.target.FillEllipse(&e, &b) };
    }

    fn stroke_ellipse(&mut self, r: RectF, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        let Some(b) = self.brush(&stroke.brush, r) else { return };
        let r = r.inflate(-stroke.width / 2.0);
        let e = D2D1_ELLIPSE { point: v(r.center()), radiusX: r.width / 2.0, radiusY: r.height / 2.0 };
        let style = self.renderer.stroke_style(stroke);
        unsafe { self.target.DrawEllipse(&e, &b, stroke.width, style.as_ref()) };
    }

    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        let area = RectF::new(from.x.min(to.x), from.y.min(to.y), (from.x - to.x).abs(), (from.y - to.y).abs()).inflate(stroke.width);
        let Some(b) = self.brush(&stroke.brush, area) else { return };
        let style = self.renderer.stroke_style(stroke);
        unsafe { self.target.DrawLine(v(from), v(to), &b, stroke.width, style.as_ref()) };
    }

    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        let (Some(g), Some(b)) = (self.geometry(path), self.brush(brush, path.bounds())) else { return };
        unsafe { self.target.FillGeometry(&g, &b, None::<&ID2D1Brush>) };
    }

    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        let (Some(g), Some(b)) = (self.geometry(path), self.brush(&stroke.brush, path.bounds().inflate(stroke.width))) else { return };
        let style = self.renderer.stroke_style(stroke);
        unsafe { self.target.DrawGeometry(&g, &b, stroke.width, style.as_ref()) };
    }

    /// Desfoque de verdade (Gaussian Blur, sigma = raio ÷ 2, como no Skia), guardado por
    /// tamanho, cantos, desfoque e cor: nos quadros seguintes a sombra é só uma imagem.
    fn draw_shadow(&mut self, r: RectF, radius: f32, shadow: &Shadow) {
        if shadow.color.a == 0 {
            return;
        }
        let area = RectF::new(r.x + shadow.offset.x, r.y + shadow.offset.y, r.width, r.height).inflate(shadow.spread);
        let corner = (radius + shadow.spread).max(0.0);
        if shadow.blur <= 0.0 {
            self.fill_rect(area, corner, &Brush::Solid(shadow.color));
            return;
        }
        if let Some((bitmap, margin, scale)) = self.shadow_image(area, corner, shadow) {
            let dest = RectF::new(area.x - margin / scale, area.y - margin / scale, 0.0, 0.0);
            let size = unsafe { bitmap.GetPixelSize() };
            let dest = RectF::new(dest.x, dest.y, size.width as f32 / scale, size.height as f32 / scale);
            unsafe { self.target.DrawBitmap(&bitmap, Some(&rect(dest)), 1.0, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, None) };
            return;
        }
        // Reserva: anéis que crescem e esmaecem.
        let steps = 8;
        let base = RectF::new(r.x + shadow.offset.x, r.y + shadow.offset.y, r.width, r.height).inflate(shadow.spread);
        let alpha = shadow.color.a as f32 / steps as f32;
        for i in 0..steps {
            let grow = shadow.blur * (i as f32 / steps as f32) - shadow.blur / 4.0;
            let c = Color::rgba(shadow.color.r, shadow.color.g, shadow.color.b, (alpha * 0.9) as u8);
            self.fill_rect(base.inflate(grow), (radius + grow).max(0.0), &Brush::Solid(c));
        }
    }

    fn draw_text(&mut self, text: &str, r: RectF, style: &TextStyle) {
        if text.is_empty() || r.width <= 0.0 {
            return;
        }
        let key = FontKey::of(&style.font);
        let text_cache = self.renderer.text.clone();
        let lh = text_cache.line_height(&key);
        let mut measure = |s: &str| text_cache.width(s, &key);
        // Uma linha que cabe: sem a regra de quebra.
        let lines: Vec<(String, bool)> = if !style.wrap && !text.contains('\n') {
            let w = measure(text);
            if w <= r.width || style.trimming == Trimming::None {
                vec![(text.to_string(), true)]
            } else {
                vec![(aylis_core::text::trim_line(text, r.width, style.trimming, &mut measure), true)]
            }
        } else {
            let max_lines = ((r.height + 1.0) / lh).floor().max(1.0) as usize;
            layout_lines_marked(text, r.width, max_lines, style.wrap, style.trimming, &mut measure)
        };
        let total = lh * lines.len() as f32;
        let top = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => r.y,
            TextAlign::Center => r.y + (r.height - total) / 2.0,
            TextAlign::Trailing => r.bottom() - total,
        };
        for (i, (line, end)) in lines.iter().enumerate() {
            let y = top + i as f32 * lh;
            // Justificado: cada palavra no lugar dela (a última linha do parágrafo fica à
            // esquerda); o sublinhado vai de ponta a ponta.
            let words = (style.h_align == TextAlign::Justify && !end).then(|| aylis_core::text::justify_line(line, r.width, &mut measure)).flatten();
            if let Some(words) = words {
                for (x, word) in &words {
                    self.draw_line_text(word, PointF::new(r.x + x, y), &key, style.color);
                }
                let right = words.last().map_or(0.0, |(x, w)| x + text_cache.width(w, &key));
                aylis_core::decorate_line(self, &style.font, style.color, r.x, right, y, lh);
                continue;
            }
            let w = text_cache.width(line, &key);
            let x = match style.h_align {
                TextAlign::Leading | TextAlign::Justify => r.x,
                TextAlign::Center => r.x + (r.width - w) / 2.0,
                TextAlign::Trailing => r.right() - w,
            };
            self.draw_line_text(line, PointF::new(x, top + i as f32 * lh), &key, style.color);
            aylis_core::decorate_line(self, &style.font, style.color, x, w, top + i as f32 * lh, lh);
        }
    }

    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        let key = FontKey::of(&style.font);
        SizeF::new(self.renderer.text.width(text, &key), self.renderer.text.line_height(&key))
    }

    fn draw_image(&mut self, image: &Image, src: RectF, dest: RectF, opacity: f32) {
        let Some((bitmap, _, _)) = self.bitmap(image) else { return };
        unsafe {
            self.target.DrawBitmap(&bitmap, Some(&rect(dest)), opacity, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, Some(&rect(src)));
        }
    }

    fn image_size(&mut self, image: &Image) -> Option<SizeF> {
        image.known_size().or_else(|| self.renderer.decode(image).map(|d| SizeF::new(d.0 as f32, d.1 as f32)))
    }

    fn backdrop_blur(&mut self, r: RectF, radius: f32, blur: f32) -> bool {
        // O pedaço do alvo (em pixels, pela transformação atual) vira um bitmap, que volta
        // desfocado pelo efeito Gaussian Blur, recortado nos cantos arredondados.
        let m = self.transform;
        let map = |x: f32, y: f32| (x * m.M11 + y * m.M21 + m.M31, x * m.M12 + y * m.M22 + m.M32);
        let (x0, y0) = map(r.x, r.y);
        let (x1, y1) = map(r.right(), r.bottom());
        let scale = (m.M11 * m.M22 - m.M12 * m.M21).abs().sqrt().max(0.01);
        let pad = (blur * scale * 3.0).ceil();
        let size = unsafe { self.target.GetPixelSize() };
        let left = (x0.min(x1) - pad).floor().max(0.0) as u32;
        let top = (y0.min(y1) - pad).floor().max(0.0) as u32;
        let right = ((x0.max(x1) + pad).ceil() as u32).min(size.width);
        let bottom = ((y0.max(y1) + pad).ceil() as u32).min(size.height);
        if right <= left || bottom <= top {
            return false;
        }
        let ok = (|| -> windows_core::Result<()> {
            unsafe {
                let dc: ID2D1DeviceContext = self.target.cast()?;
                let props = D2D1_BITMAP_PROPERTIES { pixelFormat: self.target.GetPixelFormat(), dpiX: 96.0, dpiY: 96.0 };
                let bitmap = self.target.CreateBitmap(D2D_SIZE_U { width: right - left, height: bottom - top }, None, 0, &props)?;
                let src = D2D_RECT_U { left, top, right, bottom };
                self.without_pushes(|c| c.target.Flush(None, None).and_then(|_| bitmap.CopyFromRenderTarget(None, &c.target, Some(&src))))?;
                let effect = dc.CreateEffect(&CLSID_D2D1GaussianBlur)?;
                effect.SetInput(0, &bitmap, true);
                let sigma = (blur * scale).to_le_bytes();
                effect.SetValue(D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION.0 as u32, D2D1_PROPERTY_TYPE_FLOAT, &sigma)?;
                let hard = (D2D1_BORDER_MODE_HARD.0 as u32).to_le_bytes();
                effect.SetValue(D2D1_GAUSSIANBLUR_PROP_BORDER_MODE.0 as u32, D2D1_PROPERTY_TYPE_ENUM, &hard)?;
                let output = effect.GetOutput()?;
                self.save();
                self.clip_round_rect(r, radius);
                dc.SetTransform(&Matrix3x2::identity());
                let offset = windows_numerics::Vector2 { X: left as f32, Y: top as f32 };
                dc.DrawImage(&output, Some(&offset), None, D2D1_INTERPOLATION_MODE_LINEAR, D2D1_COMPOSITE_MODE_SOURCE_OVER);
                self.restore();
                Ok(())
            }
        })();
        ok.is_ok()
    }

    /// Só a máscara normal: a invertida precisaria de "destino fora", que o Direct2D não
    /// tem nas primitivas (fica com o recorte pelo contorno).
    fn begin_masked(&mut self, inverse: bool) -> bool {
        if inverse {
            return false;
        }
        let Some(content) = self.offscreen() else { return false };
        let Ok(as_target) = content.cast::<ID2D1RenderTarget>() else { return false };
        let main = std::mem::replace(&mut self.target, as_target);
        let pushes = std::mem::take(&mut self.pushes);
        self.masks.push(MaskState { main, pushes, content, mask: None });
        true
    }

    fn begin_mask(&mut self) {
        let Some(mask) = self.offscreen() else { return };
        self.pop_until(0);
        if let Some(state) = self.masks.last_mut() {
            unsafe {
                let _ = state.content.EndDraw(None, None);
            }
            if let Ok(t) = mask.cast() {
                self.target = t;
            }
            state.mask = Some(mask);
        }
    }

    fn end_mask(&mut self) {
        self.end_mask_inner();
    }

    fn begin_cached(&mut self, key: u64, r: RectF) -> bool {
        use std::hash::{Hash, Hasher};
        let m = self.transform;
        // Só sem giro: a imagem vai para a tela pixel a pixel.
        if m.M12 != 0.0 || m.M21 != 0.0 || m.M11 <= 0.0 || m.M22 <= 0.0 {
            self.cached.push(None);
            return false;
        }
        let (x0, y0) = ((r.x * m.M11 + m.M31).round(), (r.y * m.M22 + m.M32).round());
        let (w, h) = ((r.width * m.M11).ceil(), (r.height * m.M22).ceil());
        if w < 1.0 || h < 1.0 || w > 8192.0 || h > 8192.0 {
            self.cached.push(None);
            return false;
        }
        let dest = D2D_RECT_F { left: x0, top: y0, right: x0 + w, bottom: y0 + h };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (key, w as u32, h as u32, m.M11.to_bits(), m.M22.to_bits()).hash(&mut hasher);
        let full = hasher.finish();
        if let Some(bitmap) = self.cache.layers.get(&full).cloned() {
            self.draw_layer(&bitmap, &dest);
            return true;
        }
        let size = D2D_SIZE_F { width: w, height: h };
        let pixels = D2D_SIZE_U { width: w as u32, height: h as u32 };
        let Some(content) = (unsafe { self.target.CreateCompatibleRenderTarget(Some(&size), Some(&pixels), None, D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE).ok() }) else {
            self.cached.push(None);
            return false;
        };
        let Ok(as_target) = content.cast::<ID2D1RenderTarget>() else {
            self.cached.push(None);
            return false;
        };
        unsafe {
            content.BeginDraw();
            content.SetAntialiasMode(D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
            content.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
            content.Clear(Some(&color(Color::TRANSPARENT)));
        }
        let main = std::mem::replace(&mut self.target, as_target);
        let pushes = std::mem::take(&mut self.pushes);
        let transform = self.transform;
        // A mesma escala; a origem da imagem é o canto de `dest`.
        self.set_transform(Matrix3x2 { M31: m.M31 - x0, M32: m.M32 - y0, ..m });
        self.cached.push(Some(CacheState { main, pushes, transform, content, key: full, dest }));
        false
    }

    fn end_cached(&mut self) {
        self.end_cached_inner();
    }

    fn clip_path(&mut self, path: &Path, inverse: bool) {
        // Invertido: um retângulo enorme em volta, com a regra par-ímpar (vale para
        // caminhos que não se cruzam, como as máscaras do Lottie).
        let mask = if inverse {
            let mut p = Path::new();
            p.rect(RectF::new(-1.0e6, -1.0e6, 2.0e6, 2.0e6), 0.0);
            p.append(path);
            p.even_odd = true;
            p
        } else {
            path.clone()
        };
        match self.geometry(&mask).and_then(|g| g.cast::<ID2D1Geometry>().ok()) {
            Some(g) => self.push_layer(Some(g), 1.0),
            None => {
                if !inverse {
                    self.clip_rect(path.bounds());
                }
            }
        }
    }

    fn clip_round_rect(&mut self, r: RectF, radius: f32) {
        if radius <= 0.0 {
            return self.clip_rect(r);
        }
        let geometry = unsafe { self.renderer.factory.CreateRoundedRectangleGeometry(&Self::rounded(r, radius)) };
        match geometry.ok().and_then(|g| g.cast::<ID2D1Geometry>().ok()) {
            Some(g) => self.push_layer(Some(g), 1.0),
            None => self.clip_rect(r),
        }
    }
}

// ---------------------------------------------------------------------------
// imagens

fn decode_with_wic(wic: &IWICImagingFactory, bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    unsafe {
        let stream = wic.CreateStream().ok()?;
        stream.InitializeFromMemory(bytes).ok()?;
        let decoder = wic.CreateDecoderFromStream(&stream, std::ptr::null(), WICDecodeMetadataCacheOnDemand).ok()?;
        let frame = decoder.GetFrame(0).ok()?;
        let converter = wic.CreateFormatConverter().ok()?;
        converter
            .Initialize(&frame, &GUID_WICPixelFormat32bppRGBA, WICBitmapDitherTypeNone, None, 0.0, WICBitmapPaletteTypeCustom)
            .ok()?;
        let (mut w, mut h) = (0u32, 0u32);
        converter.GetSize(&mut w, &mut h).ok()?;
        // O tamanho vem do arquivo: confere antes de reservar (e sem estourar o u32).
        let mut pixels = vec![0u8; aylis_core::image::checked_size(w, h).ok()?];
        converter.CopyPixels(std::ptr::null(), w * 4, &mut pixels).ok()?;
        Some((pixels, w, h))
    }
}

/// Decodifica uma imagem (PNG, JPEG, BMP, GIF, ICO, TIFF...) para RGBA, pelo WIC.
pub fn decode_rgba(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let wic: IWICImagingFactory = CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).ok()?;
        decode_with_wic(&wic, bytes)
    }
}

fn premultiply_rgba_to_bgra(rgba: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rgba.len());
    for p in rgba.chunks_exact(4) {
        let a = p[3] as u32;
        let m = |c: u8| ((c as u32 * a + 127) / 255) as u8;
        out.extend_from_slice(&[m(p[2]), m(p[1]), m(p[0]), p[3]]);
    }
    out
}

fn premultiply_bgra(bgra: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bgra.len());
    for p in bgra.chunks_exact(4) {
        let a = p[3] as u32;
        let m = |c: u8| ((c as u32 * a + 127) / 255) as u8;
        out.extend_from_slice(&[m(p[0]), m(p[1]), m(p[2]), p[3]]);
    }
    out
}

fn unpremultiply_bgra_to_rgba(bgra: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bgra.len());
    for p in bgra.chunks_exact(4) {
        let a = p[3] as u32;
        let u = |c: u8| (c as u32 * 255 + a / 2).checked_div(a).map_or(0, |v| v.min(255) as u8);
        out.extend_from_slice(&[u(p[2]), u(p[1]), u(p[0]), p[3]]);
    }
    out
}

// ---------------------------------------------------------------------------
// PNG (para as capturas), sem compressão e sem dependências

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

/// Grava pixels RGBA num PNG (deflate sem compressão: arquivo maior, código mínimo).
pub fn encode_png(rgba: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    if aylis_core::image::checked_size(width, height).ok()? != rgba.len() {
        return None;
    }
    let mut raw = Vec::with_capacity(rgba.len() + height as usize);
    for row in rgba.chunks_exact((width * 4) as usize) {
        raw.push(0); // filtro "nenhum"
        raw.extend_from_slice(row);
    }
    let mut z = vec![0x78, 0x01];
    let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (i, block) in blocks.iter().enumerate() {
        z.push(u8::from(i + 1 == blocks.len()));
        let len = block.len() as u16;
        z.extend_from_slice(&len.to_le_bytes());
        z.extend_from_slice(&(!len).to_le_bytes());
        z.extend_from_slice(block);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &x in &raw {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut chunk = |kind: &[u8], data: &[u8]| {
        png.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut body = kind.to_vec();
        body.extend_from_slice(data);
        png.extend_from_slice(&body);
        png.extend_from_slice(&crc32(&body).to_be_bytes());
    };
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(b"IHDR", &ihdr);
    chunk(b"IDAT", &z);
    chunk(b"IEND", &[]);
    Some(png)
}

/// A cor do gradiente na posição `t` (0..1) entre as paradas.
fn color_at(stops: &[aylis_core::GradientStop], t: f32) -> Color {
    let first = stops[0];
    if t <= first.offset {
        return first.color;
    }
    for pair in stops.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if t <= b.offset {
            let f = if b.offset > a.offset { (t - a.offset) / (b.offset - a.offset) } else { 0.0 };
            return a.color.lerp(b.color, f);
        }
    }
    stops[stops.len() - 1].color
}

/// Uma variável de diagnóstico, só com a feature `diagnostics` (o app publicado não lê).
fn diagnostic(name: &str) -> Option<String> {
    if cfg!(feature = "diagnostics") { std::env::var(name).ok() } else { None }
}
