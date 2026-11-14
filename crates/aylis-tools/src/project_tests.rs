//! Os testes de um projeto do Aylis, na ferramenta de teste de cada linguagem, com o resultado
//! por teste: o `test` do MCP da IDE (o relatório do Tabernáculo, M19: rodar os testes era só
//! proposta, em `docs/proposta-llm-ci.md`).
//!
//! - Rust: `cargo test --no-fail-fast` na pasta do código (a pasta de compilação compartilhada);
//! - Python: `python -m unittest discover -v`;
//! - Go: `go test -v ./...`;
//! - TypeScript: `bun test` ou `node --test`; C#: `dotnet test` (sem o resultado por teste, só
//!   se passou);
//! - Java, C++ e Free Pascal (decidido em 14/11): os testes da pasta `tests/` com o executor
//!   mínimo do cliente de cada um (`dev.aylis.TestRunner` com `@Test`, o `aylis_test.hpp` com
//!   `AYLIS_TEST`, o FPCUnit ouvido pela unit `aylistest`), que imprime uma linha por teste
//!   (`AYLIS_TEST ok|FAIL nome [mensagem]`); o Java usa o JUnit quando o projeto o tem
//!   (`tests/lib/junit-platform-console-standalone*.jar`). Compilados como o app, em
//!   `<projeto>/.aylis/tests`.

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::{Language, Project};

use crate::process::{self, Level, run_tool};
use crate::{Control, Event, LineKind, Sink};

/// Como um teste terminou.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Passed,
    Failed,
    Ignored,
}

impl Outcome {
    pub fn name(self) -> &'static str {
        match self {
            Outcome::Passed => "passed",
            Outcome::Failed => "failed",
            Outcome::Ignored => "ignored",
        }
    }
}

/// Um teste e como terminou.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestCase {
    pub name: String,
    pub outcome: Outcome,
}

/// O resultado de uma rodada.
#[derive(Clone, Debug, Default)]
pub struct TestReport {
    /// A linha de comando que rodou.
    pub command: String,
    /// Passou: compilou e nenhum teste falhou (a ferramenta saiu com 0).
    pub success: bool,
    pub cases: Vec<TestCase>,
    /// O que cada teste que falhou mostrou (o `assert`, o pânico), pelo nome.
    pub failures: Vec<(String, String)>,
    /// Os erros de compilação (Rust): com eles, nenhum teste rodou.
    pub compile_errors: Vec<String>,
    /// As últimas linhas da saída.
    pub last_lines: Vec<String>,
}

impl TestReport {
    pub fn count(&self, outcome: Outcome) -> usize {
        self.cases.iter().filter(|c| c.outcome == outcome).count()
    }
}

/// Roda os testes do projeto na linguagem ativa. As linhas vão também para `sink` (as
/// mensagens da IDE). `Err` só quando nem deu para começar (a ferramenta não existe).
pub fn run(project: &Project, control: &Control, sink: Sink) -> Result<TestReport, String> {
    let dir = project.code_dir();
    if !dir.is_dir() {
        return Err(format!("a pasta do código não existe: {} (gere o código antes)", dir.display()));
    }
    let mut lines = Vec::new();
    let mut report = TestReport::default();
    match project.language {
        Language::Rust => {
            report.command = "cargo test --no-fail-fast".into();
            let mut errors = Vec::new();
            let result = process::cargo("cargo", &["test", "--no-fail-fast"], &dir, &[], control, &mut |e| {
                match &e {
                    Event::Line(line, _) => lines.push(line.clone()),
                    Event::Diagnostic(d) if d.level == Level::Error => {
                        let at = d.location.as_ref().map(|l| format!("{}:{}: ", l.file.display(), l.line)).unwrap_or_default();
                        errors.push(format!("{at}{}", d.message));
                    }
                    _ => {}
                }
                sink(e);
            });
            report.compile_errors = errors;
            report.success = result.is_ok();
            if let Err(e) = result {
                if control.is_cancelled() {
                    return Err(e);
                }
            }
        }
        language => {
            let mut junit_reports = None;
            let (mut command, shown) = match language {
                Language::Java | Language::Cpp | Language::Pascal => {
                    let (command, shown, reports) = minimal(project, control, sink)?;
                    junit_reports = reports;
                    (command, shown)
                }
                _ => command_for(language)?,
            };
            report.command = shown;
            command.current_dir(&dir);
            let code = process::run_lines(&mut command, control, |line, _| {
                sink(Event::Line(line.clone(), LineKind::Program));
                lines.push(line);
            })?;
            report.success = code == Some(0);
            if let Some(reports) = junit_reports {
                lines.extend(junit_lines(&reports));
            }
        }
    }
    let (cases, failures) = parse(project.language, &lines);
    report.cases = cases;
    report.failures = failures;
    report.last_lines = lines.iter().rev().take(20).rev().cloned().collect();
    Ok(report)
}

