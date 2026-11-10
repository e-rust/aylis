//! Limpeza da pasta `target`: o cargo nunca apaga o que compilou, e cada versão de
//! dependência, feature, alvo e projeto deixa a sua cópia (dezenas de GB com o tempo; só
//! trocar o `-p` de um `cargo test` muda as features unificadas e compila tudo de novo com
//! outro hash). Editar um fonte não deixa cópia: o hash não depende do conteúdo, e o arquivo
//! novo é gravado por cima.
//!
//! Cada unidade de compilação do cargo tem um hash de 16 dígitos repetido em
//! `.fingerprint/<pacote>-<hash>`, `deps/<crate>-<hash>.*`, `examples/<nome>-<hash>.*` e
//! `build/<pacote>-<hash>` (o nome muda: `aylis-ide` num, `aylis_ide` no outro; o hash é o
//! mesmo). Uma unidade sai de todos esses lugares de uma vez.
//!
//! As que um projeto compilou pelo aylis estão no índice ([`crate::build_index`]) e saem quando
//! nenhum projeto vivo precisa mais delas:
//! - as do próprio projeto (o código, os testes, os exemplos), `project_days` depois da última
//!   compilação que as usou (o `output.cleanup_days` do `.aylisproj` muda o prazo do projeto);
//! - as compartilhadas (o aylis, as dependências), `shared_days` depois do último projeto;
//! - nos dois casos, logo que o projeto passa a usar outra do mesmo alvo (uma versão nova do
//!   Rust ou da dependência), guardada uma alternativa recente; e as de um projeto apagado.
//!   Cada projeto guarda as suas: dois projetos com features diferentes do aylis não apagam a
//!   cópia um do outro.
//!
//! As que o índice não conhece (o cargo no terminal, no editor) seguem a regra de antes: saem
//! sem uso há `untracked_days`, ou substituídas, quando o mesmo alvo (pacote + lib/bin/teste)
//! tem uma compilação mais nova e esta não foi usada junto com ela nem é a alternativa usada
//! nas últimas 24 horas.
//!
//! O `incremental/` (nomes `<crate>-<base36>`, sem o hash) segue a regra de dias e de
//! substituição por crate. Se uma unidade apagada ainda for usada, o cargo vê que falta e
//! compila de novo.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::build_index::{self, Index};

/// Quanto foi apagado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cleanup {
    pub units: usize,
    pub bytes: u64,
}

impl std::ops::AddAssign for Cleanup {
    fn add_assign(&mut self, r: Cleanup) {
        self.units += r.units;
        self.bytes += r.bytes;
    }
}

/// Dias sem uso até a limpeza apagar uma unidade fora do índice que não foi substituída.
pub const DEFAULT_DAYS: u64 = 7;
/// Dias que o que é só de um projeto fica depois da última compilação dele.
pub const DEFAULT_PROJECT_DAYS: u64 = 14;
/// Dias que o compartilhado (o aylis, as dependências) fica depois do último projeto que usou.
pub const DEFAULT_SHARED_DAYS: u64 = 30;
/// Uma unidade do índice que nenhum projeto quer mais ainda fica este tempo depois do último
/// uso no disco (o cargo de fora do aylis pode estar usando).
const ORPHAN: Duration = ALTERNATE;
/// Unidades do mesmo alvo usadas até este tempo antes da mais nova são da mesma compilação
/// (um crate com features diferentes para o build script e para o programa).
const SAME_BUILD: Duration = Duration::from_secs(15 * 60);
/// Uma alternativa (outro `-p`, outras features) fica por este tempo: alternar entre dois
/// comandos não recompila tudo a cada vez.
const ALTERNATE: Duration = Duration::from_secs(24 * 3600);
/// Quantas pastas do `incremental/` ficam por crate (lib, bin e teste da lib dividem o nome).
const INCREMENTAL_PER_CRATE: usize = 4;

/// A limpeza automática: no máximo uma vez por hora (o `aylis-tool` chama antes de compilar,
/// a IDE ao abrir). Guarda a hora da última em `target/.aylis-cleanup`.
pub fn automatic(target: &Path) -> Option<Cleanup> {
    let marker = target.join(".aylis-cleanup");
    let recent = std::fs::metadata(&marker)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.elapsed().ok())
        .is_some_and(|e| e < Duration::from_secs(3600));
    if recent || !target.is_dir() {
        return None;
    }
    let _ = std::fs::write(&marker, b"");
    Some(clean_with(target, &Policy::from_settings(), SystemTime::now()))
}

