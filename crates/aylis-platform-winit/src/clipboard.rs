//! A área de transferência do desktop (o `ClipboardService`).
//!
//! - **Windows:** o `clipboard-win` grava tudo de uma vez: o texto, o HTML (`HTML Format`), a
//!   imagem (`CF_DIB`, que o Paint, o Office e os navegadores colam) e os formatos do app,
//!   registrados pelo nome (o mesmo nome dá o mesmo formato em todos os apps, então outra
//!   instância do Aylis cola o que esta copiou). O contador do Windows diz quando outro app
//!   copiou por cima.
//! - **Linux e macOS:** o `arboard` grava o texto, o HTML com o texto ou a imagem (um só por
//!   vez: com texto, o texto e o HTML; sem, a imagem). Os formatos do app ficam na cópia do
//!   processo (`aylis_core::clipboard`).

use aylis_core::Image;
use aylis_core::clipboard::ClipboardContent;

/// Os pixels RGBA da imagem, decodificando o arquivo (PNG, JPEG...) se for o caso.
fn pixels(image: &Image) -> Option<(u32, u32, Vec<u8>)> {
    if let Some((w, h, px)) = aylis_core::clipboard::rgba(image) {
        return Some((w, h, px.into_owned()));
    }
    match image.source() {
        aylis_core::ImageSource::Encoded(bytes) => crate::decode_rgba(bytes).map(|(px, w, h)| (w, h, px)),
        _ => None,
    }
}

pub(crate) struct SystemClipboard {
    #[cfg(not(windows))]
    board: arboard::Clipboard,
}

impl SystemClipboard {
    pub(crate) fn new() -> Option<Self> {
        #[cfg(windows)]
        return Some(Self {});
        #[cfg(not(windows))]
        return arboard::Clipboard::new().ok().map(|board| Self { board });
    }
}

#[cfg(windows)]
mod win {
    use clipboard_win::{Clipboard, formats, options::NoClear, raw};

    /// Abre a área de transferência (outro app pode estar com ela: tenta algumas vezes). Ela
    /// fecha quando o valor sai de cena.
    pub fn open() -> Option<Clipboard> {
        Clipboard::new_attempts(10).ok()
    }

    /// O formato registrado com esse nome (o mesmo número em todos os apps da sessão).
    pub fn format(name: &str) -> Option<u32> {
        raw::register_format(name).map(|f| f.get())
    }

    pub fn html_format() -> Option<u32> {
        format("HTML Format")
    }

    pub fn get_text() -> Option<String> {
        let _open = open()?;
        let mut out = Vec::new();
        raw::get_string(&mut out).ok()?;
        String::from_utf8(out).ok()
    }

    pub fn get_html() -> Option<String> {
        let html = html_format()?;
        if !raw::is_format_avail(html) {
            return None;
        }
        let _open = open()?;
        let mut out = Vec::new();
        raw::get_html(html, &mut out).ok()?;
        String::from_utf8(out).ok()
    }

    pub fn get(format: u32) -> Option<Vec<u8>> {
        if !raw::is_format_avail(format) {
            return None;
        }
        let _open = open()?;
        let mut out = Vec::new();
        raw::get_vec(format, &mut out).ok()?;
        Some(out)
    }

    pub fn has(format: u32) -> bool {
        raw::is_format_avail(format)
    }

    pub fn set(content: &aylis_core::clipboard::ClipboardContent) {
        let Some(_open) = open() else { return };
        if raw::empty().is_err() {
            return;
        }
        if let Some(text) = &content.text {
            let _ = raw::set_string_with(text, NoClear);
        }
        if let (Some(html), Some(format)) = (&content.html, html_format()) {
            let _ = raw::set_html_with(format, html, NoClear);
        }
        if let Some((w, h, px)) = content.image.as_ref().and_then(super::pixels) {
            let _ = raw::set_without_clear(formats::CF_DIB, &super::to_dib(w, h, &px));
        }
        for (name, bytes) in &content.data {
            if let Some(format) = format(name) {
                let _ = raw::set_without_clear(format, bytes);
            }
        }
    }

    pub fn sequence() -> Option<u64> {
        raw::seq_num().map(|n| u64::from(n.get()))
    }

    pub const DIB: u32 = formats::CF_DIB;
}

