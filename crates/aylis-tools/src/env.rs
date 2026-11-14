//! Verificação do ambiente: o que cada plataforma precisa para compilar e executar, e o
//! comando que corrige cada falta (a IDE pergunta antes de rodar).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use aylis_project::Language;

use crate::process::quiet;
use crate::{Target, which};

/// Versão mínima do Rust: o `rust-version` do workspace do Aylis (estava 1.86 com o
/// workspace em 1.88, e o 1.87 passava na conferência e falhava no cargo). O teste
/// `the_minimum_rust_is_the_workspace_one` confere.
pub const MIN_RUST: (u32, u32) = (1, 88);

#[cfg(test)]
mod package_tests {
    use super::*;
    use crate::toolpacks::Manifest;

    fn plan(name: &str, system: &[&str]) -> Option<(Vec<String>, Vec<String>)> {
        package_plan(&Manifest::embedded(), name, &|n| system.contains(&n))
    }

    /// O "Corrigir" escolhe o pacote que resolve o item, sem trazer o que a máquina já tem.
    #[test]
    fn the_fix_chooses_the_package_and_skips_what_the_system_has() {
        let on = |p: &str| Manifest::embedded().package(p).is_some_and(|p| p.available());
        if on("jdk") {
            assert_eq!(plan("jdk", &[]), Some((vec!["jdk".to_string()], vec![])));
            // O bundletool com o Java do sistema: o JDK não vem junto.
            assert_eq!(plan("bundletool", &["jdk"]), Some((vec!["bundletool".to_string()], vec!["jdk".to_string()])));
            assert_eq!(plan("bundletool", &[]).unwrap().0, ["jdk", "bundletool"]);
        }
        if on("rust") {
            // Sem Rust nenhum: o Rust do pacote e os alvos dentro dele.
            assert_eq!(plan("rust-wasm", &[]).unwrap().0, ["rust", "rust-wasm"]);
            // Com o Rust do sistema, os alvos vão pelo rustup: o pacote dos alvos não serve.
            assert_eq!(plan("rust-wasm", &["rust"]), None);
            // O wasm-bindgen com o Rust do sistema: só ele (os alvos ficam de fora também).
            let (install, skip) = plan("wasm-bindgen", &["rust"]).unwrap();
            assert_eq!(install, ["wasm-bindgen"]);
            assert_eq!(skip, ["rust", "rust-wasm"]);
        }
        if crate::toolpacks::platform() == "windows-x86_64" {
            let (install, _) = plan("cargo-ndk", &["rust", "android-sdk"]).unwrap();
            assert_eq!(install, ["android-ndk", "cargo-ndk"]);
        }
        assert_eq!(plan("nao-existe", &[]), None);
    }

    /// Um pacote mostra o comando do `aylis-tool packages`; a alternativa (o winget) fica para
    /// quem roda sem perguntar e sem origem.
    #[test]
    fn a_package_fix_shows_the_package_command_and_keeps_the_fallback() {
        let mut fix = Fix::new("Instala o JDK 17 (winget)", &["winget", "install", "--id", "EclipseAdoptium.Temurin.17.JDK"]);
        fix.fetch = Some("jdk".into());
        assert_eq!(fix.command_line(), "aylis-tool packages install jdk");
        assert!(fix.has_fallback());
        let only = Fix { fetch: Some("codelldb".into()), ..Fix::new("", &["aylis-tool", "packages", "install", "codelldb"]) };
        assert!(!only.has_fallback());
    }

    /// Instalar ou remover um pacote muda a marca do registro, que toda conferência anota.
    #[test]
    fn the_checks_depend_on_the_package_registry() {
        let Some(paths) = crate::toolpacks::Paths::user() else { return };
        let ((), deps) = crate::env_cache::record(note_packages);
        assert!(deps.iter().any(|d| matches!(d, crate::env_cache::Dep::Path(p, _) if *p == paths.registry())), "{deps:?}");
    }
}

#[cfg(test)]
mod min_rust_tests {
    #[test]
    fn the_minimum_rust_is_the_workspace_one() {
        let manifest = include_str!("../../../Cargo.toml");
        let line = manifest.lines().find(|l| l.trim_start().starts_with("rust-version")).expect("rust-version no workspace");
        let version = line.split('"').nth(1).unwrap();
        assert_eq!(version, format!("{}.{}", super::MIN_RUST.0, super::MIN_RUST.1));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    /// Funciona, mas falta algo opcional (um aparelho conectado, por exemplo).
    Warning,
    Missing,
}

/// Um comando que corrige uma falta.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    /// O que ele faz, para mostrar antes de perguntar.
    pub description: String,
    /// Programa e argumentos.
    pub command: Vec<String>,
    /// Abre uma janela ou instalador (a IDE não espera a saída; o usuário acompanha).
    pub interactive: bool,
    /// Em vez de rodar o comando: instalar o pacote de ferramentas com este nome
    /// ([`crate::toolpacks`], docs/pacotes.md), das origens dos pacotes, conferindo o SHA-256.
    /// O `command` fica como alternativa para quem roda sem perguntar (o "Configurar
    /// linguagens") quando não há origem; se for o próprio `aylis-tool packages install`, não
    /// há alternativa.
    pub fetch: Option<String>,
    /// Variáveis de ambiente do comando (o `GOBIN` do `go install`).
    pub env: Vec<(String, String)>,
}

/// O erro de um pacote sem de onde vir (a IDE mostra e leva às Opções).
pub const NO_PACKAGE_SOURCE: &str =
    "Nenhuma origem de pacotes configurada: indique uma pasta de pacotes ou um espelho em Ferramentas → Opções da IDE → Pacotes (ou ponha a pasta pacotes ao lado da IDE).";

impl Fix {
    fn new(description: impl Into<String>, command: &[&str]) -> Self {
        Self { description: description.into(), command: command.iter().map(|s| s.to_string()).collect(), interactive: false, fetch: None, env: Vec::new() }
    }

    /// Instalar o pacote de ferramentas `name` (sem alternativa), se houver para esta
    /// plataforma.
    fn download(name: &str) -> Option<Self> {
        package_fix(name, None)
    }

    /// Instalar o pacote `name` se houver (senão, `fallback` como antes); o `fallback` fica
    /// como alternativa sem origem (veja [`Fix::fetch`]).
    fn package_or(name: &str, fallback: Fix) -> Self {
        package_fix(name, Some(&fallback)).unwrap_or(fallback)
    }

    /// Roda a correção (não interativa) e manda a saída para `sink`.
    pub fn run(&self, control: &crate::Control, sink: crate::Sink) -> Result<(), String> {
        if let Some(name) = &self.fetch {
            control.check()?;
            if package_has_source(name) || !self.has_fallback() {
                return install_package(name, control, sink);
            }
            sink(crate::Event::Line(format!("{NO_PACKAGE_SOURCE} Usando: {}", self.command_line()), crate::LineKind::Tool));
        }
        let (program, args) = self.command.split_first().ok_or("comando vazio")?;
        let mut command = Command::new(program);
        command.args(args).envs(self.env.iter().map(|(k, v)| (k, v)));
        crate::process::run_tool(&mut command, control, sink, crate::LineKind::Tool)
    }

    /// Um pacote com outro comando para quando não há origem.
    fn has_fallback(&self) -> bool {
        self.command.first().is_some_and(|p| p != "aylis-tool")
    }

    fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// O comando como se digita num terminal (num pacote, o `aylis-tool packages install`).
    pub fn command_line(&self) -> String {
        let command = match &self.fetch {
            Some(name) => vec!["aylis-tool".to_string(), "packages".into(), "install".into(), name.clone()],
            None => self.command.clone(),
        };
        command.iter().map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a.clone() }).collect::<Vec<_>>().join(" ")
    }
}

// ---------------------------------------------------------------------------
// Os pacotes de ferramentas no "Corrigir" (docs/pacotes.md)

/// A pasta dos pacotes instalados do usuário (`%USERPROFILE%\.aylis\tools`).
fn packages_dir() -> Option<PathBuf> {
    crate::toolpacks::Paths::user().map(|p| p.tools)
}

/// O caminho, se não for de um pacote do Aylis (a ferramenta do sistema).
fn outside_packages(path: PathBuf) -> Option<String> {
    let tools = packages_dir();
    (!tools.is_some_and(|t| path.starts_with(t))).then(|| path.display().to_string())
}

/// Os alvos do Rust que o rustc do sistema (não o de um pacote) já tem.
fn system_rust_has(targets: &[&str]) -> Option<String> {
    which("rustc").and_then(outside_packages)?;
    let sysroot = output("rustc", &["--print", "sysroot"])?;
    let rustlib = PathBuf::from(sysroot.trim()).join("lib").join("rustlib");
    targets.iter().all(|t| rustlib.join(t).is_dir()).then(|| rustlib.display().to_string())
}

/// Onde a máquina já tem, fora dos pacotes do Aylis, o que o pacote `name` traria (a
/// verificação de ambiente aceita a do sistema; o pacote só entra onde ela falta). Pode rodar
/// processos (o `rustc`, o `python`): chame fora da thread da interface.
pub fn on_system(name: &str) -> Option<String> {
    let program = |p: &str| which(p).and_then(outside_packages);
    match name {
        "rust" => program("cargo"),
        "rust-android" => system_rust_has(&ANDROID_ABIS.map(|(_, triple)| triple)),
        "rust-wasm" => system_rust_has(&["wasm32-unknown-unknown"]),
        "jdk" => java_bin().and_then(outside_packages),
        "android-sdk" => android_sdk().and_then(outside_packages),
        "android-ndk" => android_sdk().and_then(|s| android_ndk(&s)).and_then(outside_packages),
        "cargo-ndk" => program("cargo-ndk"),
        "wasm-bindgen" => program("wasm-bindgen"),
        "python" => python().and_then(|(p, _)| which(&p)).and_then(outside_packages),
        "node" => program("node"),
        "dotnet" => program("dotnet"),
        "go" => program("go"),
        "nsis" => crate::installer::makensis().and_then(outside_packages),
        "clangd" => program("clangd"),
        "netcoredbg" => program("netcoredbg"),
        "bundletool" => crate::publish::bundletool().and_then(outside_packages),
        _ => None,
    }
}

