//! O índice da pasta de compilação: que projeto usou que unidade do cargo, e quando.
//!
//! Toda compilação do aylis passa por [`crate::process::cargo`] com `--message-format=json`, e
//! o cargo anuncia nela cada unidade que entrou, também as que já estavam prontas. Cada
//! unidade tem o hash de 16 dígitos que a limpeza conhece (veja [`crate::cleanup`]); aqui
//! fica, por pasta de projeto (a do `Cargo.toml`), o hash, o alvo (pacote, tipo, nome e
//! perfil), se é do próprio projeto ou compartilhada (o aylis, as dependências) e a última vez
//! que o projeto a usou. Gravado em `<target>/.aylis-index.json`:
//!
//! ```json
//! { "format": 1, "projects": { "C:\\app\\rust": { "built": 1730800000, "units": {
//!     "0123456789abcdef": { "used": 1730800000, "own": true, "target": "debug/app/bin/app" } } } } }
//! ```
//!
//! Com ele a limpeza separa o lixo (o que nenhum projeto usa mais: a versão anterior do Rust
//! ou de uma dependência, um projeto apagado) do que é de um projeto e do que é compartilhado,
//! cada um com o seu prazo.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};

/// O arquivo do índice, dentro da pasta de compilação.
pub const FILE: &str = ".aylis-index.json";
const LOCK: &str = ".aylis-index.lock";

/// Uma unidade usada por um projeto.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitUse {
    /// Segundos desde 1970 da última compilação do projeto que a usou.
    pub used: u64,
    /// Do próprio projeto (o código dele, os testes e os exemplos), não do aylis nem de uma
    /// dependência.
    pub own: bool,
    /// O alvo: perfil, pacote, tipo e nome (`debug/app/bin/app`). Duas unidades do mesmo alvo
    /// no mesmo projeto: a mais nova substitui a outra.
    pub target: String,
}

/// O que se sabe de um projeto.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProjectEntry {
    /// A última compilação.
    pub built: u64,
    /// Hash → uso.
    pub units: HashMap<String, UnitUse>,
}

/// O índice inteiro: pasta do projeto → o que ele usou.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Index {
    pub projects: BTreeMap<String, ProjectEntry>,
}

pub fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// A chave de um projeto: a pasta, como o sistema a escreve por inteiro.
pub fn project_key(dir: &Path) -> String {
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let text = dir.display().to_string();
    text.strip_prefix(r"\\?\").map(str::to_string).unwrap_or(text)
}

impl Index {
    pub fn load(target: &Path) -> Index {
        let Some(v) = std::fs::read_to_string(target.join(FILE)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else {
            return Index::default();
        };
        let mut index = Index::default();
        for (dir, p) in v["projects"].as_object().into_iter().flatten() {
            let mut entry = ProjectEntry { built: p["built"].as_u64().unwrap_or(0), units: HashMap::new() };
            for (hash, u) in p["units"].as_object().into_iter().flatten() {
                entry.units.insert(
                    hash.clone(),
                    UnitUse { used: u["used"].as_u64().unwrap_or(0), own: u["own"].as_bool().unwrap_or(false), target: u["target"].as_str().unwrap_or_default().to_string() },
                );
            }
            index.projects.insert(dir.clone(), entry);
        }
        index
    }

    pub fn save(&self, target: &Path) -> std::io::Result<()> {
        let mut projects = Map::new();
        for (dir, p) in &self.projects {
            let units: BTreeMap<&String, Value> = p.units.iter().map(|(h, u)| (h, json!({ "used": u.used, "own": u.own, "target": u.target }))).collect();
            projects.insert(dir.clone(), json!({ "built": p.built, "units": units }));
        }
        let text = serde_json::to_string_pretty(&json!({ "format": 1, "projects": projects })).unwrap_or_default();
        // Grava ao lado e troca: um índice pela metade não é lido.
        let tmp = target.join(format!("{FILE}.tmp"));
        std::fs::write(&tmp, text + "\n")?;
        std::fs::rename(&tmp, target.join(FILE))
    }
}

/// A trava do índice (entre a IDE, o `aylis-tool` e a limpeza): solta ao sair do escopo.
pub fn lock(target: &Path) -> Option<File> {
    std::fs::create_dir_all(target).ok()?;
    let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(target.join(LOCK)).ok()?;
    file.lock().ok()?;
    Some(file)
}

/// O projeto em `dir` ainda existe (o `Cargo.toml` está lá).
pub fn alive(dir: &str) -> bool {
    Path::new(dir).join("Cargo.toml").is_file()
}

/// Os dias deste projeto, se o `.aylisproj` dele disser (`output.cleanup_days`): o da própria
/// pasta ou o da pasta de cima (o código fica em `<projeto>/rust`).
pub fn project_days(dir: &str) -> Option<u64> {
    let dir = Path::new(dir);
    for folder in [Some(dir), dir.parent()].into_iter().flatten() {
        for e in std::fs::read_dir(folder).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "aylisproj") {
                let days = std::fs::read_to_string(&p).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v["output"]["cleanup_days"].as_u64());
                if days.is_some() {
                    return days;
                }
            }
        }
    }
    None
}

