//! A "impressão digital" do desenho próprio de um controle (sem os filhos): um canvas que
//! resume cada operação num hash, repassando ou não ao renderizador. A pintura guarda a do
//! último quadro; quando só o estado de um contêiner muda (o mouse entrou no fundo do form),
//! o `Ui` ensaia o desenho dele de novo e, se der a mesma, não marca área nenhuma — antes,
//! passar o mouse do botão para o fundo repintava a janela inteira à toa.
//!
//! Só vale para quem declara que pinta sem efeito colateral
//! ([`Component::paint_is_pure`](crate::Component::paint_is_pure)): o ensaio chama a
//! pintura do componente fora do quadro.

use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::canvas::{Brush, Canvas, Shadow, Stroke, TextService, TextStyle};
use crate::geometry::{PointF, RectF, SizeF};
use crate::Path;

/// O hash recebe o texto do `Debug` sem montar a `String`.
struct Feed<'a>(&'a mut DefaultHasher);

impl std::fmt::Write for Feed<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}

/// Resume as operações em `hasher` e repassa a `inner` (sem `inner`, é o ensaio: nada é
/// desenhado e o texto é medido pelo serviço da plataforma).
pub(crate) struct TraceCanvas<'a> {
    pub(crate) inner: Option<&'a mut dyn Canvas>,
    pub(crate) text: Option<&'a dyn TextService>,
    pub(crate) hasher: &'a mut DefaultHasher,
    /// Dentro de um `begin_cached` que não achou a imagem: o desenho dali é o da chave, que
    /// já entrou no hash (com a imagem guardada, ele nem acontece).
    cached: u32,
    /// Desenhou algo que muda sem o hash saber (uma imagem que já está na GPU): o resumo
    /// não serve para comparar.
    pub(crate) volatile: bool,
}

impl<'a> TraceCanvas<'a> {
    pub(crate) fn new(inner: Option<&'a mut dyn Canvas>, text: Option<&'a dyn TextService>, hasher: &'a mut DefaultHasher) -> Self {
        Self { inner, text, hasher, cached: 0, volatile: false }
    }

    fn note(&mut self, tag: u8, args: impl std::fmt::Debug) {
        if self.cached == 0 {
            tag.hash(self.hasher);
            let _ = write!(Feed(self.hasher), "{args:?}");
        }
    }
}