/// O que instalar para ter o pacote `name`: as dependências que a máquina já tem no sistema
/// ficam de fora (o JDK do bundletool, o Rust do cargo-ndk), e os alvos do Rust de um Rust do
/// sistema também (eles entram pelo rustup). `None`: o pacote não serve aqui (não existe para
/// esta plataforma, falta o hash, ou é um alvo do Rust com o Rust do sistema).
pub fn package_plan(manifest: &crate::toolpacks::Manifest, name: &str, system: &dyn Fn(&str) -> bool) -> Option<(Vec<String>, Vec<String>)> {
    use crate::toolpacks::Install;
    let package = manifest.package(name)?;
    if !package.available() || package.files.is_empty() || !package.files.iter().all(|f| crate::toolpacks::is_sha256(&f.sha256)) {
        return None;
    }
    let order = manifest.with_dependencies(&[name]).ok()?;
    let mut skip: Vec<String> = Vec::new();
    for n in &order {
        let p = manifest.package(n)?;
        let rust_target_of_system = matches!(p.install, Install::RustTarget(_)) && p.depends.iter().any(|d| skip.contains(d));
        if n == name {
            if rust_target_of_system {
                return None;
            }
            continue;
        }
        if rust_target_of_system || system(n) {
            skip.push(n.clone());
        }
    }
    let install: Vec<String> = order.into_iter().filter(|n| !skip.contains(n)).collect();
    // Uma dependência que não existe para esta plataforma: o pacote não serve aqui.
    if !install.iter().all(|n| manifest.package(n).is_some_and(|p| p.available())) {
        return None;
    }
    Some((install, skip))
}

/// A correção que instala o pacote `name` (com `fallback` como alternativa sem origem).
fn package_fix(name: &str, fallback: Option<&Fix>) -> Option<Fix> {
    let installer = crate::toolpacks::Installer::new().ok()?;
    let (install, _) = package_plan(&installer.manifest, name, &|n| installer.installed_dir(n).is_none() && on_system(n).is_some())?;
    let package = installer.manifest.package(name)?;
    let installed = installer.installed();
    let size: u64 = install
        .iter()
        .filter(|n| !installed.iter().any(|i| &i.name == *n))
        .filter_map(|n| installer.manifest.package(n))
        .map(|p| p.size())
        .sum();
    let others: Vec<&String> = install.iter().filter(|n| *n != name).collect();
    let with = if others.is_empty() { String::new() } else { format!(", com {}", others.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")) };
    let mut fix = match fallback {
        Some(f) => Fix { interactive: false, ..f.clone() },
        None => Fix::new("", &["aylis-tool", "packages", "install", name]),
    };
    fix.description = format!(
        "Instala o pacote {} {} do Aylis (~{} MB{with}) em {}, conferindo o SHA-256",
        package.title,
        package.version,
        size.div_ceil(1024 * 1024),
        installer.paths.tools.display()
    );
    fix.fetch = Some(name.to_string());
    Some(fix)
}

/// Há de onde obter o pacote `name`: uma origem configurada (a pasta `pacotes`, uma pasta ou
/// um espelho das Opções) ou os arquivos já no cache.
pub fn package_has_source(name: &str) -> bool {
    let Ok(installer) = crate::toolpacks::Installer::new() else { return false };
    if installer.sources.has_any() {
        return true;
    }
    let Ok(order) = installer.manifest.with_dependencies(&[name]) else { return false };
    order
        .iter()
        .filter(|n| installer.installed_dir(n).is_none())
        .filter_map(|n| installer.manifest.package(n))
        .flat_map(|p| p.files)
        .all(|f| installer.paths.cache.join(&f.name).is_file())
}

/// Instala o pacote `name` pelas origens, sem as dependências que a máquina já tem; o
/// andamento vai para `sink` (as etapas e, a cada 5%, os bytes).
pub fn install_package(name: &str, control: &crate::Control, sink: crate::Sink) -> Result<(), String> {
    let installer = crate::toolpacks::Installer::new()?;
    if !package_has_source(name) {
        return Err(NO_PACKAGE_SOURCE.into());
    }
    let skip = package_plan(&installer.manifest, name, &|n| installer.installed_dir(n).is_none() && on_system(n).is_some()).map(|(_, skip)| skip).unwrap_or_default();
    let mut shown = std::collections::HashMap::<String, u64>::new();
    installer.install_except(&[name], &skip, control, &mut |p| {
        if let Some(line) = progress_line(&p, &mut shown) {
            sink(crate::Event::Line(line, crate::LineKind::Tool));
        }
    })?;
    // O PATH e as variáveis do pacote novo valem já (a verificação seguinte o acha).
    crate::setup::refresh_path();
    Ok(())
}

/// Uma linha do andamento de um pacote: as etapas sempre; os bytes a cada 5% (`shown`: o
/// último mostrado de cada arquivo).
pub fn progress_line(p: &crate::toolpacks::Progress, shown: &mut std::collections::HashMap<String, u64>) -> Option<String> {
    match p {
        crate::toolpacks::Progress::Step(s) => Some(s.clone()),
        crate::toolpacks::Progress::Bytes { file, done, total, .. } if *total > 0 => {
            let percent = done * 100 / total;
            let last = shown.entry(file.clone()).or_insert(0);
            if percent < *last + 5 && done != total {
                return None;
            }
            *last = percent;
            Some(format!("{file}: {percent}% ({} de {} MB)", done >> 20, total.div_ceil(1 << 20)))
        }
        crate::toolpacks::Progress::Bytes { .. } => None,
    }
}

/// Um item verificado.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub target: Target,
    pub name: String,
    pub status: Status,
    /// Versão encontrada, ou o que falta.
    pub detail: String,
    pub fix: Option<Fix>,
    /// Instrução quando não há comando automático (link, passo manual).
    pub help: Option<String>,
}

impl Check {
    fn ok(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Ok, detail: detail.into(), fix: None, help: None }
    }

    fn missing(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Missing, detail: detail.into(), fix: None, help: None }
    }

    fn warning(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Warning, detail: detail.into(), fix: None, help: None }
    }

    fn fix(mut self, fix: Fix) -> Self {
        self.fix = Some(fix);
        self
    }

    fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
}

/// Tudo que `target` precisa está presente (avisos não impedem).
pub fn ready(checks: &[Check], target: Target) -> bool {
    checks.iter().filter(|c| c.target == target).all(|c| c.status != Status::Missing)
}

/// O que falta para `target` (para a mensagem antes de executar).
pub fn missing(checks: &[Check], target: Target) -> Vec<&Check> {
    checks.iter().filter(|c| c.target == target && c.status == Status::Missing).collect()
}

/// Verifica o ambiente para as plataformas pedidas, tudo de novo (sem usar o resultado
/// guardado, mas guardando o novo). `aylis_host` é o `aylis-host` que roda projetos Python (ao
/// lado da IDE). Roda as ferramentas: chame fora da thread da interface.
pub fn check(language: Language, targets: &[Target], aylis_host: Option<&Path>) -> Vec<Check> {
    let mut request = Request::new(language, targets, aylis_host);
    request.force = targets.to_vec();
    check_with(&request, &|_, _| {})
}

/// Um pedido de verificação ([`check_with`]).
#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub language: Language,
    pub targets: &'a [Target],
    pub aylis_host: Option<&'a Path>,
    /// As plataformas conferidas de novo mesmo com as marcas iguais ("Verificar de novo", o
    /// que um "Corrigir" mexeu); com alguma, a conferência do Rust também.
    pub force: Vec<Target>,
    /// Confere de novo o que muda sem mexer em arquivo (o aparelho conectado, o simulador
    /// aberto), mesmo com o resultado guardado valendo. O F9 não precisa (são avisos).
    pub live: bool,
    /// Plataforma guardada com falta é conferida de novo (o F9: antes de recusar, confere se a
    /// pessoa já resolveu).
    pub recheck_missing: bool,
    /// Onde guardar (`None`: [`crate::env_cache::file`]; os testes usam outro).
    pub cache_file: Option<PathBuf>,
}

impl<'a> Request<'a> {
    pub fn new(language: Language, targets: &'a [Target], aylis_host: Option<&'a Path>) -> Self {
        Self { language, targets, aylis_host, force: Vec::new(), live: true, recheck_missing: false, cache_file: None }
    }
}

/// Os itens que mudam sem mexer em arquivo (veja [`Request::live`]): ficam no fim da lista
/// da plataforma.
const LIVE_ITEMS: [&str; 2] = ["Aparelho conectado", "Simulador aberto"];

