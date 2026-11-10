//! Medições do editor de código embutido na IDE: quanto custa cada passo da navegação
//! (setas, Page Down, roda do mouse) num fonte grande, no tratamento do evento e na pintura.
//! Rode à mão: `cargo test -p aylis-ide --release --test perf -- --ignored --nocapture`.

use std::time::{Duration, Instant};

use aylis_core::{KeyEvent, PointF, Ui, WheelEvent};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};

/// Um fonte grande (3 mil linhas de Rust).
fn big_source(dir: &std::path::Path) -> std::path::PathBuf {
    let mut text = String::new();
    for i in 0..300 {
        text.push_str(&format!(
            "/// Função {i}.\npub fn funcao_{i}(valor: u32, nome: &str) -> Result<String, String> {{\n    let total = valor * 2 + {i};\n    if total > 100 {{\n        return Err(format!(\"grande: {{nome}} {{total}}\"));\n    }}\n    let lista: Vec<u32> = (0..total).map(|x| x * 3).collect();\n    Ok(format!(\"{{}} itens\", lista.len()))\n}}\n\n"
        ));
    }
    let file = dir.join("src").join("grande.rs");
    std::fs::write(&file, text).unwrap();
    file
}

fn key(ui: &mut Ui, root: aylis_core::ComponentId, name: &str) {
    let (modifiers, key) = aylis_core::parse_shortcut(name).unwrap();
    let e = KeyEvent { key, modifiers, repeat: false };
    ui.key_down(root, e);
    ui.key_up(root, e);
}

#[test]
#[ignore]
fn code_editor_navigation_cost() {
    let base = std::env::temp_dir().join(format!("aylis_perf_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Medir", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let file = big_source(&project.code_dir());

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    let w = aylis_ide::codewindow::open(&mut ide.ui, &file, 1).unwrap();
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let form = ide.form;
    let (width, height) = (1600u32, 1000u32);
    ide.ui.control_mut(form).unwrap().set_width(width as f32).set_height(height as f32);

    let paint = |ui: &mut Ui| {
        let t = Instant::now();
        ui.layout();
        let background = ui.theme().color("background");
        let _ = renderer.render_rgba(width, height, 1.0, background, |c| ui.paint(form, c));
        t.elapsed()
    };
    paint(&mut ide.ui);

    let measure = |ui: &mut Ui, what: &str, n: usize, step: &dyn Fn(&mut Ui)| {
        let (mut events, mut frames) = (Duration::ZERO, Duration::ZERO);
        for _ in 0..n {
            let t = Instant::now();
            step(ui);
            events += t.elapsed();
            frames += paint(ui);
        }
        eprintln!("{what:<22} evento {:>8.2?}   pintura {:>8.2?}   (média de {n})", events / n as u32, frames / n as u32);
    };
    measure(&mut ide.ui, "seta para baixo", 60, &|ui| key(ui, form, "Down"));
    measure(&mut ide.ui, "seta para a direita", 60, &|ui| key(ui, form, "Right"));
    measure(&mut ide.ui, "Page Down", 30, &|ui| key(ui, form, "PageDown"));
    let at = ui_center(&ide.ui, w.memo);
    measure(&mut ide.ui, "roda do mouse", 40, &|ui| ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, -120.0), ..Default::default() }));
    measure(&mut ide.ui, "digitar", 30, &|ui| ui.text_input(form, "x"));
    // O que a IDE faz quando algo muda (a bomba chama a cada resposta do servidor de linguagem).
    measure(&mut ide.ui, "refresh da IDE", 20, &|ui| app::refresh(ui, &state, &ide.ids));

    // Só o editor (o form dele), com e sem o minimapa.
    let editor = w.memo;
    let r = ide.ui.absolute_rect(editor).unwrap();
    let only = |ui: &mut Ui, what: &str, n: usize, step: &dyn Fn(&mut Ui)| {
        let mut total = Duration::ZERO;
        for _ in 0..n {
            step(ui);
            ui.layout();
            let t = Instant::now();
            let background = ui.theme().color("background");
            let _ = renderer.render_rgba(r.width as u32, r.height as u32, 1.0, background, |c| ui.paint(editor, c));
            total += t.elapsed();
        }
        eprintln!("  só o editor: {what:<28} {:>8.2?}", total / n as u32);
    };
    for minimap in [true, false] {
        let _ = ide.ui.set_prop(editor, "minimap", minimap);
        let tag = if minimap { "com minimapa" } else { "sem minimapa" };
        only(&mut ide.ui, &format!("seta ({tag})"), 30, &|ui| key(ui, form, "Down"));
        only(&mut ide.ui, &format!("digitar ({tag})"), 15, &|ui| ui.text_input(form, "x"));
    }
    let _ = std::fs::remove_dir_all(base);
}

