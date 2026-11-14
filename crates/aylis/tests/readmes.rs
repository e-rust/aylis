//! Cada crate publicado no crates.io tem o README dele (a página do crates.io sai vazia sem
//! ele): o arquivo ao lado do `Cargo.toml` e o `readme = "README.md"` no manifesto.

use std::path::Path;

#[test]
fn every_published_crate_has_a_readme() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(crates).unwrap() {
        let dir = entry.unwrap().path();
        let Ok(manifest) = std::fs::read_to_string(dir.join("Cargo.toml")) else { continue };
        if manifest.lines().any(|l| l.trim().replace(' ', "") == "publish=false") {
            continue;
        }
        let name = dir.file_name().unwrap().to_string_lossy().to_string();
        if !manifest.lines().any(|l| l.trim().replace(' ', "") == "readme=\"README.md\"") {
            missing.push(format!("{name}: sem `readme = \"README.md\"` no Cargo.toml"));
        }
        match std::fs::read_to_string(dir.join("README.md")) {
            Ok(text) if text.contains("https://github.com/e-rust/aylis") => {}
            Ok(_) => missing.push(format!("{name}: o README não leva ao repositório")),
            Err(_) => missing.push(format!("{name}: sem README.md")),
        }
    }
    assert!(missing.is_empty(), "crates publicados sem README:\n{}", missing.join("\n"));
}
