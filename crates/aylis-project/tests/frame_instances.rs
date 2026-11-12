//! O frame repetido (`FraX::create_in`, um cartão por item): quanto custa cada instância,
//! contra o mesmo cartão montado em código (G21 do relatório do Tabernáculo).
//!
//! Em release, sozinho: `cargo test --release -p aylis-project --test frame_instances --
//! --ignored --nocapture`.

use std::time::Instant;

use aylis_core::{Color, ComponentId, PropValue, Ui};
use aylis_project::*;
use serde_json::json;

const CARDS: usize = 200;

/// O cartão: um form de frame com 7 controles (o fundo, o título, o texto, a imagem, o
/// rodapé com dois botões).
fn card_form() -> serde_json::Value {
    json!({ "aylis": 1, "root": { "class": "Form", "name": "fraCartao", "width": 240, "height": 120, "children": [
        { "class": "Rectangle", "name": "fundo", "align": "client", "radius": 8 },
        { "class": "Label", "name": "lblTitulo", "align": "top", "height": 24, "text": "Gênesis", "margins": { "left": 8, "top": 8, "right": 8, "bottom": 0 } },
        { "class": "Label", "name": "lblTexto", "align": "client", "text": "No princípio...", "margins": { "left": 8, "top": 4, "right": 8, "bottom": 4 } },
        { "class": "Image", "name": "imgIcone", "align": "left", "width": 32 },
        { "class": "Layout", "name": "rodape", "align": "bottom", "height": 32, "children": [
            { "class": "Button", "name": "btnAbrir", "align": "right", "width": 80, "text": "Abrir" },
            { "class": "Button", "name": "btnMarcar", "align": "right", "width": 80, "text": "Marcar" }
        ] }
    ] } })
}

/// O mesmo cartão montado em código (o contorno do Tabernáculo).
fn card_in_code(ui: &mut Ui, parent: ComponentId) -> ComponentId {
    let make = |ui: &mut Ui, class: &str, parent: ComponentId, props: &[(&str, PropValue)]| {
        let id = ui.create(class, Some(parent)).unwrap();
        for (k, v) in props {
            let _ = ui.set_prop(id, k, v.clone());
        }
        id
    };
    let root = make(ui, "Layout", parent, &[("width", PropValue::Float(240.0)), ("height", PropValue::Float(120.0))]);
    make(ui, "Rectangle", root, &[("align", "client".into()), ("radius", PropValue::Float(8.0))]);
    let margins = |ui: &mut Ui, id: ComponentId, top: f64, bottom: f64| {
        let value = aylis_json::prop_from_json(ui, id, "margins", &json!({ "left": 8, "top": top, "right": 8, "bottom": bottom })).unwrap();
        ui.set_prop(id, "margins", value).unwrap();
    };
    let title = make(ui, "Label", root, &[("align", "top".into()), ("height", PropValue::Float(24.0)), ("text", "Gênesis".into())]);
    margins(ui, title, 8.0, 0.0);
    let text = make(ui, "Label", root, &[("align", "client".into()), ("text", "No princípio...".into())]);
    margins(ui, text, 4.0, 4.0);
    make(ui, "Image", root, &[("align", "left".into()), ("width", PropValue::Float(32.0))]);
    let footer = make(ui, "Layout", root, &[("align", "bottom".into()), ("height", PropValue::Float(32.0))]);
    make(ui, "Button", footer, &[("align", "right".into()), ("width", PropValue::Float(80.0)), ("text", "Abrir".into())]);
    make(ui, "Button", footer, &[("align", "right".into()), ("width", PropValue::Float(80.0)), ("text", "Marcar".into())]);
    root
}

/// O que o `create_in` gerado faz (sem a parte tipada, que é só ler `form_components`).
fn create_in(ui: &mut Ui, parent: ComponentId) -> ComponentId {
    let root = ui.create_form_instance("fraCartao").unwrap();
    let name = ui.free_name_in(parent, "fraCartao");
    ui.set_name(root, &name).unwrap();
    ui.set_parent(root, Some(parent)).unwrap();
    if matches!(ui.get_prop(root, "fill"), Ok(PropValue::Null)) {
        let _ = ui.set_prop(root, "fill", Color::TRANSPARENT);
    }
    root
}

