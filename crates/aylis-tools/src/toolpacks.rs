//! Os pacotes de ferramentas (o desenho está em `docs/pacotes.md`): o Rust, o JDK, o SDK e o
//! NDK do Android, o Python, o Node, o .NET, o Go, os depuradores, os servidores de
//! linguagem... instalados pelo Aylis sem administrador e sem ir a site nenhum.
//!
//! - O manifesto embutido (`resources/packages.lock.json`, [`Manifest::embedded`]) fixa a
//!   versão, os arquivos de cada plataforma com o tamanho e o **SHA-256**, as dependências e
//!   o passo de instalação ([`Install`], um enum fixo do Aylis: nada de script do pacote).
//! - Os arquivos vêm das origens ([`Sources`]), na ordem: o cache
//!   (`%USERPROFILE%\.aylis\downloads`), as pastas de pacotes (a `pacotes` ao lado do
//!   executável e as das Opções) e os espelhos (um `pacotes.json` com o lugar de cada arquivo,
//!   uma URL ou um id do Google Drive). O primeiro com o hash certo vence; a origem nunca é
//!   confiável por si. A URL oficial do manifesto só é usada pelo `pack` e pelo `update`.
//! - Instalado vai para `%USERPROFILE%\.aylis\tools\<nome>\<versão>` (fora do `AppData`, que
//!   o Claude Desktop desvia), com o registro em `installed.json`. A troca é atômica: extrai
//!   numa pasta nova e só no fim põe no lugar.
//! - [`environment`] diz o `PATH` e as variáveis (`JAVA_HOME`, `ANDROID_HOME`...) que os
//!   processos abertos pelo Aylis ganham, só onde a máquina não tem a ferramenta
//!   (`setup::refresh_path` aplica).
//!
//! Para a IDE e o instalador:
//!
//! ```text
//! let installer = Installer::new()?;                     // o manifesto, as pastas, as origens
//! installer.status();                                    // a lista, com o que está instalado
//! installer.install(&["jdk"], &control, &mut |p| ...)?;  // dependências antes, progresso, cancelável
//! installer.remove("jdk", &mut |p| ...)?;
//! ```

pub(crate) mod fetch;
mod pack;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

pub use fetch::{CAUSE_MISMATCH, CAUSE_NOT_BESIDE, CAUSE_OFFLINE, Probe, Sources, causes};
pub use pack::{PackReport, pack, update_lock};

use crate::process::Control;

const LOCK: &str = include_str!("../../../resources/packages.lock.json");

/// Esta plataforma como no manifesto (`windows-x86_64`, `linux-x86_64`, `macos-aarch64`).
pub fn platform() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

/// O que instalar faz com os arquivos (fixo no Aylis; o pacote não traz script).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Install {
    /// Extrai cada arquivo na pasta do pacote (`into`, `strip`, `prefix` de cada um) ou, se
    /// não for compactado, copia. Serve para o JDK, o SDK do Android, o Node, as wheels do pip
    /// e os pacotes do npm (que são zip e tar.gz).
    Extract,
    /// Tarballs de componentes do Rust (`rustc`, `cargo`, `rust-std`...): extrai e junta os
    /// componentes de cada um (o que o `install.sh` deles faz) numa pasta só, um toolchain
    /// que roda sem rustup.
    RustToolchain,
    /// O `rust-std` de outros alvos, posto no toolchain do pacote do qual depende
    /// (`lib/rustlib/<alvo>`); remover tira só essas pastas.
    RustTarget(Vec<String>),
}

/// Um arquivo de um pacote numa plataforma.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileSpec {
    /// O nome (único entre todos os pacotes: é como fica no cache e na pasta de pacotes).
    pub name: String,
    pub size: u64,
    /// O SHA-256 fixado (vazio: falta o `aylis-tool packages update`).
    pub sha256: String,
    /// A URL oficial: só o `pack` e o `update` usam.
    pub url: Option<String>,
    /// Quantas pastas do começo de cada entrada tirar (`jdk-17.0.21+9/bin` → `bin`).
    pub strip: usize,
    /// Só as entradas abaixo desta pasta, sem ela (`tools` do pacote NuGet do Python).
    pub prefix: Option<String>,
    /// A subpasta do pacote onde extrair (`platforms/android-34`); vazia, a raiz.
    pub into: String,
    /// Extrair (zip, vsix, nupkg, whl, tar.gz) ou copiar como está.
    pub extract: bool,
    /// O nome com que fica copiado, se não for extraído (`bundletool.jar`).
    pub save_as: Option<String>,
}

impl FileSpec {
    pub fn megabytes(&self) -> u64 {
        self.size.div_ceil(1024 * 1024)
    }
}

/// O que um pacote instalado acrescenta ao ambiente dos processos (relativo à pasta dele).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Exports {
    /// Pastas que entram no `PATH` (no fim: o que a máquina já tem vem antes).
    pub path: Vec<String>,
    /// Variáveis (`JAVA_HOME` → `.`), definidas só se a máquina não tiver a ferramenta.
    pub env: Vec<(String, String)>,
}

