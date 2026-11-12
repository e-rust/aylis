//! Tradução: os textos da tela no idioma do usuário.
//!
//! O texto escrito no designer (no idioma de quem fez o app) é a chave; a tradução vem de
//! um dicionário `{"Salvar": "Save"}` (o `locales/en-US.json` do projeto). Os layouts
//! traduzem ao carregar as propriedades de texto ([`TRANSLATABLE`]); o código traduz com
//! [`Ui::tr`]. Trocar de idioma com o app aberto ([`Ui::set_translations`] +
//! [`Ui::retranslate`]) refaz os textos que vieram do layout.

use std::collections::HashMap;

use crate::component::ComponentId;
use crate::prop::PropValue;
use crate::ui::Ui;

/// Propriedades de texto traduzidas ao carregar um layout. `items`, `labels` e `columns`
/// são listas separadas por `;` (cada item é traduzido).
pub const TRANSLATABLE: [&str; 9] = ["text", "caption", "placeholder", "title", "display_label", "hint", "items", "labels", "display_values"];

/// As listas (itens separados por `;`).
fn is_list(prop: &str) -> bool {
    matches!(prop, "items" | "labels" | "display_values")
}

#[derive(Default)]
pub(crate) struct Translations {
    pub(crate) map: HashMap<String, String>,
    /// Texto original de cada propriedade traduzida (para traduzir de novo).
    pub(crate) sources: HashMap<(ComponentId, String), String>,
    /// Os modelos com valores (`"Form {} criado"` → `"Form {} created"`): as partes fixas e a
    /// tradução. Um texto já montado sem tradução própria é traduzido pelo modelo dele.
    pub(crate) templates: Vec<(Vec<String>, String)>,
}

/// Os valores de `text` nos lugares dos `{}` do modelo (`parts`: os trechos fixos).
fn match_template(parts: &[String], text: &str) -> Option<Vec<String>> {
    let first = parts.first()?;
    let mut rest = text.strip_prefix(first.as_str())?;
    let mut values = Vec::new();
    for (i, part) in parts.iter().enumerate().skip(1) {
        let last = i + 1 == parts.len();
        let at = if last {
            if !rest.ends_with(part.as_str()) {
                return None;
            }
            rest.len() - part.len()
        } else if part.is_empty() {
            return None; // dois `{}` seguidos: não dá para separar
        } else {
            rest.find(part.as_str())?
        };
        let value = &rest[..at];
        if value.is_empty() {
            return None;
        }
        values.push(value.to_string());
        rest = &rest[at + part.len()..];
    }
    rest.is_empty().then_some(values)
}

/// O modelo traduzido com os valores: `{}` na ordem, ou `{0}`, `{1}`... pela posição (para os
/// idiomas que mudam a ordem).
fn fill_template(translated: &str, values: &[String]) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut rest = translated;
    while let Some(at) = rest.find('{') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let close = after.find('}').filter(|c| *c <= 2);
        match close.map(|c| &after[..c]) {
            Some("") => {
                out.push_str(values.get(next).map_or("", String::as_str));
                next += 1;
            }
            Some(n) if n.chars().all(|c| c.is_ascii_digit()) => out.push_str(values.get(n.parse::<usize>().unwrap_or(0)).map_or("", String::as_str)),
            _ => {
                out.push('{');
                rest = after;
                continue;
            }
        }
        rest = &after[close.unwrap_or(0) + 1..];
    }
    out.push_str(rest);
    out
}

/// O dicionário do app (o último do [`Ui::set_translations`]) para quem traduz sem o `Ui`.
type Dictionary = (HashMap<String, String>, Vec<(Vec<String>, String)>);
static APP: std::sync::RwLock<Option<std::sync::Arc<Dictionary>>> = std::sync::RwLock::new(None);

