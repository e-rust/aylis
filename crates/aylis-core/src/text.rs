//! Disposição de texto em linhas: quebra automática e reticências ("…"), iguais em todos
//! os renderizadores (Skia, navegador).

/// O que fazer quando o texto não cabe na largura.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Trimming {
    /// Não corta: o texto passa da borda (e o controle recorta).
    None,
    /// Corta no caractere e põe "…".
    #[default]
    Character,
    /// Corta na palavra inteira e põe "…".
    Word,
}

impl crate::prop::PropEnum for Trimming {
    const NAMES: &'static [&'static str] = &["none", "character", "word"];

    fn from_index(index: usize) -> Option<Self> {
        [Trimming::None, Trimming::Character, Trimming::Word].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Maiúsculas ou minúsculas na hora de desenhar (o `text-transform` do CSS): o texto
/// guardado não muda.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextCase {
    #[default]
    None,
    Upper,
    Lower,
}

impl TextCase {
    pub fn apply(self, text: &str) -> String {
        match self {
            TextCase::None => text.to_string(),
            TextCase::Upper => text.to_uppercase(),
            TextCase::Lower => text.to_lowercase(),
        }
    }
}

impl crate::prop::PropEnum for TextCase {
    const NAMES: &'static [&'static str] = &["none", "upper", "lower"];

    fn from_index(index: usize) -> Option<Self> {
        [TextCase::None, TextCase::Upper, TextCase::Lower].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

const ELLIPSIS: &str = "…";

/// Corta `text` para caber em `width` com "…" no fim (por caractere ou por palavra).
/// Sem corte (`Trimming::None`) ou se já couber, devolve igual.
pub fn trim_line(text: &str, width: f32, trimming: Trimming, measure: &mut dyn FnMut(&str) -> f32) -> String {
    if trimming == Trimming::None || measure(text) <= width {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    if trimming == Trimming::Word {
        // O maior número de palavras inteiras que cabe junto com "…".
        let mut best: Option<String> = None;
        let mut end = 0;
        while end < chars.len() {
            // fim da próxima palavra
            while end < chars.len() && chars[end].is_whitespace() {
                end += 1;
            }
            while end < chars.len() && !chars[end].is_whitespace() {
                end += 1;
            }
            let candidate: String = chars[..end].iter().collect::<String>().trim_end().to_string() + ELLIPSIS;
            if measure(&candidate) <= width {
                best = Some(candidate);
            } else {
                break;
            }
        }
        if let Some(b) = best {
            return b;
        }
        // Nem a primeira palavra cabe: corta no caractere.
    }
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        let candidate = chars[..mid].iter().collect::<String>() + ELLIPSIS;
        if measure(&candidate) <= width { lo = mid } else { hi = mid - 1 }
    }
    chars[..lo].iter().collect::<String>().trim_end().to_string() + ELLIPSIS
}

/// Quebra uma linha (sem `\n`) em pedaços que cabem em `width`: nos espaços e, se uma
/// palavra sozinha não couber, no caractere. Devolve as posições (em caracteres) de
/// início e fim de cada pedaço; o espaço da quebra fica no fim do pedaço de cima.
pub fn wrap_ranges(line: &str, width: f32, measure: &mut dyn FnMut(&str) -> f32) -> Vec<(usize, usize)> {
    let chars: Vec<char> = line.chars().collect();
    let text = |a: usize, b: usize| chars[a..b].iter().collect::<String>();
    let mut out = Vec::new();
    let mut start = 0;
    if chars.is_empty() {
        return vec![(0, 0)];
    }
    while start < chars.len() {
        // Avança palavra por palavra enquanto couber.
        let mut end = start;
        let mut fits_end = start;
        loop {
            let mut next = end;
            while next < chars.len() && chars[next].is_whitespace() {
                next += 1;
            }
            while next < chars.len() && !chars[next].is_whitespace() {
                next += 1;
            }
            if next == end {
                break;
            }
            if measure(text(start, next).trim_end()) <= width {
                fits_end = next;
                end = next;
                if next >= chars.len() {
                    break;
                }
            } else {
                break;
            }
        }
        if fits_end == start {
            // Uma palavra maior que a largura: corta no caractere (pelo menos um).
            let (mut lo, mut hi) = (start + 1, chars.len());
            while lo < hi {
                let mid = (lo + hi).div_ceil(2);
                if measure(&text(start, mid)) <= width { lo = mid } else { hi = mid - 1 }
            }
            fits_end = lo.max(start + 1);
        } else {
            // Os espaços depois da última palavra ficam nesta linha.
            while fits_end < chars.len() && chars[fits_end].is_whitespace() {
                fits_end += 1;
            }
        }
        out.push((start, fits_end));
        start = fits_end;
    }
    out
}

/// As linhas a desenhar de `text` numa área de `width` × até `max_lines` linhas:
/// respeita `\n`, quebra com `wrap` e corta com "…" o que não couber (com `trimming`).
pub fn layout_lines(
    text: &str,
    width: f32,
    max_lines: usize,
    wrap: bool,
    trimming: Trimming,
    measure: &mut dyn FnMut(&str) -> f32,
) -> Vec<String> {
    layout_lines_marked(text, width, max_lines, wrap, trimming, measure).into_iter().map(|(line, _)| line).collect()
}

/// Como [`layout_lines`], dizendo também se cada linha fecha um parágrafo (vem antes de
/// um `\n`, é a última do texto ou terminou em "…"): o justificado deixa essas à esquerda.
pub fn layout_lines_marked(
    text: &str,
    width: f32,
    max_lines: usize,
    wrap: bool,
    trimming: Trimming,
    measure: &mut dyn FnMut(&str) -> f32,
) -> Vec<(String, bool)> {
    let max_lines = max_lines.max(1);
    let mut lines: Vec<(String, bool)> = Vec::new();
    let mut more = false;
    'outer: for line in text.split('\n') {
        let pieces: Vec<String> = if wrap {
            let chars: Vec<char> = line.chars().collect();
            wrap_ranges(line, width, measure).into_iter().map(|(a, b)| chars[a..b].iter().collect::<String>().trim_end().to_string()).collect()
        } else {
            vec![line.to_string()]
        };
        let count = pieces.len();
        for (i, piece) in pieces.into_iter().enumerate() {
            if lines.len() == max_lines {
                more = true;
                break 'outer;
            }
            lines.push((piece, i + 1 == count));
        }
    }
    if !wrap {
        for (l, _) in &mut lines {
            *l = trim_line(l, width, trimming, measure);
        }
    }
    // Sobrou texto além da última linha: ela termina em "…".
    if more && trimming != Trimming::None {
        if let Some((last, end)) = lines.last_mut() {
            *end = true;
            let base = last.trim_end().to_string();
            let mut candidate = base.clone() + ELLIPSIS;
            if measure(&candidate) > width {
                candidate = trim_line(&(base + "  "), width, trimming, measure);
                if !candidate.ends_with(ELLIPSIS) {
                    candidate.push_str(ELLIPSIS);
                }
            }
            *last = candidate;
        }
    }
    lines
}