/// Verifica as plataformas pedidas, cada uma numa thread: a que tem resultado guardado com as
/// marcas iguais ([`crate::env_cache`]) não roda processo nenhum. `done` recebe cada plataforma
/// assim que fica pronta (a tela mostra sem esperar as outras). Devolve tudo, na ordem pedida.
pub fn check_with(request: &Request, done: &(dyn Fn(Target, &[Check]) + Sync)) -> Vec<Check> {
    use crate::env_cache::{self as cache, Entry, Inputs};
    // O que acabou de ser instalado (pela IDE ou fora dela) entra no PATH sem reiniciar.
    crate::setup::refresh_path();
    let file = request.cache_file.clone().or_else(cache::file);
    let stored = file.as_deref().map(cache::load).unwrap_or_default();
    let now = cache::now();
    let inputs = Inputs { aylis_host: request.aylis_host.map(|p| p.display().to_string()) };
    let rust = SharedRust { cell: OnceLock::new(), cached: stored.entries.get(cache::RUST_KEY), force: !request.force.is_empty(), inputs: &inputs, now };
    let results: Vec<(Target, Vec<Check>, Option<Entry>)> = std::thread::scope(|s| {
        let handles: Vec<_> = request
            .targets
            .iter()
            .map(|&target| {
                let entry = stored.entries.get(&cache::key(target, request.language));
                let (rust, inputs) = (&rust, &inputs);
                let handle = s.spawn(move || {
                    let reuse = entry.filter(|e| {
                        !request.force.contains(&target) && e.valid(inputs, now) && !(request.recheck_missing && !ready(&e.checks, target))
                    });
                    if let Some(entry) = reuse {
                        let mut checks = entry.checks.clone();
                        if request.live {
                            checks.retain(|c| !LIVE_ITEMS.contains(&c.name.as_str()));
                            live_items(&mut checks, target, request.language);
                        }
                        done(target, &checks);
                        return (checks, None);
                    }
                    let (checks, deps) = cache::record(|| platform(target, request.language, rust, request.aylis_host));
                    done(target, &checks);
                    (checks.clone(), Some(Entry { saved: now, deps, checks, extra: serde_json::Value::Null }))
                });
                (target, handle)
            })
            .collect();
        handles
            .into_iter()
            .map(|(target, h)| match h.join() {
                Ok((checks, entry)) => (target, checks, entry),
                // Um pânico numa conferência não derruba as outras (nem a tela).
                Err(_) => (target, vec![Check::missing(target, "Verificação", "a conferência desta plataforma falhou")], None),
            })
            .collect()
    });
    let mut updates = std::collections::BTreeMap::new();
    if let Some((rust, deps, true)) = rust.cell.get() {
        updates.insert(cache::RUST_KEY.to_string(), Entry { saved: now, deps: deps.clone(), checks: Vec::new(), extra: rust.to_json() });
    }
    let mut out = Vec::new();
    for (target, checks, entry) in results {
        if let Some(e) = entry {
            updates.insert(cache::key(target, request.language), e);
        }
        out.extend(checks);
    }
    if let Some(file) = &file {
        cache::store(file, &updates);
    }
    out
}

/// O que uma plataforma precisa (sem resultado guardado).
fn platform(target: Target, language: Language, rust: &SharedRust, aylis_host: Option<&Path>) -> Vec<Check> {
    let mut out = Vec::new();
    // As escolhas das Opções (o caminho de cada ferramenta) valem para todas.
    if let Some(settings) = crate::settings::path() {
        crate::env_cache::note_path(&settings);
    }
    note_packages();
    match target {
        Target::Desktop => desktop(&mut out, language, rust, aylis_host),
        Target::Android => android(&mut out, language, rust),
        Target::Web => web(&mut out, language, rust),
        Target::Ios => ios(&mut out, language, rust),
        Target::Linux => linux_wsl(&mut out, language),
    }
    out
}

/// O Rust, conferido uma vez para todas as plataformas (na primeira que pedir; as outras
/// esperam): do resultado guardado, se as marcas dele não mudaram.
struct SharedRust<'a> {
    /// O resultado, do que dependeu e se foi conferido agora (para guardar).
    cell: OnceLock<(Rust, Vec<crate::env_cache::Dep>, bool)>,
    cached: Option<&'a crate::env_cache::Entry>,
    force: bool,
    inputs: &'a crate::env_cache::Inputs,
    now: u64,
}

impl SharedRust<'_> {
    /// O Rust (as marcas dele entram nas da plataforma que perguntou).
    fn get(&self) -> &Rust {
        let (rust, deps, _) = self.cell.get_or_init(|| {
            if let Some(entry) = self.cached.filter(|e| !self.force && e.valid(self.inputs, self.now))
                && let Some(rust) = Rust::from_json(&entry.extra)
            {
                return (rust, entry.deps.clone(), false);
            }
            let (rust, deps) = crate::env_cache::record(rust_checks);
            (rust, deps, true)
        });
        crate::env_cache::note_all(deps);
        rust
    }
}

/// O que muda sem mexer em arquivo, no fim da lista da plataforma ([`LIVE_ITEMS`]).
fn live_items(out: &mut Vec<Check>, target: Target, language: Language) {
    match target {
        Target::Android => {
            if let Some(sdk) = android_sdk().filter(|sdk| adb(sdk).is_file()) {
                out.push(android_device_check(&sdk));
            }
        }
        Target::Ios if cfg!(target_os = "macos") && !language.uses_host() => {
            let booted = output("xcrun", &["simctl", "list", "devices", "booted"]).is_some_and(|o| o.contains("Booted"));
            out.push(if booted {
                Check::ok(Target::Ios, "Simulador aberto", "sim")
            } else {
                Check::warning(Target::Ios, "Simulador aberto", "nenhum (só para Executar)").fix(Fix::new("Abre o Simulador", &["open", "-a", "Simulator"]).interactive())
            });
        }
        _ => {}
    }
}

/// O CodeLLDB (o pacote da extensão do VS Code, ~50 MB, do GitHub), na versão fixada,
/// extraído na pasta de ferramentas do aylis, onde a IDE procura.
fn install_codelldb() -> Option<Fix> {
    Fix::download("codelldb")
}

/// O que se sabe do Rust (verificado uma vez para todas as plataformas).
#[derive(Default)]
struct Rust {
    cargo: Option<String>,
    /// O cargo existe, mas falhou (o toolchain não está instalado, por exemplo): a mensagem.
    cargo_error: Option<String>,
    rustc: Option<(u32, u32, u32)>,
    targets: Vec<String>,
    has_rustup: bool,
}

impl Rust {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "cargo": self.cargo,
            "cargo_error": self.cargo_error,
            "rustc": self.rustc.map(|(a, b, c)| [a, b, c]),
            "targets": self.targets,
            "has_rustup": self.has_rustup,
        })
    }

    fn from_json(v: &serde_json::Value) -> Option<Rust> {
        let text = |v: &serde_json::Value| v.as_str().map(str::to_string);
        let rustc = v["rustc"].as_array().and_then(|a| Some((a.first()?.as_u64()? as u32, a.get(1)?.as_u64()? as u32, a.get(2)?.as_u64()? as u32)));
        Some(Rust {
            cargo: text(&v["cargo"]),
            cargo_error: text(&v["cargo_error"]),
            rustc,
            targets: v["targets"].as_array()?.iter().filter_map(text).collect(),
            has_rustup: v["has_rustup"].as_bool()?,
        })
    }
}

