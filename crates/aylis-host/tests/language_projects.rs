//! Projetos da IDE em cada linguagem: cria o projeto, põe um handler no `on_show` do form
//! principal (que fecha a tela), gera o código, compila pelo `aylis-tools` e roda pelo
//! aylis-host com o projeto. Confere também que gerar de novo mantém o corpo do handler.
//! Linguagens sem compilador na máquina são puladas (variáveis: veja `linguagens.rs`).

use std::path::PathBuf;
use std::process::Command;

use aylis_project::{Language, codegen};
use aylis_tools::{Control, hostlang};

/// O corpo vazio que a IDE gera e o que o teste põe no lugar (fechar a tela).
fn close_body(lang: Language) -> (&'static str, &'static str) {
    match lang {
        Language::TypeScript => ("void aylis, ev;", "void ev; aylis.send(\"close\");"),
        Language::CSharp => ("public static void ao_abrir(Aylis aylis, AylisEvent ev)\n    {\n", "public static void ao_abrir(Aylis aylis, AylisEvent ev)\n    {\n        aylis.Send(\"close\");\n"),
        Language::Java => ("public static void ao_abrir(Aylis aylis, Aylis.Event ev) {\n", "public static void ao_abrir(Aylis aylis, Aylis.Event ev) {\n        aylis.send(\"close\");\n"),
        Language::Go => ("ao_abrir(app *aylis.Aylis, ev aylis.Event) {\n", "ao_abrir(app *aylis.Aylis, ev aylis.Event) {\n\tapp.Send(\"close\", nil)\n"),
        Language::Cpp => ("    (void)ev;\n", "    (void)ev;\n    app.send(\"close\");\n"),
        Language::Pascal => ("Event: TJSONObject);\nbegin\n", "Event: TJSONObject);\nbegin\n  Aylis.Send('close');\n"),
        _ => unreachable!(),
    }
}

fn try_language(lang: Language) {
    let Ok(tc) = hostlang::find(lang) else {
        eprintln!("{}: sem compilador, pulado", lang.name());
        return;
    };
    let base = std::env::temp_dir().join(format!("aylis_proj_{}_{}", lang.name().replace(['#', '+', ' '], "x"), std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Loja", lang).unwrap();
    // O form principal ganha o on_show.
    let form_path = project.file("forms/principal.json");
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&form_path).unwrap()).unwrap();
    layout["root"]["on_show"] = "ao_abrir".into();
    // Um ScrollBox: a classe gerada tem um método que recebe um componente (`scroll_to`), que
    // precisa compilar em cada linguagem.
    let scroll = serde_json::json!({ "class": "ScrollBox", "name": "scrLista" });
    match layout["root"]["children"].as_array_mut() {
        Some(children) => children.push(scroll),
        None => layout["root"]["children"] = serde_json::json!([scroll]),
    }
    std::fs::write(&form_path, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    codegen::generate(&project).unwrap();
    let source = codegen::form_source(&project, "frmPrincipal").unwrap();
    let text = std::fs::read_to_string(&source).unwrap();
    let (empty, close) = close_body(lang);
    assert!(text.contains(empty), "{}: o stub gerado\n{text}", lang.name());
    std::fs::write(&source, text.replacen(empty, close, 1)).unwrap();
    // Gerar de novo não perde o que o usuário escreveu.
    codegen::generate(&project).unwrap();
    let again = std::fs::read_to_string(&source).unwrap();
    assert!(again.contains(close), "{}: o corpo do handler sumiu\n{again}", lang.name());
    assert_eq!(again.matches("ao_abrir").count(), text.replacen(empty, close, 1).matches("ao_abrir").count(), "{}: sem duplicar", lang.name());

    let out = project.dir().join(".aylis/build");
    let mut lines = Vec::new();
    let control = Control::default();
    let backend = hostlang::build(&project, &tc, &out, false, &control, &mut |e| lines.push(format!("{e:?}")))
        .unwrap_or_else(|e| panic!("{}: compilação: {e}\n{}", lang.name(), lines.join("\n")));
    let png = base.join("tela.png");
    let output = Command::new(env!("CARGO_BIN_EXE_aylis-host"))
        .current_dir(project.code_dir())
        .arg("--png")
        .arg(&png)
        .arg("--project")
        .arg(&project.path)
        .arg("--")
        .args(&backend)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success() && png.is_file(), "{}: {stderr}", lang.name());
    let _ = std::fs::remove_dir_all(PathBuf::from(&base));
}

#[test]
fn typescript_project() {
    try_language(Language::TypeScript);
}

#[test]
fn csharp_project() {
    try_language(Language::CSharp);
}

#[test]
fn java_project() {
    try_language(Language::Java);
}

#[test]
fn go_project() {
    try_language(Language::Go);
}

#[test]
fn cpp_project() {
    try_language(Language::Cpp);
}

#[test]
fn pascal_project() {
    try_language(Language::Pascal);
}
