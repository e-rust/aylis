//! Expressões sobre os campos de um registro: o `filter` do dataset e os campos
//! calculados (`expression` do [`Field`](crate::Field)).
//!
//! ```text
//! cidade = 'São Paulo' and limite >= 1000
//! nome like 'mar%' or not ativo
//! uf in ('SP', 'RJ') and data_cadastro >= '2024-01-01'
//! quantidade * preco_unitario            (campo calculado)
//! upper(nome) + ' (' + uf + ')'
//! iif(limite > 5000, 'VIP', 'Normal')
//! ```
//!
//! - comparações: `=`, `<>` (ou `!=`), `<`, `<=`, `>`, `>=`, `like` (com `%` e `_`,
//!   sem diferenciar maiúsculas), `in (...)`, `is null`, `is not null`;
//! - texto compara sem diferenciar maiúsculas e minúsculas;
//! - lógicos: `and`, `or`, `not`; contas: `+ - * / %` (`+` junta textos);
//! - funções: `upper`, `lower`, `trim`, `len`, `substr(t, início, tamanho)`, `round(x, casas, modo)`,
//!   `abs`, `coalesce(a, b)`, `iif(condição, a, b)`, `year`, `month`, `day`, `today()` e
//!   `now()` (no horário local da tela);
//! - nomes de campo com espaço ou acento: `[Data de cadastro]`.

use std::cmp::Ordering;

use crate::value::{Date, Decimal, Rounding, Value};