/// As pastas do rustup: os toolchains (o `rustup update` e o `target add` mexem nelas) e a
/// escolha do toolchain padrão. Os `cargo`/`rustc` do PATH são só os atalhos do rustup, que
/// não mudam quando o toolchain muda.
fn note_rustup() {
    let home = std::env::var_os("RUSTUP_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(|h| PathBuf::from(h).join(".rustup")));
    crate::env_cache::note_var("RUSTUP_HOME");
    crate::env_cache::note_var("RUSTUP_TOOLCHAIN");
    let Some(home) = home else { return };
    crate::env_cache::note_path(&home.join("settings.toml"));
    let toolchains = home.join("toolchains");
    crate::env_cache::note_path(&toolchains);
    for dir in std::fs::read_dir(&toolchains).into_iter().flatten().flatten().map(|e| e.path()) {
        crate::env_cache::note_path(&dir.join("bin"));
        crate::env_cache::note_path(&dir.join("lib").join("rustlib"));
        crate::env_cache::note_path(&dir.join("lib").join("rustlib").join("multirust-channel-manifest.toml"));
    }
}

/// O registro dos pacotes de ferramentas instalados: instalar ou remover um pacote (pela IDE
/// ou pelo `aylis-tool packages`) muda a marca dele, e a verificação guardada deixa de valer.
fn note_packages() {
    if let Some(paths) = crate::toolpacks::Paths::user() {
        crate::env_cache::note_path(&paths.registry());
    }
}

/// O Rust: os três processos ao mesmo tempo (cada um anota o que rodou).
fn rust_checks() -> Rust {
    note_rustup();
    note_packages();
    let (cargo, rustc, targets) = std::thread::scope(|s| {
        let cargo = side(s, || run("cargo", &["--version"]));
        let rustc = side(s, || output("rustc", &["--version"]));
        let targets = output("rustup", &["target", "list", "--installed"]);
        (joined(cargo), joined(rustc).flatten(), targets)
    });
    let (cargo, cargo_error) = match cargo {
        Some(Ok(v)) => (Some(v), None),
        Some(Err(e)) => (None, e),
        None => (None, None),
    };
    let rustc = rustc.and_then(|v| parse_version(&v));
    let has_rustup = targets.is_some();
    // Sem o rustup (o Rust de um pacote do Aylis), os alvos são as pastas do sysroot.
    let targets = match targets {
        Some(t) => t.lines().map(|l| l.trim().to_string()).collect(),
        None if cargo.is_some() => sysroot_targets(),
        None => Vec::new(),
    };
    Rust { cargo, cargo_error, rustc, has_rustup, targets }
}

/// Os alvos instalados no sysroot do `rustc` (as pastas de `lib/rustlib` com a biblioteca).
fn sysroot_targets() -> Vec<String> {
    let Some(sysroot) = output("rustc", &["--print", "sysroot"]) else { return Vec::new() };
    let rustlib = PathBuf::from(sysroot.trim()).join("lib").join("rustlib");
    crate::env_cache::note_path(&rustlib);
    std::fs::read_dir(&rustlib)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().join("lib").is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

fn rust_items(out: &mut Vec<Check>, target: Target, rust: &Rust) {
    match &rust.cargo {
        Some(v) => out.push(Check::ok(target, "Rust (cargo)", v.trim())),
        None => {
            // O erro de verdade, quando o cargo existe e falhou (antes era sempre "não
            // encontrado"; o relatório do Tabernáculo, I02).
            let detail = rust.cargo_error.as_deref().and_then(|e| e.lines().map(str::trim).find(|l| !l.is_empty())).unwrap_or("não encontrado");
            let check = Check::missing(target, "Rust (cargo)", detail);
            // Com o rustup já na máquina, ele baixa o toolchain; senão, o pacote do Aylis.
            let fix = match crate::setup::install_toolchain(Language::Rust) {
                Some(fix) if crate::setup::existing_rustup().is_some() => Some(fix),
                Some(fix) => Some(Fix::package_or("rust", fix)),
                None => Fix::download("rust"),
            };
            out.push(match fix {
                Some(fix) => check.fix(fix),
                None => check.help("Instale pelo https://rustup.rs (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)"),
            });
            return;
        }
    }
    match rust.rustc {
        Some((major, minor, patch)) if (major, minor) >= MIN_RUST => {
            out.push(Check::ok(target, "Versão do Rust", format!("{major}.{minor}.{patch}")));
        }
        Some((major, minor, patch)) => out.push(
            Check::missing(target, "Versão do Rust", format!("{major}.{minor}.{patch} (precisa de {}.{} ou mais nova)", MIN_RUST.0, MIN_RUST.1))
                .fix(match which("cargo").and_then(outside_packages) {
                    // O Rust em uso é o do pacote: o pacote na versão do manifesto.
                    None => Fix::download("rust").unwrap_or_else(|| Fix::new("Atualiza o Rust", &["rustup", "update", "stable"])),
                    Some(_) => Fix::new("Atualiza o Rust", &["rustup", "update", "stable"]),
                }),
        ),
        None => out.push(Check::missing(target, "Versão do Rust", "rustc não respondeu")),
    }
}

/// Alvos do Rust que faltam (com a correção: o rustup, se houver; senão, o pacote dos alvos
/// `package`, que entra no Rust do pacote).
fn rust_targets(out: &mut Vec<Check>, target: Target, rust: &Rust, name: &str, wanted: &[&str], package: &str) {
    let missing: Vec<&str> = wanted.iter().copied().filter(|t| !rust.targets.iter().any(|i| i == t)).collect();
    if missing.is_empty() {
        out.push(Check::ok(target, name, wanted.join(", ")));
    } else if !rust.has_rustup {
        let check = Check::missing(target, name, format!("faltam {}", missing.join(", ")));
        out.push(match Fix::download(package) {
            Some(fix) => check.fix(fix),
            None => check.help("Instale o rustup (https://rustup.rs) para adicionar alvos"),
        });
    } else {
        let mut args = vec!["rustup", "target", "add"];
        args.extend(&missing);
        out.push(
            Check::missing(target, name, format!("faltam {}", missing.join(", ")))
                .fix(Fix::new(format!("Adiciona os alvos {}", missing.join(", ")), &args)),
        );
    }
}

// ---------------------------------------------------------------------------
// Desktop

/// Do que o desktop depende além dos programas que roda: a pasta de ferramentas do Aylis (o
/// CodeLLDB, o WiX, os servidores de linguagem), as extensões do VS Code, o Visual Studio e o
/// `aylis-host`.
fn note_desktop(language: Language, aylis_host: Option<&Path>) {
    use crate::env_cache::{note_input, note_path, note_var};
    note_tools_dir();
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        note_path(&PathBuf::from(home).join(".vscode").join("extensions"));
    }
    note_var("JAVA_HOME");
    if cfg!(windows) {
        // As instalações do Visual Studio (o vswhere lê daqui).
        if let Some(data) = std::env::var_os("ProgramData") {
            note_path(&PathBuf::from(data).join("Microsoft").join("VisualStudio").join("Packages").join("_Instances"));
        }
    }
    match language {
        Language::Pascal => {
            note_var("LAZARUSDIR");
            for dir in ["C:/lazarus", "C:/Program Files/Lazarus", "/usr/lib/lazarus", "/usr/share/lazarus"] {
                note_path(Path::new(dir));
            }
        }
        Language::Python => {
            // O `pip install` mexe na pasta de pacotes do Python achado.
            for name in ["python", "py", "python3"] {
                if let Some(dir) = crate::find_in_path(name).and_then(|p| p.parent().map(Path::to_path_buf)) {
                    note_path(&dir.join("Lib").join("site-packages"));
                }
            }
            if let Some(data) = std::env::var_os("APPDATA") {
                note_path(&PathBuf::from(data).join("Python"));
            }
        }
        _ => {}
    }
    note_input("aylis-host", aylis_host.map(|p| p.display().to_string()));
    if let Some(host) = aylis_host {
        note_path(host);
    }
}

/// A pasta de ferramentas do Aylis e cada subpasta dela (o que o "Corrigir" instala vai para
/// lá).
fn note_tools_dir() {
    let Some(tools) = crate::dap::tools_dir() else { return };
    crate::env_cache::note_path(&tools);
    for dir in std::fs::read_dir(&tools).into_iter().flatten().flatten().map(|e| e.path()) {
        crate::env_cache::note_path(&dir);
    }
}

fn desktop(out: &mut Vec<Check>, language: Language, rust: &SharedRust, aylis_host: Option<&Path>) {
    let t = Target::Desktop;
    note_desktop(language, aylis_host);
    match language {
        Language::Rust => {
            rust_items(out, t, rust.get());
            out.push(match crate::dap::codelldb() {
                Some(p) => Check::ok(t, "CodeLLDB (depurar na IDE)", p.display().to_string()),
                None => {
                    let check = Check::warning(t, "CodeLLDB (depurar na IDE)", "não instalado (só para depurar)");
                    match install_codelldb() {
                        Some(fix) => check.fix(fix),
                        None => check.help("Baixe o .vsix em github.com/vadimcn/codelldb/releases e extraia em <ferramentas do Aylis>/codelldb"),
                    }
                }
            });
            if cfg!(windows) {
                out.push(match crate::publish::wix() {
                    Some(p) => Check::ok(t, "WiX (instalador .msi)", p.display().to_string()),
                    None => {
                        let check = Check::warning(t, "WiX (instalador .msi)", "não instalado (só para o instalador)");
                        match (crate::dap::tools_dir(), which("dotnet")) {
                            (Some(dir), Some(_)) => check.fix(Fix::new(
                                "Instala o WiX 5 (dotnet tool) na pasta de ferramentas do Aylis",
                                &["dotnet", "tool", "install", "wix", "--version", "5.0.2", "--tool-path", &dir.join("wix").display().to_string()],
                            )),
                            _ => check.help("Precisa do .NET SDK (dotnet.microsoft.com); depois: dotnet tool install --global wix --version 5.0.2"),
                        }
                    }
                });
                out.push(match msvc_linker() {
                    Some(v) => Check::ok(t, "Linker do Visual C++ (MSVC)", v),
                    None => Check::missing(t, "Linker do Visual C++ (MSVC)", "não encontrado (o Rust no Windows usa o link.exe)").fix(crate::setup::msvc()),
                });
            } else if cfg!(target_os = "macos") {
                out.push(match output("xcode-select", &["-p"]) {
                    Some(p) => Check::ok(t, "Ferramentas de linha de comando do Xcode", p.trim()),
                    None => Check::missing(t, "Ferramentas de linha de comando do Xcode", "não encontradas")
                        .fix(Fix::new("Instala as ferramentas do Xcode", &["xcode-select", "--install"]).interactive()),
                });
            } else {
                out.push(match output("cc", &["--version"]) {
                    Some(v) => Check::ok(t, "Compilador C (linker)", v.lines().next().unwrap_or_default()),
                    None => Check::missing(t, "Compilador C (linker)", "cc não encontrado")
                        .help("Instale o gcc ou clang (ex.: sudo apt install build-essential)"),
                });
            }
        }
        Language::Python => {
            out.push(match python() {
                Some((program, (3, minor, patch))) if minor >= 8 => Check::ok(t, "Python 3.8+", format!("3.{minor}.{patch} ({program})")),
                Some((_, (major, minor, patch))) => Check::missing(t, "Python 3.8+", format!("{major}.{minor}.{patch} é antigo")).fix(install_python()),
                None => Check::missing(t, "Python 3.8+", "não encontrado").fix(install_python()),
            });
            if let Some((program, _)) = python() {
                let has = output(&program, &["-c", "import debugpy"]).is_some();
                out.push(if has {
                    Check::ok(t, "debugpy (depurar na IDE e no VS Code)", "instalado")
                } else {
                    Check::warning(t, "debugpy (depurar na IDE e no VS Code)", "não instalado (só para depurar)")
                        .fix(Fix::new("Instala o debugpy", &[program.as_str(), "-m", "pip", "install", "debugpy"]))
                });
            }
            out.push(match aylis_host.filter(|h| h.is_file()) {
                Some(h) => Check::ok(t, "aylis-host", h.display().to_string()),
                None => Check::missing(t, "aylis-host", "não encontrado ao lado da IDE").fix(Fix::new(
                    "Compila o aylis-host (na pasta do Aylis)",
                    &["cargo", "build", "-p", "aylis-host", "--manifest-path", &crate::aylis_root().join("Cargo.toml").display().to_string()],
                )),
            });
        }
        other => desktop_host(out, other, aylis_host),
    }
    // O servidor de linguagem (sugestões, declaração, erros enquanto digita no editor).
    out.push(language_server(language));
}

/// TypeScript, C#, Java, Go, C++ e Free Pascal: o compilador e o aylis-host.
fn desktop_host(out: &mut Vec<Check>, language: Language, aylis_host: Option<&Path>) {
    let t = Target::Desktop;
    let label = format!("{} (compilar o backend)", language.name());
    let toolchain = crate::hostlang::find(language);
    out.push(match &toolchain {
        Ok(tc) => Check::ok(t, &label, format!("{} ({})", tc.describe, tc.program.display())),
        Err(e) => match crate::setup::install_toolchain(language).map(|fix| match toolchain_package(language) {
            Some(package) => Fix::package_or(package, fix),
            None => fix,
        }) {
            Some(fix) => Check::missing(t, &label, e.clone()).fix(fix),
            None => Check::missing(t, &label, e.clone()),
        },
    });
    out.push(host_debugger(language, toolchain.ok().map(|tc| tc.program)));
    out.push(match aylis_host.filter(|h| h.is_file()) {
        Some(h) => Check::ok(t, "aylis-host", h.display().to_string()),
        None => Check::missing(t, "aylis-host", "não encontrado ao lado da IDE").fix(Fix::new(
            "Compila o aylis-host (na pasta do Aylis)",
            &["cargo", "build", "-p", "aylis-host", "--manifest-path", &crate::aylis_root().join("Cargo.toml").display().to_string()],
        )),
    });
}

/// O pacote do compilador de uma linguagem (as que o manifesto tem).
fn toolchain_package(language: Language) -> Option<&'static str> {
    match language {
        Language::TypeScript => Some("node"),
        Language::CSharp => Some("dotnet"),
        Language::Java => Some("jdk"),
        Language::Go => Some("go"),
        Language::Python => Some("python"),
        _ => None,
    }
}

