//! Traduções do projeto: `locales/<idioma>.json` com `{"texto original": "tradução"}`.
//!
//! O idioma do aplicativo é o `locale` do projeto (`en-US`); o arquivo do idioma, se
//! existir, traduz os textos dos forms ao carregar e os `ui.tr("...")` do código.
//! [`update_locale`] junta no arquivo os textos novos (dos forms e de `tr("...")` no
//! código Rust ou Python) sem perder as traduções feitas.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use aylis_core::Ui;
use aylis_core::platform::LocaleInfo;
use serde_json::Value;

use crate::{Error, Project};

/// Caminho (relativo ao projeto) do arquivo de um idioma.
pub fn locale_file(language: &str) -> String {
    format!("locales/{language}.json")
}

/// Lê um arquivo de traduções (vazio se não existir ou for inválido).
pub fn parse(text: &str) -> HashMap<String, String> {
    serde_json::from_str::<BTreeMap<String, String>>(text.trim_start_matches('\u{feff}'))
        .map(|m| m.into_iter().filter(|(_, v)| !v.is_empty()).collect())
        .unwrap_or_default()
}

/// Troca o idioma do aplicativo aberto: formatos (datas, números), traduções (lidas de
/// `locales/<idioma>.json` pelos arquivos do projeto) e os textos dos forms abertos.
pub fn set_language(ui: &mut Ui, language: &str) {
    ui.services_mut().locale = Some(Box::new(locale_info(language)));
    aylis_core::messages::set_language(language);
    // Árabe, hebraico...: a interface espelhada.
    ui.set_right_to_left(aylis_core::is_right_to_left_language(language));
    let text = ui.services().resources.as_ref().and_then(|r| r.read(&locale_file(language)));
    let map = text.map(|t| parse(&String::from_utf8_lossy(&t))).unwrap_or_default();
    ui.set_translations(map);
    ui.retranslate();
}

/// Formatos de um idioma (`en-US`, `en` → inglês dos EUA; `es`, `es-ES`... → espanhol;
/// o resto, português do Brasil).
pub fn locale_info(language: &str) -> LocaleInfo {
    let lower = language.to_lowercase();
    match lower.split(['-', '_']).next().unwrap_or_default() {
        "en" => LocaleInfo::en_us(),
        "es" => LocaleInfo::es(),
        _ => LocaleInfo::pt_br(),
    }
}

/// Os textos para traduzir: as propriedades de texto dos forms e os `tr("...")` do
/// código (`src/`).
pub fn texts(project: &Project) -> Result<BTreeSet<String>, Error> {
    let mut out = BTreeSet::new();
    for form in &project.forms {
        let json = project.read_form(&form.name)?;
        let doc: Value = serde_json::from_str(json.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", form.file))?;
        collect(&doc["root"], &mut out);
    }
    // Os textos do código de todas as linguagens do projeto (e da raiz, no layout antigo).
    let mut code_dirs: Vec<std::path::PathBuf> = project.code_languages().into_iter().map(|l| project.code_dir_of(l).join("src")).collect();
    code_dirs.push(project.dir().join("src"));
    for file in code_dirs.iter().flat_map(|d| source_files(d)) {
        if let Ok(code) = std::fs::read_to_string(&file) {
            code_strings(&code, &mut out);
        }
    }
    // Sem nenhuma letra não se traduz ("-", ".00", "0,25×", "00:00", "0.0.0", "+"; achados na
    // lista da conferência de 08/11).
    out.retain(|t| translatable(t));
    Ok(out)
}

/// O texto tem o que traduzir (ao menos uma letra).
pub fn translatable(text: &str) -> bool {
    text.chars().any(char::is_alphabetic)
}

/// Tira o idioma do projeto (o `locales/<idioma>.json`).
pub fn remove_locale(project: &Project, language: &str) -> Result<(), Error> {
    let path = project.file(&locale_file(language));
    std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))
}

