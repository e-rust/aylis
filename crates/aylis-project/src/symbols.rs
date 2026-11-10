//! Os fontes do projeto (a lista das units, Ctrl+F12) e as declarações de um fonte aberto (em
//! que função o cursor está): as linhas com as palavras de declaração de cada linguagem
//! (`procedure`, `fn`, `def`, `class`, `func`...). Procurar um nome no projeto (ir para a
//! declaração, referências, renomear) é do servidor de linguagem, que é obrigatório.

use std::path::{Path, PathBuf};

use crate::{Language, Project};

/// Um nome declarado num fonte.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    /// `procedure`, `function`, `class`, `type`, `fn`...
    pub kind: &'static str,
    /// Linha a partir de 1.
    pub line: usize,
}

/// As pastas que não são fonte (compilação, dependências).
const SKIP: [&str; 12] = ["target", "node_modules", "bin", "obj", "build", "dist", "out", ".aylis", "__pycache__", ".git", ".vs", ".idea"];

/// As extensões dos fontes de cada linguagem.
pub fn extensions(language: Language) -> &'static [&'static str] {
    match language {
        Language::Rust => &["rs"],
        Language::Python => &["py"],
        Language::TypeScript => &["ts", "tsx", "js", "mjs"],
        Language::CSharp => &["cs"],
        Language::Java => &["java"],
        Language::Go => &["go"],
        Language::Cpp => &["cpp", "cc", "cxx", "c", "h", "hpp", "hh"],
        Language::Pascal => &["pas", "pp", "lpr", "dpr", "inc"],
    }
}

/// Os fontes do projeto na linguagem ativa (a pasta dela, sem as de compilação), em ordem.
pub fn sources(project: &Project) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(&project.code_dir(), extensions(project.language), &mut out);
    out.sort();
    out
}

fn collect(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            if !SKIP.contains(&name.as_str()) && !name.starts_with('.') {
                collect(&path, exts, out);
            }
        } else if path.extension().and_then(|e| e.to_str()).is_some_and(|e| exts.iter().any(|x| x.eq_ignore_ascii_case(e))) {
            out.push(path);
        }
    }
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// O identificador no começo de `s` (sem os espaços antes) e o resto.
fn ident(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    let end = s.char_indices().find(|&(_, c)| !is_ident(c)).map_or(s.len(), |(i, _)| i);
    (end > 0 && !s.starts_with(|c: char| c.is_ascii_digit())).then(|| (&s[..end], &s[end..]))
}

/// Tira as palavras do começo que estão em `words` (modificadores: `pub`, `export`...).
fn skip_words<'a>(mut s: &'a str, words: &[&str], ignore_case: bool) -> &'a str {
    loop {
        let Some((w, rest)) = ident(s) else { return s.trim_start() };
        let hit = words.iter().any(|x| if ignore_case { x.eq_ignore_ascii_case(w) } else { *x == w });
        if !hit {
            return s.trim_start();
        }
        // `pub(crate)`
        let rest = rest.strip_prefix(|c| c == '(').map_or(rest, |r| r.split_once(')').map_or(r, |(_, after)| after));
        s = rest;
    }
}

