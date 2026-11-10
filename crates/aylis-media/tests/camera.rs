//! Câmera de verdade: lista e lê alguns quadros da primeira. Pula sem câmera.

#[test]
fn the_first_camera_gives_frames() {
    let cams = aylis_media::cameras();
    println!("câmeras: {cams:?}");
    if cams.is_empty() {
        eprintln!("sem câmera aqui");
        return;
    }
    let mut camera = match aylis_media::camera::open_camera("") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("câmera não abriu: {e}");
            return;
        }
    };
    let mut frames = 0;
    let started = std::time::Instant::now();
    while frames < 10 && started.elapsed().as_secs() < 15 {
        match camera.next_frame() {
            Some(f) => {
                frames += 1;
                if frames == 1 {
                    println!("primeiro quadro: {:?} em {:?}", f.image.known_size(), started.elapsed());
                }
            }
            None => {
                eprintln!("a câmera parou: {:?} (ocupada por outro app?)", camera.error());
                return;
            }
        }
    }
    let info = camera.info();
    println!("{} quadros, {}x{}", frames, info.width, info.height);
    assert!(frames > 0 && info.width > 0);
}
