//! O índice da pasta de compilação com o cargo de verdade: um projeto pequeno (lib, bin,
//! teste e build script) compilado pelo `process::cargo` numa pasta só do teste.

use std::path::Path;

use aylis_tools::build_index::Index;
use aylis_tools::process::{cargo, Control};

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

#[test]
fn a_build_records_its_units_and_cleaning_the_project_removes_them() {
    let root = std::env::temp_dir().join(format!("aylis_build_index_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let (dir, target) = (root.join("app"), root.join("target"));
    write(&dir.join("Cargo.toml"), "[package]\nname = \"meu-app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n");
    write(&dir.join("build.rs"), "fn main() { println!(\"cargo:rerun-if-changed=build.rs\"); }\n");
    write(&dir.join("src/lib.rs"), "pub fn soma(a: i32, b: i32) -> i32 { a + b }\n");
    write(&dir.join("src/main.rs"), "fn main() { println!(\"{}\", meu_app::soma(1, 2)); }\n");
    write(&dir.join("tests/soma.rs"), "#[test]\nfn soma() { assert_eq!(meu_app::soma(1, 2), 3); }\n");

    let envs = [("CARGO_TARGET_DIR", target.as_path())];
    let mut sink = |_| {};
    cargo("cargo", &["build"], &dir, &envs, &Control::new(), &mut sink).unwrap();
    cargo("cargo", &["test", "--no-run"], &dir, &envs, &Control::new(), &mut sink).unwrap();

    let index = Index::load(&target);
    let project = index.projects.get(&aylis_tools::build_index::project_key(&dir)).expect("o projeto está no índice");
    let fingerprints: Vec<String> = std::fs::read_dir(target.join("debug").join(".fingerprint")).unwrap().flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
    // Toda unidade anotada existe no disco, e é do projeto.
    for (hash, u) in &project.units {
        assert!(fingerprints.iter().any(|f| f.ends_with(hash.as_str())), "{hash} ({}) não está no .fingerprint", u.target);
        assert!(u.own, "{} é do projeto", u.target);
    }
    let targets: Vec<&str> = project.units.values().map(|u| u.target.as_str()).collect();
    for wanted in ["debug/meu-app/lib/meu_app", "debug/meu-app/bin/meu-app", "debug/meu-app/test/soma", "debug/meu-app/custom-build/", "debug/meu-app/run-build-script"] {
        assert!(targets.iter().any(|t| t.starts_with(wanted)), "falta {wanted} em {targets:?}");
    }
    // Fora do repositório do aylis: o cargo de fora usa a mesma pasta.
    let config = std::fs::read_to_string(dir.join(".cargo").join("config.toml")).unwrap();
    assert!(config.contains(&format!("target-dir = '{}'", target.display())), "{config}");
    // E com o mesmo perfil das dependências: o cargo de fora não compila outra cópia delas.
    assert!(config.contains("[profile.dev.package.\"*\"]\ndebug = false"), "{config}");

    let r = aylis_tools::cleanup::clean_project(&target, &dir);
    assert_eq!(r.units, project.units.len());
    assert!(Index::load(&target).projects.is_empty());
    let left = std::fs::read_dir(target.join("debug").join(".fingerprint")).unwrap().flatten().filter(|e| e.file_name().to_string_lossy().starts_with("meu-app")).count();
    let names: Vec<String> = std::fs::read_dir(target.join("debug").join(".fingerprint")).unwrap().flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
    assert_eq!(left, 0, "nada do projeto ficou: {names:?} {:?}", project.units);
    let _ = std::fs::remove_dir_all(&root);
}
