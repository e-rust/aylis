use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::Form;
use aylis_core::*;
use aylis_media::*;

/// WAV PCM 16 bits com um tom de `freq` Hz.
fn wav(rate: u32, channels: u16, seconds: f32, freq: f32) -> Vec<u8> {
    let frames = (rate as f32 * seconds) as u32;
    let data_len = frames * channels as u32 * 2;
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * channels as u32 * 2).to_le_bytes());
    out.extend_from_slice(&(channels * 2).to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for i in 0..frames {
        let s = ((i as f32 / rate as f32 * freq * std::f32::consts::TAU).sin() * 12000.0) as i16;
        for _ in 0..channels {
            out.extend_from_slice(&s.to_le_bytes());
        }
    }
    out
}

/// Espera a abertura em segundo plano terminar (o `on_ready` vem pela fila da tela).
fn wait_ready(ui: &mut Ui, player: ComponentId) {
    for _ in 0..500 {
        ui.process_posted();
        if VideoPlayer::is_ready(ui, player) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[test]
fn a_file_on_disk_is_read_while_it_plays() {
    let path = std::env::temp_dir().join(format!("aylis_audio_{}.wav", std::process::id()));
    std::fs::write(&path, wav(22050, 1, 0.5, 440.0)).unwrap();
    let mut src = SymphoniaSource::open_file(&path).unwrap();
    assert_eq!((src.sample_rate(), src.channels()), (22050, 1));
    let mut samples = Vec::new();
    while src.read(&mut samples) {}
    assert_eq!(samples.len(), 11025);
    drop(src);
    let _ = std::fs::remove_file(&path);
    assert!(SymphoniaSource::open_file(&path).is_err());
}

#[test]
fn decodes_a_wav_file() {
    let mut src = SymphoniaSource::open(wav(22050, 2, 0.5, 440.0), Some("wav")).unwrap();
    assert_eq!((src.sample_rate(), src.channels()), (22050, 2));
    assert!((src.duration().unwrap() - 0.5).abs() < 0.01);
    let mut samples = Vec::new();
    while src.read(&mut samples) {}
    assert_eq!(samples.len(), 22050);
    let peak = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    assert!((0.3..0.4).contains(&peak), "pico {peak}");
    // buscar e ler de novo
    src.seek(0.25);
    let mut again = Vec::new();
    while src.read(&mut again) {}
    assert!((again.len() as i64 - 11025).abs() < 2048, "{} amostras depois da busca", again.len());
}

// ---------------------------------------------------------------------------
// Um decodificador próprio (camada 2) registrado por extensão (camada 3)

struct Ramp {
    frames: u32,
    next: u32,
}

impl VideoDecoder for Ramp {
    fn info(&self) -> VideoInfo {
        VideoInfo { width: 4, height: 2, duration: Some(self.frames as f64 / 10.0), fps: Some(10.0) }
    }
    fn seek(&mut self, seconds: f64) {
        self.next = (seconds * 10.0) as u32;
    }
    fn next_frame(&mut self) -> Option<Frame> {
        if self.next >= self.frames {
            return None;
        }
        let i = self.next;
        self.next += 1;
        Some(Frame::rgba(4, 2, vec![i as u8; 4 * 2 * 4], i as f64 / 10.0))
    }
}

struct RampFactory;

impl VideoDecoderFactory for RampFactory {
    fn name(&self) -> &str {
        "rampa"
    }
    fn accepts(&self, extension: &str) -> bool {
        extension == "rampa"
    }
    fn open(&self, _: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
        Ok(Box::new(Ramp { frames: 10, next: 0 }))
    }
}

/// Canvas que guarda a última imagem desenhada.
#[derive(Default)]
struct Probe {
    last: Option<Image>,
}

impl Canvas for Probe {
    fn save(&mut self) {}
    fn restore(&mut self) {}
    fn translate(&mut self, _: f32, _: f32) {}
    fn scale(&mut self, _: f32, _: f32) {}
    fn rotate(&mut self, _: f32) {}
    fn clip_rect(&mut self, _: RectF) {}
    fn save_layer(&mut self, _: f32) {}
    fn fill_rect(&mut self, _: RectF, _: f32, _: &Brush) {}
    fn stroke_rect(&mut self, _: RectF, _: f32, _: &Stroke) {}
    fn fill_ellipse(&mut self, _: RectF, _: &Brush) {}
    fn stroke_ellipse(&mut self, _: RectF, _: &Stroke) {}
    fn draw_line(&mut self, _: PointF, _: PointF, _: &Stroke) {}
    fn draw_text(&mut self, _: &str, _: RectF, _: &TextStyle) {}
    fn measure_text(&mut self, _: &str, _: &TextStyle) -> SizeF {
        SizeF::new(0.0, 0.0)
    }
    fn draw_image(&mut self, image: &Image, _: RectF, _: RectF, _: f32) {
        self.last = Some(image.clone());
    }
}

/// Valor (o primeiro byte) do quadro mostrado.
fn shown(ui: &mut Ui, form: ComponentId) -> Option<u8> {
    let mut probe = Probe::default();
    ui.request_repaint();
    ui.paint(form, &mut probe);
    probe.last.and_then(|i| match i.source() {
        ImageSource::Rgba { pixels, .. } => pixels.first().copied(),
        _ => None,
    })
}

/// Espera a thread de vídeo encher a fila.
fn settle() {
    std::thread::sleep(std::time::Duration::from_millis(150));
}

#[test]
fn a_registered_decoder_plays_in_the_video_player_on_its_clock() {
    register_decoder(RampFactory);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("v", 200.0, 120.0));
    let player = ui.add(VideoPlayer::new("teste.rampa"), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 200.0, 120.0);
    let ended = Rc::new(RefCell::new(false));
    let e = ended.clone();
    ui.on(player, "on_end", move |_, _, _| *e.borrow_mut() = true);

    ui.set_time(10.0);
    VideoPlayer::play(&mut ui, player).unwrap();
    wait_ready(&mut ui, player);
    settle();
    assert_eq!(shown(&mut ui, form), Some(0), "o primeiro quadro");
    ui.set_time(10.35);
    assert_eq!(shown(&mut ui, form), Some(3), "0,35 s depois: o quadro de 0,3 s");
    assert_eq!(VideoPlayer::video_size(&ui, player), Some((4, 2)));

    // pausar segura o quadro; continuar retoma de onde parou
    VideoPlayer::pause(&mut ui, player);
    ui.set_time(20.0);
    settle();
    assert_eq!(shown(&mut ui, form), Some(3));
    VideoPlayer::play(&mut ui, player).unwrap();
    wait_ready(&mut ui, player);
    ui.set_time(20.25);
    settle();
    assert_eq!(shown(&mut ui, form), Some(5));

    // buscar
    VideoPlayer::seek(&mut ui, player, 0.8);
    settle();
    assert_eq!(shown(&mut ui, form), Some(8));

    // fim: on_end chega pela fila da interface
    ui.set_time(30.0);
    settle();
    shown(&mut ui, form);
    settle();
    shown(&mut ui, form);
    ui.process_posted();
    assert!(*ended.borrow(), "on_end");
    assert!(!VideoPlayer::is_playing(&ui, player));
    // No fim, a posição fica no fim, e não volta ao último `play` (W04 do Tabernáculo).
    let end = VideoPlayer::position(&ui, player);
    assert!(end >= 0.9, "a posição no fim: {end}");
}

/// W12 e W13: sem `show_video`, o player não pinta, mas o quadro e o relógio seguem como API, e
/// cada quadro novo avisa em `on_frame`.
#[test]
fn the_player_is_a_frame_source_when_it_does_not_show_video() {
    register_decoder(RampFactory);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("v", 200.0, 120.0));
    let player = ui.add(VideoPlayer::new("teste.rampa"), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 200.0, 120.0);
    ui.set_prop(player, "show_video", false).unwrap();
    ui.set_prop(player, "frames_ahead", 3).unwrap();
    let frames = Rc::new(RefCell::new(0));
    let f = frames.clone();
    ui.on(player, "on_frame", move |_, _, _| *f.borrow_mut() += 1);
    ui.set_time(10.0);
    VideoPlayer::play(&mut ui, player).unwrap();
    wait_ready(&mut ui, player);
    settle();
    assert_eq!(shown(&mut ui, form), None, "não pinta o quadro");
    ui.process_posted();
    assert!(*frames.borrow() >= 1, "on_frame");
    assert!(VideoPlayer::current_frame(&ui, player).is_some());
    let (_, playing, rate) = VideoPlayer::clock_state(&ui, player).unwrap();
    assert!(playing && rate == 1.0);
    assert!(VideoPlayer::queued_frames(&ui, player) <= 3, "a fila respeita o `frames_ahead`");
}

