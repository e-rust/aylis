//! `use aylis::prelude::*` junto de `use crate::generated::*` num projeto de verdade: nenhum
//! registro nem enum do acesso tipado pode ter o nome de algo do `prelude`, senão o nome fica
//! ambíguo e o código do app não compila (como o `ColumnSpec` do `aylis-dbcontrols` e o
//! tipado, antes de o `prelude` passar a uma lista explícita). Também pega um registro com o
//! nome de uma classe (o `TabItem` do `TabStrip`, hoje `TabStripItem`) e dois métodos gerados
//! com o mesmo nome.
//!
//! Um form com todas as classes (todos os registros e enums gerados), e cada um citado pelo
//! nome com os dois `use *`. Compila o projeto (`cargo check`).

use aylis_project::{Language, create};
use aylis_tools::process::{Control, Event, cargo};

#[test]
fn the_typed_records_and_enums_do_not_clash_with_the_prelude() {
    let base = std::env::temp_dir().join(format!("aylis_prelude_names_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let mut project = create(&base, "Nomes", Language::Rust).unwrap();
    // O Aylis deste repositório (o padrão é o do crates.io, que não tem a mudança em teste).
    project.sdk = aylis_project::codegen::aylis_root().to_string();
    project.save().unwrap();
    // O form principal com um componente de cada classe (as raízes ficam de fora).
    let children: Vec<serde_json::Value> = aylis_build::CLASSES
        .iter()
        .filter(|(class, _)| !matches!(*class, "Form" | "DataModule" | "Frame"))
        .enumerate()
        .map(|(i, (class, _))| serde_json::json!({ "class": class, "name": format!("c{i}") }))
        .collect();
    let form = &project.forms[0];
    let json = serde_json::json!({ "aylis": 1, "root": { "class": "Form", "name": form.name, "children": children } });
    std::fs::write(project.file(&form.file), serde_json::to_string_pretty(&json).unwrap()).unwrap();
    aylis_project::codegen::generate(&project).unwrap();

    // Os registros e os enums do acesso tipado: os `pub struct`/`pub enum` do topo do arquivo
    // gerado (os forms e as classes ficam nos arquivos incluídos).
    let code = project.code_dir();
    let generated = std::fs::read_to_string(code.join("generated").join("components.rs")).unwrap();
    let names: Vec<&str> = generated
        .lines()
        .filter_map(|l| l.strip_prefix("pub struct ").or_else(|| l.strip_prefix("pub enum ")))
        .map(|rest| rest.split(|c: char| !c.is_alphanumeric() && c != '_').next().unwrap_or_default())
        .filter(|n| !n.is_empty())
        .collect();
    assert!(names.contains(&"ColumnSpec") && names.contains(&"ChartSeries"), "os registros do DataGrid e do Chart: {names:?}");
    let uses: String = names.iter().map(|n| format!("        let _: Option<{n}> = None;\n")).collect();
    let lib = code.join("src").join("lib.rs");
    let mut text = std::fs::read_to_string(&lib).unwrap();
    text.push_str(&format!(
        "\n/// Os dois `use *` juntos, como num app (o teste `prelude_names` do `aylis-tools`).\n\
         #[allow(dead_code, unused_imports)]\nmod prelude_and_generated {{\n    use aylis::prelude::*;\n    use crate::generated::*;\n\n    \
         pub fn names() {{\n{uses}    }}\n}}\n"
    ));
    std::fs::write(&lib, text).unwrap();

    let mut lines = Vec::new();
    let result = cargo("cargo", &["check", "--lib"], &code, &[], &Control::new(), &mut |e| {
        match e {
            Event::Line(line, _) => lines.push(line),
            Event::Diagnostic(d) => lines.push(format!("{d:?}")),
            Event::Step(_) => {}
        }
    });
    let errors: Vec<&String> = lines.iter().filter(|l| l.contains("rror") || l.contains("ambiguous")).collect();
    assert!(result.is_ok(), "{result:?}\n{}", errors.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"));
    let _ = std::fs::remove_dir_all(&base);
}
