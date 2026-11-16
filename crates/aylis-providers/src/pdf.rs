//! O PDF pelo krilla (o gerador de PDF do Typst): o `PdfProvider` padrão do Aylis desde 15/11,
//! no lugar do PDF escrito à mão no núcleo (só Helvetica e Latin-1). As fontes do relatório vão
//! embutidas, só com os caracteres usados: a do projeto (`assets/fonts`, registradas em
//! [`aylis_core::fonts`]) ou a do sistema (pelo `fontdb`, fora da web); um caractere que a fonte
//! não tem sai da primeira fonte de reserva que o tenha (chinês, árabe, símbolos).
//!
//! O texto é medido pelo `skrifa` (o leitor de fontes que o krilla já usa) para quebrar as linhas
//! e alinhar, e desenhado pelo krilla, que faz o shaping. No navegador não há fontes do sistema:
//! só as do projeto.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use aylis_core::canvas::{TextAlign, TextStyle};
use aylis_core::image::{Image as AylisImage, ImageSource};
use aylis_core::pages::{Op, Page};
use aylis_core::providers::{PdfDocument, PdfProvider, Provided};
use aylis_core::text::Trimming;
use aylis_core::{Color, Component, FontWeight, PointF, RectF};
use krilla::color::rgb;
use krilla::geom::{Path, PathBuilder, Point, Size, Transform};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, FillRule, Stroke, StrokeDash};
use krilla::text::{Font, TextDirection};
use krilla::Document;
use skrifa::instance::{LocationRef, Size as FontSize};
use skrifa::{FontRef, MetadataProvider};

/// O provedor de PDF pelo krilla, como componente (para ligar ao `provider` do `Report`; sem
/// provedor ligado, o `Report` já usa este, que o `register` põe como o padrão).
#[derive(Default)]
pub struct KrillaPdfProvider;

impl Component for KrillaPdfProvider {
    fn class_name(&self) -> &'static str {
        "KrillaPdfProvider"
    }

    fn provided(&self) -> Option<Provided<'_>> {
        Some(Provided::Pdf(&KrillaPdf))
    }
}

/// O PDF pelo krilla.
pub struct KrillaPdf;

impl PdfProvider for KrillaPdf {
    fn begin(&self, width: f32, height: f32) -> Result<Box<dyn PdfDocument>, String> {
        let settings = PageSettings::from_wh(width, height).ok_or_else(|| format!("tamanho de página inválido: {width} × {height}"))?;
        Ok(Box::new(KrillaDocument { doc: Document::new(), settings }))
    }
}

struct KrillaDocument {
    doc: Document,
    settings: PageSettings,
}

