//! O compilador do Aylis: gerar o código, verificar o projeto e compilar, nessa ordem e
//! parando no primeiro erro. É o `aylis-tool build`, usado pela linha de comando e pela IDE,
//! que chama o subcomando em outro processo e mostra os eventos dele (o `--json`, uma linha
//! por evento): a geração e a verificação de um projeto grande não pesam mais na IDE.

use std::path::{Path, PathBuf};

use aylis_project::Project;
use aylis_project::check::Issue;
use serde_json::{Value, json};

use crate::process::{CANCELLED, Control, Diagnostic, Event, Level, LineKind};
use aylis_project::Language;

use crate::{Location, Target};

/// A versão do protocolo do `--json`. Mudou o formato de um evento: suba, e a IDE recusa um
/// `aylis-tool` de outra versão (em vez de entender errado).
pub const PROTOCOL: u32 = 1;

/// O começo do erro de um `aylis-tool` que não conhece o `build` (de uma versão anterior).
pub const NOT_A_COMPILER: &str = "o aylis-tool não respondeu como compilador";

/// O que o `aylis-tool build` faz.
#[derive(Clone, Debug)]
pub struct Options {
    /// A plataforma: a verificação do que só vale nela (as threads na web); compila só no
    /// desktop (as outras compilam ao executar ou publicar).
    pub target: Target,
    pub release: bool,
    /// Gerar o código dos forms. Sem gerar, só põe em dia os ícones e as animações que os
    /// forms citam (o que o Publicar fazia).
    pub generate: bool,
    /// Compilar depois de verificar (sem: só gerar e verificar, o preparo do Executar e do
    /// Publicar da IDE).
    pub compile: bool,
}

impl Options {
    pub fn new(target: Target) -> Self {
        Self { target, release: false, generate: true, compile: true }
    }
}

/// Um evento do compilador.
#[derive(Clone, Debug, PartialEq)]
pub enum BuildEvent {
    /// Um passo, uma linha de saída ou uma mensagem do compilador.
    Event(Event),
    /// O código foi gerado (a IDE confere os arquivos abertos no editor).
    Generated,
    /// Os problemas da verificação (erros e avisos).
    Issues(Vec<Issue>),
    /// O executável compilado.
    Built(PathBuf),
}

/// Gera, verifica e compila. `ui`: com as classes registradas, para a verificação carregar os
/// forms (sem ela, não confere as referências entre componentes). Erro: o primeiro que parou.
pub fn build(project: &Project, options: &Options, ui: Option<&mut aylis_core::Ui>, control: &Control, sink: &mut dyn FnMut(BuildEvent)) -> Result<String, String> {
    // O que a geração e o cargo mexerem nos arquivos do projeto vai para as mensagens, com o
    // motivo (I06 do Tabernáculo), tenha compilado ou não.
    let before = crate::touched::before(project);
    let result = steps(project, options, ui, control, sink);
    before.report(&mut |e| sink(BuildEvent::Event(e)));
    result
}

fn steps(project: &Project, options: &Options, ui: Option<&mut aylis_core::Ui>, control: &Control, sink: &mut dyn FnMut(BuildEvent)) -> Result<String, String> {
    // O que mudou, antes de gerar (a geração põe tudo em dia): a verificação confere só isso.
    let changed = aylis_project::codegen::changed_forms(project);
    if options.generate {
        sink(BuildEvent::Event(Event::Step("Gerando o código".into())));
        aylis_project::codegen::generate(project)?;
        sink(BuildEvent::Generated);
    } else {
        // O app leva só os ícones e as animações que os forms citam (o Gerar faz isso; o Salvar só
        // acrescenta os do form salvo).
        aylis_project::codegen::sync_icons(project).and_then(|_| aylis_project::codegen::sync_animations(project))?;
    }
    if control.is_cancelled() {
        return Err(CANCELLED.into());
    }
    sink(BuildEvent::Event(Event::Step("Verificando o projeto".into())));
    let mut issues = aylis_project::check::project_changed(project, ui, options.target == Target::Web, changed.as_deref());
    // Os avisos do projeto inteiro (a fonte do sistema), uma vez por projeto enquanto a IDE está
    // aberta, e não a cada compilação (I11 do Tabernáculo). Os de um componente continuam.
    issues.retain(|i| i.error || !i.form.is_empty() || first_time(&project.path, &i.message));
    let errors = issues.iter().filter(|i| i.error).count();
    sink(BuildEvent::Issues(issues));
    if errors > 0 {
        return Err(format!("{errors} erro(s) no projeto: veja as mensagens (o clique leva ao lugar)"));
    }
    if !options.compile {
        return Ok("Projeto verificado.".into());
    }
    if options.target != Target::Desktop || project.language != Language::Rust {
        return Ok(format!("Projeto verificado ({} em {} compila ao executar ou publicar).", project.language.name(), options.target.name()));
    }
    let exe = crate::run::compile_desktop(project, options.release, control, &mut |e| sink(BuildEvent::Event(e)))?;
    let message = format!("Compilado: {}", exe.display());
    sink(BuildEvent::Built(exe));
    Ok(message)
}