/// Uma linha justificada: as palavras e o `x` de cada uma (a partir da esquerda), com o
/// espaço que sobra em `width` dividido entre elas. `None` quando a linha fica como está
/// (à esquerda): uma palavra só ou mais larga que a área. Os espaços do começo (o recuo)
/// ficam antes da primeira palavra.
pub fn justify_line(line: &str, width: f32, measure: &mut dyn FnMut(&str) -> f32) -> Option<Vec<(f32, String)>> {
    let body = line.trim_end();
    let indent_len = body.len() - body.trim_start().len();
    let indent = if indent_len > 0 { measure(&body[..indent_len]) } else { 0.0 };
    let words: Vec<&str> = body.split_whitespace().collect();
    if words.len() < 2 {
        return None;
    }
    let widths: Vec<f32> = words.iter().map(|w| measure(w)).collect();
    let free = width - indent - widths.iter().sum::<f32>();
    if free <= 0.0 {
        return None;
    }
    let gap = free / (words.len() - 1) as f32;
    let mut x = indent;
    let mut out = Vec::with_capacity(words.len());
    for (word, w) in words.into_iter().zip(widths) {
        out.push((x, word.to_string()));
        x += w + gap;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cada caractere mede 1.
    fn m(s: &str) -> f32 {
        s.chars().count() as f32
    }

    #[test]
    fn trims_by_character_and_by_word() {
        assert_eq!(trim_line("abc", 5.0, Trimming::Character, &mut m), "abc");
        assert_eq!(trim_line("abcdefgh", 5.0, Trimming::Character, &mut m), "abcd…");
        assert_eq!(trim_line("um dois tres", 9.0, Trimming::Word, &mut m), "um dois…");
        assert_eq!(trim_line("um dois tres", 9.0, Trimming::Character, &mut m), "um dois…");
        // nem a primeira palavra cabe: volta para o caractere
        assert_eq!(trim_line("paralelepipedo", 6.0, Trimming::Word, &mut m), "paral…");
        assert_eq!(trim_line("abcdefgh", 5.0, Trimming::None, &mut m), "abcdefgh");
    }

    #[test]
    fn wraps_at_spaces_and_inside_long_words() {
        assert_eq!(wrap_ranges("", 5.0, &mut m), [(0, 0)]);
        assert_eq!(wrap_ranges("aa bb cc", 5.0, &mut m), [(0, 6), (6, 8)]);
        assert_eq!(wrap_ranges("abcdefgh", 3.0, &mut m), [(0, 3), (3, 6), (6, 8)]);
    }

    #[test]
    fn lays_out_lines_with_ellipsis_on_the_last_one() {
        let lines = layout_lines("aa bb cc dd\nee", 5.0, 2, true, Trimming::Character, &mut m);
        assert_eq!(lines, ["aa bb", "cc d…"]);
        let lines = layout_lines("aa bb cc dd", 5.0, 2, false, Trimming::Word, &mut m);
        assert_eq!(lines, ["aa…"]);
        let lines = layout_lines("x\ny\nz", 5.0, 2, false, Trimming::None, &mut m);
        assert_eq!(lines, ["x", "y"]);
    }

    #[test]
    fn justifies_lines_except_the_last_of_each_paragraph() {
        let lines = layout_lines_marked("aa b cc dd\nee", 6.0, 10, true, Trimming::None, &mut m);
        assert_eq!(lines, [("aa b".to_string(), false), ("cc dd".to_string(), true), ("ee".to_string(), true)]);
        // "aa b" em 6: sobram 3, divididos no único espaço.
        let words = justify_line("aa b", 6.0, &mut m).unwrap();
        assert_eq!(words, [(0.0, "aa".to_string()), (5.0, "b".to_string())]);
        // Três palavras: o que sobra vai em partes iguais; o recuo fica.
        let words = justify_line("  a b c", 9.0, &mut m).unwrap();
        assert_eq!(words, [(2.0, "a".to_string()), (5.0, "b".to_string()), (8.0, "c".to_string())]);
        assert!(justify_line("palavra", 20.0, &mut m).is_none(), "uma palavra só fica à esquerda");
        assert!(justify_line("aa bb", 3.0, &mut m).is_none(), "não cabe: fica como está");
    }
}
