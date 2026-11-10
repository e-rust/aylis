//! Fontes e ícones do projeto, a partir dos recursos offline da IDE (`resources/`, montado
//! por `aylis-tool resources`): o catálogo de fontes do Google Fonts e os pacotes de ícones
//! (Material Symbols, Fluent). O projeto guarda só o que usa:
//!
//! - `assets/fonts/<pasta>/`: os arquivos da família e a licença (OFL.txt...);
//! - `assets/icons/<pacote>.txt`: só os ícones citados (o "Gerar código" mantém em dia) e
//!   `app.txt`, os SVGs importados pelo app.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, Project, codegen};

/// Uma família do catálogo offline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FontEntry {
    pub family: String,
    /// Pasta (em `resources/fonts/` e em `assets/fonts/`).
    pub folder: String,
    pub license: String,
    #[serde(default)]
    pub category: String,
    pub files: Vec<String>,
}

fn fonts_dir() -> PathBuf {
    codegen::resources_dir().join("fonts")
}

/// O catálogo de fontes offline da IDE.
pub fn font_catalog() -> Vec<FontEntry> {
    std::fs::read_to_string(fonts_dir().join("catalog.json"))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

/// As pastas de fonte que o projeto já tem (`assets/fonts/<pasta>`).
pub fn project_fonts(project: &Project) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(project.dir().join("assets/fonts"))
        .map(|d| d.flatten().filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().to_string()).collect())
        .unwrap_or_default();
    out.sort();
    out
}

const LICENSE_FILES: [&str; 3] = ["OFL.txt", "LICENSE.txt", "UFL.txt"];

/// Copia a família (arquivos e licença) para `assets/fonts/<pasta>/`.
pub fn add_font(project: &Project, entry: &FontEntry) -> Result<(), Error> {
    let from = fonts_dir().join(&entry.folder);
    let to = project.dir().join("assets/fonts").join(&entry.folder);
    std::fs::create_dir_all(&to).map_err(|e| format!("não consegui criar {}: {e}", to.display()))?;
    for file in entry.files.iter().map(String::as_str).chain(LICENSE_FILES) {
        let src = from.join(file);
        if src.exists() {
            std::fs::copy(&src, to.join(file)).map_err(|e| format!("não consegui copiar {}: {e}", src.display()))?;
        }
    }
    Ok(())
}

/// Tira a família do projeto.
pub fn remove_font(project: &Project, folder: &str) -> Result<(), Error> {
    let dir = project.dir().join("assets/fonts").join(folder);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|e| format!("não consegui apagar {}: {e}", dir.display()))?;
    }
    Ok(())
}

