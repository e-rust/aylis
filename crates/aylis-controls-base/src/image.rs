//! Imagem (como o `TImage`).

use std::cell::RefCell;

use aylis_core::{Canvas, Component, ControlBase, Image, ImageFit, PaintCtx, Stroke, properties};

use crate::leaf_control;

/// Mostra uma imagem do aplicativo (`image: "assets/images/logo.png"`, lida pelo serviço
/// de recursos do projeto) ou uma [`Image`] definida por código ([`set_picture`]).
///
/// Na IDE (modo de design), sem imagem, mostra um contorno tracejado para ser encontrada.
///
/// [`set_picture`]: ImageView::set_picture
#[derive(Default)]
pub struct ImageView {
    pub base: ControlBase,
    /// Caminho do arquivo, relativo à pasta do projeto.
    pub image: String,
    pub fit: ImageFit,
    /// Arredonda os cantos (recorta a imagem).
    pub radius: f32,
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
