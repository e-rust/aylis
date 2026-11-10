//! Saídas de áudio de verdade: lista, várias saídas no mesmo player, loopback e o mixer.
//! Tudo com o volume das saídas em zero (nada sai no alto-falante). Pula se a máquina não
//! tem saída de áudio.

use std::time::{Duration, Instant};

use aylis_core::{EventArgs, Ui};
use aylis_media::capture::{AudioInput, HAS_LOOPBACK, LOOPBACK_PREFIX, input_devices};
use aylis_media::*;

/// WAV PCM 16 bits mono com um tom de 440 Hz.
fn tone(rate: u32, seconds: f32) -> Vec<u8> {
    let frames = (rate as f32 * seconds) as u32;
    let data_len = frames * 2;
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for i in 0..frames {
        let s = ((i as f32 / rate as f32 * 440.0 * std::f32::consts::TAU).sin() * 12000.0) as i16;
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

fn no_outputs() -> bool {
    let list = output_devices();
    println!("saídas: {list:?}");
    if list.is_empty() {
        eprintln!("sem saída de áudio aqui");
    }
    list.is_empty()
}

/// A saída padrão duas vezes, muda, a segunda com 100 ms de atraso.
const TWO_SILENT_OUTPUTS: &str = ";volume=0|;volume=0;delay=100";

#[test]
fn a_player_plays_on_two_outputs_on_the_clock_and_feeds_a_tap() {
    if no_outputs() {
        return;
    }
    let source = SymphoniaSource::open(tone(22_050, 0.6), Some("wav")).unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let on_end: Box<dyn Fn() + Send> = Box::new(move || {
        let _ = tx.send(());
    });
    let player = AudioPlayer::open_on(Box::new(move || Ok(Box::new(source) as Box<dyn AudioSource>)), Some(on_end), parse_outputs(TWO_SILENT_OUTPUTS)).unwrap();
    let tap = player.tap();
    let started = Instant::now();
    player.play();
    rx.recv_timeout(Duration::from_secs(5)).expect("on_end");
    let took = started.elapsed().as_secs_f64();
    // O fim chega quando a primeira saída tocou tudo (0,6 s mais a fila).
    assert!((0.5..2.0).contains(&took), "terminou em {took:.2} s");
    assert!(!player.is_playing());
    // O ponto de escuta recebeu o som (48 kHz estéreo), com o tom.
    let frames = tap.available();
    assert!(frames > 20_000, "{frames} quadros no ponto de escuta");
    let mut samples = Vec::new();
    tap.take(frames, &mut samples);
    let peak = samples.iter().fold(0f32, |p, s| p.max(s.abs()));
    assert!((0.3..0.4).contains(&peak), "pico {peak}");
}

#[test]
fn the_input_list_has_the_loopback_of_each_output() {
    if no_outputs() || !HAS_LOOPBACK {
        return;
    }
    let inputs = input_devices();
    println!("entradas: {inputs:?}");
    let loopbacks: Vec<_> = inputs.iter().filter(|i| i.starts_with(LOOPBACK_PREFIX)).collect();
    assert_eq!(loopbacks.len(), output_devices().len());
    let default = output_devices().into_iter().find(|o| o.is_default).expect("saída padrão");
    let input = AudioInput::open(&format!("{LOOPBACK_PREFIX}{}", default.name)).expect("loopback da saída padrão");
    let (rate, channels) = input.format();
    assert!(rate >= 8000 && channels >= 1, "{rate} Hz, {channels} canais");
    let tap = input.tap();
    std::thread::sleep(Duration::from_millis(300));
    // O loopback só entrega quando alguma coisa toca; o ponto de escuta existe e não trava.
    println!("loopback: {rate} Hz, {channels} canais, {} quadros", tap.available());
}

#[test]
fn audio_output_sound_and_mixer_as_components() {
    if no_outputs() {
        return;
    }
    let path = std::env::temp_dir().join(format!("aylis_mix_{}.wav", std::process::id()));
    std::fs::write(&path, tone(44_100, 1.5)).unwrap();
    let mut ui = Ui::new();
    register(&mut ui);
    let root = ui.add_root(aylis_controls::Form::new("f", 200.0, 100.0));

    // A lista de saídas.
    let outputs = ui.add(AudioOutput::default(), root);
    let names = ui.get_prop(outputs, "devices").unwrap().to_str().unwrap().to_string();
    assert!(!names.is_empty(), "devices");
    let default = ui.get_prop(outputs, "default_device").unwrap().to_str().unwrap().to_string();
    assert!(names.split(';').any(|n| n == default), "{default} em {names}");
    assert_eq!(AudioOutput::devices(&ui, outputs).len(), names.split(';').count());

    // Um Sound toca por cima de si mesmo, até a polifonia.
    let sound = ui.add(Sound::default(), root);
    ui.set_prop(sound, "file", path.to_string_lossy().to_string()).unwrap();
    ui.set_prop(sound, "output", ";volume=0").unwrap();
    ui.set_prop(sound, "polyphony", 2i64).unwrap();
    for _ in 0..3 {
        Sound::play(&mut ui, sound).unwrap();
    }
    assert!(Sound::is_playing(&ui, sound));
    Sound::stop(&mut ui, sound);
    assert!(!Sound::is_playing(&ui, sound));

    // O mixer grava o MediaPlayer (mudo nas saídas) num WAV.
    let player = ui.add(MediaPlayer::new(path.to_string_lossy()), root);
    ui.set_name(player, "mpTom").unwrap();
    ui.set_prop(player, "output", ";volume=0").unwrap();
    let mixer = ui.add(AudioMixer::default(), root);
    let errors = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let e = errors.clone();
    ui.on(mixer, "on_error", move |_, _, args| {
        if let EventArgs::Text(t) = args {
            e.borrow_mut().push(t.clone());
        }
    });
    ui.set_prop(mixer, "sources", "mpTom;volume=0.5;pan=-1").unwrap();
    ui.set_prop(mixer, "keep_samples", true).unwrap();
    ui.set_prop(mixer, "active", true).unwrap();
    ui.process_posted();
    assert!(errors.borrow().is_empty(), "{:?}", errors.borrow());
    AudioMixer::record(&mut ui, mixer).unwrap();
    MediaPlayer::play(&mut ui, player).unwrap();
    std::thread::sleep(Duration::from_millis(800));
    assert!(AudioMixer::level(&ui, mixer) > 0.05, "nível {}", AudioMixer::level(&ui, mixer));
    let out = std::env::temp_dir().join(format!("aylis_mix_out_{}.wav", std::process::id()));
    let seconds = AudioMixer::stop_recording(&ui, mixer, &out).unwrap();
    assert!((0.5..1.5).contains(&seconds), "gravou {seconds} s");
    // Panorâmica toda à esquerda: o canal direito fica em silêncio; volume pela metade.
    let samples = AudioMixer::take_samples(&ui, mixer);
    let left = samples.iter().step_by(2).fold(0f32, |p, s| p.max(s.abs()));
    let right = samples.iter().skip(1).step_by(2).fold(0f32, |p, s| p.max(s.abs()));
    assert!((0.15..0.2).contains(&left), "esquerda {left}");
    assert_eq!(right, 0.0);
    // Mudar só o volume não religa (as mesmas fontes).
    ui.set_prop(mixer, "sources", "mpTom;mute").unwrap();
    ui.process_posted();
    std::thread::sleep(Duration::from_millis(150));
    let _ = AudioMixer::take_samples(&ui, mixer);
    std::thread::sleep(Duration::from_millis(150));
    let muted = AudioMixer::take_samples(&ui, mixer);
    assert!(!muted.is_empty() && muted.iter().all(|s| *s == 0.0), "mudo");
    // Uma fonte que não existe avisa.
    ui.set_prop(mixer, "sources", "naoExiste").unwrap();
    ui.process_posted();
    assert_eq!(errors.borrow().len(), 1, "{:?}", errors.borrow());
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&out);
}
