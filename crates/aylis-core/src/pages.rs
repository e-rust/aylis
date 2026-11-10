//! Páginas gravadas: o que se desenha num [`RecordingCanvas`] vira uma lista de operações
//! que se repete em qualquer canvas ([`Page::replay`], a visualização de um relatório) ou
//! vira PDF ([`PageSet::to_pdf`]), sem biblioteca de fora.
//!
//! As medidas são em pontos (1/72 de polegada): uma página A4 tem 595 × 842. No PDF, o
//! texto sai nas fontes padrão (Helvetica, com negrito e itálico), que todo leitor tem; as
//! imagens RGBA e JPEG entram como estão, as outras pelo decodificador que se passar.

use crate::canvas::{Brush, Canvas, TextAlign, TextService, TextStyle};
use crate::color::Color;
use crate::geometry::{PointF, RectF, SizeF};
use crate::image::{Image, ImageSource};
use crate::path::Path;
use crate::FontWeight;

/// Uma operação de desenho, já nas coordenadas da página.
#[derive(Clone, Debug)]
pub enum Op {
    FillRect { rect: RectF, radius: f32, color: Color },
    StrokeRect { rect: RectF, radius: f32, color: Color, width: f32, dash: Vec<f32> },
    FillEllipse { rect: RectF, color: Color },
    StrokeEllipse { rect: RectF, color: Color, width: f32 },
    Line { from: PointF, to: PointF, color: Color, width: f32, dash: Vec<f32> },
    /// Contornos (pontos e se fecha).
    FillPath { contours: Vec<(Vec<PointF>, bool)>, color: Color },
    StrokePath { contours: Vec<(Vec<PointF>, bool)>, color: Color, width: f32, dash: Vec<f32> },
    Text { text: String, rect: RectF, style: TextStyle },
    Image { image: Image, dest: RectF, opacity: f32 },
    Clip(RectF),
    Save,
    Restore,
}

/// Uma página: as operações em ordem.
#[derive(Clone, Debug, Default)]
pub struct Page {
    pub ops: Vec<Op>,
}

/// Páginas do mesmo tamanho (em pontos).
#[derive(Clone, Debug, Default)]
pub struct PageSet {
    pub width: f32,
    pub height: f32,
    pub pages: Vec<Page>,
}

/// A cor de um pincel (o gradiente vira a cor do meio).
fn brush_color(brush: &Brush) -> Color {
    match brush {
        Brush::Solid(c) => *c,
        Brush::Linear { stops, .. } | Brush::Radial { stops, .. } | Brush::Sweep { stops, .. } => stops.get(stops.len() / 2).map_or(Color::BLACK, |s| s.color),
    }
}

/// Um canvas que grava o que é desenhado (em vez de pintar). Translação e escala entram
/// nas coordenadas gravadas; a rotação é ignorada.
pub struct RecordingCanvas<'a> {
    pub ops: Vec<Op>,
    /// (dx, dy, sx, sy) de cada `save`.
    stack: Vec<(f32, f32, f32, f32)>,
    transform: (f32, f32, f32, f32),
    measure: Option<&'a dyn TextService>,
}

impl<'a> RecordingCanvas<'a> {
    /// `measure`: quem mede o texto (o serviço da plataforma); sem ele, uma estimativa.
    pub fn new(measure: Option<&'a dyn TextService>) -> Self {
        Self { ops: Vec::new(), stack: Vec::new(), transform: (0.0, 0.0, 1.0, 1.0), measure }
    }

    fn point(&self, p: PointF) -> PointF {
        let (dx, dy, sx, sy) = self.transform;
        PointF::new(p.x * sx + dx, p.y * sy + dy)
    }

    fn rect(&self, r: RectF) -> RectF {
        let a = self.point(PointF::new(r.x, r.y));
        let b = self.point(PointF::new(r.right(), r.bottom()));
        RectF::from_points(a, b)
    }

    fn factor(&self) -> f32 {
        (self.transform.2.abs() + self.transform.3.abs()) / 2.0
    }

