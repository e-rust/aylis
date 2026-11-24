//! `aylis-tool test-all`: a bateria do workspace sem repetir o que não mudou (pedido do autor em
//! 22/11: "um app que consiga executar todos os testes de uma vez, que produza um resultado que
//! apresentaria diferença caso algo não esteja correto, sem precisar passar por cada teste toda
//! vez").
//!
//! - O cargo compila todos os executáveis de teste (`cargo test --no-run`) e diz onde está cada
//!   um. Ele só refaz o executável de um alvo quando o código dele, ou de algo de que ele
//!   depende, muda.
//! - A chave de cada alvo é a marca do executável (o tamanho e a data) somada às marcas do que
//!   ele lê em execução, declarado em `resources/test-inputs.json` (`"pure"`: nada; uma lista
//!   de pastas e arquivos do repositório; `{"always": "motivo"}`: o que o hash não pega, como as
//!   ferramentas das linguagens, o hardware ou compilar projetos). Alvo sem declaração roda
//!   sempre, e o resumo diz quais são.
//! - Chave igual à de uma rodada que passou: o resultado vem do cache
//!   (`<target>/.aylis-tests.json`) e o alvo não roda. `--all` roda tudo.
//! - O que roda vai em paralelo por pacote (os alvos de um pacote, um depois do outro: eles
//!   dividem pastas e portas); os `always`, no fim, um por vez. Uma falha roda de novo uma vez:
//!   se passar, sai como instável.
//! - O relatório (`medicoes/testes.json`) só tem o que não varia de uma rodada boa para outra
//!   (o resultado, as contagens e os testes que falharam, em ordem): duas rodadas boas dão o
//!   mesmo arquivo, e o que quebrou aparece no diff. Os tempos e o que veio do cache ficam no
//!   resumo do terminal.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// O alvo dos testes da documentação de todo o workspace.
const DOC_ID: &str = "workspace/doc";

/// O pacote que não entra na bateria (o CLAUDE.md: ele precisa de janela e de GPU).
const EXCLUDED: &str = "aylis-platform-winit";

/// As opções da linha de comando.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Roda tudo, sem o cache (antes de uma release).
    pub all: bool,
    /// Quantos pacotes ao mesmo tempo (0: a metade dos processadores, no máximo 6).
    pub jobs: usize,
    /// Só estes pacotes (vazio: o workspace).
    pub packages: Vec<String>,
    /// Onde gravar o relatório (vazio: `medicoes/testes.json`).
    pub report: Option<PathBuf>,
}

/// O que um alvo lê em execução, fora do executável.
#[derive(Clone, Debug, PartialEq)]
pub enum Inputs {
    /// Nada: só o executável conta.
    Pure,
    /// Pastas e arquivos do repositório (relativos à raiz).
    Paths(Vec<String>),
    /// Algo que as marcas de arquivo não pegam: roda sempre.
    Always(String),
}

/// Um executável de teste.
#[derive(Clone, Debug)]
pub struct TestExe {
    /// `<pacote>/<alvo>` (`lib` para os testes de unidade da biblioteca, `bin:<nome>` para os de
    /// um binário, o nome do arquivo para os de `tests/`).
    pub id: String,
    pub package: String,
    pub exe: PathBuf,
    pub manifest_dir: PathBuf,
    /// Os testes da documentação (`cargo test --doc` do workspace: de um pacote só, as features
    /// não se somam como na bateria, e o `aylis-platform-winit` nem compila). A chave é a das
    /// bibliotecas: muda quando alguma muda.
    pub doc: bool,
}

/// O resultado de um alvo.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub passed: u32,
    pub failed: u32,
    pub ignored: u32,
    /// Os testes que falharam (em ordem).
    pub failures: Vec<String>,
    /// Falhou e passou na segunda vez.
    pub flaky: bool,
    /// O processo terminou com erro sem a linha de resultado (travou, caiu).
    pub crashed: bool,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        self.failed == 0 && !self.crashed
    }

    fn status(&self) -> &'static str {
        if !self.ok() {
            "falhou"
        } else if self.flaky {
            "instável"
        } else {
            "ok"
        }
    }

    fn to_json(&self) -> Value {
        let mut v = json!({ "status": self.status(), "passed": self.passed, "failed": self.failed, "ignored": self.ignored });
        if !self.failures.is_empty() {
            v["failures"] = json!(self.failures);
        }
        if self.crashed {
            v["crashed"] = json!(true);
        }
        v
    }

    fn from_json(v: &Value) -> Option<Self> {
        let n = |k: &str| v[k].as_u64().map(|x| x as u32);
        Some(Self {
            passed: n("passed")?,
            failed: n("failed")?,
            ignored: n("ignored")?,
            failures: v["failures"].as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            flaky: v["status"] == "instável",
            crashed: v["crashed"] == true,
        })
    }
}

