//! Renomear em todo o projeto (item 19): o designer acerta o próprio form (inclusive as
//! vistas, que guardam o nome) e a prévia acha o resto: o código do usuário (sem o gerado) e
//! os outros forms; renomear o form troca também o tipo dele no código (`FrmPrincipal`).

use std::path::Path;

use aylis_ide::document::Document;
use aylis_ide::rename::{self, Renamed};
use aylis_project::Project;

#[test]
fn renaming_in_the_designer_follows_in_the_views_of_the_form() {
    let json = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmX", "width": 900,
        "views": [{ "name": "phone", "max_width": 599, "changes": { "edtNome": { "width": 120 } } }],
        "children": [{ "class": "Edit", "name": "edtNome", "width": 300 }] } }"#;
    let mut doc = Document::from_json(json).unwrap();
    let id = doc.ui.find_in(doc.root, "edtNome").unwrap();
    doc.rename_component(id, "edtCliente").unwrap();
    let saved = doc.to_json().unwrap();
    assert!(saved.contains("\"edtCliente\": {") && !saved.contains("edtNome"), "{saved}");
}

#[test]
fn the_preview_finds_the_user_code_and_the_type_of_a_renamed_form() {
    let project = Project::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Relatorios/Relatorios.aylisproj")).unwrap();
    // Um componente: o nome no código do usuário; o gerado fica de fora.
    let plan = rename::plan(&project, "frmPrincipal", Renamed::Component, "cdsPedidos").unwrap();
    assert!(plan.text.iter().any(|h| h.text.contains("self.cdsPedidos")), "{:?}", plan.text);
    assert!(plan.text.iter().all(|h| !h.file.components().any(|c| c.as_os_str() == "generated")));
    assert!(plan.forms.is_empty(), "o próprio form já está certo na memória: {:?}", plan.forms);
    // O form: o tipo no código, com a maiúscula.
    let plan = rename::plan(&project, "frmRelatorios", Renamed::Form, "frmPrincipal").unwrap();
    let lines: Vec<&str> = plan.text.iter().map(|h| h.text.as_str()).collect();
    assert!(lines.iter().any(|l| l.contains("impl FrmPrincipal")), "{lines:?}");
    let options = rename::options(Renamed::Form, "frmPrincipal");
    assert_eq!(options.replace_all("impl FrmPrincipal {", "frmRelatorio").unwrap().0, "impl FrmRelatorio {");
}

/// O renomear pelo servidor de linguagem (decidido pelo autor: "LSP como o planejado"), com o
/// rust-analyzer de verdade no exemplo Abas: o `textDocument/rename` do `tbsAbas` (perguntado
/// no primeiro uso do código do usuário) acha os usos do código do usuário e a declaração no
/// gerado; a prévia fica só com os do usuário, cada um com a troca exata. Nada é gravado.
/// Demora (o servidor indexa): `cargo test -p aylis-ide --test rename language_server -- --ignored --nocapture`.
#[test]
#[ignore]
fn the_language_server_finds_the_code_to_rename() {
    use std::time::{Duration, Instant};
    use aylis_tools::lsp::{self, Client, Incoming};
    let dir = std::fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Abas/rust")).unwrap();
    let dir = std::path::PathBuf::from(dir.display().to_string().trim_start_matches(r"\?\"));
    let Some(server) = lsp::server(aylis_project::Language::Rust, &dir) else {
        println!("sem rust-analyzer");
        return;
    };
    lsp::prepare(aylis_project::Language::Rust, &dir);
    let mut client = Client::start(&server, &dir, None).unwrap();
    let file = dir.join("src").join("forms").join("principal.rs");
    let text = std::fs::read_to_string(&file).unwrap();
    client.open(&file, &text).unwrap();
    let at = lsp::position_in(&text, text[..text.find("tbsAbas").unwrap()].chars().count());
    let mut params = lsp::at(&file, at);
    params["newName"] = serde_json::Value::String("tbsPaginas".into());
    // Pergunta até o servidor terminar de indexar e responder com as trocas.
    let started = Instant::now();
    let mut edits = Vec::new();
    while edits.is_empty() && started.elapsed() < Duration::from_secs(300) {
        let id = client.request("textDocument/rename", params.clone()).unwrap();
        let asked = Instant::now();
        'wait: while asked.elapsed() < Duration::from_secs(60) {
            for msg in client.poll() {
                if let Incoming::Response { id: got, result } = msg {
                    if got == id {
                        edits = result.map(|v| lsp::workspace_edit(&v)).unwrap_or_default();
                        break 'wait;
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        if edits.is_empty() {
            std::thread::sleep(Duration::from_secs(3));
        }
    }
    client.stop();
    println!("{:.1} s: {:?}", started.elapsed().as_secs_f32(), edits.iter().map(|(f, e)| (f.file_name().unwrap().to_string_lossy().to_string(), e.len())).collect::<Vec<_>>());
    assert!(edits.iter().any(|(f, _)| f.components().any(|c| c.as_os_str() == "generated")), "a declaração está no gerado");
    let ui = aylis_core::Ui::new();
    let hits = rename::hits_of(&ui, &edits);
    assert!(!hits.is_empty() && hits.iter().all(|h| !h.file.components().any(|c| c.as_os_str() == "generated")), "a prévia só com o código do usuário");
    let source = std::fs::read_to_string(&file).unwrap();
    for h in &hits {
        assert_eq!(&source[h.byte_start..h.byte_end], "tbsAbas", "o trecho exato: {h:?}");
        assert_eq!(h.replace_with.as_deref(), Some("tbsPaginas"));
    }
    assert_eq!(hits.len(), source.matches("self.tbsAbas").count(), "cada uso do código do usuário");
}
