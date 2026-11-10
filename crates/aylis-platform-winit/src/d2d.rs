//! Desenho com Direct2D (Windows, sem Skia): direto na janela, acelerado pela GPU.

use std::num::NonZeroU32;
use std::rc::Rc;

use aylis_core::Color;
use aylis_d2d::{D2dCanvas, D2dRenderer, HwndTarget};
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowAttributes};

use crate::Error;

pub(crate) struct D2dBackend {
    target: HwndTarget,
}

impl D2dBackend {
    /// A placa de vídeo em que a janela desenha.
    pub(crate) fn adapter(&self) -> Option<String> {
        self.target.adapter().map(str::to_string)
    }
}

/// `vsync = false` apresenta sem esperar o monitor (medições).
pub(crate) fn create(
    event_loop: &ActiveEventLoop,
    attributes: WindowAttributes,
    renderer: &D2dRenderer,
    vsync: bool,
) -> Result<(Rc<Window>, D2dBackend), Error> {
    let window = event_loop.create_window(attributes)?;
    let RawWindowHandle::Win32(handle) = window.window_handle()?.as_raw() else {
        return Err("Direct2D: a janela não é do Windows".into());
    };
    let size = window.inner_size();
    let target = renderer.hwnd_target(handle.hwnd.get(), size.width, size.height, vsync)?;
    Ok((Rc::new(window), D2dBackend { target }))
}

impl D2dBackend {
    pub(crate) fn draw(
        &mut self,
        renderer: &D2dRenderer,
        (width, height): (NonZeroU32, NonZeroU32),
        scale: f32,
        background: Color,
        parts: Option<(&[aylis_core::ScrollMove], &[aylis_core::RectF])>,
        paint: impl FnOnce(&mut D2dCanvas<'_>, Option<&[aylis_core::RectF]>),
        capture: bool,
    ) -> Result<Option<Vec<u8>>, Error> {
        // A captura (AYLIS_SNAPSHOT) é um segundo desenho fora da tela, inteiro: o alvo da janela
        // não devolve os pixels. Por isso a pintura vai para um ou para o outro.
        if capture {
            return Ok(renderer.render_rgba(width.get(), height.get(), scale, background, |c| paint(c, None)));
        }
        if !self.target.draw_parts(renderer, width.get(), height.get(), scale, background, parts, paint)? {
            // Dispositivo perdido: o alvo foi recriado; o próximo quadro desenha.
        }
        Ok(None)
    }
}