/// Os prazos da limpeza.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    /// O que não está no índice.
    pub untracked_days: u64,
    /// O que é só de um projeto (sem o `output.cleanup_days` dele).
    pub project_days: u64,
    /// O compartilhado entre projetos.
    pub shared_days: u64,
}

impl Default for Policy {
    fn default() -> Self {
        Policy { untracked_days: DEFAULT_DAYS, project_days: DEFAULT_PROJECT_DAYS, shared_days: DEFAULT_SHARED_DAYS }
    }
}

impl Policy {
    /// Os prazos das Opções da IDE (`cleanup_project_days`, `cleanup_shared_days`).
    pub fn from_settings() -> Policy {
        let d = Policy::default();
        Policy {
            untracked_days: d.untracked_days,
            project_days: crate::settings::number("cleanup_project_days").unwrap_or(d.project_days),
            shared_days: crate::settings::number("cleanup_shared_days").unwrap_or(d.shared_days),
        }
    }
}

/// Apaga o que não serve mais, em todos os perfis e alvos de `target`; `days` é o prazo do
/// que está fora do índice (os outros vêm das Opções).
pub fn clean(target: &Path, days: u64) -> Cleanup {
    clean_with(target, &Policy { untracked_days: days, ..Policy::from_settings() }, SystemTime::now())
}

/// O que o índice decide: as unidades que ficam e todas as que ele conhece.
#[derive(Default)]
struct Verdict {
    keep: HashSet<String>,
    known: HashSet<String>,
}

fn secs(t: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(t)
}

fn days_before(now: SystemTime, days: u64) -> SystemTime {
    now.checked_sub(Duration::from_secs(days.saturating_mul(24 * 3600))).unwrap_or(UNIX_EPOCH)
}

fn verdict(index: &Index, policy: &Policy, now: SystemTime) -> Verdict {
    let mut v = Verdict::default();
    for (dir, project) in &index.projects {
        v.known.extend(project.units.keys().cloned());
        if !build_index::alive(dir) {
            continue;
        }
        let own_days = build_index::project_days(dir).unwrap_or(policy.project_days);
        // Por alvo, dentro do projeto: a mais nova substitui as outras.
        let mut groups: HashMap<(bool, &str), Vec<(&String, SystemTime)>> = HashMap::new();
        for (hash, u) in &project.units {
            groups.entry((u.own, u.target.as_str())).or_default().push((hash, secs(u.used)));
        }
        for ((own, _), group) in groups {
            let limit = days_before(now, if own { own_days } else { policy.shared_days });
            let all: Vec<&String> = group.iter().map(|(h, _)| *h).collect();
            let out: HashSet<&String> = superseded(group, limit, now, usize::MAX).into_iter().collect();
            v.keep.extend(all.into_iter().filter(|h| !out.contains(h)).cloned());
        }
    }
    v
}

/// A limpeza com prazos e "agora" dados (os testes adiantam o relógio).
pub fn clean_with(target: &Path, policy: &Policy, now: SystemTime) -> Cleanup {
    // A trava do índice: uma compilação que terminar agora espera para anotar.
    let lock = build_index::lock(target);
    let mut index = Index::load(target);
    let v = verdict(&index, policy, now);
    let limit = days_before(now, policy.untracked_days);
    let mut total = Cleanup::default();
    let mut present = HashSet::new();
    let mut deleted = HashSet::new();
    for profile in profiles(target) {
        let r = clean_profile(&profile, limit, now, &v);
        total += r.cleanup;
        present.extend(r.present);
        deleted.extend(r.deleted);
    }
    // O índice só com o que ainda existe, de projetos que ainda existem.
    if lock.is_some() && !index.projects.is_empty() {
        let before = index.clone();
        index.projects.retain(|dir, _| build_index::alive(dir));
        for p in index.projects.values_mut() {
            p.units.retain(|h, _| present.contains(h) && !deleted.contains(h));
        }
        index.projects.retain(|_, p| !p.units.is_empty());
        if index != before {
            let _ = index.save(target);
        }
    }
    total
}

