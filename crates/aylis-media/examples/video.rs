//! Um `VideoPlayer` com cantos arredondados e controles por cima.
//!
//! ```text
//! cargo run -p aylis-media --example video                 gera um MP4 animado e toca
//! cargo run -p aylis-media --example video -- filme.mp4    toca o arquivo
//! cargo run -p aylis-media --example video -- filme.mp4 --muted
//! cargo run -p aylis-media --example video -- filme.mp4 --stats   quadros por segundo no terminal
//! cargo run -p aylis-media --example video -- filme.mp4 --rate 4    em 4x
//! ```

use aylis_controls::{Button, Form, Label, Layout, Panel};
use aylis_core::*;
use aylis_media::{VideoPlayer, register};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let file = match args.iter().find(|a| !a.starts_with("--")) {
        Some(f) => f.clone(),
        None => generated()?,
    };

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    ui.set_theme(Theme::dark());
    let form = ui.add_root(Form::new("aylis — VideoPlayer", 720.0, 480.0));
    ui.set_name(form, "frmVideo")?;
    ui.control_mut(form).unwrap().set_padding(16.0);

    let bar = ui.add(Layout::default(), form);
    ui.control_mut(bar).unwrap().set_align(Align::Bottom).set_height(36.0).set_margins([0.0, 12.0, 0.0, 0.0]);
    let play = ui.add(Button::new("Pausar"), bar);
    ui.control_mut(play).unwrap().set_align(Align::Left).set_width(110.0);
    let stop = ui.add(Button::new("Do começo"), bar);
    ui.control_mut(stop).unwrap().set_align(Align::Left).set_width(110.0).set_margins([8.0, 0.0, 0.0, 0.0]);
    let _ = ui.set_prop(stop, "style", "flat");
    let time = ui.add(Label::new(""), bar);
    ui.control_mut(time).unwrap().set_align(Align::Client).set_margins([12.0, 0.0, 0.0, 0.0]);

    let player = ui.add(VideoPlayer::new(file.as_str()), form);
    ui.control_mut(player).unwrap().set_align(Align::Client);
    ui.set_prop(player, "radius", 16)?;
    ui.set_prop(player, "looping", true)?;
    ui.set_prop(player, "muted", args.iter().any(|a| a == "--muted"))?;
    if let Some(rate) = args.iter().position(|a| a == "--rate").and_then(|i| args.get(i + 1)).and_then(|r| r.parse::<f64>().ok()) {
        ui.set_prop(player, "rate", rate)?;
    }
    // Um selo por cima do vídeo: prova de que o vídeo é pintado como qualquer controle.
    let badge = ui.add(Panel::new(150.0, 32.0), player);
    ui.control_mut(badge).unwrap().set_rect(16.0, 16.0, 150.0, 32.0).set_opacity(0.85);
    let badge_text = ui.add(Label::new("aylis · ao vivo"), badge);
    ui.control_mut(badge_text).unwrap().set_align(Align::Client).set_margins([12.0, 0.0, 0.0, 0.0]);

    ui.on_click(play, move |ui, _| {
        if VideoPlayer::is_playing(ui, player) {
            VideoPlayer::pause(ui, player);
            let _ = ui.set_prop(play, "text", "Tocar");
        } else {
            let _ = VideoPlayer::play(ui, player);
            let _ = ui.set_prop(play, "text", "Pausar");
        }
    });
    ui.on_click(stop, move |ui, _| VideoPlayer::seek(ui, player, 0.0));
    ui.on(player, "on_error", move |ui, _, args| {
        if let EventArgs::Text(e) = args {
            let _ = ui.set_prop(time, "text", format!("⚠ {e}"));
        }
    });
    VideoPlayer::play(&mut ui, player)?;
    // Mostra a posição: um timer simples com a fila da interface.
    let poster = ui.poster();
    if args.iter().any(|a| a == "--stats") {
        let poster = poster.clone();
        std::thread::spawn(move || {
            let last = std::sync::Arc::new(std::sync::Mutex::new(aylis_media::PlaybackQuality::default()));
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let last = last.clone();
                poster.post(move |ui| {
                    let q = VideoPlayer::playback_quality(ui, player);
                    let mut l = last.lock().unwrap();
                    println!("{:5.1} s  mostrados {:4}/s  pulados {:4}/s", VideoPlayer::position(ui, player), q.shown - l.shown, q.dropped - l.dropped);
                    *l = q;
                });
            }
        });
    }
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        poster.post(move |ui| {
            let pos = VideoPlayer::position(ui, player);
            let total = VideoPlayer::duration(ui, player).unwrap_or(0.0);
            let size = VideoPlayer::video_size(ui, player).map(|(w, h)| format!("  ·  {w}x{h}")).unwrap_or_default();
            let text = format!("{pos:5.1} s / {total:.1} s{size}");
            // só repinta quando o texto muda (pausado, nada muda)
            if !matches!(ui.get_prop(time, "text"), Ok(PropValue::String(s)) if s == text) {
                let _ = ui.set_prop(time, "text", text);
            }
        });
    });
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}

/// Gera um MP4 de 4 s: um degradê que muda de cor e um quadrado andando.
#[cfg(windows)]
fn generated() -> Result<String, Box<dyn std::error::Error>> {
    let path = std::env::temp_dir().join("aylis_video_exemplo.mp4");
    if !path.exists() {
        let (w, h, fps) = (640u32, 360u32, 30u32);
        let frames = (0..fps * 4).map(move |i| {
            let t = i as f32 / (fps * 4) as f32;
            let mut px = vec![0u8; (w * h * 4) as usize];
            let sq = (t * (w - 80) as f32) as u32;
            for y in 0..h {
                for x in 0..w {
                    let k = ((y * w + x) * 4) as usize;
                    let inside = x >= sq && x < sq + 80 && y >= h / 2 - 40 && y < h / 2 + 40;
                    let (r, g, b) = if inside {
                        (255, 255, 255)
                    } else {
                        ((x * 255 / w) as u8, (t * 255.0) as u8, (y * 255 / h) as u8)
                    };
                    px[k..k + 4].copy_from_slice(&[b, g, r, 255]);
                }
            }
            px
        });
        aylis_media::mf::write_mp4(&path, w, h, fps, frames)?;
    }
    Ok(path.display().to_string())
}

#[cfg(not(windows))]
fn generated() -> Result<String, Box<dyn std::error::Error>> {
    Err("informe um arquivo de vídeo (a geração usa o Media Foundation, só no Windows)".into())
}
