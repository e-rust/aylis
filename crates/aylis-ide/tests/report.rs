//! O relato de problema (`.aylisreport`): dois cliques no arquivo abrem o visualizador com o
//! problema, quem relatou, a máquina e o projeto, e os botões dos anexos.

use aylis_controls::{Button, Memo};
use aylis_core::{ComponentId, PropValue, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{actions, app};
use aylis_tools::report::{self, Problem, Report, Reporter};
use serde_json::{Map, json};

fn descendants(ui: &Ui, id: ComponentId, out: &mut Vec<ComponentId>) {
    for &c in ui.children(id) {
        out.push(c);
        descendants(ui, c, out);
    }
}

#[test]
fn opening_a_report_file_shows_the_report_and_its_attachments() {
    let dir = std::env::temp_dir().join(format!("aylis_ide_report_{}", std::process::id()));
    let path = dir.join("relato-1.aylisreport");
    let mut project = Map::new();
    project.insert("name".into(), json!("Vendas"));
    let r = Report {
        problem: Problem { what: "o botão some ao redimensionar".into(), expected: "continuar".into(), steps: String::new() },
        reporter: Reporter { name: "Ana".into(), contact: "ana@exemplo.com".into() },
        created_ms: 1_700_000_000_000,
        machine: report::machine_info(),
        project: Some(project),
        files: vec![("tela.png".into(), vec![0; 10]), ("ide.aylisrec".into(), b"{}".to_vec())],
    };
    report::write(&path, &r).unwrap();

    let state = shared(Document::new());
    let mut ide = app::build(state.clone());
    let before = ide.ui.roots().len();
    actions::open_recent(&mut ide.ui, &state, &ide.ids, &path);
    assert_eq!(ide.ui.roots().len(), before + 1, "o visualizador abriu");
    let viewer = *ide.ui.roots().last().unwrap();
    let mut all = Vec::new();
    descendants(&ide.ui, viewer, &mut all);
    let text = all
        .iter()
        .find(|&&c| ide.ui.get::<Memo>(c).is_some())
        .and_then(|&c| match ide.ui.get_prop(c, "text") {
            Ok(PropValue::String(s)) => Some(s),
            _ => None,
        })
        .unwrap();
    for want in ["o botão some", "ana@exemplo.com", "Máquina", "Vendas", "tela.png"] {
        assert!(text.contains(want), "falta {want:?}:\n{text}");
    }
    let buttons: Vec<String> = all
        .iter()
        .filter(|&&c| ide.ui.get::<Button>(c).is_some())
        .filter_map(|&c| match ide.ui.get_prop(c, "text") {
            Ok(PropValue::String(s)) => Some(s),
            _ => None,
        })
        .collect();
    assert!(buttons.iter().any(|b| b == "Extrair os anexos") && buttons.iter().any(|b| b == "Ver a gravação"), "{buttons:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_session_that_ended_in_a_fatal_error_is_offered_for_reporting() {
    use aylis_ide::reportview::previous_crash_in;
    let dir = std::env::temp_dir().join(format!("aylis_ide_crash_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let crashed = dir.join("ide-1.aylisrec");
    std::fs::write(&crashed, "{\"type\":\"start\"}\n{\"t\":3.0,\"type\":\"error\",\"fatal\":true,\"message\":\"index out of bounds\",\"location\":\"src/x.rs:10\"}\n").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(30));
    let started = std::time::SystemTime::now();
    // A gravação desta sessão (mais nova) não conta.
    std::thread::sleep(std::time::Duration::from_millis(30));
    std::fs::write(dir.join("ide-2.aylisrec"), "{\"type\":\"start\"}\n").unwrap();
    let crash = previous_crash_in(&dir, started).expect("a anterior quebrou");
    assert_eq!(crash.recording, crashed);
    assert!(crash.message.contains("index out of bounds") && crash.message.contains("src/x.rs:10"), "{}", crash.message);
    // Uma sessão que terminou bem não é oferecida.
    std::fs::write(&crashed, "{\"type\":\"start\"}\n{\"t\":3.0,\"type\":\"click\"}\n").unwrap();
    assert!(previous_crash_in(&dir, std::time::SystemTime::now()).is_none());
    let _ = std::fs::remove_dir_all(&dir);
}
