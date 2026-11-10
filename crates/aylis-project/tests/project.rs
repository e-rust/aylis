use std::path::PathBuf;

use aylis_core::Ui;
use aylis_project::*;

/// Pasta temporária só deste teste.
fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_project_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui
}

#[test]
fn a_new_rust_project_has_the_folders_and_the_code() {
    let base = temp("rust");
    let project = create(&base, "Vendas", Language::Rust).unwrap();
    let dir = base.join("Vendas");
    assert_eq!(project.path, dir.join("Vendas.aylisproj"));
    for sub in ["forms", "assets/images", "assets/audio", "assets/video", "assets/fonts", "themes", "rust/src/forms"] {
        assert!(dir.join(sub).is_dir(), "{sub}");
    }
    for file in ["forms/principal.json", "rust/Cargo.toml", "rust/build.rs", "rust/src/lib.rs", "rust/src/main.rs", "rust/generated/mod.rs", "rust/generated/components.rs", "rust/src/forms/principal.rs"] {
        assert!(dir.join(file).is_file(), "{file}");
    }
    let cargo = std::fs::read_to_string(dir.join("rust/Cargo.toml")).unwrap();
    assert!(cargo.contains("name = \"vendas\"") && cargo.contains("/crates/aylis\"") && cargo.contains("/crates/aylis-build\""), "{cargo}");
    let lib = std::fs::read_to_string(dir.join("rust/src/lib.rs")).unwrap();
    assert!(lib.contains("include_project!()") && lib.contains("fn android_main") && lib.contains("fn aylis_start"), "{lib}");
    assert!(std::fs::read_to_string(dir.join("rust/src/main.rs")).unwrap().contains("vendas::app()"));

    // o pacote embutido (o que o build.rs faz) abre o mesmo projeto
    let bundle = aylis_project::Bundle::parse(&aylis_build::pack(&dir).unwrap()).unwrap();
    let packed = bundle.project().unwrap();
    assert_eq!((packed.name.as_str(), packed.forms.len()), ("Vendas", 1));
    assert!(bundle.get("forms/principal.json").is_some());
    assert!(bundle.get("rust/src/main.rs").is_none(), "o código não vai no pacote");

    // o .aylisproj volta igual
    let again = Project::open(&project.path).unwrap();
    assert_eq!(again, project);
    assert_eq!(again.main_form, "frmPrincipal");
    assert!(again.forms[0].auto_create);

    // a mesma pasta não pode ser usada de novo
    assert!(create(&base, "Vendas", Language::Rust).is_err());
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn forms_are_created_on_demand_and_assets_are_read_from_the_project() {
    let base = temp("forms");
    let mut project = create(&base, "Loja", Language::Python).unwrap();
    project.add_form("frmClientes", "Clientes").unwrap();
    assert!(project.add_form("frmClientes", "de novo").is_err());
    assert!(project.add_form("1nome", "").is_err());
    project.save().unwrap();
    std::fs::write(project.file("assets/images/logo.txt"), "olá").unwrap();

    let mut ui = ui();
    let main = install(&mut ui, &project).unwrap();
    assert_eq!(ui.name(main), Some("frmPrincipal"));
    assert_eq!(ui.roots().len(), 1, "frmClientes não é criado no início");
    let customers = ui.create_form("frmClientes").unwrap();
    assert_eq!(ui.create_form("frmClientes").unwrap(), customers, "já existe: devolve o mesmo");
    assert!(ui.create_form("frmNaoExiste").is_err());

    let resources = ui.services().resources.as_ref().unwrap();
    assert_eq!(resources.read("assets/images/logo.txt").unwrap(), "olá".as_bytes());
    assert!(resources.read("../fora.txt").is_none(), "não sai da pasta do projeto");

    // Python: main.py, cliente e um arquivo por form
    let dir = project.code_dir();
    codegen::generate(&project).unwrap();
    for file in ["src/main.py", "src/aylis_client.py", "src/forms/__init__.py", "src/forms/principal.py", "src/forms/clientes.py"] {
        assert!(dir.join(file).is_file(), "{file}");
    }
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn regenerating_keeps_user_code_and_uses_form_qualified_names() {
    let first = codegen::update_rust(None, &["salvar".into()], "frmClientes", "use aylis::prelude::*;");
    assert!(first.contains("ui.register_handler(\"frmClientes.salvar\", salvar);"));
    let edited = first.replace("let _ = (ui, sender, args);", "println!(\"salvando\");");
    let second = codegen::update_rust(Some(edited), &["salvar".into(), "excluir".into()], "frmClientes", "");
    assert!(second.contains("println!(\"salvando\");"));
    assert_eq!(second.matches("fn salvar(").count(), 1);
    assert!(second.contains("pub fn excluir("));

    let py = codegen::update_python(None, &["salvar".into()], "frmClientes");
    assert!(py.contains("def salvar(") && !py.contains("aylis.on("), "{py}");

    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmX","children":[
        {"class":"Button","name":"b1","on_click":"salvar"},
        {"class":"Button","name":"b2","on_click":"salvar","on_dbl_click":"abrir"}]}}"#;
    assert_eq!(codegen::form_handlers(layout).unwrap(), ("frmX".to_string(), vec!["salvar".to_string(), "abrir".to_string()]));
    // Editado fora da IDE: nada que não seja identificador chega ao código gerado.
    let injected = r#"{"aylis":1,"root":{"class":"Form","name":"frmX","children":[
        {"class":"Button","name":"b1","on_click":"x(ui: &mut Ui) {} pub fn y"}]}}"#;
    assert!(codegen::form_handlers(injected).unwrap_err().contains("handler inválido"));
    let bad_form = r#"{"aylis":1,"root":{"class":"Form","name":"frm\"X"}}"#;
    assert!(codegen::form_handlers(bad_form).unwrap_err().contains("form inválido"));
    assert_eq!(codegen::default_handler_name("btnSalvar", "on_click"), "btn_salvar_click");
    assert_eq!(file_stem_for("frmClientes"), "clientes");
}

