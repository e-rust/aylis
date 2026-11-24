//! Os pacotes de um app publicado para Linux. Uma árvore só (o app em `/opt/<app>`, o atalho em
//! `/usr/bin/<app>`, o `.desktop` no menu e o ícone em todos os tamanhos do tema `hicolor`), da
//! qual saem os quatro formatos de `publish.linux.installer`:
//!
//! - `deb` (Debian, Ubuntu, Mint): montado em Rust (o `ar` e os `.tar.gz` do
//!   [`crate::ide_package`]), sem o `dpkg-deb`: sai igual no Linux e no Windows (pedido do autor
//!   em 21/11: "quero poder gerar o .deb de um app no windows e no linux");
//! - `rpm` (Fedora, RHEL, openSUSE): pelo `rpmbuild`, só no Linux;
//! - `appimage`: um arquivo que roda em qualquer distribuição, pelo `appimagetool`, só no Linux;
//! - `flatpak`: o pacote do Flathub, pelo `flatpak-builder`, só no Linux.
//!
//! As opções (o nome do pacote, as categorias, a descrição, o mantenedor, as dependências, os
//! tipos de arquivo) vêm de `publish.linux` ([`aylis_project::LinuxOptions`]), e o
//! `StartupWMClass` (o painel junta a janela aberta ao atalho) é o mesmo nome que a janela
//! declara.

use std::path::{Path, PathBuf};

use aylis_project::{Language, LinuxInstaller, Project};

use crate::ide_package::{DebTar, ar, deb_version, rpm_version};

/// As bibliotecas que todo app do Aylis carrega no Linux (a janela, as fontes, o OpenGL).
const SYSTEM_DEPENDS: &str = "libc6, libfontconfig1, libxkbcommon0, libgl1, libegl1";
/// As mesmas, com os nomes do Fedora.
const SYSTEM_REQUIRES: &str = "glibc fontconfig libxkbcommon libglvnd-glx libglvnd-egl";
/// Os tamanhos do ícone no tema `hicolor` (o menu, o painel, o alternador de janelas).
const ICON_SIZES: [u32; 8] = [16, 24, 32, 48, 64, 128, 256, 512];

/// O nome do pacote ([`Project::linux_package_id`]).
pub fn package_id(project: &Project) -> String {
    project.linux_package_id()
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

/// O app toca som (a feature `media`): o `cpal` liga com a `libasound.so.2`.
fn uses_audio(project: &Project) -> bool {
    aylis_project::codegen::aylis_features(project).contains(&"media")
}

fn maintainer(project: &Project) -> String {
    let linux = &project.publish.linux;
    [linux.maintainer.trim(), project.publish.publisher.trim()].into_iter().find(|m| !m.is_empty()).map_or_else(|| project.app_name().to_string(), str::to_string)
}

/// A do Linux (`publish.linux.description`); sem ela, a do projeto (`publish.description`, a
/// mesma do `.exe`).
fn description(project: &Project) -> String {
    let d = [project.publish.linux.description.trim(), project.publish.description.trim()].into_iter().find(|d| !d.is_empty()).unwrap_or("");
    if d.is_empty() { "Feito com o Aylis.".into() } else { d.to_string() }
}

/// O `control` do `.deb`.
pub fn control(project: &Project, installed_kb: u64) -> String {
    let (depends, recommends) = runtime(project.language);
    let mut all = vec![SYSTEM_DEPENDS.to_string()];
    // Sem a ALSA o app não toca som (o relatório do player de música, 21/11).
    if uses_audio(project) {
        all.push("libasound2".into());
    }
    all.extend(depends.map(str::to_string));
    all.extend(project.publish.linux.depends.iter().map(|d| d.trim().to_string()).filter(|d| !d.is_empty()));
    let mut text = format!(
        "Package: {}\nVersion: {}\nArchitecture: amd64\nMaintainer: {}\nInstalled-Size: {installed_kb}\nDepends: {}\n",
        package_id(project),
        deb_version(&project.version()),
        maintainer(project),
        all.join(", ")
    );
    if let Some(r) = recommends {
        text.push_str(&format!("Recommends: {r}\n"));
    }
    // A descrição longa: cada linha com um espaço na frente, e a vazia com " ." (o formato do
    // Debian).
    let long: String = description(project).lines().map(|l| if l.trim().is_empty() { " .\n".to_string() } else { format!(" {l}\n") }).collect();
    text.push_str(&format!("Section: misc\nPriority: optional\nDescription: {}\n{long}", project.app_name()));
    text
}

/// As categorias do menu, com o `;` do fim.
fn categories(project: &Project) -> String {
    let list: Vec<&str> = project.publish.linux.categories.iter().map(|c| c.trim().trim_end_matches(';')).filter(|c| !c.is_empty()).collect();
    if list.is_empty() { "Utility;".into() } else { list.iter().map(|c| format!("{c};")).collect() }
}

/// O `.desktop` do menu: abre o app; com `UrlScheme` ligados, recebe o endereço (`%u`) e
/// declara os prefixos (`x-scheme-handler`), como o do WSL (decidido em 14/11); com tipos de
/// arquivo (`mime_types`), recebe os arquivos (`%F`) e aparece no "Abrir com".
pub fn desktop(project: &Project, exec: &str, icon: Option<&str>) -> String {
    let schemes = crate::publish::url_schemes(project);
    let types: Vec<String> = project.publish.linux.mime_types.iter().map(|m| m.trim().trim_end_matches(';').to_string()).filter(|m| !m.is_empty()).collect();
    let arg = if !schemes.is_empty() {
        " %u"
    } else if !types.is_empty() {
        " %F"
    } else {
        ""
    };
    let mime: String = schemes.iter().map(|(s, _)| format!("x-scheme-handler/{s};")).chain(types.iter().map(|t| format!("{t};"))).collect();
    let mime = if mime.is_empty() { String::new() } else { format!("MimeType={mime}\n") };
    let icon = icon.map(|i| format!("Icon={i}\n")).unwrap_or_default();
    format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\nExec={exec}{arg}\n{icon}Terminal=false\nCategories={}\nStartupWMClass={}\n{mime}",
        project.app_name(),
        description(project).lines().next().unwrap_or_default(),
        categories(project),
        package_id(project),
    )
}