fn curl(url: &str) -> Result<Vec<u8>, Error> {
    let out = std::process::Command::new("curl")
        .args(["-sSfL", "--proto", "=https", "-H", "User-Agent: aylis-ide", "--", url])
        .output()
        .map_err(|e| format!("curl não encontrado ({e}); ele vem no Windows 10+, macOS e Linux"))?;
    if !out.status.success() {
        return Err(format!("não consegui baixar {url}: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(out.stdout)
}

/// Baixa uma família do Google Fonts (do repositório google/fonts) para o catálogo offline.
/// `family` é o nome ("Fira Sans") ou a pasta ("firasans").
pub fn download_google_font(family: &str) -> Result<FontEntry, Error> {
    let folder: String = family.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if folder.is_empty() {
        return Err("informe o nome da família".into());
    }
    let mut found = None;
    for license in ["ofl", "apache", "ufl"] {
        if let Ok(data) = curl(&format!("https://api.github.com/repos/google/fonts/contents/{license}/{folder}")) {
            if let Ok(list) = serde_json::from_slice::<Vec<serde_json::Value>>(&data) {
                found = Some((license, list));
                break;
            }
        }
    }
    let (license, list) = found.ok_or_else(|| format!("'{family}' não está no Google Fonts (pasta {folder})"))?;
    let dir = fonts_dir().join(&folder);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut files = Vec::new();
    let mut name = family.to_string();
    for item in list {
        let (Some(file), Some(url)) = (item["name"].as_str(), item["download_url"].as_str()) else { continue };
        // A lista vem da API do GitHub: só um nome de arquivo simples (nada de pasta ou
        // "..") e só do próprio GitHub.
        let plain = !file.is_empty() && !file.starts_with('.') && file.chars().all(|c| c.is_ascii_alphanumeric() || "-_.[],".contains(c));
        if !plain || !url.starts_with("https://raw.githubusercontent.com/google/fonts/") {
            continue;
        }
        let wanted = file.ends_with(".ttf") || file.ends_with(".otf") || LICENSE_FILES.contains(&file) || file == "METADATA.pb";
        if !wanted {
            continue;
        }
        let data = curl(url)?;
        if file == "METADATA.pb" {
            let text = String::from_utf8_lossy(&data);
            if let Some(n) = text.lines().find_map(|l| l.strip_prefix("name: \"").and_then(|r| r.strip_suffix('"'))) {
                name = n.to_string();
            }
        }
        std::fs::write(dir.join(file), &data).map_err(|e| e.to_string())?;
        if file.ends_with(".ttf") || file.ends_with(".otf") {
            files.push(file.to_string());
        }
    }
    if files.is_empty() {
        return Err(format!("'{family}' não tem arquivos de fonte"));
    }
    let entry = FontEntry {
        family: name,
        folder: folder.clone(),
        license: match license {
            "ofl" => "SIL OFL 1.1",
            "apache" => "Apache-2.0",
            _ => "Ubuntu Font Licence 1.0",
        }
        .into(),
        category: String::new(),
        files,
    };
    let mut catalog = font_catalog();
    catalog.retain(|e| e.folder != folder);
    catalog.push(entry.clone());
    catalog.sort_by(|a, b| a.family.cmp(&b.family));
    let json = serde_json::to_string_pretty(&catalog).map_err(|e| e.to_string())?;
    std::fs::write(fonts_dir().join("catalog.json"), json).map_err(|e| e.to_string())?;
    Ok(entry)
}

/// Os pacotes de ícones da IDE (`material`, `material-rounded`, `material-sharp`, `fluent`...).
pub fn icon_packs() -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(codegen::resources_dir().join("icons"))
        .map(|d| d.flatten().filter_map(|e| e.file_name().to_string_lossy().strip_suffix(".txt").map(str::to_string)).collect())
        .unwrap_or_default();
    out.sort();
    out
}

/// O texto de um pacote da IDE.
pub fn icon_pack_text(pack: &str) -> Option<String> {
    std::fs::read_to_string(codegen::resources_dir().join("icons").join(format!("{pack}.txt"))).ok()
}

/// Importa um SVG para o pacote do app (`assets/icons/app.txt`), como `app:<nome>`. Os
/// caminhos são trazidos para a grade de 24 (o `viewBox` do arquivo é respeitado).
pub fn import_svg(project: &Project, name: &str, svg: &str) -> Result<String, Error> {
    let name: String = name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let name = name.trim_matches('-').to_string();
    if name.is_empty() {
        return Err("informe um nome para o ícone".into());
    }
    // O leitor de SVG do aylis: caminhos e formas, com grupos, transformações e `<use>`.
    let image = aylis_vector::SvgImage::parse(svg)?;
    let view = image.view_box();
    let scale = 24.0 / view.width.max(view.height).max(1e-3);
    let (outlines, stroke_only) = image.fill_outlines();
    let parts: Vec<String> = outlines
        .iter()
        .map(|p| p.transformed(scale, aylis_core::PointF::new(-view.x * scale, -view.y * scale)).to_svg())
        .collect();
    if parts.is_empty() {
        return Err(if stroke_only > 0 {
            "o SVG só tem traços (ícone de contorno); os ícones do Aylis são preenchidos: converta os traços em formas no editor (\"Stroke to path\")".into()
        } else {
            "o SVG não tem formas".into()
        });
    }
    let file = project.dir().join("assets/icons/app.txt");
    let mut text = std::fs::read_to_string(&file)
        .unwrap_or_else(|_| "# aylis-icons 1\n# title: Ícones do app\n# license: do próprio app\n# view: 0 0 24 24\n".into());
    let line_start = format!("{name}\t");
    let mut lines: Vec<String> = text.lines().filter(|l| !l.starts_with(&line_start)).map(str::to_string).collect();
    lines.push(format!("{name}\t{}", parts.join(" ")));
    text = lines.join("\n") + "\n";
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&file, text).map_err(|e| e.to_string())?;
    Ok(format!("app:{name}"))
}

/// O caminho de um arquivo dentro do projeto (para a IDE mostrar).
pub fn relative(project: &Project, path: &Path) -> String {
    path.strip_prefix(project.dir()).map_or_else(|_| path.display().to_string(), |p| p.display().to_string())
}

// ---------------------------------------------------------------------------
// Fontes em uso

/// Uma família de fonte usada no projeto e onde.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontUse {
    /// A família como está nos arquivos (`""` é a fonte padrão do sistema).
    pub family: String,
    /// O nome para mostrar ([`font_label`]): as genéricas ganham o nome da fonte real.
    pub label: String,
    /// Onde aparece: "projeto", "forms/main.json: lblTitulo", "themes/escuro.json"...
    pub places: Vec<String>,
}