/// Uma expressão já lida (lida uma vez, avaliada em cada registro).
#[derive(Clone, Debug, PartialEq)]
pub struct Expr(Node);

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Const(Value),
    Field(String),
    Unary(char, Box<Node>),
    Binary(Op, Box<Node>, Box<Node>),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Like(Box<Node>, Box<Node>, bool),
    In(Box<Node>, Vec<Node>, bool),
    IsNull(Box<Node>, bool),
    Call(String, Vec<Node>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Expr {
    /// Lê a expressão. O erro diz onde está o problema.
    pub fn parse(text: &str) -> Result<Expr, String> {
        if text.len() > MAX_TEXT {
            return Err(format!("expressão longa demais ({} KB; o máximo é {} KB)", text.len() / 1024, MAX_TEXT / 1024));
        }
        let tokens = tokenize(text)?;
        let mut p = Parser { tokens, pos: 0, depth: 0, nesting: 0 };
        let node = p.or()?;
        if p.pos < p.tokens.len() {
            return Err(format!("sobrou '{}' no fim da expressão", p.tokens[p.pos].text()));
        }
        Ok(Expr(node))
    }

    /// Nomes de campo usados (para conferir que existem); os parâmetros ficam de fora.
    pub fn fields(&self) -> Vec<String> {
        self.names().into_iter().filter(|n| !n.starts_with(':')).collect()
    }

    /// Parâmetros usados (`:minimo` → `minimo`).
    pub fn params(&self) -> Vec<String> {
        self.names().into_iter().filter_map(|n| n.strip_prefix(':').map(str::to_string)).collect()
    }

    fn names(&self) -> Vec<String> {
        fn walk(n: &Node, out: &mut Vec<String>) {
            match n {
                Node::Field(f) => {
                    if !out.iter().any(|o: &String| o.eq_ignore_ascii_case(f)) {
                        out.push(f.clone());
                    }
                }
                Node::Unary(_, a) | Node::Not(a) | Node::IsNull(a, _) => walk(a, out),
                Node::Binary(_, a, b) | Node::And(a, b) | Node::Or(a, b) | Node::Like(a, b, _) => {
                    walk(a, out);
                    walk(b, out);
                }
                Node::In(a, list, _) => {
                    walk(a, out);
                    list.iter().for_each(|x| walk(x, out));
                }
                Node::Call(_, args) => args.iter().for_each(|x| walk(x, out)),
                Node::Const(_) => {}
            }
        }
        let mut out = Vec::new();
        walk(&self.0, &mut out);
        out
    }

    /// Avalia com os valores do registro (`get` devolve o valor de um campo pelo nome).
    pub fn eval(&self, get: &dyn Fn(&str) -> Option<Value>) -> Result<Value, String> {
        eval(&self.0, get)
    }

    /// Avalia como condição (filtro): `Null` e erros contam como falso.
    pub fn matches(&self, get: &dyn Fn(&str) -> Option<Value>) -> bool {
        matches!(self.eval(get), Ok(Value::Bool(true)))
    }
}

// ---------------------------------------------------------------------------
// Tokens

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(String),
    Str(String),
    Ident(String),
    Sym(&'static str),
}

impl Tok {
    fn text(&self) -> String {
        match self {
            Tok::Num(s) | Tok::Ident(s) => s.clone(),
            Tok::Str(s) => format!("'{s}'"),
            Tok::Sym(s) => s.to_string(),
        }
    }

    fn is_word(&self, word: &str) -> bool {
        matches!(self, Tok::Ident(s) if s.eq_ignore_ascii_case(word))
    }
}

fn tokenize(text: &str) -> Result<Vec<Tok>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(char::is_ascii_digit)) {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            out.push(Tok::Num(chars[start..i].iter().collect()));
        } else if c == '\'' || c == '"' {
            let quote = c;
            let mut s = String::new();
            i += 1;
            loop {
                match chars.get(i) {
                    None => return Err("texto sem a aspa de fechamento".into()),
                    Some(&q) if q == quote && chars.get(i + 1) == Some(&quote) => {
                        s.push(quote);
                        i += 2;
                    }
                    Some(&q) if q == quote => {
                        i += 1;
                        break;
                    }
                    Some(&ch) => {
                        s.push(ch);
                        i += 1;
                    }
                }
            }
            out.push(Tok::Str(s));
        } else if c == '[' {
            let end = chars[i..].iter().position(|&x| x == ']').ok_or("nome de campo sem ']'")?;
            out.push(Tok::Ident(chars[i + 1..i + end].iter().collect()));
            i += end + 1;
        } else if c == ':' && chars.get(i + 1).is_some_and(|n| n.is_alphabetic() || *n == '_') {
            // Parâmetro (`:minimo`): lido como um campo de nome ":minimo"; quem avalia
            // entrega o valor.
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            out.push(Tok::Ident(chars[start..i].iter().collect()));
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.') {
                i += 1;
            }
            out.push(Tok::Ident(chars[start..i].iter().collect()));
        } else {
            let two: String = chars[i..(i + 2).min(chars.len())].iter().collect();
            let sym = match two.as_str() {
                "<>" | "!=" => Some("<>"),
                "<=" => Some("<="),
                ">=" => Some(">="),
                "==" => Some("="),
                "&&" => Some("and"),
                "||" => Some("or"),
                _ => None,
            };
            if let Some(s) = sym {
                out.push(Tok::Sym(s));
                i += 2;
                continue;
            }
            let one = match c {
                '=' => "=",
                '<' => "<",
                '>' => ">",
                '+' => "+",
                '-' => "-",
                '*' => "*",
                '/' => "/",
                '%' => "%",
                '(' => "(",
                ')' => ")",
                ',' => ",",
                '!' => "not",
                _ => return Err(format!("caractere inesperado '{c}'")),
            };
            out.push(Tok::Sym(one));
            i += 1;
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Leitura (descida recursiva)

/// Profundidade máxima da árvore: a avaliação e o descarte são recursivos, e um texto como
/// `((((…` ou `a + a + a…` com milhares de termos estouraria a pilha, o que encerra o processo
/// sem chance de tratar. Conferida enquanto a árvore é montada (montada inteira, ela já
/// estouraria ao ser descartada).
const MAX_DEPTH: usize = 256;
/// Tamanho máximo do texto de uma expressão.
const MAX_TEXT: usize = 64 * 1024;

fn too_deep() -> String {
    format!("expressão aninhada demais (o máximo é {MAX_DEPTH} níveis)")
}

struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    /// Profundidade do último nó devolvido.
    depth: usize,
    /// Leituras aninhadas abertas (parênteses, sinais, `not`): limitam a recursão da própria
    /// leitura, que em `((((a))))` não aumenta a profundidade da árvore.
    nesting: usize,
}

impl Parser {
    /// Um nó acima de filhos com profundidade `below`.
    fn node(&mut self, node: Node, below: usize) -> Result<Node, String> {
        self.depth = below + 1;
        if self.depth > MAX_DEPTH {
            return Err(too_deep());
        }
        Ok(node)
    }

