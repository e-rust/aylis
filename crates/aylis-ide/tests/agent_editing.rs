//! A IDE com um agente (MCP) editando os mesmos fontes que o usuário (relatado pelo autor em
//! 24/11): o F12 no fonte que o agente acabou de gravar por fora apagava a linha dele, e o
//! Ctrl+Z do usuário desfazia o que o agente fez.

use std::path::{Path, PathBuf};

use aylis_core::{KeyEvent, PointF, PropValue, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app, codewindow};
use serde_json::json;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_ide_agent_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Grava por fora, como a ferramenta Edit de um agente (com uma pausa, para a marca mudar).
fn write_outside(file: &Path, text: &str) {
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(file, text).unwrap();
}

/// Fecha o "Confiar neste projeto?" sem confiar (o projeto novo de teste; editar não precisa).
fn decline_trust(ui: &mut Ui) {
    for form in ui.open_forms() {
        if ui.get_prop(form, "caption").is_ok_and(|c| c == PropValue::String("Confiar neste projeto?".into())) {
            ui.set_modal_result(form, aylis_core::ModalResult::Cancel);
        }
    }
}

/// O código embutido à vista (o editor escondido não recebe o teclado).
fn show_code(ide: &mut app::Ide, state: &aylis_ide::state::Shared) {
    state.borrow_mut().code_view = true;
    app::refresh(&mut ide.ui, state, &ide.ids);
    ide.ui.layout();
}

fn key(ui: &mut Ui, form: aylis_core::ComponentId, shortcut: &str) {
    let (modifiers, key) = aylis_core::parse_shortcut(shortcut).unwrap();
    let event = KeyEvent { key, modifiers, repeat: false, form: None };
    ui.key_down(form, event);
    ui.key_up(form, event);
}

