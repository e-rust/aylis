//! Os pacotes de componente (`.aylispkg`, um ZIP) do lado da IDE: criar a partir de um form,
//! assinar com a chave do autor, conferir ao instalar, guardar por usuário e copiar para o
//! projeto que usa (veja [`aylis_project::packages`] para o formato e o uso em execução).
//!
//! - O ZIP: `package.json` (o manifesto), o form (`form.json`), o que mais o autor pôs e
//!   `signature.json` (a chave pública do autor e a assinatura Ed25519 do resumo SHA-256 de
//!   todos os outros arquivos).
//! - A chave do autor: gerada pela IDE na primeira vez, em `%LOCALAPPDATA%\aylis\author-key.json`.
//! - A confiança: `%LOCALAPPDATA%\aylis\trusted-authors.json` guarda a impressão digital de
//!   cada autor confirmada na primeira instalação; as versões seguintes são conferidas
//!   sozinhas, e uma chave diferente para o mesmo autor dá aviso.
//! - Instalados: `%LOCALAPPDATA%\aylis\packages\<Nome>\<versão>\` (várias lado a lado); o
//!   projeto usa uma, copiada para `<projeto>/packages/<Nome>/`.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use aylis_devlink::{hex, unhex32};
use aylis_project::packages::{Manifest, Version};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// A extensão do arquivo de pacote.
pub const EXTENSION: &str = "aylispkg";

fn data_dir() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.to_path_buf())
}

/// `%LOCALAPPDATA%\aylis\packages`.
pub fn store() -> Option<PathBuf> {
    Some(data_dir()?.join("packages"))
}

// ---------------------------------------------------------------------------
// Chave e assinatura

/// A chave do autor (gera e grava na primeira vez).
pub fn author_key() -> Result<SigningKey, String> {
    author_key_in(&data_dir().ok_or("sem pasta de dados do usuário")?)
}

fn author_key_in(dir: &Path) -> Result<SigningKey, String> {
    let path = dir.join("author-key.json");
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Some(secret) = serde_json::from_str::<Value>(&text).ok().and_then(|v| v["secret"].as_str().and_then(unhex32)) {
            return Ok(SigningKey::from_bytes(&secret));
        }
    }
    let mut secret = [0u8; 32];
    crate::random_bytes(&mut secret)?;
    let key = SigningKey::from_bytes(&secret);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    std::fs::write(&path, json!({ "secret": hex(&secret), "public": hex(key.verifying_key().as_bytes()) }).to_string()).map_err(|e| e.to_string())?;
    Ok(key)
}

/// A impressão digital de uma chave pública: os 8 primeiros bytes do SHA-256, em grupos
/// (`3F2A:91C0:...`), para conferir de olho.
pub fn fingerprint(public: &[u8; 32]) -> String {
    let digest = Sha256::digest(public);
    digest[..8].chunks(2).map(|c| format!("{:02X}{:02X}", c[0], c[1])).collect::<Vec<_>>().join(":")
}

/// O resumo dos arquivos (sem a assinatura), na ordem do nome.
fn digest(files: &BTreeMap<String, Vec<u8>>) -> [u8; 32] {
    let mut h = Sha256::new();
    for (name, bytes) in files.iter().filter(|(n, _)| n.as_str() != "signature.json") {
        h.update(name.as_bytes());
        h.update([0]);
        h.update(Sha256::digest(bytes));
        h.update([b'\n']);
    }
    h.finalize().into()
}

// ---------------------------------------------------------------------------
// Criar e ler