/// Os arquivos `.json` de uma pasta do projeto (e subpastas), com o caminho relativo.
fn json_files(project: &Project, folder: &str) -> Vec<(String, PathBuf)> {
    fn walk(dir: &Path, base: &Path, out: &mut Vec<(String, PathBuf)>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                walk(&p, base, out);
            } else if p.extension().is_some_and(|x| x == "json") {
                let rel = p.strip_prefix(base).unwrap_or(&p).to_string_lossy().replace('\\', "/");
                out.push((rel, p));
            }
        }
    }
    let mut out = Vec::new();
    walk(&project.dir().join(folder), &project.dir(), &mut out);
    out
}

/// A chave que guarda a família em cada tipo de arquivo: nos forms, a propriedade
/// `font_family` dos componentes; nos temas, o `family` das fontes e estilos.
fn family_key(folder: &str) -> &'static str {
    if folder == "themes" { "family" } else { "font_family" }
}

type Uses = std::collections::BTreeMap<String, Vec<String>>;

fn collect_fonts(value: &serde_json::Value, key: &str, file: &str, named: bool, map: &mut Uses) {
    match value {
        serde_json::Value::Object(o) => {
            if let Some(family) = o.get(key).and_then(|v| v.as_str()).filter(|f| !f.is_empty()) {
                let place = match o.get("name").and_then(|n| n.as_str()).filter(|_| named) {
                    Some(name) => format!("{file}: {name}"),
                    None => file.to_string(),
                };
                let places = map.entry(family.to_string()).or_default();
                if !places.contains(&place) {
                    places.push(place);
                }
            }
            o.values().for_each(|v| collect_fonts(v, key, file, named, map));
        }
        serde_json::Value::Array(a) => a.iter().for_each(|v| collect_fonts(v, key, file, named, map)),
        _ => {}
    }
}

/// O nome de uma família para mostrar: as genéricas (`""`, `sans-serif`, `monospace`...)
/// dizem qual fonte o sistema usa no lugar.
pub fn font_label(family: &str) -> String {
    match family {
        "" | "sans-serif" | "system-ui" => "Fonte do sistema (Segoe UI no Windows)".into(),
        "monospace" => "Monoespaçada do sistema (Consolas no Windows)".into(),
        "serif" => "Serifada do sistema (Times New Roman no Windows)".into(),
        other => other.to_string(),
    }
}

