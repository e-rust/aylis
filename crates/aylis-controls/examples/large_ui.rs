//! O caso grande do item 5: milhares de controles em vários forms abertos, e quanto custa
//! cada quadro nas situações do dia a dia (digitar num Edit, um relógio na barra de status,
//! uma barra de progresso andando, rolar um Memo comprido), pintando só a área que mudou e
//! pintando tudo. Sem janela: o Skia na CPU, num buffer do tamanho de uma tela.
//!
//! ```text
//! cargo run --release -p aylis-controls --example large_ui [forms] [linhas]
//! ```
//!
//! Imprime uma tabela e a linha `AYLIS_LARGE chave=valor...` (para guardar e comparar).

use std::time::Instant;

use aylis_core::*;

const W: u32 = 1280;
const H: u32 = 800;

/// A tela montada: os forms, e no primeiro (o que está na janela) os controles medidos.
pub struct Large {
    pub ui: Ui,
    pub form: ComponentId,
    pub edit: ComponentId,
    pub clock: ComponentId,
    pub progress: ComponentId,
    pub memo: ComponentId,
    /// Um botão da primeira linha (o mouse vai dele para o fundo e volta).
    pub button: ComponentId,
    /// Um painel com 30 rótulos e `cache_layer`, que desliza.
    pub slide: ComponentId,
    pub controls: usize,
}

/// Monta `forms` forms com `rows` linhas de 20 controles cada (rótulos, campos, botões).
pub fn build(forms: usize, rows: usize) -> Large {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let mut first = None;
    let mut parts = None;
    for f in 0..forms {
        let form = ui.create("Form", None).unwrap();
        ui.set_name(form, &format!("frm{f}")).unwrap();
        ui.set_prop(form, "width", W as f64).unwrap();
        ui.set_prop(form, "height", H as f64).unwrap();
        // A barra de status embaixo, com o relógio e a barra de progresso.
        let status = ui.create("Panel", Some(form)).unwrap();
        ui.set_prop(status, "align", "bottom").unwrap();
        ui.set_prop(status, "height", 28.0).unwrap();
        let clock = ui.create("Label", Some(status)).unwrap();
        ui.set_prop(clock, "align", "right").unwrap();
        ui.set_prop(clock, "width", 90.0).unwrap();
        ui.set_prop(clock, "text", "12:00:00").unwrap();
        let progress = ui.create("ProgressBar", Some(status)).unwrap();
        ui.set_prop(progress, "align", "left").unwrap();
        ui.set_prop(progress, "width", 200.0).unwrap();
        // À direita, um Memo comprido.
        let memo = ui.create("Memo", Some(form)).unwrap();
        ui.set_prop(memo, "align", "right").unwrap();
        ui.set_prop(memo, "width", 260.0).unwrap();
        let text: Vec<String> = (0..2000).map(|i| format!("Linha {i} do relatório, com algum texto para medir")).collect();
        ui.set_prop(memo, "text", text.join("\n")).unwrap();
        // O resto: linhas de controles, numa caixa rolável (mais linhas que a altura).
        let scroll = ui.create("ScrollBox", Some(form)).unwrap();
        ui.set_prop(scroll, "align", "client").unwrap();
        let mut edit = None;
        for r in 0..rows {
            let row = ui.create("Panel", Some(scroll)).unwrap();
            ui.set_prop(row, "align", "top").unwrap();
            ui.set_prop(row, "height", 30.0).unwrap();
            for c in 0..20 {
                let class = ["Label", "Edit", "Button", "CheckBox"][c % 4];
                let id = ui.create(class, Some(row)).unwrap();
                ui.set_prop(id, "align", "left").unwrap();
                ui.set_prop(id, "width", 46.0).unwrap();
                let _ = ui.set_prop(id, "text", format!("{r}.{c}"));
                if class == "Edit" && edit.is_none() {
                    edit = Some(id);
                }
            }
        }
        ui.show(form).unwrap();
        if first.is_none() {
            first = Some(form);
            // O painel que desliza (por cima das linhas), com o desenho guardado.
            let slide = ui.create("Panel", Some(form)).unwrap();
            for (prop, value) in [("align", PropValue::from("none")), ("left", 300.0.into()), ("top", 200.0.into()), ("width", 300.0.into()), ("height", 300.0.into()), ("cache_layer", true.into())] {
                ui.set_prop(slide, prop, value).unwrap();
            }
            for i in 0..30 {
                let label = ui.create("Label", Some(slide)).unwrap();
                ui.set_prop(label, "align", "top").unwrap();
                ui.set_prop(label, "height", 10.0).unwrap();
                ui.set_prop(label, "text", format!("Item {i} do painel")).unwrap();
            }
            let button = ui.children(ui.children(scroll)[0]).iter().copied().find(|&c| ui.class_name(c) == Some("Button")).unwrap();
            parts = Some((edit.unwrap(), clock, progress, memo, button, slide));
        }
    }
    let form = first.unwrap();
    let (edit, clock, progress, memo, button, slide) = parts.unwrap();
    // A janela é a do primeiro form: ele vai para a frente.
    ui.show(form).unwrap();
    ui.layout();
    let controls = ui.components().count();
    Large { ui, form, edit, clock, progress, memo, button, slide, controls }
}