/// Cria o `.aylispkg` com o manifesto, o form e os arquivos extras, assinado com `key`.
pub fn pack(out: &Path, manifest: &Manifest, form: &Value, extra: &[(String, Vec<u8>)], key: &SigningKey) -> Result<(), String> {
    let mut files = BTreeMap::new();
    files.insert("package.json".to_string(), serde_json::to_vec_pretty(manifest).map_err(|e| e.to_string())?);
    // O de código (tipo B) não tem form: o crate vai nos extras.
    if !manifest.is_code() {
        files.insert(manifest.form_file().to_string(), serde_json::to_vec_pretty(form).map_err(|e| e.to_string())?);
    }
    for (name, bytes) in extra {
        files.insert(name.clone(), bytes.clone());
    }
    let signature = key.sign(&digest(&files));
    let public = key.verifying_key();
    files.insert(
        "signature.json".into(),
        json!({ "author": manifest.author, "public_key": hex(public.as_bytes()), "signature": hex(&signature.to_bytes()) }).to_string().into_bytes(),
    );
    let file = std::fs::File::create(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default();
    for (name, bytes) in &files {
        zip.start_file(name.as_str(), opts).map_err(|e| e.to_string())?;
        zip.write_all(bytes).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

/// Cria um pacote de código (tipo B): o crate de `crate_dir` (sem `target/` nem `.git/`)
/// vai em `crate/`, e o manifesto diz o crate e a classe que o `register(ui)` dele registra.
/// Compilado no app Rust de quem usa (a IDE põe a dependência no `Cargo.toml`).
pub fn pack_code(out: &Path, manifest: &Manifest, crate_dir: &Path, key: &SigningKey) -> Result<(), String> {
    let mut m = manifest.clone();
    m.kind = "code".into();
    if m.crate_name.is_empty() {
        m.crate_name = crate_name_of(crate_dir)?;
    }
    Manifest::parse(&serde_json::to_string(&m).map_err(|e| e.to_string())?)?;
    let lib = std::fs::read_to_string(crate_dir.join("src/lib.rs")).map_err(|e| format!("{}: {e}", crate_dir.join("src/lib.rs").display()))?;
    if !lib.contains("pub fn register(") {
        return Err("o crate precisa de `pub fn register(ui: &mut aylis::core::Ui)` no src/lib.rs (registra a classe)".into());
    }
    let mut extra = Vec::new();
    collect(crate_dir, crate_dir, &mut extra)?;
    pack(out, &m, &Value::Null, &extra, key)
}

/// O `name` do `[package]` do `Cargo.toml`.
fn crate_name_of(dir: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(dir.join("Cargo.toml")).map_err(|e| format!("{}: {e}", dir.join("Cargo.toml").display()))?;
    let mut in_package = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_package = t == "[package]";
        } else if in_package {
            if let Some(v) = t.strip_prefix("name").map(str::trim_start).and_then(|r| r.strip_prefix('=')) {
                return Ok(v.trim().trim_matches('"').to_string());
            }
        }
    }
    Err("Cargo.toml sem [package] name".into())
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
        let path = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if !matches!(name.as_str(), "target" | ".git") {
                collect(root, &path, out)?;
            }
            continue;
        }
        let rel = path.strip_prefix(root).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/");
        out.push((format!("crate/{rel}"), std::fs::read(&path).map_err(|e| e.to_string())?));
    }
    Ok(())
}

/// Um pacote lido (ainda não instalado).
#[derive(Debug)]
pub struct Package {
    pub manifest: Manifest,
    pub files: BTreeMap<String, Vec<u8>>,
    /// A chave pública de quem assinou, se a assinatura confere; `None` se não tem.
    pub signer: Option<[u8; 32]>,
}

impl Package {
    /// O que a pergunta de confiança mostra: os arquivos e o tamanho.
    pub fn contents(&self) -> Vec<String> {
        self.files.iter().map(|(n, b)| format!("{n} ({} bytes)", b.len())).collect()
    }
}

