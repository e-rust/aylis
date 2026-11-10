//! Regrava `resources/components.json` (o catálogo das classes para o acesso tipado nas
//! linguagens) a partir do registro. Rode por `aylis-tool update-catalog`.

use aylis_core::Ui;

fn main() {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    // O 3D e os documentos: classes que o protocolo não registra, mas os forms usam.
    aylis_3d::register(&mut ui);
    aylis_docview::register(&mut ui);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../resources/components.json");
    std::fs::write(path, aylis_json::catalog(&ui)).expect("resources/components.json");
    println!("resources/components.json atualizado");
}
