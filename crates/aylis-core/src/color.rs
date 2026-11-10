//! Cor RGBA de 8 bits por canal.

use std::fmt;
use std::str::FromStr;

use crate::error::Error;

/// Cor RGBA. Em texto é `#RRGGBB`, `#RRGGBBAA` ou `#RGB`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Lê `#RRGGBB`, `#RRGGBBAA` ou `#RGB`.
    pub fn from_hex(s: &str) -> Option<Color> {
        let h = s.trim().strip_prefix('#')?;
        let pair = |i: usize| -> Option<u8> { u8::from_str_radix(h.get(i..i + 2)?, 16).ok() };
        let single =
            |i: usize| -> Option<u8> { u8::from_str_radix(h.get(i..i + 1)?, 16).ok().map(|v| v * 17) };
        match h.len() {
            3 => Some(Color::rgb(single(0)?, single(1)?, single(2)?)),
            6 => Some(Color::rgb(pair(0)?, pair(2)?, pair(4)?)),
            8 => Some(Color::rgba(pair(0)?, pair(2)?, pair(4)?, pair(6)?)),
            _ => None,
        }
    }

    /// `#RRGGBB` quando opaca, senão `#RRGGBBAA`.
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    /// A mesma cor com o alfa multiplicado por `opacity` (0..1).
    pub fn with_opacity(self, opacity: f32) -> Color {
        let a = (self.a as f32 * opacity.clamp(0.0, 1.0)).round() as u8;
        Color { a, ..self }
    }

    /// Mistura linear: `t = 0` devolve `self`, `t = 1` devolve `other`.
    ///
    /// Com alfa pré-multiplicado: cada cor pesa pela própria opacidade. Assim o `transparent`
    /// (preto com alfa 0) só faz a outra cor aparecer aos poucos, sem escurecer no caminho (o
    /// hover de um botão transparente piscava cinza-escuro antes da cor clara do hover). Entre
    /// cores opacas, é a mistura de sempre.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let (aa, ab) = (self.a as f32 / 255.0, other.a as f32 / 255.0);
        let alpha = aa + (ab - aa) * t;
        if alpha <= 0.0 {
            return Color::rgba(0, 0, 0, 0);
        }
        let mix = |a: u8, b: u8| ((a as f32 * aa + (b as f32 * ab - a as f32 * aa) * t) / alpha).round().clamp(0.0, 255.0) as u8;
        Color::rgba(mix(self.r, other.r), mix(self.g, other.g), mix(self.b, other.b), (alpha * 255.0).round() as u8)
    }

    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::from_hex(s).ok_or_else(|| Error::invalid(format!("cor inválida: '{s}'")))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}
