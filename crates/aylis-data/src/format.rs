//! Formatação de números e datas por máscara, respeitando o idioma ([`LocaleInfo`]).
//!
//! - números: `0`, `#`, `,` (milhar), `.` (decimal) e texto em volta, por
//!   exemplo `"R$ #,##0.00"` ou `"0.0 kg"`; `"pos;neg"` dá um formato para negativos;
//! - datas: `dd`, `d`, `mm`, `m`, `yyyy`, `yy`, `hh`, `h`, `nn`, `n`,
//!   `ss`, `s`, `zzz`; o resto é literal. `nn` é minuto.

use aylis_core::platform::LocaleInfo;

use crate::value::{Date, DateTime, Decimal, Time};

/// Formata um número com um padrão no estilo `FormatFloat`. O separador de milhar e o
/// decimal vêm do idioma.
pub fn format_number(value: Decimal, pattern: &str, locale: &LocaleInfo) -> String {
    let (positive, negative) = match pattern.split_once(';') {
        Some((p, n)) => (p, Some(n)),
        None => (pattern, None),
    };
    let (section, use_sign) = match (value.is_negative(), negative) {
        (true, Some(n)) => (n, false),
        (true, None) => (positive, true),
        (false, _) => (positive, false),
    };

    let is_spec = |c: char| matches!(c, '0' | '#' | ',' | '.');
    let start = section.find(is_spec);
    let Some(start) = start else {
        return section.to_string(); // só texto
    };
    let end = section.rfind(is_spec).map_or(start, |i| i + 1);
    let (prefix, spec, suffix) = (&section[..start], &section[start..end], &section[end..]);

    let (int_spec, frac_spec) = spec.split_once('.').unwrap_or((spec, ""));
    let grouping = int_spec.contains(',');
    let min_int = int_spec.chars().filter(|&c| c == '0').count();
    let min_frac = frac_spec.chars().filter(|&c| c == '0').count();
    let max_frac = frac_spec.chars().filter(|&c| matches!(c, '0' | '#')).count();

    let rounded = value.rescale(max_frac as u8);
    let (int, frac) = rounded.parts();
    let mut frac = frac;
    while frac.len() > min_frac && frac.ends_with('0') {
        frac.pop();
    }
    let mut int_text = int.to_string();
    if int == 0 && min_int == 0 {
        int_text.clear();
    }
    while int_text.len() < min_int {
        int_text.insert(0, '0');
    }
    if grouping {
        int_text = group_thousands(&int_text, locale.thousand_separator);
    }

    let mut out = String::new();
    if use_sign && rounded.mantissa() != 0 {
        out.push('-');
    }
    out.push_str(prefix);
    out.push_str(&int_text);
    if !frac.is_empty() {
        out.push(locale.decimal_separator);
        out.push_str(&frac);
    }
    out.push_str(suffix);
    out
}

fn group_thousands(digits: &str, separator: char) -> String {
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(separator);
        }
        out.push(c);
    }
    out
}

/// Número sem formato: dígitos com o separador decimal do idioma (`1234,5`).
pub fn plain_number(value: Decimal, locale: &LocaleInfo) -> String {
    value.to_string().replace('.', &locale.decimal_separator.to_string())
}

/// Lê um número digitado pelo usuário: aceita separador de milhar, símbolo da moeda e
/// espaços; o separador decimal é o do idioma.
pub fn parse_number(text: &str, locale: &LocaleInfo) -> Option<Decimal> {
    let cleaned: String = text
        .replace(&locale.currency_symbol, "")
        .chars()
        .filter(|&c| c != locale.thousand_separator && !c.is_whitespace())
        .collect();
    if cleaned.is_empty() {
        return None;
    }
    Decimal::parse_with(&cleaned, locale.decimal_separator)
}

// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Year4,
    Year2,
    Month2,
    Month1,
    Day2,
    Day1,
    Hour2,
    Hour1,
    Minute2,
    Minute1,
    Second2,
    Second1,
    Millis,
    Literal(&'a str),
}

type TokenMaker = fn() -> Token<'static>;

fn tokens(pattern: &str) -> Vec<Token<'_>> {
    const TABLE: [(&str, TokenMaker); 13] = [
        ("yyyy", || Token::Year4),
        ("zzz", || Token::Millis),
        ("yy", || Token::Year2),
        ("mm", || Token::Month2),
        ("dd", || Token::Day2),
        ("hh", || Token::Hour2),
        ("nn", || Token::Minute2),
        ("ss", || Token::Second2),
        ("m", || Token::Month1),
        ("d", || Token::Day1),
        ("h", || Token::Hour1),
        ("n", || Token::Minute1),
        ("s", || Token::Second1),
    ];
    let mut out = Vec::new();
    let mut rest = pattern;
    let mut literal_start: Option<&str> = None;
    'outer: while !rest.is_empty() {
        let lower = rest.to_ascii_lowercase();
        for (key, make) in TABLE {
            if lower.starts_with(key) {
                if let Some(lit) = literal_start.take() {
                    let len = lit.len() - rest.len();
                    out.push(Token::Literal(&lit[..len]));
                }
                out.push(make());
                rest = &rest[key.len()..];
                continue 'outer;
            }
        }
        literal_start.get_or_insert(rest);
        let next = rest.chars().next().map_or(1, char::len_utf8);
        rest = &rest[next..];
    }
    if let Some(lit) = literal_start {
        out.push(Token::Literal(lit));
    }
    out
}

