//! # aylis-migrate
//!
//! Converte um projeto do aylis de um formato para o seguinte. O aylis e a IDE só abrem o
//! próprio formato (sem nomes antigos nem propriedades obsoletas arrastados de versão em
//! versão); a lógica de cada mudança incompatível fica aqui, fora deles.
//!
//! ```text
//! aylis-migrate Vendas/Vendas.aylisproj            converte (com uma cópia antes)
//! aylis-migrate Vendas/Vendas.aylisproj --check    só diz o que mudaria
//! aylis-migrate Vendas/Vendas.aylisproj --apply-code   também troca no código os nomes exatos
//! ```
//!
//! Cada salto ([`STEPS`]) leva do formato `from` ao `from + 1`, e eles rodam em cadeia (um
//! projeto do formato 1 vai ao 3 passando pelo 2). Antes de mexer, o projeto inteiro vai
//! para um ZIP ao lado da pasta. Os forms e o `.aylisproj` são convertidos; o código gerado
//! é refeito; no código do usuário, o migrador lista arquivo e linha de cada nome que mudou
//! e só troca (os casos exatos) com `--apply-code`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value;

/// Um salto de formato: do `from` para o `from + 1`.
pub struct Step {
    pub from: u32,
    /// O que mudou, para o relatório.
    pub describe: &'static str,
    /// Converte o `.aylisproj`.
    pub project: fn(&mut Value),
    /// Converte um form (o documento inteiro, com `aylis` e `root`).
    pub form: fn(&mut Value),
    /// Nomes que mudaram (propriedade, evento, classe): (antigo, novo), procurados no código
    /// do usuário.
    pub renames: &'static [(&'static str, &'static str)],
    /// Converte o código do usuário (a pasta do projeto, o `.aylisproj` já convertido);
    /// devolve as linhas do relatório.
    pub code: fn(&Path, &Value) -> Result<Vec<String>, String>,
}