impl PdfDocument for KrillaDocument {
    fn add_page(&mut self, page: &Page) -> Result<(), String> {
        let mut pdf_page = self.doc.start_page_with(self.settings.clone());
        let mut surface = pdf_page.surface();
        // Cada `Save` guarda quantos recortes vieram depois dele, para o `Restore` desfazer.
        let mut saves: Vec<usize> = vec![0];
        for op in &page.ops {
            match op {
                Op::FillRect { rect, radius, color } => fill(&mut surface, &rect_path(*rect, *radius), *color),
                Op::StrokeRect { rect, radius, color, width, dash } => stroke(&mut surface, &rect_path(*rect, *radius), *color, *width, dash),
                Op::FillEllipse { rect, color } => fill(&mut surface, &ellipse_path(*rect), *color),
                Op::StrokeEllipse { rect, color, width } => stroke(&mut surface, &ellipse_path(*rect), *color, *width, &[]),
                Op::Line { from, to, color, width, dash } => stroke(&mut surface, &contours(&[(vec![*from, *to], false)]), *color, *width, dash),
                Op::FillPath { contours: c, color } => fill(&mut surface, &contours(c), *color),
                Op::StrokePath { contours: c, color, width, dash } => stroke(&mut surface, &contours(c), *color, *width, dash),
                Op::Text { text, rect, style } => draw_text(&mut surface, text, *rect, style),
                Op::Image { image, dest, opacity } => draw_image(&mut surface, image, *dest, *opacity),
                Op::Clip(rect) => {
                    if let Some(path) = rect_path(*rect, 0.0) {
                        surface.push_clip_path(&path, &FillRule::NonZero);
                        if let Some(n) = saves.last_mut() {
                            *n += 1;
                        }
                    }
                }
                Op::Save => saves.push(0),
                Op::Restore => {
                    for _ in 0..saves.pop().unwrap_or(0) {
                        surface.pop();
                    }
                    if saves.is_empty() {
                        saves.push(0);
                    }
                }
            }
        }
        for n in saves {
            for _ in 0..n {
                surface.pop();
            }
        }
        surface.finish();
        pdf_page.finish();
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<Vec<u8>, String> {
        self.doc.finish().map_err(|e| format!("krilla: {e:?}"))
    }
}

// ---------------------------------------------------------------------------
// Formas

fn paint(color: Color) -> (krilla::paint::Paint, NormalizedF32) {
    (rgb::Color::new(color.r, color.g, color.b).into(), NormalizedF32::new(color.a as f32 / 255.0).unwrap_or(NormalizedF32::ONE))
}

fn fill(surface: &mut krilla::surface::Surface, path: &Option<Path>, color: Color) {
    let Some(path) = path else { return };
    let (paint, opacity) = paint(color);
    surface.set_stroke(None);
    surface.set_fill(Some(Fill { paint, opacity, rule: FillRule::NonZero }));
    surface.draw_path(path);
}

fn stroke(surface: &mut krilla::surface::Surface, path: &Option<Path>, color: Color, width: f32, dash: &[f32]) {
    let Some(path) = path else { return };
    let (paint, opacity) = paint(color);
    let dash = (!dash.is_empty()).then(|| StrokeDash { array: dash.to_vec(), offset: 0.0 });
    surface.set_fill(None);
    surface.set_stroke(Some(Stroke { paint, width, opacity, dash, ..Stroke::default() }));
    surface.draw_path(path);
    surface.set_stroke(None);
}

/// O retângulo, com os cantos arredondados (`radius`), como curvas.
fn rect_path(r: RectF, radius: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    let rad = radius.max(0.0).min(r.width / 2.0).min(r.height / 2.0);
    let (l, t, rt, b) = (r.x, r.y, r.x + r.width, r.y + r.height);
    if rad <= 0.0 {
        pb.move_to(l, t);
        pb.line_to(rt, t);
        pb.line_to(rt, b);
        pb.line_to(l, b);
        pb.close();
        return pb.finish();
    }
    let k = rad * 0.552_284_8;
    pb.move_to(l + rad, t);
    pb.line_to(rt - rad, t);
    pb.cubic_to(rt - rad + k, t, rt, t + rad - k, rt, t + rad);
    pb.line_to(rt, b - rad);
    pb.cubic_to(rt, b - rad + k, rt - rad + k, b, rt - rad, b);
    pb.line_to(l + rad, b);
    pb.cubic_to(l + rad - k, b, l, b - rad + k, l, b - rad);
    pb.line_to(l, t + rad);
    pb.cubic_to(l, t + rad - k, l + rad - k, t, l + rad, t);
    pb.close();
    pb.finish()
}

fn ellipse_path(r: RectF) -> Option<Path> {
    let (cx, cy, rx, ry) = (r.x + r.width / 2.0, r.y + r.height / 2.0, r.width / 2.0, r.height / 2.0);
    let (kx, ky) = (rx * 0.552_284_8, ry * 0.552_284_8);
    let mut pb = PathBuilder::new();
    pb.move_to(cx + rx, cy);
    pb.cubic_to(cx + rx, cy + ky, cx + kx, cy + ry, cx, cy + ry);
    pb.cubic_to(cx - kx, cy + ry, cx - rx, cy + ky, cx - rx, cy);
    pb.cubic_to(cx - rx, cy - ky, cx - kx, cy - ry, cx, cy - ry);
    pb.cubic_to(cx + kx, cy - ry, cx + rx, cy - ky, cx + rx, cy);
    pb.close();
    pb.finish()
}

fn contours(list: &[(Vec<PointF>, bool)]) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for (points, closed) in list {
        let Some(first) = points.first() else { continue };
        pb.move_to(first.x, first.y);
        for p in &points[1..] {
            pb.line_to(p.x, p.y);
        }
        if *closed {
            pb.close();
        }
    }
    pb.finish()
}

