//! O que uma compilação mexeu nos arquivos do projeto (I06 do Tabernáculo: o build reescrevia o
//! `Cargo.lock`, o `Cargo.toml`, o `code-sync.json`, o gerado e a `.aylis/.gitignore`, e nada
//! dizia o quê nem por quê). Antes de gerar e compilar, [`before`] guarda como estavam os
//! arquivos que a geração e o cargo podem mexer; no fim, [`Before::report`] diz, nas mensagens,
//! cada um que mudou, com o motivo e as primeiras linhas trocadas.
//!
//! Os arquivos de texto pequenos são guardados inteiros (para dizer as linhas); as pastas
//! (`generated/`, `src/forms/`), pela marca de cada arquivo ([`FileStamp`]), sem ler o conteúdo.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aylis_project::{FileStamp, Project};

use crate::process::{Event, LineKind};

/// Até quanto um arquivo é guardado inteiro (o `Cargo.lock` de um app médio tem uns 300 KB).
const MAX_TEXT: u64 = 4 * 1024 * 1024;

/// Como um arquivo estava.
#[derive(Clone, Debug, PartialEq)]
enum Seen {
    Text(String),
    Stamp(FileStamp),
}

/// Os arquivos do projeto antes da compilação.
#[derive(Clone, Debug, Default)]
pub struct Before {
    files: BTreeMap<PathBuf, Seen>,
    /// As pastas olhadas arquivo por arquivo (um arquivo novo nelas também conta).
    dirs: Vec<PathBuf>,
    base: PathBuf,
}

/// Um arquivo que a compilação mudou.
#[derive(Clone, Debug, PartialEq)]
pub struct Change {
    /// Relativo à pasta do projeto, com `/`.
    pub file: String,
    /// "alterado (+3 −1)", "criado", "apagado".
    pub what: String,
    /// Por que o Aylis (ou o cargo) mexe nele.
    pub why: &'static str,
    /// As primeiras linhas que mudaram (`+ ...`, `- ...`), nos arquivos de texto.
    pub lines: Vec<String>,
}

/// O `Cargo.lock` que vale para o crate em `dir`: o dele ou o do workspace acima.
fn cargo_lock(dir: &Path) -> PathBuf {
    dir.ancestors().map(|d| d.join("Cargo.lock")).find(|p| p.is_file()).unwrap_or_else(|| dir.join("Cargo.lock"))
}

/// Os arquivos que a geração e a compilação mexem, e as pastas olhadas por inteiro.
fn watched(project: &Project) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let (dir, code) = (project.dir(), project.code_dir());
    let files = vec![
        project.path.clone(),
        dir.join(aylis_project::sync::FILE),
        dir.join(".aylis").join(".gitignore"),
        code.join("Cargo.toml"),
        cargo_lock(&code),
        code.join("build.rs"),
        code.join("src").join("lib.rs"),
        code.join("src").join("main.rs"),
    ];
    let dirs = vec![code.join("generated"), code.join("src").join("forms")];
    (files, dirs)
}

fn seen(path: &Path) -> Option<Seen> {
    let meta = std::fs::metadata(path).ok().filter(|m| m.is_file())?;
    if meta.len() <= MAX_TEXT {
        if let Ok(text) = std::fs::read_to_string(path) {
            return Some(Seen::Text(text));
        }
    }
    FileStamp::of(&meta).map(Seen::Stamp)
}

/// Os arquivos de uma pasta (e das de dentro), pela marca: a do `read_dir` (no Windows vem com a
/// listagem, sem abrir cada arquivo; o OneDrive deixa cada consulta lenta).
fn stamps_in(dir: &Path, out: &mut BTreeMap<PathBuf, Seen>) {
    let mut pending = vec![dir.to_path_buf()];
    while let Some(d) = pending.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let Ok(meta) = entry.metadata() else { continue };
            if meta.is_dir() {
                pending.push(entry.path());
            } else if let Some(stamp) = FileStamp::of(&meta) {
                out.insert(entry.path(), Seen::Stamp(stamp));
            }
        }
    }
}

