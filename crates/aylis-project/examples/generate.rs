//! Gera (ou atualiza) o código dos eventos de um projeto, como o "Gerar código" da IDE.
//!
//! ```text
//! cargo run -p aylis-project --example generate -- <projeto.aylisproj>
//! ```

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("uso: gerar <projeto.aylisproj>");
        std::process::exit(2);
    };
    let result = aylis_project::Project::open(&path).and_then(|p| aylis_project::codegen::generate(&p));
    match result {
        Ok(files) => files.iter().for_each(|f| println!("{}", f.display())),
        Err(e) => {
            eprintln!("erro: {e}");
            std::process::exit(1);
        }
    }
}
