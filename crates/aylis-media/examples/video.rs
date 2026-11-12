//! Um `VideoPlayer` com cantos arredondados e controles por cima.
//!
//! ```text
//! cargo run -p aylis-media --example video                 gera um MP4 animado e toca
//! cargo run -p aylis-media --example video -- filme.mp4    toca o arquivo
//! cargo run -p aylis-media --example video -- filme.mp4 --muted
//! cargo run -p aylis-media --example video -- filme.mp4 --stats   quadros por segundo no terminal
//! cargo run -p aylis-media --example video -- filme.mp4 --rate 4    em 4x
//! cargo run -p aylis-media --example video -- 4k.mp4 --size 3840x2160 --stats --seconds 10
//!     o player no tamanho do vídeo (passa da janela), e fecha depois de 10 s
//! cargo run -p aylis-media --example video -- 4k.mp4 --cpu-frames   quadros sempre pela CPU
//!     (sem a textura do W15: para medir o antes e o depois)
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

    #[cfg(windows)]
    if args.iter().any(|a| a == "--cpu-frames") {
        aylis_media::register_decoder(pixels_only::PixelsOnly);
    }
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
    match args.iter().position(|a| a == "--size").and_then(|i| args.get(i + 1)).and_then(|s| s.split_once('x')).and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?))) {
        Some((w, h)) => {
            ui.control_mut(player).unwrap().set_rect(16.0, 16.0, w, h);
        }
        None => {
            ui.control_mut(player).unwrap().set_align(Align::Client);
        }
    }
    if let Some(seconds) = args.iter().position(|a| a == "--seconds").and_then(|i| args.get(i + 1)).and_then(|s| s.parse::<f64>().ok()) {
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
            std::process::exit(0);
        });
    }
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
                    let texture = VideoPlayer::current_video_frame(ui, player).is_some_and(|f| f.gpu.is_some());
                    println!("{:5.1} s  mostrados {:4}/s  pulados {:4}/s  {}", VideoPlayer::position(ui, player), q.shown - l.shown, q.dropped - l.dropped, if texture { "textura" } else { "pixels" });
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

/// O Media Foundation sem a textura (W15): os quadros sempre descem para a CPU.
#[cfg(windows)]
mod pixels_only {
    use aylis_media::{AudioOpener, Frame, MediaSource, VideoDecoder, VideoDecoderFactory, VideoInfo};

    pub struct PixelsOnly;

    impl VideoDecoderFactory for PixelsOnly {
        fn name(&self) -> &str {
            "Media Foundation (pixels)"
        }
        fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
            aylis_media::mf::MediaFoundation.open(source).map(|d| Box::new(NoTexture(d)) as Box<dyn VideoDecoder>)
        }
    }

    struct NoTexture(Box<dyn VideoDecoder>);

    impl VideoDecoder for NoTexture {
        fn info(&self) -> VideoInfo {
            self.0.info()
        }
        fn seek(&mut self, seconds: f64) {
            self.0.seek(seconds)
        }
        fn next_frame(&mut self) -> Option<Frame> {
            self.0.next_frame()
        }
        fn set_output_size(&mut self, width: u32, height: u32) {
            self.0.set_output_size(width, height)
        }
        fn seek_keyframe(&mut self, seconds: f64) {
            self.0.seek_keyframe(seconds)
        }
        fn skip_until(&mut self, seconds: f64) {
            self.0.skip_until(seconds)
        }
        fn audio(&mut self) -> Option<AudioOpener> {
            self.0.audio()
        }
        fn error(&self) -> Option<String> {
            self.0.error()
        }
    }
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