#[test]
fn frames_pushed_from_another_thread_are_shown() {
    let mut ui = Ui::new();
    let form = ui.add_root(Form::new("v", 200.0, 120.0));
    let player = ui.add(VideoPlayer::default(), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 200.0, 120.0);
    let sink = VideoPlayer::sink(&ui, player).unwrap();
    std::thread::spawn(move || {
        for i in 1..=3u8 {
            sink.push_frame(Frame::rgba(2, 2, vec![i * 10; 16], 0.0));
        }
    })
    .join()
    .unwrap();
    assert_eq!(ui.process_posted(), 3, "cada quadro pede uma repintura");
    assert_eq!(shown(&mut ui, form), Some(30), "mostra o último");
}

#[cfg(windows)]
#[test]
fn media_foundation_writes_and_reads_an_mp4() {
    let path = std::env::temp_dir().join(format!("aylis_media_{}.mp4", std::process::id()));
    // 20 quadros 160x96 a 10 fps: metade vermelha, metade azul (BGRA)
    let (w, h) = (160u32, 96u32);
    let frames = (0..20).map(|i| {
        let color: [u8; 4] = if i < 10 { [0, 0, 255, 255] } else { [255, 0, 0, 255] };
        color.repeat((w * h) as usize)
    });
    mf::write_mp4(&path, w, h, 10, frames).unwrap();
    assert!(std::fs::metadata(&path).unwrap().len() > 500);

    let mut decoder = open_video(&MediaSource::File(path.clone())).unwrap();
    let first = decoder.next_frame().expect("primeiro quadro");
    let info = decoder.info();
    assert_eq!((info.width, info.height), (160, 96));
    assert!((info.duration.unwrap() - 2.0).abs() < 0.2, "duração {:?}", info.duration);
    let pixel = |f: &Frame| match f.image.source() {
        ImageSource::Bgra { pixels, width, .. } => {
            let i = ((48 * *width + 80) * 4) as usize;
            [pixels[i], pixels[i + 1], pixels[i + 2]]
        }
        _ => panic!("esperado BGRA"),
    };
    let [b, g, r] = pixel(&first);
    assert!(r > 200 && g < 60 && b < 60, "vermelho: {:?}", (r, g, b));
    let mut count = 1;
    let mut last = first;
    while let Some(f) = decoder.next_frame() {
        count += 1;
        last = f;
    }
    assert_eq!(count, 20);
    let [b, _, r] = pixel(&last);
    assert!(b > 200 && r < 60, "azul no fim");
    // buscar para o meio
    decoder.seek(1.5);
    let f = decoder.next_frame().unwrap();
    assert!((1.3..=1.6).contains(&f.time), "depois da busca: {}", f.time);
    drop(decoder);
    let _ = std::fs::remove_file(path);
}

