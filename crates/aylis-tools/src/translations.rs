//! Os idiomas da IDE: acha os textos da tela que faltam nos catálogos
//! (`crates/aylis-ide/resources/locales/<código>.json`) e acrescenta traduções.
//!
//! ```text
//! aylis-tool translations                # os textos da IDE sem tradução (um por linha)
//! aylis-tool translations --apply t.tsv  # acrescenta as traduções da tabela
//! ```
//!
//! A tabela tem uma linha por texto: o português (a chave) e as traduções separadas por
//! tabulação, na ordem de [`LANGUAGES`]. Os catálogos JSON são a fonte (o antigo
//! `scripts/ide_idiomas.py` saiu): uma linha nova na tabela vira uma chave em cada um.
//!
//! Os literais que parecem texto de tela mas não são (dados de exemplo, nomes próprios e de
//! fonte, o que vai para o agente do MCP, textos de teste) ficam em
//! `resources/locales/untranslated.txt`, um por linha, e saem da lista (decidido em 14/11:
//! assim a lista vazia quer dizer que nada falta, e o teste confere).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Os idiomas dos catálogos, na ordem das colunas da tabela.
pub const LANGUAGES: [&str; 11] = ["en", "es", "fr", "de", "ru", "zh", "ja", "hi", "ar", "bn", "id"];

/// As strings literais de um arquivo Rust (sem as de comentários; com os escapes lidos).
pub fn literals(source: &str) -> Vec<String> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        // Comentários.
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && chars.get(i + 1) == Some(&'*') {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        // Caractere: '"' não abre string.
        if c == '\'' {
            if chars.get(i + 1) == Some(&'\\') && chars.get(i + 3) == Some(&'\'') {
                i += 4;
                continue;
            }
            if chars.get(i + 2) == Some(&'\'') {
                i += 3;
                continue;
            }
            i += 1;
            continue;
        }
        // Strings cruas: r"..." e r#"..."#.
        if c == 'r' && matches!(chars.get(i + 1), Some('"') | Some('#')) && (i == 0 || !chars[i - 1].is_alphanumeric()) {
            let mut j = i + 1;
            let mut hashes = 0;
            while chars.get(j) == Some(&'#') {
                hashes += 1;
                j += 1;
            }
            if chars.get(j) == Some(&'"') {
                j += 1;
                let start = j;
                loop {
                    if j >= chars.len() {
                        break;
                    }
                    if chars[j] == '"' && (0..hashes).all(|k| chars.get(j + 1 + k) == Some(&'#')) {
                        break;
                    }
                    j += 1;
                }
                out.push(chars[start..j.min(chars.len())].iter().collect());
                i = j + 1 + hashes;
                continue;
            }
        }
        if c == '"' {
            let mut s = String::new();
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                    match chars.get(i) {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('\n') => {
                            // Continuação de linha: pula os espaços do começo da próxima.
                            while chars.get(i + 1).is_some_and(|c| c.is_whitespace()) {
                                i += 1;
                            }
                        }
                        Some(other) => s.push(*other),
                        None => {}
                    }
                } else {
                    s.push(chars[i]);
                }
                i += 1;
            }
            out.push(s);
        }
        i += 1;
    }
    out
}

/// Parece um texto da tela (e não um nome de propriedade, um caminho, um formato...).
pub fn is_screen_text(s: &str) -> bool {
    let t = s.trim();
    if t.is_empty() || t != s || t.chars().count() < 3 {
        return false;
    }
    // Uma palavra com 3 letras seguidas, ao menos.
    let mut run = 0;
    let mut has_word = false;
    for c in t.chars() {
        run = if c.is_alphabetic() { run + 1 } else { 0 };
        has_word |= run >= 3;
    }
    if !has_word {
        return false;
    }
    let bad = ['{', '}', '\\', '/', '<', '>', '=', '|', '$', '#', '@', '`'];
    if t.chars().any(|c| bad.contains(&c)) || t.contains("::") || t.contains("http") || t.contains('_') && !t.contains(' ') {
        return false;
    }
    let first = t.chars().next().unwrap_or(' ');
    let single = !t.contains(' ');
    if single {
        // Uma palavra só: começando com maiúscula (um rótulo, um botão), sem pontos no meio
        // (nomes de arquivo) nem maiúsculas no meio (nomes de classe).
        let inner_upper = t.chars().skip(1).any(char::is_uppercase);
        return first.is_uppercase() && !t.contains('.') && !inner_upper && t.chars().all(|c| c.is_alphabetic() || matches!(c, '…' | ':' | '-' | '?' | '!'));
    }
    // Uma frase: começa com letra (ou "…", "(") e tem letras de verdade.
    first.is_alphabetic() || matches!(first, '(' | '…' | '¿')
}

