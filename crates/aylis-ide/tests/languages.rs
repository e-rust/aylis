//! A IDE em vários idiomas: catálogos completos, troca com a IDE aberta e volta ao português.

use aylis_core::{PropValue, Ui};
use aylis_ide::document::Document;
use aylis_ide::state::shared;
use aylis_ide::{app, lang};

fn has_text(ui: &Ui, text: &str) -> bool {
    ui.components().any(|id| matches!(ui.get_prop(id, "text"), Ok(PropValue::String(s)) if s == text))
}

#[test]
fn every_catalog_has_every_text() {
    let keys: std::collections::BTreeSet<String> = lang::dictionary("en").into_keys().collect();
    assert!(keys.len() > 200);
    for (code, _) in lang::LANGUAGES.iter().skip(1) {
        let d = lang::dictionary(code);
        let mine: std::collections::BTreeSet<String> = d.keys().cloned().collect();
        assert_eq!(mine, keys, "{code}");
        assert!(d.values().all(|v| !v.is_empty()), "{code}: tradução vazia");
    }
}

#[test]
fn switches_the_open_ide_and_back() {
    let mut ide = app::build(shared(Document::new()));
    assert!(has_text(&ide.ui, "Arquivo"));
    lang::apply(&mut ide.ui, "en");
    assert!(has_text(&ide.ui, "File") && !has_text(&ide.ui, "Arquivo"));
    // Painéis refeitos depois da troca também saem traduzidos.
    app::refresh(&mut ide.ui, &ide.state, &ide.ids);
    assert!(has_text(&ide.ui, "Select a component") || has_text(&ide.ui, "PALETTE"));
    lang::apply(&mut ide.ui, "zh");
    assert!(has_text(&ide.ui, "文件"));
    lang::apply(&mut ide.ui, "pt");
    assert!(has_text(&ide.ui, "Arquivo") && !has_text(&ide.ui, "文件"));
}
