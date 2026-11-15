//! O form que vem para a frente sozinho (`force_foreground`): o editor de print do NoOBS abre
//! por um atalho global, e o Windows só deixa ir para a frente quem recebeu a última entrada.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example foreground --features cpu
//! ```
//!
//! Abre uma janela pequena; ponha outro app na frente (clique nele). Em 5 s, um segundo form
//! abre pelo timer (como pelo atalho): com `force_foreground`, ele vem para a frente com o
//! cursor no campo (digite, e o Esc fecha); `AYLIS_FORCE_FOREGROUND=0` desliga, e ele fica
//! atrás com o botão piscando na barra de tarefas. A linha `na frente: sim/não` diz se a janela
//! dele ficou em primeiro plano. `AYLIS_AUTO_CLOSE=1` fecha tudo 2 s depois.

use aylis_controls::*;
use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = ui.add_root(Form::new("Aylis — ponha outro app na frente", 420.0, 120.0));
    let hint = ui.create("Label", Some(main))?;
    ui.set_prop(hint, "align", "client")?;
    ui.set_prop(hint, "text", "Clique em outro app: em 5 s o print abre por cima dele.")?;
    let print = ui.add_root(Form::new("Print", 360.0, 140.0));
    let forced = std::env::var("AYLIS_FORCE_FOREGROUND").map_or(true, |v| v != "0");
    ui.set_prop(print, "force_foreground", forced)?;
    ui.set_prop(print, "key_preview", true)?;
    let edit = ui.create("Edit", Some(print))?;
    ui.set_prop(edit, "align", "top")?;
    ui.set_prop(edit, "placeholder", "Digite aqui (Esc fecha)")?;
    ui.set_prop(print, "active_control", PropValue::Component(Some(edit)))?;
    ui.on(print, "on_key_down", |ui, form, args| {
        if let EventArgs::Key(k) = args {
            if k.key == Key::Escape {
                ui.close(form);
            }
        }
    });
    ui.set_timeout(5000, move |ui| {
        let _ = ui.show(print);
        // A janela abre na volta seguinte do laço: confere um pouco depois.
        ui.set_timeout(500, move |ui| {
            println!("force_foreground={forced} na frente: {}", if is_foreground(print) { "sim" } else { "não" });
            if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
                ui.set_timeout(2000, |ui| ui.request_close());
            }
        });
    });
    aylis_platform_winit::run(ui, main)?;
    Ok(())
}

/// A janela do form é a que está em primeiro plano?
#[cfg(windows)]
fn is_foreground(form: ComponentId) -> bool {
    // SAFETY: só lê qual janela está em primeiro plano.
    let foreground = unsafe { windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow() };
    aylis_platform_winit::win::hwnd(form) == Some(foreground.0 as isize)
}

#[cfg(not(windows))]
fn is_foreground(_form: ComponentId) -> bool {
    true
}
