//! A verificação de ambiente guardada em disco (`%USERPROFILE%\.aylis\environment.json`; fora
//! do Windows, ao lado da pasta de compilação): a tela abre na hora com o último resultado e
//! confere de novo em segundo plano.
//!
//! Cada conferência anota de que dependeu (o executável achado no PATH, a pasta do SDK, uma
//! variável de ambiente), com a marca de cada arquivo ([`FileStamp`]). Sem mudança nas marcas,
//! o resultado guardado vale e nenhum processo roda de novo: acordar o WSL custava 4,5 s a
//! cada abertura. Fora do `AppData` de propósito (o MSIX do Claude Desktop dá aos processos
//! que abre uma cópia privada dele; veja `default_target_dir`).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use aylis_project::{FileStamp, Language};
use serde_json::{Value, json};

use crate::Target;
use crate::env::{Check, Fix, Status};

/// Quanto um resultado guardado vale, mesmo sem mudança nas marcas: o que nenhum arquivo
/// do Windows mostra (um pacote do `apt` dentro do WSL, um `pip install` fora da IDE) aparece
/// no máximo um dia depois.
pub const MAX_AGE_SECS: u64 = 24 * 3600;

/// Do que uma conferência dependeu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dep {
    /// Um arquivo ou uma pasta, com a marca (`None`: não existia).
    Path(PathBuf, Option<FileStamp>),
    /// Um programa procurado no PATH pelo nome: onde estava e a marca (`None`: não achado).
    Program(String, Option<(PathBuf, Option<FileStamp>)>),
    /// Uma variável de ambiente (`None`: não definida). Com `@` na frente, um valor que a
    /// conferência recebeu ([`Inputs`]), como o caminho do `aylis-host`.
    Value(String, Option<String>),
}

/// O que a conferência recebeu de quem pediu (entra nas marcas como [`Dep::Value`] com `@`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inputs {
    pub aylis_host: Option<String>,
}

impl Inputs {
    fn get(&self, name: &str) -> Option<String> {
        match name {
            "@aylis-host" => self.aylis_host.clone(),
            _ => None,
        }
    }
}

impl Dep {
    /// Continua como estava quando foi anotada.
    pub fn unchanged(&self, inputs: &Inputs) -> bool {
        match self {
            Dep::Path(path, stamp) => FileStamp::read(path) == *stamp,
            Dep::Program(name, found) => resolve(name) == *found,
            Dep::Value(name, value) if name.starts_with('@') => inputs.get(name) == *value,
            Dep::Value(name, value) => std::env::var(name).ok() == *value,
        }
    }

    fn to_json(&self) -> Value {
        let stamp = |s: &Option<FileStamp>| s.map_or(Value::Null, |s| json!([s.nanos, s.len]));
        match self {
            Dep::Path(p, s) => json!({ "path": p.display().to_string(), "stamp": stamp(s) }),
            Dep::Program(name, None) => json!({ "program": name, "at": null }),
            Dep::Program(name, Some((p, s))) => json!({ "program": name, "at": p.display().to_string(), "stamp": stamp(s) }),
            Dep::Value(name, v) => json!({ "value": name, "is": v }),
        }
    }

    fn from_json(v: &Value) -> Option<Dep> {
        let stamp = |v: &Value| v.as_array().and_then(|a| Some(FileStamp { nanos: a.first()?.as_u64()?, len: a.get(1)?.as_u64()? }));
        if let Some(p) = v["path"].as_str() {
            return Some(Dep::Path(PathBuf::from(p), stamp(&v["stamp"])));
        }
        if let Some(name) = v["program"].as_str() {
            let found = v["at"].as_str().map(|p| (PathBuf::from(p), stamp(&v["stamp"])));
            return Some(Dep::Program(name.to_string(), found));
        }
        let name = v["value"].as_str()?;
        Some(Dep::Value(name.to_string(), v["is"].as_str().map(str::to_string)))
    }
}

/// Onde o programa `name` está no PATH agora, com a marca (sem anotar).
fn resolve(name: &str) -> Option<(PathBuf, Option<FileStamp>)> {
    crate::find_in_path(name).map(|p| {
        let stamp = FileStamp::read(&p);
        (p, stamp)
    })
}

// ---------------------------------------------------------------------------
// O que a conferência em andamento nesta thread anota

thread_local! {
    static DEPS: RefCell<Option<Vec<Dep>>> = const { RefCell::new(None) };
}

