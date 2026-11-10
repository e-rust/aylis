//! Realce de código para o [`Memo`](crate::Memo) (`syntax`): Rust, Python, JSON, Pascal
//! (Free Pascal e Delphi), TypeScript e JavaScript, C#, Java, Go e C/C++.
//!
//! Lê o texto todo de uma vez e devolve, para cada linha, os trechos com o tipo de cada um.
//! Comentários de bloco e textos de várias linhas continuam na linha de baixo. Não é um
//! analisador da linguagem: é o mesmo nível de um editor de texto (palavras-chave, textos,
//! números, comentários, tipos, chamadas, atributos e diretivas). Cada linguagem é uma
//! tabela ([`Def`]): os comentários, os tipos de texto e as palavras dela.

use aylis_core::Color;

/// O que é um trecho do código.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Plain,
    /// Palavras-chave e diretivas (`#include`, `{$mode}`, `#region`).
    Keyword,
    /// Tipos (iniciais maiúsculas, primitivos) e, em JSON, as chaves.
    Type,
    String,
    Number,
    Comment,
    /// Chamadas de função, macros, atributos (`#[derive]`) e decoradores (`@property`).
    Function,
}

/// Um trecho de uma linha, em caracteres (`start..end`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
    /// A pilha de escopos TextMate (0 no realce embutido; ver [`crate::textmate`]).
    pub scope: u32,
}

/// Como um texto entre aspas trata o escape.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Escape {
    /// `\"` (C, Rust, Python...).
    Backslash,
    /// A aspa dobrada (`''` no Pascal, `""` no `@"..."` do C#).
    Doubled,
    /// Sem escape (as crases do Go, o `R"(...)"` do C++).
    None,
}

/// Um tipo de texto: abre com `open`, fecha com `close`.
#[derive(Clone, Copy)]
struct Quote {
    open: &'static str,
    close: &'static str,
    escape: Escape,
    /// Pode continuar na linha de baixo.
    multiline: bool,
    /// Letras que podem vir antes da abertura (`b"..."`, `f"..."`, `$"..."`).
    prefixes: &'static str,
}

const fn q(open: &'static str, close: &'static str, escape: Escape, multiline: bool, prefixes: &'static str) -> Quote {
    Quote { open, close, escape, multiline, prefixes }
}

/// Atributos e decoradores.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Attr {
    None,
    /// `#[...]` e `#![...]` (Rust).
    Hash,
    /// `@nome` (Python, Java, TypeScript).
    At,
}

/// Uma linguagem.
struct Def {
    line_comments: &'static [&'static str],
    /// (abre, fecha, aninha).
    blocks: &'static [(&'static str, &'static str, bool)],
    quotes: &'static [Quote],
    keywords: &'static [&'static str],
    primitives: &'static [&'static str],
    /// Palavras sem diferença de maiúsculas (Pascal).
    case_insensitive: bool,
    /// Inicial maiúscula = tipo (falso no Pascal, onde só `TNome` é tipo).
    upper_is_type: bool,
    attr: Attr,
    /// `#include`, `#region`: a diretiva no começo da linha.
    preprocessor: bool,
    /// `$FF` (hexadecimal) e `#13` (caractere) do Pascal.
    pascal_numbers: bool,
    /// `'a'` é caractere e `'a` é tempo de vida (Rust).
    rust_chars: bool,
    /// O texto antes de `:` é chave (JSON).
    json_keys: bool,
}

const C_BLOCK: &[(&str, &str, bool)] = &[("/*", "*/", false)];

static RUST: Def = Def {
    line_comments: &["//"],
    blocks: &[("/*", "*/", true)],
    quotes: &[q("r#\"", "\"#", Escape::None, true, "b"), q("\"", "\"", Escape::Backslash, true, "br")],
    keywords: &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
        "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
        "while",
    ],
    primitives: &["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "bool", "char", "str"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::Hash,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: true,
    json_keys: false,
};

