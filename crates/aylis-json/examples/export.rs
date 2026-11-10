//! Gera o JSON Schema dos layouts (com os controles padrão) e os temas padrão em JSON.
//!
//! ```text
//! cargo run -p aylis-json --example export -- <pasta>
//! ```
//!
//! Cria `<pasta>/aylis.schema.json`, `<pasta>/aylis-light.theme.json` e
//! `<pasta>/aylis-dark.theme.json`.

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".into()));
    std::fs::create_dir_all(&dir)?;

    let mut ui = aylis_core::Ui::new();
    aylis_controls::register(&mut ui);
    let schema = serde_json::to_string_pretty(&aylis_json::schema(&ui))?;
    std::fs::write(dir.join("aylis.schema.json"), schema + "\n")?;
    std::fs::write(dir.join("aylis-light.theme.json"), aylis_json::save_theme(&aylis_core::Theme::light()) + "\n")?;
    std::fs::write(dir.join("aylis-dark.theme.json"), aylis_json::save_theme(&aylis_core::Theme::dark()) + "\n")?;
    println!("gerados em {}", dir.display());
    Ok(())
}
