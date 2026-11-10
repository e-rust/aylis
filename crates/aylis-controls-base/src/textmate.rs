//! Gramáticas TextMate (as do VS Code: `.tmLanguage.json`, ou o `.tmLanguage` em plist) no
//! realce do [`Memo`](crate::Memo) e do [`CodeEditor`](crate::CodeEditor).
//!
//! Registrada uma gramática para a linguagem ([`register_grammar`]), ela passa na frente do
//! realce embutido ([`syntax`](crate::syntax)). As expressões rodam no Oniguruma, o motor do
//! VS Code (no wasm, no `fancy-regex` no modo dele; veja `textmate_regex.rs`); a regra que o
//! motor não aceita fica de fora e aparece em [`grammar_warnings`]. Cada linha guarda o estado do fim dela (as
//! regras abertas), e uma edição refaz só a partir da linha mudada, até o estado voltar a
//! bater com o de antes ([`LineCache`]).
//!
//! Cada trecho sai com a pilha de escopos (`source.rust meta.function entity.name.function`)
//! num número ([`Span::scope`](crate::syntax::Span::scope)): o tema por escopo
//! ([`textmate_theme`](crate::textmate_theme)) dá a cor; os temas embutidos usam o tipo
//! ([`scope_kind`]).

use std::cell::{OnceCell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use regex_engine::{Found, Regex};
use serde_json::{Map, Value};

use crate::syntax::{Span, TokenKind};

#[path = "textmate_plist.rs"]
mod plist;
#[path = "textmate_regex.rs"]
mod regex_engine;

// ---------------------------------------------------------------------------
// Escopos

/// As pilhas de escopos, cada uma num número (0 = vazia).
#[derive(Default)]
struct Scopes {
    /// (pilha de baixo, nome do escopo).
    stacks: Vec<(u32, Rc<str>)>,
    ids: HashMap<(u32, Rc<str>), u32>,
    kinds: Vec<Option<TokenKind>>,
}

thread_local! {
    static SCOPES: RefCell<Scopes> = RefCell::new(Scopes { stacks: vec![(0, "".into())], ..Default::default() });
}

/// A pilha `stack` com mais um ou vários escopos (separados por espaço) em cima.
pub fn push_scope(stack: u32, names: &str) -> u32 {
    SCOPES.with(|s| {
        let mut s = s.borrow_mut();
        let mut id = stack;
        for name in names.split_whitespace() {
            let key: (u32, Rc<str>) = (id, name.into());
            id = match s.ids.get(&key) {
                Some(&found) => found,
                None => {
                    let new = s.stacks.len() as u32;
                    s.stacks.push(key.clone());
                    s.ids.insert(key, new);
                    new
                }
            };
        }
        id
    })
}

/// Os escopos da pilha, de fora para dentro (`source.rust`, `meta.function`, ...).
pub fn scope_names(stack: u32) -> Vec<Rc<str>> {
    SCOPES.with(|s| {
        let s = s.borrow();
        let mut out = Vec::new();
        let mut id = stack;
        while id != 0 {
            let Some((parent, name)) = s.stacks.get(id as usize) else { break };
            out.push(name.clone());
            id = *parent;
        }
        out.reverse();
        out
    })
}

/// O tipo de trecho de uma pilha de escopos, para os temas sem regras por escopo: o escopo
/// mais de dentro que diz algo (`punctuation` dentro de `string` é texto).
pub fn scope_kind(stack: u32) -> TokenKind {
    if stack == 0 {
        return TokenKind::Plain;
    }
    if let Some(Some(kind)) = SCOPES.with(|s| s.borrow().kinds.get(stack as usize).copied()) {
        return kind;
    }
    let names = scope_names(stack);
    let kind = names.iter().rev().find_map(|n| kind_of(n)).unwrap_or(TokenKind::Plain);
    SCOPES.with(|s| {
        let mut s = s.borrow_mut();
        let len = s.stacks.len();
        s.kinds.resize(len, None);
        s.kinds[stack as usize] = Some(kind);
    });
    kind
}

fn kind_of(scope: &str) -> Option<TokenKind> {
    let has = |prefix: &str| scope == prefix || scope.starts_with(&format!("{prefix}."));
    Some(if has("comment") {
        TokenKind::Comment
    } else if has("string") || has("constant.character.escape") {
        TokenKind::String
    } else if has("constant.numeric") {
        TokenKind::Number
    } else if has("entity.name.function") || has("support.function") || has("meta.function-call.generic") || has("entity.name.tag") || has("meta.attribute") || has("meta.decorator") {
        TokenKind::Function
    } else if has("entity.name.type") || has("entity.name.class") || has("entity.name.namespace") || has("support.type") || has("support.class") || has("entity.other.inherited-class") || has("storage.type.primitive") || has("storage.type.numeric") || has("storage.type.built-in") {
        TokenKind::Type
    } else if has("keyword") && !has("keyword.operator") || has("storage") || has("constant.language") || has("variable.language") || has("keyword.operator.new") || has("keyword.operator.expression") {
        TokenKind::Keyword
    } else if has("keyword.operator") || has("punctuation") || has("meta") || has("source") || has("variable") || has("entity") {
        return None;
    } else {
        return None;
    })
}

// ---------------------------------------------------------------------------
// Regras

type RuleId = usize;

/// Uma expressão, compilada na primeira vez que é usada.
struct Re {
    source: String,
    compiled: OnceCell<Option<Rc<Regex>>>,
    /// Tem `\G` (o resultado depende de onde a busca começa: não entra no cache da linha).
    anchored: bool,
}

impl Re {
    fn new(source: &str) -> Self {
        Self { source: source.to_string(), compiled: OnceCell::new(), anchored: source.contains("\\G") }
    }

    fn get(&self, errors: &RefCell<Vec<String>>) -> Option<Rc<Regex>> {
        self.compiled
            .get_or_init(|| match build(&self.source) {
                Ok(re) => Some(Rc::new(re)),
                Err(e) => {
                    errors.borrow_mut().push(format!("{e}: {}", short(&self.source)));
                    None
                }
            })
            .clone()
    }
}

fn short(s: &str) -> String {
    let mut t: String = s.chars().take(80).collect();
    if s.chars().count() > 80 {
        t.push('…');
    }
    t
}

fn build(source: &str) -> Result<Regex, String> {
    Regex::new(source)
}

/// Um grupo das capturas: o escopo dele e, às vezes, regras para o texto dele.
struct Capture {
    name: Option<String>,
    /// Uma regra só de `patterns` (o texto do grupo é realçado de novo com elas).
    patterns: Option<RuleId>,
}

type Captures = Vec<Option<Capture>>;

enum Rule {
    Match {
        name: Option<String>,
        re: Re,
        captures: Captures,
    },
    Begin {
        name: Option<String>,
        content_name: Option<String>,
        begin: Re,
        begin_captures: Captures,
        /// O `end` (ou o `while`), com os `\1` ainda por trocar.
        end: String,
        end_re: Re,
        end_has_backrefs: bool,
        end_captures: Captures,
        is_while: bool,
        apply_end_last: bool,
        patterns: Vec<RuleId>,
    },
    /// Só uma lista (um `include`, um `patterns` sem `match`/`begin`).
    List(Vec<RuleId>),
}

/// Uma gramática compilada (com as que ela inclui de outras linguagens).
pub struct Grammar {
    pub scope_name: String,
    rules: Vec<Rule>,
    root: RuleId,
    root_scope: u32,
    errors: RefCell<Vec<String>>,
    /// As listas achatadas (só `match` e `begin`) de cada regra.
    flat: RefCell<HashMap<RuleId, Rc<Vec<RuleId>>>>,
    /// Os `end` com retrovisores já trocados.
    ends: RefCell<HashMap<String, Option<Rc<Regex>>>>,
}

struct Compiler<'v> {
    raws: &'v HashMap<String, Rc<Value>>,
    rules: Vec<Rule>,
    memo: HashMap<usize, RuleId>,
}

