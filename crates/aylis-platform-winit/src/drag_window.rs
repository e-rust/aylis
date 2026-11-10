//! Windows: a imagem do arrasto ([`aylis_core::DragVisual`]) numa janela própria, como a do
//! Explorer: meio transparente (alfa por pixel, `UpdateLayeredWindow`), por cima de tudo,
//! transparente para o mouse e sem tomar o foco. Segue o cursor pela tela inteira, fora da
//! janela do app também.

use aylis_core::{DragVisual, ImageSource};
use windows::Win32::Foundation::{HWND, POINT, SIZE};
use windows::Win32::Graphics::Gdi::{
    AC_SRC_ALPHA, AC_SRC_OVER, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, DeleteDC,
    DeleteObject, GetDC, ReleaseDC, SelectObject,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, GetCursorPos, HWND_TOPMOST, SW_HIDE, SWP_NOACTIVATE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos, ShowWindow,
    ULW_ALPHA, UpdateLayeredWindow, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use windows::core::w;

/// A janela da imagem do arrasto. Criada no primeiro arrasto com imagem e reaproveitada.
#[derive(Default)]
pub struct DragWindow {
    hwnd: Option<HWND>,
    /// O que está desenhado: (imagem, largura, altura, opacidade em milésimos).
    key: Option<(u64, u32, u32, u32)>,
    /// O ponto da imagem (em pixels) que fica no cursor.
    hotspot: (i32, i32),
    shown: bool,
}

impl DragWindow {
    /// Mostra (e põe no cursor) a imagem do arrasto em andamento, ou esconde com `None`.
    /// `scale`: pixels por ponto da janela onde o arrasto começou.
    pub fn sync(&mut self, visual: Option<(&DragVisual, f32)>) {
        let Some((v, scale)) = visual else {
            if self.shown {
                if let Some(h) = self.hwnd {
                    unsafe {
                        let _ = ShowWindow(h, SW_HIDE);
                    }
                }
                self.shown = false;
            }
            return;
        };
        let (w, h) = ((v.size.width * scale).round().max(1.0) as u32, (v.size.height * scale).round().max(1.0) as u32);
        let key = (v.image.id(), w, h, (v.opacity.clamp(0.0, 1.0) * 1000.0) as u32);
        let mut cursor = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut cursor);
        }
        if self.key != Some(key) {
            let Some(pixels) = render(v, w, h, scale) else { return };
            let Some(hwnd) = self.window() else { return };
            self.hotspot = ((v.hotspot.x * scale).round() as i32, (v.hotspot.y * scale).round() as i32);
            let at = POINT { x: cursor.x - self.hotspot.0, y: cursor.y - self.hotspot.1 };
            if !update_layered(hwnd, &pixels, w, h, at) {
                return;
            }
            self.key = Some(key);
        }
        let Some(hwnd) = self.hwnd else { return };
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                cursor.x - self.hotspot.0,
                cursor.y - self.hotspot.1,
                0,
                0,
                SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
        }
        self.shown = true;
    }

    fn window(&mut self) -> Option<HWND> {
        if self.hwnd.is_none() {
            // A classe `STATIC` do sistema: não precisa de procedimento de janela próprio.
            let ex = WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;
            self.hwnd = unsafe { CreateWindowExW(ex, w!("STATIC"), w!(""), WS_POPUP, 0, 0, 1, 1, None, None, None, None) }.ok();
        }
        self.hwnd
    }
}

impl Drop for DragWindow {
    fn drop(&mut self) {
        if let Some(h) = self.hwnd.take() {
            unsafe {
                let _ = DestroyWindow(h);
            }
        }
    }
}

/// Os pixels BGRA pré-multiplicados da imagem em `w`×`h`: o centro dela cobrindo o
/// retângulo (sem deformar), com a opacidade e os cantos arredondados.
fn render(v: &DragVisual, w: u32, h: u32, scale: f32) -> Option<Vec<u8>> {
    let (rgba, iw, ih) = match v.image.source() {
        ImageSource::Rgba { width, height, pixels } => (pixels.clone(), *width, *height),
        ImageSource::Bgra { width, height, pixels } => {
            let mut p = pixels.clone();
            for px in p.chunks_exact_mut(4) {
                px.swap(0, 2);
            }
            (p, *width, *height)
        }
        ImageSource::Encoded(bytes) => crate::decode_rgba(bytes)?,
    };
    if iw == 0 || ih == 0 || rgba.len() < (iw * ih * 4) as usize {
        return None;
    }
    let (src, _) = aylis_core::ImageFit::Cover.place(aylis_core::SizeF::new(iw as f32, ih as f32), aylis_core::RectF::new(0.0, 0.0, w as f32, h as f32));
    let radius = 6.0 * scale;
    let opacity = v.opacity.clamp(0.0, 1.0);
    let mut out = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            // Amostragem bilinear do pedaço `src` da imagem.
            let fx = src.x + (x as f32 + 0.5) * src.width / w as f32 - 0.5;
            let fy = src.y + (y as f32 + 0.5) * src.height / h as f32 - 0.5;
            let c = bilinear(&rgba, iw, ih, fx, fy);
            let a = c[3] / 255.0 * opacity * corner_coverage(x as f32 + 0.5, y as f32 + 0.5, w as f32, h as f32, radius);
            let i = ((y * w + x) * 4) as usize;
            out[i] = (c[2] * a).round() as u8;
            out[i + 1] = (c[1] * a).round() as u8;
            out[i + 2] = (c[0] * a).round() as u8;
            out[i + 3] = (a * 255.0).round() as u8;
        }
    }
    Some(out)
}

