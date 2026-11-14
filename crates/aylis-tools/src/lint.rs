//! `aylis-tool lint`: as diretrizes do `CLAUDE.md` que dá para checar por programa (item 38,
//! 1a da proposta em `docs/proposta-llm-ci.md`). O que é regra mecânica fica aqui, sem LLM, e
//! roda na CI e antes do commit; o agente da revisão fica com o que pede julgamento.
//!
//! ```text
//! aylis-tool lint                     # uma linha por achado (arquivo:linha: regra: detalhe)
//! aylis-tool lint --all               # sem descontar a base (o que já havia)
//! aylis-tool lint --update-baseline   # regrava a base com o que há hoje
//! ```
//!
//! As regras:
//! - `core-system-dependency`: o `aylis-core` depende ou importa algo de um sistema só
//!   (winit, skia, web-sys, windows...);
//! - `core-unsafe`: `unsafe` no `aylis-core` (lá não pode nenhum);
//! - `unsafe-without-safety`: `unsafe` fora do núcleo sem um `// SAFETY:` perto;
//! - `cmd-shell`: `Command::new("cmd")` (texto não vira comando: processos com argumentos
//!   separados, ou um `.bat` gravado por nós);
//! - `fixed-color`: cor literal (`Color::rgb(0x21, ...)`) num método de pintura dos controles
//!   (a cor vem de `ctx.colors` e do tema). Preto e branco puros (sombras, véus, brilhos) e o
//!   valor de reserva de um token do tema (`.unwrap_or(Color::rgb(..))`) não contam; o teste
//!   `crates/aylis/tests/colors_and_fonts.rs` confere em execução que o fundo e o texto seguem
//!   as cores do controle, e este pega a cor escrita no código, que aquele não vê;
//! - `class-not-listed`: classe que está numa lista e não na outra (`aylis_build::CLASSES` ×
//!   `aylis::classes`; o registro em execução, `register_all`, já tem o teste
//!   `crates/aylis/tests/classes.rs`);
//! - `class-without-sample`: classe sem conteúdo de exemplo em `aylis::samples`;
//! - `untranslated-text`: texto de tela da IDE sem tradução ([`crate::translations::missing`]).
//!
//! Um achado de propósito se cala com um comentário `lint-allow: <regra>` na mesma linha ou na
//! de cima, com o motivo. O que já havia quando a regra entrou fica na base
//! (`resources/lint-baseline.tsv`: regra, arquivo e quantos), que só pode diminuir: um arquivo
//! com mais achados que a base reprova.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

/// A base dos achados antigos, relativa à raiz do repositório.
pub const BASELINE: &str = "resources/lint-baseline.tsv";

/// Um achado: o arquivo (relativo à raiz, com `/`), a linha (1 em diante; 0 quando não há),
/// a regra e o detalhe.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub rule: &'static str,
    pub detail: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}: {}", self.file, self.line, self.rule, self.detail)
    }
}

impl Finding {
    /// A chave na base: o arquivo, ou o arquivo e a classe (`samples.json#Timer`) nas regras
    /// de classe, para a base dizer qual classe já estava assim e não só quantas.
    pub fn key(&self) -> String {
        match self.detail.split('`').nth(1) {
            Some(class) if self.rule.starts_with("class-") => format!("{}#{class}", self.file),
            _ => self.file.clone(),
        }
    }
}

/// Os crates dos controles, onde a pintura não pode ter cor fixa.
const CONTROL_CRATES: &[&str] = &[
    "aylis-controls",
    "aylis-controls-base",
    "aylis-controls-charts",
    "aylis-controls-industrial",
    "aylis-controls-device",
    "aylis-controls-editors",
    "aylis-dbcontrols",
    "aylis-providers",
];

/// Dependências que são de um sistema (ou de um renderizador) e não entram no núcleo. Um nome
/// que termina em `*` vale como prefixo.
const SYSTEM_CRATES: &[&str] = &[
    "winit", "skia-safe", "skia-bindings", "web-sys", "js-sys", "wasm-bindgen*", "windows*", "winapi", "objc*", "cocoa", "core-foundation*", "core-graphics*", "jni", "ndk*", "android-activity",
    "x11*", "wayland*", "libc", "nix", "rfd", "arboard", "cpal", "wgpu*", "glutin*", "softbuffer",
];