/// O `postinst` e o `postrm`: o menu e os ícones do sistema passam a ver (ou deixam de ver) o app.
const HOOK: &str = "#!/bin/sh\nset -e\nupdate-desktop-database -q /usr/share/applications >/dev/null 2>&1 || true\ngtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || true\n";

/// Executável no pacote: o principal, os binários (ELF) e os scripts (`#!`).
fn executable(rel: &str, main: &str, data: &[u8]) -> bool {
    rel == main || data.starts_with(b"\x7fELF") || data.starts_with(b"#!")
}

/// Um item da árvore do pacote.
enum Item {
    File(Vec<u8>, u32),
    Link(String),
}

/// A árvore de arquivos do app instalado (os caminhos sem `/` no começo).
struct Tree(Vec<(String, Item)>);

impl Tree {
    fn file(&mut self, path: impl Into<String>, data: Vec<u8>, mode: u32) {
        self.0.push((path.into(), Item::File(data, mode)));
    }

    /// Grava a árvore numa pasta (o `.rpm`, o AppImage e o Flatpak montam a partir dela).
    fn write_to(&self, root: &Path) -> Result<(), String> {
        for (path, item) in &self.0 {
            let dest = root.join(path);
            if let Some(dir) = dest.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            match item {
                Item::File(data, mode) => {
                    std::fs::write(&dest, data).map_err(|e| format!("{}: {e}", dest.display()))?;
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(*mode)).map_err(|e| format!("{}: {e}", dest.display()))?;
                    }
                    #[cfg(not(unix))]
                    let _ = mode;
                }
                Item::Link(target) => {
                    #[cfg(unix)]
                    std::os::unix::fs::symlink(target, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
                    #[cfg(not(unix))]
                    let _ = target;
                }
            }
        }
        Ok(())
    }
}

/// O ícone do projeto em PNG nos tamanhos pedidos e, quando o ícone do projeto é SVG, o SVG
/// (o `scalable` do tema).
fn icons(project: &Project) -> (Vec<(u32, Vec<u8>)>, Option<Vec<u8>>) {
    let icon = std::fs::read(project.dir().join(aylis_build::APP_ICON)).ok().and_then(|ico| crate::icons::from_ico(&ico).ok());
    // O PNG ou SVG original, quando há, desenha melhor os tamanhos grandes que o `.ico`.
    let source = Some(project.publish.icon.trim()).filter(|i| !i.is_empty()).map(|i| project.dir().join(i));
    let original = source.as_ref().and_then(|p| crate::icons::Icon::open(p).ok());
    let svg = source.filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("svg"))).and_then(|p| std::fs::read(p).ok());
    let Some(icon) = original.or(icon) else { return (Vec::new(), svg) };
    (ICON_SIZES.iter().map(|&s| (s, crate::icons::png(&icon.render(s)))).collect(), svg)
}