#[test]
fn only_the_classes_and_features_the_forms_use() {
    let base = temp("lean");
    let project = create(&base, "Enxuto", Language::Rust).unwrap();
    let dir = base.join("Enxuto");
    let cargo = std::fs::read_to_string(dir.join("rust/Cargo.toml")).unwrap();
    assert!(cargo.contains("default-features = false, features = []"), "sem dados nem mídia: {cargo}");
    assert!(cargo.contains("dev = [\"aylis/dev\"]"));
    // Um form com 4 tipos de componente (um deles como subtipo) e um conjunto de dados.
    let form = r#"{ "class": "Form", "name": "frmPrincipal", "children": [
        { "class": "Label", "name": "Label1" },
        { "class": "Button.icon", "name": "Button1" },
        { "class": "Edit", "name": "Edit1" },
        { "class": "ClientDataSet", "name": "cds", "children": [ { "class": "Field", "name": "cdsNome" } ] },
        { "class": "MeuGrafico", "name": "g" }
    ] }"#;
    std::fs::write(dir.join("forms/principal.json"), form).unwrap();
    let used = aylis_build::used_classes(&dir.join("forms"));
    assert_eq!(used, ["Button", "ClientDataSet", "Edit", "Field", "Form", "Label", "MeuGrafico"]);
    let source = aylis_build::classes_source(&used);
    assert!(source.contains("::aylis::classes::Button(ui);") && source.contains("::aylis::classes::Field(ui);"));
    assert!(!source.contains("classes::Memo"), "o que não é usado não é citado");
    assert!(source.contains("// classes do app (registradas pelo próprio app): MeuGrafico"));
    // "Gerar código" atualiza as features.
    codegen::generate(&project).unwrap();
    let cargo = std::fs::read_to_string(dir.join("rust/Cargo.toml")).unwrap();
    assert!(cargo.contains("default-features = false, features = [\"data\"]"), "{cargo}");
    // Uma classe citada pelo nome no código entra no registro e liga a feature dela.
    std::fs::write(dir.join("rust/src/extra.rs"), "fn col(ui: &mut Ui) { ui.create(\"Column\", None); }\n").unwrap();
    codegen::generate(&project).unwrap();
    let cargo = std::fs::read_to_string(dir.join("rust/Cargo.toml")).unwrap();
    assert!(cargo.contains("features = [\"data\", \"dbcontrols\"]"), "{cargo}");
    std::fs::remove_file(dir.join("rust/src/extra.rs")).unwrap();
    // Projeto antigo (sem default-features = false) fica como está.
    assert_eq!(codegen::sync_aylis_features("aylis = { path = \"x\" }\n", &["data"]), None);
}