/// Roda `f` anotando do que ele dependeu (o que já se anotava fora continua depois).
pub fn record<T>(f: impl FnOnce() -> T) -> (T, Vec<Dep>) {
    let outer = DEPS.with(|d| d.borrow_mut().replace(Vec::new()));
    let value = f();
    let deps = DEPS.with(|d| std::mem::replace(&mut *d.borrow_mut(), outer)).unwrap_or_default();
    (value, deps)
}

fn recording() -> bool {
    DEPS.with(|d| d.borrow().is_some())
}

/// Anota uma dependência (fora de [`record`], não faz nada).
pub fn note(dep: Dep) {
    DEPS.with(|d| {
        if let Some(list) = d.borrow_mut().as_mut()
            && !list.contains(&dep)
        {
            list.push(dep);
        }
    });
}

/// Anota várias (as da conferência do Rust, que vale para todas as plataformas).
pub fn note_all(deps: &[Dep]) {
    for d in deps {
        note(d.clone());
    }
}

/// Anota um arquivo ou uma pasta, com a marca de agora.
pub fn note_path(path: &Path) {
    if recording() {
        note(Dep::Path(path.to_path_buf(), FileStamp::read(path)));
    }
}

/// Anota uma variável de ambiente.
pub fn note_var(name: &str) {
    if recording() {
        note(Dep::Value(name.to_string(), std::env::var(name).ok()));
    }
}

/// Anota um valor recebido (`@aylis-host`).
pub fn note_input(name: &str, value: Option<String>) {
    note(Dep::Value(format!("@{name}"), value));
}

/// Anota o programa que um processo rodou: um caminho vira a marca do arquivo; um nome, o
/// que o PATH acha (e, se não acha, a falta dele).
pub fn note_program(program: &str) {
    if !recording() {
        return;
    }
    let path = Path::new(program);
    if path.is_absolute() || path.components().count() > 1 {
        note_path(path);
    } else {
        note(Dep::Program(program.to_string(), resolve(program)));
    }
}

/// Anota o resultado de uma procura no PATH já feita (o [`crate::which`]).
pub fn note_found(name: &str, found: Option<&Path>) {
    if recording() {
        note(Dep::Program(name.to_string(), found.map(|p| (p.to_path_buf(), FileStamp::read(p)))));
    }
}

// ---------------------------------------------------------------------------
// O arquivo

/// Um resultado guardado: o de uma plataforma (as conferências) ou o do Rust (`extra`).
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// Quando foi conferido (segundos desde 1970).
    pub saved: u64,
    pub deps: Vec<Dep>,
    pub checks: Vec<Check>,
    pub extra: Value,
}

impl Entry {
    /// Ainda vale: nenhuma marca mudou e não passou de [`MAX_AGE_SECS`].
    pub fn valid(&self, inputs: &Inputs, now: u64) -> bool {
        now.saturating_sub(self.saved) < MAX_AGE_SECS && self.deps.iter().all(|d| d.unchanged(inputs))
    }

    fn to_json(&self) -> Value {
        json!({
            "saved": self.saved,
            "deps": self.deps.iter().map(Dep::to_json).collect::<Vec<_>>(),
            "checks": self.checks.iter().map(check_to_json).collect::<Vec<_>>(),
            "extra": self.extra,
        })
    }

    fn from_json(v: &Value) -> Option<Entry> {
        Some(Entry {
            saved: v["saved"].as_u64()?,
            deps: v["deps"].as_array()?.iter().map(Dep::from_json).collect::<Option<_>>()?,
            checks: v["checks"].as_array()?.iter().map(check_from_json).collect::<Option<_>>()?,
            extra: v["extra"].clone(),
        })
    }
}

/// Os resultados guardados, pela chave ([`key`]; `rust` é a conferência do Rust).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cache {
    pub entries: BTreeMap<String, Entry>,
}

impl Cache {
    pub fn to_json(&self) -> Value {
        let entries: serde_json::Map<String, Value> = self.entries.iter().map(|(k, e)| (k.clone(), e.to_json())).collect();
        json!({ "aylis": 1, "entries": entries })
    }

    /// Lê o texto do arquivo; o que não se entende fica de fora (é só um atalho).
    pub fn from_text(text: &str) -> Cache {
        let v: Value = serde_json::from_str(text).unwrap_or(Value::Null);
        let entries = v["entries"].as_object().into_iter().flatten().filter_map(|(k, e)| Some((k.clone(), Entry::from_json(e)?))).collect();
        Cache { entries }
    }

    pub fn read(path: &Path) -> Cache {
        std::fs::read_to_string(path).map(|t| Cache::from_text(&t)).unwrap_or_default()
    }