/// Um pacote do manifesto, visto de uma plataforma.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub title: String,
    pub description: String,
    pub version: String,
    pub depends: Vec<String>,
    pub install: Install,
    /// A chave do manifesto que valeu para esta plataforma (`windows-x86_64`, `windows`,
    /// `any`); `None` se o pacote não existe nela.
    pub platform: Option<String>,
    pub files: Vec<FileSpec>,
    pub exports: Exports,
    /// Todas as plataformas que o pacote tem.
    pub platforms: Vec<String>,
}

impl Package {
    pub fn available(&self) -> bool {
        self.platform.is_some()
    }

    /// O tamanho dos arquivos a obter.
    pub fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    /// O nome da pasta da versão (`17.0.21+9` → `17.0.21_9`).
    pub fn folder(&self) -> String {
        folder_name(&self.version)
    }
}

fn folder_name(version: &str) -> String {
    let s: String = version.chars().map(|c| if c.is_ascii_alphanumeric() || "-_.".contains(c) { c } else { '_' }).collect();
    let s = s.trim_start_matches('.').to_string();
    if s.is_empty() { "0".into() } else { s }
}

#[derive(Clone, Debug)]
struct PlatformEntry {
    key: String,
    files: Vec<FileSpec>,
    exports: Option<Exports>,
}

#[derive(Clone, Debug)]
struct RawPackage {
    name: String,
    title: String,
    description: String,
    version: String,
    depends: Vec<String>,
    install: Install,
    exports: Exports,
    platforms: Vec<PlatformEntry>,
}

/// O manifesto dos pacotes (`resources/packages.lock.json`).
#[derive(Clone, Debug)]
pub struct Manifest {
    packages: Vec<RawPackage>,
    managers: Map<String, Value>,
}

impl Manifest {
    /// O embutido no Aylis.
    pub fn embedded() -> Manifest {
        static CELL: std::sync::OnceLock<Manifest> = std::sync::OnceLock::new();
        CELL.get_or_init(|| Manifest::parse(LOCK).expect("resources/packages.lock.json válido")).clone()
    }

    /// Lê e confere um manifesto (nomes de arquivo sem pasta, hashes, caminhos relativos,
    /// dependências que existem).
    pub fn parse(text: &str) -> Result<Manifest, String> {
        let root: Value = serde_json::from_str(text).map_err(|e| format!("manifesto de pacotes: {e}"))?;
        let mut packages = Vec::new();
        for (name, p) in root["packages"].as_object().ok_or("manifesto de pacotes sem \"packages\"")? {
            packages.push(parse_package(name, p).map_err(|e| format!("pacote {name}: {e}"))?);
        }
        let names: Vec<&str> = packages.iter().map(|p| p.name.as_str()).collect();
        for p in &packages {
            for d in &p.depends {
                if !names.contains(&d.as_str()) {
                    return Err(format!("pacote {}: depende de {d}, que não existe", p.name));
                }
            }
        }
        let managers = root["managers"].as_object().cloned().unwrap_or_default();
        Ok(Manifest { packages, managers })
    }

    /// Os nomes, na ordem do manifesto.
    pub fn names(&self) -> Vec<String> {
        self.packages.iter().map(|p| p.name.clone()).collect()
    }

    /// O pacote `name` nesta plataforma.
    pub fn package(&self, name: &str) -> Option<Package> {
        self.package_for(name, &platform())
    }

    /// O pacote `name` numa plataforma (a exata, depois só o sistema, depois `any`).
    pub fn package_for(&self, name: &str, platform: &str) -> Option<Package> {
        let raw = self.packages.iter().find(|p| p.name == name)?;
        let os = platform.split('-').next().unwrap_or_default();
        let entry = [platform, os, "any"].iter().find_map(|k| raw.platforms.iter().find(|e| e.key == *k));
        Some(Package {
            name: raw.name.clone(),
            title: raw.title.clone(),
            description: raw.description.clone(),
            version: raw.version.clone(),
            depends: raw.depends.clone(),
            install: raw.install.clone(),
            platform: entry.map(|e| e.key.clone()),
            files: entry.map(|e| e.files.clone()).unwrap_or_default(),
            exports: entry.and_then(|e| e.exports.clone()).unwrap_or_else(|| raw.exports.clone()),
            platforms: raw.platforms.iter().map(|e| e.key.clone()).collect(),
        })
    }

    /// Todos os pacotes, vistos desta plataforma.
    pub fn packages(&self) -> Vec<Package> {
        self.names().iter().filter_map(|n| self.package(n)).collect()
    }

    /// A versão fixada de algo que vem por gerenciador (`gopls`, `typescript`...).
    pub fn manager_version(&self, name: &str) -> String {
        self.managers.get(name).and_then(Value::as_str).unwrap_or_default().to_string()
    }