/// A árvore do app instalado: `files` (o caminho dentro do app, com `/`, e o arquivo) em
/// `/opt/<app>`, o atalho em `/usr/bin`, o `.desktop` e os ícones.
fn tree(project: &Project, files: &[(String, PathBuf)], main: &str) -> Result<Tree, String> {
    let id = package_id(project);
    let mut t = Tree(Vec::new());
    for (rel, path) in files {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mode = if executable(rel, main, &bytes) { 0o755 } else { 0o644 };
        t.file(format!("opt/{id}/{rel}"), bytes, mode);
    }
    t.0.push((format!("usr/bin/{id}"), Item::Link(format!("/opt/{id}/{main}"))));
    let (pngs, svg) = icons(project);
    for (size, png) in &pngs {
        t.file(format!("usr/share/icons/hicolor/{size}x{size}/apps/{id}.png"), png.clone(), 0o644);
    }
    if let Some(svg) = svg.clone() {
        t.file(format!("usr/share/icons/hicolor/scalable/apps/{id}.svg"), svg, 0o644);
    }
    // O menu abre pelo atalho do `/usr/bin` (o nome do pacote, sem espaços): o executável em
    // `/opt` tem o nome do app, e no `Exec` os espaços separam argumentos (o relatório do player
    // de música, 21/11: o atalho não abria).
    let has_icon = !pngs.is_empty() || svg.is_some();
    t.file(format!("usr/share/applications/{id}.desktop"), desktop(project, &format!("/usr/bin/{id}"), has_icon.then_some(id.as_str())).into_bytes(), 0o644);
    Ok(t)
}

/// Monta o pacote do formato de `publish.linux.installer` em `out` e devolve o caminho dele.
pub fn build(project: &Project, files: &[(String, PathBuf)], main: &str, out: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let t = tree(project, files, main)?;
    match project.publish.linux.kind() {
        LinuxInstaller::Deb => deb(project, &t, out),
        LinuxInstaller::Rpm => rpm(project, &t, out),
        LinuxInstaller::AppImage => appimage(project, &t, main, out),
        LinuxInstaller::Flatpak => flatpak(project, &t, main, out),
    }
}

/// O `.deb` (no Linux e no Windows).
fn deb(project: &Project, t: &Tree, out: &Path) -> Result<PathBuf, String> {
    let id = package_id(project);
    let mut data = DebTar::new();
    for (path, item) in &t.0 {
        match item {
            Item::File(bytes, mode) => data.file(path, bytes, *mode)?,
            Item::Link(target) => data.symlink(path, target)?,
        }
    }
    let installed_kb = data.bytes.div_ceil(1024);
    let data = data.finish()?;
    let mut control_tar = DebTar::new();
    control_tar.file("control", control(project, installed_kb).as_bytes(), 0o644)?;
    control_tar.file("postinst", HOOK.as_bytes(), 0o755)?;
    control_tar.file("postrm", HOOK.as_bytes(), 0o755)?;
    let control_tar = control_tar.finish()?;
    let deb = out.join(format!("{id}_{}_amd64.deb", deb_version(&project.version())));
    std::fs::write(&deb, ar(&[("debian-binary", b"2.0\n"), ("control.tar.gz", &control_tar), ("data.tar.gz", &data)])).map_err(|e| format!("{}: {e}", deb.display()))?;
    Ok(deb)
}