impl Canvas for TraceCanvas<'_> {
    fn save(&mut self) {
        self.note(0, ());
        if let Some(c) = self.inner.as_mut() {
            c.save();
        }
    }
    fn restore(&mut self) {
        self.note(1, ());
        if let Some(c) = self.inner.as_mut() {
            c.restore();
        }
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.note(2, (dx, dy));
        if let Some(c) = self.inner.as_mut() {
            c.translate(dx, dy);
        }
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        self.note(3, (sx, sy));
        if let Some(c) = self.inner.as_mut() {
            c.scale(sx, sy);
        }
    }
    fn rotate(&mut self, degrees: f32) {
        self.note(4, degrees);
        if let Some(c) = self.inner.as_mut() {
            c.rotate(degrees);
        }
    }
    fn clip_rect(&mut self, rect: RectF) {
        self.note(5, rect);
        if let Some(c) = self.inner.as_mut() {
            c.clip_rect(rect);
        }
    }
    fn save_layer(&mut self, opacity: f32) {
        self.note(6, opacity);
        if let Some(c) = self.inner.as_mut() {
            c.save_layer(opacity);
        }
    }
    fn fill_rect(&mut self, rect: RectF, radius: f32, brush: &Brush) {
        self.note(7, (rect, radius, brush));
        if let Some(c) = self.inner.as_mut() {
            c.fill_rect(rect, radius, brush);
        }
    }
    fn stroke_rect(&mut self, rect: RectF, radius: f32, stroke: &Stroke) {
        self.note(8, (rect, radius, stroke));
        if let Some(c) = self.inner.as_mut() {
            c.stroke_rect(rect, radius, stroke);
        }
    }
    fn fill_ellipse(&mut self, rect: RectF, brush: &Brush) {
        self.note(9, (rect, brush));
        if let Some(c) = self.inner.as_mut() {
            c.fill_ellipse(rect, brush);
        }
    }
    fn stroke_ellipse(&mut self, rect: RectF, stroke: &Stroke) {
        self.note(10, (rect, stroke));
        if let Some(c) = self.inner.as_mut() {
            c.stroke_ellipse(rect, stroke);
        }
    }
    fn draw_line(&mut self, from: PointF, to: PointF, stroke: &Stroke) {
        self.note(11, (from, to, stroke));
        if let Some(c) = self.inner.as_mut() {
            c.draw_line(from, to, stroke);
        }
    }
    fn draw_text(&mut self, text: &str, rect: RectF, style: &TextStyle) {
        self.note(12, (text, rect, style));
        if let Some(c) = self.inner.as_mut() {
            c.draw_text(text, rect, style);
        }
    }
    fn measure_text(&mut self, text: &str, style: &TextStyle) -> SizeF {
        match (self.inner.as_mut(), self.text) {
            (Some(c), _) => c.measure_text(text, style),
            (None, Some(service)) => service.measure(text, style, None),
            (None, None) => SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.2),
        }
    }
    fn draw_image(&mut self, image: &crate::Image, src: RectF, dest: RectF, opacity: f32) {
        self.note(13, (image.id(), src, dest, opacity));
        if let Some(c) = self.inner.as_mut() {
            c.draw_image(image, src, dest, opacity);
        }
    }
    fn image_size(&mut self, image: &crate::Image) -> Option<SizeF> {
        self.inner.as_mut().and_then(|c| c.image_size(image))
    }
    fn clip_round_rect(&mut self, rect: RectF, radius: f32) {
        self.note(14, (rect, radius));
        if let Some(c) = self.inner.as_mut() {
            c.clip_round_rect(rect, radius);
        }
    }
    fn fill_path(&mut self, path: &Path, brush: &Brush) {
        self.note(15, (path, brush));
        if let Some(c) = self.inner.as_mut() {
            c.fill_path(path, brush);
        }
    }
    fn clip_path(&mut self, path: &Path, inverse: bool) {
        self.note(16, (path, inverse));
        if let Some(c) = self.inner.as_mut() {
            c.clip_path(path, inverse);
        }
    }
    fn backdrop_blur(&mut self, rect: RectF, radius: f32, blur: f32) -> bool {
        self.note(17, (rect, radius, blur));
        self.inner.as_mut().is_none_or(|c| c.backdrop_blur(rect, radius, blur))
    }
    fn begin_masked(&mut self, inverse: bool) -> bool {
        self.note(18, inverse);
        self.inner.as_mut().is_none_or(|c| c.begin_masked(inverse))
    }
    fn begin_mask(&mut self) {
        self.note(19, ());
        if let Some(c) = self.inner.as_mut() {
            c.begin_mask();
        }
    }
    fn end_mask(&mut self) {
        self.note(20, ());
        if let Some(c) = self.inner.as_mut() {
            c.end_mask();
        }
    }
    fn begin_cached(&mut self, key: u64, rect: RectF) -> bool {
        self.note(21, (key, rect));
        let hit = self.inner.as_mut().is_some_and(|c| c.begin_cached(key, rect));
        if !hit {
            self.cached += 1;
        }
        hit
    }
    fn end_cached(&mut self) {
        self.cached = self.cached.saturating_sub(1);
        if let Some(c) = self.inner.as_mut() {
            c.end_cached();
        }
    }
    fn forget_cached(&mut self, key: u64) {
        if let Some(c) = self.inner.as_mut() {
            c.forget_cached(key);
        }
    }
    fn stroke_path(&mut self, path: &Path, stroke: &Stroke) {
        self.note(22, (path, stroke));
        if let Some(c) = self.inner.as_mut() {
            c.stroke_path(path, stroke);
        }
    }
    fn draw_shadow(&mut self, rect: RectF, radius: f32, shadow: &Shadow) {
        self.note(23, (rect, radius, shadow));
        if let Some(c) = self.inner.as_mut() {
            c.draw_shadow(rect, radius, shadow);
        }
    }
    fn device_scale(&self) -> f32 {
        self.inner.as_ref().map_or(1.0, |c| c.device_scale())
    }
    fn gpu_device(&self) -> Option<crate::GpuDevice> {
        self.inner.as_ref().and_then(|c| c.gpu_device())
    }
    fn draw_gpu_surface(&mut self, dest: RectF, surface: &crate::GpuSurface) -> bool {
        self.note(24, dest);
        self.volatile = true;
        self.inner.as_mut().is_some_and(|c| c.draw_gpu_surface(dest, surface))
    }
    fn gpu_state_changed(&mut self) {
        if let Some(c) = self.inner.as_mut() {
            c.gpu_state_changed();
        }
    }
}
