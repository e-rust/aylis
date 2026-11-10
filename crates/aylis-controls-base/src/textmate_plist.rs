//! O plist em XML da Apple (o `.tmLanguage` e o `.tmTheme` antigos) lido como JSON: `dict`
//! vira objeto, `array` vira lista, `string` texto, `integer`/`real` número e `true`/`false`.

use serde_json::{Map, Value};

/// Lê o plist; `Err` com o motivo se não for um.
pub fn parse(text: &str) -> Result<Value, String> {
    let mut p = Parser { s: text, i: 0 };
    loop {
        let tag = p.next_tag().ok_or("plist sem <plist>")?;
        if tag.name == "plist" && !tag.close {
            break;
        }
    }
    let value = p.value()?.ok_or("plist vazio")?;
    Ok(value)
}

struct Tag<'a> {
    name: &'a str,
    close: bool,
    /// `<true/>`.
    empty: bool,
}

struct Parser<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    /// A próxima marca (pula declarações, comentários e o DOCTYPE).
    fn next_tag(&mut self) -> Option<Tag<'a>> {
        loop {
            let start = self.i + self.s[self.i..].find('<')?;
            let rest = &self.s[start..];
            if rest.starts_with("<!--") {
                self.i = start + rest.find("-->")? + 3;
                continue;
            }
            let end = start + rest.find('>')?;
            self.i = end + 1;
            let inner = &self.s[start + 1..end];
            if inner.starts_with('?') || inner.starts_with('!') {
                continue;
            }
            let close = inner.starts_with('/');
            let empty = inner.ends_with('/');
            let body = inner.trim_start_matches('/').trim_end_matches('/');
            let name = body.split_whitespace().next().unwrap_or("");
            return Some(Tag { name, close, empty });
        }
    }

    /// O texto até `</name>`.
    fn text_until(&mut self, name: &str) -> Result<String, String> {
        let close = format!("</{name}>");
        let end = self.s[self.i..].find(&close).ok_or_else(|| format!("falta {close}"))?;
        let raw = &self.s[self.i..self.i + end];
        self.i += end + close.len();
        Ok(unescape(raw))
    }

    /// Um valor; `None` se veio o fecho de um `dict`/`array`.
    fn value(&mut self) -> Result<Option<Value>, String> {
        let Some(tag) = self.next_tag() else { return Ok(None) };
        if tag.close {
            return Ok(None);
        }
        let value = match tag.name {
            "dict" if tag.empty => Value::Object(Map::new()),
            "dict" => {
                let mut map = Map::new();
                loop {
                    let Some(key) = self.next_tag() else { return Err("dict sem fim".into()) };
                    if key.close {
                        break;
                    }
                    if key.name != "key" {
                        return Err(format!("esperava <key>, veio <{}>", key.name));
                    }
                    let k = self.text_until("key")?;
                    let v = self.value()?.ok_or("chave sem valor")?;
                    map.insert(k, v);
                }
                Value::Object(map)
            }
            "array" if tag.empty => Value::Array(Vec::new()),
            "array" => {
                let mut list = Vec::new();
                while let Some(v) = self.value()? {
                    list.push(v);
                }
                Value::Array(list)
            }
            "string" if tag.empty => Value::String(String::new()),
            "string" => Value::String(self.text_until("string")?),
            "integer" | "real" => {
                let t = self.text_until(tag.name)?;
                t.trim().parse::<f64>().ok().and_then(|n| serde_json::Number::from_f64(n)).map_or(Value::Null, Value::Number)
            }
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            other => return Err(format!("marca desconhecida <{other}>")),
        };
        Ok(Some(value))
    }
}

fn unescape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(p) = rest.find('&') {
        out.push_str(&rest[..p]);
        rest = &rest[p..];
        let Some(end) = rest.find(';') else { break };
        let entity = &rest[1..end];
        let ch = match entity {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => entity
                .strip_prefix("#x")
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| entity.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        match ch {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_dicts_arrays_and_entities() {
        let xml = r#"<?xml version="1.0"?><!DOCTYPE plist><plist version="1.0"><dict>
            <key>name</key><string>Pascal</string>
            <key>patterns</key><array><dict><key>match</key><string>a&lt;b &amp; c</string></dict></array>
            <key>empty</key><string/><key>yes</key><true/></dict></plist>"#;
        let v = super::parse(xml).unwrap();
        assert_eq!(v["name"], "Pascal");
        assert_eq!(v["patterns"][0]["match"], "a<b & c");
        assert_eq!(v["empty"], "");
        assert_eq!(v["yes"], true);
    }
}