/// Os caminhos de código desses crates (`use winit::...`, `windows::Win32::...`).
const SYSTEM_PATHS: &[&str] = &[
    "winit::", "skia_safe::", "web_sys::", "js_sys::", "wasm_bindgen", "windows::", "windows_sys::", "winapi::", "objc2", "cocoa::", "core_foundation", "jni::", "ndk::", "libc::", "nix::", "wgpu::",
];

/// Roda todas as regras e desconta a base (com `all`, não desconta).
pub fn run(root: &Path, all: bool) -> Vec<Finding> {
    let found = check(root);
    if all { found } else { apply_baseline(found, &read_baseline(root)) }
}

/// Todos os achados, sem a base, em ordem.
pub fn check(root: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let crates = root.join("crates");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&crates).map(|d| d.flatten().map(|e| e.path()).filter(|p| p.join("Cargo.toml").is_file()).collect()).unwrap_or_default();
    dirs.sort();
    for dir in &dirs {
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let mut files = Vec::new();
        rust_files(&dir.join("src"), &mut files);
        files.sort();
        for path in files {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let rel = relative(root, &path);
            let source = Source::new(&text);
            if name == "aylis-core" {
                core_source(&rel, &source, &mut out);
            } else {
                unsafe_without_safety(&rel, &source, &mut out);
            }
            cmd_shell(&rel, &source, &mut out);
            if CONTROL_CRATES.contains(&name.as_str()) {
                fixed_colors(&rel, &source, &mut out);
            }
        }
    }
    core_manifest(root, &mut out);
    classes(root, &mut out);
    untranslated(root, &mut out);
    out.sort();
    out
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

/// Os `.rs` de uma pasta, recursivo.
fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => rust_files(&path, out),
            Ok(_) if path.extension().is_some_and(|e| e == "rs") => out.push(path),
            _ => {}
        }
    }
}

/// Um fonte Rust lido para as regras: o texto, o mesmo com comentários e o conteúdo das strings
/// trocados por espaços (as posições e as linhas ficam), e os trechos de `#[cfg(test)] mod`.
struct Source<'a> {
    text: &'a str,
    code: String,
    lines: Vec<&'a str>,
    line_starts: Vec<usize>,
    tests: Vec<(usize, usize)>,
}

impl<'a> Source<'a> {
    fn new(text: &'a str) -> Self {
        let code = mask(text);
        let mut line_starts = vec![0];
        line_starts.extend(text.bytes().enumerate().filter(|(_, b)| *b == b'\n').map(|(i, _)| i + 1));
        let tests = test_modules(&code);
        Source { text, code, lines: text.lines().collect(), line_starts, tests }
    }

    /// A linha (1 em diante) de uma posição.
    fn line_of(&self, pos: usize) -> usize {
        self.line_starts.partition_point(|&s| s <= pos)
    }

    fn in_tests(&self, pos: usize) -> bool {
        self.tests.iter().any(|&(a, b)| pos >= a && pos < b)
    }

    /// O achado se cala com `lint-allow: <regra>` na linha ou num comentário sozinho na de cima.
    fn allowed(&self, line: usize, rule: &str) -> bool {
        let tag = format!("lint-allow: {rule}");
        let at = |l: usize| if l >= 1 { self.lines.get(l - 1).copied() } else { None };
        at(line).is_some_and(|t| t.contains(&tag)) || at(line.saturating_sub(1)).is_some_and(|t| t.trim_start().starts_with("//") && t.contains(&tag))
    }