/// Como estão agora os arquivos que a compilação pode mexer.
pub fn before(project: &Project) -> Before {
    let (files, dirs) = watched(project);
    let mut map = BTreeMap::new();
    for file in files {
        if let Some(s) = seen(&file) {
            map.insert(file, s);
        } else {
            // Ainda não existe: se nascer, aparece.
            map.insert(file, Seen::Stamp(FileStamp { nanos: 0, len: u64::MAX }));
        }
    }
    for d in &dirs {
        stamps_in(d, &mut map);
    }
    Before { files: map, dirs, base: project.dir() }
}

/// Por que cada arquivo muda numa compilação.
fn why(file: &str) -> &'static str {
    let name = file.rsplit('/').next().unwrap_or(file);
    match name {
        "Cargo.lock" => "o cargo resolveu as dependências de novo (mudou o Cargo.toml, as features do Aylis ou a versão dele)",
        "Cargo.toml" => "o Aylis pôs em dia as features com as classes dos forms, o renderizador, de onde vem o Aylis ou o perfil de desenvolvimento",
        aylis_project::sync::FILE => "o estado dos handlers entre as linguagens (um handler novo, alterado ou renomeado)",
        ".gitignore" => "a pasta .aylis é desta máquina: a regra que a deixa fora do git",
        "build.rs" | "lib.rs" | "main.rs" => "faltava no projeto (feito por uma versão anterior) e foi criado",
        _ if file.ends_with(".aylisproj") => "o projeto foi salvo com o que a IDE mudou nele",
        _ if file.contains("/generated/") => "o código gerado dos forms (somente leitura), refeito pela geração",
        _ => "os handlers novos dos forms, acrescentados no fim do arquivo",
    }
}

/// As linhas que mudaram, pelo começo e pelo fim iguais (bom para o `Cargo.toml` e o lock, que
/// mudam num trecho).
fn diff_lines(old: &str, new: &str) -> (usize, usize, Vec<String>) {
    let (a, b): (Vec<&str>, Vec<&str>) = (old.lines().collect(), new.lines().collect());
    let head = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let tail = a[head..].iter().rev().zip(b[head..].iter().rev()).take_while(|(x, y)| x == y).count();
    let (removed, added) = (&a[head..a.len() - tail], &b[head..b.len() - tail]);
    // Os iguais nos dois lados (uma linha que só mudou de lugar) não contam.
    let removed: Vec<&str> = removed.iter().copied().filter(|l| !added.contains(l)).collect();
    let added: Vec<&str> = added.iter().copied().filter(|l| !a[head..a.len() - tail].contains(l)).collect();
    let sample = removed.iter().map(|l| format!("- {}", l.trim_end())).chain(added.iter().map(|l| format!("+ {}", l.trim_end()))).take(4).collect();
    (added.len(), removed.len(), sample)
}

impl Before {
    /// O que mudou desde [`before`].
    pub fn changes(&self) -> Vec<Change> {
        let mut now = BTreeMap::new();
        for file in self.files.keys() {
            if let Some(s) = seen(file) {
                now.insert(file.clone(), s);
            }
        }
        for d in &self.dirs {
            stamps_in(d, &mut now);
        }
        let rel = |p: &Path| p.strip_prefix(&self.base).unwrap_or(p).to_string_lossy().replace('\\', "/");
        let mut out = Vec::new();
        for (file, old) in &self.files {
            let created = matches!(old, Seen::Stamp(s) if s.len == u64::MAX);
            let change = match (created, now.get(file)) {
                (true, None) => continue,
                (true, Some(_)) => ("criado".to_string(), Vec::new()),
                (false, None) => ("apagado".to_string(), Vec::new()),
                (false, Some(new)) if new == old => continue,
                (false, Some(Seen::Text(new))) => match old {
                    Seen::Text(old) => {
                        let (added, removed, lines) = diff_lines(old, new);
                        // Só a hora (gravado igual) ou só o fim de linha: não é mudança.
                        if added == 0 && removed == 0 && old.replace("\r\n", "\n") == new.replace("\r\n", "\n") {
                            continue;
                        }
                        (format!("alterado (+{added} −{removed})"), lines)
                    }
                    Seen::Stamp(_) => ("alterado".to_string(), Vec::new()),
                },
                (false, Some(_)) => ("alterado".to_string(), Vec::new()),
            };
            let file = rel(file);
            out.push(Change { why: why(&file), file, what: change.0, lines: change.1 });
        }
        for file in now.keys().filter(|f| !self.files.contains_key(*f)) {
            let file = rel(file);
            out.push(Change { why: why(&file), file, what: "criado".into(), lines: Vec::new() });
        }
        out
    }

