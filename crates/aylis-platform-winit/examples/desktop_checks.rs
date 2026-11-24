//! A conferência dos recursos de janela e de sistema: um cenário por vez, que escreve no console
//! o que aconteceu (para conferir na tela ou com o mouse e o teclado simulados).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example desktop_checks -- <cenário>
//! ```
//!
//! - `topmost` (item 6): um form `topmost` com `no_activate`, no canto; escreve
//!   `on_topmost_lost` quando outra janela sempre por cima passa à frente (o Gerenciador de
//!   Tarefas com "Sempre visível") e volta ao topo 1 s depois (`UpdateLevel`). Aos 2 s, o
//!   `place` (pixels físicos, no monitor 0, sem o foco) e o lugar que ficou;
//! - `cover`: outra janela sempre por cima, que ativa e cobre o canto (para o `topmost`);
//! - `border` (item 7): tela cheia sem moldura com `border_color: none`; aos 2 s tira a imagem do
//!   monitor e escreve a cor da borda de cima e a do meio (iguais: sem a linha de 1 px);
//!   `border system` e `border red` comparam com a borda do sistema e com uma vermelha;
//! - `single` (item 9): a instância única (`user`); a segunda abertura (`... single a b`) entrega
//!   os argumentos à primeira, que escreve `on_second_instance`, e sai;
//! - `monitors` (item 10): o `device` e o `device_path` de cada monitor;
//! - `capture` (item 11): a miniatura de cada monitor (320 de largura) e a cor média; sem o
//!   `allow_screen_capture`, o erro (`capture off`);
//! - `cursor` (itens 13 e 16): a cada 1 s (até 10 s), o mouse na tela e o form sob ele;
//! - `keys` (itens 14 e 32): duas janelas; cada tecla escreve o Caps Lock, o Num Lock, o form em
//!   que chegou (`KeyEvent.form`) e o `ui.active_form()`;
//! - `tray` (item 18, com `--features tray`): o clique direito no ícone escreve `on_context`; o
//!   primeiro é recusado, o segundo abre; aos 3 s o `open_menu` abre por código no mouse;
//! - `transparency` (item 21): os "Efeitos de transparência" agora e cada mudança (desligue e
//!   ligue em Configurações → Personalização → Cores).