/// Um modelo de mensagem com valores (`"Form {} criado"`): texto da tela com `{}` no meio.
pub fn is_template(s: &str) -> bool {
    s.contains("{}") && is_screen_text(&s.replace("{}", "X").replace("{:?}", "X"))
}

/// As chaves de um catálogo.
fn catalog_keys(path: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&t).ok())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default()
}

fn ide(root: &Path) -> PathBuf {
    root.join("crates/aylis-ide")
}

/// Os textos da IDE que faltam no catálogo inglês (o de referência), em ordem, menos os da
/// lista de não traduzir.
pub fn missing(root: &Path) -> Vec<String> {
    let skip = untranslated(root);
    scan(root, is_screen_text).into_iter().filter(|s| !skip.contains(s)).collect()
}

/// Os literais que não são texto de tela (`resources/locales/untranslated.txt`).
pub fn untranslated(root: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(ide(root).join("resources/locales/untranslated.txt"))
        // Um texto com quebra de linha vai com `\n` escrito.
        .map(|t| t.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).map(|l| l.replace("\\n", "\n")).collect())
        .unwrap_or_default()
}

/// Os modelos de mensagem com valores (`"Form {} criado"`) que faltam no catálogo inglês.
pub fn missing_templates(root: &Path) -> Vec<String> {
    scan(root, is_template)
}

fn scan(root: &Path, wanted: fn(&str) -> bool) -> Vec<String> {
    let keys = catalog_keys(&ide(root).join("resources/locales/en.json"));
    let mut found = BTreeSet::new();
    let Ok(dir) = std::fs::read_dir(ide(root).join("src")) else { return Vec::new() };
    for entry in dir.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "rs") {
            if let Ok(text) = std::fs::read_to_string(&path) {
                for s in literals(&text) {
                    if wanted(&s) && !keys.contains(&s) {
                        found.insert(s);
                    }
                }
            }
        }
    }
    found.into_iter().collect()
}

/// Escapa para dentro de aspas (JSON e Python aceitam o mesmo aqui).
fn quote(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_default()
}

/// Acrescenta as traduções da tabela (português e as 11 traduções por linha, separadas por
/// tabulação) nos catálogos. Devolve quantas entraram.
pub fn apply(root: &Path, table: &str) -> Result<usize, String> {
    let rows: Vec<Vec<&str>> = table.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).map(|l| l.split('\t').collect()).collect();
    for (n, r) in rows.iter().enumerate() {
        if r.len() != LANGUAGES.len() + 1 {
            return Err(format!("linha {}: {} colunas (precisa de {})", n + 1, r.len(), LANGUAGES.len() + 1));
        }
    }
    for (col, code) in LANGUAGES.iter().enumerate() {
        let path = ide(root).join(format!("resources/locales/{code}.json"));
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        for r in &rows {
            map.insert(r[0].to_string(), serde_json::Value::String(r[col + 1].to_string()));
        }
        // Como o gerador grava (o `json.dump` com `indent=1` e `sort_keys`): chaves em ordem,
        // uma por linha com um espaço, sem escapar o Unicode.
        let mut out = String::from("{\n");
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();
        for (i, k) in keys.iter().enumerate() {
            let v = map[k.as_str()].as_str().unwrap_or_default();
            out.push_str(&format!(" {}: {}{}\n", quote(k), quote(v), if i + 1 < keys.len() { "," } else { "" }));
        }
        out.push_str("}\n");
        std::fs::write(&path, out).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(rows.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Nenhum texto de tela da IDE sem tradução (decidido em 14/11): um texto novo entra no
    /// catálogo (`aylis-tool translations --apply`) ou, se não é de tela, na lista de não traduzir.
    #[test]
    fn no_screen_text_of_the_ide_lacks_a_translation() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let missing = missing(&root);
        assert!(
            missing.is_empty(),
            "{} textos sem tradução (aylis-tool translations; os que não são de tela vão para resources/locales/untranslated.txt):\n{}",
            missing.len(),
            missing.join("\n")
        );
    }

    #[test]
    fn literals_and_screen_texts() {
        let src = r##"
            // "comentário"
            let a = "Salvar como…"; let b = 'a'; let c = '"';
            label(ui, "Placa de vídeo"); set("font_size"); x("src/main.rs");
            let raw = r#"{"json": 1}"#; let f = format!("Form {} criado", n);
        "##;
        let all = literals(src);
        assert!(all.contains(&"Salvar como…".to_string()));
        assert!(!all.iter().any(|s| s.contains("comentário")));
        let screen: Vec<&String> = all.iter().filter(|s| is_screen_text(s)).collect();
        assert_eq!(screen, ["Salvar como…", "Placa de vídeo"]);
    }
}
