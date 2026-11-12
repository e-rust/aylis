//! Conversão entre valores JSON e [`PropValue`], guiada pelo tipo da propriedade.

use aylis_core::{Bounds, Color, PropKind, PropValue, Theme};
use serde_json::{Map, Number, Value};

/// JSON → valor de propriedade. `theme` resolve nomes de tokens em cores.
pub(crate) fn to_prop(value: &Value, kind: PropKind, theme: &Theme) -> Result<PropValue, String> {
    if value.is_null() {
        return Ok(match kind {
            PropKind::Event => PropValue::Event(None),
            PropKind::Component => PropValue::Component(None),
            _ => PropValue::Null,
        });
    }
    let wrong = |expected: &str| format!("esperado {expected}, recebido {}", describe(value));
    Ok(match kind {
        PropKind::Bool => PropValue::Bool(value.as_bool().ok_or_else(|| wrong("true/false"))?),
        PropKind::Int => match value.as_i64() {
            Some(v) => PropValue::Int(v),
            None => match value.as_f64() {
                Some(f) if f.fract() == 0.0 => PropValue::Int(f as i64),
                _ => return Err(wrong("número inteiro")),
            },
        },
        PropKind::Float | PropKind::Angle | PropKind::Duration => PropValue::Float(value.as_f64().ok_or_else(|| wrong("número"))?),
        // Um nome de sombra do tema (`"md"`) também vale no form, como no tema: vira a sombra
        // dele (relatório do RTSPlayer, 10/11: "md" valia no tema e era recusado no form).
        PropKind::Shadow => {
            let text = value.as_str().ok_or_else(|| wrong("sombra em texto (\"0 4 12 #00000040\") ou um nome do tema (\"md\")"))?;
            match theme.shadows.get(text.trim()) {
                Some(shadow) => PropValue::String(shadow.to_spec()),
                None => PropValue::String(text.to_string()),
            }
        }
        PropKind::Easing => PropValue::String(value.as_str().ok_or_else(|| wrong("curva (\"out_quad\", \"ease-in\" ou \"cubic-bezier(...)\")"))?.to_string()),
        PropKind::String => PropValue::String(value.as_str().ok_or_else(|| wrong("texto"))?.to_string()),
        PropKind::Color => {
            let text = value.as_str().ok_or_else(|| wrong("cor em texto (\"#RRGGBB\", um nome ou nome/NN)"))?;
            let c = color(text, theme)?;
            // Com nome, vai o nome: o `Ui` guarda e recalcula ao trocar de tema.
            if aylis_core::palette::is_named(text) { PropValue::String(text.trim().to_string()) } else { PropValue::Color(c) }
        }
        PropKind::Enum(names) => {
            let text = value.as_str().ok_or_else(|| wrong(&format!("um de: {}", names.join(", "))))?;
            PropValue::Enum(text.to_string())
        }
        PropKind::Set(_) => match value {
            Value::String(s) => PropValue::String(s.clone()),
            Value::Array(items) => PropValue::Set(
                items
                    .iter()
                    .map(|i| i.as_str().map(str::to_string).ok_or_else(|| wrong("lista de nomes")))
                    .collect::<Result<_, _>>()?,
            ),
            _ => return Err(wrong("lista de nomes")),
        },
        PropKind::Bounds => PropValue::Bounds(bounds(value)?),
        PropKind::Event => {
            let handler = value.as_str().ok_or_else(|| wrong("nome do handler"))?;
            if !handler.is_empty() && !aylis_core::is_identifier(handler) {
                return Err(format!("nome de handler inválido '{handler}' (letras, dígitos e _, sem começar por dígito)"));
            }
            PropValue::Event(Some(handler.to_string()))
        }
        PropKind::Component => PropValue::String(value.as_str().ok_or_else(|| wrong("nome do componente"))?.to_string()),
    })
}

