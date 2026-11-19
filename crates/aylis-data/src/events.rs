//! Os eventos com estrutura do [`Field`] e do [`ClientDataSet`](crate::ClientDataSet): o
//! pedido chega em JSON no `EventArgs::Text`, e cada um tem o registro dele no acesso tipado
//! ([`GetTextRequest`], [`SetTextRequest`], [`MasterChange`], [`ServerQuery`]), para o
//! handler receber o valor já lido nas 8 linguagens. As ajudas [`get_text`] e [`set_text`]
//! respondem os do `Field` a partir do handler por nome (`ui, sender, args`).
//!
//! ```ignore
//! pub fn field1_get_text(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
//!     aylis_data::get_text(ui, sender, args, |v| match v.as_i64() {
//!         Some(1) => "Aberto".into(),
//!         Some(2) => "Faturado".into(),
//!         _ => v.to_string(),
//!     });
//! }
//!
//! pub fn field1_set_text(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
//!     aylis_data::set_text(ui, sender, args, |t| match t {
//!         "Aberto" => Ok(1.into()),
//!         "Faturado" => Ok(2.into()),
//!         _ => Err(format!("situação desconhecida: {t}")),
//!     });
//! }
//! ```

use aylis_core::{ComponentId, EventArg, EventArgs, EventValue, PropKind, PropValue, RecordInfo, Ui};
use serde_json::Value as Json;

use crate::field::{Field, FieldType};
use crate::handle::data_set;
use crate::value::Value;

/// O registro do pedido do `Field.on_get_text`: o campo e os valores que esperam o texto de
/// exibição, cada um no texto canônico ([`value_text`]: `1`, `2024-05-01`, `SP`).
pub const GET_TEXT_REQUEST_RECORD: RecordInfo =
    RecordInfo { name: "GetTextRequest", fields: &[("field", PropKind::String), ("values", PropKind::List)] };

/// O registro do pedido do `Field.on_set_text`: o campo e o texto digitado.
pub const SET_TEXT_REQUEST_RECORD: RecordInfo =
    RecordInfo { name: "SetTextRequest", fields: &[("field", PropKind::String), ("text", PropKind::String)] };

/// Um campo e o valor dele, no texto canônico (os valores do mestre no `on_master_change`).
pub const FIELD_VALUE_RECORD: RecordInfo =
    RecordInfo { name: "FieldValue", fields: &[("field", PropKind::String), ("value", PropKind::String)] };

/// O registro do `ClientDataSet.on_master_change`: os campos do mestre ligados ao detalhe
/// (o `master_fields`) com o valor do registro atual do mestre.
pub const MASTER_CHANGE_RECORD: RecordInfo =
    RecordInfo { name: "MasterChange", fields: &[("values", PropKind::RecordList(&FIELD_VALUE_RECORD))] };

/// O registro do `ClientDataSet.on_server_query` (veja [`ServerQuery`]).
pub const SERVER_QUERY_RECORD: RecordInfo = RecordInfo {
    name: "ServerQuery",
    fields: &[
        ("kind", PropKind::Enum(&["filter", "sort", "locate"])),
        ("index", PropKind::String),
        ("filter", PropKind::String),
        ("loaded", PropKind::Int),
        ("field", PropKind::String),
        ("value", PropKind::String),
    ],
};

/// O valor no texto canônico dos eventos: o número, a data ISO, o texto como é; nulo é vazio.
/// É também a chave dos textos que o `on_get_text` devolve (o backend pode mandar `1` ou
/// `"1"`: os dois valem para o mesmo valor).
pub fn value_text(value: &Value) -> String {
    value.to_string()
}

/// O JSON do `args` (texto, ou valor texto).
fn json_of(args: &EventArgs) -> Option<Json> {
    let text = match args {
        EventArgs::Text(t) => t.as_str(),
        EventArgs::Value(PropValue::String(t)) => t.as_str(),
        _ => return None,
    };
    serde_json::from_str(text).ok()
}

fn text(json: &Json, key: &str) -> String {
    match &json[key] {
        Json::String(s) => s.clone(),
        Json::Null => String::new(),
        other => other.to_string(),
    }
}

/// O pedido do `on_get_text`: os valores (no texto canônico) que esperam o texto de exibição.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetTextRequest {
    pub field: String,
    pub values: Vec<String>,
}

impl GetTextRequest {
    pub fn to_json(&self) -> String {
        serde_json::json!({ "field": self.field, "values": self.values }).to_string()
    }
}

impl EventValue for GetTextRequest {
    const ARG: EventArg = EventArg::Value(PropKind::Record(&GET_TEXT_REQUEST_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        let json = json_of(args)?;
        let values = json["values"].as_array().map(|a| a.iter().map(|v| if let Json::String(s) = v { s.clone() } else { v.to_string() }).collect());
        Some(GetTextRequest { field: text(&json, "field"), values: values.unwrap_or_default() })
    }
}

/// O pedido do `on_set_text`: o texto digitado no campo.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetTextRequest {
    pub field: String,
    pub text: String,
}

impl SetTextRequest {
    pub fn to_json(&self) -> String {
        serde_json::json!({ "field": self.field, "text": self.text }).to_string()
    }
}

