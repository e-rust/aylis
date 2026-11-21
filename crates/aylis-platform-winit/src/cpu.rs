//! Desenho na CPU: Skia raster direto no buffer do `softbuffer`.

use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::rc::Rc;

use aylis_core::{Color, RectF};
use aylis_skia::{SkiaCanvas, SkiaRenderer};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::Error;

/// Quantos quadros de áreas pintadas se guardam (o Wayland alterna 2 ou 3 buffers).
const HISTORY: usize = 4;

pub(crate) struct CpuBackend {
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    // Precisa viver tanto quanto a surface.
    _context: softbuffer::Context<Rc<Window>>,
    /// As áreas pintadas nos últimos quadros, do mais novo ao mais velho (`None`: o quadro
    /// inteiro), e o tamanho delas: um buffer de idade `n` tem o quadro de `n` atrás, e perdeu
    /// o que os `n - 1` seguintes pintaram.
    history: VecDeque<Option<Vec<RectF>>>,
    size: (u32, u32),
}

pub(crate) fn create(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Result<(Rc<Window>, CpuBackend), Error> {
    let window = Rc::new(event_loop.create_window(attributes)?);
    let context = softbuffer::Context::new(window.clone())?;
    let surface = softbuffer::Surface::new(&context, window.clone())?;
    Ok((window, CpuBackend { surface, _context: context, history: VecDeque::new(), size: (0, 0) }))
}

/// O que pintar num buffer de idade `age`, dadas as áreas do quadro (`areas`) e as dos quadros
/// de antes (`history`, do mais novo ao mais velho): as do quadro mais as dos `age - 1`
/// anteriores, que o buffer não viu. `None`: o quadro inteiro (o buffer novo, de outro tamanho,
/// mais velho que o que se guardou, ou um dos quadros que ele perdeu foi inteiro).
fn areas_for_age(age: u8, areas: &[RectF], history: &VecDeque<Option<Vec<RectF>>>) -> Option<Vec<RectF>> {
    let age = age as usize;
    if age == 0 || age > history.len() + 1 {
        return None;
    }
    let mut out = areas.to_vec();
    for frame in history.iter().take(age - 1) {
        out.extend(frame.as_ref()?.iter().copied());
    }
    Some(out)
}

impl CpuBackend {
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(crate) fn draw(
        &mut self,
        renderer: &SkiaRenderer,
        (width, height): (NonZeroU32, NonZeroU32),
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[RectF])>,
        paint: impl FnOnce(&mut SkiaCanvas<'_>, Option<&[RectF]>),
        capture: bool,
    ) -> Result<Option<Vec<u8>>, Error> {
        self.surface.resize(width, height)?;
        let mut buffer = self.surface.buffer_mut()?;
        let size = (width.get(), height.get());
        if size != self.size {
            self.history.clear();
            self.size = size;
        }
        // Só as áreas que mudaram, quando o buffer ainda tem um quadro conhecido (o softbuffer
        // diz a idade dele; um buffer novo ou de outro tamanho tem idade 0). Com idade maior que
        // 1 (o Wayland alterna buffers), também o que os quadros que ele não viu pintaram: antes
        // só o deste quadro, e o efeito do mouse sobre uma aba deixava lixo na tela (testado no
        // Fedora em 20/11). As cópias da rolagem só sobre o quadro de logo antes (idade 1). A
        // captura pinta tudo.
        let age = buffer.age();
        let plan = parts.filter(|(moves, _)| !capture && (moves.is_empty() || age == 1)).and_then(|(moves, areas)| {
            let wanted = if moves.is_empty() { areas_for_age(age, areas, &self.history) } else { (age == 1).then(|| areas.to_vec()) };
            wanted.map(|all| (moves, areas.to_vec(), all))
        });
        match &plan {
            Some((moves, _, all)) => {
                renderer.render_parts_to_pixels(&mut buffer, width.get(), height.get(), scale, background, moves, all, |c, a| paint(c, Some(a)));
            }
            None => {
                renderer.render_area_to_pixels(&mut buffer, width.get(), height.get(), scale, background, None, |c| paint(c, None));
            }
        }
        // O que este quadro pintou (só as áreas dele: as dos quadros perdidos já estão na lista).
        self.history.push_front(plan.map(|(_, areas, _)| areas));
        self.history.truncate(HISTORY);
        // Cada pixel do softbuffer é 0x00RRGGBB.
        let frame: Option<Vec<u8>> = capture.then(|| buffer.iter().flat_map(|&p| [(p >> 16) as u8, (p >> 8) as u8, p as u8, 255]).collect());
        buffer.present()?;
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(x: f32) -> RectF {
        RectF::new(x, 0.0, 10.0, 10.0)
    }

    /// Com idade 2 (o Wayland), o buffer perdeu o quadro de logo antes: pinta também o que ele
    /// pintou. Idade 0, mais velho que o guardado ou um quadro inteiro no meio: tudo.
    #[test]
    fn an_older_buffer_also_paints_what_it_missed() {
        let mut history = VecDeque::new();
        history.push_front(Some(vec![r(1.0)])); // o quadro de logo antes
        history.push_front(Some(vec![r(2.0)])); // o mais novo antes deste
        assert_eq!(areas_for_age(1, &[r(9.0)], &history), Some(vec![r(9.0)]));
        assert_eq!(areas_for_age(2, &[r(9.0)], &history), Some(vec![r(9.0), r(2.0)]));
        assert_eq!(areas_for_age(3, &[r(9.0)], &history), Some(vec![r(9.0), r(2.0), r(1.0)]));
        assert_eq!(areas_for_age(4, &[r(9.0)], &history), None, "mais velho que o guardado");
        assert_eq!(areas_for_age(0, &[r(9.0)], &history), None, "buffer novo");
        history.push_front(None);
        assert_eq!(areas_for_age(2, &[r(9.0)], &history), None, "o quadro perdido foi inteiro");
    }
}
