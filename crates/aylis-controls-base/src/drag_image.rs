//! DragImage: o que acompanha o cursor num arrasto (a miniatura de um vídeo, o desenho de
//! um cartão, o ícone de um arquivo). Não visual: um controle `draggable` o aponta em
//! `drag_image`, e o arrasto que começa nele leva a imagem junto do mouse. No Windows ela
//! vai numa janela própria, meio transparente, e segue o mouse por cima de tudo (fora da
//! janela também, como o arrasto do Explorer); nos outros sistemas, o form a pinta junto do
//! mouse.
//!
//! O que vai, na ordem: a imagem definida por código ([`DragImage::set_picture`]), o
//! arquivo do projeto (`image`), o desenho do controle `control` ou, sem nada disso, o
//! desenho do próprio controle arrastado ([`Ui::snapshot`]). Para trocar só num arrasto, o
//! `on_drag_start` chama `ui.set_drag_image(Some(DragVisual::new(imagem, tamanho)))`.

use std::cell::RefCell;

use aylis_core::{Component, ComponentId, DragVisual, Image, PointF, SizeF, Ui, properties};

/// Tamanho de uma imagem sem `width`/`height` (16:9, o de uma miniatura de vídeo).
const DEFAULT_SIZE: SizeF = SizeF::new(160.0, 90.0);
/// O desenho de um controle sai com esta densidade (nítido numa tela de 200%).
const SNAPSHOT_SCALE: f32 = 2.0;

pub struct DragImage {
    /// Arquivo do projeto (`assets/images/arquivo.png`).
    pub image: String,
    /// O controle cujo desenho vai junto (vazio: o próprio controle arrastado, quando não
    /// há imagem).
    pub control: Option<ComponentId>,
    /// Tamanho na tela, em pontos; 0 = automático (160 × 90 numa imagem, o tamanho do
    /// controle num desenho).
    pub width: f32,
    pub height: f32,
    /// 0..1.
    pub opacity: f32,
    /// O ponto da imagem (em pontos) que fica no cursor; negativo = o padrão (no meio, um
    /// pouco acima: como o Explorer).
    pub hot_x: f32,
    pub hot_y: f32,
    /// Imagem definida por código: tem prioridade sobre o resto.
    picture: Option<Image>,
    /// Último arquivo lido: (caminho, imagem).
    loaded: RefCell<Option<(String, Option<Image>)>>,
}

impl Default for DragImage {
    fn default() -> Self {
        Self { image: String::new(), control: None, width: 0.0, height: 0.0, opacity: 0.8, hot_x: -1.0, hot_y: -1.0, picture: None, loaded: RefCell::new(None) }
    }
}

impl DragImage {
    /// Mostra uma imagem criada por código (a miniatura do item).
    pub fn set_picture(&mut self, picture: Option<Image>) {
        self.picture = picture;
    }

    pub fn picture(&self) -> Option<&Image> {
        self.picture.as_ref()
    }

    /// A imagem de arquivo (a definida por código ou a do `image`).
    fn file_image(&self, ui: &Ui) -> Option<Image> {
        if let Some(p) = &self.picture {
            return Some(p.clone());
        }
        if self.image.is_empty() {
            return None;
        }
        let mut loaded = self.loaded.borrow_mut();
        if loaded.as_ref().is_none_or(|(path, _)| *path != self.image) {
            let image = ui.services().resources.as_ref().and_then(|r| r.read(&self.image)).map(Image::encoded);
            *loaded = Some((self.image.clone(), image));
        }
        loaded.as_ref().and_then(|(_, image)| image.clone())
    }
}

impl Component for DragImage {
    fn class_name(&self) -> &'static str {
        "DragImage"
    }

    properties! {
        image: String => "Appearance",
        control: Option<ComponentId> => "Appearance",
        width: f32 => "Layout",
        height: f32 => "Layout",
        opacity: f32 => "Appearance",
        hot_x: f32 => "Layout",
        hot_y: f32 => "Layout",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.control == Some(freed) {
            self.control = None;
        }
    }

    fn drag_visual(&self, ui: &Ui, source: ComponentId) -> Option<DragVisual> {
        let (image, natural) = match self.file_image(ui) {
            Some(image) => (image, DEFAULT_SIZE),
            None => {
                let target = self.control.unwrap_or(source);
                let size = ui.control(target).map(|cb| SizeF::new(cb.width(), cb.height()))?;
                (ui.snapshot(target, SNAPSHOT_SCALE)?, size)
            }
        };
        let size = match (self.width > 0.0, self.height > 0.0) {
            (true, true) => SizeF::new(self.width, self.height),
            // Só um lado: o outro segue a proporção.
            (true, false) => SizeF::new(self.width, self.width * natural.height / natural.width.max(1.0)),
            (false, true) => SizeF::new(self.height * natural.width / natural.height.max(1.0), self.height),
            (false, false) => natural,
        };
        let mut v = DragVisual::new(image, size);
        if self.hot_x >= 0.0 && self.hot_y >= 0.0 {
            v.hotspot = PointF::new(self.hot_x.min(size.width), self.hot_y.min(size.height));
        }
        v.opacity = self.opacity.clamp(0.05, 1.0);
        Some(v)
    }
}
