//! O cursor do mouse a partir de uma imagem (decidido em 24/11): o `CursorShape` é uma lista
//! fixa, e quem mostra outra máquina (ou desenha o próprio cursor) precisa de qualquer imagem.
//!
//! O controle (ou o form) ganha a imagem por [`Ui::set_cursor_image`]; sob o ponteiro, ela vale
//! no lugar do `cursor` dele. A plataforma cria o cursor do sistema uma vez por imagem
//! ([`CursorImage::id`]) e guarda: mover o mouse não recria nada.

use std::sync::Arc;

use crate::component::ComponentId;
use crate::image::{Image, ImageSource};
use crate::ui::Ui;

/// O maior lado aceito, em pixels (o limite do winit é 2048; os sistemas costumam mostrar
/// até 256).
pub const MAX_CURSOR_SIDE: u32 = 1024;

/// Quantas imagens diferentes ficam guardadas para reaproveitar o id (a mesma imagem posta de
/// novo não vira outro cursor do sistema).
const RECENT: usize = 32;

/// A imagem de um cursor: pixels RGBA sem pré-multiplicar, em pixels de tela (sem escala), e o
/// ponto de clique.
#[derive(Debug, PartialEq, Eq)]
pub struct CursorImage {
    id: u64,
    width: u32,
    height: u32,
    hot_x: u32,
    hot_y: u32,
    rgba: Vec<u8>,
}

impl CursorImage {
    /// O mesmo id para os mesmos pixels e o mesmo ponto: a chave do cursor guardado na
    /// plataforma.
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// O ponto de clique, em pixels da imagem a partir do canto de cima à esquerda.
    pub fn hotspot(&self) -> (u32, u32) {
        (self.hot_x, self.hot_y)
    }

    /// Os pixels RGBA, linha a linha.
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }
}

/// As imagens por componente e as recentes (para reaproveitar o id).
#[derive(Default)]
struct CursorImages {
    next: u64,
    by_component: Vec<(ComponentId, Arc<CursorImage>)>,
    recent: Vec<Arc<CursorImage>>,
}

/// Os pixels RGBA de uma imagem em pixels (o BGRA é convertido); a codificada, o erro.
fn rgba_of(image: &Image) -> Result<(u32, u32, Vec<u8>), String> {
    match image.source() {
        ImageSource::Rgba { width, height, pixels } => Ok((*width, *height, pixels.clone())),
        ImageSource::Bgra { width, height, pixels } => {
            let mut p = pixels.clone();
            p.chunks_exact_mut(4).for_each(|px| px.swap(0, 2));
            Ok((*width, *height, p))
        }
        ImageSource::Encoded(_) => Err("o cursor precisa da imagem em pixels (RGBA ou BGRA), não de um arquivo codificado".into()),
    }
}

impl Ui {
    /// Mostra a imagem como o cursor sobre o controle (ou o form) `id`, com o ponto de clique
    /// em `hot_x, hot_y` (pixels da imagem). A imagem é RGBA ou BGRA sem pré-multiplicar, em
    /// pixels de tela: não acompanha a escala do monitor. Vale no lugar do `cursor` do
    /// controle, e os filhos sem cursor próprio herdam, como no `cursor`. A mesma imagem de
    /// novo reaproveita o cursor já criado no sistema.
    pub fn set_cursor_image(&mut self, id: ComponentId, image: &Image, hot_x: u32, hot_y: u32) -> Result<(), String> {
        if !self.contains(id) {
            return Err(crate::Error::InvalidId.to_string());
        }
        let (width, height, rgba) = rgba_of(image)?;
        if width == 0 || height == 0 || width > MAX_CURSOR_SIDE || height > MAX_CURSOR_SIDE {
            return Err(format!("o cursor precisa ter de 1 a {MAX_CURSOR_SIDE} pixels por lado (veio {width} × {height})"));
        }
        if rgba.len() != width as usize * height as usize * 4 {
            return Err(format!("o cursor de {width} × {height} precisa de {} bytes (veio {})", width as usize * height as usize * 4, rgba.len()));
        }
        if hot_x >= width || hot_y >= height {
            return Err(format!("o ponto de clique ({hot_x}, {hot_y}) fica fora da imagem de {width} × {height}"));
        }
        let mut images = std::mem::take(self.extension::<CursorImages>());
        let found = images.recent.iter().find(|c| c.width == width && c.height == height && c.hot_x == hot_x && c.hot_y == hot_y && c.rgba == rgba).cloned();
        let cursor = match found {
            Some(c) => c,
            None => {
                images.next += 1;
                let c = Arc::new(CursorImage { id: images.next, width, height, hot_x, hot_y, rgba });
                if images.recent.len() >= RECENT {
                    images.recent.remove(0);
                }
                images.recent.push(c.clone());
                c
            }
        };
        images.by_component.retain(|(c, _)| *c != id && self.contains(*c));
        images.by_component.push((id, cursor));
        *self.extension::<CursorImages>() = images;
        Ok(())
    }

    /// Tira a imagem do cursor de `id`: volta o `cursor` dele.
    pub fn clear_cursor_image(&mut self, id: ComponentId) {
        if self.extension_ref::<CursorImages>().is_some() {
            self.extension::<CursorImages>().by_component.retain(|(c, _)| *c != id);
        }
    }

    /// A imagem posta no próprio `id` (sem olhar os pais).
    pub fn cursor_image_of(&self, id: ComponentId) -> Option<Arc<CursorImage>> {
        let images = self.extension_ref::<CursorImages>()?;
        images.by_component.iter().find(|(c, _)| *c == id).map(|(_, i)| i.clone())
    }

    /// A imagem do cursor a mostrar no form `root`: a do controle sob o ponteiro ou a do pai
    /// mais próximo, parando no primeiro que tem `cursor` próprio (aí vale o [`Ui::cursor`]).
    /// `None`: o cursor é o do [`Ui::cursor`].
    pub fn cursor_image(&self, root: ComponentId) -> Option<Arc<CursorImage>> {
        let images = self.extension_ref::<CursorImages>()?;
        if images.by_component.is_empty() || self.busy_cursor() {
            return None;
        }
        let mut current = self.hovered(root);
        while let Some(c) = current {
            if let Some((_, image)) = images.by_component.iter().find(|(id, _)| *id == c) {
                return Some(image.clone());
            }
            if self.cb(c).is_some_and(|cb| cb.cursor() != crate::CursorShape::Default) {
                return None;
            }
            current = self.parent(c);
        }
        None
    }
}