/// A paleta de componentes: refazer o painel, pintar e rolar a lista.
#[test]
#[ignore]
fn palette_cost() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    state.borrow_mut().home = false;
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let form = ide.form;
    ide.ui.control_mut(form).unwrap().set_width(1600.0).set_height(1000.0);
    ide.ui.layout();
    let rows = ide.ui.children(ide.ids.palette).len();
    let time = |n: usize, f: &mut dyn FnMut()| {
        let t = Instant::now();
        for _ in 0..n {
            f();
        }
        t.elapsed() / n as u32
    };
    let ids = ide.ids;
    let rebuild = time(20, &mut || aylis_ide::panels::palette(&mut ide.ui, &state, &ids));
    let refresh = time(20, &mut || app::refresh(&mut ide.ui, &state, &ids));
    ide.ui.layout();
    let palette = ids.palette;
    let r = ide.ui.absolute_rect(palette).unwrap();
    let paint = time(20, &mut || {
        let background = ide.ui.theme().color("background");
        let _ = renderer.render_rgba(r.width as u32, r.height as u32, 1.0, background, |c| ide.ui.paint(palette, c));
    });
    let at = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
    let wheel = time(20, &mut || {
        ide.ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, -120.0), ..Default::default() });
        ide.ui.layout();
    });
    eprintln!("paleta: {rows} linhas; refazer {rebuild:.2?}; refresh da IDE {refresh:.2?}; pintar {paint:.2?}; rolar (evento + layout) {wheel:.2?}");
    // As partes do refresh.
    let parts: [(&str, &dyn Fn(&mut Ui)); 7] = [
        ("estrutura", &|ui| aylis_ide::panels::structure(ui, &state, &ids)),
        ("projeto", &|ui| aylis_ide::panels::project(ui, &state, &ids)),
        ("abas", &|ui| aylis_ide::panels::tabs(ui, &state, &ids)),
        ("inspector", &|ui| aylis_ide::inspector::rebuild(ui, &state, &ids)),
        ("mensagens", &|ui| aylis_ide::panels::messages(ui, &state, &ids)),
        ("traduzir", &|ui| aylis_ide::lang::translate(ui)),
        ("layout", &|ui| ui.layout()),
    ];
    for (name, f) in parts {
        let t = Instant::now();
        for _ in 0..20 {
            f(&mut ide.ui);
        }
        eprintln!("  refresh, {name}: {:.2?}", t.elapsed() / 20);
    }
}

fn ui_center(ui: &Ui, id: aylis_core::ComponentId) -> PointF {
    let r = ui.absolute_rect(id).unwrap();
    PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0)
}

/// A IDE parada: quantas vezes acorda e pinta em 10 segundos sem ninguém mexer.
#[test]
#[ignore]
fn idle_ide_wakeups() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ide.form;
    ide.ui.show(form).unwrap();
    ide.ui.control_mut(form).unwrap().set_width(1600.0).set_height(1000.0);
    let (mut t, end) = (1.0f64, 11.0f64);
    let (mut wakes, mut frames) = (0, 0);
    let mut reasons: std::collections::BTreeMap<String, usize> = Default::default();
    while t < end {
        ide.ui.set_time(t);
        wakes += 1;
        ide.ui.run_timers();
        ide.ui.process_posted();
        // Como a plataforma: o horário pedido chegou, ou algo mudou: um quadro.
        let due = ide.ui.next_wake().is_some_and(|at| at <= t + 0.001);
        if ide.ui.needs_repaint() || due {
            frames += 1;
            let why = format!("quadro, animando={}", ide.ui.is_animating());
            *reasons.entry(why).or_default() += 1;
            ide.ui.layout();
            let background = ide.ui.theme().color("background");
            let _ = renderer.render_rgba(160, 100, 1.0, background, |c| ide.ui.paint(form, c));
            ide.ui.frame_drawn();
        }
        let next = if ide.ui.needs_repaint() { t + 1.0 / 60.0 } else { ide.ui.next_wake().unwrap_or(end).max(t + 0.001) };
        *reasons.entry(format!("intervalo {:.3}s", next - t)).or_default() += 1;
        t = next;
    }
    eprintln!("10 s parada: {wakes} vezes acordou, {frames} quadros; {reasons:?}");
}

