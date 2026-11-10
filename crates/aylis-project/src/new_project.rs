//! O projeto novo com as escolhas do assistente (Arquivo → Novo projeto): as plataformas de
//! destino, o modelo, o tema, o renderizador de cada plataforma, o ícone, o nome e o id do
//! aplicativo e as permissões que ele vai pedir.

use std::path::{Path, PathBuf};

use crate::{Error, Language, Project, RenderOptions};

/// As plataformas de destino: a chave gravada no `.aylisproj` (`platforms`) e o nome.
pub const TARGETS: [(&str, &str); 6] =
    [("windows", "Windows"), ("linux", "Linux"), ("macos", "macOS"), ("android", "Android"), ("ios", "iOS"), ("web", "Navegador (web)")];

/// O que o assistente escolheu além do nome, da pasta e da linguagem.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NewProjectOptions {
    /// Um dos [`crate::TEMPLATES`] (vazio = o primeiro, o Desktop).
    pub template: String,
    /// As chaves de [`TARGETS`]; vazio = todas.
    pub platforms: Vec<String>,
    /// `""` (claro) ou `"dark"`.
    pub theme: String,
    pub render: RenderOptions,
    /// O idioma do app (`pt-BR`).
    pub locale: String,
    /// Um PNG ou SVG: vai para `assets/images/` e vira o ícone do app.
    pub icon: Option<PathBuf>,
    /// O nome mostrado ao usuário (vazio = o do projeto).
    pub app_name: String,
    /// O identificador no Android e no iOS (`com.empresa.vendas`; vazio = `dev.aylis.<nome>`).
    pub identifier: String,
    /// As permissões (`camera`, `location`...; [`aylis_core::platform::Permission`]): o form
    /// principal ganha um `Permissions` com elas, que a publicação declara no Android e no iOS.
    pub permissions: Vec<String>,
}

/// O id sugerido para o app a partir do nome (`Vendas Web` → `dev.aylis.vendasweb`).
pub fn suggested_identifier(name: &str) -> String {
    let clean: String = name.chars().filter(|c| c.is_ascii_alphanumeric()).flat_map(char::to_lowercase).collect();
    let clean = if clean.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) { clean } else { format!("app{clean}") };
    format!("dev.aylis.{clean}")
}