impl aylis_core::platform::ClipboardService for SystemClipboard {
    fn get_text(&mut self) -> Option<String> {
        #[cfg(windows)]
        return win::get_text();
        #[cfg(not(windows))]
        return self.board.get_text().ok();
    }

    fn set_text(&mut self, text: &str) {
        self.set_content(&ClipboardContent::text(text));
    }

    fn set_content(&mut self, content: &ClipboardContent) {
        #[cfg(windows)]
        win::set(content);
        #[cfg(not(windows))]
        {
            let text = content.text.clone().unwrap_or_default();
            let image = content.image.as_ref().filter(|_| content.text.is_none() && content.html.is_none()).and_then(pixels);
            let _ = match (&content.html, image) {
                (Some(html), _) => self.board.set_html(html.clone(), Some(text)),
                (None, Some((w, h, px))) => self.board.set_image(arboard::ImageData { width: w as usize, height: h as usize, bytes: px.into() }),
                _ => self.board.set_text(text),
            };
        }
    }

    fn get_html(&mut self) -> Option<String> {
        #[cfg(windows)]
        return win::get_html();
        #[cfg(not(windows))]
        return self.board.get().html().ok();
    }

    fn get_image(&mut self) -> Option<Image> {
        #[cfg(windows)]
        let (w, h, px) = from_dib(&win::get(win::DIB)?)?;
        #[cfg(not(windows))]
        let (w, h, px) = {
            let image = self.board.get_image().ok()?;
            (image.width as u32, image.height as u32, image.bytes.into_owned())
        };
        (px.len() == w as usize * h as usize * 4 && w > 0 && h > 0).then(|| Image::rgba(w, h, px))
    }

    fn get_data(&mut self, format: &str) -> Option<Vec<u8>> {
        #[cfg(windows)]
        return win::get(win::format(format)?);
        #[cfg(not(windows))]
        {
            let _ = format;
            return None;
        }
    }

    fn has(&mut self, format: &str) -> bool {
        #[cfg(windows)]
        return match format {
            aylis_core::clipboard::HTML => win::html_format().is_some_and(win::has),
            aylis_core::clipboard::IMAGE => win::has(win::DIB),
            _ => win::format(format).is_some_and(win::has),
        };
        #[cfg(not(windows))]
        return match format {
            aylis_core::clipboard::HTML => self.get_html().is_some(),
            aylis_core::clipboard::IMAGE => self.board.get_image().is_ok(),
            _ => false,
        };
    }

    fn sequence(&mut self) -> Option<u64> {
        #[cfg(windows)]
        return win::sequence();
        #[cfg(not(windows))]
        return None;
    }
}

/// A imagem como `CF_DIB`: o cabeçalho `BITMAPINFOHEADER` e os pixels BGRA de baixo para
/// cima, 32 bits (o alfa vai junto; quem não entende ignora).
#[cfg_attr(not(any(windows, test)), allow(dead_code))]
pub(crate) fn to_dib(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let mut out = Vec::with_capacity(40 + w * h * 4);
    out.extend(40u32.to_le_bytes());
    out.extend((w as i32).to_le_bytes());
    out.extend((h as i32).to_le_bytes());
    out.extend(1u16.to_le_bytes()); // planos
    out.extend(32u16.to_le_bytes()); // bits por pixel
    out.extend(0u32.to_le_bytes()); // BI_RGB
    out.extend(((w * h * 4) as u32).to_le_bytes());
    out.extend([0u8; 16]); // resolução e paleta
    for row in rgba.chunks_exact(w * 4).rev() {
        for px in row.chunks_exact(4) {
            out.extend([px[2], px[1], px[0], px[3]]);
        }
    }
    out
}

