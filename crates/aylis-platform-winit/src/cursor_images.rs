//! Os cursores de imagem (`Ui::set_cursor_image`) feitos cursores do sistema pelo
//! `CustomCursor::from_rgba` do winit (Windows, X11, Wayland, macOS e navegador).
//!
//! O winit só cria o cursor com o `ActiveEventLoop` em mãos, e o cursor das janelas é escolhido
//! fora dele (o `after_input`): a imagem que ainda não virou cursor fica na fila, e o
//! `about_to_wait` cria. Cada imagem (pelo id, o mesmo para os mesmos pixels) é criada uma vez
//! e guardada: mover o mouse não recria nada.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use aylis_core::CursorImage;
use winit::event_loop::ActiveEventLoop;
use winit::window::CustomCursor;

/// Quantos cursores ficam guardados (os mais antigos saem; o que uma janela mostra continua
/// com ela).
const KEEP: usize = 64;

#[derive(Default)]
pub(crate) struct CursorImages {
    made: HashMap<u64, CustomCursor>,
    /// A ordem de criação, para tirar os mais antigos.
    order: Vec<u64>,
    /// As imagens esperando o `about_to_wait`.
    missing: Vec<Arc<CursorImage>>,
    /// As que o winit recusou: não tenta de novo.
    failed: HashSet<u64>,
}

impl CursorImages {
    /// O cursor já criado da imagem; sem ele, a imagem entra na fila (e o `None` fica com o
    /// cursor da forma até lá).
    pub(crate) fn get(&mut self, image: &Arc<CursorImage>) -> Option<CustomCursor> {
        let id = image.id();
        if let Some(cursor) = self.made.get(&id) {
            return Some(cursor.clone());
        }
        if !self.failed.contains(&id) && !self.missing.iter().any(|m| m.id() == id) {
            self.missing.push(image.clone());
        }
        None
    }

    /// Cria os cursores da fila. Devolve se criou algum (o cursor das janelas é escolhido de
    /// novo).
    pub(crate) fn create_missing(&mut self, event_loop: &ActiveEventLoop) -> bool {
        let mut created = false;
        for image in std::mem::take(&mut self.missing) {
            let (hot_x, hot_y) = image.hotspot();
            // O núcleo limita o lado a 1024 (`MAX_CURSOR_SIDE`): cabe no `u16`.
            let source = CustomCursor::from_rgba(image.rgba().to_vec(), image.width() as u16, image.height() as u16, hot_x as u16, hot_y as u16);
            match source {
                Ok(source) => {
                    self.made.insert(image.id(), event_loop.create_custom_cursor(source));
                    self.order.push(image.id());
                    if self.order.len() > KEEP {
                        let old = self.order.remove(0);
                        self.made.remove(&old);
                    }
                    created = true;
                }
                Err(e) => {
                    crate::diag::diagnostic(&format!("aylis: o cursor de imagem foi recusado: {e}"));
                    self.failed.insert(image.id());
                }
            }
        }
        created
    }
}