/// O depurador da linguagem (depurar na IDE): CodeLLDB (Free Pascal e C++), Delve (Go),
/// netcoredbg (C#), js-debug com o Node (TypeScript) e o `jdb` do JDK (Java).
fn host_debugger(language: Language, program: Option<PathBuf>) -> Check {
    let t = Target::Desktop;
    let (name, found) = match language {
        Language::Pascal | Language::Cpp => ("CodeLLDB (depurar na IDE)", crate::dap::codelldb()),
        Language::Go => ("Delve (depurar na IDE)", crate::dap::delve()),
        Language::CSharp => ("netcoredbg (depurar na IDE)", crate::dap::netcoredbg()),
        Language::TypeScript => ("js-debug (depurar na IDE)", crate::dap::js_debug()),
        _ => ("jdb (depurar na IDE)", crate::dap::jdb()),
    };
    if let Some(p) = found {
        if language == Language::TypeScript {
            // O js-debug só fala com o inspetor do Node (o do Bun é outro).
            if let Err(e) = crate::hostlang::node() {
                return Check::warning(t, name, format!("precisa do Node 22.6+ para depurar: {e}")).help("Instale o Node 22.6+ (nodejs.org)");
            }
        }
        return Check::ok(t, name, p.display().to_string());
    }
    let check = Check::warning(t, name, "não instalado (só para depurar)");
    let fix = match language {
        Language::Pascal | Language::Cpp => install_codelldb(),
        Language::Go => install_delve(program),
        Language::CSharp => install_netcoredbg(),
        Language::TypeScript => install_js_debug(),
        _ => None,
    };
    match fix {
        Some(fix) => check.fix(fix),
        None if language == Language::Java => check.help("O jdb vem com o JDK (Java 17+): ponha a pasta bin do JDK no PATH ou defina JAVA_HOME"),
        None => check.help("Sem instalador automático nesta plataforma"),
    }
}

/// Um `go install` na versão fixada, com o executável em `bin`. O Go confere o módulo no
/// banco de somas dele (sum.golang.org).
fn go_install(description: &str, go: &Path, module: &str, package: &str, bin: &Path) -> Fix {
    let version = crate::downloads::version(package);
    let description = format!("{description} (go install {module}@{version}) em {}", bin.display());
    // O GOBIN vai pelo ambiente do processo: nada de shell no meio.
    let mut fix = Fix::new(description, &[&go.display().to_string(), "install", &format!("{module}@{version}")]);
    fix.env.push(("GOBIN".into(), bin.display().to_string()));
    fix
}

/// O Delve pelo `go install` (versão fixada), com o executável na pasta de ferramentas do aylis.
fn install_delve(go: Option<PathBuf>) -> Option<Fix> {
    let bin = crate::dap::tools_dir()?.join("delve");
    Some(go_install("Instala o Delve", &go?, "github.com/go-delve/delve/cmd/dlv", "delve", &bin))
}

/// O netcoredbg (Samsung, ~4 MB, do GitHub), na versão fixada.
fn install_netcoredbg() -> Option<Fix> {
    Fix::download("netcoredbg")
}

/// O servidor DAP do js-debug (o depurador de JavaScript do VS Code, do GitHub).
fn install_js_debug() -> Option<Fix> {
    Fix::download("js-debug")
}

/// O servidor de linguagem do editor de código (sugestões, ir para a declaração, os erros
/// enquanto digita). Sem ele o editor funciona com o índice próprio dos fontes.
fn language_server(language: Language) -> Check {
    let t = Target::Desktop;
    let label = "Servidor de linguagem (editor de código)";
    if let Some(s) = crate::lsp::server(language, Path::new(".")) {
        return Check::ok(t, label, format!("{} ({})", s.name, s.program.display()));
    }
    let check = Check::warning(t, label, "não instalado (o editor fica sem as sugestões e os erros do servidor)");
    match install_language_server(language) {
        Some(fix) => check.fix(fix),
        None => check.help(match language {
            Language::Pascal => "Precisa do Lazarus (lazarus-ide.org) para compilar o pasls (github.com/castle-engine/pascal-language-server)",
            _ => "Sem instalador automático nesta plataforma",
        }),
    }
}

/// Instala o servidor de linguagem na pasta de ferramentas do aylis.
fn install_language_server(language: Language) -> Option<Fix> {
    let tools = crate::dap::tools_dir()?;
    let dir = |name: &str| tools.join(name).display().to_string();
    Some(match language {
        Language::Rust => Fix::new("Instala o rust-analyzer (rustup component add rust-analyzer)", &["rustup", "component", "add", "rust-analyzer"]),
        Language::Go => {
            let go = crate::hostlang::find(Language::Go).ok()?.program;
            go_install("Instala o gopls", &go, "golang.org/x/tools/gopls", "gopls", &tools.join("gopls"))
        }
        Language::Python => {
            let (python, _) = python()?;
            let version = crate::downloads::version("python-lsp-server");
            Fix::new(
                format!("Instala o python-lsp-server {version} (pip, com pyflakes, pycodestyle, autopep8 e rope) em {}", dir("pylsp")),
                &[python.as_str(), "-m", "pip", "install", "--upgrade", "--target", &dir("pylsp"), &format!("python-lsp-server[pyflakes,pycodestyle,autopep8,rope]=={version}")],
            )
        }
        Language::TypeScript => {
            // O TypeScript fica no 5 (o 7 não tem o tsserver). O npm confere a integridade.
            let (ts, tsls) = (crate::downloads::version("typescript"), crate::downloads::version("typescript-language-server"));
            let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
            Fix::new(
                format!("Instala o typescript-language-server {tsls} e o TypeScript {ts} (npm) em {}", dir("tsls")),
                &[npm, "install", "--prefix", &dir("tsls"), &format!("typescript@{ts}"), &format!("typescript-language-server@{tsls}")],
            )
        }
        Language::CSharp => {
            // Cada csharp-ls pede um .NET: o mais novo, o .NET mais novo; com o SDK 9 e o 8, os
            // últimos que rodam neles.
            let dotnet = crate::hostlang::find(Language::CSharp).ok()?.program;
            let major = crate::env::output(&dotnet.display().to_string(), &["--version"]).and_then(|v| v.trim().split('.').next()?.parse::<u32>().ok()).unwrap_or(8);
            let version = crate::downloads::version(match major {
                ..=8 => "csharp-ls-net8",
                9 => "csharp-ls-net9",
                _ => "csharp-ls",
            });
            Fix::new(
                format!("Instala o csharp-ls {version} (dotnet tool) em {}", dir("csharp-ls")),
                &[&dotnet.display().to_string(), "tool", "install", "csharp-ls", "--version", &version, "--tool-path", &dir("csharp-ls")],
            )
        }
        // O 1.38 é o último que roda no Java 17 (os de depois pedem o 21).
        Language::Java => Fix::download("jdtls")?,
        Language::Cpp => Fix::download("clangd")?,
        Language::Pascal => {
            if !cfg!(windows) {
                return None;
            }
            // O pasls da Castle Engine, compilado com o Lazarus (que o winget instala), do
            // GitHub pelo git (instalado antes, se faltar).
            let src = dir("pasls-src");
            let lazbuild = crate::lsp::lazarus_dir().map(|d| d.join("lazbuild.exe")).filter(|p| p.is_file());
            let quiet = "--silent --accept-package-agreements --accept-source-agreements --disable-interactivity -e";
            let mut install = String::new();
            if lazbuild.is_none() {
                install.push_str(&format!("winget install --id Lazarus.Lazarus {quiet} & "));
            }
            if which("git").is_none() {
                install.push_str(&format!("winget install --id Git.Git {quiet} & "));
            }
            let git = crate::setup::git();
            let commit = crate::downloads::version("pascal-language-server");
            let lazbuild = lazbuild.map_or("C:\\lazarus\\lazbuild.exe".to_string(), |p| p.display().to_string());
            Fix::new(
                format!("{}Compila o pasls (Castle Engine, commit {}) com o Lazarus em {}", if install.is_empty() { "" } else { "Instala o Lazarus (winget) e " }, &commit[..commit.len().min(12)], dir("pasls")),
                &[
                    "cmd",
                    "/C",
                    &format!(
                        "{install}rmdir /s /q \"{src}\" 2>nul & \"{git}\" clone https://github.com/castle-engine/pascal-language-server \"{src}\" && \"{git}\" -C \"{src}\" checkout {commit} && \"{git}\" -C \"{src}\" submodule update --init --recursive && \"{lazbuild}\" --build-mode=Release \"{src}\\server\\pasls.lpi\" && mkdir \"{out}\" 2>nul & copy /y \"{src}\\server\\lib\\x86_64-win64\\pasls.exe\" \"{out}\\pasls.exe\"",
                        out = dir("pasls")
                    ),
                ],
            )
        }
    })
}

