//! O `.deb` de um app publicado (Debian, Ubuntu, Mint): o app em `/opt/<app>`, o atalho em
//! `/usr/bin/<app>`, o `.desktop` no menu (com os esquemas de link dos `UrlScheme` ligados), o
//! ícone do projeto e, nas dependências, as bibliotecas do sistema e o runtime da linguagem.
//! Montado em Rust (o `ar` e os `.tar.gz` do [`crate::ide_package`]), sem o `dpkg-deb`: sai
//! igual no Linux e no Windows. Pedido do autor em 21/11: "quero poder gerar o .deb de um app no
//! windows e no linux" (antes só saía pelo WSL, e só em Rust).

use std::path::{Path, PathBuf};

use aylis_project::{Language, Project};

use crate::ide_package::{DebTar, ar, deb_version};

/// As bibliotecas que todo app do Aylis carrega no Linux (a janela, as fontes, o OpenGL).
const SYSTEM_DEPENDS: &str = "libc6, libfontconfig1, libxkbcommon0, libgl1, libegl1";

/// O nome do pacote: o do projeto em minúsculas, com `-` (o Debian não aceita `_`).
pub fn package_id(project: &Project) -> String {
    project.package_name().replace('_', "-").to_lowercase()
}

/// O runtime da linguagem: `Depends` quando todo sistema tem com esse nome, `Recommends` quando
/// o nome ou a versão variam (o app diz o que falta ao abrir, pelo `MessageDialog`).
fn runtime(language: Language) -> (Option<&'static str>, Option<&'static str>) {
    match language {
        Language::Python => (Some("python3"), None),
        Language::TypeScript => (None, Some("nodejs")),
        Language::CSharp => (None, Some("dotnet-runtime-8.0 | dotnet-runtime-10.0")),
        Language::Java => (None, Some("default-jre-headless | openjdk-17-jre-headless")),
        _ => (None, None),
    }
}

/// O `control` do pacote.
pub fn control(project: &Project, installed_kb: u64) -> String {
    let (depends, recommends) = runtime(project.language);
    let maintainer = if project.publish.publisher.is_empty() { project.app_name().to_string() } else { project.publish.publisher.clone() };
    let mut text = format!(
        "Package: {}\nVersion: {}\nArchitecture: amd64\nMaintainer: {maintainer}\nInstalled-Size: {installed_kb}\nDepends: {SYSTEM_DEPENDS}{}\n",
        package_id(project),
        deb_version(&project.version()),
        depends.map(|d| format!(", {d}")).unwrap_or_default()
    );
    if let Some(r) = recommends {
        text.push_str(&format!("Recommends: {r}\n"));
    }
    text.push_str(&format!("Section: misc\nPriority: optional\nDescription: {}\n Feito com o Aylis.\n", project.app_name()));
    text
}

/// O `.desktop` do menu: abre o app; com `UrlScheme` ligados, recebe o endereço (`%u`) e
/// declara os prefixos (`x-scheme-handler`), como o do WSL (decidido em 14/11).
pub fn desktop(project: &Project, exec: &str, icon: bool) -> String {
    let schemes = crate::publish::url_schemes(project);
    let (arg, mime) = if schemes.is_empty() {
        (String::new(), String::new())
    } else {
        (" %u".to_string(), format!("MimeType={}\n", schemes.iter().map(|(s, _)| format!("x-scheme-handler/{s};")).collect::<String>()))
    };
    let icon = if icon { format!("Icon={}\n", package_id(project)) } else { String::new() };
    format!("[Desktop Entry]\nType=Application\nName={}\nExec={exec}{arg}\n{icon}Terminal=false\nCategories=Utility;\n{mime}", project.app_name())
}

/// O `postinst` e o `postrm`: o menu e os ícones do sistema passam a ver (ou deixam de ver) o app.
const HOOK: &str = "#!/bin/sh\nset -e\nupdate-desktop-database -q /usr/share/applications >/dev/null 2>&1 || true\ngtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || true\n";

/// Executável no pacote: o principal, os binários (ELF) e os scripts (`#!`).
fn executable(rel: &str, main: &str, data: &[u8]) -> bool {
    rel == main || data.starts_with(b"\x7fELF") || data.starts_with(b"#!")
}

