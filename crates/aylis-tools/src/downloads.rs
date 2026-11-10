//! As ferramentas que a IDE baixa (depuradores, servidores de linguagem, o bundletool) com
//! versão fixa e o SHA-256 de cada arquivo, em `resources/tools.lock.json`. O download, a
//! conferência e a extração são feitos aqui, em Rust ([`fetch`], `aylis-tool fetch <nome>`):
//! um arquivo que não confere com o hash fixado não é usado. O que vem por gerenciador de
//! pacotes (go, pip, npm, dotnet, git) usa a versão fixada em [`version`]; o gerenciador
//! confere a integridade.
//!
//! Mudar de versão: `aylis-tool update-tools` regrava o lock com os hashes publicados (os do
//! GitHub, ou o `.sha256` ao lado do arquivo); com `--latest`, passa antes para a última
//! versão de cada uma (as que têm teto, como o JDT LS que ainda roda no Java 17, ficam).

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

const LOCK: &str = include_str!("../../../resources/tools.lock.json");

fn lock() -> Value {
    serde_json::from_str(LOCK).expect("resources/tools.lock.json válido")
}

/// A versão fixada de um pacote (`gopls`, `delve`, `python-lsp-server`, `typescript`...).
pub fn version(package: &str) -> String {
    lock()["packages"][package].as_str().unwrap_or_default().to_string()
}

/// Esta plataforma como no lock (`windows-x86_64`).
fn platform() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

/// Um arquivo fixado para esta plataforma.
#[derive(Clone, Debug)]
pub struct Archive {
    pub name: String,
    pub version: String,
    /// O nome do arquivo (`codelldb-win32-x64.vsix`).
    pub file: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
    /// Onde fica na pasta de ferramentas (uma pasta, ou o arquivo se não for extraído).
    dest: String,
    extract: bool,
}

impl Archive {
    pub fn megabytes(&self) -> u64 {
        self.size.div_ceil(1024 * 1024)
    }
}

/// O arquivo de `name` para esta plataforma (a exata, depois só o sistema, depois `any`).
pub fn archive(name: &str) -> Option<Archive> {
    let lock = lock();
    let entry = &lock["archives"][name];
    let files = entry["files"].as_object()?;
    let f = files.get(&platform()).or_else(|| files.get(std::env::consts::OS)).or_else(|| files.get("any"))?;
    Some(Archive {
        name: name.to_string(),
        version: entry["version"].as_str()?.to_string(),
        file: f["name"].as_str()?.to_string(),
        url: f["url"].as_str()?.to_string(),
        sha256: f["sha256"].as_str()?.to_string(),
        size: f["size"].as_u64().unwrap_or(0),
        dest: entry["dest"].as_str()?.to_string(),
        extract: entry["extract"].as_bool().unwrap_or(true),
    })
}

