//! Linux pelo WSL de ponta a ponta: um projeto Rust novo publicado para Linux (o executável
//! e o `.deb`) e executado pela janela do WSLg. Demora minutos e precisa do WSL com uma
//! distribuição preparada (Executar → Verificar ambiente): `cargo test -p aylis-tools --test
//! wsl -- --ignored`.

use aylis_project::Language;
use aylis_tools::{Control, Event, Target, publish, run, wsl};

#[test]
#[ignore]
fn a_rust_project_publishes_and_runs_on_linux_through_wsl() {
    assert!(wsl::available(), "sem WSL");
    let distro = wsl::distro().expect("sem distribuição Linux no WSL");
    eprintln!("distribuição: {distro}");
    let base = std::env::temp_dir().join("aylis_wsl_teste");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let project = aylis_project::create(&base, "OlaLinux", Language::Rust).unwrap();
    assert!(Target::available(Language::Rust).contains(&Target::Linux));
    assert_eq!(Target::available(Language::Python), [Target::Desktop]);

    let mut lines = Vec::new();
    let mut sink = |e: Event| {
        if let Event::Step(s) | Event::Line(s, _) = &e {
            eprintln!("{s}");
            lines.push(s.clone());
        }
    };
    // Publicar: o executável Linux e o pacote .deb.
    let mut options = publish::Options::new(Target::Linux);
    options.installer = true;
    let published = publish::publish(&project, &options, &Control::new(), &mut sink).expect("publicar");
    assert!(published.main.extension().is_some_and(|e| e == "deb"), "{}", published.main.display());
    let exe = published.dir.join("OlaLinux");
    let head = std::fs::read(&exe).unwrap();
    assert_eq!(&head[..4], b"\x7fELF", "executável Linux");

    // Executar: compila no WSL e abre pelo WSLg. Sem variável de ambiente para fechar o app:
    // ele roda alguns segundos e o teste interrompe (como o Parar da IDE). Se ele cair antes,
    // o run devolve "Programa terminou" e o teste falha.
    let options = run::Options::new(Target::Linux);
    let control = Control::new();
    let mut sink = |e: Event| {
        if let Event::Step(s) | Event::Line(s, _) = &e {
            if s.contains("Executando") {
                let control = control.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(8));
                    control.cancel();
                });
            }
            eprintln!("{s}");
        }
    };
    let result = run::run(&project, &options, &control, &mut sink);
    assert_eq!(result.unwrap_err(), aylis_tools::process::CANCELLED, "o app ficou aberto até o Parar");
}
