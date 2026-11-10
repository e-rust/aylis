//! O decodificador de vídeo do sistema abre o MP4 de teste (`tests/data/sample.mp4`,
//! gerado com `cargo run -p aylis-media --example make_mp4`): 320x240, 30 fps, 4 s; o
//! quadrado do meio é vermelho no primeiro segundo e verde no segundo.
//!
//! No Linux precisa do GStreamer (gstreamer1.0-plugins-base, -good e -libav).

use aylis_core::ImageSource;
use aylis_media::{Frame, MediaSource, open_video};

fn center(f: &Frame) -> [u8; 3] {
    match f.image.source() {
        ImageSource::Rgba { pixels, width, .. } => {
            let i = ((120 * *width + 160) * 4) as usize;
            [pixels[i], pixels[i + 1], pixels[i + 2]]
        }
        ImageSource::Bgra { pixels, width, .. } => {
            let i = ((120 * *width + 160) * 4) as usize;
            [pixels[i + 2], pixels[i + 1], pixels[i]]
        }
        _ => panic!("quadro sem pixels"),
    }
}

#[test]
fn the_system_decoder_plays_the_test_video() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/sample.mp4");
    let mut decoder = match open_video(&MediaSource::File(path)) {
        Ok(d) => d,
        Err(e) if e.contains("não instalado") || e.contains("nenhum decodificador") => {
            eprintln!("sem decodificador do sistema aqui: {e}");
            return;
        }
        Err(e) => panic!("{e}"),
    };
    let first = decoder.next_frame().expect("primeiro quadro");
    let info = decoder.info();
    assert_eq!((info.width, info.height), (320, 240));
    assert!((info.duration.unwrap_or(0.0) - 4.0).abs() < 0.2, "duração {:?}", info.duration);
    let [r, g, b] = center(&first);
    assert!(r > 180 && g < 90 && b < 90, "vermelho: {:?}", (r, g, b));
    // Um segundo depois: verde.
    decoder.seek(1.5);
    let later = decoder.next_frame().expect("quadro depois da busca");
    assert!((later.time - 1.5).abs() < 0.1, "tempo {}", later.time);
    let [r, g, b] = center(&later);
    assert!(g > 140 && r < 90 && b < 110, "verde: {:?}", (r, g, b));
    let mut n = 1;
    while decoder.next_frame().is_some() {
        n += 1;
    }
    assert!((70..=80).contains(&n), "quadros depois de 1,5 s: {n}");
}