    /// As posições de uma palavra inteira no código (sem comentários nem strings).
    fn words(&self, word: &str) -> Vec<usize> {
        let b = self.code.as_bytes();
        self.code
            .match_indices(word)
            .map(|(i, _)| i)
            .filter(|&i| (i == 0 || !is_ident(b[i - 1])) && b.get(i + word.len()).is_none_or(|c| !is_ident(*c)))
            .collect()
    }
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Troca comentários e o conteúdo de strings e caracteres por espaços (as quebras de linha
/// ficam), para as regras não acharem nada dentro deles. As aspas das strings ficam.
fn mask(text: &str) -> String {
    let b = text.as_bytes();
    let mut out = b.to_vec();
    let blank = |out: &mut Vec<u8>, from: usize, to: usize| {
        for c in &mut out[from..to.min(b.len())] {
            if *c != b'\n' {
                *c = b' ';
            }
        }
    };
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        let prev_ident = i > 0 && is_ident(b[i - 1]);
        if c == b'/' && b.get(i + 1) == Some(&b'/') {
            let end = b[i..].iter().position(|&c| c == b'\n').map_or(b.len(), |p| i + p);
            blank(&mut out, i, end);
            i = end;
        } else if c == b'/' && b.get(i + 1) == Some(&b'*') {
            let (mut depth, mut j) = (1, i + 2);
            while j < b.len() && depth > 0 {
                if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                    depth += 1;
                    j += 2;
                } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            blank(&mut out, i, j);
            i = j;
        } else if !prev_ident && (c == b'r' || (c == b'b' && b.get(i + 1) == Some(&b'r'))) && {
            let k = if c == b'b' { i + 2 } else { i + 1 };
            let hashes = b[k..].iter().take_while(|&&c| c == b'#').count();
            b.get(k + hashes) == Some(&b'"')
        } {
            // r"..." e r#"..."# (e br"...").
            let k = if c == b'b' { i + 2 } else { i + 1 };
            let hashes = b[k..].iter().take_while(|&&c| c == b'#').count();
            let start = k + hashes + 1;
            let mut j = start;
            while j < b.len() && !(b[j] == b'"' && b[j + 1..].iter().take(hashes).filter(|&&c| c == b'#').count() == hashes) {
                j += 1;
            }
            blank(&mut out, start, j);
            i = j + 1 + hashes;
        } else if c == b'"' {
            let mut j = i + 1;
            while j < b.len() && b[j] != b'"' {
                j += if b[j] == b'\\' { 2 } else { 1 };
            }
            blank(&mut out, i + 1, j);
            i = j + 1;
        } else if c == b'\'' {
            // Caractere ('a', '\n', 'é') ou tempo de vida ('a sem fechar).
            if b.get(i + 1) == Some(&b'\\') {
                let end = b[i + 2..].iter().position(|&c| c == b'\'').map_or(b.len(), |p| i + 2 + p);
                blank(&mut out, i + 1, end);
                i = end + 1;
            } else {
                let len = text[i + 1..].chars().next().map_or(1, char::len_utf8);
                if b.get(i + 1 + len) == Some(&b'\'') {
                    blank(&mut out, i + 1, i + 1 + len);
                    i += 2 + len;
                } else {
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// O fim do bloco que abre em `open` (a posição do `{`), no código mascarado.
fn block_end(code: &str, open: usize) -> usize {
    let mut depth = 0;
    for (i, c) in code.bytes().enumerate().skip(open) {
        match c {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
    }
    code.len()
}

/// Os trechos dos módulos de teste (`#[cfg(test)] mod x { ... }`).
fn test_modules(code: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (i, _) in code.match_indices("#[cfg(test)]") {
        let rest = &code[i + 12..];
        let trimmed = rest.trim_start();
        if let Some(after) = trimmed.strip_prefix("mod ") {
            if let Some(open) = after.find('{').filter(|&o| !after[..o].contains(';')) {
                let start = i + 12 + (rest.len() - trimmed.len()) + 4 + open;
                out.push((i, block_end(code, start)));
            }
        }
    }
    out
}

fn push(out: &mut Vec<Finding>, file: &str, line: usize, rule: &'static str, detail: impl Into<String>) {
    out.push(Finding { file: file.to_string(), line, rule, detail: detail.into() });
}

/// O `aylis-core`: sem caminho de crate de sistema e sem `unsafe` nenhum.
fn core_source(rel: &str, s: &Source, out: &mut Vec<Finding>) {
    let b = s.code.as_bytes();
    for path in SYSTEM_PATHS {
        for (i, _) in s.code.match_indices(path) {
            if i > 0 && (is_ident(b[i - 1]) || b[i - 1] == b':') {
                continue;
            }
            let line = s.line_of(i);
            if !s.allowed(line, "core-system-dependency") {
                push(out, rel, line, "core-system-dependency", format!("o núcleo usa `{}` (o que é do sistema vira trait de serviço em platform.rs)", path.trim_end_matches("::")));
            }
        }
    }
    for i in s.words("unsafe") {
        push(out, rel, s.line_of(i), "core-unsafe", "o aylis-core não tem `unsafe`");
    }
}

/// O `Cargo.toml` do `aylis-core`: nenhuma dependência de sistema, em nenhuma tabela.
fn core_manifest(root: &Path, out: &mut Vec<Finding>) {
    let path = root.join("crates/aylis-core/Cargo.toml");
    let Ok(text) = std::fs::read_to_string(&path) else { return };
    let rel = relative(root, &path);
    let banned = |name: &str| SYSTEM_CRATES.iter().any(|b| b.strip_suffix('*').map_or(name == *b, |p| name.starts_with(p)));
    let mut section = String::new();
    for (n, line) in text.lines().enumerate() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t.trim_matches(['[', ']']).to_string();
            continue;
        }
        // `[dependencies]` (e `[target.'cfg(..)'.dependencies]`): uma por linha; `[dependencies.winit]`:
        // o nome na seção (achado na primeira linha dela).
        let name = if section.ends_with("dependencies") {
            if t.is_empty() || t.starts_with('#') { "" } else { t.split(['=', ' ', '.']).next().unwrap_or_default().trim_matches('"') }
        } else if let Some((_, dep)) = section.rsplit_once("dependencies.") {
            if t.is_empty() || t.starts_with('#') { "" } else { dep.trim_matches('"') }
        } else {
            ""
        };
        if !name.is_empty() && banned(name) && !out.iter().any(|f: &Finding| f.file == rel && f.detail.contains(&format!("`{name}`"))) {
            push(out, &rel, n + 1, "core-system-dependency", format!("o aylis-core depende de `{name}` (só o que roda em qualquer sistema)"));
        }
    }
}

/// `unsafe` sem `// SAFETY:` na linha, nas 4 de cima ou na de baixo (a primeira do bloco).
fn unsafe_without_safety(rel: &str, s: &Source, out: &mut Vec<Finding>) {
    for i in s.words("unsafe") {
        let after = s.code[i + 6..].trim_start();
        // `#[unsafe(no_mangle)]` e `unsafe extern "C"` como tipo de ponteiro de função não são blocos.
        if !(after.starts_with('{') || after.starts_with("fn ") || after.starts_with("impl") || after.starts_with("extern") || after.starts_with("trait")) {
            continue;
        }
        // `unsafe extern "C" { fn ...; }` só declara as funções do sistema (quem precisa do
        // `SAFETY:` é quem chama), e `unsafe extern "C" fn(..)` é um tipo.
        if after.starts_with("extern") {
            continue;
        }
        let line = s.line_of(i);
        let near = (line.saturating_sub(4).max(1)..=line + 1).any(|l| s.lines.get(l - 1).is_some_and(|t| t.to_ascii_uppercase().contains("SAFETY")));
        if !near && !s.allowed(line, "unsafe-without-safety") {
            push(out, rel, line, "unsafe-without-safety", "`unsafe` sem um `// SAFETY:` dizendo por que vale");
        }
    }
}

/// `Command::new("cmd")`: o texto não vira comando de shell.
fn cmd_shell(rel: &str, s: &Source, out: &mut Vec<Finding>) {
    for (i, _) in s.code.match_indices("Command::new(\"") {
        let start = i + "Command::new(\"".len();
        let Some(end) = s.text[start..].find('"') else { continue };
        let program = s.text[start..start + end].to_ascii_lowercase();
        if program == "cmd" || program == "cmd.exe" {
            let line = s.line_of(i);
            if !s.in_tests(i) && !s.allowed(line, "cmd-shell") {
                push(out, rel, line, "cmd-shell", "`Command::new(\"cmd\")`: use processos com argumentos separados ou `aylis_tools::shell::open`");
            }
        }
    }
}

/// As funções de pintura (`paint`, `paint_*`, `draw*` ou que recebem um `Canvas`): o trecho do
/// corpo de cada uma.
fn paint_bodies(s: &Source) -> Vec<(usize, usize)> {
    let code = &s.code;
    let mut out: Vec<(usize, usize)> = Vec::new();
    for i in s.words("fn") {
        let rest = &code[i + 2..];
        let name: String = rest.trim_start().bytes().take_while(|&c| is_ident(c)).map(char::from).collect();
        if name.is_empty() {
            continue;
        }
        let Some(open) = rest.find(['{', ';']) else { continue };
        if rest.as_bytes()[open] == b';' {
            continue;
        }
        let signature = &rest[..open];
        if name == "paint" || name.starts_with("paint_") || name.starts_with("draw") || signature.contains("Canvas") {
            let start = i + 2 + open;
            let end = block_end(code, start);
            // Uma função dentro de outra já está no trecho de fora.
            if !out.iter().any(|&(a, b)| start >= a && start < b) {
                out.push((start, end));
            }
        }
    }
    out
}

/// Os argumentos de uma chamada a partir do `(`, até o `)` que fecha.
fn call_args(code: &str, open: usize) -> Option<(Vec<&str>, usize)> {
    let mut depth = 0;
    for (i, c) in code.bytes().enumerate().skip(open) {
        match c {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((code[open + 1..i].split(',').map(str::trim).filter(|a| !a.is_empty()).collect(), i + 1));
                }
            }
            _ => {}
        }
    }
    None
}

/// Um inteiro escrito no código (`0x21`, `255`, `40u8`).
fn int_literal(s: &str) -> Option<u32> {
    let s = s.trim_end_matches("u8").trim_end_matches('_').replace('_', "");
    match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Some(hex) => u32::from_str_radix(hex, 16).ok(),
        None => s.parse().ok(),
    }
}

/// Cor literal num método de pintura dos controles.
fn fixed_colors(rel: &str, s: &Source, out: &mut Vec<Finding>) {
    let code = &s.code;
    for (start, end) in paint_bodies(s) {
        let body = &code[start..end];
        for pattern in ["Color::rgb(", "Color::rgba(", "Color::from_hex("] {
            for (k, _) in body.match_indices(pattern) {
                let at = start + k;
                if s.in_tests(at) || (at > 0 && is_ident(code.as_bytes()[at - 1])) {
                    continue;
                }
                let open = at + pattern.len() - 1;
                let Some((args, _)) = call_args(code, open) else { continue };
                let literal = if pattern == "Color::from_hex(" {
                    // O texto da string está no original (no mascarado, só as aspas).
                    args.first().is_some_and(|a| a.starts_with('"'))
                } else {
                    let values: Vec<Option<u32>> = args.iter().map(|a| int_literal(a)).collect();
                    let rgb: Vec<u32> = values.iter().take(3).flatten().copied().collect();
                    // Só literais; preto e branco puros (sombras, véus, brilhos) não contam.
                    values.iter().all(Option::is_some) && rgb.len() == 3 && !(rgb.iter().all(|&v| v == 0) || rgb.iter().all(|&v| v == 255))
                };
                if !literal {
                    continue;
                }
                // O valor de reserva de um token do tema: `.unwrap_or(Color::rgb(..))`.
                let line_start = s.line_starts[s.line_of(at) - 1];
                let before = &code[line_start..at];
                if before.trim_end().ends_with("unwrap_or(") || before.trim_end().ends_with("unwrap_or(Some(") {
                    continue;
                }
                let line = s.line_of(at);
                if !s.allowed(line, "fixed-color") {
                    let text = s.text[at..code[at..].find(')').map_or(at, |p| at + p + 1)].to_string();
                    push(out, rel, line, "fixed-color", format!("`{text}` na pintura: a cor vem de `ctx.colors` ou do tema"));
                }
            }
        }
    }
}

/// As classes: `aylis_build::CLASSES` × `aylis::classes`, e o exemplo de cada uma em
/// `aylis::samples` (o `samples.json` ou o `fill` do `samples.rs`).
fn classes(root: &Path, out: &mut Vec<Finding>) {
    let listed: BTreeSet<&str> = aylis_build::CLASSES.iter().map(|(n, _)| *n).collect();
    let facade_path = root.join("crates/aylis/src/lib.rs");
    let Ok(facade) = std::fs::read_to_string(&facade_path) else { return };
    let facade_rel = relative(root, &facade_path);
    // O módulo `pub mod classes { ... }`: as linhas `Nome: tipo => "Categoria"` do `classes!`
    // (com ou sem `#[cfg(..)]` na frente) e as funções escritas à mão (`pub fn Nome(ui: ...)`).
    let mut functions: BTreeMap<String, usize> = BTreeMap::new();
    let code = mask(&facade);
    let Some(start) = code.find("pub mod classes {") else { return };
    let end = block_end(&code, start + "pub mod classes ".len());
    let first_line = facade[..start].matches('\n').count();
    for (n, line) in facade[start..end].lines().enumerate() {
        let mut t = line.trim();
        while let Some(rest) = t.strip_prefix("#[") {
            t = rest.find(']').map_or("", |i| rest[i + 1..].trim_start());
        }
        let name = if let Some(rest) = t.strip_prefix("pub fn ") {
            rest.split_once("(ui").map(|(name, _)| name)
        } else {
            t.split_once(':').filter(|(_, rest)| rest.contains("=>")).map(|(name, _)| name)
        };
        if let Some(name) = name.filter(|n| !n.is_empty() && n.bytes().all(is_ident) && n.starts_with(|c: char| c.is_ascii_uppercase())) {
            functions.insert(name.to_string(), first_line + n + 1);
        }
    }
    if functions.is_empty() {
        return;
    }
    let build_rel = "crates/aylis-build/src/lib.rs";
    for name in &listed {
        if !functions.contains_key(*name) {
            push(out, &facade_rel, 0, "class-not-listed", format!("`{name}` está em aylis_build::CLASSES e não em aylis::classes"));
        }
    }
    for (name, line) in &functions {
        if !listed.contains(name.as_str()) {
            push(out, &facade_rel, *line, "class-not-listed", format!("`{name}` está em aylis::classes e não em aylis_build::CLASSES ({build_rel})"));
        }
    }
    let samples_json = std::fs::read_to_string(root.join("crates/aylis/src/samples.json")).ok().and_then(|t| serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&t).ok()).unwrap_or_default();
    let samples_code = std::fs::read_to_string(root.join("crates/aylis/src/samples.rs")).unwrap_or_default();
    for name in &listed {
        if !samples_json.contains_key(*name) && !samples_code.contains(&format!("\"{name}\"")) {
            push(out, "crates/aylis/src/samples.json", 0, "class-without-sample", format!("`{name}` sem conteúdo de exemplo em aylis::samples"));
        }
    }
}

/// Os textos de tela da IDE sem tradução, com o arquivo e a linha do primeiro uso.
fn untranslated(root: &Path, out: &mut Vec<Finding>) {
    let missing = crate::translations::missing(root);
    if missing.is_empty() {
        return;
    }
    let dir = root.join("crates/aylis-ide/src");
    let mut files = Vec::new();
    rust_files(&dir, &mut files);
    files.sort();
    let texts: Vec<(PathBuf, String)> = files.into_iter().filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p, t))).collect();
    for text in missing {
        let quoted = serde_json::to_string(&text).unwrap_or_default();
        let place = texts.iter().find_map(|(p, t)| t.find(&quoted).map(|i| (relative(root, p), t[..i].matches('\n').count() + 1)));
        let (file, line) = place.unwrap_or_else(|| (relative(root, &dir), 0));
        push(out, &file, line, "untranslated-text", format!("{quoted} sem tradução (aylis-tool translations; o que não é de tela vai para resources/locales/untranslated.txt)"));
    }
}

