//! Nenhuma referência ao Delphi, ao FMX ou à VCL nas fontes e na documentação (decidido pelo
//! autor em 14/11: "Tirar qualquer referencia a Delphi/FMX/VCL de qualquer fonte ou
//! documentação"). Ficam de fora o histórico e os pendentes (as falas do autor, citadas como
//! foram ditas) e os poucos nomes que o código usa de verdade (abaixo).

use std::path::{Path, PathBuf};

/// Os trechos que podem ter a palavra: o apelido da linguagem nos blocos de código e a extensão
/// do arquivo na gramática do Pascal, e o filtro que tira essas comparações das páginas da
/// documentação (por arquivo, o pedaço da linha).
const ALLOWED: &[(&str, &str)] = &[
    ("crates/aylis-controls-base/src/syntax.rs", "\"delphi\""),
    ("crates/aylis-markdown/src/layout.rs", "\"delphi\""),
    ("crates/aylis-ide/resources/grammars/pascal.tmLanguage", "fmx"),
    ("crates/aylis-ide/src/reference.rs", "contains(\"FMX\")"),
    ("crates/aylis-ide/src/reference.rs", "Delphi/FMX"),
    ("crates/aylis-ide/src/reference.rs", "\"Delphi\""),
    ("crates/aylis-ide/src/reference.rs", "do FMX"),
    // O literal "Delphi" do filtro acima, que não é texto de tela.
    ("crates/aylis-ide/resources/locales/untranslated.txt", "Delphi"),
    ("crates/aylis/tests/no_delphi.rs", ""),
];

/// As pastas e os arquivos conferidos (relativos à raiz).
const ROOTS: &[&str] = &["crates", "clients", "examples", "resources", "docs", "README.md", "CLAUDE.md"];

/// Os registros com as falas do autor (citadas como foram ditas).
const RECORDS: &[&str] = &["docs/historico-decisoes.md", "docs/ajustes-pendentes.md"];

fn root() -> PathBuf {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().unwrap();
    PathBuf::from(root.display().to_string().trim_start_matches(r"\\?\"))
}

fn text_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| {
        matches!(e, "rs" | "md" | "toml" | "json" | "pas" | "lpr" | "py" | "ts" | "go" | "java" | "cs" | "cpp" | "hpp" | "h" | "tmLanguage" | "txt" | "tsv" | "html" | "js" | "css")
    })
}

/// A palavra solta (`Delphi`, `FMX`, `VCL`, em qualquer caixa), não no meio de outra.
fn mentions(line: &str) -> bool {
    let lower = line.to_lowercase();
    ["delphi", "fmx", "vcl"].iter().any(|word| {
        lower.match_indices(word).any(|(i, _)| {
            let before = lower[..i].chars().next_back();
            let after = lower[i + word.len()..].chars().next();
            !before.is_some_and(|c| c.is_alphanumeric()) && !after.is_some_and(|c| c.is_alphanumeric())
        })
    })
}

#[test]
fn nothing_mentions_delphi_fmx_or_vcl() {
    let root = root();
    let mut found = Vec::new();
    let mut stack: Vec<PathBuf> = ROOTS.iter().map(|r| root.join(r)).collect();
    while let Some(path) = stack.pop() {
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
            if matches!(name, "target" | "node_modules" | ".git" | "third_party") {
                continue;
            }
            stack.extend(std::fs::read_dir(&path).into_iter().flatten().flatten().map(|e| e.path()));
            continue;
        }
        if !text_file(&path) {
            continue;
        }
        let rel = path.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
        if RECORDS.contains(&rel.as_str()) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for (n, line) in text.lines().enumerate() {
            if mentions(line) && !ALLOWED.iter().any(|(file, piece)| *file == rel && line.contains(piece)) {
                found.push(format!("{rel}:{}: {}", n + 1, line.trim()));
            }
        }
    }
    assert!(found.is_empty(), "{} menções ao Delphi/FMX/VCL (decidido em 14/11 tirar todas):\n{}", found.len(), found.join("\n"));
}