/// Lê a saída do libtest: as linhas `test result:` (somadas) e as `test <nome> ... FAILED`.
pub fn parse_output(text: &str) -> Option<Outcome> {
    let mut out = Outcome { passed: 0, failed: 0, ignored: 0, failures: Vec::new(), flaky: false, crashed: false };
    let mut seen = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("test result: ") {
            seen = true;
            let count = |word: &str| {
                rest.split(';').find_map(|part| {
                    let mut it = part.split_whitespace().rev();
                    (it.next() == Some(word)).then(|| it.next()?.parse::<u32>().ok()).flatten()
                })
            };
            out.passed += count("passed").unwrap_or(0);
            out.failed += count("failed").unwrap_or(0);
            out.ignored += count("ignored").unwrap_or(0);
        } else if let Some(name) = line.strip_prefix("test ").and_then(|l| l.strip_suffix(" ... FAILED")) {
            out.failures.push(name.to_string());
        }
    }
    out.failures.sort();
    out.failures.dedup();
    seen.then_some(out)
}

/// A declaração de `resources/test-inputs.json` (`{"<pacote>/<alvo>": "pure" | [...] |
/// {"always": "motivo"}}`).
pub fn parse_inputs(text: &str) -> Result<BTreeMap<String, Inputs>, String> {
    let v: Value = serde_json::from_str(text).map_err(|e| format!("test-inputs.json: {e}"))?;
    let map = v.as_object().ok_or("test-inputs.json: esperava um objeto")?;
    let mut out = BTreeMap::new();
    for (k, v) in map {
        if k.starts_with('_') {
            continue; // comentários
        }
        let inputs = match v {
            // Todos os testes do alvo com `#[ignore]`: roda rápido e não lê nada.
            Value::String(s) if s == "pure" || s == "ignored" => Inputs::Pure,
            Value::Array(a) => Inputs::Paths(a.iter().filter_map(|p| p.as_str().map(str::to_string)).collect()),
            Value::Object(o) => Inputs::Always(o.get("always").and_then(Value::as_str).unwrap_or("").to_string()),
            other => return Err(format!("test-inputs.json: {k}: valor inesperado {other}")),
        };
        out.insert(k.clone(), inputs);
    }
    Ok(out)
}

/// A marca de um arquivo (o tamanho e a data), para a chave.
fn stamp(path: &Path) -> String {
    match std::fs::metadata(path) {
        Ok(m) => {
            let at = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos());
            format!("{}:{at}", m.len())
        }
        Err(_) => "-".into(),
    }
}

/// As marcas de tudo o que fica em `path` (um arquivo ou uma pasta, recursiva), em ordem.
fn stamps_under(root: &Path, rel: &str, out: &mut Vec<String>) {
    let path = root.join(rel);
    let Ok(meta) = std::fs::metadata(&path) else {
        out.push(format!("{rel} -"));
        return;
    };
    if meta.is_file() {
        out.push(format!("{rel} {}", stamp(&path)));
        return;
    }
    let mut stack = vec![path];
    let mut found = Vec::new();
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            let name = e.file_name();
            // O que uma compilação ou um teste grava não é entrada.
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            match e.file_type() {
                Ok(t) if t.is_dir() => stack.push(p),
                Ok(_) => found.push(p),
                Err(_) => {}
            }
        }
    }
    found.sort();
    for p in found {
        let shown = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
        out.push(format!("{shown} {}", stamp(&p)));
    }
}

