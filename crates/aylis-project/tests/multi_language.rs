//! Um projeto com o código em várias linguagens (uma pasta cada) e o visual em comum: a
//! geração mantém todas com as funções que o form chama, renomeia junto e diz o que falta
//! portar.

use aylis_project::sync::{self, Pending};
use aylis_project::{Language, Project, codegen, create};

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_multi_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// O form principal com um botão (`btnSalvar.on_click` → `handler`) e um campo `edtNome`.
fn set_form(project: &Project, handler: &str, with_edit: bool) {
    let edit = if with_edit { r#",{"class":"Edit","name":"edtNome"}"# } else { "" };
    let layout = format!(r#"{{"aylis":1,"root":{{"class":"Form","name":"frmPrincipal","children":[{{"class":"Button","name":"btnSalvar","on_click":"{handler}"}}{edit}]}}}}"#);
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
}

fn source(project: &Project, language: Language) -> std::path::PathBuf {
    codegen::form_source(&project.with_language(language), "frmPrincipal").unwrap()
}

fn edit(path: &std::path::Path, from: &str, to: &str) {
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains(from), "{}: falta {from:?}\n{text}", path.display());
    std::fs::write(path, text.replacen(from, to, 1)).unwrap();
}

#[test]
fn each_language_has_its_folder_and_the_logic_is_tracked_between_them() {
    let base = temp("sync");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    set_form(&project, "salvar_click", true);
    codegen::generate(&project).unwrap();
    assert!(project.dir().join("rust/Cargo.toml").is_file() && !project.dir().join("src").exists(), "o código fica em rust/");
    let rust = source(&project, Language::Rust);
    edit(&rust, "let _ = (ui, sender, args);", "let _ = (ui, sender, args);\n    let nome = ui.find(\"edtNome\");\n    println!(\"{nome:?}\");");
    codegen::generate(&project).unwrap();
    assert_eq!(sync::pending(&project, Language::Rust).unwrap(), vec![]);

    // Python ativo: a pasta nasce com o handler vazio, pendente (alterado em Rust).
    project.language = Language::Python;
    codegen::generate(&project).unwrap();
    assert_eq!(project.code_languages(), vec![Language::Rust, Language::Python]);
    let python = source(&project, Language::Python);
    assert!(python.starts_with(project.dir().join("python")));
    let pending = sync::pending(&project, Language::Python).unwrap();
    assert_eq!(pending, vec![Pending::Changed { form: "frmPrincipal".into(), handler: "salvar_click".into(), by: vec![Language::Rust], empty: true }]);
    assert!(std::fs::read_to_string(&rust).unwrap().contains("println!"), "o Rust fica intacto");

    // Portado: o corpo do Python mudou, a pendência sai (e o Rust não fica pendente por isso).
    edit(&python, "def salvar_click(aylis, event):\n    pass", "def salvar_click(aylis, event):\n    nome = aylis.call(\"get\", target=\"edtNome\", prop=\"text\")\n    print(nome)");
    codegen::generate(&project).unwrap();
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![]);
    assert_eq!(sync::pending(&project, Language::Rust).unwrap(), vec![]);

    // De volta ao Rust, a lógica muda: agora o Python está atrasado.
    project.language = Language::Rust;
    edit(&rust, "println!(\"{nome:?}\");", "println!(\"salvando {nome:?}\");");
    codegen::generate(&project).unwrap();
    assert!(matches!(&sync::pending(&project, Language::Python).unwrap()[..], [Pending::Changed { empty: false, .. }]));
    // Marcar como portado (sem mudar o código) também resolve.
    sync::mark_ported(&project, Language::Python, "frmPrincipal", "salvar_click").unwrap();
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![]);

    // O handler é renomeado no form: o código das duas linguagens acompanha, com o corpo.
    set_form(&project, "gravar_click", true);
    codegen::generate(&project).unwrap();
    let r = std::fs::read_to_string(&rust).unwrap();
    let p = std::fs::read_to_string(&python).unwrap();
    assert!(r.contains("fn gravar_click(") && !r.contains("fn salvar_click(") && r.contains("salvando"), "{r}");
    assert!(p.contains("def gravar_click(") && !p.contains("def salvar_click(") && p.contains("print(nome)"), "{p}");
    assert_eq!(sync::pending(&project, Language::Python).unwrap(), vec![], "renomear não é mudar a lógica");

    // Sai o handler e sai o campo: a função fica (sem uso) e o código que cita o campo aparece.
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Button","name":"btnSalvar"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    codegen::generate(&project).unwrap();
    let pending = sync::pending(&project, Language::Python).unwrap();
    assert!(pending.contains(&Pending::Unused { form: "frmPrincipal".into(), handler: "gravar_click".into() }), "{pending:?}");
    assert!(pending.iter().any(|p| matches!(p, Pending::MissingComponent { name, line: 3.., .. } if name == "edtNome")), "{pending:?}");
    assert!(pending.iter().all(|p| !p.describe().is_empty()));
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn an_old_project_moves_its_code_to_the_language_folder() {
    let base = temp("legacy");
    let project = create(&base, "Antigo", Language::Python).unwrap();
    // Como as versões antigas deixavam: o código na raiz.
    let dir = project.dir();
    std::fs::rename(dir.join("python/src"), dir.join("src")).unwrap();
    std::fs::remove_dir_all(dir.join("python")).unwrap();
    assert!(project.migrate_layout().unwrap());
    assert!(dir.join("python/src/main.py").is_file() && !dir.join("src").exists());
    assert!(!project.migrate_layout().unwrap(), "uma vez só");
    std::fs::remove_dir_all(&base).unwrap();
}

