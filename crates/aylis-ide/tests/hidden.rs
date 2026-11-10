//! No designer, os componentes escondidos (`visible = false`) não aparecem nem ocupam lugar,
//! a não ser selecionados (ou com Exibir → Mostrar componentes escondidos).

use aylis_ide::document::Document;
use serde_json::json;

#[test]
fn hidden_components_appear_only_when_selected() {
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmTeste", "width": 400, "height": 300, "padding": 0, "children": [
        { "class": "Panel", "name": "pnlLado", "align": "right", "width": 100, "visible": false,
          "children": [ { "class": "Button", "name": "btnDentro", "align": "top" } ] },
        { "class": "Panel", "name": "pnlCorpo", "align": "client" }
    ] } });
    let mut doc = Document::from_json(&layout.to_string()).unwrap();
    let lado = doc.ui.find_in(doc.root, "pnlLado").unwrap();
    let dentro = doc.ui.find_in(doc.root, "btnDentro").unwrap();
    let corpo = doc.ui.find_in(doc.root, "pnlCorpo").unwrap();
    let shown = |doc: &Document, id| doc.ui.control(id).unwrap().visible();
    // Escondido: não aparece e o corpo ocupa a largura toda, como no app.
    assert!(!shown(&doc, lado));
    assert_eq!(doc.ui.bounds(corpo).unwrap().width, 400.0);
    // Selecionado (ou algo dentro dele): aparece para editar.
    doc.select(Some(dentro));
    assert!(shown(&doc, lado));
    assert_eq!(doc.ui.bounds(corpo).unwrap().width, 300.0);
    doc.select(Some(corpo));
    assert!(!shown(&doc, lado));
    // Mostrar todos.
    doc.show_hidden = true;
    doc.sync_reveal();
    assert!(shown(&doc, lado));
    doc.show_hidden = false;
    doc.sync_reveal();
    // O arquivo continua com o painel escondido.
    let saved = aylis_json::save_value(&doc.ui, doc.root).unwrap();
    assert_eq!(saved["root"]["children"][0]["visible"], json!(false));
    // Ligar pelo Object Inspector mostra na hora.
    doc.set_prop(lado, "visible", &json!(true)).unwrap();
    assert!(shown(&doc, lado));
}