/// Baixa (ou reaproveita o que já está no cache), confere o SHA-256 e instala na pasta de
/// ferramentas do aylis. Devolve onde ficou.
pub fn fetch(name: &str, say: &mut dyn FnMut(String)) -> Result<PathBuf, String> {
    let a = archive(name).ok_or_else(|| format!("{name}: nenhum arquivo fixado para {}", platform()))?;
    if !plain_name(&a.file) || !plain_name(&a.dest) {
        return Err(format!("{name}: nome inválido no tools.lock.json"));
    }
    let tools = crate::dap::tools_dir().ok_or("sem a pasta de ferramentas do Aylis")?;
    let cache = tools.parent().ok_or("sem a pasta de dados do Aylis")?.join("downloads");
    std::fs::create_dir_all(&cache).map_err(|e| format!("{}: {e}", cache.display()))?;
    let file = cache.join(&a.file);
    if file.is_file() && sha256_file(&file)? == a.sha256 {
        say(format!("{} {}: usando o que já foi baixado", a.name, a.version));
    } else {
        say(format!("Baixando {} {} ({}, {} MB)…", a.name, a.version, a.file, a.megabytes()));
        download(&a.url, &file, &a.sha256)?;
    }
    say(format!("SHA-256 confere: {}", a.sha256));
    let dest = tools.join(&a.dest);
    if a.extract {
        // Extrai numa pasta nova e troca no fim: nada de arquivos velhos misturados, e uma
        // extração que falha não estraga a versão que já funcionava.
        let fresh = tools.join(format!("{}.new-{}", a.dest, std::process::id()));
        let _ = std::fs::remove_dir_all(&fresh);
        say(format!("Extraindo em {}…", dest.display()));
        if let Err(e) = extract(&file, &fresh) {
            let _ = std::fs::remove_dir_all(&fresh);
            return Err(e);
        }
        if dest.exists() && std::fs::remove_dir_all(&dest).is_err() {
            // Não deu para apagar (um arquivo em uso): a anterior sai do caminho com outro nome.
            let old = tools.join(format!("{}.old-{}", a.dest, std::process::id()));
            std::fs::rename(&dest, &old).map_err(|e| format!("não consegui tirar a versão anterior de {}: {e}", dest.display()))?;
            say(format!("A versão anterior ficou em {} (apague quando puder)", old.display()));
        }
        std::fs::rename(&fresh, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    } else {
        std::fs::create_dir_all(&tools).map_err(|e| e.to_string())?;
        std::fs::copy(&file, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    }
    say(format!("{} {} instalado em {}", a.name, a.version, dest.display()));
    Ok(dest)
}

/// Um nome de arquivo sem pasta, sem `..` e sem nada estranho.
fn plain_name(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('.') && name.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

/// No navegador (a IDE lite) não se baixa nada: o corpo vazio só para compilar.
#[cfg(target_arch = "wasm32")]
struct Body;

#[cfg(target_arch = "wasm32")]
impl Body {
    fn as_reader(&mut self) -> std::io::Empty {
        std::io::empty()
    }

    fn read_to_string(&mut self) -> std::io::Result<String> {
        Ok(String::new())
    }
}

#[cfg(target_arch = "wasm32")]
fn get(url: &str) -> Result<Body, String> {
    Err(format!("{url}: baixar não existe no navegador"))
}

#[cfg(not(target_arch = "wasm32"))]
fn get(url: &str) -> Result<ureq::Body, String> {
    let response = ureq::get(url).header("User-Agent", "aylis-ide").call().map_err(|e| format!("{url}: {e}"))?;
    Ok(response.into_body())
}

/// Baixa `url` em `to` conferindo o SHA-256 no caminho: se não bater, apaga e devolve erro.
fn download(url: &str, to: &Path, sha256: &str) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err(format!("{url}: só https"));
    }
    let part = to.with_extension("part");
    let mut body = get(url)?;
    let mut reader = body.as_reader();
    let mut out = File::create(&part).map_err(|e| format!("{}: {e}", part.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = reader.read(&mut buf).map_err(|e| format!("{url}: {e}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        out.write_all(&buf[..n]).map_err(|e| e.to_string())?;
    }
    drop(out);
    let got = hex(&hasher.finalize());
    if got != sha256 {
        let _ = std::fs::remove_file(&part);
        return Err(format!("o arquivo baixado de {url} não confere (SHA-256 {got}, o fixado é {sha256}): não foi usado"));
    }
    std::fs::rename(&part, to).map_err(|e| format!("{}: {e}", to.display()))
}

/// Extrai um `.zip`/`.vsix` ou `.tar.gz` em `to`. Entradas que sairiam da pasta (`..`,
/// caminho absoluto) são recusadas.
fn extract(archive: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| e.to_string())?;
    let name = archive.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    let file = File::open(archive).map_err(|e| format!("{}: {e}", archive.display()))?;
    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(file));
        tar.set_preserve_permissions(cfg!(unix));
        for entry in tar.entries().map_err(|e| e.to_string())? {
            let mut entry = entry.map_err(|e| e.to_string())?;
            // `unpack_in` não escreve fora de `to`.
            entry.unpack_in(to).map_err(|e| e.to_string())?;
        }
        return Ok(());
    }
    let mut zip = zip::ZipArchive::new(file).map_err(|e| format!("{}: {e}", archive.display()))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        let Some(relative) = entry.enclosed_name() else {
            return Err(format!("{}: entrada fora da pasta ({})", archive.display(), entry.name().unwrap_or_default()));
        };
        let path = to.join(relative);
        if entry.is_dir() {
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut out = File::create(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// `aylis-tool update-tools`: regrava o lock.

/// De onde vem cada arquivo.
enum Source {
    /// Um release do GitHub (`dono/repo`); o hash vem do `digest` do arquivo no release.
    GitHub(&'static str),
    /// Um endereço fixo (`{v}` = a versão); o hash vem do `<endereço>.sha256`.
    Url(&'static str),
}

struct Spec {
    name: &'static str,
    source: Source,
    /// Plataforma (`windows-x86_64`, `linux`, `any`...) e o nome do arquivo (`{tag}` = a tag
    /// do release, `{v}` = a versão sem o `v`).
    files: &'static [(&'static str, &'static str)],
    dest: &'static str,
    extract: bool,
    /// O `--latest` muda a versão (as com teto ficam).
    follow_latest: bool,
}

const SPECS: &[Spec] = &[
    Spec {
        name: "codelldb",
        source: Source::GitHub("vadimcn/codelldb"),
        files: &[
            ("windows-x86_64", "codelldb-win32-x64.vsix"),
            ("windows-aarch64", "codelldb-win32-arm64.vsix"),
            ("linux-x86_64", "codelldb-linux-x64.vsix"),
            ("linux-aarch64", "codelldb-linux-arm64.vsix"),
            ("macos-x86_64", "codelldb-darwin-x64.vsix"),
            ("macos-aarch64", "codelldb-darwin-arm64.vsix"),
        ],
        dest: "codelldb",
        extract: true,
        follow_latest: true,
    },
    Spec {
        name: "netcoredbg",
        source: Source::GitHub("Samsung/netcoredbg"),
        files: &[
            ("windows-x86_64", "netcoredbg-win64.zip"),
            ("linux-x86_64", "netcoredbg-linux-amd64.tar.gz"),
            ("linux-aarch64", "netcoredbg-linux-arm64.tar.gz"),
            ("macos-aarch64", "netcoredbg-osx-arm64.zip"),
        ],
        dest: "netcoredbg",
        extract: true,
        follow_latest: true,
    },
    Spec {
        name: "js-debug",
        source: Source::GitHub("microsoft/vscode-js-debug"),
        files: &[("any", "js-debug-dap-{tag}.tar.gz")],
        dest: "js-debug",
        extract: true,
        follow_latest: true,
    },
    Spec {
        name: "clangd",
        source: Source::GitHub("clangd/clangd"),
        files: &[("windows", "clangd-windows-{tag}.zip"), ("linux", "clangd-linux-{tag}.zip"), ("macos", "clangd-mac-{tag}.zip")],
        dest: "clangd",
        extract: true,
        follow_latest: true,
    },
    Spec {
        name: "bundletool",
        source: Source::GitHub("google/bundletool"),
        files: &[("any", "bundletool-all-{tag}.jar")],
        dest: "bundletool.jar",
        extract: false,
        follow_latest: true,
    },
    // O 1.38 é o último que roda no Java 17 (os de depois pedem o 21).
    Spec {
        name: "jdtls",
        source: Source::Url("https://download.eclipse.org/jdtls/milestones/{v}/jdt-language-server-1.38.0-202408011337.tar.gz"),
        files: &[("any", "jdt-language-server-1.38.0-202408011337.tar.gz")],
        dest: "jdtls",
        extract: true,
        follow_latest: false,
    },
];

fn get_json(url: &str) -> Result<Value, String> {
    let mut body = get(url)?;
    let text = body.read_to_string().map_err(|e| format!("{url}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{url}: {e}"))
}

#[cfg(target_arch = "wasm32")]
fn content_length(_url: &str) -> u64 {
    0
}

/// O tamanho anunciado de `url` (0 se o servidor não disser).
#[cfg(not(target_arch = "wasm32"))]
fn content_length(url: &str) -> u64 {
    ureq::head(url)
        .header("User-Agent", "aylis-ide")
        .call()
        .ok()
        .and_then(|r| r.headers().get("content-length").and_then(|v| v.to_str().ok()).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn get_text(url: &str) -> Result<String, String> {
    get(url)?.read_to_string().map_err(|e| format!("{url}: {e}"))
}

/// A última versão de cada pacote (o `--latest`). Os de teto ficam como estão.
fn latest_packages(current: &Map<String, Value>, say: &mut dyn FnMut(String)) -> Result<Map<String, Value>, String> {
    let mut out = current.clone();
    let mut set = |name: &str, version: String, say: &mut dyn FnMut(String)| {
        if current.get(name).and_then(Value::as_str) != Some(version.as_str()) {
            say(format!("{name}: {} → {version}", current.get(name).and_then(Value::as_str).unwrap_or("-")));
        }
        out.insert(name.into(), Value::String(version));
    };
    let go = |module: &str| -> Result<String, String> {
        get_json(&format!("https://proxy.golang.org/{module}/@latest"))?["Version"].as_str().map(str::to_string).ok_or_else(|| format!("{module}: sem versão"))
    };
    set("gopls", go("golang.org/x/tools/gopls")?, say);
    set("delve", go("github.com/go-delve/delve")?, say);
    let pypi = get_json("https://pypi.org/pypi/python-lsp-server/json")?;
    set("python-lsp-server", pypi["info"]["version"].as_str().ok_or("pypi: sem versão")?.to_string(), say);
    let tsls = get_json("https://registry.npmjs.org/typescript-language-server/latest")?;
    set("typescript-language-server", tsls["version"].as_str().ok_or("npm: sem versão")?.to_string(), say);
    // O TypeScript fica no 5 (o 7 não tem o tsserver que o servidor usa).
    let ts = get_json("https://registry.npmjs.org/typescript")?;
    let newest_5 = ts["versions"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.keys())
        .filter(|v| v.starts_with("5.") && !v.contains('-'))
        .max_by_key(|v| v.split('.').map(|n| n.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>())
        .cloned()
        .ok_or("npm: sem TypeScript 5")?;
    set("typescript", newest_5, say);
    // O csharp-ls mais novo pede o .NET mais novo; os de teto (csharp-ls-net9, -net8) ficam.
    let nuget = get_json("https://api.nuget.org/v3-flatcontainer/csharp-ls/index.json")?;
    let csharp = nuget["versions"].as_array().and_then(|v| v.last()).and_then(Value::as_str).ok_or("nuget: sem versão")?;
    set("csharp-ls", csharp.to_string(), say);
    let commit = get_json("https://api.github.com/repos/castle-engine/pascal-language-server/commits/master")?;
    set("pascal-language-server", commit["sha"].as_str().ok_or("github: sem commit")?.to_string(), say);
    Ok(out)
}

/// Regrava `resources/tools.lock.json` em `root` (o repositório do aylis): os hashes
/// publicados de cada arquivo; com `latest`, antes passa para a última versão.
pub fn update_lock(root: &Path, latest: bool, say: &mut dyn FnMut(String)) -> Result<(), String> {
    let old = lock();
    let mut archives = Map::new();
    for spec in SPECS {
        let mut version = old["archives"][spec.name]["version"].as_str().unwrap_or_default().to_string();
        let release = match &spec.source {
            Source::GitHub(repo) => {
                let url = if latest && spec.follow_latest || version.is_empty() {
                    format!("https://api.github.com/repos/{repo}/releases/latest")
                } else {
                    format!("https://api.github.com/repos/{repo}/releases/tags/{version}")
                };
                let release = get_json(&url)?;
                let tag = release["tag_name"].as_str().ok_or_else(|| format!("{repo}: sem tag"))?.to_string();
                if tag != version {
                    say(format!("{}: {} → {tag}", spec.name, if version.is_empty() { "-" } else { &version }));
                }
                version = tag;
                Some(release)
            }
            Source::Url(_) => {
                if version.is_empty() {
                    version = "1.38.0".into();
                }
                None
            }
        };
        let bare = version.trim_start_matches('v');
        let mut files = Map::new();
        for (platform, template) in spec.files {
            let file = template.replace("{tag}", &version).replace("{v}", bare);
            let (url, sha256, size) = match (&spec.source, &release) {
                (Source::GitHub(_), Some(release)) => {
                    let Some(asset) = release["assets"].as_array().and_then(|a| a.iter().find(|a| a["name"] == file.as_str())) else {
                        say(format!("{}: {file} não está no release {version} (fica sem {platform})", spec.name));
                        continue;
                    };
                    let digest = asset["digest"].as_str().and_then(|d| d.strip_prefix("sha256:")).ok_or_else(|| format!("{}: {file} sem o SHA-256 no GitHub", spec.name))?;
                    (asset["browser_download_url"].as_str().unwrap_or_default().to_string(), digest.to_string(), asset["size"].as_u64().unwrap_or(0))
                }
                (Source::Url(template), _) => {
                    let url = template.replace("{v}", bare);
                    let sha = get_text(&format!("{url}.sha256"))?.split_whitespace().next().unwrap_or_default().to_lowercase();
                    (url.clone(), sha, content_length(&url))
                }
                _ => unreachable!(),
            };
            if sha256.len() != 64 || !sha256.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("{}: SHA-256 inválido para {file}", spec.name));
            }
            files.insert(platform.to_string(), json!({ "name": file, "url": url, "sha256": sha256, "size": size }));
        }
        archives.insert(spec.name.into(), json!({ "version": version, "dest": spec.dest, "extract": spec.extract, "files": files }));
    }
    let packages = old["packages"].as_object().cloned().unwrap_or_default();
    let packages = if latest { latest_packages(&packages, say)? } else { packages };
    let lock = json!({
        "_": "Gerado por `aylis-tool update-tools [--latest]`: versões fixas e o SHA-256 de cada arquivo que a IDE baixa.",
        "archives": archives,
        "packages": packages,
    });
    let path = root.join("resources/tools.lock.json");
    std::fs::write(&path, serde_json::to_string_pretty(&lock).map_err(|e| e.to_string())? + "\n").map_err(|e| format!("{}: {e}", path.display()))?;
    say(format!("{} atualizado (recompile o aylis-tool e a IDE)", path.display()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lock_has_a_file_and_a_hash_for_each_tool() {
        let lock = lock();
        for spec in SPECS {
            let files = lock["archives"][spec.name]["files"].as_object().unwrap_or_else(|| panic!("{} no lock", spec.name));
            assert!(!files.is_empty(), "{}", spec.name);
            for f in files.values() {
                let sha = f["sha256"].as_str().unwrap();
                assert!(sha.len() == 64 && sha.chars().all(|c| c.is_ascii_hexdigit()), "{}: {sha}", spec.name);
                assert!(f["url"].as_str().unwrap().starts_with("https://"));
                assert!(plain_name(f["name"].as_str().unwrap()));
            }
        }
        for package in ["gopls", "delve", "python-lsp-server", "typescript", "typescript-language-server", "csharp-ls", "csharp-ls-net9", "csharp-ls-net8", "pascal-language-server"] {
            assert!(!version(package).is_empty(), "{package}");
        }
    }

    #[test]
    fn names_with_folders_are_refused() {
        assert!(plain_name("clangd-windows-23.1.0.zip"));
        for bad in ["../x.zip", "a/b.zip", r"a\b.zip", ".hidden", ""] {
            assert!(!plain_name(bad), "{bad}");
        }
    }

    #[test]
    fn a_zip_entry_outside_the_folder_is_refused() {
        let dir = std::env::temp_dir().join(format!("aylis_zip_slip_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let archive = dir.join("bad.zip");
        let mut zip = zip::ZipWriter::new(File::create(&archive).unwrap());
        zip.start_file("../fora.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"x").unwrap();
        zip.finish().unwrap();
        let err = extract(&archive, &dir.join("out")).unwrap_err();
        assert!(err.contains("fora da pasta"), "{err}");
        assert!(!dir.join("fora.txt").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
