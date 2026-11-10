//! Dados em JSON: carga de registros, delta e resposta do backend.
//!
//! Formatos (os mesmos do protocolo com backends em qualquer linguagem):
//!
//! ```json
//! [ { "id": 1, "nome": "Maria", "limite": "1500.00", "nascimento": "1990-04-12" } ]
//! ```
//!
//! Delta (enviado em `on_apply_updates`):
//! ```json
//! { "data_set": "cdsClientes", "changes": [
//!     { "ref": 1, "op": "update", "data_set": "cdsClientes", "key": { "id": 5 }, "old": { "email": "a@x.com" }, "new": { "email": "b@x.com" } },
//!     { "ref": 2, "op": "insert", "data_set": "cdsClientes", "key": { "id": -1 }, "new": { "id": -1, "nome": "João" } },
//!     { "ref": 3, "op": "delete", "data_set": "cdsClientes", "key": { "id": 9 }, "old": { ... } } ] }
//! ```
//!
//! Com detalhes (mestre-detalhe sem `apply_separately`), as alterações deles vêm no mesmo
//! delta, na ordem de gravar; o detalhe incluído sob um mestre incluído diz de quem depende:
//! ```json
//! { "ref": 8, "op": "insert", "data_set": "cdsItens", "key": { "id": -1 }, "new": { "id": -1, "pedido_id": -1 },
//!   "master": { "data_set": "cdsPedidos", "ref": 2, "fields": { "pedido_id": "id" } } }
//! ```
//!
//! Resposta (para `reconcile_json`; o `ref` acha o dataset de cada registro, detalhes
//! incluídos):
//! ```json
//! { "results": [ { "ref": 1 }, { "ref": 2, "values": { "id": 1043 } }, { "ref": 3, "error": "Cliente possui pedidos" } ] }
//! ```
//!
//! Decimais vão como texto (`"1500.00"`) para não perder precisão; datas e horas em ISO
//! (`"2024-05-01"`, `"13:45:00"`, `"2024-05-01T13:45:00"`).

use serde_json::{Map, Value as Json, json};

use crate::dataset::{ApplyResult, Change, DataError, DataResult};
use crate::field::{Field, FieldType};
use crate::handle::DataSet;
use crate::value::{Date, DateTime, Decimal, Time, Timestamp, Value};

pub fn value_to_json(value: &Value) -> Json {
    match value {
        Value::Null => Json::Null,
        Value::Bool(b) => Json::Bool(*b),
        Value::Int(i) => Json::from(*i),
        Value::Float(f) => serde_json::Number::from_f64(*f).map_or(Json::Null, Json::Number),
        Value::Decimal(d) => Json::String(d.to_string()),
        Value::String(s) => Json::String(s.clone()),
        Value::Date(d) => Json::String(d.to_string()),
        Value::Time(t) => Json::String(t.to_string()),
        Value::DateTime(dt) => Json::String(dt.to_string()),
        // Com o fuso (UTC): qualquer linguagem lê o mesmo instante.
        Value::Timestamp(ts) => Json::String(ts.to_string()),
        Value::Blob(b) => Json::Array(b.iter().map(|x| Json::from(*x)).collect()),
    }
}