/// Um quadro como a plataforma faz: a área que mudou (ou tudo) por cima do quadro anterior.
/// Devolve o tempo em ms e a fração da janela repintada.
pub fn frame(large: &mut Large, renderer: &aylis_skia::SkiaRenderer, pixels: &mut [u32], partial: bool) -> (f64, f64) {
    let started = Instant::now();
    let damage = large.ui.take_frame(large.form).snapped(1.0);
    let (ui, form) = (&mut large.ui, large.form);
    // Por parte: as cópias (a rolagem) e as áreas, como a janela faz.
    let fraction = match damage {
        FrameDamage::Parts { moves, areas } if partial => {
            renderer.render_parts_to_pixels(pixels, W, H, 1.0, Color::WHITE, &moves, &areas, |c, a| ui.paint_regions(form, c, a));
            areas.iter().map(|a| (a.width * a.height) as f64).sum::<f64>() / (W * H) as f64
        }
        _ => {
            renderer.render_area_to_pixels(pixels, W, H, 1.0, Color::WHITE, None, |c| ui.paint_region(form, c, None));
            1.0
        }
    };
    (started.elapsed().as_secs_f64() * 1000.0, fraction.min(1.0))
}

/// Os cenários: (nome, o que muda antes de cada quadro).
pub fn scenarios() -> Vec<(&'static str, fn(&mut Large, usize))> {
    vec![
        ("digitar", |l, i| {
            if i == 0 {
                l.ui.set_focus(l.edit);
            }
            l.ui.text_input(l.form, if i % 2 == 0 { "a" } else { "b" });
        }),
        ("relogio", |l, i| {
            let _ = l.ui.set_prop(l.clock, "text", format!("12:00:{:02}", i % 60));
        }),
        ("progresso", |l, i| {
            let _ = l.ui.set_prop(l.progress, "value", (i % 100) as f64);
        }),
        ("mouse", |l, i| {
            // Do botão para o fundo da linha e de volta (o fundo não muda de desenho).
            let r = l.ui.absolute_rect(l.button).unwrap_or_default();
            let at = if i % 2 == 0 { PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0) } else { PointF::new(940.0, r.y + r.height / 2.0) };
            l.ui.pointer_move(l.form, PointerEvent { position: at, ..Default::default() });
            l.ui.set_time(l.ui.time() + 1.0);
        }),
        ("deslizar", |l, i| {
            // Um painel com 30 rótulos andando de lado (a camada guardada é recomposta).
            let _ = l.ui.set_prop(l.slide, "translate_x", ((i % 20) * 8) as f64);
        }),
        ("rolar", |l, i| {
            let r = l.ui.absolute_rect(l.memo).unwrap_or_default();
            let at = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
            let delta = if (i / 20) % 2 == 0 { -40.0 } else { 40.0 };
            l.ui.wheel(l.form, WheelEvent { position: at, delta: PointF::new(0.0, delta), ..Default::default() });
            // O Memo desliza a roda pelos quadros: anda até parar.
            l.ui.set_time(l.ui.time() + 1.0);
        }),
    ]
}

/// Mede cada cenário (`frames` quadros) por parte e inteiro. Devolve (nome, ms por parte,
/// ms inteiro, fração da janela repintada).
pub fn measure(large: &mut Large, frames: usize) -> Vec<(&'static str, f64, f64, f64)> {
    let renderer = aylis_skia::SkiaRenderer::new();
    let mut pixels = vec![0u32; (W * H) as usize];
    // O primeiro quadro é inteiro.
    frame(large, &renderer, &mut pixels, false);
    let mut out = Vec::new();
    for (name, change) in scenarios() {
        let mut times = [0.0f64; 2];
        let mut covered = 0.0;
        for (k, partial) in [true, false].into_iter().enumerate() {
            for i in 0..frames {
                change(large, i);
                let (ms, fraction) = frame(large, &renderer, &mut pixels, partial);
                times[k] += ms;
                if partial {
                    covered += fraction;
                }
            }
        }
        out.push((name, times[0] / frames as f64, times[1] / frames as f64, covered / frames as f64));
    }
    out
}

#[allow(dead_code)]
fn main() {
    let args: Vec<usize> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let forms = args.first().copied().unwrap_or(8);
    let rows = args.get(1).copied().unwrap_or(40);
    let started = Instant::now();
    let mut large = build(forms, rows);
    let built = started.elapsed().as_secs_f64() * 1000.0;
    // O layout de todos os forms (o que um `invalidate()` sem controle custa).
    let started = Instant::now();
    large.ui.layout();
    let layout_all = started.elapsed().as_secs_f64() * 1000.0;
    // Achar pelo nome (o índice do item 6).
    let started = Instant::now();
    for f in 0..forms {
        let _ = large.ui.find_path(&format!("frm{f}"));
    }
    let find = started.elapsed().as_secs_f64() * 1000.0 / forms as f64;
    println!("{} controles em {forms} forms: montar {built:.0} ms, layout de todos {layout_all:.1} ms, achar um form {find:.4} ms", large.controls);
    println!("{:12} {:>12} {:>12} {:>10}", "cenário", "por parte", "inteiro", "área");
    let mut line = format!("AYLIS_LARGE controles={} layout_ms={layout_all:.2}", large.controls);
    for (name, partial, full, area) in measure(&mut large, 60) {
        println!("{name:12} {partial:9.2} ms {full:9.2} ms {:9.1}%", area * 100.0);
        line.push_str(&format!(" {name}_parte_ms={partial:.2} {name}_inteiro_ms={full:.2} {name}_area={:.3}", area));
    }
    println!("{line}");
}
