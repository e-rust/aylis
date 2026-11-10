//! As linguagens e os temas do VS Code no editor da IDE: as gramáticas oficiais das 10
//! linguagens embutidas (de `resources/grammars`, MIT), as regras por escopo do Dark+ e do
//! Light+ e o que o usuário importa ("Importar linguagem…" e "Importar tema…": um `.vsix`, uma
//! gramática `.tmLanguage(.json)` ou um tema `.json` soltos), guardado em
//! `%LOCALAPPDATA%\aylis\vscode` e lido de novo ao abrir a IDE.

use std::io::Read;
use std::path::{Path, PathBuf};

use aylis_controls::{textmate, textmate_theme};
use aylis_core::Ui;
use serde_json::Value;

macro_rules! grammar {
    ($file:literal) => {
        include_str!(concat!("../resources/grammars/", $file))
    };
}

/// (linguagem, gramática, configuração). Linguagem vazia: só incluída por outra.
const BUILTIN: &[(&str, &str, Option<&str>)] = &[
    ("rust", grammar!("rust.tmLanguage.json"), Some(grammar!("rust.language-configuration.json"))),
    ("python", grammar!("python.tmLanguage.json"), Some(grammar!("python.language-configuration.json"))),
    ("", grammar!("regexp.python.tmLanguage.json"), None),
    ("json", grammar!("json.tmLanguage.json"), Some(grammar!("json.language-configuration.json"))),
    ("pascal", grammar!("pascal.tmLanguage"), Some(grammar!("pascal.language-configuration.json"))),
    ("typescript", grammar!("typescript.tmLanguage.json"), Some(grammar!("typescript.language-configuration.json"))),
    ("javascript", grammar!("javascript.tmLanguage.json"), Some(grammar!("javascript.language-configuration.json"))),
    ("csharp", grammar!("csharp.tmLanguage.json"), Some(grammar!("csharp.language-configuration.json"))),
    ("java", grammar!("java.tmLanguage.json"), Some(grammar!("java.language-configuration.json"))),
    ("go", grammar!("go.tmLanguage.json"), Some(grammar!("go.language-configuration.json"))),
    ("cpp", grammar!("cpp.tmLanguage.json"), Some(grammar!("cpp.language-configuration.json"))),
    ("c", grammar!("c.tmLanguage.json"), Some(grammar!("cpp.language-configuration.json"))),
];

/// Os temas embutidos que ganham as regras por escopo (o arquivo e o de base que ele inclui).
const THEMES: &[(&str, &str, &str)] = &[
    ("Dark+", include_str!("../resources/themes/dark_plus.json"), include_str!("../resources/themes/dark_vs.json")),
    ("Light+", include_str!("../resources/themes/light_plus.json"), include_str!("../resources/themes/light_vs.json")),
];

/// Registra as embutidas e as importadas (ao abrir a IDE).
pub fn register() {
    for (lang, source, config) in BUILTIN {
        if let Err(e) = textmate::register_grammar(lang, source) {
            eprintln!("gramática {lang}: {e}");
        }
        if let Some(config) = config {
            if let Err(e) = textmate::register_language_config(lang, config) {
                eprintln!("configuração {lang}: {e}");
            }
        }
    }
    for (name, theme, base) in THEMES {
        if let Err(e) = textmate_theme::register_theme(theme, name, &|_| Some(base.to_string())) {
            eprintln!("tema {name}: {e}");
        }
    }
    let Some(store) = store() else { return };
    let Ok(entries) = std::fs::read_dir(&store) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.join("extension").join("package.json").is_file() || path.join("package.json").is_file() {
            let _ = load_extension(&path);
        } else if path.is_dir() {
            for file in std::fs::read_dir(&path).into_iter().flatten().flatten() {
                let _ = load_loose(&file.path());
            }
        }
    }
}

/// `%LOCALAPPDATA%\aylis\vscode`.
pub fn store() -> Option<PathBuf> {
    Some(aylis_tools::dap::tools_dir()?.parent()?.join("vscode"))
}

/// O que uma importação trouxe.
#[derive(Debug, Default)]
pub struct Imported {
    pub languages: Vec<String>,
    pub themes: Vec<String>,
    /// As regras com expressões que o motor não aceita (ficam de fora do realce).
    pub warnings: Vec<String>,
}

/// Importa um `.vsix`, uma gramática ou um tema: copia para a pasta da IDE e registra.
pub fn import(file: &Path) -> Result<Imported, String> {
    import_to(file, &store().ok_or("sem a pasta de dados da IDE")?)
}

/// [`import`] numa pasta escolhida (os testes).
pub fn import_to(file: &Path, store: &Path) -> Result<Imported, String> {
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("importado").to_string();
    let mut result = if ext == "vsix" {
        let dir = store.join(safe_name(&stem));
        extract_vsix(file, &dir)?;
        load_extension(&dir)?
    } else {
        let dir = store.join("soltos");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let target = dir.join(file.file_name().ok_or("arquivo sem nome")?);
        std::fs::copy(file, &target).map_err(|e| e.to_string())?;
        load_loose(&target)?
    };
    for lang in &result.languages {
        result.warnings.extend(textmate::grammar_warnings(lang).into_iter().map(|w| format!("{lang}: {w}")));
    }
    Ok(result)
}

fn safe_name(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() || matches!(c, '-' | '_' | '.') { c } else { '_' }).collect()
}

