//! A fachada antiga das ferramentas baixadas (depuradores, servidores de linguagem, o
//! bundletool): agora são pacotes do [`crate::toolpacks`], no mesmo manifesto
//! (`resources/packages.lock.json`, sucessor do `tools.lock.json`), com a versão fixa e o
//! SHA-256 de cada arquivo. O que vem por gerenciador de pacotes (go, pip, npm, dotnet, git)
//! usa a versão fixada em [`version`]; o gerenciador confere a integridade.
//!
//! [`fetch`] (o `aylis-tool fetch`) instala o pacote pelas origens dos pacotes (a pasta
//! `pacotes`, as pastas e os espelhos das Opções). Nunca cai na URL oficial do manifesto
//! (decisão do autor em docs/pacotes.md): só o `pack` e o `update` a usam.
//!
//! Mudar de versão: `aylis-tool update-tools` regrava estes pacotes no manifesto com os hashes
//! publicados (os do GitHub, ou o `.sha256` ao lado do arquivo); com `--latest`, passa antes
//! para a última versão de cada um (os que têm teto, como o JDT LS que ainda roda no Java 17,
//! ficam).

use std::io::Read;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use crate::toolpacks::{Installer, Manifest, Progress};

/// A versão fixada de um pacote de gerenciador (`gopls`, `delve`, `python-lsp-server`,
/// `typescript`...).
pub fn version(package: &str) -> String {
    Manifest::embedded().manager_version(package)
}

/// O arquivo de uma ferramenta para esta plataforma.
#[derive(Clone, Debug)]
pub struct Archive {
    pub name: String,
    pub version: String,
    /// O nome do arquivo (`codelldb-win32-x64.vsix`).
    pub file: String,
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

impl Archive {
    pub fn megabytes(&self) -> u64 {
        self.size.div_ceil(1024 * 1024)
    }
}

/// O arquivo do pacote `name` para esta plataforma (o primeiro, se tiver vários).
pub fn archive(name: &str) -> Option<Archive> {
    let p = Manifest::embedded().package(name)?;
    let f = p.files.first()?;
    Some(Archive { name: p.name.clone(), version: p.version.clone(), file: f.name.clone(), url: f.url.clone().unwrap_or_default(), sha256: f.sha256.clone(), size: p.size() })
}

/// Instala o pacote `name` pelas origens dos pacotes e devolve a pasta onde ficou.
pub fn fetch(name: &str, say: &mut dyn FnMut(String)) -> Result<PathBuf, String> {
    let installer = Installer::new()?;
    let control = crate::process::Control::default();
    installer.install(&[name], &control, &mut |p| {
        if let Progress::Step(s) = p {
            say(s)
        }
    })?;
    installer.installed_dir(name).ok_or_else(|| format!("{name}: não ficou instalado"))
}

// ---------------------------------------------------------------------------
// `aylis-tool update-tools`: regrava estes pacotes no manifesto.

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
        follow_latest: true,
    },
    Spec { name: "js-debug", source: Source::GitHub("microsoft/vscode-js-debug"), files: &[("any", "js-debug-dap-{tag}.tar.gz")], follow_latest: true },
    Spec {
        name: "clangd",
        source: Source::GitHub("clangd/clangd"),
        files: &[("windows", "clangd-windows-{tag}.zip"), ("linux", "clangd-linux-{tag}.zip"), ("macos", "clangd-mac-{tag}.zip")],
        follow_latest: true,
    },
    Spec { name: "bundletool", source: Source::GitHub("google/bundletool"), files: &[("any", "bundletool-all-{tag}.jar")], follow_latest: true },
    // O 1.38 é o último que roda no Java 17 (os de depois pedem o 21).
    Spec {
        name: "jdtls",
        source: Source::Url("https://download.eclipse.org/jdtls/milestones/{v}/jdt-language-server-1.38.0-202408011337.tar.gz"),
        files: &[("any", "jdt-language-server-1.38.0-202408011337.tar.gz")],
        follow_latest: false,
    },
];

#[cfg(target_arch = "wasm32")]
fn get(url: &str) -> Result<Box<dyn Read>, String> {
    Err(format!("{url}: baixar não existe no navegador"))
}

