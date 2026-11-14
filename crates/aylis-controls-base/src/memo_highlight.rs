//! O realce do `Memo` retomado da linha editada: o estado do começo de cada linha fica
//! guardado; uma edição troca só as linhas dela (as de depois andam junto, com os trechos e
//! os estados de antes) e o realce recomeça ali, até o estado bater de novo com o guardado
//! (dali em diante, o que estava certo continua certo). Num fonte de um milhão de linhas,
//! uma tecla realça poucas linhas, em vez de recomeçar do início.

use std::rc::Rc;

use crate::syntax::{Carry, LineHighlighter, Span};
use crate::textmate::{Grammar, LineState};

/// As linhas realçadas e os estados, num estado `S` qualquer (o da gramática TextMate ou o do
/// realce embutido).
pub(super) struct Lines<S> {
    /// Os trechos de cada linha (vazio: ainda não feita; aparece na cor do texto).
    spans: Rc<Vec<Vec<Span>>>,
    /// O estado no começo de cada linha (vale nas `valid` primeiras e nas faixas de `kept`).
    starts: Vec<S>,
    /// Quantas linhas, de cima, já estão certas.
    valid: usize,
    /// O estado no começo da linha `valid`.
    next: S,
    /// Faixas depois de `valid` feitas numa sequência só, antes de uma edição: `[de, até)` e o
    /// estado no fim. Chegando numa delas com o mesmo estado guardado, pula até o fim.
    kept: Vec<(usize, usize, S)>,
}

impl<S: Clone + Default + PartialEq> Lines<S> {
    fn new(lines: usize) -> Self {
        Self { spans: Rc::new(vec![Vec::new(); lines]), starts: vec![S::default(); lines], valid: 0, next: S::default(), kept: Vec::new() }
    }

    fn lines(&self) -> usize {
        self.starts.len()
    }

    /// As linhas `[a, b)` viraram `count` linhas novas (as de depois andam junto).
    fn edit(&mut self, a: usize, b: usize, count: usize) {
        let b = b.min(self.lines());
        let a = a.min(b);
        let delta = count as isize - (b - a) as isize;
        let at = |i: usize| (i as isize + delta) as usize;
        let mut kept = Vec::with_capacity(self.kept.len() + 1);
        // O que estava certo depois do trecho vira uma faixa; o certo vai até a linha `a`.
        if a < self.valid {
            if b < self.valid {
                kept.push((b, self.valid, self.next.clone()));
            }
            self.next = self.starts[a].clone();
            self.valid = a;
        }
        for (f, t, end) in std::mem::take(&mut self.kept) {
            if t <= a || f >= b {
                kept.push((f, t, end));
                continue;
            }
            // A faixa cruza o trecho: fica o pedaço de antes (termina no começo de `a`) e o
            // de depois.
            if f < a {
                kept.push((f, a, self.starts[a].clone()));
            }
            if b < t {
                kept.push((b, t, end));
            }
        }
        for (f, t, _) in &mut kept {
            if *f >= b {
                *f = at(*f);
                *t = at(*t);
            }
        }
        kept.sort_by_key(|(f, _, _)| *f);
        self.kept = kept;
        self.starts.splice(a..b, std::iter::repeat_n(S::default(), count));
        Rc::make_mut(&mut self.spans).splice(a..b, std::iter::repeat_n(Vec::new(), count));
    }

