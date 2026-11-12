//! Os testes de um projeto do Aylis, na ferramenta de teste de cada linguagem, com o resultado
//! por teste: o `test` do MCP da IDE (o relatório do Tabernáculo, M19: rodar os testes era só
//! proposta, em `docs/proposta-llm-ci.md`).
//!
//! - Rust: `cargo test --no-fail-fast` na pasta do código (a pasta de compilação compartilhada);
//! - Python: `python -m unittest discover -v`;
//! - Go: `go test -v ./...`;
//! - TypeScript: `bun test` ou `node --test`; C#: `dotnet test` (sem o resultado por teste, só
//!   se passou);
//! - Java, C++ e Free Pascal não têm executor de testes padrão: o erro diz.

use std::process::Command;

use aylis_project::{Language, Project};

use crate::process::{self, Level};
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
            let (mut command, shown) = command_for(language)?;
            report.command = shown;
            command.current_dir(&dir);
            let code = process::run_lines(&mut command, control, |line, _| {
                sink(Event::Line(line.clone(), LineKind::Program));
                lines.push(line);
            })?;
            report.success = code == Some(0);
        }
    }
    let (cases, failures) = parse(project.language, &lines);
    report.cases = cases;
    report.failures = failures;
    report.last_lines = lines.iter().rev().take(20).rev().cloned().collect();
    Ok(report)
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
    fn go_output_gives_each_test_and_the_failure_text() {
        let out = lines("=== RUN   TestSoma\n--- PASS: TestSoma (0.00s)\n=== RUN   TestDivide\n    calc_test.go:12: esperado 3, veio 2\n--- FAIL: TestDivide (0.00s)\nFAIL");
        let (cases, failures) = parse(Language::Go, &out);
        assert_eq!(cases, [TestCase { name: "TestSoma".into(), outcome: Outcome::Passed }, TestCase { name: "TestDivide".into(), outcome: Outcome::Failed }]);
        assert_eq!(failures[0].1, "calc_test.go:12: esperado 3, veio 2");
    }
}