    /// `names` e as dependências deles, cada dependência antes de quem a usa.
    pub fn with_dependencies(&self, names: &[&str]) -> Result<Vec<String>, String> {
        fn visit(m: &Manifest, name: &str, stack: &mut Vec<String>, out: &mut Vec<String>) -> Result<(), String> {
            if out.iter().any(|n| n == name) {
                return Ok(());
            }
            if stack.iter().any(|n| n == name) {
                return Err(format!("dependência circular: {} → {name}", stack.join(" → ")));
            }
            let raw = m.packages.iter().find(|p| p.name == name).ok_or_else(|| format!("{name}: pacote desconhecido (aylis-tool packages list mostra os que existem)"))?;
            stack.push(name.to_string());
            for d in &raw.depends {
                visit(m, d, stack, out)?;
            }
            stack.pop();
            out.push(name.to_string());
            Ok(())
        }
        let mut out = Vec::new();
        for n in names {
            visit(self, n, &mut Vec::new(), &mut out)?;
        }
        Ok(out)
    }
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn parse_exports(v: &Value) -> Result<Exports, String> {
    let path = strings(&v["path"]);
    let env: Vec<(String, String)> = v["env"].as_object().map(|m| m.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or(".").to_string())).collect()).unwrap_or_default();
    for p in path.iter().chain(env.iter().map(|(_, v)| v)) {
        safe_relative(p)?;
    }
    for (k, _) in &env {
        if k.is_empty() || !k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!("variável inválida: {k}"));
        }
    }
    Ok(Exports { path, env })
}

fn parse_package(name: &str, p: &Value) -> Result<RawPackage, String> {
    if !plain_name(name) {
        return Err("nome inválido".into());
    }
    let install = match &p["install"] {
        Value::String(s) if s == "extract" => Install::Extract,
        Value::String(s) if s == "rust-toolchain" => Install::RustToolchain,
        Value::Object(m) if m.contains_key("rust-target") => {
            let targets = strings(&m["rust-target"]);
            if targets.is_empty() || !targets.iter().all(|t| plain_name(t)) {
                return Err("rust-target sem alvos válidos".into());
            }
            Install::RustTarget(targets)
        }
        other => return Err(format!("passo de instalação desconhecido: {other}")),
    };
    let mut platforms = Vec::new();
    for (key, entry) in p["platforms"].as_object().ok_or("sem \"platforms\"")? {
        let mut files = Vec::new();
        for f in entry["files"].as_array().ok_or_else(|| format!("{key}: sem \"files\""))? {
            files.push(parse_file(f).map_err(|e| format!("{key}: {e}"))?);
        }
        let exports = if entry.get("exports").is_some() { Some(parse_exports(&entry["exports"])?) } else { None };
        platforms.push(PlatformEntry { key: key.clone(), files, exports });
    }
    let depends = strings(&p["depends"]);
    if matches!(install, Install::RustTarget(_)) && depends.is_empty() {
        return Err("rust-target precisa depender do pacote do toolchain".into());
    }
    Ok(RawPackage {
        name: name.to_string(),
        title: p["title"].as_str().unwrap_or(name).to_string(),
        description: p["description"].as_str().unwrap_or_default().to_string(),
        version: p["version"].as_str().ok_or("sem \"version\"")?.to_string(),
        depends,
        install,
        exports: parse_exports(&p["exports"])?,
        platforms,
    })
}

fn parse_file(f: &Value) -> Result<FileSpec, String> {
    let name = f["name"].as_str().unwrap_or_default().to_string();
    if !plain_name(&name) {
        return Err(format!("nome de arquivo inválido: {name:?}"));
    }
    let sha256 = f["sha256"].as_str().unwrap_or_default().to_lowercase();
    if !sha256.is_empty() && !is_sha256(&sha256) {
        return Err(format!("{name}: SHA-256 inválido"));
    }
    let url = f["url"].as_str().map(str::to_string);
    if let Some(u) = &url
        && !u.starts_with("https://")
    {
        return Err(format!("{name}: a URL oficial tem de ser https"));
    }
    let into = f["into"].as_str().unwrap_or_default().to_string();
    safe_relative(&into)?;
    let prefix = f["prefix"].as_str().map(str::to_string);
    if let Some(p) = &prefix {
        safe_relative(p)?;
    }
    let save_as = f["save_as"].as_str().map(str::to_string);
    if let Some(s) = &save_as
        && !plain_name(s)
    {
        return Err(format!("{name}: save_as inválido"));
    }
    let extract = f["extract"].as_bool().unwrap_or_else(|| archive_kind(&name).is_some());
    Ok(FileSpec { name, size: f["size"].as_u64().unwrap_or(0), sha256, url, strip: f["strip"].as_u64().unwrap_or(0) as usize, prefix, into, extract, save_as })
}

/// Um nome de arquivo sem pasta, sem `..` e sem nada estranho (`+` vale: `jdk-17.0.21+9`).
pub(crate) fn plain_name(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('.') && name.chars().all(|c| c.is_ascii_alphanumeric() || "-_.+".contains(c))
}