/// A imagem de um `CF_DIB` de 24 ou 32 bits (o que o Windows entrega de qualquer imagem
/// copiada). Sem alfa nenhum (o comum), a imagem é opaca.
#[cfg_attr(not(any(windows, test)), allow(dead_code))]
pub(crate) fn from_dib(dib: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let u32_at = |i: usize| dib.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let header = u32_at(0)? as usize;
    let width = i32::from_le_bytes(dib.get(4..8)?.try_into().ok()?);
    let height = i32::from_le_bytes(dib.get(8..12)?.try_into().ok()?);
    let bits = u16::from_le_bytes(dib.get(14..16)?.try_into().ok()?);
    let compression = u32_at(16)?;
    let colors = u32_at(32)? as usize;
    if width <= 0 || height == 0 || !(bits == 24 || bits == 32) || width > 16_384 || height.unsigned_abs() > 16_384 {
        return None;
    }
    // BI_BITFIELDS com o cabeçalho de 40 bytes: as três máscaras vêm depois dele.
    let masks = if compression == 3 && header == 40 { 12 } else { 0 };
    if !(compression == 0 || compression == 3) {
        return None;
    }
    let (w, h) = (width as usize, height.unsigned_abs() as usize);
    let bytes = (bits / 8) as usize;
    let stride = (w * bytes).div_ceil(4) * 4;
    let start = header + masks + colors * 4;
    let pixels = dib.get(start..start + stride * h)?;
    let mut rgba = vec![0u8; w * h * 4];
    let mut any_alpha = false;
    for y in 0..h {
        let src = &pixels[y * stride..y * stride + w * bytes];
        // De baixo para cima, a não ser com a altura negativa.
        let row = if height > 0 { h - 1 - y } else { y };
        for x in 0..w {
            let p = &src[x * bytes..x * bytes + bytes];
            let a = if bytes == 4 { p[3] } else { 255 };
            any_alpha |= bytes == 4 && a != 0;
            rgba[(row * w + x) * 4..(row * w + x) * 4 + 4].copy_from_slice(&[p[2], p[1], p[0], a]);
        }
    }
    if !any_alpha {
        rgba.chunks_exact_mut(4).for_each(|px| px[3] = 255);
    }
    Some((w as u32, h as u32, rgba))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dib_round_trip_keeps_the_pixels() {
        let rgba = vec![255, 0, 0, 255, 0, 255, 0, 128, 0, 0, 255, 255, 10, 20, 30, 40, 1, 2, 3, 255, 4, 5, 6, 255];
        assert_eq!(from_dib(&to_dib(3, 2, &rgba)), Some((3, 2, rgba)));
    }

    /// Mexe na área de transferência de verdade (devolve o texto que estava):
    /// `cargo test -p aylis-platform-winit --lib clipboard -- --ignored`.
    #[test]
    #[ignore]
    #[cfg(windows)]
    fn the_windows_clipboard_keeps_every_format_at_once() {
        use aylis_core::platform::ClipboardService;
        let mut board = SystemClipboard::new().unwrap();
        let before = board.get_text();
        let image = Image::rgba(2, 1, vec![255, 0, 0, 255, 0, 0, 255, 255]);
        let format = "application/x-aylis-test+json";
        let content = ClipboardContent::text("olá")
            .with_html("<b>olá</b>")
            .with_image(image.clone())
            .with_data(format, b"{\"a\":1}".to_vec());
        let sequence = board.sequence();
        board.set_content(&content);
        // Outra "instância": um serviço novo lê o que foi gravado.
        let mut other = SystemClipboard::new().unwrap();
        let read = (other.get_text(), other.get_html(), other.get_image(), other.get_data(format), other.has(format));
        assert_ne!(other.sequence(), sequence);
        board.set_text(before.as_deref().unwrap_or(""));
        assert_eq!(read.0.as_deref(), Some("olá"));
        assert_eq!(read.1.as_deref(), Some("<b>olá</b>"));
        let read_image = read.2.unwrap();
        assert_eq!(aylis_core::clipboard::rgba(&read_image).map(|(w, h, px)| (w, h, px.into_owned())), aylis_core::clipboard::rgba(&image).map(|(w, h, px)| (w, h, px.into_owned())));
        assert_eq!(read.3.as_deref(), Some(&b"{\"a\":1}"[..]));
        assert!(read.4);
    }

    #[test]
    fn a_dib_of_24_bits_comes_opaque() {
        // 1 × 2, 24 bits, de baixo para cima: a linha de baixo azul, a de cima vermelha.
        let mut dib = Vec::new();
        dib.extend(40u32.to_le_bytes());
        dib.extend(1i32.to_le_bytes());
        dib.extend(2i32.to_le_bytes());
        dib.extend(1u16.to_le_bytes());
        dib.extend(24u16.to_le_bytes());
        dib.extend([0u8; 24]);
        dib.extend([255, 0, 0, 0, 0, 0, 255, 0]);
        assert_eq!(from_dib(&dib), Some((1, 2, vec![255, 0, 0, 255, 0, 0, 255, 255])));
    }
}
