//! O indicador (ponto ou número) desenhado sobre um ícone RGBA: o da bandeja e o do botão na
//! barra de tarefas.

/// Algarismos 3x5 para o indicador (o ícone tem 16 a 32 pixels).
const DIGITS: [u16; 10] = [0x7B6F, 0x2C97, 0x73E7, 0x73CF, 0x5BC9, 0x79CF, 0x79EF, 0x7249, 0x7BEF, 0x7BCF];

/// Desenha o indicador: um círculo no canto de cima à direita (`size` = a fração do ícone
/// que ele ocupa; 1 = o ícone inteiro) e, se houver, o número em branco.
pub(crate) fn draw(px: &mut [u8], w: u32, h: u32, badge: &str, color: aylis_core::Color, size: f32) {
    if badge.is_empty() {
        return;
    }
    let r = (w.min(h) as f32 * size / 2.0).max(4.0);
    let (cx, cy) = (w as f32 - r - 0.5, r - 0.5);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f32 - cx, y as f32 - cy);
            if dx * dx + dy * dy <= r * r {
                let i = ((y * w + x) * 4) as usize;
                px[i..i + 4].copy_from_slice(&[color.r, color.g, color.b, 255]);
            }
        }
    }
    let digits: Vec<u32> = badge.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.is_empty() || badge == "•" {
        return;
    }
    // Uma ou duas casas ("9+" vira 9), em branco, escaladas ao tamanho do círculo.
    let digits = &digits[..digits.len().min(2)];
    let scale = ((r * 1.2) / 5.0).floor().max(1.0) as u32;
    let total_w = digits.len() as u32 * 4 * scale - scale;
    let x0 = (cx - total_w as f32 / 2.0).round() as i32;
    let y0 = (cy - 2.5 * scale as f32).round() as i32;
    for (k, d) in digits.iter().enumerate() {
        let bits = DIGITS[*d as usize];
        for row in 0..5 {
            for col in 0..3 {
                if bits >> (14 - (row * 3 + col)) & 1 == 1 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let x = x0 + (k as u32 * 4 * scale + col * scale + sx) as i32;
                            let y = y0 + (row * scale + sy) as i32;
                            if x >= 0 && y >= 0 && (x as u32) < w && (y as u32) < h {
                                let i = ((y as u32 * w + x as u32) * 4) as usize;
                                px[i..i + 4].copy_from_slice(&[255, 255, 255, 255]);
                            }
                        }
                    }
                }
            }
        }
    }
}
