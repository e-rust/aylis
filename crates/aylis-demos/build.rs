//! O registro das classes da família deste wasm: a categoria da paleta em `AYLIS_DEMO_FAMILY`
//! ("Standard", "Data Controls"...; sem ela, nenhuma). Só elas entram no wasm. A categoria de
//! cada classe vem do `aylis::classes` (a mesma da paleta e das seções da documentação).

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../aylis/src/lib.rs");
    println!("cargo::rerun-if-env-changed=AYLIS_DEMO_FAMILY");
    let family = std::env::var("AYLIS_DEMO_FAMILY").unwrap_or_default();
    let facade = std::fs::read_to_string("../aylis/src/lib.rs").expect("ler crates/aylis/src/lib.rs");
    let mut source = String::from("/// Registra as classes da família deste wasm.\npub fn register(ui: &mut ::aylis::prelude::Ui) {\n    let _ = &ui;\n");
    for (class, category) in aylis_build::class_categories(&facade) {
        // A feature que a classe pede no `aylis` tem de estar ligada (o `aylis-tool demos` liga).
        let enabled = aylis_build::demo_feature(class).is_none_or(|f| std::env::var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok());
        if category == family && aylis_build::demo_on_web(class) && enabled {
            source.push_str(&format!("    ::aylis::classes::{class}(ui);\n"));
        }
    }
    source.push_str("}\n");
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("family.rs");
    std::fs::write(out, source).unwrap();
}