/// Apaga o que é só do projeto em `dir` (o que outro projeto vivo usa fica) e tira o projeto
/// do índice: o "Limpar a compilação deste projeto".
pub fn clean_project(target: &Path, dir: &Path) -> Cleanup {
    let Some(_lock) = build_index::lock(target) else { return Cleanup::default() };
    let mut index = Index::load(target);
    let Some(entry) = index.projects.remove(&build_index::project_key(dir)) else { return Cleanup::default() };
    let others = verdict(&index, &Policy::from_settings(), SystemTime::now());
    let stale: HashSet<String> = entry.units.into_iter().filter(|(h, u)| u.own && !others.keep.contains(h)).map(|(h, _)| h).collect();
    let mut total = Cleanup::default();
    for profile in profiles(target) {
        let Some(_cargo) = cargo_lock(&profile) else { continue };
        total += delete_units(&profile, &stale);
    }
    let _ = index.save(target);
    total
}

/// O tamanho da pasta de compilação (para a tela de opções).
pub fn size_of(target: &Path) -> u64 {
    size(target)
}

/// As pastas de perfil (`target/debug`, `target/x86_64-linux-android/release`...): as que
/// têm `.fingerprint`.
fn profiles(target: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(target) else { return out };
    for e in entries.flatten() {
        let p = e.path();
        if !p.is_dir() {
            continue;
        }
        if p.join(".fingerprint").is_dir() {
            out.push(p);
        } else if let Ok(sub) = std::fs::read_dir(&p) {
            out.extend(sub.flatten().map(|s| s.path()).filter(|s| s.join(".fingerprint").is_dir()));
        }
    }
    out
}

/// O nome e o hash de `aylis_core-0123456789abcdef.rlib` ou `aylis-core-0123456789abcdef`.
pub(crate) fn name_and_hash(file: &str) -> Option<(&str, &str)> {
    let base = file.split('.').next()?;
    let (name, hash) = base.rsplit_once('-')?;
    (hash.len() == 16 && hash.chars().all(|c| c.is_ascii_hexdigit()) && !name.is_empty()).then_some((name, hash))
}

/// O alvo de uma unidade: o pacote e o tipo (`lib-aylis_core`, `test-integration-test-fps`,
/// `run-build-script-build-script-build`...), do arquivo que o cargo grava na pasta dela.
fn target_key(folder: &Path, package: &str) -> String {
    let kind = std::fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| !n.ends_with(".json") && !n.starts_with("dep-") && n != "invoked.timestamp" && n != "output")
        .min()
        .unwrap_or_default();
    format!("{package}/{kind}")
}

/// A modificação mais nova dentro de uma pasta (um nível) ou do arquivo.
fn newest(p: &Path) -> Option<SystemTime> {
    let mut t = std::fs::metadata(p).and_then(|m| m.modified()).ok()?;
    if let Ok(entries) = std::fs::read_dir(p) {
        for e in entries.flatten() {
            if let Ok(m) = e.metadata().and_then(|m| m.modified()) {
                t = t.max(m);
            }
        }
    }
    Some(t)
}

fn size(p: &Path) -> u64 {
    match std::fs::symlink_metadata(p) {
        Ok(m) if m.is_dir() => std::fs::read_dir(p).map(|r| r.flatten().map(|e| size(&e.path())).sum()).unwrap_or(0),
        Ok(m) => m.len(),
        Err(_) => 0,
    }
}

fn delete(p: &Path) -> u64 {
    let bytes = size(p);
    let ok = if p.is_dir() { std::fs::remove_dir_all(p).is_ok() } else { std::fs::remove_file(p).is_ok() };
    if ok { bytes } else { 0 }
}

/// Dos itens de um grupo (`(chave, último uso)`), os que saem: os sem uso desde `limit` e
/// os substituídos (fica a mais nova, as da mesma compilação que ela e uma alternativa
/// recente; no máximo `max`).
fn superseded<K: Clone>(mut group: Vec<(K, SystemTime)>, limit: SystemTime, now: SystemTime, max: usize) -> Vec<K> {
    group.sort_by(|a, b| b.1.cmp(&a.1));
    let Some(&(_, latest)) = group.first() else { return Vec::new() };
    let mut took_alternate = false;
    let mut kept = 0;
    let mut out = Vec::new();
    for (k, used) in group {
        let together = latest.duration_since(used).is_ok_and(|d| d <= SAME_BUILD);
        let recent = now.duration_since(used).map_or(true, |d| d <= ALTERNATE);
        let keep = used >= limit
            && kept < max
            && (together || (recent && !took_alternate && {
                took_alternate = true;
                true
            }));
        if keep {
            kept += 1;
        } else {
            out.push(k);
        }
    }
    out
}