/// A galeria de componentes: pintar, rolar e se ela fica pedindo quadros parada.
#[test]
#[ignore]
fn gallery_cost() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let ids = ide.ids;
    let t = Instant::now();
    let dialog = aylis_ide::gallery::open(&mut ide.ui, &state, &ids);
    ide.ui.layout();
    eprintln!("galeria: abrir {:.2?}, {} componentes no total", t.elapsed(), ide.ui.registry().iter().count());
    let r = ide.ui.absolute_rect(dialog).unwrap();
    let (w, h) = (r.width as u32, r.height as u32);
    let mut now = 1.0;
    let mut paint = |ui: &mut Ui| {
        now += 1.0 / 60.0;
        ui.set_time(now);
        ui.tick();
        let t = Instant::now();
        let background = ui.theme().color("background");
        let _ = renderer.render_rgba(w, h, 1.0, background, |c| ui.paint(dialog, c));
        ui.frame_drawn();
        t.elapsed()
    };
    let first = paint(&mut ide.ui);
    let still = paint(&mut ide.ui);
    eprintln!("pintar: primeira {first:.2?}, de novo {still:.2?}; parada pede quadro? {} (animando {}, próximo {:?})", ide.ui.needs_repaint(), ide.ui.is_animating(), ide.ui.next_paint_wake().map(|a| a - ide.ui.time()));
    let at = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
    let mut total = Duration::ZERO;
    for _ in 0..20 {
        let t = Instant::now();
        ide.ui.wheel(dialog, WheelEvent { position: at, delta: PointF::new(0.0, -120.0), ..Default::default() });
        ide.ui.layout();
        total += t.elapsed() + paint(&mut ide.ui);
    }
    eprintln!("rolar (evento + layout + pintura): {:.2?} por passo", total / 20);
    if let Ok(dir) = std::env::var("AYLIS_SHOTS") {
        let background = ide.ui.theme().color("background");
        let png = renderer.render_png(w, h, 1.0, background, |c| ide.ui.paint(dialog, c)).unwrap();
        std::fs::write(std::path::Path::new(&dir).join("galeria_rolada.png"), png).unwrap();
    }
}