    fn contours(&self, path: &Path) -> Vec<(Vec<PointF>, bool)> {
        path.flatten(12).into_iter().map(|(pts, closed)| (pts.into_iter().map(|p| self.point(p)).collect(), closed)).collect()
    }
}

impl Canvas for RecordingCanvas<'_> {
    fn save(&mut self) {
        self.stack.push(self.transform);
        self.ops.push(Op::Save);
    }

    fn restore(&mut self) {
        if let Some(t) = self.stack.pop() {
            self.transform = t;
            self.ops.push(Op::Restore);
        }
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        let (x, y, sx, sy) = self.transform;
        self.transform = (x + dx * sx, y + dy * sy, sx, sy);
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        let (x, y, a, b) = self.transform;
        self.transform = (x, y, a * sx, b * sy);
    }

    fn rotate(&mut self, _degrees: f32) {}

    fn clip_rect(&mut self, rect: RectF) {
        let r = self.rect(rect);
        self.ops.push(Op::Clip(r));
    }

    fn save_layer(&mut self, _opacity: f32) {
        self.save();
    }

    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        let (rect, radius) = (self.rect(rect), radius * self.factor());
        self.ops.push(Op::FillRect { rect, radius, color: brush_color(brush) });
    }

    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &crate::Stroke) {
        let (rect, radius) = (self.rect(rect), radius * self.factor());
        let width = stroke.width * self.factor();
        self.ops.push(Op::StrokeRect { rect, radius, color: brush_color(&stroke.brush), width, dash: stroke.dash.clone().unwrap_or_default() });
    }

    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        let rect = self.rect(rect);
        self.ops.push(Op::FillEllipse { rect, color: brush_color(brush) });
    }

    fn stroke_ellipse(&mut self, rect: RectF, stroke: &crate::Stroke) {
        let rect = self.rect(rect);
        let width = stroke.width * self.factor();
        self.ops.push(Op::StrokeEllipse { rect, color: brush_color(&stroke.brush), width });
    }

    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &crate::Stroke) {
        let (from, to) = (self.point(from), self.point(to));
        let width = stroke.width * self.factor();
        self.ops.push(Op::Line { from, to, color: brush_color(&stroke.brush), width, dash: stroke.dash.clone().unwrap_or_default() });
    }

    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        let rect = self.rect(rect);
        let mut style = style.clone();
        style.font.size *= self.factor();
        self.ops.push(Op::Text { text: text.to_string(), rect, style });
    }

    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        match self.measure {
            Some(m) => m.measure(text, style, None),
            None => SizeF::new(text_width(text, style), style.font.size * 1.2),
        }
    }

    fn draw_image(&mut self, image: &Image, _src: RectF, dest: RectF, opacity: f32) {
        let dest = self.rect(dest);
        self.ops.push(Op::Image { image: image.clone(), dest, opacity });
    }

    fn clip_round_rect(&mut self, rect: RectF, _radius: f32) {
        self.clip_rect(rect);
    }

    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        let contours = self.contours(path);
        self.ops.push(Op::FillPath { contours, color: brush_color(brush) });
    }

    fn stroke_path(&mut self, path: &Path, stroke: &crate::Stroke) {
        let contours = self.contours(path);
        let width = stroke.width * self.factor();
        self.ops.push(Op::StrokePath { contours, color: brush_color(&stroke.brush), width, dash: stroke.dash.clone().unwrap_or_default() });
    }
}

fn path_of(contours: &[(Vec<PointF>, bool)]) -> Path {
    let mut p = Path::new();
    for (pts, closed) in contours {
        if let Some(first) = pts.first() {
            p.move_to(first.x, first.y);
            for q in &pts[1..] {
                p.line_to(q.x, q.y);
            }
            if *closed {
                p.close();
            }
        }
    }
    p
}

