//! [`FrameView`]: mostra quadros prontos vindos de fora (um decodificador de vídeo, uma
//! câmera, um renderizador próprio).

use aylis_core::{Brush, Canvas, Color, Component, ControlBase, Image, PaintCtx, PointF, RectF, SizeF, properties};

use crate::leaf_control;

/// O palco de um player ou de uma prévia que decodifica o vídeo por conta própria: o fundo
/// preto e o último quadro entregue ([`FrameView::set_frame`]) no retângulo dito, em
/// coordenadas do controle (o decodificador já escalou para o tamanho da tela).
/// [`set_shift`](FrameView::set_shift) desloca o quadro enquanto o próximo não chega (o
/// arrasto de uma vista com zoom). Os gestos (roda, arrasto, clique) chegam pelos eventos
/// comuns (`on_wheel`, `on_pointer_down`...).
pub struct FrameView {
    pub base: ControlBase,
    /// A cor do fundo (onde o quadro não cobre).
    pub background: Color,
    frame: Option<(Image, RectF)>,
    shift: PointF,
}

impl Default for FrameView {
    fn default() -> Self {
        Self { base: ControlBase::new(640.0, 360.0), background: Color::rgb(0, 0, 0), frame: None, shift: PointF::ZERO }
    }
}

impl FrameView {
    /// O quadro novo e onde ele fica (pixels lógicos do controle). `None` limpa.
    pub fn set_frame(&mut self, frame: Option<(Image, RectF)>) {
        self.frame = frame;
        self.shift = PointF::ZERO;
    }

    pub fn has_frame(&self) -> bool {
        self.frame.is_some()
    }

    pub fn set_shift(&mut self, dx: f32, dy: f32) {
        self.shift = PointF::new(dx, dy);
    }
}

impl Component for FrameView {
    leaf_control!("FrameView");

    properties! {
        background: Color => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // A cor própria do controle (`fill`), quando definida, ganha da `background`.
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.colors.fill.unwrap_or(self.background)));
        let Some((image, dest)) = &self.frame else { return };
        let size = image.known_size().or_else(|| canvas.image_size(image)).unwrap_or(SizeF::new(dest.width, dest.height));
        canvas.save();
        canvas.clip_rect(r);
        let dest = RectF::new(r.x + dest.x + self.shift.x, r.y + dest.y + self.shift.y, dest.width, dest.height);
        canvas.draw_image(image, RectF::new(0.0, 0.0, size.width, size.height), dest, 1.0);
        canvas.restore();
    }
}