/// As declarações de um fonte.
pub fn declarations(language: Language, text: &str) -> Vec<Declaration> {
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = i + 1;
        let trimmed = raw.trim_start();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') && language != Language::Cpp {
            continue;
        }
        let mut push = |name: &str, kind: &'static str| out.push(Declaration { name: name.to_string(), kind, line });
        match language {
            Language::Pascal => pascal(trimmed, &mut push),
            Language::Rust => {
                let s = skip_words(trimmed, &["pub", "async", "unsafe", "extern", "default"], false);
                let s = s.strip_prefix("\"C\"").map_or(s, str::trim_start);
                let s = s.strip_prefix("const fn").map(|r| format!("fn{r}"));
                let s = s.as_deref().unwrap_or(skip_words(trimmed, &["pub", "async", "unsafe", "extern", "default"], false));
                if let Some(rest) = s.strip_prefix("macro_rules!") {
                    if let Some((n, _)) = ident(rest) {
                        push(n, "macro");
                    }
                } else if let Some((kw, rest)) = ident(s) {
                    if let Some(kind) = ["fn", "struct", "enum", "trait", "type", "mod", "const", "static", "union"].iter().find(|k| **k == kw) {
                        if let Some((n, _)) = ident(rest) {
                            push(n, kind);
                        }
                    }
                }
            }
            Language::Python => {
                let s = skip_words(trimmed, &["async"], false);
                if let Some((kw, rest)) = ident(s) {
                    if kw == "def" || kw == "class" {
                        if let Some((n, _)) = ident(rest) {
                            push(n, if kw == "def" { "def" } else { "class" });
                        }
                    } else if raw.starts_with(kw) && rest.trim_start().starts_with('=') && !rest.trim_start().starts_with("==") {
                        push(kw, "variable");
                    }
                }
            }
            Language::TypeScript => {
                let s = skip_words(trimmed, &["export", "default", "async", "declare", "abstract", "static", "public", "private", "protected", "readonly", "get", "set"], false);
                let Some((kw, rest)) = ident(s) else { continue };
                let kinds = ["function", "class", "interface", "type", "enum", "const", "let", "var", "namespace"];
                if let Some(kind) = kinds.iter().find(|k| **k == kw) {
                    let rest = rest.trim_start().trim_start_matches('*');
                    if let Some((n, _)) = ident(rest) {
                        push(n, kind);
                    }
                } else if raw.starts_with(char::is_whitespace) && rest.trim_start().starts_with('(') && !CONTROL.contains(&kw) && !raw.trim_end().ends_with(';') {
                    push(kw, "method");
                }
            }
            Language::CSharp | Language::Java => {
                let modifiers = [
                    "public", "private", "protected", "internal", "static", "final", "abstract", "sealed", "override", "virtual", "async", "partial", "readonly", "synchronized", "native", "extern", "unsafe", "new", "default",
                ];
                let s = skip_words(trimmed, &modifiers, false);
                let Some((kw, rest)) = ident(s) else { continue };
                if ["class", "interface", "struct", "enum", "record"].contains(&kw) {
                    if let Some((n, _)) = ident(rest) {
                        push(n, "class");
                    }
                } else if !CONTROL.contains(&kw) && !trimmed.trim_end().ends_with(';') && !trimmed.starts_with('@') {
                    if let Some(n) = name_before_paren(s) {
                        if n != kw || s.trim_start().starts_with(&format!("{n}(")) {
                            push(n, "method");
                        }
                    }
                }
            }
            Language::Go => {
                let Some((kw, rest)) = ident(trimmed) else { continue };
                match kw {
                    "func" => {
                        let rest = rest.trim_start();
                        // O receptor: func (f FrmX) nome(...)
                        let rest = if rest.starts_with('(') { rest.split_once(')').map_or(rest, |(_, r)| r) } else { rest };
                        if let Some((n, _)) = ident(rest) {
                            push(n, "func");
                        }
                    }
                    "type" | "var" | "const" => {
                        if let Some((n, _)) = ident(rest) {
                            push(n, if kw == "type" { "type" } else { "variable" });
                        }
                    }
                    _ => {}
                }
            }
            Language::Cpp => {
                if let Some(rest) = trimmed.strip_prefix("#define") {
                    if let Some((n, _)) = ident(rest) {
                        push(n, "macro");
                    }
                    continue;
                }
                let s = skip_words(trimmed, &["static", "inline", "constexpr", "virtual", "explicit", "extern", "template", "typedef"], false);
                let Some((kw, rest)) = ident(s) else { continue };
                if ["class", "struct", "enum", "namespace", "union"].contains(&kw) {
                    let rest = skip_words(rest, &["class", "struct"], false);
                    if let Some((n, after)) = ident(rest) {
                        if !after.trim_start().starts_with(';') {
                            push(n, "class");
                        }
                    }
                } else if !CONTROL.contains(&kw) && !trimmed.trim_end().ends_with(';') && !trimmed.starts_with('#') {
                    if let Some(n) = name_before_paren(s) {
                        push(n, "function");
                    }
                }
            }
        }
    }
    out
}

/// As palavras que abrem um comando (não uma declaração) antes de um parêntese.
const CONTROL: [&str; 16] = ["if", "for", "while", "switch", "catch", "return", "new", "else", "throw", "using", "lock", "foreach", "do", "await", "yield", "sizeof"];

