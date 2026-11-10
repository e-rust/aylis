//! As gramáticas oficiais do VS Code embutidas: todas compilam, realçam o básico de cada
//! linguagem e uma tecla em 5 mil linhas fica abaixo da meta (medida em `--release`).

use std::time::{Duration, Instant};

use aylis_controls::syntax::TokenKind;
use aylis_controls::textmate::{self, LineCache};

const SAMPLES: &[(&str, &str, &str, &str)] = &[
    // (linguagem, uma linha típica, uma palavra-chave dela, um texto dela)
    ("rust", "    pub fn soma(a: i32) -> i32 { let s = \"oi\"; a + 42 } // fim", "fn", "\"oi\""),
    ("python", "def soma(a):  return a + 42 if a else \"oi\"  # fim", "def", "\"oi\""),
    ("json", "  { \"nome\": \"oi\", \"idade\": 42, \"ativo\": true },", "true", "\"oi\""),
    ("pascal", "  if A > 42 then Writeln('oi'); // fim", "if", "'oi'"),
    ("typescript", "  export function soma(a: number): number { const s = \"oi\"; return a + 42; } // fim", "function", "\"oi\""),
    ("javascript", "  function soma(a) { const s = \"oi\"; return a + 42; } // fim", "function", "\"oi\""),
    ("csharp", "    public int Soma(int a) { var s = \"oi\"; return a + 42; } // fim", "public", "\"oi\""),
    ("java", "    public int soma(int a) { String s = \"oi\"; return a + 42; } // fim", "public", "\"oi\""),
    ("go", "func soma(a int) int { s := \"oi\"; return a + 42 } // fim", "func", "\"oi\""),
    ("cpp", "  int soma(int a) { const char* s = \"oi\"; return a + 42; } // fim", "int", "\"oi\""),
    ("c", "  int soma(int a) { const char* s = \"oi\"; return a + 42; } // fim", "int", "\"oi\""),
];

fn text_of(spans: &[aylis_controls::syntax::Span], line: &str, kind: TokenKind) -> Vec<String> {
    let c: Vec<char> = line.chars().collect();
    spans.iter().filter(|s| s.kind == kind).map(|s| c[s.start..s.end].iter().collect()).collect()
}

#[test]
fn the_official_grammars_highlight_every_language_fast() {
    aylis_ide::grammars::register();
    let release = !cfg!(debug_assertions);
    let mut report = Vec::new();
    let mut slow: Vec<String> = Vec::new();
    for &(lang, line, keyword, string) in SAMPLES {
        let grammar = textmate::grammar_for(lang).unwrap_or_else(|| panic!("{lang} sem gramática"));
        let spans = grammar.tokenize_line(line, &mut Default::default());
        let warnings = grammar.check();
        let keywords = text_of(&spans, line, TokenKind::Keyword);
        let types = text_of(&spans, line, TokenKind::Type);
        assert!(keywords.iter().chain(&types).any(|k| k.contains(keyword)), "{lang}: sem `{keyword}` em {keywords:?} {types:?}");
        let strings = text_of(&spans, line, TokenKind::String).concat();
        let dump = || {
            let c: Vec<char> = line.chars().collect();
            spans.iter().map(|s| format!("{:?} {:?} {:?}", c[s.start..s.end].iter().collect::<String>(), s.kind, textmate::scope_names(s.scope))).collect::<Vec<_>>().join("\n")
        };
        assert!(strings.contains(string), "{lang}: sem {string} em {strings:?}\n{}\n{warnings:?}", dump());
        // 5 mil linhas e uma tecla no meio.
        let lines = if release { 5000 } else { 300 };
        let text: String = (0..lines).map(|_| format!("{line}\n")).collect();
        let mut cache = LineCache::default();
        let t = Instant::now();
        cache.update(&grammar, &text, None);
        let full = t.elapsed();
        let at = text.len() / 2;
        let at = (at..text.len()).find(|&i| text.is_char_boundary(i) && text[i..].starts_with(' ')).unwrap_or(at);
        let mut edited = text.clone();
        let mut worst = Duration::ZERO;
        for k in 0..10 {
            edited.insert(at + k, 'x');
            let t = Instant::now();
            cache.update(&grammar, &edited, None);
            worst = worst.max(t.elapsed());
        }
        report.push(format!("{lang:<11} {lines} linhas: tudo {full:>10.2?}, pior tecla {worst:>9.2?}, {} aviso(s)", warnings.len()));
        for w in warnings.iter().take(3) {
            report.push(format!("    {w}"));
        }
        // A mesma meta para todas, o C++ inclusive (com o Oniguruma, ~1,2 ms).
        let limit = Duration::from_millis(2);
        if release && worst >= limit {
            slow.push(format!("{lang}: {worst:?}"));
        }
    }
    println!("{}", report.join("\n"));
    assert!(slow.is_empty(), "teclas acima da meta: {slow:?}");
}