/// Compila os testes de `tests/` de um projeto Java, C++ ou Free Pascal com o executor mínimo
/// do cliente (veja o módulo) e devolve o comando que os roda, como ele aparece e, com o JUnit,
/// a pasta dos relatórios em XML (lidos depois de rodar, [`junit_lines`]).
fn minimal(project: &Project, control: &Control, sink: Sink) -> Result<(Command, String, Option<PathBuf>), String> {
    use crate::hostlang::files;
    let language = project.language;
    let dir = project.code_dir();
    let tests = dir.join("tests");
    let ext = match language {
        Language::Java => "java",
        Language::Cpp => "cpp",
        _ => "pas",
    };
    let mut sources = Vec::new();
    files(&tests, ext, &mut sources);
    if sources.is_empty() {
        return Err(format!("sem testes em {} (arquivos .{ext}; veja docs/testes-do-projeto.md)", tests.display()));
    }
    let tc = crate::hostlang::find(language)?;
    let out = project.dir().join(".aylis").join("tests").join(language.folder());
    std::fs::create_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    let clients = PathBuf::from(aylis_project::codegen::aylis_root()).join("clients");
    let s = |p: &Path| p.display().to_string();
    sink(Event::Step(format!("Compilando os testes de {} ({})…", project.name, language.name())));
    match language {
        Language::Java => {
            let mut all = Vec::new();
            files(&dir.join("src"), "java", &mut all);
            files(&clients.join("java/dev/aylis"), "java", &mut all);
            all.extend(sources);
            let classes = out.join("classes");
            let _ = std::fs::remove_dir_all(&classes);
            std::fs::create_dir_all(&classes).map_err(|e| e.to_string())?;
            let junit = junit_jar(&tests);
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).args(["-encoding", "UTF-8", "-g"]);
            if let Some(jar) = &junit {
                c.arg("-cp").arg(jar);
            }
            c.arg("-d").arg(&classes).args(&all);
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            let java = tc.extra.clone().unwrap_or_else(|| PathBuf::from("java"));
            let mut run = Command::new(&java);
            Ok(match junit {
                Some(jar) => {
                    let reports = out.join("reports");
                    let _ = std::fs::remove_dir_all(&reports);
                    run.arg("-jar").arg(&jar).args(["execute", "--disable-banner", "--details=none", "--scan-class-path", "--class-path"]).arg(&classes).arg("--reports-dir").arg(&reports);
                    (run, format!("{} -jar {} execute --scan-class-path", s(&java), s(&jar)), Some(reports))
                }
                None => {
                    run.arg("-cp").arg(&classes).arg("dev.aylis.TestRunner").arg(&classes);
                    (run, format!("{} -cp {} dev.aylis.TestRunner", s(&java), s(&classes)), None)
                }
            })
        }
        Language::Cpp => {
            let program = out.join(crate::env::exe("tests"));
            let main = out.join("aylis_tests_main.cpp");
            std::fs::write(&main, "// Gerado pelo Aylis: o main dos testes (veja clients/cpp/aylis_test.hpp).\n#define AYLIS_TEST_MAIN\n#include \"aylis_test.hpp\"\n").map_err(|e| e.to_string())?;
            // Os forms (sem o `main.cpp` do app, que tem o próprio `main`) e os testes.
            let mut all = vec![main];
            files(&dir.join("src/forms"), "cpp", &mut all);
            all.extend(sources);
            let includes = [clients.join("cpp"), dir.join("src")];
            if tc.program.extension().is_some_and(|e| e.eq_ignore_ascii_case("bat")) {
                // Visual C++: um .bat que prepara o ambiente e chama o cl, como o build.
                let objs = out.join("obj");
                std::fs::create_dir_all(&objs).map_err(|e| e.to_string())?;
                let bat_path = |p: &Path| p.display().to_string().replace('%', "%%");
                let list: Vec<String> = all.iter().map(|p| format!("\"{}\"", bat_path(p))).collect();
                let inc: Vec<String> = includes.iter().map(|p| format!("/I\"{}\"", bat_path(p))).collect();
                let script = format!(
                    "@call \"{}\" >nul\r\ncl /nologo /std:c++17 /EHsc /utf-8 /Od {} {} /Fe:\"{}\" /Fo:\"{}/\"\r\n",
                    bat_path(&tc.program),
                    inc.join(" "),
                    list.join(" "),
                    bat_path(&program),
                    bat_path(&objs)
                );
                let bat = out.join("compilar-testes.bat");
                std::fs::write(&bat, script).map_err(|e| e.to_string())?;
                // lint-allow: cmd-shell (o .bat é nosso: chama o vcvars do MSVC antes do cl)
                run_tool(Command::new("cmd").current_dir(&dir).arg("/C").arg(&bat), control, sink, LineKind::Tool)?;
            } else {
                let mut c = Command::new(&tc.program);
                c.current_dir(&dir).arg("-std=c++17").arg("-g");
                for i in &includes {
                    c.arg(format!("-I{}", i.display()));
                }
                c.args(&all).arg("-o").arg(&program);
                run_tool(&mut c, control, sink, LineKind::Tool)?;
            }
            Ok((Command::new(&program), s(&program), None))
        }
        _ => {
            // O programa que chama o executor, com as units dos testes no `uses`.
            let units: Vec<String> = sources.iter().filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().to_string())).collect();
            let program = out.join("aylistests.lpr");
            let text = format!(
                "{{ Gerado pelo Aylis: roda os testes de tests/ (veja clients/pascal/aylistest.pas). }}\nprogram aylistests;\n\n{{$mode objfpc}}{{$H+}}\n\nuses\n  aylistest, {};\n\nbegin\n  RunAll;\nend.\n",
                units.join(", ")
            );
            std::fs::write(&program, text).map_err(|e| e.to_string())?;
            let unit_dir = out.join("units");
            std::fs::create_dir_all(&unit_dir).map_err(|e| e.to_string())?;
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).arg("-Mobjfpc");
            if cfg!(windows) {
                c.arg("-Px86_64");
            }
            if let Some(u) = &tc.extra {
                c.arg(format!("-Fu{}", u.join("*").display())).arg(format!("-Fu{}", u.join("rtl").display()));
            }
            // O FPCUnit: o `fpc.cfg` de uma instalação de 32 bits não aponta as units do alvo de
            // 64 (o `-Px86_64`), então a pasta dele vai na linha de comando.
            let target = if cfg!(windows) { "x86_64-win64" } else { "x86_64-linux" };
            let fpcunit = tc.extra.clone().or_else(|| Some(tc.program.parent()?.parent()?.parent()?.join("units").join(target))).map(|u| u.join("fcl-fpcunit"));
            if let Some(dir) = fpcunit.filter(|d| d.is_dir()) {
                c.arg(format!("-Fu{}", dir.display()));
            }
            c.arg("-gw3").arg("-Fusrc").arg("-Fusrc/forms").arg("-Futests").arg(format!("-Fu{}", clients.join("pascal").display()));
            c.arg(format!("-FU{}", unit_dir.display())).arg(format!("-FE{}", out.display())).arg(&program);
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            let exe = out.join(crate::env::exe("aylistests"));
            Ok((Command::new(&exe), s(&exe), None))
        }
    }
}

