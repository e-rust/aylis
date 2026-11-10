//! Medição da rolagem do editor de código na GPU (o Skia com OpenGL, como a IDE no Windows):
//! repintando a janela inteira e só a área que mudou. Sem o harness de testes (o winit só cria
//! o laço de janelas na thread principal); só mede com `AYLIS_SCROLL_PROFILE=1`.
//!
//! ```text
//! AYLIS_SCROLL_PROFILE=1 cargo test -p aylis-ide --test scroll_gpu
//! ```

use std::path::PathBuf;
use std::time::Instant;

use aylis_core::*;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

fn stats(name: &str, v: &mut [f64]) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |q: f64| v[((v.len() - 1) as f64 * q) as usize];
    println!("{name:>22}: p50 {:7.2} ms  p90 {:7.2} ms  max {:7.2} ms", at(0.5), at(0.9), at(1.0));
}

fn main() {
    if std::env::var("AYLIS_SCROLL_PROFILE").is_err() {
        return;
    }
    profile_partial_repaint_on_the_gpu();
}

/// A IDE inteira com uma cópia do `ProcessamentoAssincrono` aberta e o `aylis.pas` no editor
/// embutido, em Full HD (lógico): a janela e o editor.
fn ide_with_source(renderer: &aylis_skia::SkiaRenderer) -> (Ui, ComponentId, ComponentId) {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/ProcessamentoAssincrono");
    let copy = std::env::temp_dir().join(format!("aylis_scroll_gpu_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&copy);
    aylis_tools::copy_dir(&source, &copy).unwrap();
    let state = aylis_ide::state::shared(aylis_ide::document::Document::new());
    let ide = aylis_ide::app::build(state.clone());
    let mut ui = ide.ui;
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    aylis_ide::actions::open_recent(&mut ui, &state, &ide.ids, &copy.join("ProcessamentoAssincrono.aylisproj"));
    aylis_ide::app::refresh(&mut ui, &state, &ide.ids);
    aylis_ide::actions::open_code(&mut ui, &state, None);
    aylis_ide::app::refresh(&mut ui, &state, &ide.ids);
    let window = aylis_ide::codewindow::open(&mut ui, &copy.join("pascal/src/aylis.pas"), 1).unwrap();
    let form = ui.root_of(window.form);
    if let Some(cb) = ui.control_mut(form) {
        cb.set_size(1920.0, 1080.0);
    }
    // Como a plataforma faz ao abrir a janela (a área que mudou só vale para um form aberto).
    if !ui.open_forms().contains(&form) {
        let _ = ui.show(form);
    }
    ui.layout();
    (ui, form, window.memo)
}

/// Na GPU (o Skia com OpenGL, como a IDE no Windows): a rolagem repintando a janela inteira e
/// só a área que mudou.


fn profile_partial_repaint_on_the_gpu() {
    aylis_platform_winit::with_offscreen_gl(|gl| {
        let scale = 1.5f32;
        let (pw, ph) = ((1920.0 * scale) as u32, (1080.0 * scale) as u32);
        for partial in [false, true] {
            let renderer = aylis_skia::SkiaRenderer::new();
            let (mut ui, form, memo) = ide_with_source(&renderer);
            let background = ui.theme().color("background");
            let bounds = ui.control(form).unwrap().bounds();
            let r = ui.absolute_rect(memo).unwrap();
            let at = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
            let (mut paint_t, mut partial_frames, mut area_sum) = (Vec::new(), 0usize, 0.0f32);
            let mut now = 10.0;
            for i in 0..240 {
                if i % 8 == 0 {
                    let dy = if i < 160 { -100.0 } else { 100.0 };
                    ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, dy), lines: true, ..Default::default() });
                }
                now += 1.0 / 160.0;
                ui.set_time(now);
                ui.tick();
                let damage = ui.take_damage(form);
                let area = damage.filter(|_| partial && gl.keeps_image(pw, ph)).map(|a| {
                    let (l, t) = ((a.x * scale).floor() / scale, (a.y * scale).floor() / scale);
                    let (rr, b) = ((a.right() * scale).ceil() / scale, (a.bottom() * scale).ceil() / scale);
                    RectF::new(l.max(0.0), t.max(0.0), rr.min(bounds.width) - l.max(0.0), b.min(bounds.height) - t.max(0.0))
                });
                if let Some(a) = area {
                    partial_frames += 1;
                    area_sum += a.width * a.height / (bounds.width * bounds.height);
                }
                let t = Instant::now();
                gl.render_area(&renderer, pw, ph, scale, background, area, |c| ui.paint_region(form, c, area)).unwrap();
                ui.frame_drawn();
                paint_t.push(ms(t));
            }
            println!(
                "\n{}: {} quadros parciais (área média {:.0}% da janela)",
                if partial { "só a área que mudou" } else { "janela inteira" },
                partial_frames,
                if partial_frames > 0 { area_sum * 100.0 / partial_frames as f32 } else { 100.0 }
            );
            stats("pintura (GPU)", &mut paint_t);
            // A conferência: a imagem montada aos pedaços é igual à pintada inteira de uma vez.
            if partial {
                let pieces = gl.read_frame().unwrap();
                gl.render_area(&renderer, pw, ph, scale, background, None, |c| ui.paint_region(form, c, None)).unwrap();
                let whole = gl.read_frame().unwrap();
                let differ = pieces.chunks(4).zip(whole.chunks(4)).filter(|(a, b)| a.iter().zip(b.iter()).any(|(x, y)| x.abs_diff(*y) > 2)).count();
                println!("pixels diferentes da pintura inteira: {differ} de {}", whole.len() / 4);
                assert_eq!(differ, 0, "a repintura parcial deixou restos na imagem");
            }
        }
    })
    .unwrap();
}