impl<'v> Compiler<'v> {
    fn compile(&mut self, value: &'v Value, grammar: &'v Value, repos: &[&'v Map<String, Value>]) -> RuleId {
        let ptr = value as *const Value as usize;
        if let Some(&id) = self.memo.get(&ptr) {
            return id;
        }
        let id = self.rules.len();
        self.rules.push(Rule::List(Vec::new()));
        self.memo.insert(ptr, id);
        let Some(obj) = value.as_object() else { return id };
        let mut repos = repos.to_vec();
        if let Some(r) = obj.get("repository").and_then(Value::as_object) {
            repos.push(r);
        }
        if let Some(include) = obj.get("include").and_then(Value::as_str) {
            let target = self.include(include, grammar, &repos);
            self.rules[id] = Rule::List(target.into_iter().collect());
            return id;
        }
        let patterns: Vec<RuleId> = obj
            .get("patterns")
            .and_then(Value::as_array)
            .map(|list| list.iter().map(|p| self.compile(p, grammar, &repos)).collect())
            .unwrap_or_default();
        let text = |key: &str| obj.get(key).and_then(Value::as_str).map(str::to_string);
        let rule = if let Some(m) = obj.get("match").and_then(Value::as_str) {
            let captures = self.captures(obj.get("captures"), grammar, &repos);
            Rule::Match { name: text("name"), re: Re::new(m), captures }
        } else if let Some(begin) = obj.get("begin").and_then(Value::as_str) {
            let (end, is_while) = match (text("end"), text("while")) {
                (Some(e), _) => (e, false),
                (None, Some(w)) => (w, true),
                (None, None) => ("\u{FFFF}".to_string(), false),
            };
            let captures = obj.get("captures");
            let begin_captures = self.captures(obj.get("beginCaptures").or(captures), grammar, &repos);
            let end_key = if is_while { "whileCaptures" } else { "endCaptures" };
            let end_captures = self.captures(obj.get(end_key).or(captures), grammar, &repos);
            let end_has_backrefs = has_backrefs(&end);
            Rule::Begin {
                name: text("name"),
                content_name: text("contentName"),
                begin: Re::new(begin),
                begin_captures,
                end_re: Re::new(&end),
                end,
                end_has_backrefs,
                end_captures,
                is_while,
                apply_end_last: obj.get("applyEndPatternLast").is_some_and(|v| v.as_bool() == Some(true) || v.as_i64() == Some(1)),
                patterns,
            }
        } else {
            Rule::List(patterns)
        };
        self.rules[id] = rule;
        id
    }

    fn captures(&mut self, value: Option<&'v Value>, grammar: &'v Value, repos: &[&'v Map<String, Value>]) -> Captures {
        let Some(obj) = value.and_then(Value::as_object) else { return Vec::new() };
        let mut out: Captures = Vec::new();
        for (key, cap) in obj {
            let Ok(index) = key.parse::<usize>() else { continue };
            if out.len() <= index {
                out.resize_with(index + 1, || None);
            }
            let name = cap.get("name").and_then(Value::as_str).map(str::to_string);
            let patterns = cap.get("patterns").and_then(Value::as_array).map(|_| self.compile(cap, grammar, repos));
            out[index] = Some(Capture { name, patterns });
        }
        out
    }

    fn include(&mut self, include: &str, grammar: &'v Value, repos: &[&'v Map<String, Value>]) -> Option<RuleId> {
        let root_repos = |g: &'v Value| -> Vec<&'v Map<String, Value>> { g.get("repository").and_then(Value::as_object).into_iter().collect() };
        if include == "$self" || include == "$base" {
            return Some(self.compile(grammar, grammar, &[]));
        }
        if let Some(name) = include.strip_prefix('#') {
            let (i, entry) = repos.iter().enumerate().rev().find_map(|(i, r)| r.get(name).map(|e| (i, e)))?;
            return Some(self.compile(entry, grammar, &repos[..=i]));
        }
        let (scope, name) = include.split_once('#').map_or((include, None), |(s, n)| (s, Some(n)));
        let other: &'v Value = self.raws.get(scope)?;
        match name {
            None => Some(self.compile(other, other, &[])),
            Some(name) => {
                let repos = root_repos(other);
                let entry = repos.first()?.get(name)?;
                Some(self.compile(entry, other, &repos))
            }
        }
    }
}

fn has_backrefs(source: &str) -> bool {
    let b = source.as_bytes();
    (0..b.len().saturating_sub(1)).any(|i| b[i] == b'\\' && b[i + 1].is_ascii_digit() && (i == 0 || b[i - 1] != b'\\'))
}

/// As linguagens que uma gramática inclui (`source.js`, `text.html.basic#tags`...).
fn external_scopes(value: &Value, out: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            if let Some(inc) = map.get("include").and_then(Value::as_str) {
                if !inc.starts_with('#') && !inc.starts_with('$') {
                    out.insert(inc.split('#').next().unwrap_or(inc).to_string());
                }
            }
            for v in map.values() {
                external_scopes(v, out);
            }
        }
        Value::Array(list) => list.iter().for_each(|v| external_scopes(v, out)),
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Estado de uma linha

/// Uma regra aberta (um `begin` sem o `end` ainda).
#[derive(Clone, Debug)]
struct Frame {
    rule: RuleId,
    /// O `end` com os retrovisores trocados (`None`: o da regra).
    end: Option<Rc<str>>,
    name_scope: u32,
    content_scope: u32,
    /// O `begin` foi até o fim da linha (o `\G` vale no começo da seguinte).
    eol: bool,
    /// Onde a busca que achou o `begin` começou, na linha em que entrou (contra laço sem
    /// avanço).
    entered: Option<usize>,
    /// O `G` de fora quando entrou (volta a valer ao fechar), na mesma linha.
    outer_anchor: usize,
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule && self.end == other.end && self.name_scope == other.name_scope && self.content_scope == other.content_scope && self.eol == other.eol
    }
}