/// O aviso `message` do projeto em `path` ainda não foi dado (e conta como dado).
fn first_time(path: &std::path::Path, message: &str) -> bool {
    static SHOWN: std::sync::Mutex<Vec<(std::path::PathBuf, String)>> = std::sync::Mutex::new(Vec::new());
    let Ok(mut shown) = SHOWN.lock() else { return true };
    let key = (path.to_path_buf(), message.to_string());
    if shown.contains(&key) {
        return false;
    }
    shown.push(key);
    true
}

// ---------------------------------------------------------------------------
// O `--json`: uma linha por evento, a primeira com o protocolo e a última com o fim.

/// A primeira linha: o protocolo e a versão do Aylis.
pub fn start_json() -> Value {
    json!({ "event": "start", "protocol": PROTOCOL, "version": env!("CARGO_PKG_VERSION") })
}

/// A última linha: deu certo (com a mensagem) ou o erro.
pub fn end_json(result: &Result<String, String>) -> Value {
    match result {
        Ok(message) => json!({ "event": "done", "ok": true, "message": message }),
        Err(message) => json!({ "event": "done", "ok": false, "message": message }),
    }
}

/// Um evento numa linha de JSON.
pub fn to_json(event: &BuildEvent) -> Value {
    match event {
        BuildEvent::Event(Event::Step(text)) => json!({ "event": "step", "text": text }),
        BuildEvent::Event(Event::Line(text, kind)) => json!({ "event": "line", "text": text, "kind": kind_name(*kind) }),
        BuildEvent::Event(Event::Diagnostic(d)) => {
            let mut v = json!({ "event": "diagnostic", "level": level_name(d.level), "message": d.message, "rendered": d.rendered });
            if let Some(code) = &d.code {
                v["code"] = json!(code);
            }
            if let Some(l) = &d.location {
                v["file"] = json!(l.file);
                v["line"] = json!(l.line);
                v["column"] = json!(l.column);
            }
            v
        }
        BuildEvent::Generated => json!({ "event": "generated" }),
        BuildEvent::Issues(issues) => json!({ "event": "issues", "issues": issues }),
        BuildEvent::Built(exe) => json!({ "event": "built", "exe": exe }),
    }
}

/// O que uma linha do `--json` diz.
#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    Start { protocol: u32, version: String },
    Event(BuildEvent),
    Done(Result<String, String>),
}

/// Lê uma linha do `--json`. `None`: não é um evento (um texto solto de alguma ferramenta).
pub fn from_json(line: &str) -> Option<Line> {
    let v: Value = serde_json::from_str(line.trim()).ok()?;
    let text = |key: &str| v[key].as_str().unwrap_or_default().to_string();
    let event = match v["event"].as_str()? {
        "start" => return Some(Line::Start { protocol: v["protocol"].as_u64()? as u32, version: text("version") }),
        "done" => return Some(Line::Done(if v["ok"].as_bool()? { Ok(text("message")) } else { Err(text("message")) })),
        "step" => Event::Step(text("text")),
        "line" => Event::Line(text("text"), kind_from(&text("kind"))),
        "diagnostic" => Event::Diagnostic(Diagnostic {
            level: level_from(&text("level")),
            message: text("message"),
            code: v["code"].as_str().map(str::to_string),
            location: v["file"].as_str().map(|f| Location {
                file: PathBuf::from(f),
                line: v["line"].as_u64().unwrap_or(1) as u32,
                column: v["column"].as_u64().unwrap_or(1) as u32,
            }),
            rendered: text("rendered"),
        }),
        "generated" => return Some(Line::Event(BuildEvent::Generated)),
        "issues" => return Some(Line::Event(BuildEvent::Issues(serde_json::from_value(v["issues"].clone()).ok()?))),
        "built" => return Some(Line::Event(BuildEvent::Built(PathBuf::from(text("exe"))))),
        _ => return None,
    };
    Some(Line::Event(BuildEvent::Event(event)))
}

