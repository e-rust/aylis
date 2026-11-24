//! O runtime do Visual C++ dentro do `.exe` (`publish.windows.static_crt`) com o cargo de
//! verdade: um projeto pequeno compilado pelo `process::cargo`, o `+crt-static` no
//! `.cargo/config.toml` do projeto (nunca no global) e, no Windows, o `.exe` sem pedir o
//! `VCRUNTIME140.dll` (lido nos imports do PE, como o `dumpbin /dependents`).

use std::path::Path;

use aylis_tools::process::{Control, cargo};

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// As DLLs que um executável PE importa (a tabela de importação), em minúsculas.
fn imports(exe: &Path) -> Vec<String> {
    let b = std::fs::read(exe).unwrap();
    let u16_at = |o: usize| u16::from_le_bytes([b[o], b[o + 1]]) as usize;
    let u32_at = |o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as usize;
    let pe = u32_at(0x3c);
    assert_eq!(&b[pe..pe + 4], b"PE\0\0");
    let sections = u16_at(pe + 6);
    let optional = pe + 24;
    let optional_size = u16_at(pe + 20);
    // O diretório de importação: o segundo do PE32+ (o `magic` 0x20b), a 112 + 8.
    assert_eq!(u16_at(optional), 0x20b, "PE32+");
    let import_rva = u32_at(optional + 112 + 8);
    let table = optional + optional_size;
    let offset = |rva: usize| -> usize {
        (0..sections)
            .map(|i| table + i * 40)
            .find_map(|s| {
                let (size, va, raw) = (u32_at(s + 8).max(u32_at(s + 16)), u32_at(s + 12), u32_at(s + 20));
                (rva >= va && rva < va + size).then(|| rva - va + raw)
            })
            .expect("o RVA está numa seção")
    };
    let mut names = Vec::new();
    let mut at = offset(import_rva);
    while u32_at(at + 12) != 0 {
        let start = offset(u32_at(at + 12));
        let end = b[start..].iter().position(|&c| c == 0).unwrap() + start;
        names.push(String::from_utf8_lossy(&b[start..end]).to_ascii_lowercase());
        at += 20;
    }
    names
}

#[test]
fn the_static_runtime_goes_in_the_project_config_and_out_of_the_exe() {
    let root = std::env::temp_dir().join(format!("aylis_static_crt_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let (dir, target) = (root.join("App"), root.join("target"));
    let project = |on: bool| format!(r#"{{"aylis_project": 1, "name": "App", "publish": {{"windows": {{"static_crt": {on}}}}}}}"#);
    write(&dir.join("App.aylisproj"), &project(true));
    let code = dir.join("rust");
    write(&code.join("Cargo.toml"), "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n");
    write(&code.join("src/main.rs"), "fn main() { println!(\"{}\", std::env::args().count()); }\n");
    assert!(aylis_tools::process::project_static_crt(&code), "o projeto é achado da pasta do código");

    let envs = [("CARGO_TARGET_DIR", target.as_path())];
    let mut sink = |_| {};
    let built = cargo("cargo", &["build"], &code, &envs, &Control::new(), &mut sink).unwrap();
    let config = std::fs::read_to_string(code.join(".cargo").join("config.toml")).unwrap();
    assert!(config.contains("[target.'cfg(all(windows, target_env = \"msvc\"))']\nrustflags = [\"-C\", \"target-feature=+crt-static\"]"), "{config}");
    let exe = built.artifacts.iter().find_map(|a| a.executable.clone()).expect("o executável");
    if cfg!(all(windows, target_env = "msvc")) {
        let dlls = imports(&exe);
        assert!(dlls.iter().any(|d| d == "kernel32.dll"), "{dlls:?}");
        assert!(!dlls.iter().any(|d| d.starts_with("vcruntime") || d.starts_with("api-ms-win-crt")), "o runtime ficou de fora: {dlls:?}");
    }

    // Desligada: as flags saem do arquivo, e o `.exe` volta a pedir o runtime.
    write(&dir.join("App.aylisproj"), &project(false));
    let built = cargo("cargo", &["build"], &code, &envs, &Control::new(), &mut sink).unwrap();
    let config = std::fs::read_to_string(code.join(".cargo").join("config.toml")).unwrap();
    assert!(!config.contains("crt-static"), "{config}");
    if cfg!(all(windows, target_env = "msvc")) {
        let exe = built.artifacts.iter().find_map(|a| a.executable.clone()).unwrap();
        assert!(imports(&exe).iter().any(|d| d.starts_with("vcruntime")), "{:?}", imports(&exe));
    }

    // Com um `config.toml` do autor do projeto (o Aylis não mexe nele), as flags vão na linha
    // de comando.
    write(&dir.join("App.aylisproj"), &project(true));
    write(&code.join(".cargo").join("config.toml"), "# do autor\n[build]\nincremental = false\n");
    let built = cargo("cargo", &["build"], &code, &envs, &Control::new(), &mut sink).unwrap();
    assert_eq!(std::fs::read_to_string(code.join(".cargo").join("config.toml")).unwrap(), "# do autor\n[build]\nincremental = false\n");
    if cfg!(all(windows, target_env = "msvc")) {
        let exe = built.artifacts.iter().find_map(|a| a.executable.clone()).unwrap();
        assert!(!imports(&exe).iter().any(|d| d.starts_with("vcruntime")), "{:?}", imports(&exe));
    }
    let _ = std::fs::remove_dir_all(&root);
}
