//! A IDE empacotada para Linux e macOS, com os tipos de arquivo do aylis (`.aylisproj`,
//! `.aylispkg`, `.aylisrec`, `.aylisreport`) abrindo nela com dois cliques e um ícone por tipo
//! (os mesmos do instalador do Windows, `icons::FILE_TYPES`).
//!
//! - Linux: uma pasta com `bin/` (a IDE, o `aylis-host`, o `aylis-tool` e o `aylis-migrate`
//!   compilados ao lado dela, e a pasta `aylis/` com o código do Aylis, como no instalador do
//!   Windows: os projetos compilam contra ela), o `.desktop` (com o `MimeType`), o XML dos tipos
//!   (`share/mime/packages`), os ícones (`hicolor`) e o `instalar.sh`, que copia para
//!   `~/.local/share/aylis-ide`, liga a IDE e o `aylis-tool` em `~/.local/bin` e atualiza os
//!   bancos do sistema (`update-mime-database`, `update-desktop-database`).
//! - macOS: o `Aylis IDE.app` com o `Info.plist` (`CFBundleDocumentTypes` e os tipos exportados,
//!   `UTExportedTypeDeclarations`) e um `.icns` por tipo.
//!
//! O executável vem de fora (compilado no próprio sistema: no Linux, pelo WSL ou um Docker; no
//! macOS, num Mac): `aylis-tool ide-package --target linux|macos --binary aylis-ide --out pasta`.

use std::path::{Path, PathBuf};

use image::RgbaImage;

/// O nome de cada tipo de arquivo (na ordem de `icons::FILE_TYPES`).
pub const DESCRIPTIONS: [(&str, &str); 4] = [
    ("aylisproj", "Projeto do Aylis"),
    ("aylispkg", "Pacote de componentes do Aylis"),
    ("aylisrec", "Gravação de uso da IDE do Aylis"),
    ("aylisreport", "Relato de problema do Aylis"),
];

/// O identificador do app no macOS e o prefixo dos tipos exportados.
pub const BUNDLE_ID: &str = "dev.aylis.ide";

/// O tipo MIME de uma extensão do aylis (`application/x-aylis-aylisproj`).
pub fn mime(ext: &str) -> String {
    format!("application/x-aylis-{ext}")
}

/// O `.desktop` da IDE no Linux (`exec`: o caminho do executável).
pub fn linux_desktop(exec: &str) -> String {
    let types: Vec<String> = DESCRIPTIONS.iter().map(|(e, _)| mime(e)).collect();
    format!(
        "[Desktop Entry]\nType=Application\nName=Aylis IDE\nComment=Interfaces visuais em Rust e nas linguagens do Aylis, com designer\nExec=\"{exec}\" %f\nIcon=aylis-ide\nTerminal=false\nCategories=Development;IDE;\nMimeType={};\n",
        types.join(";")
    )
}

/// O XML dos tipos de arquivo do aylis para o banco de MIME do Linux.
pub fn linux_mime_xml() -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<mime-info xmlns=\"http://www.freedesktop.org/standards/shared-mime-info\">\n");
    for (ext, desc) in DESCRIPTIONS {
        out.push_str(&format!(
            "  <mime-type type=\"{}\">\n    <comment>{desc}</comment>\n    <icon name=\"application-x-aylis-{ext}\"/>\n    <glob pattern=\"*.{ext}\"/>\n  </mime-type>\n",
            mime(ext)
        ));
    }
    out.push_str("</mime-info>\n");
    out
}