/// O `.spec` do `.rpm`: a árvore de `stage` como está, com os nomes do Fedora nas dependências
/// e os arquivos de `icons` (os caminhos dos ícones na árvore; sem ícone, nenhum).
pub fn rpm_spec(project: &Project, stage: &Path, icons: &[String]) -> String {
    let id = package_id(project);
    let mut requires = vec![SYSTEM_REQUIRES.to_string()];
    if uses_audio(project) {
        requires.push("alsa-lib".into());
    }
    if project.language == Language::Python {
        requires.push("python3".into());
    }
    requires.extend(project.publish.linux.rpm_depends.iter().map(|d| d.trim().to_string()).filter(|d| !d.is_empty()));
    let hook = "update-desktop-database -q /usr/share/applications >/dev/null 2>&1 || :\ngtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || :";
    format!(
        "Name: {id}\nVersion: {}\nRelease: 1\nSummary: {}\nLicense: Proprietary\nPackager: {}\nBuildArch: x86_64\nAutoReqProv: no\nRequires: {}\n\
         %global debug_package %{{nil}}\n%global __os_install_post %{{nil}}\n%global _build_id_links none\n\n\
         %description\n{}\n\n\
         %install\nmkdir -p %{{buildroot}}\ncp -a '{}'/. %{{buildroot}}/\n\n\
         %post\n{hook}\n\n%postun\n{hook}\n\n\
         %files\n/opt/{id}\n/usr/bin/{id}\n/usr/share/applications/{id}.desktop\n{}",
        rpm_version(&project.version()),
        project.app_name(),
        maintainer(project),
        requires.join(" "),
        description(project),
        stage.display(),
        icons.iter().map(|i| format!("/{i}\n")).collect::<String>()
    )
}

/// Uma pasta de trabalho vazia dentro de `out`.
fn work_dir(out: &Path, name: &str) -> Result<PathBuf, String> {
    let work = out.join(name);
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    Ok(work)
}

/// Roda uma ferramenta do sistema (o `rpmbuild`, o `appimagetool`, o `flatpak-builder`), com o
/// passo a passo de instalar quando ela não existe.
fn tool(program: &str, args: &[&std::ffi::OsStr], env: &[(&str, &str)], install: &str) -> Result<(), String> {
    let status = std::process::Command::new(program).args(args).envs(env.iter().copied()).status().map_err(|e| format!("{program}: {e} ({install})"))?;
    if status.success() { Ok(()) } else { Err(format!("o {program} falhou (veja acima)")) }
}

fn only_linux(what: &str) -> Result<(), String> {
    if cfg!(target_os = "linux") { Ok(()) } else { Err(format!("o {what} é montado no Linux (publique de um Linux, ou pelo WSL)")) }
}

/// O `.rpm`, pelo `rpmbuild` (só no Linux; no Debian e no Ubuntu, o pacote `rpm` o traz).
fn rpm(project: &Project, t: &Tree, out: &Path) -> Result<PathBuf, String> {
    only_linux(".rpm")?;
    let id = package_id(project);
    let work = work_dir(out, "rpm-build")?;
    let stage = work.join("root");
    t.write_to(&stage)?;
    let spec = work.join(format!("{id}.spec"));
    let icons: Vec<String> = t.0.iter().map(|(p, _)| p.clone()).filter(|p| p.starts_with("usr/share/icons/")).collect();
    std::fs::write(&spec, rpm_spec(project, &stage, &icons)).map_err(|e| format!("{}: {e}", spec.display()))?;
    let (top, rpmdir) = (format!("_topdir {}", work.display()), format!("_rpmdir {}", work.display()));
    tool(
        "rpmbuild",
        &["-bb".as_ref(), "--define".as_ref(), top.as_ref(), "--define".as_ref(), rpmdir.as_ref(), spec.as_os_str()],
        &[],
        "instale o pacote rpm: sudo dnf install rpm-build, ou sudo apt install rpm",
    )?;
    let name = format!("{id}-{}-1.x86_64.rpm", rpm_version(&project.version()));
    let built = work.join("x86_64").join(&name);
    let rpm = out.join(&name);
    std::fs::rename(&built, &rpm).map_err(|e| format!("{}: {e}", built.display()))?;
    let _ = std::fs::remove_dir_all(&work);
    Ok(rpm)
}

/// O `AppRun` do AppImage: abre o app de dentro da imagem montada.
fn app_run(id: &str, main: &str) -> String {
    format!("#!/bin/sh\nHERE=\"$(dirname \"$(readlink -f \"$0\")\")\"\nexec \"$HERE/opt/{id}/{main}\" \"$@\"\n")
}

