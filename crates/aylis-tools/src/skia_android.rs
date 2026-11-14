//! O Skia do Android de 32 bits (`armeabi-v7a`: TV box, celular de entrada). O projeto do Skia
//! para Rust (`skia-binaries`) não publica binário pronto para ele com as features do Aylis (404
//! em 13/11/2026, relatório do RTSPlayer, item 5), e compilar o Skia a cada publicar levaria
//! muito tempo e pediria Python, Ninja e libclang na máquina de quem publica.
//!
//! Então quem distribui o Aylis compila uma vez (`aylis-tool skia-android`), no formato que o
//! `skia-bindings` baixa (`skia-binaries-<chave>.tar.gz`), e o arquivo vira o pacote de
//! ferramenta `skia-android-armv7` (SHA-256 no manifesto, vindo da pasta de pacotes ou dos
//! espelhos, como os outros). O publicar instala o pacote e aponta o `SKIA_BINARIES_URL` para ele.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::process::{Control, Event, LineKind, Sink, run_tool};

/// A versão do `skia-bindings` que o Aylis usa (o `Cargo.lock`; o teste confere).
pub const SKIA_BINDINGS_VERSION: &str = "0.153.3";
/// A marca do rust-skia nessa versão: o começo da chave dos binários (a mesma do oficial).
pub const SKIA_REPO_HASH: &str = "b7f043e0b1e2a850e702";
/// O Skia que o rust-skia usa nessa versão (`[package.metadata] skia` do `skia-bindings`).
pub const SKIA_TAG: &str = "m153-0.101.2";
pub const TRIPLE: &str = "armv7-linux-androideabi";
/// As features do Skia que o `aylis-platform-winit` liga no Android (`cpu` e `gpu`).
pub const FEATURES_KEY: &str = "ganesh-gl-jpegd-jpege-pdf-textlayout";
/// O pacote de ferramenta com o arquivo.
pub const PACKAGE: &str = "skia-android-armv7";

/// A chave dos binários, como o `skia-bindings` monta.
pub fn key() -> String {
    format!("{SKIA_REPO_HASH}-{TRIPLE}-{FEATURES_KEY}")
}

/// O nome do arquivo (o do pacote e o que o `skia-bindings` procura).
pub fn archive_name() -> String {
    format!("skia-binaries-{}.tar.gz", key())
}

/// O `SKIA_BINARIES_URL` para a pasta `dir` do pacote instalado: o `skia-bindings` troca o
/// `{key}` pela chave das features do app (outras features: o arquivo não existe, e o erro diz).
/// O `skia-bindings` só tira o `file://` e lê o resto como caminho: `file://C:/...` no Windows,
/// `file:///home/...` nos outros.
pub fn binaries_url(dir: &Path) -> String {
    let d = dir.to_string_lossy().replace('\\', "/");
    format!("file://{d}/skia-binaries-{{key}}.tar.gz")
}

/// O que a compilação precisa além do Android NDK.
pub struct Tools {
    /// A pasta com o `libclang.dll` (ou `.so`), para o bindgen.
    pub libclang: PathBuf,
    pub ninja: PathBuf,
    pub python: PathBuf,
    pub ndk: PathBuf,
}

impl Tools {
    /// Acha as ferramentas: as passadas, o `LIBCLANG_PATH`, o `PATH` e o Ninja do Visual Studio.
    pub fn find(libclang: Option<PathBuf>, ninja: Option<PathBuf>) -> Result<Tools, String> {
        let libclang = libclang
            .or_else(|| std::env::var_os("LIBCLANG_PATH").map(PathBuf::from))
            .filter(|d| d.join(if cfg!(windows) { "libclang.dll" } else { "libclang.so" }).is_file())
            .ok_or("falta o libclang: --libclang <pasta com o libclang.dll> (o do pacote `libclang` do PyPI serve: pip download libclang)")?;
        let ninja = ninja.or_else(|| crate::which("ninja")).or_else(visual_studio_ninja).ok_or("falta o Ninja: --ninja <ninja.exe> (vem no Visual Studio Build Tools, no componente do CMake)")?;
        let python = crate::which("python").or_else(|| crate::which("python3")).ok_or("falta o Python 3 no PATH")?;
        let sdk = crate::env::android_sdk().ok_or("Android SDK não encontrado")?;
        let ndk = crate::env::android_ndk(&sdk).ok_or("NDK não encontrado")?;
        Ok(Tools { libclang, ninja, python, ndk })
    }
}

fn visual_studio_ninja() -> Option<PathBuf> {
    let roots = ["C:/Program Files/Microsoft Visual Studio", "C:/Program Files (x86)/Microsoft Visual Studio"];
    for root in roots {
        let Ok(years) = std::fs::read_dir(root) else { continue };
        for year in years.flatten() {
            for edition in std::fs::read_dir(year.path()).into_iter().flatten().flatten() {
                let ninja = edition.path().join("Common7/IDE/CommonExtensions/Microsoft/CMake/Ninja/ninja.exe");
                if ninja.is_file() {
                    return Some(ninja);
                }
            }
        }
    }
    None
}