/// O JUnit do projeto (o `junit-platform-console-standalone*.jar` em `tests/lib`), se houver.
fn junit_jar(tests: &Path) -> Option<PathBuf> {
    std::fs::read_dir(tests.join("lib")).ok()?.flatten().map(|e| e.path()).find(|p| {
        let name = p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        name.starts_with("junit-platform-console-standalone") && name.ends_with(".jar")
    })
}

/// Os relatórios em XML do JUnit (`TEST-*.xml`, o formato do Ant) como as linhas do executor
/// mínimo, para um analisador só.
fn junit_lines(reports: &Path) -> Vec<String> {
    let mut lines = Vec::new();
    let Ok(entries) = std::fs::read_dir(reports) else { return lines };
    for path in entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "xml")) {
        if let Ok(text) = std::fs::read_to_string(&path) {
            lines.extend(junit_xml_lines(&text));
        }
    }
    lines
}

fn junit_xml_lines(text: &str) -> Vec<String> {
    fn attr(tag: &str, name: &str) -> String {
        let key = format!(" {name}=\"");
        tag.find(&key)
            .and_then(|i| tag[i + key.len()..].split('"').next())
            .map(|v| v.replace("&quot;", "\"").replace("&lt;", "<").replace("&gt;", ">").replace("&apos;", "'").replace("&amp;", "&"))
            .unwrap_or_default()
    }
    let mut lines = Vec::new();
    for chunk in text.split("<testcase").skip(1) {
        let head = chunk.split('>').next().unwrap_or_default();
        let name = format!("{}.{}", attr(head, "classname"), attr(head, "name").trim_end_matches("()"));
        // O caso vai até o fim dele (`/>` no próprio cabeçalho, ou `</testcase>`).
        let body = if head.ends_with('/') { "" } else { chunk.split("</testcase>").next().unwrap_or_default() };
        let line = if let Some(at) = body.find("<failure").or_else(|| body.find("<error")) {
            let message = attr(body[at..].split('>').next().unwrap_or_default(), "message").replace(['\r', '\n'], " ");
            format!("AYLIS_TEST FAIL {name} {message}")
        } else if body.contains("<skipped") {
            format!("AYLIS_TEST skip {name}")
        } else {
            format!("AYLIS_TEST ok {name}")
        };
        lines.push(line);
    }
    lines
}