/// O AppImage, pelo `appimagetool` (no PATH, ou em `AYLIS_APPIMAGETOOL`): a mesma árvore numa
/// AppDir, com o `AppRun`, o `.desktop` e o ícone na raiz.
fn appimage(project: &Project, t: &Tree, main: &str, out: &Path) -> Result<PathBuf, String> {
    only_linux("AppImage")?;
    let id = package_id(project);
    let work = work_dir(out, "appimage-build")?;
    let dir = work.join(format!("{id}.AppDir"));
    t.write_to(&dir)?;
    // Na raiz: o `AppRun`, o `.desktop` (o `Exec` é o nome, que o AppImage acha pelo AppRun) e
    // o ícone.
    let run = dir.join("AppRun");
    std::fs::write(&run, app_run(&id, main)).map_err(|e| format!("{}: {e}", run.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&run, std::fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;
    }
    let (pngs, _) = icons(project);
    let has_icon = pngs.iter().any(|(s, png)| *s == 256 && std::fs::write(dir.join(format!("{id}.png")), png).is_ok());
    std::fs::write(dir.join(format!("{id}.desktop")), desktop(project, &id, has_icon.then_some(id.as_str()))).map_err(|e| e.to_string())?;
    let image = out.join(format!("{}-{}-x86_64.AppImage", file_name(project.app_name()), project.version()));
    let program = std::env::var("AYLIS_APPIMAGETOOL").unwrap_or_else(|_| "appimagetool".into());
    tool(
        &program,
        &[dir.as_os_str(), image.as_os_str()],
        &[("ARCH", "x86_64")],
        "baixe o appimagetool em https://github.com/AppImage/appimagetool/releases, dê chmod +x e ponha no PATH (ou em AYLIS_APPIMAGETOOL)",
    )?;
    let _ = std::fs::remove_dir_all(&work);
    Ok(image)
}

/// O nome do app para um arquivo: sem os caracteres que atrapalham num caminho.
fn file_name(name: &str) -> String {
    name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '-' }).collect()
}

/// O identificador do Flatpak (`com.empresa.App`): o `identifier` do projeto.
pub fn flatpak_id(project: &Project) -> String {
    project.identifier()
}

/// O manifesto do `flatpak-builder`: a árvore de `stage` em `/app`, com a janela (Wayland e
/// X11), a placa de vídeo, o som e as permissões de `flatpak_permissions`. Sem rede nem pastas
/// do usuário por padrão (o que mexe em segurança nasce fechado); o diálogo de arquivos do
/// sistema chega aos arquivos pelo portal.
pub fn flatpak_manifest(project: &Project, stage: &str) -> String {
    let mut args = vec!["--socket=wayland", "--socket=fallback-x11", "--share=ipc", "--device=dri"];
    if uses_audio(project) {
        args.push("--socket=pulseaudio");
    }
    let mut args: Vec<String> = args.into_iter().map(str::to_string).collect();
    args.extend(project.publish.linux.flatpak_permissions.iter().map(|p| p.trim().to_string()).filter(|p| !p.is_empty()));
    let manifest = serde_json::json!({
        "app-id": flatpak_id(project),
        "runtime": "org.freedesktop.Platform",
        "runtime-version": "24.08",
        "sdk": "org.freedesktop.Sdk",
        "command": package_id(project),
        "finish-args": args,
        "modules": [{
            "name": package_id(project),
            "buildsystem": "simple",
            "build-commands": ["cp -a . /app/"],
            "sources": [{ "type": "dir", "path": stage }],
        }],
    });
    serde_json::to_string_pretty(&manifest).unwrap_or_default()
}

