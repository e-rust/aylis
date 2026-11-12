//! Temas em JSON.
//!
//! ```json
//! {
//!   "name": "Aylis Dark",
//!   "colors":    { "primary": "#3B82F6", "text": "#E6E6E6" },
//!   "fonts":     { "default": { "family": "Inter", "size": 14 }, "title": { "size": 20, "weight": "bold" } },
//!   "radius":    { "sm": 4, "md": 6 },
//!   "spacing":   { "sm": 8 },
//!   "durations": { "fast": 120 },
//!   "styles": {
//!     "Button": {
//!       "fill": "primary", "text": "#FFFFFF", "radius": "md", "padding": [12, 6], "transition": "fast",
//!       "hover":   { "fill": "primary_hover" },
//!       "focused": { "border": ["focus_ring", 2] },
//!       "disabled": { "opacity": 0.5 }
//!     },
//!     "Button.danger": { "fill": "danger" },
//!     "Panel.card": { "fill": "surface", "radius": "lg", "shadow": "sm" }
//!   },
//!   "shadows":    { "sm": { "color": "#0000001A", "blur": 4, "offset": [0, 1] } },
//!   "components": { "Button": "Aylis Dark" }
//! }
//! ```
//!
//! `look` escolhe a forma dos controles (`aylis`, `material2`, `material3`, `ios7`, `ios13`,
//! `ios26`, `win7`, `win10`, `win11`); `family` e `dark` agrupam o claro e o escuro na IDE.
//! Um tema próprio pode partir de um de plataforma só trocando as cores.
//!
//! `base` parte de um tema da biblioteca (`"base": "Aylis Dark"`): o arquivo só diz o que
//! muda (cores, fontes, estilos de algumas classes), o resto vem dele.
//!
//! `components` mistura temas: as classes listadas usam o visual de outro tema da
//! biblioteca do `Ui` (pelo nome). Uma variante nova em `styles` (`"Button.minha"`)
//! aparece sozinha como subtipo na paleta da IDE.
//!
//! Nos estilos, cor/medida em texto é um token do tema; `"#..."` é cor fixa; número é
//! medida fixa. Só referências e valores — nenhuma expressão.

use std::collections::HashMap;

use aylis_core::{Color, ColorRef, FontSpec, FontWeight, Look, Metric, PointF, PropEnum, Shadow, Style, StyleProps, Theme};
use serde_json::{Map, Value};

use crate::value::{bounds, bounds_value, describe, number, object};
use crate::{Error, Result};

const STATES: [&str; 5] = ["hover", "pressed", "focused", "checked", "disabled"];

/// Lê um tema de um texto JSON.
pub fn load_theme(json: &str) -> Result<Theme> {
    let value: Value = serde_json::from_str(json.trim_start_matches('\u{feff}'))?;
    theme_from_value(&value)
}

/// Grava o tema como JSON formatado (chaves em ordem alfabética, para diffs estáveis).
pub fn save_theme(theme: &Theme) -> String {
    serde_json::to_string_pretty(&theme_to_value(theme)).expect("valores JSON sempre serializam")
}