    /// Grava por cima o que mudou, relendo o arquivo antes (a IDE e o `aylis-tool` gravam o
    /// mesmo), por um temporário renomeado (quem lê ao mesmo tempo não pega pela metade).
    pub fn merge_into(path: &Path, updates: &BTreeMap<String, Entry>) -> std::io::Result<Cache> {
        let mut cache = Cache::read(path);
        for (k, e) in updates {
            cache.entries.insert(k.clone(), e.clone());
        }
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let temp = path.with_extension(format!("json.{}.tmp", std::process::id()));
        std::fs::write(&temp, serde_json::to_string_pretty(&cache.to_json()).unwrap_or_default())?;
        std::fs::rename(&temp, path).inspect_err(|_| {
            let _ = std::fs::remove_file(&temp);
        })?;
        Ok(cache)
    }
}

/// O arquivo: `%USERPROFILE%\.aylis\environment.json` no Windows e
/// `~/.cache/aylis/environment.json` nos outros (o mesmo lugar-base da pasta de compilação,
/// [`crate::default_target_dir`]).
pub fn file() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join(".aylis"))
    } else {
        std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache"))).map(|b| b.join("aylis"))
    };
    base.map(|b| b.join("environment.json"))
}

/// O arquivo lido, guardado na memória pela marca dele (a IDE abre o diálogo várias vezes).
static MEMORY: Mutex<Option<(PathBuf, Option<FileStamp>, Cache)>> = Mutex::new(None);

/// O que está guardado em `path` (sem conferir as marcas).
pub fn load(path: &Path) -> Cache {
    let stamp = FileStamp::read(path);
    let mut memory = MEMORY.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((p, s, cache)) = memory.as_ref()
        && p == path
        && *s == stamp
    {
        return cache.clone();
    }
    let cache = Cache::read(path);
    *memory = Some((path.to_path_buf(), stamp, cache.clone()));
    cache
}

/// Guarda os resultados novos em `path` (um erro de disco só perde o atalho).
pub fn store(path: &Path, updates: &BTreeMap<String, Entry>) {
    if updates.is_empty() {
        return;
    }
    if let Ok(cache) = Cache::merge_into(path, updates) {
        *MEMORY.lock().unwrap_or_else(|e| e.into_inner()) = Some((path.to_path_buf(), FileStamp::read(path), cache));
    }
}

/// A chave de uma plataforma numa linguagem (`android|rust`).
pub fn key(target: Target, language: Language) -> String {
    format!("{}|{}", target_key(target), language.folder())
}

/// A chave da conferência do Rust (a mesma para todas as plataformas).
pub const RUST_KEY: &str = "rust";

fn target_key(target: Target) -> &'static str {
    match target {
        Target::Desktop => "desktop",
        Target::Android => "android",
        Target::Web => "web",
        Target::Ios => "ios",
        Target::Linux => "wsl",
    }
}

fn target_from_key(key: &str) -> Option<Target> {
    Target::ALL.into_iter().find(|t| target_key(*t) == key)
}