/// O comando de teste de uma linguagem (menos Rust) e como ele aparece.
fn command_for(language: Language) -> Result<(Command, String), String> {
    Ok(match language {
        Language::Python => {
            let (python, _) = crate::env::python().ok_or("Python não encontrado (Ferramentas → Configurar linguagens)")?;
            let mut c = Command::new(&python);
            c.args(["-m", "unittest", "discover", "-v"]);
            (c, format!("{python} -m unittest discover -v"))
        }
        Language::Go => {
            let tc = crate::hostlang::find(Language::Go)?;
            let mut c = Command::new(&tc.program);
            c.args(["test", "-v", "./..."]);
            (c, "go test -v ./...".into())
        }
        Language::TypeScript => {
            let tc = crate::hostlang::find(Language::TypeScript)?;
            let bun = tc.program.file_stem().is_some_and(|s| s.to_string_lossy().eq_ignore_ascii_case("bun"));
            let mut c = Command::new(&tc.program);
            c.arg(if bun { "test" } else { "--test" });
            (c, if bun { "bun test".into() } else { "node --test".into() })
        }
        Language::CSharp => {
            let tc = crate::hostlang::find(Language::CSharp)?;
            let mut c = Command::new(&tc.program);
            c.arg("test");
            (c, "dotnet test".into())
        }
        other => return Err(format!("{} não tem um executor de testes padrão: rode os testes do projeto pela ferramenta dele", other.name())),
    })
}