fn install_python() -> Fix {
    Fix::package_or("python", crate::setup::install_toolchain(Language::Python).unwrap_or_else(|| Fix::new("Instala o Python 3", &["python3", "--version"])))
}

/// O Python disponível: (programa, versão). No Windows também tenta o `py`.
pub fn python() -> Option<(String, (u32, u32, u32))> {
    let candidates: &[&str] = if cfg!(windows) { &["python", "py", "python3"] } else { &["python3", "python"] };
    candidates.iter().find_map(|p| {
        let v = output(p, &["--version"])?;
        // O "python" da Microsoft Store só abre a loja: não tem versão.
        Some((p.to_string(), parse_version(&v)?))
    })
}

/// O link.exe do Visual C++ (pelo vswhere, como o Rust faz).
fn msvc_linker() -> Option<String> {
    let program_files = std::env::var("ProgramFiles(x86)").ok()?;
    let vswhere = Path::new(&program_files).join("Microsoft Visual Studio/Installer/vswhere.exe");
    if vswhere.is_file() {
        let found = output(
            &vswhere.display().to_string(),
            &["-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "displayName"],
        );
        if let Some(name) = found.map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
            return Some(name);
        }
    }
    which("link").filter(|p| !p.display().to_string().contains("Git")).map(|p| p.display().to_string())
}

// ---------------------------------------------------------------------------
// Android

/// Arquiteturas de Android que o aylis compila: celulares (arm64) e o emulador (x86_64).
pub const ANDROID_ABIS: [(&str, &str); 2] = [("arm64-v8a", "aarch64-linux-android"), ("x86_64", "x86_64-linux-android")];

/// Todas as que o publicar aceita: as padrão e, pedida no projeto (`publish.android.abis`) ou no
/// `publish`, a de 32 bits (TV box, celular de entrada; relatório do RTSPlayer de 13/11, item
/// 5). Fica fora do padrão: dobraria o tempo de compilação e o tamanho do APK, e o Skia não tem
/// binário pronto para ela (falta compilá-lo; veja os pendentes).
pub const ANDROID_ALL_ABIS: [(&str, &str); 3] = [ANDROID_ABIS[0], ANDROID_ABIS[1], ("armeabi-v7a", "armv7-linux-androideabi")];

/// A pasta do Android SDK (`ANDROID_HOME`, `ANDROID_SDK_ROOT` ou o lugar padrão).
pub fn android_sdk() -> Option<PathBuf> {
    let from_env = ["ANDROID_HOME", "ANDROID_SDK_ROOT"].iter().find_map(std::env::var_os).map(PathBuf::from);
    let default = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(|d| PathBuf::from(d).join("Android").join("Sdk"))
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Android/sdk"))
    } else {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Android/Sdk"))
    };
    from_env.into_iter().chain(default).find(|p| p.is_dir())
}

/// A subpasta mais nova (por nome) de `dir` que satisfaz `ok`.
fn newest(dir: &Path, ok: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let mut all: Vec<PathBuf> = std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir() && ok(p)).collect();
    all.sort_by_key(|p| version_key(&p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()));
    all.pop()
}

/// "34.0.0" e "android-34" ordenados como números.
fn version_key(name: &str) -> Vec<u64> {
    name.split(|c: char| !c.is_ascii_digit()).filter_map(|n| n.parse().ok()).collect()
}

pub fn android_ndk(sdk: &Path) -> Option<PathBuf> {
    if let Some(ndk) = std::env::var_os("ANDROID_NDK_HOME").map(PathBuf::from).filter(|p| p.is_dir()) {
        return Some(ndk);
    }
    newest(&sdk.join("ndk"), |p| p.join("source.properties").is_file())
}

pub fn android_build_tools(sdk: &Path) -> Option<PathBuf> {
    newest(&sdk.join("build-tools"), |p| p.join(exe("aapt2")).is_file())
}

pub fn android_platform(sdk: &Path) -> Option<PathBuf> {
    newest(&sdk.join("platforms"), |p| p.join("android.jar").is_file())
}

pub fn adb(sdk: &Path) -> PathBuf {
    sdk.join("platform-tools").join(exe("adb"))
}

fn sdkmanager(sdk: &Path) -> Option<PathBuf> {
    let name = if cfg!(windows) { "sdkmanager.bat" } else { "sdkmanager" };
    let latest = sdk.join("cmdline-tools/latest/bin").join(name);
    if latest.is_file() {
        return Some(latest);
    }
    newest(&sdk.join("cmdline-tools"), |p| p.join("bin").join(name).is_file()).map(|p| p.join("bin").join(name))
}

/// `nome.exe` no Windows.
pub fn exe(name: &str) -> String {
    if cfg!(windows) { format!("{name}.exe") } else { name.to_string() }
}

/// A pasta `bin` do Java (JDK): `JAVA_HOME` ou o `java` do PATH.
pub fn java_bin() -> Option<PathBuf> {
    if let Some(home) = std::env::var_os("JAVA_HOME").map(PathBuf::from) {
        let bin = home.join("bin");
        if bin.join(exe("java")).is_file() {
            return Some(bin);
        }
    }
    which("java").and_then(|j| j.parent().map(Path::to_path_buf))
}

/// Os aparelhos conectados (`adb devices`): números de série.
pub fn android_devices(sdk: &Path) -> Vec<String> {
    android_device_list(sdk).into_iter().filter(|d| d.is_ready()).map(|d| d.serial).collect()
}

/// Um aparelho que o adb vê.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AndroidDevice {
    pub serial: String,
    /// O modelo (`ASUS_X00QD`); vazio enquanto o aparelho não autoriza.
    pub model: String,
    /// `device` (pronto), `unauthorized`, `offline`...
    pub state: String,
}

impl AndroidDevice {
    pub fn is_ready(&self) -> bool {
        self.state == "device"
    }

    /// O nome para a lista da IDE: "ASUS_X00QD (JBAXB601U871Z49)", com o estado se não está
    /// pronto.
    pub fn label(&self) -> String {
        let name = if self.model.is_empty() { self.serial.clone() } else { format!("{} ({})", self.model, self.serial) };
        match self.state.as_str() {
            "device" => name,
            "unauthorized" => format!("{name} — sem autorização"),
            "offline" => format!("{name} — offline"),
            other => format!("{name} — {other}"),
        }
    }
}

/// Os aparelhos do `adb devices -l`, também os que não estão prontos (sem autorização,
/// offline), para a IDE mostrar.
pub fn android_device_list(sdk: &Path) -> Vec<AndroidDevice> {
    output(&adb(sdk).display().to_string(), &["devices", "-l"]).map(|o| parse_devices(&o)).unwrap_or_default()
}

/// Lê a saída do `adb devices -l`.
fn parse_devices(text: &str) -> Vec<AndroidDevice> {
    text.lines()
        .filter(|l| !l.starts_with("List of devices") && !l.starts_with('*'))
        .filter_map(|l| {
            let mut words = l.split_whitespace();
            let serial = words.next()?.to_string();
            let state = words.next()?.to_string();
            let model = words.find_map(|w| w.strip_prefix("model:")).unwrap_or_default().to_string();
            Some(AndroidDevice { serial, model, state })
        })
        .collect()
}

/// O estado de um aparelho (`adb get-state`): `device` quando pronto; `None` se sumiu.
pub fn android_device_state(sdk: &Path, serial: &str) -> Option<String> {
    output(&adb(sdk).display().to_string(), &["-s", serial, "get-state"]).map(|o| o.trim().to_string()).filter(|s| !s.is_empty())
}

/// Os emuladores criados (AVDs).
pub fn android_avds(sdk: &Path) -> Vec<String> {
    let emulator = sdk.join("emulator").join(exe("emulator"));
    output(&emulator.display().to_string(), &["-list-avds"])
        .map(|o| o.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with("INFO")).map(str::to_string).collect())
        .unwrap_or_default()
}