/// As regras abertas no fim de uma linha.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineState {
    frames: Vec<Frame>,
}


enum Candidate {
    End,
    Rule(RuleId),
}

/// O cache das buscas numa linha: cada expressão lembra o achado a partir de uma posição
/// (vale enquanto a busca seguinte começa antes dele).
#[derive(Default)]
struct LineSearch {
    cache: HashMap<usize, (usize, Option<Found>)>,
}

impl Grammar {
    /// O tipo da linguagem: o escopo de fora (`source.rust`).
    pub fn root_scope(&self) -> u32 {
        self.root_scope
    }

    fn flat(&self, id: RuleId) -> Rc<Vec<RuleId>> {
        if let Some(list) = self.flat.borrow().get(&id) {
            return list.clone();
        }
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        let start: &[RuleId] = match &self.rules[id] {
            Rule::Begin { patterns, .. } => patterns,
            Rule::List(list) => list,
            Rule::Match { .. } => &[],
        };
        for &p in start {
            self.flatten(p, &mut out, &mut seen);
        }
        let list = Rc::new(out);
        self.flat.borrow_mut().insert(id, list.clone());
        list
    }

    fn flatten(&self, id: RuleId, out: &mut Vec<RuleId>, seen: &mut HashSet<RuleId>) {
        if !seen.insert(id) {
            return;
        }
        match &self.rules[id] {
            Rule::List(list) => {
                for &p in list {
                    self.flatten(p, out, seen);
                }
            }
            _ => out.push(id),
        }
    }

    fn end_regex(&self, frame: &Frame) -> Option<Rc<Regex>> {
        let Rule::Begin { end_re, .. } = &self.rules[frame.rule] else { return None };
        match &frame.end {
            None => end_re.get(&self.errors),
            Some(src) => {
                if let Some(re) = self.ends.borrow().get(&**src) {
                    return re.clone();
                }
                let re = build(src).ok().map(Rc::new);
                self.ends.borrow_mut().insert(src.to_string(), re.clone());
                re
            }
        }
    }

    fn search(&self, re: &Regex, anchored: bool, key: usize, line: &str, pos: usize, anchor: usize, cache: &mut LineSearch) -> Option<Found> {
        if !anchored {
            if let Some((from, found)) = cache.cache.get(&key) {
                if *from <= pos && found.as_ref().is_none_or(|f| f[0].is_some_and(|(s, _)| s >= pos)) {
                    return found.clone();
                }
            }
        }
        let found = re.captures(line, pos, pos == anchor).map_err(|e| self.errors.borrow_mut().push(format!("{e} (ao realçar): {}", short(re.as_str())))).ok().flatten();
        if !anchored {
            cache.cache.insert(key, (pos, found.clone()));
        }
        found
    }

    /// Realça uma linha (sem o `\n`) a partir do estado do fim da anterior; devolve os
    /// trechos (em caracteres) e deixa em `state` o do fim desta.
    pub fn tokenize_line(&self, text: &str, state: &mut LineState) -> Vec<Span> {
        let mut line = String::with_capacity(text.len() + 1);
        line.push_str(text);
        line.push('\n');
        for f in &mut state.frames {
            f.entered = None;
            f.outer_anchor = usize::MAX;
        }
        let mut tokens: Vec<(usize, u32)> = Vec::new();
        let mut search = LineSearch::default();
        let mut pos = 0;
        let mut anchor = if state.frames.last().is_some_and(|f| f.eol) { 0 } else { usize::MAX };
        // As regras `while` abertas: cada linha precisa confirmar (senão fecham).
        let mut i = 0;
        while i < state.frames.len() {
            let f = state.frames[i].clone();
            if let Rule::Begin { is_while: true, end_captures, .. } = &self.rules[f.rule] {
                let found = self.end_regex(&f).and_then(|re| re.captures(&line, pos, pos == anchor).ok().flatten());
                match found {
                    Some(found) if found[0].is_some() => {
                        let e = found[0].map_or(pos, |(_, e)| e);
                        self.emit(&mut tokens, pos, f.content_scope);
                        self.captures(&mut tokens, &line, &found, end_captures, f.name_scope);
                        pos = e;
                        anchor = e;
                    }
                    _ => {
                        state.frames.truncate(i);
                        break;
                    }
                }
            }
            i += 1;
        }
        self.run(&line, &mut pos, line.len(), &mut anchor, &mut state.frames, &mut tokens, &mut search);
        to_spans(text, &line, &tokens)
    }

    fn emit(&self, tokens: &mut Vec<(usize, u32)>, at: usize, scope: u32) {
        if let Some(last) = tokens.last_mut() {
            if last.0 == at {
                last.1 = scope;
                return;
            }
            if last.1 == scope {
                return;
            }
        }
        tokens.push((at, scope));
    }

