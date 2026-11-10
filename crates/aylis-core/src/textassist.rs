//! O motor de texto: correção ortográfica e sugestões para completar palavras, usado pelos
//! campos de texto (`spellcheck`, `autocomplete`). A plataforma instala o motor nativo (no
//! Windows, o corretor e a previsão de texto do sistema) em `Services::text_assist`; o app
//! troca por outro motor (Hunspell, LanguageTool, uma lista de termos do negócio, um modelo
//! de IA) implementando [`TextAssist`] e pondo no lugar:
//!
//! ```ignore
//! ui.services_mut().text_assist = Some(Box::new(MeuMotor::new()));
//! ```
//!
//! [`WordList`] é um motor pronto com uma lista de palavras; [`Chain`] junta dois motores
//! (o do app primeiro, o nativo depois).

/// Uma palavra errada no texto: o trecho em bytes (`start..end`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Misspelling {
    pub start: usize,
    pub end: usize,
}

/// Correção e sugestão de texto. `language` é a marca do idioma (`"pt-BR"`, `"en-US"`);
/// vazio: o do sistema.
pub trait TextAssist {
    /// As palavras erradas do texto.
    fn check(&mut self, text: &str, language: &str) -> Vec<Misspelling>;
    /// Sugestões de correção para uma palavra errada (o menu do clique direito).
    fn suggest(&mut self, word: &str, language: &str) -> Vec<String>;
    /// Palavras que completam o que está sendo digitado. `before` é o texto até o cursor
    /// (a palavra incompleta está no fim dele; o resto é contexto para prever melhor). As
    /// sugestões são a palavra inteira (`"compu"` → `"computador"`).
    fn complete(&mut self, before: &str, language: &str) -> Vec<String>;
    /// "Adicionar ao dicionário": a palavra passa a valer.
    fn add_word(&mut self, word: &str, language: &str) {
        let _ = (word, language);
    }
}

/// A palavra incompleta no fim de `before` (letras, dígitos, `_`, `-` e `'`).
pub fn word_before(before: &str) -> &str {
    let start = before
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_word_char(*c))
        .last()
        .map_or(before.len(), |(i, _)| i);
    &before[start..]
}

/// Faz parte de uma palavra (para separar o texto em palavras).
pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\'' || c == '-'
}

/// As palavras do texto, com o trecho de cada uma em bytes.
pub fn words(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in text.char_indices() {
        match (is_word_char(c), start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                out.push((s, i));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        out.push((s, text.len()));
    }
    // Hífen ou apóstrofo nas pontas não é palavra ("-", "'oi'").
    out.into_iter()
        .map(|(s, e)| {
            let w = &text[s..e];
            let lead = w.len() - w.trim_start_matches(['-', '\'']).len();
            let tail = w.len() - w.trim_end_matches(['-', '\'']).len();
            (s + lead, e - tail)
        })
        .filter(|(s, e)| s < e)
        .collect()
}

/// Um motor com uma lista de palavras: completa pelo começo (as mais usadas primeiro) e
/// marca como erradas as palavras fora da lista. Serve para termos do negócio (produtos,
/// clientes), para testes e como base de um motor próprio.
#[derive(Clone, Debug, Default)]
pub struct WordList {
    /// (palavra, peso): o peso ordena as sugestões (maior primeiro).
    words: Vec<(String, u32)>,
    /// Só completa, sem marcar erros (uma lista de termos não é um dicionário inteiro).
    pub complete_only: bool,
}

impl WordList {
    pub fn new(words: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut list = Self::default();
        for w in words {
            list.add(&w.into(), 1);
        }
        list
    }

    /// Acrescenta (ou reforça) uma palavra.
    pub fn add(&mut self, word: &str, weight: u32) {
        match self.words.iter_mut().find(|(w, _)| w.eq_ignore_ascii_case(word)) {
            Some((_, n)) => *n += weight,
            None => self.words.push((word.to_string(), weight)),
        }
    }

    fn contains(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.words.iter().any(|(w, _)| w.to_lowercase() == lower) || word.chars().all(|c| !c.is_alphabetic())
    }
}

impl TextAssist for WordList {
    fn check(&mut self, text: &str, _language: &str) -> Vec<Misspelling> {
        if self.complete_only {
            return Vec::new();
        }
        words(text).into_iter().filter(|&(s, e)| !self.contains(&text[s..e])).map(|(start, end)| Misspelling { start, end }).collect()
    }

    fn suggest(&mut self, word: &str, _language: &str) -> Vec<String> {
        // As mais parecidas: mesmo começo, depois menor distância.
        let lower = word.to_lowercase();
        let mut scored: Vec<(usize, &String)> = self.words.iter().map(|(w, _)| (distance(&lower, &w.to_lowercase()), w)).filter(|(d, _)| *d <= 2.max(word.chars().count() / 3)).collect();
        scored.sort_by_key(|(d, _)| *d);
        scored.into_iter().take(5).map(|(_, w)| w.clone()).collect()
    }

    fn complete(&mut self, before: &str, _language: &str) -> Vec<String> {
        let prefix = word_before(before);
        if prefix.chars().count() < 2 {
            return Vec::new();
        }
        let lower = prefix.to_lowercase();
        let mut found: Vec<&(String, u32)> = self.words.iter().filter(|(w, _)| w.len() > prefix.len() && w.to_lowercase().starts_with(&lower)).collect();
        found.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        found.into_iter().take(5).map(|(w, _)| keep_case(prefix, w)).collect()
    }

    fn add_word(&mut self, word: &str, _language: &str) {
        self.add(word, 1);
    }
}

/// A sugestão com a caixa do que foi digitado (`"Compu"` → `"Computador"`).
fn keep_case(typed: &str, word: &str) -> String {
    let upper_first = typed.chars().next().is_some_and(char::is_uppercase);
    let all_upper = typed.chars().count() > 1 && typed.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase);
    if all_upper {
        word.to_uppercase()
    } else if upper_first {
        let mut chars = word.chars();
        chars.next().map(|f| f.to_uppercase().chain(chars).collect()).unwrap_or_default()
    } else {
        word.to_string()
    }
}

/// Distância de edição (Levenshtein), para ordenar sugestões.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = row[0];
        row[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let cur = row[j + 1];
            row[j + 1] = (row[j + 1] + 1).min(row[j] + 1).min(prev + usize::from(ca != cb));
            prev = cur;
        }
    }
    row[b.len()]
}

/// Dois motores juntos: as sugestões do primeiro vêm antes; uma palavra é errada só se os
/// dois discordam dela (o primeiro conhece os termos do app, o segundo o idioma).
pub struct Chain {
    pub first: Box<dyn TextAssist>,
    pub second: Box<dyn TextAssist>,
}

impl TextAssist for Chain {
    fn check(&mut self, text: &str, language: &str) -> Vec<Misspelling> {
        let a = self.first.check(text, language);
        let b = self.second.check(text, language);
        a.into_iter().filter(|m| b.contains(m)).collect()
    }

    fn suggest(&mut self, word: &str, language: &str) -> Vec<String> {
        let mut out = self.first.suggest(word, language);
        for s in self.second.suggest(word, language) {
            if !out.contains(&s) {
                out.push(s);
            }
        }
        out
    }

    fn complete(&mut self, before: &str, language: &str) -> Vec<String> {
        let mut out = self.first.complete(before, language);
        for s in self.second.complete(before, language) {
            if !out.contains(&s) {
                out.push(s);
            }
        }
        out
    }

    fn add_word(&mut self, word: &str, language: &str) {
        self.first.add_word(word, language);
    }
}
