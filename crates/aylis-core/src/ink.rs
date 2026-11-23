//! Onde cada controle desenhou de verdade: a pintura passa por um canvas que repassa tudo ao
//! renderizador e anota o retângulo de cada desenho (já com a transformação e o recorte). A
//! repintura por parte ([`Ui::take_damage`](crate::Ui::take_damage)) usa essa extensão, e não
//! só o retângulo do controle: um termômetro cujo bulbo passa da borda, uma lista de opções
//! maior que a caixa, um texto sem reticências mais largo que o controle entram na área sem
//! o componente declarar nada.

use std::cell::RefCell;

use crate::canvas::{Brush, Canvas, Shadow, Stroke, TextStyle};
use crate::geometry::{PointF, RectF, SizeF};
use crate::{Path, TextAlign};

/// Transformação afim: `(x, y)` vai para `(a·x + c·y + e, b·x + d·y + f)`.
#[derive(Clone, Copy)]
struct Affine([f32; 6]);

impl Affine {
    const IDENTITY: Affine = Affine([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

    /// `self` seguida de `m` aplicada antes (como o `concat` dos canvases).
    fn then(self, m: [f32; 6]) -> Affine {
        let [a, b, c, d, e, f] = self.0;
        let [ma, mb, mc, md, me, mf] = m;
        Affine([a * ma + c * mb, b * ma + d * mb, a * mc + c * md, b * mc + d * md, a * me + c * mf + e, b * me + d * mf + f])
    }

    fn map(&self, p: PointF) -> PointF {
        let [a, b, c, d, e, f] = self.0;
        PointF::new(a * p.x + c * p.y + e, b * p.x + d * p.y + f)
    }

    /// O retângulo que contém `r` transformado (os quatro cantos).
    fn map_rect(&self, r: RectF) -> RectF {
        let corners = [PointF::new(r.x, r.y), PointF::new(r.right(), r.y), PointF::new(r.x, r.bottom()), PointF::new(r.right(), r.bottom())].map(|p| self.map(p));
        let (mut l, mut t, mut rr, mut b) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for p in corners {
            l = l.min(p.x);
            t = t.min(p.y);
            rr = rr.max(p.x);
            b = b.max(p.y);
        }
        RectF::new(l, t, rr - l, b - t)
    }
}

/// O que um `save` guarda.
#[derive(Clone, Copy)]
struct State {
    transform: Affine,
    clip: Option<RectF>,
}

/// O que o próprio desenho de um controle foi na última pintura, para saber se o fundo atrás
/// de uma rolagem é liso ([`Ui::take_frame`](crate::Ui::take_frame) copia os pixels de uma
/// caixa transparente só sobre um fundo de cor única).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum Plain {
    /// Não desenhou nada (um `Layout`, um `ScrollBox` sem fundo).
    #[default]
    Nothing,
    /// Só um retângulo de cor sólida (`color`), sem cantos e sem giro (o fundo de um form),
    /// aqui (nas coordenadas do form, já recortado).
    Fill(RectF, crate::Color),
    /// Qualquer outra coisa.
    Other,
}

impl Plain {
    /// O de uma pintura nova (`self`) somado ao da anterior: pintado só numa área (a pintura
    /// por parte), o fundo da mesma cor continua liso onde já estava.
    pub(crate) fn after(self, before: Plain) -> Plain {
        match (self, before) {
            (Plain::Fill(r, c), Plain::Fill(old, before)) if c == before => Plain::Fill(r.union(old), c),
            _ => self,
        }
    }

    /// Andou com a rolagem (a cópia levou os pixels junto).
    pub(crate) fn offset(self, dx: f32, dy: f32) -> Plain {
        match self {
            Plain::Fill(r, c) => Plain::Fill(r.offset(dx, dy), c),
            other => other,
        }
    }
}

/// O canvas que anota a extensão de cada desenho em `ink` (nas coordenadas em que a pintura
/// começou) e repassa tudo a `inner`. Em `plain`, se o desenho foi só um fundo liso.
pub(crate) struct InkCanvas<'a> {
    pub(crate) inner: &'a mut dyn Canvas,
    pub(crate) ink: &'a RefCell<Option<RectF>>,
    plain: &'a std::cell::Cell<Plain>,
    state: State,
    stack: Vec<State>,
}

