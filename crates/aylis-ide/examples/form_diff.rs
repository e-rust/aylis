//! Ferramenta de investigação do item 22: para cada form de exemplo (menos o ERP inteiro),
//! o que muda no CONTEÚDO (sem contar a ordem das chaves) ao abrir e salvar na IDE.

use std::path::Path;

use aylis_ide::actions::project_document;
use aylis_project::Project;
use serde_json::Value;

fn diff(path: &str, a: &Value, b: &Value, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for (k, v) in x {
                match y.get(k) {
                    Some(w) => diff(&format!("{path}.{k}"), v, w, out),
                    None => out.push(format!("{path}.{k}: some no salvo (era {v})")),
                }
            }
            for (k, w) in y {
                if !x.contains_key(k) {
                    out.push(format!("{path}.{k}: aparece no salvo ({w})"));
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                let label = v.get("name").and_then(Value::as_str).map_or(format!("{i}"), str::to_string);
                diff(&format!("{path}[{label}]"), v, w, out);
            }
        }
        (Value::Number(x), Value::Number(y)) if x.as_f64() == y.as_f64() => {}
        _ if a != b => out.push(format!("{path}: {a} → {b}")),
        _ => {}
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for entry in std::fs::read_dir(&root).unwrap().flatten() {
        for f in std::fs::read_dir(entry.path()).into_iter().flatten().flatten() {
            if f.path().extension().is_none_or(|e| e != "aylisproj") {
                continue;
            }
            let Ok(project) = Project::open(f.path()) else { continue };
            let erp = project.dir().ends_with("ERP");
            for form in project.forms.iter().take(if erp { 12 } else { usize::MAX }) {
                let path = project.file(&form.file);
                let original: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
                let saved: Value = serde_json::from_str(&project_document(&project, &form.name, None).unwrap().to_json().unwrap()).unwrap();
                let mut out = Vec::new();
                diff("", &original, &saved, &mut out);
                if !out.is_empty() {
                    println!("== {}", path.display());
                    for d in out.iter().take(12) {
                        println!("   {d}");
                        let key = d.split(':').next().unwrap_or("").rsplit('.').next().unwrap_or("").to_string();
                        *kinds.entry(key).or_default() += 1;
                    }
                }
            }
        }
    }
    println!("\nPor chave: {kinds:?}");
}