#[test]
fn renderer_per_platform_goes_to_cargo_toml() {
    let base = temp("render");
    let mut project = create(&base, "Render", Language::Rust).unwrap();
    // Padrão dos projetos novos: Direct2D no Windows, Skia nas outras.
    assert_eq!(project.render(), RenderOptions::default());
    assert_eq!(project.render().windows, RenderChoice::D2d);
    let cargo = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap();
    assert!(cargo.contains("[target.'cfg(windows)'.dependencies]"), "{cargo}");
    let windows = cargo.split("[target.'cfg(windows)'.dependencies]").nth(1).unwrap().lines().nth(1).unwrap();
    assert!(windows.contains("features = [\"d2d\"]"), "{windows}");
    assert!(cargo.contains("cfg(target_os = \"linux\")") && cargo.contains("\"skia-gpu\", \"skia-cpu\""));

    // Trocar depois: as Opções salvam e o Cargo.toml acompanha.
    let mut render = project.render();
    render.set(Platform::Windows, RenderChoice::Gpu);
    render.set(Platform::Ios, RenderChoice::D2d); // não existe no iOS: ignorado
    project.render = Some(render);
    project.save().unwrap();
    assert!(codegen::sync_cargo(&project).unwrap());
    let cargo = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap();
    let windows = cargo.split("[target.'cfg(windows)'.dependencies]").nth(1).unwrap().lines().nth(1).unwrap();
    assert!(windows.contains("features = [\"skia-gpu\"]"), "{windows}");
    assert_eq!(cargo.matches("# aylis:render-end").count(), 1, "o bloco é trocado, não repetido");
    assert!(!codegen::sync_cargo(&project).unwrap(), "nada mudou");
    let reopened = Project::open(&project.path).unwrap();
    assert_eq!(reopened.render().windows, RenderChoice::Gpu);
    assert_eq!(reopened.render().ios, RenderChoice::Cpu);

    // Projeto de antes da opção: Skia em todo lugar, Cargo.toml do jeito que estava.
    let legacy: Project = serde_json::from_str(r#"{"aylis_project":1,"name":"Velho","main_form":"f","forms":[]}"#).unwrap();
    assert_eq!(legacy.render().windows, RenderChoice::Auto);
    assert_eq!(codegen::sync_render("[dependencies]\naylis = { path = \"x\" }\n", &legacy), None);
}

#[test]
fn project_appearance_platform_theme_and_mix() {
    let base = temp("appearance");
    let mut project = create(&base, "Visual", Language::Rust).unwrap();
    project.theme = "Material 3 Dark".into();
    project.theme_mix.insert("Button".into(), "iOS 26 Light".into());
    project.save().unwrap();
    let reopened = Project::open(&project.path).unwrap();
    assert_eq!(reopened.theme_mix.get("Button").map(String::as_str), Some("iOS 26 Light"));
    let theme = resolve_theme(&reopened, &Files::Dir(reopened.dir())).unwrap();
    assert_eq!(theme.name, "Material 3 Dark");
    assert!(theme.dark);
    assert_eq!(theme.components.get("Button").map(String::as_str), Some("iOS 26 Light"));
    let mut ui = ui();
    let form = install(&mut ui, &reopened).unwrap();
    assert_eq!(ui.theme().name, "Material 3 Dark");
    let _ = form;
}

#[test]
fn an_old_cargo_toml_is_upgraded_without_losing_the_users_changes() {
    let root = codegen::aylis_root();
    let old = "[package]\nname = \"loja\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n# minha anotação\n[dependencies]\naylis = { path = \"C:/instalacao-velha/aylis/crates/aylis\" }\nrand = \"0.8\"\n";
    let (new, notes) = codegen::upgrade_cargo(old, &root, &std::env::temp_dir()).expect("muda");
    assert_eq!(notes, ["caminho do aylis", "aylis-build (para o build.rs)", "feature dev"]);
    assert!(new.contains(&format!("aylis = {{ path = \"{root}/crates/aylis\" }}")), "{new}");
    assert!(new.contains(&format!("aylis-build = {{ path = \"{root}/crates/aylis-build\" }}")), "{new}");
    assert!(new.contains("dev = [\"aylis/dev\"]") && !new.contains("inspect"), "{new}");
    assert!(new.contains("# minha anotação") && new.contains("rand = \"0.8\""), "o resto fica: {new}");
    // De novo: nada a fazer.
    assert_eq!(codegen::upgrade_cargo(&new, &root, &std::env::temp_dir()), None);
    // Um caminho que existe (outra cópia do aylis, de propósito) fica.
    let mine = format!("[dependencies]\naylis = {{ path = \"{root}/crates/aylis-core\" }}\n[build-dependencies]\naylis-build = {{ path = \"{root}/crates/aylis-build\" }}\n[features]\ndev = []\n");
    assert_eq!(codegen::upgrade_cargo(&mine, &root, &std::env::temp_dir()), None);

    // No "Gerar código": o build.rs novo e o Cargo.toml mexido de um projeto antigo.
    let base = temp("upgrade");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let dir = project.code_dir();
    std::fs::write(dir.join("Cargo.toml"), old).unwrap();
    let _ = std::fs::remove_file(dir.join("build.rs"));
    codegen::generate(&project).unwrap();
    assert!(dir.join("build.rs").exists());
    let cargo = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(cargo.contains("aylis-build") && cargo.contains("rand = \"0.8\"") && cargo.contains("# minha anotação"), "{cargo}");
}

#[test]
fn data_modules_come_first_and_forms_reach_their_components() {
    let base = temp("datamodule");
    let mut project = create(&base, "Loja", Language::Python).unwrap();
    project.add_form("frmClientes", "Clientes").unwrap();
    project.add_data_module("dmDados").unwrap();
    assert!(project.add_data_module("dmDados").is_err());
    // O DataModule: um menu e um Timer (não visuais); o form usa o menu dele.
    let dm_file = project.file(&project.form("dmDados").unwrap().file);
    assert!(dm_file.ends_with("forms/dados.json"));
    std::fs::write(
        dm_file,
        r#"{ "aylis": 1, "root": { "class": "DataModule", "name": "dmDados", "on_create": "dm_criado",
            "children": [ { "class": "PopupMenu", "name": "pmAcoes" }, { "class": "Timer", "name": "tmrRelogio" } ] } }"#,
    )
    .unwrap();
    std::fs::write(
        project.file("forms/clientes.json"),
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmClientes", "uses": ["dmDados"],
            "children": [ { "class": "Button", "name": "btnAcoes", "popup_menu": "dmDados.pmAcoes" } ] } }"#,
    )
    .unwrap();
    // Escritos por fora da IDE: o que o `.aylisproj` anota de cada form é lido de novo.
    project.refresh_forms();
    project.save().unwrap();
    // A hora de cada form fica fora do `.aylisproj` (senão ele mudava a cada salvar de form e
    // conflitava entre duas pessoas), em `.aylis/form-stamps.json`, e volta ao abrir.
    let saved = std::fs::read_to_string(&project.path).unwrap();
    assert!(!saved.contains("\"modified\""), "{saved}");
    let reopened = aylis_project::Project::open(&project.path).unwrap();
    assert!(reopened.form("dmDados").unwrap().modified.is_some(), "a hora volta do .aylis");
    assert!(std::fs::read_to_string(project.dir().join(".aylis").join(".gitignore")).unwrap().contains("form-stamps.json"));
    assert!(project.is_data_module("dmDados") && !project.is_data_module("frmClientes"));
    assert_eq!(project.uses_of("frmClientes"), ["dmDados"]);
    assert_eq!(project.used_by("dmDados"), ["frmClientes"]);

    let mut ui = ui();
    let created = std::rc::Rc::new(std::cell::Cell::new(0));
    let seen = created.clone();
    ui.register_handler("dm_criado", move |_, _, _| seen.set(seen.get() + 1));
    install(&mut ui, &project).unwrap();
    let dm = ui.find_path("dmDados").expect("criado no início");
    assert_eq!(ui.class_name(dm), Some("DataModule"));
    assert_eq!(created.get(), 1, "on_create do DataModule");
    // Criar o form não cria o DataModule de novo, e a referência aponta para ele.
    let form = ui.create_form("frmClientes").unwrap();
    assert_eq!(created.get(), 1);
    let button = ui.find_in(form, "btnAcoes").unwrap();
    let menu = ui.find_path("dmDados.pmAcoes").unwrap();
    assert_eq!(ui.get_prop(button, "popup_menu").unwrap(), aylis_core::PropValue::Component(Some(menu)));
    // O DataModule não é form: o aplicativo não abre janela para ele.
    assert!(!ui.open_forms().contains(&dm));
}