/// Anota as unidades de uma compilação feita em `dir`.
pub fn record(target: &Path, dir: &Path, units: &[(String, bool, String)]) {
    if units.is_empty() {
        return;
    }
    let Some(_lock) = lock(target) else { return };
    let mut index = Index::load(target);
    let now = now_secs();
    let entry = index.projects.entry(project_key(dir)).or_default();
    entry.built = now;
    for (hash, own, unit_target) in units {
        entry.units.insert(hash.clone(), UnitUse { used: now, own: *own, target: unit_target.clone() });
    }
    let _ = index.save(target);
}

/// O perfil de um arquivo da pasta de compilação, relativo a ela (`debug`,
/// `x86_64-linux-android/release`): a pasta acima de `deps`, `build` ou `examples`, ou a do
/// próprio arquivo.
fn profile_of(target: &Path, file: &Path) -> String {
    let Some(d) = profile_dir(file) else { return String::new() };
    // O cargo escreve `app..	argetdebug`: as duas por inteiro.
    let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let (d, target) = (canon(&d), canon(target));
    let rel = d.strip_prefix(&target).unwrap_or(&d);
    rel.to_string_lossy().replace('\\', "/")
}

/// A pasta do perfil (a que tem `.fingerprint`) de um arquivo da pasta de compilação.
fn profile_dir(file: &Path) -> Option<PathBuf> {
    file.ancestors().skip(1).find(|d| d.join(".fingerprint").is_dir()).map(Path::to_path_buf)
}

/// O hash do arquivo, quando está no nome (`deps/libaylis_core-<hash>.rlib`,
/// `deps/teste-<hash>.exe`).
pub fn hash_of(file: &Path) -> Option<String> {
    let name = file.file_name()?.to_string_lossy().to_string();
    crate::cleanup::name_and_hash(&name).map(|(_, h)| h.to_string())
}

/// O hash de um executável ou biblioteca sem hash no nome (`debug/app.exe`: no Windows nem a
/// cópia de `deps` tem): o da pasta `.fingerprint/<pacote>-<hash>` do perfil com o arquivo
/// `<tipo>-<nome>`; com mais de uma (outras features), a usada por último.
pub fn fingerprint_hash(profile: &Path, package: &str, kind: &str, name: &str) -> Option<String> {
    let file = format!("{kind}-{name}");
    let mut best: Option<(SystemTime, String)> = None;
    for e in std::fs::read_dir(profile.join(".fingerprint")).into_iter().flatten().flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        let Some((pkg, h)) = crate::cleanup::name_and_hash(&n) else { continue };
        if pkg != package {
            continue;
        }
        let Ok(modified) = std::fs::metadata(e.path().join(&file)).and_then(|m| m.modified()) else { continue };
        if best.as_ref().is_none_or(|(t, _)| modified > *t) {
            best = Some((modified, h.to_string()));
        }
    }
    best.map(|(_, h)| h)
}

/// Junta as unidades das mensagens do cargo de uma compilação em `dir`.
#[derive(Default)]
pub struct Collector {
    units: Vec<(String, bool, String)>,
    /// Pacote → é do projeto (para a execução do build script, que não diz o manifesto).
    own_packages: HashMap<String, bool>,
}

