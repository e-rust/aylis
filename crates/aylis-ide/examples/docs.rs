//! Refaz a documentação da página do aylis: `aylis-tool docs` (ou
//! `cargo run -p aylis-ide --example docs -- <pasta de saída>`). Veja `aylis_ide::reference`.

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::env::args().nth(1).map(std::path::PathBuf::from).unwrap_or_else(|| root.join("examples/Landing/assets/docs"));
    match aylis_ide::reference::write(&out, &root.join("docs")) {
        Ok(pages) => println!("{pages} páginas em {}", out.display()),
        Err(e) => {
            eprintln!("docs: {e}");
            std::process::exit(1);
        }
    }
}
