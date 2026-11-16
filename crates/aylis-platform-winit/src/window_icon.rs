//! O ícone da janela (barra de título e barra de tarefas) e o da bandeja, tirados da mesma
//! imagem do ícone do app, recortados como o `.ico` (o `aylis_tools::icons`): o quadrado do meio,
//! sem distorcer. Antes ia a imagem inteira, e o sistema esticava uma de 1024 × 768 num quadrado
//! de 16 px (relatado em 15/11: o desenho saía achatado e minúsculo). Reduzida a no máximo
//! 256 px (o maior tamanho do `.ico`) pela média de cada área, que não perde as linhas finas.

/// O maior lado do ícone entregue ao sistema.
pub(crate) const MAX_SIDE: u32 = 256;

/// O quadrado do meio de `pixels` (RGBA, `width` × `height`), reduzido a no máximo
/// [`MAX_SIDE`]: (os pixels, o lado).
pub(crate) fn square(pixels: &[u8], width: u32, height: u32) -> (Vec<u8>, u32) {
    let side = width.min(height);
    if side == 0 || pixels.len() < (width as usize * height as usize * 4) {
        return (Vec::new(), 0);
    }
    let (x0, y0) = ((width - side) / 2, (height - side) / 2);
    let out_side = side.min(MAX_SIDE);
    let mut out = Vec::with_capacity((out_side * out_side * 4) as usize);
    for oy in 0..out_side {
        // A área da origem que cai neste pixel (pelo menos um pixel).
        let sy0 = y0 + oy * side / out_side;
        let sy1 = (y0 + (oy + 1) * side / out_side).max(sy0 + 1);
        for ox in 0..out_side {
            let sx0 = x0 + ox * side / out_side;
            let sx1 = (x0 + (ox + 1) * side / out_side).max(sx0 + 1);
            // A média pesada pelo alfa: a borda transparente não escurece a cor.
            let (mut r, mut g, mut b, mut a, mut n) = (0u64, 0u64, 0u64, 0u64, 0u64);
            for sy in sy0..sy1 {
                for sx in sx0..sx1 {
                    let i = ((sy * width + sx) * 4) as usize;
                    let alpha = pixels[i + 3] as u64;
                    r += pixels[i] as u64 * alpha;
                    g += pixels[i + 1] as u64 * alpha;
                    b += pixels[i + 2] as u64 * alpha;
                    a += alpha;
                    n += 1;
                }
            }
            let color = |c: u64| if a == 0 { 0 } else { (c / a) as u8 };
            out.extend_from_slice(&[color(r), color(g), color(b), (a / n) as u8]);
        }
    }
    (out, out_side)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Uma imagem de 1024 × 768 (a margem transparente e um quadrado azul no meio) vira um
    /// quadrado de 256, sem achatar: o azul continua quadrado.
    #[test]
    fn a_wide_image_becomes_the_middle_square() {
        let (w, h) = (1024u32, 768u32);
        let mut px = vec![0u8; (w * h * 4) as usize];
        for y in 234..534 {
            for x in 362..662 {
                let i = ((y * w + x) * 4) as usize;
                px[i..i + 4].copy_from_slice(&[30, 90, 220, 255]);
            }
        }
        let (out, side) = square(&px, w, h);
        assert_eq!(side, 256);
        assert_eq!(out.len(), (256 * 256 * 4) as usize);
        // O azul: 300 px de 768 → 100 px de 256, na largura e na altura.
        let blue = |x: u32, y: u32| out[((y * 256 + x) * 4 + 3) as usize] > 128;
        let wide = (0..256).filter(|&x| blue(x, 128)).count();
        let tall = (0..256).filter(|&y| blue(128, y)).count();
        assert!((99..=101).contains(&wide) && (99..=101).contains(&tall), "largura {wide}, altura {tall}");
        let i = ((128 * 256 + 128) * 4) as usize;
        assert_eq!(&out[i..i + 4], &[30, 90, 220, 255], "a cor do meio fica a mesma");
    }

    #[test]
    fn a_small_square_stays_as_it_is() {
        let px: Vec<u8> = (0..16 * 16).flat_map(|i| [i as u8, 0, 0, 255]).collect();
        assert_eq!(square(&px, 16, 16), (px, 16));
        assert_eq!(square(&[], 0, 0).1, 0);
    }
}
