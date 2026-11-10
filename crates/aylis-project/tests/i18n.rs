//! Tradução: arquivo por idioma, textos dos forms e do código, troca em execução.

use aylis_core::Ui;
use aylis_project::*;

fn temp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_i18n_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn texts_are_extracted_translated_and_switched_at_runtime() {
    let base = temp("a");
    let project = create(&base, "Loja", Language::Rust).unwrap();
    let form = r#"{ "aylis": 1, "root": { "class": "Form", "name": "frmPrincipal", "caption": "Clientes", "children": [
        { "class": "Button", "name": "btnSalvar", "text": "Salvar" },
        { "class": "Edit", "name": "edtNome", "placeholder": "Nome" },
        { "class": "ComboBox", "name": "cbUf", "items": "Sim;Não" } ] } }"#;
    std::fs::write(project.file("forms/principal.json"), form).unwrap();
    std::fs::write(project.code_dir().join("src/extra.rs"), r#"fn f(ui: &Ui) { ui.tr("Salvo com sucesso"); let s = str("não"); }"#).unwrap();

    // extrair: os textos dos forms e os tr("...") do código, sem o str(...)
    let texts = i18n::texts(&project).unwrap();
    for t in ["Clientes", "Salvar", "Nome", "Sim", "Não", "Salvo com sucesso"] {
        assert!(texts.contains(t), "{t}");
    }
    assert!(!texts.contains("não"));
    // juntar (o que a aba Traduções mostra) não grava: passar pela aba não cria idioma
    let merged = i18n::merge_locale(&project, "en-US").unwrap();
    assert!(merged.added >= 6 && merged.saved.is_empty() && !merged.path.exists());
    assert!(i18n::languages(&project).is_empty());
    let (path, added, missing) = i18n::update_locale(&project, "en-US").unwrap();
    assert!(added >= 6 && missing == added);
    // traduz alguns e atualiza de novo: as traduções ficam
    let mut map: std::collections::BTreeMap<String, String> = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    for (k, v) in [("Clientes", "Customers"), ("Salvar", "Save"), ("Nome", "Name"), ("Sim", "Yes"), ("Não", "No"), ("Salvo com sucesso", "Saved")] {
        map.insert(k.into(), v.into());
    }
    i18n::save_locale(&project, "en-US", &map).unwrap();
    let (_, added, _) = i18n::update_locale(&project, "en-US").unwrap();
    assert_eq!(added, 0);
    assert_eq!(i18n::languages(&project), ["en-US"]);

    // abrir em inglês
    let mut en = project.clone();
    en.locale = "en-US".into();
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let main = install(&mut ui, &en).unwrap();
    let button = ui.find_in(main, "btnSalvar").unwrap();
    assert_eq!(ui.get_prop(main, "caption").unwrap().to_str().unwrap().to_string(), "Customers");
    assert_eq!(ui.get_prop(button, "text").unwrap().to_str().unwrap().to_string(), "Save");
    assert_eq!(ui.get::<aylis_controls::ComboBox>(ui.find_in(main, "cbUf").unwrap()).unwrap().items, "Yes;No");
    assert_eq!(ui.tr("Salvo com sucesso"), "Saved");
    assert_eq!(ui.tr("sem tradução"), "sem tradução");
    // texto mudado pelo código não volta ao trocar de idioma
    let edit = ui.find_in(main, "edtNome").unwrap();
    ui.set_prop(edit, "placeholder", "do código").unwrap();
    // trocar para português com o app aberto (sem arquivo: os originais)
    i18n::set_language(&mut ui, "pt-BR");
    assert_eq!(ui.get_prop(button, "text").unwrap().to_str().unwrap().to_string(), "Salvar");
    assert_eq!(ui.get_prop(edit, "placeholder").unwrap().to_str().unwrap().to_string(), "do código");
    assert_eq!(ui.locale().decimal_separator, ',');
    i18n::set_language(&mut ui, "en-US");
    assert_eq!(ui.get_prop(button, "text").unwrap().to_str().unwrap().to_string(), "Save");
    assert_eq!(ui.locale().decimal_separator, '.');
    let _ = std::fs::remove_dir_all(&base);
}

/// Texto sem letra nenhuma não vai para a lista das traduções.
#[test]
fn texts_without_letters_are_not_translated() {
    for t in ["-", ".00", "0", "0,25×", "00:00", "0.0.0", "+"] {
        assert!(!aylis_project::i18n::translatable(t), "{t}");
    }
    for t in ["Salvar", "R$ 0,00", "Olá!"] {
        assert!(aylis_project::i18n::translatable(t), "{t}");
    }
}
