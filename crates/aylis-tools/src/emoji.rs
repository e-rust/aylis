//! Gera `resources/emoji/emoji.tsv` (o seletor de emoji do aylis) a partir das listas
//! oficiais, numa pasta com:
//!
//! - `emoji-test.txt`: <https://unicode.org/Public/emoji/latest/emoji-test.txt>;
//! - `pt.json`: cldr-json, `cldr-annotations-full/annotations/pt/annotations.json`;
//! - `pt-derived.json`: cldr-json, `cldr-annotations-derived-full/annotationsDerived/pt/annotations.json`.
//!
//! ```text
//! aylis-tool emoji <pasta com os três arquivos>
//! ```
//!
//! Só os emojis completos ("fully-qualified") até a versão 15.1 (as fontes de emoji dos
//! sistemas já desenham), sem as variações de tom de pele (o seletor aplica o tom). Cada linha:
//! grupo, emoji, nome em português, palavras de busca (português e inglês).

use std::collections::HashMap;
use std::path::Path;

use serde_json::Value;

const MAX_VERSION: f32 = 15.1;
const GROUPS: [(&str, &str); 9] = [
    ("Smileys & Emotion", "Carinhas e emoções"),
    ("People & Body", "Pessoas"),
    ("Animals & Nature", "Animais e natureza"),
    ("Food & Drink", "Comidas e bebidas"),
    ("Travel & Places", "Viagens e lugares"),
    ("Activities", "Atividades"),
    ("Objects", "Objetos"),
    ("Symbols", "Símbolos"),
    ("Flags", "Bandeiras"),
];

#[derive(Default)]
struct Names {
    tts: String,
    words: Vec<String>,
}

/// Lê a pasta e grava `dest`. Devolve quantos emojis entraram.
pub fn generate(folder: &Path, dest: &Path) -> Result<usize, String> {
    let read = |name: &str| std::fs::read_to_string(folder.join(name)).map_err(|e| format!("{name}: {e}"));
    let mut pt: HashMap<String, Names> = HashMap::new();
    for name in ["pt.json", "pt-derived.json"] {
        let data: Value = serde_json::from_str(&read(name)?).map_err(|e| format!("{name}: {e}"))?;
        let root = data.get("annotations").or_else(|| data.get("annotationsDerived")).ok_or(format!("{name}: sem anotações"))?;
        for (emoji, info) in root["annotations"].as_object().ok_or(format!("{name}: sem anotações"))? {
            let entry = pt.entry(emoji.clone()).or_default();
            if let Some(tts) = info["tts"].get(0).and_then(Value::as_str) {
                entry.tts = tts.to_string();
            }
            entry.words.extend(info["default"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string));
        }
    }
    let out = lines(&read("emoji-test.txt")?, &pt);
    if let Some(dir) = dest.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let header = format!("# grupos: {}\n", GROUPS.map(|(_, pt)| pt).join(";"));
    std::fs::write(dest, format!("{header}{}\n", out.join("\n"))).map_err(|e| format!("{}: {e}", dest.display()))?;
    Ok(out.len())
}

fn lines(emoji_test: &str, pt: &HashMap<String, Names>) -> Vec<String> {
    let mut group: Option<usize> = None;
    let mut out = Vec::new();
    for line in emoji_test.lines() {
        if let Some(name) = line.strip_prefix("# group:") {
            group = GROUPS.iter().position(|(en, _)| *en == name.trim());
            continue;
        }
        let Some(g) = group else { continue };
        let Some((codes, rest)) = line.split_once(';') else { continue };
        let Some((status, comment)) = rest.split_once('#') else { continue };
        if status.trim() != "fully-qualified" {
            continue;
        }
        // "# 😀 E1.0 grinning face": o emoji, a versão e o nome em inglês.
        let comment = comment.trim();
        let Some((emoji, rest)) = comment.split_once(char::is_whitespace) else { continue };
        let Some((version, english)) = rest.trim_start().split_once(char::is_whitespace) else { continue };
        let Some(version) = version.strip_prefix('E').and_then(|v| v.parse::<f32>().ok()) else { continue };
        if version > MAX_VERSION + 1e-4 {
            continue;
        }
        let skin = codes.split_whitespace().filter_map(|c| u32::from_str_radix(c, 16).ok()).any(|c| (0x1F3FB..0x1F400).contains(&c));
        if skin {
            continue;
        }
        // O CLDR guarda sem o seletor de variação (FE0F).
        let empty = Names::default();
        let info = pt.get(emoji).or_else(|| pt.get(&emoji.replace('\u{FE0F}', ""))).unwrap_or(&empty);
        let english = english.trim();
        let name = if info.tts.is_empty() { english.to_string() } else { info.tts.clone() };
        let mut words: Vec<String> = Vec::new();
        for w in info.words.iter().map(|w| w.to_lowercase()).chain(english.to_lowercase().split_whitespace().map(str::to_string)) {
            if !words.contains(&w) {
                words.push(w);
            }
        }
        out.push(format!("{g}\t{emoji}\t{name}\t{}", words.join(" ")));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_unicode_list_with_portuguese_names() {
        let text = "# group: Smileys & Emotion\n1F600 ; fully-qualified # 😀 E1.0 grinning face\n263A FE0F ; fully-qualified # ☺️ E0.6 smiling face\n263A ; unqualified # ☺ E0.6 smiling face\n# group: People & Body\n1F44B 1F3FB ; fully-qualified # 👋🏻 E1.0 waving hand: light skin tone\n1FAE8 ; fully-qualified # 🫨 E15.1 shaking face\n1FAE9 ; fully-qualified # 🫩 E16.0 face with bags\n";
        let mut pt = HashMap::new();
        pt.insert("😀".to_string(), Names { tts: "rosto risonho".into(), words: vec!["Feliz".into(), "riso".into()] });
        pt.insert("☺".to_string(), Names { tts: "rosto sorridente".into(), words: vec![] });
        let out = lines(text, &pt);
        assert_eq!(out[0], "0\t😀\trosto risonho\tfeliz riso grinning face");
        assert_eq!(out[1], "0\t☺️\trosto sorridente\tsmiling face", "o CLDR sem o FE0F");
        assert_eq!(out[2], "1\t🫨\tshaking face\tshaking face", "sem nome em português: o inglês");
        assert_eq!(out.len(), 3, "sem o não qualificado, o tom de pele e a versão 16");
    }
}