/// O `Info.plist` do `Aylis IDE.app`: o app, os documentos que ele abre e os tipos exportados.
pub fn macos_info_plist(version: &str) -> String {
    let mut documents = String::new();
    let mut exported = String::new();
    for (ext, desc) in DESCRIPTIONS {
        let uti = format!("{BUNDLE_ID}.{ext}");
        documents.push_str(&format!(
            "    <dict>\n      <key>CFBundleTypeName</key><string>{desc}</string>\n      <key>CFBundleTypeRole</key><string>Editor</string>\n      <key>CFBundleTypeIconFile</key><string>{ext}.icns</string>\n      <key>LSHandlerRank</key><string>Owner</string>\n      <key>LSItemContentTypes</key><array><string>{uti}</string></array>\n    </dict>\n"
        ));
        exported.push_str(&format!(
            "    <dict>\n      <key>UTTypeIdentifier</key><string>{uti}</string>\n      <key>UTTypeDescription</key><string>{desc}</string>\n      <key>UTTypeIconFile</key><string>{ext}.icns</string>\n      <key>UTTypeConformsTo</key><array><string>public.data</string></array>\n      <key>UTTypeTagSpecification</key><dict><key>public.filename-extension</key><array><string>{ext}</string></array><key>public.mime-type</key><string>{}</string></dict>\n    </dict>\n",
            mime(ext)
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n  <key>CFBundleName</key><string>Aylis IDE</string>\n  <key>CFBundleDisplayName</key><string>Aylis IDE</string>\n  <key>CFBundleIdentifier</key><string>{BUNDLE_ID}</string>\n  <key>CFBundleExecutable</key><string>aylis-ide</string>\n  <key>CFBundleIconFile</key><string>aylis-ide.icns</string>\n  <key>CFBundlePackageType</key><string>APPL</string>\n  <key>CFBundleShortVersionString</key><string>{version}</string>\n  <key>CFBundleVersion</key><string>{version}</string>\n  <key>LSMinimumSystemVersion</key><string>11.0</string>\n  <key>NSHighResolutionCapable</key><true/>\n  <key>CFBundleDocumentTypes</key>\n  <array>\n{documents}  </array>\n  <key>UTExportedTypeDeclarations</key>\n  <array>\n{exported}  </array>\n</dict>\n</plist>\n"
    )
}

/// Um `.icns` com os PNG de 128 a 1024 desenhados por `render`.
fn icns_of(render: impl Fn(u32) -> RgbaImage) -> Vec<u8> {
    let mut body = Vec::new();
    for (kind, size) in [(b"ic07", 128), (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024)] {
        let data = crate::icons::png(&render(size));
        body.extend_from_slice(kind);
        body.extend_from_slice(&((data.len() + 8) as u32).to_be_bytes());
        body.extend_from_slice(&data);
    }
    let mut out = b"icns".to_vec();
    out.extend_from_slice(&((body.len() + 8) as u32).to_be_bytes());
    out.extend_from_slice(&body);
    out
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

/// O script que instala a pasta da IDE para o usuário (`~/.local`).
/// O `instalar.sh` do Linux: a IDE (com os outros executáveis e a pasta `aylis/`) em
/// `~/.local/share/aylis-ide/bin`, o lançador da IDE e o `aylis-tool` em `~/.local/bin`, e os
/// tipos de arquivo associados a ela.
const LINUX_INSTALL: &str = r#"#!/bin/sh
# Instala a IDE do Aylis para este usuário (~/.local) e associa os arquivos do Aylis a ela.
set -e
here="$(cd "$(dirname "$0")" && pwd)"
dest="$HOME/.local/share/aylis-ide"
rm -rf "$dest"
mkdir -p "$dest" "$HOME/.local/bin" "$HOME/.local/share"
cp -r "$here/bin" "$dest/"
for exe in aylis-ide aylis-host aylis-tool aylis-migrate; do
  if [ -f "$dest/bin/$exe" ]; then chmod +x "$dest/bin/$exe"; fi
done
# O lançador: no WSL, a IDE e os apps que ela abre (herdam o ambiente, mesmo os compilados
# com um Aylis anterior) vão pelo X11 e pela placa do Windows (d3d12): pelo Wayland do WSLg,
# uma janela desenhada pelo EGL ao lado de outra derruba o compositor.
rm -f "$HOME/.local/bin/aylis-ide"
cat > "$HOME/.local/bin/aylis-ide" <<LAUNCHER
#!/bin/sh
if [ -e /dev/dxg ] && [ -n "\$DISPLAY" ]; then
  unset WAYLAND_DISPLAY
  if [ -z "\$GALLIUM_DRIVER" ] && [ -e /usr/lib/wsl/lib/libd3d12.so ]; then export GALLIUM_DRIVER=d3d12; fi
fi
exec "$dest/bin/aylis-ide" "\$@"
LAUNCHER
chmod +x "$HOME/.local/bin/aylis-ide"
if [ -f "$dest/bin/aylis-tool" ]; then ln -sf "$dest/bin/aylis-tool" "$HOME/.local/bin/aylis-tool"; fi
cp -r "$here/share/." "$HOME/.local/share/"
sed -i "s|@EXEC@|$HOME/.local/bin/aylis-ide|" "$HOME/.local/share/applications/aylis-ide.desktop"
update-mime-database "$HOME/.local/share/mime" 2>/dev/null || true
update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
gtk-update-icon-cache "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
echo "Aylis IDE instalada: $HOME/.local/bin/aylis-ide"
"#;

/// Copia a pasta `from` para `to`, sem o que o instalador também deixa de fora (as saídas de
/// compilação e o que é da máquina: `installer::SKIP`, pelo nome ou por `*.extensão`).
fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    let skipped = |name: &str| {
        crate::installer::SKIP.iter().any(|s| match s.strip_prefix('*') {
            Some(ext) => name.ends_with(ext),
            None => name == *s,
        })
    };
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if skipped(&name) {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            copy_tree(&path, &to.join(&name))?;
        } else {
            std::fs::copy(&path, to.join(&name)).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

/// A pasta `aylis/` ao lado da IDE (`aylis_project::codegen::aylis_root` acha ela lá): o código
/// do Aylis sem os exemplos, os recursos, os clientes e a documentação, como no instalador do
/// Windows.
fn copy_sdk(root: &Path, to: &Path) -> Result<(), String> {
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).map_err(|e| e.to_string())?;
    write(&to.join("Cargo.toml"), crate::installer::without_examples(&manifest).as_bytes())?;
    for file in ["Cargo.lock", "README.md", "LICENSE"] {
        if root.join(file).is_file() {
            std::fs::copy(root.join(file), to.join(file)).map_err(|e| format!("{file}: {e}"))?;
        }
    }
    for dir in ["crates", "third_party", "clients", "resources", "docs", "schema"] {
        if root.join(dir).is_dir() {
            copy_tree(&root.join(dir), &to.join(dir))?;
        }
    }
    // As páginas dos componentes (o MCP da IDE lê), sem as imagens.
    let reference = root.join("examples/Landing/assets/docs/componentes");
    for entry in std::fs::read_dir(&reference).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            let target = to.join("docs/referencia").join(entry.file_name());
            write(&target, &std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?)?;
        }
    }
    Ok(())
}

/// Monta a IDE para `target` (`linux` ou `macos`) com o executável `binary` em `out`.
/// `sdk`: o repositório do Aylis, para o Linux levar os outros executáveis (os que estão ao
/// lado de `binary`) e a pasta `aylis/`. Devolve a pasta (Linux) ou o `.app` (macOS).
pub fn package(target: &str, binary: &Path, logo_ico: &Path, out: &Path, version: &str, sdk: Option<&Path>) -> Result<PathBuf, String> {
    let exe = std::fs::read(binary).map_err(|e| format!("{}: {e}", binary.display()))?;
    let logo = crate::icons::from_ico(&std::fs::read(logo_ico).map_err(|e| format!("{}: {e}", logo_ico.display()))?)?;
    let type_image = |ext: &str, size: u32| crate::icons::file_type_image(&logo, ext, size).expect("tipo do Aylis");
    match target {
        "linux" => {
            let root = out.join("aylis-ide");
            let _ = std::fs::remove_dir_all(&root);
            write(&root.join("bin/aylis-ide"), &exe)?;
            if let Some(sdk) = sdk {
                // Os outros executáveis compilados com ela (o `aylis-host` roda as outras
                // linguagens; o `aylis-tool`, as ferramentas).
                let dir = binary.parent().unwrap_or(Path::new("."));
                for (_, name) in crate::installer::BINARIES.iter().filter(|(_, n)| *n != "aylis-ide") {
                    let other = dir.join(name);
                    if other.is_file() {
                        write(&root.join("bin").join(name), &std::fs::read(&other).map_err(|e| format!("{}: {e}", other.display()))?)?;
                    }
                }
                copy_sdk(sdk, &root.join("bin/aylis"))?;
            }
            write(&root.join("share/applications/aylis-ide.desktop"), linux_desktop("@EXEC@").as_bytes())?;
            write(&root.join("share/mime/packages/aylis-ide.xml"), linux_mime_xml().as_bytes())?;
            for size in [48u32, 128, 256] {
                let dir = root.join(format!("share/icons/hicolor/{size}x{size}"));
                write(&dir.join("apps/aylis-ide.png"), &crate::icons::png(&logo.render(size)))?;
                for (ext, _) in DESCRIPTIONS {
                    write(&dir.join(format!("mimetypes/application-x-aylis-{ext}.png")), &crate::icons::png(&type_image(ext, size)))?;
                }
            }
            write(&root.join("instalar.sh"), LINUX_INSTALL.as_bytes())?;
            Ok(root)
        }
        "macos" => {
            let app = out.join("Aylis IDE.app");
            let _ = std::fs::remove_dir_all(&app);
            let contents = app.join("Contents");
            write(&contents.join("MacOS/aylis-ide"), &exe)?;
            write(&contents.join("Info.plist"), macos_info_plist(version).as_bytes())?;
            write(&contents.join("Resources/aylis-ide.icns"), &crate::icons::icns(&logo))?;
            for (ext, _) in DESCRIPTIONS {
                write(&contents.join(format!("Resources/{ext}.icns")), &icns_of(|size| type_image(ext, size)))?;
            }
            Ok(app)
        }
        other => Err(format!("alvo sem empacotamento da IDE: {other} (linux ou macos; o Windows tem o installer-ide)")),
    }
}

/// O lançador `/usr/bin/aylis-ide` do `.deb` (o mesmo do `instalar.sh`): no WSL, a IDE e os
/// apps que ela abre vão pelo X11 e pela placa do Windows (d3d12).
const DEB_LAUNCHER: &str = r#"#!/bin/sh
if [ -e /dev/dxg ] && [ -n "$DISPLAY" ]; then
  unset WAYLAND_DISPLAY
  if [ -z "$GALLIUM_DRIVER" ] && [ -e /usr/lib/wsl/lib/libd3d12.so ]; then export GALLIUM_DRIVER=d3d12; fi
fi
exec /opt/aylis-ide/bin/aylis-ide "$@"
"#;

/// O `postinst` e o `postrm` do `.deb`: os bancos de tipos, de atalhos e de ícones do sistema
/// passam a ver (ou deixam de ver) os do Aylis.
const DEB_HOOK: &str = "#!/bin/sh\nset -e\nupdate-mime-database /usr/share/mime >/dev/null 2>&1 || true\nupdate-desktop-database -q /usr/share/applications >/dev/null 2>&1 || true\ngtk-update-icon-cache -q -t /usr/share/icons/hicolor >/dev/null 2>&1 || true\n";

/// A versão no formato do Debian: o pré-lançamento (`0.2.0-beta.1`) vira `0.2.0~beta.1`, que
/// vem antes do `0.2.0` (o hífen seria a revisão do pacote).
pub fn deb_version(version: &str) -> String {
    version.replacen('-', "~", 1)
}

/// O `control` do `.deb`. As dependências são as bibliotecas que um app do Aylis precisa para
/// compilar no Linux (as de `crate::wsl::PACKAGES`, as mesmas da CI): o `apt` as instala junto,
/// e o primeiro projeto compila sem o usuário procurar o que falta.
pub fn deb_control(version: &str, installed_kb: u64) -> String {
    format!(
        "Package: aylis-ide\nVersion: {}\nArchitecture: amd64\nMaintainer: e-rust <347532650+e-rust@users.noreply.github.com>\nInstalled-Size: {installed_kb}\nDepends: {}\nSection: devel\nPriority: optional\nHomepage: https://github.com/e-rust/aylis\nDescription: Aylis IDE: interfaces visuais com designer, em Rust e mais 7 linguagens\n A IDE do Aylis: o designer de forms, o editor de código, compilar, executar,\n depurar e publicar para Windows, Linux, macOS, Android, iOS e navegador. Traz o\n código do Aylis em /opt/aylis-ide; o Rust e as outras linguagens vêm pelos pacotes\n de ferramentas (Ferramentas → Pacotes de ferramentas) ou pelo sistema.\n",
        deb_version(version),
        crate::wsl::PACKAGES.join(", ")
    )
}

/// Um arquivo `ar` (o envelope do `.deb`): os membros na ordem, cada um com o cabeçalho de 60
/// bytes e alinhado em 2.
fn ar(members: &[(&str, &[u8])]) -> Vec<u8> {
    let mtime = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let mut out = b"!<arch>\n".to_vec();
    for (name, data) in members {
        out.extend_from_slice(format!("{name:<16}{mtime:<12}{:<6}{:<6}{:<8}{:<10}`\n", 0, 0, "100644", data.len()).as_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(b'\n');
        }
    }
    out
}

/// Um `.tar.gz` com dono root e as permissões dadas (o Windows não guarda o bit de execução).
struct DebTar {
    builder: tar::Builder<flate2::write::GzEncoder<Vec<u8>>>,
    dirs: std::collections::BTreeSet<String>,
    mtime: u64,
    bytes: u64,
}

impl DebTar {
    fn new() -> Self {
        let mtime = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        Self { builder: tar::Builder::new(encoder), dirs: Default::default(), mtime, bytes: 0 }
    }

    fn header(&self, kind: tar::EntryType, mode: u32, size: u64) -> tar::Header {
        let mut h = tar::Header::new_gnu();
        h.set_entry_type(kind);
        h.set_mode(mode);
        h.set_uid(0);
        h.set_gid(0);
        h.set_mtime(self.mtime);
        h.set_size(size);
        h
    }

    /// As pastas de cima de `path` (o dpkg quer cada uma na lista).
    fn parents(&mut self, path: &str) -> Result<(), String> {
        let mut acc = String::from(".");
        let parts: Vec<&str> = path.split('/').collect();
        for part in &parts[..parts.len() - 1] {
            acc = format!("{acc}/{part}");
            if self.dirs.insert(acc.clone()) {
                let mut h = self.header(tar::EntryType::Directory, 0o755, 0);
                self.builder.append_data(&mut h, format!("{acc}/"), std::io::empty()).map_err(|e| format!("{acc}: {e}"))?;
            }
        }
        Ok(())
    }

    fn file(&mut self, path: &str, data: &[u8], mode: u32) -> Result<(), String> {
        self.parents(path)?;
        let mut h = self.header(tar::EntryType::Regular, mode, data.len() as u64);
        self.bytes += data.len() as u64;
        self.builder.append_data(&mut h, format!("./{path}"), data).map_err(|e| format!("{path}: {e}"))
    }

    fn symlink(&mut self, path: &str, target: &str) -> Result<(), String> {
        self.parents(path)?;
        let mut h = self.header(tar::EntryType::Symlink, 0o777, 0);
        self.builder.append_link(&mut h, format!("./{path}"), target).map_err(|e| format!("{path}: {e}"))
    }

    fn finish(self) -> Result<Vec<u8>, String> {
        self.builder.into_inner().and_then(|gz| gz.finish()).map_err(|e| e.to_string())
    }
}

/// Executável no pacote: os binários do Aylis (em `bin/`, `rel` sem pasta) e os scripts (`#!`)
/// da pasta do código.
fn executable(rel: &str, data: &[u8]) -> bool {
    (!rel.contains('/') && crate::installer::BINARIES.iter().any(|(_, n)| *n == rel)) || data.starts_with(b"#!")
}

/// Os arquivos de `dir` (recursivo), com o caminho relativo em `/`.
fn files_of(dir: &Path, prefix: &str, out: &mut Vec<(String, PathBuf)>) -> Result<(), String> {
    let mut entries: Vec<_> = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let rel = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
        if path.is_dir() {
            files_of(&path, &rel, out)?;
        } else {
            out.push((rel, path));
        }
    }
    Ok(())
}

