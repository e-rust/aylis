//! Os projetos de exemplo usam o acesso tipado (pedido do autor em 07/11: "todos os projetos
//! de exemplo estão usando os componentes tipados? Sem localizar pelo nome"). O código do
//! usuário, nas 8 linguagens, não procura componente pelo nome: nada de `find_in`,
//! `find_path`, `FindButton("...")` nem nome de componente entre aspas. O gerado pela IDE
//! (`generated/`, `components.*`, o registro) fica de fora: ele é o próprio acesso tipado.

use std::path::{Path, PathBuf};

/// As extensões do código do usuário em cada linguagem.
const SOURCES: [&str; 10] = ["rs", "py", "ts", "cs", "java", "go", "cpp", "hpp", "pas", "lpr"];

/// O que é gerado pela IDE (o acesso tipado e o registro) ou não é código do app.
const GENERATED: [&str; 12] = [
    "generated", "target", "node_modules", "bin", "obj", "tests", "components.ts", "components.py", "Components.cs", "Components.java", "components.go", "components.hpp",
];
const GENERATED_FILES: [&str; 7] = ["ayliscomponents.pas", "forms.go", "forms.hpp", "AllForms.cs", "Forms.java", "__init__.py", "index.ts"];

/// Os prefixos dos nomes de componente nos exemplos (`lblVisor`, `cdsMestre`...).
const PREFIXES: [&str; 30] = [
    "lbl", "btn", "edt", "cds", "ds", "grd", "tb", "pnl", "lv", "cb", "chk", "mmo", "img", "pc", "flu", "fon", "wb", "lay", "md", "ntf", "cam", "mic", "m3d", "nb", "tab",
    "seg", "lvl", "vp", "svc", "frm",
];

/// O que fica, com o motivo: a tabela das 1009 telas do ERP (o nome do form e o do dataset
/// são dados dela, que o menu mostra e o delta traz), não procura de componente.
const ALLOWED: [&str; 2] = ["ERP/rust/src/erp/esquema.rs", "ERP/rust/src/erp/banco.rs"];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if GENERATED.contains(&name.as_str()) || GENERATED_FILES.contains(&name.as_str()) {
            continue;
        }
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| SOURCES.contains(&e.to_string_lossy().as_ref())) {
            out.push(path);
        }
    }
}

/// Um nome de componente entre aspas na linha (`"btnLapis"`, `'cdsEstados'`, `"frmX.cdsY"`).
fn quoted_component(line: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let mut parts = line.split(quote);
        parts.next();
        while let Some(inside) = parts.next() {
            let starts = PREFIXES.iter().any(|p| inside.strip_prefix(p).is_some_and(|rest| rest.chars().next().is_some_and(char::is_uppercase)));
            let ident = !inside.is_empty() && inside.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.');
            if starts && ident {
                return Some(inside.to_string());
            }
            parts.next();
        }
    }
    None
}

#[test]
fn the_examples_reach_components_by_typed_access_not_by_name() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut files = Vec::new();
    sources(&root, &mut files);
    assert!(files.len() > 50, "achou o código dos exemplos: {}", files.len());
    let lookups = ["find_in(", "find_path(", "ui.find(", "FindButton(", "findButton(", "find_button(", "FindCircle(", "findCircle(", "find_circle(", "FindComponent("];
    let mut found = Vec::new();
    for file in files {
        let rel = file.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
        if ALLOWED.contains(&rel.as_str()) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        for (i, line) in text.lines().enumerate() {
            let code = line.trim_start();
            // O registro da classe do form em Pascal (`RegisterForm(TFrmX, 'frmX', ...)`)
            // dá o nome ao form; não procura componente.
            if code.starts_with("//") || code.starts_with('#') || code.starts_with('{') || code.starts_with("///") || code.starts_with("/*") || code.starts_with('*') || code.starts_with("RegisterForm(") {
                continue;
            }
            let lookup = lookups.iter().find(|l| line.contains(*l)).map(|l| l.to_string());
            if let Some(what) = lookup.or_else(|| quoted_component(line)) {
                found.push(format!("{rel}:{}: {what}", i + 1));
            }
        }
    }
    assert!(found.is_empty(), "código de exemplo achando componente pelo nome (use o acesso tipado):\n{}", found.join("\n"));
}
