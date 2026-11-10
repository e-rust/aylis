//! Data e hora no horário local da tela: o `today()` e a leitura ISO com fuso.

use aylis_data::{Date, DateTime, Time, Value};

/// Brasília (UTC−3) em todo este arquivo: o fuso é global, e os testes rodam juntos.
fn brasilia() {
    aylis_core::platform::set_local_offset_fn(|_| -3 * 3600);
}

fn dt(text: &str) -> DateTime {
    DateTime::parse_iso(text).unwrap_or_else(|| panic!("não leu {text}"))
}

#[test]
fn days_since_1970_go_both_ways() {
    assert_eq!(Date::new(1970, 1, 1).unwrap().to_days(), 0);
    for (y, m, d) in [(2000, 2, 29), (1969, 12, 31), (2026, 11, 2), (1899, 12, 30)] {
        let date = Date::new(y, m, d).unwrap();
        assert_eq!(Date::from_days(date.to_days()), Some(date), "{date}");
    }
}

#[test]
fn late_evening_in_brasilia_is_still_today() {
    brasilia();
    // 02/11/2026 às 22:30 em Brasília = 03/11 01:30 em UTC: antes, o today() dava 03/11.
    let utc = dt("2026-11-03T01:30:00").to_unix();
    let local = DateTime::from_unix_local(utc, 0).unwrap();
    assert_eq!(local.date, Date::new(2026, 11, 2).unwrap());
    assert_eq!(local.time, Time::new(22, 30, 0).unwrap());
}

#[test]
fn the_offset_in_iso_text_becomes_local_time() {
    brasilia();
    let ten = DateTime::new(Date::new(2026, 11, 2).unwrap(), Time::new(10, 0, 0).unwrap());
    // O toISOString do JavaScript (UTC com fração) e um fuso explícito: antes, o primeiro
    // virava 13:00 local, e o segundo era recusado.
    assert_eq!(dt("2026-11-02T13:00:00Z"), ten);
    assert_eq!(dt("2026-11-02T10:00:00-03:00"), ten);
    assert_eq!(dt("2026-11-02T08:00:00-0500"), ten);
    assert_eq!(dt("2026-11-02 10:00:00"), ten, "sem fuso: já é o horário local");
    let with_ms = dt("2026-11-02T13:00:00.123Z");
    assert_eq!(with_ms.time, Time::with_millis(10, 0, 0, 123).unwrap());
    // Passar da meia-noite muda o dia.
    assert_eq!(dt("2026-11-03T01:30:00Z").date, Date::new(2026, 11, 2).unwrap());
}

#[test]
fn an_invalid_offset_is_refused_not_ignored() {
    brasilia();
    for text in ["2026-11-02T10:00:00+99:00", "2026-11-02T10:00:00+3", "2026-11-02T10:00:00+03:0x"] {
        assert_eq!(DateTime::parse_iso(text), None, "{text}");
    }
}

#[test]
fn today_and_now_in_expressions_are_local() {
    brasilia();
    let today = aylis_data::expr::Expr::parse("today()").unwrap().eval(&|_| None).unwrap();
    assert_eq!(today, Value::Date(Date::today()));
    let now = aylis_data::expr::Expr::parse("now()").unwrap().eval(&|_| None).unwrap();
    assert!(matches!(now, Value::DateTime(d) if d.date == Date::today()), "{now:?}");
}

#[test]
fn a_timestamp_is_one_instant_shown_in_local_time() {
    use aylis_data::{Field, FieldType, Timestamp};
    brasilia();
    let ts = Timestamp::parse_iso("2026-11-02T10:00:00-03:00").unwrap();
    assert_eq!(Timestamp::parse_iso("2026-11-02T13:00:00Z"), Some(ts), "o mesmo instante");
    assert_eq!(ts.to_string(), "2026-11-02T13:00:00.000Z", "no protocolo, em UTC");
    assert_eq!(ts.to_local(), DateTime::new(Date::new(2026, 11, 2).unwrap(), Time::new(10, 0, 0).unwrap()));
    // Sem fuso, é o horário local de quem escreveu; e volta igual.
    assert_eq!(Timestamp::parse_iso("2026-11-02T10:00:00"), Some(ts));
    assert_eq!(Timestamp::from_local(ts.to_local()), ts);

    let field = Field::new("criado_em", FieldType::Timestamp);
    let locale = aylis_core::platform::LocaleInfo::pt_br();
    assert_eq!(field.text_of(&Value::Timestamp(ts), true, &locale), "02/11/2026 10:00:00");
    assert_eq!(field.parse_text("02/11/2026 10:00:00", &locale), Ok(Value::Timestamp(ts)));
    let json = aylis_data::json::value_to_json(&Value::Timestamp(ts));
    assert_eq!(json, serde_json::json!("2026-11-02T13:00:00.000Z"));
    assert_eq!(aylis_data::json::json_to_value(&json, &field), Ok(Value::Timestamp(ts)));
}