fn android(out: &mut Vec<Check>, language: Language, rust: &SharedRust) {
    let t = Target::Android;
    if language.uses_host() {
        // O app leva os forms; o código roda num servidor (o app casca é em Rust).
        out.push(
            Check::ok(t, "Linguagem", format!("{}: o código roda num servidor (Publicar → Servidor)", language.name()))
                .help("No servidor: aylis-host --bridge 0.0.0.0:9443 --project <projeto> -- <comando do backend>"),
        );
    }
    note_android();
    // O cargo-ndk, o Java e o aparelho conectado ao mesmo tempo que o Rust (cada um roda um
    // processo).
    let (ndk, java, device) = std::thread::scope(|s| {
        let ndk = side(s, || match which("cargo-ndk") {
            Some(_) => Check::ok(t, "cargo-ndk", output("cargo", &["ndk", "--version"]).unwrap_or_default().trim().to_string()),
            None => Check::missing(t, "cargo-ndk", "não instalado").fix(Fix::package_or("cargo-ndk", Fix::new("Instala o cargo-ndk", &["cargo", "install", "cargo-ndk"]))),
        });
        let java = side(s, java_check);
        let device = side(s, || {
            let mut live = Vec::new();
            live_items(&mut live, t, language);
            live
        });
        rust_items(out, t, rust.get());
        rust_targets(out, t, rust.get(), "Alvos do Rust para Android", &ANDROID_ABIS.map(|(_, triple)| triple), "rust-android");
        (joined(ndk), joined(java), joined(device))
    });
    out.extend(ndk);
    out.extend(java);
    out.push(match crate::publish::bundletool() {
        Some(p) => Check::ok(t, "bundletool (AAB para a Play Store)", p.display().to_string()),
        None => match crate::dap::tools_dir() {
            Some(_) => match Fix::download("bundletool") {
                Some(fix) => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)").fix(fix),
                None => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)"),
            },
            None => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)"),
        },
    });
    let Some(sdk) = android_sdk() else {
        out.push(
            Check::missing(t, "Android SDK", "não encontrado (defina ANDROID_HOME)").fix(Fix::package_or(
                "android-sdk",
                Fix::new("Instala o Android Studio (traz o SDK; abra-o uma vez para baixar os componentes)", &["winget", "install", "--id", "Google.AndroidStudio", "-e"])
                    .interactive(),
            )),
        );
        return;
    };
    out.push(Check::ok(t, "Android SDK", sdk.display().to_string()));
    let manager = sdkmanager(&sdk);
    let sdk_fix = |package: &str, what: &str| -> Option<Fix> {
        manager.as_ref().map(|m| Fix::new(format!("Instala {what} pelo sdkmanager"), &[&m.display().to_string(), "--install", package]))
    };
    let mut item = |name: &str, found: Option<String>, package: &str, what: &str| {
        let check = match found {
            Some(v) => Check::ok(t, name, v),
            None => {
                let c = Check::missing(t, name, "não instalado");
                match sdk_fix(package, what) {
                    Some(f) => c.fix(f),
                    None => c.help(format!("Instale \"{package}\" pelo SDK Manager do Android Studio")),
                }
            }
        };
        out.push(check);
    };
    let name_of = |p: PathBuf| p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    item("Plataforma Android", android_platform(&sdk).map(name_of), "platforms;android-34", "a plataforma Android 34");
    item("Build tools", android_build_tools(&sdk).map(name_of), "build-tools;34.0.0", "as build tools 34");
    item("Platform tools (adb)", adb(&sdk).is_file().then(|| "adb".to_string()), "platform-tools", "as platform tools");
    // O NDK pelo pacote (o SDK do sistema fica como está; o pacote aponta o ANDROID_NDK_HOME).
    match (android_ndk(&sdk), package_fix("android-ndk", sdk_fix("ndk;27.2.12479018", "o NDK 27").as_ref())) {
        (None, Some(fix)) => out.push(Check::missing(t, "NDK", "não instalado").fix(fix)),
        (found, _) => item("NDK", found.map(name_of), "ndk;27.2.12479018", "o NDK 27"),
    }
    out.extend(device.into_iter().flatten());
}

/// O Java (JDK 17+), pelo `java -version`.
fn java_check() -> Check {
    let t = Target::Android;
    match java_bin().and_then(|bin| output(&bin.join(exe("java")).display().to_string(), &["-version"])) {
        Some(v) => {
            let version = v.lines().next().unwrap_or_default().to_string();
            let major = version.split('"').nth(1).and_then(|v| v.split('.').next()).and_then(|m| m.parse::<u32>().ok()).unwrap_or(0);
            if major >= 17 {
                Check::ok(t, "Java (JDK 17+)", version)
            } else {
                Check::missing(t, "Java (JDK 17+)", format!("{version} é antigo")).fix(install_jdk())
            }
        }
        None => Check::missing(t, "Java (JDK 17+)", "não encontrado").fix(Fix::package_or("jdk", install_jdk())),
    }
}

/// Do que o Android depende além dos programas que roda: as variáveis e as pastas do SDK
/// (o sdkmanager e o Android Studio instalam nelas) e a pasta de ferramentas (o bundletool).
fn note_android() {
    use crate::env_cache::{note_path, note_var};
    for name in ["ANDROID_HOME", "ANDROID_SDK_ROOT", "ANDROID_NDK_HOME", "JAVA_HOME", "LOCALAPPDATA"] {
        note_var(name);
    }
    note_tools_dir();
    if let Some(sdk) = android_sdk() {
        note_path(&sdk);
        for sub in ["ndk", "build-tools", "platforms", "platform-tools", "cmdline-tools", "cmdline-tools/latest/bin", "emulator"] {
            note_path(&sdk.join(sub));
        }
    }
}

/// Um trabalho noutra thread que anota o que rodou ([`joined`] passa as marcas para a thread
/// que esperou).
fn side<'s, T: Send + 's>(s: &'s std::thread::Scope<'s, '_>, f: impl FnOnce() -> T + Send + 's) -> std::thread::ScopedJoinHandle<'s, (T, Vec<crate::env_cache::Dep>)> {
    s.spawn(move || crate::env_cache::record(f))
}

fn joined<T>(h: std::thread::ScopedJoinHandle<'_, (T, Vec<crate::env_cache::Dep>)>) -> Option<T> {
    let (value, deps) = h.join().ok()?;
    crate::env_cache::note_all(&deps);
    Some(value)
}

/// O aparelho conectado (`adb devices`; sem nenhum, o emulador que dá para abrir).
fn android_device_check(sdk: &Path) -> Check {
    let t = Target::Android;
    let devices = android_devices(sdk);
    if !devices.is_empty() {
        return Check::ok(t, "Aparelho conectado", devices.join(", "));
    }
    let avds = android_avds(sdk);
    let check = Check::warning(t, "Aparelho conectado", "nenhum (só para Executar; publicar o APK não precisa)");
    match avds.first() {
        Some(avd) => check.fix(Fix::new(format!("Abre o emulador {avd}"), &[&sdk.join("emulator").join(exe("emulator")).display().to_string(), "-avd", avd]).interactive()),
        None => check.help("Conecte um celular com a depuração USB ligada, ou crie um emulador no Android Studio (Device Manager)"),
    }
}

fn install_jdk() -> Fix {
    if cfg!(windows) {
        Fix::new("Instala o OpenJDK 17 (winget)", &["winget", "install", "--id", "Microsoft.OpenJDK.17", "-e"]).interactive()
    } else if cfg!(target_os = "macos") {
        Fix::new("Instala o OpenJDK 17 (Homebrew)", &["brew", "install", "openjdk@17"])
    } else {
        Fix::new("Instala o OpenJDK 17 (apt)", &["sudo", "apt", "install", "-y", "openjdk-17-jdk"]).interactive()
    }
}

// ---------------------------------------------------------------------------
// Web

fn web(out: &mut Vec<Check>, language: Language, rust: &SharedRust) {
    let t = Target::Web;
    if language.uses_host() {
        out.push(
            Check::ok(t, "Linguagem", format!("{}: o código roda num servidor (Publicar → Servidor)", language.name()))
                .help("No servidor: aylis-host --bridge 0.0.0.0:9443 --project <projeto> -- <comando do backend>"),
        );
    }
    // O wasm-bindgen ao mesmo tempo que o Rust.
    let found = std::thread::scope(|s| {
        let found = side(s, || output("wasm-bindgen", &["--version"]).and_then(|v| v.split_whitespace().nth(1).map(str::to_string)));
        rust_items(out, t, rust.get());
        rust_targets(out, t, rust.get(), "Alvo do Rust para WebAssembly", &["wasm32-unknown-unknown"], "rust-wasm");
        joined(found).flatten()
    });
    // A versão que o Aylis usa vem do Cargo.lock dele.
    crate::env_cache::note_path(&crate::aylis_root().join("Cargo.lock"));
    let wanted = wasm_bindgen_version();
    // O pacote só serve na mesma versão que o Aylis usa (precisam ser iguais).
    let install = |v: &str| {
        let fix = Fix::new(format!("Instala o wasm-bindgen-cli {v}"), &["cargo", "install", "wasm-bindgen-cli", "--version", v, "--locked"]);
        if crate::toolpacks::Manifest::embedded().package("wasm-bindgen").is_some_and(|p| p.version == v) { Fix::package_or("wasm-bindgen", fix) } else { fix }
    };
    out.push(match (found, wanted) {
        (Some(f), Some(w)) if f == w => Check::ok(t, "wasm-bindgen", f),
        (Some(f), Some(w)) => Check::missing(t, "wasm-bindgen", format!("{f} instalado; o Aylis usa {w} (precisam ser iguais)")).fix(install(&w)),
        (Some(f), None) => Check::ok(t, "wasm-bindgen", f),
        (None, Some(w)) => Check::missing(t, "wasm-bindgen", "não instalado").fix(install(&w)),
        (None, None) => Check::missing(t, "wasm-bindgen", "não instalado").fix(Fix::new("Instala o wasm-bindgen-cli", &["cargo", "install", "wasm-bindgen-cli"])),
    });
}

/// A versão do `wasm-bindgen` que o aylis usa (do `Cargo.lock` dele).
pub fn wasm_bindgen_version() -> Option<String> {
    lock_version(&crate::aylis_root().join("Cargo.lock"), "wasm-bindgen")
}

/// Versão de um pacote num `Cargo.lock`.
pub fn lock_version(lock: &Path, package: &str) -> Option<String> {
    let text = std::fs::read_to_string(lock).ok()?;
    let marker = format!("name = \"{package}\"\nversion = \"");
    let start = text.find(&marker)? + marker.len();
    Some(text[start..].split('"').next()?.to_string())
}

// ---------------------------------------------------------------------------
// iOS

