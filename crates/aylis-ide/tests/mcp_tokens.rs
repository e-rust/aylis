//! O MCP enxuto (15/11): o `list_components` numa linha por componente, recuada pelo nível, e
//! o `class_info` com as propriedades comuns só pelo nome.

use std::path::PathBuf;

use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};
use serde_json::json;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_tokens_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// O recuo diz o pai, também debaixo de um componente sem nome (antes, o pai pelo nome saía
/// vazio ali, o mesmo vazio do form).
#[test]
fn the_component_tree_is_one_indented_line_per_component() {
    let base = temp("tree");
    let project = aylis_project::create(&base, "Arvore", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal", "children": [
        { "class": "Layout", "children": [{ "class": "Label", "name": "lblTotal" }] },
        { "class": "Button", "name": "btnOk" }
    ] } });
    let set = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "set_layout", &json!({ "layout": layout.to_string() }));
    assert!(set.is_ok(), "{set:?}");
    let tree = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "list_components", &json!({})).unwrap();
    assert_eq!(tree.as_str().unwrap(), "frmPrincipal Form\n  (sem nome) Layout\n    lblTotal Label\n  btnOk Button\n");

    let info = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "class_info", &json!({ "class": "Button" })).unwrap();
    assert!(info["properties"].get("align").is_none() && info["common"]["properties"].as_array().unwrap().iter().any(|p| p == "align"), "{info}");
    let full = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "class_info", &json!({ "class": "Button", "inherited": true })).unwrap();
    assert!(full["properties"].get("align").is_some(), "{full}");
    // As comuns só pelo nome custam bem menos que com o tipo.
    assert!(info.to_string().len() * 3 < full.to_string().len() * 2, "{} × {}", info.to_string().len(), full.to_string().len());
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}
