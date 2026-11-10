//! O ícone do app em cada plataforma, gerado de uma imagem só (PNG de 256 px ou mais, ou um
//! SVG, que não tem limite): Windows `.ico` (16 a 256), Android (os 5 tamanhos e o
//! adaptativo: a imagem na área segura sobre uma cor de fundo), iOS (`AppIcon.appiconset`),
//! macOS `.icns` e web (favicon e os 192/512 do PWA).
//!
//! Abaixo de 1024 px o iOS, o PWA e o adaptativo do Android saem ampliados ([`Icon::warnings`]).

use std::io::Cursor;
use std::path::Path;

use image::{ImageFormat, Rgba, RgbaImage, imageops::FilterType};

/// A imagem de origem do ícone.
pub enum Icon {
    Raster(RgbaImage),
    Svg(Box<resvg::usvg::Tree>),
}

/// O menor lado aceito para um PNG.
pub const MIN_SIZE: u32 = 256;
/// Daqui para baixo, o iOS, o PWA e o adaptativo saem ampliados.
pub const IDEAL_SIZE: u32 = 1024;

/// Os tamanhos do Android (pasta, lado do ícone comum, lado do adaptativo).
pub const ANDROID: [(&str, u32, u32); 5] = [("mdpi", 48, 108), ("hdpi", 72, 162), ("xhdpi", 96, 216), ("xxhdpi", 144, 324), ("xxxhdpi", 192, 432)];

impl Icon {
    /// Lê o PNG/JPEG ou o SVG.
    pub fn open(path: &Path) -> Result<Self, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("ícone {}: {e}", path.display()))?;
        let svg = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("svg"));
        Self::from_bytes(&bytes, svg)
    }

    pub fn from_bytes(bytes: &[u8], svg: bool) -> Result<Self, String> {
        if svg {
            let tree = resvg::usvg::Tree::from_data(bytes, &resvg::usvg::Options::default()).map_err(|e| format!("SVG do ícone: {e}"))?;
            return Ok(Icon::Svg(Box::new(tree)));
        }
        let img = image::load_from_memory(bytes).map_err(|e| format!("imagem do ícone: {e}"))?.to_rgba8();
        if img.width().min(img.height()) < MIN_SIZE {
            return Err(format!("o ícone tem {}×{} px; use {MIN_SIZE} px ou mais (ou um SVG)", img.width(), img.height()));
        }
        Ok(Icon::Raster(img))
    }

    /// O lado da imagem de origem (o SVG: sem limite).
    pub fn side(&self) -> Option<u32> {
        match self {
            Icon::Raster(img) => Some(img.width().min(img.height())),
            Icon::Svg(_) => None,
        }
    }

    /// O que a IDE avisa (PNG menor que 1024).
    pub fn warnings(&self) -> Vec<String> {
        match self.side() {
            Some(s) if s < IDEAL_SIZE => vec![format!("O ícone tem {s} px: o iOS (1024), o PWA (512) e o adaptativo do Android saem ampliados. Um PNG de {IDEAL_SIZE} px ou um SVG fica nítido em todos.")],
            _ => Vec::new(),
        }
    }

    /// A imagem num quadrado de `size` px (centralizada, sem distorcer).
    pub fn render(&self, size: u32) -> RgbaImage {
        match self {
            Icon::Raster(img) => {
                let side = img.width().min(img.height());
                let x = (img.width() - side) / 2;
                let y = (img.height() - side) / 2;
                let square = image::imageops::crop_imm(img, x, y, side, side).to_image();
                image::imageops::resize(&square, size, size, FilterType::Lanczos3)
            }
            Icon::Svg(tree) => {
                let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size).expect("tamanho do ícone");
                let s = tree.size();
                let scale = size as f32 / s.width().max(s.height());
                let dx = (size as f32 - s.width() * scale) / 2.0;
                let dy = (size as f32 - s.height() * scale) / 2.0;
                let t = resvg::tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, dx, dy);
                resvg::render(tree, t, &mut pixmap.as_mut());
                // O tiny-skia guarda pré-multiplicado.
                let mut img = RgbaImage::new(size, size);
                for (i, p) in pixmap.pixels().iter().enumerate() {
                    let c = p.demultiply();
                    img.put_pixel(i as u32 % size, i as u32 / size, Rgba([c.red(), c.green(), c.blue(), c.alpha()]));
                }
                img
            }
        }
    }

    /// A cor sugerida para o fundo do adaptativo: a média da borda da imagem (os pixels
    /// transparentes não contam; tudo transparente: branco).
    pub fn border_color(&self) -> [u8; 3] {
        let img = self.render(128);
        let (mut r, mut g, mut b, mut n) = (0u64, 0u64, 0u64, 0u64);
        let side = img.width();
        for i in 0..side {
            for (x, y) in [(i, 0), (i, side - 1), (0, i), (side - 1, i)] {
                let p = img.get_pixel(x, y);
                if p[3] > 128 {
                    (r, g, b, n) = (r + p[0] as u64, g + p[1] as u64, b + p[2] as u64, n + 1);
                }
            }
        }
        if n == 0 { [255, 255, 255] } else { [(r / n) as u8, (g / n) as u8, (b / n) as u8] }
    }

    /// O adaptativo do Android (só a frente): a imagem nos 66% do meio (a área segura) de um
    /// quadrado de `size` transparente.
    pub fn adaptive_foreground(&self, size: u32) -> RgbaImage {
        let inner = (size as f32 * 0.66).round() as u32;
        let icon = self.render(inner);
        let mut out = RgbaImage::new(size, size);
        let off = ((size - inner) / 2) as i64;
        image::imageops::overlay(&mut out, &icon, off, off);
        out
    }
}

