//! Fontes e ícones do projeto, a partir dos recursos offline da IDE (`resources/`).

use std::path::PathBuf;

use aylis_core::Ui;
use aylis_project::assets;
use aylis_project::*;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_assets_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

/// Os recursos são baixados por `aylis-tool resources`; sem eles o teste não tem o que ver.
fn has_resources() -> bool {
    let ok = codegen::resources_dir().join("fonts/catalog.json").exists() && codegen::resources_dir().join("icons/material.txt").exists();
    if !ok {
        eprintln!("sem resources/ (rode aylis-tool resources): teste pulado");
    }
    ok
}

#[test]
fn a_catalog_font_goes_to_the_project_with_its_license_and_becomes_the_project_font() {
    if !has_resources() {
        return;
    }
    let base = temp("fonts");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let catalog = assets::font_catalog();
    assert!(catalog.len() >= 20, "{}", catalog.len());
    let inter = catalog.iter().find(|e| e.folder == "inter").expect("Inter no catálogo");
    assert_eq!(inter.license, "SIL OFL 1.1");

    assets::add_font(&project, inter).unwrap();
    let dir = project.dir().join("assets/fonts/inter");
    assert!(dir.join("OFL.txt").is_file());
    for f in &inter.files {
        assert!(dir.join(f).is_file(), "{f}");
    }
    assert_eq!(assets::project_fonts(&project), ["inter"]);

    // Registrada ao instalar; a fonte do projeto troca a do tema, menos a monoespaçada.
    project.font = "Inter".into();
    project.save().unwrap();
    let reopened = Project::open(&project.path).unwrap();
    assert_eq!(reopened.font, "Inter");
    let mut ui = ui();
    install(&mut ui, &reopened).unwrap();
    assert!(aylis_core::fonts::families().contains(&"Inter".to_string()), "{:?}", aylis_core::fonts::families());
    let theme = ui.theme();
    assert!(theme.fonts.values().any(|f| f.family == "Inter"));
    assert!(theme.fonts.values().all(|f| f.family == "Inter" || f.family.to_lowercase().contains("mono")));

    assets::remove_font(&project, "inter").unwrap();
    assert!(assets::project_fonts(&project).is_empty());
}

#[test]
fn the_app_gets_only_the_icons_it_cites() {
    if !has_resources() {
        return;
    }
    let base = temp("icons");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let code = project.code_dir().join("src/forms/principal.rs");
    let mut text = std::fs::read_to_string(&code).unwrap();
    text.push_str("\n// ícones: \"material:home\", \"fluent:person-fill\", nada:x, xmaterial:settings\n");
    std::fs::write(&code, text).unwrap();

    let used = codegen::used_icons(&project);
    assert_eq!(used.get("material").map(|s| s.iter().cloned().collect::<Vec<_>>()), Some(vec!["home".to_string()]));
    assert_eq!(used.get("fluent").map(|s| s.iter().cloned().collect::<Vec<_>>()), Some(vec!["person-fill".to_string()]));
    assert_eq!(used.len(), 2, "{used:?}");

    let written = codegen::sync_icons(&project).unwrap();
    assert_eq!(written.len(), 2);
    let material = std::fs::read_to_string(project.dir().join("assets/icons/material.txt")).unwrap();
    let icons: Vec<&str> = material.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(icons.len(), 1);
    assert!(icons[0].starts_with("home\t"));
    assert!(material.contains("# license: Apache-2.0"), "a licença vai junto");
    assert!(material.len() < 20_000, "{} bytes", material.len());
    // De novo: nada muda.
    assert!(codegen::sync_icons(&project).unwrap().is_empty());

    let mut ui = ui();
    install(&mut ui, &project).unwrap();
    assert!(ui.icon("material:home").is_some());
    assert!(ui.icon("fluent:person-fill").is_some());
    assert!(ui.icon("material:settings").is_none());

    // Sem citação, o pacote sai.
    std::fs::write(&code, "// nada\n").unwrap();
    codegen::sync_icons(&project).unwrap();
    assert!(!project.dir().join("assets/icons/material.txt").exists());
}