/// A base: quantos achados de cada regra cada arquivo pode ter.
pub fn read_baseline(root: &Path) -> BTreeMap<(String, String), usize> {
    std::fs::read_to_string(root.join(BASELINE)).map(|t| parse_baseline(&t)).unwrap_or_default()
}

fn parse_baseline(text: &str) -> BTreeMap<(String, String), usize> {
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let mut cols = l.split('\t');
            let (rule, file, count) = (cols.next()?, cols.next()?, cols.next()?.trim().parse().ok()?);
            Some(((rule.to_string(), file.to_string()), count))
        })
        .collect()
}

/// Os achados que passam da base: num arquivo com mais achados de uma regra que a base
/// permite, todos os daquela regra aparecem (não dá para saber qual é o novo).
pub fn apply_baseline(found: Vec<Finding>, baseline: &BTreeMap<(String, String), usize>) -> Vec<Finding> {
    let mut groups: BTreeMap<(String, String), Vec<Finding>> = BTreeMap::new();
    for f in found {
        groups.entry((f.rule.to_string(), f.key())).or_default().push(f);
    }
    let mut out = Vec::new();
    for (key, list) in groups {
        let allowed = baseline.get(&key).copied().unwrap_or(0);
        if list.len() > allowed {
            let n = list.len();
            out.extend(list.into_iter().map(|mut f| {
                if allowed > 0 {
                    f.detail = format!("{} ({n} no arquivo, a base permite {allowed})", f.detail);
                }
                f
            }));
        }
    }
    out.sort();
    out
}