use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let scenario = args.get(1).cloned().unwrap_or_else(|| "cursor".into());
    if scenario == "single" && !aylis_platform_winit::claim_single_instance(SingleInstance::User, "dev.aylis.desktop-checks") {
        println!("outra instância recebeu os argumentos; esta sai");
        return Ok(());
    }
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None)?;
    ui.set_name(form, "frmChecks")?;
    ui.set_prop(form, "caption", format!("desktop_checks {scenario}"))?;
    ui.set_prop(form, "width", 640.0)?;
    ui.set_prop(form, "height", 400.0)?;
    let label = ui.create("Label", Some(form))?;
    ui.set_prop(label, "align", "top")?;
    ui.set_prop(label, "height", 40.0)?;
    ui.set_prop(label, "text", format!("cenário: {scenario}"))?;
    let say = |text: String| {
        println!("{text}");
        let _ = std::io::Write::flush(&mut std::io::stdout());
    };
    let screen = ui.create("Screen", Some(form))?;
    match scenario.as_str() {
        "topmost" => {
            ui.set_prop(form, "topmost", true)?;
            ui.set_prop(form, "width", 360.0)?;
            ui.set_prop(form, "height", 160.0)?;
            let settings = ui.create("WindowSettings", Some(form))?;
            ui.set_prop(settings, "no_activate", true)?;
            ui.on(form, "on_topmost_lost", move |ui, f, _| {
                say("on_topmost_lost".into());
                ui.set_timeout(1000, move |ui| {
                    ui.window_command(f, WindowCommand::UpdateLevel);
                    say("UpdateLevel".into());
                });
            });
            ui.set_timeout(2000, move |ui| {
                let _ = ui.invoke(form, "place", &[0.into(), 40.into(), 60.into(), 900.into(), 300.into(), false.into()]);
                ui.set_timeout(500, move |ui| {
                    let p = ui.window_placement(form).unwrap_or_default();
                    say(format!("place: x {} y {} {}x{}", p.x, p.y, p.width, p.height));
                });
            });
        }
        // Outra janela sempre por cima, que ativa e cobre o lugar do `topmost` (o outro processo).
        "cover" => {
            ui.set_prop(form, "topmost", true)?;
            ui.set_timeout(300, move |ui| {
                let _ = ui.invoke(form, "place", &[0.into(), 0.into(), 0.into(), 1200.into(), 500.into(), true.into()]);
                say("cover: por cima".into());
            });
        }
        "border" => {
            let variant = args.get(2).cloned().unwrap_or_default();
            // `custom`: a barra própria (a janela mantém a moldura de redimensionar do sistema).
            let style = if args.iter().any(|a| a == "custom") { "custom" } else { "none" };
            ui.set_prop(form, "border_style", style)?;
            ui.set_prop(form, "fill", Color::rgb(0x20, 0x60, 0x20))?;
            let settings = ui.create("WindowSettings", Some(form))?;
            let border = match variant.as_str() {
                "system" => PropValue::Null,
                "red" => PropValue::Color(Color::rgb(255, 0, 0)),
                _ => PropValue::String("none".into()),
            };
            ui.set_prop(settings, "border_color", border)?;
            ui.set_allow_screen_capture(true);
            // `window`: numa janela comum (a borda em volta dela); sem: em tela cheia.
            let windowed = args.get(3).is_some_and(|a| a == "window");
            if !windowed {
                ui.set_timeout(300, move |ui| ui.window_command(form, WindowCommand::SetFullscreen(true)));
            }
            ui.set_timeout(2500, move |ui| {
                let monitor = ui.form_monitor(form).unwrap_or(0);
                let p = ui.window_placement(form).unwrap_or_default();
                let origin = ui.monitors().get(monitor).map_or((0, 0), |m| (m.x, m.y));
                ui.capture_monitor(monitor, 0, move |_, result| match result {
                    Ok(image) => {
                        let at = |x: i32, y: i32| pixel(&image, (x - origin.0).max(0) as u32, (y - origin.1).max(0) as u32);
                        let (w, h) = image_size(&image);
                        if windowed {
                            let (left, top) = (p.x, p.y);
                            say(format!("janela em {left},{top}: em volta {:?} {:?} {:?}", at(left + p.width as i32 / 2, top - 1), at(left + p.width as i32 / 2, top), at(left + p.width as i32 / 2, top + 1)));
                        } else {
                            say(format!("borda de cima {:?}, esquerda {:?}, meio {:?}", at(w as i32 / 2, 0), at(0, h as i32 / 2), at(w as i32 / 2, h as i32 / 2)));
                        }
                    }
                    Err(e) => say(format!("captura: {e}")),
                });
            });
        }
        "single" => {
            let events = ui.create("ApplicationEvents", Some(form))?;
            ui.on(events, "on_second_instance", move |ui, _, args| {
                say(format!("on_second_instance: {:?} (exatos: {:?})", args, ui.second_instance_args()));
            });
        }
        "monitors" => {
            ui.set_timeout(500, move |ui| {
                for (i, m) in ui.monitors().iter().enumerate() {
                    say(format!("{i}: {} · {} · {}", m.name, m.device, m.device_path));
                }
                let json = ui.invoke(screen, "monitor", &[0.into()]).unwrap_or_default().to_str().unwrap_or_default().to_string();
                say(format!("Screen.monitor(0): {}", &json[..json.find("\"modes\"").unwrap_or(json.len())]));
            });
        }
        "capture" => {
            let off = args.get(2).is_some_and(|a| a == "off");
            ui.set_allow_screen_capture(!off);
            ui.on(screen, "on_capture", move |_, _, args| {
                let EventArgs::Text(json) = args else { return };
                say(format!("on_capture: {}", &json[..json.len().min(120)]));
            });
            ui.set_timeout(500, move |ui| {
                for i in 0..ui.monitors().len() {
                    ui.capture_monitor(i, 320, move |_, result| match result {
                        Ok(image) => {
                            let (w, h) = image_size(&image);
                            say(format!("monitor {i}: {w}x{h}, cor média {:?}", average(&image)));
                        }
                        Err(e) => say(format!("monitor {i}: {e}")),
                    });
                }
                let _ = ui.invoke(screen, "capture_monitor", &[0.into(), 160.into()]);
            });
        }
        "cursor" => {
            for s in 1..=10u64 {
                ui.set_timeout(s * 1000, move |ui| {
                    let at = ui.cursor_position();
                    let over = at.and_then(|c| ui.form_at_screen(c.x, c.y)).and_then(|f| ui.name(f).map(str::to_string));
                    let json = ui.invoke(screen, "cursor_position", &[]).unwrap_or_default();
                    say(format!("{s}: {at:?} sobre {over:?} · Screen {json:?}"));
                });
            }
        }
        "keys" => {
            let other = ui.create("Form", None)?;
            ui.set_name(other, "frmOutra")?;
            ui.set_prop(other, "caption", "desktop_checks outra")?;
            ui.set_prop(other, "width", 400.0)?;
            ui.set_prop(other, "height", 200.0)?;
            let events = ui.create("ApplicationEvents", Some(form))?;
            ui.on(events, "on_key_down", move |ui, _, args| {
                let EventArgs::Key(k) = args else { return };
                let name = |f: Option<ComponentId>| f.and_then(|f| ui.name(f).map(str::to_string));
                say(format!("{:?}: caps {} num {} · form {:?} · ativo {:?}", k.key, k.modifiers.caps_lock, k.modifiers.num_lock, name(k.form), name(ui.active_form())));
            });
            ui.set_timeout(200, move |ui| {
                let _ = ui.show(other);
            });
        }
        "tray" => {
            let tray = ui.create("TrayIcon", Some(form))?;
            ui.set_prop(tray, "hint", "desktop_checks")?;
            ui.set_prop(tray, "menu_items", r#"[{"text": "Abrir"}, {"separator": true}, {"text": "Sair"}]"#)?;
            let count = std::rc::Rc::new(std::cell::Cell::new(0));
            ui.on(tray, "on_context", move |ui, _, args| {
                count.set(count.get() + 1);
                say(format!("on_context {}: {args:?}", count.get()));
                if count.get() == 1 {
                    ui.refuse("recusado no primeiro");
                }
            });
            ui.on(tray, "on_menu", move |_, _, args| say(format!("on_menu: {args:?}")));
            ui.set_timeout(3000, move |ui| {
                let _ = ui.invoke(tray, "open_menu", &[(-1).into(), (-1).into()]);
                say("open_menu".into());
            });
        }
        "transparency" => {
            say(format!("agora: {}", ui.system_transparency()));
            ui.on(screen, "on_transparency_change", move |ui, _, args| say(format!("on_transparency_change: {args:?} (agora {})", ui.system_transparency())));
        }
        other => say(format!("cenário desconhecido: {other}")),
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}

fn image_size(image: &Image) -> (u32, u32) {
    match image.source() {
        image::ImageSource::Rgba { width, height, .. } => (*width, *height),
        _ => (0, 0),
    }
}

fn pixel(image: &Image, x: u32, y: u32) -> Option<[u8; 3]> {
    let image::ImageSource::Rgba { width, height, pixels } = image.source() else { return None };
    (x < *width && y < *height).then(|| {
        let i = (y as usize * *width as usize + x as usize) * 4;
        [pixels[i], pixels[i + 1], pixels[i + 2]]
    })
}

fn average(image: &Image) -> [u8; 3] {
    let image::ImageSource::Rgba { pixels, .. } = image.source() else { return [0; 3] };
    let n = (pixels.len() / 4).max(1) as u64;
    let mut sum = [0u64; 3];
    for px in pixels.chunks_exact(4) {
        for c in 0..3 {
            sum[c] += u64::from(px[c]);
        }
    }
    [(sum[0] / n) as u8, (sum[1] / n) as u8, (sum[2] / n) as u8]
}