impl Page {
    /// Desenha a página em `canvas` (nas coordenadas dela: use `translate`/`scale` antes).
    pub fn replay(&self, canvas: &mut dyn Canvas) {
        let stroke = |c: Color, w: f32, dash: &[f32]| {
            let s = crate::Stroke::new(c, w);
            if dash.is_empty() { s } else { s.dashed(dash.to_vec()) }
        };
        let mut depth = 0;
        for op in &self.ops {
            match op {
                Op::FillRect { rect, radius, color } => canvas.fill_rect(*rect, *radius, &Brush::Solid(*color)),
                Op::StrokeRect { rect, radius, color, width, dash } => canvas.stroke_rect(*rect, *radius, &stroke(*color, *width, dash)),
                Op::FillEllipse { rect, color } => canvas.fill_ellipse(*rect, &Brush::Solid(*color)),
                Op::StrokeEllipse { rect, color, width } => canvas.stroke_ellipse(*rect, &stroke(*color, *width, &[])),
                Op::Line { from, to, color, width, dash } => canvas.draw_line(*from, *to, &stroke(*color, *width, dash)),
                Op::FillPath { contours, color } => canvas.fill_path(&path_of(contours), &Brush::Solid(*color)),
                Op::StrokePath { contours, color, width, dash } => canvas.stroke_path(&path_of(contours), &stroke(*color, *width, dash)),
                Op::Text { text, rect, style } => canvas.draw_text(text, *rect, style),
                Op::Image { image, dest, opacity } => {
                    let size = canvas.image_size(image).unwrap_or(SizeF::new(dest.width, dest.height));
                    canvas.draw_image(image, RectF::new(0.0, 0.0, size.width, size.height), *dest, *opacity);
                }
                Op::Clip(r) => canvas.clip_rect(*r),
                Op::Save => {
                    depth += 1;
                    canvas.save();
                }
                Op::Restore if depth > 0 => {
                    depth -= 1;
                    canvas.restore();
                }
                Op::Restore => {}
            }
        }
        for _ in 0..depth {
            canvas.restore();
        }
    }

    /// Todos os textos da página, na ordem (para conferir e para buscar).
    pub fn texts(&self) -> Vec<&str> {
        self.ops.iter().filter_map(|op| if let Op::Text { text, .. } = op { Some(text.as_str()) } else { None }).collect()
    }
}

// ---------------------------------------------------------------------------
// PDF

/// Larguras da Helvetica (normal e negrito) dos caracteres 32 a 126, em milésimos do tamanho.
const HELVETICA: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556,
    1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469,
    556, 333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];
const HELVETICA_BOLD: [u16; 95] = [
    278, 333, 474, 556, 556, 889, 722, 238, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 333, 333, 584, 584, 584, 611,
    975, 722, 722, 722, 722, 667, 611, 778, 722, 278, 556, 722, 611, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 333, 278, 333, 584,
    556, 333, 556, 611, 556, 611, 556, 333, 611, 611, 278, 278, 556, 278, 889, 611, 611, 611, 611, 389, 556, 333, 611, 556, 778, 556, 556, 500, 389, 280, 389, 584,
];

/// A letra sem acento (para a largura), e o byte no WinAnsi (`?` se não houver).
fn win_ansi(c: char) -> (char, u8) {
    let base = match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'å' => 'a',
        'Á' | 'À' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'A',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'É' | 'È' | 'Ê' | 'Ë' => 'E',
        'í' | 'ì' | 'î' | 'ï' => 'i',
        'Í' | 'Ì' | 'Î' | 'Ï' => 'I',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
        'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => 'O',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'Ú' | 'Ù' | 'Û' | 'Ü' => 'U',
        'ç' => 'c',
        'Ç' => 'C',
        'ñ' => 'n',
        'Ñ' => 'N',
        _ => c,
    };
    let byte = match c {
        '€' => 0x80,
        '‚' => 0x82,
        '„' => 0x84,
        '…' => 0x85,
        '‘' => 0x91,
        '’' => 0x92,
        '“' => 0x93,
        '”' => 0x94,
        '•' => 0x95,
        '–' => 0x96,
        '—' => 0x97,
        '™' => 0x99,
        c if (c as u32) < 0x80 || ((c as u32) >= 0xA0 && (c as u32) <= 0xFF) => c as u32 as u8,
        _ => b'?',
    };
    (base, byte)
}