pub(crate) fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Um caminho relativo que não sai da pasta (`a/b`, `.`, vazio).
fn safe_relative(p: &str) -> Result<(), String> {
    let path = Path::new(p);
    if path.components().all(|c| matches!(c, Component::Normal(_) | Component::CurDir)) && !p.contains(':') {
        Ok(())
    } else {
        Err(format!("caminho fora da pasta: {p:?}"))
    }
}

// ---------------------------------------------------------------------------
// As pastas e o registro.

/// Onde ficam os pacotes instalados e o cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Paths {
    /// `%USERPROFILE%\.aylis\tools` (cada pacote em `<nome>\<versão>`, e o `installed.json`).
    pub tools: PathBuf,
    /// `%USERPROFILE%\.aylis\downloads`.
    pub cache: PathBuf,
}

impl Paths {
    /// As do usuário: `%USERPROFILE%\.aylis` no Windows (fora do `AppData`, que um programa
    /// MSIX como o Claude Desktop desvia para quem ele abre), `~/.aylis` nos outros.
    pub fn user() -> Option<Paths> {
        let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })?;
        let base = PathBuf::from(home).join(".aylis");
        Some(Paths { tools: base.join("tools"), cache: base.join("downloads") })
    }

    pub fn registry(&self) -> PathBuf {
        self.tools.join("installed.json")
    }
}

/// Um pacote instalado (uma linha do `installed.json`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Installed {
    pub name: String,
    pub version: String,
    /// A pasta (num `RustTarget`, a do toolchain onde ele entrou).
    pub dir: PathBuf,
    pub platform: String,
}

