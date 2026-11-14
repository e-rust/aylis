//! Valores dos campos: [`Value`] (o "Variant" tipado), [`Decimal`] para dinheiro e
//! [`Date`]/[`Time`]/[`DateTime`].

use std::cmp::Ordering;
use std::fmt;

/// Valor de um campo. `Null` é explícito.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    /// Ponto fixo exato (dinheiro).
    Decimal(Decimal),
    String(String),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    /// Um instante (a hora do pedido, o log de auditoria), guardado em UTC e mostrado no
    /// fuso de quem vê, como o `ftTimeStampOffset` do FireDAC.
    Timestamp(Timestamp),
    Blob(Vec<u8>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            Value::Float(v) if v.fract() == 0.0 => Some(*v as i64),
            Value::Decimal(d) => d.to_i64(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(v) => Some(*v as f64),
            Value::Float(v) => Some(*v),
            Value::Decimal(d) => Some(d.to_f64()),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<Decimal> {
        match self {
            Value::Decimal(d) => Some(*d),
            Value::Int(v) => Some(Decimal::new(*v as i128, 0)),
            Value::Float(v) => Decimal::from_f64(*v, 6),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<Date> {
        match self {
            Value::Date(d) => Some(*d),
            Value::DateTime(dt) => Some(dt.date),
            Value::Timestamp(ts) => Some(ts.to_local().date),
            _ => None,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Value::Null => "nulo",
            Value::Bool(_) => "lógico",
            Value::Int(_) => "inteiro",
            Value::Float(_) => "número",
            Value::Decimal(_) => "decimal",
            Value::String(_) => "texto",
            Value::Date(_) => "data",
            Value::Time(_) => "hora",
            Value::DateTime(_) => "data e hora",
            Value::Timestamp(_) => "instante",
            Value::Blob(_) => "binário",
        }
    }
}

/// Texto neutro (ISO): `2024-05-01`, `13:45:00`, `1234.56`. A formatação para o usuário
/// fica com o `Field` (display_format e idioma).
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => Ok(()),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::Decimal(d) => write!(f, "{d}"),
            Value::String(s) => f.write_str(s),
            Value::Date(d) => write!(f, "{d}"),
            Value::Time(t) => write!(f, "{t}"),
            Value::DateTime(dt) => write!(f, "{dt}"),
            Value::Timestamp(ts) => write!(f, "{ts}"),
            Value::Blob(b) => write!(f, "({} bytes)", b.len()),
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<Decimal> for Value {
    fn from(v: Decimal) -> Self {
        Value::Decimal(v)
    }
}

impl From<Date> for Value {
    fn from(v: Date) -> Self {
        Value::Date(v)
    }
}

impl From<Time> for Value {
    fn from(v: Time) -> Self {
        Value::Time(v)
    }
}

impl From<DateTime> for Value {
    fn from(v: DateTime) -> Self {
        Value::DateTime(v)
    }
}

impl From<Timestamp> for Value {
    fn from(v: Timestamp) -> Self {
        Value::Timestamp(v)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        v.map_or(Value::Null, Into::into)
    }
}

// ---------------------------------------------------------------------------

/// Número decimal de ponto fixo: `mantissa × 10^-scale`. Exato para dinheiro.
#[derive(Clone, Copy, Debug)]
pub struct Decimal {
    mantissa: i128,
    scale: u8,
}

impl Decimal {
    pub const ZERO: Decimal = Decimal { mantissa: 0, scale: 0 };

    /// `Decimal::new(12345, 2)` = 123,45.
    pub const fn new(mantissa: i128, scale: u8) -> Self {
        Self { mantissa, scale }
    }

    pub fn mantissa(&self) -> i128 {
        self.mantissa
    }

    pub fn scale(&self) -> u8 {
        self.scale
    }

    /// Lê `"-1234.56"` (separador decimal `.`, sem separador de milhar).
    pub fn parse(text: &str) -> Option<Decimal> {
        Self::parse_with(text, '.')
    }

    /// Lê com o separador decimal dado (sem separador de milhar).
    pub fn parse_with(text: &str, decimal_separator: char) -> Option<Decimal> {
        let text = text.trim();
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text.strip_prefix('+').unwrap_or(text)),
        };
        let (int, frac) = match digits.split_once(decimal_separator) {
            Some((i, f)) => (i, f),
            None => (digits, ""),
        };
        if (int.is_empty() && frac.is_empty())
            || !int.chars().all(|c| c.is_ascii_digit())
            || !frac.chars().all(|c| c.is_ascii_digit())
            || frac.len() > 28
        {
            return None;
        }
        let mut mantissa: i128 = 0;
        for c in int.chars().chain(frac.chars()) {
            mantissa = mantissa.checked_mul(10)?.checked_add(c.to_digit(10)? as i128)?;
        }
        Some(Decimal { mantissa: if negative { -mantissa } else { mantissa }, scale: frac.len() as u8 })
    }

    /// Converte arredondando para `scale` casas.
    pub fn from_f64(value: f64, scale: u8) -> Option<Decimal> {
        if !value.is_finite() {
            return None;
        }
        let factor = 10f64.powi(scale as i32);
        Some(Decimal { mantissa: (value * factor).round() as i128, scale })
    }

    pub fn to_f64(&self) -> f64 {
        self.mantissa as f64 / 10f64.powi(self.scale as i32)
    }

    pub fn to_i64(&self) -> Option<i64> {
        let r = self.checked_rescale(0, Rounding::HalfUp)?;
        i64::try_from(r.mantissa).ok()
    }

    /// Muda o número de casas, arredondando metade para longe do zero. Entra em pânico se o
    /// número não couber (só com valores perto de 10³⁸); para tratar, [`checked_rescale`](Self::checked_rescale).
    pub fn rescale(&self, scale: u8) -> Decimal {
        self.checked_rescale(scale, Rounding::HalfUp).unwrap_or_else(|| panic!("estouro do decimal: {self} com {scale} casas"))
    }

    /// Muda o número de casas, arredondando do jeito `mode`; `None` se não couber.
    pub fn checked_rescale(&self, scale: u8, mode: Rounding) -> Option<Decimal> {
        if scale > MAX_SCALE {
            return None;
        }
        match scale.cmp(&self.scale) {
            Ordering::Equal => Some(*self),
            Ordering::Greater => Some(Decimal { mantissa: self.mantissa.checked_mul(10i128.checked_pow((scale - self.scale) as u32)?)?, scale }),
            Ordering::Less => Some(Decimal { mantissa: div_round(self.mantissa, 10i128.pow((self.scale - scale) as u32), mode)?, scale }),
        }
    }

    /// Arredonda para `scale` casas do jeito `mode`.
    pub fn round(&self, scale: u8, mode: Rounding) -> Option<Decimal> {
        self.checked_rescale(scale, mode)
    }

    pub fn checked_add(&self, other: &Decimal) -> Option<Decimal> {
        let scale = self.scale.max(other.scale);
        let (a, b) = (self.checked_rescale(scale, Rounding::HalfUp)?, other.checked_rescale(scale, Rounding::HalfUp)?);
        Some(Decimal { mantissa: a.mantissa.checked_add(b.mantissa)?, scale })
    }

    pub fn checked_sub(&self, other: &Decimal) -> Option<Decimal> {
        self.checked_add(&Decimal { mantissa: other.mantissa.checked_neg()?, scale: other.scale })
    }

    /// O produto exato (as casas somam); se passar de [`MAX_SCALE`], arredonda metade para
    /// cima até caber.
    pub fn checked_mul(&self, other: &Decimal) -> Option<Decimal> {
        let product = Decimal { mantissa: self.mantissa.checked_mul(other.mantissa)?, scale: self.scale.checked_add(other.scale)? };
        if product.scale > MAX_SCALE { product.reduce(MAX_SCALE, Rounding::HalfUp) } else { Some(product) }
    }

    /// A divisão exata com `scale` casas no resultado, arredondada do jeito `mode` (sem passar
    /// por `f64`: 0,30 / 0,10 = 3). `None` se `other` é zero ou se não couber.
    pub fn checked_div(&self, other: &Decimal, scale: u8, mode: Rounding) -> Option<Decimal> {
        if other.mantissa == 0 || scale > MAX_SCALE {
            return None;
        }
        // resultado = (a / 10^sa) / (b / 10^sb), com `scale` casas: a · 10^(scale + sb − sa) / b.
        let exponent = scale as i32 + other.scale as i32 - self.scale as i32;
        let (n, d) = if exponent >= 0 {
            (self.mantissa.checked_mul(10i128.checked_pow(exponent as u32)?)?, other.mantissa)
        } else {
            (self.mantissa, other.mantissa.checked_mul(10i128.checked_pow((-exponent) as u32)?)?)
        };
        Some(Decimal { mantissa: div_round(n, d, mode)?, scale })
    }

    /// As casas da divisão quando ninguém diz quantas: as do maior dos dois mais 4
    /// (100,00 / 3 = 33,333333).
    pub fn default_div_scale(&self, other: &Decimal) -> u8 {
        (self.scale.max(other.scale) + 4).min(MAX_SCALE)
    }

    /// Corta casas até `scale` (só para menos casas).
    fn reduce(&self, scale: u8, mode: Rounding) -> Option<Decimal> {
        Some(Decimal { mantissa: div_round(self.mantissa, 10i128.checked_pow((self.scale - scale) as u32)?, mode)?, scale })
    }

    pub fn is_negative(&self) -> bool {
        self.mantissa < 0
    }

    /// Partes inteira e fracionária (com `scale` dígitos), sem sinal.
    pub(crate) fn parts(&self) -> (u128, String) {
        let abs = self.mantissa.unsigned_abs();
        let factor = 10u128.pow(self.scale as u32);
        let frac = abs % factor;
        let frac = if self.scale == 0 { String::new() } else { format!("{frac:0width$}", width = self.scale as usize) };
        (abs / factor, frac)
    }
}

/// O máximo de casas de um [`Decimal`] (o `i128` guarda uns 38 dígitos no total).
pub const MAX_SCALE: u8 = 28;

/// Como arredondar ao cortar casas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Rounding {
    /// Metade para longe do zero: 2,345 → 2,35. O padrão.
    #[default]
    HalfUp,
    /// Metade para o par, o "bancário": 2,345 → 2,34 e 2,355 → 2,36.
    HalfEven,
    /// Corta as casas: 2,349 → 2,34 (o `Trunc`).
    Truncate,
    /// Para longe do zero sempre que sobra algo: 2,341 → 2,35.
    Up,
}

