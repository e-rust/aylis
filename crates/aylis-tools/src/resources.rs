//! Monta a pasta `resources/` da IDE do Aylis: ícones e fontes livres para usar offline.
//!
//! ```text
//! aylis-tool resources            # baixa e monta tudo
//! aylis-tool resources --icons    # só os ícones
//! aylis-tool resources --fonts    # só as fontes
//! aylis-tool resources --lottie   # só as animações
//! ```
//!
//! O que vai para `resources/` (e o que o app pode embutir):
//!
//! - `icons/*.txt`: pacotes de ícones (nome → caminho SVG), com a licença no cabeçalho:
//!   - Material Symbols (Google, Apache-2.0): `material`, `material-rounded`, `material-sharp`
//!     (cada ícone também com `-fill`, o preenchido);
//!   - Fluent System Icons (Microsoft, MIT): `fluent` (regular e `-fill`).
//! - `fonts/<família>/`: os `.ttf` e a licença (OFL.txt, LICENSE.txt) de uma seleção do Google
//!   Fonts (google/fonts no GitHub). Quase todas são SIL OFL 1.1: podem ir junto com o app,
//!   desde que a licença vá junto e a fonte não seja vendida sozinha.
//! - `lottie/noto/*.json`: emojis animados do Google (Animated Noto Emoji, CC BY 4.0: pode ir
//!   no app, com o crédito "Animated Noto Emoji © Google, CC BY 4.0"); `catalog.json` tem
//!   nome, título e código.
//!
//! O app só leva o que o projeto usa: os ícones citados, as fontes escolhidas e as animações
//! citadas.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::Path;

use serde_json::{Value, json};

const MATERIAL: (&str, &str) = ("@material-symbols/svg-400", "0.48.1");
const FLUENT: (&str, &str) = ("@fluentui/svg-icons", "1.1.344");

/// Fontes populares do Google Fonts (pasta em google/fonts: licença/nome).
const FONTS: [&str; 32] = [
    "ofl/inter", "ofl/roboto", "ofl/robotomono", "ofl/robotoflex", "ofl/opensans", "ofl/lato", "ofl/montserrat", "ofl/poppins", "ofl/nunito",
    "ofl/notosans", "ofl/notoserif", "ofl/sourcesans3", "ofl/raleway", "ofl/worksans", "ofl/rubik", "ofl/manrope", "ofl/dmsans",
    "ofl/ibmplexsans", "ofl/ibmplexmono", "ufl/ubuntu", "ofl/merriweather", "ofl/playfairdisplay", "ofl/jetbrainsmono", "ofl/firacode",
    "ofl/firasans", "ofl/lora", "ofl/outfit", "ofl/quicksand", "ofl/oswald", "ofl/ptsans", "ofl/mulish", "ofl/notocoloremoji",
];

const FLUENT_LICENSE: &str = "MIT License

Copyright (c) 2020 Microsoft Corporation

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.";