/// A trava do cargo no perfil, se nenhum cargo estiver compilando nele. Um cargo compilando
/// segura a trava: apagar no meio deixaria uma unidade apontando para uma dependência que
/// sumiu (E0460). Com a trava na mão, o cargo que começar agora espera a limpeza acabar.
fn cargo_lock(profile: &Path) -> Option<std::fs::File> {
    let lock = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(profile.join(".cargo-lock")).ok()?;
    lock.try_lock().ok()?;
    Some(lock)
}

/// Apaga as unidades `stale` do perfil, de todas as pastas.
fn delete_units(profile: &Path, stale: &HashSet<String>) -> Cleanup {
    let mut r = Cleanup::default();
    if stale.is_empty() {
        return r;
    }
    // A impressão digital por último: se algo falhar no meio, o cargo ainda vê a unidade
    // incompleta e compila de novo.
    for folder in ["deps", "examples", "build", ".fingerprint"] {
        for e in std::fs::read_dir(profile.join(folder)).into_iter().flatten().flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name_and_hash(&name).is_some_and(|(_, h)| stale.contains(h)) {
                r.bytes += delete(&e.path());
                if folder == ".fingerprint" {
                    r.units += 1;
                }
            }
        }
    }
    r
}

/// O resultado de um perfil: o que saiu e os hashes que havia nele.
struct ProfileCleanup {
    cleanup: Cleanup,
    present: HashSet<String>,
    deleted: HashSet<String>,
}

