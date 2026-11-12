//! O caminho do aylis no `Cargo.toml`: relativo (dentro do repositório e fora dele, no mesmo
//! disco), para o projeto ir para o git sem o caminho desta máquina.

use std::path::Path;

use aylis_project::codegen;

fn repo() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
}

#[test]
fn inside_the_aylis_the_path_is_relative() {
    assert_eq!(codegen::aylis_path_for(&repo().join("examples/Paint/rust")), "../../..");
    // A casca ainda nem existe: conta do mesmo jeito.
    assert_eq!(codegen::aylis_path_for(&repo().join("examples/Paint/.aylis/shell")), "../../../..");
    let name = repo().canonicalize().unwrap().file_name().unwrap().to_string_lossy().to_string();
    // Nada do caminho desta máquina, nem de um projeto bem longe no mesmo disco.
    let far = std::env::temp_dir().join("um-projeto");
    let path = codegen::aylis_path_for(&far);
    assert!(path.starts_with("..") && path.ends_with(&name), "{path}");
}

#[test]
fn an_absolute_path_inside_the_aylis_becomes_relative() {
    let root = codegen::aylis_root();
    let dir = repo().join("examples/Paint/rust");
    let old = format!("[dependencies]\naylis = {{ path = \"{root}/crates/aylis\" }}\n[build-dependencies]\naylis-build = {{ path = \"{root}/crates/aylis-build\" }}\n[features]\ndev = []\n");
    let (new, notes) = codegen::upgrade_cargo(&old, "../../..", &dir).expect("muda");
    assert_eq!(notes, ["caminho do aylis", "caminho do aylis-build"]);
    assert!(new.contains("aylis = { path = \"../../../crates/aylis\" }"), "{new}");
    assert_eq!(codegen::upgrade_cargo(&new, "../../..", &dir), None, "o relativo já serve");
}

/// Os exemplos commitados funcionam em qualquer clone: nenhum caminho absoluto.
#[test]
fn the_examples_have_no_absolute_paths() {
    for entry in std::fs::read_dir(repo().join("examples")).unwrap().flatten() {
        let cargo = entry.path().join("rust/Cargo.toml");
        let Ok(text) = std::fs::read_to_string(&cargo) else { continue };
        for line in text.lines().filter(|l| l.contains("path = \"")) {
            let path = line.split("path = \"").nth(1).and_then(|r| r.split('"').next()).unwrap_or_default();
            assert!(!Path::new(path).is_absolute() && !path.contains(':'), "{}: {line}", cargo.display());
        }
    }
}

#[test]
fn forms_outside_the_project_are_refused() {
    assert!(aylis_project::inside_project("forms/clientes.json"));
    for bad in ["../fora.json", "forms/../../x.json", "C:/Windows/x.json", "/etc/x.json", ""] {
        assert!(!aylis_project::inside_project(bad), "{bad}");
    }
    let dir = std::env::temp_dir().join(format!("aylis_proj_outside_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut project = aylis_project::create(&dir, "Fora", aylis_project::Language::Rust).unwrap();
    project.forms[0].file = "../../fora.json".into();
    project.save().unwrap();
    let err = aylis_project::Project::open(&project.path).unwrap_err();
    assert!(err.contains("fora do projeto"), "{err}");
    let _ = std::fs::remove_dir_all(dir);
}

/// Um projeto do usuário (com o `.aylisproj`) aponta para `<projeto>/.aylis/sdk`, sempre o mesmo
/// em qualquer máquina; o aylis mantém ali a junção até a instalação daqui, fora do git.
#[test]
fn a_user_project_points_to_its_sdk_link() {
    let base = std::env::temp_dir().join(format!("aylis_sdk_link_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("rust")).unwrap();
    std::fs::write(base.join("App.aylisproj"), "{}").unwrap();
    assert_eq!(codegen::aylis_path_for(&base.join("rust")), "../.aylis/sdk");
    let link = base.join(".aylis/sdk");
    let root = std::path::PathBuf::from(codegen::aylis_root());
    assert_eq!(link.canonicalize().unwrap(), root.canonicalize().unwrap(), "a junção chega ao aylis");
    assert!(link.join("crates/aylis/Cargo.toml").is_file());
    let ignore = std::fs::read_to_string(base.join(".aylis/.gitignore")).unwrap();
    // Tudo na `.aylis` é desta máquina: uma regra só, que não muda quando aparece outro arquivo.
    assert!(ignore.lines().any(|l| l == "*"), "{ignore}");
    // De novo: nada muda (nem duplica a regra), e a casca em `.aylis/shell` também chega lá.
    assert_eq!(codegen::aylis_path_for(&base.join("rust")), "../.aylis/sdk");
    assert_eq!(codegen::aylis_path_for(&base.join(".aylis/shell")), "../sdk");
    assert_eq!(std::fs::read_to_string(base.join(".aylis/.gitignore")).unwrap(), ignore);
    // Remover a pasta do projeto não pode apagar o aylis (a junção sai, o destino fica).
    std::fs::remove_dir_all(&base).unwrap();
    assert!(root.join("crates/aylis/Cargo.toml").is_file());
}