    /// Anda o realce a partir de `valid` até o tempo acabar (`None`: até o fim). `line(i)` é
    /// o texto da linha `i`; `tokenize` realça uma linha a partir de um estado.
    fn resume<'t>(&mut self, line: impl Fn(usize) -> &'t str, mut tokenize: impl FnMut(&str, &mut S) -> Vec<Span>, budget: Option<std::time::Duration>) -> bool {
        let n = self.lines();
        let deadline = budget.map(|b| web_time::Instant::now() + b);
        let spans = Rc::make_mut(&mut self.spans);
        let mut i = self.valid;
        let mut state = self.next.clone();
        let mut r = 0;
        let mut count = 0u32;
        while i < n {
            while r < self.kept.len() && self.kept[r].1 <= i {
                r += 1;
            }
            // Numa faixa feita antes, com o mesmo estado: dali até o fim dela, é igual.
            if let Some((f, t, end)) = self.kept.get(r)
                && *f <= i
                && self.starts[i] == state
            {
                state = end.clone();
                i = *t;
                r += 1;
                continue;
            }
            // Olha o relógio de tempos em tempos (ele custa mais que uma linha curta).
            count += 1;
            if count.is_multiple_of(32) && deadline.is_some_and(|d| web_time::Instant::now() >= d) {
                break;
            }
            self.starts[i] = state.clone();
            spans[i] = tokenize(line(i), &mut state);
            i += 1;
        }
        self.valid = i;
        self.next = state;
        // As faixas já passadas saem; a que ficou no meio começa onde parou.
        self.kept.retain(|(_, t, _)| *t > i);
        for (f, _, _) in &mut self.kept {
            *f = (*f).max(i);
        }
        i >= n
    }
}

/// O realce guardado de um texto: pela gramática TextMate ou pelo realce embutido.
#[derive(Default)]
pub(super) enum Highlight {
    #[default]
    None,
    Grammar { language: String, grammar: Rc<Grammar>, lines: Lines<LineState> },
    Builtin { language: String, highlighter: LineHighlighter, lines: Lines<Carry> },
}

impl Highlight {
    /// Começa do zero o realce de `language` num texto de `lines` linhas.
    pub(super) fn start(language: &str, lines: usize) -> Self {
        match crate::textmate::grammar_for(language) {
            Some(grammar) => Self::Grammar { language: language.to_string(), grammar, lines: Lines::new(lines) },
            None => Self::Builtin { language: language.to_string(), highlighter: LineHighlighter::new(language), lines: Lines::new(lines) },
        }
    }

    /// É o realce de `language` (pela gramática, se há uma registrada agora, senão o embutido).
    pub(super) fn is_for(&self, language: &str) -> bool {
        match self {
            Self::None => false,
            Self::Grammar { language: l, grammar, .. } => l == language && crate::textmate::grammar_for(language).is_some_and(|g| Rc::ptr_eq(&g, grammar)),
            Self::Builtin { language: l, .. } => l == language && crate::textmate::grammar_for(language).is_none(),
        }
    }