    /// Diz nas mensagens o que mudou (nada mudou: não diz nada).
    pub fn report(&self, sink: &mut dyn FnMut(Event)) {
        let changes = self.changes();
        for line in lines(&changes) {
            sink(Event::Line(line, LineKind::Tool));
        }
    }
}

/// As linhas das mensagens: o título e, por arquivo, o que mudou, o motivo e as linhas. Muitos
/// arquivos do gerado viram uma linha só.
pub fn lines(changes: &[Change]) -> Vec<String> {
    if changes.is_empty() {
        return Vec::new();
    }
    let mut out = vec![format!("ℹ A compilação mudou {} arquivo(s) do projeto:", changes.len())];
    let (generated, others): (Vec<&Change>, Vec<&Change>) = changes.iter().partition(|c| c.file.contains("/generated/"));
    for c in others {
        out.push(format!("  {} — {}: {}", c.file, c.what, c.why));
        out.extend(c.lines.iter().map(|l| format!("      {l}")));
    }
    match generated.len() {
        0 => {}
        1..=3 => out.extend(generated.iter().map(|c| format!("  {} — {}: {}", c.file, c.what, c.why))),
        n => out.push(format!("  {n} arquivos em generated/ — {}", why("x/generated/x"))),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O `Cargo.toml` alterado diz as linhas; o lock que nasce, que nasceu; o resto, sem mudança,
    /// não aparece.
    #[test]
    fn the_changed_files_are_told_with_the_reason() {
        let dir = std::env::temp_dir().join(format!("aylis_touched_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let project = aylis_project::create(&dir, "Loja", aylis_project::Language::Rust).unwrap();
        aylis_project::codegen::generate(&project).unwrap();
        let code = project.code_dir();
        let lock = code.join("Cargo.lock");
        let _ = std::fs::remove_file(&lock);
        let before = before(&project);
        assert!(before.changes().is_empty(), "{:?}", before.changes());
        let toml = std::fs::read_to_string(code.join("Cargo.toml")).unwrap();
        std::fs::write(code.join("Cargo.toml"), format!("{}\n[package.metadata.loja]\nextra = 1\n", toml.trim_end())).unwrap();
        std::fs::write(&lock, "version = 4\n").unwrap();
        let changes = before.changes();
        let files: Vec<&str> = changes.iter().map(|c| c.file.as_str()).collect();
        assert_eq!(files.len(), 2, "{files:?}");
        let toml = changes.iter().find(|c| c.file.ends_with("Cargo.toml")).expect("o Cargo.toml");
        assert_eq!(toml.what, "alterado (+2 −0)");
        assert!(toml.lines.contains(&"+ extra = 1".to_string()), "{:?}", toml.lines);
        assert!(toml.why.contains("features"));
        let lock = changes.iter().find(|c| c.file.ends_with("Cargo.lock")).expect("o Cargo.lock");
        assert_eq!(lock.what, "criado");
        let text = lines(&changes).join("\n");
        assert!(text.starts_with("ℹ A compilação mudou 2 arquivo(s)"), "{text}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_diff_counts_only_what_changed() {
        let (added, removed, lines) = diff_lines("a\nb\nc\nd\n", "a\nx\nc\nd\ny\n");
        assert_eq!((added, removed), (2, 1));
        assert_eq!(lines, ["- b", "+ x", "+ y"]);
    }
}
