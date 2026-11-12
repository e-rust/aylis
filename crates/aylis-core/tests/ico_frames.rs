//! C41 (relatório do Tabernáculo): do `.ico` com vários tamanhos, o quadro do tamanho pedido.

use aylis_core::image::ico_frame_for_size;

/// Um `.ico` com um quadro por largura; os dados de cada um são a largura repetida.
fn ico(widths: &[u32]) -> Vec<u8> {
    let mut out = vec![0, 0, 1, 0];
    out.extend_from_slice(&(widths.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * widths.len() as u32;
    let mut data = Vec::new();
    for &w in widths {
        let bytes = vec![(w % 256) as u8; 8];
        let side = if w >= 256 { 0 } else { w as u8 };
        out.extend_from_slice(&[side, side, 0, 0, 1, 0, 32, 0]);
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += bytes.len() as u32;
        data.extend_from_slice(&bytes);
    }
    out.extend_from_slice(&data);
    out
}

/// A largura do único quadro do `.ico` devolvido.
fn width_of(one: &[u8]) -> u32 {
    assert_eq!(&one[..6], &[0, 0, 1, 0, 1, 0]);
    assert_eq!(u32::from_le_bytes(one[18..22].try_into().unwrap()), 22, "os dados logo depois da entrada");
    if one[6] == 0 { 256 } else { one[6] as u32 }
}

#[test]
fn the_tray_gets_the_frame_of_its_size() {
    let file = ico(&[16, 32, 48, 256]);
    // 100%: 16; 150%: 24 (o de 32 reduzido, não o de 16 ampliado); 200%: 32; maior que tudo: o maior.
    for (size, want) in [(16, 16), (24, 32), (32, 32), (40, 48), (300, 256)] {
        let one = ico_frame_for_size(&file, size).unwrap();
        assert_eq!(width_of(&one), want, "{size} pixels");
        assert_eq!(&one[22..], &vec![(want % 256) as u8; 8][..]);
    }
    // A ordem do arquivo não importa.
    assert_eq!(width_of(&ico_frame_for_size(&ico(&[48, 16, 32]), 24).unwrap()), 32);
    // Um quadro só, ou outro formato: vale o arquivo como está.
    assert!(ico_frame_for_size(&ico(&[32]), 24).is_none());
    assert!(ico_frame_for_size(b"\x89PNG\r\n\x1a\n", 24).is_none());
    // Truncado: nada, sem pânico.
    let mut broken = ico(&[16, 32]);
    broken.truncate(30);
    assert!(ico_frame_for_size(&broken, 24).is_none());
}
