//! Embute o projeto (forms, temas, assets) no executável. Gerado pela IDE do aylis.

fn main() {
    aylis_build::project();
    // A documentação ao vivo: as famílias leves vêm no próprio wasm da página (sem baixar o
    // núcleo de novo num wasm à parte). As pesadas (Additional, 3D, mídia, dados, relatórios,
    // provedores) continuam no `aylis-tool demos`, carregadas só na página delas. O tamanho de
    // cada uma está em `medicoes/web_wasm.json`.
    aylis_build::demo_classes(
        std::path::Path::new("../../../crates/aylis/src/lib.rs"),
        &["Standard", "Layouts", "Menus", "Shapes", "Animations", "Electrical", "Industrial", "Trader", "BI", "Device", "System", "Quasar"],
    );
}