// ---------------------------------------------------------------------------
// Imagens

fn draw_image(surface: &mut krilla::surface::Surface, image: &AylisImage, dest: RectF, opacity: f32) {
    let Some(pdf_image) = pdf_image(image) else { return };
    let Some(size) = Size::from_wh(dest.width, dest.height) else { return };
    let faded = opacity < 1.0;
    if faded {
        surface.push_opacity(NormalizedF32::new(opacity.clamp(0.0, 1.0)).unwrap_or(NormalizedF32::ONE));
    }
    surface.push_transform(&Transform::from_translate(dest.x, dest.y));
    surface.draw_image(pdf_image, size);
    surface.pop();
    if faded {
        surface.pop();
    }
}

fn pdf_image(image: &AylisImage) -> Option<krilla::image::Image> {
    match image.source() {
        ImageSource::Rgba { width, height, pixels } => Some(krilla::image::Image::from_rgba8(pixels.clone(), *width, *height)),
        // O BGRA é pré-multiplicado (o contrato do tipo): volta ao RGBA comum.
        ImageSource::Bgra { width, height, pixels } => {
            let mut rgba = Vec::with_capacity(pixels.len());
            for p in pixels.chunks_exact(4) {
                let a = p[3];
                let un = |c: u8| if a == 0 { 0 } else { ((c as u32 * 255 + a as u32 / 2) / a as u32).min(255) as u8 };
                rgba.extend_from_slice(&[un(p[2]), un(p[1]), un(p[0]), a]);
            }
            Some(krilla::image::Image::from_rgba8(rgba, *width, *height))
        }
        ImageSource::Encoded(bytes) => {
            let data: krilla::Data = bytes.clone().into();
            if bytes.starts_with(&[0xFF, 0xD8]) {
                krilla::image::Image::from_jpeg(data, true).ok()
            } else if bytes.starts_with(b"\x89PNG") {
                krilla::image::Image::from_png(data, true).ok()
            } else if bytes.starts_with(b"GIF8") {
                krilla::image::Image::from_gif(data, true).ok()
            } else if bytes.len() > 12 && &bytes[8..12] == b"WEBP" {
                krilla::image::Image::from_webp(data, true).ok()
            } else {
                None
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Texto

/// Uma fonte carregada: os bytes (para medir pelo `skrifa`) e a do krilla (para desenhar).
struct LoadedFont {
    data: Arc<Vec<u8>>,
    index: u32,
    pdf: Font,
}

impl LoadedFont {
    fn new(data: Arc<Vec<u8>>, index: u32) -> Option<Arc<Self>> {
        FontRef::from_index(&data, index).ok()?;
        let pdf = Font::new(data.clone().into(), index)?;
        Some(Arc::new(Self { data, index, pdf }))
    }

    fn font_ref(&self) -> Option<FontRef<'_>> {
        FontRef::from_index(&self.data, self.index).ok()
    }

    fn has(&self, c: char) -> bool {
        self.font_ref().is_some_and(|f| f.charmap().map(c).is_some())
    }

    /// A largura de `text` (sem o kerning: a medida para quebrar e alinhar).
    fn width(&self, text: &str, size: f32) -> f32 {
        let Some(font) = self.font_ref() else { return 0.0 };
        let charmap = font.charmap();
        let metrics = font.glyph_metrics(FontSize::new(size), LocationRef::default());
        text.chars().map(|c| charmap.map(c).and_then(|g| metrics.advance_width(g)).unwrap_or(size * 0.5)).sum()
    }

    fn ascent(&self, size: f32) -> f32 {
        self.font_ref().map_or(size * 0.8, |f| f.metrics(FontSize::new(size), LocationRef::default()).ascent)
    }
}

/// As fontes do sistema mais usadas por plataforma, na ordem: a padrão quando o estilo não diz
/// a família, e as de reserva quando a fonte pedida não tem um caractere.
const DEFAULT_FAMILIES: &[&str] = &["Segoe UI", "Helvetica Neue", "Helvetica", "Roboto", "Noto Sans", "DejaVu Sans", "Liberation Sans", "Arial"];
const FALLBACK_FAMILIES: &[&str] = &[
    "Segoe UI",
    "Segoe UI Symbol",
    "Segoe UI Emoji",
    "Microsoft YaHei",
    "Yu Gothic",
    "Malgun Gothic",
    "Nirmala UI",
    "Segoe UI Historic",
    "Noto Sans",
    "Noto Sans CJK SC",
    "Noto Sans Symbols",
    "Noto Color Emoji",
    "DejaVu Sans",
    "PingFang SC",
    "Hiragino Sans",
    "Apple Color Emoji",
    "Arial Unicode MS",
    "Droid Sans Fallback",
];

/// As fontes já procuradas (a mesma família no mesmo peso não é procurada de novo).
#[derive(Default)]
struct FontBook {
    chosen: HashMap<(String, u16, bool), Option<Arc<LoadedFont>>>,
    fallbacks: Option<Vec<Arc<LoadedFont>>>,
    #[cfg(not(target_arch = "wasm32"))]
    system: Option<fontdb::Database>,
}

thread_local! {
    static BOOK: RefCell<FontBook> = RefCell::new(FontBook::default());
}

fn weight_of(weight: FontWeight) -> u16 {
    match weight {
        FontWeight::Thin => 100,
        FontWeight::Light => 300,
        FontWeight::Normal => 400,
        FontWeight::Medium => 500,
        FontWeight::SemiBold => 600,
        FontWeight::Bold => 700,
        FontWeight::Black => 900,
    }
}

impl FontBook {
    /// A fonte de `family` no peso e no estilo mais perto do pedido: a do projeto, depois a do
    /// sistema; sem família (ou não achada), a padrão.
    fn font(&mut self, family: &str, weight: u16, italic: bool) -> Option<Arc<LoadedFont>> {
        let key = (family.to_lowercase(), weight, italic);
        if let Some(found) = self.chosen.get(&key) {
            return found.clone();
        }
        let found = (!family.is_empty())
            .then(|| self.find(family, weight, italic))
            .flatten()
            .or_else(|| DEFAULT_FAMILIES.iter().find_map(|f| self.find(f, weight, italic)))
            .or_else(|| aylis_core::fonts::registered().into_iter().find_map(|f| LoadedFont::new(f.data, 0)));
        self.chosen.insert(key, found.clone());
        found
    }

    fn find(&mut self, family: &str, weight: u16, italic: bool) -> Option<Arc<LoadedFont>> {
        // As do projeto: a de peso e estilo mais perto.
        let mut best: Option<(u32, Arc<Vec<u8>>)> = None;
        for f in aylis_core::fonts::registered().into_iter().filter(|f| f.family.eq_ignore_ascii_case(family)) {
            let Ok(font) = FontRef::from_index(&f.data, 0) else { continue };
            let attrs = font.attributes();
            let is_italic = !matches!(attrs.style, skrifa::attribute::Style::Normal);
            let distance = (attrs.weight.value() as i32 - weight as i32).unsigned_abs() + if is_italic == italic { 0 } else { 1000 };
            if best.as_ref().is_none_or(|(d, _)| distance < *d) {
                best = Some((distance, f.data.clone()));
            }
        }
        if let Some((_, data)) = best {
            return LoadedFont::new(data, 0);
        }
        self.system_font(family, weight, italic)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn system_font(&mut self, family: &str, weight: u16, italic: bool) -> Option<Arc<LoadedFont>> {
        let db = self.system.get_or_insert_with(|| {
            let mut db = fontdb::Database::new();
            db.load_system_fonts();
            db
        });
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(family)],
            weight: fontdb::Weight(weight),
            style: if italic { fontdb::Style::Italic } else { fontdb::Style::Normal },
            stretch: fontdb::Stretch::Normal,
        };
        let id = db.query(&query)?;
        // O `query` volta outra família quando não acha a pedida: só vale a mesma.
        let same = db.face(id).is_some_and(|f| f.families.iter().any(|(name, _)| name.eq_ignore_ascii_case(family)));
        if !same {
            return None;
        }
        let (data, index) = db.with_face_data(id, |data, index| (data.to_vec(), index))?;
        LoadedFont::new(Arc::new(data), index)
    }

    #[cfg(target_arch = "wasm32")]
    fn system_font(&mut self, _family: &str, _weight: u16, _italic: bool) -> Option<Arc<LoadedFont>> {
        None
    }

    /// A primeira fonte de reserva que tem `c` (carregadas na primeira vez que faltar um).
    fn fallback(&mut self, c: char) -> Option<Arc<LoadedFont>> {
        if self.fallbacks.is_none() {
            let mut list: Vec<Arc<LoadedFont>> = FALLBACK_FAMILIES.iter().filter_map(|f| self.find(f, 400, false)).collect();
            list.extend(aylis_core::fonts::registered().into_iter().filter_map(|f| LoadedFont::new(f.data, 0)));
            self.fallbacks = Some(list);
        }
        self.fallbacks.as_ref()?.iter().find(|f| f.has(c)).cloned()
    }
}

/// Um pedaço de linha numa fonte só.
struct Run {
    text: String,
    font: Arc<LoadedFont>,
    width: f32,
}

/// Divide `line` em pedaços pela fonte: a pedida, ou a de reserva para o caractere que ela não tem.
fn runs(book: &mut FontBook, primary: &Arc<LoadedFont>, line: &str, size: f32) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::new();
    for c in line.chars() {
        let font = if primary.has(c) || c.is_whitespace() { primary.clone() } else { book.fallback(c).unwrap_or_else(|| primary.clone()) };
        match out.last_mut() {
            Some(run) if Arc::ptr_eq(&run.font, &font) => run.text.push(c),
            _ => out.push(Run { text: c.to_string(), font, width: 0.0 }),
        }
    }
    for run in &mut out {
        run.width = run.font.width(&run.text, size);
    }
    out
}

fn line_width(book: &mut FontBook, primary: &Arc<LoadedFont>, line: &str, size: f32) -> f32 {
    runs(book, primary, line, size).iter().map(|r| r.width).sum()
}

/// As linhas de `text` no retângulo: as quebras do texto e, com `wrap`, nas palavras (a palavra
/// maior que a largura quebra no caractere); sem `wrap`, a linha larga demais ganha "…" pelo
/// `trimming`.
fn layout(book: &mut FontBook, primary: &Arc<LoadedFont>, text: &str, rect: RectF, style: &TextStyle) -> Vec<String> {
    let size = style.font.size;
    let mut lines = Vec::new();
    for para in text.split('\n') {
        if !style.wrap {
            lines.push(trim(book, primary, para, rect.width, size, style.trimming));
            continue;
        }
        let mut line = String::new();
        for word in para.split(' ') {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if line_width(book, primary, &candidate, size) <= rect.width || line.is_empty() && line_width(book, primary, word, size) <= rect.width {
                line = candidate;
                continue;
            }
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            // A palavra sozinha não cabe: quebra no caractere.
            for c in word.chars() {
                let next = format!("{line}{c}");
                if !line.is_empty() && line_width(book, primary, &next, size) > rect.width {
                    lines.push(std::mem::take(&mut line));
                    line.push(c);
                } else {
                    line = next;
                }
            }
        }
        lines.push(line);
    }
    lines
}

fn trim(book: &mut FontBook, primary: &Arc<LoadedFont>, line: &str, width: f32, size: f32, trimming: Trimming) -> String {
    if matches!(trimming, Trimming::None) || line_width(book, primary, line, size) <= width {
        return line.to_string();
    }
    let mut kept: String = line.to_string();
    while !kept.is_empty() {
        match trimming {
            Trimming::Word => match kept.trim_end().rfind(' ') {
                Some(at) => kept.truncate(at),
                None => {
                    kept.pop();
                }
            },
            _ => {
                kept.pop();
            }
        }
        let candidate = format!("{}…", kept.trim_end());
        if line_width(book, primary, &candidate, size) <= width {
            return candidate;
        }
    }
    "…".to_string()
}

fn draw_text(surface: &mut krilla::surface::Surface, text: &str, rect: RectF, style: &TextStyle) {
    if text.is_empty() {
        return;
    }
    BOOK.with(|book| {
        let mut book = book.borrow_mut();
        let size = style.font.size;
        let Some(primary) = book.font(&style.font.family, weight_of(style.font.weight), style.font.italic) else { return };
        let lines = layout(&mut book, &primary, text, rect, style);
        let line_height = size * 1.2;
        let total = line_height * lines.len() as f32;
        let mut y = match style.v_align {
            TextAlign::Center => rect.y + (rect.height - total) / 2.0,
            TextAlign::Trailing => rect.y + rect.height - total,
            _ => rect.y,
        };
        let (paint, opacity) = paint(style.color);
        surface.set_stroke(None);
        surface.set_fill(Some(Fill { paint, opacity, rule: FillRule::NonZero }));
        let last = lines.len().saturating_sub(1);
        for (i, line) in lines.iter().enumerate() {
            let pieces = runs(&mut book, &primary, line, size);
            let width: f32 = pieces.iter().map(|r| r.width).sum();
            // Justificado: as linhas quebradas (menos a última) esticam nos espaços.
            let spaces = line.matches(' ').count();
            let justify = matches!(style.h_align, TextAlign::Justify) && style.wrap && i < last && spaces > 0;
            let extra = if justify { ((rect.width - width) / spaces as f32).max(0.0) } else { 0.0 };
            let mut x = match style.h_align {
                TextAlign::Center => rect.x + (rect.width - width) / 2.0,
                TextAlign::Trailing => rect.x + rect.width - width,
                _ => rect.x,
            };
            let start = x;
            let baseline = y + (line_height - size) / 2.0 + primary.ascent(size);
            for run in &pieces {
                if justify {
                    // Palavra por palavra, com o espaço a mais.
                    for (n, word) in run.text.split(' ').enumerate() {
                        if n > 0 {
                            x += run.font.width(" ", size) + extra;
                        }
                        if !word.is_empty() {
                            surface.draw_text(Point::from_xy(x, baseline), run.font.pdf.clone(), size, word, false, TextDirection::Auto);
                        }
                        x += run.font.width(word, size);
                    }
                } else {
                    surface.draw_text(Point::from_xy(x, baseline), run.font.pdf.clone(), size, &run.text, false, TextDirection::Auto);
                    x += run.width;
                }
            }
            let thickness = (size / 14.0).max(0.5);
            if style.font.underline {
                let at = baseline + size * 0.12;
                stroke(surface, &contours(&[(vec![PointF::new(start, at), PointF::new(x, at)], false)]), style.color, thickness, &[]);
                surface.set_fill(Some(Fill { paint: paint_of(style.color), opacity, rule: FillRule::NonZero }));
            }
            if style.font.strikeout {
                let at = baseline - size * 0.3;
                stroke(surface, &contours(&[(vec![PointF::new(start, at), PointF::new(x, at)], false)]), style.color, thickness, &[]);
                surface.set_fill(Some(Fill { paint: paint_of(style.color), opacity, rule: FillRule::NonZero }));
            }
            y += line_height;
        }
    });
}

fn paint_of(color: Color) -> krilla::paint::Paint {
    paint(color).0
}