/// Animated Noto Emoji: (código no endereço do Google, nome, título) — a lista do conversa-web.
const NOTO: [(&str, &str, &str); 55] = [
    ("1f602", "chorando-de-rir", "Chorando de rir"), ("1f923", "rolando-de-rir", "Rolando de rir"), ("1f60a", "sorriso", "Sorriso"),
    ("1f605", "ufa", "Ufa"), ("1f609", "piscadinha", "Piscadinha"), ("1f60d", "apaixonado", "Apaixonado"),
    ("1f970", "cheio-de-amor", "Cheio de amor"), ("1f618", "beijinho", "Beijinho"), ("1f60e", "legal", "Legal"),
    ("1f929", "deslumbrado", "Deslumbrado"), ("1f973", "festejando", "Festejando"), ("1f917", "abraco", "Abraço"),
    ("1f92d", "ops", "Ops"), ("1f914", "pensando", "Pensando"), ("1f928", "desconfiado", "Desconfiado"),
    ("1f644", "revirando-os-olhos", "Revirando os olhos"), ("1f643", "de-cabeca-para-baixo", "De cabeça para baixo"),
    ("1f633", "envergonhado", "Envergonhado"), ("1f97a", "por-favorzinho", "Por favorzinho"), ("1f622", "triste", "Triste"),
    ("1f62d", "chorando", "Chorando"), ("1f621", "bravo", "Bravo"), ("1f631", "assustado", "Assustado"),
    ("1f92f", "cabeca-explodindo", "Cabeça explodindo"), ("1f971", "bocejando", "Bocejando"), ("1f634", "dormindo", "Dormindo"),
    ("1f912", "doente", "Doente"), ("1f480", "morri", "Morri"), ("1f921", "palhaco", "Palhaço"), ("1f47b", "fantasma", "Fantasma"),
    ("1f44d", "joinha", "Joinha"), ("1f44e", "negativo", "Negativo"), ("1f44f", "palmas", "Palmas"), ("1f64c", "aleluia", "Aleluia"),
    ("1f64f", "por-favor", "Por favor"), ("1f44b", "tchau", "Tchau"), ("1f4aa", "forca", "Força"), ("1f91d", "combinado", "Combinado"),
    ("270c_fe0f", "paz", "Paz"), ("1f440", "de-olho", "De olho"), ("2764_fe0f", "coracao", "Coração"),
    ("1f494", "coracao-partido", "Coração partido"), ("1f525", "fogo", "Fogo"), ("1f4af", "cem", "Cem"), ("2728", "brilhos", "Brilhos"),
    ("1f389", "festa", "Festa"), ("1f38a", "confete", "Confete"), ("1f381", "presente", "Presente"), ("1f382", "bolo", "Bolo"),
    ("1f680", "foguete", "Foguete"), ("1f4a1", "ideia", "Ideia"), ("26a1", "raio", "Raio"), ("1f308", "arco-iris", "Arco-íris"),
    ("2615", "cafe", "Café"), ("1f37b", "brinde", "Brinde"),
];

const NOTO_CREDIT: &str = "Animated Noto Emoji © Google, CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)";

#[cfg(target_arch = "wasm32")]
fn fetch(url: &str) -> Result<Vec<u8>, String> {
    Err(format!("{url}: baixar não existe no navegador"))
}

/// Baixa `url` inteiro (até 512 MB).
#[cfg(not(target_arch = "wasm32"))]
fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let response = ureq::get(url).header("User-Agent", "aylis-tool").call().map_err(|e| format!("falhou {url}: {e}"))?;
    let mut bytes = Vec::new();
    response.into_body().into_with_config().limit(512 * 1024 * 1024).reader().read_to_end(&mut bytes).map_err(|e| format!("{url}: {e}"))?;
    Ok(bytes)
}

/// Os arquivos de um pacote do npm: (caminho dentro do pacote, bytes).
fn npm_files(name: &str, version: &str, wanted: impl Fn(&str) -> bool) -> Result<Vec<(String, Vec<u8>)>, String> {
    let short = name.rsplit('/').next().unwrap_or(name);
    let tgz = fetch(&format!("https://registry.npmjs.org/{name}/-/{short}-{version}.tgz"))?;
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(&tgz[..]));
    let mut out = Vec::new();
    for entry in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path().map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
        if wanted(&path) {
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
            out.push((path, bytes));
        }
    }
    Ok(out)
}

/// Os `d="..."` dos `<path>` de um SVG, juntos.
fn path_data(svg: &str) -> String {
    let mut out = Vec::new();
    let mut rest = svg;
    // `d="` logo depois de um espaço (não o fim de outro atributo, como `id="`).
    while let Some(at) = rest.match_indices("d=\"").map(|(i, _)| i).find(|&i| i > 0 && rest.as_bytes()[i - 1].is_ascii_whitespace()) {
        let tail = &rest[at + 3..];
        let Some(end) = tail.find('"') else { break };
        out.push(&tail[..end]);
        rest = &tail[end..];
    }
    out.join(" ")
}

