//! As datas e as horas do calendário (sem fuso): o `DateEdit`, o `Calendar` e o acesso
//! tipado as usam, e o `aylis-data` também (os campos de data dos datasets).

use std::fmt;

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
        DateTime::from_unix(utc + crate::platform::local_offset(utc), millisecond)
    }

    /// Agora, no horário local.
    pub fn now() -> DateTime {
        let since = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap_or_default();
        DateTime::from_unix_local(since.as_secs() as i64, since.subsec_millis() as u16).expect("o relógio do sistema dá uma data válida")
    }
}

/// Separa o fuso do fim da hora: `"10:00:00-03:00"` → (`"10:00:00"`, −10800 segundos);
/// `Z` é o UTC; sem fuso, `None`. `None` no todo se o fuso for inválido.
#[doc(hidden)]
pub fn split_offset(time: &str) -> Option<(&str, Option<i64>)> {
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