#[test]
fn every_language_gets_typed_access_to_the_components() {
    let base = temp("typed");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Panel","name":"pnlLateral","children":[{"class":"Label","name":"lblTotal"}]}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
    }
    // Em Rust o arquivo inclui um pedaço por form e por classe (`generated/components`): o texto
    // é o de todos juntos.
    let read = |language: Language| {
        let dir = project.code_dir_of(language);
        let mut text = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            for part in ["generated/components/classes", "generated/components/forms"] {
                for e in std::fs::read_dir(dir.join(part)).unwrap().flatten() {
                    text.push_str(&std::fs::read_to_string(e.path()).unwrap());
                }
            }
        }
        text
    };
    // O mesmo form, os mesmos componentes e propriedades, no jeito de cada linguagem.
    let expect = [
        (Language::Rust, &["pub struct FrmPrincipal {", "pub fn bind(ui: &mut Ui, root: ComponentId)", "pub fn visible(self, ui: &Ui) -> bool", "pub pnlLateral: Panel,"][..]),
        (Language::Python, &["frmPrincipal = CurrentForm(\"frmPrincipal\")", "def visible(self):", "self.lblTotal = self._resolve(components, \"lblTotal\", \"Label\", Label)"]),
        (Language::TypeScript, &["export let frmPrincipal: FrmPrincipalComponents;", "get visible(): Promise<boolean>"]),
        (Language::CSharp, &["public static AylisUi.FrmPrincipalComponents frmPrincipal = null!;", "public bool Visible {"]),
        (Language::Java, &["public static FrmPrincipalComponents frmPrincipal;", "public boolean isVisible()", "public void setVisible(boolean value)"]),
        (Language::Go, &["var frmPrincipal *frmPrincipalComponents", "func (c aylisPanel) Visible() bool", "func (c aylisPanel) SetVisible(v bool)"]),
        (Language::Cpp, &["inline aylis_components::FrmPrincipalComponents frmPrincipal;", "bool visible() const", "lblTotal = Label(resolve_(list, \"lblTotal\", \"Label\"));"]),
        (Language::Pascal, &["TFrmPrincipalBase = class(TForm)", "property Visible: Boolean read GetVisible write SetVisible;"]),
    ];
    for (language, texts) in expect {
        let text = read(language);
        for t in texts {
            assert!(text.contains(t), "{}: falta {t:?}", language.name());
        }
    }
    // O arquivo de eventos traz a instância do form.
    let python = std::fs::read_to_string(source(&project, Language::Python)).unwrap();
    assert!(python.contains("from .components import frmPrincipal"), "{python}");
    let pascal = std::fs::read_to_string(source(&project, Language::Pascal)).unwrap();
    assert!(pascal.contains("ayliscomponents"), "{pascal}");
    std::fs::remove_dir_all(&base).unwrap();
}