fn bilinear(rgba: &[u8], w: u32, h: u32, x: f32, y: f32) -> [f32; 4] {
    let x = x.clamp(0.0, (w - 1) as f32);
    let y = y.clamp(0.0, (h - 1) as f32);
    let (x0, y0) = (x.floor() as u32, y.floor() as u32);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let (tx, ty) = (x - x0 as f32, y - y0 as f32);
    let px = |x: u32, y: u32, c: usize| rgba[((y * w + x) * 4) as usize + c] as f32;
    let mut out = [0.0; 4];
    for (c, o) in out.iter_mut().enumerate() {
        let top = px(x0, y0, c) * (1.0 - tx) + px(x1, y0, c) * tx;
        let bottom = px(x0, y1, c) * (1.0 - tx) + px(x1, y1, c) * tx;
        *o = top * (1.0 - ty) + bottom * ty;
    }
    out
}

/// Quanto do pixel (centro em `x`, `y`) fica dentro do retângulo de cantos arredondados.
fn corner_coverage(x: f32, y: f32, w: f32, h: f32, r: f32) -> f32 {
    let cx = if x < r { r } else if x > w - r { w - r } else { return 1.0 };
    let cy = if y < r { r } else if y > h - r { h - r } else { return 1.0 };
    let d = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
    (r - d + 0.5).clamp(0.0, 1.0)
}

/// Põe os pixels na janela em camadas, em `at` (coordenadas da tela).
fn update_layered(hwnd: HWND, bgra: &[u8], w: u32, h: u32, at: POINT) -> bool {
    unsafe {
        let screen = GetDC(None);
        let mem = CreateCompatibleDC(Some(screen));
        let info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                // Negativo: as linhas de cima para baixo, como os pixels.
                biHeight: -(h as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut bits = std::ptr::null_mut();
        let ok = match CreateDIBSection(Some(mem), &info, DIB_RGB_COLORS, &mut bits, None, 0) {
            Ok(bitmap) if !bits.is_null() => {
                std::ptr::copy_nonoverlapping(bgra.as_ptr(), bits as *mut u8, bgra.len());
                let old = SelectObject(mem, bitmap.into());
                let size = SIZE { cx: w as i32, cy: h as i32 };
                let blend = BLENDFUNCTION { BlendOp: AC_SRC_OVER as u8, BlendFlags: 0, SourceConstantAlpha: 255, AlphaFormat: AC_SRC_ALPHA as u8 };
                let ok = UpdateLayeredWindow(hwnd, Some(screen), Some(&at), Some(&size), Some(mem), Some(&POINT::default()), Default::default(), Some(&blend), ULW_ALPHA)
                    .is_ok();
                SelectObject(mem, old);
                let _ = DeleteObject(bitmap.into());
                ok
            }
            _ => false,
        };
        let _ = DeleteDC(mem);
        ReleaseDC(None, screen);
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_are_rounded_and_the_middle_is_solid() {
        assert_eq!(corner_coverage(50.0, 30.0, 100.0, 60.0, 6.0), 1.0, "o meio");
        assert_eq!(corner_coverage(0.5, 0.5, 100.0, 60.0, 6.0), 0.0, "a ponta do canto fica de fora");
        assert_eq!(corner_coverage(3.0, 30.0, 100.0, 60.0, 6.0), 1.0, "a borda reta fica");
    }

    #[test]
    fn a_red_image_comes_out_premultiplied_in_bgra() {
        let image = aylis_core::Image::rgba(2, 2, [255, 0, 0, 255].repeat(4));
        let mut v = DragVisual::new(image, aylis_core::SizeF::new(20.0, 20.0));
        v.opacity = 0.5;
        let px = render(&v, 20, 20, 1.0).unwrap();
        let mid = ((10 * 20 + 10) * 4) as usize;
        assert_eq!(&px[mid..mid + 4], &[0, 0, 128, 128], "B G R A, com metade de opacidade");
    }
}
