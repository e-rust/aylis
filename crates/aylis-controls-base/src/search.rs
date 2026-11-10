//! Localizar e substituir num texto, como no VS Code: texto exato ou expressão regular,
//! diferenciando maiúsculas ou não, só a palavra inteira; na substituição, os grupos da
//! expressão (`$1`, `${nome}`) e, se pedido, a caixa do que foi achado ("Nome" → "Valor",
//! "NOME" → "VALOR"). O editor de código e a pesquisa no projeto da IDE usam o mesmo.

use regex::{Regex, RegexBuilder};

/// O que procurar e como.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchOptions {
    pub query: String,
    /// Diferenciar maiúsculas de minúsculas.
    pub case: bool,
    /// Só a palavra inteira.
    pub whole: bool,
    /// `query` é uma expressão regular.
    pub regex: bool,
    /// Na substituição, a caixa do achado vale para o substituto.
    pub preserve_case: bool,
}

/// Uma ocorrência: onde está, em caracteres e em bytes do texto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Found {
    pub start: usize,
    pub end: usize,
    pub byte_start: usize,
    pub byte_end: usize,
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl SearchOptions {
    pub fn new(query: impl Into<String>) -> Self {
        Self { query: query.into(), ..Default::default() }
    }

    /// A expressão que procura (o texto exato vira uma expressão escapada).
    fn compile(&self) -> Result<Regex, String> {
        let pattern = if self.regex { self.query.clone() } else { regex::escape(&self.query) };
        let pattern = if self.whole { format!(r"\b(?:{pattern})\b") } else { pattern };
        RegexBuilder::new(&pattern).case_insensitive(!self.case).multi_line(true).build().map_err(|e| {
            // Só a última linha do erro do regex diz o que houve.
            let text = e.to_string();
            text.lines().last().unwrap_or(&text).trim().trim_start_matches("error: ").to_string()
        })
    }

    /// As ocorrências em `text`. Erro: a expressão regular não é válida (a mensagem diz por quê).
    pub fn find_all(&self, text: &str) -> Result<Vec<Found>, String> {
        if self.query.is_empty() {
            return Ok(Vec::new());
        }
        let re = self.compile()?;
        let mut out = Vec::new();
        // Os bytes viram caracteres andando junto (as ocorrências vêm em ordem).
        let (mut byte, mut chars) = (0usize, 0usize);
        let mut to_chars = |target: usize| {
            chars += text[byte..target].chars().count();
            byte = target;
            chars
        };
        for m in re.find_iter(text) {
            if m.start() == m.end() {
                continue; // uma expressão que casa o vazio não marca nada
            }
            // O `\b` do regex não vale entre letra e `_` como nos editores: confere de novo.
            if self.whole {
                let before = text[..m.start()].chars().next_back();
                let after = text[m.end()..].chars().next();
                if before.is_some_and(is_word) || after.is_some_and(is_word) {
                    continue;
                }
            }
            let start = to_chars(m.start());
            let end = to_chars(m.end());
            out.push(Found { start, end, byte_start: m.start(), byte_end: m.end() });
        }
        Ok(out)
    }

    /// O texto que entra no lugar de `found` (os grupos e a caixa já aplicados).
    pub fn replacement(&self, text: &str, found: &Found, template: &str) -> String {
        let matched = &text[found.byte_start..found.byte_end];
        let mut out = String::new();
        if self.regex {
            match self.compile().ok().and_then(|re| re.captures_at(text, found.byte_start)) {
                Some(caps) if caps.get(0).is_some_and(|m| m.start() == found.byte_start) => caps.expand(template, &mut out),
                _ => out.push_str(template),
            }
        } else {
            out.push_str(template);
        }
        if self.preserve_case {
            out = match_case(matched, &out);
        }
        out
    }

    /// `text` com todas as ocorrências trocadas; devolve também quantas.
    pub fn replace_all(&self, text: &str, template: &str) -> Result<(String, usize), String> {
        let found = self.find_all(text)?;
        let mut out = String::with_capacity(text.len());
        let mut last = 0;
        for f in &found {
            out.push_str(&text[last..f.byte_start]);
            out.push_str(&self.replacement(text, f, template));
            last = f.byte_end;
        }
        out.push_str(&text[last..]);
        Ok((out, found.len()))
    }
}

/// `replacement` com a caixa de `matched`: tudo maiúsculo, tudo minúsculo ou só a primeira.
pub fn match_case(matched: &str, replacement: &str) -> String {
    let letters: Vec<char> = matched.chars().filter(|c| c.is_alphabetic()).collect();
    if letters.is_empty() {
        return replacement.to_string();
    }
    if letters.iter().all(|c| c.is_uppercase()) && letters.len() > 1 {
        return replacement.to_uppercase();
    }
    if letters.iter().all(|c| c.is_lowercase()) {
        return replacement.to_lowercase();
    }
    if letters[0].is_uppercase() {
        let mut chars = replacement.chars();
        return chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default();
    }
    replacement.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(query: &str) -> SearchOptions {
        SearchOptions::new(query)
    }

    #[test]
    fn plain_search_respects_case_and_whole_words() {
        let text = "Valor valor valores _valor ação Ação";
        assert_eq!(opts("valor").find_all(text).unwrap().len(), 4);
        assert_eq!(SearchOptions { case: true, ..opts("valor") }.find_all(text).unwrap().len(), 3);
        assert_eq!(SearchOptions { whole: true, ..opts("valor") }.find_all(text).unwrap().len(), 2);
        // Caracteres, não bytes (o "ç" e o "ã" têm dois bytes).
        let f = opts("ação").find_all(text).unwrap();
        assert_eq!((f[1].start, f[1].end), (32, 36));
        // O texto exato não é expressão: o ponto é ponto.
        assert_eq!(opts("a.b").find_all("axb a.b").unwrap().len(), 1);
    }

    #[test]
    fn regex_with_groups_and_errors() {
        let o = SearchOptions { regex: true, ..opts(r"(\w+)=(\d+)") };
        let (out, n) = o.replace_all("a=1, bb=22", "$2:$1").unwrap();
        assert_eq!((out.as_str(), n), ("1:a, 22:bb", 2));
        assert!(SearchOptions { regex: true, ..opts("(") }.find_all("x").is_err());
    }

    #[test]
    fn preserving_case() {
        let o = SearchOptions { preserve_case: true, ..opts("nome") };
        let (out, _) = o.replace_all("nome Nome NOME", "valor").unwrap();
        assert_eq!(out, "valor Valor VALOR");
    }
}
