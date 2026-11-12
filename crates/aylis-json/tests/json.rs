use std::cell::RefCell;
use std::rc::Rc;

use aylis_controls::{Button, CheckBox, Edit, Form, Label, Panel};
use aylis_core::*;

/// Componente não visual com referência a outro (como um DataSource -> DataSet).
#[derive(Default)]
struct Link {
    target: Option<ComponentId>,
}

impl Component for Link {
    fn class_name(&self) -> &'static str {
        "Link"
    }

    properties! {
        target: Option<ComponentId> => "Data",
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.target == Some(freed) {
            self.target = None;
        }
    }
}

fn new_ui() -> Ui {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    ui.register_class::<Link>("Data");
    ui
}

const DOC: &str = r##"{
  "aylis": 1,
  "root": {
    "class": "Form",
    "name": "FrmClientes",
    "caption": "Clientes",
    "width": 640,
    "height": 480,
    "padding": 8,
    "children": [
      { "class": "Link", "name": "lnk", "target": "btnSalvar" },
      {
        "class": "Panel",
        "name": "pnlTopo",
        "align": "top",
        "height": 48,
        "padding": [8, 4],
        "fill": "primary",
        "children": [
          {
            "class": "Button",
            "name": "btnSalvar",
            "align": "left",
            "width": 100,
            "margins": { "right": 4 },
            "text": "Salvar",
            "style": "danger",
            "on_click": "btn_salvar_click"
          },
          { "class": "Label", "text": "sem nome", "align": "client", "margins.left": 10, "color": "#FF000080" }
        ]
      },
      { "class": "Edit", "name": "edtNome", "align": "top", "anchors": ["left", "right"], "placeholder": "Nome" },
      { "class": "CheckBox", "name": "chkAtivo", "checked": true, "anchors": "left,bottom" }
    ]
  }
}"##;

#[test]
fn load_builds_the_tree_with_properties() {
    let mut ui = new_ui();
    let form = aylis_json::load(&mut ui, DOC, None).unwrap();

    assert_eq!(ui.get::<Form>(form).unwrap().caption, "Clientes");
    assert_eq!(ui.control(form).unwrap().padding(), Bounds::all(8.0));

    let panel = ui.find("pnlTopo").unwrap();
    assert_eq!(ui.control(panel).unwrap().padding(), Bounds::symmetric(8.0, 4.0));
    assert_eq!(ui.get::<aylis_controls::Rectangle>(panel).unwrap().fill, Some(ui.theme().color("primary")));

    let btn = ui.find("btnSalvar").unwrap();
    assert_eq!(ui.parent(btn), Some(panel));
    assert_eq!(ui.get::<Button>(btn).unwrap().text, "Salvar");
    assert_eq!(ui.control(btn).unwrap().margins(), Bounds::new(0.0, 0.0, 4.0, 0.0));
    assert_eq!(ui.control(btn).unwrap().style(), "danger");

    let label = ui.children(panel)[1];
    assert_eq!(ui.control(label).unwrap().margins().left, 10.0);
    assert_eq!(ui.get::<Label>(label).unwrap().color, Some(Color::rgba(255, 0, 0, 128)));

    let edit = ui.find("edtNome").unwrap();
    assert_eq!(ui.control(edit).unwrap().anchors(), Anchors::LEFT | Anchors::RIGHT);
    assert!(ui.get::<CheckBox>(ui.find("chkAtivo").unwrap()).unwrap().checked);
    assert_eq!(ui.control(ui.find("chkAtivo").unwrap()).unwrap().anchors(), Anchors::LEFT | Anchors::BOTTOM);
}

#[test]
fn references_are_resolved_after_everything_exists() {
    let mut ui = new_ui();
    aylis_json::load(&mut ui, DOC, None).unwrap();
    // "lnk" aparece antes de "btnSalvar" no arquivo
    let link = ui.find("lnk").unwrap();
    assert_eq!(ui.get::<Link>(link).unwrap().target, ui.find("btnSalvar"));
}