fn clean_profile(profile: &Path, limit: SystemTime, now: SystemTime, v: &Verdict) -> ProfileCleanup {
    // As unidades por alvo: as do índice já estão decididas; as outras, pela regra de dias e
    // de substituição.
    let mut targets: HashMap<String, Vec<(String, SystemTime)>> = HashMap::new();
    let mut present = HashSet::new();
    let mut stale = HashSet::new();
    for e in std::fs::read_dir(profile.join(".fingerprint")).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let Some((package, hash)) = name_and_hash(&name) else { continue };
        present.insert(hash.to_string());
        let Some(used) = newest(&e.path()) else { continue };
        if v.keep.contains(hash) {
            continue;
        }
        if v.known.contains(hash) {
            if now.duration_since(used).is_ok_and(|d| d > ORPHAN) {
                stale.insert(hash.to_string());
            }
            continue;
        }
        targets.entry(target_key(&e.path(), package)).or_default().push((hash.to_string(), used));
    }
    let mut r = ProfileCleanup { cleanup: Cleanup::default(), present, deleted: HashSet::new() };
    let Some(_lock) = cargo_lock(profile) else { return r };
    stale.extend(targets.into_values().flat_map(|g| superseded(g, limit, now, usize::MAX)));
    r.cleanup = delete_units(profile, &stale);
    r.deleted = stale;
    // O incremental: por crate, as pastas mais novas.
    let mut crates: HashMap<String, Vec<(PathBuf, SystemTime)>> = HashMap::new();
    for e in std::fs::read_dir(profile.join("incremental")).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let Some((krate, _)) = name.rsplit_once('-') else { continue };
        let Some(used) = newest(&e.path()) else { continue };
        crates.entry(krate.to_string()).or_default().push((e.path(), used));
    }
    for group in crates.into_values() {
        for folder in superseded(group, limit, now, INCREMENTAL_PER_CRATE) {
            r.cleanup.bytes += delete(&folder);
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_index::{ProjectEntry, UnitUse};

    /// A regra sem índice.
    fn plain(profile: &Path, limit: SystemTime, now: SystemTime) -> Cleanup {
        clean_profile(profile, limit, now, &Verdict::default()).cleanup
    }

    fn at(t: SystemTime) -> u64 {
        t.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    #[test]
    fn the_index_separates_garbage_from_project_and_shared_units() {
        let target = std::env::temp_dir().join(format!("aylis_cleanup_index_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&target);
        let profile = target.join("debug");
        let (a, b) = (target.join("projetos").join("a"), target.join("projetos").join("b"));
        for dir in [&a, &b] {
            std::fs::create_dir_all(dir).unwrap();
            std::fs::write(dir.join("Cargo.toml"), b"[package]").unwrap();
        }
        let (shared, own, old_own, dead_own, untracked) = ("00000000000000c1", "00000000000000a1", "00000000000000a0", "00000000000000b1", "00000000000000d1");
        unit(&profile, "aylis-core", shared, "lib-aylis_core", "aylis_core");
        unit(&profile, "app", own, "bin-app", "app");
        unit(&profile, "app", old_own, "bin-app", "app");
        unit(&profile, "outro", dead_own, "bin-outro", "outro");
        unit(&profile, "solto", untracked, "lib-solto", "solto");
        // Daqui a 2 dias (os arquivos no disco parecem sem uso há 2 dias).
        let later = SystemTime::now() + Duration::from_secs(2 * 24 * 3600);
        let hour = Duration::from_secs(3600);
        let use_ = |used: SystemTime, own: bool, target: &str| UnitUse { used: at(used), own, target: target.into() };
        let mut index = Index::default();
        let mut pa = ProjectEntry { built: at(later - hour), ..Default::default() };
        pa.units.insert(shared.into(), use_(later - hour, false, "debug/aylis-core/lib/aylis_core"));
        pa.units.insert(own.into(), use_(later - hour, true, "debug/app/bin/app"));
        // A compilação anterior do mesmo alvo (outra versão do Rust, por exemplo).
        pa.units.insert(old_own.into(), use_(later - 30 * hour, true, "debug/app/bin/app"));
        index.projects.insert(build_index::project_key(&a), pa);
        let mut pb = ProjectEntry { built: at(later - hour), ..Default::default() };
        pb.units.insert(shared.into(), use_(later - hour, false, "debug/aylis-core/lib/aylis_core"));
        pb.units.insert(dead_own.into(), use_(later - hour, true, "debug/outro/bin/outro"));
        index.projects.insert(build_index::project_key(&b), pb);
        index.save(&target).unwrap();
        // O projeto B foi apagado.
        std::fs::remove_file(b.join("Cargo.toml")).unwrap();

        let r = clean_with(&target, &Policy::default(), later);
        assert_eq!(r.units, 2, "a compilação substituída de A e o que era só de B");
        let exists = |h: &str| std::fs::read_dir(profile.join(".fingerprint")).unwrap().flatten().any(|e| e.file_name().to_string_lossy().ends_with(h));
        assert!(!exists(old_own) && !exists(dead_own));
        assert!(exists(shared) && exists(own), "o que A usa fica");
        assert!(exists(untracked), "fora do índice, 2 dias ainda não são 7");
        let index = Index::load(&target);
        assert_eq!(index.projects.len(), 1, "o projeto apagado sai do índice");
        assert!(!index.projects[&build_index::project_key(&a)].units.contains_key(old_own));

        // A com prazo de 1 dia no .aylisproj: 2 dias depois, o dele sai e o compartilhado fica.
        std::fs::write(a.join("a.aylisproj"), br#"{ "output": { "cleanup_days": 1 } }"#).unwrap();
        let r = clean_with(&target, &Policy::default(), later + 2 * 24 * hour);
        assert_eq!(r.units, 1);
        assert!(!exists(own) && exists(shared));

        // Limpar o projeto: sai o que é só dele (aqui, nada mais) e ele sai do índice.
        clean_project(&target, &a);
        assert!(Index::load(&target).projects.is_empty());
        assert!(exists(shared), "o compartilhado não é do projeto");
        let _ = std::fs::remove_dir_all(&target);
    }

    #[test]
    fn unit_names() {
        let h = "0123456789abcdef";
        assert_eq!(name_and_hash(&format!("libaylis_core-{h}.rlib")), Some(("libaylis_core", h)));
        assert_eq!(name_and_hash(&format!("aylis-core-{h}")), Some(("aylis-core", h)));
        assert_eq!(name_and_hash(&format!("editores-{h}.pdb")), Some(("editores", h)));
        assert_eq!(name_and_hash("aylis-ide.exe"), None);
        assert_eq!(name_and_hash("aylis_ide-010btv3xgs82q"), None);
    }

    fn unit(profile: &Path, package: &str, hash: &str, kind: &str, crate_: &str) {
        let fp = profile.join(".fingerprint").join(format!("{package}-{hash}"));
        std::fs::create_dir_all(&fp).unwrap();
        std::fs::write(fp.join("invoked.timestamp"), b"").unwrap();
        std::fs::write(fp.join(kind), b"").unwrap();
        std::fs::write(fp.join(format!("{kind}.json")), b"").unwrap();
        std::fs::create_dir_all(profile.join("deps")).unwrap();
        std::fs::write(profile.join("deps").join(format!("lib{crate_}-{hash}.rlib")), vec![0u8; 100]).unwrap();
        std::fs::write(profile.join("deps").join(format!("{crate_}-{hash}.pdb")), vec![0u8; 100]).unwrap();
    }

    #[test]
    fn removes_stale_units_everywhere() {
        let target = std::env::temp_dir().join(format!("aylis_cleanup_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&target);
        let profile = target.join("debug");
        let (a, b) = ("0000000000000001", "0000000000000002");
        unit(&profile, "meu-crate", a, "lib-meu_crate", "meu_crate");
        unit(&profile, "outro", b, "lib-outro", "outro");
        std::fs::create_dir_all(profile.join("incremental").join("meu_crate-abc123")).unwrap();
        std::fs::write(profile.join("app.exe"), b"x").unwrap();
        // "Agora" daqui a 10 dias: as duas parecem velhas para 7 dias...
        let later = SystemTime::now() + Duration::from_secs(10 * 24 * 3600);
        let r = plain(&profile, later - Duration::from_secs(7 * 24 * 3600), later);
        assert_eq!(r.units, 2);
        assert!(r.bytes >= 400);
        assert!(!profile.join("deps").join(format!("libmeu_crate-{a}.rlib")).exists(), "o nome com _ do deps também sai");
        assert!(!profile.join(".fingerprint").join(format!("outro-{b}")).exists());
        assert!(!profile.join("incremental").join("meu_crate-abc123").exists());
        assert!(profile.join("app.exe").exists(), "o executável final fica");
        // ...e hoje nada é apagado.
        unit(&profile, "outro", b, "lib-outro", "outro");
        let now = SystemTime::now();
        assert_eq!(plain(&profile, now - Duration::from_secs(3600), now).units, 0);
        assert_eq!(profiles(&target), vec![profile]);
        let _ = std::fs::remove_dir_all(&target);
    }

    #[test]
    fn nothing_is_deleted_while_cargo_is_building() {
        let target = std::env::temp_dir().join(format!("aylis_cleanup_lock_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&target);
        let profile = target.join("debug");
        unit(&profile, "old", "0000000000000003", "lib-old", "old");
        // O cargo compilando: a trava do perfil está com ele.
        let cargo = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(profile.join(".cargo-lock")).unwrap();
        cargo.lock().unwrap();
        let later = SystemTime::now() + Duration::from_secs(10 * 24 * 3600);
        assert_eq!(plain(&profile, later - Duration::from_secs(7 * 24 * 3600), later), Cleanup::default());
        assert!(profile.join("deps").join("libold-0000000000000003.rlib").exists());
        // Terminou: agora sai.
        drop(cargo);
        assert_eq!(plain(&profile, later - Duration::from_secs(7 * 24 * 3600), later).units, 1);
        let _ = std::fs::remove_dir_all(&target);
    }

    #[test]
    fn a_newer_build_of_the_same_target_replaces_the_old_one() {
        let t = |h: u64| SystemTime::now() - Duration::from_secs(h * 3600);
        let limit = t(7 * 24);
        // Mais nova, uma da mesma compilação, uma alternativa de ontem e duas antigas.
        let group = vec![("latest", t(0)), ("same_build", t(0) - Duration::from_secs(60)), ("alternate", t(5)), ("old", t(30)), ("other", t(6))];
        let mut removed = superseded(group, limit, SystemTime::now(), usize::MAX);
        removed.sort();
        assert_eq!(removed, ["old", "other"]);
        // Testes diferentes do mesmo pacote são alvos diferentes: não se substituem.
        let dir = std::env::temp_dir().join(format!("aylis_cleanup_target_{}", std::process::id()));
        let (x, y) = (dir.join("x"), dir.join("y"));
        for (p, kind) in [(&x, "test-integration-test-fps"), (&y, "test-integration-test-valores")] {
            std::fs::create_dir_all(p).unwrap();
            std::fs::write(p.join(kind), b"").unwrap();
            std::fs::write(p.join(format!("dep-{kind}")), b"").unwrap();
            std::fs::write(p.join("invoked.timestamp"), b"").unwrap();
        }
        assert_eq!(target_key(&x, "aylis-core"), "aylis-core/test-integration-test-fps");
        assert_ne!(target_key(&x, "aylis-core"), target_key(&y, "aylis-core"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
