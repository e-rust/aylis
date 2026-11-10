//! Os aparelhos de referência para ver um form como ele fica em cada um: o tamanho da tela
//! em pontos (em pé), os cantos, o recorte da câmera e as faixas cobertas pelo sistema (a
//! barra de status em cima, a barra de gestos embaixo). A IDE desenha a moldura no designer
//! e o app rodando no desktop abre do tamanho do aparelho (`AYLIS_DEVICE`), com a área segura
//! simulada e a mesma barra de status por cima.
//!
//! Os números são os pontos lógicos de cada aparelho (o que o layout enxerga), não pixels.

use crate::{Bounds, Brush, Canvas, Color, FontSpec, FontWeight, RectF, SizeF, Stroke, TextAlign, TextStyle};

/// O tipo de aparelho: celular e tablet giram; a tela de computador mostra o form como uma
/// janela nela.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Phone,
    Tablet,
    Desktop,
}

/// O recorte da câmera no alto da tela.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cutout {
    None,
    /// O entalhe largo (iPhone X a 14).
    Notch,
    /// A ilha (iPhone 14 Pro em diante).
    Island,
    /// O furo no meio (a maioria dos Android).
    Hole,
}

/// O sistema do aparelho: separa os celulares e tablets em Android e iOS no seletor da IDE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceOs {
    Android,
    Ios,
    Desktop,
}

/// Um aparelho de referência.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Device {
    pub name: &'static str,
    pub kind: DeviceKind,
    pub os: DeviceOs,
    /// A tela em pé, em pontos.
    pub width: f32,
    pub height: f32,
    /// O raio dos cantos da tela.
    pub radius: f32,
    /// A barra de status (com o recorte) e a barra de gestos, em pé.
    pub top: f32,
    pub bottom: f32,
    pub cutout: Cutout,
}

const fn iphone(name: &'static str, width: f32, height: f32, radius: f32, top: f32, bottom: f32, cutout: Cutout) -> Device {
    Device { name, kind: DeviceKind::Phone, os: DeviceOs::Ios, width, height, radius, top, bottom, cutout }
}

/// Celular Android: o furo da câmera no meio, barra de status de 32 e gestos de 24.
const fn android(name: &'static str, width: f32, height: f32, radius: f32) -> Device {
    Device { name, kind: DeviceKind::Phone, os: DeviceOs::Android, width, height, radius, top: 32.0, bottom: 24.0, cutout: Cutout::Hole }
}

const fn ipad(name: &'static str, width: f32, height: f32) -> Device {
    Device { name, kind: DeviceKind::Tablet, os: DeviceOs::Ios, width, height, radius: 18.0, top: 24.0, bottom: 20.0, cutout: Cutout::None }
}

const fn android_tablet(name: &'static str, width: f32, height: f32) -> Device {
    Device { name, kind: DeviceKind::Tablet, os: DeviceOs::Android, width, height, radius: 16.0, top: 24.0, bottom: 24.0, cutout: Cutout::None }
}

const fn screen(name: &'static str, width: f32, height: f32) -> Device {
    Device { name, kind: DeviceKind::Desktop, os: DeviceOs::Desktop, width, height, radius: 0.0, top: 0.0, bottom: 0.0, cutout: Cutout::None }
}

