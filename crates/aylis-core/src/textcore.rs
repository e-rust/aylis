//! O texto igual em todos os renderizadores (item 10): o modo de texto de cada plataforma.
//!
//! - `native`: cada renderizador mede e desenha (Skia, DirectWrite, o navegador), como sempre;
//! - `layout`: o núcleo mede (com as fontes do projeto e o `harfrust`) e decide as quebras e
//!   os tamanhos; o renderizador só desenha cada linha no lugar que o núcleo deu;
//! - `core`: o núcleo também desenha, letra por letra (o contorno de cada letra vira um
//!   caminho preenchido): o mesmo desenho em toda parte.
//!
//! O núcleo só conhece as fontes registradas ([`crate::fonts`]: as do projeto). Um texto com
//! um caractere que nenhuma delas tem (um emoji, sem uma fonte de emoji no projeto) fica com o
//! renderizador, como no `native`.

use crate::canvas::{Brush, Canvas, Shadow, Stroke, TextStyle};
use crate::geometry::{PointF, RectF, SizeF};
use crate::path::Path;
use engine::{engine_face, line_metrics_of as line_metrics, line_width, outline};
use crate::{TextAlign, Ui};

/// Quem mede e desenha o texto (veja o módulo).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextMode {
    #[default]
    Native,
    Layout,
    Core,
}

impl crate::prop::PropEnum for TextMode {
    const NAMES: &'static [&'static str] = &["native", "layout", "core"];

    fn from_index(index: usize) -> Option<Self> {
        [TextMode::Native, TextMode::Layout, TextMode::Core].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Mede como o núcleo mede: `None` se o texto não fica com ele (sem fonte, letra que falta).
pub fn measure(text: &str, style: &TextStyle, max_width: Option<f32>) -> Option<SizeF> {
    let face = engine_face(text, &style.font)?;
    let size = style.font.size;
    let (line_height, _) = line_metrics(face, size);
    let mut measure = |s: &str| line_width(face, s, size);
    let lines: Vec<String> = match max_width {
        Some(w) => crate::text::layout_lines(text, w, usize::MAX, true, crate::Trimming::None, &mut measure),
        _ => text.split('\n').map(str::to_string).collect(),
    };
    let width = lines.iter().map(|l| measure(l)).fold(0.0f32, f32::max);
    Some(SizeF::new(width, line_height * lines.len().max(1) as f32))
}

/// O canvas que põe o texto no modo do app: repassa tudo a `inner`, menos medir e desenhar
/// texto (veja o módulo).
pub(crate) struct TextCanvas<'a> {
    pub(crate) inner: &'a mut dyn Canvas,
    pub(crate) mode: TextMode,
}

impl TextCanvas<'_> {
    /// Uma linha no lugar que o núcleo decidiu: o renderizador desenha (`layout`) ou o núcleo
    /// desenha as letras (`core`).
    fn draw_line(&mut self, face: usize, line: &str, x: f32, top: f32, line_height: f32, style: &TextStyle) {
        let size = style.font.size;
        let (width, baseline) = (line_width(face, line, size), line_metrics(face, size).1);
        match self.mode {
            TextMode::Core => {
                let path = outline(face, line, size, x, top + baseline);
                self.inner.fill_path(&path, &Brush::Solid(style.color));
                crate::canvas::decorate_line(self.inner, &style.font, style.color, x, width, top, line_height);
            }
            _ => {
                let mut one = style.clone();
                one.h_align = TextAlign::Leading;
                one.v_align = TextAlign::Center;
                one.wrap = false;
                one.trimming = crate::Trimming::None;
                // Uma folga à direita: a medida do renderizador pode ser um pouco maior, e a
                // linha não pode ganhar "…" nem quebrar.
                self.inner.draw_text(line, RectF::new(x, top, width + size, line_height), &one);
            }
        }
    }
}