#[test]
fn plays_audio_on_the_real_output_if_there_is_one() {
    let source = SymphoniaSource::open(wav(44100, 1, 0.4, 660.0), Some("wav")).unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let player = match AudioPlayer::start(Box::new(source), Some(Box::new(move || tx.send(()).unwrap()))) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("sem saída de áudio ({e}): teste ignorado");
            return;
        }
    };
    player.set_volume(0.05); // baixinho
    player.play();
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(player.is_playing());
    assert!(player.position() > 0.05, "posição {}", player.position());
    rx.recv_timeout(std::time::Duration::from_secs(3)).expect("on_end");
    assert!(!player.is_playing());
}

#[test]
fn the_video_player_plays_music_files_without_a_picture() {
    // Uma "música" de verdade no disco (o player do usuário dizia "o arquivo não tem vídeo").
    let path = std::env::temp_dir().join(format!("aylis_musica_{}.wav", std::process::id()));
    std::fs::write(&path, wav(44100, 2, 0.4, 523.0)).unwrap();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("m", 200.0, 120.0));
    let player = ui.add(VideoPlayer::new(path.to_string_lossy()), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 200.0, 120.0);
    ui.set_prop(player, "volume", 0.05).unwrap(); // baixinho
    let ended = Rc::new(RefCell::new(false));
    let e = ended.clone();
    ui.on(player, "on_end", move |_, _, _| *e.borrow_mut() = true);
    let errors = Rc::new(RefCell::new(Vec::<String>::new()));
    let er = errors.clone();
    ui.on(player, "on_error", move |_, _, args| er.borrow_mut().push(args.text().unwrap_or_default().to_string()));

    let played = VideoPlayer::play(&mut ui, player);
    wait_ready(&mut ui, player);
    // A falha da abertura vem pelo on_error.
    let played = played.and_then(|_| match errors.borrow().first() {
        Some(e) => Err(e.clone()),
        None => Ok(()),
    });
    let _ = std::fs::remove_file(&path);
    if let Err(e) = &played {
        // Sem placa de som na máquina não há como tocar.
        if e.contains("saída") || e.contains("output") || e.contains("dispositivo") {
            eprintln!("sem saída de áudio ({e}): teste ignorado");
            return;
        }
    }
    played.unwrap();
    assert!(errors.borrow().is_empty(), "{:?}", errors.borrow());
    assert!(VideoPlayer::is_audio_only(&ui, player));
    assert!((VideoPlayer::duration(&ui, player).unwrap() - 0.4).abs() < 0.02);
    assert!(VideoPlayer::is_playing(&ui, player));
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(VideoPlayer::position(&ui, player) > 0.05, "a posição anda com o som");
    // Até o fim: on_end e parado.
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        ui.process_posted();
        if *ended.borrow() {
            break;
        }
    }
    assert!(*ended.borrow(), "on_end no fim da música");
    assert!(!VideoPlayer::is_playing(&ui, player));
}

#[test]
fn rate_speeds_up_the_clock() {
    register_decoder(RampFactory);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("v", 200.0, 120.0));
    let player = ui.add(VideoPlayer::new("teste.rampa"), form);
    ui.control_mut(player).unwrap().set_rect(0.0, 0.0, 200.0, 120.0);
    ui.set_time(10.0);
    VideoPlayer::play(&mut ui, player).unwrap();
    wait_ready(&mut ui, player);
    ui.set_prop(player, "rate", 2.0).unwrap();
    settle();
    assert_eq!(shown(&mut ui, form), Some(0));
    ui.set_time(10.21);
    assert_eq!(shown(&mut ui, form), Some(4), "0,21 s em 2x: o quadro de 0,4 s");
    assert!((VideoPlayer::position(&ui, player) - 0.42).abs() < 1e-6);
    // De volta a 1x: continua de onde estava.
    ui.set_prop(player, "rate", 1.0).unwrap();
    ui.set_time(10.31);
    assert!((VideoPlayer::position(&ui, player) - 0.52).abs() < 1e-6);
    assert!(VideoPlayer::playback_quality(&ui, player).shown >= 2);
}