/// Uma família monoespaçada (a "Fonte do projeto" não troca essas: o código fica alinhado).
pub fn is_monospace(family: &str) -> bool {
    family == "monospace" || family.to_lowercase().contains("mono")
}

/// As famílias do tema embutido do projeto (o Windows 11, o Material 3...), que não estão
/// em arquivo: sem a "Fonte do projeto", todas; com ela, só as monoespaçadas (as outras
/// já viraram a do projeto). Vazio quando o tema é um arquivo do projeto.
fn builtin_theme_families(project: &Project) -> Option<(String, Vec<String>)> {
    if project.theme.ends_with(".json") {
        return None;
    }
    let mut plain = project.clone();
    plain.font.clear();
    let theme = crate::resolve_theme(&plain, &crate::Files::Dir(project.dir())).ok()?;
    let mut families: Vec<String> = theme
        .fonts
        .values()
        .map(|f| f.family.clone())
        .filter(|f| project.font.is_empty() || is_monospace(f))
        .collect();
    families.sort_by_key(|f| f.to_lowercase());
    families.dedup();
    Some((theme.name, families))
}

/// As famílias de fonte que o projeto usa: a do projeto, as do tema embutido, as dos
/// componentes dos forms e as dos temas próprios. Em ordem alfabética.
pub fn fonts_in_use(project: &Project) -> Vec<FontUse> {
    let mut map = Uses::new();
    if !project.font.is_empty() {
        map.entry(project.font.clone()).or_default().push("projeto".into());
    }
    if let Some((name, families)) = builtin_theme_families(project) {
        for family in families {
            map.entry(family).or_default().push(format!("tema {name}"));
        }
    }
    for folder in ["forms", "themes"] {
        let key = family_key(folder);
        for (rel, path) in json_files(project, folder) {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(text.trim_start_matches('\u{feff}')) else { continue };
            collect_fonts(&value, key, &rel, folder == "forms", &mut map);
        }
    }
    // Um nome do projeto aparece com a família dele.
    let label = |family: &str| match project.fonts.get(family) {
        Some(target) => format!("{family} → {}", font_label(target)),
        None => font_label(family),
    };
    let mut out: Vec<FontUse> = map.into_iter().map(|(family, places)| FontUse { label: label(&family), family, places }).collect();
    out.sort_by_key(|u| u.label.to_lowercase());
    out
}

/// Troca o valor `"key": "from"` por `to` no texto JSON, sem mexer no resto (a formatação
/// fica como está). Devolve o texto novo e quantas trocou.
fn replace_value(text: &str, key: &str, from: &str, to: &str) -> (String, usize) {
    let quoted_key = format!("\"{key}\"");
    let from_json = serde_json::to_string(from).unwrap_or_default();
    let to_json = serde_json::to_string(to).unwrap_or_default();
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut count = 0;
    while let Some(i) = rest.find(&quoted_key) {
        let after_key = &rest[i + quoted_key.len()..];
        let trimmed = after_key.trim_start();
        let gap = after_key.len() - trimmed.len();
        if let Some(value) = trimmed.strip_prefix(':') {
            let value_trim = value.trim_start();
            let gap2 = value.len() - value_trim.len();
            if value_trim.starts_with(&from_json) {
                out.push_str(&rest[..i + quoted_key.len() + gap + 1 + gap2]);
                out.push_str(&to_json);
                rest = &value_trim[from_json.len()..];
                count += 1;
                continue;
            }
        }
        out.push_str(&rest[..i + quoted_key.len()]);
        rest = after_key;
    }
    out.push_str(rest);
    (out, count)
}