pub fn theme_from_value(value: &Value) -> Result<Theme> {
    let root = object(value, "o tema").map_err(fail(""))?;
    let name = root.get("name").and_then(Value::as_str).unwrap_or("");
    let mut theme = match root.get("base") {
        None => Theme::new(name),
        Some(base) => {
            let base = base.as_str().ok_or_else(|| fail(" > base")("\"base\" deve ser o nome de um tema".into()))?;
            let mut t = aylis_core::themes::builtin(base)
                .cloned()
                .ok_or_else(|| fail(" > base")(format!("tema '{base}' não existe na biblioteca")))?;
            t.name = name.to_string();
            t
        }
    };
    for (key, value) in root {
        let path = format!(" > {key}");
        match key.as_str() {
            "name" | "$schema" | "base" => {}
            "family" => theme.family = value.as_str().ok_or_else(|| fail(&path)("\"family\" deve ser texto".into()))?.to_string(),
            "dark" => theme.dark = value.as_bool().ok_or_else(|| fail(&path)("\"dark\" deve ser true/false".into()))?,
            "look" => {
                let name = value.as_str().ok_or_else(|| fail(&path)("\"look\" deve ser texto".into()))?;
                let index = Look::NAMES.iter().position(|n| *n == name).ok_or_else(|| {
                    fail(&path)(format!("look '{name}' desconhecido; use um de: {}", Look::NAMES.join(", ")))
                })?;
                theme.look = Look::from_index(index).unwrap_or_default();
            }
            "colors" => {
                // `#hex` ou o nome de outra cor (`nome/NN` com opacidade): os nomes se
                // resolvem depois dos hex, em passadas, porque um pode citar o outro.
                let mut named = Vec::new();
                for (token, v) in object(value, "colors").map_err(fail(&path))? {
                    let text = v.as_str().ok_or_else(|| fail(&path)(format!("'{token}': esperado \"#RRGGBB\" ou o nome de uma cor")))?;
                    if aylis_core::palette::is_named(text) {
                        named.push((token.clone(), text.to_string()));
                    } else {
                        let color = Color::from_hex(text).ok_or_else(|| fail(&path)(format!("'{token}': cor inválida '{text}'")))?;
                        theme.colors.insert(token.clone(), color);
                    }
                }
                while !named.is_empty() {
                    let before = named.len();
                    named.retain(|(token, text)| match theme.resolve_color(text) {
                        Some(c) => {
                            theme.colors.insert(token.clone(), c);
                            false
                        }
                        None => true,
                    });
                    if named.len() == before {
                        let (token, text) = &named[0];
                        return Err(fail(&path)(format!("'{token}': cor desconhecida '{text}'")));
                    }
                }
            }
            "fonts" => {
                for (token, v) in object(value, "fonts").map_err(fail(&path))? {
                    theme.fonts.insert(token.clone(), font(v).map_err(|m| fail(&path)(format!("'{token}': {m}")))?);
                }
            }
            "radius" => theme.radius = metrics(value).map_err(fail(&path))?,
            "spacing" => theme.spacing = metrics(value).map_err(fail(&path))?,
            "durations" => theme.durations = metrics(value).map_err(fail(&path))?,
            "shadows" => {
                for (token, v) in object(value, "shadows").map_err(fail(&path))? {
                    theme.shadows.insert(token.clone(), shadow(v).map_err(|m| fail(&path)(format!("'{token}': {m}")))?);
                }
            }
            "components" => {
                for (class, v) in object(value, "components").map_err(fail(&path))? {
                    let name = v.as_str().ok_or_else(|| fail(&path)(format!("'{class}': esperado o nome de um tema")))?;
                    theme.mix(class, name);
                }
            }
            "styles" => {
                for (class, v) in object(value, "styles").map_err(fail(&path))? {
                    let style = style(v).map_err(|m| fail(&format!("{path} > {class}"))(m))?;
                    theme.set_style(class, style);
                }
            }
            other => {
                return Err(fail("")(format!(
                    "chave desconhecida '{other}' (use name, base, family, dark, look, colors, fonts, radius, spacing, durations, shadows, styles, components)"
                )));
            }
        }
    }
    // Sem `"dark"` no arquivo (nem um `base` escuro), o fundo decide: um tema de fundo escuro
    // é escuro (relatório do InsightDB, 11/11: `ui.theme().dark` vinha false com o tema escuro).
    if !root.contains_key("dark") && root.get("base").is_none() {
        let bg = theme.color("background");
        let luminance = 0.2126 * bg.r as f32 + 0.7152 * bg.g as f32 + 0.0722 * bg.b as f32;
        theme.dark = bg.a > 0 && luminance < 110.0;
    }
    Ok(theme)
}

fn fail(path: &str) -> impl Fn(String) -> Error + '_ {
    move |message| Error::Structure { path: format!("tema{path}"), message }
}