/// Agora, em segundos desde 1970 (no navegador, 0: o `SystemTime` do wasm não roda, e lá
/// não há verificação de ambiente).
pub fn now() -> u64 {
    if cfg!(target_arch = "wasm32") {
        return 0;
    }
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn check_to_json(c: &Check) -> Value {
    json!({
        "target": target_key(c.target),
        "name": c.name,
        "status": match c.status { Status::Ok => "ok", Status::Warning => "warning", Status::Missing => "missing" },
        "detail": c.detail,
        "fix": c.fix.as_ref().map_or(Value::Null, |f| json!({
            "description": f.description,
            "command": f.command,
            "interactive": f.interactive,
            "fetch": f.fetch,
            "env": f.env.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
        })),
        "help": c.help,
    })
}

fn check_from_json(v: &Value) -> Option<Check> {
    let text = |v: &Value| v.as_str().map(str::to_string);
    let fix = match &v["fix"] {
        Value::Null => None,
        f => Some(Fix {
            description: text(&f["description"])?,
            command: f["command"].as_array()?.iter().map(text).collect::<Option<_>>()?,
            interactive: f["interactive"].as_bool().unwrap_or(false),
            fetch: text(&f["fetch"]),
            env: f["env"].as_array().into_iter().flatten().filter_map(|p| Some((text(&p[0])?, text(&p[1])?))).collect(),
        }),
    };
    Some(Check {
        target: target_from_key(v["target"].as_str()?)?,
        name: text(&v["name"])?,
        status: match v["status"].as_str()? {
            "ok" => Status::Ok,
            "warning" => Status::Warning,
            _ => Status::Missing,
        },
        detail: text(&v["detail"]).unwrap_or_default(),
        fix,
        help: text(&v["help"]),
    })
}

/// O resultado guardado de cada plataforma, como está (para a tela abrir na hora), com a hora
/// em que foi conferido.
pub fn cached(language: Language) -> Vec<(Target, Vec<Check>, u64)> {
    let Some(path) = file() else { return Vec::new() };
    let cache = load(&path);
    Target::ALL.into_iter().filter_map(|t| cache.entries.get(&key(t, language)).map(|e| (t, e.checks.clone(), e.saved))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Check {
        let mut fix = Fix { description: "Adiciona os alvos".into(), command: vec!["rustup".into(), "target".into(), "add".into()], interactive: false, fetch: None, env: Vec::new() };
        fix.env.push(("GOBIN".into(), "C:\\x".into()));
        Check { target: Target::Android, name: "Alvos do Rust para Android".into(), status: Status::Missing, detail: "faltam x".into(), fix: Some(fix), help: None }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("aylis_envcache_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_cache_survives_the_file() {
        let dir = temp_dir("roundtrip");
        let file = dir.join("environment.json");
        let entry = Entry {
            saved: 123,
            deps: vec![
                Dep::Path(dir.clone(), FileStamp::read(&dir)),
                Dep::Program("cargo".into(), Some((PathBuf::from("C:\\cargo.exe"), Some(FileStamp { nanos: 5, len: 6 })))),
                Dep::Program("nada".into(), None),
                Dep::Value("ANDROID_HOME".into(), None),
                Dep::Value("@aylis-host".into(), Some("h".into())),
            ],
            checks: vec![sample(), Check { help: Some("ajuda".into()), fix: None, status: Status::Warning, ..sample() }],
            extra: json!({ "x": 1 }),
        };
        let mut updates = BTreeMap::new();
        updates.insert(key(Target::Android, Language::Rust), entry.clone());
        Cache::merge_into(&file, &updates).unwrap();
        // Uma segunda gravação (outro processo) mantém a primeira.
        let mut other = BTreeMap::new();
        other.insert(RUST_KEY.to_string(), Entry { checks: Vec::new(), ..entry.clone() });
        Cache::merge_into(&file, &other).unwrap();
        let back = Cache::read(&file);
        assert_eq!(back.entries.len(), 2);
        assert_eq!(back.entries[&key(Target::Android, Language::Rust)], entry);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_changed_stamp_invalidates_the_entry() {
        let dir = temp_dir("stamps");
        let tool = dir.join("ferramenta.txt");
        std::fs::write(&tool, "1").unwrap();
        let ((), deps) = record(|| {
            note_path(&tool);
            note_path(&dir.join("ainda-nao-existe"));
            note_input("aylis-host", Some("a".into()));
        });
        assert_eq!(deps.len(), 3);
        let inputs = Inputs { aylis_host: Some("a".into()) };
        let entry = Entry { saved: now(), deps, checks: vec![sample()], extra: Value::Null };
        assert!(entry.valid(&inputs, now()));
        // Outro aylis-host: confere de novo.
        assert!(!entry.valid(&Inputs { aylis_host: None }, now()));
        // Velho demais.
        assert!(!entry.valid(&inputs, now() + MAX_AGE_SECS));
        // O que não existia apareceu (um SDK instalado).
        std::fs::write(dir.join("ainda-nao-existe"), "").unwrap();
        assert!(!entry.valid(&inputs, now()));
        std::fs::remove_file(dir.join("ainda-nao-existe")).unwrap();
        assert!(entry.valid(&inputs, now()));
        // O arquivo mudou (outro tamanho).
        std::fs::write(&tool, "22").unwrap();
        assert!(!entry.valid(&inputs, now()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn notes_outside_a_recording_are_ignored_and_recordings_nest() {
        note_path(Path::new("."));
        let ((), outer) = record(|| {
            note_var("AYLIS_NAO_DEFINIDA");
            let ((), inner) = record(|| note_program("programa-que-nao-existe-aylis"));
            assert_eq!(inner, vec![Dep::Program("programa-que-nao-existe-aylis".into(), None)]);
            note_all(&inner);
            note_all(&inner);
        });
        assert_eq!(outer.len(), 2, "{outer:?}");
        assert!(outer.iter().all(|d| d.unchanged(&Inputs::default())));
    }
}