/// Os testes e as falhas na saída da ferramenta de teste da linguagem.
pub fn parse(language: Language, lines: &[String]) -> (Vec<TestCase>, Vec<(String, String)>) {
    match language {
        Language::Rust => parse_rust(lines),
        Language::Python => parse_python(lines),
        Language::Go => parse_go(lines),
        Language::Java | Language::Cpp | Language::Pascal => parse_aylis(lines),
        _ => (Vec::new(), Vec::new()),
    }
}

/// `test mod::nome ... ok|FAILED|ignored`, e os blocos `---- mod::nome stdout ----`.
fn parse_rust(lines: &[String]) -> (Vec<TestCase>, Vec<(String, String)>) {
    let mut cases = Vec::new();
    for line in lines {
        let Some(rest) = line.strip_prefix("test ") else { continue };
        let Some((name, result)) = rest.rsplit_once(" ... ") else { continue };
        let outcome = match result.trim() {
            "ok" => Outcome::Passed,
            "FAILED" => Outcome::Failed,
            r if r.starts_with("ignored") => Outcome::Ignored,
            _ => continue,
        };
        cases.push(TestCase { name: name.trim().to_string(), outcome });
    }
    let mut failures = Vec::new();
    let mut current: Option<(String, Vec<&str>)> = None;
    for line in lines {
        let header = line.strip_prefix("---- ").and_then(|r| r.strip_suffix(" ----")).and_then(|r| r.rsplit_once(' ')).map(|(name, _)| name.to_string());
        if header.is_some() || line.trim() == "failures:" || line.starts_with("test result:") {
            if let Some((name, text)) = current.take() {
                failures.push((name, text.join("\n").trim().to_string()));
            }
            current = header.map(|h| (h, Vec::new()));
            continue;
        }
        if let Some((_, text)) = current.as_mut() {
            text.push(line);
        }
    }
    if let Some((name, text)) = current {
        failures.push((name, text.join("\n").trim().to_string()));
    }
    (cases, failures)
}

/// `test_x (modulo.Classe.test_x) ... ok|FAIL|ERROR|skipped`, e os blocos `FAIL: ...`.
fn parse_python(lines: &[String]) -> (Vec<TestCase>, Vec<(String, String)>) {
    let mut cases = Vec::new();
    for line in lines {
        let Some((name, result)) = line.rsplit_once(" ... ") else { continue };
        let outcome = match result.trim() {
            "ok" => Outcome::Passed,
            "FAIL" | "ERROR" => Outcome::Failed,
            r if r.starts_with("skipped") || r.starts_with("expected failure") => Outcome::Ignored,
            _ => continue,
        };
        cases.push(TestCase { name: name.trim().to_string(), outcome });
    }
    let mut failures = Vec::new();
    let mut current: Option<(String, Vec<&str>)> = None;
    for line in lines {
        let header = line.strip_prefix("FAIL: ").or_else(|| line.strip_prefix("ERROR: ")).map(|n| n.trim().to_string());
        let rule = line.starts_with("=====") || line.starts_with("-----");
        if header.is_some() || (rule && current.as_ref().is_some_and(|(_, t)| !t.is_empty())) {
            if let Some((name, text)) = current.take() {
                failures.push((name, text.join("\n").trim().to_string()));
            }
            current = header.map(|h| (h, Vec::new()));
            continue;
        }
        if let Some((_, text)) = current.as_mut().filter(|_| !rule) {
            text.push(line);
        }
    }
    if let Some((name, text)) = current {
        failures.push((name, text.join("\n").trim().to_string()));
    }
    (cases, failures)
}

