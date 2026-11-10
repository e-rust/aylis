//! Imagens com dimensões absurdas pelo WIC: recusadas antes de reservar memória.
#![cfg(windows)]

use aylis_d2d::{decode_rgba, encode_png};

#[test]
fn an_image_bomb_is_refused_and_a_normal_image_decodes() {
    let png = encode_png(&[255; 3 * 2 * 4], 3, 2).unwrap();
    let (pixels, w, h) = decode_rgba(&png).unwrap();
    assert_eq!((w, h, pixels.len()), (3, 2, 24));
    // O mesmo PNG com o cabeçalho dizendo 40.000 × 40.000 (o CRC do IHDR não importa para
    // o WIC ler o tamanho; se ele recusar o arquivo, melhor ainda).
    let mut bomb = png.clone();
    bomb[16..20].copy_from_slice(&40_000u32.to_be_bytes());
    bomb[20..24].copy_from_slice(&40_000u32.to_be_bytes());
    assert!(decode_rgba(&bomb).is_none());
    // Tamanho que não confere com os pixels (e que estourava o u32 na conta).
    assert!(encode_png(&[0; 16], 70_000, 70_000).is_none());
}