/// A IDE com o ERP de exemplo (item 21): abrir o projeto, abrir 20 forms um depois do outro
/// (cada um com o refresh e a pintura da IDE), a busca no projeto e o "onde é usado" (item 20),
/// e a memória do processo antes e depois. Rode em release:
/// `cargo test -p aylis-ide --release --test perf -- --ignored --nocapture --exact erp_ide_cost`
/// (sozinho: a memória é a do processo).
#[test]
#[ignore]
fn erp_ide_cost() {
    let mb = |b: usize| b as f64 / 1_048_576.0;
    let (private0, ws0) = aylis_platform_winit::process_memory();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ide.form;
    ide.ui.show(form).unwrap();
    ide.ui.control_mut(form).unwrap().set_width(1600.0).set_height(1000.0);
    let ids = ide.ids.clone();
    let paint = |ui: &mut Ui| {
        app::refresh(ui, &state, &ids);
        ui.layout();
        let background = ui.theme().color("background");
        let _ = renderer.render_rgba(1600, 1000, 1.0, background, |c| ui.paint(form, c));
        ui.frame_drawn();
    };
    paint(&mut ide.ui);
    let (private1, ws1) = aylis_platform_winit::process_memory();

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/ERP/ERP.aylisproj");
    let t = Instant::now();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &path);
    paint(&mut ide.ui);
    let open = t.elapsed();
    let (private2, ws2) = aylis_platform_winit::process_memory();

    let project = state.borrow().project.clone().unwrap();
    let names: Vec<String> = project.forms.iter().skip(1).step_by(50).take(20).map(|f| f.name.clone()).collect();
    let mut each = Vec::new();
    let (mut opening, mut refreshing) = (Duration::ZERO, Duration::ZERO);
    for name in &names {
        let t = Instant::now();
        actions::open_project_form(&state, name);
        opening += t.elapsed();
        let r = Instant::now();
        app::refresh(&mut ide.ui, &state, &ids);
        refreshing += r.elapsed();
        paint(&mut ide.ui);
        each.push(t.elapsed());
    }
    eprintln!("por form: abrir o documento {:.2?}, refresh da IDE {:.2?}", opening / names.len() as u32, refreshing / names.len() as u32);
    let tabs = state.borrow().tab_count();
    let (private3, ws3) = aylis_platform_winit::process_memory();
    let total: Duration = each.iter().sum();
    let slowest = each.iter().max().copied().unwrap_or_default();

    // A busca de texto no projeto inteiro (forms e fontes) e o "onde é usado" de um dataset.
    let t = Instant::now();
    let form_files: Vec<std::path::PathBuf> = project.forms.iter().map(|f| project.file(&f.file)).collect();
    let files = aylis_ide::projectsearch::files(&project.dir(), "", "", &form_files);
    let forms: Vec<(String, std::path::PathBuf)> = project.forms.iter().map(|f| (f.name.clone(), project.file(&f.file))).collect();
    let options = aylis_controls::search::SearchOptions { query: "cdsMestre".into(), case: true, whole: true, ..Default::default() };
    let found = aylis_ide::projectsearch::search(&options, &files, &forms).unwrap();
    let search = t.elapsed();
    let t = Instant::now();
    let target = aylis_ide::usages::Target::Component { form: project.main_form.clone(), name: "cdsTelas".into() };
    let usages = aylis_ide::usages::find(&project, &target).unwrap();
    let usage = t.elapsed();

    eprintln!("memória (privada / working set, MB): IDE vazia {:.0} / {:.0}; antes de construir {:.0} / {:.0}", mb(private1), mb(ws1), mb(private0), mb(ws0));
    eprintln!("abrir o ERP e desenhar: {open:.2?}; memória {:.0} / {:.0} MB", mb(private2), mb(ws2));
    eprintln!(
        "{} forms abertos ({tabs} abas): {:.2?} no total, {:.2?} em média, o mais lento {slowest:.2?}; memória {:.0} / {:.0} MB ({:.1} MB por form)",
        names.len(),
        total,
        total / names.len() as u32,
        mb(private3),
        mb(ws3),
        mb(private3.saturating_sub(private2)) / names.len() as f64
    );
    eprintln!("busca \"cdsMestre\" no projeto ({} fontes, {} forms): {search:.2?}, {} achados", files.len(), forms.len(), found.forms.len() + found.text.len());
    eprintln!("onde é usado (cdsTelas): {usage:.2?}, {} achados", usages.forms.len() + usages.text.len());
    // Os limites (em release, com folga sobre o medido em 07/11: medicoes/erp_1000_forms.json).
    if !cfg!(debug_assertions) {
        assert!(open < Duration::from_secs(2), "abrir o ERP: {open:?}");
        assert!(opening / names.len() as u32 <= Duration::from_millis(50), "abrir o documento de um form: {:?}", opening / names.len() as u32);
        assert!(total / names.len() as u32 <= Duration::from_millis(300), "abrir um form com o refresh e a pintura: {:?}", total / names.len() as u32);
        assert!(private3.saturating_sub(private2) / names.len() < 10 << 20, "memória por form aberto");
    }
}

