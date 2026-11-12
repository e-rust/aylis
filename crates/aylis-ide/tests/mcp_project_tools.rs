//! As ferramentas do MCP para o que o usuário faz pelos menus do projeto (o relatório do
//! Tabernáculo, M17, M18 e M19): a pasta do form novo, o frame, o tema, as traduções, o ícone,
//! a verificação, os testes no modo restrito e a foto de um form.

use std::path::PathBuf;

use aylis_core::{ModalResult, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::{Shared, shared};
use aylis_ide::{actions, app, mcp, mcp_project};
use serde_json::{Value, json};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A IDE com um projeto novo aberto.
fn ide_with_project(name: &str) -> (PathBuf, aylis_project::Project, Shared, app::Ide) {
    let base = temp(name);
    let project = aylis_project::create(&base, "Biblia", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &project.path);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    (base, project, state, ide)
}

fn run(ide: &mut app::Ide, state: &Shared, name: &str, args: Value) -> Result<Value, String> {
    let r = mcp::run_tool(&mut ide.ui, state, &ide.ids, name, &args);
    app::refresh(&mut ide.ui, state, &ide.ids);
    r
}

/// Os Edit do diálogo por cima, na ordem da tela.
fn dialog_edits(ui: &Ui) -> (aylis_core::ComponentId, Vec<aylis_core::ComponentId>) {
    let form = *ui.open_forms().last().unwrap();
    let mut all = vec![form];
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    let edits = all.into_iter().filter(|&c| ui.get::<aylis_controls::Edit>(c).is_some()).collect();
    (form, edits)
}

#[test]
fn new_form_over_mcp_and_the_new_form_dialog_take_the_folder() {
    let (base, project, state, mut ide) = ide_with_project("mcp_folder");
    run(&mut ide, &state, "new_form", json!({ "name": "fraBiblia", "folder": "biblia" })).unwrap();
    let saved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(saved.form("fraBiblia").unwrap().file, "forms/biblia/frabiblia.json");
    assert!(project.file("forms/biblia/frabiblia.json").is_file());
    assert!(!project.file("forms/frabiblia.json").exists(), "não fica cópia na raiz");
    assert_eq!(state.borrow().doc.form_name(), "fraBiblia");
    // Caminho absoluto e `..` são recusados, sem criar o form.
    for bad in ["../fora", "C:/fora", "/fora", "biblia/../.."] {
        assert!(run(&mut ide, &state, "new_form", json!({ "name": "frmFora", "folder": bad })).is_err(), "{bad}");
    }
    assert!(aylis_project::Project::open(&project.path).unwrap().form("frmFora").is_none());
    assert!(actions::form_file_in("forms/cadastros", "frmCliente").unwrap() == "forms/cadastros/cliente.json");

    // O "Novo form" da IDE: o nome e a pasta.
    actions::new_form(&mut ide.ui, &state, &ide.ids);
    let (dialog, edits) = dialog_edits(&ide.ui);
    assert_eq!(edits.len(), 2, "o nome e a pasta");
    let _ = ide.ui.set_prop(edits[0], "text", "frmCliente");
    let _ = ide.ui.set_prop(edits[1], "text", "cadastros");
    ide.ui.set_modal_result(dialog, ModalResult::Ok);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let saved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(saved.form("frmCliente").map(|f| f.file.as_str()), Some("forms/cadastros/cliente.json"), "{}", state.borrow().status);
    assert_eq!(state.borrow().doc.form_name(), "frmCliente");
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn new_frame_makes_a_transparent_form_and_puts_a_frame_in_the_host() {
    let (base, project, state, mut ide) = ide_with_project("mcp_frame");
    let out = run(&mut ide, &state, "new_frame", json!({ "name": "fraCartao", "folder": "cartoes", "width": 300, "height": 90, "host": "frmPrincipal", "frame_name": "fraTopo", "align": "top" })).unwrap();
    assert_eq!(out["frame"], "fraTopo", "{out}");
    let saved: Value = serde_json::from_str(&std::fs::read_to_string(project.file("forms/cartoes/fracartao.json")).unwrap()).unwrap();
    assert_eq!(saved["root"]["fill"], "transparent", "{saved}");
    assert_eq!(saved["root"]["width"], 300);
    // O host fica aberto com o Frame mostrando o form novo.
    let st = state.borrow();
    assert_eq!(st.doc.form_name(), "frmPrincipal");
    let frame = st.doc.ui.find_in(st.doc.root, "fraTopo").unwrap();
    assert_eq!(st.doc.ui.class_name(frame), Some("Frame"));
    assert_eq!(st.doc.props_json(frame)["form"], "fraCartao");
    drop(st);
    assert!(run(&mut ide, &state, "new_frame", json!({ "name": "fraOutro", "host": "frmNaoExiste" })).is_err());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn the_theme_tokens_and_styles_change_through_the_project_theme_file() {
    let (base, project, state, mut ide) = ide_with_project("mcp_theme");
    let before = run(&mut ide, &state, "get_theme", json!({ "section": "colors", "key": "primary" })).unwrap();
    assert!(before["value"].as_str().is_some_and(|c| c.starts_with('#')), "{before}");
    // Num tema da biblioteca, o arquivo nasce em themes/ partindo dele.
    let out = run(&mut ide, &state, "set_theme", json!({ "section": "colors", "key": "primary", "value": "#123456" })).unwrap();
    assert_eq!(out["created"], true, "{out}");
    let file = out["file"].as_str().unwrap().to_string();
    assert!(file.starts_with("themes/") && project.file(&file).is_file());
    assert_eq!(aylis_project::Project::open(&project.path).unwrap().theme, file);
    assert_eq!(state.borrow().doc.ui.theme().color("primary").to_hex().to_uppercase(), "#123456", "o designer mostra o tema novo");
    // Um estilo, e o valor inválido recusado sem gravar.
    run(&mut ide, &state, "set_theme", json!({ "section": "styles", "key": "Button.aviso", "value": { "fill": "primary" } })).unwrap();
    let style = run(&mut ide, &state, "get_theme", json!({ "section": "styles", "key": "Button.aviso" })).unwrap();
    assert_eq!(style["own"]["fill"], "primary", "{style}");
    let text = std::fs::read_to_string(project.file(&file)).unwrap();
    assert!(run(&mut ide, &state, "set_theme", json!({ "section": "colors", "key": "primary", "value": "azul-que-nao-existe" })).is_err());
    assert_eq!(std::fs::read_to_string(project.file(&file)).unwrap(), text, "nada gravado");
    assert!(run(&mut ide, &state, "set_theme", json!({ "section": "nada", "key": "x", "value": 1 })).is_err());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn translations_list_and_set_the_texts_of_a_language() {
    let (base, project, state, mut ide) = ide_with_project("mcp_translations");
    run(&mut ide, &state, "add_component", json!({ "class": "Button", "name": "btnSalvar" })).unwrap();
    run(&mut ide, &state, "set_property", json!({ "component": "btnSalvar", "property": "text", "value": "Salvar" })).unwrap();
    let texts = run(&mut ide, &state, "translations", json!({ "language": "en-US", "missing_only": true })).unwrap();
    assert_eq!(texts["texts"]["Salvar"], "", "{texts}");
    let out = run(&mut ide, &state, "set_translations", json!({ "language": "en-US", "texts": { "Salvar": "Save", "Texto que não existe": "x" } })).unwrap();
    assert_eq!(out["not_in_project"][0], "Texto que não existe", "{out}");
    let saved: Value = serde_json::from_str(&std::fs::read_to_string(project.file("locales/en-US.json")).unwrap()).unwrap();
    assert_eq!(saved["Salvar"], "Save");
    let list = run(&mut ide, &state, "translations", json!({})).unwrap();
    assert_eq!(list["languages"][0]["language"], "en-US", "{list}");
    assert!(run(&mut ide, &state, "translations", json!({ "language": "../fora" })).is_err());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn the_app_icon_is_copied_into_the_project_and_saved() {
    let (base, project, state, mut ide) = ide_with_project("mcp_icon");
    let outside = base.join("logo.png");
    image::RgbaImage::from_pixel(256, 256, image::Rgba([20, 120, 200, 255])).save(&outside).unwrap();
    let out = run(&mut ide, &state, "set_app_icon", json!({ "image": outside.display().to_string(), "background": "#FFFFFF" })).unwrap();
    assert_eq!(out["icon"], "assets/images/logo.png", "{out}");
    assert!(project.file("assets/images/logo.png").is_file());
    let saved = aylis_project::Project::open(&project.path).unwrap();
    assert_eq!(saved.publish.icon, "assets/images/logo.png");
    assert_eq!(saved.publish.icon_background, "#FFFFFF");
    assert!(run(&mut ide, &state, "set_app_icon", json!({ "image": "../logo.png" })).is_err());
    assert!(run(&mut ide, &state, "set_app_icon", json!({ "image": outside.display().to_string(), "background": "azul" })).is_err());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn check_lists_the_project_issues_and_test_is_refused_in_restricted_mode() {
    let (base, _project, state, mut ide) = ide_with_project("mcp_check");
    let ok = run(&mut ide, &state, "check", json!({})).unwrap();
    assert!(ok["issues"].is_array() && ok["errors"].is_number(), "{ok}");
    // Um handler que o código não tem é um erro da verificação.
    run(&mut ide, &state, "add_component", json!({ "class": "Button", "name": "btnSalvar" })).unwrap();
    run(&mut ide, &state, "set_property", json!({ "component": "btnSalvar", "property": "on_click", "value": "btnSalvarClick" })).unwrap();
    let found = run(&mut ide, &state, "check", json!({})).unwrap();
    let issues = found["issues"].as_array().unwrap();
    assert!(issues.iter().any(|i| i["message"].as_str().is_some_and(|m| m.contains("btnSalvarClick"))), "{found}");
    // No modo restrito, os testes não rodam (o MCP não confirma confiança).
    state.borrow_mut().restricted = true;
    let refused = run(&mut ide, &state, "test", json!({})).unwrap_err();
    assert!(refused.contains("restrito"), "{refused}");
    assert!(!mcp_project::tests_running());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn screenshot_paints_a_form_that_is_not_open() {
    let (base, _project, state, mut ide) = ide_with_project("mcp_screenshot");
    run(&mut ide, &state, "new_form", json!({ "name": "frmOutro" })).unwrap();
    run(&mut ide, &state, "set_property", json!({ "property": "width", "value": 320 })).unwrap();
    run(&mut ide, &state, "save", json!({})).unwrap();
    run(&mut ide, &state, "open_form", json!({ "name": "frmPrincipal" })).unwrap();
    // Aberto numa aba, sai como está; fechado, é lido do arquivo.
    assert!(mcp_project::form_png(&state, "frmOutro", "").is_ok());
    let tab = state.borrow().find_tab("frmOutro").unwrap();
    state.borrow_mut().close_tab(tab);
    let tabs = state.borrow().tab_count();
    let (title, png) = mcp_project::form_png(&state, "frmOutro", "").unwrap();
    assert_eq!(title, "frmOutro");
    assert!(png.starts_with(b"\x89PNG"));
    assert_eq!(image::load_from_memory(&png).unwrap().width(), 320);
    assert_eq!(state.borrow().tab_count(), tabs, "não abre aba");
    assert_eq!(state.borrow().doc.form_name(), "frmPrincipal");
    assert!(mcp_project::form_png(&state, "frmNaoExiste", "").is_err());
    let _ = std::fs::remove_dir_all(base);
}
