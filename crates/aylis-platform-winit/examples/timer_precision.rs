//! Precisão do Timer: um timer de 10 ms passa por `normal`, `high`, `exact` e `high` numa
//! thread própria (`background`), 200 disparos em cada, e mostra o atraso (`drift`) médio e
//! o pior. `AYLIS_AUTO_CLOSE=1` imprime e fecha no fim.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example timer_precision --features cpu --release
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::*;
use aylis_core::*;

const TICKS: usize = 200;
const MODES: [(TimerPrecision, bool); 4] = [
    (TimerPrecision::Normal, false),
    (TimerPrecision::High, false),
    (TimerPrecision::Exact, false),
    (TimerPrecision::High, true),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("Precisão do Timer", 460.0, 220.0));
    let label = ui.add(Label::new("Medindo..."), form);
    ui.control_mut(label).unwrap().set_align(Align::Client);
    let timer = ui.add(Timer::new(10), form);
    let state = Rc::new(RefCell::new((0usize, Vec::<f64>::new(), String::new())));
    ui.on(timer, "on_timer", move |ui, _, _| {
        let drift = ui.get::<Timer>(timer).map(|t| t.drift()).unwrap_or(0.0);
        let mut s = state.borrow_mut();
        s.1.push(drift);
        if s.1.len() < TICKS {
            return;
        }
        let (precision, background) = MODES[s.0];
        // O primeiro disparo depois da troca ainda vem do agendamento anterior.
        let samples = &s.1[1..];
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let worst = samples.iter().copied().fold(0.0, f64::max);
        let line = format!(
            "{:<7}{:<12} médio {mean:6.3} ms   pior {worst:6.3} ms\n",
            precision.name(),
            if background { " (thread)" } else { "" }
        );
        print!("{line}");
        s.2.push_str(&line);
        let _ = ui.set_prop(label, "text", s.2.clone());
        s.1.clear();
        s.0 += 1;
        match MODES.get(s.0) {
            Some(&(precision, background)) => {
                let _ = ui.set_prop(timer, "precision", precision.name());
                let _ = ui.set_prop(timer, "background", background);
            }
            None => {
                let _ = ui.set_prop(timer, "enabled", false);
                if std::env::var("AYLIS_AUTO_CLOSE").is_ok() {
                    ui.request_close();
                }
            }
        }
    });
    aylis_platform_winit::run(ui, form)
}