/// O texto da base com os achados de hoje.
pub fn baseline_text(found: &[Finding]) -> String {
    let mut counts: BTreeMap<(&str, String), usize> = BTreeMap::new();
    for f in found {
        *counts.entry((f.rule, f.key())).or_default() += 1;
    }
    let mut text = String::from(
        "# Os achados do `aylis-tool lint` que já havia quando a regra entrou: regra, arquivo e quantos\n\
         # (separados por tabulação). Só diminui: corrigiu, rode `aylis-tool lint --update-baseline`.\n",
    );
    for ((rule, file), n) in counts {
        text.push_str(&format!("{rule}\t{file}\t{n}\n"));
    }
    text
}

/// Regrava a base com o que há hoje. Devolve quantos achados ficaram nela.
pub fn update_baseline(root: &Path) -> Result<usize, String> {
    let found = check(root);
    let path = root.join(BASELINE);
    std::fs::write(&path, baseline_text(&found)).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(found.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn findings(rel: &str, text: &str, rule: fn(&str, &Source, &mut Vec<Finding>)) -> Vec<(usize, &'static str)> {
        let s = Source::new(text);
        let mut out = Vec::new();
        rule(rel, &s, &mut out);
        out.into_iter().map(|f| (f.line, f.rule)).collect()
    }

    #[test]
    fn the_mask_hides_comments_strings_and_chars() {
        let src = "let a = \"unsafe { }\"; // unsafe {\nlet b = '\"'; let c = r#\"x\"y\"#; /* Color::rgb(1, 2, 3) */ let d: &'static str = \"\";";
        let m = mask(src);
        assert_eq!(m.len(), src.len());
        assert!(!m.contains("unsafe"));
        assert!(!m.contains("Color"));
        assert!(m.contains("&'static str"));
        assert_eq!(m.matches('\n').count(), 1);
    }

    #[test]
    fn fixed_colors_only_in_paint_and_not_black_white_or_theme_fallback() {
        let src = r##"
impl Component for X {
    fn paint(&self, ctx: &PaintContext, canvas: &mut dyn Canvas) {
        let a = Color::rgb(0x21, 0x96, 0xF3);
        let shadow = Color::rgba(0, 0, 0, 40);
        let white = Color::rgb(255, 255, 255);
        let t = ctx.theme.colors.get("x").copied().unwrap_or(Color::rgb(1, 2, 3));
        let mixed = Color::rgba(c.r, c.g, c.b, 10);
        let ok = Color::rgb(9, 9, 9); // lint-allow: fixed-color (o desenho é a própria cor)
        let hex = Color::from_hex("#ff0000");
    }
}
fn new() -> Self { Self { color: Color::rgb(0x21, 0x96, 0xF3) } }
fn helper(canvas: &mut dyn Canvas) { canvas.fill(Color::rgb(10, 20, 30)); }
#[cfg(test)]
mod tests {
    fn paint_test() { let _ = Color::rgb(1, 2, 3); }
}
"##;
        assert_eq!(findings("x.rs", src, fixed_colors), [(4, "fixed-color"), (10, "fixed-color"), (14, "fixed-color")]);
    }

    #[test]
    fn unsafe_needs_a_safety_comment() {
        let src = "fn b() {\n    unsafe { g() };\n}\nfn a() {\n    // SAFETY: o ponteiro vem do sistema.\n    unsafe { f() };\n}\n#[unsafe(no_mangle)]\npub extern \"C\" fn h() {}\ntype F = unsafe extern \"C\" fn();\nfn c() {\n    let x = 1;\n    let y = 2;\n    let z = 3;\n    let w = 4;\n    let v = 5;\n    unsafe {\n        // SAFETY: logo abaixo.\n        g()\n    };\n}\n";
        assert_eq!(findings("x.rs", src, unsafe_without_safety), [(2, "unsafe-without-safety")]);
        let mut out = Vec::new();
        let manifest = "[dependencies]\nserde = \"1\"\nwinit = { version = \"0.30\" }\n[target.'cfg(windows)'.dependencies]\nwindows-sys = \"0.59\"\n[dependencies.libc]\nversion = \"0.2\"\n";
        let dir = std::env::temp_dir().join(format!("aylis_lint_{}", std::process::id()));
        std::fs::create_dir_all(dir.join("crates/aylis-core")).unwrap();
        std::fs::write(dir.join("crates/aylis-core/Cargo.toml"), manifest).unwrap();
        core_manifest(&dir, &mut out);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(out.iter().map(|f| f.line).collect::<Vec<_>>(), [3, 5, 7]);
        assert_eq!(findings("core.rs", "fn a() { unsafe { f() } }\n// unsafe no comentário\n", core_source), [(1, "core-unsafe")]);
    }

    #[test]
    fn system_paths_in_the_core_and_cmd() {
        let src = "use winit::window::Window;\nuse crate::my_winit::x;\nlet s = \"winit::\";\n";
        assert_eq!(findings("core.rs", src, core_source), [(1, "core-system-dependency")]);
        let src = "let c = Command::new(\"cmd\");\nlet d = Command::new(\"cargo\");\n// lint-allow: cmd-shell (um .bat nosso)\nlet e = Command::new(\"cmd.exe\");\n";
        assert_eq!(findings("x.rs", src, cmd_shell), [(1, "cmd-shell")]);
    }

    #[test]
    fn the_baseline_only_lets_through_what_was_already_there() {
        let f = |file: &str, line| Finding { file: file.into(), line, rule: "fixed-color", detail: String::new() };
        let found = vec![f("a.rs", 1), f("a.rs", 2), f("b.rs", 3)];
        let base = parse_baseline(&baseline_text(&[f("a.rs", 1), f("a.rs", 9)]));
        let left = apply_baseline(found.clone(), &base);
        assert_eq!(left.iter().map(|f| (f.file.as_str(), f.line)).collect::<Vec<_>>(), [("b.rs", 3)]);
        let base = parse_baseline(&baseline_text(&[f("a.rs", 1)]));
        assert_eq!(apply_baseline(found, &base).len(), 3);
    }

    #[test]
    fn the_display_is_file_line_rule() {
        let f = Finding { file: "crates/x/src/a.rs".into(), line: 7, rule: "cmd-shell", detail: "d".into() };
        assert_eq!(f.to_string(), "crates/x/src/a.rs:7: cmd-shell: d");
    }

    /// O repositório de hoje passa limpo (com a base): uma regra quebrada aparece aqui. Com
    /// `AYLIS_UPDATE_LINT=1`, regrava a base antes (como o `aylis-tool lint --update-baseline`).
    #[test]
    fn the_repository_passes_the_lint() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        if std::env::var_os("AYLIS_UPDATE_LINT").is_some() {
            update_baseline(&root).unwrap();
        }
        let left = run(&root, false);
        assert!(left.is_empty(), "{} achados do aylis-tool lint:\n{}", left.len(), left.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n"));
    }
}