#[cfg(not(target_arch = "wasm32"))]
fn get(url: &str) -> Result<Box<dyn Read>, String> {
    let response = ureq::get(url).header("User-Agent", "aylis-ide").call().map_err(|e| format!("{url}: {e}"))?;
    Ok(Box::new(response.into_body().into_reader()))
}

fn get_text(url: &str) -> Result<String, String> {
    let mut s = String::new();
    get(url)?.take(16 << 20).read_to_string(&mut s).map_err(|e| format!("{url}: {e}"))?;
    Ok(s)
}

fn get_json(url: &str) -> Result<Value, String> {
    serde_json::from_str(&get_text(url)?).map_err(|e| format!("{url}: {e}"))
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

/// A última versão de cada pacote de gerenciador (o `--latest`). Os de teto ficam.
fn latest_managers(current: &Map<String, Value>, say: &mut dyn FnMut(String)) -> Result<Map<String, Value>, String> {
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

/// Regrava, no `resources/packages.lock.json` de `root`, os pacotes de [`SPECS`] com os
/// hashes publicados de cada arquivo (o resto de cada pacote, como o título e o `PATH`, fica);
/// com `latest`, antes passa para a última versão, e também a dos pacotes de gerenciador.
pub fn update_lock(root: &Path, latest: bool, say: &mut dyn FnMut(String)) -> Result<(), String> {
    let path = root.join("resources/packages.lock.json");
    let mut lock: Value = serde_json::from_str(&std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?).map_err(|e| e.to_string())?;
    for spec in SPECS {
        let entry = &mut lock["packages"][spec.name];
        let mut version = entry["version"].as_str().unwrap_or_default().to_string();
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
        let old_platforms = entry["platforms"].as_object().cloned().unwrap_or_default();
        let mut platforms = Map::new();
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
            if !crate::toolpacks::is_sha256(&sha256) {
                return Err(format!("{}: SHA-256 inválido para {file}", spec.name));
            }
            // Os campos de instalação (`strip`, `save_as`...) do arquivo anterior ficam.
            let mut f = old_platforms[*platform]["files"][0].as_object().cloned().unwrap_or_default();
            f.insert("name".into(), json!(file));
            f.insert("size".into(), json!(size));
            f.insert("sha256".into(), json!(sha256));
            f.insert("url".into(), json!(url));
            platforms.insert(platform.to_string(), json!({ "files": [f] }));
        }
        entry["version"] = json!(version);
        entry["platforms"] = Value::Object(platforms);
    }
    if latest {
        let managers = lock["managers"].as_object().cloned().unwrap_or_default();
        lock["managers"] = Value::Object(latest_managers(&managers, say)?);
    }
    let text = serde_json::to_string_pretty(&lock).map_err(|e| e.to_string())? + "\n";
    Manifest::parse(&text)?;
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    say(format!("{} atualizado (recompile o aylis-tool e a IDE)", path.display()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lock_has_a_file_and_a_hash_for_each_tool() {
        let m = Manifest::embedded();
        for spec in SPECS {
            let p = m.package_for(spec.name, "windows-x86_64").unwrap_or_else(|| panic!("{} no manifesto", spec.name));
            assert!(!p.platforms.is_empty(), "{}", spec.name);
            for plat in &p.platforms {
                for f in m.package_for(spec.name, plat).unwrap().files {
                    assert!(crate::toolpacks::is_sha256(&f.sha256), "{}: {}", spec.name, f.sha256);
                    assert!(f.url.unwrap().starts_with("https://"));
                }
            }
        }
        for package in ["gopls", "delve", "python-lsp-server", "typescript", "typescript-language-server", "csharp-ls", "csharp-ls-net9", "csharp-ls-net8", "pascal-language-server"] {
            assert!(!version(package).is_empty(), "{package}");
        }
    }

    #[test]
    fn the_old_archives_keep_their_files() {
        let a = archive("bundletool").expect("bundletool");
        assert!(a.file.ends_with(".jar") && a.size > 0);
        let p = Manifest::embedded().package("bundletool").unwrap();
        assert_eq!(p.files[0].save_as.as_deref(), Some("bundletool.jar"));
        assert!(!p.files[0].extract);
    }
}
