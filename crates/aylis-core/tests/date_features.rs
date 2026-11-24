//! O `Date` com as features `serde` (texto ISO) e `chrono` (`From`/`Into` de `NaiveDate`).

use aylis_core::date::Date;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct Row {
    created: Date,
    closed: Option<Date>,
}

#[test]
fn date_goes_to_json_as_iso_text_and_null_is_none() {
    let row: Row = serde_json::from_str(r#"{"created": "2026-11-23", "closed": null}"#).unwrap();
    assert_eq!(row, Row { created: Date::new(2026, 11, 23).unwrap(), closed: None });
    assert_eq!(serde_json::to_string(&row).unwrap(), r#"{"created":"2026-11-23","closed":null}"#);
    let back: Row = serde_json::from_str(&serde_json::to_string(&row).unwrap()).unwrap();
    assert_eq!(back, row);
}

#[test]
fn an_invalid_date_is_a_json_error() {
    assert!(serde_json::from_str::<Date>(r#""2026-02-30""#).is_err());
    assert!(serde_json::from_str::<Date>(r#""23/11/2026""#).is_err());
    assert!(serde_json::from_str::<Date>("20261123").is_err());
}

#[test]
fn date_converts_from_and_into_chrono() {
    let naive = chrono::NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    let date: Date = naive.into();
    assert_eq!(date, Date::new(2024, 2, 29).unwrap());
    let again: chrono::NaiveDate = date.into();
    assert_eq!(again, naive);
    // Fora do alcance do `chrono`, o limite dele.
    assert_eq!(chrono::NaiveDate::from(Date::new(300_000, 1, 1).unwrap()), chrono::NaiveDate::MAX);
    assert_eq!(chrono::NaiveDate::from(Date::new(-300_000, 1, 1).unwrap()), chrono::NaiveDate::MIN);
}
