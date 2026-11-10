use aylis_core::{Align, PointF, PropValue};
use aylis_ide::document::{Document, Handle};
use serde_json::json;

fn text(doc: &Document, name: &str, prop: &str) -> PropValue {
    doc.ui.get_prop(doc.ui.find(name).unwrap(), prop).unwrap()
}

#[test]
fn add_names_like_delphi_and_snaps_to_the_grid() {
    let mut doc = Document::new();
    let b1 = doc.add("Button", Some(PointF::new(37.0, 21.0))).unwrap();
    let b2 = doc.add("Button", Some(PointF::new(100.0, 100.0))).unwrap();
    assert_eq!(doc.ui.name(b1), Some("Button1"));
    assert_eq!(doc.ui.name(b2), Some("Button2"));
    assert_eq!(text(&doc, "Button1", "text"), PropValue::String("Button1".into()), "texto inicial = nome");
    let cb = doc.ui.control(b1).unwrap();
    assert_eq!((cb.left(), cb.top()), (40.0, 24.0));
    assert_eq!(doc.selection, Some(b2));
    assert!(doc.modified);
}

#[test]
fn controls_dropped_on_a_panel_go_inside_it() {
    let mut doc = Document::new();
    let panel = doc.add("Panel", Some(PointF::new(40.0, 40.0))).unwrap();
    doc.set_prop(panel, "width", &json!(200)).unwrap();
    doc.set_prop(panel, "height", &json!(120)).unwrap();
    // um Label solto sobre o painel entra nele, na posição relativa
    let label = doc.add("Label", Some(PointF::new(56.0, 64.0))).unwrap();
    assert_eq!(doc.ui.parent(label), Some(panel));
    assert_eq!(doc.ui.control(label).unwrap().left(), 16.0);
    // e o Label (hit_test = false na execução) é selecionável no designer
    assert_eq!(doc.component_at(PointF::new(60.0, 70.0)), Some(label));
    assert_eq!(doc.container_at(PointF::new(60.0, 70.0)), panel);
}

#[test]
fn non_visual_components_need_the_right_owner() {
    let mut doc = Document::new();
    assert!(doc.add("Field", None).unwrap_err().contains("ClientDataSet"));
    let cds = doc.add("ClientDataSet", None).unwrap();
    let field = doc.add("Field", None).unwrap();
    assert_eq!(doc.ui.parent(field), Some(cds), "Field entra no ClientDataSet selecionado");
    assert_eq!(doc.non_visual(), vec![cds], "na bandeja só o que está direto no form");

    let button = doc.add("Button", Some(PointF::new(10.0, 10.0))).unwrap();
    let anim = doc.add("FloatAnimation", None).unwrap();
    assert_eq!(doc.ui.parent(anim), Some(button));
    assert_eq!(doc.non_visual(), vec![cds]);
}

#[test]
fn move_and_resize_respect_alignment() {
    let mut doc = Document::new();
    let b = doc.add("Button", Some(PointF::new(16.0, 16.0))).unwrap();
    let start = doc.start_rect(b).unwrap();
    doc.move_from(b, start, PointF::new(23.0, 9.0));
    let cb = doc.ui.control(b).unwrap();
    assert_eq!((cb.left(), cb.top()), (40.0, 24.0));

    let start = doc.start_rect(b).unwrap();
    doc.resize_from(b, Handle::BottomRight, start, PointF::new(30.0, 10.0));
    let cb = doc.ui.control(b).unwrap();
    assert_eq!((cb.width(), cb.height()), (128.0, 40.0));

    // alinhado no topo: só a altura muda e não dá para arrastar
    doc.set_prop(b, "align", &json!("top")).unwrap();
    assert!(!doc.movable(b));
    let start = doc.start_rect(b).unwrap();
    doc.resize_from(b, Handle::BottomRight, start, PointF::new(100.0, 16.0));
    let cb = doc.ui.control(b).unwrap();
    assert_eq!((cb.width(), cb.height()), (128.0, 56.0));
    assert_eq!(cb.align(), Align::Top);
}

#[test]
fn undo_and_redo_restore_the_whole_document() {
    let mut doc = Document::new();
    doc.add("Button", Some(PointF::new(8.0, 8.0))).unwrap();
    let edit = doc.add("Edit", Some(PointF::new(8.0, 56.0))).unwrap();
    doc.set_prop(edit, "text", &json!("olá")).unwrap();
    assert!(doc.undo());
    assert_eq!(text(&doc, "Edit1", "text"), PropValue::String("Edit1".into()));
    assert!(doc.undo());
    assert!(doc.ui.find("Edit1").is_none());
    assert!(doc.redo());
    assert!(doc.redo());
    assert_eq!(text(&doc, "Edit1", "text"), PropValue::String("olá".into()));
    assert_eq!(doc.selection.and_then(|s| doc.ui.name(s)), Some("Edit1"), "seleção volta pelo nome");

    // valor inválido não entra no histórico
    let before = doc.can_undo();
    assert!(doc.set_prop(doc.ui.find("Edit1").unwrap(), "max_length", &json!("muito")).is_err());
    assert_eq!(doc.can_undo(), before);
}

#[test]
fn copy_paste_renames_and_offsets() {
    let mut doc = Document::new();
    let panel = doc.add("Panel", Some(PointF::new(16.0, 16.0))).unwrap();
    doc.add("Button", Some(PointF::new(24.0, 24.0))).unwrap();
    doc.select(Some(panel));
    assert!(doc.copy_selected());
    doc.select(Some(doc.root));
    let copy = doc.paste().unwrap();
    assert_eq!(doc.ui.name(copy), Some("Panel2"));
    assert_eq!(doc.ui.name(doc.ui.children(copy)[0]), Some("Button2"));
    assert_eq!(doc.ui.control(copy).unwrap().left(), 32.0);
}

#[test]
fn delete_save_and_reopen() {
    let mut doc = Document::new();
    let b = doc.add("Button", Some(PointF::new(8.0, 8.0))).unwrap();
    doc.set_prop(b, "on_click", &json!("button1_click")).unwrap();
    doc.add("Label", Some(PointF::new(8.0, 64.0))).unwrap();
    assert!(doc.delete_selected());
    assert!(doc.ui.find("Label1").is_none());
    assert!(!{
        let mut d = Document::new();
        d.delete_selected()
    }, "o form não pode ser excluído");

    assert_eq!(doc.handlers(), vec![("Button1".to_string(), "on_click".to_string(), "button1_click".to_string())]);
    let path = std::env::temp_dir().join("aylis_ide_doc_test.json");
    doc.save_to(&path).unwrap();
    let reopened = Document::open(&path).unwrap();
    assert_eq!(reopened.to_json().unwrap(), doc.to_json().unwrap());
    assert!(!reopened.modified);
    let _ = std::fs::remove_file(path);
}