/// Tira do `.vsix` (um ZIP) só o que o editor usa: o `package.json` e os JSON/plist.
fn extract_vsix(file: &Path, dir: &Path) -> Result<(), String> {
    let f = std::fs::File::open(file).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipArchive::new(f).map_err(|e| format!("não é um .vsix: {e}"))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        let Some(name) = entry.enclosed_name() else { continue };
        let lower = name.to_string_lossy().to_ascii_lowercase();
        if !(lower.ends_with(".json") || lower.ends_with(".tmlanguage") || lower.ends_with(".plist") || lower.ends_with(".tmtheme")) {
            continue;
        }
        if entry.size() > 20 * 1024 * 1024 {
            continue;
        }
        let target = dir.join(&name);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        std::fs::write(&target, bytes).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Registra o que o `package.json` de uma extensão contribui (linguagens, gramáticas e temas).
fn load_extension(dir: &Path) -> Result<Imported, String> {
    let root = if dir.join("extension").join("package.json").is_file() { dir.join("extension") } else { dir.to_path_buf() };
    let text = std::fs::read_to_string(root.join("package.json")).map_err(|e| e.to_string())?;
    let pkg: Value = serde_json::from_str(&textmate::strip_jsonc(&text)).map_err(|e| e.to_string())?;
    let contributes = pkg.get("contributes").cloned().unwrap_or(Value::Null);
    let read = |rel: &str| std::fs::read_to_string(root.join(rel.trim_start_matches("./"))).ok();
    let mut out = Imported::default();
    for lang in contributes.get("languages").and_then(Value::as_array).into_iter().flatten() {
        let Some(id) = lang.get("id").and_then(Value::as_str) else { continue };
        let exts: Vec<String> = lang.get("extensions").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        textmate::register_extensions(id, &exts);
        if let Some(config) = lang.get("configuration").and_then(Value::as_str).and_then(read) {
            let _ = textmate::register_language_config(id, &config);
        }
    }
    for g in contributes.get("grammars").and_then(Value::as_array).into_iter().flatten() {
        let Some(source) = g.get("path").and_then(Value::as_str).and_then(read) else { continue };
        let lang = g.get("language").and_then(Value::as_str).unwrap_or("");
        match textmate::register_grammar(lang, &source) {
            Ok(_) if !lang.is_empty() => out.languages.push(lang.to_string()),
            Ok(_) => {}
            Err(e) => out.warnings.push(format!("{lang}: {e}")),
        }
    }
    for t in contributes.get("themes").and_then(Value::as_array).into_iter().flatten() {
        let Some(path) = t.get("path").and_then(Value::as_str) else { continue };
        let Some(source) = read(path) else { continue };
        let label = t.get("label").and_then(Value::as_str).unwrap_or("");
        let base = root.join(path.trim_start_matches("./"));
        let base_dir = base.parent().map(Path::to_path_buf).unwrap_or_default();
        match textmate_theme::register_theme(&source, label, &|inc| std::fs::read_to_string(base_dir.join(inc.trim_start_matches("./"))).ok()) {
            Ok(name) => out.themes.push(name),
            Err(e) => out.warnings.push(format!("{label}: {e}")),
        }
    }
    if out.languages.is_empty() && out.themes.is_empty() {
        return Err("a extensão não traz linguagem nem tema".into());
    }
    Ok(out)
}

/// Uma gramática ou um tema soltos.
fn load_loose(file: &Path) -> Result<Imported, String> {
    let text = std::fs::read_to_string(file).map_err(|e| e.to_string())?;
    let mut out = Imported::default();
    if let Ok(grammar) = textmate::parse_grammar(&text) {
        let stem = file.file_name().and_then(|s| s.to_str()).unwrap_or("").split('.').next().unwrap_or("").to_ascii_lowercase();
        let name = grammar.get("name").and_then(Value::as_str).map(str::to_ascii_lowercase).unwrap_or(stem);
        let lang = safe_name(&name.replace(' ', "-"));
        textmate::register_grammar(&lang, &text)?;
        let exts: Vec<String> = grammar.get("fileTypes").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        textmate::register_extensions(&lang, &exts);
        out.languages.push(lang);
        return Ok(out);
    }
    let dir = file.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let name = textmate_theme::register_theme(&text, stem, &|inc| std::fs::read_to_string(dir.join(inc.trim_start_matches("./"))).ok())?;
    out.themes.push(name);
    Ok(out)
}

/// Exibir → Importar linguagem… / Importar tema…: escolhe o arquivo e conta o que entrou.
pub fn import_dialog(ui: &mut Ui, theme: bool) {
    let dialog = crate::files::FileDialog::new();
    let dialog = if theme {
        dialog.add_filter("Tema do VS Code", &["vsix", "json"])
    } else {
        dialog.add_filter("Linguagem do VS Code", &["vsix", "json", "tmLanguage"])
    };
    let Some(file) = dialog.pick_file() else { return };
    let (kind, text) = match import(&file) {
        Ok(r) => {
            let mut text = String::new();
            if !r.languages.is_empty() {
                text.push_str(&format!("Linguagens: {}\n", r.languages.join(", ")));
            }
            if !r.themes.is_empty() {
                text.push_str(&format!("Temas: {} (Exibir → Tema do editor)\n", r.themes.join(", ")));
            }
            if !r.warnings.is_empty() {
                let shown: Vec<&str> = r.warnings.iter().take(5).map(String::as_str).collect();
                text.push_str(&format!(
                    "\n{} regra(s) usam expressões do Oniguruma que o editor não aceita e ficam sem realce:\n{}",
                    r.warnings.len(),
                    shown.join("\n")
                ));
            }
            (if r.warnings.is_empty() { aylis_controls::DlgKind::Information } else { aylis_controls::DlgKind::Warning }, text)
        }
        Err(e) => (aylis_controls::DlgKind::Error, format!("Não deu para importar: {e}")),
    };
    aylis_controls::message_dlg(ui, &text, kind, &[aylis_core::ModalResult::Ok], |_, _| {});
    ui.request_repaint();
}
