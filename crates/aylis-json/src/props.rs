//! Propriedades individuais em JSON (usado pelo protocolo remoto e pela C ABI).

use aylis_core::{ComponentId, PropValue, Ui};
use serde_json::{Map, Value};

use crate::value::{from_prop, to_prop};

/// Converte um valor JSON para a propriedade `name` do componente, conforme o tipo dela.
/// Aceita `"margins.left"` e afins; referências a componentes vão pelo nome.
pub fn prop_from_json(ui: &Ui, id: ComponentId, name: &str, value: &Value) -> Result<PropValue, String> {
    if name == "name" {
        return value.as_str().map(PropValue::from).ok_or_else(|| "\"name\" deve ser texto".to_string());
    }
    if name.contains('.') {
        return value.as_f64().map(PropValue::Float).ok_or_else(|| format!("'{name}' deve ser número"));
    }
    let info = ui.prop_info(id, name).ok_or_else(|| {
        let class = ui.class_name(id).unwrap_or("?");
        format!("a classe {class} não tem a propriedade '{name}'")
    })?;
    to_prop(value, info.kind, ui.theme())
}

/// Converte o valor de uma propriedade para JSON. Referências a componentes viram o
/// caminho (`frmClientes.dsClientes`, sem ambiguidade entre forms) ou o id numérico, se o
/// componente não tiver nome.
pub fn prop_to_json(ui: &Ui, value: &PropValue) -> Value {
    match value {
        PropValue::Component(Some(target)) => match ui.name(*target).filter(|n| !n.is_empty()) {
            Some(_) => Value::String(ui.path_of(*target)),
            None => Value::from(target.to_u64()),
        },
        other => from_prop(other),
    }
}

/// Grava várias propriedades (`{"text": "OK", "width": 100, "on_click": "salvar"}`).
/// Para no primeiro erro, informando qual propriedade falhou.
pub fn set_props(ui: &mut Ui, id: ComponentId, props: &Map<String, Value>) -> Result<(), String> {
    for (name, value) in props {
        let prop = prop_from_json(ui, id, name, value)?;
        ui.set_prop(id, name, prop).map_err(|e| e.to_string())?;
    }
    Ok(())
}
