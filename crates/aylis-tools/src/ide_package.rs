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
}