fn char_width(c: char, bold: bool) -> f32 {
    let (base, _) = win_ansi(c);
    let table = if bold { &HELVETICA_BOLD } else { &HELVETICA };
    let code = base as u32;
    let w = if (32..=126).contains(&code) { table[(code - 32) as usize] } else { 556 };
    w as f32 / 1000.0
}

fn is_bold(style: &TextStyle) -> bool {
    matches!(style.font.weight, FontWeight::SemiBold | FontWeight::Bold | FontWeight::Black)
}

/// Largura do texto em Helvetica (a do PDF), no tamanho do estilo.
pub fn text_width(text: &str, style: &TextStyle) -> f32 {
    let bold = is_bold(style);
    text.chars().map(|c| char_width(c, bold)).sum::<f32>() * style.font.size
}

/// As linhas do texto no retângulo: quebra nas palavras com `wrap`, e nas quebras de linha.
fn lines_of(text: &str, rect: RectF, style: &TextStyle) -> Vec<String> {
    let mut out = Vec::new();
    for para in text.split('\n') {
        if !style.wrap {
            out.push(para.to_string());
            continue;
        }
        let mut line = String::new();
        for word in para.split(' ') {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if !line.is_empty() && text_width(&candidate, style) > rect.width {
                out.push(std::mem::take(&mut line));
                line = word.to_string();
            } else {
                line = candidate;
            }
        }
        out.push(line);
    }
    out
}

/// Um número curto para o PDF.
fn num(v: f32) -> String {
    let s = format!("{v:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.to_string() }
}

fn rgb(c: Color) -> (f32, f32, f32) {
    // Sem transparência no PDF simples: a cor misturada com o branco do papel.
    let a = c.a as f32 / 255.0;
    let mix = |v: u8| (v as f32 / 255.0) * a + (1.0 - a);
    (mix(c.r), mix(c.g), mix(c.b))
}

/// Retângulo (com cantos arredondados) como caminho do PDF.
fn rect_path(out: &mut String, r: RectF, radius: f32) {
    let radius = radius.min(r.width / 2.0).min(r.height / 2.0);
    if radius <= 0.5 {
        out.push_str(&format!("{} {} {} {} re\n", num(r.x), num(r.y), num(r.width), num(r.height)));
        return;
    }
    let k = radius * 0.5523;
    let (l, t, rr, b) = (r.x, r.y, r.right(), r.bottom());
    out.push_str(&format!("{} {} m\n", num(l + radius), num(t)));
    out.push_str(&format!("{} {} l\n", num(rr - radius), num(t)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(rr - radius + k), num(t), num(rr), num(t + radius - k), num(rr), num(t + radius)));
    out.push_str(&format!("{} {} l\n", num(rr), num(b - radius)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(rr), num(b - radius + k), num(rr - radius + k), num(b), num(rr - radius), num(b)));
    out.push_str(&format!("{} {} l\n", num(l + radius), num(b)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(l + radius - k), num(b), num(l), num(b - radius + k), num(l), num(b - radius)));
    out.push_str(&format!("{} {} l\n", num(l), num(t + radius)));
    out.push_str(&format!("{} {} {} {} {} {} c h\n", num(l), num(t + radius - k), num(l + radius - k), num(t), num(l + radius), num(t)));
}

fn ellipse_path(out: &mut String, r: RectF) {
    let (cx, cy, rx, ry) = (r.x + r.width / 2.0, r.y + r.height / 2.0, r.width / 2.0, r.height / 2.0);
    let (kx, ky) = (rx * 0.5523, ry * 0.5523);
    out.push_str(&format!("{} {} m\n", num(cx + rx), num(cy)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(cx + rx), num(cy + ky), num(cx + kx), num(cy + ry), num(cx), num(cy + ry)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(cx - kx), num(cy + ry), num(cx - rx), num(cy + ky), num(cx - rx), num(cy)));
    out.push_str(&format!("{} {} {} {} {} {} c\n", num(cx - rx), num(cy - ky), num(cx - kx), num(cy - ry), num(cx), num(cy - ry)));
    out.push_str(&format!("{} {} {} {} {} {} c h\n", num(cx + kx), num(cy - ry), num(cx + rx), num(cy - ky), num(cx + rx), num(cy)));
}

fn stroke_style(out: &mut String, color: Color, width: f32, dash: &[f32]) {
    let (r, g, b) = rgb(color);
    out.push_str(&format!("{} {} {} RG {} w [{}] 0 d\n", num(r), num(g), num(b), num(width.max(0.1)), dash.iter().map(|d| num(*d)).collect::<Vec<_>>().join(" ")));
}

fn fill_style(out: &mut String, color: Color) {
    let (r, g, b) = rgb(color);
    out.push_str(&format!("{} {} {} rg\n", num(r), num(g), num(b)));
}

fn contours_path(out: &mut String, contours: &[(Vec<PointF>, bool)]) {
    for (pts, closed) in contours {
        for (i, p) in pts.iter().enumerate() {
            out.push_str(&format!("{} {} {}\n", num(p.x), num(p.y), if i == 0 { "m" } else { "l" }));
        }
        if *closed {
            out.push_str("h\n");
        }
    }
}

/// Pixels de uma imagem para o PDF: (largura, altura, RGB, alfa) ou o JPEG como está.
enum PdfImage {
    Raw(u32, u32, Vec<u8>, Option<Vec<u8>>),
    Jpeg(u32, u32, Vec<u8>),
}

/// Lê o tamanho de um JPEG (o marcador SOF).
fn jpeg_size(bytes: &[u8]) -> Option<(u32, u32)> {
    let mut i = 2;
    while i + 9 < bytes.len() {
        if bytes[i] != 0xFF {
            return None;
        }
        let marker = bytes[i + 1];
        let len = u16::from_be_bytes([bytes[i + 2], bytes[i + 3]]) as usize;
        if matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF) {
            let h = u16::from_be_bytes([bytes[i + 5], bytes[i + 6]]) as u32;
            let w = u16::from_be_bytes([bytes[i + 7], bytes[i + 8]]) as u32;
            return Some((w, h));
        }
        i += 2 + len;
    }
    None
}