/// O que saiu: o arquivo (já no cache de downloads), o SHA-256 e o tamanho.
pub struct Built {
    pub archive: PathBuf,
    pub sha256: String,
    pub size: u64,
}

/// Compila o Skia para o `armeabi-v7a` em `work` (o fonte fica lá para a próxima vez), monta o
/// arquivo e o guarda no cache de downloads do Aylis, de onde o `packages pack` o leva.
pub fn build(work: &Path, tools: &Tools, control: &Control, sink: Sink) -> Result<Built, String> {
    std::fs::create_dir_all(work).map_err(|e| format!("{}: {e}", work.display()))?;
    let source = work.join("skia");
    // Sem links simbólicos: o Windows sem o modo de desenvolvedor não deixa criar (o fonte
    // baixado pelo próprio skia-bindings parava aí), e o Aylis não usa os arquivos que são link.
    let git_config = [("GIT_CONFIG_COUNT", "2"), ("GIT_CONFIG_KEY_0", "core.longpaths"), ("GIT_CONFIG_VALUE_0", "true"), ("GIT_CONFIG_KEY_1", "core.symlinks"), ("GIT_CONFIG_VALUE_1", "false")];
    if !source.join("DEPS").is_file() {
        sink(Event::Step(format!("Baixando o Skia {SKIA_TAG}…")));
        let mut git = Command::new("git");
        git.args(["clone", "--depth", "1", "--branch", SKIA_TAG, "https://github.com/rust-skia/skia"]).arg(&source).envs(git_config);
        run_tool(&mut git, control, sink, LineKind::Tool)?;
    }
    let gn = source.join("bin").join(if cfg!(windows) { "gn.exe" } else { "gn" });
    if !gn.is_file() {
        sink(Event::Step("Baixando as dependências do Skia (git-sync-deps)…".into()));
        let mut sync = Command::new(&tools.python);
        sync.arg("tools/git-sync-deps").current_dir(&source).envs(git_config).env("GIT_SYNC_DEPS_SKIP_EMSDK", "1").env("GIT_SYNC_DEPS_PATH", "DEPS");
        run_tool(&mut sync, control, sink, LineKind::Tool)?;
    }
    // O Ninja chama `python3`, que no Windows costuma ser o atalho da Microsoft Store.
    let shim = work.join("shim");
    let mut path = std::env::var_os("PATH").unwrap_or_default();
    if cfg!(windows) {
        std::fs::create_dir_all(&shim).map_err(|e| e.to_string())?;
        std::fs::write(shim.join("python3.bat"), format!("@\"{}\" %*\r\n", tools.python.display())).map_err(|e| e.to_string())?;
        path = std::env::join_paths(std::iter::once(shim.clone()).chain(std::env::split_paths(&path))).map_err(|e| e.to_string())?;
    }
    // O libclang do PyPI não traz os cabeçalhos internos do clang: os do clang do NDK. A variável
    // com o alvo vence a que o cargo-ndk define.
    let host = if cfg!(windows) { "windows-x86_64" } else if cfg!(target_os = "macos") { "darwin-x86_64" } else { "linux-x86_64" };
    let clang_lib = tools.ndk.join("toolchains/llvm/prebuilt").join(host).join("lib/clang");
    let resource_dir = std::fs::read_dir(&clang_lib).map_err(|e| format!("{}: {e}", clang_lib.display()))?.flatten().map(|e| e.path()).max().ok_or("o NDK não tem lib/clang")?;
    sink(Event::Step(format!("Compilando o Skia para {TRIPLE} (demora: ~1700 passos)…")));
    let root = crate::aylis_root();
    let mut cargo = Command::new("cargo");
    cargo
        .args(["ndk", "-t", "armeabi-v7a", "-P", "26", "check", "-p", "aylis-platform-winit"])
        .current_dir(&root)
        .env("PATH", &path)
        .env("ANDROID_NDK", &tools.ndk)
        .env("ANDROID_NDK_HOME", &tools.ndk)
        .env("LIBCLANG_PATH", &tools.libclang)
        .env("SKIA_NINJA_COMMAND", &tools.ninja)
        .env("SKIA_GN_COMMAND", &gn)
        .env("SKIA_SOURCE_DIR", &source)
        .env(format!("BINDGEN_EXTRA_CLANG_ARGS_{TRIPLE}"), format!("-resource-dir={}", resource_dir.to_string_lossy().replace('\\', "/")))
        .env_remove("SKIA_BINARIES_URL");
    run_tool(&mut cargo, control, sink, LineKind::Tool)?;

    let out = newest_output(&crate::shared_target_dir().join(TRIPLE).join("debug").join("build")).ok_or("a compilação não deixou out/skia/bindings.rs")?;
    let cache = crate::toolpacks::Paths::user().ok_or("sem pasta do usuário")?.cache;
    std::fs::create_dir_all(&cache).map_err(|e| format!("{}: {e}", cache.display()))?;
    let archive = cache.join(archive_name());
    pack(&out, &source.join("LICENSE"), &archive)?;
    let sha256 = crate::toolpacks::sha256_file(&archive)?;
    let size = std::fs::metadata(&archive).map_err(|e| e.to_string())?.len();
    Ok(Built { archive, sha256, size })
}