/// O projeto com o cartão, do pacote embutido (como o app publicado) ou da pasta (o `dev`).
fn setup(test: &str, bundle: bool) -> (Ui, ComponentId, std::path::PathBuf) {
    let base = std::env::temp_dir().join(format!("aylis_project_frames_{test}_{bundle}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let mut project = create(&base, "Cartoes", Language::Rust).unwrap();
    project.add_form("fraCartao", "Cartão").unwrap();
    project.forms.iter_mut().find(|f| f.name == "fraCartao").unwrap().auto_create = false;
    project.save().unwrap();
    let file = project.file(&project.form("fraCartao").unwrap().file);
    std::fs::write(&file, card_form().to_string()).unwrap();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = if bundle {
        let packed = Bundle::parse(&aylis_build::pack(&project.dir()).unwrap()).unwrap();
        install_with(&mut ui, &packed.project().unwrap(), Files::Bundle(packed)).unwrap()
    } else {
        install(&mut ui, &project).unwrap()
    };
    let list = ui.create("Layout", Some(main)).unwrap();
    // O relógio que a plataforma anda a cada evento (os cartões de um handler, no mesmo instante).
    ui.set_time(1.0);
    (ui, list, base)
}

/// Cada instância é um cartão inteiro, com os seus componentes e o nome livre no form.
#[test]
fn each_instance_is_a_whole_card() {
    for bundle in [true, false] {
        let (mut ui, list, base) = setup("whole", bundle);
        let a = create_in(&mut ui, list);
        let b = create_in(&mut ui, list);
        assert_eq!((ui.name(a), ui.name(b)), (Some("fraCartao"), Some("fraCartao2")));
        let title = ui.find_in(b, "lblTitulo").expect("o segundo cartão tem o título");
        assert_eq!(ui.get_prop(title, "text").unwrap(), PropValue::String("Gênesis".into()));
        let open = ui.find_in(b, "btnAbrir").unwrap();
        assert_ne!(Some(open), ui.find_in(a, "btnAbrir"), "cada cartão com os seus");
        assert_eq!(ui.get_prop(b, "fill").unwrap(), PropValue::Color(Color::TRANSPARENT));
        std::fs::remove_dir_all(&base).unwrap();
    }
}

/// Da pasta (a feature `dev`, a IDE), o form mudado no disco vale no próximo cartão: no mesmo
/// instante do relógio, o que já foi lido; no próximo evento, o arquivo novo.
#[test]
fn a_changed_form_file_is_read_again() {
    let (mut ui, list, base) = setup("changed", false);
    let title = |ui: &Ui, card: ComponentId| ui.get_prop(ui.find_in(card, "lblTitulo").unwrap(), "text").unwrap();
    let first = create_in(&mut ui, list);
    assert_eq!(title(&ui, first), PropValue::String("Gênesis".into()));
    let file = base.join("Cartoes/forms").join(std::fs::read_dir(base.join("Cartoes/forms")).unwrap().flatten().find(|e| e.file_name().to_string_lossy().to_lowercase().contains("cartao")).unwrap().file_name());
    let mut doc = card_form();
    doc["root"]["children"][1]["text"] = json!("Êxodo");
    std::fs::write(&file, doc.to_string()).unwrap();
    ui.set_time(2.0);
    let second = create_in(&mut ui, list);
    assert_eq!(title(&ui, second), PropValue::String("Êxodo".into()), "o arquivo novo");
    // Sem relógio (um teste sem plataforma), confere sempre.
    doc["root"]["children"][1]["text"] = json!("Levítico");
    std::fs::write(&file, doc.to_string()).unwrap();
    ui.set_time(0.0);
    let third = create_in(&mut ui, list);
    assert_eq!(title(&ui, third), PropValue::String("Levítico".into()));
    std::fs::remove_dir_all(&base).unwrap();
}

fn per_card(mut f: impl FnMut()) -> f64 {
    let start = Instant::now();
    for _ in 0..CARDS {
        f();
    }
    start.elapsed().as_secs_f64() * 1e6 / CARDS as f64
}

/// Os tempos por cartão: o pacote embutido, a pasta e o montado em código.
#[test]
#[ignore = "medição: em release, com --ignored --nocapture"]
fn create_in_cost_per_card() {
    let mut results = Vec::new();
    for bundle in [true, false] {
        let (mut ui, list, base) = setup("cost", bundle);
        create_in(&mut ui, list); // o primeiro lê e interpreta o arquivo
        let us = per_card(|| {
            create_in(&mut ui, list);
        });
        results.push((if bundle { "create_in (pacote)" } else { "create_in (pasta)" }, us));
        std::fs::remove_dir_all(&base).unwrap();
    }
    let (mut ui, list, base) = setup("code", true);
    card_in_code(&mut ui, list);
    let code = per_card(|| {
        card_in_code(&mut ui, list);
    });
    std::fs::remove_dir_all(&base).unwrap();
    results.push(("em código", code));
    for (what, us) in &results {
        println!("{what:>20}: {us:8.2} µs por cartão ({:.1}× o código)", us / code);
    }
    // Nem o pacote embutido (o app publicado) nem a pasta (a feature `dev`) podem voltar a
    // custar várias vezes o código: antes do G21, a pasta relia o arquivo a cada cartão (11×).
    // A diferença que fica é o que o código não faz: os nomes (o acesso tipado) e a tradução.
    if !cfg!(debug_assertions) {
        for (what, us) in &results[..2] {
            assert!(*us < code * 3.0, "{what}: {us:.1} µs contra {code:.1} µs em código");
        }
    }
}