/// A busca do [`Ui::tr`] e do [`tr`]: o dicionário do app, um texto já montado pelo modelo
/// dele e, por fim, as mensagens do próprio aylis.
/// O contexto de um texto ambíguo (`arquivo|Abrir`: o "Abrir" do menu Arquivo, que noutra
/// língua não é o "Abrir" de uma porta): uma palavra minúscula, sem espaço, antes da barra. A
/// chave da tradução é o texto inteiro; sem tradução, aparece só o texto (o relatório do
/// Tabernáculo, O02). `Ana Souza|Vendas` (as colunas de uma lista) não é contexto.
pub fn split_context(text: &str) -> Option<(&str, &str)> {
    let (context, rest) = text.split_once('|')?;
    let word = !context.is_empty() && context.len() <= 32 && context.chars().all(|c| c.is_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    (word && !rest.is_empty()).then_some((context, rest))
}

fn lookup(map: &HashMap<String, String>, templates: &[(Vec<String>, String)], text: &str) -> String {
    lookup_in(map, templates, text, true)
}

/// O [`lookup`]; `context`: o texto pode ter contexto (`arquivo|Abrir`). Os itens de uma lista
/// não têm (a barra separa as colunas).
fn lookup_in(map: &HashMap<String, String>, templates: &[(Vec<String>, String)], text: &str, context: bool) -> String {
    if let Some(t) = map.get(text).filter(|t| !t.is_empty()) {
        return t.clone();
    }
    // Com contexto e sem tradução própria: a do texto sem o contexto (ou ele mesmo).
    if let Some((_, plain)) = split_context(text).filter(|_| context) {
        return lookup_in(map, templates, plain, false);
    }
    // Uma mensagem já montada (`"Form Cadastro criado"`): pelo modelo dela, com os
    // valores no lugar (e traduzidos, se forem textos do catálogo).
    for (parts, translated) in templates {
        if let Some(values) = match_template(parts, text) {
            let values: Vec<String> = values.iter().map(|v| map.get(v).filter(|t| !t.is_empty()).cloned().unwrap_or_else(|| v.clone())).collect();
            return fill_template(translated, &values);
        }
    }
    crate::messages::t(text)
}

/// Os modelos com valores do dicionário (`"Form {} criado"`): as partes fixas e a tradução,
/// os mais longos primeiro (os mais específicos).
fn templates_of(map: &HashMap<String, String>) -> Vec<(Vec<String>, String)> {
    let mut templates: Vec<(Vec<String>, String)> = map
        .iter()
        .filter(|(k, v)| k.contains("{}") && !v.is_empty() && k.replace("{}", "").trim().chars().any(char::is_alphabetic))
        .map(|(k, v)| (k.split("{}").map(str::to_string).collect(), v.clone()))
        .collect();
    templates.sort_by_key(|(parts, _): &(Vec<String>, String)| std::cmp::Reverse(parts.iter().map(String::len).sum::<usize>()));
    templates
}

/// Troca o dicionário do [`tr`] sem o `Ui` (vazio: os textos originais): o app que traduz
/// antes de a janela existir (a bandeja, o log da largada). O [`Ui::set_translations`] faz
/// isto e mais os textos da tela.
pub fn set_dictionary(map: HashMap<String, String>) {
    let templates = templates_of(&map);
    if let Ok(mut app) = APP.write() {
        *app = Some(std::sync::Arc::new((map, templates)));
    }
}

/// O texto no idioma do app, de qualquer lugar e de qualquer thread: o mesmo dicionário do
/// [`Ui::tr`] (`locales/<idioma>.json`), para o que nasce sem o `Ui` à mão — o menu da
/// bandeja, uma notificação, uma mensagem montada numa thread de trabalho. Antes do app
/// carregar as traduções, só as mensagens do aylis.
pub fn tr(text: &str) -> String {
    let app = APP.read().ok().and_then(|a| a.clone());
    match app {
        Some(dict) => lookup(&dict.0, &dict.1, text),
        None => crate::messages::t(text),
    }
}

/// [`tr`] com valores: `{0}`, `{1}`... no texto (`tr_with("{0} de {1}", &[&atual, &total])`).
/// Numerados, a tradução pode mudar a ordem (`"{1}: {0}"`).
pub fn tr_with(text: &str, values: &[&dyn std::fmt::Display]) -> String {
    let values: Vec<String> = values.iter().map(|v| v.to_string()).collect();
    fill_template(&tr(text), &values)
}

impl Ui {
    /// O texto no idioma atual (ou o próprio texto, sem tradução).
    /// Sem tradução no dicionário do app, vale a do aylis (as mensagens do próprio aylis:
    /// [`crate::messages`]); o app pode trocar qualquer uma no `locales/` dele.
    pub fn tr(&self, text: &str) -> String {
        lookup(&self.translations.map, &self.translations.templates, text)
    }

    /// Troca o dicionário de traduções (vazio = textos originais). Para refazer a tela
    /// aberta, chame [`retranslate`](Self::retranslate) em seguida.
    pub fn set_translations(&mut self, map: HashMap<String, String>) {
        let templates = templates_of(&map);
        // O mesmo dicionário para o [`tr`] livre (a bandeja, uma notificação, uma thread).
        if let Ok(mut app) = APP.write() {
            *app = Some(std::sync::Arc::new((map.clone(), templates.clone())));
        }
        self.translations.templates = templates;
        self.translations.map = map;
    }

    pub fn has_translations(&self) -> bool {
        !self.translations.map.is_empty()
    }

    /// Traduz `text` para a propriedade `prop` (listas item a item).
    pub fn translate_prop(&self, prop: &str, text: &str) -> String {
        if is_list(prop) {
            // Item a item, sem contexto (as colunas de uma lista usam a barra).
            text.split(';').map(|item| lookup_in(&self.translations.map, &self.translations.templates, item, false)).collect::<Vec<_>>().join(";")
        } else {
            self.tr(text)
        }
    }

    /// Grava uma propriedade de texto traduzida, lembrando o original (para
    /// [`retranslate`](Self::retranslate)). Usado ao carregar layouts.
    pub fn set_translated(&mut self, id: ComponentId, prop: &str, original: &str) -> crate::Result<()> {
        // Sem tradução (`translate` desligado), ou no designer (o texto como está escrito, com o
        // contexto que a O02 permite): o original.
        if self.design_mode || self.cb(id).is_some_and(|cb| !cb.translatable()) {
            return self.set_prop_quiet(id, prop, PropValue::String(original.to_string()));
        }
        let translated = self.translate_prop(prop, original);
        self.set_prop_quiet(id, prop, PropValue::String(translated))?;
        self.translations.sources.insert((id, prop.to_string()), original.to_string());
        Ok(())
    }

    /// Aplica de novo as traduções nos textos que vieram dos layouts (depois de trocar o
    /// idioma). Textos mudados pelo código depois de carregados ficam como estão.
    pub fn retranslate(&mut self) {
        let sources: Vec<((ComponentId, String), String)> = self.translations.sources.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for ((id, prop), original) in sources {
            if !self.contains(id) {
                self.translations.sources.remove(&(id, prop));
                continue;
            }
            let translated = self.translate_prop(&prop, &original);
            let _ = self.set_prop_quiet(id, &prop, PropValue::String(translated));
            // `set_prop` esquece a origem (o código mudou o texto): lembra de novo.
            self.translations.sources.insert((id, prop), original);
        }
        self.invalidate();
    }

    /// Traduz os textos dos componentes que ainda não passaram pela tradução: telas
    /// montadas pelo código (a IDE do Aylis), ou textos que o código trocou depois. Chame
    /// depois de montar ou refazer a tela. `skip_text(classe)`: as classes cujo `text` é dado
    /// do usuário (campos de edição), que ficam como estão.
    pub fn translate_untranslated(&mut self, skip_text: impl Fn(&str) -> bool) {
        if self.translations.map.is_empty() {
            return;
        }
        let ids: Vec<ComponentId> = self.components().collect();
        for id in ids {
            let class = self.class_name(id).unwrap_or_default();
            if self.cb(id).is_some_and(|cb| !cb.translatable()) {
                continue;
            }
            for prop in TRANSLATABLE {
                if (prop == "text" && skip_text(class)) || self.translations.sources.contains_key(&(id, prop.to_string())) {
                    continue;
                }
                let Ok(PropValue::String(original)) = self.get_prop(id, prop) else { continue };
                if original.is_empty() {
                    continue;
                }
                let translated = self.translate_prop(prop, &original);
                if translated != original {
                    let _ = self.set_translated(id, prop, &original);
                }
            }
        }
    }

    /// O código mudou a propriedade: ela não é mais o texto do layout.
    pub(crate) fn forget_translation_source(&mut self, id: ComponentId, prop: &str) {
        if !self.translations.sources.is_empty() {
            self.translations.sources.remove(&(id, prop.to_string()));
        }
    }
}