/// Os eventos que perguntam ganham o esqueleto com a assinatura certa: `var` em Pascal
/// (`var CanClose: Boolean`), `Result` em Rust; e a assinatura comum de
/// um projeto de antes é atualizada.
#[test]
fn question_events_get_the_right_signature() {
    let base = temp("questions");
    let mut project = create(&base, "Loja", Language::Pascal).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","on_close_query":"pode_fechar","children":[{"class":"Edit","name":"edtNome","on_key_press":"tecla"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    codegen::generate(&project).unwrap();
    let pascal = std::fs::read_to_string(source(&project, Language::Pascal)).unwrap();
    assert!(pascal.contains("procedure pode_fechar(Sender: TAylisComponent; var CanClose: Boolean);"), "{pascal}");
    assert!(pascal.contains("procedure TFrmPrincipal.tecla(Sender: TAylisComponent; var Key: string);"), "{pascal}");
    // Um projeto de antes, com a assinatura comum: passa a ser a do evento.
    let path = source(&project, Language::Pascal);
    let old = pascal.replace("var CanClose: Boolean", "Event: TJSONObject");
    std::fs::write(&path, old).unwrap();
    codegen::generate(&project).unwrap();
    let pascal = std::fs::read_to_string(&path).unwrap();
    assert_eq!(pascal.matches("var CanClose: Boolean").count(), 2, "{pascal}");

    project.language = Language::Rust;
    codegen::generate(&project).unwrap();
    let rust = std::fs::read_to_string(source(&project, Language::Rust)).unwrap();
    assert!(rust.contains("pub fn pode_fechar(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) -> Result<(), String> {"), "{rust}");
    std::fs::remove_dir_all(&base).unwrap();
}

/// O F12 gera só o código do form aberto: a unit dele ganha o handler novo e o acesso
/// tipado acompanha, sem refazer o que é de compilar (o `Cargo.toml` mexido fica). Um form
/// sem unit ainda gera o projeto inteiro.
#[test]
fn generating_one_form_touches_only_its_code() {
    let base = temp("um_form");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    set_form(&project, "salvar_click", true);
    codegen::generate(&project).unwrap();
    let cargo = project.dir().join("rust/Cargo.toml");
    let marked = format!("{}\n# marca do teste\n", std::fs::read_to_string(&cargo).unwrap());
    std::fs::write(&cargo, &marked).unwrap();

    set_form(&project, "gravar_click", true);
    let written = codegen::generate_form(&project, "frmPrincipal").unwrap();
    let unit = source(&project, Language::Rust);
    assert!(written.contains(&unit));
    let text = std::fs::read_to_string(&unit).unwrap();
    assert!(text.contains("fn gravar_click"), "o handler novo entra na unit:\n{text}");
    assert_eq!(std::fs::read_to_string(&cargo).unwrap(), marked, "o Cargo.toml não é refeito");

    // Sem a unit (form novo): o projeto inteiro.
    std::fs::remove_file(&unit).unwrap();
    codegen::generate_form(&project, "frmPrincipal").unwrap();
    assert!(unit.is_file());
}