/// O nome logo antes do primeiro `(` (`Classe::nome(` dá `nome`), se a linha tem a cara de
/// uma declaração: um tipo antes do nome (`void nome(`, `int Classe::nome(`).
fn name_before_paren(s: &str) -> Option<&str> {
    let paren = s.find('(')?;
    let head = s[..paren].trim_end();
    if head.contains('=') || head.contains('.') && !head.contains("::") {
        return None;
    }
    let start = head.rfind(|c: char| !is_ident(c)).map_or(0, |i| i + 1);
    let name = &head[start..];
    let before = head[..start].trim_end();
    (!name.is_empty() && !before.is_empty() && !name.starts_with(|c: char| c.is_ascii_digit())).then_some(name)
}

/// Pascal: `procedure`, `function`, `constructor`, `destructor` (com `TClasse.Nome`), os
/// tipos (`TNome = class`...) e a `unit`/`program`.
fn pascal(s: &str, push: &mut impl FnMut(&str, &'static str)) {
    let s = skip_words(s, &["class"], true);
    let Some((kw, rest)) = ident(s) else { return };
    let lower = kw.to_ascii_lowercase();
    match lower.as_str() {
        "procedure" | "function" | "constructor" | "destructor" | "operator" => {
            if let Some((first, after)) = ident(rest) {
                // TFrm.tecla_click: o nome é o do método.
                let name = after.strip_prefix('.').and_then(ident).map_or(first, |(m, _)| m);
                push(name, if lower == "function" { "function" } else { "procedure" });
            }
        }
        "unit" | "program" | "library" => {
            if let Some((n, _)) = ident(rest) {
                push(n, "unit");
            }
        }
        _ => {
            // TNome = class / record / interface / object / (enumeração) / procedure of object
            let after = rest.trim_start();
            if let Some(value) = after.strip_prefix('=') {
                let value = value.trim_start();
                let word = ident(value).map(|(w, _)| w.to_ascii_lowercase()).unwrap_or_default();
                if value.starts_with('(') || ["class", "record", "interface", "object", "packed", "set", "array", "type"].contains(&word.as_str()) {
                    push(kw, "type");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(language: Language, text: &str) -> Vec<(String, usize)> {
        declarations(language, text).into_iter().map(|d| (d.name, d.line)).collect()
    }

    #[test]
    fn pascal_finds_procedures_methods_and_types() {
        let text = "unit calc;\ntype\n  TCalculadora = class\n    procedure Usar(V: Double);\n  end;\nprocedure TCalculadora.Usar(V: Double);\nbegin\nend;\nfunction Formatar(X: Double): string;\n";
        assert_eq!(
            names(Language::Pascal, text),
            [("calc".into(), 1), ("TCalculadora".into(), 3), ("Usar".into(), 4), ("Usar".into(), 6), ("Formatar".into(), 9)]
        );
    }

    #[test]
    fn the_other_languages_find_their_declarations() {
        assert_eq!(names(Language::Rust, "pub fn apertar(ui: &mut Ui) {\n    let x = 1;\n}\npub(crate) struct Estado;\n"), [("apertar".into(), 1), ("Estado".into(), 4)]);
        assert_eq!(names(Language::Python, "def apertar(aylis):\n    pass\nclass Calc:\n    def usar(self):\nESTADO = {}\n"), [("apertar".into(), 1), ("Calc".into(), 3), ("usar".into(), 4), ("ESTADO".into(), 5)]);
        assert_eq!(names(Language::TypeScript, "export async function tecla(ev) {\n}\nexport class Calc {\n    usar(x: number) {\n        if (x) {\n"), [("tecla".into(), 1), ("Calc".into(), 3), ("usar".into(), 4)]);
        assert_eq!(names(Language::CSharp, "public static class FrmCalc\n{\n    static void Apertar(Aylis aylis, string t)\n    {\n        if (x) Foo(1);\n"), [("FrmCalc".into(), 1), ("Apertar".into(), 3)]);
        assert_eq!(names(Language::Go, "func (f FrmCalc) tecla_click(app *aylis.Aylis) {\nfunc apertar(s string) {\ntype Estado struct {\n"), [("tecla_click".into(), 1), ("apertar".into(), 2), ("Estado".into(), 3)]);
        assert_eq!(names(Language::Cpp, "static void apertar(aylis::Aylis& app, const std::string& t) {\nvoid Calc::usar(double v) {\n    foo(1);\nclass Calc {\n"), [("apertar".into(), 1), ("usar".into(), 2), ("Calc".into(), 4)]);
        assert_eq!(names(Language::Java, "public class FrmCalc {\n    static void apertar(Aylis aylis, String t) {\n        apertar(aylis, x);\n"), [("FrmCalc".into(), 1), ("apertar".into(), 2)]);
    }
}