#[test]
fn an_imported_svg_joins_the_app_pack_on_the_24_grid() {
    let base = temp("svg");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><path d="M0 0H48V48Z"/><path fill="red" d="M24 12h12v12h-12z"/></svg>"#;
    assert_eq!(assets::import_svg(&project, "Minha Logo", svg).unwrap(), "app:minha-logo");
    let text = std::fs::read_to_string(project.dir().join("assets/icons/app.txt")).unwrap();
    assert!(text.contains("minha-logo\tM0 0L24 0L24 24Z M12 6L18 6L18 12L12 12Z"), "{text}");
    // Reimportar troca, não duplica.
    assets::import_svg(&project, "minha-logo", svg).unwrap();
    let text = std::fs::read_to_string(project.dir().join("assets/icons/app.txt")).unwrap();
    assert_eq!(text.matches("minha-logo\t").count(), 1);

    let mut ui = ui();
    install(&mut ui, &project).unwrap();
    let icon = ui.icon("app:minha-logo").expect("registrado");
    assert_eq!(icon.size, 24.0);
    // Formas (círculo, retângulo com transform) também viram contorno, na grade de 24.
    let shapes = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48"><circle cx="24" cy="24" r="12"/><g transform="translate(4 4)"><rect width="8" height="8"/></g></svg>"#;
    assert_eq!(assets::import_svg(&project, "formas", shapes).unwrap(), "app:formas");
    let text = std::fs::read_to_string(project.dir().join("assets/icons/app.txt")).unwrap();
    let line = text.lines().find(|l| l.starts_with("formas\t")).unwrap();
    let path = aylis_core::Path::from_svg(line.split('\t').nth(1).unwrap()).unwrap();
    let b = path.bounds();
    assert!((b.x - 2.0).abs() < 0.01 && (b.y - 2.0).abs() < 0.01 && (b.right() - 18.0).abs() < 0.05, "{b:?}");
    install(&mut ui, &project).unwrap();
    assert!(ui.icon("app:formas").is_some());
    // Só traço não cabe num ícone preenchido: erro que explica.
    let err = assets::import_svg(&project, "x", r#"<svg viewBox="0 0 24 24"><circle r="3" fill="none" stroke="black"/></svg>"#).unwrap_err();
    assert!(err.to_string().contains("traços"), "{err}");
}

#[test]
fn the_ide_preview_finds_the_pack_icons_cited_in_the_form() {
    if !has_resources() {
        return;
    }
    // A prévia (F9 sem projeto) registra os ícones citados no JSON do form.
    let form = r#"{"root":{"class":"Form","children":[{"class":"Button","icon":"material:desktop_windows","checked_icon":"fluent:person-fill"}]}}"#;
    let used = codegen::icons_cited_in(form);
    assert_eq!(used.get("material").map(|s| s.iter().cloned().collect::<Vec<_>>()), Some(vec!["desktop_windows".to_string()]));
    assert_eq!(used.get("fluent").map(|s| s.iter().cloned().collect::<Vec<_>>()), Some(vec!["person-fill".to_string()]));
    assert!(codegen::icons_cited_in(r#"{"icon":"home"}"#).is_empty(), "os do aylis (sem prefixo) já existem no app");
}

#[test]
fn a_new_project_comes_with_an_embedded_font() {
    if !has_resources() {
        return;
    }
    // A fonte do sistema muda de plataforma para plataforma (e com ela as medidas): o projeto
    // novo já nasce com uma embutida.
    let project = create(&temp("default-font"), "Loja", Language::Rust).unwrap();
    assert_eq!(project.font, aylis_project::DEFAULT_FONT);
    assert_eq!(assets::project_fonts(&project), ["inter"]);
    assert!(project.dir().join("assets/fonts/inter/OFL.txt").is_file());
}

#[test]
fn a_system_font_in_the_project_or_in_a_component_is_warned() {
    if !has_resources() {
        return;
    }
    let mut project = create(&temp("font-check"), "Loja", Language::Rust).unwrap();
    // O projeto novo, com a fonte embutida: nada a avisar.
    assert_eq!(aylis_project::check::fonts(&project), []);
    // Um componente com a fonte do sistema.
    let form = project.dir().join(&project.forms[0].file);
    let text = std::fs::read_to_string(&form).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(&text).unwrap();
    json["root"]["children"] = serde_json::json!([{ "class": "Label", "name": "lblTitulo", "font_family": "Segoe UI" }]);
    std::fs::write(&form, serde_json::to_string_pretty(&json).unwrap()).unwrap();
    let issues = aylis_project::check::fonts(&project);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!((issues[0].form.as_str(), issues[0].component.as_str(), issues[0].error), ("frmPrincipal", "lblTitulo", false));
    // Sem fonte no projeto: a do sistema.
    project.font.clear();
    assert!(aylis_project::check::fonts(&project).iter().any(|i| i.component.is_empty() && i.message.contains("fonte do sistema")));
}