impl EventValue for SetTextRequest {
    const ARG: EventArg = EventArg::Value(PropKind::Record(&SET_TEXT_REQUEST_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        let json = json_of(args)?;
        Some(SetTextRequest { field: text(&json, "field"), text: text(&json, "text") })
    }
}

/// Um campo e o valor dele, no texto canônico.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldValue {
    pub field: String,
    pub value: String,
}

/// O `on_master_change`: o mestre mudou de registro, e estes são os valores dos campos dele
/// ligados ao detalhe (para o backend carregar os detalhes sob demanda).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MasterChange {
    pub values: Vec<FieldValue>,
}

impl MasterChange {
    /// O valor de um campo do mestre (sem diferença de maiúsculas).
    pub fn get(&self, field: &str) -> Option<&str> {
        self.values.iter().find(|v| v.field.eq_ignore_ascii_case(field)).map(|v| v.value.as_str())
    }

    pub fn to_json(&self) -> String {
        let values: Vec<Json> = self.values.iter().map(|v| serde_json::json!({ "field": v.field, "value": v.value })).collect();
        serde_json::json!({ "values": values }).to_string()
    }
}

impl EventValue for MasterChange {
    const ARG: EventArg = EventArg::Value(PropKind::Record(&MASTER_CHANGE_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        let json = json_of(args)?;
        let values = json["values"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|v| FieldValue { field: text(v, "field"), value: text(v, "value") })
            .collect();
        Some(MasterChange { values })
    }
}

/// O `on_server_query`: com só parte da consulta carregada, ordenar, filtrar ou localizar pede
/// ao backend que refaça a consulta.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ServerQuery {
    /// `sort` (ordenar), `filter` (filtrar) ou `locate` (localizar).
    pub kind: String,
    /// A ordem pedida (`cliente;total desc`; vazio: a de chegada).
    pub index: String,
    /// O filtro em vigor (vazio: sem filtro).
    pub filter: String,
    /// Quantos registros estão carregados.
    pub loaded: i64,
    /// Só no `locate`: o campo e o valor procurados (no texto canônico).
    pub field: String,
    pub value: String,
}

impl ServerQuery {
    pub fn to_json(&self) -> String {
        serde_json::json!({
            "kind": self.kind,
            "index": self.index,
            "filter": self.filter,
            "loaded": self.loaded,
            "field": self.field,
            "value": self.value,
        })
        .to_string()
    }
}

impl EventValue for ServerQuery {
    const ARG: EventArg = EventArg::Value(PropKind::Record(&SERVER_QUERY_RECORD));
    fn from_args(args: &EventArgs) -> Option<Self> {
        let json = json_of(args)?;
        Some(ServerQuery {
            kind: text(&json, "kind"),
            index: text(&json, "index"),
            filter: text(&json, "filter"),
            loaded: json["loaded"].as_i64().unwrap_or_default(),
            field: text(&json, "field"),
            value: text(&json, "value"),
        })
    }
}

/// Um valor no texto canônico de volta ao tipo do campo.
pub(crate) fn value_from_text(text: &str, field: &Field) -> Result<Value, String> {
    let json = match field.field_type {
        // Número, lógico e bytes: o texto é o JSON deles.
        FieldType::Integer | FieldType::AutoInc | FieldType::Boolean | FieldType::Blob => {
            serde_json::from_str(text).unwrap_or_else(|_| Json::String(text.to_string()))
        }
        _ => Json::String(text.to_string()),
    };
    if text.is_empty() && field.field_type != FieldType::String && field.field_type != FieldType::Memo {
        return Ok(Value::Null);
    }
    crate::json::json_to_value(&json, field)
}

/// Responde um `on_get_text`: o texto de exibição de cada valor pedido (o grid e os
/// controles de dados repintam com eles).
pub fn get_text(ui: &mut Ui, field: ComponentId, args: &EventArgs, text: impl Fn(&Value) -> String) {
    let Some(request) = GetTextRequest::from_args(args) else { return };
    let Some(cds) = ui.parent(field) else { return };
    let Some(f) = ui.get::<Field>(field) else { return };
    let texts: Vec<(Value, String)> = request
        .values
        .iter()
        .filter_map(|v| value_from_text(v, f).ok())
        .map(|v| {
            let t = text(&v);
            (v, t)
        })
        .collect();
    let _ = data_set(ui, cds).set_field_texts(&request.field, texts);
}

/// Responde um `on_set_text`: o texto digitado vira o valor do campo, ou a mensagem de
/// erro marca o campo (o `post` recusa enquanto houver).
pub fn set_text(ui: &mut Ui, field: ComponentId, args: &EventArgs, value: impl Fn(&str) -> Result<Value, String>) {
    let Some(request) = SetTextRequest::from_args(args) else { return };
    let Some(cds) = ui.parent(field) else { return };
    let mut ds = data_set(ui, cds);
    match value(&request.text) {
        Ok(v) => {
            let _ = ds.set_value(&request.field, v);
        }
        Err(message) => {
            let _ = ds.set_field_error(&request.field, Some(message));
        }
    }
}
