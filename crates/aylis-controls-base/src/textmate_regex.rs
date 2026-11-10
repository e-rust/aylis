//! O motor das expressões das gramáticas: o Oniguruma (o mesmo do VS Code, para quem as
//! gramáticas são escritas; muito mais rápido nas expressões enormes do C++) e, no wasm, onde
//! não há como compilar C, o fancy-regex no modo Oniguruma.
//!
//! O `\G` vale só onde a regra anterior terminou (`g_here`): no Oniguruma, uma segunda versão
//! da expressão com o `\G` trocado por algo que nunca casa (como o vscode-textmate).

/// Os grupos de um achado: (começo, fim) em bytes.
pub type Found = Vec<Option<(usize, usize)>>;

/// Escapa um texto para entrar literal numa expressão (os `\1` do `end` trocados pelo texto).
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if "\\.+*?()|[]{}^$".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Tem `\G` de verdade (não `\\G`).
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn has_g(source: &str) -> bool {
    let b = source.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b'\\' {
            if b[i + 1] == b'G' {
                return true;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    false
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::Found;
    use onig::{Region, RegexOptions, SearchOptions, Syntax};

    pub struct Regex {
        source: String,
        re: onig::Regex,
        /// Com o `\G` desligado (para quando a busca não começa onde a anterior terminou).
        without_g: Option<onig::Regex>,
    }

    fn compile(source: &str) -> Result<onig::Regex, String> {
        onig::Regex::with_options(source, RegexOptions::REGEX_OPTION_CAPTURE_GROUP, Syntax::default()).map_err(|e| e.description().to_string())
    }

    /// Troca cada `\G` por `\x{FFFF}` (um caractere que não aparece no texto).
    fn disable_g(source: &str) -> String {
        let mut out = String::with_capacity(source.len());
        let mut chars = source.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('G') => out.push_str("\\x{FFFF}"),
                    Some(n) => {
                        out.push('\\');
                        out.push(n);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    impl Regex {
        pub fn new(source: &str) -> Result<Self, String> {
            let re = compile(source)?;
            let without_g = if super::has_g(source) { Some(compile(&disable_g(source))?) } else { None };
            Ok(Self { source: source.to_string(), re, without_g })
        }

        pub fn as_str(&self) -> &str {
            &self.source
        }

        pub fn captures(&self, line: &str, pos: usize, g_here: bool) -> Result<Option<Found>, String> {
            let re = match &self.without_g {
                Some(other) if !g_here => other,
                _ => &self.re,
            };
            let mut region = Region::new();
            let found = re.search_with_options(line, pos, line.len(), SearchOptions::SEARCH_OPTION_NONE, Some(&mut region));
            Ok(found.map(|_| (0..region.len()).map(|i| region.pos(i)).collect()))
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::Found;
    use fancy_regex::{RegexBuilder, RegexInput};

    pub struct Regex(fancy_regex::Regex);

    impl Regex {
        pub fn new(source: &str) -> Result<Self, String> {
            RegexBuilder::new(source).oniguruma_mode(true).multi_line(true).allow_input_assertion_overrides(true).build().map(Self).map_err(|e| e.to_string())
        }

        pub fn as_str(&self) -> &str {
            self.0.as_str()
        }

        pub fn captures(&self, line: &str, pos: usize, g_here: bool) -> Result<Option<Found>, String> {
            let input = RegexInput::new(line).from_pos(pos).continue_from_previous_match_end(g_here);
            let c = self.0.captures_input(input).map_err(|e| e.to_string())?;
            Ok(c.map(|c| (0..c.len()).map(|i| c.get(i).map(|m| (m.start(), m.end()))).collect()))
        }
    }
}

pub use imp::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_only_matches_where_the_previous_match_ended() {
        let re = Regex::new(r"\G(ab)").unwrap();
        let line = "xxab\n";
        assert_eq!(re.captures(line, 2, true).unwrap().map(|f| f[1]), Some(Some((2, 4))));
        assert_eq!(re.captures(line, 2, false).unwrap(), None);
        assert_eq!(re.captures(line, 0, true).unwrap(), None);
        // `\\G` é uma barra e um G, não o `\G`.
        assert!(!has_g(r"\\G"));
        assert!(Regex::new(r"\\G").unwrap().captures("a\\G\n", 0, false).unwrap().is_some());
    }

    #[test]
    fn dollar_matches_before_the_newline_and_lookbehind_works() {
        let re = Regex::new(r"(?<=\s)fim$").unwrap();
        assert_eq!(re.captures("a fim\n", 0, false).unwrap().map(|f| f[0]), Some(Some((2, 5))));
        let re = Regex::new(&format!("{}x", escape("a.b"))).unwrap();
        assert!(re.captures("a.bx\n", 0, false).unwrap().is_some());
        assert!(re.captures("aZbx\n", 0, false).unwrap().is_none());
    }
}
