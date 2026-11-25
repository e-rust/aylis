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
        let dest = pixel_aligned(dest, canvas.device_scale(), size);
        canvas.draw_image(image, RectF::new(0.0, 0.0, size.width, size.height), dest, 1.0);
        canvas.restore();
    }
}

/// O retângulo do quadro nos pixels da tela (`scale`: pixels por unidade, o
/// `Canvas::device_scale`): o canto cai num pixel inteiro e, quando o quadro tem o tamanho do
/// retângulo em pixels (o decodificador escalou para a tela, `largura = pixels / escala`), o
/// tamanho fica exato. Assim o quadro sai pixel a pixel, sem o borrão da interpolação, com
/// qualquer escala (1,25, 1,5, a fracionária do Wayland). Supõe o deslocamento da janela em
/// pixels inteiros, como a pintura do núcleo faz.
fn pixel_aligned(dest: RectF, scale: f32, image: SizeF) -> RectF {
    if !(scale.is_finite() && scale > 0.0) {
        return dest;
    }
    let snap = |v: f32| (v * scale).round() / scale;
    let (x, y) = (snap(dest.x), snap(dest.y));
    // O tamanho em pixels a menos de meio pixel do da imagem: é 1:1.
    let side = |start: f32, len: f32, pixels: f32| {
        if pixels > 0.0 && (len * scale - pixels).abs() < 0.5 { pixels / scale } else { snap(start + len) - snap(start) }
    };
    RectF::new(x, y, side(dest.x, dest.width, image.width), side(dest.y, dest.height, image.height))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Com a escala 1,5, o quadro de 1920 × 1080 num retângulo de 1280 × 720 fica com o canto
    /// num pixel inteiro e exatamente 1920 × 1080 pixels; um retângulo de outro tamanho só tem
    /// as bordas alinhadas.
    #[test]
    fn the_frame_lands_on_whole_device_pixels() {
        let image = SizeF::new(1920.0, 1080.0);
        let r = pixel_aligned(RectF::new(10.3, 5.1, 1280.2, 719.9), 1.5, image);
        assert_eq!(((r.x * 1.5).round(), (r.y * 1.5).round()), (15.0, 8.0));
        assert!((r.x * 1.5 - 15.0).abs() < 1e-3 && (r.y * 1.5 - 8.0).abs() < 1e-3);
        assert!((r.width * 1.5 - 1920.0).abs() < 1e-3 && (r.height * 1.5 - 1080.0).abs() < 1e-3);
        let r = pixel_aligned(RectF::new(0.0, 0.0, 333.3, 100.0), 1.25, image);
        assert!((r.width * 1.25 - (333.3f32 * 1.25).round()).abs() < 1e-3);
        assert_eq!(pixel_aligned(RectF::new(1.0, 2.0, 3.0, 4.0), 0.0, image), RectF::new(1.0, 2.0, 3.0, 4.0));
    }
}
