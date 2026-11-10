//! `resources/components.json`, o catálogo das classes que a geração de código usa para o
//! acesso tipado (`frmX.pnlLateral.Visible`) nas linguagens, em dia com o registro. Depois
//! de mudar uma classe: `aylis-tool update-catalog`.

use aylis_core::Ui;

#[test]
fn the_components_catalog_matches_the_registry() {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    // O 3D e os documentos: classes que o protocolo não registra, mas os forms usam.
    aylis_3d::register(&mut ui);
    aylis_docview::register(&mut ui);
    let text = aylis_json::catalog(&ui);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../resources/components.json");
    let saved = std::fs::read_to_string(path).unwrap_or_default().replace("\r\n", "\n");
    assert!(saved == text, "resources/components.json desatualizado: rode aylis-tool update-catalog");
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["Panel"]["visible"], "bool");
    assert_eq!(value["Label"]["text"], "string");
}