/// Troca a família `from` por `to` no projeto inteiro: a fonte do projeto, os componentes
/// dos forms e os temas próprios. Devolve quantos lugares mudaram. A fonte do projeto muda
/// no `project` (quem chama salva o `.aylisproj`).
pub fn replace_font(project: &mut Project, from: &str, to: &str) -> Result<usize, Error> {
    let mut total = 0;
    // Uma fonte nomeada: troca a família dela (um lugar só; os componentes seguem o nome).
    if let Some(target) = project.fonts.get_mut(from) {
        *target = to.to_string();
        return Ok(1);
    }
    if project.font == from {
        project.font = to.to_string();
        total += 1;
    } else if !is_monospace(from) && builtin_theme_families(project).is_some_and(|(_, f)| f.iter().any(|x| x == from)) {
        // Uma família do tema embutido: a "Fonte do projeto" troca ela em todos os estilos.
        project.font = to.to_string();
        total += 1;
    }
    for folder in ["forms", "themes"] {
        for (_, path) in json_files(project, folder) {
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let (new, n) = replace_value(&text, family_key(folder), from, to);
            if n > 0 {
                std::fs::write(&path, new).map_err(|e| format!("{}: {e}", path.display()))?;
                total += n;
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod font_tests {
    use super::*;

    #[test]
    fn replace_keeps_the_formatting() {
        let text = "{ \"font_family\":  \"Arial\", \"x\": 1,\n  \"font_family\": \"Arial Black\", \"family\": \"Arial\" }";
        let (out, n) = replace_value(text, "font_family", "Arial", "Inter");
        assert_eq!(n, 1, "só o valor igual (não o 'Arial Black')");
        assert_eq!(out, "{ \"font_family\":  \"Inter\", \"x\": 1,\n  \"font_family\": \"Arial Black\", \"family\": \"Arial\" }");
    }

    #[test]
    fn fonts_in_use_and_replace_across_the_project() {
        let dir = std::env::temp_dir().join(format!("aylis_fonts_in_use_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("forms")).unwrap();
        std::fs::create_dir_all(dir.join("themes")).unwrap();
        std::fs::write(
            dir.join("forms/main.json"),
            r#"{ "root": { "class": "Form", "name": "frmMain", "children": [
                { "class": "Label", "name": "lblA", "font_family": "Arial" },
                { "class": "Label", "name": "lblB", "font_family": "Roboto" } ] } }"#,
        )
        .unwrap();
        std::fs::write(dir.join("themes/escuro.json"), r#"{ "fonts": { "titulo": { "family": "Arial", "size": 20 } } }"#).unwrap();
        let mut project: Project =
            serde_json::from_value(serde_json::json!({ "aylis_project": 1, "name": "t", "main_form": "frmMain", "forms": [], "font": "Roboto" })).unwrap();
        project.path = dir.join("t.aylisproj");
        let uses = fonts_in_use(&project);
        // A monoespaçada do tema embutido fica (a fonte do projeto não troca ela).
        assert_eq!(uses.iter().map(|u| u.family.as_str()).collect::<Vec<_>>(), ["Arial", "monospace", "Roboto"]);
        assert_eq!(uses[1].label, "Monoespaçada do sistema (Consolas no Windows)");
        assert_eq!(uses[0].places, ["forms/main.json: lblA", "themes/escuro.json"]);
        assert_eq!(uses[2].places, ["projeto", "forms/main.json: lblB"]);

        assert_eq!(replace_font(&mut project, "Arial", "Inter").unwrap(), 2);
        let uses = fonts_in_use(&project);
        assert_eq!(uses.iter().map(|u| u.family.as_str()).collect::<Vec<_>>(), ["Inter", "monospace", "Roboto"]);
        assert_eq!(replace_font(&mut project, "Roboto", "Inter").unwrap(), 2);
        assert_eq!(project.font, "Inter");
        // Sem fonte do projeto, a do sistema (família vazia do tema) aparece e troca por ela.
        project.font.clear();
        let uses = fonts_in_use(&project);
        assert_eq!(uses.iter().find(|u| u.family.is_empty()).map(|u| u.label.as_str()), Some("Fonte do sistema (Segoe UI no Windows)"));
        assert_eq!(replace_font(&mut project, "", "Lato").unwrap(), 1);
        assert_eq!(project.font, "Lato");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