/// Valor de propriedade → JSON (na forma mais curta).
pub(crate) fn from_prop(value: &PropValue) -> Value {
    match value {
        PropValue::Null | PropValue::Component(None) | PropValue::Event(None) => Value::Null,
        PropValue::Bool(b) => Value::Bool(*b),
        PropValue::Int(i) => Value::from(*i),
        PropValue::Float(f) => number(*f),
        PropValue::String(s) | PropValue::Enum(s) | PropValue::Event(Some(s)) => Value::String(s.clone()),
        PropValue::Color(c) => Value::String(c.to_hex()),
        PropValue::Set(items) => Value::Array(items.iter().cloned().map(Value::String).collect()),
        PropValue::Bounds(b) => bounds_value(*b),
        // Referências são gravadas pelo nome em `layout.rs`.
        PropValue::Component(Some(_)) => Value::Null,
    }
}

/// `"#RRGGBB"`, `"#RRGGBBAA"`, um nome do tema ou do projeto, ou `nome/NN` (opacidade).
pub(crate) fn color(text: &str, theme: &Theme) -> Result<Color, String> {
    // `#2563EB` ou, com a opacidade, `#2563EB/15` (o `resolve_color` entende os dois).
    if text.trim_start().starts_with('#') {
        return theme.resolve_color(text).ok_or_else(|| format!("cor inválida: '{text}' (use #RRGGBB, #RRGGBBAA ou #RRGGBB/NN)"));
    }
    theme.resolve_color(text).ok_or_else(|| format!("'{text}' não é uma cor (#RRGGBB) nem um nome de cor do tema ou do projeto"))
}

/// `8`, `[h, v]`, `[l, t, r, b]` ou `{ "left": .., "top": .., "right": .., "bottom": .. }`.
pub(crate) fn bounds(value: &Value) -> Result<Bounds, String> {
    let num = |v: &Value| v.as_f64().map(|f| f as f32).ok_or_else(|| format!("esperado número, recebido {}", describe(v)));
    match value {
        Value::Number(_) => Ok(Bounds::all(num(value)?)),
        Value::Array(items) => match items.as_slice() {
            [h, v] => Ok(Bounds::symmetric(num(h)?, num(v)?)),
            [l, t, r, b] => Ok(Bounds::new(num(l)?, num(t)?, num(r)?, num(b)?)),
            _ => Err("use 1, 2 ([h, v]) ou 4 ([l, t, r, b]) números".into()),
        },
        Value::Object(map) => {
            let mut b = Bounds::ZERO;
            for (key, v) in map {
                let target = match key.as_str() {
                    "left" => &mut b.left,
                    "top" => &mut b.top,
                    "right" => &mut b.right,
                    "bottom" => &mut b.bottom,
                    other => return Err(format!("lado desconhecido '{other}' (use left, top, right, bottom)")),
                };
                *target = num(v)?;
            }
            Ok(b)
        }
        _ => Err(format!("esperado número, lista ou objeto, recebido {}", describe(value))),
    }
}

/// Forma mais curta de um `Bounds`.
pub(crate) fn bounds_value(b: Bounds) -> Value {
    if b.left == b.top && b.top == b.right && b.right == b.bottom {
        number(b.left as f64)
    } else if b.left == b.right && b.top == b.bottom {
        Value::Array(vec![number(b.left as f64), number(b.top as f64)])
    } else {
        Value::Array([b.left, b.top, b.right, b.bottom].iter().map(|v| number(*v as f64)).collect())
    }
}

/// Número sem o "ruído" da conversão de f32 (0.3 em vez de 0.30000001192092896) e sem
/// casa decimal quando é inteiro.
pub(crate) fn number(f: f64) -> Value {
    let clean: f64 = format!("{}", f as f32).parse().unwrap_or(f);
    if clean.fract() == 0.0 && clean.abs() < 1e15 {
        Value::from(clean as i64)
    } else {
        Number::from_f64(clean).map_or(Value::Null, Value::Number)
    }
}

pub(crate) fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{s}\""),
        Value::Array(_) => "uma lista".into(),
        Value::Object(_) => "um objeto".into(),
    }
}

pub(crate) fn object<'a>(value: &'a Value, what: &str) -> Result<&'a Map<String, Value>, String> {
    value.as_object().ok_or_else(|| format!("{what} deve ser um objeto, recebido {}", describe(value)))
}