impl Canvas for TextCanvas<'_> {
    fn save(&mut self) {
        self.inner.save();
    }
    fn restore(&mut self) {
        self.inner.restore();
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.inner.translate(dx, dy);
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        self.inner.scale(sx, sy);
    }
    fn rotate(&mut self, degrees: f32) {
        self.inner.rotate(degrees);
    }
    fn clip_rect(&mut self, rect: RectF) {
        self.inner.clip_rect(rect);
    }
    fn save_layer(&mut self, opacity: f32) {
        self.inner.save_layer(opacity);
    }
    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        self.inner.fill_rect(rect, radius, brush);
    }
    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke) {
        self.inner.stroke_rect(rect, radius, stroke);
    }
    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        self.inner.fill_ellipse(rect, brush);
    }
    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke) {
        self.inner.stroke_ellipse(rect, stroke);
    }
    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        self.inner.draw_line(from, to, stroke);
    }
    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        if text.is_empty() || rect.width <= 0.0 {
            return;
        }
        let Some(face) = (self.mode != TextMode::Native).then(|| engine_face(text, &style.font)).flatten() else {
            self.inner.draw_text(text, rect, style);
            return;
        };
        // A mesma regra de linhas dos renderizadores (quebra, reticências, alinhamento), com
        // as medidas do núcleo.
        let size = style.font.size;
        let (line_height, _) = line_metrics(face, size);
        let mut measure = |s: &str| line_width(face, s, size);
        let lines = if !style.wrap && !text.contains('\n') {
            let width = measure(text);
            let line = if width > rect.width && style.trimming != crate::Trimming::None { crate::text::trim_line(text, rect.width, style.trimming, &mut measure) } else { text.to_string() };
            vec![(line, true)]
        } else {
            let max_lines = ((rect.height + 1.0) / line_height).floor().max(1.0) as usize;
            crate::text::layout_lines_marked(text, rect.width, max_lines, style.wrap, style.trimming, &mut measure)
        };
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
            let y = top + line_height * i as f32;
            let words = (style.h_align == TextAlign::Justify && !end).then(|| crate::text::justify_line(line, rect.width, &mut |s: &str| line_width(face, s, size))).flatten();
            if let Some(words) = words {
                for (x, word) in words {
                    self.draw_line(face, &word, rect.x + x, y, line_height, style);
                }
                continue;
            }
            let width = line_width(face, line, size);
            let x = match style.h_align {
                TextAlign::Leading | TextAlign::Justify => rect.x,
                TextAlign::Center => rect.x + (rect.width - width) / 2.0,
                TextAlign::Trailing => rect.right() - width,
            };
            self.draw_line(face, line, x, y, line_height, style);
        }
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        if self.mode != TextMode::Native {
            if let Some(size) = measure(text, style, None) {
                return size;
            }
        }
        self.inner.measure_text(text, style)
    }
    fn draw_image(&mut self, image: &crate::Image, src: RectF, dest: RectF, opacity: f32) {
        self.inner.draw_image(image, src, dest, opacity);
    }
    fn image_size(&mut self, image: &crate::Image) -> Option<SizeF> {
        self.inner.image_size(image)
    }
    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        self.inner.clip_round_rect(rect, radius);
    }
    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        self.inner.fill_path(path, brush);
    }
    fn clip_path(&mut self, path: &Path, inverse: bool) {
        self.inner.clip_path(path, inverse);
    }
    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        self.inner.backdrop_blur(rect, radius, blur)
    }
    fn begin_masked(&mut self, inverse: bool) -> bool {
        self.inner.begin_masked(inverse)
    }
    fn begin_mask(&mut self) {
        self.inner.begin_mask();
    }
    fn end_mask(&mut self) {
        self.inner.end_mask();
    }
    fn begin_cached(&mut self, key: u64, rect: RectF) -> bool {
        self.inner.begin_cached(key, rect)
    }
    fn end_cached(&mut self) {
        self.inner.end_cached();
    }
    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        self.inner.stroke_path(path, stroke);
    }
    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &Shadow) {
        self.inner.draw_shadow(rect, radius, shadow);
    }
    fn device_scale(&self) -> f32 {
        self.inner.device_scale()
    }
    fn gpu_device(&self) -> Option<crate::GpuDevice> {
        self.inner.gpu_device()
    }
    fn draw_gpu_surface(&mut self, dest: RectF, surface: &crate::GpuSurface) -> bool {
        self.inner.draw_gpu_surface(dest, surface)
    }
    fn gpu_state_changed(&mut self) {
        self.inner.gpu_state_changed();
    }
}

impl Ui {
    /// O modo de texto do app (veja o módulo): o `text` do `.aylisproj` para a plataforma.
    pub fn set_text_mode(&mut self, mode: TextMode) {
        if self.text_mode != mode {
            self.text_mode = mode;
            self.invalidate();
        }
    }

    pub fn text_mode(&self) -> TextMode {
        self.text_mode
    }
}

/// O peso no número das fontes (400 o normal, 700 o negrito).
#[cfg(feature = "text-shaping")]
fn weight_number(weight: crate::canvas::FontWeight) -> u16 {
    use crate::canvas::FontWeight::*;
    match weight {
        Thin => 100,
        Light => 300,
        Normal => 400,
        Medium => 500,
        SemiBold => 600,
        Bold => 700,
        Black => 900,
    }
}