/// A chave de um alvo: o executável e o que ele lê. `None` para o que roda sempre.
pub fn key(root: &Path, exe: &TestExe, inputs: Option<&Inputs>) -> Option<String> {
    let extra = match inputs {
        None | Some(Inputs::Always(_)) => return None,
        Some(Inputs::Pure) => Vec::new(),
        Some(Inputs::Paths(paths)) => {
            let mut out = Vec::new();
            for p in paths {
                stamps_under(root, p, &mut out);
            }
            out
        }
    };
    let mut h = Sha256::new();
    h.update(exe.exe.to_string_lossy().as_bytes());
    h.update(stamp(&exe.exe).as_bytes());
    for line in extra {
        h.update(b"\n");
        h.update(line.as_bytes());
    }
    Some(format!("{:x}", h.finalize()))
}

/// O nome do alvo como a declaração usa.
fn target_id(kind: &[Value], name: &str) -> String {
    let kind: Vec<&str> = kind.iter().filter_map(Value::as_str).collect();
    if kind.iter().any(|k| matches!(*k, "lib" | "rlib" | "proc-macro" | "cdylib" | "dylib" | "staticlib")) {
        "lib".into()
    } else if kind.contains(&"bin") {
        format!("bin:{name}")
    } else if kind.contains(&"test") {
        name.to_string()
    } else {
        format!("{}:{name}", kind.first().copied().unwrap_or("?"))
    }
}

/// Compila os testes e devolve os executáveis.
fn build(root: &Path, opts: &Options) -> Result<Vec<TestExe>, String> {
    // Os nomes dos pacotes pela pasta do manifesto.
    let meta = crate::process::quiet(Command::new("cargo").args(["metadata", "--no-deps", "--format-version", "1"]).current_dir(root))
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    let meta: Value = serde_json::from_slice(&meta.stdout).map_err(|e| format!("cargo metadata: {e}"))?;
    let mut names: BTreeMap<PathBuf, String> = BTreeMap::new();
    for p in meta["packages"].as_array().into_iter().flatten() {
        if let (Some(m), Some(n)) = (p["manifest_path"].as_str(), p["name"].as_str()) {
            names.insert(PathBuf::from(m), n.to_string());
        }
    }
    let mut args: Vec<String> = vec!["test".into(), "--no-run".into(), "--message-format=json-render-diagnostics".into()];
    if opts.packages.is_empty() {
        args.extend(["--workspace".into(), "--exclude".into(), EXCLUDED.into()]);
    } else {
        for p in &opts.packages {
            args.extend(["-p".into(), p.clone()]);
        }
    }
    println!("▶ compilando os testes (cargo test --no-run)…");
    let started = Instant::now();
    let out = crate::process::quiet(Command::new("cargo").args(&args).current_dir(root).stderr(std::process::Stdio::inherit()))
        .output()
        .map_err(|e| format!("cargo test --no-run: {e}"))?;
    if !out.status.success() {
        return Err("a compilação dos testes falhou (veja acima)".into());
    }
    println!("  compilado em {:.0} s", started.elapsed().as_secs_f64());
    let mut exes = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
        if v["reason"] != "compiler-artifact" || v["profile"]["test"] != true {
            continue;
        }
        let (Some(exe), Some(manifest)) = (v["executable"].as_str(), v["manifest_path"].as_str()) else { continue };
        let manifest = PathBuf::from(manifest);
        let package = names.get(&manifest).cloned().unwrap_or_else(|| manifest.parent().and_then(|d| d.file_name()).map_or_else(String::new, |n| n.to_string_lossy().to_string()));
        let target = target_id(v["target"]["kind"].as_array().map_or(&[][..], Vec::as_slice), v["target"]["name"].as_str().unwrap_or(""));
        let manifest_dir = manifest.parent().map(Path::to_path_buf).unwrap_or_default();
        exes.push(TestExe { id: format!("{package}/{target}"), package, exe: PathBuf::from(exe), manifest_dir, doc: false });
    }
    exes.sort_by(|a, b| a.id.cmp(&b.id));
    exes.dedup_by(|a, b| a.id == b.id);
    if opts.packages.is_empty() {
        exes.push(TestExe { id: DOC_ID.into(), package: "workspace".into(), exe: PathBuf::new(), manifest_dir: root.to_path_buf(), doc: true });
    }
    Ok(exes)
}