#[test]
fn platform_variants_share_the_handlers_and_the_code_knows_what_is_optional() {
    let base = temp("variants");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let default = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal",
        "children": [ { "class": "Button", "name": "btnSalvar", "on_click": "salvar" } ] } }"#;
    std::fs::write(project.file("forms/principal.json"), default).unwrap();
    assert!(project.add_variant("frmPrincipal", "desktop").is_ok());
    assert!(project.add_variant("frmPrincipal", "desktop").is_err(), "já existe");
    assert!(project.add_variant("frmPrincipal", "tv").is_err(), "desconhecida");
    // A do desktop é diferente: um menu lateral a mais, com o próprio handler.
    std::fs::write(
        project.file("forms/principal.desktop.json"),
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal", "children": [
            { "class": "Button", "name": "btnSalvar", "on_click": "salvar" },
            { "class": "ListBox", "name": "lstMenu", "on_change": "menu_mudou" } ] } }"#,
    )
    .unwrap();
    assert_eq!(project.variants_of("frmPrincipal"), ["desktop"]);
    assert_eq!(variant_file("forms/principal.json", "android"), "forms/principal.android.json");

    // Na execução vale a variante do sistema (aqui, desktop).
    let mut ui = ui();
    install(&mut ui, &project).unwrap();
    let on_desktop = !cfg!(any(target_os = "android", target_os = "ios", target_arch = "wasm32"));
    assert_eq!(ui.find_path("frmPrincipal.lstMenu").is_some(), on_desktop);

    // O código: os handlers de todas as variantes no arquivo do form; na struct gerada, o
    // menu (só no desktop) como Option.
    codegen::generate(&project).unwrap();
    let code = std::fs::read_to_string(project.code_dir().join("src/forms/principal.rs")).unwrap();
    assert!(code.contains("fn salvar(&mut self") && code.contains("fn menu_mudou(&mut self"), "{code}");
    // O pedaço do form, que o `generated/components.rs` inclui.
    let generated = std::fs::read_to_string(project.code_dir().join("generated/components/forms/principal.rs")).unwrap();
    assert!(generated.contains("pub btnSalvar: Button,") && generated.contains("pub lstMenu: Option<ListBox>,"), "{generated}");
    // Gerar de novo não duplica os handlers.
    codegen::generate(&project).unwrap();
    let again = std::fs::read_to_string(project.code_dir().join("src/forms/principal.rs")).unwrap();
    assert_eq!(again.matches("fn salvar(").count(), 1);
}