/// Decodifica uma imagem guardada como arquivo (PNG...): (largura, altura, RGBA).
pub type Decoder<'a> = &'a dyn Fn(&[u8]) -> Option<(u32, u32, Vec<u8>)>;

fn pdf_image(image: &Image, decode: Option<Decoder>) -> Option<PdfImage> {
    let split = |w: u32, h: u32, px: &[u8], bgra: bool| {
        let mut rgb = Vec::with_capacity((w * h * 3) as usize);
        let mut alpha = Vec::with_capacity((w * h) as usize);
        for p in px.chunks_exact(4) {
            if bgra {
                rgb.extend_from_slice(&[p[2], p[1], p[0]]);
            } else {
                rgb.extend_from_slice(&p[..3]);
            }
            alpha.push(p[3]);
        }
        let opaque = alpha.iter().all(|a| *a == 255);
        PdfImage::Raw(w, h, rgb, (!opaque).then_some(alpha))
    };
    match image.source() {
        ImageSource::Rgba { width, height, pixels } => Some(split(*width, *height, pixels, false)),
        ImageSource::Bgra { width, height, pixels } => Some(split(*width, *height, pixels, true)),
        ImageSource::Encoded(bytes) if bytes.starts_with(&[0xFF, 0xD8]) => {
            let (w, h) = jpeg_size(bytes)?;
            Some(PdfImage::Jpeg(w, h, bytes.clone()))
        }
        ImageSource::Encoded(bytes) => {
            let (w, h, px) = decode?(bytes)?;
            Some(split(w, h, &px, false))
        }
    }
}

impl PageSet {
    /// O PDF das páginas (imagens PNG e outras fora do JPEG ficam de fora).
    pub fn to_pdf(&self) -> Vec<u8> {
        self.to_pdf_with(None)
    }

