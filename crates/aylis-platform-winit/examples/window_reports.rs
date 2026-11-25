//! A conferência da janela de vídeo igual no Windows e no Linux: a escala (`form_scale`,
//! `on_scale_change`), o fim do redimensionamento (`on_resize_end`), o trazer para a frente
//! (`bring_to_front`) e o mouse sobre a janela (`pointer_over`, `form_at_screen`). Escreve no
//! console o que aconteceu.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example window_reports [-- auto]
//! ```
//!
//! - ao abrir: a escala da janela e a sessão (o `cursor_position` some no Wayland);
//! - a cada 1 s: o mouse na tela, o form sob ele e o `pointer_over`;
//! - arraste a borda: o `on_resize_end` sai uma vez, ao soltar (Windows) ou 300 ms depois do
//!   último tamanho (Linux, macOS);
//! - leve a janela a um monitor de outra escala: o `on_scale_change` com a escala nova;
//! - com `auto`: aos 1 s muda a largura pelo código (o `on_resize_end` 300 ms depois), aos 2 s
//!   minimiza e aos 3 s traz para a frente (`bring_to_front`), escreve se voltou e fecha aos
//!   5 s. No meio, o form sob o centro da janela (`form_at_screen`).
//! - com `size`: aos 1 s põe a área cliente em 900 × 560 e aos 2,5 s a largura em 520 pelo
//!   código (o `ApplySize`), e escreve o tamanho do form, o da janela e o `form_monitor` antes e
//!   depois (no Wayland, o winit aplica o tamanho na hora, sem `Resized`).

use std::time::Instant;

use aylis_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auto = std::env::args().any(|a| a == "auto");
    let size = std::env::args().any(|a| a == "size");
    let started = Instant::now();
    let say = move |text: String| println!("[{:>5.2}s] {text}", started.elapsed().as_secs_f64());
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.create("Form", None)?;
    ui.set_name(form, "frmReports")?;
    ui.set_prop(form, "caption", "window_reports")?;
    ui.set_prop(form, "width", 640.0)?;
    ui.set_prop(form, "height", 400.0)?;
    let label = ui.create("Label", Some(form))?;
    ui.set_prop(label, "align", "client")?;
    ui.set_prop(label, "text", "Arraste a borda, troque de monitor, passe o mouse")?;
    ui.on(form, "on_scale_change", move |_, _, args| say(format!("on_scale_change {args:?}")));
    ui.on(form, "on_resize_end", move |ui, f, _| {
        let w = ui.get_prop(f, "client_width").unwrap_or_default();
        say(format!("on_resize_end (client_width {w:?})"));
    });
    ui.on(form, "on_window_created", move |ui, f, _| {
        let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        say(format!("escala {} (sessão '{session}', WAYLAND_DISPLAY {:?})", ui.form_scale(f), std::env::var("WAYLAND_DISPLAY").ok()));
    });
    fn tick(ui: &mut Ui, form: ComponentId, n: u32, say: impl Fn(String) + Copy + 'static) {
        let at = ui.cursor_position();
        let under = at.and_then(|c| ui.form_at_screen(c.x, c.y)).and_then(|f| ui.name(f).map(str::to_string));
        say(format!("escala {} | mouse {:?} | form sob ele {under:?} | pointer_over {}", ui.form_scale(form), at.map(|c| (c.x, c.y)), ui.pointer_over(form)));
        if n < 10 {
            ui.set_timeout(1000, move |ui| tick(ui, form, n + 1, say));
        }
    }
    ui.set_timeout(1000, move |ui| tick(ui, form, 1, say));
    if auto {
        ui.set_timeout(1000, move |ui| {
            let _ = ui.set_prop(form, "width", 700.0);
            say("largura 700 pelo código".into());
        });
        ui.set_timeout(1800, move |ui| {
            if let Some(p) = ui.window_placement(form) {
                let (x, y) = (p.x + p.width as i32 / 2, p.y + p.height as i32 / 2);
                say(format!("form_at_screen no centro ({x}, {y}): {:?}", ui.form_at_screen(x, y).and_then(|f| ui.name(f).map(str::to_string))));
            }
        });
        ui.set_timeout(2000, move |ui| {
            ui.window_command(form, WindowCommand::Minimize);
            say("minimizar".into());
        });
        ui.set_timeout(3000, move |ui| {
            say(format!("bring_to_front (minimizada: {})", ui.is_minimized(form)));
            let _ = ui.invoke(form, "bring_to_front", &[]);
        });
        ui.set_timeout(4000, move |ui| say(format!("depois do bring_to_front, minimizada: {}", ui.is_minimized(form))));
        ui.set_timeout(5000, move |ui| ui.request_close());
    }
    if size {
        let report = move |ui: &mut Ui, when: &str| {
            let (w, h) = (ui.get_prop(form, "client_width").unwrap_or_default(), ui.get_prop(form, "client_height").unwrap_or_default());
            let placed = ui.window_placement(form).map(|p| (p.width, p.height));
            say(format!("{when}: client {w:?} × {h:?} | janela (pixels) {placed:?} | escala {} | form_monitor {:?}", ui.form_scale(form), ui.form_monitor(form)));
        };
        ui.set_timeout(1000, move |ui| {
            report(ui, "antes");
            let _ = ui.set_prop(form, "client_width", 900.0);
            let _ = ui.set_prop(form, "client_height", 560.0);
            say("client_width 900, client_height 560 pelo código".into());
        });
        ui.set_timeout(2000, move |ui| report(ui, "1 s depois"));
        ui.set_timeout(2500, move |ui| {
            let _ = ui.set_prop(form, "client_width", 520.0);
            say("client_width 520 pelo código".into());
        });
        ui.set_timeout(3500, move |ui| report(ui, "1 s depois"));
        ui.set_timeout(4000, move |ui| ui.request_close());
    }
    aylis_platform_winit::run(ui, form)?;
    Ok(())
}
