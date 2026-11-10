//! Cria um projeto do aylis pela linha de comando (como um `cargo new`).
//!
//! ```text
//! cargo run -p aylis-project --example new_project -- <pasta> <nome> [rust|python]
//! ```

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(dir), Some(name)) = (args.first(), args.get(1)) else {
        eprintln!("uso: novo <pasta> <nome> [rust|python]");
        std::process::exit(2);
    };
    let language = match args.get(2).map(String::as_str) {
        Some("python") => aylis_project::Language::Python,
        _ => aylis_project::Language::Rust,
    };
    match aylis_project::create(std::path::Path::new(dir), name, language) {
        Ok(project) => println!("projeto criado: {}", project.path.display()),
        Err(e) => {
            eprintln!("erro: {e}");
            std::process::exit(1);
        }
    }
}
