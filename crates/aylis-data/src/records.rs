//! As propriedades do [`ClientDataSet`](crate::ClientDataSet) que são listas de registros em
//! JSON (`master_fields`, `indexes`, `params`): o registro de cada uma no acesso tipado e a
//! leitura. Vazio, `[]` ou JSON inválido: nenhum item.

use aylis_core::{PropKind, RecordInfo};
use serde_json::Value as Json;

/// Um par do mestre-detalhe: o campo daqui e o do mestre (vazio: o mesmo nome).
pub const MASTER_FIELD_RECORD: RecordInfo =
    RecordInfo { name: "MasterField", fields: &[("field", PropKind::String), ("master_field", PropKind::String)] };

/// Um índice com nome: a ordem no formato de `index_field_names` (`valor desc;nome`).
pub const INDEX_DEF_RECORD: RecordInfo = RecordInfo { name: "IndexDef", fields: &[("name", PropKind::String), ("fields", PropKind::String)] };

/// Um parâmetro do filtro (`:minimo`) e o valor dele em texto.
pub const FILTER_PARAM_RECORD: RecordInfo = RecordInfo { name: "FilterParam", fields: &[("name", PropKind::String), ("value", PropKind::String)] };

/// Os objetos da lista JSON.
fn items(text: &str) -> Vec<Json> {
    match serde_json::from_str::<Json>(text) {
        Ok(Json::Array(items)) => items,
        _ => Vec::new(),
    }
}

/// Um campo de texto do registro (número e lógico viram o texto deles).
fn text(json: &Json, key: &str) -> String {
    match &json[key] {
        Json::String(s) => s.trim().to_string(),
        Json::Null => String::new(),
        other => other.to_string(),
    }
}

/// Pares (campo do detalhe, campo do mestre) de `master_fields`.
pub(crate) fn master_fields(spec: &str) -> Vec<(String, String)> {
    items(spec)
        .iter()
        .filter_map(|j| {
            let field = text(j, "field");
            if field.is_empty() {
                return None;
            }
            let master = text(j, "master_field");
            let master = if master.is_empty() { field.clone() } else { master };
            Some((field, master))
        })
        .collect()
}

/// Os índices de `indexes`: (nome, ordem).
pub(crate) fn index_defs(spec: &str) -> Vec<(String, String)> {
    items(spec).iter().map(|j| (text(j, "name"), text(j, "fields"))).filter(|(n, _)| !n.is_empty()).collect()
}

/// Os parâmetros de `params`: (nome, valor em texto).
pub(crate) fn filter_params(spec: &str) -> Vec<(String, String)> {
    items(spec).iter().map(|j| (text(j, "name"), text(j, "value"))).filter(|(n, _)| !n.is_empty()).collect()
}