    /// Quantas linhas o realce tem.
    pub(super) fn lines(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Grammar { lines, .. } => lines.lines(),
            Self::Builtin { lines, .. } => lines.lines(),
        }
    }

    /// As linhas `[a, b)` do texto viraram `count` linhas.
    pub(super) fn edit(&mut self, a: usize, b: usize, count: usize) {
        match self {
            Self::None => {}
            Self::Grammar { lines, .. } => lines.edit(a, b, count),
            Self::Builtin { lines, .. } => lines.edit(a, b, count),
        }
    }

    /// Anda o realce (veja [`Lines::resume`]); devolve os trechos e se chegou ao fim.
    pub(super) fn resume<'t>(&mut self, line: impl Fn(usize) -> &'t str, budget: Option<std::time::Duration>) -> (Rc<Vec<Vec<Span>>>, bool) {
        match self {
            Self::None => (Rc::new(Vec::new()), true),
            Self::Grammar { grammar, lines, .. } => {
                let done = lines.resume(line, |text, state| grammar.tokenize_line(text, state), budget);
                (lines.spans.clone(), done)
            }
            Self::Builtin { highlighter, lines, .. } => {
                let done = lines.resume(line, |text, state| highlighter.line(text, state), budget);
                (lines.spans.clone(), done)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::TokenKind;

    /// Realça `text` inteiro de uma vez (a referência).
    fn whole(language: &str, text: &str) -> Vec<Vec<Span>> {
        let lines: Vec<&str> = text.split('\n').collect();
        let mut h = Highlight::start(language, lines.len());
        h.resume(|i| lines[i], None).0.as_ref().clone()
    }

    /// Troca as linhas `[a, b)` de `before` por `new` e realça só a partir da edição.
    fn edited(language: &str, before: &str, a: usize, b: usize, new: &[&str]) -> (String, Vec<Vec<Span>>, usize) {
        let old: Vec<&str> = before.split('\n').collect();
        let mut h = Highlight::start(language, old.len());
        h.resume(|i| old[i], None);
        let mut lines: Vec<&str> = old.clone();
        lines.splice(a..b, new.iter().copied());
        h.edit(a, b, new.len());
        // Conta quantas linhas a retomada realça.
        let count = std::cell::Cell::new(0);
        let (spans, done) = h.resume(
            |i| {
                count.set(count.get() + 1);
                lines[i]
            },
            None,
        );
        assert!(done);
        (lines.join("\n"), spans.as_ref().clone(), count.get())
    }

    /// Uma gramática TextMate mínima (comentário de bloco, texto e palavra-chave).
    const GRAMMAR: &str = r#"{"scopeName": "source.hltest", "patterns": [
        {"name": "comment.block.hltest", "begin": "/\\*", "end": "\\*/"},
        {"name": "string.quoted.hltest", "begin": "\"", "end": "\""},
        {"name": "keyword.hltest", "match": "\\b(let|fn)\\b"}]}"#;

    /// O realce embutido e o por gramática.
    fn languages() -> [&'static str; 2] {
        crate::textmate::register_grammar("hltest", GRAMMAR).unwrap();
        ["rust", "hltest"]
    }

    #[test]
    fn editing_a_line_retokenizes_only_until_the_state_matches() {
        let text = (0..2000).map(|i| format!("let x{i} = {i}; // linha")).collect::<Vec<_>>().join("\n");
        for language in languages() {
            let (after, spans, count) = edited(language, &text, 1000, 1001, &["let novo = \"texto\";"]);
            assert_eq!(spans, whole(language, &after), "{language}");
            assert!(count <= 3, "{language}: realçou {count} linhas para uma edição");
        }
    }

    #[test]
    fn opening_a_block_comment_recolors_the_rest_and_closing_it_restores() {
        let text = (0..50).map(|i| format!("fn f{i}() {{}}")).collect::<Vec<_>>().join("\n");
        for language in languages() {
            // Abre um comentário de bloco no meio: tudo depois vira comentário.
            let (after, spans, _) = edited(language, &text, 20, 21, &["/* aberto"]);
            assert_eq!(spans, whole(language, &after), "{language}");
            assert!(spans[40].iter().all(|s| s.kind == TokenKind::Comment), "{language}: {:?}", spans[40]);
            // Linhas inseridas e apagadas no meio de um texto já realçado.
            let (after, spans, _) = edited(language, &text, 10, 13, &["// a", "// b", "// c", "// d", "// e"]);
            assert_eq!(spans, whole(language, &after), "{language}");
            let (after, spans, _) = edited(language, &text, 10, 30, &["x"]);
            assert_eq!(spans, whole(language, &after), "{language}");
        }
    }

    #[test]
    fn several_edits_before_resuming_and_a_partial_run() {
        for language in languages() {
            several_edits(language);
        }
    }

    fn several_edits(language: &str) {
        let text = (0..3000).map(|i| if i % 7 == 0 { "/* começa".to_string() } else if i % 7 == 3 { "termina */ fn x() {}".to_string() } else { format!("let a{i} = \"s\";") }).collect::<Vec<_>>().join("\n");
        let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
        let mut h = Highlight::start(language, lines.len());
        h.resume(|i| lines[i].as_str(), None);
        // Várias edições seguidas, sem realçar entre elas, e depois uma retomada com pouco tempo
        // (para no meio) seguida de outra edição.
        for (a, b, new) in [(100usize, 101usize, vec!["/* outro"]), (2000, 2002, vec!["a", "b", "c"]), (50, 50 + 1, vec![]), (10, 11, vec!["x */"])] {
            let new: Vec<String> = new.into_iter().map(str::to_string).collect();
            let count = new.len();
            lines.splice(a..b, new);
            h.edit(a, b, count);
        }
        h.resume(|i| lines[i].as_str(), Some(std::time::Duration::ZERO));
        lines.splice(1500..1501, ["/* mais".to_string()]);
        h.edit(1500, 1501, 1);
        let (spans, done) = h.resume(|i| lines[i].as_str(), None);
        assert!(done);
        assert_eq!(*spans, whole(language, &lines.join("\n")), "{language}");
    }
}
