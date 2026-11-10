//! A verificação do projeto antes de compilar (item 1b): os exemplos passam sem erro, e um
//! projeto com cada defeito plantado tem cada um apontado.

use aylis_core::Ui;
use aylis_project::{Language, Project, check, create};

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis-check-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn the_example_projects_have_no_errors() {
    let examples = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut checked = 0;
    for entry in std::fs::read_dir(&examples).unwrap().flatten() {
        let Some(file) = std::fs::read_dir(entry.path()).ok().and_then(|d| d.flatten().map(|e| e.path()).find(|p| p.extension().is_some_and(|e| e == "aylisproj"))) else { continue };
        let Ok(project) = Project::open(&file) else { continue };
        let mut ui = ui();
        let issues = check::project(&project, Some(&mut ui), false);
        // As classes do 3D vêm do aylis-3d, que este teste não registra.
        let errors: Vec<_> = issues.iter().filter(|i| i.error && !i.message.contains("não registrada")).collect();
        assert!(errors.is_empty(), "{}: {errors:#?}", file.display());
        checked += 1;
    }
    assert!(checked >= 4, "{checked} exemplos");
}

#[test]
fn the_project_colors_are_known_when_checking() {
    let mut project = create(&temp("colors"), "Loja", Language::Rust).unwrap();
    project.colors.insert("green".into(), serde_json::json!("#10B981"));
    project.save().unwrap();
    let form = project.dir().join(&project.forms[0].file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&form).unwrap()).unwrap();
    layout["root"]["children"] = serde_json::json!([
        { "class": "Circle", "name": "dotStatus", "fill": "green" },
        { "class": "Circle", "name": "dotErrado", "fill": "verde_que_nao_existe" }
    ]);
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    let mut ui = ui();
    let issues = check::project(&project, Some(&mut ui), false);
    let messages: Vec<&str> = issues.iter().filter(|i| i.error).map(|i| i.message.as_str()).collect();
    assert!(!messages.iter().any(|m| m.contains("'green'")), "a cor do projeto vale: {messages:#?}");
    assert!(messages.iter().any(|m| m.contains("verde_que_nao_existe")), "{messages:#?}");
}

#[test]
fn each_planted_defect_is_found() {
    let project = create(&temp("defects"), "Loja", Language::Python).unwrap();
    let form = project.dir().join(&project.forms[0].file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&form).unwrap()).unwrap();
    layout["root"]["children"] = serde_json::json!([
        { "class": "Button", "name": "btnSalvar", "on_click": "salvar_click" },
        { "class": "Image", "name": "imgLogo", "image": "assets/images/nao-existe.png" },
        { "class": "Button", "name": "btnMenu", "popup_menu": "mnuQueNaoExiste" }
    ]);
    std::fs::write(&form, serde_json::to_string_pretty(&layout).unwrap()).unwrap();
    // Um caminho em texto no código que não existe no form.
    let code = project.code_dir().join("src/extra.py");
    std::fs::write(&code, "x = aylis.find_path(\"frmPrincipal.edtQueSumiu\")\n").unwrap();

    let mut ui = ui();
    let issues = check::project(&project, Some(&mut ui), false);
    let has = |component: &str, text: &str| issues.iter().any(|i| i.error && i.component == component && i.message.contains(text));
    assert!(has("btnSalvar", "salvar_click"), "{issues:#?}");
    assert!(has("imgLogo", "nao-existe.png"), "{issues:#?}");
    assert!(has("btnMenu", "mnuQueNaoExiste"), "{issues:#?}");
    assert!(issues.iter().any(|i| !i.error && i.message.contains("frmPrincipal.edtQueSumiu")), "{issues:#?}");
}

#[test]
fn panic_abort_and_threads_on_the_web_are_warned() {
    let project = create(&temp("platform"), "Loja", Language::Rust).unwrap();
    let cargo = project.code_dir().join("Cargo.toml");
    let text = std::fs::read_to_string(&cargo).unwrap();
    std::fs::write(&cargo, format!("{text}\n[profile.release]\npanic = \"abort\"\n")).unwrap();
    std::fs::write(project.code_dir().join("src/trabalho.rs"), "pub fn f() { std::thread::spawn(|| ()); }\n").unwrap();
    let desktop = check::platform(&project, false);
    assert!(desktop.iter().any(|i| !i.error && i.message.contains("abort")), "{desktop:?}");
    assert!(!desktop.iter().any(|i| i.message.contains("thread")), "no desktop a thread vale");
    let web = check::platform(&project, true);
    assert!(web.iter().any(|i| i.error && i.message.contains("thread::spawn")), "{web:?}");
}

/// O `data_field` que o dataset não tem (com os campos declarados) é erro; sem campos
/// declarados, não se sabe e passa.
#[test]
fn a_data_field_the_dataset_lacks_is_an_error() {
    let base = temp("fields");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[
        {"class":"ClientDataSet","name":"cdsClientes","children":[{"class":"Field","name":"nome"},{"class":"Field","name":"fCidade","field_name":"cidade"}]},
        {"class":"ClientDataSet","name":"cdsLivre"},
        {"class":"DataSource","name":"dsClientes","data_set":"cdsClientes"},
        {"class":"DataSource","name":"dsLivre","data_set":"cdsLivre"},
        {"class":"DBEdit","name":"edtNome","data_source":"dsClientes","data_field":"nome"},
        {"class":"DBEdit","name":"edtCidade","data_source":"dsClientes","data_field":"CIDADE"},
        {"class":"DBEdit","name":"edtErrado","data_source":"dsClientes","data_field":"nmoe"},
        {"class":"DBEdit","name":"edtLivre","data_source":"dsLivre","data_field":"qualquer"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let issues = check::data_fields(&project);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(issues[0].component, "edtErrado");
    assert!(issues[0].error && issues[0].message.contains("nmoe"));
    std::fs::remove_dir_all(&base).unwrap();
}

/// O form e o código mudaram, mas a geração diz que nada mudou (o F12 já tinha marcado o form em
/// dia): a verificação confere de novo pelas marcas dos arquivos, sem mostrar o erro de antes.
#[test]
fn the_check_follows_the_files_even_when_nothing_is_marked_changed() {
    let base = std::env::temp_dir().join(format!("aylis_check_marcas_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "Marcas", aylis_project::Language::Python).unwrap();
    let form = project.dir().join("forms/principal.json");
    let layout = |handler: &str| format!(r#"{{ "aylis": 1, "root": {{ "class": "Form", "name": "frmPrincipal", "children": [ {{ "class": "Button", "name": "btnOk", "on_click": "{handler}" }} ] }} }}"#);
    std::fs::write(&form, layout("ok_click")).unwrap();
    // O código sem a função: o erro.
    let unit = aylis_project::codegen::form_source(&project, "frmPrincipal").unwrap();
    std::fs::write(&unit, "from .components import frmPrincipal  # noqa: E402\n").unwrap();
    let issues = aylis_project::check::project_changed(&project, None, false, None);
    assert!(issues.iter().any(|i| i.error && i.message.contains("ok_click")));
    // Ligado a outro handler, que existe no código; a geração não marcou nada.
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(&form, layout("btn_ok_click")).unwrap();
    std::fs::write(&unit, "from .components import frmPrincipal  # noqa: E402\n\n\ndef btn_ok_click(aylis, event):\n    pass\n").unwrap();
    let issues = aylis_project::check::project_changed(&project, None, false, Some(&[]));
    assert!(!issues.iter().any(|i| i.error), "{:?}", issues.iter().map(|i| &i.message).collect::<Vec<_>>());
    let _ = std::fs::remove_dir_all(&base);
}