    #[allow(clippy::too_many_arguments)]
    fn run(&self, line: &str, pos: &mut usize, limit: usize, anchor: &mut usize, frames: &mut Vec<Frame>, tokens: &mut Vec<(usize, u32)>, search: &mut LineSearch) {
        loop {
            let top = frames.last().cloned();
            let scope = top.as_ref().map_or(self.root_scope, |f| f.content_scope);
            if *pos >= limit {
                break;
            }
            let context = top.as_ref().map_or(self.root, |f| f.rule);
            let list = self.flat(context);
            let mut best: Option<(Found, Candidate)> = None;
            let consider = |found: Option<Found>, cand: Candidate, best: &mut Option<(Found, Candidate)>| {
                let Some(found) = found else { return };
                let Some((s, _)) = found[0] else { return };
                if s >= limit {
                    return;
                }
                if best.as_ref().is_none_or(|(b, _)| s < b[0].map_or(usize::MAX, |x| x.0)) {
                    *best = Some((found, cand));
                }
            };
            let end_last = top.as_ref().is_some_and(|f| matches!(self.rules[f.rule], Rule::Begin { apply_end_last: true, .. }));
            let end_found = |this: &Self, search: &mut LineSearch| -> Option<Found> {
                let f = top.as_ref()?;
                if !matches!(this.rules[f.rule], Rule::Begin { is_while: false, .. }) {
                    return None;
                }
                let re = this.end_regex(f)?;
                let key = Rc::as_ptr(&re) as usize;
                let anchored = re.as_str().contains("\\G");
                this.search(&re, anchored, key, line, *pos, *anchor, search)
            };
            if !end_last {
                let found = end_found(self, search);
                consider(found, Candidate::End, &mut best);
            }
            for &id in list.iter() {
                // Um achado bem onde a busca começa já ganhou (no empate vale o primeiro da
                // lista): as regras seguintes nem precisam procurar.
                if best.as_ref().is_some_and(|(b, _)| b[0].is_some_and(|(s, _)| s == *pos)) {
                    break;
                }
                let re = match &self.rules[id] {
                    Rule::Match { re, .. } => re,
                    Rule::Begin { begin, .. } => begin,
                    Rule::List(_) => continue,
                };
                let Some(compiled) = re.get(&self.errors) else { continue };
                let found = self.search(&compiled, re.anchored, Rc::as_ptr(&compiled) as usize, line, *pos, *anchor, search);
                consider(found, Candidate::Rule(id), &mut best);
            }
            if end_last {
                let found = end_found(self, search);
                consider(found, Candidate::End, &mut best);
            }
            let Some((found, cand)) = best else {
                self.emit(tokens, *pos, scope);
                break;
            };
            let (start, end) = found[0].unwrap_or((*pos, *pos));
            let end = end.min(limit);
            self.emit(tokens, *pos, scope);
            match cand {
                Candidate::End => {
                    let Some(f) = frames.pop() else { break };
                    if let Rule::Begin { end_captures, .. } = &self.rules[f.rule] {
                        self.emit(tokens, start, f.name_scope);
                        self.captures(tokens, line, &found, end_captures, f.name_scope);
                    }
                    let parent = frames.last().map_or(self.root_scope, |p| p.content_scope);
                    if end == *pos && f.entered == Some(*pos) {
                        // Entrou e saiu sem andar: para a linha aqui.
                        self.emit(tokens, end, parent);
                        *pos = limit;
                        break;
                    }
                    self.emit(tokens, end, parent);
                    *pos = end;
                    *anchor = f.outer_anchor;
                }
                Candidate::Rule(id) => match &self.rules[id] {
                    Rule::Match { name, captures, .. } => {
                        let inner = name.as_ref().map_or(scope, |n| push_scope(scope, &substitute(n, line, &found)));
                        self.emit(tokens, start, inner);
                        self.captures(tokens, line, &found, captures, inner);
                        self.emit(tokens, end, scope);
                        if end == *pos {
                            // Achado vazio: não anda sozinho.
                            *pos = limit;
                            break;
                        }
                        *pos = end;
                    }
                    Rule::Begin { name, content_name, begin_captures, end: end_src, end_has_backrefs, .. } => {
                        let name_scope = name.as_ref().map_or(scope, |n| push_scope(scope, &substitute(n, line, &found)));
                        self.emit(tokens, start, name_scope);
                        self.captures(tokens, line, &found, begin_captures, name_scope);
                        let content_scope = content_name.as_ref().map_or(name_scope, |n| push_scope(name_scope, &substitute(n, line, &found)));
                        self.emit(tokens, end, content_scope);
                        let resolved = end_has_backrefs.then(|| Rc::<str>::from(resolve_backrefs(end_src, line, &found)));
                        if end == *pos && top.as_ref().is_some_and(|t| t.rule == id) {
                            *pos = limit;
                            break;
                        }
                        frames.push(Frame { rule: id, end: resolved, name_scope, content_scope, eol: end == line.len(), entered: Some(*pos), outer_anchor: *anchor });
                        *pos = end;
                        *anchor = end;
                    }
                    Rule::List(_) => {}
                },
            }
        }
    }

    /// Os grupos do achado com o escopo de cada um (e o realce de novo dos que têm regras).
    fn captures(&self, tokens: &mut Vec<(usize, u32)>, line: &str, found: &Found, rules: &Captures, whole: u32) {
        let Some((ms, me)) = found.first().copied().flatten() else { return };
        if rules.is_empty() || ms == me {
            return;
        }
        let mut scopes = vec![whole; me - ms];
        for (i, cap) in rules.iter().enumerate() {
            let Some(cap) = cap else { continue };
            let Some((s, e)) = found.get(i).copied().flatten() else { continue };
            let (s, e) = (s.max(ms), e.min(me));
            if s >= e {
                continue;
            }
            if let Some(name) = &cap.name {
                let name = substitute(name, line, found);
                let mut k = s;
                while k < e {
                    let run = scopes[k - ms];
                    let mut j = k;
                    while j < e && scopes[j - ms] == run {
                        j += 1;
                    }
                    let pushed = push_scope(run, &name);
                    scopes[k - ms..j - ms].fill(pushed);
                    k = j;
                }
            }
            if let Some(rule) = cap.patterns {
                // O texto do grupo realçado só com as regras dele.
                let base = scopes[s - ms];
                let mut frames = vec![Frame { rule, end: None, name_scope: base, content_scope: base, eol: false, entered: None, outer_anchor: usize::MAX }];
                let mut sub = Vec::new();
                let mut p = s;
                let mut anchor = s;
                self.run(line, &mut p, e, &mut anchor, &mut frames, &mut sub, &mut LineSearch::default());
                for (n, &(at, scope)) in sub.iter().enumerate() {
                    let to = sub.get(n + 1).map_or(e, |x| x.0).min(e);
                    if at < to && at >= s {
                        scopes[at - ms..to - ms].fill(scope);
                    }
                }
            }
        }
        let mut k = ms;
        while k < me {
            let run = scopes[k - ms];
            self.emit(tokens, k, run);
            while k < me && scopes[k - ms] == run {
                k += 1;
            }
        }
    }

