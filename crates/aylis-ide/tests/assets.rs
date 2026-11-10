//! Projeto → Fontes e Projeto → Ícones na IDE.

use std::path::PathBuf;

use aylis_controls::{Button, CheckBox, Edit};
use aylis_core::{ComponentId, ModalResult, PointF, PropValue, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_assets_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn has_resources() -> bool {
    let dir = aylis_project::codegen::resources_dir();
    let ok = dir.join("fonts/catalog.json").exists() && dir.join("icons/material.txt").exists();
    if !ok {
        eprintln!("sem resources/ (rode aylis-tool resources): teste pulado");
    }
    ok
}

/// Todos os descendentes de `id`.
fn all(ui: &Ui, id: ComponentId) -> Vec<ComponentId> {
    let mut out = vec![id];
    let mut i = 0;
    while i < out.len() {
        out.extend(ui.children(out[i]).iter().copied());
        i += 1;
    }
    out
}

fn text(ui: &Ui, id: ComponentId) -> String {
    match ui.get_prop(id, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

/// Desenha o diálogo num PNG (em `AYLIS_SHOTS`, se definido) para conferir a aparência.
fn shot(ui: &mut Ui, form: ComponentId, name: &str) {
    let Ok(dir) = std::env::var("AYLIS_SHOTS") else { return };
    let renderer = aylis_skia::SkiaRenderer::new();
    ui.services_mut().text = Some(Box::new(renderer.text_service()));
    ui.layout();
    let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    let png = renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)).unwrap();
    std::fs::write(PathBuf::from(dir).join(name), png).unwrap();
}

#[test]
fn pick_an_icon_from_the_material_pack_for_the_selected_button() {
    if !has_resources() {
        return;
    }
    let base = temp("icons");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let button = state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    state.borrow_mut().doc.select(Some(button));

    actions::project_icons(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    assert!(ide.ui.is_modal(dialog));
    ide.ui.layout();
    let ids = all(&ide.ui, dialog);
    let search = ids.iter().copied().find(|&id| ide.ui.get::<Edit>(id).is_some()).unwrap();
    let grid_buttons = |ui: &Ui| -> Vec<(ComponentId, String)> {
        all(ui, dialog)
            .into_iter()
            .filter_map(|id| ui.get::<Button>(id).filter(|b| b.icon.contains(':')).map(|b| (id, b.icon.clone())))
            .collect()
    };
    assert_eq!(grid_buttons(&ide.ui).len(), 240, "a grade mostra os primeiros 240");

    let _ = ide.ui.set_prop(search, "text", "home");
    ide.ui.fire(search, "on_change", &aylis_core::EventArgs::None);
    ide.ui.layout();
    let found = grid_buttons(&ide.ui);
    assert!(found.iter().any(|(_, n)| n == "material:home"), "{found:?}");
    assert!(found.iter().all(|(_, n)| n.contains("home") && !n.ends_with("-fill")));
    shot(&mut ide.ui, dialog, "icones.png");

    // Preenchido: só as variantes -fill.
    let fill = ids.iter().copied().find(|&id| ide.ui.get::<CheckBox>(id).is_some()).unwrap();
    let _ = ide.ui.set_prop(fill, "checked", true);
    ide.ui.fire(fill, "on_change", &aylis_core::EventArgs::None);
    let found = grid_buttons(&ide.ui);
    let (home, _) = found.iter().find(|(_, n)| n == "material:home-fill").cloned().expect("home-fill");
    ide.ui.click(home);
    shot(&mut ide.ui, dialog, "icones-escolhido.png");
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert!(!ide.ui.contains(dialog));

    let st = state.borrow();
    assert_eq!(st.doc.ui.get::<Button>(button).unwrap().icon, "material:home-fill");
    assert!(st.doc.ui.icon("material:home-fill").is_some(), "o designer desenha o ícone");
    drop(st);

    // Salvar o form leva o ícone (só ele) para assets/icons.
    assert_eq!(actions::save_all(&state), 0);
    let pack = std::fs::read_to_string(base.join("Loja/assets/icons/material.txt")).unwrap();
    assert_eq!(pack.lines().filter(|l| !l.starts_with('#')).count(), 1);
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn add_a_google_font_and_make_it_the_project_font() {
    if !has_resources() {
        return;
    }
    let base = temp("fonts");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);

    actions::project_fonts(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    ide.ui.layout();
    // A linha "Roboto": marcar.
    let ids = all(&ide.ui, dialog);
    // O nome da família é um rótulo clicável, alinhado à esquerda (era um botão).
    let roboto = ids.iter().copied().find(|&id| ide.ui.get::<aylis_controls::Label>(id).is_some_and(|l| l.text == "Roboto")).expect("Roboto");
    let row = ide.ui.parent(roboto).unwrap();
    let check = ide.ui.children(row).iter().copied().find(|&id| ide.ui.get::<CheckBox>(id).is_some()).unwrap();
    let _ = ide.ui.set_prop(check, "checked", true);
    ide.ui.fire(check, "on_change", &aylis_core::EventArgs::None);
    ide.ui.click(roboto);
    // O combo da fonte do projeto ganhou "Roboto": escolhe.
    let combo = ids.iter().copied().find(|&id| ide.ui.get::<aylis_controls::ComboBox>(id).is_some()).unwrap();
    let items = match ide.ui.get_prop(combo, "items") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    };
    let index = items.split(';').position(|l| l == "Roboto").expect("Roboto no combo");
    let _ = ide.ui.set_prop(combo, "item_index", index as i64);
    shot(&mut ide.ui, dialog, "fontes.png");
    ide.ui.set_modal_result(dialog, ModalResult::Ok);

    assert!(base.join("Loja/assets/fonts/roboto/OFL.txt").is_file());
    let st = state.borrow();
    assert_eq!(st.project.as_ref().unwrap().font, "Roboto");
    assert!(st.doc.ui.theme().fonts.values().any(|f| f.family == "Roboto"));
    assert!(aylis_core::fonts::families().contains(&"Roboto".to_string()));
    drop(st);
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn pick_a_noto_animation_for_the_selected_lottie_and_save_copies_only_it() {
    if !aylis_project::codegen::resources_dir().join("lottie/noto/catalog.json").exists() {
        return;
    }
    let base = temp("lottie");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let form = ide.form;
    ide.ui.show(form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let lottie = state.borrow_mut().doc.add("Lottie", Some(PointF::new(40.0, 40.0))).unwrap();
    state.borrow_mut().doc.select(Some(lottie));

    actions::project_animations(&mut ide.ui, &state, &ide.ids);
    let dialog = *ide.ui.open_forms().last().unwrap();
    ide.ui.layout();
    let cells: Vec<ComponentId> = all(&ide.ui, dialog)
        .into_iter()
        .filter(|&id| ide.ui.get::<Button>(id).is_some() && ide.ui.children(id).iter().any(|&c| ide.ui.class_name(c) == Some("Lottie")))
        .collect();
    assert_eq!(cells.len(), 55, "o pacote Noto inteiro");
    // A que se chama "fogo".
    let fire = cells
        .iter()
        .copied()
        .find(|&b| matches!(ide.ui.get_prop(b, "hint"), Ok(PropValue::String(h)) if h.starts_with("noto:fogo ")))
        .expect("noto:fogo");
    ide.ui.click(fire);
    shot(&mut ide.ui, dialog, "animacoes.png");
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    assert_eq!(text_prop(&state.borrow().doc.ui, lottie, "source"), "noto:fogo");

    assert_eq!(actions::save_all(&state), 0);
    let dir = base.join("Loja/assets/lottie/noto");
    assert!(dir.join("fogo.json").is_file());
    assert!(dir.join("LICENSE.txt").is_file());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2, "só a citada e a licença");
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

fn text_prop(ui: &Ui, id: ComponentId, name: &str) -> String {
    match ui.get_prop(id, name) {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

#[test]
fn importing_fbx_writes_a_glb_into_assets_models() {
    let base = temp("fbx");
    let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
    let fbx = base.join("cubo.fbx");
    std::fs::write(&fbx, include_bytes!("../../aylis-3d/tests/data/cube_bones.fbx")).unwrap();
    let (glb, summary) = aylis_ide::inspector::import_model(&fbx, Some(&project.dir())).unwrap();
    assert_eq!(glb, project.dir().join("assets").join("models").join("cubo.glb"));
    assert_eq!(summary, "1 malha, 2 ossos, 1 clipe, 0 texturas");
    let model = aylis_3d::Model::from_glb(&std::fs::read(&glb).unwrap()).unwrap();
    assert_eq!(model.clips[0].name, "Dobrar");
    // O caminho gravado no form é relativo ao projeto.
    let rel = aylis_ide::inspector::relative_asset(&project.dir(), &glb, "assets/models").unwrap();
    assert_eq!(rel.replace('\\', "/"), "assets/models/cubo.glb");
}
