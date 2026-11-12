//! O relatório do Tabernáculo (11/11), no MCP: o `menu` e o `dialog_click` acham o item certo
//! quando há dois com o mesmo texto (M10, M11), e o `output` separa a compilação do programa,
//! com o nível de cada linha, sem perder o histórico ao trocar de projeto (M20).

use std::path::PathBuf;

use aylis_ide::document::Document;
use aylis_ide::jobs::{OutKind, OutputLine};
use aylis_ide::state::shared;
use aylis_ide::{actions, app, mcp};
use aylis_tools::Target;
use aylis_tools::env::{Check, Fix, Status};
use serde_json::{Value, json};

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_choices_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// M10: o "Ligar a campo…" do Object Inspector existe em cada propriedade; o caminho genérico é
/// recusado com os candidatos, e o da propriedade (ou o `id`) acerta o item dela.
#[test]
fn the_menu_picks_the_item_by_path_or_id() {
    let base = temp("menu");
    let project = aylis_project::create(&base, "Menus", aylis_project::Language::Rust).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    let run = |ui: &mut aylis_core::Ui, name: &str, args: Value| {
        let r = mcp::run_tool(ui, &state, &ide.ids, name, &args);
        app::refresh(ui, &state, &ide.ids);
        r
    };
    assert_eq!(run(&mut ide.ui, "add_component", json!({ "class": "Edit", "name": "edtNome" })).unwrap(), "edtNome");
    run(&mut ide.ui, "select", json!({ "components": ["edtNome"] })).unwrap();

    let menus = run(&mut ide.ui, "ide_menus", json!({})).unwrap();
    let link = menus.as_array().unwrap().iter().find(|m| m["path"] == "Object Inspector › (propriedade) › Ligar a campo…").expect("o item do Object Inspector").clone();
    assert!(link["count"].as_u64().unwrap() > 1, "um por propriedade: {link}");
    assert!(link["id"].as_u64().is_some(), "{link}");
    let compile = menus.as_array().unwrap().iter().find(|m| m["path"] == "Executar/Compilar").expect("Executar/Compilar").clone();
    assert!(compile["id"].as_u64().is_some());

    // O genérico (e só o texto) valem para várias linhas: recusados, com o caminho exato de cada.
    for path in ["Object Inspector › (propriedade) › Ligar a campo…", "Ligar a campo…"] {
        let refused = run(&mut ide.ui, "menu", json!({ "path": path })).unwrap_err();
        assert!(refused.contains("itens de menu valem") && refused.contains("Object Inspector › align › Ligar a campo…") && refused.contains("(id "), "{refused}");
    }
    // O da propriedade: o clique é o daquela linha (sem DataSource, a IDE avisa na situação).
    run(&mut ide.ui, "menu", json!({ "path": "Object Inspector › text › Ligar a campo…" })).unwrap();
    assert!(state.borrow().status.contains("DataSource"), "{}", state.borrow().status);
    // Pelo id: o mesmo; um id que não é item de menu, recusado.
    state.borrow_mut().status.clear();
    run(&mut ide.ui, "menu", json!({ "id": link["id"] })).unwrap();
    assert!(state.borrow().status.contains("DataSource"), "{}", state.borrow().status);
    assert!(run(&mut ide.ui, "menu", json!({ "id": 999_999_999u64 })).is_err());
    // O caminho de um menu da janela continua valendo.
    assert!(run(&mut ide.ui, "menu", json!({ "path": "Arquivo/Nada que exista" })).unwrap_err().contains("ide_menus"));
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

/// M11: o "Verificar ambiente" com três "Corrigir…": cada botão tem um `label` com o item e a
/// plataforma, e o `dialog_click` pelo texto repetido recusa; pelo label ou pelo id, acerta.
#[test]
fn the_same_button_text_gets_distinct_labels() {
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    let fix = |what: &str| Fix { description: format!("Instala o {what}"), command: vec!["echo".into(), what.into()], interactive: false, fetch: None, env: Vec::new() };
    let check = |target, name: &str| Check { target, name: name.into(), status: Status::Missing, detail: "não encontrado".into(), fix: Some(fix(name)), help: None };
    // A verificação das linguagens que a IDE começa ao abrir desabilitaria os botões.
    state.borrow_mut().env_job = None;
    state.borrow_mut().checks = vec![check(Target::Desktop, "Rust"), check(Target::Android, "Android SDK"), check(Target::Web, "Rust")];
    aylis_ide::tooldialogs::environment(&mut ide.ui, &state, &ide.ids, false);
    app::refresh(&mut ide.ui, &state, &ide.ids);
    let run = |ui: &mut aylis_core::Ui, name: &str, args: Value| mcp::run_tool(ui, &state, &ide.ids, name, &args);

    let dialog = run(&mut ide.ui, "dialog_state", json!({})).unwrap();
    let fixes: Vec<&Value> = dialog["buttons"].as_array().unwrap().iter().filter(|b| b["text"] == "Corrigir…").collect();
    assert_eq!(fixes.len(), 3, "{dialog}");
    let labels: Vec<&str> = fixes.iter().map(|b| b["label"].as_str().unwrap()).collect();
    assert!(labels.contains(&"Corrigir… — Android SDK (Android)"), "{labels:?}");
    let unique: std::collections::HashSet<&&str> = labels.iter().collect();
    assert_eq!(unique.len(), 3, "{labels:?}");

    let refused = run(&mut ide.ui, "dialog_click", json!({ "text": "Corrigir…" })).unwrap_err();
    assert!(refused.contains("Corrigir… — Android SDK (Android)") && refused.contains("(id "), "{refused}");
    // Pelo label: abre a confirmação daquela correção.
    run(&mut ide.ui, "dialog_click", json!({ "text": "Corrigir… — Android SDK (Android)" })).unwrap();
    let confirm = run(&mut ide.ui, "dialog_state", json!({})).unwrap().to_string();
    assert!(confirm.contains("Instala o Android SDK"), "{confirm}");
    assert!(run(&mut ide.ui, "dialog_click", json!({ "text": "Não" })).is_ok());
    // Pelo id.
    let web_name = format!("({})", Target::Web.name());
    let web = fixes.iter().find(|b| b["label"].as_str().unwrap().contains(&web_name)).expect("o do navegador");
    let web_id = web["id"].clone();
    let web_label = web["label"].as_str().unwrap().to_string();
    let top = run(&mut ide.ui, "dialog_state", json!({})).unwrap();
    assert!(top["buttons"].as_array().unwrap().iter().any(|b| b["label"] == web_label.as_str()), "de volta ao Verificar ambiente: {top}");
    run(&mut ide.ui, "dialog_click", json!({ "id": web_id })).unwrap();
    let confirm = run(&mut ide.ui, "dialog_state", json!({})).unwrap().to_string();
    assert!(confirm.contains("Instala o Rust"), "{confirm}");
}

/// M20: cada linha com a origem e o nível; a última execução por padrão, e as de antes (de
/// outro projeto também) com `previous`.
#[test]
fn the_output_tells_the_source_and_the_level_of_each_line() {
    let state = shared(Document::new());
    {
        let mut st = state.borrow_mut();
        st.run_number = 1;
        st.push(OutputLine::new("da execução anterior", OutKind::Info));
        st.run_number = 2;
        st.push(OutputLine::new("Compiling loja", OutKind::Muted));
        st.push(OutputLine::new("✖ erro[E0425]: src/lib.rs:3:5  não achei x", OutKind::Error));
        let mut panic = OutputLine::new("thread 'main' panicked", OutKind::Error);
        panic.program = true;
        st.push(panic);
        st.push(OutputLine::new("olá do app", OutKind::Program));
        // Sair do projeto limpa o painel, não o histórico.
        st.clear_output();
    }
    let st = state.borrow();
    let out = mcp::output_json(&st, &json!({})).unwrap();
    let lines = out["lines"].as_array().unwrap();
    assert_eq!(lines.len(), 4, "{out}");
    assert_eq!(lines[0], json!({ "source": "build", "level": "note", "text": "Compiling loja" }));
    assert_eq!(lines[1]["level"], "error");
    assert_eq!(lines[2], json!({ "source": "program", "level": "error", "text": "thread 'main' panicked" }));
    assert_eq!(lines[3]["source"], "program");

    let errors = mcp::output_json(&st, &json!({ "level": "error", "source": "build" })).unwrap();
    assert_eq!(errors["lines"].as_array().unwrap().len(), 1, "{errors}");
    let program = mcp::output_json(&st, &json!({ "source": "program", "lines": 1 })).unwrap();
    assert_eq!(program["lines"][0]["text"], "olá do app");
    let all = mcp::output_json(&st, &json!({ "previous": true })).unwrap();
    assert_eq!(all["lines"][0]["text"], "da execução anterior");
    assert_eq!(all["lines"][0]["run"], 1);
    assert!(mcp::output_json(&st, &json!({ "source": "app" })).is_err());
}