/// JSON → valor no tipo do campo.
pub fn json_to_value(json: &Json, field: &Field) -> Result<Value, String> {
    let caption = field.caption();
    let wrong = || format!("{caption}: valor JSON inválido para o tipo do campo: {json}");
    if json.is_null() {
        return Ok(Value::Null);
    }
    let value = match field.field_type {
        FieldType::String | FieldType::Memo => Value::String(json.as_str().map(str::to_string).unwrap_or_else(|| json.to_string())),
        FieldType::Integer | FieldType::AutoInc => Value::Int(json.as_i64().ok_or_else(wrong)?),
        FieldType::Float => Value::Float(match json {
            Json::String(s) => s.parse().map_err(|_| wrong())?,
            other => other.as_f64().ok_or_else(wrong)?,
        }),
        FieldType::Currency => Value::Decimal(match json {
            Json::String(s) => Decimal::parse(s).ok_or_else(wrong)?,
            Json::Number(n) => Decimal::parse(&n.to_string()).ok_or_else(wrong)?,
            _ => return Err(wrong()),
        }),
        FieldType::Boolean => Value::Bool(json.as_bool().ok_or_else(wrong)?),
        FieldType::Date => Value::Date(json.as_str().and_then(Date::parse_iso).ok_or_else(wrong)?),
        FieldType::Time => Value::Time(json.as_str().and_then(Time::parse_iso).ok_or_else(wrong)?),
        FieldType::DateTime => Value::DateTime(json.as_str().and_then(DateTime::parse_iso).ok_or_else(wrong)?),
        FieldType::Timestamp => Value::Timestamp(json.as_str().and_then(Timestamp::parse_iso).ok_or_else(wrong)?),
        FieldType::Blob => Value::Blob(
            json.as_array()
                .ok_or_else(wrong)?
                .iter()
                .map(|b| b.as_u64().and_then(|b| u8::try_from(b).ok()).ok_or_else(wrong))
                .collect::<Result<_, _>>()?,
        ),
    };
    field.coerce(value)
}

fn pairs(values: &[(String, Value)]) -> Json {
    Json::Object(values.iter().map(|(k, v)| (k.clone(), value_to_json(v))).collect())
}

/// Delta em JSON (veja o formato no topo do módulo).
pub fn delta_to_json(data_set: &str, changes: &[Change]) -> String {
    let changes: Vec<Json> = changes
        .iter()
        .map(|c| {
            let mut m = Map::new();
            m.insert("ref".into(), Json::from(c.reference));
            m.insert("op".into(), Json::String(c.kind.name().into()));
            m.insert("data_set".into(), Json::String(if c.data_set.is_empty() { data_set.to_string() } else { c.data_set.clone() }));
            m.insert("key".into(), pairs(&c.key));
            if let Some(link) = &c.master {
                let fields: Map<String, Json> = link.fields.iter().map(|(d, m)| (d.clone(), Json::String(m.clone()))).collect();
                m.insert("master".into(), json!({ "data_set": link.data_set, "ref": link.reference, "fields": fields }));
            }
            if !c.old.is_empty() {
                m.insert("old".into(), pairs(&c.old));
            }
            if !c.new.is_empty() {
                m.insert("new".into(), pairs(&c.new));
            }
            Json::Object(m)
        })
        .collect();
    json!({ "data_set": data_set, "changes": changes }).to_string()
}