/// Um método que recebe um componente (`scroll_to` do `ScrollBox`) pede o componente no jeito de
/// cada linguagem e manda o id dele, não um caminho em texto.
#[test]
fn a_component_argument_takes_the_component() {
    let base = temp("component_arg");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"ScrollBox","name":"scrLista","children":[{"class":"Label","name":"lblItem"}]}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        let mut t = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            t.push_str(&std::fs::read_to_string(dir.join("generated/components/classes/ScrollBox.rs")).unwrap());
        }
        text.insert(language.name(), t);
    }
    let line = |language: Language, needle: &str| text[language.name()].lines().find(|l| l.contains(needle)).unwrap_or_default().trim().to_string();
    let expect = [
        (Language::Rust, &["pub fn scroll_to(self, ui: &mut Ui, component: impl Handle)", "PropValue::Component(Some(component.id()))"][..]),
        (Language::Python, &["def scroll_to(self, component):", "args=[component.id]"]),
        (Language::TypeScript, &["scrollTo(component: AylisComponent)", "args: [component.id]"]),
        (Language::CSharp, &["public void ScrollTo(AylisComponent component)", "new JsonArray(component.Id)"]),
        (Language::Java, &["public void scrollTo(Aylis.AylisComponent component)", "java.util.List.of(component.id)"]),
        (Language::Go, &["ScrollTo(component aylis.Ref)", "[]any{component.ComponentID()}"]),
        (Language::Cpp, &["void scroll_to(const aylis::Component& component) const", "aylis::Json::array({component.id()})"]),
        (Language::Pascal, &["procedure ScrollTo(AComponent: TAylisComponent);", "TJSONArray.Create([AComponent.Id])"]),
    ];
    for (language, texts) in expect {
        for t in texts {
            assert!(text[language.name()].contains(t), "{}: falta {t:?}\n{}", language.name(), line(language, "croll"));
        }
    }
    std::fs::remove_dir_all(&base).unwrap();
}

/// Os métodos com o nome de cada argumento (do catálogo) em todas as linguagens e, em Rust, a
/// falha como `Err` com o motivo (não um `false`): `Export::save_to(ui, path)`.
#[test]
fn methods_have_named_parameters_and_rust_returns_the_error() {
    let base = temp("named_params");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Export","name":"expDados"},{"class":"PaintCanvas","name":"pcDesenho"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        let mut t = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            for class in ["Export", "PaintCanvas"] {
                t.push_str(&std::fs::read_to_string(dir.join(format!("generated/components/classes/{class}.rs"))).unwrap());
            }
        }
        text.insert(language.name(), t);
    }
    let expect = [
        (
            Language::Rust,
            &[
                "pub fn save_to(self, ui: &mut Ui, path: &str) -> Result<bool, String>",
                "pub fn resize(self, ui: &mut Ui, width: i64, height: i64) -> Result<bool, String>",
                "pub fn extension(self, ui: &mut Ui) -> Result<String, String>",
                "handles::try_call(ui, self.id, \"save_to\", &[PropValue::String(path.to_string())])",
            ][..],
        ),
        (Language::Python, &["def save_to(self, path):", "args=[path]", "def resize(self, width, height):"]),
        (Language::TypeScript, &["saveTo(path: string)", "resize(width: number, height: number)"]),
        (Language::CSharp, &["SaveTo(string path)", "new JsonArray(path)"]),
        (Language::Java, &["saveTo(String path)", "java.util.List.of(path)"]),
        (Language::Go, &["SaveTo(path string)", "[]any{path}"]),
        (Language::Cpp, &["save_to(const std::string& path) const", "aylis::Json::array({path})"]),
        // Em Pascal, com o `A` na frente (o `Width` do `resize` bateria com a propriedade).
        (Language::Pascal, &["SaveTo(APath: ", "TJSONArray.Create([APath])", "Resize(AWidth: Int64; AHeight: Int64)"]),
    ];
    let mut missing = Vec::new();
    for (language, texts) in expect {
        for t in texts {
            if !text[language.name()].contains(t) {
                let near: Vec<&str> = text[language.name()].lines().filter(|l| l.contains("ave") && l.contains("o(") || l.contains("esize")).collect();
                missing.push(format!("{}: falta {t:?}\n{}", language.name(), near.join("\n")));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n\n"));
    std::fs::remove_dir_all(&base).unwrap();
}