    /// O PDF, com `decode` para as imagens guardadas como arquivo (PNG...).
    pub fn to_pdf_with(&self, decode: Option<Decoder>) -> Vec<u8> {
        let mut writer = PdfWriter::new(Vec::new(), self.width, self.height, decode).expect("gravar na memória não falha");
        for page in &self.pages {
            writer.add_page(page).expect("gravar na memória não falha");
        }
        writer.finish().expect("gravar na memória não falha")
    }
}

/// O PDF gravado página por página, direto num arquivo (ou onde for): só os endereços dos
/// objetos ficam na memória, não as páginas. É o que o relatório grande usa
/// (`report::write_pdf`): as páginas vão para o disco à medida que ficam prontas.
pub struct PdfWriter<'a, W: std::io::Write> {
    out: W,
    width: f32,
    height: f32,
    decode: Option<Decoder<'a>>,
    /// Quantos bytes já foram gravados (o endereço do próximo objeto).
    written: usize,
    /// O endereço de cada objeto, pelo número (o 2, a lista das páginas, só no fim).
    offsets: Vec<usize>,
    kids: Vec<usize>,
}

impl<'a, W: std::io::Write> PdfWriter<'a, W> {
    /// Começa o PDF com páginas de `width` × `height` pontos.
    pub fn new(out: W, width: f32, height: f32, decode: Option<Decoder<'a>>) -> std::io::Result<Self> {
        let mut w = Self { out, width, height, decode, written: 0, offsets: vec![0, 0], kids: Vec::new() };
        w.raw(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n")?;
        // Objetos: 1 catálogo, 2 páginas (no fim), 3 a 6 fontes; depois, por página, a página,
        // o conteúdo e as imagens.
        w.object(1, b"<< /Type /Catalog /Pages 2 0 R >>")?;
        for (i, font) in ["Helvetica", "Helvetica-Bold", "Helvetica-Oblique", "Helvetica-BoldOblique"].iter().enumerate() {
            w.object(3 + i, format!("<< /Type /Font /Subtype /Type1 /BaseFont /{font} /Encoding /WinAnsiEncoding >>").as_bytes())?;
        }
        Ok(w)
    }

    /// Quantas páginas já foram gravadas.
    pub fn pages(&self) -> usize {
        self.kids.len()
    }

    fn raw(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.out.write_all(bytes)?;
        self.written += bytes.len();
        Ok(())
    }

    fn object(&mut self, id: usize, body: &[u8]) -> std::io::Result<()> {
        if self.offsets.len() < id {
            self.offsets.resize(id, 0);
        }
        self.offsets[id - 1] = self.written;
        self.raw(format!("{id} 0 obj\n").as_bytes())?;
        self.raw(body)?;
        self.raw(b"\nendobj\n")
    }

    /// Grava uma página (e as imagens dela).
    pub fn add_page(&mut self, page: &Page) -> std::io::Result<()> {
        let (width, height, decode) = (self.width, self.height, self.decode);
        // Os números dos objetos desta página começam depois dos já gravados.
        let base = self.offsets.len().max(6);
        let mut objects: Vec<Vec<u8>> = Vec::new();
        {
            let page_id = base + objects.len() + 1;
            let content_id = page_id + 1;
            objects.push(Vec::new());
            objects.push(Vec::new());
            self.kids.push(page_id);
            let mut images: Vec<(usize, usize)> = Vec::new(); // (nome, objeto)
            let mut out = String::new();
            // Origem no alto à esquerda, como na tela.
            out.push_str(&format!("1 0 0 -1 0 {} cm\n", num(height)));
            let mut depth = 0;
            for op in &page.ops {
                match op {
                    Op::FillRect { rect, radius, color } => {
                        fill_style(&mut out, *color);
                        rect_path(&mut out, *rect, *radius);
                        out.push_str("f\n");
                    }
                    Op::StrokeRect { rect, radius, color, width, dash } => {
                        stroke_style(&mut out, *color, *width, dash);
                        rect_path(&mut out, *rect, *radius);
                        out.push_str("S\n");
                    }
                    Op::FillEllipse { rect, color } => {
                        fill_style(&mut out, *color);
                        ellipse_path(&mut out, *rect);
                        out.push_str("f\n");
                    }
                    Op::StrokeEllipse { rect, color, width } => {
                        stroke_style(&mut out, *color, *width, &[]);
                        ellipse_path(&mut out, *rect);
                        out.push_str("S\n");
                    }
                    Op::Line { from, to, color, width, dash } => {
                        stroke_style(&mut out, *color, *width, dash);
                        out.push_str(&format!("{} {} m {} {} l S\n", num(from.x), num(from.y), num(to.x), num(to.y)));
                    }
                    Op::FillPath { contours, color } => {
                        fill_style(&mut out, *color);
                        contours_path(&mut out, contours);
                        out.push_str("f\n");
                    }
                    Op::StrokePath { contours, color, width, dash } => {
                        stroke_style(&mut out, *color, *width, dash);
                        contours_path(&mut out, contours);
                        out.push_str("S\n");
                    }
                    Op::Text { text, rect, style } => text_ops(&mut out, text, *rect, style),
                    Op::Image { image, dest, .. } => {
                        let Some(img) = pdf_image(image, decode) else { continue };
                        let id = base + objects.len() + 1;
                        let (w, h, body, dict) = match img {
                            PdfImage::Jpeg(w, h, bytes) => (w, h, bytes, "/ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode".to_string()),
                            PdfImage::Raw(w, h, rgb, alpha) => {
                                let mut dict = "/ColorSpace /DeviceRGB /BitsPerComponent 8".to_string();
                                if let Some(alpha) = alpha {
                                    let mask_id = id + 1;
                                    dict.push_str(&format!(" /SMask {mask_id} 0 R"));
                                    objects.push(Vec::new());
                                    let mut mask = format!("<< /Type /XObject /Subtype /Image /Width {w} /Height {h} /ColorSpace /DeviceGray /BitsPerComponent 8 /Length {} >>\nstream\n", alpha.len()).into_bytes();
                                    mask.extend_from_slice(&alpha);
                                    mask.extend_from_slice(b"\nendstream");
                                    objects.push(mask);
                                    let image_index = id - 1 - base;
                                    let mut body = format!("<< /Type /XObject /Subtype /Image /Width {w} /Height {h} {dict} /Length {} >>\nstream\n", rgb.len()).into_bytes();
                                    body.extend_from_slice(&rgb);
                                    body.extend_from_slice(b"\nendstream");
                                    objects[image_index] = body;
                                    images.push((images.len(), id));
                                    out.push_str(&format!("q {} 0 0 {} {} {} cm /Im{} Do Q\n", num(dest.width), num(-dest.height), num(dest.x), num(dest.bottom()), images.len() - 1));
                                    continue;
                                }
                                (w, h, rgb, dict)
                            }
                        };
                        let mut obj = format!("<< /Type /XObject /Subtype /Image /Width {w} /Height {h} {dict} /Length {} >>\nstream\n", body.len()).into_bytes();
                        obj.extend_from_slice(&body);
                        obj.extend_from_slice(b"\nendstream");
                        objects.push(obj);
                        images.push((images.len(), id));
                        // A imagem cobre o quadrado unitário: escala e volta o Y (a página está virada).
                        out.push_str(&format!("q {} 0 0 {} {} {} cm /Im{} Do Q\n", num(dest.width), num(-dest.height), num(dest.x), num(dest.bottom()), images.len() - 1));
                    }
                    Op::Clip(r) => {
                        out.push_str(&format!("{} {} {} {} re W n\n", num(r.x), num(r.y), num(r.width), num(r.height)));
                    }
                    Op::Save => {
                        depth += 1;
                        out.push_str("q\n");
                    }
                    Op::Restore if depth > 0 => {
                        depth -= 1;
                        out.push_str("Q\n");
                    }
                    Op::Restore => {}
                }
            }
            for _ in 0..depth {
                out.push_str("Q\n");
            }
            let xobjects: String = images.iter().map(|(n, id)| format!("/Im{n} {id} 0 R ")).collect();
            objects[page_id - 1 - base] = format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents {content_id} 0 R /Resources << /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R /F4 6 0 R >> /XObject << {xobjects}>> >> >>",
                num(width),
                num(height)
            )
            .into_bytes();
            let bytes = out.into_bytes();
            let mut content = format!("<< /Length {} >>\nstream\n", bytes.len()).into_bytes();
            content.extend_from_slice(&bytes);
            content.extend_from_slice(b"\nendstream");
            objects[content_id - 1 - base] = content;
        }
        for (i, obj) in objects.iter().enumerate() {
            self.object(base + i + 1, obj)?;
        }
        Ok(())
    }