static PYTHON: Def = Def {
    line_comments: &["#"],
    blocks: &[],
    quotes: &[
        q("\"\"\"", "\"\"\"", Escape::Backslash, true, "rbfuRBFU"),
        q("'''", "'''", Escape::Backslash, true, "rbfuRBFU"),
        q("\"", "\"", Escape::Backslash, false, "rbfuRBFU"),
        q("'", "'", Escape::Backslash, false, "rbfuRBFU"),
    ],
    keywords: &[
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally",
        "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "self", "try", "while", "with",
        "yield", "match", "case",
    ],
    primitives: &["int", "float", "str", "bool", "list", "dict", "tuple", "set", "bytes"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::At,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

static JSON: Def = Def {
    line_comments: &[],
    blocks: &[],
    quotes: &[q("\"", "\"", Escape::Backslash, false, "")],
    keywords: &["true", "false", "null"],
    primitives: &[],
    case_insensitive: false,
    upper_is_type: false,
    attr: Attr::None,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: true,
};

static PASCAL: Def = Def {
    line_comments: &["//"],
    blocks: &[("{", "}", false), ("(*", "*)", false)],
    quotes: &[q("'", "'", Escape::Doubled, false, "")],
    keywords: &[
        "and", "array", "as", "asm", "begin", "case", "class", "const", "constructor", "destructor", "div", "do", "downto", "else", "end", "except", "exports",
        "file", "finalization", "finally", "for", "function", "goto", "if", "implementation", "in", "inherited", "initialization", "inline", "interface",
        "is", "label", "library", "mod", "nil", "not", "object", "of", "on", "or", "out", "packed", "procedure", "program", "property", "raise", "record",
        "repeat", "resourcestring", "set", "shl", "shr", "string", "then", "threadvar", "to", "try", "type", "unit", "until", "uses", "var", "while", "with",
        "xor", "private", "protected", "public", "published", "strict", "override", "virtual", "abstract", "overload", "reintroduce", "static", "true",
        "false", "self", "result", "exit", "specialize", "generic", "operator",
    ],
    primitives: &[
        "integer", "cardinal", "shortint", "smallint", "longint", "int64", "byte", "word", "longword", "qword", "boolean", "char", "widechar", "ansistring",
        "unicodestring", "widestring", "real", "single", "double", "extended", "currency", "pointer", "variant", "tobject",
    ],
    case_insensitive: true,
    upper_is_type: false,
    attr: Attr::None,
    preprocessor: false,
    pascal_numbers: true,
    rust_chars: false,
    json_keys: false,
};

static TYPESCRIPT: Def = Def {
    line_comments: &["//"],
    blocks: C_BLOCK,
    quotes: &[q("`", "`", Escape::Backslash, true, ""), q("\"", "\"", Escape::Backslash, false, ""), q("'", "'", Escape::Backslash, false, "")],
    keywords: &[
        "abstract", "any", "as", "async", "await", "break", "case", "catch", "class", "const", "continue", "debugger", "declare", "default", "delete", "do",
        "else", "enum", "export", "extends", "false", "finally", "for", "from", "function", "get", "if", "implements", "import", "in", "instanceof",
        "interface", "keyof", "let", "namespace", "new", "null", "of", "private", "protected", "public", "readonly", "return", "set", "static", "super",
        "switch", "this", "throw", "true", "try", "type", "typeof", "undefined", "var", "void", "while", "yield",
    ],
    primitives: &["number", "string", "boolean", "bigint", "symbol", "object", "never", "unknown"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::At,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

static CSHARP: Def = Def {
    line_comments: &["//"],
    blocks: C_BLOCK,
    quotes: &[
        q("\"\"\"", "\"\"\"", Escape::None, true, "$"),
        q("@\"", "\"", Escape::Doubled, true, "$"),
        q("\"", "\"", Escape::Backslash, false, "$"),
        q("'", "'", Escape::Backslash, false, ""),
    ],
    keywords: &[
        "abstract", "as", "async", "await", "base", "break", "case", "catch", "checked", "class", "const", "continue", "default", "delegate", "do", "else",
        "enum", "event", "explicit", "extern", "false", "finally", "fixed", "for", "foreach", "get", "goto", "if", "implicit", "in", "init", "interface",
        "internal", "is", "lock", "namespace", "new", "null", "operator", "out", "override", "params", "partial", "private", "protected", "public",
        "readonly", "record", "ref", "required", "return", "sealed", "set", "sizeof", "stackalloc", "static", "struct", "switch", "this", "throw", "true",
        "try", "typeof", "unchecked", "unsafe", "using", "value", "var", "virtual", "void", "volatile", "when", "where", "while", "yield",
    ],
    primitives: &["bool", "byte", "sbyte", "char", "decimal", "double", "float", "int", "uint", "long", "ulong", "short", "ushort", "object", "string", "nint", "nuint", "dynamic"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::None,
    preprocessor: true,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

static JAVA: Def = Def {
    line_comments: &["//"],
    blocks: C_BLOCK,
    quotes: &[q("\"\"\"", "\"\"\"", Escape::Backslash, true, ""), q("\"", "\"", Escape::Backslash, false, ""), q("'", "'", Escape::Backslash, false, "")],
    keywords: &[
        "abstract", "assert", "break", "case", "catch", "class", "const", "continue", "default", "do", "else", "enum", "extends", "false", "final", "finally",
        "for", "goto", "if", "implements", "import", "instanceof", "interface", "native", "new", "null", "package", "private", "protected", "public", "record",
        "return", "sealed", "static", "strictfp", "super", "switch", "synchronized", "this", "throw", "throws", "transient", "true", "try", "var", "void",
        "volatile", "while", "yield",
    ],
    primitives: &["boolean", "byte", "char", "short", "int", "long", "float", "double"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::At,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

static GO: Def = Def {
    line_comments: &["//"],
    blocks: C_BLOCK,
    quotes: &[q("`", "`", Escape::None, true, ""), q("\"", "\"", Escape::Backslash, false, ""), q("'", "'", Escape::Backslash, false, "")],
    keywords: &[
        "break", "case", "chan", "const", "continue", "default", "defer", "else", "fallthrough", "for", "func", "go", "goto", "if", "import", "interface",
        "map", "package", "range", "return", "select", "struct", "switch", "type", "var", "true", "false", "nil", "iota",
    ],
    primitives: &[
        "bool", "byte", "complex64", "complex128", "error", "float32", "float64", "int", "int8", "int16", "int32", "int64", "rune", "string", "uint", "uint8",
        "uint16", "uint32", "uint64", "uintptr", "any",
    ],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::None,
    preprocessor: false,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

static CPP: Def = Def {
    line_comments: &["//"],
    blocks: C_BLOCK,
    quotes: &[q("R\"(", ")\"", Escape::None, true, "uUL8"), q("\"", "\"", Escape::Backslash, false, "uUL8"), q("'", "'", Escape::Backslash, false, "uUL")],
    keywords: &[
        "alignas", "alignof", "auto", "break", "case", "catch", "class", "const", "consteval", "constexpr", "continue", "decltype", "default", "delete", "do",
        "else", "enum", "explicit", "export", "extern", "false", "for", "friend", "goto", "if", "inline", "mutable", "namespace", "new", "noexcept",
        "nullptr", "operator", "override", "private", "protected", "public", "return", "sizeof", "static", "static_assert", "static_cast", "struct",
        "switch", "template", "this", "throw", "true", "try", "typedef", "typename", "union", "using", "virtual", "volatile", "while", "dynamic_cast",
        "reinterpret_cast", "const_cast", "final", "concept", "requires", "co_await", "co_return", "co_yield",
    ],
    primitives: &["bool", "char", "char8_t", "char16_t", "char32_t", "double", "float", "int", "long", "short", "signed", "unsigned", "void", "wchar_t", "size_t"],
    case_insensitive: false,
    upper_is_type: true,
    attr: Attr::None,
    preprocessor: true,
    pascal_numbers: false,
    rust_chars: false,
    json_keys: false,
};

/// A tabela de uma linguagem pelo nome (ou pela extensão do arquivo).
fn def_of(name: &str) -> Option<&'static Def> {
    Some(match name.trim().to_ascii_lowercase().as_str() {
        "rust" | "rs" => &RUST,
        "python" | "py" => &PYTHON,
        "json" | "aylisproj" => &JSON,
        "pascal" | "pas" | "pp" | "lpr" | "dpr" | "delphi" | "freepascal" | "fpc" | "object pascal" => &PASCAL,
        "typescript" | "ts" | "tsx" | "javascript" | "js" | "jsx" | "mjs" => &TYPESCRIPT,
        "csharp" | "c#" | "cs" => &CSHARP,
        "java" => &JAVA,
        "go" | "golang" => &GO,
        "cpp" | "c++" | "cc" | "cxx" | "hpp" | "hh" | "c" | "h" => &CPP,
        _ => return None,
    })
}

/// As linguagens que o realce conhece (o primeiro nome de cada uma).
pub const LANGUAGES: [&str; 9] = ["rust", "python", "json", "pascal", "typescript", "csharp", "java", "go", "cpp"];

/// O que continua de uma linha para a outra.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Carry {
    None,
    /// Comentário de bloco (qual, e a profundidade nos que aninham).
    Block(usize, u32),
    /// Texto de várias linhas (qual).
    Str(usize),
}

/// Os trechos de cada linha (`text.split('\n')`) na linguagem `language`. Linguagem
/// desconhecida: tudo `Plain`.
pub fn highlight(language: &str, text: &str) -> Vec<Vec<Span>> {
    let Some(def) = def_of(language) else {
        return text.split('\n').map(|l| vec![Span { start: 0, end: l.chars().count(), kind: TokenKind::Plain, scope: 0 }]).collect();
    };
    let mut carry = Carry::None;
    text.split('\n').map(|line| line_spans(def, line, &mut carry)).collect()
}

/// Os trechos de cada linha como o editor de código faz: pela gramática TextMate da linguagem,
/// se há uma registrada (a do VS Code), senão pelo realce embutido ([`highlight`]). Para um
/// trecho curto fora do editor (o código da dica do servidor de linguagem), com as mesmas
/// cores dele.
pub fn highlight_like_editor(language: &str, text: &str) -> Vec<Vec<Span>> {
    match crate::textmate::grammar_for(language) {
        Some(grammar) => crate::textmate::LineCache::default().update(&grammar, text, None).0.as_ref().clone(),
        None => highlight(language, text),
    }
}

/// O realce de um texto grande feito aos poucos ([`Progressive::advance`]): cada chamada
/// anda linhas até o tempo acabar, guardando de onde parou. As linhas ainda não feitas
/// ficam sem trechos (aparecem na cor do texto).
pub struct Progressive {
    language: String,
    spans: std::rc::Rc<Vec<Vec<Span>>>,
    carry: Carry,
    /// O byte do começo da próxima linha a fazer.
    at: usize,
    done: bool,
}

impl Progressive {
    pub fn new(language: &str) -> Self {
        Self { language: language.to_string(), spans: std::rc::Rc::new(Vec::new()), carry: Carry::None, at: 0, done: false }
    }

    /// Anda o realce de `text` até `budget` acabar (o texto é o mesmo de todas as chamadas).
    /// Devolve os trechos feitos até agora e se terminou.
    pub fn advance(&mut self, text: &str, budget: std::time::Duration) -> (std::rc::Rc<Vec<Vec<Span>>>, bool) {
        if self.done {
            return (self.spans.clone(), true);
        }
        let started = std::time::Instant::now();
        let def = def_of(&self.language);
        // Sem outra referência viva (a pintura já soltou a dela), acrescenta no lugar.
        let spans = std::rc::Rc::make_mut(&mut self.spans);
        let mut count = 0u32;
        loop {
            let rest = &text[self.at..];
            let (line, next) = match rest.find('\n') {
                Some(n) => (&rest[..n], Some(self.at + n + 1)),
                None => (rest, None),
            };
            spans.push(match def {
                Some(def) => line_spans(def, line, &mut self.carry),
                None => vec![Span { start: 0, end: line.chars().count(), kind: TokenKind::Plain, scope: 0 }],
            });
            match next {
                Some(n) => self.at = n,
                None => {
                    self.done = true;
                    break;
                }
            }
            count += 1;
            // Olha o relógio de tempos em tempos (ele custa mais que uma linha curta).
            if count % 256 == 0 && started.elapsed() >= budget {
                break;
            }
        }
        (self.spans.clone(), self.done)
    }
}

fn starts(c: &[char], i: usize, s: &str) -> bool {
    let mut k = i;
    for ch in s.chars() {
        if c.get(k) != Some(&ch) {
            return false;
        }
        k += 1;
    }
    true
}

/// Fecha um comentário de bloco a partir de `i`: (fim, profundidade que sobrou).
fn scan_block(c: &[char], mut i: usize, (open, close, nests): (&str, &str, bool), mut depth: u32) -> (usize, u32) {
    let (ol, cl) = (open.chars().count(), close.chars().count());
    while i < c.len() && depth > 0 {
        if starts(c, i, close) {
            depth -= 1;
            i += cl;
        } else if nests && starts(c, i, open) {
            depth += 1;
            i += ol;
        } else {
            i += 1;
        }
    }
    (i, depth)
}

/// Fecha um texto a partir de `i`: (fim, fechou).
fn scan_quote(c: &[char], mut i: usize, quote: &Quote) -> (usize, bool) {
    let cl = quote.close.chars().count();
    while i < c.len() {
        if quote.escape == Escape::Backslash && c[i] == '\\' {
            i += 2;
            continue;
        }
        if starts(c, i, quote.close) {
            if quote.escape == Escape::Doubled && starts(c, i + cl, quote.close) {
                i += 2 * cl;
                continue;
            }
            return (i + cl, true);
        }
        i += 1;
    }
    (c.len(), false)
}

fn line_spans(def: &Def, line: &str, carry: &mut Carry) -> Vec<Span> {
    let c: Vec<char> = line.chars().collect();
    let n = c.len();
    let mut out: Vec<Span> = Vec::new();
    let mut push = |start: usize, end: usize, kind: TokenKind| {
        if end > start {
            match out.last_mut() {
                Some(last) if last.kind == kind && last.end == start => last.end = end,
                _ => out.push(Span { start, end, kind, scope: 0 }),
            }
        }
    };
    let mut i = 0;
    // O que veio da linha de cima.
    match *carry {
        Carry::Block(b, depth) => {
            let (end, left) = scan_block(&c, 0, def.blocks[b], depth);
            push(0, end, TokenKind::Comment);
            i = end;
            *carry = if left > 0 { Carry::Block(b, left) } else { Carry::None };
        }
        Carry::Str(k) => {
            let (end, closed) = scan_quote(&c, 0, &def.quotes[k]);
            push(0, end, TokenKind::String);
            i = end;
            *carry = if closed { Carry::None } else { Carry::Str(k) };
        }
        Carry::None => {}
    }
    'scan: while i < n {
        let ch = c[i];
        let word_start = i == 0 || !is_ident(c[i - 1]);
        // Comentário de linha.
        if def.line_comments.iter().any(|lc| starts(&c, i, lc)) {
            push(i, n, TokenKind::Comment);
            break;
        }
        // Comentário de bloco (no Pascal, `{$...}` é diretiva).
        for (b, block) in def.blocks.iter().enumerate() {
            if starts(&c, i, block.0) {
                let directive = def.pascal_numbers && block.0 == "{" && c.get(i + 1) == Some(&'$');
                let (end, left) = scan_block(&c, i + block.0.chars().count(), *block, 1);
                push(i, end, if directive { TokenKind::Keyword } else { TokenKind::Comment });
                if left > 0 {
                    *carry = Carry::Block(b, left);
                }
                i = end;
                continue 'scan;
            }
        }
        // Diretiva do pré-processador (`#include`, `#region`) no começo da linha.
        if def.preprocessor && ch == '#' && c[..i].iter().all(|x| x.is_whitespace()) {
            let end = i + 1 + c[i + 1..].iter().take_while(|&&x| x.is_whitespace()).count();
            let end = end + c[end..].iter().take_while(|&&x| is_ident(x)).count();
            push(i, end, TokenKind::Keyword);
            i = end;
            continue;
        }
        // Atributos do Rust: `#[...]` e `#![...]` até o `]` que fecha.
        if def.attr == Attr::Hash && ch == '#' && (c.get(i + 1) == Some(&'[') || (c.get(i + 1) == Some(&'!') && c.get(i + 2) == Some(&'['))) {
            let mut depth = 0;
            let mut end = i;
            while end < n {
                match c[end] {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            end += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                end += 1;
            }
            push(i, end, TokenKind::Function);
            i = end;
            continue;
        }
        // Caractere e tempo de vida do Rust ('a' e 'a).
        if def.rust_chars && ch == '\'' {
            let close = if c.get(i + 1) == Some(&'\\') { c[i + 2..].iter().position(|&x| x == '\'').map(|p| i + 2 + p) } else { (c.get(i + 2) == Some(&'\'')).then_some(i + 2) };
            match close {
                Some(end) => {
                    push(i, end + 1, TokenKind::String);
                    i = end + 1;
                }
                None => {
                    let end = i + 1 + c[i + 1..].iter().take_while(|&&x| is_ident(x)).count();
                    push(i, end, TokenKind::Type);
                    i = end;
                }
            }
            continue;
        }
        // Textos (com os prefixos: b"", f"", $"", u8"").
        for (k, quote) in def.quotes.iter().enumerate() {
            let prefix = if word_start { c[i..].iter().take_while(|x| quote.prefixes.contains(**x)).count().min(2) } else { 0 };
            for p in (0..=prefix).rev() {
                if starts(&c, i + p, quote.open) {
                    let (end, closed) = scan_quote(&c, i + p + quote.open.chars().count(), quote);
                    let key = def.json_keys && c[end..].iter().find(|x| !x.is_whitespace()) == Some(&':');
                    push(i, end, if key { TokenKind::Type } else { TokenKind::String });
                    if !closed && quote.multiline {
                        *carry = Carry::Str(k);
                    }
                    i = end;
                    continue 'scan;
                }
            }
        }
        // Números (`42`, `0xFF`, `1.5e3`; no Pascal, `$FF` e o caractere `#13`).
        let pascal_number = def.pascal_numbers && (ch == '$' || ch == '#') && c.get(i + 1).is_some_and(|x| x.is_ascii_hexdigit());
        if (ch.is_ascii_digit() && word_start) || pascal_number {
            let start = i;
            i += 1;
            while i < n && (c[i].is_ascii_alphanumeric() || c[i] == '_' || (c[i] == '.' && c.get(i + 1).is_some_and(char::is_ascii_digit))) {
                i += 1;
            }
            push(start, i, TokenKind::Number);
            continue;
        }
        // Palavras.
        if is_ident(ch) && !ch.is_ascii_digit() {
            let start = i;
            while i < n && is_ident(c[i]) {
                i += 1;
            }
            let word: String = c[start..i].iter().collect();
            let lower = word.to_lowercase();
            let key = if def.case_insensitive { lower.as_str() } else { word.as_str() };
            let next = c[i..].iter().find(|x| !x.is_whitespace()).copied();
            let kind = if def.keywords.contains(&key) {
                TokenKind::Keyword
            } else if def.attr == Attr::Hash && next == Some('!') {
                TokenKind::Function
            } else if def.attr == Attr::At && start > 0 && c[start - 1] == '@' {
                TokenKind::Function
            } else if next == Some('(') {
                TokenKind::Function
            } else if def.primitives.contains(&key) {
                TokenKind::Type
            } else if def.upper_is_type && word.chars().next().is_some_and(char::is_uppercase) {
                TokenKind::Type
            } else if !def.upper_is_type && def.case_insensitive && is_pascal_type(&word) {
                TokenKind::Type
            } else {
                TokenKind::Plain
            };
            push(start, i, kind);
            continue;
        }
        push(i, i + 1, TokenKind::Plain);
        i += 1;
    }
    out
}

/// Os tipos do Pascal pela convenção: `TNome`, `INome`, `ENome` (classe, interface, exceção).
fn is_pascal_type(word: &str) -> bool {
    let mut chars = word.chars();
    matches!((chars.next(), chars.next()), (Some('T' | 'I' | 'E'), Some(second)) if second.is_uppercase())
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// As cores do realce, conforme o fundo seja claro ou escuro (como o VS Code).
/// Um tema de cores do editor de código (como os do VS Code): o fundo, o texto, a calha, a
/// linha do cursor, a seleção, o cursor e a cor de cada tipo de trecho.
#[derive(Clone, Copy, Debug)]
pub struct CodeTheme {
    pub name: &'static str,
    pub dark: bool,
    pub background: &'static str,
    pub foreground: &'static str,
    pub gutter: &'static str,
    pub line_highlight: &'static str,
    pub selection: &'static str,
    pub cursor: &'static str,
    pub keyword: &'static str,
    pub type_: &'static str,
    pub string: &'static str,
    pub number: &'static str,
    pub comment: &'static str,
    pub function: &'static str,
}

impl CodeTheme {
    fn hex(h: &str) -> Color {
        Color::from_hex(h).unwrap_or(Color::BLACK)
    }

    pub fn background(&self) -> Color {
        Self::hex(self.background)
    }

    pub fn foreground(&self) -> Color {
        Self::hex(self.foreground)
    }

    pub fn gutter(&self) -> Color {
        Self::hex(self.gutter)
    }

    pub fn line_highlight(&self) -> Color {
        Self::hex(self.line_highlight)
    }

    pub fn selection(&self) -> Color {
        Self::hex(self.selection)
    }

    pub fn cursor(&self) -> Color {
        Self::hex(self.cursor)
    }

    /// A cor de um tipo de trecho.
    pub fn token(&self, kind: TokenKind) -> Color {
        Self::hex(match kind {
            TokenKind::Plain => self.foreground,
            TokenKind::Keyword => self.keyword,
            TokenKind::Type => self.type_,
            TokenKind::String => self.string,
            TokenKind::Number => self.number,
            TokenKind::Comment => self.comment,
            TokenKind::Function => self.function,
        })
    }
}

/// Os temas de cores do editor de código (o `color_theme` do `CodeEditor`).
pub const CODE_THEMES: &[CodeTheme] = &[
    CodeTheme { name: "Dark+", dark: true, background: "#1E1E1E", foreground: "#D4D4D4", gutter: "#858585", line_highlight: "#FFFFFF0F", selection: "#264F78", cursor: "#AEAFAD", keyword: "#569CD6", type_: "#4EC9B0", string: "#CE9178", number: "#B5CEA8", comment: "#6A9955", function: "#DCDCAA" },
    CodeTheme { name: "Light+", dark: false, background: "#FFFFFF", foreground: "#000000", gutter: "#237893", line_highlight: "#0000000A", selection: "#ADD6FF", cursor: "#000000", keyword: "#0000FF", type_: "#267F99", string: "#A31515", number: "#098658", comment: "#008000", function: "#795E26" },
    CodeTheme { name: "Monokai", dark: true, background: "#272822", foreground: "#F8F8F2", gutter: "#90908A", line_highlight: "#3E3D32", selection: "#49483E", cursor: "#F8F8F0", keyword: "#F92672", type_: "#66D9EF", string: "#E6DB74", number: "#AE81FF", comment: "#88846F", function: "#A6E22E" },
    CodeTheme { name: "Dracula", dark: true, background: "#282A36", foreground: "#F8F8F2", gutter: "#6272A4", line_highlight: "#44475A75", selection: "#44475A", cursor: "#F8F8F2", keyword: "#FF79C6", type_: "#8BE9FD", string: "#F1FA8C", number: "#BD93F9", comment: "#6272A4", function: "#50FA7B" },
    CodeTheme { name: "One Dark", dark: true, background: "#282C34", foreground: "#ABB2BF", gutter: "#495162", line_highlight: "#2C313C", selection: "#3E4451", cursor: "#528BFF", keyword: "#C678DD", type_: "#E5C07B", string: "#98C379", number: "#D19A66", comment: "#7F848E", function: "#61AFEF" },
    CodeTheme { name: "Solarized Dark", dark: true, background: "#002B36", foreground: "#839496", gutter: "#586E75", line_highlight: "#073642", selection: "#274642", cursor: "#D30102", keyword: "#859900", type_: "#CB4B16", string: "#2AA198", number: "#D33682", comment: "#586E75", function: "#268BD2" },
    CodeTheme { name: "Solarized Light", dark: false, background: "#FDF6E3", foreground: "#657B83", gutter: "#93A1A1", line_highlight: "#EEE8D5", selection: "#EEE8D5", cursor: "#657B83", keyword: "#859900", type_: "#CB4B16", string: "#2AA198", number: "#D33682", comment: "#93A1A1", function: "#268BD2" },
    CodeTheme { name: "GitHub Light", dark: false, background: "#FFFFFF", foreground: "#24292F", gutter: "#8C959F", line_highlight: "#EAEEF280", selection: "#0969DA33", cursor: "#0969DA", keyword: "#CF222E", type_: "#953800", string: "#0A3069", number: "#0550AE", comment: "#6E7781", function: "#8250DF" },
    CodeTheme { name: "GitHub Dark", dark: true, background: "#0D1117", foreground: "#E6EDF3", gutter: "#6E7681", line_highlight: "#6E76811A", selection: "#264F78", cursor: "#2F81F7", keyword: "#FF7B72", type_: "#FFA657", string: "#A5D6FF", number: "#79C0FF", comment: "#8B949E", function: "#D2A8FF" },
    CodeTheme { name: "Nord", dark: true, background: "#2E3440", foreground: "#D8DEE9", gutter: "#4C566A", line_highlight: "#3B4252", selection: "#434C5E", cursor: "#D8DEE9", keyword: "#81A1C1", type_: "#8FBCBB", string: "#A3BE8C", number: "#B48EAD", comment: "#616E88", function: "#88C0D0" },
];

/// O tema de cores do editor pelo nome (sem diferenciar maiúsculas), entre os embutidos e os
/// importados; vazio ou desconhecido: `None` (o editor segue o tema do app).
pub fn code_theme(name: &str) -> Option<&'static CodeTheme> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    code_themes().into_iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

/// Os temas do editor: os embutidos e, depois, os importados do VS Code
/// ([`register_theme`](crate::textmate_theme::register_theme)).
pub fn code_themes() -> Vec<&'static CodeTheme> {
    CODE_THEMES.iter().chain(crate::textmate_theme::imported_themes()).collect()
}

/// A cor de um trecho: a do escopo, se o tema (ou, sem tema, o Dark+/Light+) tem regras por
/// escopo; senão a do tipo de trecho.
pub fn span_color(theme: Option<&CodeTheme>, span: &Span, dark: bool, text: Color) -> Color {
    let name = theme.map_or(if dark { "Dark+" } else { "Light+" }, |t| t.name);
    if let Some(c) = crate::textmate_theme::scope_color(name, span.scope) {
        return c;
    }
    theme.map_or_else(|| color(span.kind, dark, text), |t| t.token(span.kind))
}

pub fn color(kind: TokenKind, dark: bool, text: Color) -> Color {
    let hex = match (kind, dark) {
        (TokenKind::Plain, _) => return text,
        (TokenKind::Keyword, false) => "#0000FF",
        (TokenKind::Keyword, true) => "#569CD6",
        (TokenKind::Type, false) => "#267F99",
        (TokenKind::Type, true) => "#4EC9B0",
        (TokenKind::String, false) => "#A31515",
        (TokenKind::String, true) => "#CE9178",
        (TokenKind::Number, false) => "#098658",
        (TokenKind::Number, true) => "#B5CEA8",
        (TokenKind::Comment, false) => "#008000",
        (TokenKind::Comment, true) => "#6A9955",
        (TokenKind::Function, false) => "#795E26",
        (TokenKind::Function, true) => "#DCDCAA",
    };
    Color::from_hex(hex).unwrap_or(text)
}
