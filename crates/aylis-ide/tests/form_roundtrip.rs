//! Forms em JSON bons para merge (item 22): abrir um form de exemplo na IDE e salvar sem mudar
//! nada dá o mesmo arquivo, byte a byte (forms de outra origem: `aylis-ide --format-forms`). Com
//! `AYLIS_FORM_DIFF=1`, mostra a primeira diferença de cada form que muda.

use std::path::Path;

use aylis_ide::actions::project_document;
use aylis_project::Project;

/// Os projetos de exemplo (cada `examples/<x>/<x>.aylisproj`).
fn projects() -> Vec<Project> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&root).unwrap().flatten() {
        let dir = entry.path();
        let Ok(files) = std::fs::read_dir(&dir) else { continue };
        for f in files.flatten() {
            if f.path().extension().is_some_and(|e| e == "aylisproj") {
                if let Ok(p) = Project::open(f.path()) {
                    out.push(p);
                }
            }
        }
    }
    out.sort_by_key(|p| p.dir());
    out
}

#[test]
fn opening_and_saving_an_example_form_changes_nothing() {
    let show = std::env::var_os("AYLIS_FORM_DIFF").is_some();
    let mut checked = 0;
    let mut changed = Vec::new();
    for project in projects() {
        // O ERP tem mil forms gerados do mesmo jeito: uma amostra basta (todos levam minutos).
        let step = if project.forms.len() > 100 { 20 } else { 1 };
        for form in project.forms.iter().step_by(step) {
            let path = project.file(&form.file);
            let Ok(original) = std::fs::read_to_string(&path) else { continue };
            let doc = match project_document(&project, &form.name, None) {
                Ok(d) => d,
                Err(e) => panic!("{}: {e}", path.display()),
            };
            let saved = doc.to_json().unwrap();
            checked += 1;
            let original = original.replace("\r\n", "\n");
            if saved != original {
                if show && changed.len() < 15 {
                    let (a, b): (Vec<&str>, Vec<&str>) = (original.lines().collect(), saved.lines().collect());
                    let i = a.iter().zip(&b).position(|(x, y)| x != y).unwrap_or(a.len().min(b.len()));
                    println!("{}:{}\n  arquivo: {}\n  salvo:   {}", path.display(), i + 1, a.get(i).unwrap_or(&""), b.get(i).unwrap_or(&""));
                }
                changed.push(path.display().to_string());
            }
        }
    }
    println!("{checked} forms; {} mudam ao salvar", changed.len());
    assert!(checked > 0);
    assert!(changed.is_empty(), "{} forms mudam ao abrir e salvar: {:?}", changed.len(), changed.iter().take(20).collect::<Vec<_>>());
}
