//! Desenho na CPU: Skia raster direto no buffer do `softbuffer`.

use std::num::NonZeroU32;
use std::rc::Rc;

use aylis_core::Color;
use aylis_skia::{SkiaCanvas, SkiaRenderer};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::Error;

pub(crate) struct CpuBackend {
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    // Precisa viver tanto quanto a surface.
    _context: softbuffer::Context<Rc<Window>>,
}

pub(crate) fn create(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Result<(Rc<Window>, CpuBackend), Error> {
    let window = Rc::new(event_loop.create_window(attributes)?);
    let context = softbuffer::Context::new(window.clone())?;
    let surface = softbuffer::Surface::new(&context, window.clone())?;
    Ok((window, CpuBackend { surface, _context: context }))
}

impl CpuBackend {
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(crate) fn draw(
        &mut self,
        renderer: &SkiaRenderer,
        (width, height): (NonZeroU32, NonZeroU32),
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[aylis_core::RectF])>,
        paint: impl FnOnce(&mut SkiaCanvas<'_>, Option<&[aylis_core::RectF]>),
        capture: bool,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.surface.resize(width, height)?;
        let mut buffer = self.surface.buffer_mut()?;
        // Só as áreas que mudaram, quando o buffer ainda tem o quadro anterior (o softbuffer
        // diz, pela idade dele; um buffer novo ou de outro tamanho tem idade 0); as cópias da
        // rolagem só sobre o quadro de logo antes (idade 1). A captura pinta tudo.
        let parts = parts.filter(|(moves, _)| buffer.age() != 0 && !capture && (moves.is_empty() || buffer.age() == 1));
        match parts {
            Some((moves, areas)) => {
                renderer.render_parts_to_pixels(&mut buffer, width.get(), height.get(), scale, background, moves, areas, |c, a| paint(c, Some(a)));
            }
            None => {
                renderer.render_area_to_pixels(&mut buffer, width.get(), height.get(), scale, background, None, |c| paint(c, None));
            }
        }
        // Cada pixel do softbuffer é 0x00RRGGBB.
        let frame: Option<Vec<u8>> = capture.then(|| buffer.iter().flat_map(|&p| [(p >> 16) as u8, (p >> 8) as u8, p as u8, 255]).collect());
        buffer.present()?;
        Ok(frame)
    }
}