/// O Flatpak, pelo `flatpak-builder` (o runtime vem do Flathub na primeira vez): a árvore em
/// `/app` (o app em `/app/lib/<app>`, o atalho em `/app/bin`, o `.desktop` e os ícones com o
/// nome do identificador, como o Flatpak pede) e um `.flatpak` instalável com `flatpak install`.
fn flatpak(project: &Project, t: &Tree, main: &str, out: &Path) -> Result<PathBuf, String> {
    only_linux("Flatpak")?;
    let (id, app_id) = (package_id(project), flatpak_id(project));
    let work = work_dir(out, "flatpak-build")?;
    let stage = work.join("stage");
    for (path, item) in &t.0 {
        let Item::File(data, mode) = item else { continue };
        let dest = if let Some(rest) = path.strip_prefix(&format!("opt/{id}/")) {
            format!("lib/{id}/{rest}")
        } else if let Some(rest) = path.strip_prefix("usr/share/") {
            format!("share/{}", rest.replace(&format!("/{id}."), &format!("/{app_id}.")))
        } else {
            continue;
        };
        let data = if dest.ends_with(".desktop") {
            String::from_utf8_lossy(data).replace(&format!("Exec=/usr/bin/{id}"), &format!("Exec={id}")).replace(&format!("Icon={id}\n"), &format!("Icon={app_id}\n")).into_bytes()
        } else {
            data.clone()
        };
        Tree(vec![(dest, Item::File(data, *mode))]).write_to(&stage)?;
    }
    Tree(vec![(format!("bin/{id}"), Item::File(format!("#!/bin/sh\nexec /app/lib/{id}/{main} \"$@\"\n").into_bytes(), 0o755))]).write_to(&stage)?;
    let manifest = work.join(format!("{app_id}.json"));
    std::fs::write(&manifest, flatpak_manifest(project, &stage.display().to_string())).map_err(|e| e.to_string())?;
    let (build, repo) = (work.join("build"), work.join("repo"));
    let repo_arg = format!("--repo={}", repo.display());
    let install = "instale o flatpak e o flatpak-builder (sudo dnf install flatpak flatpak-builder, ou sudo apt install flatpak flatpak-builder) e o Flathub: flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo";
    tool(
        "flatpak-builder",
        &["--user".as_ref(), "--force-clean".as_ref(), "--install-deps-from=flathub".as_ref(), repo_arg.as_ref(), build.as_os_str(), manifest.as_os_str()],
        &[],
        install,
    )?;
    let bundle = out.join(format!("{id}.flatpak"));
    tool("flatpak", &["build-bundle".as_ref(), repo.as_os_str(), bundle.as_os_str(), app_id.as_ref()], &[], install)?;
    let _ = std::fs::remove_dir_all(&work);
    Ok(bundle)
}