    /// Compila todas as expressões e devolve as que o motor não aceitou.
    pub fn check(&self) -> Vec<String> {
        for rule in &self.rules {
            match rule {
                Rule::Match { re, .. } => {
                    re.get(&self.errors);
                }
                Rule::Begin { begin, end_re, end_has_backrefs, .. } => {
                    begin.get(&self.errors);
                    if !end_has_backrefs {
                        end_re.get(&self.errors);
                    }
                }
                Rule::List(_) => {}
            }
        }
        self.errors.borrow().clone()
    }
}

/// `$1`, `${1:/downcase}` e `${1:/upcase}` num nome de escopo.
fn substitute(name: &str, line: &str, found: &Found) -> String {
    if !name.contains('$') {
        return name.to_string();
    }
    let group = |i: usize| found.get(i).copied().flatten().map_or("", |(s, e)| &line[s..e]);
    let mut out = String::new();
    let mut rest = name;
    while let Some(p) = rest.find('$') {
        out.push_str(&rest[..p]);
        rest = &rest[p + 1..];
        if let Some(inner) = rest.strip_prefix('{') {
            if let Some(close) = inner.find('}') {
                let body = &inner[..close];
                let (num, op) = body.split_once(":/").unwrap_or((body, ""));
                if let Ok(i) = num.parse::<usize>() {
                    let text = group(i).trim_start_matches('.');
                    match op {
                        "downcase" => out.push_str(&text.to_lowercase()),
                        "upcase" => out.push_str(&text.to_uppercase()),
                        _ => out.push_str(text),
                    }
                    rest = &inner[close + 1..];
                    continue;
                }
            }
            out.push('$');
        } else {
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            if digits.is_empty() {
                out.push('$');
            } else {
                out.push_str(group(digits.parse().unwrap_or(0)).trim_start_matches('.'));
                rest = &rest[digits.len()..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// O `end` com os `\1` trocados pelo texto (escapado) do `begin`.
fn resolve_backrefs(end: &str, line: &str, found: &Found) -> String {
    let mut out = String::new();
    let mut chars = end.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c == '\\' {
            match chars.peek().copied() {
                Some((_, d)) if d.is_ascii_digit() => {
                    chars.next();
                    let i = d.to_digit(10).unwrap_or(0) as usize;
                    let text = found.get(i).copied().flatten().map_or("", |(s, e)| &line[s..e]);
                    out.push_str(&regex_engine::escape(text));
                }
                Some((_, other)) => {
                    chars.next();
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Dos começos (em bytes, na linha com o `\n`) para os trechos em caracteres.
fn to_spans(text: &str, line: &str, tokens: &[(usize, u32)]) -> Vec<Span> {
    let char_at = |byte: usize| text.get(..byte.min(text.len())).map_or_else(|| text.char_indices().take_while(|(b, _)| *b < byte).count(), |t| t.chars().count());
    let total = text.chars().count();
    let mut spans: Vec<Span> = Vec::new();
    for (n, &(at, scope)) in tokens.iter().enumerate() {
        let to = tokens.get(n + 1).map_or(line.len(), |x| x.0);
        let (s, e) = (char_at(at), char_at(to).min(total));
        if s >= e {
            continue;
        }
        let kind = scope_kind(scope);
        match spans.last_mut() {
            Some(last) if last.scope == scope && last.end == s => last.end = e,
            _ => spans.push(Span { start: s, end: e, kind, scope }),
        }
    }
    if spans.is_empty() {
        spans.push(Span { start: 0, end: total, kind: TokenKind::Plain, scope: 0 });
    }
    spans
}

// ---------------------------------------------------------------------------
// Cache das linhas

/// O realce de um texto, refeito só a partir da linha mudada e, num arquivo grande, aos
/// poucos (um tanto por quadro, de cima para baixo; as linhas que faltam ficam com as cores
/// de antes).
#[derive(Default)]
pub struct LineCache {
    scope_name: String,
    lines: Vec<String>,
    /// O estado no começo de cada linha (vale nas `valid` primeiras).
    starts: Vec<LineState>,
    spans: Rc<Vec<Vec<Span>>>,
    /// Quantas linhas, de cima, já estão certas.
    valid: usize,
    /// O estado no começo da linha `valid`.
    next: LineState,
}

impl LineCache {
    /// Os trechos de cada linha de `text`, aproveitando o que não mudou, e se ficou tudo
    /// pronto. Com `budget`, para quando o tempo acaba (chame de novo, com o mesmo texto, para
    /// continuar). Quem segura o `Rc` devolvido da vez anterior faz a próxima copiar tudo:
    /// solte antes de chamar.
    pub fn update(&mut self, grammar: &Grammar, text: &str, budget: Option<std::time::Duration>) -> (Rc<Vec<Vec<Span>>>, bool) {
        if self.scope_name != grammar.scope_name {
            *self = Self { scope_name: grammar.scope_name.clone(), ..Self::default() };
        }
        let new: Vec<&str> = text.split('\n').collect();
        let old_len = self.lines.len();
        let first = self.lines.iter().zip(&new).take_while(|(a, b)| a.as_str() == **b).count();
        let max_suffix = old_len.min(new.len()) - first;
        let suffix = (0..max_suffix).take_while(|&k| self.lines[old_len - 1 - k] == new[new.len() - 1 - k]).count();
        let (old_mid, new_mid) = (old_len - suffix, new.len() - suffix);
        let (old_valid, old_next) = (self.valid, self.next.clone());
        // De onde refazer: a primeira linha mudada ou a primeira que ainda faltava.
        let start = first.min(old_valid);
        let mut state = if start < old_valid { self.starts[start].clone() } else { old_next.clone() };
        // Troca o miolo; o fim que não mudou fica com os estados e trechos de antes.
        let count = new_mid - first;
        self.lines.splice(first..old_mid, new[first..new_mid].iter().map(|s| s.to_string()));
        self.starts.splice(first..old_mid, std::iter::repeat_n(LineState::default(), count));
        let spans = Rc::make_mut(&mut self.spans);
        spans.splice(first..old_mid, new[first..new_mid].iter().map(|l| vec![Span { start: 0, end: l.chars().count(), kind: TokenKind::Plain, scope: 0 }]));
        // As linhas do fim que já estavam certas, nos números novos.
        let suffix_valid_end = (old_valid + new.len()).saturating_sub(old_len).max(new_mid);
        let deadline = budget.map(|b| web_time::Instant::now() + b);
        let mut i = start;
        while i < new.len() {
            // No fim que não mudou, com o mesmo estado: até onde estava certo, é igual.
            if i >= new_mid && i < suffix_valid_end && self.starts[i] == state {
                i = suffix_valid_end;
                state = old_next.clone();
                continue;
            }
            if deadline.is_some_and(|d| web_time::Instant::now() >= d) {
                break;
            }
            self.starts[i] = state.clone();
            spans[i] = grammar.tokenize_line(&self.lines[i], &mut state);
            i += 1;
        }
        self.valid = i.min(new.len());
        self.next = state;
        (self.spans.clone(), self.valid >= new.len())
    }
}

// ---------------------------------------------------------------------------
// Configuração da linguagem

/// O `language-configuration.json` do VS Code: comentários, pares, fechamento automático,
/// regions e recuo.
#[derive(Clone, Debug, Default)]
pub struct LanguageConfig {
    pub line_comment: Option<String>,
    pub block_comment: Option<(String, String)>,
    pub brackets: Vec<(String, String)>,
    pub auto_closing: Vec<(String, String)>,
    /// O começo e o fim de uma region (`folding.markers`).
    pub region: Option<(regex::Regex, regex::Regex)>,
    pub increase_indent: Option<regex::Regex>,
    pub decrease_indent: Option<regex::Regex>,
}

impl LanguageConfig {
    /// Lê o JSON (aceita os comentários e as vírgulas sobrando do VS Code).
    pub fn parse(text: &str) -> Result<Self, String> {
        let v: Value = serde_json::from_str(&strip_jsonc(text)).map_err(|e| e.to_string())?;
        let pair = |v: &Value| -> Option<(String, String)> {
            match v {
                Value::Array(a) => Some((a.first()?.as_str()?.to_string(), a.get(1)?.as_str()?.to_string())),
                Value::Object(o) => Some((o.get("open")?.as_str()?.to_string(), o.get("close")?.as_str()?.to_string())),
                _ => None,
            }
        };
        let list = |key: &str| v.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(pair).collect()).unwrap_or_default();
        let pattern = |v: Option<&Value>| -> Option<regex::Regex> {
            let src = match v? {
                Value::String(s) => s.clone(),
                Value::Object(o) => o.get("pattern")?.as_str()?.to_string(),
                _ => return None,
            };
            regex::Regex::new(&src).ok()
        };
        let markers = v.get("folding").and_then(|f| f.get("markers"));
        let region = match (pattern(markers.and_then(|m| m.get("start"))), pattern(markers.and_then(|m| m.get("end")))) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        };
        let indent = v.get("indentationRules");
        Ok(Self {
            line_comment: v.get("comments").and_then(|c| c.get("lineComment")).and_then(Value::as_str).map(str::to_string),
            block_comment: v.get("comments").and_then(|c| c.get("blockComment")).and_then(pair),
            brackets: list("brackets"),
            auto_closing: list("autoClosingPairs"),
            region,
            increase_indent: pattern(indent.and_then(|i| i.get("increaseIndentPattern"))),
            decrease_indent: pattern(indent.and_then(|i| i.get("decreaseIndentPattern"))),
        })
    }

    /// `Some(true)` se a linha abre uma region, `Some(false)` se fecha.
    pub fn region_mark(&self, line: &str) -> Option<bool> {
        let (start, end) = self.region.as_ref()?;
        if start.is_match(line) {
            Some(true)
        } else if end.is_match(line) {
            Some(false)
        } else {
            None
        }
    }
}

/// Tira os comentários (`//` e `/* */`, fora dos textos) e as vírgulas antes de `]`/`}`.
pub fn strip_jsonc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let c: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut in_str = false;
    // A última vírgula, enquanto só vieram espaços e comentários depois dela.
    let mut comma: Option<usize> = None;
    while i < c.len() {
        let ch = c[i];
        if in_str {
            out.push(ch);
            if ch == '\\' && i + 1 < c.len() {
                out.push(c[i + 1]);
                i += 2;
                continue;
            }
            if ch == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if ch == '"' {
            in_str = true;
            comma = None;
            out.push(ch);
            i += 1;
        } else if ch == '/' && c.get(i + 1) == Some(&'/') {
            while i < c.len() && c[i] != '\n' {
                i += 1;
            }
        } else if ch == '/' && c.get(i + 1) == Some(&'*') {
            i += 2;
            while i + 1 < c.len() && !(c[i] == '*' && c[i + 1] == '/') {
                i += 1;
            }
            i += 2;
        } else if ch == ',' {
            comma = Some(out.len());
            out.push(ch);
            i += 1;
        } else {
            if !ch.is_whitespace() {
                match comma.take() {
                    Some(p) if matches!(ch, ']' | '}') => {
                        out.remove(p);
                    }
                    _ => {}
                }
            }
            out.push(ch);
            i += 1;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Registro

#[derive(Default)]
struct Registry {
    /// As gramáticas lidas, pelo `scopeName`.
    raws: HashMap<String, Rc<Value>>,
    /// Linguagem (`rust`, `csharp`...) → `scopeName`.
    languages: HashMap<String, String>,
    compiled: HashMap<String, Rc<Grammar>>,
    configs: HashMap<String, Rc<LanguageConfig>>,
    /// Extensão (sem o ponto) → linguagem, das importadas.
    extensions: HashMap<String, String>,
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::default();
}

/// Lê uma gramática (`.tmLanguage.json` ou o `.tmLanguage` em plist) e a devolve como JSON.
pub fn parse_grammar(source: &str) -> Result<Value, String> {
    let trimmed = source.trim_start_matches('\u{FEFF}').trim_start();
    let value: Value = if trimmed.starts_with('<') { plist::parse(trimmed)? } else { serde_json::from_str(&strip_jsonc(trimmed)).map_err(|e| e.to_string())? };
    if value.get("scopeName").and_then(Value::as_str).is_none() {
        return Err("a gramática não tem scopeName".into());
    }
    Ok(value)
}

/// Registra a gramática da linguagem `language` (`""`: só para ser incluída por outras,
/// como a das expressões regulares do Python). Devolve o `scopeName`.
pub fn register_grammar(language: &str, source: &str) -> Result<String, String> {
    let value = parse_grammar(source)?;
    let scope = value.get("scopeName").and_then(Value::as_str).unwrap_or_default().to_string();
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        r.raws.insert(scope.clone(), Rc::new(value));
        // As compiladas que podiam incluir esta ficam velhas.
        r.compiled.clear();
        if !language.is_empty() {
            r.languages.insert(language.to_ascii_lowercase(), scope.clone());
        }
    });
    Ok(scope)
}

/// Registra o `language-configuration.json` da linguagem.
pub fn register_language_config(language: &str, json: &str) -> Result<(), String> {
    let config = LanguageConfig::parse(json)?;
    REGISTRY.with(|r| r.borrow_mut().configs.insert(language.to_ascii_lowercase(), Rc::new(config)));
    Ok(())
}

/// A configuração registrada da linguagem.
pub fn language_config(language: &str) -> Option<Rc<LanguageConfig>> {
    REGISTRY.with(|r| r.borrow().configs.get(&language.to_ascii_lowercase()).cloned())
}

/// Os arquivos com estas extensões (`.foo` ou `foo`) abrem na linguagem `language`.
pub fn register_extensions(language: &str, extensions: &[String]) {
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        for ext in extensions {
            r.extensions.insert(ext.trim_start_matches('.').to_ascii_lowercase(), language.to_ascii_lowercase());
        }
    });
}

/// A linguagem registrada para a extensão (sem o ponto).
pub fn language_for_extension(ext: &str) -> Option<String> {
    REGISTRY.with(|r| r.borrow().extensions.get(&ext.to_ascii_lowercase()).cloned())
}

/// A marca de region da linha pelas configurações registradas (`folding.markers`): as
/// linguagens importadas com regions próprias. `None` se nenhuma reconhece.
pub fn region_mark(line: &str) -> Option<bool> {
    // Só as importadas (as embutidas já estão no realce de sempre): é chamada por linha.
    REGISTRY.with(|r| {
        let r = r.borrow();
        if r.extensions.is_empty() {
            return None;
        }
        let imported: HashSet<&String> = r.extensions.values().collect();
        r.configs.iter().filter(|(lang, _)| imported.contains(lang)).find_map(|(_, c)| c.region_mark(line))
    })
}

/// As linguagens com gramática registrada.
pub fn languages() -> Vec<String> {
    let mut list: Vec<String> = REGISTRY.with(|r| r.borrow().languages.keys().cloned().collect());
    list.sort();
    list
}

/// A gramática compilada da linguagem (compilada na primeira vez).
pub fn grammar_for(language: &str) -> Option<Rc<Grammar>> {
    if language.is_empty() {
        return None;
    }
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        let scope = r.languages.get(&language.to_ascii_lowercase())?.clone();
        if let Some(g) = r.compiled.get(&scope) {
            return Some(g.clone());
        }
        // As linguagens incluídas (e as que elas incluem), já lidas.
        let mut needed = HashSet::new();
        let mut todo = vec![scope.clone()];
        let mut raws = HashMap::new();
        while let Some(s) = todo.pop() {
            if !needed.insert(s.clone()) {
                continue;
            }
            let Some(v) = r.raws.get(&s).cloned() else { continue };
            let mut ext = HashSet::new();
            external_scopes(&v, &mut ext);
            todo.extend(ext);
            raws.insert(s, v);
        }
        let grammar = compile(&scope, &raws)?;
        let grammar = Rc::new(grammar);
        r.compiled.insert(scope, grammar.clone());
        Some(grammar)
    })
}

fn compile(scope: &str, raws: &HashMap<String, Rc<Value>>) -> Option<Grammar> {
    let root_value = raws.get(scope)?.clone();
    let mut c = Compiler { raws, rules: Vec::new(), memo: HashMap::new() };
    let root = c.compile(&root_value, &root_value, &[]);
    let rules = c.rules;
    Some(Grammar {
        scope_name: scope.to_string(),
        rules,
        root,
        root_scope: push_scope(0, scope),
        errors: RefCell::default(),
        flat: RefCell::default(),
        ends: RefCell::default(),
    })
}

/// As regras da gramática da linguagem que o motor não aceita (o aviso ao importar).
pub fn grammar_warnings(language: &str) -> Vec<String> {
    grammar_for(language).map(|g| g.check()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINI: &str = r##"{
        "scopeName": "source.mini",
        "patterns": [
            { "include": "#comment" },
            { "match": "\\b(fn|let)\\b", "name": "keyword.control.mini" },
            { "match": "\\b(\\w+)\\s*(?=\\()", "captures": { "1": { "name": "entity.name.function.mini" } } },
            { "begin": "\"", "end": "\"", "name": "string.quoted.double.mini",
              "patterns": [ { "match": "\\\\.", "name": "constant.character.escape.mini" } ] },
            { "begin": "r(#*)\"", "end": "\"\\1", "name": "string.quoted.raw.mini" },
            { "match": "\\b\\d+\\b", "name": "constant.numeric.mini" }
        ],
        "repository": {
            "comment": { "begin": "/\\*", "end": "\\*/", "name": "comment.block.mini" }
        }
    }"##;

    fn kinds(spans: &[Span], text: &str) -> Vec<(String, TokenKind)> {
        let c: Vec<char> = text.chars().collect();
        spans.iter().filter(|s| s.kind != TokenKind::Plain).map(|s| (c[s.start..s.end].iter().collect(), s.kind)).collect()
    }

    #[test]
    fn tokenizes_with_state_across_lines() {
        register_grammar("mini", MINI).unwrap();
        let g = grammar_for("mini").unwrap();
        assert!(g.check().is_empty(), "{:?}", g.check());
        let mut st = LineState::default();
        let line = "fn soma(a) { let s = \"x\\n\"; 42 /* abre";
        let spans = g.tokenize_line(line, &mut st);
        let k = kinds(&spans, line);
        assert!(k.contains(&("fn".into(), TokenKind::Keyword)), "{k:?}");
        assert!(k.contains(&("soma".into(), TokenKind::Function)), "{k:?}");
        assert!(k.contains(&("42".into(), TokenKind::Number)), "{k:?}");
        assert!(k.iter().any(|(t, kind)| t.starts_with('"') && *kind == TokenKind::String), "{k:?}");
        assert!(k.contains(&("/* abre".into(), TokenKind::Comment)), "{k:?}");
        // O comentário continua na linha de baixo e fecha nela.
        let line2 = "fecha */ let";
        let k2 = kinds(&g.tokenize_line(line2, &mut st), line2);
        assert_eq!(k2, vec![("fecha */".into(), TokenKind::Comment), ("let".into(), TokenKind::Keyword)]);
        assert_eq!(st, LineState::default());
    }

    #[test]
    fn end_with_backreference_closes_only_on_the_same_marks() {
        register_grammar("mini", MINI).unwrap();
        let g = grammar_for("mini").unwrap();
        let mut st = LineState::default();
        let line = "r##\"um \"# dois\"## 7";
        let k = kinds(&g.tokenize_line(line, &mut st), line);
        assert_eq!(k[0], ("r##\"um \"# dois\"##".into(), TokenKind::String), "{k:?}");
        assert_eq!(k[1], ("7".into(), TokenKind::Number));
    }

    #[test]
    fn the_cache_redoes_only_until_the_state_matches() {
        register_grammar("mini", MINI).unwrap();
        let g = grammar_for("mini").unwrap();
        let mut cache = LineCache::default();
        let text: String = (0..2000).map(|i| format!("fn f{i}(x) {{ let y = {i}; }}\n")).collect();
        let first = cache.update(&g, &text, None).0;
        assert_eq!(first.len(), 2001);
        // Abrir um comentário muda o resto; fechar volta.
        let opened = text.replacen("let y = 5;", "let y = 5; /*", 1);
        let spans = cache.update(&g, &opened, None).0;
        assert_eq!(spans[1999][0].kind, TokenKind::Comment);
        let t = std::time::Instant::now();
        let edited = text.replacen("let y = 5;", "let y = 55;", 1);
        let spans = cache.update(&g, &edited, None).0;
        assert_eq!(spans[1999][0].kind, TokenKind::Keyword);
        let t2 = std::time::Instant::now();
        let again = edited.replacen("let y = 55;", "let y = 555;", 1);
        cache.update(&g, &again, None).0;
        let small = t2.elapsed();
        assert!(small < std::time::Duration::from_millis(50), "uma tecla levou {small:?} (refazer tudo: {:?})", t2 - t);
    }

    #[test]
    fn a_big_text_is_highlighted_a_little_per_call() {
        register_grammar("mini", MINI).unwrap();
        let g = grammar_for("mini").unwrap();
        let text: String = (0..500).map(|i| format!("fn f{i}() {{}}\n")).collect();
        let mut cache = LineCache::default();
        let (spans, done) = cache.update(&g, &text, Some(std::time::Duration::ZERO));
        assert!(!done);
        assert_eq!(spans.len(), 501, "as linhas que faltam ficam simples");
        assert_eq!(spans[400][0].kind, TokenKind::Plain);
        drop(spans);
        // Editando no meio do caminho: continua de onde estava e chega ao fim.
        let edited = text.replacen("fn f3()", "fn f3() /*", 1);
        let mut calls = 0;
        let spans = loop {
            calls += 1;
            let (spans, done) = cache.update(&g, &edited, Some(std::time::Duration::from_micros(200)));
            if done {
                break spans;
            }
        };
        assert!(calls > 1);
        assert_eq!(spans[400][0].kind, TokenKind::Comment, "o comentário aberto vale até o fim");
        let full = LineCache::default().update(&g, &edited, None).0;
        assert_eq!(*spans, *full);
    }

    #[test]
    fn plist_grammars_and_jsonc_configs_are_read() {
        let xml = r#"<plist><dict><key>scopeName</key><string>source.p</string><key>patterns</key><array>
            <dict><key>match</key><string>\b(?i:begin|end)\b</string><key>name</key><string>keyword.p</string></dict></array></dict></plist>"#;
        register_grammar("p", xml).unwrap();
        let g = grammar_for("p").unwrap();
        let line = "BEGIN x end";
        let k = kinds(&g.tokenize_line(line, &mut LineState::default()), line);
        assert_eq!(k, vec![("BEGIN".into(), TokenKind::Keyword), ("end".into(), TokenKind::Keyword)]);
        let cfg = LanguageConfig::parse(
            r#"{ // comentário
            "comments": { "lineComment": "//", "blockComment": ["/*", "*/"] },
            "autoClosingPairs": [ { "open": "{", "close": "}" }, ["(", ")"], // sobra
            ],
            "folding": { "markers": { "start": "^\\s*//\\s*#region\\b", "end": "^\\s*//\\s*#endregion\\b" } }
        }"#,
        )
        .unwrap();
        assert_eq!(cfg.line_comment.as_deref(), Some("//"));
        assert_eq!(cfg.auto_closing.len(), 2);
        assert_eq!(cfg.region_mark("  // #region Cálculos"), Some(true));
        assert_eq!(cfg.region_mark("// #endregion"), Some(false));
    }

    #[test]
    fn scope_names_substitute_captures() {
        let found: Found = vec![Some((0, 3)), Some((0, 3))];
        assert_eq!(substitute("keyword.$1.x", "abc", &found), "keyword.abc.x");
        assert_eq!(substitute("k.${1:/upcase}", "abc", &found), "k.ABC");
    }
}
