//! Teclado do form: atalhos ("Ctrl+S"), as teclas que fazem o papel do Tab, os caracteres
//! que cada campo aceita (`allowed_chars`/`blocked_chars`) e a letra do `&` nas legendas
//! (o Alt+letra do Windows, como no Delphi).

use crate::event::{Key, KeyEvent, Modifiers};

/// `"Ctrl+Shift+S"` → modificadores e tecla. `None` se não for um atalho válido.
pub fn parse_shortcut(text: &str) -> Option<(Modifiers, Key)> {
    let mut modifiers = Modifiers::default();
    let mut key = None;
    // "Ctrl++" é o Ctrl com a tecla "+".
    let text = text.trim();
    let (text, plus) = match text.strip_suffix("++") {
        Some(rest) => (rest, true),
        None => (text, text == "+"),
    };
    for part in text.split('+').map(str::trim).filter(|p| !p.is_empty()) {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "shift" => modifiers.shift = true,
            "alt" => modifiers.alt = true,
            "cmd" | "meta" | "win" => modifiers.meta = true,
            p => {
                key = Some(match p {
                    "del" | "delete" => Key::Delete,
                    "ins" | "insert" => Key::Insert,
                    "enter" | "return" => Key::Enter,
                    "esc" | "escape" => Key::Escape,
                    "tab" => Key::Tab,
                    "space" => Key::Space,
                    "backspace" => Key::Backspace,
                    "home" => Key::Home,
                    "end" => Key::End,
                    "pgup" | "pageup" => Key::PageUp,
                    "pgdn" | "pagedown" => Key::PageDown,
                    "up" => Key::Up,
                    "down" => Key::Down,
                    "left" => Key::Left,
                    "right" => Key::Right,
                    f if f.len() > 1 && f.starts_with('f') && f[1..].parse::<u8>().is_ok() => Key::F(f[1..].parse().ok()?),
                    c if c.chars().count() == 1 => Key::Char(c.chars().next()?),
                    _ => return None,
                })
            }
        }
    }
    if plus {
        key = Some(Key::Char('+'));
    }
    key.map(|k| (modifiers, k))
}

/// A tecla bate com o atalho. No macOS o Cmd faz o papel do Ctrl.
pub fn shortcut_matches((modifiers, key): (Modifiers, Key), event: &KeyEvent) -> bool {
    let m = event.modifiers;
    let same_key = match (key, event.key) {
        (Key::Char(a), Key::Char(b)) => a.to_lowercase().eq(b.to_lowercase()),
        (a, b) => a == b,
    };
    same_key && modifiers.ctrl == (m.ctrl || m.meta) && modifiers.shift == m.shift && modifiers.alt == m.alt
}

/// O atalho escrito para mostrar (o `ShortCutToText` do Delphi): `"ctrl+shift+s"` vira
/// `"Ctrl+Shift+S"`, `"del"` vira `"Del"`, no idioma do Aylis (o espaço). Texto que não é atalho
/// volta como veio (o relatório do Tabernáculo, J13: o nome era escrito à mão).
pub fn shortcut_text(text: &str) -> String {
    let Some((m, key)) = parse_shortcut(text) else { return text.to_string() };
    let mut parts: Vec<String> = Vec::new();
    if m.ctrl {
        parts.push("Ctrl".into());
    }
    if m.meta {
        parts.push(if cfg!(target_os = "macos") { "Cmd".into() } else { "Win".into() });
    }
    if m.alt {
        parts.push("Alt".into());
    }
    if m.shift {
        parts.push("Shift".into());
    }
    parts.push(match key {
        Key::Enter => "Enter".into(),
        Key::Tab => "Tab".into(),
        Key::Escape => "Esc".into(),
        Key::Backspace => "Backspace".into(),
        Key::Delete => "Del".into(),
        Key::Insert => "Ins".into(),
        Key::Home => "Home".into(),
        Key::End => "End".into(),
        Key::PageUp => "PgUp".into(),
        Key::PageDown => "PgDn".into(),
        Key::Left => "←".into(),
        Key::Right => "→".into(),
        Key::Up => "↑".into(),
        Key::Down => "↓".into(),
        Key::Space => crate::messages::t("Espaço"),
        Key::F(n) => format!("F{n}"),
        Key::Char(c) => c.to_uppercase().collect(),
        _ => String::new(),
    });
    parts.join("+")
}