/// As fontes do projeto abertas pelo núcleo (o `harfrust`: medir, formar e o contorno das
/// letras). Só com a feature `text-shaping`: sem ela, nenhuma fonte fica com o núcleo e o texto
/// é todo do renderizador, como sem fonte registrada (o `harfrust` e o `read-fonts` são ~520 KB
/// de cada wasm; a IDE liga a feature no projeto que tem fontes em `assets/fonts`).
#[cfg(feature = "text-shaping")]
mod engine {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use super::weight_number;
    use crate::canvas::FontSpec;
    use crate::geometry::PointF;
    use crate::path::{Path, PathCmd};
    use harfrust::font::attribute::Style;
    use harfrust::font::glyph::outline::{DrawMemory, DrawOptions, PathSink};
    use harfrust::font::{Blob, Font};
    use harfrust::GlyphId;

    /// Uma fonte registrada, aberta para medir e desenhar (divide os bytes com o registro).
    struct Face {
        family: String,
        weight: u16,
        italic: bool,
        /// Uma cópia de uma fonte variável em outro peso (não entra na escolha da família).
        variant: bool,
        font: Font,
        units: f32,
        ascender: f32,
        descender: f32,
        gap: f32,
    }

    #[derive(Default)]
    struct Faces {
        generation: u64,
        list: Vec<Face>,
        /// O contorno de cada letra, nas unidades da fonte (o desenho só escala e move).
        outlines: HashMap<(usize, u32), Vec<PathCmd>>,
        /// As cópias das fontes variáveis em outro peso: (a original, o peso) → a cópia.
        variants: HashMap<(usize, u16), usize>,
        /// O plano de formação de cada fonte, direção, escrita e idioma: montá-lo é a parte cara de
        /// formar um texto (o `harfrust::shape` sem plano monta um a cada chamada).
        plans: HashMap<PlanKey, harfrust::ShapePlan>,
        /// As letras já formadas de cada texto, em unidades da fonte (servem em qualquer tamanho).
        /// Formar a cada medida e a cada desenho custava 1,7 ms por texto em debug: a grade do menu
        /// do ERP levava 400 ms por quadro (relatado pelo autor em 07/11). Esvazia ao passar de
        /// `SHAPED_KEEP` textos.
        shaped: HashMap<(usize, String), std::rc::Rc<Shaped>>,
    }

    type PlanKey = (usize, harfrust::Direction, Option<harfrust::Script>, Option<harfrust::Language>);

    /// As letras de uma linha: (letra, avanço, deslocamento x, deslocamento y) e a soma dos
    /// avanços, em unidades da fonte.
    struct Shaped {
        glyphs: Vec<(u32, f32, f32, f32)>,
        advance: f32,
    }

    const SHAPED_KEEP: usize = 4096;

    thread_local! {
        static FACES: RefCell<Faces> = RefCell::new(Faces::default());
    }

    /// As fontes abertas, refeitas quando uma fonte nova é registrada.
    fn with_faces<R>(f: impl FnOnce(&mut Faces) -> R) -> R {
        FACES.with(|faces| {
            let mut faces = faces.borrow_mut();
            let generation = crate::fonts::generation();
            if faces.generation != generation || (faces.list.is_empty() && generation > 0) {
                faces.generation = generation;
                faces.outlines.clear();
                faces.variants.clear();
                faces.plans.clear();
                faces.shaped.clear();
                faces.list = crate::fonts::registered()
                    .into_iter()
                    .flat_map(|file| {
                        // Os bytes são os do registro (`Arc`): nada é copiado. Uma coleção (`.ttc`)
                        // abre uma fonte por índice, até o primeiro que não existe.
                        let data: std::sync::Arc<dyn AsRef<[u8]> + Send + Sync> = file.data.clone();
                        let blob = Blob::from(data);
                        let family = file.family.clone();
                        (0u32..).map_while(move |index| Font::new(blob.clone(), index)).map(move |font| {
                            let units = font.units_per_em() as f32;
                            let attributes = font.default_attributes();
                            // A linha como o `ttf-parser` dava: a tipográfica se o OS/2 pede, senão
                            // a do `hhea`, a tipográfica ou a do Windows.
                            let line = font.metrics().h_line().unwrap_or_default();
                            Face {
                                family: family.clone(),
                                weight: attributes.weight.value().round().clamp(1.0, 1000.0) as u16,
                                italic: attributes.style == Style::Italic,
                                variant: false,
                                ascender: line.ascender.to_f32(),
                                descender: line.descender.to_f32(),
                                gap: line.line_gap.to_f32(),
                                units: if units > 0.0 { units } else { 1000.0 },
                                font,
                            }
                        })
                    })
                    .collect();
            }
            f(&mut faces)
        })
    }