#[test]
fn field_text_handlers_come_with_the_answer_ready() {
    // O OnGetText e o OnSetText de um Field: o esqueleto já responde pelo aylis::data.
    let text = codegen::update_rust(None, &["status_get_text".into(), "status_set_text".into(), "salvar_click".into()], "frmPedidos", "use aylis::prelude::*;");
    assert!(text.contains("aylis::data::get_text(ui, sender, args, |value| value.to_string());"), "{text}");
    assert!(text.contains("aylis::data::set_text(ui, sender, args, |text| Ok(aylis::data::Value::String(text.to_string())));"), "{text}");
    assert!(text.contains("pub fn salvar_click(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
    let _ = (ui, sender, args);"));
}

#[test]
fn a_project_inside_another_workspace_gets_its_own() {
    let base = temp("workspace");
    std::fs::create_dir_all(&base).unwrap();
    std::fs::write(base.join("Cargo.toml"), "[workspace]\nmembers = [\"crates/*\", \"apps/rust/Loja\"]\nexclude = [\"fora\"]\n").unwrap();
    let cargo = "[package]\nname = \"x\"\n";
    let dir = |p: &str| {
        let d = base.join(p);
        std::fs::create_dir_all(&d).unwrap();
        d
    };
    assert!(codegen::needs_own_workspace(cargo, &dir("apps/pascal/Paint")), "fora dos membros: precisa");
    assert!(!codegen::needs_own_workspace(cargo, &dir("apps/rust/Loja")), "membro");
    assert!(!codegen::needs_own_workspace(cargo, &dir("crates/abc")), "membro pelo *");
    assert!(!codegen::needs_own_workspace(cargo, &dir("fora/Loja")), "excluído");
    assert!(!codegen::needs_own_workspace("[package]\nname = \"x\"\n[workspace]\n", &dir("apps/pascal/Paint")), "já tem");

    // No "Gerar código": o Cargo.toml ganha o [workspace] uma vez só.
    let project = create(&base.join("apps/pascal"), "Agenda", Language::Rust).unwrap();
    codegen::generate(&project).unwrap();
    codegen::generate(&project).unwrap();
    let text = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap();
    assert_eq!(text.matches("[workspace]").count(), 1, "{text}");
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn package_names_keep_acronyms_and_dataset_events_count_as_handlers() {
    assert_eq!(codegen::package_name("IBGE"), "ibge");
    assert_eq!(codegen::package_name("HTTPServer"), "http_server");
    assert_eq!(codegen::package_name("Vendas Web"), "vendas_web");
    // Os eventos do Delphi (after_scroll, before_post) também são handlers do form.
    let layout = r#"{"root": {"class": "Form", "name": "frmX", "on_show": "mostrar",
        "children": [{"class": "ClientDataSet", "name": "cds", "after_scroll": "mudou", "before_post": "gravar"}]}}"#;
    let (form, handlers) = codegen::form_handlers(layout).unwrap();
    assert_eq!(form, "frmX");
    assert_eq!(handlers, ["mostrar", "mudou", "gravar"]);
}

#[test]
fn frames_bring_their_form_inside_another() {
    let base = temp("frames");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    project.add_form("fraBotoes", "").unwrap();
    std::fs::write(
        project.file(&project.form("fraBotoes").unwrap().file),
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "fraBotoes", "width": 200, "height": 50,
            "children": [ { "class": "Button", "name": "btnSalvar", "on_click": "salvar" } ] } }"#,
    )
    .unwrap();
    let main = project.file(&project.form(&project.main_form).unwrap().file);
    let mut layout: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&main).unwrap()).unwrap();
    layout["root"]["children"] = serde_json::json!([
        { "class": "Frame", "name": "fraTopo", "form": "fraBotoes", "align": "top", "height": 60 }
    ]);
    std::fs::write(&main, serde_json::to_string(&layout).unwrap()).unwrap();
    project.save().unwrap();

    let mut ui = ui();
    let clicked = std::rc::Rc::new(std::cell::Cell::new(0));
    let seen = clicked.clone();
    // O evento vai para o handler do form do frame.
    ui.register_handler("fraBotoes.salvar", move |_, _, _| seen.set(seen.get() + 1));
    let form = install(&mut ui, &project).unwrap();
    ui.show(form).unwrap();
    let frame = ui.find_in(form, "fraTopo").unwrap();
    let inner = ui.find_path("fraBotoes").expect("o form do frame foi criado");
    assert_eq!(ui.parent(inner), Some(frame));
    assert!(!ui.roots().contains(&inner), "embutido, não é uma janela");
    let button = ui.find_path("fraBotoes.btnSalvar").unwrap();
    ui.layout();
    // O form do frame ocupa a área toda do frame, sem fundo próprio.
    let (fb, ib) = (ui.bounds(frame).unwrap(), ui.bounds(inner).unwrap());
    assert_eq!((ib.x, ib.y, ib.width, ib.height), (0.0, 0.0, fb.width, fb.height));
    assert_eq!(fb.height, 60.0);
    assert_eq!(ui.get_prop(inner, "fill").unwrap(), aylis_core::PropValue::Color(aylis_core::Color::TRANSPARENT));
    ui.click(button);
    assert_eq!(clicked.get(), 1);

    // Um frame que usa o próprio form não trava: dá erro.
    std::fs::write(
        project.file(&project.form("fraBotoes").unwrap().file),
        r#"{ "aylis": 1, "root": { "class": "Form", "name": "fraBotoes", "children": [ { "class": "Frame", "name": "fraEu", "form": "fraBotoes" } ] } }"#,
    )
    .unwrap();
    let error = install(&mut self::ui(), &project).err().map(|e| e.to_string()).unwrap_or_default();
    assert!(error.contains("entra nele mesmo"), "{error}");
}

