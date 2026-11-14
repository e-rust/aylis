//! O pacote `fpc` (Free Pascal 3.2.2 no Windows). O projeto do Free Pascal só publica o
//! instalador (Inno Setup), que o Aylis não extrai e não deve executar. Então quem distribui o
//! Aylis tira os arquivos uma vez com o `innoextract` (que lê o instalador sem rodá-lo), e
//! `aylis-tool fpc-package` monta o zip do pacote: o compilador (`bin`), as mensagens (`msg`) e as
//! units de 64 bits (`units/x86_64-win64`; os exemplos, a documentação e as de 32 bits ficam de
//! fora). O zip vai para o cache de downloads e o SHA-256 para o manifesto, como o
//! `skia-android`.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::process::{Control, LineKind, Sink, run_tool};

/// O pacote no manifesto.
pub const PACKAGE: &str = "fpc";
/// O instalador oficial (SourceForge, `freepascal/Win32/3.2.2`) e o SHA-256 dele.
pub const INSTALLER: &str = "fpc-3.2.2.win32.and.win64.exe";
pub const INSTALLER_SHA256: &str = "8c255390544b051388b577eb61c6191a04883264afe0e3369b3600a56daf7bde";
/// O arquivo do pacote.
pub const ARCHIVE: &str = "fpc-3.2.2-x86_64-win64.zip";
/// O que entra no pacote (debaixo da pasta `app` do instalador).
const PARTS: [&str; 3] = ["bin", "msg", "units/x86_64-win64"];

/// O zip feito.
pub struct Built {
    pub archive: PathBuf,
    pub sha256: String,
    pub size: u64,
}

/// Confere o instalador, extrai com o `innoextract` numa pasta temporária e monta o zip em `cache`.
pub fn build(installer: &Path, innoextract: &Path, cache: &Path, control: &Control, sink: Sink) -> Result<Built, String> {
    let sha = crate::toolpacks::sha256_file(installer)?;
    if sha != INSTALLER_SHA256 {
        return Err(format!("{}: não é o {INSTALLER} oficial (SHA-256 {sha})", installer.display()));
    }
    let work = std::env::temp_dir().join(format!("aylis-fpc-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let result = (|| {
        run_tool(Command::new(innoextract).arg("-s").arg("-d").arg(&work).arg(installer), control, sink, LineKind::Tool)?;
        let app = work.join("app");
        if !app.join("bin/i386-win32/fpc.exe").is_file() {
            return Err(format!("{}: o instalador não tem bin/i386-win32/fpc.exe", installer.display()));
        }
        std::fs::create_dir_all(cache).map_err(|e| format!("{}: {e}", cache.display()))?;
        let archive = cache.join(ARCHIVE);
        zip_parts(&app, &archive)?;
        Ok(Built { sha256: crate::toolpacks::sha256_file(&archive)?, size: std::fs::metadata(&archive).map_err(|e| e.to_string())?.len(), archive })
    })();
    let _ = std::fs::remove_dir_all(&work);
    result
}

/// As `PARTS` de `app` num zip, debaixo de `fpc-3.2.2/` (o pacote tira com `strip: 1`).
fn zip_parts(app: &Path, archive: &Path) -> Result<(), String> {
    let partial = archive.with_extension("part");
    let file = std::fs::File::create(&partial).map_err(|e| format!("{}: {e}", partial.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut files = Vec::new();
    for part in PARTS {
        walk(&app.join(part), &mut files);
    }
    files.sort();
    for path in files {
        let rel = path.strip_prefix(app).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
        zip.start_file(format!("fpc-3.2.2/{rel}"), options).map_err(|e| e.to_string())?;
        zip.write_all(&std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    std::fs::rename(&partial, archive).map_err(|e| format!("{}: {e}", archive.display()))
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// Grava o SHA-256 e o tamanho do zip no `resources/packages.lock.json` de `root`.
pub fn update_lock(root: &Path, built: &Built) -> Result<(), String> {
    let path = root.join("resources/packages.lock.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lock: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let files = lock["packages"][PACKAGE]["platforms"]["windows-x86_64"]["files"].as_array_mut().ok_or_else(|| format!("o manifesto não tem o pacote {PACKAGE}"))?;
    let file = files.iter_mut().find(|f| f["name"] == ARCHIVE).ok_or_else(|| format!("o pacote {PACKAGE} não tem {ARCHIVE}"))?;
    file["sha256"] = built.sha256.clone().into();
    file["size"] = built.size.into();
    let text = serde_json::to_string_pretty(&lock).map_err(|e| e.to_string())? + "\n";
    crate::toolpacks::Manifest::parse(&text)?;
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_package_in_the_manifest_is_the_one_this_command_makes() {
        let manifest = crate::toolpacks::Manifest::embedded();
        let package = manifest.package_for(PACKAGE, "windows-x86_64").expect("o pacote no manifesto");
        let file = package.files.iter().find(|f| f.name == ARCHIVE).expect("o zip do pacote");
        assert_eq!(file.built.as_deref(), Some("aylis-tool fpc-package"));
        assert_eq!(file.strip, 1);
    }
}
