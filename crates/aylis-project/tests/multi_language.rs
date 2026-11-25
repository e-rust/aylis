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
    edit(&rust, "let _ = (ui, sender);", "let _ = (ui, sender);\n    let nome = ui.find(\"edtNome\");\n    println!(\"{nome:?}\");");
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
    edit(&python, "def salvar_click(aylis, sender: \"Button\"):\n    pass", "def salvar_click(aylis, sender: \"Button\"):\n    nome = aylis.call(\"get\", target=\"edtNome\", prop=\"text\")\n    print(nome)");
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
        (Language::Python, &["frmPrincipal = CurrentForm(\"frmPrincipal\")", "def visible(self) -> bool:", "self.lblTotal = self._resolve(components, \"lblTotal\", \"Label\", Label)"]),
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

/// O que não é número nem texto chega tipado em todas as linguagens: o enum (`window_state`),
/// o conjunto (`anchors`), a cor (`fill`), as margens (`margins`) e a referência a outro
/// componente (`popup_menu`), em vez de texto.
#[test]
fn enums_colors_bounds_and_references_are_typed() {
    let base = temp("typed_kinds");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Panel","name":"pnlLateral"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        let mut t = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            for class in ["Form", "Panel"] {
                t.push_str(&std::fs::read_to_string(dir.join(format!("generated/components/classes/{class}.rs"))).unwrap());
            }
        }
        text.insert(language.name(), t);
    }
    let expect = [
        (
            Language::Rust,
            &[
                "pub fn window_state(self, ui: &Ui) -> aylis::prelude::WindowState",
                "pub fn set_window_state(self, ui: &mut Ui, value: aylis::prelude::WindowState)",
                "pub fn set_anchors(self, ui: &mut Ui, value: aylis::prelude::Anchors)",
                "pub fn set_fill(self, ui: &mut Ui, value: impl Into<handles::ColorRef>)",
                "pub fn margins(self, ui: &Ui) -> aylis::prelude::Bounds",
                "pub fn popup_menu(self, ui: &Ui) -> Option<ComponentId>",
                "pub fn set_popup_menu(self, ui: &mut Ui, value: impl Into<handles::Ref>)",
            ][..],
        ),
        (
            Language::Python,
            &[
                "class WindowState(str, Enum):",
                "    FULL_SCREEN = \"full_screen\"",
                "def window_state(self) -> WindowState:",
                "return _enum(WindowState, self._get(\"window_state\"))",
                "def anchors(self) -> list[Anchor]:",
                "def fill(self) -> Color:",
                "self._set(\"margins\", value.to_json())",
                "return self._get_ref(\"popup_menu\")",
            ],
        ),
        (
            Language::TypeScript,
            &[
                "export type WindowState = \"normal\" | \"maximized\" | \"minimized\" | \"full_screen\";",
                "get windowState(): Promise<WindowState>",
                "get anchors(): Promise<Anchor[]>",
                "get fill(): Promise<Color>",
                "set margins(value: Bounds)",
                "get popupMenu(): Promise<AylisComponent | null>",
            ],
        ),
        (
            Language::CSharp,
            &[
                "public enum WindowState",
                "FullScreen,",
                "public WindowState WindowState { get => EnumNames.Of<WindowState>(GetProp(\"window_state\"), EnumNames.WindowState);",
                "public Anchor[] Anchors {",
                "public Color Fill {",
                "public Bounds Margins {",
                "public AylisComponent? PopupMenu { get => GetRef(\"popup_menu\"); set => SetRef(\"popup_menu\", value); }",
            ],
        ),
        (
            Language::Java,
            &[
                "public enum WindowState {",
                "FULL_SCREEN(\"full_screen\")",
                "public WindowState getWindowState()",
                "public void setAnchors(java.util.List<Anchor> value)",
                "public Aylis.Color getFill()",
                "public void setMargins(Aylis.Bounds value)",
                "public Aylis.AylisComponent getPopupMenu()",
            ],
        ),
        (
            Language::Go,
            &[
                "type WindowState string",
                "WindowStateFullScreen WindowState = \"full_screen\"",
                "func (c aylisForm) WindowState() WindowState",
                "func (c aylisPanel) Anchors() []Anchor",
                "func (c aylisPanel) Fill() aylis.Color",
                "func (c aylisPanel) SetMargins(v aylis.Bounds)",
                "func (c aylisPanel) SetPopupMenu(v aylis.Ref)",
            ],
        ),
        (
            Language::Cpp,
            &[
                "enum class WindowState { Normal, Maximized, Minimized, FullScreen };",
                "WindowState window_state() const",
                "std::vector<Anchor> anchors() const",
                "aylis::Color fill() const",
                "void set_margins(const aylis::Bounds& v) const",
                "std::optional<aylis::Component> popup_menu() const",
            ],
        ),
        (
            Language::Pascal,
            &[
                "TWindowState = (Normal, Maximized, Minimized, FullScreen);",
                "property WindowState: TWindowState read GetWindowState write SetWindowState;",
                "property Anchors: TAnchorSet read GetAnchors write SetAnchors;",
                "property Fill: TAylisColor read GetFill write SetFill;",
                "property Margins: TAylisBounds read GetMargins write SetMargins;",
                "property PopupMenu: TAylisComponent read GetPopupMenu write SetPopupMenu;",
            ],
        ),
    ];
    let mut missing = Vec::new();
    for (language, texts) in expect {
        for t in texts {
            if !text[language.name()].contains(t) {
                missing.push(format!("{}: falta {t:?}", language.name()));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
    std::fs::remove_dir_all(&base).unwrap();
}

/// A sombra, os pontos, os itens com campos, os registros que um método devolve e os enums dos
/// argumentos de método também chegam tipados, e não como texto.
#[test]
fn structures_records_and_method_enums_are_typed() {
    let base = temp("structures");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Polygon","name":"polForma"},{"class":"ListView","name":"lvwItens"},{"class":"Screen","name":"scrTela"},{"class":"Permissions","name":"perApp"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        let mut t = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            for class in ["Polygon", "ListView", "Screen", "Permissions"] {
                t.push_str(&std::fs::read_to_string(dir.join(format!("generated/components/classes/{class}.rs"))).unwrap());
            }
        }
        text.insert(language.name(), t);
    }
    let expect = [
        (
            Language::Rust,
            &[
                "pub fn shadow(self, ui: &Ui) -> Option<aylis::prelude::Shadow>",
                "pub fn points(self, ui: &Ui) -> Vec<aylis::prelude::PointF>",
                "pub fn set_items(self, ui: &mut Ui, value: &[ListViewItem])",
                "pub fn monitors(self, ui: &mut Ui) -> Result<Vec<Monitor>, String>",
                "pub struct Monitor {",
                "pub modes: Vec<VideoMode>,",
                "pub fn state(self, ui: &mut Ui, permission: aylis::prelude::Permission) -> Result<aylis::prelude::PermissionState, String>",
            ][..],
        ),
        (Language::Python, &["def shadow(self) -> Shadow | None:", "def items(self) -> list[ListViewItem]:", "def monitors(self) -> list[Monitor]:", "class Monitor:", "def state(self, permission: Permission) -> PermissionState:"]),
        (Language::TypeScript, &["get points(): Promise<Point[]>", "get items(): Promise<ListViewItem[]>", "export interface Monitor {", "monitors(): Promise<Monitor[]>"]),
        (Language::CSharp, &["public Shadow? Shadow {", "public global::App.Forms.ListViewItem[] Items {", "public sealed record Monitor(", "public global::App.Forms.Monitor[] Monitors()"]),
        (Language::Java, &["public java.util.List<Aylis.Point> getPoints()", "public record Monitor(", "public java.util.List<Monitor> monitors()"]),
        (Language::Go, &["func (c aylisPolygon) Points() []aylis.Point", "type Monitor struct {", "func (c aylisScreen) Monitors() []Monitor"]),
        (Language::Cpp, &["std::vector<aylis::Point> points() const", "struct Monitor {", "std::vector<Monitor> monitors() const"]),
        (Language::Pascal, &["property Points: TAylisPointArray", "TMonitor = record", "property Items: TListViewItemArray", "function Monitors: TMonitorArray;"]),
    ];
    let mut missing = Vec::new();
    for (language, texts) in expect {
        for t in texts {
            if !text[language.name()].contains(t) {
                missing.push(format!("{}: falta {t:?}", language.name()));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
    std::fs::remove_dir_all(&base).unwrap();
}

/// Os nomes do projeto viram enums em todas as linguagens: o `style` de cada classe (as
/// variantes do Aylis e as dos temas do projeto), o `theme` (os do Aylis e os do projeto) e as
/// referências a form (os forms do projeto). Mudou um tema, o gerado é refeito com os valores novos.
#[test]
fn project_names_are_typed() {
    let base = temp("project_names");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Button","name":"btnOk"},{"class":"Frame","name":"fraQuadro"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    std::fs::create_dir_all(project.file("themes")).unwrap();
    std::fs::write(project.file("themes/empresa.json"), r##"{"name":"Empresa","base":"Aylis Light","styles":{"Button.perigo":{"fill":"#DC2626"}}}"##).unwrap();
    // Um pacote: o form dele entra como `pkg:Termometro`.
    std::fs::create_dir_all(project.file("packages/Termometro")).unwrap();
    std::fs::write(project.file("packages/Termometro/package.json"), r#"{"name":"Termometro","version":"1.0.0","kind":"composite","form":"form.json"}"#).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        let mut t = std::fs::read_to_string(dir.join(aylis_project::bindings::file(language))).unwrap();
        if language == Language::Rust {
            for class in ["Button", "Frame"] {
                t.push_str(&std::fs::read_to_string(dir.join(format!("generated/components/classes/{class}.rs"))).unwrap());
            }
        }
        text.insert(language.name(), t);
    }
    let expect = [
        (Language::Rust, &["pub enum ButtonStyle {", "    Perigo,", "    Empresa,", "    FrmPrincipal,", "pub fn set_style(self, ui: &mut Ui, value: ButtonStyle)", "pub fn form(self, ui: &Ui) -> FormName"][..]),
        (Language::Python, &["class ButtonStyle(str, Enum):", "PERIGO = \"perigo\"", "EMPRESA = \"Empresa\"", "FRM_PRINCIPAL = \"frmPrincipal\"", "def style(self) -> ButtonStyle:"]),
        (Language::TypeScript, &["export type ButtonStyle = \"\" | \"accent\"", "| \"perigo\"", "export type FormName = \"\" | \"frmPrincipal\" | \"pkg:Termometro\""]),
        (Language::CSharp, &["        Perigo,", "        Empresa,", "public ButtonStyle Style {", "public FormName Form {"]),
        (Language::Java, &["PERIGO(\"perigo\")", "FRM_PRINCIPAL(\"frmPrincipal\")"]),
        (Language::Go, &["ButtonStylePerigo ButtonStyle = \"perigo\"", "ThemeNameEmpresa ThemeName = \"Empresa\"", "func (c aylisFrame) Form() FormName"]),
        (Language::Cpp, &["enum class FormName { Default, FrmPrincipal, PkgTermometro };", ", Perigo };"]),
        (Language::Pascal, &["TFormName = (Default, FrmPrincipal, PkgTermometro);", "property Style: TButtonStyle"]),
    ];
    let mut missing = Vec::new();
    for (language, texts) in expect {
        for t in texts {
            if !text[language.name()].contains(t) {
                missing.push(format!("{}: falta {t:?}", language.name()));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
    // Uma variante nova no tema: o gerado ganha o valor, sem mexer no form.
    project.language = Language::Rust;
    codegen::generate(&project).unwrap();
    std::fs::write(project.file("themes/empresa.json"), r##"{"name":"Empresa","base":"Aylis Light","styles":{"Button.perigo":{"fill":"#DC2626"},"Button.alerta":{"fill":"#F59E0B"}}}"##).unwrap();
    codegen::generate(&project).unwrap();
    let main = std::fs::read_to_string(project.code_dir_of(Language::Rust).join(aylis_project::bindings::file(Language::Rust))).unwrap();
    assert!(main.contains("    Alerta,"), "a variante nova do tema não entrou");
    std::fs::remove_dir_all(&base).unwrap();
}

/// O evento que declara o que leva ganha o handler tipado em todas as linguagens: quem
/// disparou na classe dele e o valor no tipo (o `on_change` do DateEdit, a data), e o registro
/// gerado converte. O handler de antes (o evento inteiro) continua sendo chamado do jeito dele.
#[test]
fn handlers_get_the_sender_and_the_value_typed() {
    let base = temp("typed_handlers");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","children":[{"class":"Button","name":"btnOk","on_click":"ok_click"},{"class":"DateEdit","name":"dteData","on_change":"data_change"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    let mut text = std::collections::HashMap::new();
    for language in Language::ALL {
        project.language = language;
        codegen::generate(&project).unwrap();
        let dir = project.code_dir_of(language);
        // O arquivo do usuário e o registro gerado.
        let mut t = std::fs::read_to_string(source(&project, language)).unwrap();
        for registry in ["generated/mod.rs", "src/forms/__init__.py", "src/forms/index.ts", "src/Forms/AllForms.cs", "src/forms/Forms.java", "src/forms/forms.go", "src/forms/forms.hpp", "src/forms/aylisforms.pas"] {
            t.push_str(&std::fs::read_to_string(dir.join(registry)).unwrap_or_default());
        }
        text.insert(language.name(), t);
    }
    let expect = [
        (
            Language::Rust,
            &[
                "pub fn ok_click(&mut self, ui: &mut Ui, sender: crate::generated::Button) {",
                "pub fn data_change(&mut self, ui: &mut Ui, sender: crate::generated::DateEdit, value: Option<aylis::prelude::Date>) {",
                "form_handler(FrmPrincipal::data_change)",
            ][..],
        ),
        (
            Language::Python,
            &[
                "def ok_click(aylis, sender: \"Button\"):",
                "def data_change(aylis, sender: \"DateEdit\", value: \"datetime.date | None\"):",
                "principal.data_change(aylis, components.frmPrincipal.sender_as_date_edit(event), _date(event.get(\"args\")))",
            ],
        ),
        (
            Language::TypeScript,
            &[
                "export function ok_click(aylis: Aylis, sender: import(\"./components.ts\").Button) {",
                "principal.data_change(aylis, components.frmPrincipal.senderAsDateEdit(ev)!, components.LocalDate.fromJson(ev.args))",
            ],
        ),
        (
            Language::CSharp,
            &["public static void data_change(Aylis aylis, AylisUi.DateEdit sender, DateOnly? value)", "FrmPrincipal.data_change(aylis, Components.frmPrincipal.SenderAsDateEdit(ev)!, Components.DateOf(ev.Args))"],
        ),
        (
            Language::Java,
            &[
                "public static void data_change(Aylis aylis, Components.DateEdit sender, java.time.LocalDate value) {",
                "FrmPrincipal.data_change(aylis, Components.frmPrincipal.senderAsDateEdit(ev), Components.asDate(ev.args()))",
            ],
        ),
        (Language::Go, &["func (f FrmPrincipal) data_change(app *aylis.Aylis, sender aylisDateEdit, value Date) {", "FrmPrincipal{}.data_change(app, frmPrincipal.SenderAsDateEdit(ev), asDate(ev.Args))"]),
        (
            Language::Cpp,
            &[
                "void data_change(aylis::Aylis& app, aylis_components::DateEdit sender, std::optional<aylis_components::LocalDate> value) {",
                "frm_principal::data_change(app, frmPrincipal.sender_as_date_edit(ev), LocalDate::parse(ev.args.str()));",
            ],
        ),
        (
            Language::Pascal,
            &["procedure TFrmPrincipal.data_change(Sender: TDateEdit; Value: TDateTime);", "TFrmPrincipal(Instance).data_change(TDateEdit(Sender), AsDate(Msg.Find('args')))", "RegisterTypedHandlers('frmPrincipal', @DispatchFrmPrincipal);"],
        ),
    ];
    let mut missing = Vec::new();
    for (language, texts) in expect {
        for t in texts {
            if !text[language.name()].contains(t) {
                missing.push(format!("{}: falta {t:?}", language.name()));
            }
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));

    // O handler de antes (o evento inteiro): o registro chama do jeito dele.
    project.language = Language::Python;
    let unit = source(&project, Language::Python);
    edit(&unit, "def ok_click(aylis, sender: \"Button\"):", "def ok_click(aylis, event):");
    codegen::generate(&project).unwrap();
    let registry = std::fs::read_to_string(project.code_dir_of(Language::Python).join("src/forms/__init__.py")).unwrap();
    assert!(registry.contains("(lambda event: principal.ok_click(aylis, event))"), "{registry}");
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

/// O `on_key_down` é pergunta e tipado ao mesmo tempo (como o `on_key_up`): o handler novo
/// recebe o `KeyEvent` e ainda responde (o `Result` em Rust; o `reply` dos clientes nas outras),
/// e o registro gerado chama o tipado com o registro da tecla.
#[test]
fn the_key_down_handler_gets_the_typed_key_and_still_answers() {
    let base = temp("key_down");
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    let layout = r#"{"aylis":1,"root":{"class":"Form","name":"frmPrincipal","key_preview":true,"on_key_down":"form_tecla","children":[{"class":"Edit","name":"edtNome","on_key_up":"solta"}]}}"#;
    std::fs::write(project.file("forms/principal.json"), layout).unwrap();
    codegen::generate(&project).unwrap();
    let rust = std::fs::read_to_string(source(&project, Language::Rust)).unwrap();
    assert!(
        rust.contains("pub fn form_tecla(&mut self, ui: &mut Ui, sender: crate::generated::Form, value: aylis::prelude::KeyEvent) -> Result<(), String> {"),
        "{rust}"
    );
    assert!(rust.contains("pub fn solta(&mut self, ui: &mut Ui, sender: crate::generated::Edit, value: aylis::prelude::KeyEvent) {"), "{rust}");

    project.language = Language::Python;
    codegen::generate(&project).unwrap();
    let python = std::fs::read_to_string(source(&project, Language::Python)).unwrap();
    assert!(python.contains("def form_tecla(aylis, sender: \"Form\", value: \"KeyEvent\"):"), "{python}");
    let init = std::fs::read_to_string(project.code_dir_of(Language::Python).join("src/forms/__init__.py")).unwrap();
    assert!(init.contains("form_tecla(aylis, components.frmPrincipal, KeyEvent.from_json("), "{init}");

    project.language = Language::Pascal;
    codegen::generate(&project).unwrap();
    let pascal = std::fs::read_to_string(source(&project, Language::Pascal)).unwrap();
    assert!(pascal.contains("procedure TFrmPrincipal.form_tecla(Sender: TForm; Value: TKeyEvent);"), "{pascal}");
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
        (Language::Python, &["def scroll_to(self, component: AylisComponent) -> None:", "args=[component.id]"]),
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
        (Language::Python, &["def save_to(self, path: str) -> bool:", "args=[path]", "def resize(self, width: int, height: int) -> bool:"]),
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
