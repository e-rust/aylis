//! Os registros em JSON: as listas das propriedades (`PropKind::RecordList`), os resultados
//! dos métodos e o que os eventos levam (`EventArg::Value(PropKind::Record)`).

use serde::de::DeserializeOwned;

/// A lista de registros em JSON; vazio, `[]` ou JSON inválido: nenhum (sem pânico).
pub fn parse_list<T: DeserializeOwned>(text: &str) -> Vec<T> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    serde_json::from_str(text).unwrap_or_default()
}

/// O registro que um evento leva (o JSON no `EventArgs::Text`, ou num valor texto).
pub fn event_record<T: DeserializeOwned>(args: &aylis_core::EventArgs) -> Option<T> {
    let text = match args {
        aylis_core::EventArgs::Text(t) => t.as_str(),
        aylis_core::EventArgs::Value(aylis_core::PropValue::String(t)) => t.as_str(),
        _ => return None,
    };
    serde_json::from_str(text).ok()
}

/// O `EventValue` de uma struct de evento: o JSON do registro `$record`.
macro_rules! record_event {
    ($ty:ty, $record:expr) => {
        impl aylis_core::EventValue for $ty {
            const ARG: aylis_core::EventArg = aylis_core::EventArg::Value(aylis_core::PropKind::Record(&$record));
            fn from_args(args: &aylis_core::EventArgs) -> Option<Self> {
                crate::records::event_record(args)
            }
        }
    };
}
pub(crate) use record_event;

/// O tipo de uma propriedade calculada com a lista em JSON (o `computed:` do `properties!`
/// declara pelo tipo, não pelo `as`): `records_text!(Nome, REGISTRO)`.
macro_rules! records_text {
    ($name:ident, $record:expr) => {
        #[doc = concat!("A lista em JSON do registro [`", stringify!($record), "`] (uma propriedade calculada).")]
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $name(pub String);

        impl aylis_core::prop::PropType for $name {
            const KIND: aylis_core::PropKind = aylis_core::PropKind::RecordList(&$record);
            fn to_value(&self) -> aylis_core::PropValue {
                aylis_core::PropValue::String(self.0.clone())
            }
            fn from_value(value: aylis_core::PropValue) -> aylis_core::Result<Self> {
                value.to_str().map(|t| $name(t.to_string()))
            }
        }
    };
}
pub(crate) use records_text;
