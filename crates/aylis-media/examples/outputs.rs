//! Várias saídas de áudio tocando juntas (o alto-falante e um fone Bluetooth, por exemplo):
//! marque as saídas, ajuste o atraso de cada uma (ms) e toque um metrônomo. Fora de
//! sincronia, o clique de uma saída soa como eco do da outra; aumente o `delay` da que
//! adianta até o eco sumir.
//!
//! ```text
//! cargo run -p aylis-media --example outputs
//! ```

use aylis_controls::{Button, CheckBox, Edit, Form, Label, Panel};
use aylis_core::*;
use aylis_media::{MediaPlayer, OutputSpec, format_outputs, output_devices, register};

/// Um metrônomo de 4 s (um clique a cada meio segundo; o primeiro de cada 4, mais agudo)
/// num WAV temporário.
fn metronome() -> std::io::Result<String> {
    let rate = 48_000u32;
    let mut samples = vec![0i16; rate as usize * 4];
    for beat in 0..8 {
        let start = beat * rate as usize / 2;
        let freq = if beat % 4 == 0 { 1760.0 } else { 880.0 };
        for i in 0..(rate as usize / 40) {
            let t = i as f32 / rate as f32;
            let fade = 1.0 - i as f32 / (rate as f32 / 40.0);
            samples[start + i] = ((t * freq * std::f32::consts::TAU).sin() * fade * 0.8 * f32::from(i16::MAX)) as i16;
        }
    }
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let mut wav = Vec::with_capacity(44 + data.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data.len() as u32).to_le_bytes());
    wav.extend_from_slice(&data);
    let path = std::env::temp_dir().join("aylis-metronomo.wav");
    std::fs::write(&path, wav)?;
    Ok(path.display().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = metronome()?;
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    register(&mut ui);
    let form = ui.add_root(Form::new("Saídas de áudio", 620.0, 420.0));
    let _ = ui.set_prop(form, "padding", 16);

    let title = ui.add(Label::new("Marque as saídas e ajuste o atraso (ms) de cada uma:"), form);
    ui.control_mut(title).unwrap().set_align(Align::Top).set_height(32.0);

    // Uma linha por saída: a caixa (o nome) e o atraso.
    let mut rows = Vec::new();
    for device in output_devices() {
        let row = ui.add(Panel::default(), form);
        ui.control_mut(row).unwrap().set_align(Align::Top).set_height(40.0);
        let check = ui.add(CheckBox::new(if device.is_default { format!("{} (padrão)", device.name) } else { device.name.clone() }), row);
        let _ = ui.set_prop(check, "checked", device.is_default);
        ui.control_mut(check).unwrap().set_align(Align::Client);
        let delay = ui.add(Edit::new("0"), row);
        ui.control_mut(delay).unwrap().set_align(Align::Right).set_width(90.0).set_margins([8.0, 4.0, 0.0, 4.0]);
        rows.push((device.name, check, delay));
    }

    let player = ui.add(MediaPlayer::new(file), form);
    let _ = ui.set_prop(player, "looping", true);
    let play = ui.add(Button::new("Tocar (aplica as saídas)"), form);
    ui.control_mut(play).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 12.0, 300.0, 0.0]);
    let stop = ui.add(Button::new("Parar"), form);
    ui.control_mut(stop).unwrap().set_align(Align::Top).set_height(40.0).set_margins([0.0, 8.0, 300.0, 0.0]);
    let status = ui.add(Label::new(""), form);
    ui.control_mut(status).unwrap().set_align(Align::Top).set_height(48.0).set_margins([0.0, 8.0, 0.0, 0.0]);

    ui.on_click(play, move |ui, _| {
        let specs: Vec<OutputSpec> = rows
            .iter()
            .filter(|(_, check, _)| ui.get::<CheckBox>(*check).is_some_and(|c| c.checked))
            .map(|(name, _, delay)| {
                let ms = ui.get::<Edit>(*delay).and_then(|e| e.text.trim().replace(',', ".").parse::<f32>().ok()).unwrap_or(0.0);
                OutputSpec { delay_ms: ms.max(0.0), ..OutputSpec::new(name.clone()) }
            })
            .collect();
        let outputs = format_outputs(&specs);
        MediaPlayer::stop(ui, player);
        let _ = ui.set_prop(player, "output", outputs.clone());
        let text = match MediaPlayer::play(ui, player) {
            Ok(()) if outputs.is_empty() => "Tocando na saída padrão.".to_string(),
            Ok(()) => format!("Tocando em: {outputs}"),
            Err(e) => format!("Erro: {e}"),
        };
        let _ = ui.set_prop(status, "text", text);
    });
    ui.on_click(stop, move |ui, _| {
        MediaPlayer::stop(ui, player);
        let _ = ui.set_prop(status, "text", "Parado.");
    });
    aylis_platform_winit::run(ui, form)
}