/// Roda um executável de teste como o `cargo test` roda (na pasta do pacote, com as variáveis
/// do cargo).
fn run_exe(exe: &TestExe) -> (Outcome, Duration) {
    let started = Instant::now();
    let mut cmd = if exe.doc {
        let mut c = Command::new("cargo");
        c.args(["test", "--doc", "--workspace", "--exclude", EXCLUDED]).current_dir(&exe.manifest_dir);
        c
    } else {
        let mut c = Command::new(&exe.exe);
        c.current_dir(&exe.manifest_dir).env("CARGO_MANIFEST_DIR", &exe.manifest_dir).env("CARGO_PKG_NAME", &exe.package);
        c
    };
    let out = crate::process::quiet(&mut cmd).output();
    let elapsed = started.elapsed();
    let Ok(out) = out else {
        return (Outcome { passed: 0, failed: 0, ignored: 0, failures: vec!["(não iniciou)".into()], flaky: false, crashed: true }, elapsed);
    };
    let text = format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    // Sem a linha de resultado: um alvo sem o harness do libtest (`harness = false`) diz pelo
    // código de saída; com erro, caiu.
    let mut outcome = parse_output(&text).unwrap_or(Outcome { passed: 0, failed: 0, ignored: 0, failures: Vec::new(), flaky: false, crashed: !out.status.success() });
    if !out.status.success() && outcome.failed == 0 {
        outcome.crashed = true;
    }
    (outcome, elapsed)
}

/// Roda e, se falhar, roda de novo: passou na segunda, é instável.
fn run_with_retry(exe: &TestExe) -> (Outcome, Duration) {
    let (first, t1) = run_exe(exe);
    if first.ok() {
        return (first, t1);
    }
    let (mut second, t2) = run_exe(exe);
    if second.ok() {
        second.flaky = true;
        second.failures = first.failures;
    }
    (second, t1 + t2)
}

/// O cache: a chave e o resultado de cada alvo na última rodada.
fn load_cache(path: &Path) -> BTreeMap<String, (String, Outcome)> {
    let Ok(text) = std::fs::read_to_string(path) else { return BTreeMap::new() };
    let Ok(v) = serde_json::from_str::<Value>(&text) else { return BTreeMap::new() };
    v.as_object()
        .map(|o| o.iter().filter_map(|(k, e)| Some((k.clone(), (e["key"].as_str()?.to_string(), Outcome::from_json(&e["result"])?)))).collect())
        .unwrap_or_default()
}

fn save_cache(path: &Path, cache: &BTreeMap<String, (String, Outcome)>) -> Result<(), String> {
    let v: serde_json::Map<String, Value> = cache.iter().map(|(k, (key, o))| (k.clone(), json!({ "key": key, "result": o.to_json() }))).collect();
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(&Value::Object(v)).unwrap_or_default()).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("{}: {e}", path.display()))
}

/// O relatório: só o que não muda de uma rodada boa para outra.
pub fn report(results: &BTreeMap<String, Outcome>) -> String {
    let targets: serde_json::Map<String, Value> = results.iter().map(|(k, o)| (k.clone(), o.to_json())).collect();
    let failed = results.values().filter(|o| !o.ok()).count();
    let v = json!({
        "_": "aylis-tool test-all: o resultado de cada alvo de teste do workspace (sem os tempos: duas rodadas boas dão o mesmo arquivo).",
        "summary": { "targets": results.len(), "failed": failed, "tests": results.values().map(|o| o.passed + o.failed).sum::<u32>() },
        "targets": targets,
    });
    let mut text = serde_json::to_string_pretty(&v).unwrap_or_default();
    text.push('\n');
    text
}