/// Os aparelhos, dos celulares às telas de computador (as telas de PC a 100%).
pub const DEVICES: &[Device] = &[
    iphone("iPhone SE", 375.0, 667.0, 0.0, 20.0, 0.0, Cutout::None),
    iphone("iPhone 13 mini", 375.0, 812.0, 44.0, 50.0, 34.0, Cutout::Notch),
    iphone("iPhone 14", 390.0, 844.0, 47.0, 47.0, 34.0, Cutout::Notch),
    iphone("iPhone 15", 393.0, 852.0, 55.0, 59.0, 34.0, Cutout::Island),
    iphone("iPhone 15 Pro Max", 430.0, 932.0, 55.0, 59.0, 34.0, Cutout::Island),
    iphone("iPhone 16", 393.0, 852.0, 55.0, 59.0, 34.0, Cutout::Island),
    iphone("iPhone 16 Pro", 402.0, 874.0, 62.0, 62.0, 34.0, Cutout::Island),
    iphone("iPhone 16 Pro Max", 440.0, 956.0, 62.0, 62.0, 34.0, Cutout::Island),
    android("Galaxy S24", 360.0, 780.0, 36.0),
    android("Galaxy S24 Ultra", 412.0, 891.0, 24.0),
    android("Galaxy A15", 411.0, 891.0, 30.0),
    android("Galaxy A54", 412.0, 915.0, 32.0),
    android("Pixel 8", 412.0, 915.0, 40.0),
    android("Pixel 8 Pro", 448.0, 998.0, 40.0),
    android("Pixel 9", 411.0, 923.0, 40.0),
    android("Moto G84", 393.0, 873.0, 30.0),
    android("Redmi Note 13", 393.0, 873.0, 30.0),
    ipad("iPad", 820.0, 1180.0),
    ipad("iPad mini", 744.0, 1133.0),
    ipad("iPad Air 11", 820.0, 1180.0),
    ipad("iPad Pro 11", 834.0, 1210.0),
    ipad("iPad Pro 13", 1032.0, 1376.0),
    android_tablet("Galaxy Tab S9", 800.0, 1280.0),
    android_tablet("Galaxy Tab S9 Ultra", 924.0, 1480.0),
    android_tablet("Galaxy Tab A9+", 800.0, 1280.0),
    android_tablet("Pixel Tablet", 800.0, 1280.0),
    android_tablet("Xiaomi Pad 6", 900.0, 1440.0),
    screen("HD (1366×768)", 1366.0, 768.0),
    screen("Full HD (1920×1080)", 1920.0, 1080.0),
    screen("2,5K (2560×1440)", 2560.0, 1440.0),
    screen("4K (3840×2160)", 3840.0, 2160.0),
    screen("8K (7680×4320)", 7680.0, 4320.0),
];

/// O aparelho pelo nome (sem diferença de maiúsculas).
pub fn find(name: &str) -> Option<&'static Device> {
    DEVICES.iter().find(|d| d.name.eq_ignore_ascii_case(name.trim()))
}

/// O que vem no `AYLIS_DEVICE` e no designer: `Nome` ou `Nome;deitado`.
pub fn parse(spec: &str) -> Option<(&'static Device, bool)> {
    let (name, landscape) = spec.split_once(';').map_or((spec, false), |(n, rest)| (n, rest.trim() == "deitado" || rest.trim() == "landscape"));
    find(name).map(|d| (d, landscape))
}

impl Device {
    /// Gira (celular e tablet).
    pub fn rotates(&self) -> bool {
        self.kind != DeviceKind::Desktop
    }

    /// A tela, em pé ou deitada.
    pub fn size(&self, landscape: bool) -> SizeF {
        if landscape && self.rotates() { SizeF::new(self.height, self.width) } else { SizeF::new(self.width, self.height) }
    }

    /// As faixas cobertas pelo sistema. Deitado, o recorte vai para o lado (o iPhone reserva
    /// os dois lados) e a barra de status do iPhone some; o Android mantém a de cima.
    pub fn safe_area(&self, landscape: bool) -> Bounds {
        if !landscape || !self.rotates() {
            return Bounds::new(0.0, self.top, 0.0, self.bottom);
        }
        match self.cutout {
            Cutout::Notch | Cutout::Island => Bounds::new(self.top, 0.0, self.top, if self.bottom > 0.0 { 21.0 } else { 0.0 }),
            _ => Bounds::new(0.0, self.top.min(24.0), 0.0, self.bottom),
        }
    }
}