/// O `.deb` da IDE a partir da pasta do pacote do Linux (`package("linux", ...)`): instala em
/// `/opt/aylis-ide` (a IDE, as ferramentas e o código do Aylis), põe a IDE (pelo lançador) e
/// o `aylis-tool` em `/usr/bin`, e o atalho, os tipos de arquivo e os ícones em `/usr/share`.
/// Montado aqui mesmo (o `ar` e os `.tar.gz`), sem o `dpkg-deb`: sai igual no Windows e no
/// Linux. Devolve o caminho do `.deb`.
pub fn linux_deb(pkg: &Path, out: &Path, version: &str) -> Result<PathBuf, String> {
    let mut data = DebTar::new();
    let mut files = Vec::new();
    files_of(&pkg.join("bin"), "", &mut files)?;
    for (rel, path) in &files {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mode = if executable(rel, &bytes) { 0o755 } else { 0o644 };
        data.file(&format!("opt/aylis-ide/bin/{rel}"), &bytes, mode)?;
    }
    let mut shared = Vec::new();
    files_of(&pkg.join("share"), "", &mut shared)?;
    for (rel, path) in &shared {
        let mut bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if rel.ends_with(".desktop") {
            bytes = String::from_utf8_lossy(&bytes).replace("@EXEC@", "/usr/bin/aylis-ide").into_bytes();
        }
        data.file(&format!("usr/share/{rel}"), &bytes, 0o644)?;
    }
    data.file("usr/bin/aylis-ide", DEB_LAUNCHER.as_bytes(), 0o755)?;
    data.symlink("usr/bin/aylis-tool", "/opt/aylis-ide/bin/aylis-tool")?;
    let installed_kb = data.bytes.div_ceil(1024);
    let data = data.finish()?;

    let mut control = DebTar::new();
    control.file("control", deb_control(version, installed_kb).as_bytes(), 0o644)?;
    control.file("postinst", DEB_HOOK.as_bytes(), 0o755)?;
    control.file("postrm", DEB_HOOK.as_bytes(), 0o755)?;
    let control = control.finish()?;

    let deb = out.join(format!("aylis-ide_{}_amd64.deb", deb_version(version)));
    write(&deb, &ar(&[("debian-binary", b"2.0\n"), ("control.tar.gz", &control), ("data.tar.gz", &data)]))?;
    Ok(deb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_file_types_go_to_the_desktop_the_mime_database_and_the_plist() {
        assert_eq!(DESCRIPTIONS.map(|(e, _)| e), crate::icons::FILE_TYPES.map(|(e, _, _)| e), "os mesmos tipos do Windows");
        let desktop = linux_desktop("/opt/aylis/aylis-ide");
        assert!(desktop.contains("Exec=\"/opt/aylis/aylis-ide\" %f") && desktop.contains("MimeType=application/x-aylis-aylisproj;application/x-aylis-aylispkg;"));
        let xml = linux_mime_xml();
        assert!(xml.contains("<glob pattern=\"*.aylisreport\"/>") && xml.contains("application-x-aylis-aylisrec"));
        let plist = macos_info_plist("1.2.3");
        assert!(plist.contains("<string>dev.aylis.ide.aylisproj</string>") && plist.contains("<string>aylispkg.icns</string>") && plist.contains("1.2.3"));
        // O plist é XML válido (o macOS recusa o app com um plist quebrado).
        assert_eq!(plist.matches("<dict>").count(), plist.matches("</dict>").count());
    }

    #[test]
    fn the_packages_have_the_icons_of_each_type() {
        let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let logo = root.join("crates/aylis-ide/resources/aylis-ide.ico");
        let out = std::env::temp_dir().join(format!("aylis_ide_package_{}", std::process::id()));
        let fake = out.join("aylis-ide-bin");
        write(&fake, b"exe").unwrap();
        let linux = package("linux", &fake, &logo, &out, "1.0.0", None).unwrap();
        assert!(linux.join("share/icons/hicolor/256x256/mimetypes/application-x-aylis-aylisproj.png").is_file());
        assert!(linux.join("instalar.sh").is_file() && linux.join("bin/aylis-ide").is_file());
        let app = package("macos", &fake, &logo, &out, "1.0.0", None).unwrap();
        let icns = std::fs::read(app.join("Contents/Resources/aylisrec.icns")).unwrap();
        assert_eq!(&icns[..4], b"icns");
        assert_eq!(u32::from_be_bytes(icns[4..8].try_into().unwrap()) as usize, icns.len());
        assert!(package("windows", &fake, &logo, &out, "1.0.0", None).is_err());
        let _ = std::fs::remove_dir_all(&out);
    }
    /// O `.deb`: o envelope `ar` com os três membros na ordem, o `control` com a versão do
    /// Debian e as dependências de compilação, e os arquivos com dono root, o lançador e a IDE
    /// executáveis, o `aylis-tool` ligado em `/usr/bin` e o atalho apontando para o lançador.
    #[test]
    fn the_deb_installs_in_opt_with_the_launcher_and_the_dependencies() {
        let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let logo = root.join("crates/aylis-ide/resources/aylis-ide.ico");
        let out = std::env::temp_dir().join(format!("aylis_ide_deb_{}", std::process::id()));
        let fake = out.join("aylis-ide");
        write(&fake, b"ELF").unwrap();
        let pkg = package("linux", &fake, &logo, &out.join("pkg"), "0.9.0-beta.2", None).unwrap();
        let deb = linux_deb(&pkg, &out, "0.9.0-beta.2").unwrap();
        assert!(deb.ends_with("aylis-ide_0.9.0~beta.2_amd64.deb"), "{}", deb.display());
        let bytes = std::fs::read(&deb).unwrap();
        assert!(bytes.starts_with(b"!<arch>
debian-binary   "));
        // Os membros do `ar`: nome, tamanho e o conteúdo.
        let mut members = Vec::new();
        let mut at = 8;
        while at + 60 <= bytes.len() {
            let name = String::from_utf8_lossy(&bytes[at..at + 16]).trim().to_string();
            let size: usize = String::from_utf8_lossy(&bytes[at + 48..at + 58]).trim().parse().unwrap();
            members.push((name, bytes[at + 60..at + 60 + size].to_vec()));
            at += 60 + size + size % 2;
        }
        assert_eq!(members.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(), ["debian-binary", "control.tar.gz", "data.tar.gz"]);
        assert_eq!(members[0].1, b"2.0
");
        let entries = |gz: &[u8]| -> Vec<(String, u32, u64, Vec<u8>)> {
            let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(gz));
            archive
                .entries()
                .unwrap()
                .map(|e| {
                    let mut e = e.unwrap();
                    // O leitor tira o `./` do começo; o teste compara sem ele.
                    let path = e.path().unwrap().to_string_lossy().trim_start_matches("./").to_string();
                    let (mode, uid) = (e.header().mode().unwrap(), e.header().uid().unwrap());
                    let mut data = Vec::new();
                    std::io::Read::read_to_end(&mut e, &mut data).unwrap();
                    (path, mode, uid, data)
                })
                .collect()
        };
        let control = entries(&members[1].1);
        let text = String::from_utf8(control.iter().find(|e| e.0 == "control").unwrap().3.clone()).unwrap();
        assert!(text.contains("Version: 0.9.0~beta.2
") && text.contains("Architecture: amd64"));
        assert!(text.contains("Depends: build-essential, pkg-config,") && text.contains("libfontconfig1-dev"));
        assert_eq!(control.iter().find(|e| e.0 == "postinst").unwrap().1, 0o755);
        let data = entries(&members[2].1);
        let find = |p: &str| data.iter().find(|e| e.0 == p).unwrap_or_else(|| panic!("falta {p}"));
        assert!(data.iter().all(|e| e.2 == 0), "dono root");
        assert_eq!(find("opt/aylis-ide/bin/aylis-ide").1, 0o755);
        assert_eq!(find("usr/bin/aylis-ide").1, 0o755);
        assert!(String::from_utf8_lossy(&find("usr/bin/aylis-ide").3).contains("exec /opt/aylis-ide/bin/aylis-ide"));
        assert!(data.iter().any(|e| e.0 == "usr/bin/aylis-tool"));
        assert!(String::from_utf8_lossy(&find("usr/share/applications/aylis-ide.desktop").3).contains("Exec=\"/usr/bin/aylis-ide\" %f"));
        assert_eq!(find("usr/share/icons/hicolor/48x48/apps/aylis-ide.png").1, 0o644);
        assert!(data.iter().any(|e| e.0.trim_end_matches('/') == "opt"), "as pastas de cima também");
        let _ = std::fs::remove_dir_all(&out);
    }
}
