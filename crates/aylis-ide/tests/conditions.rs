//! Pontos de parada com condição e contagem de passagens na IDE.

use aylis_ide::debugger;
use aylis_ide::document::Document;
use aylis_ide::state::shared;

#[test]
fn conditions_are_kept_per_line_and_removed_with_the_breakpoint() {
    let state = shared(Document::new());
    let mut ui = aylis_core::Ui::new();
    let file = std::env::temp_dir().join(format!("aylis_cond_{}.rs", std::process::id()));
    std::fs::write(&file, "fn main() {}\n").unwrap();
    debugger::set_condition(&mut ui, &state, &file, 1, "i == 3", ">= 2");
    assert_eq!(debugger::breakpoints_of(&state, &file), [1], "a condição cria o ponto");
    assert_eq!(debugger::condition_of(&state, &file, 1), ("i == 3".to_string(), ">= 2".to_string()));
    // Desligar o ponto apaga a condição.
    debugger::toggle_breakpoint(&mut ui, &state, &file, 1);
    assert!(debugger::breakpoints_of(&state, &file).is_empty());
    assert_eq!(debugger::condition_of(&state, &file, 1), (String::new(), String::new()));
    let _ = std::fs::remove_file(file);
}

/// A janela de código aberta por um caminho com ".." (um projeto da lista de recentes)
/// mostra o ponto de parada ligado pelo caminho normal, e vice-versa.
#[test]
fn the_code_window_shows_breakpoints_whatever_the_path_spelling() {
    let state = shared(Document::new());
    let mut ui = aylis_core::Ui::new();
    let dir = std::env::temp_dir().join(format!("aylis_bp_path_{}", std::process::id()));
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    let file = dir.join("forms.pas");
    std::fs::write(&file, "begin\n  x := 1;\nend.\n").unwrap();
    let odd = dir.join("sub").join("..").join("forms.pas");
    let window = aylis_ide::codewindow::open(&mut ui, &odd, 1).unwrap();
    debugger::toggle_breakpoint(&mut ui, &state, &file, 2);
    let memo = ui.get::<aylis_controls::Memo>(window.memo).unwrap();
    assert_eq!(memo.breakpoints(), [2]);
    assert_eq!(debugger::breakpoints_of(&state, &odd), [2]);
    let _ = std::fs::remove_dir_all(dir);
}