/// Pinta por cima da tela `rect` (já do tamanho do aparelho) o que o sistema desenha: a hora e
/// a bateria na barra de status, o recorte da câmera e a barra de gestos. `dark`: o fundo do
/// app é escuro (os textos ficam claros).
pub fn paint_overlay(canvas: &mut dyn Canvas, device: &Device, landscape: bool, rect: RectF, dark: bool) {
    if device.kind == DeviceKind::Desktop {
        return;
    }
    let ink = if dark { Color::WHITE } else { Color::rgb(0x11, 0x12, 0x16) };
    let safe = device.safe_area(landscape);
    let bar = if landscape && matches!(device.cutout, Cutout::Notch | Cutout::Island) { 24.0 } else { safe.top.max(20.0) };
    let style = TextStyle {
        font: FontSpec { size: 14.0, weight: FontWeight::SemiBold, ..FontSpec::default() },
        color: ink,
        h_align: TextAlign::Center,
        v_align: TextAlign::Center,
        ..TextStyle::default()
    };
    let side = (rect.width * 0.22).min(110.0);
    let status = RectF::new(rect.x + (safe.left.max(12.0)), rect.y, side, bar);
    canvas.draw_text("9:41", status, &style);
    // A bateria: um contorno com a carga dentro e o polo.
    let b = RectF::new(rect.right() - safe.right.max(12.0) - 34.0, rect.y + bar / 2.0 - 6.0, 25.0, 12.0);
    canvas.stroke_rect(b, 3.5, &Stroke::new(ink.with_opacity(0.5), 1.0));
    canvas.fill_rect(b.inflate(-2.0), 2.0, &Brush::Solid(ink));
    canvas.fill_rect(RectF::new(b.right() + 1.5, b.y + 4.0, 2.0, 4.0), 1.0, &Brush::Solid(ink.with_opacity(0.5)));
    // O recorte, no alto em pé ou no lado esquerdo deitado.
    let black = Brush::Solid(Color::BLACK);
    let (cx, cy, along) = if landscape { (rect.x + 14.0, rect.y + rect.height / 2.0, false) } else { (rect.x + rect.width / 2.0, rect.y + 14.0, true) };
    let (long, short) = match device.cutout {
        Cutout::Notch => (rect.width.min(rect.height) * 0.42, 30.0),
        Cutout::Island => (122.0, 36.0),
        Cutout::Hole => (12.0, 12.0),
        Cutout::None => (0.0, 0.0),
    };
    if long > 0.0 {
        let r = if along {
            let top = if device.cutout == Cutout::Notch { rect.y } else { cy - short / 2.0 + 2.0 };
            RectF::new(cx - long / 2.0, top, long, short)
        } else {
            let left = if device.cutout == Cutout::Notch { rect.x } else { cx - short / 2.0 + 2.0 };
            RectF::new(left, cy - long / 2.0, short, long)
        };
        canvas.fill_rect(r, short / 2.0, &black);
    }
    // A barra de gestos.
    if safe.bottom > 0.0 {
        let w = (rect.width * 0.36).min(140.0);
        let y = rect.bottom() - safe.bottom / 2.0 - 2.5;
        canvas.fill_rect(RectF::new(rect.x + (rect.width - w) / 2.0, y, w, 5.0), 2.5, &Brush::Solid(ink.with_opacity(0.85)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devices_rotate_and_move_the_safe_area() {
        let (iphone, landscape) = parse("iPhone 15;deitado").unwrap();
        assert!(landscape);
        assert_eq!(iphone.size(false), SizeF::new(393.0, 852.0));
        assert_eq!(iphone.size(true), SizeF::new(852.0, 393.0));
        assert_eq!(iphone.safe_area(false), Bounds::new(0.0, 59.0, 0.0, 34.0));
        assert_eq!(iphone.safe_area(true), Bounds::new(59.0, 0.0, 59.0, 21.0));
        let pixel = find("pixel 8").unwrap();
        assert_eq!(pixel.safe_area(true), Bounds::new(0.0, 24.0, 0.0, 24.0));
        let hd = find("HD (1366×768)").unwrap();
        assert!(!hd.rotates() && hd.size(true) == SizeF::new(1366.0, 768.0) && hd.safe_area(true) == Bounds::default());
        assert!(parse("Nenhum").is_none());
        // Os nomes não se repetem.
        for (i, d) in DEVICES.iter().enumerate() {
            assert!(DEVICES[i + 1..].iter().all(|o| o.name != d.name), "{} repetido", d.name);
        }
    }
}