/// Monta `<out>/<app>_<versão>_amd64.deb` com `files` (o caminho dentro do app, com `/`, e o
/// arquivo) em `/opt/<app>`; `main` é o executável que o menu e o `/usr/bin` abrem.
pub fn build(project: &Project, files: &[(String, PathBuf)], main: &str, out: &Path) -> Result<PathBuf, String> {
    let id = package_id(project);
    let mut data = DebTar::new();
    for (rel, path) in files {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mode = if executable(rel, main, &bytes) { 0o755 } else { 0o644 };
        data.file(&format!("opt/{id}/{rel}"), &bytes, mode)?;
    }
    data.symlink(&format!("usr/bin/{id}"), &format!("/opt/{id}/{main}"))?;
    // O ícone do projeto (`.aylis/app-icon.ico`), em PNG para o menu.
    let icon = std::fs::read(project.dir().join(aylis_build::APP_ICON)).ok().and_then(|ico| crate::icons::from_ico(&ico).ok());
    if let Some(icon) = &icon {
        data.file(&format!("usr/share/icons/hicolor/256x256/apps/{id}.png"), &crate::icons::png(&icon.render(256)), 0o644)?;
    }
    data.file(&format!("usr/share/applications/{id}.desktop"), desktop(project, &format!("/opt/{id}/{main}"), icon.is_some()).as_bytes(), 0o644)?;
    let installed_kb = data.bytes.div_ceil(1024);
    let data = data.finish()?;

    let mut control_tar = DebTar::new();
    control_tar.file("control", control(project, installed_kb).as_bytes(), 0o644)?;
    control_tar.file("postinst", HOOK.as_bytes(), 0o755)?;
    control_tar.file("postrm", HOOK.as_bytes(), 0o755)?;
    let control_tar = control_tar.finish()?;

    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let deb = out.join(format!("{id}_{}_amd64.deb", deb_version(&project.version())));
    std::fs::write(&deb, ar(&[("debian-binary", b"2.0\n"), ("control.tar.gz", &control_tar), ("data.tar.gz", &data)])).map_err(|e| format!("{}: {e}", deb.display()))?;
    Ok(deb)
}

/// Os arquivos de uma pasta (recursivo), com o caminho relativo em `/`.
pub fn files_in(dir: &Path) -> Result<Vec<(String, PathBuf)>, String> {
    let mut out = Vec::new();
    crate::ide_package::files_of(dir, "", &mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(language: Language) -> (PathBuf, Project) {
        let base = std::env::temp_dir().join(format!("aylis_app_deb_{}_{language:?}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let p = aylis_project::create(&base, "Minha_Loja", language).unwrap();
        (base, p)
    }

    /// O `.deb` de um app: em `/opt/<app>`, o principal executável, o atalho no `/usr/bin` e o
    /// `.desktop`; o runtime do Python nas dependências e o do Node como recomendado.
    #[test]
    fn the_app_deb_installs_in_opt_with_the_menu_entry() {
        let (base, p) = project(Language::Python);
        let app = base.join("pub");
        std::fs::create_dir_all(app.join("backend")).unwrap();
        std::fs::write(app.join("minha-loja"), b"\x7fELF fake").unwrap();
        std::fs::write(app.join("backend/main.py"), b"print('oi')\n").unwrap();
        let deb = build(&p, &files_in(&app).unwrap(), "minha-loja", &base.join("out")).unwrap();
        assert_eq!(deb.file_name().unwrap(), "minha-loja_1.0.0_amd64.deb");
        let bytes = std::fs::read(&deb).unwrap();
        assert!(bytes.starts_with(b"!<arch>\n") && bytes.windows(14).any(|w| w == b"data.tar.gz   "));
        let ctl = control(&p, 10);
        assert!(ctl.contains("Package: minha-loja\n") && ctl.contains("Depends: libc6, libfontconfig1, libxkbcommon0, libgl1, libegl1, python3\n"), "{ctl}");
        let (ts_base, ts) = project(Language::TypeScript);
        assert!(control(&ts, 1).contains("Recommends: nodejs\n"));
        let entry = desktop(&p, "/opt/minha-loja/minha-loja", false);
        assert!(entry.contains("Exec=/opt/minha-loja/minha-loja\n") && !entry.contains("Icon="), "{entry}");
        assert!(executable("backend/app", "x", b"\x7fELF") && !executable("backend/main.py", "x", b"print"));
        let _ = std::fs::remove_dir_all(&base);
        let _ = std::fs::remove_dir_all(&ts_base);
    }
}
