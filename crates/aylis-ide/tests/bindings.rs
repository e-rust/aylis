//! "Ligar a campo…" no designer: a ligação entra no form (com desfazer), a lista mostra os
//! DataSource e os campos, e o JSON guarda a ligação, não o valor do registro.

use aylis_ide::document::Document;
use serde_json::json;

#[test]
fn the_designer_binds_a_property_to_a_field_with_undo() {
    let mut doc = Document::new();
    let cds = doc.add("ClientDataSet", None).unwrap();
    doc.rename_component(cds, "cdsProdutos").unwrap();
    doc.select(Some(cds));
    let field = doc.add("Field", None).unwrap();
    doc.set_prop(field, "field_name", &json!("nome")).unwrap();
    doc.select(None);
    let ds = doc.add("DataSource", None).unwrap();
    doc.rename_component(ds, "dsProdutos").unwrap();
    doc.set_prop(ds, "data_set", &json!("cdsProdutos")).unwrap();
    let label = doc.add("Label", None).unwrap();
    assert_eq!(doc.data_sources(), ["dsProdutos"]);
    assert_eq!(doc.fields_of("dsProdutos"), ["nome"]);

    doc.set_binding(label, "text", "dsProdutos", "nome").unwrap();
    let b = doc.ui.binding(label, "text").unwrap();
    assert_eq!((b.source, b.path.as_str()), (Some(ds), "nome"));
    let saved = doc.to_json().unwrap();
    assert!(saved.contains("\"bindings\"") && saved.contains("\"dsProdutos\""), "{saved}");
    // Desfazer tira a ligação.
    assert!(doc.undo());
    let label = doc.ui.find("Label1").unwrap();
    assert!(doc.ui.binding(label, "text").is_none());
    assert!(doc.set_binding(label, "text", "dsNaoExiste", "nome").is_err());
}

#[test]
fn the_old_data_controls_no_longer_exist() {
    let old = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmAntigo", "children": [
        { "class": "DBEdit", "name": "edtNome", "data_source": "ds", "data_field": "nome" }
    ] } });
    let err = Document::from_json(&old.to_string()).err().expect("o DBEdit não abre mais");
    assert!(err.contains("DBEdit"), "{err}");
}

/// Ligar dois componentes arrastando (o `Pipe` escolhido na paleta): cada ponta no lado mais
/// perto de onde o mouse apertou e soltou, num passo do desfazer.
#[test]
fn dragging_a_pipe_from_one_component_to_another_connects_them() {
    use aylis_core::{PointF, PropValue};
    let mut doc = Document::new();
    let tank = doc.add("TankLevel", Some(PointF::new(20.0, 20.0))).unwrap();
    let valve = doc.add("Valve", Some(PointF::new(300.0, 20.0))).unwrap();
    let (t, v) = (doc.rect_of(tank).unwrap(), doc.rect_of(valve).unwrap());
    // Do lado direito do tanque ao lado esquerdo da válvula.
    let pipe = doc.connect("Pipe", tank, PointF::new(t.right() - 2.0, t.y + t.height / 2.0), valve, PointF::new(v.x + 2.0, v.y + v.height / 2.0)).unwrap();
    assert_eq!(doc.ui.get_prop(pipe, "from").unwrap(), PropValue::Component(Some(tank)));
    assert_eq!(doc.ui.get_prop(pipe, "to").unwrap(), PropValue::Component(Some(valve)));
    assert_eq!(doc.ui.get_prop(pipe, "from_port").unwrap(), PropValue::Enum("right".into()));
    assert_eq!(doc.ui.get_prop(pipe, "to_port").unwrap(), PropValue::Enum("left".into()));
    let json = doc.to_json().unwrap();
    assert!(json.contains("\"class\": \"Pipe\"") || json.contains("\"class\":\"Pipe\""), "{json}");
    assert!(doc.undo());
    assert!(doc.ui.find("Pipe1").is_none(), "Ctrl+Z tira o cano");
    assert!(doc.connect("Pipe", tank, PointF::ZERO, tank, PointF::ZERO).is_err(), "no mesmo componente, não");
}

/// Editar numa vista por largura no designer: o que muda vira mudança da vista, o form fica
/// com o base, e o Ctrl+Z continua na vista.
#[test]
fn editing_in_a_width_view_keeps_the_base_and_records_the_view() {
    use aylis_core::{PointF, PropValue};
    let mut doc = Document::new();
    let btn = doc.add("Button", Some(PointF::new(300.0, 40.0))).unwrap();
    let base_left = doc.ui.get_prop(btn, "left").unwrap();
    doc.enter_view(Some("phone"), 390.0);
    assert_eq!(doc.ui.get_prop(doc.root, "width").unwrap(), PropValue::Float(390.0), "a prévia na largura do celular");
    // Mexe no celular: só a vista muda.
    doc.begin_change();
    doc.ui.set_prop(btn, "left", 8.0).unwrap();
    doc.ui.set_prop(btn, "visible", false).unwrap();
    doc.sync_view();
    let saved: serde_json::Value = serde_json::from_str(&doc.to_json().unwrap()).unwrap();
    let b = saved["root"]["children"].as_array().unwrap().iter().find(|c| c["class"] == "Button").unwrap().clone();
    assert_eq!(b["left"].as_f64(), match base_left { PropValue::Float(f) => Some(f), _ => None }, "o form guarda o base");
    assert!(b.get("visible").is_none());
    assert_ne!(saved["root"]["width"], 390.0, "a prévia não muda a largura do form");
    let phone = saved["root"]["views"].as_array().unwrap().iter().find(|v| v["name"] == "phone").unwrap().clone();
    assert_eq!(phone["max_width"], 599.0);
    let name = b["name"].as_str().unwrap();
    assert_eq!(phone["changes"][name]["left"], 8.0);
    assert_eq!(phone["changes"][name]["visible"], false);
    // Ctrl+Z: continua no celular, sem as mudanças.
    assert!(doc.undo());
    assert_eq!(doc.view_mode.as_ref().map(|m| m.name.as_str()), Some("phone"));
    assert_eq!(doc.ui.get_prop(btn_of(&doc), "visible").unwrap(), PropValue::Bool(true));
    // De volta ao base: tudo como estava.
    doc.enter_view(None, 0.0);
    assert!(doc.view_mode.is_none());
    assert_eq!(doc.ui.get_prop(btn_of(&doc), "left").unwrap(), base_left);
}

fn btn_of(doc: &Document) -> aylis_core::ComponentId {
    doc.ui.find("Button1").unwrap()
}