/// Lê e confere um `.aylispkg`. Assinatura que não confere é erro (o arquivo foi mexido).
pub fn read(path: &Path) -> Result<Package, String> {
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut zip = zip::ZipArchive::new(f).map_err(|e| format!("não é um .aylispkg: {e}"))?;
    let mut files = BTreeMap::new();
    let mut total = 0usize;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() {
            continue;
        }
        let Some(name) = entry.enclosed_name().map(|p| p.to_string_lossy().replace('\\', "/")) else {
            return Err("o pacote tem um caminho fora dele".into());
        };
        total += entry.size() as usize;
        if total > 100 * 1024 * 1024 {
            return Err("o pacote passa de 100 MB".into());
        }
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        files.insert(name, bytes);
    }
    let manifest = Manifest::parse(std::str::from_utf8(files.get("package.json").ok_or("o pacote não tem package.json")?).map_err(|e| e.to_string())?)?;
    if manifest.is_code() && !files.contains_key("crate/Cargo.toml") {
        return Err("o pacote de código não tem o crate (crate/Cargo.toml)".into());
    }
    if !manifest.is_code() && !files.contains_key(manifest.form_file()) {
        return Err(format!("o pacote não tem o form {}", manifest.form_file()));
    }
    let signer = match files.get("signature.json") {
        None => None,
        Some(bytes) => {
            let v: Value = serde_json::from_slice(bytes).map_err(|e| format!("signature.json: {e}"))?;
            let public = v["public_key"].as_str().and_then(unhex32).ok_or("signature.json sem chave")?;
            let sig: [u8; 64] = v["signature"].as_str().and_then(unhex64).ok_or("signature.json sem assinatura")?;
            let key = VerifyingKey::from_bytes(&public).map_err(|e| e.to_string())?;
            key.verify(&digest(&files), &Signature::from_bytes(&sig)).map_err(|_| "a assinatura não confere: o pacote foi alterado depois de assinado")?;
            Some(public)
        }
    };
    Ok(Package { manifest, files, signer })
}

fn unhex64(text: &str) -> Option<[u8; 64]> {
    if text.len() != 128 {
        return None;
    }
    let mut out = [0u8; 64];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(text.get(i * 2..i * 2 + 2)?, 16).ok()?;
    }
    Some(out)
}

// ---------------------------------------------------------------------------
// Confiança

/// O que a IDE sabe do autor de um pacote.
#[derive(Clone, Debug, PartialEq)]
pub enum Trust {
    /// Sem assinatura.
    Unsigned,
    /// Primeira vez deste autor: confirmar a impressão digital.
    NewAuthor { fingerprint: String },
    /// A chave é a mesma das outras vezes.
    Known,
    /// O autor já veio com outra chave: pode ser outra pessoa com o mesmo nome.
    Changed { expected: String, got: String },
}

fn trusted_path(dir: &Path) -> PathBuf {
    dir.join("trusted-authors.json")
}

/// O que se sabe de quem assinou o pacote.
pub fn trust_of(package: &Package) -> Trust {
    data_dir().map_or(Trust::Unsigned, |d| trust_in(&d, package))
}