#[test]
fn the_package_name_comes_from_the_cargo_toml() {
    let base = temp("package");
    let project = create(&base, "NoOBS", Language::Rust).unwrap();
    // O nome que sai do projeto, enquanto o Cargo.toml diz o mesmo.
    assert_eq!(project.package_name(), "no_obs");
    // Um projeto que escolheu outro nome no Cargo.toml: executar e publicar usam o dele.
    let cargo = project.code_dir_of(Language::Rust).join("Cargo.toml");
    let text = std::fs::read_to_string(&cargo).unwrap().replacen("name = \"no_obs\"", "name = \"noobs\"", 1);
    std::fs::write(&cargo, text).unwrap();
    assert_eq!(project.package_name(), "noobs");
}

#[test]
fn the_output_folder_is_where_the_executable_runs() {
    let base = temp("output");
    let mut project = create(&base, "Gravador", Language::Rust).unwrap();
    let built = std::path::Path::new("target").join("debug").join("gravador.exe");
    assert_eq!(project.output_exe(&built), None, "sem pasta de saída, roda do target");
    project.output.dir = "rust/exe".into();
    assert_eq!(project.output_exe(&built), Some(project.dir().join("rust/exe").join("gravador.exe")));
    project.output.name = "Gravador".into();
    assert_eq!(project.output_exe(&built), Some(project.dir().join("rust/exe").join("Gravador.exe")));
    // Fica no .aylisproj.
    project.save().unwrap();
    let back = Project::open(&project.path).unwrap();
    assert_eq!((back.output.dir.as_str(), back.output.name.as_str()), ("rust/exe", "Gravador"));
}

