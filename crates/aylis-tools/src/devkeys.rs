//! As chaves do canal de desenvolvimento (veja `aylis-devlink`): o par X25519 da IDE (um por
//! máquina, `%LOCALAPPDATA%\aylis\dev-key.json`) e o segredo de cada projeto
//! (`dev-secrets.json`), fora do projeto e do git. Antes de compilar com a feature `dev`, a
//! IDE grava o alvo (endereço, chave pública e segredo) onde o `build.rs` do app lê
//! ([`aylis_build::dev_target_path`]).

use std::path::{Path, PathBuf};

use aylis_devlink::{Target, hex, unhex32};
use aylis_project::Project;
use serde_json::{Map, Value, json};

fn data_dir() -> Option<PathBuf> {
    Some(crate::dap::tools_dir()?.parent()?.to_path_buf())
}

fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// O par de chaves da IDE: (privada, pública). Na primeira vez, gera e grava.
pub fn ide_keys() -> Result<([u8; 32], [u8; 32]), String> {
    let path = data_dir().ok_or("sem pasta de dados do usuário")?.join("dev-key.json");
    if let Some(v) = read_json(&path) {
        if let (Some(private), Some(public)) = (v["private"].as_str().and_then(unhex32), v["public"].as_str().and_then(unhex32)) {
            return Ok((private, public));
        }
    }
    let (private, public) = aylis_devlink::keypair()?;
    write_json(&path, &json!({ "private": hex(&private), "public": hex(&public) }))?;
    Ok((private, public))
}

/// A chave de um projeto no arquivo dos segredos (a pasta, comparável).
fn project_key(project: &Project) -> String {
    let dir = project.dir();
    let dir = dir.canonicalize().unwrap_or(dir);
    dir.to_string_lossy().trim_start_matches(r"\\?\").replace('\\', "/").to_lowercase()
}

/// O segredo do projeto (32 bytes); na primeira vez, gera e grava.
pub fn project_secret(project: &Project) -> Result<[u8; 32], String> {
    let path = data_dir().ok_or("sem pasta de dados do usuário")?.join("dev-secrets.json");
    let mut map: Map<String, Value> = read_json(&path).and_then(|v| v.as_object().cloned()).unwrap_or_default();
    let key = project_key(project);
    if let Some(s) = map.get(&key).and_then(Value::as_str).and_then(unhex32) {
        return Ok(s);
    }
    let secret = aylis_devlink::secret()?;
    map.insert(key, Value::String(hex(&secret)));
    write_json(&path, &Value::Object(map))?;
    Ok(secret)
}

/// O endereço do canal do projeto (`host:porta`).
pub fn address(project: &Project) -> String {
    if project.dev_address.trim().is_empty() { format!("127.0.0.1:{}", aylis_devlink::DEFAULT_PORT) } else { project.dev_address.trim().to_string() }
}

/// O alvo completo do projeto (o que o app embute).
pub fn target(project: &Project) -> Result<Target, String> {
    Ok(Target { address: address(project), ide_public: ide_keys()?.1, secret: project_secret(project)? })
}

/// Grava o alvo para o crate em `crate_dir` (o `build.rs` dele lê ao compilar com `dev`).
/// Só regrava se mudou (o cargo refaz o build quando o arquivo muda).
pub fn write_build_target(project: &Project, crate_dir: &Path) -> Result<(), String> {
    let path = aylis_build::dev_target_path(crate_dir).ok_or("sem pasta de dados do usuário")?;
    let text = target(project)?.to_json().to_string();
    if std::fs::read_to_string(&path).is_ok_and(|old| old == text) {
        return Ok(());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}