/// A pasta `out/skia` da última compilação do `skia-bindings` (há uma por conjunto de opções).
fn newest_output(build: &Path) -> Option<PathBuf> {
    std::fs::read_dir(build)
        .ok()?
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("skia-bindings-"))
        .map(|e| e.path().join("out").join("skia"))
        .filter(|d| d.join("bindings.rs").is_file())
        .max_by_key(|d| d.join("bindings.rs").metadata().and_then(|m| m.modified()).ok())
}

/// Os arquivos do pacote oficial, na mesma pasta `skia-binaries/` (o `libskia.a` já traz o
/// HarfBuzz, o FreeType, o ICU...).
pub const FILES: [&str; 7] = ["libskia.a", "libskparagraph.a", "libskshaper.a", "libskunicode_core.a", "libskunicode_icu.a", "libskia-bindings.a", "bindings.rs"];

fn pack(out: &Path, license: &Path, archive: &Path) -> Result<(), String> {
    let partial = archive.with_extension("part");
    let file = std::fs::File::create(&partial).map_err(|e| format!("{}: {e}", partial.display()))?;
    let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(file, flate2::Compression::default()));
    let add_bytes = |tar: &mut tar::Builder<_>, name: &str, bytes: &[u8]| -> Result<(), String> {
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, format!("skia-binaries/{name}"), bytes).map_err(|e| e.to_string())
    };
    for name in FILES {
        let path = out.join(name);
        tar.append_path_with_name(&path, format!("skia-binaries/{name}")).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    add_bytes(&mut tar, "LICENSE_SKIA", &std::fs::read(license).map_err(|e| format!("{}: {e}", license.display()))?)?;
    add_bytes(&mut tar, "tag.txt", SKIA_BINDINGS_VERSION.as_bytes())?;
    add_bytes(&mut tar, "key.txt", key().as_bytes())?;
    tar.into_inner().and_then(|gz| gz.finish()).map_err(|e| e.to_string())?;
    std::fs::rename(&partial, archive).map_err(|e| format!("{}: {e}", archive.display()))
}

/// Grava o SHA-256 e o tamanho do arquivo no `resources/packages.lock.json` de `root`.
pub fn update_lock(root: &Path, built: &Built) -> Result<(), String> {
    let path = root.join("resources/packages.lock.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut lock: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let name = archive_name();
    let files = lock["packages"][PACKAGE]["platforms"]["any"]["files"].as_array_mut().ok_or_else(|| format!("o manifesto não tem o pacote {PACKAGE}"))?;
    let file = files.iter_mut().find(|f| f["name"] == name.as_str()).ok_or_else(|| format!("o pacote {PACKAGE} não tem {name}"))?;
    file["sha256"] = built.sha256.clone().into();
    file["size"] = built.size.into();
    let text = serde_json::to_string_pretty(&lock).map_err(|e| e.to_string())? + "\n";
    crate::toolpacks::Manifest::parse(&text)?;
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Subiu o `skia-bindings`: a chave muda (outra marca do rust-skia e outro Skia), e o pacote
    /// tem de ser refeito com `aylis-tool skia-android`.
    #[test]
    fn the_constants_follow_the_skia_bindings_in_use() {
        let lock = std::fs::read_to_string(crate::aylis_root().join("Cargo.lock")).unwrap();
        let version = lock.split("[[package]]").find(|p| p.contains("name = \"skia-bindings\"")).and_then(|p| p.lines().find_map(|l| l.strip_prefix("version = \""))).map(|v| v.trim_end_matches('"').to_string());
        assert_eq!(version.as_deref(), Some(SKIA_BINDINGS_VERSION), "o skia-bindings mudou: atualize as constantes e refaça o pacote {PACKAGE} (aylis-tool skia-android)");
        let manifest = crate::toolpacks::Manifest::embedded();
        let package = manifest.package(PACKAGE).expect("o pacote no manifesto");
        assert_eq!(package.version, SKIA_BINDINGS_VERSION);
        assert!(package.files.iter().any(|f| f.name == archive_name()), "o arquivo do pacote é o da chave atual");
    }

    #[test]
    fn the_url_points_to_the_package_folder() {
        let url = binaries_url(Path::new(r"C:\Users\x\.aylis\tools\skia-android-armv7\0.153.3"));
        assert_eq!(url, "file://C:/Users/x/.aylis/tools/skia-android-armv7/0.153.3/skia-binaries-{key}.tar.gz");
    }
}