/// Partir de um exemplo (`apply_example`, a Landing): o form, o tema e, em Rust, os handlers dele.
#[test]
fn the_landing_template_brings_the_form_the_theme_and_the_handlers() {
    let base = temp("template");
    let mut project = create(&base, "Pagina", Language::Rust).unwrap();
    aylis_project::apply_example(&mut project, "Landing").unwrap();
    let form = std::fs::read_to_string(project.dir().join("forms/principal.json")).unwrap();
    assert!(form.contains("btnExperimentar") && form.contains("AppLink"));
    assert_eq!(project.theme, "Aylis Dark");
    let code = std::fs::read_to_string(project.dir().join("rust/src/forms/principal.rs")).unwrap();
    assert!(code.contains("ingot::apply") && code.contains("pub fn experimentar"), "os handlers da Landing");
    assert!(aylis_project::Project::open(&project.path).unwrap().theme == "Aylis Dark");
    // Em outra linguagem, os handlers vazios.
    let mut py = create(&base, "PaginaPy", Language::Python).unwrap();
    aylis_project::apply_example(&mut py, "Landing").unwrap();
    assert!(std::fs::read_to_string(py.dir().join("forms/principal.json")).unwrap().contains("btnExperimentar"));
    // Um modelo (Desktop) configura o projeto em branco; um nome desconhecido dá erro.
    let mut blank = create(&base, "Branco", Language::Rust).unwrap();
    aylis_project::apply_template(&mut blank, "Desktop").unwrap();
    assert!(aylis_project::apply_template(&mut blank, "Nada").is_err());
    let _ = std::fs::remove_dir_all(&base);
}

