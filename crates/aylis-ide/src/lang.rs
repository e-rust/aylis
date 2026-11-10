//! Idioma da IDE: os textos da tela vêm em português (a chave) e são traduzidos pelo
//! catálogo embutido (`resources/locales/<código>.json`). Padrão: o idioma do sistema;
//! Exibir → Idioma troca e fica guardado.

use std::collections::HashMap;

use aylis_core::Ui;

/// Os idiomas da IDE: (código, nome no próprio idioma). Os mais falados do mundo.
pub const LANGUAGES: [(&str, &str); 12] = [
    ("pt", "Português"),
    ("en", "English"),
    ("zh", "中文"),
    ("hi", "हिन्दी"),
    ("es", "Español"),
    ("fr", "Français"),
    ("ar", "العربية"),
    ("bn", "বাংলা"),
    ("ru", "Русский"),
    ("ja", "日本語"),
    ("de", "Deutsch"),
    ("id", "Bahasa Indonesia"),
];

fn catalog(code: &str) -> Option<&'static str> {
    Some(match code {
        "en" => include_str!("../resources/locales/en.json"),
        "zh" => include_str!("../resources/locales/zh.json"),
        "hi" => include_str!("../resources/locales/hi.json"),
        "es" => include_str!("../resources/locales/es.json"),
        "fr" => include_str!("../resources/locales/fr.json"),
        "ar" => include_str!("../resources/locales/ar.json"),
        "bn" => include_str!("../resources/locales/bn.json"),
        "ru" => include_str!("../resources/locales/ru.json"),
        "ja" => include_str!("../resources/locales/ja.json"),
        "de" => include_str!("../resources/locales/de.json"),
        "id" => include_str!("../resources/locales/id.json"),
        _ => return None,
    })
}

/// O idioma do sistema entre os da IDE (`pt` se não houver).
pub fn system_language() -> &'static str {
    let tag = sys_locale::get_locale().unwrap_or_default().to_lowercase();
    let base = tag.split(['-', '_']).next().unwrap_or_default();
    LANGUAGES.iter().map(|(c, _)| *c).find(|c| *c == base).unwrap_or("pt")
}

/// O idioma em uso: o escolhido (`AYLIS_IDE_LANG` vence, para testes) ou o do sistema.
pub fn resolve(chosen: &str) -> &'static str {
    let wanted = std::env::var("AYLIS_IDE_LANG").ok().filter(|v| !v.is_empty()).unwrap_or_else(|| chosen.to_string());
    LANGUAGES.iter().map(|(c, _)| *c).find(|c| *c == wanted).unwrap_or_else(system_language)
}

/// O dicionário de um idioma (vazio para o português), com os nomes das propriedades.
pub fn dictionary(code: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = catalog(code).and_then(|text| serde_json::from_str(text).ok()).unwrap_or_default();
    let table = properties();
    if let Some(col) = table.columns.iter().position(|c| c == code).filter(|_| code != "pt") {
        for (_, labels) in &table.rows {
            if let (Some(pt), Some(t)) = (labels.first(), labels.get(col)) {
                map.entry(pt.clone()).or_insert_with(|| t.clone());
            }
        }
    }
    map
}

/// Os nomes legíveis das propriedades do Object Inspector (`resources/locales/properties.tsv`):
/// por propriedade (e por categoria, com `@` na frente), o texto em cada idioma da IDE.
struct PropertyNames {
    /// Os idiomas das colunas (`pt`, `en`...).
    columns: Vec<String>,
    rows: HashMap<String, Vec<String>>,
}

fn properties() -> &'static PropertyNames {
    static TABLE: std::sync::OnceLock<PropertyNames> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let text = include_str!("../resources/locales/properties.tsv");
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        let columns = lines.next().map(|h| h.split('\t').skip(1).map(str::to_string).collect()).unwrap_or_default();
        let rows = lines
            .filter_map(|l| {
                let mut cells = l.split('\t').map(|c| c.trim().to_string());
                let key = cells.next()?;
                Some((key, cells.collect()))
            })
            .collect();
        PropertyNames { columns, rows }
    })
}

/// O nome legível da propriedade, em português (a chave da tradução); sem nome na tabela,
/// o próprio identificador.
pub fn property_label(name: &str) -> String {
    properties().rows.get(name).and_then(|l| l.first()).cloned().unwrap_or_else(|| name.to_string())
}

/// A propriedade tem nome legível na tabela.
pub fn has_property_label(name: &str) -> bool {
    properties().rows.contains_key(name)
}

/// O nome da categoria do Object Inspector, em português.
pub fn category_label(category: &str) -> String {
    properties().rows.get(&format!("@{category}")).and_then(|l| l.first()).cloned().unwrap_or_else(|| category.to_string())
}

/// O nome de uma vista por largura, em português (o seletor do designer e o inspector).
pub fn view_label(name: &str) -> String {
    match name {
        "" => "Desktop (base)".into(),
        "phone" => "Celular".into(),
        "tablet" => "Tablet".into(),
        other => other.to_string(),
    }
}

/// Troca o idioma da IDE aberta: o dicionário, as mensagens do aylis (diálogos) e os textos
/// já na tela.
pub fn apply(ui: &mut Ui, code: &str) {
    ui.set_translations(dictionary(code));
    // Árabe: a IDE espelhada (o designer fica como o form é).
    ui.set_right_to_left(aylis_core::is_right_to_left_language(code));
    aylis_core::messages::set_language(code);
    ui.retranslate();
    translate(ui);
}

/// Traduz o que foi montado ou mudado desde a última vez (chamado depois de refazer os
/// painéis e ao abrir janelas). Campos de edição ficam como estão.
pub fn translate(ui: &mut Ui) {
    ui.translate_untranslated(|class| class.contains("Edit") || class.contains("Memo") || class.contains("Code") || class == "ComboBox" || class == "SpinBox");
}

/// A etiqueta de idioma do app (`pt-BR`, `es-MX`...) para o idioma `code` da IDE: a do
/// sistema quando é o mesmo idioma (com o país dele), senão a mais comum do idioma. É o
/// idioma sugerido para um projeto novo e nas opções do projeto.
pub fn app_locale(code: &str) -> String {
    let system = sys_locale::get_locale().unwrap_or_default().replace('_', "-");
    let mut parts = system.split('-');
    let (base, region) = (parts.next().unwrap_or_default().to_lowercase(), parts.next().unwrap_or_default().to_uppercase());
    if base == code && region.len() == 2 {
        return format!("{code}-{region}");
    }
    match code {
        "pt" => "pt-BR",
        "en" => "en-US",
        "zh" => "zh-CN",
        "hi" => "hi-IN",
        "es" => "es-ES",
        "fr" => "fr-FR",
        "ar" => "ar-SA",
        "bn" => "bn-BD",
        "ru" => "ru-RU",
        "ja" => "ja-JP",
        "de" => "de-DE",
        "id" => "id-ID",
        other => return other.to_string(),
    }
    .to_string()
}
