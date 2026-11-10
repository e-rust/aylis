//! Temas do VS Code por escopo (o `tokenColors` do JSON do tema, com o `include` do tema de
//! base) para o realce das gramáticas TextMate ([`crate::textmate`]).
//!
//! [`register_theme`] lê o tema e o põe na lista dos temas do editor ([`crate::syntax::code_themes`],
//! o `color_theme` do `CodeEditor`), com as cores do editor que ele traz (`editor.background`,
//! a seleção, o cursor...) e as do tipo de trecho tiradas das regras. Um tema com o nome de um
//! embutido (Dark+, Light+...) dá a ele as regras por escopo.

use std::cell::RefCell;
use std::collections::HashMap;

use aylis_core::Color;
use serde_json::Value;

use crate::syntax::{CodeTheme, TokenKind};
use crate::textmate::{scope_names, strip_jsonc};

/// Uma regra: os seletores (cada um: os escopos do de fora para o de dentro) e a cor.
struct ScopeRule {
    selectors: Vec<Vec<String>>,
    color: Color,
}

struct ScopeTheme {
    rules: Vec<ScopeRule>,
    cache: RefCell<HashMap<u32, Option<Color>>>,
}

thread_local! {
    static THEMES: RefCell<HashMap<String, std::rc::Rc<ScopeTheme>>> = RefCell::default();
    static IMPORTED: RefCell<Vec<&'static CodeTheme>> = const { RefCell::new(Vec::new()) };
}

/// Lê o JSON de um tema do VS Code. `include` lê o tema de base pelo caminho que o tema dá
/// (`"include": "./dark_vs.json"`). Devolve o nome, se é escuro, as cores do editor e as
/// regras.
fn read(json: &str, include: &dyn Fn(&str) -> Option<String>, depth: u32) -> Result<(String, Option<bool>, HashMap<String, String>, Vec<ScopeRule>), String> {
    let v: Value = serde_json::from_str(&strip_jsonc(json.trim_start_matches('\u{FEFF}'))).map_err(|e| e.to_string())?;
    let (mut name, mut dark, mut colors, mut rules) = (String::new(), None, HashMap::new(), Vec::new());
    if let Some(base) = v.get("include").and_then(Value::as_str) {
        if depth < 4 {
            if let Some(text) = include(base) {
                (_, dark, colors, rules) = read(&text, include, depth + 1)?;
            }
        }
    }
    if let Some(n) = v.get("name").and_then(Value::as_str) {
        name = n.to_string();
    }
    if let Some(t) = v.get("type").and_then(Value::as_str) {
        dark = Some(t != "light");
    }
    if let Some(c) = v.get("colors").and_then(Value::as_object) {
        for (k, value) in c {
            if let Some(s) = value.as_str() {
                colors.insert(k.clone(), s.to_string());
            }
        }
    }
    for entry in v.get("tokenColors").and_then(Value::as_array).into_iter().flatten() {
        let Some(fg) = entry.get("settings").and_then(|s| s.get("foreground")).and_then(Value::as_str).and_then(Color::from_hex) else { continue };
        let scopes: Vec<String> = match entry.get("scope") {
            Some(Value::String(s)) => s.split(',').map(|x| x.trim().to_string()).collect(),
            Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).map(str::to_string).collect(),
            _ => continue,
        };
        let selectors = scopes.iter().filter(|s| !s.is_empty() && !s.contains(" -")).map(|s| s.split_whitespace().map(str::to_string).collect()).collect();
        rules.push(ScopeRule { selectors, color: fg });
    }
    Ok((name, dark, colors, rules))
}

/// `selector` (`string.quoted`) vale para o escopo `scope` (`string.quoted.double.rust`)?
fn part_matches(selector: &str, scope: &str) -> bool {
    scope == selector || (scope.starts_with(selector) && scope.as_bytes().get(selector.len()) == Some(&b'.'))
}

/// A pontuação de um seletor contra a pilha (de fora para dentro): a profundidade em que o
/// último escopo bate e o tamanho dele; `None` se não vale.
fn score(selector: &[String], stack: &[std::rc::Rc<str>]) -> Option<(usize, usize, usize)> {
    let (last, parents) = selector.split_last()?;
    let depth = stack.iter().rposition(|s| part_matches(last, s))?;
    // Os de fora, na ordem, abaixo dele.
    let mut k = depth;
    for p in parents.iter().rev() {
        k = stack[..k].iter().rposition(|s| part_matches(p, s))?;
    }
    Some((depth, last.len(), parents.len()))
}

impl ScopeTheme {
    fn color(&self, stack: u32) -> Option<Color> {
        if let Some(c) = self.cache.borrow().get(&stack) {
            return *c;
        }
        let names = scope_names(stack);
        let mut best: Option<((usize, usize, usize), usize, Color)> = None;
        for (i, rule) in self.rules.iter().enumerate() {
            for sel in &rule.selectors {
                if let Some(s) = score(sel, &names) {
                    // Pontuação maior ganha; empate: a regra que vem depois.
                    if best.as_ref().is_none_or(|(b, bi, _)| (s, i) >= (*b, *bi)) {
                        best = Some((s, i, rule.color));
                    }
                }
            }
        }
        let color = best.map(|b| b.2);
        self.cache.borrow_mut().insert(stack, color);
        color
    }
}

/// A cor do escopo `stack` no tema `theme` (`None`: o tema não tem regras por escopo, ou
/// nenhuma vale; fica a do tipo de trecho).
pub fn scope_color(theme: &str, stack: u32) -> Option<Color> {
    if stack == 0 {
        return None;
    }
    let t = THEMES.with(|t| t.borrow().get(&theme.to_ascii_lowercase()).cloned())?;
    t.color(stack)
}

