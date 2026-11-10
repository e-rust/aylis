//! Microfone de verdade: abre o padrão, grava um segundo e salva em WAV. Pula se a máquina
//! não tem microfone.

use aylis_media::capture::{AudioInput, input_devices};

#[test]
fn records_a_second_from_the_default_microphone() {
    println!("microfones: {:?}", input_devices());
    let input = match AudioInput::open("") {
        Ok(i) => i,
        Err(e) => {
            eprintln!("sem microfone aqui: {e}");
            return;
        }
    };
    let (rate, channels) = input.format();
    assert!(rate >= 8000 && channels >= 1, "{rate} Hz, {channels} canais");
    input.start_recording();
    assert!(input.is_recording());
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let path = std::env::temp_dir().join(format!("aylis_mic_{}.wav", std::process::id()));
    let seconds = input.stop_recording(&path).unwrap();
    assert!((0.7..1.5).contains(&seconds), "gravou {seconds} s");
    let size = std::fs::metadata(&path).unwrap().len();
    assert!(size > 44 + (rate as u64 * channels as u64), "{size} bytes");
    println!("gravou {seconds:.2} s, nível {:.3}", input.level());
    let _ = std::fs::remove_file(path);
}