/// PNG em bytes.
pub fn png(img: &RgbaImage) -> Vec<u8> {
    let mut out = Vec::new();
    img.write_to(&mut Cursor::new(&mut out), ImageFormat::Png).expect("PNG em memória");
    out
}

/// Um `.ico` com as imagens em PNG (aceito do Vista em diante).
pub fn ico(icon: &Icon, sizes: &[u32]) -> Vec<u8> {
    ico_of(sizes.iter().map(|&s| (s, png(&icon.render(s)))).collect())
}

/// A maior imagem de um `.ico` com PNG dentro (o do aylis), como ícone de origem.
pub fn from_ico(bytes: &[u8]) -> Result<Icon, String> {
    let count = bytes.get(4..6).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]) as usize);
    let best = (0..count)
        .filter_map(|i| {
            let e = bytes.get(6 + 16 * i..6 + 16 * (i + 1))?;
            let side = if e[0] == 0 { 256 } else { e[0] as u32 };
            let len = u32::from_le_bytes([e[8], e[9], e[10], e[11]]) as usize;
            let at = u32::from_le_bytes([e[12], e[13], e[14], e[15]]) as usize;
            Some((side, bytes.get(at..at + len)?))
        })
        .filter(|(_, data)| data.starts_with(b"\x89PNG"))
        .max_by_key(|(side, _)| *side)
        .ok_or("o .ico não tem imagem em PNG")?;
    Icon::from_bytes(best.1, false)
}

