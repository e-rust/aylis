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

mod compositor;

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
use windows::Win32::Graphics::Gdi::{HMONITOR, MONITOR_DEFAULTTONEAREST, MonitorFromWindow};
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

/// Os trechos (em bytes) que o Unicode manda mostrar como texto, e não como emoji: os
/// caracteres emoji cuja apresentação padrão é de texto (`Emoji_Presentation=No`: `⚠`, `ℹ`,
/// `☎`), a não ser seguidos de U+FE0F, e qualquer emoji seguido de U+FE0E (que entra no
/// trecho). As sequências (ZWJ, tom de pele, tecla) ficam como estão. A mesma regra do
/// `aylis-skia`; o navegador já faz assim.
fn text_presentation_ranges(text: &str) -> Vec<(usize, usize)> {
    use unicode_properties::{EmojiStatus, UnicodeEmoji};
    let mut out = Vec::new();
    let mut prev = '\0';
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let next = chars.peek().map(|&(_, n)| n);
        let joined = prev == '\u{200D}' || matches!(next, Some('\u{200D}' | '\u{20E3}' | '\u{1F3FB}'..='\u{1F3FF}'));
        prev = c;
        if c.is_ascii() || joined || next == Some('\u{FE0F}') {
            continue;
        }
        if next == Some('\u{FE0E}') && c.is_emoji_char() {
            let (j, v) = chars.next().unwrap_or((i, c));
            prev = v;
            out.push((i, j + v.len_utf8()));
        } else if matches!(c.emoji_status(), EmojiStatus::EmojiOther | EmojiStatus::EmojiModifierBase) {
            out.push((i, i + c.len_utf8()));
        }
    }
    out
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
    /// O `letter_spacing` do estilo (bits do `f32`).
    spacing: u32,
    /// `color_glyphs: false`: emoji na fonte sem cor.
    mono_emoji: bool,
}

impl FontKey {
    /// A chave do texto de um estilo (com o espaço entre letras e as cores dos emoji).
    fn of_style(style: &TextStyle) -> Self {
        Self { spacing: style.letter_spacing.to_bits(), mono_emoji: !style.color_glyphs, ..Self::of(&style.font) }
    }

    fn letter_spacing(&self) -> f32 {
        f32::from_bits(self.spacing)
    }

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
        Self { family: font.family.clone(), size: (font.size * 100.0).round() as u32, weight, italic: font.italic, spacing: 0, mono_emoji: false }
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
    /// Os tamanhos de `measure_text_wrapped` já calculados.
    wrapped: RefCell<aylis_core::canvas::WrapCache>,
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

    /// Os símbolos de apresentação de texto (`⚠`, `ℹ`, ou com U+FE0E) que a fonte pedida não
    /// tem vão para a Segoe UI Symbol, monocromática: o fallback do sistema os mandava para a
    /// Segoe UI Emoji, e o desenho com `ENABLE_COLOR_FONT` os pintava coloridos. A mesma regra
    /// do Skia; o navegador já faz assim.
    fn text_presentation(&self, layout: &IDWriteTextLayout, text: &str, key: &FontKey) {
        let ranges = text_presentation_ranges(text);
        if ranges.is_empty() {
            return;
        }
        let symbol = wide("Segoe UI Symbol\0");
        for (start, end) in ranges {
            let Some(c) = text[start..].chars().next() else { continue };
            if self.has_char(key, c) {
                continue;
            }
            let at = text[..start].encode_utf16().count() as u32;
            let length = text[start..end].encode_utf16().count() as u32;
            // SAFETY: `symbol` termina em zero e vive até o fim da chamada; o layout é válido.
            unsafe {
                let _ = layout.SetFontFamilyName(PCWSTR(symbol.as_ptr()), DWRITE_TEXT_RANGE { startPosition: at, length });
            }
        }
    }

    /// `color_glyphs: false`: os emoji ([`aylis_core::canvas::MONO_EMOJI_RANGES`]) vão para a
    /// Noto Emoji sem cor quando o app a registrou; sem ela, o desenho sem
    /// `ENABLE_COLOR_FONT` já tira a cor da fonte de emoji do sistema.
    fn mono_emoji(&self, layout: &IDWriteTextLayout, text: &str) {
        if !aylis_core::canvas::mono_emoji_available() {
            return;
        }
        let family = wide(&format!("{}\0", aylis_core::canvas::MONO_EMOJI_FAMILY));
        for (start, end) in aylis_core::canvas::mono_emoji_ranges(text) {
            let at = text[..start].encode_utf16().count() as u32;
            let length = text[start..end].encode_utf16().count() as u32;
            // SAFETY: `family` termina em zero e vive até o fim da chamada; o layout é válido.
            unsafe {
                let _ = layout.SetFontFamilyName(PCWSTR(family.as_ptr()), DWRITE_TEXT_RANGE { startPosition: at, length });
            }
        }
    }

    /// A fonte pedida (da coleção do Aylis ou do sistema) tem o caractere?
    fn has_char(&self, key: &FontKey, c: char) -> bool {
        let collection = match self.collection.borrow().clone() {
            Some(c) => c,
            None => {
                let mut system = None;
                // SAFETY: a fábrica DirectWrite é válida e `system` recebe a coleção do sistema.
                if unsafe { self.dwrite.GetSystemFontCollection(&mut system, false) }.is_err() {
                    return false;
                }
                let Some(system) = system else { return false };
                system
            }
        };
        let mut family = wide(key.family_name());
        family.push(0);
        let (mut index, mut exists) = (0u32, windows::core::BOOL(0));
        // SAFETY: `family` termina em zero e vive até o fim do bloco; os ponteiros de saída são
        // variáveis locais.
        unsafe {
            if collection.FindFamilyName(PCWSTR(family.as_ptr()), &mut index, &mut exists).is_err() || !exists.as_bool() {
                return false;
            }
            let style = if key.italic { DWRITE_FONT_STYLE_ITALIC } else { DWRITE_FONT_STYLE_NORMAL };
            collection
                .GetFontFamily(index)
                .and_then(|f| f.GetFirstMatchingFont(DWRITE_FONT_WEIGHT(key.weight), DWRITE_FONT_STRETCH_NORMAL, style))
                .and_then(|font| font.HasCharacter(c as u32))
                .is_ok_and(|has| has.as_bool())
        }
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
        self.text_presentation(&layout, text, key);
        if key.mono_emoji {
            self.mono_emoji(&layout, text);
        }
        let spacing = key.letter_spacing();
        if spacing != 0.0 {
            // O espaço vai depois de cada letra; a largura medida não conta o da última (a
            // regra do `TextStyle::letter_spacing`, igual no Skia e no navegador).
            if let Ok(l1) = layout.cast::<IDWriteTextLayout1>() {
                let all = DWRITE_TEXT_RANGE { startPosition: 0, length: w.len() as u32 };
                // SAFETY: `l1` é o mesmo layout válido, visto pela interface `IDWriteTextLayout1`.
                unsafe {
                    let _ = l1.SetCharacterSpacing(0.0, spacing, 0.0, all);
                }
            }
        }
        let mut m = DWRITE_TEXT_METRICS::default();
        unsafe { layout.GetMetrics(&mut m).ok()? };
        let width = if spacing != 0.0 && !text.is_empty() { m.widthIncludingTrailingWhitespace - spacing } else { m.widthIncludingTrailingWhitespace };
        let entry = (layout, width);
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
        let key = FontKey::of_style(style);
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
    /// Os dispositivos do Direct3D 11 e do Direct2D já criados, um por placa pedida: cada
    /// janela nova na mesma placa usa o mesmo (criar um custa ~30 ms; o form em vários monitores
    /// abre uma janela por monitor, e o print do NoOBS precisa aparecer em ~100 ms). Cada janela
    /// continua com o próprio contexto do Direct2D e a própria swap chain.
    devices: RefCell<Vec<SharedDevice>>,
    /// Uma mistura já abriu dentro de uma camada de opacidade com desenho: daí em diante, as
    /// camadas de opacidade são abertas fora da tela (veja `isolate_open_layer`).
    isolate_layers: std::cell::Cell<bool>,
    /// Os parâmetros de desenho do texto (a suavização simétrica), feitos uma vez: em cinza e
    /// em ClearType.
    text_params: RefCell<Option<[Option<IDWriteRenderingParams>; 2]>>,
}

/// Um dispositivo do Direct3D 11 (com o do Direct2D por cima) para as janelas de uma placa.
struct SharedDevice {
    /// A placa pedida (o LUID; `None`: a padrão do sistema).
    key: Option<i64>,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    d2d: ID2D1Device,
}

/// A placa pedida por `choice` para a janela `hwnd` (`None`: a padrão do sistema). Com
/// [`GpuChoice::FollowMonitor`](aylis_core::gpu::GpuChoice::FollowMonitor), a que tem o monitor
/// em que está a maior parte da janela.
fn adapter_for(choice: &aylis_core::gpu::GpuChoice, hwnd: isize) -> Option<IDXGIAdapter> {
    use aylis_core::gpu::GpuChoice;
    // SAFETY: chamadas COM do DXGI, sem ponteiros guardados.
    unsafe {
        let factory: IDXGIFactory6 = CreateDXGIFactory1().ok()?;
        let adapter: IDXGIAdapter1 = match choice {
            GpuChoice::Auto => return None,
            GpuChoice::HighPerformance => factory.EnumAdapterByGpuPreference(0, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE).ok()?,
            GpuChoice::LowPower => factory.EnumAdapterByGpuPreference(0, DXGI_GPU_PREFERENCE_MINIMUM_POWER).ok()?,
            GpuChoice::FollowMonitor => adapter_of_monitor(&factory, monitor_of(hwnd))?,
            GpuChoice::Named(_) => (0..).map_while(|i| factory.EnumAdapters1(i).ok()).find(|a| a.GetDesc1().is_ok_and(|d| choice.matches_name(&adapter_name(&d.Description))))?,
        };
        adapter.cast().ok()
    }
}

/// O monitor com a maior parte da janela (o mais perto, se ela está fora de todos).
fn monitor_of(hwnd: isize) -> HMONITOR {
    // SAFETY: só lê; uma janela que não existe mais dá o monitor principal.
    unsafe { MonitorFromWindow(HWND(hwnd as *mut _), MONITOR_DEFAULTTONEAREST) }
}

/// A placa que tem uma saída ligada a `monitor` (`None`: nenhuma, como numa sessão remota).
fn adapter_of_monitor(factory: &IDXGIFactory6, monitor: HMONITOR) -> Option<IDXGIAdapter1> {
    // SAFETY: chamadas COM do DXGI; as saídas são só lidas.
    unsafe {
        (0..).map_while(|i| factory.EnumAdapters1(i).ok()).find(|adapter| {
            (0..).map_while(|j| adapter.EnumOutputs(j).ok()).any(|output| output.GetDesc().is_ok_and(|d| d.Monitor == monitor))
        })
    }
}

/// O LUID da placa como o [`GpuLink`] guarda.
fn luid_of(adapter: &IDXGIAdapter) -> Option<i64> {
    // SAFETY: só lê a descrição.
    let luid = unsafe { adapter.GetDesc().ok()?.AdapterLuid };
    Some(((luid.HighPart as i64) << 32) | luid.LowPart as i64)
}

/// A placa pelo LUID (ou, com `software`, o WARP, o renderizador por software do Windows).
fn adapter_where(test: impl Fn(&DXGI_ADAPTER_DESC1) -> bool) -> Option<IDXGIAdapter> {
    // SAFETY: chamadas COM do DXGI, sem ponteiros guardados.
    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1().ok()?;
        (0..).map_while(|i| factory.EnumAdapters1(i).ok()).find(|a| a.GetDesc1().is_ok_and(|d| test(&d)))?.cast().ok()
    }
}