/// Publicar sem instalador (`menu_shortcut`): o atalho no menu deste usuário
/// (`~/.local/share/applications`) e o ícone (`~/.local/share/icons`), apontando para o
/// executável publicado. Devolve o `.desktop` gravado.
pub fn user_shortcut(project: &Project, exe: &Path) -> Result<PathBuf, String> {
    let home = std::env::var_os("HOME").map(PathBuf::from).ok_or("sem HOME: não sei onde fica o menu deste usuário")?;
    let share = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| home.join(".local/share"));
    let id = package_id(project);
    let (pngs, svg) = icons(project);
    for (size, png) in &pngs {
        Tree(vec![(format!("icons/hicolor/{size}x{size}/apps/{id}.png"), Item::File(png.clone(), 0o644))]).write_to(&share)?;
    }
    if let Some(svg) = svg.clone() {
        Tree(vec![(format!("icons/hicolor/scalable/apps/{id}.svg"), Item::File(svg, 0o644))]).write_to(&share)?;
    }
    // O caminho entre aspas: a pasta publicada pode ter espaços.
    let exec = format!("\"{}\"", exe.display().to_string().replace('"', "\\\""));
    let has_icon = !pngs.is_empty() || svg.is_some();
    let path = share.join("applications").join(format!("{id}.desktop"));
    Tree(vec![(format!("applications/{id}.desktop"), Item::File(desktop(project, &exec, has_icon.then_some(id.as_str())).into_bytes(), 0o644))]).write_to(&share)?;
    let _ = std::process::Command::new("update-desktop-database").arg("-q").arg(share.join("applications")).status();
    Ok(path)
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

    fn project(language: Language, tag: &str) -> (PathBuf, Project) {
        let base = std::env::temp_dir().join(format!("aylis_app_linux_{}_{language:?}_{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let p = aylis_project::create(&base, "Minha_Loja", language).unwrap();
        (base, p)
    }

    /// Um arquivo de dentro do `data.tar.gz` de um `.deb` (`None`: não está).
    pub(crate) fn deb_file(deb: &Path, wanted: &str) -> Option<Vec<u8>> {
        let bytes = std::fs::read(deb).unwrap();
        let mut at = 8;
        while at + 60 <= bytes.len() {
            let name = String::from_utf8_lossy(&bytes[at..at + 16]).trim().to_string();
            let size: usize = String::from_utf8_lossy(&bytes[at + 48..at + 58]).trim().parse().unwrap();
            if name == "data.tar.gz" {
                let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(&bytes[at + 60..at + 60 + size]));
                for e in archive.entries().unwrap() {
                    let mut e = e.unwrap();
                    if e.path().unwrap().to_string_lossy().trim_start_matches("./") == wanted {
                        let mut data = Vec::new();
                        std::io::Read::read_to_end(&mut e, &mut data).unwrap();
                        return Some(data);
                    }
                }
            }
            at += 60 + size + size % 2;
        }
        None
    }

    /// O `.deb` de um app: em `/opt/<app>`, o principal executável, o atalho no `/usr/bin` e o
    /// `.desktop`; o runtime do Python nas dependências e o do Node como recomendado.
    #[test]
    fn the_app_deb_installs_in_opt_with_the_menu_entry() {
        let (base, p) = project(Language::Python, "py");
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
        assert!(ctl.contains("Description: Minha_Loja\n Feito com o Aylis.\n"), "{ctl}");
        let (ts_base, ts) = project(Language::TypeScript, "ts");
        assert!(control(&ts, 1).contains("Recommends: nodejs\n"));
        let entry = desktop(&p, "/usr/bin/minha-loja", None);
        assert!(entry.contains("Exec=/usr/bin/minha-loja\n") && !entry.contains("Icon=") && entry.contains("Categories=Utility;\n") && entry.contains("StartupWMClass=minha-loja\n"), "{entry}");
        assert!(executable("backend/app", "x", b"\x7fELF") && !executable("backend/main.py", "x", b"print"));
        let _ = std::fs::remove_dir_all(&base);
        let _ = std::fs::remove_dir_all(&ts_base);
    }

    /// O relatório do player de música (21/11): com espaço no nome do app, o menu abre pelo
    /// `/usr/bin/<id>`; com mídia, a ALSA; e as opções de `publish.linux` no pacote.
    #[test]
    fn the_linux_options_reach_the_package() {
        let (base, mut p) = project(Language::Rust, "options");
        p.features = vec!["media".into()];
        let linux = &mut p.publish.linux;
        linux.package = "audio-player".into();
        linux.categories = vec!["AudioVideo".into(), "Audio".into(), "Player;".into()];
        linux.description = "Um player de música.\n\nToca MP3 e FLAC.".into();
        linux.maintainer = "Fulano <fulano@exemplo.com>".into();
        linux.depends = vec!["ffmpeg".into()];
        linux.rpm_depends = vec!["ffmpeg-free".into()];
        linux.mime_types = vec!["audio/mpeg".into(), "audio/flac".into()];
        let ctl = control(&p, 1);
        assert!(ctl.contains("Package: audio-player\n") && ctl.contains(", libasound2, ffmpeg\n") && ctl.contains("Maintainer: Fulano <fulano@exemplo.com>\n"), "{ctl}");
        assert!(ctl.contains("Description: Minha_Loja\n Um player de música.\n .\n Toca MP3 e FLAC.\n"), "{ctl}");
        let entry = desktop(&p, "/usr/bin/audio-player", Some("audio-player"));
        assert!(entry.contains("Categories=AudioVideo;Audio;Player;\n") && entry.contains("Exec=/usr/bin/audio-player %F\n") && entry.contains("MimeType=audio/mpeg;audio/flac;\n"), "{entry}");
        assert!(entry.contains("StartupWMClass=audio-player\n") && entry.contains("Comment=Um player de música.\n"), "{entry}");
        let spec = rpm_spec(&p, Path::new("/tmp/stage"), &["usr/share/icons/hicolor/48x48/apps/audio-player.png".into()]);
        assert!(spec.contains("/usr/share/applications/audio-player.desktop\n/usr/share/icons/hicolor/48x48/apps/audio-player.png\n"), "{spec}");
        assert!(spec.contains("Name: audio-player\n") && spec.contains("Requires: glibc fontconfig libxkbcommon libglvnd-glx libglvnd-egl alsa-lib ffmpeg-free\n"), "{spec}");
        let manifest = flatpak_manifest(&p, "/tmp/stage");
        assert!(manifest.contains("\"--socket=pulseaudio\"") && manifest.contains("\"command\": \"audio-player\"") && !manifest.contains("--share=network"), "{manifest}");

        let app = base.join("pub");
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(app.join("Audio Player Free"), b"\x7fELF fake").unwrap();
        let deb = build(&p, &files_in(&app).unwrap(), "Audio Player Free", &base.join("out")).unwrap();
        let entry = String::from_utf8(deb_file(&deb, "usr/share/applications/audio-player.desktop").unwrap()).unwrap();
        assert!(entry.contains("Exec=/usr/bin/audio-player %F\n"), "{entry}");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// Com o ícone do projeto, os tamanhos do `hicolor` no pacote.
    #[test]
    fn the_icon_goes_in_every_size() {
        let (base, p) = project(Language::Rust, "icons");
        let img = ::image::RgbaImage::from_pixel(256, 256, ::image::Rgba([200, 30, 30, 255]));
        let dir = p.dir().join(aylis_build::APP_ICON);
        std::fs::create_dir_all(dir.parent().unwrap()).unwrap();
        std::fs::write(&dir, crate::icons::ico(&crate::icons::Icon::from_bytes(&crate::icons::png(&img), false).unwrap(), &[256])).unwrap();
        let app = base.join("pub");
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(app.join("minha-loja"), b"\x7fELF fake").unwrap();
        let deb = build(&p, &files_in(&app).unwrap(), "minha-loja", &base.join("out")).unwrap();
        for size in ICON_SIZES {
            assert!(deb_file(&deb, &format!("usr/share/icons/hicolor/{size}x{size}/apps/minha-loja.png")).is_some(), "falta {size}");
        }
        assert!(String::from_utf8(deb_file(&deb, "usr/share/applications/minha-loja.desktop").unwrap()).unwrap().contains("Icon=minha-loja\n"));
        let _ = std::fs::remove_dir_all(&base);
    }

    /// O `.desktop` declara os prefixos dos `UrlScheme` ligados (`x-scheme-handler`) e recebe o
    /// endereço (`%u`), decidido em 14/11.
    #[test]
    fn the_menu_entry_declares_the_url_schemes() {
        let (base, p) = project(Language::Rust, "schemes");
        let plain = desktop(&p, "/usr/bin/minha-loja", None);
        assert!(!plain.contains("MimeType") && !plain.contains("%u"), "sem UrlScheme, nada de prefixo");
        std::fs::write(
            p.dir().join("forms").join("links.json"),
            r#"{ "root": { "class": "Form", "children": [
                { "class": "UrlScheme", "name": "a", "scheme": "Pedidos", "active": true },
                { "class": "UrlScheme", "name": "b", "scheme": "desligado", "active": false } ] } }"#,
        )
        .unwrap();
        let entry = desktop(&p, "/usr/bin/minha-loja", None);
        assert!(entry.contains("Exec=/usr/bin/minha-loja %u\n") && entry.contains("MimeType=x-scheme-handler/pedidos;\n"), "{entry}");
        assert!(!entry.contains("desligado"));
        let _ = std::fs::remove_dir_all(&base);
    }

    /// O `.rpm` de verdade, pelo `rpmbuild` (só no Linux com ele instalado; sem ele, pula).
    #[test]
    fn the_rpm_builds_where_rpmbuild_exists() {
        let has_rpmbuild = cfg!(target_os = "linux") && std::process::Command::new("rpmbuild").arg("--version").output().is_ok_and(|o| o.status.success());
        if !has_rpmbuild {
            return;
        }
        let (base, mut p) = project(Language::Rust, "rpm");
        p.publish.linux.installer = "rpm".into();
        let app = base.join("pub");
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(app.join("Minha Loja"), b"\x7fELF fake").unwrap();
        let rpm = build(&p, &files_in(&app).unwrap(), "Minha Loja", &base.join("out")).unwrap();
        assert!(rpm.ends_with("minha-loja-1.0.0-1.x86_64.rpm") && rpm.is_file(), "{}", rpm.display());
        let list = std::process::Command::new("rpm").args(["-qlp"]).arg(&rpm).output().unwrap();
        let list = String::from_utf8_lossy(&list.stdout);
        assert!(list.contains("/opt/minha-loja/Minha Loja") && list.contains("/usr/bin/minha-loja") && list.contains("/usr/share/applications/minha-loja.desktop"), "{list}");
        let _ = std::fs::remove_dir_all(&base);
    }

    /// O `AppRun` abre o app de dentro da imagem, com os argumentos.
    #[test]
    fn the_appimage_runs_the_app_from_the_mounted_image() {
        let run = app_run("audio-player", "Audio Player Free");
        assert!(run.starts_with("#!/bin/sh\n") && run.contains("exec \"$HERE/opt/audio-player/Audio Player Free\" \"$@\""), "{run}");
    }
}