/// Os arquivos do aylis que abrem na IDE com dois cliques, cada um com o seu ícone: o logo
/// numa folha e uma marca colorida no canto. (extensão, cor da marca, desenho branco da marca
/// num quadro de 256, em volta do centro 180,184.)
pub const FILE_TYPES: [(&str, &str, &str); 4] = [
    // Projeto: uma janela (o form).
    ("aylisproj", "#2563EB", r##"<rect x="146" y="156" width="68" height="56" rx="6" fill="none" stroke="#fff" stroke-width="9"/><rect x="146" y="156" width="68" height="16" fill="#fff"/>"##),
    // Pacote de componentes: uma caixa.
    ("aylispkg", "#D97706", r##"<path d="M180 146 L216 164 L180 182 L144 164 Z" fill="#fff"/><path d="M144 172 L176 188 L176 224 L144 208 Z" fill="#fff" fill-opacity="0.85"/><path d="M216 172 L184 188 L184 224 L216 208 Z" fill="#fff" fill-opacity="0.65"/>"##),
    // Gravação de uso: o botão de gravar.
    ("aylisrec", "#DC2626", r##"<circle cx="180" cy="184" r="32" fill="none" stroke="#fff" stroke-width="9"/><circle cx="180" cy="184" r="15" fill="#fff"/>"##),
    // Relato de problema: uma exclamação.
    ("aylisreport", "#7C3AED", r##"<rect x="171" y="144" width="18" height="50" rx="9" fill="#fff"/><circle cx="180" cy="214" r="10" fill="#fff"/>"##),
];

fn svg(source: &str, size: u32) -> RgbaImage {
    Icon::from_bytes(source.as_bytes(), true).expect("SVG embutido").render(size)
}

/// O ícone do arquivo `ext` (um de [`FILE_TYPES`]) com o logo `logo`; `None` se o tipo não
/// for do aylis.
pub fn file_type_image(logo: &Icon, ext: &str, size: u32) -> Option<RgbaImage> {
    let (_, color, mark) = FILE_TYPES.iter().find(|(e, _, _)| *e == ext)?;
    let page = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path d="M40 10 H166 L218 62 V246 H40 Z" fill="#FFFFFF" stroke="#9CA3AF" stroke-width="8" stroke-linejoin="round"/><path d="M166 10 V62 H218" fill="#E5E7EB" stroke="#9CA3AF" stroke-width="8" stroke-linejoin="round"/></svg>"##;
    let mut out = svg(page, size);
    let side = size * 124 / 256;
    let at = |v: u32| (size * v / 256) as i64;
    image::imageops::overlay(&mut out, &logo.render(side.max(1)), at(52), at(46));
    let badge = format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><circle cx="180" cy="184" r="66" fill="{color}" stroke="#fff" stroke-width="8"/>{mark}</svg>"##);
    image::imageops::overlay(&mut out, &svg(&badge, size), 0, 0);
    Some(out)
}

/// O `.ico` do arquivo `ext` (16 a 256).
pub fn file_type_ico(logo: &Icon, ext: &str) -> Option<Vec<u8>> {
    let images: Option<Vec<(u32, Vec<u8>)>> = [16, 24, 32, 48, 64, 128, 256].iter().map(|&s| file_type_image(logo, ext, s).map(|img| (s, png(&img)))).collect();
    images.map(ico_of)
}

fn ico_of(images: Vec<(u32, Vec<u8>)>) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * images.len() as u32;
    for (size, data) in &images {
        let b = if *size >= 256 { 0 } else { *size as u8 };
        out.extend_from_slice(&[b, b, 0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += data.len() as u32;
    }
    for (_, data) in &images {
        out.extend_from_slice(data);
    }
    out
}

/// Um `.icns` do macOS com os PNG de 128 a 1024.
pub fn icns(icon: &Icon) -> Vec<u8> {
    let entries: Vec<(&[u8; 4], u32)> = vec![(b"ic07", 128), (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024)];
    let mut body = Vec::new();
    for (kind, size) in entries {
        let data = png(&icon.render(size));
        body.extend_from_slice(kind);
        body.extend_from_slice(&((data.len() + 8) as u32).to_be_bytes());
        body.extend_from_slice(&data);
    }
    let mut out = Vec::new();
    out.extend_from_slice(b"icns");
    out.extend_from_slice(&((body.len() + 8) as u32).to_be_bytes());
    out.extend_from_slice(&body);
    out
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

/// O ícone do Android em `res/`: os 5 tamanhos e o adaptativo (frente + cor de fundo).
/// `background`: a cor do fundo (`None`: a da borda da imagem).
pub fn android(icon: &Icon, res: &Path, background: Option<[u8; 3]>) -> Result<(), String> {
    for (dpi, size, adaptive) in ANDROID {
        write(&res.join(format!("mipmap-{dpi}/ic_launcher.png")), &png(&icon.render(size)))?;
        write(&res.join(format!("mipmap-{dpi}/ic_launcher_foreground.png")), &png(&icon.adaptive_foreground(adaptive)))?;
    }
    let [r, g, b] = background.unwrap_or_else(|| icon.border_color());
    write(&res.join("values/ic_launcher_background.xml"), format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<resources>\n    <color name=\"ic_launcher_background\">#{r:02X}{g:02X}{b:02X}</color>\n</resources>\n").as_bytes())?;
    write(
        &res.join("mipmap-anydpi-v26/ic_launcher.xml"),
        b"<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<adaptive-icon xmlns:android=\"http://schemas.android.com/apk/res/android\">\n    <background android:drawable=\"@color/ic_launcher_background\"/>\n    <foreground android:drawable=\"@mipmap/ic_launcher_foreground\"/>\n</adaptive-icon>\n",
    )
}

/// O `AppIcon.appiconset` do iOS (o de 1024, que o Xcode reduz).
pub fn ios(icon: &Icon, assets: &Path) -> Result<(), String> {
    let dir = assets.join("AppIcon.appiconset");
    // O iOS não aceita transparência no ícone: fundo branco.
    let mut img = RgbaImage::from_pixel(1024, 1024, Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut img, &icon.render(1024), 0, 0);
    write(&dir.join("icon-1024.png"), &png(&img))?;
    write(
        &dir.join("Contents.json"),
        br#"{ "images": [ { "filename": "icon-1024.png", "idiom": "universal", "platform": "ios", "size": "1024x1024" } ], "info": { "author": "aylis", "version": 1 } }"#,
    )
}

/// O favicon (`.ico` 16/32/48 e o PNG) e os ícones do PWA (192 e 512), e o trecho do
/// `manifest.webmanifest`.
pub fn web(icon: &Icon, dir: &Path) -> Result<String, String> {
    write(&dir.join("favicon.ico"), &ico(icon, &[16, 32, 48]))?;
    write(&dir.join("icon.png"), &png(&icon.render(256)))?;
    write(&dir.join("icon-192.png"), &png(&icon.render(192)))?;
    write(&dir.join("icon-512.png"), &png(&icon.render(512)))?;
    Ok(r#""icons": [ { "src": "icon-192.png", "sizes": "192x192", "type": "image/png" }, { "src": "icon-512.png", "sizes": "512x512", "type": "image/png" } ]"#.to_string())
}

/// O `.ico` do Windows (16 a 256).
pub fn windows(icon: &Icon, path: &Path) -> Result<(), String> {
    write(path, &ico(icon, &[16, 24, 32, 48, 64, 128, 256]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(side: u32) -> Vec<u8> {
        let b = side / 16;
        let img = RgbaImage::from_fn(side, side, |x, y| if x < b || y < b || x >= side - b || y >= side - b { Rgba([200, 30, 30, 255]) } else { Rgba([255, 255, 255, 255]) });
        png(&img)
    }

    #[test]
    fn small_images_are_refused_and_every_platform_gets_its_sizes() {
        assert!(Icon::from_bytes(&sample(128), false).is_err(), "abaixo de 256");
        let icon = Icon::from_bytes(&sample(512), false).unwrap();
        assert_eq!(icon.warnings().len(), 1, "abaixo de 1024 avisa");
        assert_eq!(icon.border_color(), [200, 30, 30]);
        let dir = std::env::temp_dir().join(format!("aylis-icons-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        android(&icon, &dir.join("res"), None).unwrap();
        let xx = image::open(dir.join("res/mipmap-xxxhdpi/ic_launcher.png")).unwrap();
        assert_eq!((xx.width(), xx.height()), (192, 192));
        let bg = std::fs::read_to_string(dir.join("res/values/ic_launcher_background.xml")).unwrap();
        assert!(bg.contains("#C81E1E"), "{bg}");
        let ico_bytes = ico(&icon, &[16, 32, 256]);
        assert_eq!(&ico_bytes[..6], &[0, 0, 1, 0, 3, 0]);
        let mac = icns(&icon);
        assert_eq!(&mac[..4], b"icns");
        assert_eq!(u32::from_be_bytes(mac[4..8].try_into().unwrap()) as usize, mac.len());
        ios(&icon, &dir.join("Assets.xcassets")).unwrap();
        assert!(web(&icon, &dir.join("web")).unwrap().contains("icon-512.png"));
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="#0000FF"/></svg>"##;
        let s = Icon::from_bytes(svg, true).unwrap();
        assert!(s.warnings().is_empty(), "SVG não tem limite");
        assert_eq!(*s.render(64).get_pixel(32, 32), Rgba([0, 0, 255, 255]));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn each_aylis_file_type_gets_its_own_icon_from_the_ide_logo() {
        let logo = from_ico(include_bytes!("../../aylis-ide/resources/aylis-ide.ico")).unwrap();
        assert_eq!(logo.side(), Some(256));
        let mut badges = Vec::new();
        for (ext, color, _) in FILE_TYPES {
            let img = file_type_image(&logo, ext, 256).unwrap();
            // A folha é branca em cima à esquerda; a marca, da cor do tipo, no canto.
            assert_eq!(*img.get_pixel(60, 30), Rgba([255, 255, 255, 255]), "{ext}");
            let badge = img.get_pixel(130, 184);
            let want = u32::from_str_radix(&color[1..], 16).unwrap().to_be_bytes();
            assert_eq!(&badge.0[..3], &want[1..], "{ext}");
            badges.push(badge.0);
            let bytes = file_type_ico(&logo, ext).unwrap();
            assert_eq!(&bytes[..6], &[0, 0, 1, 0, 7, 0], "{ext}");
            assert_eq!(from_ico(&bytes).unwrap().side(), Some(256));
        }
        badges.dedup();
        assert_eq!(badges.len(), FILE_TYPES.len(), "uma cor por tipo");
        assert!(file_type_image(&logo, "txt", 32).is_none());
    }
}