/// O ERP com os 1009 forms abertos em abas ao mesmo tempo: quanto custa abrir todos, a memória,
/// trocar de aba (pela faixa de abas, como o clique) e ir do Design para o Código e voltar
/// (os botões Design | Código). Rode em release:
/// `cargo test -p aylis-ide --release --test perf -- --ignored --nocapture --exact erp_all_forms_in_tabs`
/// (sozinho: a memória é a do processo, e outro teste rodando junto entra na conta).
#[test]
#[ignore]
fn erp_all_forms_in_tabs() {
    let mb = |b: usize| b as f64 / 1_048_576.0;
    let ms = |d: Duration| d.as_secs_f64() * 1000.0;
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ide.form;
    ide.ui.show(form).unwrap();
    ide.ui.control_mut(form).unwrap().set_width(1600.0).set_height(1000.0);
    let ids = ide.ids.clone();
    // Um quadro como o da plataforma: o layout pendente e a pintura da janela inteira.
    let frame = |ui: &mut Ui| {
        ui.process_posted();
        ui.layout();
        let background = ui.theme().color("background");
        let _ = renderer.render_rgba(1600, 1000, 1.0, background, |c| ui.paint(form, c));
        ui.frame_drawn();
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/ERP/ERP.aylisproj");
    actions::open_project(&mut ide.ui, &state, &ide.ids, &path);
    app::refresh(&mut ide.ui, &state, &ids);
    frame(&mut ide.ui);
    let (private0, _) = aylis_platform_winit::process_memory();

    // Abrir todos (o refresh e o quadro a cada 100, como quem abre um depois do outro).
    let project = state.borrow().project.clone().unwrap();
    let t = Instant::now();
    for (i, entry) in project.forms.iter().enumerate() {
        actions::open_project_form(&state, &entry.name);
        if i % 100 == 99 {
            app::refresh(&mut ide.ui, &state, &ids);
            frame(&mut ide.ui);
        }
    }
    app::refresh(&mut ide.ui, &state, &ids);
    frame(&mut ide.ui);
    let open_all = t.elapsed();
    let tabs = state.borrow().tab_count();
    let (private1, ws1) = aylis_platform_winit::process_memory();
    eprintln!("{tabs} abas abertas em {:.1} s; memória privada {:.0} MB (antes {:.0} MB; {:.2} MB por aba), working set {:.0} MB", open_all.as_secs_f64(), mb(private1), mb(private0), mb(private1.saturating_sub(private0)) / tabs as f64, mb(ws1));

    // Trocar de aba pela faixa (o on_change do TabStrip, que faz o switch_to e o refresh).
    let strip = ide.ui.children(ids.tabs).iter().copied().find(|&c| ide.ui.get::<aylis_controls::TabStrip>(c).is_some()).expect("a faixa de abas");
    let picks: Vec<usize> = (0..40).map(|k| (k * 397 + 13) % tabs).collect();
    let mut switch = Vec::new();
    for &i in &picks {
        let t = Instant::now();
        ide.ui.fire(strip, "on_change", &aylis_core::EventArgs::Value(aylis_core::PropValue::Int(i as i64)));
        frame(&mut ide.ui);
        switch.push(t.elapsed());
        assert_eq!(state.borrow().active, i, "a aba {i} ficou ativa");
    }

    // Design → Código → Design, nos botões, em 20 abas.
    let (mut to_code, mut to_design) = (Vec::new(), Vec::new());
    for &i in picks.iter().take(20) {
        ide.ui.fire(strip, "on_change", &aylis_core::EventArgs::Value(aylis_core::PropValue::Int(i as i64)));
        frame(&mut ide.ui);
        let t = Instant::now();
        ide.ui.click(ids.view_code);
        frame(&mut ide.ui);
        to_code.push(t.elapsed());
        assert!(state.borrow().code_view, "a aba {i} mostrou o código");
        let t = Instant::now();
        ide.ui.click(ids.view_design);
        frame(&mut ide.ui);
        to_design.push(t.elapsed());
        assert!(!state.borrow().code_view);
    }
    let (private2, ws2) = aylis_platform_winit::process_memory();
    let stats = |v: &[Duration]| {
        let mut s: Vec<f64> = v.iter().map(|d| ms(*d)).collect();
        s.sort_by(f64::total_cmp);
        format!("mediana {:.1} ms, pior {:.1} ms, primeira {:.1} ms", s[s.len() / 2], s[s.len() - 1], ms(v[0]))
    };
    eprintln!("trocar de aba ({} trocas): {}", switch.len(), stats(&switch));
    // As partes de uma troca: o evento (switch_to e refresh), o layout e a pintura.
    let (mut event, mut layout, mut paint) = (Duration::ZERO, Duration::ZERO, Duration::ZERO);
    for &i in picks.iter().take(10) {
        let t = Instant::now();
        ide.ui.fire(strip, "on_change", &aylis_core::EventArgs::Value(aylis_core::PropValue::Int(i as i64)));
        ide.ui.process_posted();
        event += t.elapsed();
        let t = Instant::now();
        ide.ui.layout();
        layout += t.elapsed();
        let t = Instant::now();
        let background = ide.ui.theme().color("background");
        let _ = renderer.render_rgba(1600, 1000, 1.0, background, |c| ide.ui.paint(form, c));
        ide.ui.frame_drawn();
        paint += t.elapsed();
    }
    eprintln!("uma troca: evento {:.1} ms, layout {:.1} ms, pintura {:.1} ms", ms(event) / 10.0, ms(layout) / 10.0, ms(paint) / 10.0);
    let t = Instant::now();
    for _ in 0..10 {
        app::refresh(&mut ide.ui, &state, &ids);
    }
    eprintln!("só o refresh da IDE: {:.1} ms", ms(t.elapsed()) / 10.0);
    eprintln!("Design → Código: {}", stats(&to_code));
    eprintln!("Código → Design: {}", stats(&to_design));
    eprintln!("memória no fim: privada {:.0} MB, working set {:.0} MB", mb(private2), mb(ws2));
    // Os limites (em release, com folga sobre o medido em 07/11: medicoes/erp_1000_forms.json).
    if !cfg!(debug_assertions) {
        let median = |v: &[Duration]| {
            let mut s = v.to_vec();
            s.sort();
            s[s.len() / 2]
        };
        assert!(median(&switch) < Duration::from_millis(100), "trocar de aba: {:?}", median(&switch));
        assert!(median(&to_code) < Duration::from_millis(200), "Design → Código: {:?}", median(&to_code));
        assert!(median(&to_design) < Duration::from_millis(100), "Código → Design: {:?}", median(&to_design));
        assert!(private1.saturating_sub(private0) / tabs < 600 << 10, "memória por aba: {:.2} MB", mb(private1.saturating_sub(private0)) / tabs as f64);
    }
}

/// De onde vem a memória de cada aba: um `Ui` vazio com as classes registradas, um
/// `Document` vazio e um form do ERP aberto (100 de cada).
#[test]
#[ignore]
fn memory_per_document() {
    let mb = |b: usize| b as f64 / 1_048_576.0;
    let measure = |what: &str, make: &dyn Fn() -> Box<dyn std::any::Any>| {
        let (before, _) = aylis_platform_winit::process_memory();
        let kept: Vec<_> = (0..100).map(|_| make()).collect();
        let (after, _) = aylis_platform_winit::process_memory();
        eprintln!("{what}: {:.2} MB cada", mb(after.saturating_sub(before)) / 100.0);
        drop(kept);
    };
    measure("Theme::light", &|| Box::new(aylis_core::Theme::light()));
    measure("Services::default", &|| Box::new(aylis_core::platform::Services::default()));
    measure("Ui::new", &|| Box::new(Ui::new()));
    measure("Ui com as classes", &|| {
        let mut ui = Ui::new();
        aylis_controls::register(&mut ui);
        Box::new(ui)
    });
    measure("Document::new", &|| Box::new(Document::new()));
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/ERP/ERP.aylisproj");
    let project = aylis_project::Project::open(&path).unwrap();
    let name = project.forms[5].name.clone();
    measure("form do ERP", &|| Box::new(actions::project_document(&project, &name, None).unwrap()));
    let text = std::fs::read_to_string(project.file(&project.forms[5].file)).unwrap();
    measure("form do ERP só pelo JSON (from_json)", &|| Box::new(Document::from_json(&text).unwrap()));
    let mut ui = aylis_ide::document::new_document_ui(false);
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let (before, _) = aylis_platform_winit::process_memory();
    let roots: Vec<_> = (0..100).map(|_| aylis_json::load_value(&mut ui, &value, None).unwrap()).collect();
    let (after, _) = aylis_platform_winit::process_memory();
    eprintln!("o form carregado num Ui só (100 vezes): {:.2} MB cada ({} raízes)", mb(after.saturating_sub(before)) / 100.0, roots.len());
}

/// Selecionar componentes no designer com a Landing aberta (relatado pelo autor em 07/11:
/// "muito lenta a seleção de componente"): a seleção, o `on_change` do designer (que refaz a
/// IDE) e o quadro. Rode em release:
/// `cargo test -p aylis-ide --release --test perf -- --ignored --nocapture --exact landing_selection_cost`.
#[test]
#[ignore]
fn landing_selection_cost() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let form = ide.form;
    ide.ui.show(form).unwrap();
    ide.ui.control_mut(form).unwrap().set_width(1600.0).set_height(1000.0);
    let ids = ide.ids.clone();
    let frame = |ui: &mut Ui| {
        ui.process_posted();
        ui.layout();
        let background = ui.theme().color("background");
        let _ = renderer.render_rgba(1600, 1000, 1.0, background, |c| ui.paint(form, c));
        ui.frame_drawn();
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Landing/Landing.aylisproj");
    actions::open_project(&mut ide.ui, &state, &ide.ids, &path);
    app::refresh(&mut ide.ui, &state, &ids);
    frame(&mut ide.ui);
    let components: Vec<aylis_core::ComponentId> = {
        let st = state.borrow();
        let mut all = vec![st.doc.root];
        let mut i = 0;
        while i < all.len() {
            all.extend(st.doc.ui.children(all[i]).iter().copied());
            i += 1;
        }
        all
    };
    eprintln!("{} componentes no form {}", components.len(), state.borrow().doc.form_name());
    let picks: Vec<_> = (0..20).map(|k| components[(k * 37 + 5) % components.len()]).collect();
    let (mut select, mut whole) = (Vec::new(), Vec::new());
    for &id in &picks {
        let t = Instant::now();
        state.borrow_mut().doc.select(Some(id));
        ide.ui.fire(ids.surface, "on_change", &aylis_core::EventArgs::None);
        select.push(t.elapsed());
        frame(&mut ide.ui);
        whole.push(t.elapsed());
    }
    let median = |v: &[Duration]| {
        let mut s = v.to_vec();
        s.sort();
        s[s.len() / 2]
    };
    eprintln!("selecionar: o evento {:.1?} (mediana), com o quadro {:.1?}, o pior {:.1?}", median(&select), median(&whole), whole.iter().max().unwrap());
}

/// Trocar de seção nas Opções do projeto com o NoOBS (relatado pelo autor em 07/11: a tela
/// parava uns 3 s a cada troca, na gravação de uso). O projeto fica ao lado do aylis
/// (`../e-rust/noobs`); sem ele, o teste não roda. Rode com
/// `cargo test -p aylis-ide --test perf -- --ignored --nocapture --exact project_options_sections_cost`.
#[test]
#[ignore]
fn project_options_sections_cost() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../e-rust/noobs/NoOBS.aylisproj");
    if !path.is_file() {
        eprintln!("sem o NoOBS em {}", path.display());
        return;
    }
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &path);
    let t = Instant::now();
    aylis_ide::projectsettings::open(&mut ide.ui, &state, &ide.ids);
    ide.ui.layout();
    eprintln!("abrir as Opções (Geral): {:.0?}", t.elapsed());
    let dialog = *ide.ui.open_forms().last().unwrap();
    let mut all = vec![dialog];
    let mut k = 0;
    while k < all.len() {
        all.extend(ide.ui.children(all[k]).iter().copied());
        k += 1;
    }
    let nav = all.iter().copied().find(|&c| ide.ui.get::<aylis_controls::ListBox>(c).is_some()).unwrap();
    for (i, title) in aylis_ide::projectsettings::section_titles().iter().enumerate().skip(1).chain(std::iter::once((0, &"Geral"))) {
        let t = Instant::now();
        let _ = ide.ui.set_prop(nav, "item_index", i as i64);
        ide.ui.fire(nav, "on_change", &aylis_core::EventArgs::None);
        ide.ui.layout();
        eprintln!("{title}: {:.0?}", t.elapsed());
    }
}
