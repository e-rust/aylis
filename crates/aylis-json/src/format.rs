//! O texto dos forms gravados: um formato fixo, para o arquivo ser bom de comparar e de juntar
//! (merge) quando várias pessoas mexem no mesmo form (item 22).
//!
//! - uma chave por linha, na ordem do valor (a gravação já põe `class`, `name`, as
//!   propriedades na ordem da classe, `bindings`, `views` e `children` por último);
//! - listas só de números, textos ou booleanos numa linha (`[24, 10]`): mudar uma margem muda
//!   uma linha;
//! - objetos e listas de objetos abertos, dois espaços por nível;
//! - fim de linha `\n` e uma quebra no fim do arquivo.
//!
//! Abrir e salvar sem mudança dá o mesmo arquivo (o teste `form_roundtrip` da IDE confere em
//! todos os exemplos).

use serde_json::Value;

/// O texto de um documento de form (ou de qualquer JSON do aylis) no formato fixo, com a quebra
/// de linha no fim.
pub fn to_form_text(value: &Value) -> String {
    let mut out = String::new();
    write(&mut out, value, 0);
    out.push('\n');
    out
}

fn scalar(value: &Value) -> bool {
    !matches!(value, Value::Array(_) | Value::Object(_))
}

fn write(out: &mut String, value: &Value, level: usize) {
    let indent = |out: &mut String, level: usize| out.extend(std::iter::repeat_n("  ", level));
    match value {
        Value::Object(map) if map.is_empty() => out.push_str("{}"),
        Value::Object(map) => {
            out.push_str("{\n");
            for (i, (k, v)) in map.iter().enumerate() {
                indent(out, level + 1);
                out.push_str(&Value::String(k.clone()).to_string());
                out.push_str(": ");
                write(out, v, level + 1);
                if i + 1 < map.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(out, level);
            out.push('}');
        }
        Value::Array(items) if items.is_empty() => out.push_str("[]"),
        Value::Array(items) if items.iter().all(scalar) => {
            out.push('[');
            for (i, v) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&v.to_string());
            }
            out.push(']');
        }
        Value::Array(items) => {
            out.push_str("[\n");
            for (i, v) in items.iter().enumerate() {
                indent(out, level + 1);
                write(out, v, level + 1);
                if i + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(out, level);
            out.push(']');
        }
        other => out.push_str(&other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn short_lists_stay_on_one_line_and_objects_open_up() {
        let doc = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmX", "padding": [24, 10], "items": [], "changes": {}, "children": [{ "class": "Button", "text": "Olá \"x\"" }] } });
        let text = to_form_text(&doc);
        assert_eq!(
            text,
            "{\n  \"aylis\": 1,\n  \"root\": {\n    \"class\": \"Form\",\n    \"name\": \"frmX\",\n    \"padding\": [24, 10],\n    \"items\": [],\n    \"changes\": {},\n    \"children\": [\n      {\n        \"class\": \"Button\",\n        \"text\": \"Olá \\\"x\\\"\"\n      }\n    ]\n  }\n}\n"
        );
        let back: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(back, doc, "lê de volta igual");
    }
}
