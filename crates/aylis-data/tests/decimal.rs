//! O `Decimal` (dinheiro): contas exatas, divisão sem `f64`, os modos de arredondamento e o
//! estouro como erro.

use aylis_core::platform::LocaleInfo;
use aylis_data::expr::Expr;
use aylis_data::{Decimal, Field, FieldType, Rounding, Value};

fn d(text: &str) -> Decimal {
    Decimal::parse(text).unwrap()
}

fn eval(text: &str) -> Result<Value, String> {
    Expr::parse(text).unwrap().eval(&|_| None)
}

#[test]
fn division_is_exact() {
    // Em f64, 0,3 / 0,1 = 2,9999999999999996.
    let q = d("0.30").checked_div(&d("0.10"), 2, Rounding::HalfUp).unwrap();
    assert_eq!(q, d("3"));
    assert_eq!(d("0.30") / d("0.10"), d("3"));
    assert_eq!(d("100.00") / d("3"), d("33.333333"), "as casas do maior mais 4, como o Currency");
    assert_eq!(d("100").checked_div(&d("3"), 2, Rounding::HalfUp), Some(d("33.33")));
    assert_eq!(d("1").checked_div(&d("0"), 2, Rounding::HalfUp), None, "dividir por zero");
    // Nas expressões também: decimal, não Float, e o quociente certo.
    assert_eq!(eval("0.30 / 0.10"), Ok(Value::Decimal(d("3"))));
    assert_eq!(eval("7 / 2"), Ok(Value::Decimal(d("3.5"))), "inteiros dividem exatos");
    assert_eq!(eval("1.0 / 0"), Ok(Value::Null), "divisão por zero continua nula");
}

#[test]
fn the_rounding_modes() {
    let cases = [
        // valor, half_up, half_even, truncate, up (2 casas)
        ("2.345", "2.35", "2.34", "2.34", "2.35"),
        ("2.355", "2.36", "2.36", "2.35", "2.36"),
        ("2.341", "2.34", "2.34", "2.34", "2.35"),
        ("-2.345", "-2.35", "-2.34", "-2.34", "-2.35"),
    ];
    for (value, up, even, trunc, away) in cases {
        let v = d(value);
        assert_eq!(v.round(2, Rounding::HalfUp), Some(d(up)), "{value} half_up");
        assert_eq!(v.round(2, Rounding::HalfEven), Some(d(even)), "{value} half_even");
        assert_eq!(v.round(2, Rounding::Truncate), Some(d(trunc)), "{value} truncate");
        assert_eq!(v.round(2, Rounding::Up), Some(d(away)), "{value} up");
    }
    assert_eq!(eval("round(2.345, 2, 'half_even')"), Ok(Value::Decimal(d("2.34"))));
    assert_eq!(eval("round(2.345, 2)"), Ok(Value::Decimal(d("2.35"))));
    assert!(eval("round(2.345, 2, 'bancario')").unwrap_err().contains("half_even"));
}

#[test]
fn the_field_rounds_its_way() {
    let locale = LocaleInfo::pt_br();
    let mut field = Field::new("valor", FieldType::Currency);
    field.rounding = Rounding::HalfEven;
    assert_eq!(field.parse_text("10,125", &locale), Ok(Value::Decimal(d("10.12"))));
    field.rounding = Rounding::Truncate;
    assert_eq!(field.parse_text("10,129", &locale), Ok(Value::Decimal(d("10.12"))));
    field.rounding = Rounding::HalfUp;
    assert_eq!(field.parse_text("10,125", &locale), Ok(Value::Decimal(d("10.13"))), "o padrão, como antes");
}

#[test]
fn overflow_is_an_error_not_a_wrong_value() {
    let huge = Decimal::new(i128::MAX / 2 + 1, 0);
    assert_eq!(huge.checked_add(&huge), None);
    assert_eq!(huge.checked_mul(&d("100")), None);
    assert_eq!(huge.checked_rescale(5, Rounding::HalfUp), None, "antes saturava em silêncio");
    assert!(eval("9223372036854775807 + 1").unwrap_err().contains("estouro"), "inteiros também");
    let field = Field::new("valor", FieldType::Currency);
    assert!(field.parse_text("99999999999999999999999999999999999999", &LocaleInfo::pt_br()).is_err());
    // A comparação nunca falha, mesmo com valores que não cabem na mesma escala.
    assert!(huge > d("1.5"));
    assert!(Decimal::new(-(i128::MAX / 2), 0) < d("-1.5"));
}

#[test]
fn the_operators_for_rust_code() {
    let total = d("10.50") + d("0.25") - d("1.00");
    assert_eq!(total, d("9.75"));
    assert_eq!(d("2.5") * d("4"), d("10"));
    assert_eq!(-d("1.5"), d("-1.5"));
    assert_eq!(Decimal::from(7) / Decimal::from(2), d("3.5"));
}

#[test]
#[should_panic(expected = "estouro do decimal")]
fn an_operator_overflow_panics_like_integers() {
    let huge = Decimal::new(i128::MAX / 2 + 1, 0);
    let _ = huge + huge;
}