fn write_pack(root: &Path, file: &str, title: &str, license_name: &str, license_text: &str, view: &str, icons: &BTreeMap<String, String>) -> Result<(), String> {
    let path = root.join("icons").join(file);
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut text = format!("# aylis-icons 1\n# title: {title}\n# license: {license_name}\n# view: {view}\n");
    for line in license_text.trim().lines() {
        text.push_str(&format!("#| {line}\n"));
    }
    for (name, d) in icons {
        text.push_str(&format!("{name}\t{d}\n"));
    }
    std::fs::write(&path, &text).map_err(|e| format!("{}: {e}", path.display()))?;
    println!("  icons/{file}: {} ícones, {} KB", icons.len(), text.len() / 1024);
    Ok(())
}

/// Os pacotes de ícones (Material Symbols e Fluent).
pub fn icons(root: &Path) -> Result<(), String> {
    println!("Material Symbols (Apache-2.0)…");
    let styles = [("outlined", "material", "Material Symbols (Outlined)"), ("rounded", "material-rounded", "Material Symbols (Rounded)"), ("sharp", "material-sharp", "Material Symbols (Sharp)")];
    let files = npm_files(MATERIAL.0, MATERIAL.1, |p| p == "package/LICENSE" || (p.ends_with(".svg") && p.split('/').count() == 3))?;
    let license = files.iter().find(|(p, _)| p == "package/LICENSE").map(|(_, b)| String::from_utf8_lossy(b).to_string()).unwrap_or_default();
    let mut packs: BTreeMap<&str, BTreeMap<String, String>> = BTreeMap::new();
    for (p, bytes) in &files {
        let parts: Vec<&str> = p.split('/').collect();
        let Some(&(_, pack, _)) = styles.iter().find(|(s, _, _)| parts.len() == 3 && *s == parts[1]) else { continue };
        let d = path_data(&String::from_utf8_lossy(bytes));
        if !d.is_empty() {
            packs.entry(pack).or_default().insert(parts[2].trim_end_matches(".svg").to_string(), d);
        }
    }
    for (_, pack, title) in styles {
        write_pack(root, &format!("{pack}.txt"), title, "Apache-2.0 (Google)", &license, "0 -960 960 960", packs.get(pack).unwrap_or(&BTreeMap::new()))?;
    }

    println!("Fluent System Icons (MIT)…");
    let files = npm_files(FLUENT.0, FLUENT.1, |p| p.ends_with("_24_regular.svg") || p.ends_with("_24_filled.svg"))?;
    let mut items = BTreeMap::new();
    for (p, bytes) in &files {
        let name = p.rsplit('/').next().unwrap_or(p);
        let (base, fill) = match (name.strip_suffix("_24_regular.svg"), name.strip_suffix("_24_filled.svg")) {
            (Some(b), _) => (b, false),
            (_, Some(b)) => (b, true),
            _ => continue,
        };
        let d = path_data(&String::from_utf8_lossy(bytes));
        if !d.is_empty() {
            items.insert(format!("{}{}", base.replace('_', "-"), if fill { "-fill" } else { "" }), d);
        }
    }
    write_pack(root, "fluent.txt", "Fluent System Icons", "MIT (Microsoft)", FLUENT_LICENSE, "0 0 24 24", &items)
}

/// Um campo `nome: "valor"` do METADATA.pb.
fn metadata_field(text: &str, field: &str) -> Option<String> {
    text.lines().find_map(|l| l.strip_prefix(&format!("{field}: \""))?.strip_suffix('"').map(str::to_string))
}

