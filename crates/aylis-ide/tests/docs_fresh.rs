//! A documentação da Landing (`examples/Landing/assets/docs`) é a que o gerador escreve agora
//! (decidido em 14/11): mudar uma propriedade, um comentário de campo ou um guia de `docs/` sem
//! rodar o `aylis-tool docs` deixa a página velha, e este teste avisa. E toda propriedade das
//! páginas dos componentes tem o "O que faz" (o comentário `///` do campo).

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().unwrap();
    PathBuf::from(root.display().to_string().trim_start_matches(r"\\?\"))
}

/// Os arquivos de `dir` (relativos a ele), em ordem.
fn files(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "md" || e == "json") {
                out.push(path.strip_prefix(dir).unwrap().to_string_lossy().replace('\\', "/"));
            }
        }
    }
    out.sort();
    out
}

/// Gera em `aylis_docs_fresh_<pid>_<tag>`: os testes rodam em paralelo, cada um na sua pasta.
fn generate(tag: &str) -> PathBuf {
    let out = std::env::temp_dir().join(format!("aylis_docs_fresh_{}_{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    aylis_ide::reference::write(&out, &root().join("docs")).unwrap();
    out
}

#[test]
fn the_landing_docs_are_what_the_generator_writes_now() {
    let out = generate("landing");
    let landing = root().join("examples/Landing/assets/docs");
    let mut stale = Vec::new();
    for file in files(&out) {
        let fresh = std::fs::read_to_string(out.join(&file)).unwrap().replace("\r\n", "\n");
        match std::fs::read_to_string(landing.join(&file)) {
            Ok(old) if old.replace("\r\n", "\n") == fresh => {}
            Ok(_) => stale.push(format!("{file}: diferente")),
            Err(_) => stale.push(format!("{file}: falta")),
        }
    }
    // As páginas de componentes que o gerador não escreve mais (a classe saiu).
    let generated: Vec<String> = files(&out);
    for file in files(&landing).into_iter().filter(|f| f.starts_with("componentes/")) {
        if !generated.contains(&file) {
            stale.push(format!("{file}: sobrou"));
        }
    }
    let _ = std::fs::remove_dir_all(&out);
    assert!(stale.is_empty(), "a documentação da Landing está velha (rode `aylis-tool docs`):\n{}", stale.join("\n"));
}

#[test]
fn every_property_says_what_it_does() {
    let out = generate("properties");
    let mut missing = Vec::new();
    for file in files(&out).into_iter().filter(|f| f.starts_with("componentes/") && f.ends_with(".md")) {
        let text = std::fs::read_to_string(out.join(&file)).unwrap();
        // Só as tabelas de propriedades (`| Propriedade | ... | O que faz |`); a de eventos tem a
        // coluna "Pergunta" vazia de propósito.
        let mut in_properties = false;
        for line in text.lines() {
            if line.starts_with("| Propriedade") {
                in_properties = true;
            } else if !line.starts_with('|') {
                in_properties = false;
            }
            // As linhas da tabela de propriedades: `| `nome` | rótulo | tipo | o que faz |`.
            if in_properties && line.starts_with("| `") && line.trim_end().ends_with("|  |") {
                let name = line.split('`').nth(1).unwrap_or_default();
                missing.push(format!("{file}: {name}"));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&out);
    missing.sort();
    missing.dedup();
    assert!(missing.is_empty(), "{} propriedades sem o \"O que faz\" (o comentário `///` do campo):\n{}", missing.len(), missing.join("\n"));
}
