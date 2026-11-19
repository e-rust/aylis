//! As propriedades que são listas de registros (`PropKind::RecordList`): o valor é o texto com
//! o JSON (no form, a lista de objetos de verdade). Os controles leem para structs próprias só
//! quando o texto muda e gravam de volta pelo mesmo JSON.

use aylis_core::EventArgs;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// A lista de registros do texto: vazio, `[]` ou JSON inválido = nenhum (nunca entra em pânico).
pub(crate) fn parse_list<T: DeserializeOwned>(text: &str) -> Vec<T> {
    let text = text.trim();
    if text.is_empty() {
        return Vec::new();
    }
    serde_json::from_str(text).unwrap_or_default()
}

/// O JSON de uma lista de registros (o texto da propriedade).
pub(crate) fn to_json<T: Serialize + ?Sized>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// O registro que um evento leva no `args`: o JSON no `Text` (ou num valor texto).
pub(crate) fn from_event<T: DeserializeOwned>(args: &EventArgs) -> Option<T> {
    match args {
        EventArgs::Text(t) | EventArgs::Value(aylis_core::PropValue::String(t)) => serde_json::from_str(t).ok(),
        _ => None,
    }
}

/// Pula o campo na gravação quando está no padrão (o JSON fica curto, bom para merge).
pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