fn kind_name(kind: LineKind) -> &'static str {
    match kind {
        LineKind::Tool => "tool",
        LineKind::Program => "program",
        LineKind::ProgramError => "program_error",
    }
}

fn kind_from(name: &str) -> LineKind {
    match name {
        "program" => LineKind::Program,
        "program_error" => LineKind::ProgramError,
        _ => LineKind::Tool,
    }
}

fn level_name(level: Level) -> &'static str {
    match level {
        Level::Error => "error",
        Level::Warning => "warning",
        Level::Note => "note",
    }
}

fn level_from(name: &str) -> Level {
    match name {
        "error" => Level::Error,
        "warning" => Level::Warning,
        _ => Level::Note,
    }
}

// ---------------------------------------------------------------------------
// Quem chama o `aylis-tool build` em outro processo (a IDE).

/// Roda `aylis-tool build --json` (o `tool`) e passa cada evento para `sink`, como se fosse o
/// [`build`] daqui. Interromper (`control`) encerra o `aylis-tool` e o que ele iniciou.
pub fn build_external(tool: &Path, project: &Project, options: &Options, control: &Control, sink: &mut dyn FnMut(BuildEvent)) -> Result<String, String> {
    let mut command = std::process::Command::new(tool);
    command.arg("build").arg(&project.path).arg("--json").args(["--target", options.target.id()]).args(["--language", project.language.folder()]);
    if options.release {
        command.arg("--release");
    }
    if !options.generate {
        command.arg("--no-generate");
    }
    if !options.compile {
        command.arg("--no-compile");
    }
    let mut started = false;
    let mut end = None;
    // As linhas que não são eventos (um pânico, um erro antes do JSON): vão para as mensagens.
    let mut loose = Vec::new();
    let mut refused = None;
    let code = crate::process::run_lines(&mut command, control, |line, _| match from_json(&line) {
        Some(Line::Start { protocol, version }) => {
            started = true;
            if protocol != PROTOCOL {
                refused = Some(format!(
                    "o aylis-tool ao lado da IDE é de outra versão ({version}, protocolo {protocol}; a IDE espera o {PROTOCOL}): reinstale o Aylis ou recompile o aylis-tools"
                ));
            }
        }
        Some(Line::Event(e)) if refused.is_none() => sink(e),
        Some(Line::Done(result)) => end = Some(result),
        Some(Line::Event(_)) => {}
        None if !line.trim().is_empty() => {
            sink(BuildEvent::Event(Event::Line(line.clone(), LineKind::Tool)));
            loose.push(line);
        }
        None => {}
    })?;
    if let Some(e) = refused {
        return Err(e);
    }
    match end {
        Some(result) => result,
        None if !started => Err(format!("{NOT_A_COMPILER} (é de uma versão anterior?): {}", loose.join(" ").trim())),
        None => Err(format!("o aylis-tool terminou sem dizer o fim (código {})", code.map_or("?".into(), |c| c.to_string()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_events_go_and_come_back_through_the_json() {
        let events = [
            BuildEvent::Event(Event::Step("Gerando o código".into())),
            BuildEvent::Event(Event::Line("Compiling app v0.1.0".into(), LineKind::Tool)),
            BuildEvent::Event(Event::Line("saída".into(), LineKind::ProgramError)),
            BuildEvent::Event(Event::Diagnostic(Diagnostic {
                level: Level::Error,
                message: "cannot find value `x`".into(),
                code: Some("E0425".into()),
                location: Some(Location { file: PathBuf::from("src/forms/main.rs"), line: 12, column: 5 }),
                rendered: "error[E0425]...".into(),
            })),
            BuildEvent::Event(Event::Diagnostic(Diagnostic { level: Level::Note, message: "nota".into(), code: None, location: None, rendered: String::new() })),
            BuildEvent::Generated,
            BuildEvent::Issues(vec![Issue { error: true, form: "frmMain".into(), component: "btnOk".into(), message: "o handler falta".into() }]),
            BuildEvent::Built(PathBuf::from("target/debug/app.exe")),
        ];
        for event in events {
            let line = to_json(&event).to_string();
            assert!(!line.contains('\n'), "uma linha por evento: {line}");
            assert_eq!(from_json(&line), Some(Line::Event(event)));
        }
        assert_eq!(from_json(&start_json().to_string()), Some(Line::Start { protocol: PROTOCOL, version: env!("CARGO_PKG_VERSION").into() }));
        assert_eq!(from_json(&end_json(&Err("2 erro(s)".into())).to_string()), Some(Line::Done(Err("2 erro(s)".into()))));
        assert_eq!(from_json("   Compiling serde v1.0"), None);
    }
}