pub fn theme_to_value(theme: &Theme) -> Value {
    let mut root = Map::new();
    root.insert("name".into(), Value::String(theme.name.clone()));
    if !theme.family.is_empty() {
        root.insert("family".into(), Value::String(theme.family.clone()));
    }
    if theme.dark {
        root.insert("dark".into(), Value::Bool(true));
    }
    if theme.look != Look::Aylis {
        root.insert("look".into(), Value::String(Look::NAMES[theme.look.index()].to_string()));
    }
    root.insert("colors".into(), sorted(&theme.colors, |c| Value::String(c.to_hex())));
    root.insert("fonts".into(), sorted(&theme.fonts, font_value));
    root.insert("radius".into(), sorted(&theme.radius, |v| number(*v as f64)));
    root.insert("spacing".into(), sorted(&theme.spacing, |v| number(*v as f64)));
    root.insert("durations".into(), sorted(&theme.durations, |v| number(*v as f64)));
    root.insert("styles".into(), sorted(&theme.styles, style_value));
    if !theme.shadows.is_empty() {
        root.insert("shadows".into(), sorted(&theme.shadows, shadow_value));
    }
    if !theme.components.is_empty() {
        root.insert("components".into(), sorted(&theme.components, |n| Value::String(n.clone())));
    }
    Value::Object(root)
}

fn sorted<T>(map: &HashMap<String, T>, f: impl Fn(&T) -> Value) -> Value {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    Value::Object(keys.into_iter().map(|k| (k.clone(), f(&map[k]))).collect())
}

fn metrics(value: &Value) -> Result<HashMap<String, f32>, String> {
    object(value, "a lista de medidas")?
        .iter()
        .map(|(k, v)| {
            v.as_f64().map(|f| (k.clone(), f as f32)).ok_or_else(|| format!("'{k}': esperado número, recebido {}", describe(v)))
        })
        .collect()
}

// ----- fontes -----

const WEIGHTS: [(&str, FontWeight); 7] = [
    ("thin", FontWeight::Thin),
    ("light", FontWeight::Light),
    ("normal", FontWeight::Normal),
    ("medium", FontWeight::Medium),
    ("semi_bold", FontWeight::SemiBold),
    ("bold", FontWeight::Bold),
    ("black", FontWeight::Black),
];

fn font(value: &Value) -> Result<FontSpec, String> {
    let mut f = FontSpec::default();
    for (key, v) in object(value, "a fonte")? {
        match key.as_str() {
            "family" => f.family = v.as_str().ok_or("\"family\" deve ser texto")?.to_string(),
            "size" => f.size = v.as_f64().ok_or("\"size\" deve ser número")? as f32,
            "italic" => f.italic = v.as_bool().ok_or("\"italic\" deve ser true/false")?,
            "weight" => {
                let name = v.as_str().ok_or("\"weight\" deve ser texto")?;
                f.weight = WEIGHTS
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, w)| *w)
                    .ok_or_else(|| format!("peso '{name}' inválido; use um de: {}", WEIGHTS.map(|(n, _)| n).join(", ")))?;
            }
            other => return Err(format!("chave desconhecida '{other}' (use family, size, weight, italic)")),
        }
    }
    Ok(f)
}

fn font_value(f: &FontSpec) -> Value {
    let mut map = Map::new();
    map.insert("family".into(), Value::String(f.family.clone()));
    map.insert("size".into(), number(f.size as f64));
    let weight = WEIGHTS.iter().find(|(_, w)| *w == f.weight).map_or("normal", |(n, _)| n);
    map.insert("weight".into(), Value::String(weight.into()));
    if f.italic {
        map.insert("italic".into(), Value::Bool(true));
    }
    Value::Object(map)
}

// ----- sombras -----

fn shadow(value: &Value) -> Result<Shadow, String> {
    let mut sh = Shadow::new(Color::rgba(0, 0, 0, 40), 8.0, 0.0, 2.0);
    for (key, v) in object(value, "a sombra")? {
        match key.as_str() {
            "color" => {
                let text = v.as_str().ok_or("\"color\" deve ser \"#RRGGBBAA\"")?;
                sh.color = Color::from_hex(text).ok_or_else(|| format!("cor inválida '{text}'"))?;
            }
            "blur" => sh.blur = v.as_f64().ok_or("\"blur\" deve ser número")? as f32,
            "spread" => sh.spread = v.as_f64().ok_or("\"spread\" deve ser número")? as f32,
            "offset" => match v.as_array().map(Vec::as_slice) {
                Some([x, y]) => {
                    sh.offset = PointF::new(
                        x.as_f64().ok_or("deslocamento deve ser número")? as f32,
                        y.as_f64().ok_or("deslocamento deve ser número")? as f32,
                    );
                }
                _ => return Err("\"offset\" deve ser [x, y]".into()),
            },
            other => return Err(format!("chave desconhecida '{other}' (use color, blur, offset, spread)")),
        }
    }
    Ok(sh)
}

