//! O som por endereço (o relatório do Tabernáculo, W11), servido por um servidor HTTP
//! local do próprio teste (sem rede de fora): lido aos poucos com `Range`, e a busca pede
//! a partir do byte certo.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

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

/// Um servidor HTTP em 127.0.0.1 que serve `body` em qualquer caminho, com `Range`
/// (`ranges`) ou sem (responde 200 sempre). Devolve o endereço e os `Range` pedidos.
fn serve(body: Vec<u8>, content_type: &'static str, ranges: bool) -> (String, Arc<Mutex<Vec<Option<u64>>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let asked = Arc::new(Mutex::new(Vec::new()));
    let log = asked.clone();
    let body = Arc::new(body);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let (body, log) = (body.clone(), log.clone());
            std::thread::spawn(move || {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut from = None;
                let mut head = false;
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        return;
                    }
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    head |= line.starts_with("HEAD ");
                    let lower = line.to_ascii_lowercase();
                    if let Some(r) = lower.strip_prefix("range: bytes=") {
                        from = r.split('-').next().and_then(|s| s.parse::<u64>().ok());
                    }
                }
                log.lock().unwrap().push(from);
                let total = body.len() as u64;
                let start = if ranges { from.unwrap_or(0) } else { 0 };
                let response = if ranges && from.is_some() {
                    if start >= total {
                        let _ = write!(stream, "HTTP/1.1 416 Range Not Satisfiable\r\nContent-Range: bytes */{total}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
                        return;
                    }
                    format!("HTTP/1.1 206 Partial Content\r\nContent-Type: {content_type}\r\nAccept-Ranges: bytes\r\nContent-Range: bytes {start}-{}/{total}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", total - 1, total - start)
                } else {
                    format!("HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {total}\r\nConnection: close\r\n\r\n")
                };
                if stream.write_all(response.as_bytes()).is_err() || head {
                    return;
                }
                // Em pedaços, como na rede (o cliente pode fechar no meio: a busca).
                for chunk in body[start as usize..].chunks(16 * 1024) {
                    if stream.write_all(chunk).is_err() {
                        return;
                    }
                }
            });
        }
    });
    (format!("http://127.0.0.1:{port}"), asked)
}

fn read_all(source: &mut dyn AudioSource) -> usize {
    let mut samples = Vec::new();
    while source.read(&mut samples) {}
    samples.len()
}

#[test]
fn sound_by_address_is_read_with_ranges_and_seeks() {
    // 2 s em mono a 22.050 Hz: 88 KB.
    let (base, asked) = serve(wav(22050, 1, 2.0, 440.0), "audio/wav", true);
    let mut source = open_audio_location(&format!("{base}/midia/tom.wav?token=abc")).unwrap();
    assert_eq!((source.sample_rate(), source.channels()), (22050, 1));
    assert!((source.duration().unwrap() - 2.0).abs() < 0.01);
    assert_eq!(read_all(source.as_mut()), 44100);
    // A busca para trás pede de novo a partir do meio, não do começo.
    source.seek(1.0);
    let rest = read_all(source.as_mut());
    assert!((rest as i64 - 22050).abs() < 2048, "{rest} amostras depois da busca");
    let asked = asked.lock().unwrap().clone();
    assert_eq!(asked.first(), Some(&Some(0)), "o primeiro pedido já é com Range: {asked:?}");
    assert!(asked.iter().any(|r| r.is_some_and(|b| b > 40_000)), "a busca pediu do meio: {asked:?}");
}

#[test]
fn a_server_without_ranges_still_plays_and_seeks() {
    let (base, asked) = serve(wav(22050, 2, 1.0, 330.0), "audio/wav", false);
    let mut source = open_audio_location(&format!("{base}/tom.wav")).unwrap();
    assert_eq!(read_all(source.as_mut()), 44100);
    source.seek(0.5);
    let rest = read_all(source.as_mut());
    assert!((rest as i64 - 22050).abs() < 4096, "{rest} amostras depois da busca");
    assert!(asked.lock().unwrap().len() >= 2, "voltou a baixar do começo");
}

#[test]
fn a_missing_address_is_an_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut stream = stream;
            let mut line = String::new();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            while reader.read_line(&mut line).unwrap_or(0) > 2 {
                line.clear();
            }
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        }
    });
    let e = open_audio_location(&format!("http://127.0.0.1:{port}/nada.mp3")).err().expect("erro");
    assert!(e.contains("404") || e.to_lowercase().contains("not found") || e.contains("Media Foundation"), "{e}");
}

/// O Media Foundation também abre o endereço (o caminho sem a feature `http`, e o WMA).
#[cfg(windows)]
#[test]
fn media_foundation_opens_sound_by_address() {
    let (base, _) = serve(wav(44100, 2, 1.0, 440.0), "audio/wav", true);
    let mut source = match mf::open_audio(&format!("{base}/tom.wav")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(source.channels(), 2);
    assert!((source.duration().unwrap_or(0.0) - 1.0).abs() < 0.05, "duração {:?}", source.duration());
    let samples = read_all(source.as_mut());
    let frames = samples as f64 / 2.0 / source.sample_rate() as f64;
    assert!((frames - 1.0).abs() < 0.05, "{frames:.3} s");
}

#[test]
fn the_video_player_plays_sound_by_address() {
    use aylis_controls::Form;
    use aylis_core::*;
    let (base, _) = serve(wav(44100, 2, 0.4, 523.0), "audio/wav", true);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("m", 200.0, 120.0));
    let player = ui.add(VideoPlayer::new(format!("{base}/musica.wav")), form);
    ui.set_prop(player, "volume", 0.05).unwrap();
    let errors = Arc::new(Mutex::new(Vec::<String>::new()));
    let er = errors.clone();
    ui.on(player, "on_error", move |_, _, args| er.lock().unwrap().push(args.text().unwrap_or_default().to_string()));
    VideoPlayer::play(&mut ui, player).unwrap();
    for _ in 0..500 {
        ui.process_posted();
        if VideoPlayer::is_ready(&ui, player) || !errors.lock().unwrap().is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    if let Some(e) = errors.lock().unwrap().first() {
        if e.contains("saída") || e.contains("output") || e.contains("dispositivo") {
            eprintln!("sem saída de áudio ({e}): teste ignorado");
            return;
        }
        panic!("{e}");
    }
    assert!(VideoPlayer::is_audio_only(&ui, player));
    assert!((VideoPlayer::duration(&ui, player).unwrap() - 0.4).abs() < 0.02);
}
