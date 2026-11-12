//! A placa de vídeo de cada janela (`gpu = monitor`): mostra em que placa a janela desenha e
//! imprime cada troca (`on_gpu_change`).
//!
//! - `--auto N`: N idas e voltas entre os monitores 0 e 1 (Windows; no Wayland o app não move
//!   a janela, e a troca vem do compositor ao arrastá-la).
//! - `--toggle N`: N trocas da propriedade `gpu` do form entre a placa padrão e `--other`
//!   (padrão: o WARP no Windows, o llvmpipe no Linux), com o app aberto.
//!
//! Com uma placa só, no Windows, `AYLIS_GPU_SWITCH=warp` faz cada troca de monitor alternar
//! entre a placa e o WARP (o renderizador por software, outra placa para o DXGI).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example gpu_monitor --features d2d -- --auto 3
//! cargo run -p aylis-platform-winit --example gpu_monitor -- --toggle 4
//! ```

use std::cell::Cell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;
use aylis_platform_winit::{Options, Renderer};

fn text_of(value: std::result::Result<PropValue, String>) -> String {
    match value {
        Ok(PropValue::String(text)) => text,
        _ => String::new(),
    }
}

fn arg<T: std::str::FromStr>(args: &[String], name: &str) -> Option<T> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|n| n.parse().ok())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let auto: Option<u32> = arg(&args, "--auto");
    let toggle: Option<u32> = arg(&args, "--toggle");
    let other: String = arg(&args, "--other").unwrap_or_else(|| if cfg!(windows) { "Basic Render".into() } else { "llvmpipe".into() });
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Placa de vídeo por monitor", 520.0, 360.0));
    let _ = ui.set_name(form, "frmPlaca");
    let _ = ui.set_prop(form, "padding", 16);
    let _ = ui.set_prop(form, "gpu", "monitor");

    // Algo que vai para a GPU (gradiente, sombra, cantos): refeito na placa nova.
    let card = ui.add(Rectangle::default(), form);
    ui.control_mut(card).unwrap().set_align(Align::Top).set_height(120.0);
    let _ = ui.set_prop(card, "fill", "primary");
    let _ = ui.set_prop(card, "fill2", "accent");
    let _ = ui.set_prop(card, "radius", 16.0);
    let _ = ui.set_prop(card, "shadow", "lg");
    let info = ui.add(Label::new(""), form);
    ui.control_mut(info).unwrap().set_align(Align::Client).set_margins([0.0, 12.0, 0.0, 0.0]);
    let _ = ui.set_prop(info, "wrap", true);
    let buttons = ui.add(FlowLayout::default(), form);
    ui.control_mut(buttons).unwrap().set_align(Align::Bottom).set_height(44.0);
    for i in 0..2i64 {
        let b = ui.add(Button::new(format!("Monitor {i}")), buttons);
        ui.control_mut(b).unwrap().set_size(140.0, 36.0);
        ui.on_click(b, move |ui, _| {
            let _ = ui.invoke(form, "move_to_monitor", &[PropValue::from(i)]);
        });
    }

    let changes = Rc::new(Cell::new(0u32));
    let seen = changes.clone();
    ui.on(form, "on_gpu_change", move |_, _, args| {
        seen.set(seen.get() + 1);
        println!("on_gpu_change: {}", args.text().unwrap_or_default());
    });

    let timer = ui.add(Timer::new(700), form);
    let step = Cell::new(0u32);
    ui.on(timer, "on_timer", move |ui, _, _| {
        let adapter = text_of(ui.invoke(form, "gpu_adapter", &[]));
        let all = text_of(ui.invoke(form, "gpu_adapters", &[]));
        let rule = text_of(ui.get_prop(form, "gpu").map_err(|e| e.to_string()));
        let text = format!(
            "Regra: {rule}\nMonitor: {:?} de {}\nDesenhando em: {adapter}\nPlacas: {all}\nTrocas de placa: {}",
            ui.form_monitor(form),
            ui.monitors().len(),
            changes.get()
        );
        let _ = ui.set_prop(info, "text", text);
        step.set(step.get() + 1);
        let n = step.get();
        if n == 1 {
            println!("abriu no monitor {:?}, placa {adapter}; placas: {all}", ui.form_monitor(form));
            return;
        }
        if let Some(rounds) = toggle {
            if n <= rounds + 1 {
                let next = if n % 2 == 0 { other.clone() } else { String::new() };
                println!("gpu = {next:?} (estava em {adapter})");
                let _ = ui.set_prop(form, "gpu", next);
            } else {
                println!("fim: placa {adapter}, {} trocas", changes.get());
                ui.request_close();
            }
            return;
        }
        let Some(rounds) = auto else { return };
        if ui.monitors().len() < 2 {
            println!("o --auto precisa de dois monitores (o sistema mostra {})", ui.monitors().len());
            ui.request_close();
            return;
        }
        if n <= rounds * 2 + 1 {
            let target = (n % 2) as i64;
            let _ = ui.invoke(form, "move_to_monitor", &[PropValue::from(target)]);
            println!("movendo para o monitor {target} (estava em {adapter})");
        } else {
            println!("fim: monitor {:?}, placa {adapter}, {} trocas", ui.form_monitor(form), changes.get());
            ui.request_close();
        }
    });
    // No Windows o Direct2D (compile com a feature `d2d`); nos outros, o Skia na GPU.
    let renderer = if cfg!(windows) { Renderer::D2d } else { Renderer::Gpu };
    aylis_platform_winit::run_with(ui, form, Options { renderer, ..Default::default() })?;
    Ok(())
}