/// O fonte aberto e sem alteração; o agente grava uma linha por fora; o F12 (que regrava o
/// fonte com o handler novo) e o salvar: a linha do agente continua, no editor e no disco.
#[test]
fn f12_and_save_keep_a_line_the_agent_wrote_outside() {
    let base = temp("f12");
    let project = aylis_project::create(&base, "Corrida", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    decline_trust(&mut ide.ui);
    actions::open_code(&mut ide.ui, &state, None);
    let source = aylis_project::codegen::form_source(&project, &project.main_form).unwrap();
    let source = aylis_ide::debugger::norm(&source);
    let opened = codewindow::text_of(&ide.ui, &source).expect("o fonte do form aberto");

    // O agente grava por fora; a IDE ainda não conferiu o disco (nem o foco nem o MCP).
    write_outside(&source, &format!("{opened}\n// linha do agente\n"));
    // Um botão com evento: o F12 acrescenta o handler dele no fonte.
    {
        let mut st = state.borrow_mut();
        let button = st.doc.add("Button", Some(PointF::new(40.0, 40.0))).unwrap();
        st.doc.ui.set_prop(button, "on_click", PropValue::Event(Some("btnCorridaClick".into()))).unwrap();
        st.doc.modified = true;
    }
    actions::open_code(&mut ide.ui, &state, None);
    let text = codewindow::text_of(&ide.ui, &source).unwrap();
    assert!(text.contains("// linha do agente"), "o editor perdeu a linha do agente:\n{text}");
    assert!(text.contains("btn_corrida_click") || text.contains("btnCorridaClick"), "o handler novo no editor:\n{text}");

    // O salvar (Ctrl+S, o antes de Executar): a linha continua no disco.
    codewindow::save_embedded(&mut ide.ui, true);
    codewindow::save_file(&mut ide.ui, &source).unwrap();
    let disk = std::fs::read_to_string(&source).unwrap();
    assert!(disk.contains("// linha do agente"), "o salvar apagou a linha do agente:\n{disk}");

    // Sem o F12: o disco muda de novo por fora e o salvar não grava o texto velho por cima.
    write_outside(&source, &format!("{disk}// outra do agente\n"));
    assert!(codewindow::save_file(&mut ide.ui, &source).unwrap());
    let disk = std::fs::read_to_string(&source).unwrap();
    assert!(disk.contains("// outra do agente"), "{disk}");
    assert!(codewindow::text_of(&ide.ui, &source).unwrap().contains("// outra do agente"), "o editor recarregou");
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

/// Alteração dos dois lados: o salvar não grava por cima; pelo MCP, o erro leva o diff.
#[test]
fn saving_over_a_file_changed_outside_becomes_a_conflict() {
    let base = temp("conflito");
    let project = aylis_project::create(&base, "Conflito", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    decline_trust(&mut ide.ui);
    let main = aylis_ide::debugger::norm(&project.code_dir().join("src").join("main.rs"));
    let disk = std::fs::read_to_string(&main).unwrap();
    let first = disk.lines().next().unwrap().to_string();
    codewindow::edit_text(&mut ide.ui, &main, &first, "// do usuário", false).unwrap();
    write_outside(&main, &format!("{disk}// do agente\n"));

    let refused = codewindow::save_file(&mut ide.ui, &main).unwrap_err();
    assert!(refused.contains("resolve_conflict") && refused.contains("+// do agente"), "{refused}");
    assert!(std::fs::read_to_string(&main).unwrap().contains("// do agente"), "não gravou por cima");
    // Pela tela (o salvar de todos): também não grava por cima.
    codewindow::save_all(&mut ide.ui);
    assert!(std::fs::read_to_string(&main).unwrap().contains("// do agente"), "não gravou por cima");
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}

/// A IDE aberta por um agente: o fonte do usuário abre travado para o teclado (com a faixa e o
/// "Liberar edição"); o `edit_code` e o `editor_type` do agente editam; liberado, o Ctrl+Z do
/// usuário não desfaz o que o agente fez.
#[test]
fn opened_by_an_agent_the_sources_are_read_only_for_the_user() {
    codewindow::set_opened_by_agent(true);
    let base = temp("travado");
    let project = aylis_project::create(&base, "Travado", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    actions::open_project(&mut ide.ui, &state, &ide.ids, &project.path);
    decline_trust(&mut ide.ui);
    let main = aylis_ide::debugger::norm(&project.code_dir().join("src").join("main.rs"));
    let w = codewindow::open(&mut ide.ui, &main, 1).unwrap();
    show_code(&mut ide, &state);
    assert!(codewindow::is_locked_for_user(&ide.ui, &main));
    let banner_visible = |ui: &Ui| {
        let mut stack = ui.children(w.form).to_vec();
        while let Some(id) = stack.pop() {
            if ui.get::<aylis_controls::Button>(id).is_some_and(|b| b.text == "Liberar edição") {
                return ui.parent(id).and_then(|p| ui.control(p)).is_some_and(|c| c.visible());
            }
            stack.extend(ui.children(id).iter().copied());
        }
        false
    };
    assert!(banner_visible(&ide.ui), "a faixa com o Liberar edição");
    let opened = codewindow::text_of(&ide.ui, &main).unwrap();

    // O teclado do usuário não muda nada.
    ide.ui.set_focus(w.memo);
    ide.ui.text_input(ide.form, "x");
    assert_eq!(codewindow::text_of(&ide.ui, &main).unwrap(), opened, "travado para o usuário");

    // O agente edita (o edit_code e o editor_type).
    let first = opened.lines().next().unwrap().to_string();
    let edited = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "edit_code", &json!({ "file": main.display().to_string(), "old": first, "new": "// do agente" }));
    assert!(edited.is_ok(), "{edited:?}");
    ide.ui.set_focus(w.memo);
    let typed = aylis_ide::mcp::run_tool(&mut ide.ui, &state, &ide.ids, "editor_type", &json!({ "text": "z" }));
    assert!(typed.is_ok(), "{typed:?}");
    let by_agent = codewindow::text_of(&ide.ui, &main).unwrap();
    assert!(by_agent.contains("// do agente") && by_agent.contains('z'), "{by_agent}");
    assert!(codewindow::is_locked_for_user(&ide.ui, &main), "continua travado para o usuário");

    // O Ctrl+Z do usuário, travado, não desfaz o agente.
    ide.ui.set_focus(w.memo);
    key(&mut ide.ui, ide.form, "Ctrl+Z");
    assert_eq!(codewindow::text_of(&ide.ui, &main).unwrap(), by_agent);

    // Liberado: digita, e o Ctrl+Z desfaz só o que o usuário fez.
    assert!(codewindow::release_editing(&mut ide.ui, &main));
    assert!(!codewindow::is_locked_for_user(&ide.ui, &main));
    assert!(!banner_visible(&ide.ui), "a faixa some");
    ide.ui.set_focus(w.memo);
    key(&mut ide.ui, ide.form, "Ctrl+Z");
    assert_eq!(codewindow::text_of(&ide.ui, &main).unwrap(), by_agent, "nada do agente para desfazer");
    ide.ui.text_input(ide.form, "u");
    assert_ne!(codewindow::text_of(&ide.ui, &main).unwrap(), by_agent);
    key(&mut ide.ui, ide.form, "Ctrl+Z");
    key(&mut ide.ui, ide.form, "Ctrl+Z");
    assert_eq!(codewindow::text_of(&ide.ui, &main).unwrap(), by_agent, "desfez só o do usuário");
    drop(ide);
    codewindow::set_opened_by_agent(false);
    let _ = std::fs::remove_dir_all(&base);
}

/// Aberta pelo usuário (sem o MCP da ponte), nada muda: o fonte abre livre.
#[test]
fn opened_by_the_user_the_sources_are_editable() {
    codewindow::set_opened_by_agent(false);
    let base = temp("livre");
    let file = base.join("livre.rs");
    std::fs::write(&file, "fn main() {}\n").unwrap();
    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    ide.ui.show(ide.form).unwrap();
    let w = codewindow::open(&mut ide.ui, &file, 1).unwrap();
    show_code(&mut ide, &state);
    assert!(!codewindow::is_locked_for_user(&ide.ui, &file));
    ide.ui.set_focus(w.memo);
    ide.ui.text_input(ide.form, "// x\n");
    assert!(codewindow::text_of(&ide.ui, &file).unwrap().starts_with("// x"), "{:?}", codewindow::text_of(&ide.ui, &file));
    drop(ide);
    let _ = std::fs::remove_dir_all(&base);
}