fn read_registry(paths: &Paths) -> Vec<Installed> {
    let Some(v) = std::fs::read_to_string(paths.registry()).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else {
        return Vec::new();
    };
    v["packages"]
        .as_object()
        .map(|m| {
            m.iter()
                .filter_map(|(name, e)| {
                    Some(Installed {
                        name: name.clone(),
                        version: e["version"].as_str()?.to_string(),
                        dir: PathBuf::from(e["dir"].as_str()?),
                        platform: e["platform"].as_str().unwrap_or_default().to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Grava o registro inteiro: escreve ao lado e troca (quem lê nunca vê meio arquivo).
fn write_registry(paths: &Paths, list: &[Installed]) -> Result<(), String> {
    std::fs::create_dir_all(&paths.tools).map_err(|e| format!("{}: {e}", paths.tools.display()))?;
    let mut packages = Map::new();
    for i in list {
        packages.insert(i.name.clone(), json!({ "version": i.version, "dir": i.dir.display().to_string(), "platform": i.platform }));
    }
    let text = serde_json::to_string_pretty(&json!({ "aylis": 1, "packages": packages })).map_err(|e| e.to_string())? + "\n";
    let tmp = paths.tools.join(format!("installed.json.tmp-{}", std::process::id()));
    std::fs::write(&tmp, text).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, paths.registry()).map_err(|e| format!("{}: {e}", paths.registry().display()))
}

/// Uma trava curta para ler e regravar o registro (a IDE e o `aylis-tool` ao mesmo tempo).
struct RegistryLock(PathBuf);

impl RegistryLock {
    fn take(paths: &Paths) -> Result<RegistryLock, String> {
        std::fs::create_dir_all(&paths.tools).map_err(|e| format!("{}: {e}", paths.tools.display()))?;
        let path = paths.tools.join("installed.lock");
        let started = std::time::Instant::now();
        loop {
            match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(_) => return Ok(RegistryLock(path)),
                Err(_) => {
                    // Uma trava esquecida (um processo que caiu) não segura para sempre.
                    let stale = std::fs::metadata(&path).and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|d| d.as_secs() > 60);
                    if stale {
                        let _ = std::fs::remove_file(&path);
                    } else if started.elapsed().as_secs() > 15 {
                        return Err(format!("o registro de pacotes está travado por outra instalação ({})", path.display()));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}

impl Drop for RegistryLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

// ---------------------------------------------------------------------------
// Instalar e remover.

/// O andamento, para a IDE e o instalador mostrarem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Progress {
    /// Uma etapa ("Instalando o jdk 17.0.21+9…").
    Step(String),
    /// Bytes de um arquivo obtido (baixado ou copiado de uma pasta), de `from`.
    Bytes { file: String, from: String, done: u64, total: u64 },
}

/// O manifesto, as pastas e as origens: tudo o que instalar e remover precisa.
#[derive(Clone, Debug)]
pub struct Installer {
    pub manifest: Manifest,
    pub paths: Paths,
    pub sources: Sources,
}

/// Um pacote e, se estiver, como está instalado (a lista da tela de pacotes).
#[derive(Clone, Debug)]
pub struct Status {
    pub package: Package,
    pub installed: Option<Installed>,
}

impl Status {
    /// Instalado na versão do manifesto.
    pub fn up_to_date(&self) -> bool {
        self.installed.as_ref().is_some_and(|i| i.version == self.package.version)
    }
}

impl Installer {
    /// O manifesto embutido, as pastas do usuário e as origens das Opções.
    pub fn new() -> Result<Installer, String> {
        Ok(Installer { manifest: Manifest::embedded(), paths: Paths::user().ok_or("sem a pasta do usuário (USERPROFILE/HOME)")?, sources: Sources::configured() })
    }

    pub fn installed(&self) -> Vec<Installed> {
        read_registry(&self.paths)
    }

    /// A pasta do pacote `name`, se estiver instalado.
    pub fn installed_dir(&self, name: &str) -> Option<PathBuf> {
        self.installed().into_iter().find(|i| i.name == name).map(|i| i.dir).filter(|d| d.is_dir())
    }

    /// Todos os pacotes desta plataforma com o que está instalado.
    pub fn status(&self) -> Vec<Status> {
        let installed = self.installed();
        self.manifest.packages().into_iter().map(|p| Status { installed: installed.iter().find(|i| i.name == p.name).cloned(), package: p }).collect()
    }

    /// Instala `names` e as dependências (antes). O que já está na versão do manifesto fica.
    /// Devolve o que foi instalado agora.
    pub fn install(&self, names: &[&str], control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<Vec<Installed>, String> {
        self.install_except(names, &[], control, progress)
    }

    /// O [`install`](Self::install) sem as dependências de `skip`: as que a máquina já tem no
    /// sistema (o JDK do bundletool, o Rust do cargo-ndk), que o pacote não precisa trazer. Os
    /// de `names` são instalados mesmo se estiverem em `skip`.
    pub fn install_except(&self, names: &[&str], skip: &[String], control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<Vec<Installed>, String> {
        let mut order = self.manifest.with_dependencies(names)?;
        order.retain(|n| names.contains(&n.as_str()) || !skip.contains(n));
        for n in &order {
            let p = self.manifest.package(n).ok_or_else(|| format!("{n}: pacote desconhecido"))?;
            if !p.available() {
                return Err(format!("{n}: não há {} para {} (só {})", p.title, platform(), p.platforms.join(", ")));
            }
        }
        let mut done = Vec::new();
        for n in &order {
            control.check()?;
            let p = self.manifest.package(n).expect("conferido acima");
            if let Some(i) = self.current(&p) {
                progress(Progress::Step(format!("{} {} já está instalado em {}", p.name, p.version, i.dir.display())));
                continue;
            }
            let installed = self.install_one(&p, control, progress)?;
            done.push(installed);
            // Os alvos do Rust moram dentro do toolchain: um toolchain novo leva os que já
            // estavam instalados (do cache, conferidos de novo).
            for dependent in self.installed() {
                let Some(dp) = self.manifest.package(&dependent.name) else { continue };
                if matches!(dp.install, Install::RustTarget(_)) && dp.depends.contains(&p.name) && !order.contains(&dp.name) {
                    progress(Progress::Step(format!("Recolocando {} no toolchain novo…", dp.name)));
                    done.push(self.install_one(&dp, control, progress)?);
                }
            }
        }
        Ok(done)
    }

    /// Instalado na versão do manifesto, com a pasta (e os alvos do Rust) no lugar.
    fn current(&self, p: &Package) -> Option<Installed> {
        let i = self.installed().into_iter().find(|i| i.name == p.name && i.version == p.version)?;
        let ok = match &p.install {
            Install::RustTarget(targets) => targets.iter().all(|t| i.dir.join("lib/rustlib").join(t).is_dir()),
            _ => i.dir.is_dir(),
        };
        ok.then_some(i)
    }

    fn install_one(&self, p: &Package, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<Installed, String> {
        progress(Progress::Step(format!("Instalando {} {} ({} MB)…", p.name, p.version, p.size().div_ceil(1024 * 1024))));
        let mut archives = Vec::new();
        for f in &p.files {
            control.check()?;
            archives.push(fetch::obtain(f, &self.sources, &self.paths.cache, control, progress)?);
        }
        let base = self.paths.tools.join(&p.name);
        std::fs::create_dir_all(&base).map_err(|e| format!("{}: {e}", base.display()))?;
        let staging = base.join(format!(".new-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&staging);
        let result = self.unpack(p, &archives, &staging, control, progress).and_then(|()| self.put_in_place(p, &staging, progress));
        let _ = std::fs::remove_dir_all(&staging);
        // Num alvo do Rust a pasta própria fica vazia (o que importa entrou no toolchain).
        let _ = std::fs::remove_dir(&base);
        let dir = result?;
        let installed = Installed { name: p.name.clone(), version: p.version.clone(), dir, platform: platform() };
        let previous = {
            let _lock = RegistryLock::take(&self.paths)?;
            let mut list = self.installed();
            let previous = list.iter().find(|i| i.name == p.name).cloned();
            list.retain(|i| i.name != p.name);
            list.push(installed.clone());
            write_registry(&self.paths, &list)?;
            previous
        };
        // A versão anterior sai (num alvo do Rust a pasta é a do toolchain: fica).
        if let Some(old) = previous
            && old.dir != installed.dir
            && !matches!(p.install, Install::RustTarget(_))
            && old.dir.starts_with(&base)
        {
            let _ = std::fs::remove_dir_all(&old.dir);
        }
        progress(Progress::Step(format!("{} {} instalado em {}", p.name, p.version, installed.dir.display())));
        Ok(installed)
    }

    /// Extrai os arquivos em `staging` conforme o passo do pacote.
    fn unpack(&self, p: &Package, archives: &[PathBuf], staging: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
        std::fs::create_dir_all(staging).map_err(|e| format!("{}: {e}", staging.display()))?;
        match &p.install {
            Install::Extract => {
                for (f, archive) in p.files.iter().zip(archives) {
                    let to = staging.join(&f.into);
                    if f.extract {
                        progress(Progress::Step(format!("Extraindo {}…", f.name)));
                        extract(archive, &to, f.strip, f.prefix.as_deref(), control)?;
                    } else {
                        std::fs::create_dir_all(&to).map_err(|e| format!("{}: {e}", to.display()))?;
                        let dest = to.join(f.save_as.as_deref().unwrap_or(&f.name));
                        std::fs::copy(archive, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
                    }
                }
            }
            Install::RustToolchain | Install::RustTarget(_) => {
                for (i, (f, archive)) in p.files.iter().zip(archives).enumerate() {
                    progress(Progress::Step(format!("Extraindo {}…", f.name)));
                    let part = staging.join(format!(".part-{i}"));
                    extract(archive, &part, 0, None, control)?;
                    merge_rust_components(&part, staging)?;
                    let _ = std::fs::remove_dir_all(&part);
                }
            }
        }
        Ok(())
    }

    /// Põe o extraído no lugar: a pasta da versão (troca atômica) ou, num alvo do Rust, as
    /// pastas `lib/rustlib/<alvo>` no toolchain.
    fn put_in_place(&self, p: &Package, staging: &Path, progress: &mut dyn FnMut(Progress)) -> Result<PathBuf, String> {
        if let Install::RustTarget(targets) = &p.install {
            let toolchain = p
                .depends
                .iter()
                .find_map(|d| self.installed_dir(d))
                .ok_or_else(|| format!("{}: o toolchain ({}) não está instalado", p.name, p.depends.join(", ")))?;
            for t in targets {
                let from = staging.join("lib/rustlib").join(t);
                if !from.is_dir() {
                    return Err(format!("{}: os arquivos não têm lib/rustlib/{t}", p.name));
                }
                let to = toolchain.join("lib/rustlib").join(t);
                swap_dir(&from, &to, progress)?;
            }
            return Ok(toolchain);
        }
        let dest = self.paths.tools.join(&p.name).join(p.folder());
        swap_dir(staging, &dest, progress)?;
        Ok(dest)
    }

    /// Remove `name`. Recusa se outro pacote instalado depende dele.
    pub fn remove(&self, name: &str, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
        let installed = self.installed();
        let me = installed.iter().find(|i| i.name == name).cloned().ok_or_else(|| format!("{name}: não está instalado"))?;
        let users: Vec<&str> = installed.iter().filter(|i| self.manifest.package(&i.name).is_some_and(|p| p.depends.iter().any(|d| d == name))).map(|i| i.name.as_str()).collect();
        if !users.is_empty() {
            return Err(format!("{name}: {} depende dele (remova antes)", users.join(", ")));
        }
        match self.manifest.package(name).map(|p| p.install) {
            Some(Install::RustTarget(targets)) => {
                for t in targets {
                    remove_dir(&me.dir.join("lib/rustlib").join(&t), progress)?;
                }
            }
            _ => {
                let base = self.paths.tools.join(name);
                if !me.dir.starts_with(&base) {
                    return Err(format!("{name}: a pasta registrada ({}) não é a do pacote; não apaguei", me.dir.display()));
                }
                remove_dir(&me.dir, progress)?;
                let _ = std::fs::remove_dir(&base);
            }
        }
        let _lock = RegistryLock::take(&self.paths)?;
        let mut list = self.installed();
        list.retain(|i| i.name != name);
        write_registry(&self.paths, &list)?;
        progress(Progress::Step(format!("{name} {} removido", me.version)));
        Ok(())
    }

    /// O que os pacotes instalados acrescentam ao ambiente.
    pub fn environment(&self) -> Environment {
        let mut env = Environment::default();
        for i in self.installed() {
            let Some(p) = self.manifest.package_for(&i.name, &i.platform) else { continue };
            let at = |d: &str| if d == "." || d.is_empty() { i.dir.clone() } else { i.dir.join(d) };
            for d in &p.exports.path {
                let dir = at(d);
                if dir.is_dir() && !env.path.contains(&dir) {
                    env.path.push(dir);
                }
            }
            for (k, v) in &p.exports.env {
                env.vars.push((k.clone(), at(v)));
            }
        }
        env
    }
}

/// O `PATH` e as variáveis dos pacotes instalados.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Environment {
    /// Pastas para o fim do `PATH`.
    pub path: Vec<PathBuf>,
    /// Todas as variáveis que os pacotes definem.
    pub vars: Vec<(String, PathBuf)>,
}

impl Environment {
    /// As variáveis que valem agora: só as que a máquina não tem (ou que apontam para uma
    /// pasta que não existe), e só se a ferramenta do sistema não foi achada. Um pacote do
    /// Aylis não toma o lugar do que já está instalado. Chame antes de mexer no `PATH`.
    pub fn wanted_vars(&self) -> Vec<(String, PathBuf)> {
        self.vars
            .iter()
            .filter(|(k, v)| {
                let current = std::env::var_os(k).map(PathBuf::from);
                if current.as_ref().is_some_and(|c| c.exists()) || !v.exists() {
                    return false;
                }
                match k.as_str() {
                    "JAVA_HOME" => crate::env::java_bin().is_none(),
                    "ANDROID_HOME" => crate::env::android_sdk().is_none(),
                    "ANDROID_NDK_HOME" => crate::env::android_sdk().and_then(|s| crate::env::android_ndk(&s)).is_none(),
                    "DOTNET_ROOT" => crate::which("dotnet").is_none(),
                    _ => true,
                }
            })
            .cloned()
            .collect()
    }
}

/// O ambiente dos pacotes instalados do usuário (vazio sem a pasta do usuário).
pub fn environment() -> Environment {
    Installer::new().map(|i| i.environment()).unwrap_or_default()
}

/// A pasta do pacote `name` do usuário, se instalado.
pub fn installed_dir(name: &str) -> Option<PathBuf> {
    Installer::new().ok()?.installed_dir(name)
}

/// Troca `to` por `from`: a anterior sai do caminho com outro nome e só então é apagada;
/// se não der para apagar (um arquivo em uso), fica ao lado, com aviso.
fn swap_dir(from: &Path, to: &Path, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let mut old = None;
    if to.exists() {
        let name = to.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let aside = to.with_file_name(format!(".old-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&aside);
        std::fs::rename(to, &aside).map_err(|e| format!("não consegui tirar a versão anterior de {} (em uso?): {e}", to.display()))?;
        old = Some(aside);
    }
    if let Err(e) = std::fs::rename(from, to) {
        // Volta a anterior: a instalação que falha não estraga a que funcionava.
        if let Some(aside) = &old {
            let _ = std::fs::rename(aside, to);
        }
        return Err(format!("{}: {e}", to.display()));
    }
    if let Some(aside) = old
        && std::fs::remove_dir_all(&aside).is_err()
    {
        progress(Progress::Step(format!("A versão anterior ficou em {} (apague quando puder)", aside.display())));
    }
    Ok(())
}

fn remove_dir(dir: &Path, progress: &mut dyn FnMut(Progress)) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    if std::fs::remove_dir_all(dir).is_err() {
        let name = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let aside = dir.with_file_name(format!(".old-{name}-{}", std::process::id()));
        std::fs::rename(dir, &aside).map_err(|e| format!("não consegui remover {} (em uso?): {e}", dir.display()))?;
        let _ = std::fs::remove_dir_all(&aside);
        if aside.exists() {
            progress(Progress::Step(format!("Ficou em {} o que estava em uso (apague quando puder)", aside.display())));
        }
    }
    Ok(())
}

/// Junta os componentes de um tarball do Rust extraído em `part` (`<pasta>/components` lista
/// os componentes, cada um numa subpasta) em `to`, sem o `manifest.in` de cada um.
fn merge_rust_components(part: &Path, to: &Path) -> Result<(), String> {
    let top = std::fs::read_dir(part)
        .map_err(|e| format!("{}: {e}", part.display()))?
        .flatten()
        .map(|e| e.path())
        .find(|p| p.join("components").is_file())
        .ok_or_else(|| format!("{}: não é um tarball de componentes do Rust (sem o arquivo components)", part.display()))?;
    let list = std::fs::read_to_string(top.join("components")).map_err(|e| e.to_string())?;
    for component in list.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !plain_name(component) {
            return Err(format!("componente com nome inválido: {component:?}"));
        }
        let dir = top.join(component);
        for entry in std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?.flatten() {
            if entry.file_name() == "manifest.in" {
                continue;
            }
            merge_move(&entry.path(), &to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Move `from` para `to`, juntando pastas que já existem.
fn merge_move(from: &Path, to: &Path) -> Result<(), String> {
    if from.is_dir() {
        if !to.exists() {
            return std::fs::rename(from, to).map_err(|e| format!("{}: {e}", to.display()));
        }
        for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?.flatten() {
            merge_move(&entry.path(), &to.join(entry.file_name()))?;
        }
        return Ok(());
    }
    if to.exists() {
        std::fs::remove_file(to).map_err(|e| format!("{}: {e}", to.display()))?;
    }
    std::fs::rename(from, to).map_err(|e| format!("{}: {e}", to.display()))
}

// ---------------------------------------------------------------------------
// Hash e extração.

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn sha256_file(path: &Path) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf).map_err(|e| format!("{}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArchiveKind {
    Zip,
    TarGz,
}

fn archive_kind(name: &str) -> Option<ArchiveKind> {
    let n = name.to_lowercase();
    if [".zip", ".vsix", ".nupkg", ".whl"].iter().any(|e| n.ends_with(e)) {
        Some(ArchiveKind::Zip)
    } else if n.ends_with(".tar.gz") || n.ends_with(".tgz") {
        Some(ArchiveKind::TarGz)
    } else {
        None
    }
}

/// O caminho de uma entrada dentro de `to`: recusa o que sairia da pasta (`..`, absoluto,
/// unidade), aplica o `prefix` e o `strip`. `Ok(None)`: a entrada fica de fora.
fn entry_target(raw: &Path, strip: usize, prefix: Option<&str>) -> Result<Option<PathBuf>, String> {
    let mut parts = Vec::new();
    for c in raw.components() {
        match c {
            Component::Normal(s) => parts.push(s.to_os_string()),
            Component::CurDir => {}
            _ => return Err(format!("entrada fora da pasta ({})", raw.display())),
        }
    }
    if let Some(prefix) = prefix {
        let want: Vec<_> = Path::new(prefix).components().filter_map(|c| if let Component::Normal(s) = c { Some(s.to_os_string()) } else { None }).collect();
        if parts.len() < want.len() || parts[..want.len()] != want[..] {
            return Ok(None);
        }
        parts.drain(..want.len());
    }
    if parts.len() <= strip {
        return Ok(None);
    }
    Ok(Some(parts[strip..].iter().collect()))
}

/// Extrai um zip (`.zip`, `.vsix`, `.nupkg`, `.whl`) ou um `.tar.gz` em `to`.
pub(crate) fn extract(archive: &Path, to: &Path, strip: usize, prefix: Option<&str>, control: &Control) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    let name = archive.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let file = File::open(archive).map_err(|e| format!("{}: {e}", archive.display()))?;
    let at = |e: String| format!("{name}: {e}");
    match archive_kind(&name).ok_or_else(|| format!("{name}: formato que o Aylis não extrai"))? {
        ArchiveKind::TarGz => {
            let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(std::io::BufReader::new(file)));
            for entry in tar.entries().map_err(|e| at(e.to_string()))? {
                control.check()?;
                let mut entry = entry.map_err(|e| at(e.to_string()))?;
                let raw = entry.path().map_err(|e| at(e.to_string()))?.into_owned();
                let Some(rel) = entry_target(&raw, strip, prefix).map_err(at)? else { continue };
                let path = to.join(rel);
                let kind = entry.header().entry_type();
                if kind.is_dir() {
                    std::fs::create_dir_all(&path).map_err(|e| format!("{}: {e}", path.display()))?;
                } else if kind.is_file() || kind == tar::EntryType::Continuous {
                    write_entry(&mut entry, &path)?;
                    #[cfg(unix)]
                    if let Ok(mode) = entry.header().mode() {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode));
                    }
                } else if kind.is_symlink() {
                    // Um atalho relativo que fica dentro da pasta (os do JDK e do Node no
                    // Linux); no Windows, os pacotes não têm.
                    let target = entry.link_name().map_err(|e| at(e.to_string()))?.map(|t| t.into_owned()).unwrap_or_default();
                    let inside = path.parent().map(|p| p.join(&target)).and_then(|p| normalize(&p)).is_some_and(|p| p.starts_with(to));
                    if !inside || target.is_absolute() {
                        return Err(at(format!("atalho para fora da pasta ({} → {})", raw.display(), target.display())));
                    }
                    #[cfg(unix)]
                    {
                        if let Some(parent) = path.parent() {
                            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                        }
                        let _ = std::fs::remove_file(&path);
                        std::os::unix::fs::symlink(&target, &path).map_err(|e| format!("{}: {e}", path.display()))?;
                    }
                }
            }
        }
        ArchiveKind::Zip => {
            let mut zip = zip::ZipArchive::new(std::io::BufReader::new(file)).map_err(|e| at(e.to_string()))?;
            for i in 0..zip.len() {
                control.check()?;
                let mut entry = zip.by_index(i).map_err(|e| at(e.to_string()))?;
                let Some(raw) = entry.enclosed_name() else {
                    return Err(at(format!("entrada fora da pasta ({})", entry.name().unwrap_or_default())));
                };
                let Some(rel) = entry_target(&raw, strip, prefix).map_err(at)? else { continue };
                let path = to.join(rel);
                if entry.is_dir() {
                    std::fs::create_dir_all(&path).map_err(|e| format!("{}: {e}", path.display()))?;
                    continue;
                }
                write_entry(&mut entry, &path)?;
                #[cfg(unix)]
                if let Some(mode) = entry.unix_mode() {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode));
                }
            }
        }
    }
    Ok(())
}

fn write_entry(reader: &mut dyn Read, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let mut out = File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    std::io::copy(reader, &mut out).map_err(|e| format!("{}: {e}", path.display()))?;
    out.flush().map_err(|e| e.to_string())
}

/// `a/b/../c` → `a/c` sem ir ao disco (`None` se subir além da raiz).
fn normalize(path: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    Some(out)
}