impl<'a> InkCanvas<'a> {
    pub(crate) fn new(inner: &'a mut dyn Canvas, ink: &'a RefCell<Option<RectF>>, plain: &'a std::cell::Cell<Plain>) -> Self {
        Self { inner, ink, plain, state: State { transform: Affine::IDENTITY, clip: None }, stack: Vec::new() }
    }

    /// Anota `r` (nas coordenadas atuais), recortado pelo clip: um desenho qualquer.
    fn mark(&mut self, r: RectF) {
        if self.mark_area(r).is_some() {
            self.plain.set(Plain::Other);
        }
    }

    /// Anota `r` e devolve onde ele ficou (nas coordenadas do começo, recortado); `None` se
    /// ficou todo fora do recorte.
    fn mark_area(&mut self, r: RectF) -> Option<RectF> {
        if !(r.width.is_finite() && r.height.is_finite()) {
            return None;
        }
        let mut r = self.state.transform.map_rect(r);
        if let Some(c) = self.state.clip {
            let x = r.x.max(c.x);
            let y = r.y.max(c.y);
            let (right, bottom) = (r.right().min(c.right()), r.bottom().min(c.bottom()));
            if right <= x || bottom <= y {
                return None;
            }
            r = RectF::new(x, y, right - x, bottom - y);
        }
        let mut ink = self.ink.borrow_mut();
        *ink = Some(ink.map_or(r, |i| i.union(r)));
        Some(r)
    }

    fn mark_stroke(&mut self, r: RectF, stroke: &Stroke) {
        // O traço fica metade para fora; a ponta quadrada e a junção em bico passam um pouco mais.
        self.mark(r.inflate(stroke.width.max(1.0) * 2.0));
    }

    fn clip(&mut self, r: RectF) {
        let r = self.state.transform.map_rect(r);
        self.state.clip = Some(match self.state.clip {
            Some(c) => {
                let x = r.x.max(c.x);
                let y = r.y.max(c.y);
                RectF::new(x, y, (r.right().min(c.right()) - x).max(0.0), (r.bottom().min(c.bottom()) - y).max(0.0))
            }
            None => r,
        });
    }

    /// Onde o texto aparece: sem quebra, ele pode passar do retângulo (centralizado, sobra
    /// para os dois lados). Com quebra, fica na largura (as linhas a mais, embaixo, não são
    /// medidas: o controle que deixa o texto quebrado passar da altura recorta). Com
    /// reticências, também fica na largura, e não precisa medir.
    fn text_rect(&mut self, text: &str, rect: RectF, style: &TextStyle) -> RectF {
        let line = style.font.size * 1.4;
        let size = if style.wrap {
            return rect.inflate(style.font.size * 0.25);
        } else if style.trimming != crate::text::Trimming::None {
            SizeF::new(rect.width, line)
        } else {
            self.inner.measure_text(text, style)
        };
        let place = |start: f32, length: f32, needed: f32, align: TextAlign| {
            if needed <= length {
                return (start, length);
            }
            match align {
                TextAlign::Leading | TextAlign::Justify => (start, needed),
                TextAlign::Center => (start - (needed - length) / 2.0, needed),
                TextAlign::Trailing => (start + length - needed, needed),
            }
        };
        let (x, w) = place(rect.x, rect.width, size.width, style.h_align);
        let (y, h) = place(rect.y, rect.height, size.height, style.v_align);
        // O desenho das letras passa um pouco da medida (acentos, itálico).
        RectF::new(x, y, w, h).inflate(style.font.size * 0.25)
    }
}