fn desc_luid(d: &DXGI_ADAPTER_DESC1) -> i64 {
    ((d.AdapterLuid.HighPart as i64) << 32) | d.AdapterLuid.LowPart as i64
}

fn is_software(d: &DXGI_ADAPTER_DESC1) -> bool {
    d.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0
}

/// As placas de vídeo da máquina, pelo nome, sem o renderizador por software do Windows.
/// Quantos bitmaps de pixels crus (quadros de câmera e de vídeo) foram criados na placa.
static PIXEL_BITMAPS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Diagnóstico: quantos bitmaps de quadros (BGRA) o Direct2D criou na placa até agora. Com o
/// quadro anterior reaproveitado, fica em um por câmera, e não um por quadro.
pub fn pixel_bitmaps_created() -> u64 {
    PIXEL_BITMAPS.load(std::sync::atomic::Ordering::Relaxed)
}

static SHADER_BLENDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Quantas camadas com mistura o compositor em Direct3D já juntou ao fundo no processo (os
/// testes conferem que o shader rodou, e não o efeito do Direct2D no lugar dele).
pub fn shader_blends() -> u64 {
    SHADER_BLENDS.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn adapters() -> Vec<String> {
    // SAFETY: chamadas COM do DXGI, sem ponteiros guardados.
    unsafe {
        let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() else { return Vec::new() };
        (0..)
            .map_while(|i| factory.EnumAdapters1(i).ok())
            .filter_map(|a| a.GetDesc1().ok())
            .filter(|d| !is_software(d))
            .map(|d| adapter_name(&d.Description))
            .collect()
    }
}

/// Abre uma camada (com a máscara `mask` e a opacidade `opacity`) em `target`. Com
/// `cleartype` (o alvo opaco, em ClearType), o texto dentro dela continua em ClearType: no
/// contexto do Direct2D 1.1, a camada começa com o fundo e ignora o alfa; sem ele, a opção do
/// Direct2D 1.0. Sem a opção, o Direct2D desenha o texto da camada em cinza.
fn push_layer_on(target: &ID2D1RenderTarget, mask: Option<ID2D1Geometry>, mask_antialias: D2D1_ANTIALIAS_MODE, opacity: f32, cleartype: bool) {
    let dc = if cleartype { target.cast::<ID2D1DeviceContext>().ok() } else { None };
    // SAFETY: chamadas COM no alvo válido; os parâmetros vivem até a chamada e soltam a
    // máscara (o `ManuallyDrop`) logo depois.
    unsafe {
        match dc {
            Some(dc) => {
                let mut params = D2D1_LAYER_PARAMETERS1 {
                    contentBounds: INFINITE,
                    geometricMask: std::mem::ManuallyDrop::new(mask),
                    maskAntialiasMode: mask_antialias,
                    maskTransform: Matrix3x2::identity(),
                    opacity,
                    opacityBrush: std::mem::ManuallyDrop::new(None),
                    layerOptions: D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND | D2D1_LAYER_OPTIONS1_IGNORE_ALPHA,
                };
                dc.PushLayer(&params, None::<&ID2D1Layer>);
                std::mem::ManuallyDrop::drop(&mut params.geometricMask);
                std::mem::ManuallyDrop::drop(&mut params.opacityBrush);
            }
            None => {
                let mut params = D2D1_LAYER_PARAMETERS {
                    contentBounds: INFINITE,
                    geometricMask: std::mem::ManuallyDrop::new(mask),
                    maskAntialiasMode: mask_antialias,
                    maskTransform: Matrix3x2::identity(),
                    opacity,
                    opacityBrush: std::mem::ManuallyDrop::new(None),
                    layerOptions: if cleartype { D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE } else { D2D1_LAYER_OPTIONS_NONE },
                };
                target.PushLayer(&params, None::<&ID2D1Layer>);
                std::mem::ManuallyDrop::drop(&mut params.geometricMask);
                std::mem::ManuallyDrop::drop(&mut params.opacityBrush);
            }
        }
    }
}

/// O nome da placa na descrição do DXGI.
fn adapter_name(description: &[u16]) -> String {
    let end = description.iter().position(|&c| c == 0).unwrap_or(description.len());
    String::from_utf16_lossy(&description[..end])
}

impl D2dRenderer {
    /// A suavização do texto de `target`: ClearType ou cinza, sempre com o desenho simétrico
    /// (`NATURAL_SYMMETRIC`: suaviza também na vertical, e as diagonais da letra pequena não
    /// saem em degraus). Só muda a pintura: o tamanho medido vem do layout do DirectWrite.
    fn set_text_mode(&self, target: &ID2D1RenderTarget, cleartype: bool) {
        let mut params = self.text_params.borrow_mut();
        let params = params.get_or_insert_with(|| {
            // SAFETY: chamadas COM na fábrica do DirectWrite, viva enquanto o renderizador.
            let make = |cleartype: bool| unsafe {
                // O gama, o contraste e o subpixel do monitor (os do sistema); no cinza, sem o
                // nível de ClearType.
                let base = self.text.dwrite.CreateRenderingParams().ok()?;
                let level = if cleartype { base.GetClearTypeLevel() } else { 0.0 };
                self.text
                    .dwrite
                    .CreateCustomRenderingParams(base.GetGamma(), base.GetEnhancedContrast(), level, base.GetPixelGeometry(), DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC)
                    .ok()
            };
            [make(false), make(true)]
        });
        // SAFETY: o alvo é válido (quem chama o tem emprestado) e os parâmetros vivem no cache.
        unsafe {
            target.SetTextAntialiasMode(if cleartype { D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE } else { D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE });
            if let Some(p) = &params[usize::from(cleartype)] {
                target.SetTextRenderingParams(p);
            }
        }
    }

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
                    wrapped: RefCell::default(),
                }),
                strokes: RefCell::default(),
                geometries: RefCell::default(),
                decoded: RefCell::new(ImageCache::named("Direct2D · arquivos decodificados")),
                devices: RefCell::default(),
                isolate_layers: std::cell::Cell::new(false),
                text_params: RefCell::new(None),
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
            // Já pré-multiplicado (o contrato do tipo): multiplicar de novo escurecia o
            // semitransparente (T08 do Tabernáculo).
            ImageSource::Bgra { width, height, pixels } => Some((*width, *height, pixels.clone())),
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
        self.hwnd_target_for(hwnd, width, height, vsync, aylis_core::gpu::choice())
    }

    /// Como [`D2dRenderer::hwnd_target`], na placa que `choice` pede (a do form; veja
    /// [`aylis_core::gpu::choice_for`]). Com `FollowMonitor`, [`HwndTarget::follow_monitor`]
    /// troca de placa quando a janela muda de monitor.
    pub fn hwnd_target_for(&self, hwnd: isize, width: u32, height: u32, vsync: bool, choice: aylis_core::gpu::GpuChoice) -> Result<HwndTarget, Error> {
        self.hwnd_target_on(hwnd, width, height, vsync, choice, aylis_core::gpu::present(), None)
    }

    /// Como [`D2dRenderer::hwnd_target_for`], com o modo de apresentar do form (a propriedade
    /// `present`; veja [`aylis_core::gpu::present_for`]).
    pub fn hwnd_target_with(&self, hwnd: isize, width: u32, height: u32, vsync: bool, choice: aylis_core::gpu::GpuChoice, present: aylis_core::gpu::PresentMode) -> Result<HwndTarget, Error> {
        self.hwnd_target_on(hwnd, width, height, vsync, choice, present, None)
    }

    /// O alvo na placa `luid` (a que a janela vai seguir) ou, sem ela, na que `choice` pede.
    #[allow(clippy::too_many_arguments)]
    fn hwnd_target_on(&self, hwnd: isize, width: u32, height: u32, vsync: bool, choice: aylis_core::gpu::GpuChoice, present: aylis_core::gpu::PresentMode, luid: Option<i64>) -> Result<HwndTarget, Error> {
        let monitor = monitor_of(hwnd).0 as isize;
        if diagnostic("AYLIS_D2D_TARGET").is_none_or(|v| v != "hwnd") {
            let chosen = luid.and_then(|l| adapter_where(|d| desc_luid(d) == l)).or_else(|| adapter_for(&choice, hwnd));
            let swap = self.swap_chain(hwnd, width, height, chosen, present);
            if diagnostic("AYLIS_D2D_DEBUG").is_some() {
                aylis_core::stderr_line!("aylis-d2d: alvo {}", match &swap { Ok(_) => "swap chain".to_string(), Err(e) => format!("da janela ({e})") });
            }
            if let Ok(swap) = swap {
                return Ok(HwndTarget { kind: Kind::Swap(Box::new(swap)), hwnd, size: (width, height), vsync, cache: DeviceCache::default(), choice, present, monitor });
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
        Ok(HwndTarget { kind: Kind::Hwnd(target), hwnd, size: (width, height), vsync, cache: DeviceCache::default(), choice, present, monitor })
    }

    /// O dispositivo da placa `chosen` (`None`: a padrão): o já criado, se ainda vale (não foi
    /// perdido), ou um novo, que fica guardado para as próximas janelas.
    fn shared_device(&self, chosen: Option<&IDXGIAdapter>) -> Result<(ID3D11Device, ID3D11DeviceContext, ID2D1Device), Error> {
        let key = chosen.and_then(luid_of);
        let mut devices = self.devices.borrow_mut();
        // SAFETY: só pergunta ao dispositivo se ele foi perdido.
        devices.retain(|d| unsafe { d.device.GetDeviceRemovedReason() }.is_ok());
        if let Some(d) = devices.iter().find(|d| d.key == key && (chosen.is_none() || key.is_some())) {
            return Ok((d.device.clone(), d.context.clone(), d.d2d.clone()));
        }
        // SAFETY: chamadas COM do Direct3D e do Direct2D; os ponteiros de saída são locais.
        unsafe {
            let mut device = None;
            let mut context = None;
            D3D11CreateDevice(
                chosen,
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
            let d2d = self.factory1.CreateDevice(&dxgi)?;
            // Uma placa sem LUID (não deveria acontecer) não entra no guardado.
            if chosen.is_none() || key.is_some() {
                devices.push(SharedDevice { key, device: device.clone(), context: context.clone(), d2d: d2d.clone() });
            }
            Ok((device, context, d2d))
        }
    }

    /// Direct3D 11 + swap chain "flip" + contexto do Direct2D numa janela.
    fn swap_chain(&self, hwnd: isize, width: u32, height: u32, chosen: Option<IDXGIAdapter>, present: aylis_core::gpu::PresentMode) -> Result<Swap, Error> {
        let (device, context, d2d) = self.shared_device(chosen.as_ref())?;
        unsafe {
            let dxgi: IDXGIDevice = device.cast()?;
            let dc = d2d.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?;
            let adapter = dxgi.GetAdapter()?;
            let adapter_desc = adapter.GetDesc()?;
            let factory: IDXGIFactory2 = adapter.GetParent()?;
            // Transparente: a composição com o alfa por pixel (só a swap chain da composição
            // aceita o alfa pré-multiplicado).
            let transparent = present == aylis_core::gpu::PresentMode::Transparent;
            let compose = present == aylis_core::gpu::PresentMode::Composition || transparent;
            // O modelo de cópia (*blit*): o DWM sempre compõe, sem MPO nem independent flip.
            let blit = present == aylis_core::gpu::PresentMode::Composed;
            // Tearing só na swap chain da janela: a composição sempre espera o DWM.
            let tearing = !compose
                && !blit
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
                AlphaMode: if transparent { DXGI_ALPHA_MODE_PREMULTIPLIED } else { DXGI_ALPHA_MODE_IGNORE },
                Flags: flags,
            };
            // DirectComposition: a swap chain sem janela vira o conteúdo de uma camada que o DWM
            // junta à janela (como os navegadores). Se não der, a swap chain da janela.
            let composed = if compose { compose_into(&factory, &device, &dxgi, hwnd, &desc).ok() } else { None };
            // No modelo de cópia, um buffer só (o quadro guardado vai inteiro para ele a cada
            // apresentação); se a placa recusar, a swap chain da janela de sempre.
            // A swap chain da janela não aceita o alfa pré-multiplicado: sem a composição, opaca.
            let desc = if composed.is_none() && transparent { DXGI_SWAP_CHAIN_DESC1 { AlphaMode: DXGI_ALPHA_MODE_IGNORE, ..desc } } else { desc };
            let copied = if blit {
                let blit_desc = DXGI_SWAP_CHAIN_DESC1 { SwapEffect: DXGI_SWAP_EFFECT_SEQUENTIAL, BufferCount: 1, ..desc };
                factory.CreateSwapChainForHwnd(&device, HWND(hwnd as *mut _), &blit_desc, None, None).ok()
            } else {
                None
            };
            let (swap, composition) = match (composed, copied) {
                (Some((swap, layer)), _) => (swap, Some(layer)),
                (None, Some(swap)) => {
                    let _ = factory.MakeWindowAssociation(HWND(hwnd as *mut _), DXGI_MWA_NO_ALT_ENTER);
                    (swap, None)
                }
                (None, None) => {
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
                    luid: luid_of(&adapter).unwrap_or_default(),
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

    /// Como [`render_rgba`](Self::render_rgba), desenhando na placa de vídeo (um contexto do
    /// Direct2D sobre uma textura do Direct3D 11, como a janela) e lendo os pixels de volta: o
    /// caminho da janela sem janela, para os testes do que só existe na placa (o compositor da
    /// mistura em Direct3D, a fila de envios). `None` sem placa.
    pub fn render_rgba_on_gpu(&self, width: u32, height: u32, scale: f32, background: Color, paint: impl FnOnce(&mut D2dCanvas<'_>)) -> Option<Vec<u8>> {
        let (_, _, d2d) = self.shared_device(None).ok()?;
        // SAFETY: chamadas COM do Direct2D com propriedades locais; o mapa da cópia para a CPU
        // é lido pelo passo que ele informa e solto antes da volta.
        unsafe {
            let dc = d2d.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE).ok()?;
            let size = D2D_SIZE_U { width: width.max(1), height: height.max(1) };
            let format = D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED };
            let props = |options| D2D1_BITMAP_PROPERTIES1 { pixelFormat: format, dpiX: 96.0, dpiY: 96.0, bitmapOptions: options, colorContext: std::mem::ManuallyDrop::new(None) };
            let frame = dc.CreateBitmap(size, None, 0, &props(D2D1_BITMAP_OPTIONS_TARGET)).ok()?;
            let readable = dc.CreateBitmap(size, None, 0, &props(D2D1_BITMAP_OPTIONS_CPU_READ | D2D1_BITMAP_OPTIONS_CANNOT_DRAW)).ok()?;
            dc.SetTarget(&frame);
            let target: ID2D1RenderTarget = dc.cast().ok()?;
            let mut cache = DeviceCache::default();
            self.draw_on(&target, &mut cache, None, scale, background, None, paint);
            target.EndDraw(None, None).ok()?;
            readable.CopyFromBitmap(None, &frame, None).ok()?;
            let map = readable.Map(D2D1_MAP_OPTIONS_READ).ok()?;
            let stride = map.pitch as usize;
            let mut bgra = Vec::with_capacity((size.width * size.height * 4) as usize);
            for y in 0..size.height as usize {
                bgra.extend_from_slice(std::slice::from_raw_parts(map.bits.add(y * stride), size.width as usize * 4));
            }
            let _ = readable.Unmap();
            dc.SetTarget(None);
            Some(unpremultiply_bgra_to_rgba(&bgra))
        }
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
            // O ClearType (`text_antialias` do projeto) só com o fundo opaco: o subpixel
            // precisa da cor de trás; a janela transparente fica em cinza.
            let cleartype = background.a == 255 && aylis_core::canvas::text_antialias() == aylis_core::TextAntialias::ClearType;
            self.set_text_mode(target, cleartype);
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
                push_layer_on(target, group.clone(), D2D1_ANTIALIAS_MODE_ALIASED, 1.0, cleartype);
                pushes.push(Push::Layer(group, 1.0, Matrix3x2::identity(), 0));
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
                // Fundo com alfa (a janela transparente): o retângulo substitui o pixel, como o
                // `Clear`, em vez de misturar com o quadro anterior.
                let copy = (background.a < 255).then(|| target.cast::<ID2D1DeviceContext>().ok()).flatten();
                if let Some(dc) = &copy {
                    dc.SetPrimitiveBlend(D2D1_PRIMITIVE_BLEND_COPY);
                }
                for a in areas {
                    target.FillRectangle(&pixels(a), &brush);
                }
                if let Some(dc) = &copy {
                    dc.SetPrimitiveBlend(D2D1_PRIMITIVE_BLEND_SOURCE_OVER);
                }
                target.SetTransform(&base);
            }
            let mut canvas = D2dCanvas {
                target: target.clone(),
                masks: Vec::new(),
                cached: Vec::new(),
                blends: Vec::new(),
                renderer: self,
                brush,
                transform: base,
                frames: Vec::new(),
                pushes,
                cache,
                gpu,
                draws: 0,
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
    /// Texturas abertas (pelo id e pelo handle da textura), com a hora em que foram usadas
    /// pela última vez.
    surfaces: HashMap<(u64, isize), (ID2D1Bitmap1, std::time::Instant)>,
    fences: HashMap<isize, ID3D11Fence>,
    /// Cercas a marcar quando o quadro terminar (a textura foi lida).
    signals: Vec<(ID3D11Fence, u64)>,
    frame: u64,
}

/// Quanto tempo uma textura sem ser desenhada fica aberta no cache.
const SURFACE_KEEP: std::time::Duration = std::time::Duration::from_secs(2);
/// Quantas texturas abertas no máximo (as mais antigas saem antes).
const SURFACE_MAX: usize = 64;

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
        // A textura fica aberta enquanto for desenhada nos últimos segundos, e não só nos
        // últimos 2 quadros: o vídeo gira várias texturas (até 12 por vídeo) e a tela pinta
        // mais vezes do que a câmera manda quadros (89 contra 15), e cada textura saía do cache
        // antes de voltar — o Direct2D reabria a textura a cada quadro de câmera (~8% da thread
        // da interface num app com várias câmeras, 13/11). O teto limita a memória presa.
        let now = std::time::Instant::now();
        self.surfaces.retain(|_, (_, used)| now.duration_since(*used) < SURFACE_KEEP);
        if self.surfaces.len() > SURFACE_MAX {
            let mut ages: Vec<((u64, isize), std::time::Instant)> = self.surfaces.iter().map(|(k, (_, used))| (*k, *used)).collect();
            ages.sort_by_key(|(_, used)| *used);
            for (key, _) in ages.into_iter().take(self.surfaces.len() - SURFACE_MAX) {
                self.surfaces.remove(&key);
            }
        }
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
    /// Solto para recriar em outra placa ([`HwndTarget::rebuild`]); se a recriação falhou, o
    /// próximo quadro tenta de novo, como num dispositivo perdido.
    Released,
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
    layers: aylis_core::LayerCache<ID2D1Bitmap>,
    /// Os bitmaps dos quadros que o app soltou, com quantas pinturas têm de reserva: o quadro
    /// novo de uma câmera, do mesmo tamanho, copia os pixels num deles em vez de criar outro na
    /// placa. Só os soltos há uma pintura ou mais (o da pintura anterior a placa ainda pode
    /// estar lendo, e copiar nele faria a CPU esperar); o que passar de duas sai.
    spare_frames: Vec<(ID2D1Bitmap, u32, u32, u8)>,
    /// O `frame_reuse_kb` próprio do form desta janela ([`HwndTarget::set_frame_reuse_kb`]);
    /// `None`: o do projeto.
    frame_reuse: Option<u32>,
    /// As peças das camadas com mistura, reaproveitadas de uma camada para a outra e de um
    /// quadro para o outro (criar a cada camada custava ~70 µs: 256 marcas d'água por quadro).
    blend_pool: BlendPool,
    /// O `blend_compositor` próprio do form desta janela ([`HwndTarget::set_blend_compositor`]);
    /// `None`: o do projeto.
    blend_compositor: Option<aylis_core::gpu::BlendCompositor>,
}

/// O compositor da mistura de uma janela, em três níveis: no diagnóstico,
/// `AYLIS_BLEND_COMPOSITOR` (`direct2d` ou `direct3d`, para medir sem mexer no projeto); o do
/// form (`form`); o do projeto, que o `RenderSettings` troca em execução.
fn blend_compositor(form: Option<aylis_core::gpu::BlendCompositor>) -> aylis_core::gpu::BlendCompositor {
    use aylis_core::PropEnum;
    static FORCED: std::sync::OnceLock<Option<aylis_core::gpu::BlendCompositor>> = std::sync::OnceLock::new();
    let forced = *FORCED.get_or_init(|| diagnostic("AYLIS_BLEND_COMPOSITOR").and_then(|v| aylis_core::gpu::BlendCompositor::from_name(v.trim())));
    forced.or(form).unwrap_or_else(aylis_core::gpu::blend_compositor)
}

/// O que as camadas com mistura ([`Canvas::save_layer_blend`]) reaproveitam: os alvos fora
/// da tela e os bitmaps do fundo, por tamanho, e os efeitos, pelo contexto que os criou.
#[derive(Default)]
struct BlendPool {
    /// Os alvos devolvidos, com o [`BlendPool::flushes`] de quando voltaram: só servem de
    /// novo depois de um envio à placa (o desenho que lê o bitmap deles ainda está na fila
    /// do alvo principal, e o Direct2D mandaria o desenho novo antes).
    targets: Vec<(u32, u32, u64, ID2D1BitmapRenderTarget)>,
    /// Quantos envios do alvo principal à placa (`Flush`) a mistura já fez.
    flushes: u64,
    backdrops: Vec<(u32, u32, ID2D1Bitmap)>,
    /// O efeito Blend e o Opacity, com o contexto que os criou (guardado junto: um contexto
    /// solto e outro criado no mesmo endereço não pegam o efeito errado).
    effects: Vec<(ID2D1DeviceContext, ID2D1Effect, ID2D1Effect)>,
    /// O compositor em Direct3D (`blend_compositor = direct3d`), criado no primeiro uso.
    /// `Some(None)`: não deu nesta placa (fica o efeito do Direct2D).
    compositor: Option<Option<compositor::Compositor>>,
}

impl BlendPool {
    /// Quantas peças de cada tipo ficam guardadas (as camadas abertas ao mesmo tempo, umas
    /// dentro das outras, e tamanhos diferentes na mesma tela).
    const KEEP: usize = 8;

    fn take_target(&mut self, w: u32, h: u32) -> Option<ID2D1BitmapRenderTarget> {
        let flushes = self.flushes;
        let at = self.targets.iter().position(|(tw, th, at, _)| (*tw, *th) == (w, h) && *at < flushes)?;
        Some(self.targets.remove(at).3)
    }

    fn give_target(&mut self, w: u32, h: u32, target: ID2D1BitmapRenderTarget) {
        if self.targets.len() >= Self::KEEP {
            self.targets.remove(0);
        }
        self.targets.push((w, h, self.flushes, target));
    }

    /// O bitmap do fundo de `w × h`: guardado para a próxima camada do mesmo tamanho (a placa
    /// ordena a cópia nova depois da leitura da anterior).
    fn backdrop(&mut self, target: &ID2D1RenderTarget, w: u32, h: u32) -> windows_core::Result<ID2D1Bitmap> {
        if let Some((_, _, b)) = self.backdrops.iter().find(|(bw, bh, _)| (*bw, *bh) == (w, h)) {
            return Ok(b.clone());
        }
        // SAFETY: chamada COM do Direct2D com as propriedades locais.
        let bitmap = unsafe {
            let props = D2D1_BITMAP_PROPERTIES { pixelFormat: target.GetPixelFormat(), dpiX: 96.0, dpiY: 96.0 };
            target.CreateBitmap(D2D_SIZE_U { width: w, height: h }, None, 0, &props)?
        };
        if self.backdrops.len() >= Self::KEEP {
            self.backdrops.remove(0);
        }
        self.backdrops.push((w, h, bitmap.clone()));
        Ok(bitmap)
    }

    /// O Blend e o Opacity do contexto `dc` (criados uma vez).
    fn effects(&mut self, dc: &ID2D1DeviceContext) -> windows_core::Result<(ID2D1Effect, ID2D1Effect)> {
        let key = windows_core::Interface::as_raw(dc);
        if let Some((_, blend, fade)) = self.effects.iter().find(|(k, ..)| windows_core::Interface::as_raw(k) == key) {
            return Ok((blend.clone(), fade.clone()));
        }
        // SAFETY: chamadas COM do Direct2D no contexto válido.
        let (blend, fade) = unsafe { (dc.CreateEffect(&CLSID_D2D1Blend)?, dc.CreateEffect(&CLSID_D2D1Opacity)?) };
        if self.effects.len() >= Self::KEEP {
            self.effects.remove(0);
        }
        self.effects.push((dc.clone(), blend.clone(), fade.clone()));
        Ok((blend, fade))
    }
}

impl Default for DeviceCache {
    fn default() -> Self {
        Self {
            bitmaps: ImageCache::named("Direct2D · arquivos na placa"),
            frames: ImageCache::named("Direct2D · pixels na placa"),
            gradients: RefCell::default(),
            shadows: RefCell::default(),
            sweeps: RefCell::default(),
            layers: aylis_core::LayerCache::named("Direct2D · camadas guardadas"),
            spare_frames: Vec::new(),
            frame_reuse: None,
            blend_pool: BlendPool::default(),
            blend_compositor: None,
        }
    }
}

impl DeviceCache {
    /// Solta os bitmaps das imagens que o app já soltou: ninguém mais pede aquele id (uma
    /// prévia trocada a cada meio segundo enchia a placa até o teto).
    fn forget_dropped(&mut self) {
        // Os de reserva envelhecem uma pintura (os de mais de duas saem); os soltos agora entram.
        self.spare_frames.retain_mut(|s| {
            s.3 += 1;
            s.3 <= 2
        });
        let small = self.frames.drain_dropped().into_iter().filter(|(_, w, h)| reuses_frame(*w, *h, self.frame_reuse));
        self.spare_frames.extend(small.map(|(b, w, h)| (b, w, h, 0)));
        self.bitmaps.forget_dropped();
    }

    /// Um bitmap de reserva de `w × h` (de um quadro já solto).
    fn take_spare(&mut self, w: u32, h: u32) -> Option<ID2D1Bitmap> {
        if !reuses_frame(w, h, self.frame_reuse) {
            return None;
        }
        let at = self.spare_frames.iter().position(|(_, bw, bh, age)| (*bw, *bh) == (w, h) && *age >= 1)?;
        Some(self.spare_frames.swap_remove(at).0)
    }
}

/// Se o quadro de `w × h` reaproveita o bitmap do anterior, em três níveis
/// ([`aylis_core::image_cache::frame_reuse_limit`]): no diagnóstico, `AYLIS_FRAME_REUSE_KB`
/// (para medir outro limite na sua placa sem mexer no projeto; `0` desliga); o
/// `frame_reuse_kb` do form da janela (`form`); o do projeto, que o `RenderSettings` troca em
/// execução.
fn reuses_frame(w: u32, h: u32, form: Option<u32>) -> bool {
    static FORCED: std::sync::OnceLock<Option<u32>> = std::sync::OnceLock::new();
    let forced = *FORCED.get_or_init(|| diagnostic("AYLIS_FRAME_REUSE_KB").and_then(|v| v.trim().parse().ok()));
    aylis_core::image_cache::reuses_frame(w, h, aylis_core::image_cache::frame_reuse_limit(forced, form))
}

/// O alvo de desenho de uma janela.
pub struct HwndTarget {
    kind: Kind,
    hwnd: isize,
    size: (u32, u32),
    vsync: bool,
    /// Bitmaps já enviados à GPU (são do alvo: refeitos se ele for recriado).
    cache: DeviceCache,
    /// A placa pedida (a do form ou a do projeto), para recriar na mesma regra.
    choice: aylis_core::gpu::GpuChoice,
    /// O modo de apresentar (o do form ou o do projeto), para recriar no mesmo.
    present: aylis_core::gpu::PresentMode,
    /// O monitor (`HMONITOR`) em que a janela estava ao conferir a placa pela última vez.
    monitor: isize,
}

/// A janela fechou: o dispositivo pode continuar com as outras janelas, então o alvo solta o que
/// é dele na hora ([`HwndTarget::release`]).
impl Drop for HwndTarget {
    fn drop(&mut self) {
        self.release();
    }
}

impl HwndTarget {
    /// Com a placa que segue o monitor: se a janela passou para um monitor de outra placa,
    /// recria o alvo nela (o mesmo caminho do dispositivo perdido: os bitmaps voltam do que
    /// o renderizador guarda na CPU). `Ok(true)` se trocou; o próximo quadro sai inteiro.
    /// `force`: confere mesmo no monitor de antes (os monitores mudaram).
    ///
    /// No mesmo monitor custa só o `MonitorFromWindow`. Com `AYLIS_GPU_SWITCH=warp`
    /// (diagnóstico), cada troca de monitor alterna entre a placa dele e o WARP: testa a troca
    /// de placa numa máquina com uma placa só.
    pub fn follow_monitor(&mut self, renderer: &D2dRenderer, force: bool) -> Result<bool, Error> {
        if self.choice != aylis_core::gpu::GpuChoice::FollowMonitor {
            return Ok(false);
        }
        let monitor = monitor_of(self.hwnd);
        if !force && monitor.0 as isize == self.monitor {
            return Ok(false);
        }
        self.monitor = monitor.0 as isize;
        let current = match &self.kind {
            Kind::Swap(swap) => Some(swap.link.luid),
            Kind::Released => None,
            Kind::Hwnd(_) => return Ok(false),
        };
        // SAFETY: chamadas COM do DXGI, sem ponteiros guardados.
        let mut wanted = unsafe { CreateDXGIFactory1::<IDXGIFactory6>().ok().and_then(|f| adapter_of_monitor(&f, monitor)).and_then(|a| a.cast::<IDXGIAdapter>().ok()).and_then(|a| luid_of(&a)) };
        if diagnostic("AYLIS_GPU_SWITCH").is_some_and(|v| v == "warp") && wanted == current {
            wanted = adapter_where(is_software).and_then(|a| luid_of(&a));
        }
        // Um monitor sem placa conhecida (sessão remota, projetor sem driver): fica onde está.
        if wanted.is_none() || wanted == current {
            return Ok(false);
        }
        self.rebuild(renderer, wanted)?;
        Ok(true)
    }

    /// Recria o alvo na placa da regra, soltando antes a swap chain e o alvo de composição
    /// atuais: o Windows aceita uma swap chain "flip" e um alvo do DirectComposition por
    /// janela, e criar os novos com os antigos vivos falha (ou cai para o alvo antigo).
    fn rebuild(&mut self, renderer: &D2dRenderer, luid: Option<i64>) -> Result<(), Error> {
        self.release();
        let (width, height) = self.size;
        *self = renderer.hwnd_target_on(self.hwnd, width, height, self.vsync, self.choice.clone(), self.present, luid)?;
        Ok(())
    }

    /// Solta a swap chain, o alvo e os bitmaps do alvo, e o Direct3D executa as liberações
    /// pendentes: sem o `Flush`, a swap chain antiga continua presa à janela e, com o dispositivo
    /// dividido entre as janelas ([`D2dRenderer::shared_device`]), a memória da janela fechada
    /// (os buffers e as imagens na placa) ficava presa até outro quadro.
    fn release(&mut self) {
        self.cache = DeviceCache::default();
        if let Kind::Swap(old) = std::mem::replace(&mut self.kind, Kind::Released) {
            let context = old.link.context4.clone();
            let device: Option<IDXGIDevice3> = old.link.device.cast().ok();
            // SAFETY: o contexto do Direct2D solta o buffer de trás; o do Direct3D executa as
            // liberações pendentes.
            unsafe { old.dc.SetTarget(None::<&ID2D1Image>) };
            drop(old);
            if let Some(c) = context {
                unsafe {
                    c.ClearState();
                    c.Flush();
                }
            }
            // O driver guarda a memória solta para reusar: devolve (a janela fechou; as outras
            // do mesmo dispositivo continuam com a delas).
            if let Some(d) = device {
                // SAFETY: chamada COM sem argumentos, depois do `ClearState`.
                unsafe { d.Trim() };
            }
        }
    }

    /// O `frame_reuse_kb` próprio do form desta janela (`None`: o do projeto). A plataforma
    /// passa a cada quadro; vale a partir do próximo quadro de vídeo.
    pub fn set_frame_reuse_kb(&mut self, kb: Option<u32>) {
        self.cache.frame_reuse = kb;
    }

    /// O `blend_compositor` próprio do form desta janela (`None`: o do projeto). A plataforma
    /// passa a cada quadro; vale a partir da próxima camada com mistura.
    pub fn set_blend_compositor(&mut self, compositor: Option<aylis_core::gpu::BlendCompositor>) {
        self.cache.blend_compositor = compositor;
    }

    /// Troca a regra da placa (a propriedade `gpu` do form mudou) e recria o alvo se a placa
    /// pedida não é a de agora.
    pub fn set_choice(&mut self, renderer: &D2dRenderer, choice: aylis_core::gpu::GpuChoice) -> Result<bool, Error> {
        if choice == self.choice {
            return Ok(false);
        }
        let current = match &self.kind {
            Kind::Swap(s) => Some(s.link.luid),
            Kind::Hwnd(_) | Kind::Released => None,
        };
        let wanted = adapter_for(&choice, self.hwnd).and_then(|a| luid_of(&a));
        self.choice = choice;
        self.monitor = monitor_of(self.hwnd).0 as isize;
        // `None` pedido: a padrão do sistema, que não dá para comparar sem abrir; recria.
        if wanted.is_some() && wanted == current {
            return Ok(false);
        }
        self.rebuild(renderer, wanted)?;
        Ok(true)
    }

    /// O modo de apresentar do form mudou (a propriedade `present`): recria o alvo no modo novo
    /// (`Ok(true)`: recriou).
    pub fn set_present(&mut self, renderer: &D2dRenderer, present: aylis_core::gpu::PresentMode) -> Result<bool, Error> {
        if present == self.present {
            return Ok(false);
        }
        self.present = present;
        let luid = match &self.kind {
            Kind::Swap(s) => Some(s.link.luid),
            Kind::Hwnd(_) | Kind::Released => None,
        };
        self.rebuild(renderer, luid)?;
        Ok(true)
    }

    /// Como o último quadro chegou ao monitor, pelo DXGI (`GetFrameStatisticsMedia`):
    /// `composed` (pelo DWM), `overlay` (num plano de overlay, o MPO), `none` (direto, o
    /// independent flip) ou `failed`. `None`: ainda sem quadro, ou o sistema não diz.
    pub fn presentation(&self) -> Option<&'static str> {
        let Kind::Swap(s) = &self.kind else { return None };
        let media: IDXGISwapChainMedia = s.swap.cast().ok()?;
        let mut stats = DXGI_FRAME_STATISTICS_MEDIA::default();
        // SAFETY: chamada COM que preenche a estrutura dada.
        unsafe { media.GetFrameStatisticsMedia(&mut stats) }.ok()?;
        Some(match stats.CompositionMode {
            DXGI_FRAME_PRESENTATION_MODE_COMPOSED => "composed",
            DXGI_FRAME_PRESENTATION_MODE_OVERLAY => "overlay",
            DXGI_FRAME_PRESENTATION_MODE_NONE => "none",
            _ => "failed",
        })
    }

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
            // A troca de placa não conseguiu recriar: tenta de novo, como num dispositivo perdido.
            Kind::Released => Err(DXGI_ERROR_DEVICE_REMOVED.into()),
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
                self.rebuild(renderer, None)?;
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
    /// Máscara, opacidade, transformação e o [`D2dCanvas::draws`] de quando abriu (a camada
    /// ainda vazia pode virar isolada sem perder nada: veja `isolate_open_layer`).
    Layer(Option<ID2D1Geometry>, f32, Matrix3x2, usize),
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
    /// A chave do núcleo (para soltar pela `forget_cached`).
    owner: u64,
    dest: D2D_RECT_F,
}

struct MaskState {
    main: ID2D1RenderTarget,
    pushes: Vec<Push>,
    content: ID2D1BitmapRenderTarget,
    mask: Option<ID2D1BitmapRenderTarget>,
    /// A máscara é de luminância (o `<mask>` do SVG): o brilho vira o alfa antes de recortar.
    luminance: bool,
}

/// Uma camada fora da tela aberta: a com modo de mistura ([`Canvas::save_layer_blend`]) ou a
/// de opacidade isolada (`mode` `None`). O alvo de antes, a camada (do tamanho do pedaço) e o
/// pedaço (em pixels do alvo de antes) onde ela se junta ao fundo no `restore`.
struct BlendState {
    main: ID2D1RenderTarget,
    pushes: Vec<Push>,
    content: ID2D1BitmapRenderTarget,
    mode: Option<D2D1_BLEND_MODE>,
    /// O modo na ordem do `aylis_core::BlendMode` (o do shader do compositor em Direct3D).
    index: u32,
    opacity: f32,
    area: D2D_RECT_U,
    /// O `frames.len()` do `save` que a abriu (o `restore` dele fecha a camada).
    frame: usize,
}

pub struct D2dCanvas<'a> {
    /// O alvo atual (o da janela, ou um fora da tela enquanto desenha uma máscara).
    target: ID2D1RenderTarget,
    masks: Vec<MaskState>,
    /// Os desenhos guardados abertos (`None`: não deu para guardar, desenha direto).
    cached: Vec<Option<CacheState>>,
    /// As camadas com modo de mistura abertas.
    blends: Vec<BlendState>,
    renderer: &'a D2dRenderer,
    /// Um pincel sólido reaproveitado (só troca a cor).
    brush: ID2D1SolidColorBrush,
    transform: Matrix3x2,
    frames: Vec<Frame>,
    pushes: Vec<Push>,
    cache: &'a mut DeviceCache,
    /// O Direct3D por baixo (só no alvo por swap chain), para compor texturas de fora.
    gpu: Option<&'a mut GpuLink>,
    /// Quantos desenhos já foram feitos (para saber se uma camada aberta ainda está vazia).
    draws: usize,
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
            let mask = if state.luminance { self.luminance_to_alpha(&mask).unwrap_or(mask) } else { mask };
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

    /// A máscara com o brilho no alfa (o efeito Color Matrix nas cores pré-multiplicadas: o
    /// brilho vezes o alfa, com os pesos do SVG e do `LumaColorFilter` do Skia), num bitmap do
    /// mesmo tamanho.
    fn luminance_to_alpha(&self, mask: &ID2D1Bitmap) -> Option<ID2D1Bitmap> {
        let out = self.offscreen()?;
        // SAFETY: chamadas COM do Direct2D; os bytes passados ao `SetValue` vivem até a volta.
        unsafe {
            let dc: ID2D1DeviceContext = out.cast().ok()?;
            let effect = dc.CreateEffect(&CLSID_D2D1ColorMatrix).ok()?;
            effect.SetInput(0, mask, true);
            // Linhas R, G, B, A e a constante; colunas R', G', B', A'.
            #[rustfmt::skip]
            let matrix: [f32; 20] = [
                0.0, 0.0, 0.0, 0.2125,
                0.0, 0.0, 0.0, 0.7154,
                0.0, 0.0, 0.0, 0.0721,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            let bytes: Vec<u8> = matrix.iter().flat_map(|v| v.to_le_bytes()).collect();
            effect.SetValue(D2D1_COLORMATRIX_PROP_COLOR_MATRIX.0 as u32, D2D1_PROPERTY_TYPE_MATRIX_5X4, &bytes).ok()?;
            let mode = (D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED.0 as u32).to_le_bytes();
            effect.SetValue(D2D1_COLORMATRIX_PROP_ALPHA_MODE.0 as u32, D2D1_PROPERTY_TYPE_ENUM, &mode).ok()?;
            let output = effect.GetOutput().ok()?;
            dc.SetTransform(&Matrix3x2::identity());
            dc.DrawImage(&output, None, None, D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR, D2D1_COMPOSITE_MODE_SOURCE_COPY);
            out.EndDraw(None, None).ok()?;
            out.GetBitmap().ok()
        }
    }

    /// O pedaço do alvo (em pixels) que os recortes abertos deixam desenhar.
    fn clip_area(&self) -> D2D_RECT_U {
        // SAFETY: chamada COM sem argumentos num alvo válido.
        let size = unsafe { self.target.GetPixelSize() };
        let (mut x0, mut y0, mut x1, mut y1) = (0.0f32, 0.0f32, size.width as f32, size.height as f32);
        for p in &self.pushes {
            if let Push::Clip(r, m) = p {
                let map = |x: f32, y: f32| (x * m.M11 + y * m.M21 + m.M31, x * m.M12 + y * m.M22 + m.M32);
                let pts = [map(r.left, r.top), map(r.right, r.top), map(r.left, r.bottom), map(r.right, r.bottom)];
                x0 = x0.max(pts.iter().map(|p| p.0).fold(f32::MAX, f32::min));
                y0 = y0.max(pts.iter().map(|p| p.1).fold(f32::MAX, f32::min));
                x1 = x1.min(pts.iter().map(|p| p.0).fold(f32::MIN, f32::max));
                y1 = y1.min(pts.iter().map(|p| p.1).fold(f32::MIN, f32::max));
            }
        }
        D2D_RECT_U {
            left: x0.floor().max(0.0) as u32,
            top: y0.floor().max(0.0) as u32,
            right: (x1.ceil().max(0.0) as u32).min(size.width),
            bottom: (y1.ceil().max(0.0) as u32).min(size.height),
        }
    }

    /// Abre uma camada num alvo fora da tela do tamanho de `area` (pixels do alvo de agora): a
    /// mistura `mode`, ou a camada de opacidade isolada (`None`). Ela fecha no `restore` que
    /// encontrar `frame` saves abertos. `false`: não deu (pedaço vazio, sem memória).
    fn open_offscreen(&mut self, mode: Option<(D2D1_BLEND_MODE, u32)>, opacity: f32, area: D2D_RECT_U, frame: usize) -> bool {
        let (w, h) = (area.right.saturating_sub(area.left), area.bottom.saturating_sub(area.top));
        if w == 0 || h == 0 {
            return false;
        }
        // Um alvo do mesmo tamanho já usado (devolvido no `end_blend_inner`), ou um novo.
        let content = match self.cache.blend_pool.take_target(w, h) {
            Some(content) => content,
            // SAFETY: chamadas COM do Direct2D; o alvo novo fica guardado no `BlendState` até o
            // `end_blend_inner`.
            None => match unsafe {
                self.target.CreateCompatibleRenderTarget(
                    Some(&D2D_SIZE_F { width: w as f32, height: h as f32 }),
                    Some(&D2D_SIZE_U { width: w, height: h }),
                    None,
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE,
                )
            } {
                Ok(content) => content,
                Err(_) => return false,
            },
        };
        let Ok(as_target) = content.cast::<ID2D1RenderTarget>() else { return false };
        // SAFETY: o alvo está fora de um `BeginDraw` (novo ou devolvido depois do `EndDraw`); o
        // `BeginDraw` fecha no `end_blend_inner`.
        unsafe {
            content.BeginDraw();
            content.SetTransform(&Matrix3x2::identity());
            content.Clear(Some(&color(Color::TRANSPARENT)));
        }
        let main = std::mem::replace(&mut self.target, as_target);
        let pushes = std::mem::take(&mut self.pushes);
        let (mode, index) = (mode.map(|m| m.0), mode.map_or(0, |m| m.1));
        self.blends.push(BlendState { main, pushes, content, mode, index, opacity, area, frame });
        // As mesmas coordenadas, com o canto do pedaço na origem do alvo novo.
        let m = mul(&self.transform, &Matrix3x2::translation(-(area.left as f32), -(area.top as f32)));
        self.set_transform(m);
        true
    }

    /// A mistura lê o alvo, e para isso o Direct2D fecha as camadas abertas nele: uma camada
    /// de opacidade aberta era composta duas vezes (o que já estava nela) e a mistura se juntava
    /// ao que está fora dela. Fica igual ao Skia, ao navegador e ao CSS (a camada com opacidade
    /// é um grupo isolado) assim: a camada de opacidade ainda vazia vira, aqui, uma camada fora
    /// da tela; a que já tem desenho não tem mais como, e as camadas de opacidade passam a ser
    /// abertas fora da tela desde a pintura seguinte.
    fn isolate_open_layer(&mut self) {
        if let Some(Push::Layer(None, opacity, _, draws)) = self.pushes.last() {
            let (opacity, empty, i) = (*opacity, *draws == self.draws, self.pushes.len() - 1);
            if opacity < 1.0 && empty {
                if let Some(pos) = self.frames.iter().rposition(|f| f.pushes == i) {
                    self.pop_until(i);
                    let area = self.clip_area();
                    if !self.open_offscreen(None, opacity, area, pos + 1) {
                        self.push_layer(None, opacity);
                    }
                    return;
                }
            }
        }
        if self.pushes.iter().any(|p| matches!(p, Push::Layer(None, o, ..) if *o < 1.0)) {
            self.renderer.isolate_layers.set(true);
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
        // O limite é em bytes (`image_cache.layers_mb`): cada destaque novo de um mapa é uma
        // imagem nova, e sai a usada há mais tempo.
        let d = state.dest;
        let bytes = (d.right - d.left).max(0.0) as usize * (d.bottom - d.top).max(0.0) as usize * 4;
        self.cache.layers.insert(state.owner, state.key, bitmap.clone(), bytes);
        self.draw_layer(&bitmap, &state.dest);
    }

    /// Fecha a camada com mistura aberta: volta ao alvo de antes e junta o pedaço dela ao
    /// fundo pelo efeito Blend (o fundo copiado do alvo, como o desfoque do fundo faz).
    fn end_blend_inner(&mut self) {
        let Some(BlendState { main, pushes, content, mode, index, opacity, area: a, frame: _ }) = self.blends.pop() else { return };
        self.pop_until(0);
        unsafe {
            let _ = content.EndDraw(None, None);
        }
        self.target = main;
        self.pushes = pushes;
        if a.right > a.left && a.bottom > a.top {
            self.compose_blend(&content, mode.map(|m| (m, index)), opacity, a);
        }
        // O alvo volta para uma próxima camada do mesmo tamanho.
        self.cache.blend_pool.give_target(a.right - a.left, a.bottom - a.top, content);
    }

    /// Junta a camada `content` (já fechada), do pedaço `a` em pixels, ao fundo do alvo atual.
    fn compose_blend(&mut self, content: &ID2D1BitmapRenderTarget, mode: Option<(D2D1_BLEND_MODE, u32)>, opacity: f32, a: D2D_RECT_U) {
        let piece = D2D_RECT_F { left: a.left as f32, top: a.top as f32, right: a.right as f32, bottom: a.bottom as f32 };
        let Some((mode, index)) = mode else {
            // A camada de opacidade isolada: o pedaço por cima, com a opacidade (os recortes e
            // as camadas de fora valem).
            // SAFETY: chamadas COM do Direct2D com o bitmap e o retângulo locais.
            unsafe {
                if let Ok(layer) = content.GetBitmap() {
                    self.target.SetTransform(&Matrix3x2::identity());
                    self.target.DrawBitmap(&layer, Some(&piece), opacity, D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, None);
                }
                self.target.SetTransform(&self.transform);
            }
            return;
        };
        let _ = (|| -> windows_core::Result<()> {
            unsafe {
                // O envio à placa vem antes de mexer no fundo e nos efeitos: o desenho da camada
                // anterior, ainda na fila, lê o mesmo bitmap e os mesmos efeitos.
                self.cache.blend_pool.flushes += 1;
                if blend_compositor(self.cache.blend_compositor) == aylis_core::gpu::BlendCompositor::Direct3d {
                    // O shader do Aylis; se não der aqui (recorte arredondado, alvo na CPU), o
                    // efeito do Direct2D logo abaixo.
                    self.target.Flush(None, None)?;
                    if self.compose_on_gpu(content, index, opacity, a) {
                        return Ok(());
                    }
                }
                let dc: ID2D1DeviceContext = self.target.cast()?;
                let backdrop = self.cache.blend_pool.backdrop(&self.target, a.right - a.left, a.bottom - a.top)?;
                self.without_pushes(|c| c.target.Flush(None, None).and_then(|_| backdrop.CopyFromRenderTarget(None, &c.target, Some(&a))))?;
                // A camada já é do tamanho do pedaço.
                let layer = content.GetBitmap()?;
                let (effect, fade) = self.cache.blend_pool.effects(&dc)?;
                effect.SetInput(0, &backdrop, true);
                if opacity < 1.0 {
                    fade.SetInput(0, &layer, true);
                    fade.SetValue(D2D1_OPACITY_PROP_OPACITY.0 as u32, D2D1_PROPERTY_TYPE_FLOAT, &opacity.to_le_bytes())?;
                    effect.SetInput(1, &fade.GetOutput()?, true);
                } else {
                    effect.SetInput(1, &layer, true);
                }
                effect.SetValue(D2D1_BLEND_PROP_MODE.0 as u32, D2D1_PROPERTY_TYPE_ENUM, &(mode.0 as u32).to_le_bytes())?;
                // O resultado já é o fundo com a camada: substitui o pedaço (os recortes de
                // fora continuam valendo). Recortado nele: a cópia apagaria o resto do recorte.
                dc.SetTransform(&Matrix3x2::identity());
                dc.PushAxisAlignedClip(&piece, D2D1_ANTIALIAS_MODE_ALIASED);
                let offset = windows_numerics::Vector2 { X: a.left as f32, Y: a.top as f32 };
                dc.DrawImage(&effect.GetOutput()?, Some(&offset), None, D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR, D2D1_COMPOSITE_MODE_SOURCE_COPY);
                dc.PopAxisAlignedClip();
                Ok(())
            }
        })();
        unsafe { self.target.SetTransform(&self.transform) };
    }

    /// Junta a camada pelo compositor em Direct3D ([`compositor::Compositor`]), se o alvo de
    /// agora é uma textura da placa e os recortes abertos são retângulos alinhados (viram o
    /// *scissor*). `false`: não deu, e quem chama usa o efeito do Direct2D. O que o Direct2D
    /// tinha na fila já foi mandado à placa.
    fn compose_on_gpu(&mut self, content: &ID2D1BitmapRenderTarget, index: u32, opacity: f32, a: D2D_RECT_U) -> bool {
        // Uma camada aberta (máscara, opacidade) junta o que vem depois a ela: o shader
        // escreveria por fora dela.
        if self.pushes.iter().any(|p| matches!(p, Push::Layer(..))) {
            return false;
        }
        if self.pushes.iter().any(|p| matches!(p, Push::Clip(_, m) if m.M12 != 0.0 || m.M21 != 0.0)) {
            return false;
        }
        let clip = self.clip_area();
        let scissor = windows::Win32::Foundation::RECT {
            left: a.left.max(clip.left) as i32,
            top: a.top.max(clip.top) as i32,
            right: a.right.min(clip.right) as i32,
            bottom: a.bottom.min(clip.bottom) as i32,
        };
        if scissor.right <= scissor.left || scissor.bottom <= scissor.top {
            // Nada à vista: nada a compor.
            return true;
        }
        // SAFETY: chamadas COM do Direct2D e do DXGI que só perguntam as texturas por trás dos
        // bitmaps (o alvo de agora e a camada).
        let textures = unsafe {
            let texture_of = |bitmap: ID2D1Bitmap1| -> Option<ID3D11Texture2D> { bitmap.GetSurface().ok()?.cast().ok() };
            let dc: Option<ID2D1DeviceContext> = self.target.cast().ok();
            let target = dc.and_then(|dc| dc.GetTarget().ok()).and_then(|t| t.cast::<ID2D1Bitmap1>().ok()).and_then(texture_of);
            let layer = content.GetBitmap().ok().and_then(|b| b.cast::<ID2D1Bitmap1>().ok()).and_then(texture_of);
            target.zip(layer)
        };
        let Some((target, layer)) = textures else { return false };
        let pool = &mut self.cache.blend_pool;
        if pool.compositor.as_ref().is_some_and(|c| c.as_ref().is_some_and(|c| !c.serves(&target))) {
            pool.compositor = None;
        }
        let Some(compositor) = pool.compositor.get_or_insert_with(|| compositor::Compositor::new(&target)).as_mut() else { return false };
        let area = windows::Win32::Foundation::RECT { left: a.left as i32, top: a.top as i32, right: a.right as i32, bottom: a.bottom as i32 };
        let done = compositor.compose(&target, &layer, area, scissor, index, opacity).is_ok();
        if done {
            SHADER_BLENDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        done
    }

    /// Fecha clips e camadas que a pintura deixou abertos (o EndDraw exige).
    fn unwind(&mut self) {
        while !self.blends.is_empty() {
            self.end_blend_inner();
        }
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
        self.pushes.push(Push::Layer(mask, opacity, self.transform, self.draws));
    }

    fn push_layer_raw(&self, mask: Option<ID2D1Geometry>, opacity: f32) {
        // Com o ClearType no alvo, a camada opaca (um recorte) leva o fundo para dentro e o
        // texto nela continua em ClearType; a translúcida fica em cinza (o subpixel não se
        // mistura).
        // SAFETY: só lê o modo do alvo atual, que é válido.
        let cleartype = opacity >= 1.0 && unsafe { self.target.GetTextAntialiasMode() } == D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE;
        push_layer_on(&self.target, mask, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE, opacity, cleartype);
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
                Push::Layer(mask, opacity, m, _) => {
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
            // O quadro anterior (já solto pelo app) com o mesmo tamanho: copia os pixels nele em
            // vez de criar outro bitmap na placa a cada quadro.
            if let Some(bitmap) = self.cache.take_spare(w, h) {
                // SAFETY: o bitmap é deste dispositivo e tem `w × h`; `pixels` tem `w * h * 4`.
                if unsafe { bitmap.CopyFromMemory(None, pixels.as_ptr() as *const _, w * 4) }.is_ok() {
                    self.remember_frame(image, &bitmap, w, h);
                    return Some((bitmap, w, h));
                }
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
            PIXEL_BITMAPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.remember_frame(image, &bitmap, w, h);
            return Some((bitmap, w, h));
        }
        let decoded = self.renderer.decode(image)?;
        let (w, h, pixels) = (decoded.0, decoded.1, &decoded.2);
        if let Some(b) = self.cache.bitmaps.get(image) {
            return Some((b.clone(), w, h));
        }
        // Pixels RGBA (um quadro de vídeo convertido): também reaproveita o do quadro anterior.
        if !matches!(image.source(), ImageSource::Encoded(_)) {
            if let Some(bitmap) = self.cache.take_spare(w, h) {
                // SAFETY: o bitmap é deste dispositivo e tem `w × h`; `pixels` tem `w * h * 4`.
                if unsafe { bitmap.CopyFromMemory(None, pixels.as_ptr() as *const _, w * 4) }.is_ok() {
                    self.remember_frame(image, &bitmap, w, h);
                    return Some((bitmap, w, h));
                }
            }
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
            // `color_glyphs: false`: sem a cor das fontes de emoji.
            let options = if key.mono_emoji { D2D1_DRAW_TEXT_OPTIONS_NONE } else { D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT };
            self.target.DrawTextLayout(v(at), &layout, &self.brush, options);
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
        let now = std::time::Instant::now();
        let key = (id, texture);
        let bitmap = match link.surfaces.get_mut(&key) {
            Some((bitmap, used)) => {
                *used = now;
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
                link.surfaces.insert(key, (bitmap.clone(), now));
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
        self.draws += 1;
        self.draw_gpu(dest, surface).is_some()
    }

    fn save(&mut self) {
        self.frames.push(Frame { transform: self.transform, pushes: self.pushes.len() });
    }

    fn restore(&mut self) {
        // O `restore` do `save_layer_blend`: a camada se junta ao fundo antes.
        if self.blends.last().is_some_and(|b| b.frame == self.frames.len()) {
            self.end_blend_inner();
        }
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
        // Desde que uma mistura abriu dentro de uma camada com desenho: isolada (fora da tela, do
        // tamanho do recorte de agora).
        if opacity < 1.0 && self.renderer.isolate_layers.get() {
            let area = self.clip_area();
            if self.open_offscreen(None, opacity, area, self.frames.len()) {
                return;
            }
        }
        self.push_layer(None, opacity);
    }

    /// A camada num alvo fora da tela; no `restore`, o pedaço dela (o `bounds` em pixels) se
    /// junta ao fundo pelo efeito Blend. Sem o alvo fora da tela, a camada normal.
    fn save_layer_blend(&mut self, opacity: f32, mode: aylis_core::BlendMode, bounds: RectF) {
        use aylis_core::BlendMode as B;
        // A ordem do `BlendMode` é a do shader do compositor em Direct3D.
        let index = aylis_core::PropEnum::index(mode) as u32;
        let mode = match mode {
            B::Normal => return self.save_layer(opacity),
            B::Multiply => D2D1_BLEND_MODE_MULTIPLY,
            B::Screen => D2D1_BLEND_MODE_SCREEN,
            B::Overlay => D2D1_BLEND_MODE_OVERLAY,
            B::Darken => D2D1_BLEND_MODE_DARKEN,
            B::Lighten => D2D1_BLEND_MODE_LIGHTEN,
            B::ColorDodge => D2D1_BLEND_MODE_COLOR_DODGE,
            B::ColorBurn => D2D1_BLEND_MODE_COLOR_BURN,
            B::HardLight => D2D1_BLEND_MODE_HARD_LIGHT,
            B::SoftLight => D2D1_BLEND_MODE_SOFT_LIGHT,
            B::Difference => D2D1_BLEND_MODE_DIFFERENCE,
            B::Exclusion => D2D1_BLEND_MODE_EXCLUSION,
            B::Hue => D2D1_BLEND_MODE_HUE,
            B::Saturation => D2D1_BLEND_MODE_SATURATION,
            B::Color => D2D1_BLEND_MODE_COLOR,
            B::Luminosity => D2D1_BLEND_MODE_LUMINOSITY,
        };
        self.isolate_open_layer();
        // O pedaço em pixels: os quatro cantos pela transformação de agora (com giro, a caixa
        // em volta), dentro do alvo.
        let m = self.transform;
        let map = |x: f32, y: f32| (x * m.M11 + y * m.M21 + m.M31, x * m.M12 + y * m.M22 + m.M32);
        let corners = [map(bounds.x, bounds.y), map(bounds.right(), bounds.y), map(bounds.x, bounds.bottom()), map(bounds.right(), bounds.bottom())];
        let size = unsafe { self.target.GetPixelSize() };
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for (x, y) in corners {
            (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
        }
        let area = D2D_RECT_U {
            left: x0.floor().max(0.0) as u32,
            top: y0.floor().max(0.0) as u32,
            right: (x1.ceil().max(0.0) as u32).min(size.width),
            bottom: (y1.ceil().max(0.0) as u32).min(size.height),
        };
        // A camada do tamanho do pedaço (antes, do alvo inteiro: 52 marcas d'água numa tela
        // cheia criavam 52 alvos do tamanho da tela por quadro).
        self.save();
        if !self.open_offscreen(Some((mode, index)), opacity, area, self.frames.len()) {
            self.frames.pop();
            self.save_layer(opacity);
        }
    }

    fn fill_rect(&mut self, r: RectF, radius: f32, brush: &Brush) {
        self.draws += 1;
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
        self.draws += 1;
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
        self.draws += 1;
        let Some(b) = self.brush(brush, r) else { return };
        let e = D2D1_ELLIPSE { point: v(r.center()), radiusX: r.width / 2.0, radiusY: r.height / 2.0 };
        unsafe { self.target.FillEllipse(&e, &b) };
    }

    fn stroke_ellipse(&mut self, r: RectF, stroke: &Stroke) {
        self.draws += 1;
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
        self.draws += 1;
        let area = RectF::new(from.x.min(to.x), from.y.min(to.y), (from.x - to.x).abs(), (from.y - to.y).abs()).inflate(stroke.width);
        let Some(b) = self.brush(&stroke.brush, area) else { return };
        let style = self.renderer.stroke_style(stroke);
        unsafe { self.target.DrawLine(v(from), v(to), &b, stroke.width, style.as_ref()) };
    }

    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        self.draws += 1;
        let (Some(g), Some(b)) = (self.geometry(path), self.brush(brush, path.bounds())) else { return };
        unsafe { self.target.FillGeometry(&g, &b, None::<&ID2D1Brush>) };
    }

    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        self.draws += 1;
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
        self.draws += 1;
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
        self.draws += 1;
        if text.is_empty() || r.width <= 0.0 {
            return;
        }
        let key = FontKey::of_style(style);
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
        let key = FontKey::of_style(style);
        SizeF::new(self.renderer.text.width(text, &key), self.renderer.text.line_height(&key))
    }

    fn measure_text_wrapped(&mut self, text: &str, style: &TextStyle, max_width: f32) -> SizeF {
        let text_cache = self.renderer.text.clone();
        let key = FontKey::of_style(style);
        let mut wrapped = text_cache.wrapped.borrow_mut();
        wrapped.get_or(text, style, max_width, || {
            aylis_core::canvas::wrapped_size(text, max_width, text_cache.line_height(&key), &mut |s| text_cache.width(s, &key))
        })
    }

    fn draw_image(&mut self, image: &Image, src: RectF, dest: RectF, opacity: f32) {
        self.draws += 1;
        let Some((bitmap, _, _)) = self.bitmap(image) else { return };
        unsafe {
            self.target.DrawBitmap(&bitmap, Some(&rect(dest)), opacity, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, Some(&rect(src)));
        }
    }

    fn image_size(&mut self, image: &Image) -> Option<SizeF> {
        image.known_size().or_else(|| self.renderer.decode(image).map(|d| SizeF::new(d.0 as f32, d.1 as f32)))
    }

    fn backdrop_blur(&mut self, r: RectF, radius: f32, blur: f32) -> bool {
        self.draws += 1;
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
        self.masks.push(MaskState { main, pushes, content, mask: None, luminance: false });
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

    fn begin_luminance_mask(&mut self) {
        self.begin_mask();
        if let Some(state) = self.masks.last_mut() {
            state.luminance = true;
        }
    }

    fn end_mask(&mut self) {
        self.draws += 1;
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
        if let Some(bitmap) = self.cache.layers.get(full).cloned() {
            self.draws += 1;
            self.draw_layer(&bitmap, &dest);
            return true;
        }
        // Sem lugar no limite das camadas (`image_cache.layers_mb`): desenha direto.
        if !aylis_core::LayerCache::<ID2D1Bitmap>::accepts(w as usize * h as usize * 4) {
            self.cached.push(None);
            return false;
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
            // A camada guardada é transparente: sempre em cinza.
            self.renderer.set_text_mode(&as_target, false);
            content.Clear(Some(&color(Color::TRANSPARENT)));
        }
        let main = std::mem::replace(&mut self.target, as_target);
        let pushes = std::mem::take(&mut self.pushes);
        let transform = self.transform;
        // A mesma escala; a origem da imagem é o canto de `dest`.
        self.set_transform(Matrix3x2 { M31: m.M31 - x0, M32: m.M32 - y0, ..m });
        self.cached.push(Some(CacheState { main, pushes, transform, content, key: full, owner: key, dest }));
        false
    }

    fn end_cached(&mut self) {
        self.draws += 1;
        self.end_cached_inner();
    }

    fn forget_cached(&mut self, key: u64) {
        self.cache.layers.forget(key);
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
    // Só no desenvolvimento: em release, só com a `release-diagnostics` (as medições do Aylis).
    let on = cfg!(feature = "diagnostics") && (cfg!(debug_assertions) || cfg!(feature = "release-diagnostics"));
    if on { std::env::var(name).ok() } else { None }
}
