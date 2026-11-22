//! O `Image` com arquivos de fora do projeto e bytes na memória, e a cor dominante (o relatório
//! do player de música, 21/11: a capa da música não aparecia, e o fundo devia seguir a cor dela).

use aylis_controls::{Form, ImageView};
use aylis_core::*;

/// Um PNG de `w × h`: a metade de cima `top`, a de baixo `bottom`.
fn png(w: u32, h: u32, top: [u8; 3], bottom: [u8; 3]) -> Vec<u8> {
    let img = ::image::RgbaImage::from_fn(w, h, |_, y| {
        let c = if y < h / 4 { top } else { bottom };
        ::image::Rgba([c[0], c[1], c[2], 255])
    });
    let mut out = Vec::new();
    ::image::DynamicImage::ImageRgba8(img).write_to(&mut std::io::Cursor::new(&mut out), ::image::ImageFormat::Png).unwrap();
    out
}

#[test]
fn base64_goes_and_comes_back() {
    for bytes in [&b""[..], b"f", b"fo", b"foo", b"foob", &[0u8, 255, 128, 7, 9]] {
        assert_eq!(image::decode_base64(&image::encode_base64(bytes)).as_deref(), Some(bytes));
    }
    assert_eq!(image::decode_base64("Zm9v\nYg"), Some(b"foob".to_vec()), "sem = e com quebra");
    assert_eq!(image::decode_base64("Zm9v*"), None);
}

#[test]
fn the_dominant_color_is_the_most_common_ignoring_black_and_white() {
    // 3/4 azul, 1/4 vermelho, com uma borda preta e branca que não conta.
    let mut px = Vec::new();
    for i in 0..400 {
        let c = match i % 20 {
            0 => [0, 0, 0],
            1 => [255, 255, 255],
            n if n < 6 => [200, 30, 30],
            _ => [30, 60, 200],
        };
        px.extend_from_slice(&[c[0], c[1], c[2], 255]);
    }
    let c = image::dominant_color(20, 20, &px).unwrap();
    assert!(c.b > 150 && c.r < 80, "{c:?}");
    // Só preto: a média dele (não some).
    assert_eq!(image::dominant_color(1, 1, &[0, 0, 0, 255]), Some(Color::rgb(0, 0, 0)));
    assert_eq!(image::dominant_color(0, 0, &[]), None);
}

#[test]
fn the_image_shows_bytes_and_disk_files_and_tells_the_color() {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("x", 200.0, 200.0));
    let img = ui.add(ImageView::default(), form);
    let bytes = png(16, 16, [250, 250, 250], [20, 160, 60]);
    // Rust: os bytes; protocolo e outras linguagens: base64 pelo invoke.
    ImageView::set_image_bytes(&mut ui, img, bytes.clone());
    let c = ImageView::dominant_color(&ui, img).unwrap();
    assert!(c.g > 120 && c.r < 60, "{c:?}");
    ui.invoke(img, "set_image_bytes", &[PropValue::String(String::new())]).unwrap();
    assert!(ImageView::dominant_color(&ui, img).is_none(), "vazio volta ao `image` (sem nada)");
    ui.invoke(img, "set_image_bytes", &[PropValue::String(image::encode_base64(&bytes))]).unwrap();
    assert!(matches!(ui.invoke(img, "dominant_color", &[]), Ok(PropValue::Color(_))));
    // Um arquivo do disco.
    let path = std::env::temp_dir().join(format!("aylis_image_sources_{}.png", std::process::id()));
    std::fs::write(&path, png(8, 8, [200, 30, 30], [200, 30, 30])).unwrap();
    ImageView::set_image_file(&mut ui, img, &path.to_string_lossy()).unwrap();
    assert!(ImageView::dominant_color(&ui, img).unwrap().r > 150);
    assert!(ImageView::set_image_file(&mut ui, img, "/nao/existe.png").is_err());
    let _ = std::fs::remove_file(&path);
}