/// Roda a bateria. Devolve `Err` com o resumo das falhas, se houver.
pub fn run(root: &Path, opts: &Options) -> Result<(), String> {
    let declared = match std::fs::read_to_string(root.join("resources/test-inputs.json")) {
        Ok(text) => parse_inputs(&text)?,
        Err(_) => BTreeMap::new(),
    };
    let exes = build(root, opts)?;
    let cache_path = crate::shared_target_dir().join(".aylis-tests.json");
    let mut cache = load_cache(&cache_path);
    let mut results: BTreeMap<String, Outcome> = BTreeMap::new();
    let mut parallel: BTreeMap<String, Vec<(TestExe, Option<String>)>> = BTreeMap::new();
    let mut serial: Vec<(TestExe, Option<String>)> = Vec::new();
    let mut undeclared = Vec::new();
    let mut from_cache = 0;
    // A chave dos testes da documentação: as marcas de todas as bibliotecas.
    let doc_key = {
        let mut h = Sha256::new();
        for e in exes.iter().filter(|e| e.id.ends_with("/lib")) {
            h.update(e.exe.to_string_lossy().as_bytes());
            h.update(stamp(&e.exe).as_bytes());
        }
        format!("{:x}", h.finalize())
    };
    for exe in exes {
        let inputs = declared.get(&exe.id);
        if inputs.is_none() && !exe.doc {
            undeclared.push(exe.id.clone());
        }
        let k = if exe.doc { Some(doc_key.clone()) } else { key(root, &exe, inputs) };
        if let (false, Some(k), Some((old, outcome))) = (opts.all, &k, cache.get(&exe.id)) {
            if k == old && outcome.ok() {
                results.insert(exe.id.clone(), outcome.clone());
                from_cache += 1;
                continue;
            }
        }
        // Os que rodam o cargo (a documentação) e os `always`: no fim, um por vez.
        if exe.doc || matches!(inputs, None | Some(Inputs::Always(_))) {
            serial.push((exe, k));
        } else {
            parallel.entry(exe.package.clone()).or_default().push((exe, k));
        }
    }
    let to_run = parallel.values().map(Vec::len).sum::<usize>() + serial.len();
    println!("▶ {} alvos: {from_cache} do cache, {to_run} para rodar", from_cache + to_run);
    let started = Instant::now();
    let jobs = if opts.jobs > 0 { opts.jobs } else { std::thread::available_parallelism().map_or(4, |n| (n.get() / 2).clamp(1, 6)) };
    let done: Arc<Mutex<Vec<(TestExe, Option<String>, Outcome, Duration)>>> = Arc::new(Mutex::new(Vec::new()));
    let queue: Arc<Mutex<Vec<Vec<(TestExe, Option<String>)>>>> = Arc::new(Mutex::new(parallel.into_values().collect()));
    let report_one = |exe: &TestExe, o: &Outcome, t: Duration| {
        let mark = if !o.ok() { "FALHOU" } else if o.flaky { "instável" } else { "ok" };
        println!("  {mark:>8} {:>6.1} s  {} ({} testes)", t.as_secs_f64(), exe.id, o.passed + o.failed);
    };
    std::thread::scope(|s| {
        for _ in 0..jobs {
            let (queue, done) = (queue.clone(), done.clone());
            s.spawn(move || {
                loop {
                    let Some(group) = queue.lock().ok().and_then(|mut q| q.pop()) else { break };
                    for (exe, k) in group {
                        let (o, t) = run_with_retry(&exe);
                        report_one(&exe, &o, t);
                        if let Ok(mut d) = done.lock() {
                            d.push((exe, k, o, t));
                        }
                    }
                }
            });
        }
    });
    for (exe, k) in serial {
        let (o, t) = run_with_retry(&exe);
        report_one(&exe, &o, t);
        if let Ok(mut d) = done.lock() {
            d.push((exe, k, o, t));
        }
    }
    let done = std::mem::take(&mut *done.lock().map_err(|_| "resultados")?);
    for (exe, k, o, _) in done {
        if let Some(k) = k {
            cache.insert(exe.id.clone(), (k, o.clone()));
        } else {
            cache.remove(&exe.id);
        }
        results.insert(exe.id, o);
    }
    save_cache(&cache_path, &cache)?;
    let report_path = opts.report.clone().unwrap_or_else(|| root.join("medicoes/testes.json"));
    if opts.packages.is_empty() {
        std::fs::write(&report_path, report(&results)).map_err(|e| format!("{}: {e}", report_path.display()))?;
    }
    let failed: Vec<&String> = results.iter().filter(|(_, o)| !o.ok()).map(|(k, _)| k).collect();
    let flaky: Vec<&String> = results.iter().filter(|(_, o)| o.ok() && o.flaky).map(|(k, _)| k).collect();
    println!(
        "▶ {} alvos ({} testes) em {:.0} s: {from_cache} do cache, {to_run} rodaram; {} falharam, {} instáveis",
        results.len(),
        results.values().map(|o| o.passed + o.failed).sum::<u32>(),
        started.elapsed().as_secs_f64(),
        failed.len(),
        flaky.len()
    );
    if !flaky.is_empty() {
        println!("  instáveis (falharam e passaram na segunda vez): {}", flaky.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
    }
    if !undeclared.is_empty() {
        println!("  sem declaração em resources/test-inputs.json (rodam sempre): {}", undeclared.join(", "));
    }
    if opts.packages.is_empty() {
        println!("  relatório: {}", report_path.display());
    }
    if failed.is_empty() {
        Ok(())
    } else {
        let lines: Vec<String> = failed.iter().map(|k| format!("{k}: {}", results[*k].failures.join(", "))).collect();
        Err(format!("{} alvos falharam:\n  {}", failed.len(), lines.join("\n  ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_libtest_output_is_read() {
        let text = "running 3 tests\ntest a ... ok\ntest b::c ... FAILED\ntest d ... ignored\n\nfailures:\n\ntest result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s\n";
        let o = parse_output(text).unwrap();
        assert_eq!((o.passed, o.failed, o.ignored), (1, 1, 1));
        assert_eq!(o.failures, vec!["b::c".to_string()]);
        assert!(!o.ok());
        assert!(parse_output("nada aqui").is_none(), "sem a linha de resultado: caiu");
    }

    #[test]
    fn the_declaration_reads_the_three_kinds() {
        let d = parse_inputs(r#"{"_": "comentário", "a/lib": "pure", "b/visual": ["crates/b/tests/visual"], "c/apps": {"always": "compila projetos"}}"#).unwrap();
        assert_eq!(d["a/lib"], Inputs::Pure);
        assert_eq!(d["b/visual"], Inputs::Paths(vec!["crates/b/tests/visual".into()]));
        assert_eq!(d["c/apps"], Inputs::Always("compila projetos".into()));
        assert!(!d.contains_key("_"));
    }

    #[test]
    fn the_key_changes_with_the_executable_and_the_inputs_and_not_otherwise() {
        let dir = std::env::temp_dir().join(format!("aylis-test-all-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("data/sub")).unwrap();
        std::fs::write(dir.join("t.exe"), b"um").unwrap();
        std::fs::write(dir.join("data/sub/x.json"), b"1").unwrap();
        let exe = TestExe { id: "p/t".into(), package: "p".into(), exe: dir.join("t.exe"), manifest_dir: dir.clone(), doc: false };
        let paths = Inputs::Paths(vec!["data".into()]);
        let k1 = key(&dir, &exe, Some(&paths)).unwrap();
        assert_eq!(key(&dir, &exe, Some(&paths)).unwrap(), k1, "nada mudou: a mesma chave");
        std::fs::write(dir.join("data/sub/x.json"), b"22").unwrap();
        let k2 = key(&dir, &exe, Some(&paths)).unwrap();
        assert_ne!(k2, k1, "uma entrada mudou");
        std::fs::write(dir.join("t.exe"), b"dois!").unwrap();
        assert_ne!(key(&dir, &exe, Some(&paths)).unwrap(), k2, "o executável mudou");
        assert_eq!(key(&dir, &exe, Some(&Inputs::Always("x".into()))), None, "o always roda sempre");
        assert_eq!(key(&dir, &exe, None), None, "sem declaração roda sempre");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_report_has_no_times_and_is_ordered() {
        let mut r = BTreeMap::new();
        r.insert("b/lib".to_string(), Outcome { passed: 2, failed: 0, ignored: 0, failures: Vec::new(), flaky: false, crashed: false });
        r.insert("a/lib".to_string(), Outcome { passed: 1, failed: 1, ignored: 0, failures: vec!["x".into()], flaky: false, crashed: false });
        let text = report(&r);
        assert!(text.find("a/lib").unwrap() < text.find("b/lib").unwrap());
        assert!(!text.contains("seconds") && !text.contains("cache"));
        assert_eq!(text, report(&r), "o mesmo resultado, o mesmo texto");
    }

    #[test]
    fn each_kind_of_target_gets_its_name() {
        assert_eq!(target_id(&[json!("lib")], "aylis_core"), "lib");
        assert_eq!(target_id(&[json!("bin")], "aylis-tool"), "bin:aylis-tool");
        assert_eq!(target_id(&[json!("test")], "visual"), "visual");
    }
}
