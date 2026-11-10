//! Depuração dentro da IDE de ponta a ponta, com um projeto Rust de verdade e o CodeLLDB.
//! Demora (compila o projeto) e abre a janela do app: rode à mão com
//! `cargo test -p aylis-ide --test debug -- --ignored`.

use std::time::{Duration, Instant};

use aylis_controls::{Memo, TreeView};
use aylis_ide::debugger::{self, Cmd};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app, jobs};
use aylis_project::{Language, create};

#[test]
#[ignore]
fn the_ide_stops_at_a_breakpoint_steps_and_stops() {
    if aylis_tools::dap::codelldb().is_none() {
        eprintln!("CodeLLDB não encontrado: teste pulado");
        return;
    }
    let base = std::env::temp_dir().join(format!("aylis_depurar_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = create(&base, "Depura", Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let lib = project.code_dir().join("src/lib.rs");
    let text = std::fs::read_to_string(&lib).unwrap();
    let line = text.lines().position(|l| l.contains("generated::register(&mut app.ui);")).expect("linha do register") + 1;

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let ui = &mut ide.ui;
    actions::open_recent(ui, &state, &ide.ids, &project.path);
    debugger::toggle_breakpoint(ui, &state, &lib, line);
    assert_eq!(debugger::breakpoints_of(&state, &lib), [line]);

    debugger::start(ui, &state);
    // O laço de eventos da IDE: a compilação, a sessão e os eventos do depurador.
    let step = |ui: &mut aylis_core::Ui| {
        jobs::drain(ui, &state, &ide.ids);
        debugger::poll(ui, &state);
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(30));
    };
    let deadline = Instant::now() + Duration::from_secs(900);
    while !state.borrow().debugger.has_stack() {
        assert!(Instant::now() < deadline, "não parou: {}", state.borrow().status);
        step(ui);
    }
    let status = state.borrow().status.clone();
    assert!(status.starts_with("Parado no ponto de parada"), "{status}");
    // A janela de código no lib.rs, com a linha amarela e o ponto de parada.
    let memo = |ui: &aylis_core::Ui| -> (Option<usize>, Vec<usize>) {
        let found = ui.roots().iter().flat_map(|&r| {
            let mut all = vec![r];
            let mut i = 0;
            while i < all.len() {
                all.extend(ui.children(all[i]).iter().copied());
                i += 1;
            }
            all
        }).find_map(|id| ui.get::<Memo>(id).filter(|m| m.syntax == "rust").map(|m| (m.exec_line(), m.breakpoints().to_vec())));
        found.expect("janela de código")
    };
    assert_eq!(memo(ui), (Some(line), vec![line]));
    // A janela "Depuração" com a pilha (o app() em cima).
    let mut all: Vec<_> = ui.roots().to_vec();
    let mut i = 0;
    while i < all.len() {
        all.extend(ui.children(all[i]).iter().copied());
        i += 1;
    }
    let stack_text: String = all
        .iter()
        .filter_map(|&id| ui.get::<TreeView>(id))
        .flat_map(|t| t.nodes().iter().map(|n| n.text.clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(stack_text.contains("app"), "{stack_text}");

    // F8: a próxima linha.
    debugger::command(ui, &state, Cmd::StepOver);
    assert_eq!(memo(ui).0, None, "rodando: sem linha amarela");
    let deadline = Instant::now() + Duration::from_secs(60);
    while !state.borrow().debugger.has_stack() {
        assert!(Instant::now() < deadline, "o passo não parou");
        step(ui);
    }
    assert_eq!(memo(ui).0, Some(line + 1));

    // Ctrl+F2: para tudo.
    debugger::command(ui, &state, Cmd::Stop);
    assert!(!state.borrow().debugger.active());
    assert_eq!(memo(ui).0, None);
    let _ = std::fs::remove_dir_all(&base);
}

/// Go (as outras linguagens do aylis-host seguem o mesmo caminho): a IDE compila, executa
/// pelo aylis-host, recebe o processo da lógica, anexa o Delve e para no handler.
#[test]
#[ignore]
fn the_ide_debugs_a_go_project_by_attaching() {
    if aylis_tools::dap::delve().is_none() || aylis_tools::hostlang::find(Language::Go).is_err() || jobs::aylis_host().is_none() {
        eprintln!("sem Go, Delve ou aylis-host (cargo build -p aylis-host): teste pulado");
        return;
    }
    let base = std::env::temp_dir().join(format!("aylis_depurar_go_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = create(&base, "DepuraGo", Language::Go).unwrap();
    // Um Timer chama o handler `tique` a cada 300 ms.
    let form = project.file("forms/principal.json");
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&form).unwrap()).unwrap();
    let timer = serde_json::json!({ "class": "Timer", "name": "tmrTique", "interval": 300, "on_timer": "tique" });
    match layout["root"]["children"].as_array_mut() {
        Some(children) => children.push(timer),
        None => layout["root"]["children"] = serde_json::json!([timer]),
    }
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let source = aylis_project::codegen::form_source(&project, "frmPrincipal").unwrap();
    let text = std::fs::read_to_string(&source).unwrap();
    let stub = "tique(app *aylis.Aylis, ev aylis.Event) {\n";
    assert!(text.contains(stub), "{text}");
    let text = text.replacen(stub, &format!("{stub}\tmarca := 1\n\t_ = marca\n"), 1);
    std::fs::write(&source, &text).unwrap();
    let line = text.lines().position(|l| l.contains("marca := 1")).unwrap() + 1;

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let ui = &mut ide.ui;
    actions::open_recent(ui, &state, &ide.ids, &project.path);
    debugger::toggle_breakpoint(ui, &state, &source, line);
    debugger::start(ui, &state);
    let deadline = Instant::now() + Duration::from_secs(300);
    while !state.borrow().debugger.has_stack() {
        assert!(Instant::now() < deadline, "não parou: {}", state.borrow().status);
        jobs::drain(ui, &state, &ide.ids);
        debugger::poll(ui, &state);
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(30));
    }
    let status = state.borrow().status.clone();
    assert!(status.starts_with("Parado no ponto de parada"), "{status}");
    debugger::command(ui, &state, Cmd::Stop);
    assert!(!state.borrow().debugger.active());
    jobs::stop_all(&state);
    let _ = std::fs::remove_dir_all(&base);
}