impl aylis_core::prop::PropEnum for Rounding {
    const NAMES: &'static [&'static str] = &["half_up", "half_even", "truncate", "up"];

    fn from_index(index: usize) -> Option<Self> {
        [Rounding::HalfUp, Rounding::HalfEven, Rounding::Truncate, Rounding::Up].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl Rounding {
    /// Pelo nome (`"half_even"`); `None` se não for um dos quatro.
    pub fn from_name(name: &str) -> Option<Rounding> {
        use aylis_core::prop::PropEnum;
        Rounding::NAMES.iter().position(|n| n.eq_ignore_ascii_case(name)).and_then(Rounding::from_index)
    }
}

/// `n / d` arredondado do jeito `mode` (`d` diferente de zero). `None` só no caso que não
/// cabe (`i128::MIN / −1`).
fn div_round(n: i128, d: i128, mode: Rounding) -> Option<i128> {
    let q = n.checked_div(d)?;
    let r = n % d;
    if r == 0 {
        return Some(q);
    }
    let away = if (n < 0) != (d < 0) { -1 } else { 1 };
    let (r, d) = (r.unsigned_abs(), d.unsigned_abs());
    // Comparar o resto com a metade sem dobrar (dobrar poderia estourar).
    let half = r.cmp(&(d - r));
    let bump = match mode {
        Rounding::Truncate => false,
        Rounding::Up => true,
        Rounding::HalfUp => half != Ordering::Less,
        Rounding::HalfEven => half == Ordering::Greater || (half == Ordering::Equal && q % 2 != 0),
    };
    if bump { q.checked_add(away) } else { Some(q) }
}

/// Os operadores. Estouro entra em pânico (como os inteiros);
/// para tratar, os `checked_*`. A divisão usa [`Decimal::default_div_scale`] e arredonda
/// metade para cima; dividir por zero entra em pânico.
impl std::ops::Add for Decimal {
    type Output = Decimal;
    fn add(self, other: Decimal) -> Decimal {
        self.checked_add(&other).unwrap_or_else(|| panic!("estouro do decimal: {self} + {other}"))
    }
}

impl std::ops::Sub for Decimal {
    type Output = Decimal;
    fn sub(self, other: Decimal) -> Decimal {
        self.checked_sub(&other).unwrap_or_else(|| panic!("estouro do decimal: {self} - {other}"))
    }
}

impl std::ops::Mul for Decimal {
    type Output = Decimal;
    fn mul(self, other: Decimal) -> Decimal {
        self.checked_mul(&other).unwrap_or_else(|| panic!("estouro do decimal: {self} * {other}"))
    }
}

impl std::ops::Div for Decimal {
    type Output = Decimal;
    fn div(self, other: Decimal) -> Decimal {
        assert!(other.mantissa != 0, "divisão por zero: {self} / {other}");
        self.checked_div(&other, self.default_div_scale(&other), Rounding::HalfUp).unwrap_or_else(|| panic!("estouro do decimal: {self} / {other}"))
    }
}

impl std::ops::Neg for Decimal {
    type Output = Decimal;
    fn neg(self) -> Decimal {
        Decimal { mantissa: self.mantissa.checked_neg().expect("estouro do decimal ao trocar o sinal"), scale: self.scale }
    }
}

impl From<i64> for Decimal {
    fn from(v: i64) -> Self {
        Decimal { mantissa: v as i128, scale: 0 }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let scale = self.scale.max(other.scale);
        match (self.checked_rescale(scale, Rounding::HalfUp), other.checked_rescale(scale, Rounding::HalfUp)) {
            (Some(a), Some(b)) => a.mantissa.cmp(&b.mantissa),
            // Perto de 10³⁸ as casas não cabem: a parte inteira decide (e a fração, se empatar).
            _ => {
                let (a, b) = (self.parts(), other.parts());
                let signed = |neg: bool, o: Ordering| if neg { o.reverse() } else { o };
                match (self.is_negative(), other.is_negative()) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    (neg, _) => signed(neg, a.0.cmp(&b.0).then_with(|| {
                        let width = a.1.len().max(b.1.len());
                        format!("{:0<width$}", a.1).cmp(&format!("{:0<width$}", b.1))
                    })),
                }
            }
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (int, frac) = self.parts();
        let sign = if self.is_negative() { "-" } else { "" };
        if frac.is_empty() { write!(f, "{sign}{int}") } else { write!(f, "{sign}{int}.{frac}") }
    }
}

// ---------------------------------------------------------------------------

/// Data do calendário (sem fuso).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// `None` se a data não existir (31/02, mês 13...).
    pub fn new(year: i32, month: u8, day: u8) -> Option<Date> {
        (1..=12).contains(&month).then_some(())?;
        (day >= 1 && day <= days_in_month(year, month)).then_some(Date { year, month, day })
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    /// Dias desde 1970-01-01 (negativos antes).
    pub fn to_days(&self) -> i64 {
        let (y, m, d) = (self.year as i64 - i64::from(self.month <= 2), self.month as i64, self.day as i64);
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
        era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719_468
    }

    /// A data `days` dias depois de 1970-01-01.
    pub fn from_days(days: i64) -> Option<Date> {
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let y = yoe + era * 400 + i64::from(m <= 2);
        Date::new(i32::try_from(y).ok()?, m, d)
    }

    /// Hoje, no horário local: em Brasília, às 22h ainda é hoje.
    pub fn today() -> Date {
        DateTime::now().date
    }

    /// Lê `yyyy-mm-dd`.
    pub fn parse_iso(text: &str) -> Option<Date> {
        let mut parts = text.trim().splitn(3, '-');
        let y = parts.next()?.parse().ok()?;
        let m = parts.next()?.parse().ok()?;
        let d = parts.next()?.parse().ok()?;
        Date::new(y, m, d)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Hora do dia.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
}

impl Time {
    pub fn new(hour: u8, minute: u8, second: u8) -> Option<Time> {
        Self::with_millis(hour, minute, second, 0)
    }

    pub fn with_millis(hour: u8, minute: u8, second: u8, millisecond: u16) -> Option<Time> {
        (hour < 24 && minute < 60 && second < 60 && millisecond < 1000).then_some(Time { hour, minute, second, millisecond })
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn millisecond(&self) -> u16 {
        self.millisecond
    }

    /// Lê `hh:nn`, `hh:nn:ss` ou `hh:nn:ss.fff` (sem fuso; veja [`DateTime::parse_iso`]).
    pub fn parse_iso(text: &str) -> Option<Time> {
        let mut parts = text.trim().split(':');
        let h = parts.next()?.parse().ok()?;
        let m = parts.next()?.parse().ok()?;
        let (s, ms) = match parts.next() {
            None => (0, 0),
            Some(s) => {
                let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
                // Os três primeiros dígitos da fração (o resto não cabe em milissegundos).
                let digits: String = frac.chars().take(3).collect();
                let ms = if digits.is_empty() { 0 } else { format!("{digits:0<3}").parse().ok()? };
                (whole.parse().ok()?, ms)
            }
        };
        if parts.next().is_some() {
            return None;
        }
        Time::with_millis(h, m, s, ms)
    }

    fn seconds_of_day(&self) -> i64 {
        self.hour as i64 * 3600 + self.minute as i64 * 60 + self.second as i64
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

/// Data e hora.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Lê `yyyy-mm-ddThh:nn:ss` (ou com espaço no lugar do `T`, com fração de segundo). Com
    /// fuso (`Z`, `-03:00`, `+0530`), o instante vira o horário local desta tela: um backend
    /// que manda `13:00:00Z` mostra 10:00 em Brasília. O fuso nunca é ignorado.
    pub fn parse_iso(text: &str) -> Option<DateTime> {
        let text = text.trim();
        let (d, t) = text.split_once(['T', ' ']).unwrap_or((text, "00:00:00"));
        let (t, offset) = split_offset(t)?;
        let local = DateTime { date: Date::parse_iso(d)?, time: Time::parse_iso(t)? };
        match offset {
            None => Some(local),
            Some(offset) => DateTime::from_unix_local(local.to_unix() - offset, local.time.millisecond()),
        }
    }

    /// Segundos desde 1970, lendo esta data e hora como se fosse UTC.
    pub fn to_unix(&self) -> i64 {
        self.date.to_days() * 86_400 + self.time.seconds_of_day()
    }

    /// A data e a hora (sem fuso) de `secs` segundos desde 1970.
    pub fn from_unix(secs: i64, millisecond: u16) -> Option<DateTime> {
        let date = Date::from_days(secs.div_euclid(86_400))?;
        let s = secs.rem_euclid(86_400);
        let time = Time::with_millis((s / 3600) as u8, (s / 60 % 60) as u8, (s % 60) as u8, millisecond)?;
        Some(DateTime { date, time })
    }

    /// O instante `utc` (segundos desde 1970) no horário local desta tela (com horário de
    /// verão, pelo sistema; sem a plataforma, como nos testes, o local é o UTC).
    pub fn from_unix_local(utc: i64, millisecond: u16) -> Option<DateTime> {
        DateTime::from_unix(utc + aylis_core::platform::local_offset(utc), millisecond)
    }

    /// Agora, no horário local.
    pub fn now() -> DateTime {
        let since = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap_or_default();
        DateTime::from_unix_local(since.as_secs() as i64, since.subsec_millis() as u16).expect("o relógio do sistema dá uma data válida")
    }
}

/// Separa o fuso do fim da hora: `"10:00:00-03:00"` → (`"10:00:00"`, −10800 segundos);
/// `Z` é o UTC; sem fuso, `None`. `None` no todo se o fuso for inválido.
fn split_offset(time: &str) -> Option<(&str, Option<i64>)> {
    if let Some(t) = time.strip_suffix(['Z', 'z']) {
        return Some((t, Some(0)));
    }
    // O sinal do fuso vem depois dos dois-pontos da hora (não confunde com nada antes).
    let Some(at) = time.rfind(['+', '-']).filter(|&i| i > 0) else { return Some((time, None)) };
    let (t, zone) = time.split_at(at);
    let sign = if zone.starts_with('-') { -1 } else { 1 };
    let digits: String = zone[1..].chars().filter(|c| *c != ':').collect();
    let (h, m) = match digits.len() {
        2 => (digits.parse::<i64>().ok()?, 0),
        4 => (digits[..2].parse::<i64>().ok()?, digits[2..].parse::<i64>().ok()?),
        _ => return None,
    };
    (h <= 14 && m < 60).then_some((t, Some(sign * (h * 3600 + m * 60))))
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

/// Um instante: milissegundos desde 1970 em UTC. É o mesmo momento em qualquer fuso; para
/// mostrar, vira o horário local de quem vê ([`to_local`](Self::to_local)). Use para o que
/// aconteceu num momento (a hora do pedido, a última alteração); para uma data ou hora do
/// calendário (vencimento, aniversário), o [`DateTime`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp {
    millis: i64,
}

impl Timestamp {
    pub fn from_unix_millis(millis: i64) -> Timestamp {
        Timestamp { millis }
    }

    pub fn unix_millis(&self) -> i64 {
        self.millis
    }

    pub fn now() -> Timestamp {
        let since = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap_or_default();
        Timestamp { millis: since.as_millis() as i64 }
    }

    /// O horário local desta tela neste instante.
    pub fn to_local(&self) -> DateTime {
        DateTime::from_unix_local(self.millis.div_euclid(1000), self.millis.rem_euclid(1000) as u16).expect("um instante válido")
    }

    /// O instante de um horário local desta tela (o que o usuário digitou).
    pub fn from_local(local: DateTime) -> Timestamp {
        let wall = local.to_unix();
        // O fuso depende do instante (horário de verão): a primeira conta usa o fuso do
        // horário de parede, e a segunda acerta com o do instante encontrado.
        let first = wall - aylis_core::platform::local_offset(wall);
        let utc = wall - aylis_core::platform::local_offset(first);
        Timestamp { millis: utc * 1000 + local.time.millisecond() as i64 }
    }

    /// Lê ISO com fuso (`2026-11-02T13:00:00Z`, `...-03:00`); sem fuso, é o horário local.
    pub fn parse_iso(text: &str) -> Option<Timestamp> {
        let text = text.trim();
        let (d, t) = text.split_once(['T', ' ']).unwrap_or((text, "00:00:00"));
        let (t, offset) = split_offset(t)?;
        let wall = DateTime { date: Date::parse_iso(d)?, time: Time::parse_iso(t)? };
        Some(match offset {
            Some(offset) => Timestamp { millis: (wall.to_unix() - offset) * 1000 + wall.time.millisecond() as i64 },
            None => Timestamp::from_local(wall),
        })
    }
}

/// Em UTC, com milissegundos: `2026-11-02T13:00:00.000Z` (o que o protocolo manda).
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utc = DateTime::from_unix(self.millis.div_euclid(1000), 0).ok_or(fmt::Error)?;
        write!(f, "{}T{}.{:03}Z", utc.date, utc.time, self.millis.rem_euclid(1000))
    }
}
