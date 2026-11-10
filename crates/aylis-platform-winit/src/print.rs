//! Imprimir as páginas gravadas ([`aylis_core::pages`]). No Windows, o diálogo de impressão
//! do sistema e o desenho pelo GDI na impressora escolhida (páginas, cópias e intervalo como
//! o usuário pediu). Nos outros sistemas, o PDF numa pasta temporária, aberto no visualizador
//! padrão (que imprime).

use aylis_core::pages::PageSet;
use aylis_core::platform::PrintService;

/// A impressora do sistema.
pub(crate) struct SystemPrinter;

impl PrintService for SystemPrinter {
    fn print(&mut self, title: &str, pages: PageSet, done: Box<dyn FnOnce(Result<bool, String>) + Send>) {
        let title = title.to_string();
        // O diálogo é modal e a impressão demora: numa thread, sem travar a interface.
        std::thread::spawn(move || done(print_pages(&title, &pages)));
    }
}

#[cfg(not(windows))]
fn print_pages(title: &str, pages: &PageSet) -> Result<bool, String> {
    let name: String = title.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
    let path = std::env::temp_dir().join(format!("{}.pdf", if name.is_empty() { "relatorio".into() } else { name }));
    std::fs::write(&path, pages.to_pdf()).map_err(|e| format!("{}: {e}", path.display()))?;
    let opener = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
    std::process::Command::new(opener).arg(&path).spawn().map_err(|e| format!("{opener}: {e}"))?;
    Ok(true)
}

#[cfg(windows)]
fn print_pages(title: &str, pages: &PageSet) -> Result<bool, String> {
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::Storage::Xps::{DOCINFOW, EndDoc, EndPage, StartDocW, StartPage};
    use windows::Win32::UI::Controls::Dialogs::*;
    use windows::core::PCWSTR;

    if pages.pages.is_empty() {
        return Ok(false);
    }
    let count = pages.pages.len() as u16;
    let mut dialog = PRINTDLGW {
        lStructSize: std::mem::size_of::<PRINTDLGW>() as u32,
        Flags: PD_RETURNDC | PD_USEDEVMODECOPIESANDCOLLATE | PD_NOSELECTION,
        nFromPage: 1,
        nToPage: count,
        nMinPage: 1,
        nMaxPage: count,
        nCopies: 1,
        ..Default::default()
    };
    // SAFETY: a estrutura está preenchida e vive até o fim da chamada.
    if !unsafe { PrintDlgW(&mut dialog) }.as_bool() {
        return Ok(false);
    }
    let hdc = dialog.hDC;
    if hdc.is_invalid() {
        return Err("a impressora não abriu".into());
    }
    let (first, last) = if dialog.Flags.contains(PD_PAGENUMS) { (dialog.nFromPage.max(1) as usize - 1, (dialog.nToPage as usize).min(pages.pages.len())) } else { (0, pages.pages.len()) };
    let name: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    let info = DOCINFOW { cbSize: std::mem::size_of::<DOCINFOW>() as i32, lpszDocName: PCWSTR(name.as_ptr()), ..Default::default() };
    // SAFETY: o DC é o da impressora escolhida; cada página entre StartPage e EndPage.
    let result = unsafe {
        if StartDocW(hdc, &info) <= 0 {
            Err("a impressora recusou o documento".to_string())
        } else {
            // Pontos (1/72") → pixels da impressora, a partir da borda física do papel.
            let dpi = (GetDeviceCaps(Some(hdc), LOGPIXELSX) as f32, GetDeviceCaps(Some(hdc), LOGPIXELSY) as f32);
            let offset = (GetDeviceCaps(Some(hdc), PHYSICALOFFSETX) as f32, GetDeviceCaps(Some(hdc), PHYSICALOFFSETY) as f32);
            let device = Device { hdc, sx: dpi.0 / 72.0, sy: dpi.1 / 72.0, ox: -offset.0, oy: -offset.1 };
            for page in &pages.pages[first..last.max(first)] {
                StartPage(hdc);
                device.draw(page);
                EndPage(hdc);
            }
            EndDoc(hdc);
            Ok(true)
        }
    };
    // SAFETY: o DC veio do diálogo e não é mais usado.
    unsafe {
        let _ = DeleteDC(hdc);
    }
    result
}

/// O DC da impressora e a conversão de pontos para pixels dela.
#[cfg(windows)]
struct Device {
    hdc: windows::Win32::Graphics::Gdi::HDC,
    sx: f32,
    sy: f32,
    ox: f32,
    oy: f32,
}

#[cfg(windows)]
impl Device {
    fn x(&self, v: f32) -> i32 {
        (v * self.sx + self.ox).round() as i32
    }

    fn y(&self, v: f32) -> i32 {
        (v * self.sy + self.oy).round() as i32
    }

