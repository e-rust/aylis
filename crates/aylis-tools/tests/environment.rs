//! A verificação de ambiente guardada: o resultado reaproveitado pelas marcas e os tempos.

use std::path::PathBuf;
use std::time::Instant;

use aylis_project::Language;
use aylis_tools::{Target, env, env_cache};

/// Uma plataforma que não roda processo nenhum nesta máquina (o iOS fora do Mac, o "Linux
/// pelo WSL" fora do Windows): o teste não depende das ferramentas instaladas.
fn quiet_target() -> Target {
    if cfg!(target_os = "macos") { Target::Linux } else if cfg!(windows) { Target::Ios } else { Target::Linux }
}

fn temp_file(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aylis_environment_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("environment.json")
}

#[test]
fn the_stored_result_is_reused_until_forced() {
    let file = temp_file("reuse");
    let target = quiet_target();
    let targets = [target];
    let mut request = env::Request::new(Language::Rust, &targets, None);
    request.cache_file = Some(file.clone());
    let first = env::check_with(&request, &|_, _| {});
    assert!(!first.is_empty());
    // O resultado guardado, mexido à mão: se a próxima conferência devolver o mexido, ela não
    // conferiu de novo.
    let mut cache = env_cache::Cache::read(&file);
    let key = env_cache::key(target, Language::Rust);
    cache.entries.get_mut(&key).expect("guardado").checks[0].detail = "do arquivo".into();
    std::fs::write(&file, serde_json::to_string(&cache.to_json()).unwrap()).unwrap();
    let seen = std::sync::Mutex::new(Vec::new());
    let again = env::check_with(&request, &|t, checks| seen.lock().unwrap().push((t, checks.len())));
    assert_eq!(again[0].detail, "do arquivo");
    assert_eq!(*seen.lock().unwrap(), vec![(target, first.len())], "cada plataforma avisa quando fica pronta");
    // "Verificar de novo": confere e grava por cima.
    request.force = vec![target];
    let forced = env::check_with(&request, &|_, _| {});
    assert_eq!(forced, first);
    assert_eq!(env_cache::Cache::read(&file).entries[&key].checks, first);
    let _ = std::fs::remove_dir_all(file.parent().unwrap());
}

#[test]
fn a_cached_result_with_missing_items_is_rechecked_before_running() {
    let file = temp_file("missing");
    let target = quiet_target();
    let targets = [target];
    let mut request = env::Request::new(Language::Rust, &targets, None);
    request.cache_file = Some(file.clone());
    let first = env::check_with(&request, &|_, _| {});
    // Guardado como "falta algo" (a pessoa já pode ter instalado): o F9 confere de novo.
    let mut cache = env_cache::Cache::read(&file);
    let key = env_cache::key(target, Language::Rust);
    let entry = cache.entries.get_mut(&key).unwrap();
    entry.checks = vec![env::Check { status: env::Status::Missing, detail: "do arquivo".into(), ..first[0].clone() }];
    std::fs::write(&file, serde_json::to_string(&cache.to_json()).unwrap()).unwrap();
    request.recheck_missing = true;
    request.live = false;
    assert_eq!(env::check_with(&request, &|_, _| {}), first);
    let _ = std::fs::remove_dir_all(file.parent().unwrap());
}

/// Os tempos de abrir o "Verificar ambiente" (roda as ferramentas de verdade, no arquivo
/// de verdade): `cargo test -p aylis-tools --release --test environment -- --ignored --nocapture`
#[test]
#[ignore]
fn environment_check_timings() {
    let t = Instant::now();
    let shown = env_cache::cached(Language::Rust);
    println!("abrir com o guardado (ler o arquivo): {:?} ({} plataformas)", t.elapsed(), shown.len());
    let t = Instant::now();
    aylis_tools::setup::refresh_path();
    println!("refresh_path: {:?}", t.elapsed());
    let t = Instant::now();
    let _ = env::check(Language::Rust, &Target::ALL, None);
    println!("reconferência completa (Verificar de novo), em paralelo: {:?}", t.elapsed());
    for target in Target::ALL {
        let t = Instant::now();
        let _ = env::check(Language::Rust, &[target], None);
        println!("  só {} (forçado): {:?}", target.name(), t.elapsed());
    }
    let t = Instant::now();
    let _ = env::check_with(&env::Request::new(Language::Rust, &Target::ALL, None), &|_, _| {});
    println!("reconferência sem mudança (todas, com o aparelho do Android): {:?}", t.elapsed());
    let mut request = env::Request::new(Language::Rust, &[Target::Desktop], None);
    request.live = false;
    request.recheck_missing = true;
    let t = Instant::now();
    let _ = env::check_with(&request, &|_, _| {});
    println!("F9 (Windows, sem mudança): {:?}", t.elapsed());
}
