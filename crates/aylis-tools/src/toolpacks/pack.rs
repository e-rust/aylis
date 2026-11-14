//! `aylis-tool packages pack` (montar a pasta de pacotes, numa máquina com internet) e
//! `aylis-tool packages update` (preencher os hashes do manifesto).

use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use super::fetch::{download_verified, save_hashing};
use super::{FileSpec, Manifest, Progress, sha256_file};
use crate::process::Control;

/// O que o `pack` gravou.
#[derive(Clone, Debug, Default)]
pub struct PackReport {
    /// (arquivo, tamanho, pacote, plataforma, de onde veio: "cache", "baixado", "já estava").
    pub files: Vec<(String, u64, String, String, &'static str)>,
    /// O `pacotes.json`.
    pub index: PathBuf,
}

impl PackReport {
    pub fn total(&self) -> u64 {
        self.files.iter().map(|f| f.1).sum()
    }
}

/// Grava em `out` os arquivos de `names` (todos, se vazio) e das dependências, nas
/// `platforms` (vazio: esta), e o `pacotes.json` com o lugar de cada um (relativo: a pasta
/// funciona como pasta de pacotes e, posta num servidor HTTPS, como espelho). O que está no
/// cache com o hash certo é reaproveitado (ligado, sem cópia, quando dá); o resto vem da URL
/// oficial e é conferido com o hash do manifesto.
pub fn pack(manifest: &Manifest, cache: &Path, out: &Path, names: &[&str], platforms: &[String], control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<PackReport, String> {
    let all = manifest.names();
    let wanted: Vec<&str> = if names.is_empty() { all.iter().map(String::as_str).collect() } else { names.to_vec() };
    let order = manifest.with_dependencies(&wanted)?;
    let platforms = if platforms.is_empty() { vec![super::platform()] } else { platforms.to_vec() };
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let index_path = out.join("pacotes.json");
    let mut files: Map<String, Value> = std::fs::read_to_string(&index_path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v["files"].as_object().cloned())
        .unwrap_or_default();
    let mut report = PackReport { index: index_path.clone(), ..Default::default() };
    let mut seen = Vec::new();
    for name in &order {
        for plat in &platforms {
            let Some(p) = manifest.package_for(name, plat) else { continue };
            let Some(key) = &p.platform else {
                progress(Progress::Step(format!("{name}: não há para {plat} (fica de fora)")));
                continue;
            };
            for f in &p.files {
                if seen.contains(&f.name) {
                    continue;
                }
                seen.push(f.name.clone());
                control.check()?;
                let how = pack_file(f, cache, out, control, progress).map_err(|e| format!("{name} ({key}): {e}"))?;
                files.insert(f.name.clone(), json!({ "url": f.name, "sha256": f.sha256, "size": f.size, "package": name, "version": p.version, "platform": key }));
                report.files.push((f.name.clone(), f.size, name.clone(), key.clone(), how));
            }
        }
    }
    let index = json!({
        "_": "Pasta de pacotes do Aylis (aylis-tool packages pack). O lugar de cada arquivo: \"url\" (relativa a este arquivo ou https) ou \"drive\": \"<id>\". O hash que vale é o do manifesto embutido no Aylis.",
        "aylis": 1,
        "files": files,
    });
    std::fs::write(&index_path, serde_json::to_string_pretty(&index).map_err(|e| e.to_string())? + "\n").map_err(|e| format!("{}: {e}", index_path.display()))?;
    progress(Progress::Step(format!("{} arquivos ({} MB) em {}", report.files.len(), report.total().div_ceil(1024 * 1024), out.display())));
    Ok(report)
}

fn pack_file(f: &FileSpec, cache: &Path, out: &Path, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<&'static str, String> {
    if !super::is_sha256(&f.sha256) {
        return Err(format!("{}: sem SHA-256 no manifesto (rode aylis-tool packages update)", f.name));
    }
    let dest = out.join(&f.name);
    if dest.is_file() && sha256_file(&dest)? == f.sha256 {
        return Ok("já estava");
    }
    let _ = std::fs::remove_file(&dest);
    let cached = cache.join(&f.name);
    if cached.is_file() && sha256_file(&cached)? == f.sha256 {
        if std::fs::hard_link(&cached, &dest).is_err() {
            std::fs::copy(&cached, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
        }
        return Ok("cache");
    }
    let url = f.url.as_deref().ok_or_else(|| match &f.built {
        // Feito pelo Aylis: não há de onde baixar; o comando o põe no cache.
        Some(command) => format!("{}: não está no cache ({}); gere com `{command}`", f.name, cache.display()),
        None => format!("{}: o manifesto não tem a URL oficial", f.name),
    })?;
    progress(Progress::Step(format!("Baixando {} ({} MB) de {url}…", f.name, f.megabytes())));
    download_verified(url, f, &dest, control, progress).map_err(|e| format!("{url}: {e}"))?;
    Ok("baixado")
}

/// Preenche no `resources/packages.lock.json` de `root` o SHA-256 e o tamanho dos arquivos
/// de `names` (todos, se vazio) que estão sem hash (ou de todos, com `force`): baixa cada um
/// da URL oficial para o cache e grava o que veio. Mudar de versão: troque a versão e as URLs
/// no manifesto, apague os hashes e rode de novo.
pub fn update_lock(root: &Path, cache: &Path, names: &[&str], force: bool, control: &Control, progress: &mut dyn FnMut(Progress)) -> Result<usize, String> {
    let path = root.join("resources/packages.lock.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    Manifest::parse(&text)?;
    let mut lock: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(cache).map_err(|e| format!("{}: {e}", cache.display()))?;
    let mut changed = 0;
    let packages = lock["packages"].as_object_mut().ok_or("sem \"packages\"")?;
    for (name, p) in packages.iter_mut() {
        if !names.is_empty() && !names.contains(&name.as_str()) {
            continue;
        }
        let Some(platforms) = p["platforms"].as_object_mut() else { continue };
        for (plat, entry) in platforms.iter_mut() {
            let Some(list) = entry["files"].as_array_mut() else { continue };
            for f in list {
                let sha = f["sha256"].as_str().unwrap_or_default();
                if !force && super::is_sha256(sha) && f["size"].as_u64().unwrap_or(0) > 0 {
                    continue;
                }
                // Feito pelo Aylis (sem URL oficial): o comando dele grava o hash.
                if f.get("built").is_some() {
                    continue;
                }
                control.check()?;
                let file_name = f["name"].as_str().unwrap_or_default().to_string();
                let url = f["url"].as_str().ok_or_else(|| format!("{name} ({plat}): {file_name} sem URL"))?.to_string();
                progress(Progress::Step(format!("{name} ({plat}): baixando {url}…")));
                let spec = FileSpec { name: file_name.clone(), size: 0, sha256: String::new(), url: Some(url.clone()), strip: 0, prefix: None, into: String::new(), extract: false, save_as: None, built: None };
                let dest = cache.join(&file_name);
                let mut body = super::fetch::get(&url).map_err(|e| format!("{url}: {e}"))?;
                let (sha, size) = save_hashing(&mut body, &spec, &dest, &url, control, progress).map_err(|e| format!("{url}: {e}"))?;
                if super::is_sha256(f["sha256"].as_str().unwrap_or_default()) && f["sha256"].as_str() != Some(sha.as_str()) {
                    progress(Progress::Step(format!("{file_name}: o SHA-256 mudou ({} → {sha})", f["sha256"].as_str().unwrap_or_default())));
                }
                f["sha256"] = Value::String(sha);
                f["size"] = Value::from(size);
                changed += 1;
            }
        }
    }
    let text = serde_json::to_string_pretty(&lock).map_err(|e| e.to_string())? + "\n";
    Manifest::parse(&text)?;
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    progress(Progress::Step(format!("{changed} arquivos atualizados em {} (recompile o aylis-tool e a IDE)", path.display())));
    Ok(changed)
}