/// Os temas importados (depois dos embutidos na lista do editor).
pub fn imported_themes() -> Vec<&'static CodeTheme> {
    IMPORTED.with(|i| i.borrow().clone())
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// Registra um tema do VS Code (JSON). `include` lê o tema de base que ele inclui. Devolve o
/// nome com que ele aparece na lista (`fallback_name` se o JSON não tiver).
pub fn register_theme(json: &str, fallback_name: &str, include: &dyn Fn(&str) -> Option<String>) -> Result<String, String> {
    let (name, dark, colors, rules) = read(json, include, 0)?;
    let name = if name.is_empty() { fallback_name.to_string() } else { name };
    if name.is_empty() {
        return Err("o tema não tem nome".into());
    }
    let theme = std::rc::Rc::new(ScopeTheme { rules, cache: RefCell::default() });
    let key = name.to_ascii_lowercase();
    // Um embutido com o mesmo nome só ganha as regras.
    let builtin = crate::syntax::CODE_THEMES.iter().any(|t| t.name.eq_ignore_ascii_case(&name));
    if !builtin {
        let dark = dark.unwrap_or_else(|| colors.get("editor.background").and_then(|c| Color::from_hex(c)).is_none_or(|c| luminance(c) < 0.5));
        let pick = |keys: &[&str], default: &str| -> &'static str { leak(keys.iter().find_map(|k| colors.get(*k).cloned()).unwrap_or_else(|| default.to_string())) };
        let (bg, fg) = if dark { ("#1E1E1E", "#D4D4D4") } else { ("#FFFFFF", "#000000") };
        let kind = |k: TokenKind, default: &str| -> &'static str {
            let sample = match k {
                TokenKind::Keyword => "keyword.control",
                TokenKind::Type => "entity.name.type",
                TokenKind::String => "string.quoted.double",
                TokenKind::Number => "constant.numeric",
                TokenKind::Comment => "comment.line",
                TokenKind::Function => "entity.name.function",
                TokenKind::Plain => "source",
            };
            let stack = crate::textmate::push_scope(crate::textmate::push_scope(0, "source.x"), sample);
            leak(theme.color(stack).map_or_else(|| default.to_string(), hex))
        };
        let code = CodeTheme {
            name: leak(name.clone()),
            dark,
            background: pick(&["editor.background"], bg),
            foreground: pick(&["editor.foreground", "foreground"], fg),
            gutter: pick(&["editorLineNumber.foreground"], "#858585"),
            line_highlight: pick(&["editor.lineHighlightBackground"], if dark { "#FFFFFF0F" } else { "#0000000A" }),
            selection: pick(&["editor.selectionBackground"], if dark { "#264F78" } else { "#ADD6FF" }),
            cursor: pick(&["editorCursor.foreground"], fg),
            keyword: kind(TokenKind::Keyword, "#569CD6"),
            type_: kind(TokenKind::Type, "#4EC9B0"),
            string: kind(TokenKind::String, "#CE9178"),
            number: kind(TokenKind::Number, "#B5CEA8"),
            comment: kind(TokenKind::Comment, "#6A9955"),
            function: kind(TokenKind::Function, "#DCDCAA"),
        };
        IMPORTED.with(|i| {
            let mut i = i.borrow_mut();
            i.retain(|t| !t.name.eq_ignore_ascii_case(&name));
            i.push(Box::leak(Box::new(code)));
        });
    }
    THEMES.with(|t| t.borrow_mut().insert(key, theme));
    Ok(name)
}

fn luminance(c: Color) -> f32 {
    let (r, g, b) = (c.r, c.g, c.b);
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 255.0
}

fn hex(c: Color) -> String {
    let (r, g, b, a) = (c.r, c.g, c.b, c.a);
    if a == 255 { format!("#{r:02X}{g:02X}{b:02X}") } else { format!("#{r:02X}{g:02X}{b:02X}{a:02X}") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textmate::push_scope;

    #[test]
    fn the_deepest_and_longest_selector_wins_and_includes_are_read() {
        let base = r##"{ "type": "dark", "colors": { "editor.background": "#101010" },
            "tokenColors": [ { "scope": "string", "settings": { "foreground": "#AA0000" } } ] }"##;
        let theme = r##"{ "name": "Teste Escopo", "include": "./base.json", // comentário
            "tokenColors": [
                { "scope": ["string.quoted", "keyword"], "settings": { "foreground": "#00AA00" } },
                { "scope": "source.rust string", "settings": { "foreground": "#0000AA" } },
                { "scope": "meta.function keyword", "settings": { "foreground": "#FFFFFF" } }
            ] }"##;
        let name = register_theme(theme, "", &|p| (p == "./base.json").then(|| base.to_string())).unwrap();
        assert_eq!(name, "Teste Escopo");
        let rust = push_scope(0, "source.rust");
        let string = push_scope(rust, "string.quoted.double.rust");
        // `string.quoted` é mais comprido que `string` (do `source.rust string`).
        assert_eq!(scope_color(&name, string), Color::from_hex("#00AA00"));
        let other = push_scope(push_scope(0, "source.go"), "string.other");
        assert_eq!(scope_color(&name, other), Color::from_hex("#AA0000"), "o tema de base");
        let kw = push_scope(push_scope(rust, "meta.function.rust"), "keyword.other");
        assert_eq!(scope_color(&name, kw), Color::from_hex("#FFFFFF"));
        let t = imported_themes().into_iter().find(|t| t.name == "Teste Escopo").unwrap();
        assert!(t.dark);
        assert_eq!(t.background, "#101010");
        assert_eq!(t.string, "#00AA00");
    }
}
