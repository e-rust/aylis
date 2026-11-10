//! Os monitores: a lista como o app a recebe (nome do aparelho, nome do sistema, posição,
//! área útil, escala, taxa de atualização e modos de vídeo), em qual deles está a janela, um
//! botão para levá-la a cada um e o `remember_placement` (feche e abra de novo: volta onde
//! estava, no mesmo monitor). `--list` só imprime a lista e sai; `--move N` leva a janela ao
//! monitor N, diz em qual ficou e fecha (o teste do `move_to_monitor` e do `remember_placement`).
//!
//! ```text
//! cargo run -p aylis-platform-winit --example monitors --features cpu
//! cargo run -p aylis-platform-winit --example monitors --features cpu -- --list
//! ```

use aylis_controls::*;
use aylis_core::*;

fn describe(ui: &Ui, form: ComponentId) -> String {
    let current = ui.form_monitor(form);
    let mut text = String::new();
    for (i, m) in ui.monitors().iter().enumerate() {
        let here = if current == Some(i) { "   ← esta janela" } else { "" };
        let main = if m.primary { " (principal)" } else { "" };
        let top = m.modes.iter().map(|v| v.refresh_hz).fold(0.0, f64::max);
        text.push_str(&format!(
            "Monitor {i}{main}: {}{here}\n  sistema: {}\n  posição {}, {} · {} x {} pixels · escala {:.0}%\n  área útil {}, {} · {} x {} (sem a barra de tarefas)\n  {:.0} Hz agora · {} modos de vídeo (até {:.0} Hz)\n\n",
            m.name,
            m.device,
            m.x,
            m.y,
            m.width,
            m.height,
            m.scale * 100.0,
            m.work_x,
            m.work_y,
            m.work_width,
            m.work_height,
            m.refresh_hz,
            m.modes.len(),
            top
        ));
    }
    text
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let list_only = args.iter().any(|a| a == "--list");
    let move_to: Option<i64> = args.iter().position(|a| a == "--move").and_then(|i| args.get(i + 1)).and_then(|n| n.parse().ok());
    let step = std::cell::Cell::new(0u32);
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Monitores", 640.0, 520.0));
    let _ = ui.set_name(form, "frmMonitores");
    let _ = ui.set_prop(form, "padding", 16);
    let _ = ui.set_prop(form, "remember_placement", true);

    let buttons = ui.add(FlowLayout::default(), form);
    ui.control_mut(buttons).unwrap().set_align(Align::Top).set_height(44.0);
    let info = ui.add(Label::new("Lendo os monitores…"), form);
    ui.control_mut(info).unwrap().set_align(Align::Client).set_margins([0.0, 12.0, 0.0, 0.0]);
    let _ = ui.set_prop(info, "wrap", true);

    // A lista chega depois de a janela abrir: confere a cada meio segundo.
    let timer = ui.add(Timer::new(500), form);
    let made = std::cell::Cell::new(0usize);
    ui.on(timer, "on_timer", move |ui, _, _| {
        if let Some(target) = move_to.filter(|_| !ui.monitors().is_empty()) {
            step.set(step.get() + 1);
            match step.get() {
                1 => println!("abriu no monitor {:?}", ui.form_monitor(form)),
                2 => {
                    let _ = ui.invoke(form, "move_to_monitor", &[PropValue::from(target)]);
                }
                5 => {
                    println!("depois de mover: monitor {:?}", ui.form_monitor(form));
                    ui.request_close();
                }
                _ => {}
            }
            return;
        }
        if list_only && !ui.monitors().is_empty() {
            print!("{}", describe(ui, form));
            ui.request_close();
            return;
        }
        let text = describe(ui, form);
        let _ = ui.set_prop(info, "text", text);
        // Um botão por monitor (refeitos se a quantidade mudar).
        let count = ui.monitors().len();
        if count != made.get() {
            made.set(count);
            for child in ui.children(buttons).to_vec() {
                ui.free(child);
            }
            for i in 0..count {
                let b = ui.add(Button::new(format!("Mover para o monitor {i}")), buttons);
                ui.control_mut(b).unwrap().set_size(200.0, 36.0);
                ui.on_click(b, move |ui, _| {
                    let _ = ui.invoke(form, "move_to_monitor", &[PropValue::from(i as i64)]);
                });
            }
        }
    });
    aylis_platform_winit::run(ui, form)
}