fn shadow_value(sh: &Shadow) -> Value {
    let mut map = Map::new();
    map.insert("color".into(), Value::String(sh.color.to_hex()));
    map.insert("blur".into(), number(sh.blur as f64));
    map.insert("offset".into(), Value::Array(vec![number(sh.offset.x as f64), number(sh.offset.y as f64)]));
    if sh.spread != 0.0 {
        map.insert("spread".into(), number(sh.spread as f64));
    }
    Value::Object(map)
}

// ----- estilos -----

fn style(value: &Value) -> Result<Style, String> {
    let obj = object(value, "o estilo")?;
    let mut style = Style::new(style_props(obj, true)?);
    for state in STATES {
        if let Some(v) = obj.get(state) {
            let props = style_props(object(v, state)?, false).map_err(|m| format!("{state}: {m}"))?;
            match state {
                "hover" => style.hover = props,
                "pressed" => style.pressed = props,
                "focused" => style.focused = props,
                "checked" => style.checked = props,
                _ => style.disabled = props,
            }
        }
    }
    Ok(style)
}

fn style_props(obj: &Map<String, Value>, allow_states: bool) -> Result<StyleProps, String> {
    let mut p = StyleProps::default();
    for (key, v) in obj {
        match key.as_str() {
            "fill" => p.fill = Some(color_ref(v)?),
            "text" => p.text = Some(color_ref(v)?),
            "border_color" => p.border_color = Some(color_ref(v)?),
            "border_width" => p.border_width = Some(v.as_f64().ok_or("\"border_width\" deve ser número")? as f32),
            "border" => match v.as_array().map(Vec::as_slice) {
                Some([color, width]) => {
                    p.border_color = Some(color_ref(color)?);
                    p.border_width = Some(width.as_f64().ok_or("largura da borda deve ser número")? as f32);
                }
                _ => return Err("\"border\" deve ser [cor, largura]".into()),
            },
            "radius" => p.radius = Some(metric(v)?),
            "transition" => p.transition = Some(metric(v)?),
            "padding" => p.padding = Some(bounds(v)?),
            "font" => p.font = Some(v.as_str().ok_or("\"font\" deve ser o nome de uma fonte do tema")?.to_string()),
            "opacity" => p.opacity = Some(v.as_f64().ok_or("\"opacity\" deve ser número")? as f32),
            "shadow" => p.shadow = Some(v.as_str().ok_or("\"shadow\" deve ser o nome de uma sombra do tema")?.to_string()),
            "material" => p.material = Some(v.as_str().ok_or("\"material\" deve ser glass, frosted ou none")?.to_string()),
            "text_case" => p.text_case = Some(v.as_str().ok_or("\"text_case\" deve ser upper, lower ou none")?.to_string()),
            "letter_spacing" => p.letter_spacing = Some(v.as_f64().ok_or("\"letter_spacing\" deve ser número")? as f32),
            "gap" => p.gap = Some(v.as_f64().ok_or("\"gap\" deve ser número (0 cola as partes)")? as f32),
            "border_sides" => p.border_sides = Some(bounds(v)?),
            "border_style" => p.border_style = Some(v.as_str().ok_or("\"border_style\" deve ser solid, dashed ou dotted")?.to_string()),
            "line_height" => p.line_height = Some(v.as_f64().ok_or("\"line_height\" deve ser número (1.5 = uma vez e meia a fonte)")? as f32),
            "font_size" => p.font_size = Some(v.as_f64().ok_or("\"font_size\" deve ser número")? as f32),
            "italic" => p.italic = Some(v.as_bool().ok_or("\"italic\" deve ser true/false")?),
            "font_weight" => {
                let name = v.as_str().ok_or("\"font_weight\" deve ser texto")?;
                p.font_weight = Some(
                    WEIGHTS
                        .iter()
                        .find(|(n, _)| *n == name)
                        .map(|(_, w)| *w)
                        .ok_or_else(|| format!("peso '{name}' inválido; use um de: {}", WEIGHTS.map(|(n, _)| n).join(", ")))?,
                );
            }
            state if allow_states && STATES.contains(&state) => {}
            other => {
                return Err(format!(
                    "chave desconhecida '{other}' (use fill, text, border, border_color, border_width, border_sides, \
                     border_style, radius, padding, font, font_weight, font_size, italic, line_height, opacity, transition, shadow, \
                     material, text_case, letter_spacing, gap{})",
                    if allow_states { " e os estados hover, pressed, focused, checked, disabled" } else { "" }
                ));
            }
        }
    }
    Ok(p)
}

