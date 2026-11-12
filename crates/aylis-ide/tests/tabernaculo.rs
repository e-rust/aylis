//! O relatório do Tabernáculo (11/11), na IDE: o disco mudado por fora (M13), fechar o projeto
//! com forms alterados (I07, M05) e o renomear e o `usages` pelo MCP (M07, M08).

use std::path::PathBuf;

use aylis_core::PointF;
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};
use serde_json::{Value, json};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_tab_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// O form aberto mudou no disco: sem alteração na IDE, recarrega; com alteração dos dois lados,
/// o MCP recusa com o diff até o `resolve_conflict`.
#[test]
fn a_form_changed_on_disk_reloads_or_reports_the_conflict() {
    let base = temp("disk");
    let project = aylis_project::create(&base, "Disco", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let file = state.borrow().doc.path.clone().unwrap();

    // Por fora, sem alteração na IDE: recarrega.
    let mut layout: Value = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    layout["root"]["caption"] = json!("Mudado por fora");
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(&file, aylis_json::to_form_text(&layout)).unwrap();
    let (reloaded, conflicts) = aylis_ide::disk::check(&mut ide.ui, &state);
    assert_eq!((reloaded, conflicts.len()), (1, 0));
    assert!(state.borrow().doc.to_json().unwrap().contains("Mudado por fora"));
    assert!(!state.borrow().doc.modified);

    // Dos dois lados: o MCP recebe o diff.
    state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    layout["root"]["caption"] = json!("De novo por fora");
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(&file, aylis_json::to_form_text(&layout)).unwrap();
    let refused = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "list_components", &json!({})).unwrap_err();
    assert!(refused.contains("resolve_conflict") && refused.contains("De novo por fora"), "{refused}");
    let kept = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "resolve_conflict", &json!({ "file": file.display().to_string(), "keep": "disk" }));
    assert!(kept.is_ok(), "{kept:?}");
    assert!(state.borrow().doc.to_json().unwrap().contains("De novo por fora"));
    assert!(aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "list_components", &json!({})).is_ok());
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

/// Fechar o projeto com um form alterado avisa uma vez (e o MCP diz que não fechou); a
/// segunda vez descarta.
#[test]
fn closing_a_project_with_changed_forms_asks_first() {
    let base = temp("close");
    let project = aylis_project::create(&base, "Fechar", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    state.borrow_mut().doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
    let first = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "close_project", &json!({}));
    assert!(first.as_ref().is_err_and(|e| e.contains("não fechou")), "{first:?}");
    assert!(state.borrow().project.is_some());
    let second = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "close_project", &json!({}));
    assert!(second.is_ok(), "{second:?}");
    assert!(state.borrow().project.is_none());
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}
