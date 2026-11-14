//! Ajudas para os handlers por nome dos eventos de texto do [`Field`] (o código gerado
//! pela IDE recebe `ui, sender, args`): `on_get_text` e `on_set_text`.
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

use aylis_core::{ComponentId, EventArgs, Ui};

use crate::field::Field;
use crate::handle::data_set;
use crate::value::Value;

/// O pedido do evento (`{"field": ..., ...}`) e o dataset dono do campo.
fn request(ui: &Ui, field: ComponentId, args: &EventArgs) -> Option<(serde_json::Value, ComponentId, String)> {
    let json: serde_json::Value = serde_json::from_str(args.text()?).ok()?;
    let name = json["field"].as_str()?.to_string();
    let cds = ui.parent(field)?;
    Some((json, cds, name))
}

/// Responde um `on_get_text`: o texto de exibição de cada valor pedido (o grid e os
/// controles de dados repintam com eles).
pub fn get_text(ui: &mut Ui, field: ComponentId, args: &EventArgs, text: impl Fn(&Value) -> String) {
    let Some((json, cds, name)) = request(ui, field, args) else { return };
    let Some(f) = ui.get::<Field>(field) else { return };
    let texts: Vec<(Value, String)> = json["values"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|j| crate::json::json_to_value(j, f).ok())
        .map(|v| {
            let t = text(&v);
            (v, t)
        })
        .collect();
    let _ = data_set(ui, cds).set_field_texts(&name, texts);
}

/// Responde um `on_set_text`: o texto digitado vira o valor do campo, ou a mensagem de
/// erro marca o campo (o `post` recusa enquanto houver).
pub fn set_text(ui: &mut Ui, field: ComponentId, args: &EventArgs, value: impl Fn(&str) -> Result<Value, String>) {
    let Some((json, cds, name)) = request(ui, field, args) else { return };
    let typed = json["text"].as_str().unwrap_or_default();
    let mut ds = data_set(ui, cds);
    match value(typed) {
        Ok(v) => {
            let _ = ds.set_value(&name, v);
        }
        Err(message) => {
            let _ = ds.set_field_error(&name, Some(message));
        }
    }
}