/// Formata data e/ou hora com um padrão no estilo `FormatDateTime`.
pub fn format_date_time(date: Option<Date>, time: Option<Time>, pattern: &str) -> String {
    let d = date.unwrap_or_else(|| Date::new(1899, 12, 30).expect("data válida"));
    let t = time.unwrap_or_default();
    let mut out = String::new();
    for token in tokens(pattern) {
        match token {
            Token::Year4 => out.push_str(&format!("{:04}", d.year())),
            Token::Year2 => out.push_str(&format!("{:02}", d.year().rem_euclid(100))),
            Token::Month2 => out.push_str(&format!("{:02}", d.month())),
            Token::Month1 => out.push_str(&d.month().to_string()),
            Token::Day2 => out.push_str(&format!("{:02}", d.day())),
            Token::Day1 => out.push_str(&d.day().to_string()),
            Token::Hour2 => out.push_str(&format!("{:02}", t.hour())),
            Token::Hour1 => out.push_str(&t.hour().to_string()),
            Token::Minute2 => out.push_str(&format!("{:02}", t.minute())),
            Token::Minute1 => out.push_str(&t.minute().to_string()),
            Token::Second2 => out.push_str(&format!("{:02}", t.second())),
            Token::Second1 => out.push_str(&t.second().to_string()),
            Token::Millis => out.push_str(&format!("{:03}", t.millisecond())),
            Token::Literal(s) => out.push_str(s),
        }
    }
    out
}

/// Lê data e/ou hora digitadas seguindo o padrão. Separadores são flexíveis (`/`, `-`,
/// `.` ou espaço servem para qualquer literal); os números podem ter 1 ou 2 dígitos.
pub fn parse_date_time(text: &str, pattern: &str) -> Option<(Option<Date>, Option<Time>)> {
    let mut rest = text.trim();
    let (mut y, mut mo, mut d) = (None, None, None);
    let (mut h, mut mi, mut s, mut ms) = (None, None, None, None);
    let take_digits = |rest: &mut &str, max: usize| -> Option<u32> {
        let len = rest.chars().take(max).take_while(char::is_ascii_digit).count();
        if len == 0 {
            return None;
        }
        let value = rest[..len].parse().ok()?;
        *rest = &rest[len..];
        Some(value)
    };
    for token in tokens(pattern) {
        match token {
            Token::Literal(_) => {
                rest = rest.trim_start_matches(|c: char| !c.is_ascii_digit());
            }
            Token::Year4 => {
                let v = take_digits(&mut rest, 4)?;
                y = Some(if v < 100 { 2000 + v as i32 } else { v as i32 });
            }
            Token::Year2 => y = Some(2000 + take_digits(&mut rest, 2)? as i32),
            Token::Month2 | Token::Month1 => mo = Some(take_digits(&mut rest, 2)? as u8),
            Token::Day2 | Token::Day1 => d = Some(take_digits(&mut rest, 2)? as u8),
            Token::Hour2 | Token::Hour1 => h = Some(take_digits(&mut rest, 2)? as u8),
            Token::Minute2 | Token::Minute1 => mi = Some(take_digits(&mut rest, 2)? as u8),
            Token::Second2 | Token::Second1 => s = Some(take_digits(&mut rest, 2)? as u8),
            Token::Millis => ms = Some(take_digits(&mut rest, 3)? as u16),
        }
    }
    if !rest.trim().is_empty() {
        return None;
    }
    let date = match (y, mo, d) {
        (None, None, None) => None,
        (Some(y), Some(m), Some(d)) => Some(Date::new(y, m, d)?),
        _ => return None,
    };
    let time = match (h, mi) {
        (None, None) => None,
        (Some(h), Some(m)) => Some(Time::with_millis(h, m, s.unwrap_or(0), ms.unwrap_or(0))?),
        _ => return None,
    };
    Some((date, time))
}

/// Data e hora juntas, com o formato de data e de hora do idioma.
pub fn date_time_pattern(locale: &LocaleInfo) -> String {
    format!("{} {}", locale.short_date_format, locale.long_time_format)
}

/// Converte um `DateTime` em partes.
pub(crate) fn split(dt: DateTime) -> (Option<Date>, Option<Time>) {
    (Some(dt.date), Some(dt.time))
}
