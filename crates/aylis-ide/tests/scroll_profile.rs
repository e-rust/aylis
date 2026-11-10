//! Medição da rolagem do editor de código (não roda nos testes comuns): abre o `aylis.pas` do
//! `examples/ProcessamentoAssincrono` na janela de código, rola com a roda quadro a quadro e
//! mede cada etapa (roda, `tick`, pintura) e, no Windows x64, onde o tempo vai (a amostragem
//! da pilha do profiler).
//!
//! ```text
//! cargo test -p aylis-ide --test scroll_profile -- --ignored --nocapture
//! ```

use std::collections::HashMap;
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

#[test]
#[ignore]
fn profile_scrolling_the_code_editor() {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/ProcessamentoAssincrono/pascal/src/aylis.pas");
    // `AYLIS_PROFILE_FULL=1`: a IDE inteira (uma cópia do projeto aberta, o fonte no editor
    // embutido); senão, só a janela de código.
    let full = std::env::var("AYLIS_PROFILE_FULL").is_ok();
    let (mut ui, ide_form, file) = if full {
        let copy = std::env::temp_dir().join(format!("aylis_scroll_profile_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&copy);
        aylis_tools::copy_dir(&file.parent().unwrap().join("../.."), &copy).unwrap();
        let state = aylis_ide::state::shared(aylis_ide::document::Document::new());
        let ide = aylis_ide::app::build(state.clone());
        let mut ui = ide.ui;
        aylis_ide::actions::open_recent(&mut ui, &state, &ide.ids, &copy.join("ProcessamentoAssincrono.aylisproj"));
        aylis_ide::app::refresh(&mut ui, &state, &ide.ids);
        // O F12: o código no centro da IDE (o editor embutido), onde o fonte vai abrir.
        aylis_ide::actions::open_code(&mut ui, &state, None);
        aylis_ide::app::refresh(&mut ui, &state, &ide.ids);
        (ui, Some(ide.form), copy.join("pascal/src/aylis.pas"))
    } else {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        (ui, None, file)
    };
    // `AYLIS_PROFILE_D2D=1`: o Direct2D (o padrão da IDE no Windows); senão, o Skia na CPU.
    #[cfg(windows)]
    let d2d = std::env::var("AYLIS_PROFILE_D2D").is_ok().then(|| aylis_d2d::D2dRenderer::new().unwrap());
    #[cfg(not(windows))]
    let d2d: Option<()> = None;
    let renderer = aylis_skia::SkiaRenderer::new();
    #[cfg(windows)]
    if let Some(d) = &d2d {
        ui.services_mut().text = Some(Box::new(d.text_service()));
    }
    if d2d.is_none() {
        ui.services_mut().text = Some(Box::new(renderer.text_service()));
    }
    println!("renderizador: {}", if d2d.is_some() { "Direct2D" } else { "Skia CPU" });
    // A janela da IDE em Full HD (lógico), escala 150%: 2880 x 1620 pixels.
    let (w, h, scale) = (1920.0f32, 1080.0f32, 1.5f32);
    if ide_form.is_none() {
        let main = ui.add_root(aylis_controls::Form::new("IDE", w, h));
        ui.show(main).unwrap();
    }
    let window = aylis_ide::codewindow::open(&mut ui, &file, 1).unwrap();
    // A janela inteira (o editor embutido devolve o painel do centro da IDE).
    let form = ui.root_of(window.form);
    println!("{}", if Some(form) == ide_form { "editor embutido na IDE inteira" } else { "janela de código sozinha" });
    if let Some(cb) = ui.control_mut(form) {
        cb.set_size(w, h);
    }
    ui.layout();
    let (pw, ph) = ((w * scale) as u32, (h * scale) as u32);
    let mut pixels = vec![0u32; (pw * ph) as usize];
    let background = ui.theme().color("background");
    let mut frame = |ui: &mut Ui| {
        #[cfg(windows)]
        if let Some(d) = &d2d {
            return d.render_rgba(pw, ph, scale, background, |c| ui.paint(form, c)).is_some();
        }
        renderer.render_to_pixels(&mut pixels, pw, ph, scale, background, |c| ui.paint(form, c))
    };
    // Aquece (a primeira pintura mede e colore o texto).
    let warm = Instant::now();
    frame(&mut ui);
    println!("primeira pintura: {:.1} ms", ms(warm));

    let memo_rect = ui.absolute_rect(window.memo).unwrap();
    let at = PointF::new(memo_rect.x + memo_rect.width / 2.0, memo_rect.y + memo_rect.height / 2.0);
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        aylis_core::profile::start();
        aylis_platform_winit::sampler::start();
    }
    let (mut wheel_t, mut tick_t, mut paint_t, mut total_t) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut now = 10.0;
    for i in 0..240 {
        let start = Instant::now();
        // Um dente a cada 8 quadros (uma roda girando), rolando para baixo e depois para cima.
        if i % 8 == 0 {
            let t = Instant::now();
            let dy = if i < 160 { -100.0 } else { 100.0 };
            ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, dy), lines: true, ..Default::default() });
            wheel_t.push(ms(t));
        }
        now += 1.0 / 160.0;
        ui.set_time(now);
        let t = Instant::now();
        ui.tick();
        tick_t.push(ms(t));
        let t = Instant::now();
        frame(&mut ui);
        ui.frame_drawn();
        paint_t.push(ms(t));
        total_t.push(ms(start));
    }
    #[cfg(all(windows, target_arch = "x86_64"))]
    aylis_platform_winit::sampler::stop();
    println!("deslize: {}", ui.smooth_scrolling());
    stats("roda", &mut wheel_t);
    stats("tick", &mut tick_t);
    stats("pintura", &mut paint_t);
    stats("quadro inteiro", &mut total_t);

    // Onde o tempo vai: as funções mais vistas nas amostras (a própria e a que inclui).
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let stacks = aylis_core::profile::take_stacks();
        aylis_core::profile::stop();
        let mut own: HashMap<String, usize> = HashMap::new();
        let mut inclusive: HashMap<String, usize> = HashMap::new();
        for s in &stacks {
            if let Some(top) = s.frames.last() {
                *own.entry(top.clone()).or_default() += 1;
            }
            let mut seen = std::collections::HashSet::new();
            for f in &s.frames {
                if seen.insert(f.clone()) {
                    *inclusive.entry(f.clone()).or_default() += 1;
                }
            }
        }
        let n = stacks.len().max(1) as f64;
        let mut top: Vec<_> = own.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1));
        println!("\n{} amostras. Tempo próprio:", stacks.len());
        for (f, c) in top.iter().take(25) {
            println!("{:5.1}%  {f}", *c as f64 * 100.0 / n);
        }
        let mut incl: Vec<_> = inclusive.into_iter().filter(|(f, _)| f.contains("aylis")).collect();
        incl.sort_by(|a, b| b.1.cmp(&a.1));
        println!("\nTempo incluindo as chamadas (só do aylis):");
        for (f, c) in incl.iter().take(40) {
            println!("{:5.1}%  {f}", *c as f64 * 100.0 / n);
        }
    }
}

