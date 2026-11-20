//! A abertura da janela: ela só aparece com o primeiro quadro pintado (sem o branco), e a tela
//! cheia, de nascença ou pedida depois, cobre o monitor inteiro (a barra de tarefas também).
//! O form é escuro de propósito: um quadro branco na abertura salta à vista.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example window_start --features d2d -- [normal|full|full_later|back] [--stay] [--fill #rrggbb] [--cursor wait] [--icon logo.png] [--border custom] [--monitor N] [--remember]
//! ```
//!
//! - `normal`: a janela comum;
//! - `full`: já nasce em tela cheia (`window_state = full_screen`);
//! - `full_later`: nasce comum e vai para a tela cheia 600 ms depois;
//! - `back`: nasce em tela cheia e volta ao normal 600 ms depois;
//! - `switch`: em tela cheia, troca de monitor com o app rodando (a cada 2 s, veja abaixo).
//!
//! Sem `--stay`, depois de 1,5 s escreve o tamanho do form e o do monitor e fecha.

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "normal".into());
    let stay = args.iter().any(|a| a == "--stay");
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.set_theme(Theme::dark());

    let form = ui.add_root(Form::new("Aylis — abertura da janela", 640.0, 400.0));
    ui.set_prop(form, "name", "frmAbertura")?;
    // `--fill #rrggbb`: outra cor (para achar a janela numa gravação da tela).
    let fill = args.iter().position(|a| a == "--fill").and_then(|i| args.get(i + 1)).and_then(|c| Color::from_hex(c)).unwrap_or(Color::rgb(24, 24, 28));
    ui.set_prop(form, "fill", fill)?;
    // `--cursor wait`: o cursor do form (para conferir os cursores do sistema).
    if let Some(cursor) = args.iter().position(|a| a == "--cursor").and_then(|i| args.get(i + 1)) {
        ui.set_prop(form, "cursor", cursor.as_str())?;
    }
    // `--border custom`: o `border_style` (a barra própria redimensiona pelas bordas, menos em
    // tela cheia).
    if let Some(border) = args.iter().position(|a| a == "--border").and_then(|i| args.get(i + 1)) {
        ui.set_prop(form, "border_style", border.as_str())?;
    }
    // `--remember`: o `remember_placement` (reabre onde estava).
    if args.iter().any(|a| a == "--remember") {
        ui.set_prop(form, "remember_placement", true)?;
    }
    // `--monitor N`: o monitor em que ele nasce.
    if let Some(m) = args.iter().position(|a| a == "--monitor").and_then(|i| args.get(i + 1)).and_then(|m| m.parse::<i64>().ok()) {
        ui.set_prop(form, "monitor", m)?;
    }
    if matches!(mode.as_str(), "full" | "back" | "switch") {
        ui.set_prop(form, "window_state", "full_screen")?;
    }
    let text = ui.create("Label", Some(form))?;
    ui.set_prop(text, "align", "client")?;
    ui.set_prop(text, "font_size", 22.0)?;
    ui.set_prop(text, "text", format!("modo: {mode}"))?;

    match mode.as_str() {
        "full_later" => {
            ui.set_timeout(600, move |ui| {
                let _ = ui.set_prop(form, "window_state", "full_screen");
            });
        }
        "back" => {
            ui.set_timeout(600, move |ui| {
                let _ = ui.set_prop(form, "window_state", "normal");
            });
        }
        // Com o app rodando, a cada 2 s: a tela cheia passa para o monitor 1, volta a janela
        // comum (no monitor 1), vai para o 0 e fica em tela cheia nele.
        "switch" => {
            let steps: [(u64, &'static str, i64); 4] = [(2000, "monitor", 1), (4000, "normal", -1), (6000, "monitor", 0), (8000, "full_screen", -1)];
            for (at, what, value) in steps {
                ui.set_timeout(at, move |ui| {
                    let _ = if what == "monitor" { ui.set_prop(form, "monitor", value) } else { ui.set_prop(form, "window_state", what) };
                    println!("passo: {what} {value}");
                });
            }
        }
        _ => {}
    }
    if !stay {
        ui.set_timeout(1500, move |ui| {
            let (w, h) = ui.control(form).map_or((0.0, 0.0), |cb| (cb.width(), cb.height()));
            let state = ui.get_prop(form, "window_state").map(|v| format!("{v:?}")).unwrap_or_default();
            let monitor = ui.form_monitor(form).and_then(|i| ui.monitors().get(i)).or_else(|| ui.monitors().first());
            match monitor {
                Some(m) => println!(
                    "form {w:.0}x{h:.0} (lógico) | monitor {}x{} físico, escala {:.2} = {:.0}x{:.0} lógico | estado {state}",
                    m.width,
                    m.height,
                    m.scale,
                    m.width as f64 / m.scale,
                    m.height as f64 / m.scale
                ),
                None => println!("form {w:.0}x{h:.0} | sem monitor | estado {state}"),
            }
            ui.request_close();
        });
    }
    // `--icon logo.png`: o ícone da janela (para conferir como o sistema o recebe).
    let icon = args.iter().position(|a| a == "--icon").and_then(|i| args.get(i + 1)).map(std::fs::read).transpose()?;
    aylis_platform_winit::run_with(ui, form, aylis_platform_winit::Options { icon, ..Default::default() })
}
