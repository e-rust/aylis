//! A barra de reprodução do Player com uma música de verdade: o tempo anda (o timer põe a
//! posição e a duração na barra) e clicar na barra leva a música para aquele ponto.

use aylis::prelude::*;

/// WAV PCM 16 bits estéreo com um tom.
fn wav(seconds: f32) -> Vec<u8> {
    let rate = 44100u32;
    let frames = (rate as f32 * seconds) as u32;
    let data_len = frames * 4;
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for i in 0..frames {
        let s = ((i as f32 / rate as f32 * 440.0 * std::f32::consts::TAU).sin() * 3000.0) as i16;
        out.extend_from_slice(&s.to_le_bytes());
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

fn text(ui: &Ui, name: &str) -> String {
    match ui.get_prop(ui.find(name).unwrap(), "text").unwrap() {
        PropValue::String(s) => s,
        v => panic!("{v:?}"),
    }
}

fn number(ui: &Ui, name: &str, prop: &str) -> f64 {
    ui.get_prop(ui.find(name).unwrap(), prop).unwrap().to_f64().unwrap()
}

#[test]
fn time_runs_and_clicking_the_bar_seeks_the_song() {
    let path = std::env::temp_dir().join(format!("aylis_player_barra_{}.wav", std::process::id()));
    std::fs::write(&path, wav(8.0)).unwrap();
    let mut app = player::app().unwrap();
    let ui = &mut app.ui;
    let form = app.main_form;
    let _ = ui.show(form);
    ui.layout();
    let video = ui.find("vpVideo").unwrap();
    ui.set_prop(video, "volume", 0.02).unwrap();
    let dlg = ui.find("dlgAbrir").unwrap();
    ui.fire(dlg, "on_select", &EventArgs::Text(path.to_string_lossy().into()));
    if !VideoPlayer::is_playing(ui, video) {
        eprintln!("sem saída de áudio: teste ignorado");
        return;
    }

    // O timer (250 ms) põe a duração e a posição na barra e nos rótulos.
    std::thread::sleep(std::time::Duration::from_millis(600));
    let now = ui.time() + 1.0;
    ui.set_time(now);
    ui.run_timers();
    assert!((number(ui, "sbPosicao", "duration") - 8.0).abs() < 0.05, "duração na barra");
    assert!(number(ui, "sbPosicao", "position") > 0.3, "posição na barra");
    assert_eq!(text(ui, "lblDuracao"), "0:08");

    // Clicar no meio da barra: a música vai para ~4 s.
    let bar = ui.find("sbPosicao").unwrap();
    let r = ui.absolute_rect(bar).unwrap();
    let middle = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
    ui.pointer_down(form, PointerEvent { position: middle, ..Default::default() });
    // O timer do player dispara entre apertar e soltar (era o que fazia o clique falhar).
    std::thread::sleep(std::time::Duration::from_millis(300));
    let now = ui.time() + 0.3;
    ui.set_time(now);
    ui.run_timers();
    ui.pointer_up(form, PointerEvent { position: middle, ..Default::default() });
    let pos = VideoPlayer::position(ui, video);
    let _ = std::fs::remove_file(&path);
    // ~4 s, mais os 0,3 s com o botão apertado e o que já está na fila do áudio; sem a
    // correção ia para onde o timer deixou (~1 s).
    assert!((3.8..5.3).contains(&pos), "a música foi para o meio: {pos}");
}
