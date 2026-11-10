//! Os erros dos campos no idioma do app (o idioma vale para o processo: um teste só).

use aylis_core::messages;
use aylis_core::platform::LocaleInfo;
use aylis_data::{Field, FieldType};

#[test]
fn field_errors_follow_the_language() {
    let locale = LocaleInfo::pt_br();
    let age = Field::new("idade", FieldType::Integer).label("Idade");
    let active = Field::new("ativo", FieldType::Boolean).label("Ativo");

    assert_eq!(age.parse_text("x", &locale).unwrap_err(), "'x' não é um número inteiro válido para Idade");
    assert_eq!(active.parse_text("talvez", &locale).unwrap_err(), "'talvez' não é Sim nem Não");

    messages::set_language("en");
    assert_eq!(age.parse_text("x", &locale).unwrap_err(), "'x' is not a valid whole number for Idade");
    assert_eq!(active.parse_text("talvez", &locale).unwrap_err(), "'talvez' is neither Yes nor No");
    assert_eq!(active.parse_text("Yes", &locale).unwrap(), aylis_data::Value::Bool(true));

    messages::set_language("es");
    assert_eq!(age.parse_text("x", &locale).unwrap_err(), "'x' no es un número entero válido para Idade");
    messages::set_language("pt");
}