/// A linha do executor mínimo (Java, C++, Pascal): `AYLIS_TEST ok|FAIL|skip nome [mensagem]`.
fn parse_aylis(lines: &[String]) -> (Vec<TestCase>, Vec<(String, String)>) {
    let mut cases = Vec::new();
    let mut failures = Vec::new();
    for line in lines {
        let Some(rest) = line.trim().strip_prefix("AYLIS_TEST ") else { continue };
        let mut parts = rest.splitn(3, ' ');
        let (Some(result), Some(name)) = (parts.next(), parts.next()) else { continue };
        let outcome = match result {
            "ok" => Outcome::Passed,
            "FAIL" => Outcome::Failed,
            "skip" => Outcome::Ignored,
            _ => continue,
        };
        if outcome == Outcome::Failed {
            failures.push((name.to_string(), parts.next().unwrap_or_default().trim().to_string()));
        }
        cases.push(TestCase { name: name.to_string(), outcome });
    }
    (cases, failures)
}

/// `--- PASS|FAIL|SKIP: TestX (0.00s)`; a falha leva as linhas do teste (`x_test.go:12: ...`).
fn parse_go(lines: &[String]) -> (Vec<TestCase>, Vec<(String, String)>) {
    let mut cases = Vec::new();
    let mut failures = Vec::new();
    let mut output: Vec<&str> = Vec::new();
    for line in lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with("=== RUN") {
            output.clear();
            continue;
        }
        let result = [("--- PASS: ", Outcome::Passed), ("--- FAIL: ", Outcome::Failed), ("--- SKIP: ", Outcome::Ignored)]
            .into_iter()
            .find_map(|(prefix, outcome)| trimmed.strip_prefix(prefix).map(|r| (r, outcome)));
        match result {
            Some((rest, outcome)) => {
                let name = rest.split(" (").next().unwrap_or(rest).trim().to_string();
                if outcome == Outcome::Failed {
                    failures.push((name.clone(), output.join("\n").trim().to_string()));
                }
                cases.push(TestCase { name, outcome });
                output.clear();
            }
            None => output.push(line),
        }
    }
    (cases, failures)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(str::to_string).collect()
    }

    #[test]
    fn rust_output_gives_each_test_and_the_failure_text() {
        let out = lines(
            "running 3 tests\ntest calc::soma ... ok\ntest calc::divide ... FAILED\ntest calc::lento ... ignored, demora\n\nfailures:\n\n---- calc::divide stdout ----\nthread 'calc::divide' panicked at src/calc.rs:9:5:\nassertion `left == right` failed\n\nfailures:\n    calc::divide\n\ntest result: FAILED. 1 passed; 1 failed; 1 ignored",
        );
        let (cases, failures) = parse(Language::Rust, &out);
        assert_eq!(cases.len(), 3);
        assert_eq!(cases[1], TestCase { name: "calc::divide".into(), outcome: Outcome::Failed });
        assert_eq!(cases[2].outcome, Outcome::Ignored);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].0, "calc::divide");
        assert!(failures[0].1.contains("src/calc.rs:9:5"), "{:?}", failures[0]);
    }

    #[test]
    fn python_output_gives_each_test_and_the_failure_text() {
        let out = lines(
            "test_soma (test_calc.Calc.test_soma) ... ok\ntest_divide (test_calc.Calc.test_divide) ... FAIL\ntest_lento (test_calc.Calc.test_lento) ... skipped 'demora'\n\n======================================================================\nFAIL: test_divide (test_calc.Calc.test_divide)\n----------------------------------------------------------------------\nTraceback (most recent call last):\nAssertionError: 2 != 3\n\n----------------------------------------------------------------------\nRan 3 tests in 0.001s\n\nFAILED (failures=1, skipped=1)",
        );
        let (cases, failures) = parse(Language::Python, &out);
        assert_eq!(cases.iter().map(|c| c.outcome).collect::<Vec<_>>(), [Outcome::Passed, Outcome::Failed, Outcome::Ignored]);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].0, "test_divide (test_calc.Calc.test_divide)");
        assert!(failures[0].1.contains("AssertionError: 2 != 3"), "{:?}", failures[0]);
    }

    #[test]
    fn the_minimal_runner_and_junit_xml_give_each_test() {
        let out = lines("Compilando...
AYLIS_TEST ok forms.MotorTest.soma
AYLIS_TEST FAIL forms.MotorTest.divide esperado 2, veio 3
AYLIS_TEST skip TTestMotor.Lento
AYLIS_TEST_DONE 1 ok, 1 falharam");
        let (cases, failures) = parse(Language::Java, &out);
        assert_eq!(cases.iter().map(|c| c.outcome).collect::<Vec<_>>(), [Outcome::Passed, Outcome::Failed, Outcome::Ignored]);
        assert_eq!(failures, [("forms.MotorTest.divide".to_string(), "esperado 2, veio 3".to_string())]);
        // O relatório do JUnit (o formato do Ant) vira as mesmas linhas.
        let xml = r#"<?xml version="1.0"?><testsuite name="JUnit Jupiter"><testcase name="soma()" classname="MotorTest" time="0"/><testcase name="divide()" classname="MotorTest" time="0"><failure message="expected: &lt;2&gt; but was: &lt;3&gt;" type="org.opentest4j.AssertionFailedError">pilha</failure></testcase><testcase name="lento()" classname="MotorTest"><skipped/></testcase></testsuite>"#;
        assert_eq!(junit_xml_lines(xml), ["AYLIS_TEST ok MotorTest.soma", "AYLIS_TEST FAIL MotorTest.divide expected: <2> but was: <3>", "AYLIS_TEST skip MotorTest.lento"]);
    }

    /// Os testes da Calculadora em Java, C++ e Free Pascal (`examples/Calculadora/<linguagem>/tests`)
    /// pelo executor mínimo; a linguagem sem compilador na máquina é pulada com aviso.
    #[test]
    fn java_cpp_and_pascal_run_the_project_tests() {
        let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).canonicalize().unwrap();
        let root = PathBuf::from(root.display().to_string().trim_start_matches(r"\\?\"));
        for language in [Language::Java, Language::Cpp, Language::Pascal] {
            if let Err(e) = crate::hostlang::find(language) {
                eprintln!("{}: pulado ({e})", language.name());
                continue;
            }
            let mut project = Project::open(root.join("examples/Calculadora/Calculadora.aylisproj")).unwrap();
            project.language = language;
            let mut output = Vec::new();
            let report = run(&project, &Control::new(), &mut |e| {
                if let Event::Line(line, _) = e {
                    output.push(line);
                }
            })
            .unwrap_or_else(|e| panic!("{}: {e}", language.name()));
            let passed = report.count(Outcome::Passed);
            assert!(report.success && passed == 2 && report.count(Outcome::Failed) == 0, "{}: {passed} passaram, {:?}
{}", language.name(), report.failures, output.join("
"));
        }
    }

    #[test]
    fn go_output_gives_each_test_and_the_failure_text() {
        let out = lines("=== RUN   TestSoma\n--- PASS: TestSoma (0.00s)\n=== RUN   TestDivide\n    calc_test.go:12: esperado 3, veio 2\n--- FAIL: TestDivide (0.00s)\nFAIL");
        let (cases, failures) = parse(Language::Go, &out);
        assert_eq!(cases, [TestCase { name: "TestSoma".into(), outcome: Outcome::Passed }, TestCase { name: "TestDivide".into(), outcome: Outcome::Failed }]);
        assert_eq!(failures[0].1, "calc_test.go:12: esperado 3, veio 2");
    }
}