    fn enter(&mut self) -> Result<(), String> {
        self.nesting += 1;
        if self.nesting > MAX_DEPTH { Err(too_deep()) } else { Ok(()) }
    }

    fn leave(&mut self) {
        self.nesting -= 1;
    }

    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn eat_sym(&mut self, s: &str) -> bool {
        if matches!(self.peek(), Some(Tok::Sym(x)) if *x == s) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn eat_word(&mut self, w: &str) -> bool {
        if self.peek().is_some_and(|t| t.is_word(w)) || (matches!(self.peek(), Some(Tok::Sym(x)) if *x == w)) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, s: &str) -> Result<(), String> {
        if self.eat_sym(s) {
            Ok(())
        } else {
            Err(format!("esperava '{s}'{}", self.peek().map(|t| format!(" e veio '{}'", t.text())).unwrap_or_default()))
        }
    }

    fn or(&mut self) -> Result<Node, String> {
        let mut left = self.and()?;
        let mut depth = self.depth;
        while self.eat_word("or") {
            let right = self.and()?;
            left = self.node(Node::Or(Box::new(left), Box::new(right)), depth.max(self.depth))?;
            depth = self.depth;
        }
        self.depth = depth;
        Ok(left)
    }

    fn and(&mut self) -> Result<Node, String> {
        let mut left = self.not()?;
        let mut depth = self.depth;
        while self.eat_word("and") {
            let right = self.not()?;
            left = self.node(Node::And(Box::new(left), Box::new(right)), depth.max(self.depth))?;
            depth = self.depth;
        }
        self.depth = depth;
        Ok(left)
    }

    fn not(&mut self) -> Result<Node, String> {
        if self.eat_word("not") {
            self.enter()?;
            let inner = self.not()?;
            self.leave();
            let below = self.depth;
            return self.node(Node::Not(Box::new(inner)), below);
        }
        self.compare()
    }

    fn compare(&mut self) -> Result<Node, String> {
        let left = self.add()?;
        let below = self.depth;
        let op = match self.peek() {
            Some(Tok::Sym("=")) => Some(Op::Eq),
            Some(Tok::Sym("<>")) => Some(Op::Ne),
            Some(Tok::Sym("<")) => Some(Op::Lt),
            Some(Tok::Sym("<=")) => Some(Op::Le),
            Some(Tok::Sym(">")) => Some(Op::Gt),
            Some(Tok::Sym(">=")) => Some(Op::Ge),
            _ => None,
        };
        if let Some(op) = op {
            self.pos += 1;
            let right = self.add()?;
            let below = below.max(self.depth);
            return self.node(Node::Binary(op, Box::new(left), Box::new(right)), below);
        }
        let negated = self.peek().is_some_and(|t| t.is_word("not"))
            && self.tokens.get(self.pos + 1).is_some_and(|t| t.is_word("like") || t.is_word("in"));
        if negated {
            self.pos += 1;
        }
        if self.eat_word("like") {
            let pattern = self.add()?;
            let below = below.max(self.depth);
            return self.node(Node::Like(Box::new(left), Box::new(pattern), negated), below);
        }
        if self.eat_word("in") {
            self.expect("(")?;
            let mut list = vec![self.add()?];
            let mut below = below.max(self.depth);
            while self.eat_sym(",") {
                list.push(self.add()?);
                below = below.max(self.depth);
            }
            self.expect(")")?;
            return self.node(Node::In(Box::new(left), list, negated), below);
        }
        if self.eat_word("is") {
            let not = self.eat_word("not");
            if !self.eat_word("null") {
                return Err("esperava 'null' depois de 'is'".into());
            }
            return self.node(Node::IsNull(Box::new(left), not), below);
        }
        self.depth = below;
        Ok(left)
    }

    fn add(&mut self) -> Result<Node, String> {
        let mut left = self.mul()?;
        let mut depth = self.depth;
        loop {
            let op = if self.eat_sym("+") {
                Op::Add
            } else if self.eat_sym("-") {
                Op::Sub
            } else {
                self.depth = depth;
                return Ok(left);
            };
            let right = self.mul()?;
            left = self.node(Node::Binary(op, Box::new(left), Box::new(right)), depth.max(self.depth))?;
            depth = self.depth;
        }
    }

    fn mul(&mut self) -> Result<Node, String> {
        let mut left = self.unary()?;
        let mut depth = self.depth;
        loop {
            let op = if self.eat_sym("*") {
                Op::Mul
            } else if self.eat_sym("/") {
                Op::Div
            } else if self.eat_sym("%") {
                Op::Mod
            } else {
                self.depth = depth;
                return Ok(left);
            };
            let right = self.unary()?;
            left = self.node(Node::Binary(op, Box::new(left), Box::new(right)), depth.max(self.depth))?;
            depth = self.depth;
        }
    }

    fn unary(&mut self) -> Result<Node, String> {
        if self.eat_sym("-") {
            self.enter()?;
            let inner = self.unary()?;
            self.leave();
            let below = self.depth;
            return self.node(Node::Unary('-', Box::new(inner)), below);
        }
        if self.eat_sym("+") {
            self.enter()?;
            let inner = self.unary()?;
            self.leave();
            return Ok(inner);
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Node, String> {
        let tok = self.peek().cloned().ok_or("expressão incompleta")?;
        self.pos += 1;
        // Folhas; os parênteses devolvem a profundidade do que está dentro.
        self.depth = 1;
        match tok {
            Tok::Num(n) => {
                if n.contains('.') {
                    Decimal::parse(&n).map(|d| Node::Const(Value::Decimal(d))).ok_or_else(|| format!("número inválido '{n}'"))
                } else {
                    n.parse::<i64>().map(|v| Node::Const(Value::Int(v))).map_err(|_| format!("número inválido '{n}'"))
                }
            }
            Tok::Str(s) => Ok(Node::Const(Value::String(s))),
            Tok::Sym("(") => {
                self.enter()?;
                let inner = self.or()?;
                self.expect(")")?;
                self.leave();
                Ok(inner)
            }
            Tok::Ident(name) => {
                match name.to_lowercase().as_str() {
                    "true" | "verdadeiro" => return Ok(Node::Const(Value::Bool(true))),
                    "false" | "falso" => return Ok(Node::Const(Value::Bool(false))),
                    "null" => return Ok(Node::Const(Value::Null)),
                    _ => {}
                }
                if self.eat_sym("(") {
                    self.enter()?;
                    let mut args = Vec::new();
                    let mut below = 0;
                    if !self.eat_sym(")") {
                        args.push(self.or()?);
                        below = self.depth;
                        while self.eat_sym(",") {
                            args.push(self.or()?);
                            below = below.max(self.depth);
                        }
                        self.expect(")")?;
                    }
                    self.leave();
                    return self.node(Node::Call(name.to_lowercase(), args), below);
                }
                Ok(Node::Field(name))
            }
            Tok::Sym(s) => Err(format!("'{s}' fora do lugar")),
        }
    }
}

// ---------------------------------------------------------------------------
// Avaliação

fn eval(node: &Node, get: &dyn Fn(&str) -> Option<Value>) -> Result<Value, String> {
    Ok(match node {
        Node::Const(v) => v.clone(),
        Node::Field(name) => get(name).ok_or_else(|| format!("campo '{name}' não existe"))?,
        Node::Unary(_, a) => match eval(a, get)? {
            Value::Null => Value::Null,
            Value::Int(i) => Value::Int(-i),
            Value::Float(f) => Value::Float(-f),
            Value::Decimal(d) => Value::Decimal(Decimal::new(-d.mantissa(), d.scale())),
            v => return Err(format!("não dá para negar {}", v.kind_name())),
        },
        Node::Not(a) => match truth(&eval(a, get)?) {
            Some(b) => Value::Bool(!b),
            None => Value::Null,
        },
        Node::And(a, b) => {
            let left = truth(&eval(a, get)?);
            if left == Some(false) {
                return Ok(Value::Bool(false));
            }
            match (left, truth(&eval(b, get)?)) {
                (_, Some(false)) => Value::Bool(false),
                (Some(true), Some(true)) => Value::Bool(true),
                _ => Value::Null,
            }
        }
        Node::Or(a, b) => {
            let left = truth(&eval(a, get)?);
            if left == Some(true) {
                return Ok(Value::Bool(true));
            }
            match (left, truth(&eval(b, get)?)) {
                (_, Some(true)) => Value::Bool(true),
                (Some(false), Some(false)) => Value::Bool(false),
                _ => Value::Null,
            }
        }
        Node::IsNull(a, not) => Value::Bool(eval(a, get)?.is_null() != *not),
        Node::Like(a, pattern, negated) => {
            let (v, p) = (eval(a, get)?, eval(pattern, get)?);
            if v.is_null() || p.is_null() {
                return Ok(Value::Null);
            }
            Value::Bool(like(&v.to_string().to_lowercase(), &p.to_string().to_lowercase()) != *negated)
        }
        Node::In(a, list, negated) => {
            let v = eval(a, get)?;
            if v.is_null() {
                return Ok(Value::Null);
            }
            let mut found = false;
            for item in list {
                if compare(&v, &eval(item, get)?) == Some(Ordering::Equal) {
                    found = true;
                    break;
                }
            }
            Value::Bool(found != *negated)
        }
        Node::Binary(op, a, b) => binary(*op, eval(a, get)?, eval(b, get)?)?,
        Node::Call(name, args) => call(name, args, get)?,
    })
}

fn truth(v: &Value) -> Option<bool> {
    match v {
        Value::Null => None,
        Value::Bool(b) => Some(*b),
        Value::Int(i) => Some(*i != 0),
        other => Some(!other.to_string().is_empty()),
    }
}

/// Compara dois valores (texto sem diferenciar maiúsculas; números de tipos diferentes
/// entre si; datas com texto ISO). `None` se não forem comparáveis ou um for `Null`.
pub fn compare(a: &Value, b: &Value) -> Option<Ordering> {
    use Value::*;
    match (a, b) {
        (Null, _) | (_, Null) => None,
        (String(x), String(y)) => Some(x.to_lowercase().cmp(&y.to_lowercase())),
        (Bool(x), Bool(y)) => Some(x.cmp(y)),
        (Date(x), Date(y)) => Some(x.cmp(y)),
        (Time(x), Time(y)) => Some(x.cmp(y)),
        (DateTime(x), DateTime(y)) => Some(x.cmp(y)),
        (Date(x), DateTime(y)) => Some(x.cmp(&y.date)),
        (DateTime(x), Date(y)) => Some(x.date.cmp(y)),
        (Timestamp(x), Timestamp(y)) => Some(x.cmp(y)),
        // Com uma data e hora sem fuso (ou uma data), compara no horário local.
        (Timestamp(x), DateTime(y)) => Some(x.to_local().cmp(y)),
        (DateTime(x), Timestamp(y)) => Some(x.cmp(&y.to_local())),
        (Timestamp(x), Date(y)) => Some(x.to_local().date.cmp(y)),
        (Date(x), Timestamp(y)) => Some(x.cmp(&y.to_local().date)),
        (Date(x), String(s)) => crate::value::Date::parse_iso(s).map(|y| x.cmp(&y)),
        (String(s), Date(y)) => crate::value::Date::parse_iso(s).map(|x| x.cmp(y)),
        (Decimal(x), Decimal(y)) => Some(x.cmp(y)),
        (Int(x), Int(y)) => Some(x.cmp(y)),
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => x.partial_cmp(&y),
            _ => Some(a.to_string().to_lowercase().cmp(&b.to_string().to_lowercase())),
        },
    }
}

/// Ordem para o índice: `Null` antes de tudo.
pub fn sort_order(a: &Value, b: &Value) -> Ordering {
    match (a.is_null(), b.is_null()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => compare(a, b).unwrap_or(Ordering::Equal),
    }
}

fn binary(op: Op, a: Value, b: Value) -> Result<Value, String> {
    let cmp = |want: fn(Ordering) -> bool| Ok(compare(&a, &b).map_or(Value::Null, |o| Value::Bool(want(o))));
    match op {
        Op::Eq => cmp(|o| o == Ordering::Equal),
        Op::Ne => cmp(|o| o != Ordering::Equal),
        Op::Lt => cmp(|o| o == Ordering::Less),
        Op::Le => cmp(|o| o != Ordering::Greater),
        Op::Gt => cmp(|o| o == Ordering::Greater),
        Op::Ge => cmp(|o| o != Ordering::Less),
        _ if a.is_null() || b.is_null() => Ok(Value::Null),
        Op::Add if matches!(a, Value::String(_)) || matches!(b, Value::String(_)) => Ok(Value::String(format!("{a}{b}"))),
        Op::Add | Op::Sub if matches!(a, Value::Date(_)) => {
            let days = b.as_i64().ok_or("só dá para somar dias (inteiros) a uma data")?;
            let Value::Date(d) = a else { unreachable!() };
            Ok(Value::Date(add_days(d, if op == Op::Add { days } else { -days })))
        }
        _ => arithmetic(op, &a, &b),
    }
}

fn add_days(d: Date, days: i64) -> Date {
    Date::from_days(d.to_days() + days).unwrap_or(d)
}

fn op_symbol(op: Op) -> &'static str {
    match op {
        Op::Add => "+",
        Op::Sub => "-",
        Op::Mul => "*",
        Op::Div => "/",
        Op::Mod => "%",
        Op::Eq => "=",
        Op::Ne => "<>",
        Op::Lt => "<",
        Op::Le => "<=",
        Op::Gt => ">",
        Op::Ge => ">=",
    }
}

/// Contas: inteiros ficam inteiros (a divisão vira decimal), decimais ficam exatos (a divisão
/// também), e estouro é erro, nunca um valor errado.
fn arithmetic(op: Op, a: &Value, b: &Value) -> Result<Value, String> {
    let not_number = |v: &Value| format!("'{v}' não é número");
    let overflow = || format!("estouro: {a} {} {b} não cabe", op_symbol(op));
    if let (Value::Int(x), Value::Int(y)) = (a, b) {
        return match op {
            Op::Add => x.checked_add(*y).map(Value::Int).ok_or_else(overflow),
            Op::Sub => x.checked_sub(*y).map(Value::Int).ok_or_else(overflow),
            Op::Mul => x.checked_mul(*y).map(Value::Int).ok_or_else(overflow),
            Op::Mod if *y != 0 => Ok(Value::Int(x % y)),
            // Inteiros são exatos: a divisão também (7 / 2 = 3,5000), sem passar por f64.
            Op::Div if *y != 0 => {
                let (x, y) = (Decimal::from(*x), Decimal::from(*y));
                x.checked_div(&y, x.default_div_scale(&y), Rounding::HalfUp).map(Value::Decimal).ok_or_else(overflow)
            }
            _ => Ok(Value::Null), // divisão por zero
        };
    }
    let decimal = matches!(a, Value::Decimal(_)) || matches!(b, Value::Decimal(_));
    let floaty = matches!(a, Value::Float(_)) || matches!(b, Value::Float(_));
    if decimal && !floaty && op != Op::Mod {
        let (x, y) = (a.as_decimal().ok_or_else(|| not_number(a))?, b.as_decimal().ok_or_else(|| not_number(b))?);
        let result = match op {
            Op::Add => x.checked_add(&y),
            Op::Sub => x.checked_sub(&y),
            // O produto com as casas do maior mais 2 (2,5 × 1,333 = 3,33250), como antes.
            Op::Mul => x.checked_mul(&y).and_then(|p| p.round(p.scale().min(x.scale().max(y.scale()).max(2) + 2), Rounding::HalfUp)),
            // Exata, sem f64 (0,30 / 0,10 = 3): o campo da expressão depois arredonda para as
            // casas e o modo dele.
            Op::Div if y.mantissa() == 0 => return Ok(Value::Null),
            _ => x.checked_div(&y, x.default_div_scale(&y), Rounding::HalfUp),
        };
        return result.map(Value::Decimal).ok_or_else(overflow);
    }
    let (x, y) = (a.as_f64().ok_or_else(|| not_number(a))?, b.as_f64().ok_or_else(|| not_number(b))?);
    let r = match op {
        Op::Add => x + y,
        Op::Sub => x - y,
        Op::Mul => x * y,
        Op::Div if y != 0.0 => x / y,
        Op::Mod if y != 0.0 => x % y,
        _ => return Ok(Value::Null),
    };
    Ok(Value::Float(r))
}

/// `%` = qualquer sequência, `_` = um caractere.
fn like(text: &str, pattern: &str) -> bool {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pattern.chars().collect();
    let (mut ti, mut pi) = (0, 0);
    let (mut star, mut mark) = (None, 0);
    while ti < t.len() {
        if pi < p.len() && (p[pi] == '_' || p[pi] == t[ti]) {
            ti += 1;
            pi += 1;
        } else if pi < p.len() && p[pi] == '%' {
            star = Some(pi);
            mark = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            mark += 1;
            ti = mark;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '%' {
        pi += 1;
    }
    pi == p.len()
}

fn call(name: &str, args: &[Node], get: &dyn Fn(&str) -> Option<Value>) -> Result<Value, String> {
    let values: Vec<Value> = if name == "iif" { Vec::new() } else { args.iter().map(|a| eval(a, get)).collect::<Result<_, _>>()? };
    let arg = |i: usize| values.get(i).cloned().unwrap_or(Value::Null);
    let need = |n: usize| if args.len() < n { Err(format!("{name} precisa de {n} argumento(s)")) } else { Ok(()) };
    let text = |v: &Value| v.to_string();
    Ok(match name {
        "upper" => {
            need(1)?;
            if arg(0).is_null() { Value::Null } else { Value::String(text(&arg(0)).to_uppercase()) }
        }
        "lower" => {
            need(1)?;
            if arg(0).is_null() { Value::Null } else { Value::String(text(&arg(0)).to_lowercase()) }
        }
        "trim" => {
            need(1)?;
            if arg(0).is_null() { Value::Null } else { Value::String(text(&arg(0)).trim().to_string()) }
        }
        "len" | "length" => {
            need(1)?;
            Value::Int(if arg(0).is_null() { 0 } else { text(&arg(0)).chars().count() as i64 })
        }
        "substr" | "substring" => {
            need(2)?;
            let s = text(&arg(0));
            let start = arg(1).as_i64().unwrap_or(1).max(1) as usize - 1;
            let len = values.get(2).and_then(Value::as_i64).map_or(usize::MAX, |l| l.max(0) as usize);
            Value::String(s.chars().skip(start).take(len).collect())
        }
        "abs" => {
            need(1)?;
            match arg(0) {
                Value::Int(i) => Value::Int(i.abs()),
                Value::Decimal(d) => Value::Decimal(Decimal::new(d.mantissa().abs(), d.scale())),
                v => v.as_f64().map_or(Value::Null, |f| Value::Float(f.abs())),
            }
        }
        "round" => {
            need(1)?;
            let places = values.get(1).and_then(Value::as_i64).unwrap_or(0).clamp(0, 12) as u8;
            // round(x, casas, 'half_even'): o modo (half_up, o padrão; half_even, o bancário;
            // truncate; up).
            let mode = match values.get(2) {
                None | Some(Value::Null) => Rounding::HalfUp,
                Some(m) => {
                    let name = m.to_string();
                    Rounding::from_name(&name).ok_or_else(|| format!("arredondamento '{name}' desconhecido (half_up, half_even, truncate, up)"))?
                }
            };
            match arg(0) {
                Value::Null => Value::Null,
                Value::Decimal(d) => Value::Decimal(d.round(places, mode).ok_or_else(|| format!("estouro: round({d}, {places})"))?),
                Value::Int(i) => Value::Int(i),
                v => {
                    let f = v.as_f64().ok_or("round precisa de número")?;
                    let factor = 10f64.powi(places as i32);
                    let r = (f * factor).round() / factor;
                    if places == 0 { Value::Int(r as i64) } else { Value::Float(r) }
                }
            }
        }
        "coalesce" | "ifnull" => values.into_iter().find(|v| !v.is_null()).unwrap_or(Value::Null),
        "iif" | "if" => {
            need(3)?;
            if truth(&eval(&args[0], get)?) == Some(true) { eval(&args[1], get)? } else { eval(&args[2], get)? }
        }
        "year" | "month" | "day" => {
            need(1)?;
            match arg(0).as_date() {
                Some(d) => Value::Int(match name {
                    "year" => d.year() as i64,
                    "month" => d.month() as i64,
                    _ => d.day() as i64,
                }),
                None => Value::Null,
            }
        }
        // No horário local da tela (antes era o dia em UTC: em Brasília, depois das 21h, já
        // era amanhã).
        "today" => Value::Date(Date::today()),
        "now" => Value::DateTime(crate::value::DateTime::now()),
        other => return Err(format!("função desconhecida '{other}'")),
    })
}