    /// A família padrão do núcleo (a de um `FontSpec` sem família): a primeira registrada, que é a
    /// do projeto.
    fn default_family(faces: &Faces) -> Option<String> {
        faces.list.first().map(|f| f.family.clone())
    }

    /// A fonte para `spec`: a família pedida (ou a padrão), com o peso e o itálico mais perto.
    fn pick(faces: &mut Faces, spec: &FontSpec) -> Option<usize> {
        let family = if spec.family.is_empty() { default_family(faces)? } else { spec.family.clone() };
        let want = weight_number(spec.weight);
        let best = faces
            .list
            .iter()
            .enumerate()
            .filter(|(_, f)| f.family.eq_ignore_ascii_case(&family) && !f.variant)
            .min_by_key(|(_, f)| (f.italic != spec.italic, (f.weight as i32 - want as i32).abs()))
            .map(|(i, _)| i)?;
        // Uma fonte variável (a Inter do projeto) no peso pedido: uma cópia com o eixo `wght` posto.
        let wght = harfrust::Tag::new(b"wght");
        let variable = faces.list[best].font.axes().iter().any(|a| a.tag() == wght);
        if !variable || faces.list[best].weight == want {
            return Some(best);
        }
        if let Some(&i) = faces.variants.get(&(best, want)) {
            return Some(i);
        }
        let base = &faces.list[best];
        let font = base.font.instance_builder().variations([(wght, want as f32)]).build();
        let face = Face {
            family: base.family.clone(),
            weight: want,
            italic: base.italic,
            variant: true,
            font,
            units: base.units,
            ascender: base.ascender,
            descender: base.descender,
            gap: base.gap,
        };
        faces.list.push(face);
        let i = faces.list.len() - 1;
        faces.variants.insert((best, want), i);
        Some(i)
    }

    /// A fonte (`spec`) tem todas as letras do texto? Sem isso, o texto fica com o renderizador.
    fn covers(face: &Face, text: &str) -> bool {
        let charmap = face.font.charmap();
        text.chars().all(|c| c.is_control() || c == '\u{200d}' || c == '\u{fe0f}' || charmap.map_unicode(c).is_some())
    }

    /// As letras montadas de uma linha (da memória, ou formadas e guardadas), em unidades da fonte.
    fn shaped(faces: &mut Faces, face: usize, text: &str) -> std::rc::Rc<Shaped> {
        if let Some(found) = faces.shaped.get(&(face, text.to_string())) {
            return found.clone();
        }
        let mut buffer = harfrust::Buffer::new();
        buffer.push_str(text);
        buffer.guess_segment_properties();
        let key: PlanKey = (face, buffer.direction(), buffer.script(), buffer.language().cloned());
        let font = &faces.list[face].font;
        let plan = faces.plans.entry(key).or_insert_with(|| {
            harfrust::ShapePlan::new(font, buffer.direction(), buffer.script(), buffer.language(), &[])
        });
        let shaper = harfrust::ShaperFont::new(font);
        // Só falha com um plano de outra direção ou escrita, e a chave do plano é a do texto.
        let shaped = harfrust::shape(&shaper, &mut buffer, harfrust::ShapeOptions::new().plan(Some(plan)));
        let glyphs: Vec<(u32, f32, f32, f32)> = match shaped {
            Ok(()) => buffer
                .glyph_infos()
                .iter()
                .zip(buffer.glyph_positions())
                .map(|(info, pos)| (info.glyph_id, pos.x_advance as f32, pos.x_offset as f32, pos.y_offset as f32))
                .collect(),
            Err(_) => Vec::new(),
        };
        let advance = glyphs.iter().map(|g| g.1).sum::<f32>();
        let result = std::rc::Rc::new(Shaped { glyphs, advance });
        if faces.shaped.len() >= SHAPED_KEEP {
            faces.shaped.clear();
        }
        faces.shaped.insert((face, text.to_string()), result.clone());
        result
    }

    /// A largura de uma linha formada, no tamanho.
    fn width_of(faces: &mut Faces, face: usize, text: &str, size: f32) -> f32 {
        let units = faces.list[face].units;
        shaped(faces, face, text).advance * size / units
    }

    /// A altura de uma linha e a distância do topo à linha de base, no tamanho.
    fn line_metrics(face: &Face, size: f32) -> (f32, f32) {
        let scale = size / face.units;
        let ascent = face.ascender * scale;
        let height = (face.ascender - face.descender + face.gap) * scale;
        (height.max(size), ascent + face.gap * scale / 2.0)
    }