fn collect(node: &Value, out: &mut BTreeSet<String>) {
    if let Some(obj) = node.as_object() {
        for prop in aylis_core::i18n::TRANSLATABLE {
            if let Some(text) = obj.get(prop).and_then(Value::as_str) {
                let items: Vec<&str> = if matches!(prop, "items" | "labels" | "display_values") { text.split(';').collect() } else { vec![text] };
                for item in items {
                    if !item.trim().is_empty() {
                        out.insert(item.to_string());
                    }
                }
            }
        }
        if let Some(children) = obj.get("children").and_then(Value::as_array) {
            children.iter().for_each(|c| collect(c, out));
        }
    }
}

fn source_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(source_files(&p));
        } else if p.extension().is_some_and(|x| x == "rs" || x == "py") {
            out.push(p);
        }
    }
    out
}

/// `tr("texto")` no código (Rust: `ui.tr("...")`, `aylis::tr("...")` e `aylis::tr_with("{0}...",
/// ...)`; Python: `aylis.tr("...")`).
fn code_strings(code: &str, out: &mut BTreeSet<String>) {
    // O `tr_with` tem o mesmo texto que o `tr` (os valores vêm depois).
    let code = code.replace("tr_with(", "tr(");
    let mut rest = code.as_str();
    while let Some(i) = rest.find("tr(") {
        let before = rest[..i].chars().last();
        rest = &rest[i + 3..];
        // `str(`, `attr(`...: só `tr(` sozinho ou depois de `.`.
        if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let trimmed = rest.trim_start();
        let Some(quote) = trimmed.chars().next().filter(|c| *c == '"' || *c == '\'') else { continue };
        let body = &trimmed[1..];
        let mut text = String::new();
        let mut chars = body.chars();
        let mut closed = false;
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(n) = chars.next() {
                        text.push(match n {
                            'n' => '\n',
                            't' => '\t',
                            other => other,
                        });
                    }
                }
                c if c == quote => {
                    closed = true;
                    break;
                }
                c => text.push(c),
            }
        }
        if closed && !text.is_empty() {
            out.insert(text);
        }
    }
}

/// Atualiza `locales/<idioma>.json`: acrescenta os textos novos (vazios, para traduzir),
/// mantém as traduções feitas e marca as que não são mais usadas (ficam, com o valor, numa
/// chave de comentário no fim). Devolve (caminho, quantos novos, quantos sem tradução).
pub fn update_locale(project: &Project, language: &str) -> Result<(PathBuf, usize, usize), Error> {
    let merged = merge_locale(project, language)?;
    save_locale(project, language, &merged.map)?;
    Ok((merged.path, merged.added, merged.missing))
}

/// O `locales/<idioma>.json` com os textos novos juntados, sem gravar.
pub struct MergedLocale {
    pub path: PathBuf,
    pub map: BTreeMap<String, String>,
    /// Como está no disco (vazio, se o arquivo não existe).
    pub saved: BTreeMap<String, String>,
    pub added: usize,
    pub missing: usize,
}

/// O que o [`update_locale`] gravaria, sem gravar: a aba Traduções mostra isso e só grava
/// ao aplicar (passar pela aba não cria arquivo de idioma).
pub fn merge_locale(project: &Project, language: &str) -> Result<MergedLocale, Error> {
    let path = project.file(&locale_file(language));
    let saved: BTreeMap<String, String> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(t.trim_start_matches('\u{feff}')).ok())
        .unwrap_or_default();
    let mut map = saved.clone();
    let texts = texts(project)?;
    let mut added = 0;
    for t in &texts {
        if !map.contains_key(t) {
            map.insert(t.clone(), String::new());
            added += 1;
        }
    }
    let missing = map.iter().filter(|(k, v)| texts.contains(*k) && v.is_empty()).count();
    Ok(MergedLocale { path, map, saved, added, missing })
}

/// Grava as traduções (editadas na IDE).
pub fn save_locale(project: &Project, language: &str, map: &BTreeMap<String, String>) -> Result<PathBuf, Error> {
    let path = project.file(&locale_file(language));
    std::fs::create_dir_all(path.parent().expect("tem pai")).map_err(|e| e.to_string())?;
    let text = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

/// Os idiomas com arquivo em `locales/`.
pub fn languages(project: &Project) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(project.dir().join("locales"))
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| e.path().file_stem().filter(|_| e.path().extension().is_some_and(|x| x == "json")).map(|s| s.to_string_lossy().to_string()))
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}