/// Os saltos, em ordem. O formato 1 é o atual: ainda não há nenhum (os projetos de antes dos
/// forms com instância foram convertidos e o formato voltou a 1, em 02/11/2026).
pub const STEPS: &[Step] = &[];

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = args.iter().find(|a| !a.starts_with("--")).map(PathBuf::from) else {
        eprintln!("uso: aylis-migrate <projeto.aylisproj> [--check] [--apply-code]");
        return ExitCode::from(2);
    };
    let check = args.iter().any(|a| a == "--check");
    let apply_code = args.iter().any(|a| a == "--apply-code");
    match migrate(&path, check, apply_code) {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("aylis-migrate: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Converte o projeto em `path` até o formato atual. Devolve o relatório.
pub fn migrate(path: &Path, check: bool, apply_code: bool) -> Result<String, String> {
    let target = aylis_project::FORMAT_VERSION;
    let mut project = read_json(path)?;
    let from = project.get("aylis_project").and_then(Value::as_u64).ok_or("o .aylisproj não tem \"aylis_project\" (a versão do formato)")? as u32;
    if from > target {
        return Err(format!("o projeto é do formato {from}, mais novo que este migrador ({target}): atualize o aylis"));
    }
    let dir = path.parent().ok_or("caminho sem pasta")?.to_path_buf();
    let forms = form_files(&project, &dir);
    let steps: Vec<&Step> = STEPS.iter().filter(|s| s.from >= from && s.from < target).collect();
    if from == target && forms.iter().all(|f| form_version(f) == Some(aylis_json::FORMAT_VERSION as u64)) {
        return Ok(format!("{}: já está no formato {target}; nada a fazer", path.display()));
    }
    let mut report = vec![format!("{}: formato {from} → {target}", path.display())];
    for step in &steps {
        report.push(format!("  {} → {}: {}", step.from, step.from + 1, step.describe));
    }
    let usages = code_usages(&dir, &steps);
    for (file, line, old, new) in &usages {
        report.push(format!("  {}:{line}: '{old}' agora é '{new}'", file.display()));
    }
    if check {
        report.push("(--check: nada foi alterado)".into());
        return Ok(report.join("\n"));
    }

    let backup = backup(&dir, from)?;
    report.push(format!("  cópia de antes: {}", backup.display()));
    for step in &steps {
        (step.project)(&mut project);
    }
    project["aylis_project"] = Value::from(target);
    write_json(path, &project)?;
    for form in &forms {
        let mut doc = read_json(form)?;
        let before = doc.clone();
        for step in &steps {
            (step.form)(&mut doc);
        }
        doc["aylis"] = Value::from(aylis_json::FORMAT_VERSION);
        // Só regrava o que mudou (o form intocado fica com a formatação de quem escreveu).
        if doc != before {
            write_json(form, &doc)?;
        }
    }
    if apply_code {
        let changed = apply_renames(&usages)?;
        report.push(format!("  código: {changed} troca(s) feita(s)"));
    }
    for step in &steps {
        report.extend((step.code)(&dir, &project)?);
    }
    // O código gerado (o acesso tipado, o registro) é refeito a partir dos forms novos.
    let converted = aylis_project::Project::open(path)?;
    aylis_project::codegen::generate(&converted)?;
    report.push("  código gerado refeito".into());
    Ok(report.join("\n"))
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", path.display()))
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

fn form_files(project: &Value, dir: &Path) -> Vec<PathBuf> {
    project["forms"].as_array().into_iter().flatten().filter_map(|f| f["file"].as_str()).map(|f| dir.join(f)).collect()
}

fn form_version(path: &Path) -> Option<u64> {
    read_json(path).ok()?.get("aylis").and_then(Value::as_u64)
}

/// O projeto inteiro num ZIP ao lado da pasta (`Vendas-formato-1.zip`), sem as pastas de
/// compilação.
fn backup(dir: &Path, from: u32) -> Result<PathBuf, String> {
    use std::io::Write;
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("projeto");
    let zip_path = dir.with_file_name(format!("{name}-formato-{from}.zip"));
    let file = std::fs::File::create(&zip_path).map_err(|e| format!("{}: {e}", zip_path.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for entry in walk(dir) {
        let relative = entry.strip_prefix(dir).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
        zip.start_file(relative, options).map_err(|e| e.to_string())?;
        zip.write_all(&std::fs::read(&entry).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(zip_path)
}

/// Os arquivos do projeto, sem as pastas de compilação e de dependências.
fn walk(dir: &Path) -> Vec<PathBuf> {
    const SKIP: [&str; 6] = ["target", "node_modules", "bin", "obj", "out", ".git"];
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if !SKIP.iter().any(|s| path.file_name().is_some_and(|n| n == *s)) {
                    stack.push(path);
                }
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Onde o código do usuário usa um nome que mudou: (arquivo, linha, antigo, novo).
fn code_usages(dir: &Path, steps: &[&Step]) -> Vec<(PathBuf, usize, &'static str, &'static str)> {
    const CODE: [&str; 9] = ["rs", "py", "ts", "cs", "java", "go", "cpp", "hpp", "pas"];
    let renames: Vec<(&str, &str)> = steps.iter().flat_map(|s| s.renames.iter().copied()).collect();
    if renames.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for file in walk(dir).into_iter().filter(|f| f.extension().and_then(|e| e.to_str()).is_some_and(|e| CODE.contains(&e))) {
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        for (n, line) in text.lines().enumerate() {
            for &(old, new) in &renames {
                if contains_word(line, old) {
                    out.push((file.clone(), n + 1, old, new));
                }
            }
        }
    }
    out
}

/// A palavra inteira (não um pedaço de outro nome).
fn contains_word(line: &str, word: &str) -> bool {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    line.match_indices(word).any(|(i, _)| {
        let before = line[..i].chars().next_back();
        let after = line[i + word.len()..].chars().next();
        !before.is_some_and(ident) && !after.is_some_and(ident)
    })
}

/// Troca os nomes exatos listados (só a palavra inteira). Devolve quantas trocas.
fn apply_renames(usages: &[(PathBuf, usize, &str, &str)]) -> Result<usize, String> {
    let mut count = 0;
    let mut files: Vec<&PathBuf> = usages.iter().map(|u| &u.0).collect();
    files.dedup();
    for file in files {
        let text = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
        let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
        for (_, n, old, new) in usages.iter().filter(|u| &u.0 == file) {
            if let Some(line) = lines.get_mut(n - 1) {
                *line = replace_word(line, old, new);
                count += 1;
            }
        }
        std::fs::write(file, lines.join("\n")).map_err(|e| e.to_string())?;
    }
    Ok(count)
}

fn replace_word(line: &str, old: &str, new: &str) -> String {
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out = String::new();
    let mut copied = 0;
    for (i, _) in line.match_indices(old) {
        let before = line[..i].chars().next_back();
        let after = line[i + old.len()..].chars().next();
        if !before.is_some_and(ident) && !after.is_some_and(ident) {
            out.push_str(&line[copied..i]);
            out.push_str(new);
            copied = i + old.len();
        }
    }
    out.push_str(&line[copied..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_whole_words_are_renamed() {
        assert!(contains_word("btn.caption := 'x'", "caption"));
        assert!(!contains_word("btn.caption_color", "caption"));
        assert_eq!(replace_word("caption + caption_color + caption", "caption", "text"), "text + caption_color + text");
    }

    #[test]
    fn a_project_in_the_current_format_has_nothing_to_do() {
        let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/ProcessamentoAssincrono/ProcessamentoAssincrono.aylisproj"));
        let report = migrate(&root, true, false).unwrap();
        assert!(report.contains("nada a fazer"), "{report}");
    }
}
