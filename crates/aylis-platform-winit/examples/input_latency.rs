//! A latência da entrada ao pixel (26/11): um quadrado segue o mouse, e uma thread manda os
//! movimentos do mouse **só para a própria janela** (`PostMessageW` com `WM_MOUSEMOVE`, sem o
//! `SendInput`, que iria para a janela em foco, fosse qual fosse). Mede, em cada movimento:
//!
//! - `fila_ms`: de postar a mensagem até o Aylis tratar (a fila do Windows e o laço do winit);
//! - a latência do `Ui` (`AYLIS_LATENCY`): de tratar a entrada até o quadro apresentado
//!   (`Ui::input_latency`), com a pintura e a espera da placa.
//!
//! Não conta o caminho do mouse até a fila (o driver, o USB) nem o compositor e o monitor
//! depois da apresentação: isso só com uma câmera rápida ou um fotodiodo
//! ([testes-em-dispositivos](../../../docs/testes-em-dispositivos.md)).
//!
//! ```text
//! cargo run --release -p aylis-platform-winit --example input_latency --features d2d -- [cenario] [amostras]
//! ```
//!
//! Cenários: `parado` (um movimento a cada 50 ms, a janela dorme entre eles), `rajada` (um a
//! cada 1 ms, um mouse de 1000 Hz) e `animando` (a cada 50 ms, com uma animação contínua na
//! tela). A última linha, `AYLIS_INPUT_LATENCY`, é o JSON do cenário (para
//! `medicoes/latencia.json`). Só no Windows.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    return windows_main::run();
    #[cfg(not(windows))]
    {
        eprintln!("o exemplo input_latency é só do Windows (PostMessageW)");
        Ok(())
    }
}

#[cfg(windows)]
mod windows_main {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    use aylis_controls::*;
    use aylis_core::*;
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, PostMessageW, WM_MOUSEMOVE};
    use windows::core::PCWSTR;

    const TITLE: &str = "aylis — latencia da entrada";

    /// O resumo do cenário, na última linha (`AYLIS_INPUT_LATENCY`).
    fn report(ui: &Ui, name: &str, queue: &[f64]) {
        let mut fila = queue.to_vec();
        fila.sort_by(f64::total_cmp);
        let at = |v: &[f64], q: f64| if v.is_empty() { 0.0 } else { v[((v.len() - 1) as f64 * q).round() as usize] };
        let r = |v: f64| (v * 100.0).round() / 100.0;
        let l = ui.input_latency();
        let json = serde_json::json!({
            "cenario": name,
            "movimentos": fila.len(),
            "fila_p50_ms": r(at(&fila, 0.5)),
            "fila_p95_ms": r(at(&fila, 0.95)),
            "quadros_medidos": l.map_or(0, |l| l.count),
            "entrada_ao_quadro_p50_ms": l.map_or(0.0, |l| r(l.p50_ms)),
            "entrada_ao_quadro_p95_ms": l.map_or(0.0, |l| r(l.p95_ms)),
            "entrada_ao_quadro_p99_ms": l.map_or(0.0, |l| r(l.p99_ms)),
            "entrada_ao_quadro_max_ms": l.map_or(0.0, |l| r(l.max_ms)),
        });
        println!("AYLIS_INPUT_LATENCY {json}");
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut args = std::env::args().skip(1);
        let scenario = args.next().unwrap_or_else(|| "parado".into());
        let samples: usize = args.next().and_then(|n| n.parse().ok()).unwrap_or(300);
        let every = match scenario.as_str() {
            "rajada" => Duration::from_millis(1),
            "parado" | "animando" => Duration::from_millis(50),
            other => return Err(format!("cenário desconhecido: {other} (parado, rajada, animando)").into()),
        };

        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        ui.set_measure_input_latency(true);
        let form = ui.add_root(Form::new(TITLE, 800.0, 600.0));
        let square = ui.add(Rectangle::default(), form);
        ui.control_mut(square).unwrap().set_position(0.0, 0.0).set_size(40.0, 40.0);
        if scenario == "animando" {
            let spinner = ui.add(Rectangle::default(), form);
            ui.control_mut(spinner).unwrap().set_position(600.0, 400.0).set_size(120.0, 120.0);
            let spin = AnimationOptions { duration_ms: 1500.0, easing: Easing::Linear, looping: true, ..Default::default() };
            ui.animate(spinner, "rotation", 359.0, spin)?;
        }

        // Quando cada movimento foi postado (na ordem).
        // Cada movimento postado: a posição (em pixels da janela) e quando.
        let posted: Arc<Mutex<VecDeque<(i32, i32, Instant)>>> = Arc::default();
        let queue: Arc<Mutex<Vec<f64>>> = Arc::default();
        let (p, q) = (posted.clone(), queue.clone());
        let name = scenario.clone();
        let seen = std::cell::Cell::new(0usize);
        ui.on(form, "on_pointer_move", move |ui, _, args| {
            let EventArgs::Pointer(e) = args else { return };
            // O movimento desta posição (os de antes que não viraram evento saem da fila).
            let scale = ui.form_scale(form);
            let (x, y) = ((e.position.x * scale).round() as i32, (e.position.y * scale).round() as i32);
            let mut posted = p.lock().unwrap();
            if let Some(k) = posted.iter().position(|(px, py, _)| (px - x).abs() <= 1 && (py - y).abs() <= 1) {
                let at = posted[k].2;
                posted.drain(..=k);
                q.lock().unwrap().push(at.elapsed().as_secs_f64() * 1000.0);
            }
            drop(posted);
            // O quadrado vai para longe do cursor (o form continua recebendo o movimento).
            if let Some(c) = ui.control_mut(square) {
                c.set_position(e.position.x + 30.0, e.position.y + 30.0);
            }
            seen.set(seen.get() + 1);
            if seen.get() == samples {
                report(ui, &name, &q.lock().unwrap());
                ui.request_close();
            }
        });

        let posted_by_thread = posted.clone();
        std::thread::spawn(move || {
            let title: Vec<u16> = TITLE.encode_utf16().chain([0]).collect();
            // A janela aparece em algum momento: procura pelo título.
            let hwnd = loop {
                // SAFETY: o título é uma string UTF-16 terminada em zero que vive até o fim do laço.
                if let Ok(h) = unsafe { FindWindowW(PCWSTR::null(), PCWSTR(title.as_ptr())) } {
                    if !h.is_invalid() {
                        break h;
                    }
                }
                std::thread::sleep(Duration::from_millis(50));
            };
            std::thread::sleep(Duration::from_millis(500));
            let hwnd = HWND(hwnd.0);
            for i in 0..samples + 50 {
                let (x, y) = (50 + (i * 7) % 600, 50 + (i * 13) % 400);
                posted_by_thread.lock().unwrap().push_back((x as i32, y as i32, Instant::now()));
                // SAFETY: só posta uma mensagem na fila da janela do próprio exemplo.
                if unsafe { PostMessageW(Some(hwnd), WM_MOUSEMOVE, WPARAM(0), LPARAM(((y << 16) | x) as isize)) }.is_err() {
                    return; // a janela fechou
                }
                std::thread::sleep(every);
            }
        });

        aylis_platform_winit::run(ui, form)?;
        Ok(())
    }
}
