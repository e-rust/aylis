//! Páginas gravadas: o que se desenha num [`RecordingCanvas`] vira uma lista de operações
//! que se repete em qualquer canvas ([`Page::replay`], a visualização de um relatório) ou
//! vira PDF pelo provedor de PDF ([`crate::providers::PdfProvider`]; o núcleo não escreve PDF).
//!
//! As medidas são em pontos (1/72 de polegada): uma página A4 tem 595 × 842.

use crate::canvas::{Brush, Canvas, TextService, TextStyle};
use crate::color::Color;
use crate::geometry::{PointF, RectF, SizeF};
use crate::image::Image;
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

/// Largura aproximada do texto (pelas medidas da Helvetica), no tamanho do estilo: a medida do
/// [`RecordingCanvas`] sem serviço de texto (sem janela).
pub fn text_width(text: &str, style: &TextStyle) -> f32 {
    let bold = is_bold(style);
    text.chars().map(|c| char_width(c, bold)).sum::<f32>() * style.font.size
}