fn trust_in(dir: &Path, package: &Package) -> Trust {
    let Some(public) = package.signer else { return Trust::Unsigned };
    let got = fingerprint(&public);
    let known: BTreeMap<String, String> = std::fs::read_to_string(trusted_path(dir)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    match known.get(&package.manifest.author) {
        None => Trust::NewAuthor { fingerprint: got },
        Some(expected) if *expected == got => Trust::Known,
        Some(expected) => Trust::Changed { expected: expected.clone(), got },
    }
}

/// Guarda a impressão digital do autor (depois que o usuário confirmou).
pub fn trust_author(package: &Package) -> Result<(), String> {
    trust_author_in(&data_dir().ok_or("sem pasta de dados")?, package)
}

fn trust_author_in(dir: &Path, package: &Package) -> Result<(), String> {
    let Some(public) = package.signer else { return Ok(()) };
    let path = trusted_path(dir);
    let mut known: BTreeMap<String, String> = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    known.insert(package.manifest.author.clone(), fingerprint(&public));
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    std::fs::write(&path, serde_json::to_string_pretty(&known).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Instalar e usar

/// Instala para o usuário (`packages\<Nome>\<versão>`). Devolve a pasta.
pub fn install(package: &Package) -> Result<PathBuf, String> {
    install_in(&store().ok_or("sem pasta de dados")?, package)
}

fn install_in(store: &Path, package: &Package) -> Result<PathBuf, String> {
    let dir = store.join(&package.manifest.name).join(&package.manifest.version);
    write_files(&dir, &package.files)?;
    Ok(dir)
}

fn write_files(dir: &Path, files: &BTreeMap<String, Vec<u8>>) -> Result<(), String> {
    for (name, bytes) in files {
        let target = dir.join(name);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&target, bytes).map_err(|e| format!("{}: {e}", target.display()))?;
    }
    Ok(())
}

/// Um pacote instalado (uma versão).
#[derive(Clone, Debug)]
pub struct Installed {
    pub manifest: Manifest,
    pub dir: PathBuf,
}

/// Os instalados, agrupados por nome (as versões da mais nova para a mais velha).
pub fn installed() -> Vec<(String, Vec<Installed>)> {
    store().map(|s| installed_in(&s)).unwrap_or_default()
}

fn installed_in(store: &Path) -> Vec<(String, Vec<Installed>)> {
    let mut out = Vec::new();
    let Ok(names) = std::fs::read_dir(store) else { return out };
    for name in names.flatten().filter(|e| e.path().is_dir()) {
        let mut versions: Vec<Installed> = std::fs::read_dir(name.path())
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|v| {
                let dir = v.path();
                let manifest = Manifest::parse(&std::fs::read_to_string(dir.join("package.json")).ok()?).ok()?;
                Some(Installed { manifest, dir })
            })
            .collect();
        versions.sort_by_key(|i| std::cmp::Reverse(Version::parse(&i.manifest.version)));
        if !versions.is_empty() {
            out.push((name.file_name().to_string_lossy().to_string(), versions));
        }
    }
    out.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    out
}

/// A versão que o projeto usa (a de `<projeto>/packages/<Nome>`).
pub fn project_version(project_dir: &Path, name: &str) -> Option<Manifest> {
    Manifest::parse(&std::fs::read_to_string(project_dir.join(aylis_project::packages::DIR).join(name).join("package.json")).ok()?).ok()
}

/// Copia a versão instalada para o projeto (troca a que estava). Devolve o que a troca
/// muda nos forms quando a versão maior muda (as notas do autor).
pub fn use_in_project(project_dir: &Path, installed: &Installed) -> Result<Vec<String>, String> {
    // O de código é um crate Rust: só no projeto em Rust (as outras linguagens rodam no
    // aylis-host, que não compila o crate).
    if installed.manifest.is_code() {
        let project = project_in(project_dir).ok_or("pacote de código: abra o projeto (.aylisproj) primeiro")?;
        if project.language != aylis_project::Language::Rust {
            return Err(format!("o pacote {} é de código (um crate Rust): só em projetos em Rust; este é em {}", installed.manifest.name, project.language.name()));
        }
    }
    let target = project_dir.join(aylis_project::packages::DIR).join(&installed.manifest.name);
    let old = project_version(project_dir, &installed.manifest.name);
    if target.exists() {
        std::fs::remove_dir_all(&target).map_err(|e| e.to_string())?;
    }
    copy_dir(&installed.dir, &target)?;
    // O crate veio com o caminho do aylis da máquina do autor: aponta para o daqui.
    if installed.manifest.is_code() {
        let cargo = target.join("crate").join("Cargo.toml");
        if let Ok(text) = std::fs::read_to_string(&cargo) {
            let local = aylis_project::codegen::aylis_path_for(&target.join("crate"));
            if let Some(fixed) = local_aylis_paths(&text, &local) {
                std::fs::write(&cargo, fixed).map_err(|e| e.to_string())?;
            }
        }
    }
    let mut notes = Vec::new();
    if let (Some(old), Some(new)) = (old.as_ref().and_then(|m| Version::parse(&m.version)), Version::parse(&installed.manifest.version)) {
        if !old.compatible(new) {
            notes.push(format!("{} {old} → {new}: versão maior", installed.manifest.name));
            for (version, note) in &installed.manifest.notes {
                if Version::parse(version).is_some_and(|v| v > old) {
                    notes.push(format!("{version}: {note}"));
                }
            }
            notes.extend(migrate_forms(project_dir, &installed.manifest));
        }
    }
    Ok(notes)
}

/// Ajusta os `values` dos frames do pacote nos forms do projeto (propriedades renomeadas ou
/// removidas numa versão maior). Devolve o que mudou, por form.
fn migrate_forms(project_dir: &Path, manifest: &Manifest) -> Vec<String> {
    fn walk(value: &mut Value, form: &str, manifest: &Manifest, notes: &mut Vec<String>) -> bool {
        let mut changed = false;
        if let Some(obj) = value.as_object_mut() {
            let mine = obj.get("form").and_then(Value::as_str).and_then(aylis_project::packages::package_of) == Some(manifest.name.as_str());
            if mine {
                let old = obj.get("values").and_then(Value::as_str).unwrap_or("").to_string();
                if !old.is_empty() {
                    let (new, what) = aylis_project::packages::migrate_values(&old, manifest);
                    let name = obj.get("name").and_then(Value::as_str).unwrap_or("Frame").to_string();
                    notes.extend(what.into_iter().map(|w| format!("{form}.{name}: {w}")));
                    if new != old {
                        obj.insert("values".into(), Value::String(new));
                        changed = true;
                    }
                }
            }
            if let Some(Value::Array(children)) = obj.get_mut("children") {
                for c in children {
                    changed |= walk(c, form, manifest, notes);
                }
            }
        }
        changed
    }
    let mut notes = Vec::new();
    for e in std::fs::read_dir(project_dir.join("forms")).into_iter().flatten().flatten() {
        let path = e.path();
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(mut doc) = serde_json::from_str::<Value>(&text) else { continue };
        let form = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if walk(&mut doc["root"], &form, manifest, &mut notes) {
            if let Ok(text) = serde_json::to_string_pretty(&doc) {
                let _ = std::fs::write(&path, text + "\n");
            }
        }
    }
    notes
}

/// Troca o `path` de cada dependência `aylis`/`aylis-*` do `Cargo.toml` pelo aylis daqui
/// (`root/crates/<nome>`). Devolve o texto novo, se mudou.
pub fn local_aylis_paths(cargo: &str, root: &str) -> Option<String> {
    let mut changed = false;
    let lines: Vec<String> = cargo
        .lines()
        .map(|line| {
            let t = line.trim_start();
            let name = t.split(['=', ' ']).next().unwrap_or_default();
            let is_aylis = name == "aylis" || name.starts_with("aylis-");
            if let (true, Some(start)) = (is_aylis, line.find("path = \"")) {
                let from = start + "path = \"".len();
                if let Some(len) = line[from..].find('"') {
                    let new = format!("{}{root}/crates/{name}{}", &line[..from], &line[from + len..]);
                    if new != line {
                        changed = true;
                        return new;
                    }
                }
            }
            line.to_string()
        })
        .collect();
    changed.then(|| lines.join("\n") + if cargo.ends_with('\n') { "\n" } else { "" })
}

/// O projeto da pasta (o primeiro `.aylisproj` dela).
fn project_in(dir: &Path) -> Option<aylis_project::Project> {
    let file = std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).find(|p| p.extension().is_some_and(|x| x == "aylisproj"))?;
    aylis_project::Project::open(file).ok()
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for e in std::fs::read_dir(from).map_err(|e| e.to_string())?.flatten() {
        let p = e.path();
        let t = to.join(e.file_name());
        if p.is_dir() {
            copy_dir(&p, &t)?;
        } else {
            std::fs::copy(&p, &t).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Manda para a Lixeira (no navegador não há).
fn to_trash(path: &Path) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    return trash::delete(path).map_err(|e| e.to_string());
    #[cfg(target_arch = "wasm32")]
    Err(format!("{}: sem Lixeira no navegador", path.display()))
}

/// Remove uma versão instalada: vai para a Lixeira (dá para voltar).
pub fn remove(installed: &Installed) -> Result<(), String> {
    to_trash(&installed.dir)?;
    // A pasta do nome, se ficou vazia.
    if let Some(parent) = installed.dir.parent() {
        if std::fs::read_dir(parent).is_ok_and(|mut d| d.next().is_none()) {
            let _ = std::fs::remove_dir(parent);
        }
    }
    Ok(())
}

/// Os forms do projeto que usam o pacote `name` (para avisar antes de remover).
pub fn forms_using(project_dir: &Path, name: &str) -> Vec<String> {
    let needle = format!("\"{}{name}\"", aylis_project::packages::PREFIX);
    let mut out = Vec::new();
    let forms = project_dir.join("forms");
    for e in std::fs::read_dir(&forms).into_iter().flatten().flatten() {
        if std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains(&needle)) {
            out.push(e.file_name().to_string_lossy().to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use aylis_project::packages::ExposedProp;

    fn manifest(version: &str) -> Manifest {
        Manifest {
            name: "Termometro".into(),
            version: version.into(),
            author: "Ana".into(),
            category: "Industrial".into(),
            kind: "composite".into(),
            properties: vec![ExposedProp { name: "valor".into(), kind: "number".into(), target: "pb.value".into(), default: json!(20), description: String::new() }],
            ..Default::default()
        }
    }

    #[test]
    fn a_signed_package_is_checked_trusted_and_installed() {
        let dir = std::env::temp_dir().join(format!("aylis-pkg-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let key = author_key_in(&dir).unwrap();
        assert_eq!(author_key_in(&dir).unwrap().to_bytes(), key.to_bytes(), "a chave fica guardada");
        let form = json!({ "aylis": 1, "root": { "class": "Form", "name": "Termometro", "children": [ { "class": "ProgressBar", "name": "pb" } ] } });
        let file = dir.join("t.aylispkg");
        pack(&file, &manifest("1.0.0"), &form, &[], &key).unwrap();
        let p = read(&file).unwrap();
        assert_eq!(p.manifest.name, "Termometro");
        assert_eq!(p.signer, Some(key.verifying_key().to_bytes()));
        assert!(matches!(trust_in(&dir, &p), Trust::NewAuthor { .. }));
        trust_author_in(&dir, &p).unwrap();
        assert_eq!(trust_in(&dir, &p), Trust::Known);
        // Outra chave com o mesmo autor: aviso.
        let other = SigningKey::from_bytes(&[7u8; 32]);
        let file2 = dir.join("t2.aylispkg");
        pack(&file2, &manifest("1.1.0"), &form, &[], &other).unwrap();
        assert!(matches!(trust_in(&dir, &read(&file2).unwrap()), Trust::Changed { .. }));
        // Mexer no arquivo depois de assinar: não lê.
        let mut bytes = std::fs::read(&file).unwrap();
        let at = bytes.windows(4).position(|w| w == b"pb\"}").unwrap_or(40);
        bytes[at] = b'q';
        std::fs::write(dir.join("mexido.aylispkg"), bytes).unwrap();
        assert!(read(&dir.join("mexido.aylispkg")).is_err());
        // Instalado por versão; o projeto usa uma e avisa a troca de versão maior.
        let store = dir.join("store");
        install_in(&store, &p).unwrap();
        let mut m2 = manifest("2.0.0");
        m2.notes.insert("2.0.0".into(), "valor virou temperatura".into());
        pack(&dir.join("t3.aylispkg"), &m2, &form, &[], &key).unwrap();
        install_in(&store, &read(&dir.join("t3.aylispkg")).unwrap()).unwrap();
        let list = installed_in(&store);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].1.iter().map(|i| i.manifest.version.clone()).collect::<Vec<_>>(), ["2.0.0", "1.0.0"]);
        let project = dir.join("proj");
        assert!(use_in_project(&project, &list[0].1[1]).unwrap().is_empty());
        std::fs::create_dir_all(project.join("forms")).unwrap();
        let form = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmUsina", "children": [ { "class": "Frame", "name": "t1", "form": "pkg:Termometro", "values": "{\"valor\": 7}" } ] } });
        std::fs::write(project.join("forms").join("usina.json"), form.to_string()).unwrap();
        m2.renamed.insert("valor".into(), "temperatura".into());
        m2.version = "2.0.1".into();
        pack(&dir.join("t4.aylispkg"), &m2, &form, &[], &key).unwrap();
        install_in(&store, &read(&dir.join("t4.aylispkg")).unwrap()).unwrap();
        let list = installed_in(&store);
        let notes = use_in_project(&project, &list[0].1[0]).unwrap();
        let saved = std::fs::read_to_string(project.join("forms").join("usina.json")).unwrap();
        assert!(saved.contains("temperatura") && notes.iter().any(|n| n.contains("valor → temperatura")), "{saved} {notes:?}");
        assert!(notes.iter().any(|n| n.contains("valor virou temperatura")), "{notes:?}");
        assert_eq!(project_version(&project, "Termometro").unwrap().version, "2.0.1");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_code_package_carries_the_crate_and_only_goes_into_rust_projects() {
        let dir = std::env::temp_dir().join(format!("aylis-pkg-code-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let krate = dir.join("medidor");
        std::fs::create_dir_all(krate.join("src")).unwrap();
        std::fs::create_dir_all(krate.join("target/debug")).unwrap();
        std::fs::write(krate.join("Cargo.toml"), "[package]\nname = \"medidor-aylis\"\nversion = \"1.0.0\"\n").unwrap();
        std::fs::write(krate.join("target/debug/lixo.rlib"), "x").unwrap();
        let key = SigningKey::from_bytes(&[3u8; 32]);
        let m = Manifest { name: "Medidor".into(), version: "1.0.0".into(), component: "Gauge".into(), ..Default::default() };
        let file = dir.join("m.aylispkg");
        // Sem o register, recusa.
        std::fs::write(krate.join("src/lib.rs"), "pub struct Gauge;\n").unwrap();
        assert!(pack_code(&file, &m, &krate, &key).unwrap_err().contains("register"));
        std::fs::write(krate.join("src/lib.rs"), "pub fn register(ui: &mut aylis::core::Ui) {}\n").unwrap();
        pack_code(&file, &m, &krate, &key).unwrap();
        let p = read(&file).unwrap();
        assert!(p.manifest.is_code());
        assert_eq!(p.manifest.crate_name, "medidor-aylis", "o nome vem do Cargo.toml");
        assert!(p.files.contains_key("crate/src/lib.rs") && p.files.contains_key("crate/Cargo.toml"), "{:?}", p.files.keys());
        assert!(!p.files.keys().any(|k| k.contains("target")), "sem o target");
        assert!(!p.files.contains_key("form.json"), "sem form");
        // Num projeto Python, recusa; num Rust, entra com o crate.
        let store = dir.join("store");
        let installed = Installed { manifest: p.manifest.clone(), dir: install_in(&store, &p).unwrap() };
        let python = aylis_project::create(&dir.join("py"), "Py", aylis_project::Language::Python).unwrap();
        assert!(use_in_project(&python.dir(), &installed).unwrap_err().contains("Rust"));
        let rust = aylis_project::create(&dir.join("rs"), "Rs", aylis_project::Language::Rust).unwrap();
        use_in_project(&rust.dir(), &installed).unwrap();
        assert!(rust.dir().join("packages/Medidor/crate/src/lib.rs").is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_crate_points_to_the_local_aylis() {
        let cargo = "[dependencies]\naylis-core = { path = \"C:/autor/aylis/crates/aylis-core\" }\naylis = { path = \"../../aylis/crates/aylis\", default-features = false }\nserde = \"1\"\n";
        let fixed = local_aylis_paths(cargo, "D:/aylis").unwrap();
        assert!(fixed.contains("aylis-core = { path = \"D:/aylis/crates/aylis-core\" }"), "{fixed}");
        assert!(fixed.contains("aylis = { path = \"D:/aylis/crates/aylis\", default-features = false }"), "{fixed}");
        assert!(fixed.contains("serde = \"1\""));
        assert_eq!(local_aylis_paths(&fixed, "D:/aylis"), None, "já certo");
    }
}
