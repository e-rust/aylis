//! Imagens com dimensões absurdas: recusadas antes de reservar memória.

use aylis_core::Color;
use aylis_core::image::checked_size;
use aylis_skia::{SkiaRenderer, decode_rgba};

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

/// Um PNG de poucos bytes cujo cabeçalho declara `width × height` (a "bomba de imagem").
fn png_header(width: u32, height: u32) -> Vec<u8> {
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = b"IHDR".to_vec();
    ihdr.extend(width.to_be_bytes());
    ihdr.extend(height.to_be_bytes());
    ihdr.extend([8, 6, 0, 0, 0]);
    png.extend(13u32.to_be_bytes());
    png.extend(&ihdr);
    png.extend(crc32(&ihdr).to_be_bytes());
    png.extend(0u32.to_be_bytes());
    png.extend(b"IEND");
    png.extend(crc32(b"IEND").to_be_bytes());
    png
}

#[test]
fn sizes_are_checked_without_overflow() {
    assert_eq!(checked_size(2, 3), Ok(24));
    assert!(checked_size(16_384, 6_000).is_ok());
    for (w, h) in [(0, 10), (16_385, 1), (16_384, 16_384), (40_000, 40_000), (u32::MAX, u32::MAX)] {
        assert!(checked_size(w, h).is_err(), "{w} × {h}");
    }
}

#[test]
fn an_image_bomb_is_refused_and_a_normal_image_decodes() {
    // 40.000 × 40.000: em u32, `w * h * 4` dava a volta; agora nem chega a reservar.
    assert!(decode_rgba(&png_header(40_000, 40_000)).is_none());
    assert!(decode_rgba(&png_header(20_000, 20_000)).is_none());

    let renderer = SkiaRenderer::new();
    let png = renderer.render_png(3, 2, 1.0, Color::rgb(10, 20, 30), |_| {}).unwrap();
    let (pixels, w, h) = decode_rgba(&png).unwrap();
    assert_eq!((w, h, pixels.len()), (3, 2, 24));
    assert!(renderer.render_png(1_000_000, 1_000, 1.0, Color::BLACK, |_| {}).is_none());
}