impl Collector {
    pub fn message(&mut self, json: &Value, target: &Path, dir: &Path) {
        let package = json["package_id"].as_str().unwrap_or_default().to_string();
        match json["reason"].as_str() {
            Some("compiler-artifact") => {
                let manifest = json["manifest_path"].as_str().map(PathBuf::from);
                let own = manifest.as_deref().and_then(Path::parent).is_some_and(|m| inside(m, dir));
                self.own_packages.insert(package.clone(), own);
                let files: Vec<PathBuf> = json["filenames"].as_array().into_iter().flatten().filter_map(Value::as_str).map(PathBuf::from).collect();
                let Some(first) = files.iter().find(|f| f.extension().is_none_or(|e| e != "d" && e != "rmeta")).or(files.first()) else { return };
                let kinds: Vec<&str> = json["target"]["kind"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
                let name = json["target"]["name"].as_str().unwrap_or_default();
                let package_name = package_name(&package);
                // No nome; ou na pasta (o build script: `build/<pacote>-<hash>/build-script-build.exe`).
                let hash = hash_of(first).or_else(|| first.parent().and_then(hash_of)).or_else(|| {
                    // Copiado para fora sem hash (o executável, um exemplo, uma cdylib).
                    let kind = if kinds.contains(&"bin") { "bin" } else if kinds.contains(&"example") { "example" } else { "lib" };
                    fingerprint_hash(&profile_dir(first)?, &package_name, kind, name)
                });
                let Some(hash) = hash else { return };
                let test = if json["profile"]["test"].as_bool() == Some(true) && kinds != ["test"] { "-test" } else { "" };
                let unit_target = format!("{}/{package_name}/{}{test}/{name}", profile_of(target, first), kinds.join(","));
                self.units.push((hash, own, unit_target));
            }
            Some("build-script-executed") => {
                // A pasta `build/<pacote>-<hash>/out`: o hash da execução.
                let Some(out) = json["out_dir"].as_str().map(PathBuf::from) else { return };
                let Some(folder) = out.parent() else { return };
                let Some(hash) = folder.file_name().map(|n| n.to_string_lossy().to_string()).and_then(|n| crate::cleanup::name_and_hash(&n).map(|(_, h)| h.to_string())) else { return };
                let own = self.own_packages.get(&package).copied().unwrap_or(false);
                self.units.push((hash, own, format!("{}/{}/run-build-script", profile_of(target, folder), package_name(&package))));
            }
            _ => {}
        }
    }

    /// Grava no índice.
    pub fn finish(self, target: &Path, dir: &Path) {
        record(target, dir, &self.units);
    }

    pub fn units(&self) -> &[(String, bool, String)] {
        &self.units
    }
}

/// O nome do pacote num `package_id` (`path+file:///C:/app#app@0.1.0`,
/// `registry+https://...#serde@1.0.0`, ou o formato antigo `serde 1.0.0 (registry+...)`).
fn package_name(id: &str) -> String {
    if let Some((_, tail)) = id.rsplit_once('#') {
        let name = tail.split('@').next().unwrap_or(tail);
        // `path+file:///C:/app#0.1.0`: o nome é o da pasta.
        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return id.split('#').next().and_then(|u| u.rsplit('/').next()).unwrap_or(name).to_string();
        }
        return name.to_string();
    }
    id.split(' ').next().unwrap_or(id).to_string()
}

fn inside(path: &Path, dir: &Path) -> bool {
    let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    canon(path).starts_with(canon(dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_names() {
        assert_eq!(package_name("path+file:///C:/app/rust#olanome@0.1.0"), "olanome");
        assert_eq!(package_name("path+file:///C:/app/rust#0.1.0"), "rust");
        assert_eq!(package_name("registry+https://github.com/rust-lang/crates.io-index#serde@1.0.200"), "serde");
        assert_eq!(package_name("serde 1.0.200 (registry+https://github.com/rust-lang/crates.io-index)"), "serde");
    }

    #[test]
    fn the_index_survives_a_save() {
        let target = std::env::temp_dir().join(format!("aylis_index_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&target);
        let dir = target.join("app");
        std::fs::create_dir_all(&dir).unwrap();
        record(&target, &dir, &[("0123456789abcdef".into(), true, "debug/app/bin/app".into())]);
        let index = Index::load(&target);
        let entry = &index.projects[&project_key(&dir)];
        assert!(entry.built > 0);
        assert_eq!(entry.units["0123456789abcdef"].target, "debug/app/bin/app");
        assert!(entry.units["0123456789abcdef"].own);
        let _ = std::fs::remove_dir_all(&target);
    }

    #[test]
    fn an_executable_without_hash_finds_it_in_the_fingerprint() {
        let target = std::env::temp_dir().join(format!("aylis_index_hash_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&target);
        let profile = target.join("debug");
        for hash in ["00000000000000aa", "00000000000000bb"] {
            let fp = profile.join(".fingerprint").join(format!("meu-app-{hash}"));
            std::fs::create_dir_all(&fp).unwrap();
            std::fs::write(fp.join("bin-meu-app"), hash).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert_eq!(hash_of(&profile.join("meu-app.exe")), None);
        assert_eq!(fingerprint_hash(&profile, "meu-app", "bin", "meu-app").as_deref(), Some("00000000000000bb"), "a usada por último");
        assert_eq!(fingerprint_hash(&profile, "outro", "bin", "meu-app"), None);
        assert_eq!(hash_of(&profile.join("deps").join("meu_app-00000000000000aa.exe")).as_deref(), Some("00000000000000aa"));
        let _ = std::fs::remove_dir_all(&target);
    }
}