/// As fontes do Google Fonts e o `fonts/catalog.json`.
pub fn fonts(root: &Path) -> Result<(), String> {
    println!("Google Fonts…");
    let mut total = 0usize;
    let mut catalog = Vec::new();
    for folder in FONTS {
        let (license_dir, name) = folder.split_once('/').unwrap_or(("ofl", folder));
        let entries: Value = match fetch(&format!("https://api.github.com/repos/google/fonts/contents/{folder}")).and_then(|b| serde_json::from_slice(&b).map_err(|e| e.to_string())) {
            Ok(v) => v,
            Err(e) => {
                println!("  {folder}: {e}");
                continue;
            }
        };
        let family_dir = root.join("fonts").join(name);
        if family_dir.is_dir() {
            // As fontes desta família são refeitas inteiras (são baixadas, não do usuário).
            std::fs::remove_dir_all(&family_dir).map_err(|e| e.to_string())?;
        }
        std::fs::create_dir_all(&family_dir).map_err(|e| e.to_string())?;
        let mut files = Vec::new();
        for e in entries.as_array().into_iter().flatten() {
            let file = e["name"].as_str().unwrap_or_default();
            let wanted = file.ends_with(".ttf") || file.ends_with(".otf") || matches!(file, "OFL.txt" | "LICENSE.txt" | "UFL.txt" | "METADATA.pb");
            if e["type"] == "file" && wanted {
                let bytes = fetch(e["download_url"].as_str().unwrap_or_default())?;
                total += bytes.len();
                std::fs::write(family_dir.join(file), &bytes).map_err(|e| e.to_string())?;
                files.push(file.to_string());
            }
        }
        let meta = std::fs::read_to_string(family_dir.join("METADATA.pb")).unwrap_or_default();
        let family = metadata_field(&meta, "name").unwrap_or_else(|| name.to_string());
        let category = metadata_field(&meta, "category").unwrap_or_default();
        let license = match license_dir {
            "apache" => "Apache-2.0",
            "ufl" => "Ubuntu Font Licence 1.0",
            _ => "SIL OFL 1.1",
        };
        let fonts: Vec<&String> = files.iter().filter(|f| f.ends_with(".ttf") || f.ends_with(".otf")).collect();
        println!("  {family}: {}", fonts.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        catalog.push(json!({ "family": family, "folder": name, "license": license, "category": category, "files": fonts }));
    }
    std::fs::create_dir_all(root.join("fonts")).map_err(|e| e.to_string())?;
    std::fs::write(root.join("fonts").join("catalog.json"), serde_json::to_string_pretty(&catalog).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    println!("  total: {} MB", total / (1024 * 1024));
    Ok(())
}

/// Os emojis animados (Animated Noto Emoji).
pub fn lottie(root: &Path) -> Result<(), String> {
    let out = root.join("lottie").join("noto");
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut catalog = Vec::new();
    let mut total = 0;
    for (code, name, title) in NOTO {
        let data: Value = match fetch(&format!("https://fonts.gstatic.com/s/e/notoemoji/latest/{code}/lottie.json")).and_then(|b| serde_json::from_slice(&b).map_err(|e| e.to_string())) {
            Ok(v) => v,
            Err(e) => {
                // O Google pode não ter a animação deste.
                println!("  ⚠ {title} ({code}): {e}");
                continue;
            }
        };
        if data.get("v").is_none() || !data["layers"].is_array() {
            println!("  ⚠ {title} ({code}) não veio em Lottie");
            continue;
        }
        let text = serde_json::to_string(&data).map_err(|e| e.to_string())?;
        total += text.len();
        std::fs::write(out.join(format!("{name}.json")), text).map_err(|e| e.to_string())?;
        catalog.push(json!({ "name": name, "title": title, "code": code }));
    }
    let doc = json!({ "title": "Noto (emojis animados do Google)", "license": NOTO_CREDIT, "animations": catalog });
    std::fs::write(out.join("catalog.json"), serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    std::fs::write(out.join("LICENSE.txt"), format!("{NOTO_CREDIT}\n")).map_err(|e| e.to_string())?;
    println!("  {} de {} animações, {} KB", catalog.len(), NOTO.len(), total / 1024);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_paths_and_metadata_fields() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d="M1 2L3 4"/><path fill="red" d="M5 6Z"/></svg>"#;
        assert_eq!(path_data(svg), "M1 2L3 4 M5 6Z");
        let meta = "name: \"Roboto Mono\"\ndesigner: \"x\"\ncategory: \"MONOSPACE\"\n";
        assert_eq!(metadata_field(meta, "name").as_deref(), Some("Roboto Mono"));
        assert_eq!(metadata_field(meta, "category").as_deref(), Some("MONOSPACE"));
        assert_eq!(metadata_field(meta, "license"), None);
    }
}