    /// Fecha o PDF (a lista das páginas e o índice dos objetos) e devolve o destino.
    pub fn finish(mut self) -> std::io::Result<W> {
        // A lista das páginas e o índice saem aos pedaços (com dezenas de milhares de páginas,
        // um texto só com tudo dobrava o que o índice ocupa).
        let kids = std::mem::take(&mut self.kids);
        self.offsets[1] = self.written;
        self.raw(b"2 0 obj\n<< /Type /Pages /Kids [")?;
        for (i, chunk) in kids.chunks(256).enumerate() {
            let mut text = String::with_capacity(chunk.len() * 10);
            for (j, k) in chunk.iter().enumerate() {
                if i + j > 0 {
                    text.push(' ');
                }
                text.push_str(&format!("{k} 0 R"));
            }
            self.raw(text.as_bytes())?;
        }
        self.raw(format!("] /Count {} >>\nendobj\n", kids.len()).as_bytes())?;
        let xref = self.written;
        let count = self.offsets.len();
        self.raw(format!("xref\n0 {}\n0000000000 65535 f \n", count + 1).as_bytes())?;
        let offsets = std::mem::take(&mut self.offsets);
        for chunk in offsets.chunks(256) {
            let mut text = String::with_capacity(chunk.len() * 20);
            for o in chunk {
                text.push_str(&format!("{o:010} 00000 n \n"));
            }
            self.raw(text.as_bytes())?;
        }
        self.raw(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", count + 1).as_bytes())?;
        self.out.flush()?;
        Ok(self.out)
    }
}

/// O texto no retângulo: as linhas, o alinhamento e a fonte (Helvetica, negrito, itálico).
fn text_ops(out: &mut String, text: &str, rect: RectF, style: &TextStyle) {
    let size = style.font.size.max(1.0);
    let lines = lines_of(text, rect, style);
    let line_h = size * 1.2;
    let block = line_h * lines.len() as f32;
    let top = match style.v_align {
        TextAlign::Leading => rect.y,
        TextAlign::Trailing => rect.bottom() - block,
        _ => rect.y + (rect.height - block) / 2.0,
    };
    let font = match (is_bold(style), style.font.italic) {
        (false, false) => "F1",
        (true, false) => "F2",
        (false, true) => "F3",
        (true, true) => "F4",
    };
    fill_style(out, style.color);
    for (i, line) in lines.iter().enumerate() {
        let w = text_width(line, style);
        let x = match style.h_align {
            TextAlign::Center => rect.x + (rect.width - w) / 2.0,
            TextAlign::Trailing => rect.right() - w,
            _ => rect.x,
        };
        // A linha de base: 80% da altura da fonte abaixo do topo da linha.
        let baseline = top + line_h * i as f32 + (line_h - size) / 2.0 + size * 0.8;
        let mut encoded = String::new();
        for c in line.chars() {
            let (_, b) = win_ansi(c);
            match b {
                b'(' | b')' | b'\\' => {
                    encoded.push('\\');
                    encoded.push(b as char);
                }
                0x20..=0x7E => encoded.push(b as char),
                _ => encoded.push_str(&format!("\\{b:03o}")),
            }
        }
        out.push_str(&format!("BT /{font} {} Tf 1 0 0 -1 {} {} Tm ({encoded}) Tj ET\n", num(size), num(x), num(baseline)));
        if style.font.underline {
            out.push_str(&format!("{} w {} {} m {} {} l S\n", num(size / 14.0), num(x), num(baseline + size * 0.12), num(x + w), num(baseline + size * 0.12)));
        }
    }
}