/// O BackgroundService nos forms: a geração cria `src/background.rs` e liga no `lib.rs`
/// (uma vez: gerar de novo não repete nem apaga o que foi escrito).
#[test]
fn the_background_service_gets_its_unit_and_the_android_entry() {
    let base = temp("background");
    let project = create(&base, "Servico", Language::Rust).unwrap();
    let form = r#"{ "root": { "class": "Form", "name": "frmPrincipal", "children": [ { "class": "BackgroundService", "name": "svc" } ] } }"#;
    std::fs::write(project.dir().join("forms/principal.json"), form).unwrap();
    codegen::generate(&project).unwrap();
    let unit = std::fs::read_to_string(project.dir().join("rust/src/background.rs")).unwrap();
    assert!(unit.contains("pub fn run(service: aylis::background::Service)"));
    std::fs::write(project.dir().join("rust/src/background.rs"), "pub fn run(_: aylis::background::Service) {} // meu\n").unwrap();
    codegen::generate(&project).unwrap();
    let lib = std::fs::read_to_string(project.dir().join("rust/src/lib.rs")).unwrap();
    assert_eq!(lib.matches("pub mod background;").count(), 1);
    assert!(lib.contains("aylis::background::set_worker(background::run);") && lib.contains("Java_dev_aylis_AylisActivity_00024Background_nativeStart"));
    assert!(std::fs::read_to_string(project.dir().join("rust/src/background.rs")).unwrap().contains("// meu"));
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_project_outside_the_aylis_tree_builds_aylis_optimized_in_debug() {
    let dir = std::env::temp_dir().join(format!("aylis_perfil_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let project = aylis_project::create(&dir, "Perfil", aylis_project::Language::Rust).unwrap();
    aylis_project::codegen::generate(&project).unwrap();
    let cargo = std::fs::read_to_string(project.code_dir().join("Cargo.toml")).unwrap();
    assert!(cargo.contains("[profile.dev.package.\"*\"]\nopt-level = 2"), "{cargo}");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn the_safety_limits_come_from_the_project() {
    let limits: aylis_project::LimitSettings = serde_json::from_str(r#"{ "tree_depth": 300 }"#).unwrap();
    assert_eq!((limits.tree_depth, limits.event_nesting), (300, aylis_core::MAX_EVENT_NESTING), "o que falta fica no padrão");
    assert!(limits.warnings().is_empty());
    let risky: aylis_project::LimitSettings = serde_json::from_str(r#"{ "tree_depth": 5000, "event_nesting": 1000 }"#).unwrap();
    assert_eq!(risky.warnings().len(), 2, "a IDE avisa dos dois");
}

/// Apagar do código um handler que o form cita: a geração não recria a função vazia, e a
/// verificação acusa (o erro leva ao lugar); ligar o evento de novo no form cria (decidido pelo
/// autor em 08/11: avisar, não recriar).
#[test]
fn a_deleted_handler_is_reported_not_recreated() {
    let base = temp("deleted_handler");
    let project = create(&base, "Apagado", Language::Python).unwrap();
    let form = project.dir().join("forms/principal.json");
    let layout = |event: &str| {
        format!(r#"{{ "aylis": 1, "root": {{ "class": "Form", "name": "frmPrincipal", "children": [ {{ "class": "Button", "name": "btnOk", "{event}": "ok_click" }} ] }} }}"#)
    };
    std::fs::write(&form, layout("on_click")).unwrap();
    codegen::generate(&project).unwrap();
    let unit = codegen::form_source(&project, "frmPrincipal").unwrap();
    let text = std::fs::read_to_string(&unit).unwrap();
    assert!(text.contains("def ok_click("), "a ligação nova ganha a função: {text}");
    // O usuário apaga a função.
    let start = text.find("def ok_click(").unwrap();
    let end = text[start..].find("pass").map(|i| start + i + "pass".len()).unwrap();
    std::fs::write(&unit, format!("{}{}", &text[..start], &text[end..])).unwrap();
    codegen::generate(&project).unwrap();
    let after = std::fs::read_to_string(&unit).unwrap();
    assert!(!after.contains("def ok_click("), "não recria: {after}");
    let issues = check::project(&project, None, false);
    assert!(issues.iter().any(|i| i.error && i.message.contains("ok_click")), "a verificação acusa: {:?}", issues.iter().map(|i| &i.message).collect::<Vec<_>>());
    // Ligado de novo (outro evento): a ligação é nova e a função volta.
    std::fs::write(&form, layout("on_dbl_click")).unwrap();
    codegen::generate(&project).unwrap();
    assert!(std::fs::read_to_string(&unit).unwrap().contains("def ok_click("));
}

/// Renomear um handler, abrir o código do form (F12, `generate_form`) e depois gerar tudo
/// (Executar): o registro dos handlers fica com o nome novo (ficava com o antigo, porque o F12
/// marcava o form em dia sem atualizar o índice; conferência de 08/11).
#[test]
fn the_handler_registry_follows_a_rename_after_f12() {
    let base = temp("registro_f12");
    let project = create(&base, "Registro", Language::Rust).unwrap();
    let form = project.dir().join("forms/principal.json");
    let layout = |handler: &str| format!(r#"{{ "aylis": 1, "root": {{ "class": "Form", "name": "frmPrincipal", "on_show": "{handler}" }} }}"#);
    std::fs::write(&form, layout("form_show")).unwrap();
    codegen::generate(&project).unwrap();
    let registry = || std::fs::read_to_string(project.code_dir().join("generated/mod.rs")).unwrap();
    assert!(registry().contains("FrmPrincipal::form_show"));
    // O renomear: o form e a função no código.
    std::fs::write(&form, layout("ao_exibir_show")).unwrap();
    let unit = codegen::form_source(&project, "frmPrincipal").unwrap();
    let text = std::fs::read_to_string(&unit).unwrap().replace("fn form_show(", "fn ao_exibir_show(");
    std::fs::write(&unit, text).unwrap();
    codegen::generate_form(&project, "frmPrincipal").unwrap();
    codegen::generate(&project).unwrap();
    let r = registry();
    assert!(r.contains("FrmPrincipal::ao_exibir_show") && !r.contains("FrmPrincipal::form_show"), "{r}");
}