#[test]
fn events_are_bound_by_handler_name() {
    let mut ui = new_ui();
    let clicks = Rc::new(RefCell::new(0));
    let c = clicks.clone();
    ui.register_handler("btn_salvar_click", move |_, _, _| *c.borrow_mut() += 1);
    aylis_json::load(&mut ui, DOC, None).unwrap();
    ui.click(ui.find("btnSalvar").unwrap());
    assert_eq!(*clicks.borrow(), 1);
}

#[test]
fn invalid_handler_names_are_refused() {
    let mut ui = new_ui();
    let doc = r#"{"aylis":1,"root":{"class":"Form","name":"frmX","children":[
        {"class":"Button","name":"btnSalvar","on_click":"x() {} pub fn y"}]}}"#;
    let err = aylis_json::load(&mut ui, doc, None).unwrap_err().to_string();
    assert!(err.contains("handler inválido"), "{err}");
}

#[test]
fn load_inside_an_existing_parent() {
    let mut ui = new_ui();
    let host = ui.add_root(Form::default());
    let piece = r#"{ "root": { "class": "Panel", "name": "pnlPeca", "children": [ { "class": "Button", "text": "OK" } ] } }"#;
    let panel = aylis_json::load(&mut ui, piece, Some(host)).unwrap();
    assert_eq!(ui.parent(panel), Some(host));
    assert_eq!(ui.children(panel).len(), 1);
}

fn error_of(doc: &str) -> String {
    let mut ui = new_ui();
    let err = aylis_json::load(&mut ui, doc, None).unwrap_err().to_string();
    assert!(ui.roots().is_empty(), "nada pode sobrar no Ui depois de um erro: {err}");
    err
}