impl DataSet<'_> {
    /// Substitui os dados por uma lista de objetos JSON (um por registro, chaves = nomes
    /// dos campos; campos ausentes ficam `Null`).
    pub fn load_json(&mut self, rows: &Json) -> DataResult {
        self.open()?;
        let loaded = self.rows_from_json(rows)?;
        self.load_rows(loaded)
    }

    /// Acrescenta o próximo pacote (lista de objetos JSON) da busca sob demanda.
    pub fn append_json(&mut self, rows: &Json, has_more: bool) -> DataResult {
        self.open()?;
        let loaded = self.rows_from_json(rows)?;
        self.append_rows(loaded, has_more)
    }

    /// Substitui os dados pelo formato colunar (item 13: a carga grande, sem repetir os nomes
    /// dos campos em cada registro): `fields` com os nomes e `columns` com uma lista de valores
    /// por campo, na mesma ordem. Campos do dataset que não vêm ficam `Null`.
    pub fn load_columns(&mut self, fields: &Json, columns: &Json) -> DataResult {
        self.open()?;
        let names: Vec<&str> = fields.as_array().ok_or_else(|| DataError::new("fields: a lista dos nomes dos campos"))?.iter().map(|f| f.as_str().unwrap_or_default()).collect();
        let columns = columns.as_array().ok_or_else(|| DataError::new("columns: uma lista de valores por campo"))?;
        if columns.len() != names.len() {
            return Err(DataError::new(format!("columns tem {} listas e fields {} nomes", columns.len(), names.len())));
        }
        let lists: Vec<&Vec<Json>> = columns.iter().map(|c| c.as_array().ok_or_else(|| DataError::new("cada coluna é uma lista"))).collect::<DataResult<_>>()?;
        let count = lists.first().map_or(0, |l| l.len());
        if lists.iter().any(|l| l.len() != count) {
            return Err(DataError::new("as colunas têm tamanhos diferentes"));
        }
        let view = self.view();
        let dataset_names = view.field_names();
        // Para cada campo do dataset, a coluna que traz os valores dele (ou nenhuma).
        let sources: Vec<(Option<&Vec<Json>>, &Field)> = dataset_names
            .iter()
            .map(|name| {
                let field = view.field(name)?;
                let col = names.iter().position(|n| n.eq_ignore_ascii_case(name)).map(|i| lists[i]);
                Ok((col, field))
            })
            .collect::<DataResult<_>>()?;
        let mut loaded = Vec::with_capacity(count);
        for n in 0..count {
            let mut values = Vec::with_capacity(sources.len());
            for (col, field) in &sources {
                let json = col.map_or(&Json::Null, |c| &c[n]);
                values.push(json_to_value(json, field).map_err(|e| DataError::new(format!("registro {n}: {e}")))?);
            }
            loaded.push(values);
        }
        self.load_rows(loaded)
    }

    /// Os registros de uma lista de objetos JSON, convertidos pelos campos (cada campo é achado
    /// uma vez, não a cada valor).
    fn rows_from_json(&self, rows: &Json) -> DataResult<Vec<Vec<Value>>> {
        let list = rows.as_array().ok_or_else(|| DataError::new("os dados devem ser uma lista de objetos"))?;
        let view = self.view();
        let names = view.field_names();
        let fields: Vec<&Field> = names.iter().map(|n| view.field(n)).collect::<DataResult<_>>()?;
        let mut loaded = Vec::with_capacity(list.len());
        for (n, item) in list.iter().enumerate() {
            let obj = item.as_object().ok_or_else(|| DataError::new(format!("registro {n}: esperado um objeto")))?;
            let mut values = Vec::with_capacity(names.len());
            for (name, field) in names.iter().zip(&fields) {
                let json = obj.get(name).unwrap_or(&Json::Null);
                values.push(json_to_value(json, field).map_err(|e| DataError::new(format!("registro {n}: {e}")))?);
            }
            loaded.push(values);
        }
        Ok(loaded)
    }

    /// Aplica a resposta do backend em JSON (veja o formato no topo do módulo).
    pub fn reconcile_json(&mut self, response: &str) -> DataResult {
        let results = self.parse_results(response)?;
        self.reconcile(&results)
    }

    /// Lê a resposta do backend em JSON sem aplicá-la.
    pub fn parse_results(&self, response: &str) -> DataResult<Vec<ApplyResult>> {
        let json: Json = serde_json::from_str(response).map_err(|e| DataError::new(format!("resposta inválida: {e}")))?;
        let list = json["results"].as_array().ok_or_else(|| DataError::new("a resposta precisa de \"results\""))?;
        let mut results = Vec::with_capacity(list.len());
        for item in list {
            let reference = item["ref"].as_u64().ok_or_else(|| DataError::new("cada resultado precisa de \"ref\""))?;
            let mut result = ApplyResult { reference, error: item["error"].as_str().map(str::to_string), values: Vec::new() };
            if let Some(values) = item["values"].as_object() {
                // O registro pode ser de um detalhe (no delta do mestre): os campos são os dele.
                let this = self.view();
                let owner = this.other(this.owner_of(reference).unwrap_or(this.id()));
                for (name, json) in values {
                    let field = owner.field(name)?;
                    result.values.push((name.clone(), json_to_value(json, field).map_err(DataError::new)?));
                }
            }
            results.push(result);
        }
        Ok(results)
    }
}