#[test]
fn a_vsix_brings_its_language_and_theme() {
    use std::io::Write;
    let dir = std::env::temp_dir().join(format!("aylis-vsix-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let vsix = dir.join("ext.vsix");
    {
        let mut zip = zip::ZipWriter::new(std::fs::File::create(&vsix).unwrap());
        let opts = zip::write::SimpleFileOptions::default();
        let mut put = |name: &str, text: &str| {
            zip.start_file(name, opts).unwrap();
            zip.write_all(text.as_bytes()).unwrap();
        };
        put(
            "extension/package.json",
            r#"{ "name": "lua-teste", "contributes": {
                "languages": [ { "id": "luateste", "extensions": [".luat"], "configuration": "./lang.json" } ],
                "grammars": [ { "language": "luateste", "scopeName": "source.luat", "path": "./syntaxes/luat.json" } ],
                "themes": [ { "label": "Verde Teste", "uiTheme": "vs-dark", "path": "./themes/verde.json" } ] } }"#,
        );
        put("extension/lang.json", r#"{ "comments": { "lineComment": "--" }, "folding": { "markers": { "start": "^\\s*--\\s*#region\\b", "end": "^\\s*--\\s*#endregion\\b" } } }"#);
        put(
            "extension/syntaxes/luat.json",
            r#"{ "scopeName": "source.luat", "patterns": [ { "match": "\\b(local|function|end)\\b", "name": "keyword.control.luat" }, { "match": "--.*$", "name": "comment.line.luat" } ] }"#,
        );
        put("extension/themes/verde.json", r##"{ "type": "dark", "tokenColors": [ { "scope": "keyword", "settings": { "foreground": "#00FF00" } } ] }"##);
        zip.finish().unwrap();
    }
    let store = dir.join("store");
    let got = aylis_ide::grammars::import_to(&vsix, &store).unwrap();
    assert_eq!(got.languages, ["luateste"]);
    assert_eq!(got.themes, ["Verde Teste"]);
    assert!(got.warnings.is_empty(), "{:?}", got.warnings);
    assert_eq!(textmate::language_for_extension("luat").as_deref(), Some("luateste"));
    assert_eq!(aylis_ide::codewindow::syntax_of(std::path::Path::new("x.luat")), "luateste");
    let g = textmate::grammar_for("luateste").unwrap();
    let line = "local x -- nota";
    let spans = g.tokenize_line(line, &mut Default::default());
    assert_eq!(text_of(&spans, line, TokenKind::Keyword), ["local"], "{:?} {:?}", spans, spans.iter().map(|s| textmate::scope_names(s.scope)).collect::<Vec<_>>());
    assert_eq!(text_of(&spans, line, TokenKind::Comment), ["-- nota"]);
    assert_eq!(textmate::language_config("luateste").unwrap().line_comment.as_deref(), Some("--"));
    assert_eq!(textmate::region_mark("  -- #region Parte"), Some(true));
    let kw = spans.iter().find(|s| s.kind == TokenKind::Keyword).unwrap();
    assert_eq!(aylis_controls::textmate_theme::scope_color("Verde Teste", kw.scope), aylis_core::Color::from_hex("#00FF00"));
    assert!(aylis_controls::syntax::code_theme("verde teste").is_some(), "na lista do editor");
    assert!(store.join("ext").join("extension").join("package.json").is_file(), "guardado para a próxima vez");
    let _ = std::fs::remove_dir_all(&dir);
}
