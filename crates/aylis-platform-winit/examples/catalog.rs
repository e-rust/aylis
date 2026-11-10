//! Os mesmos componentes em dois "celulares": Material 3 e iOS 26 (Liquid Glass). Cada
//! tela só troca a propriedade `theme` — o look do tema muda a forma dos controles.
//!
//! ```text
//! cargo run -p aylis-platform-winit --example catalog
//! cargo run -p aylis-platform-winit --example catalog -- --dark
//! cargo run -p aylis-platform-winit --example catalog -- --list   # a 2ª tela de cada
//! ```

#[path = "shared/catalog_form.rs"]
mod catalog_form;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let dark = args.iter().any(|a| a == "--dark");
    let second = args.iter().any(|a| a == "--list");
    let (ui, form) = catalog_form::build(dark, second)?;
    aylis_platform_winit::run(ui, form)
}