/// Confere o id do app: partes com letras, números e `_`, separadas por ponto, cada uma
/// começando por letra (a regra do Android; o iOS aceita isso também).
pub fn check_identifier(id: &str) -> Result<(), Error> {
    let parts: Vec<&str> = id.split('.').collect();
    let ok = parts.len() >= 2
        && parts.iter().all(|p| p.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
    if ok { Ok(()) } else { Err(format!("'{id}' não serve como id do aplicativo (ex.: com.empresa.vendas)")) }
}

/// Cria o projeto (como [`crate::create`]) e aplica as escolhas do assistente.
pub fn create_with(location: &Path, name: &str, language: Language, options: &NewProjectOptions) -> Result<Project, Error> {
    if !options.identifier.is_empty() {
        check_identifier(&options.identifier)?;
    }
    let mut project = crate::create(location, name, language)?;
    let template = if options.template.is_empty() { crate::TEMPLATES[0].name } else { options.template.as_str() };
    crate::apply_template(&mut project, template)?;
    project.render = Some(options.render);
    if !options.locale.is_empty() {
        project.locale = options.locale.clone();
    }
    // O modelo pode trazer o tema dele; o assistente só troca quando foi escolhido o escuro.
    if options.theme == "dark" && project.theme.is_empty() {
        project.theme = "dark".into();
    }
    // As plataformas marcadas no assistente (sem nenhuma, as do modelo).
    if !options.platforms.is_empty() {
        let all = TARGETS.iter().all(|(k, _)| options.platforms.iter().any(|p| p == k));
        project.platforms = if all { Vec::new() } else { options.platforms.clone() };
    }
    project.publish.app_name = if options.app_name.trim() == name { String::new() } else { options.app_name.trim().to_string() };
    project.publish.identifier = if options.identifier == suggested_identifier(name) { String::new() } else { options.identifier.clone() };
    if let Some(icon) = &options.icon {
        let file = icon.file_name().ok_or_else(|| format!("ícone sem nome: {}", icon.display()))?;
        let target = project.dir().join("assets/images").join(file);
        std::fs::copy(icon, &target).map_err(|e| format!("não consegui copiar o ícone {}: {e}", icon.display()))?;
        project.publish.icon = format!("assets/images/{}", file.to_string_lossy());
    }
    if !options.permissions.is_empty() {
        add_permissions(&project, &options.permissions)?;
    }
    project.save()?;
    crate::codegen::sync_cargo(&project)?;
    Ok(project)
}

/// Põe no form principal um `Permissions` (`prmApp`) com a lista: a publicação declara as
/// permissões dele, e o app pede com `frmX.prmApp.request_all()`.
fn add_permissions(project: &Project, permissions: &[String]) -> Result<(), Error> {
    let list: Vec<&str> = permissions.iter().map(String::as_str).filter(|p| aylis_core::platform::Permission::parse(p).is_some()).collect();
    if list.is_empty() {
        return Ok(());
    }
    let entry = project.form(&project.main_form).ok_or_else(|| format!("o projeto não tem o form '{}'", project.main_form))?;
    let path = project.file(&entry.file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
    let mut json: serde_json::Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", path.display()))?;
    let root = json.get_mut("root").and_then(|r| r.as_object_mut()).ok_or_else(|| format!("{}: sem o form", path.display()))?;
    let children = root.entry("children").or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if let Some(children) = children.as_array_mut() {
        children.push(serde_json::json!({ "class": "Permissions", "name": "prmApp", "permissions": list.join(";") }));
    }
    let text = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("não consegui gravar {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wizard_choices_go_to_the_project() {
        let dir = std::env::temp_dir().join(format!("aylis-novo-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let icon = dir.join("logo.png");
        std::fs::write(&icon, b"\x89PNG\r\n\x1a\n").unwrap();
        let options = NewProjectOptions {
            platforms: vec!["windows".into(), "android".into()],
            theme: "dark".into(),
            locale: "pt-BR".into(),
            icon: Some(icon),
            app_name: "Vendas da Loja".into(),
            identifier: "com.loja.vendas".into(),
            permissions: vec!["camera".into(), "location".into(), "nada".into()],
            ..Default::default()
        };
        let project = create_with(&dir, "Vendas", Language::Rust, &options).unwrap();
        let again = Project::open(&project.path).unwrap();
        assert_eq!(again.platforms, ["windows", "android"]);
        assert_eq!(again.theme, "dark");
        assert_eq!(again.publish.app_name, "Vendas da Loja");
        assert_eq!(again.publish.identifier, "com.loja.vendas");
        assert_eq!(again.publish.icon, "assets/images/logo.png");
        assert!(again.dir().join("assets/images/logo.png").is_file());
        let form = again.read_form(&again.main_form).unwrap();
        assert!(form.contains("\"Permissions\"") && form.contains("camera;location"), "{form}");
        // Sem plataformas marcadas, as do modelo (o primeiro, o Desktop); o id sugerido não grava.
        let plain = create_with(&dir, "Outro", Language::Rust, &NewProjectOptions { identifier: suggested_identifier("Outro"), ..Default::default() }).unwrap();
        assert_eq!(plain.platforms, ["windows", "linux", "macos"]);
        assert!(plain.publish.identifier.is_empty());
        assert!(create_with(&dir, "Errado", Language::Rust, &NewProjectOptions { identifier: "sem ponto".into(), ..Default::default() }).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Os modelos Desktop, Smartphone e Web: em branco, com as plataformas, o tamanho do form
    /// e (no Smartphone) o aparelho com a moldura.
    #[test]
    fn the_templates_configure_the_blank_project() {
        let dir = std::env::temp_dir().join(format!("aylis-modelos-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for (template, platforms, width, device) in [("Desktop", vec!["windows", "linux", "macos"], 1024.0, ""), ("Smartphone", vec!["android", "ios"], 412.0, "Pixel 8"), ("Web", vec!["web"], 1280.0, "")] {
            let options = NewProjectOptions { template: template.into(), ..Default::default() };
            let project = create_with(&dir, template, Language::Rust, &options).unwrap();
            let again = Project::open(&project.path).unwrap();
            assert_eq!(again.platforms, platforms, "{template}");
            assert_eq!(again.device, device, "{template}");
            assert!(device.is_empty() || aylis_core::devices::parse(&again.device).is_some(), "o aparelho existe");
            let form: serde_json::Value = serde_json::from_str(&again.read_form(&again.main_form).unwrap()).unwrap();
            assert_eq!(form["root"]["width"].as_f64(), Some(width), "{template}");
            assert_eq!(form["root"]["children"].as_array().map_or(0, Vec::len), 0, "em branco");
        }
        assert!(crate::template("Landing page").is_none(), "o Landing page saiu dos modelos");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn identifiers() {
        assert_eq!(suggested_identifier("Vendas Web"), "dev.aylis.vendasweb");
        assert_eq!(suggested_identifier("3D"), "dev.aylis.app3d");
        assert!(check_identifier("com.empresa.vendas").is_ok());
        assert!(check_identifier("com.1empresa").is_err() && check_identifier("vendas").is_err() && check_identifier("com.em-presa").is_err());
    }
}
