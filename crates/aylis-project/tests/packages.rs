//! Pacote de componente composto no projeto: o mesmo pacote duas vezes no form, cada uso com
//! o seu valor exposto, e a troca de versão maior renomeando os valores.

use aylis_core::{PropValue, Ui};
use aylis_project::packages::{self, Manifest};
use aylis_project::*;
use serde_json::json;

#[test]
fn the_same_package_twice_on_a_form_each_with_its_values() {
    let base = std::env::temp_dir().join(format!("aylis_project_pkg_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = create(&base, "Usina", Language::Rust).unwrap();
    let pkg = project.file("packages/Termometro");
    std::fs::create_dir_all(&pkg).unwrap();
    let manifest = json!({ "name": "Termometro", "version": "1.0.0", "author": "Ana", "kind": "composite",
        "properties": [
            { "name": "valor", "type": "number", "target": "pb.value", "default": 10 },
            { "name": "titulo", "type": "text", "target": "lbl.text", "default": "Temperatura" }
        ] });
    std::fs::write(pkg.join("package.json"), manifest.to_string()).unwrap();
    let form = json!({ "aylis": 1, "root": { "class": "Form", "name": "Termometro", "children": [
        { "class": "ProgressBar", "name": "pb" }, { "class": "Label", "name": "lbl" } ] } });
    std::fs::write(pkg.join("form.json"), form.to_string()).unwrap();
    // O form principal com dois usos.
    let main = project.file(&project.form(&project.main_form).unwrap().file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&main).unwrap()).unwrap();
    layout["root"]["children"] = json!([
        { "class": "Frame", "name": "termo1", "form": "pkg:Termometro", "values": "{\"valor\": 42}" },
        { "class": "Frame", "name": "termo2", "form": "pkg:Termometro", "values": "{\"titulo\": \"Caldeira\"}" }
    ]);
    std::fs::write(&main, layout.to_string()).unwrap();

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    install(&mut ui, &project).unwrap();
    let pb1 = ui.find_path("termo1_Termometro.pb").expect("o primeiro uso");
    let pb2 = ui.find_path("termo2_Termometro.pb").expect("o segundo uso, à parte");
    assert_eq!(ui.get_prop(pb1, "value").unwrap(), PropValue::Float(42.0));
    assert_eq!(ui.get_prop(pb2, "value").unwrap(), PropValue::Float(10.0), "o padrão do pacote");
    let lbl2 = ui.find_path("termo2_Termometro.lbl").unwrap();
    assert_eq!(ui.get_prop(lbl2, "text").unwrap(), PropValue::String("Caldeira".into()));
    assert_eq!(packages::used(&Files::Dir(project.dir())).len(), 1);
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn a_major_version_renames_and_drops_values() {
    let new: Manifest = serde_json::from_value(json!({ "name": "Termometro", "version": "2.0.0",
        "properties": [ { "name": "temperatura", "type": "number", "target": "pb.value" } ],
        "renamed": { "valor": "temperatura", "cor": "" } }))
    .unwrap();
    let (values, notes) = packages::migrate_values(r#"{"valor": 42, "cor": "red", "outra": 1}"#, &new);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&values).unwrap(), json!({ "temperatura": 42 }));
    assert_eq!(notes.len(), 3, "{notes:?}");
    assert!(packages::Version::parse("1.4.2").unwrap().compatible(packages::Version::parse("1.9").unwrap()));
    assert!(!packages::Version::parse("1.4.2").unwrap().compatible(packages::Version::parse("2.0.0").unwrap()));
}

/// Um evento exposto do pacote (`on_apertou` = o clique do botão de dentro) chama o handler
/// do form do usuário, escrito nos `values` do frame.
#[test]
fn an_exposed_event_calls_the_handler_of_the_users_form() {
    let base = std::env::temp_dir().join(format!("aylis_project_pkg_events_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = create(&base, "Usina", Language::Rust).unwrap();
    let pkg = project.file("packages/Botoeira");
    std::fs::create_dir_all(&pkg).unwrap();
    let manifest = json!({ "name": "Botoeira", "version": "1.0.0", "kind": "composite",
        "events": [ { "name": "on_apertou", "target": "btn.on_click" } ] });
    std::fs::write(pkg.join("package.json"), manifest.to_string()).unwrap();
    let form = json!({ "aylis": 1, "root": { "class": "Form", "name": "Botoeira", "children": [ { "class": "Button", "name": "btn" } ] } });
    std::fs::write(pkg.join("form.json"), form.to_string()).unwrap();
    let main = project.file(&project.form(&project.main_form).unwrap().file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&main).unwrap()).unwrap();
    let main_name = layout["root"]["name"].as_str().unwrap().to_string();
    layout["root"]["children"] = json!([ { "class": "Frame", "name": "parada", "form": "pkg:Botoeira", "values": "{\"on_apertou\": \"parada_apertou\"}" } ]);
    std::fs::write(&main, layout.to_string()).unwrap();

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let calls = std::rc::Rc::new(std::cell::Cell::new(0));
    let c = calls.clone();
    ui.register_handler(&format!("{main_name}.parada_apertou"), move |_, _, _| c.set(c.get() + 1));
    install(&mut ui, &project).unwrap();
    let btn = ui.find_path("parada_Botoeira.btn").expect("o botão de dentro");
    ui.fire(btn, "on_click", &aylis_core::EventArgs::None);
    assert_eq!(calls.get(), 1, "o handler do form do usuário");
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn a_code_package_shows_the_crate_class_and_goes_into_the_generated_rust() {
    let base = std::env::temp_dir().join(format!("aylis_project_pkg_code_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = create(&base, "Usina", Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let pkg = project.file("packages/Medidor");
    std::fs::create_dir_all(pkg.join("crate/src")).unwrap();
    // A classe do crate: aqui uma que o aylis já registra, no lugar da do pacote.
    let manifest = json!({ "name": "Medidor", "version": "1.0.0", "kind": "code", "crate": "medidor-aylis", "component": "ProgressBar",
        "properties": [{ "name": "nivel", "type": "number", "target": "value", "default": 5 }] });
    std::fs::write(pkg.join("package.json"), manifest.to_string()).unwrap();
    assert!(Manifest::parse(&manifest.to_string()).unwrap().is_code());
    assert!(Manifest::parse(&json!({ "name": "X", "version": "1.0.0", "kind": "code" }).to_string()).is_err(), "sem crate nem classe");
    let main = project.file(&project.form(&project.main_form).unwrap().file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&main).unwrap()).unwrap();
    layout["root"]["children"] = json!([{ "class": "Frame", "name": "med1", "form": "pkg:Medidor", "values": "{\"nivel\": 42}" }]);
    std::fs::write(&main, layout.to_string()).unwrap();

    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    install(&mut ui, &project).unwrap();
    let inner = ui.find_path("med1_Medidor").expect("a classe do crate dentro do frame");
    assert_eq!(ui.class_name(inner), Some("ProgressBar"));
    assert_eq!(ui.get_prop(inner, "value").unwrap(), PropValue::Float(42.0), "o valor exposto, sem componente no alvo");

    // O código gerado: a dependência (marcada) e o register do crate.
    aylis_project::codegen::generate(&project).unwrap();
    let cargo = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap();
    assert!(cargo.contains("medidor-aylis = { path = \"../packages/Medidor/crate\" }  # pacote do aylis"), "{cargo}");
    let forms = std::fs::read_to_string(project.code_dir().join("generated/mod.rs")).unwrap();
    assert!(forms.contains("    medidor_aylis::register(ui);"), "{forms}");
    // Gerar de novo não duplica; sem o pacote, a linha sai.
    aylis_project::codegen::generate(&project).unwrap();
    assert_eq!(std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap().matches("medidor-aylis").count(), 1);
    std::fs::remove_dir_all(&pkg).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    assert!(!std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap().contains("medidor-aylis"));
    std::fs::remove_dir_all(&base).unwrap();
}