    fn color(c: aylis_core::Color) -> windows::Win32::Foundation::COLORREF {
        // Sem transparência no papel: a cor misturada com o branco.
        let a = c.a as f32 / 255.0;
        let mix = |v: u8| ((v as f32) * a + 255.0 * (1.0 - a)).round() as u32;
        windows::Win32::Foundation::COLORREF(mix(c.r) | (mix(c.g) << 8) | (mix(c.b) << 16))
    }

    /// Desenha a página no DC.
    ///
    /// # Safety
    /// O DC precisa estar entre `StartPage` e `EndPage`.
    unsafe fn draw(&self, page: &aylis_core::pages::Page) {
        use aylis_core::pages::Op;
        use windows::Win32::Foundation::{POINT, RECT};
        use windows::Win32::Graphics::Gdi::*;
        let hdc = self.hdc;
        let rect = |r: aylis_core::RectF| (self.x(r.x), self.y(r.y), self.x(r.right()), self.y(r.bottom()));
        let pen_width = |w: f32| ((w * self.sx).round() as i32).max(1);
        unsafe {
            SetBkMode(hdc, TRANSPARENT);
            for op in &page.ops {
                match op {
                    Op::FillRect { rect: r, radius, color } => {
                        let brush = CreateSolidBrush(Self::color(*color));
                        let old_brush = SelectObject(hdc, brush.into());
                        let old_pen = SelectObject(hdc, GetStockObject(NULL_PEN));
                        let (l, t, rr, b) = rect(*r);
                        let d = (radius * 2.0 * self.sx) as i32;
                        let _ = RoundRect(hdc, l, t, rr + 1, b + 1, d, d);
                        SelectObject(hdc, old_pen);
                        SelectObject(hdc, old_brush);
                        let _ = DeleteObject(brush.into());
                    }
                    Op::StrokeRect { rect: r, radius, color, width, dash } => {
                        let pen = CreatePen(if dash.is_empty() { PS_SOLID } else { PS_DASH }, pen_width(*width), Self::color(*color));
                        let old_pen = SelectObject(hdc, pen.into());
                        let old_brush = SelectObject(hdc, GetStockObject(NULL_BRUSH));
                        let (l, t, rr, b) = rect(*r);
                        let d = (radius * 2.0 * self.sx) as i32;
                        let _ = RoundRect(hdc, l, t, rr, b, d, d);
                        SelectObject(hdc, old_brush);
                        SelectObject(hdc, old_pen);
                        let _ = DeleteObject(pen.into());
                    }
                    Op::FillEllipse { rect: r, color } => {
                        let brush = CreateSolidBrush(Self::color(*color));
                        let old_brush = SelectObject(hdc, brush.into());
                        let old_pen = SelectObject(hdc, GetStockObject(NULL_PEN));
                        let (l, t, rr, b) = rect(*r);
                        let _ = Ellipse(hdc, l, t, rr + 1, b + 1);
                        SelectObject(hdc, old_pen);
                        SelectObject(hdc, old_brush);
                        let _ = DeleteObject(brush.into());
                    }
                    Op::StrokeEllipse { rect: r, color, width } => {
                        let pen = CreatePen(PS_SOLID, pen_width(*width), Self::color(*color));
                        let old_pen = SelectObject(hdc, pen.into());
                        let old_brush = SelectObject(hdc, GetStockObject(NULL_BRUSH));
                        let (l, t, rr, b) = rect(*r);
                        let _ = Ellipse(hdc, l, t, rr, b);
                        SelectObject(hdc, old_brush);
                        SelectObject(hdc, old_pen);
                        let _ = DeleteObject(pen.into());
                    }
                    Op::Line { from, to, color, width, dash } => {
                        let pen = CreatePen(if dash.is_empty() { PS_SOLID } else { PS_DASH }, pen_width(*width), Self::color(*color));
                        let old = SelectObject(hdc, pen.into());
                        let _ = MoveToEx(hdc, self.x(from.x), self.y(from.y), None);
                        let _ = LineTo(hdc, self.x(to.x), self.y(to.y));
                        SelectObject(hdc, old);
                        let _ = DeleteObject(pen.into());
                    }
                    Op::FillPath { contours, color } => {
                        let points: Vec<POINT> = contours.iter().flat_map(|(pts, _)| pts.iter().map(|p| POINT { x: self.x(p.x), y: self.y(p.y) })).collect();
                        let sizes: Vec<i32> = contours.iter().map(|(pts, _)| pts.len() as i32).filter(|n| *n > 0).collect();
                        if points.is_empty() {
                            continue;
                        }
                        let brush = CreateSolidBrush(Self::color(*color));
                        let old_brush = SelectObject(hdc, brush.into());
                        let old_pen = SelectObject(hdc, GetStockObject(NULL_PEN));
                        let _ = PolyPolygon(hdc, points.as_ptr(), &sizes);
                        SelectObject(hdc, old_pen);
                        SelectObject(hdc, old_brush);
                        let _ = DeleteObject(brush.into());
                    }
                    Op::StrokePath { contours, color, width, dash } => {
                        let pen = CreatePen(if dash.is_empty() { PS_SOLID } else { PS_DASH }, pen_width(*width), Self::color(*color));
                        let old = SelectObject(hdc, pen.into());
                        for (pts, closed) in contours {
                            let mut line: Vec<POINT> = pts.iter().map(|p| POINT { x: self.x(p.x), y: self.y(p.y) }).collect();
                            if *closed {
                                if let Some(first) = line.first().copied() {
                                    line.push(first);
                                }
                            }
                            if line.len() > 1 {
                                let _ = Polyline(hdc, &line);
                            }
                        }
                        SelectObject(hdc, old);
                        let _ = DeleteObject(pen.into());
                    }
                    Op::Text { text, rect: r, style } => {
                        let family = if style.font.family.is_empty() { "Segoe UI".to_string() } else { style.font.family.clone() };
                        let face: Vec<u16> = family.encode_utf16().chain(std::iter::once(0)).collect();
                        let weight = match style.font.weight {
                            aylis_core::FontWeight::Thin => 100,
                            aylis_core::FontWeight::Light => 300,
                            aylis_core::FontWeight::Normal => 400,
                            aylis_core::FontWeight::Medium => 500,
                            aylis_core::FontWeight::SemiBold => 600,
                            aylis_core::FontWeight::Bold => 700,
                            aylis_core::FontWeight::Black => 900,
                        };
                        let font = CreateFontW(
                            -((style.font.size * self.sy).round() as i32),
                            0,
                            0,
                            0,
                            weight,
                            style.font.italic as u32,
                            style.font.underline as u32,
                            style.font.strikeout as u32,
                            DEFAULT_CHARSET,
                            OUT_DEFAULT_PRECIS,
                            CLIP_DEFAULT_PRECIS,
                            ANTIALIASED_QUALITY,
                            0,
                            windows::core::PCWSTR(face.as_ptr()),
                        );
                        let old = SelectObject(hdc, font.into());
                        SetTextColor(hdc, Self::color(style.color));
                        let (l, t, rr, b) = rect(*r);
                        let mut area = RECT { left: l, top: t, right: rr, bottom: b };
                        let mut flags = DT_NOPREFIX
                            | match style.h_align {
                                aylis_core::TextAlign::Center => DT_CENTER,
                                aylis_core::TextAlign::Trailing => DT_RIGHT,
                                _ => DT_LEFT,
                            };
                        if style.wrap {
                            flags |= DT_WORDBREAK;
                        } else {
                            flags |= DT_SINGLELINE
                                | match style.v_align {
                                    aylis_core::TextAlign::Leading => DT_TOP,
                                    aylis_core::TextAlign::Trailing => DT_BOTTOM,
                                    _ => DT_VCENTER,
                                };
                        }
                        let mut wide: Vec<u16> = text.encode_utf16().collect();
                        DrawTextW(hdc, &mut wide, &mut area, flags);
                        SelectObject(hdc, old);
                        let _ = DeleteObject(font.into());
                    }
                    Op::Image { image, dest, .. } => {
                        let (w, h, bgra) = match image.source() {
                            aylis_core::ImageSource::Rgba { width, height, pixels } => (*width, *height, pixels.chunks_exact(4).flat_map(|p| over_white([p[2], p[1], p[0], p[3]])).collect::<Vec<u8>>()),
                            aylis_core::ImageSource::Bgra { width, height, pixels } => (*width, *height, pixels.chunks_exact(4).flat_map(|p| over_white([p[0], p[1], p[2], p[3]])).collect::<Vec<u8>>()),
                            aylis_core::ImageSource::Encoded(_) => continue,
                        };
                        let info = BITMAPINFO {
                            bmiHeader: BITMAPINFOHEADER {
                                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                                biWidth: w as i32,
                                // Negativo: as linhas de cima para baixo.
                                biHeight: -(h as i32),
                                biPlanes: 1,
                                biBitCount: 32,
                                biCompression: BI_RGB.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        };
                        let (l, t, rr, b) = rect(*dest);
                        StretchDIBits(hdc, l, t, rr - l, b - t, 0, 0, w as i32, h as i32, Some(bgra.as_ptr() as *const _), &info, DIB_RGB_COLORS, SRCCOPY);
                    }
                    Op::Clip(r) => {
                        let (l, t, rr, b) = rect(*r);
                        IntersectClipRect(hdc, l, t, rr, b);
                    }
                    Op::Save => {
                        SaveDC(hdc);
                    }
                    Op::Restore => {
                        let _ = RestoreDC(hdc, -1);
                    }
                }
            }
        }
    }
}

/// Um pixel BGRA (alfa comum) sobre o branco do papel.
#[cfg(windows)]
fn over_white(p: [u8; 4]) -> [u8; 4] {
    let a = p[3] as u32;
    let mix = |v: u8| ((v as u32 * a + 255 * (255 - a)) / 255) as u8;
    [mix(p[0]), mix(p[1]), mix(p[2]), 255]
}