fn ios(out: &mut Vec<Check>, language: Language, rust: &SharedRust) {
    let t = Target::Ios;
    if !cfg!(target_os = "macos") {
        out.push(Check::missing(t, "macOS", "o app de iOS só é gerado num Mac (exigência da Apple: Xcode)").help("Abra o projeto na IDE do Aylis num Mac"));
        return;
    }
    if language.uses_host() {
        out.push(Check::missing(t, "Linguagem", format!("projetos em {} ainda não publicam para o iOS", language.name())).help("Use Rust, ou publique para o navegador (o iPhone abre)."));
        return;
    }
    rust_items(out, t, rust.get());
    rust_targets(out, t, rust.get(), "Alvos do Rust para iOS", &["aarch64-apple-ios", "aarch64-apple-ios-sim"], "rust-ios");
    crate::env_cache::note_path(Path::new("/Applications/Xcode.app"));
    out.push(match output("xcrun", &["--sdk", "iphoneos", "--show-sdk-version"]) {
        Some(v) => Check::ok(t, "Xcode (SDK do iOS)", v.trim()),
        None => Check::missing(t, "Xcode (SDK do iOS)", "não encontrado").help("Instale o Xcode pela App Store e abra-o uma vez"),
    });
    live_items(out, t, language);
}

// ---------------------------------------------------------------------------
// Linux pelo WSL

fn linux_wsl(out: &mut Vec<Check>, language: Language) {
    let t = Target::Linux;
    if !cfg!(windows) {
        out.push(Check::ok(t, "WSL", "não se aplica (fora do Windows, a saída do sistema já é o Linux ou o macOS)"));
        return;
    }
    if language != Language::Rust {
        out.push(Check::missing(t, "Linguagem", format!("projetos em {} ainda não rodam no Linux pelo WSL", language.name())).help("Use Rust para gerar o executável Linux."));
        return;
    }
    // O que dentro do Linux mudou o Windows não vê: vale o prazo do resultado guardado
    // (`env_cache::MAX_AGE_SECS`), o "Verificar de novo" e o "Corrigir".
    crate::env_cache::note_program("wsl.exe");
    crate::env_cache::note_var("AYLIS_WSL_DISTRO");
    let (available, distro) = std::thread::scope(|s| {
        let distro = s.spawn(crate::wsl::distro);
        (crate::wsl::available(), distro.join().ok().flatten())
    });
    if !available {
        out.push(Check::missing(t, "WSL", "não instalado").fix(Fix::new("Instala o WSL com o Ubuntu (pede permissão de administrador e reiniciar)", &["wsl.exe", "--install", "-d", "Ubuntu"]).interactive()));
        return;
    }
    let Some(distro) = distro else {
        out.push(Check::missing(t, "Distribuição Linux", "nenhuma (a docker-desktop não serve)").fix(Fix::new("Instala o Ubuntu no WSL", &["wsl.exe", "--install", "-d", "Ubuntu", "--no-launch"])));
        return;
    };
    out.push(Check::ok(t, "Distribuição Linux", &distro));
    let run = |cmd: String| ["wsl.exe".to_string(), "-d".into(), distro.clone(), "--exec".into(), "bash".into(), "-lc".into(), cmd];
    let fix = |description: &str, cmd: String| Fix { description: description.into(), command: run(cmd).to_vec(), interactive: false, fetch: None, env: Vec::new() };
    // As bibliotecas (o que falta pelo dpkg).
    let list = crate::wsl::PACKAGES.join(" ");
    // As três perguntas ao Linux ao mesmo tempo.
    let (missing, cargo, wslg) = std::thread::scope(|s| {
        let cargo = s.spawn(|| crate::wsl::shell_output(&distro, "[ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"; cargo --version"));
        let wslg = s.spawn(|| crate::wsl::shell_output(&distro, "ls /mnt/wslg/runtime-dir/wayland-0 >/dev/null 2>&1 && echo sim").is_some_and(|o| o.contains("sim")));
        let missing = crate::wsl::shell_output(&distro, &format!("for p in {list}; do dpkg -s $p >/dev/null 2>&1 || echo $p; done")).unwrap_or_default();
        (missing, cargo.join().ok().flatten(), wslg.join().unwrap_or(false))
    });
    let missing: Vec<&str> = missing.split_whitespace().collect();
    out.push(if missing.is_empty() {
        Check::ok(t, "Bibliotecas de compilação", format!("{} pacotes", crate::wsl::PACKAGES.len()))
    } else {
        Check::missing(t, "Bibliotecas de compilação", format!("faltam {}", missing.join(", ")))
            .fix(fix("Instala as bibliotecas pelo apt (como root)", format!("export DEBIAN_FRONTEND=noninteractive; (apt-get update && apt-get install -y {list}) || sudo apt-get install -y {list}")))
    });
    out.push(match cargo {
        Some(v) => Check::ok(t, "Rust no Linux", v.trim()),
        None => Check::missing(t, "Rust no Linux", "não instalado")
            .fix(fix("Instala o Rust no Linux (rustup)", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal".into())),
    });
    out.push(if wslg {
        Check::ok(t, "Janela (WSLg)", "sim")
    } else {
        Check::warning(t, "Janela (WSLg)", "não encontrado: o app compila, mas a janela pode não aparecer").help("Atualize o WSL: wsl --update")
    });
}

// ---------------------------------------------------------------------------

/// Como [`output`], dizendo por que falhou: `Err(None)` se o programa não existe,
/// `Err(Some(mensagem))` se rodou e falhou (o stderr).
pub fn run(program: &str, args: &[&str]) -> Result<String, Option<String>> {
    crate::env_cache::note_program(program);
    let mut command = Command::new(program);
    command.args(args);
    let out = match output_within(quiet(&mut command), PROCESS_LIMIT) {
        Ok(out) => out,
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => return Err(Some(format!("{program} não respondeu em {} s", PROCESS_LIMIT.as_secs()))),
        Err(_) => return Err(None),
    };
    if !out.status.success() {
        let text = if out.stderr.is_empty() { out.stdout } else { out.stderr };
        return Err(Some(String::from_utf8_lossy(&text).trim().to_string()).filter(|t| !t.is_empty()));
    }
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Ok(String::from_utf8_lossy(&text).to_string())
}

/// Saída de um comando que terminou bem (stdout, ou stderr se o stdout veio vazio, como o
/// `java -version`). `None` se não existe ou falhou.
pub fn output(program: &str, args: &[&str]) -> Option<String> {
    crate::env_cache::note_program(program);
    let mut command = Command::new(program);
    command.args(args);
    let out = output_within(quiet(&mut command), PROCESS_LIMIT).ok()?;
    if !out.status.success() {
        return None;
    }
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Some(String::from_utf8_lossy(&text).to_string())
}

/// O prazo de um processo da verificação: um que trava (um `adb` sem resposta, um instalador
/// esperando alguém) não prende a tela nem o F9.
pub const PROCESS_LIMIT: Duration = Duration::from_secs(20);

/// O `Command::output` com prazo: passado `limit`, o processo é encerrado e o erro é
/// `TimedOut`. A saída é lida em duas threads (um processo que enche o stderr não trava
/// esperando quem leia o stdout); se o processo terminou mas um filho dele segura a saída,
/// vale o que já chegou.
pub fn output_within(command: &mut Command, limit: Duration) -> std::io::Result<std::process::Output> {
    use std::io::Read;
    let mut child = command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let buffers = [Arc::new(Mutex::new(Vec::new())), Arc::new(Mutex::new(Vec::new()))];
    let pipes: [Option<Box<dyn Read + Send>>; 2] = [child.stdout.take().map(|p| Box::new(p) as _), child.stderr.take().map(|p| Box::new(p) as _)];
    for (pipe, buffer) in pipes.into_iter().zip(&buffers) {
        let (tx, buffer) = (tx.clone(), buffer.clone());
        std::thread::spawn(move || {
            if let Some(mut pipe) = pipe {
                let mut chunk = [0u8; 8192];
                while let Ok(n) = pipe.read(&mut chunk) {
                    if n == 0 {
                        break;
                    }
                    buffer.lock().unwrap_or_else(|e| e.into_inner()).extend_from_slice(&chunk[..n]);
                }
            }
            let _ = tx.send(());
        });
    }
    drop(tx);
    let deadline = std::time::Instant::now() + limit;
    let mut closed = 0;
    while closed < 2 {
        match rx.recv_timeout(deadline.saturating_duration_since(std::time::Instant::now())) {
            Ok(()) => closed += 1,
            Err(_) => break,
        }
    }
    let status = if closed == 2 {
        child.wait()?
    } else {
        match child.try_wait()? {
            Some(status) => status,
            None => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "o processo não respondeu no prazo"));
            }
        }
    };
    let take = |b: &Arc<Mutex<Vec<u8>>>| std::mem::take(&mut *b.lock().unwrap_or_else(|e| e.into_inner()));
    Ok(std::process::Output { status, stdout: take(&buffers[0]), stderr: take(&buffers[1]) })
}

/// A primeira versão `a.b.c` de um texto.
pub fn parse_version(text: &str) -> Option<(u32, u32, u32)> {
    text.split(|c: char| !(c.is_ascii_digit() || c == '.')).find_map(|word| {
        let mut parts = word.split('.').map(|p| p.parse::<u32>());
        match (parts.next(), parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => Some((a, b, c)),
            (Some(Ok(a)), Some(Ok(b)), None) => Some((a, b, 0)),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_adb_device_list_keeps_model_and_state() {
        let text = "List of devices attached\nJBAXB601U871Z49        device product:WW_X00QD model:ASUS_X00QD device:ASUS_X00QD transport_id:2\nemulator-5554 offline transport_id:3\nR58M123 unauthorized usb:1-1 transport_id:4\n\n";
        let list = parse_devices(text);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].label(), "ASUS_X00QD (JBAXB601U871Z49)");
        assert!(list[0].is_ready() && !list[1].is_ready());
        assert_eq!(list[1].label(), "emulator-5554 — offline");
        assert_eq!(list[2].label(), "R58M123 — sem autorização");
        assert!(parse_devices("* daemon started successfully\nList of devices attached\n").is_empty());
    }
}