impl Canvas for InkCanvas<'_> {
    fn save(&mut self) {
        self.stack.push(self.state);
        self.inner.save();
    }
    fn restore(&mut self) {
        if let Some(s) = self.stack.pop() {
            self.state = s;
        }
        self.inner.restore();
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.state.transform = self.state.transform.then([1.0, 0.0, 0.0, 1.0, dx, dy]);
        self.inner.translate(dx, dy);
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        self.state.transform = self.state.transform.then([sx, 0.0, 0.0, sy, 0.0, 0.0]);
        self.inner.scale(sx, sy);
    }
    fn rotate(&mut self, degrees: f32) {
        let (s, c) = degrees.to_radians().sin_cos();
        self.state.transform = self.state.transform.then([c, s, -s, c, 0.0, 0.0]);
        self.inner.rotate(degrees);
    }
    fn clip_rect(&mut self, rect: RectF) {
        self.clip(rect);
        self.inner.clip_rect(rect);
    }
    fn save_layer(&mut self, opacity: f32) {
        self.stack.push(self.state);
        self.inner.save_layer(opacity);
    }
    fn save_layer_blend(&mut self, opacity: f32, mode: crate::BlendMode, bounds: RectF) {
        self.stack.push(self.state);
        self.inner.save_layer_blend(opacity, mode, bounds);
    }
    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        // O primeiro desenho, um retângulo liso sem giro: pode ser um fundo de cor única.
        let [_, b, c, ..] = self.state.transform.0;
        let flat = match brush {
            Brush::Solid(color) if color.a == 255 && radius <= 0.0 && b == 0.0 && c == 0.0 => Some(*color),
            _ => None,
        };
        let first = self.plain.get() == Plain::Nothing;
        if let Some(r) = self.mark_area(rect) {
            self.plain.set(match flat {
                Some(color) if first => Plain::Fill(r, color),
                _ => Plain::Other,
            });
        }
        self.inner.fill_rect(rect, radius, brush);
    }
    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke) {
        self.mark_stroke(rect, stroke);
        self.inner.stroke_rect(rect, radius, stroke);
    }
    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        self.mark(rect);
        self.inner.fill_ellipse(rect, brush);
    }
    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke) {
        self.mark_stroke(rect, stroke);
        self.inner.stroke_ellipse(rect, stroke);
    }
    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        let r = RectF::new(from.x.min(to.x), from.y.min(to.y), (from.x - to.x).abs(), (from.y - to.y).abs());
        self.mark_stroke(r, stroke);
        self.inner.draw_line(from, to, stroke);
    }
    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        let r = self.text_rect(text, rect, style);
        self.mark(r);
        self.inner.draw_text(text, rect, style);
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        self.inner.measure_text(text, style)
    }
    fn draw_image(&mut self, image: &crate::Image, src: RectF, dest: RectF, opacity: f32) {
        self.mark(dest);
        self.inner.draw_image(image, src, dest, opacity);
    }
    fn image_size(&mut self, image: &crate::Image) -> Option<SizeF> {
        self.inner.image_size(image)
    }
    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        self.clip(rect);
        self.inner.clip_round_rect(rect, radius);
    }
    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        self.mark(path.bounds());
        self.inner.fill_path(path, brush);
    }
    fn clip_path(&mut self, path: &Path, inverse: bool) {
        if !inverse {
            self.clip(path.bounds());
        }
        self.inner.clip_path(path, inverse);
    }
    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        self.mark(rect);
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
        // Guardado, o desenho nem passa por aqui: a área é a do cache.
        self.mark(rect);
        self.inner.begin_cached(key, rect)
    }
    fn end_cached(&mut self) {
        self.inner.end_cached();
    }
    fn forget_cached(&mut self, key: u64) {
        self.inner.forget_cached(key);
    }
    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        self.mark_stroke(path.bounds(), stroke);
        self.inner.stroke_path(path, stroke);
    }
    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &Shadow) {
        self.mark(rect.offset(shadow.offset.x, shadow.offset.y).inflate(shadow.blur * 1.5 + shadow.spread.max(0.0)));
        self.inner.draw_shadow(rect, radius, shadow);
    }
    fn device_scale(&self) -> f32 {
        self.inner.device_scale()
    }
    fn gpu_device(&self) -> Option<crate::GpuDevice> {
        self.inner.gpu_device()
    }
    fn draw_gpu_surface(&mut self, dest: RectF, surface: &crate::GpuSurface) -> bool {
        self.mark(dest);
        self.inner.draw_gpu_surface(dest, surface)
    }
    fn gpu_state_changed(&mut self) {
        self.inner.gpu_state_changed();
    }
}