/// Uma lista de atalhos separados por `;` (`"Enter;Down"`).
pub fn parse_shortcut_list(text: &str) -> Vec<(Modifiers, Key)> {
    text.split(';').filter_map(parse_shortcut).collect()
}

/// Um conjunto de caracteres como os campos usam: os caracteres soltos e as faixas `a-z`,
/// `0-9`; `\` tira o significado do próximo (`\-` é o hífen, `\\` a barra) e `\s` é o
/// espaço.
pub fn char_set_contains(set: &str, ch: char) -> bool {
    let chars: Vec<char> = set.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let (first, next) = read_set_char(&chars, i);
        // Uma faixa: "a-z" (o "-" no fim do conjunto é o próprio hífen).
        if next + 1 < chars.len() && chars[next] == '-' {
            let (last, after) = read_set_char(&chars, next + 1);
            if (first..=last).contains(&ch) {
                return true;
            }
            i = after;
            continue;
        }
        if first == ch {
            return true;
        }
        i = next;
    }
    false
}

/// O caractere em `i` (com o escape) e a posição seguinte.
fn read_set_char(chars: &[char], i: usize) -> (char, usize) {
    if chars[i] == '\\' && i + 1 < chars.len() {
        let c = match chars[i + 1] {
            's' => ' ',
            't' => '\t',
            other => other,
        };
        (c, i + 2)
    } else {
        (chars[i], i + 1)
    }
}

/// O que um componente diz sobre os caracteres digitados nele (e nos filhos, se é um
/// contêiner): `allowed_chars`, `blocked_chars`, `only_allowed` e `chars_override`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharFilter {
    /// Liberados aqui (vencem um bloqueio de um pai).
    pub allowed: String,
    /// Bloqueados aqui.
    pub blocked: String,
    /// Só os de `allowed` (um campo só de números: `allowed = "0-9"`).
    pub only_allowed: bool,
    /// O contêiner manda nos filhos (sem isso, o filho decide primeiro).
    pub overrides: bool,
    /// Trocas do que o teclado produz (`char_map`): pares "digitado, entregue" em seguida
    /// (`",."` troca a vírgula por ponto; `",.;:"` também o ponto e vírgula por dois-pontos).
    /// Vale ao digitar e ao colar, antes do `char_case` e dos filtros.
    pub map: String,
    /// Maiúsculas ou minúsculas no valor (`char_case`): só o do próprio campo.
    pub case: CharCase,
}

/// Maiúsculas ou minúsculas no valor de um campo de texto (o `CharCase` do Delphi): vale ao
/// digitar, ao colar e quando o código grava o texto, com as regras do Unicode (`ß` → `SS`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CharCase {
    #[default]
    Normal,
    Upper,
    Lower,
}

impl CharCase {
    pub fn apply(self, text: &str) -> String {
        match self {
            CharCase::Normal => text.to_string(),
            CharCase::Upper => text.to_uppercase(),
            CharCase::Lower => text.to_lowercase(),
        }
    }
}

