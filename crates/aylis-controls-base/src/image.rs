//! Imagem.

use std::cell::{Cell, RefCell};

use aylis_core::{Canvas, Component, ControlBase, EventCtx, GestureEvent, Image, ImageFit, PaintCtx, PointF, RectF, Stroke, WheelEvent, properties};

use crate::leaf_control;

/// Mostra uma imagem do aplicativo (`image: "assets/images/logo.png"`, lida pelo serviço
/// de recursos do projeto) ou uma [`Image`] definida por código ([`set_picture`]).
///
/// Na IDE (modo de design), sem imagem, mostra um contorno tracejado para ser encontrada.
///
/// Com `zoomable` (decidido em 14/11; desligado para o ícone e o logo não reagirem), a pinça
/// amplia de 1x a 5x em volta dos dedos e os dois dedos arrastam; ampliada, um dedo também
/// arrasta (sem ampliar, o arrasto vai para a rolagem de fora).
///
/// [`set_picture`]: ImageView::set_picture
#[derive(Default)]
pub struct ImageView {
    pub base: ControlBase,
    /// Caminho do arquivo, relativo à pasta do projeto.
    pub image: String,
    /// Como a imagem ocupa o controle: `contain` (inteira, mantendo a proporção), `cover`
    /// (cobre, cortando as bordas), `stretch` (estica), `center` ou `original` (tamanho real).
    pub fit: ImageFit,
    /// Arredonda os cantos (recorta a imagem).
    pub radius: f32,
    /// A pinça amplia e o dedo arrasta a imagem ampliada.
    pub zoomable: bool,
    /// A ampliação de agora (1 = sem; 0 vale 1) e o deslocamento da imagem ampliada.
    zoom: Cell<f32>,
    offset: Cell<PointF>,
    /// Imagem definida por código: tem prioridade sobre `image`.
    picture: Option<Image>,
    /// Último arquivo lido: (caminho, imagem).
    loaded: RefCell<Option<(String, Option<Image>)>>,
}

impl ImageView {
    pub fn new(path: impl Into<String>) -> Self {
        Self { image: path.into(), ..Default::default() }
    }

    /// Mostra uma imagem criada por código (gerada, baixada, um quadro de vídeo).
    pub fn set_picture(&mut self, picture: Option<Image>) {
        self.picture = picture;
    }

    pub fn picture(&self) -> Option<&Image> {
        self.picture.as_ref()
    }

    /// A imagem mostrada agora (a definida por código ou a do arquivo), fora da pintura: o
    /// `Clipboard` copia esta.
    pub fn current_image(&self, ui: &aylis_core::Ui) -> Option<Image> {
        if let Some(p) = &self.picture {
            return Some(p.clone());
        }
        if let Some((_, image)) = self.loaded.borrow().as_ref().filter(|(path, _)| *path == self.image) {
            return image.clone();
        }
        (!self.image.is_empty()).then(|| ui.services().resources.as_ref()?.read(&self.image).map(Image::encoded)).flatten()
    }

    fn zoom(&self) -> f32 {
        self.zoom.get().max(1.0)
    }

    /// O deslocamento que mantém a imagem ampliada cobrindo o controle.
    fn clamp_offset(&self, offset: PointF, size: (f32, f32)) -> PointF {
        let z = self.zoom();
        let (w, h) = size;
        PointF::new(offset.x.clamp(w - w * z, 0.0), offset.y.clamp(h - h * z, 0.0))
    }

    fn current(&self, ctx: &PaintCtx<'_>) -> Option<Image> {
        if let Some(p) = &self.picture {
            return Some(p.clone());
        }
        if self.image.is_empty() {
            return None;
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(path, _)| *path != self.image) {
            let image = ctx.resource(&self.image).map(Image::encoded);
            *loaded = Some((self.image.clone(), image));
        }
        loaded.as_ref().and_then(|(_, image)| image.clone())
    }
}

impl Component for ImageView {
    leaf_control!("Image");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        image: String => "Appearance",
        fit: ImageFit => "Appearance",
        radius: f32 => "Appearance",
        zoomable: bool => "Behavior",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        // Desligado, volta ao tamanho de antes.
        if name == "zoomable" && !self.zoomable {
            self.zoom.set(1.0);
            self.offset.set(PointF::ZERO);
            ctx.repaint();
        }
    }

    fn gesture(&mut self, ctx: &mut EventCtx<'_>, event: &GestureEvent) {
        if !self.zoomable {
            return;
        }
        let b = self.base.bounds();
        let (old, new) = (self.zoom(), (self.zoom() * event.delta_scale).clamp(1.0, 5.0));
        // O ponto entre os dedos fica no lugar; o arrasto dos dois dedos anda junto.
        let o = self.offset.get();
        let c = event.center;
        let moved = PointF::new(c.x - (c.x - o.x) * (new / old) + event.pan.x, c.y - (c.y - o.y) * (new / old) + event.pan.y);
        self.zoom.set(new);
        self.offset.set(self.clamp_offset(moved, (b.width, b.height)));
        ctx.repaint();
        ctx.set_handled();
    }

    fn wheel(&mut self, ctx: &mut EventCtx<'_>, event: &WheelEvent) {
        // Ampliada: o dedo arrasta a imagem (sem ampliar, a rolagem de fora recebe).
        if !self.zoomable || !event.touch || self.zoom() <= 1.0 {
            return;
        }
        let b = self.base.bounds();
        let o = self.offset.get();
        self.offset.set(self.clamp_offset(PointF::new(o.x + event.delta.x, o.y + event.delta.y), (b.width, b.height)));
        ctx.repaint();
        ctx.set_handled();
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let image = self.current(ctx);
        let size = image.as_ref().and_then(|i| canvas.image_size(i));
        let (Some(image), Some(size)) = (image, size) else {
            if ctx.design {
                let color = ctx.theme.color("text_muted").with_opacity(0.6);
                canvas.stroke_rect(r.inflate(-0.5), self.radius, &Stroke::new(color, 1.0).dashed(vec![4.0, 3.0]));
            }
            return;
        };
        let (src, dest) = self.fit.place(size, r);
        // Ampliada (`zoomable`): o destino cresce a partir do canto, deslocado.
        let z = self.zoom();
        let dest = if z > 1.0 {
            let o = self.offset.get();
            RectF::new(r.x + o.x + (dest.x - r.x) * z, r.y + o.y + (dest.y - r.y) * z, dest.width * z, dest.height * z)
        } else {
            dest
        };
        canvas.save();
        if self.radius > 0.0 {
            canvas.clip_round_rect(r, self.radius);
        } else {
            canvas.clip_rect(r);
        }
        canvas.draw_image(&image, src, dest, 1.0);
        canvas.restore();
    }
}