#[test]
fn errors_say_where_and_what() {
    let e = error_of(r#"{ "root": { "class": "Form", "name": "Frm", "children": [ { "class": "Panel", "name": "pnl", "children": [ { "class": "Button", "texto": "x" } ] } ] } }"#);
    assert_eq!(e, "Frm > pnl > Button: propriedade 'texto': a classe Button não tem esta propriedade");

    let e = error_of(r#"{ "root": { "class": "Form", "children": [ { "class": "Botao" } ] } }"#);
    assert!(e.contains("Botao") && e.contains("não registrada"), "{e}");

    let e = error_of(r#"{ "root": { "class": "Form", "width": "grande" } }"#);
    assert_eq!(e, "Form: propriedade 'width': esperado número, recebido \"grande\"");

    let e = error_of(r#"{ "root": { "class": "Form", "align": "meio" } }"#);
    assert!(e.contains("'align'") && e.contains("client"), "{e}");

    let e = error_of(r#"{ "root": { "class": "Panel", "fill": "rosa_choque" } }"#);
    assert!(e.contains("nome de cor"), "{e}");

    let e = error_of(r#"{ "root": { "class": "Link", "target": "naoExiste" } }"#);
    assert!(e.contains("naoExiste") && e.contains("não encontrado"), "{e}");

    let e = error_of("{ \"root\": { \"class\": \"Form\", }\n}");
    assert!(e.starts_with("JSON inválido") && e.contains("line 1"), "{e}");

    let e = error_of(r#"{ "aylis": 99, "root": { "class": "Form" } }"#);
    assert!(e.contains("99"), "{e}");

    let e = error_of(r#"{ "raiz": { "class": "Form" } }"#);
    assert!(e.contains("raiz"), "{e}");

    let e = error_of(r#"{ "root": { "name": "x" } }"#);
    assert!(e.contains("class"), "{e}");
}

#[test]
fn save_writes_only_what_differs_from_the_defaults() {
    let mut ui = new_ui();
    let form = ui.add_root(Form::new("Teste", 640.0, 480.0));
    let btn = ui.add(Button::new("OK"), form);
    ui.set_name(btn, "btnOk").unwrap();
    ui.set_prop(btn, "on_click", "ok_click").unwrap();
    ui.on(btn, "on_pointer_enter", |_, _, _| {}); // closure: não vai para o arquivo
    let json: serde_json::Value = serde_json::from_str(&aylis_json::save(&ui, form).unwrap()).unwrap();

    assert_eq!(json["aylis"], 1);
    let root = &json["root"];
    assert_eq!(root["class"], "Form");
    assert_eq!(root["caption"], "Teste");
    assert!(root.get("visible").is_none(), "padrão não é gravado");
    assert!(root.get("width").is_none(), "640 já é o padrão do Form");
    let button = &root["children"][0];
    assert_eq!(*button, serde_json::json!({ "class": "Button", "name": "btnOk", "text": "OK", "on_click": "ok_click" }));
}

#[test]
fn load_save_load_round_trip_is_stable() {
    let mut ui = new_ui();
    let form = aylis_json::load(&mut ui, DOC, None).unwrap();
    let first = aylis_json::save(&ui, form).unwrap();

    let mut ui2 = new_ui();
    let form2 = aylis_json::load(&mut ui2, &first, None).unwrap();
    let second = aylis_json::save(&ui2, form2).unwrap();
    assert_eq!(first, second);

    // e o arquivo salvo usa as formas curtas
    let v: serde_json::Value = serde_json::from_str(&first).unwrap();
    let panel = &v["root"]["children"][1];
    assert_eq!(panel["padding"], serde_json::json!([8, 4]));
    assert_eq!(v["root"]["children"][0]["target"], "btnSalvar");
    assert_eq!(v["root"]["children"][1]["children"][1]["margins"], serde_json::json!([10, 0, 0, 0]));
}

#[test]
fn a_form_built_in_code_survives_save_and_load() {
    let mut ui = new_ui();
    let form = ui.add_root(Form::new("Código", 500.0, 300.0));
    let panel = ui.add(Panel::new(0.0, 40.0), form);
    ui.control_mut(panel).unwrap().set_align(Align::Top).set_opacity(0.75);
    let edit = ui.add(Edit::new("olá").with_placeholder("digite"), panel);
    ui.set_prop(edit, "password", true).unwrap();
    ui.add(PropertyAnimation::float("opacity", 0.5).with_trigger(Trigger::Hover), panel);

    let saved = aylis_json::save(&ui, form).unwrap();
    let mut ui2 = new_ui();
    let form2 = aylis_json::load(&mut ui2, &saved, None).unwrap();
    assert_eq!(aylis_json::save(&ui2, form2).unwrap(), saved);

    let panel2 = ui2.children(form2)[0];
    assert_eq!(ui2.control(panel2).unwrap().opacity(), 0.75);
    let edit2 = ui2.children(panel2)[0];
    assert!(ui2.get::<Edit>(edit2).unwrap().password);
    let anim = ui2.children(panel2)[1];
    assert_eq!(ui2.get::<PropertyAnimation>(anim).unwrap().trigger, Trigger::Hover);
}

#[test]
fn unregistered_classes_cannot_be_saved() {
    struct Secret {
        base: ControlBase,
    }
    impl Component for Secret {
        fn class_name(&self) -> &'static str {
            "Secret"
        }
        fn control(&self) -> Option<&ControlBase> {
            Some(&self.base)
        }
        fn control_mut(&mut self) -> Option<&mut ControlBase> {
            Some(&mut self.base)
        }
    }
    let mut ui = new_ui();
    let form = ui.add_root(Form::default());
    ui.add(Secret { base: ControlBase::default() }, form);
    let e = aylis_json::save(&ui, form).unwrap_err().to_string();
    assert!(e.contains("Secret") && e.contains("registr"), "{e}");
}

#[test]
fn themes_round_trip() {
    for theme in [Theme::light(), Theme::dark()] {
        let json = aylis_json::save_theme(&theme);
        let back = aylis_json::load_theme(&json).unwrap();
        assert_eq!(back, theme, "{}", theme.name);
        assert_eq!(aylis_json::save_theme(&back), json);
    }
}

#[test]
fn hand_written_theme_with_short_forms() {
    let theme = aylis_json::load_theme(
        r##"{
          "name": "Meu",
          "colors": { "primary": "#10B981", "text": "#111111" },
          "fonts": { "default": { "family": "Inter", "size": 15 }, "title": { "size": 22, "weight": "bold" } },
          "radius": { "md": 8 },
          "durations": { "fast": 100 },
          "styles": {
            "Button": { "fill": "primary", "radius": "md", "padding": [12, 6], "transition": "fast",
                        "hover": { "fill": "#34D399" }, "focused": { "border": ["primary", 2] } }
          }
        }"##,
    )
    .unwrap();
    let style = theme.resolve(&["Button"], "", ControlState::default());
    assert_eq!(style.fill, Color::rgb(0x10, 0xB9, 0x81));
    assert_eq!(style.radius, 8.0);
    assert_eq!(style.font.family, "Inter");
    let hover = theme.resolve(&["Button"], "", ControlState { hover: true, ..Default::default() });
    assert_eq!(hover.fill, Color::rgb(0x34, 0xD3, 0x99));

    let e = aylis_json::load_theme(r#"{ "styles": { "Button": { "fil": "primary" } } }"#).unwrap_err().to_string();
    assert!(e.contains("Button") && e.contains("'fil'"), "{e}");
    let e = aylis_json::load_theme(r#"{ "fonts": { "x": { "weight": "gordinha" } } }"#).unwrap_err().to_string();
    assert!(e.contains("gordinha") && e.contains("bold"), "{e}");
}

#[test]
fn schema_lists_every_registered_class() {
    let ui = new_ui();
    let schema = aylis_json::schema(&ui);
    let defs = &schema["$defs"];
    let button = &defs["Button"]["properties"];
    assert_eq!(button["class"]["const"], "Button");
    assert_eq!(button["text"]["type"], "string");
    assert!(button["on_click"].is_object(), "eventos aparecem");
    assert_eq!(button["align"]["enum"][9], "client");
    assert!(defs["Link"]["properties"]["target"].is_object());
    let classes = defs["node"]["properties"]["class"]["enum"].as_array().unwrap().len();
    assert_eq!(defs["node"]["anyOf"].as_array().unwrap().len(), classes);
    assert!(classes >= 14);
}

#[test]
fn the_example_layout_file_loads() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../aylis-platform-winit/examples/forms/registration.json");
    let json = std::fs::read_to_string(path).unwrap();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = aylis_json::load(&mut ui, &json, None).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(ui.name(form), Some("FrmClientes"));
    assert!(ui.find("btnOk").is_some() && ui.find("lblStatus").is_some());
}

#[test]
fn theme_mixes_components_and_shadows() {
    let json = r##"{
        "name": "Marca",
        "colors": { "primary": "#C00000" },
        "shadows": { "card": { "color": "#00000033", "blur": 10, "offset": [0, 3] } },
        "styles": { "Panel.vitrine": { "fill": "surface", "shadow": "card" } },
        "components": { "Button": "Aylis Dark" }
    }"##;
    let theme = aylis_json::load_theme(json).unwrap();
    assert_eq!(theme.components.get("Button").map(String::as_str), Some("Aylis Dark"));
    assert_eq!(theme.shadows["card"].blur, 10.0);
    assert_eq!(theme.styles["Panel.vitrine"].base.shadow.as_deref(), Some("card"));
    let again = aylis_json::load_theme(&aylis_json::save_theme(&theme)).unwrap();
    assert_eq!(again, theme, "ida e volta sem perder nada");
    assert!(aylis_json::load_theme(r#"{"components": {"Button": 3}}"#).is_err());
}

#[test]
fn theme_look_family_and_dark_round_trip() {
    let mut theme = aylis_core::themes::builtin("Windows 11 Dark").unwrap().clone();
    theme.name = "Empresa".into();
    let json = aylis_json::save_theme(&theme);
    assert!(json.contains("\"look\": \"win11\"") && json.contains("\"dark\": true"), "{json}");
    let back = aylis_json::load_theme(&json).unwrap();
    assert_eq!((back.look, back.dark, back.family.as_str()), (Look::Win11, true, "Windows 11"));
    assert!(aylis_json::load_theme(r#"{"look": "windows95"}"#).unwrap_err().to_string().contains("win11"));
}

#[test]
fn theme_can_start_from_a_library_theme() {
    let theme = aylis_json::load_theme(r##"{ "name": "Meu", "base": "Aylis Dark", "colors": { "primary": "#FF0000" } }"##).unwrap();
    assert_eq!(theme.name, "Meu");
    assert_eq!(theme.color("primary"), aylis_core::Color::from_hex("#FF0000").unwrap());
    let dark = aylis_core::themes::builtin("Aylis Dark").unwrap();
    assert_eq!(theme.color("background"), dark.color("background"), "o resto vem da base");
    assert!(theme.style("Button").is_some(), "com os estilos da base");
    assert!(aylis_json::load_theme(r#"{ "base": "Não Existe" }"#).unwrap_err().to_string().contains("não existe"));
}

#[test]
fn theme_colors_can_cite_other_colors() {
    let theme = aylis_json::load_theme(r##"{ "name": "T", "base": "Aylis Dark", "colors": { "accent_soft": "accent/15", "accent": "#3B82F6" } }"##).unwrap();
    let accent = aylis_core::Color::rgb(0x3B, 0x82, 0xF6);
    assert_eq!(theme.color("accent_soft"), accent.with_opacity(0.15));
    // Nos estilos, `nome/NN` também vale.
    assert_eq!(theme.color("accent/50"), accent.with_opacity(0.5));
}

/// Forms bons para merge (item 22): o formato gravado é fixo (uma chave por linha, listas
/// curtas numa linha), salvar o que se abriu dá o mesmo texto, e o `value` gravado antes do
/// `max` (a ordem da classe) não é cortado pelo máximo padrão ao abrir.
#[test]
fn saved_forms_are_stable_and_limits_load_before_the_value() {
    let mut ui = new_ui();
    let doc = r#"{
  "aylis": 1,
  "root": {
    "class": "Form",
    "name": "frmX",
    "padding": [24, 10],
    "children": [
      {
        "class": "NumberBox",
        "name": "nbTotal",
        "value": 10000000,
        "min": 1000,
        "max": 20000000
      }
    ]
  }
}
"#;
    let form = aylis_json::load(&mut ui, doc, None).unwrap();
    let nb = ui.find_in(form, "nbTotal").unwrap();
    assert_eq!(ui.get_prop(nb, "value").unwrap().to_f64().unwrap(), 10_000_000.0, "não cortou no máximo padrão");
    let saved = aylis_json::save(&ui, form).unwrap();
    assert!(saved.contains("\n    \"padding\": [24, 10],\n"), "lista curta numa linha:\n{saved}");
    let mut ui2 = new_ui();
    let form2 = aylis_json::load(&mut ui2, &saved, None).unwrap();
    assert_eq!(aylis_json::save(&ui2, form2).unwrap(), saved, "abrir e salvar de novo não muda nada");
}

/// Um tema sem `"dark"`, de fundo escuro, é escuro (relatório do InsightDB, 11/11).
#[test]
fn a_dark_background_makes_a_dark_theme() {
    let dark = aylis_json::load_theme(r##"{"name": "Painel", "colors": {"background": "#0B0F19", "text": "#E5E7EB"}}"##).unwrap();
    assert!(dark.dark);
    let light = aylis_json::load_theme(r##"{"name": "Claro", "colors": {"background": "#FFFFFF"}}"##).unwrap();
    assert!(!light.dark);
    let forced = aylis_json::load_theme(r##"{"name": "X", "dark": false, "colors": {"background": "#000000"}}"##).unwrap();
    assert!(!forced.dark, "o \"dark\" do arquivo vale");
}