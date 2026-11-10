//! O editor de código com um fonte enorme: o `mod.rs` gerado do ERP repetido até o número
//! de linhas pedido. Mede o primeiro quadro, a roda, o arrasto no minimapa e a digitação
//! (Skia na CPU, sem janela): rolar e navegar não podem depender do tamanho do arquivo.
//!
//! ```text
//! cargo run --release -p aylis-controls --example large_text [linhas]
//! ```

use std::time::Instant;

use aylis_core::*;

const W: u32 = 1600;
const H: u32 = 900;

fn main() {
    let base = std::fs::read_to_string("examples/ERP/rust/generated/mod.rs").unwrap();
    let lines: Vec<&str> = base.lines().collect();
    let wanted: usize = std::env::args().nth(1).and_then(|n| n.parse().ok()).unwrap_or(lines.len());
    let text = (0..wanted).map(|i| lines[i % lines.len()]).collect::<Vec<_>>().join("
");
    let started = Instant::now();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ui.create("Form", None).unwrap();
    ui.set_prop(form, "width", W as f64).unwrap();
    ui.set_prop(form, "height", H as f64).unwrap();
    ui.show(form).unwrap();
    let memo = ui.create("CodeEditor", Some(form)).unwrap();
    ui.set_prop(memo, "align", "client").unwrap();
    ui.set_prop(memo, "syntax", "rust").unwrap();
    ui.set_prop(memo, "text", text).unwrap();
    ui.layout();
    println!("{wanted} linhas: carregar {:.0} ms", started.elapsed().as_secs_f64() * 1000.0);
    let mut pixels = vec![0u32; (W * H) as usize];
    let mut frame = |ui: &mut Ui| {
        let t = Instant::now();
        let damage = ui.take_frame(form).snapped(1.0);
        match damage {
            FrameDamage::Parts { moves, areas } => {
                renderer.render_parts_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
            }
            FrameDamage::Whole => {
                renderer.render_area_to_pixels(&mut pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
            }
        }
        t.elapsed().as_secs_f64() * 1000.0
    };
    // O realce completo (vem aos poucos).
    let mut first = 0.0;
    for i in 0..200 {
        ui.set_time(i as f64);
        let ms = frame(&mut ui);
        if i == 0 {
            first = ms;
        }
    }
    println!("primeiro quadro {first:.1} ms");
    let r = ui.absolute_rect(memo).unwrap();
    let at = PointF::new(r.x + 300.0, r.y + 300.0);
    let (mut wheel, mut paint) = (0.0, 0.0);
    for i in 0..40 {
        let t = Instant::now();
        ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, if i < 30 { -40.0 } else { 40.0 }), ..Default::default() });
        wheel += t.elapsed().as_secs_f64() * 1000.0;
        paint += frame(&mut ui);
    }
    println!("roda: evento {:.2} ms, quadro {:.2} ms", wheel / 40.0, paint / 40.0);
    // Clique e arrasto no minimapa, de cima até embaixo.
    let mx = r.right() - 60.0;
    let (mut ev, mut pt) = (0.0, 0.0);
    ui.pointer_down(form, PointerEvent { position: PointF::new(mx, r.y + 5.0), ..Default::default() });
    let steps = 40;
    for i in 0..=steps {
        let y = r.y + 5.0 + (r.height - 10.0) * i as f32 / steps as f32;
        let t = Instant::now();
        ui.pointer_move(form, PointerEvent { position: PointF::new(mx, y), ..Default::default() });
        ev += t.elapsed().as_secs_f64() * 1000.0;
        pt += frame(&mut ui);
    }
    ui.pointer_up(form, PointerEvent { position: PointF::new(mx, r.bottom() - 5.0), ..Default::default() });
    let top = ui.get_prop(memo, "caret_line").ok();
    println!("minimapa: evento {:.2} ms, quadro {:.2} ms, linha do cursor {top:?}", ev / (steps + 1) as f64, pt / (steps + 1) as f64);
    // Digitar no meio do texto.
    ui.pointer_down(form, PointerEvent { position: PointF::new(r.x + 400.0, r.y + 200.0), ..Default::default() });
    ui.pointer_up(form, PointerEvent { position: PointF::new(r.x + 400.0, r.y + 200.0), ..Default::default() });
    frame(&mut ui);
    let (mut ev, mut pt) = (0.0, 0.0);
    for _ in 0..20 {
        let t = Instant::now();
        ui.text_input(form, "a");
        ev += t.elapsed().as_secs_f64() * 1000.0;
        pt += frame(&mut ui);
    }
    println!("digitar: evento {:.2} ms, quadro {:.2} ms", ev / 20.0, pt / 20.0);
}