    /// O texto todo pode ficar com o núcleo (há fonte registrada e ela tem todas as letras)?
    pub(super) fn engine_face(text: &str, spec: &FontSpec) -> Option<usize> {
        with_faces(|faces| pick(faces, spec).filter(|&i| covers(&faces.list[i], text)))
    }

    /// A largura de uma linha (sem quebra) pelo núcleo.
    pub(super) fn line_width(face: usize, text: &str, size: f32) -> f32 {
        with_faces(|faces| width_of(faces, face, text, size))
    }

    /// O contorno de uma linha de letras, já no tamanho e na posição (`x`, linha de base).
    fn outline_line(faces: &mut Faces, face: usize, line: &str, size: f32, x: f32, baseline: f32) -> Path {
        let glyphs = shaped(faces, face, line);
        let scale = size / faces.list[face].units;
        let mut cmds = Vec::new();
        let mut pen = x;
        for &(glyph, advance, dx, dy) in glyphs.glyphs.iter() {
            let outline = faces.outlines.entry((face, glyph)).or_insert_with(|| {
                let mut builder = Outline(Vec::new());
                if let Some(outline) = faces.list[face].font.outline_glyphs().get(GlyphId::new(glyph)) {
                    // Sem tamanho: nas unidades da fonte, como o `ttf-parser` dava. Uma letra que não
                    // abre fica sem desenho, como antes.
                    if outline.draw(&DrawOptions::default(), DrawMemory::Scratch, &mut builder).is_err() {
                        builder.0.clear();
                    }
                }
                builder.0
            });
            let ox = pen + dx * scale;
            let oy = baseline - dy * scale;
            let map = |p: PointF| PointF::new(ox + p.x * scale, oy - p.y * scale);
            for cmd in outline.iter() {
                cmds.push(match *cmd {
                    PathCmd::MoveTo(p) => PathCmd::MoveTo(map(p)),
                    PathCmd::LineTo(p) => PathCmd::LineTo(map(p)),
                    PathCmd::QuadTo(a, b) => PathCmd::QuadTo(map(a), map(b)),
                    PathCmd::CubicTo(a, b, c) => PathCmd::CubicTo(map(a), map(b), map(c)),
                    PathCmd::Close => PathCmd::Close,
                });
            }
            pen += advance * scale;
        }
        Path { cmds, even_odd: false }
    }

    /// Junta o contorno de uma letra (nas unidades da fonte).
    struct Outline(Vec<PathCmd>);

    impl PathSink for Outline {
        fn move_to(&mut self, x: f32, y: f32) {
            self.0.push(PathCmd::MoveTo(PointF::new(x, y)));
        }
        fn line_to(&mut self, x: f32, y: f32) {
            self.0.push(PathCmd::LineTo(PointF::new(x, y)));
        }
        fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
            self.0.push(PathCmd::QuadTo(PointF::new(x1, y1), PointF::new(x, y)));
        }
        fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
            self.0.push(PathCmd::CubicTo(PointF::new(x1, y1), PointF::new(x2, y2), PointF::new(x, y)));
        }
        fn close(&mut self) {
            self.0.push(PathCmd::Close);
        }
    }

    /// A altura de uma linha e a distância do topo à linha de base, no tamanho.
    pub(super) fn line_metrics_of(face: usize, size: f32) -> (f32, f32) {
        with_faces(|faces| line_metrics(&faces.list[face], size))
    }

    /// O contorno de uma linha de letras, já no tamanho e na posição.
    pub(super) fn outline(face: usize, line: &str, size: f32, x: f32, baseline: f32) -> Path {
        with_faces(|faces| outline_line(faces, face, line, size, x, baseline))
    }
}

/// Sem a feature `text-shaping`: o núcleo não abre fonte nenhuma (veja o de cima).
#[cfg(not(feature = "text-shaping"))]
mod engine {
    use crate::canvas::FontSpec;
    use crate::path::Path;

    pub(super) fn engine_face(_text: &str, _spec: &FontSpec) -> Option<usize> {
        None
    }
    pub(super) fn line_width(_face: usize, _text: &str, _size: f32) -> f32 {
        0.0
    }
    pub(super) fn line_metrics_of(_face: usize, size: f32) -> (f32, f32) {
        (size, size)
    }
    pub(super) fn outline(_face: usize, _line: &str, _size: f32, _x: f32, _baseline: f32) -> Path {
        Path { cmds: Vec::new(), even_odd: false }
    }
}