fn color_ref(v: &Value) -> Result<ColorRef, String> {
    let text = v.as_str().ok_or_else(|| format!("cor deve ser texto, recebido {}", describe(v)))?;
    if text.trim_start().starts_with('#') {
        Color::from_hex(text).map(ColorRef::Value).ok_or_else(|| format!("cor inválida '{text}'"))
    } else {
        Ok(ColorRef::Token(text.to_string()))
    }
}

fn metric(v: &Value) -> Result<Metric, String> {
    match v {
        Value::String(s) => Ok(Metric::Token(s.clone())),
        Value::Number(n) => Ok(Metric::Value(n.as_f64().unwrap_or(0.0) as f32)),
        other => Err(format!("esperado token (texto) ou número, recebido {}", describe(other))),
    }
}

fn style_value(style: &Style) -> Value {
    let mut map = props_value(&style.base);
    for (state, props) in [
        ("hover", &style.hover),
        ("pressed", &style.pressed),
        ("focused", &style.focused),
        ("checked", &style.checked),
        ("disabled", &style.disabled),
    ] {
        if *props != StyleProps::default() {
            map.insert(state.into(), Value::Object(props_value(props)));
        }
    }
    Value::Object(map)
}

fn props_value(p: &StyleProps) -> Map<String, Value> {
    let color = |c: &ColorRef| match c {
        ColorRef::Token(t) => Value::String(t.clone()),
        ColorRef::Value(c) => Value::String(c.to_hex()),
    };
    let metric = |m: &Metric| match m {
        Metric::Token(t) => Value::String(t.clone()),
        Metric::Value(v) => number(*v as f64),
    };
    let mut map = Map::new();
    if let Some(c) = &p.fill {
        map.insert("fill".into(), color(c));
    }
    if let Some(c) = &p.text {
        map.insert("text".into(), color(c));
    }
    match (&p.border_color, p.border_width) {
        (Some(c), Some(w)) => {
            map.insert("border".into(), Value::Array(vec![color(c), number(w as f64)]));
        }
        (Some(c), None) => {
            map.insert("border_color".into(), color(c));
        }
        (None, Some(w)) => {
            map.insert("border_width".into(), number(w as f64));
        }
        (None, None) => {}
    }
    if let Some(m) = &p.radius {
        map.insert("radius".into(), metric(m));
    }
    if let Some(b) = p.padding {
        map.insert("padding".into(), bounds_value(b));
    }
    if let Some(f) = &p.font {
        map.insert("font".into(), Value::String(f.clone()));
    }
    if let Some(o) = p.opacity {
        map.insert("opacity".into(), number(o as f64));
    }
    if let Some(m) = &p.transition {
        map.insert("transition".into(), metric(m));
    }
    if let Some(t) = &p.shadow {
        map.insert("shadow".into(), Value::String(t.clone()));
    }
    if let Some(m) = &p.material {
        map.insert("material".into(), Value::String(m.clone()));
    }
    if let Some(t) = &p.text_case {
        map.insert("text_case".into(), Value::String(t.clone()));
    }
    if let Some(l) = p.letter_spacing {
        map.insert("letter_spacing".into(), number(l as f64));
    }
    if let Some(g) = p.gap {
        map.insert("gap".into(), number(g as f64));
    }
    if let Some(b) = p.border_sides {
        map.insert("border_sides".into(), bounds_value(b));
    }
    if let Some(b) = &p.border_style {
        map.insert("border_style".into(), Value::String(b.clone()));
    }
    if let Some(l) = p.line_height {
        map.insert("line_height".into(), number(l as f64));
    }
    if let Some(w) = p.font_weight {
        if let Some((name, _)) = WEIGHTS.iter().find(|(_, x)| *x == w) {
            map.insert("font_weight".into(), Value::String((*name).to_string()));
        }
    }
    if let Some(s) = p.font_size {
        map.insert("font_size".into(), number(s as f64));
    }
    if let Some(i) = p.italic {
        map.insert("italic".into(), Value::Bool(i));
    }
    map
}