impl crate::prop::PropEnum for CharCase {
    const NAMES: &'static [&'static str] = &["normal", "upper", "lower"];

    fn from_index(index: usize) -> Option<Self> {
        [CharCase::Normal, CharCase::Upper, CharCase::Lower].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// O caractere que `ch` vira pelo `char_map` de um componente (`None` se não troca).
fn map_lookup(map: &str, ch: char) -> Option<char> {
    let chars: Vec<char> = map.chars().collect();
    chars.chunks_exact(2).find(|pair| pair[0] == ch).map(|pair| pair[1])
}

/// Troca o caractere pela cadeia de `char_map`, do campo até o form, com a mesma regra dos
/// filtros: o contêiner com `chars_override` manda; senão, o mais perto do campo.
pub fn map_char(chain: &[CharFilter], ch: char) -> char {
    if let Some(to) = chain.iter().rev().filter(|f| f.overrides).find_map(|f| map_lookup(&f.map, ch)) {
        return to;
    }
    chain.iter().find_map(|f| map_lookup(&f.map, ch)).unwrap_or(ch)
}

impl CharFilter {
    /// `Some(true)` libera, `Some(false)` bloqueia, `None` não diz nada sobre o caractere.
    pub fn opinion(&self, ch: char) -> Option<bool> {
        if char_set_contains(&self.blocked, ch) {
            Some(false)
        } else if char_set_contains(&self.allowed, ch) {
            Some(true)
        } else if self.only_allowed {
            Some(false)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.allowed.is_empty() && self.blocked.is_empty() && !self.only_allowed && self.map.is_empty() && self.case == CharCase::Normal
    }
}

/// Decide um caractere pela cadeia de filtros, do campo até o form: o contêiner com
/// `overrides` que tem opinião manda (o mais de fora); senão, o primeiro com opinião, a
/// partir do campo. Sem opinião, passa. Controles (Enter, Tab...) sempre passam.
pub fn char_allowed(chain: &[CharFilter], ch: char) -> bool {
    if ch.is_control() {
        return true;
    }
    if let Some(verdict) = chain.iter().rev().filter(|f| f.overrides).find_map(|f| f.opinion(ch)) {
        return verdict;
    }
    chain.iter().find_map(|f| f.opinion(ch)).unwrap_or(true)
}

/// A letra do `&` numa legenda (`"&Salvar"` → `s`); `&&` é o próprio `&`.
pub fn mnemonic(text: &str) -> Option<char> {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            match chars.next() {
                Some('&') => {}
                Some(m) if !m.is_whitespace() => return m.to_lowercase().next(),
                _ => return None,
            }
        }
    }
    None
}

/// A legenda sem os `&` (o `&&` vira `&`) e a posição (em caracteres) da letra sublinhada.
pub fn strip_mnemonic(text: &str) -> (String, Option<usize>) {
    if !text.contains('&') {
        return (text.to_string(), None);
    }
    let mut out = String::new();
    let mut at = None;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            match chars.peek() {
                Some('&') => {
                    chars.next();
                    out.push('&');
                }
                Some(m) if !m.is_whitespace() && at.is_none() => at = Some(out.chars().count()),
                _ => {}
            }
            continue;
        }
        out.push(c);
    }
    (out, at)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcuts_parse_and_match() {
        let (m, k) = parse_shortcut("Shift+Enter").unwrap();
        assert!(m.shift && !m.ctrl && k == Key::Enter);
        assert_eq!(parse_shortcut("Ctrl++").map(|(_, k)| k), Some(Key::Char('+')));
        assert_eq!(parse_shortcut_list("Enter; Down").len(), 2);
        let event = KeyEvent { key: Key::Char('S'), modifiers: Modifiers { ctrl: true, ..Default::default() }, repeat: false };
        assert!(shortcut_matches(parse_shortcut("Ctrl+S").unwrap(), &event));
    }

    #[test]
    fn char_sets_take_ranges_and_escapes() {
        assert!(char_set_contains("0-9", '5'));
        assert!(!char_set_contains("0-9", 'a'));
        assert!(char_set_contains("a-zA-Z_", 'Q'));
        assert!(char_set_contains("@.\\-", '-'));
        assert!(char_set_contains("0-9-", '-'), "o hífen no fim é ele mesmo");
        assert!(char_set_contains("\\s", ' '));
    }

    #[test]
    fn the_child_decides_first_unless_the_parent_overrides() {
        let form = CharFilter { blocked: "@".into(), ..Default::default() };
        let edit = CharFilter { allowed: "@".into(), ..Default::default() };
        assert!(char_allowed(&[edit.clone(), form.clone()], '@'), "o filho libera");
        assert!(!char_allowed(&[CharFilter::default(), form.clone()], '@'), "sem opinião, vale o pai");
        let strict = CharFilter { overrides: true, ..form };
        assert!(!char_allowed(&[edit, strict], '@'), "o pai com prioridade manda");
        let digits = CharFilter { allowed: "0-9".into(), only_allowed: true, ..Default::default() };
        assert!(char_allowed(&[digits.clone()], '7') && !char_allowed(&[digits], 'x'));
    }

    #[test]
    fn mnemonics_read_and_strip() {
        assert_eq!(mnemonic("&Salvar"), Some('s'));
        assert_eq!(mnemonic("Sal&var"), Some('v'));
        assert_eq!(mnemonic("A && B"), None);
        assert_eq!(strip_mnemonic("Sal&var"), ("Salvar".to_string(), Some(3)));
        assert_eq!(strip_mnemonic("A && &B"), ("A & B".to_string(), Some(4)));
    }
}
