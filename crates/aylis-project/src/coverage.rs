//! Cobertura de handlers, em qualquer linguagem (4b da proposta de CI com LLM): os handlers que
//! os forms do projeto ligam a eventos, menos os que já rodaram. O que rodou vem do app em
//! execução pelo canal de desenvolvimento (o observador de atividade do app avisa a IDE do
//! primeiro disparo de cada handler), somado entre as execuções em `.aylis/coverage.json` (fora
//! do controle de versão), até alguém limpar. Funciona igual nas 8 linguagens: não depende da
//! ferramenta de cobertura de nenhuma.

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::{Error, Project};

/// O arquivo, na pasta `.aylis` do projeto.
pub const FILE: &str = "coverage.json";

/// Um handler de um form: onde ele é ligado (`componente.evento`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub form: String,
    pub handler: String,
    /// Os eventos que chamam o handler (`btnSalvar.on_click`; o do próprio form, `frmX.on_show`).
    pub events: Vec<String>,
}

impl Handler {
    /// `frmClientes.btnSalvar_click`: o nome com que o app avisa e o Profiler mostra.
    pub fn key(&self) -> String {
        format!("{}.{}", self.form, self.handler)
    }
}

/// Os handlers ligados nos forms do projeto (o layout padrão e as variantes por plataforma), na
/// ordem dos forms, sem repetir dentro de um form.
pub fn project_handlers(project: &Project) -> Vec<Handler> {
    let mut out: Vec<Handler> = Vec::new();
    for entry in &project.forms {
        let Ok(layouts) = project.read_form_variants(&entry.name) else { continue };
        let start = out.len();
        for (_, text) in layouts {
            let Ok(doc) = serde_json::from_str::<Value>(text.trim_start_matches('\u{feff}')) else { continue };
            let mut stack = vec![doc.get("root").unwrap_or(&doc)];
            while let Some(node) = stack.pop() {
                let Some(obj) = node.as_object() else { continue };
                let component = obj.get("name").and_then(Value::as_str).unwrap_or(&entry.name);
                for (key, value) in obj {
                    let Some(handler) = value.as_str().filter(|h| !h.is_empty() && crate::codegen::is_event_key(key)) else { continue };
                    let event = format!("{component}.{key}");
                    match out[start..].iter_mut().find(|h| h.handler == handler) {
                        Some(h) if !h.events.contains(&event) => h.events.push(event),
                        Some(_) => {}
                        None => out.push(Handler { form: entry.name.clone(), handler: handler.to_string(), events: vec![event] }),
                    }
                }
                // Os filhos na ordem do layout (a pilha inverte).
                if let Some(children) = obj.get("children").and_then(Value::as_array) {
                    stack.extend(children.iter().rev());
                }
            }
        }
    }
    out
}

fn file(project: &Project) -> PathBuf {
    project.dir().join(".aylis").join(FILE)
}

/// O que já rodou (`form.handler`) e desde quando (milissegundos desde 1970; 0: nunca).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ran {
    pub since_ms: u64,
    pub handlers: BTreeSet<String>,
}

/// O que já rodou neste projeto.
pub fn load(project: &Project) -> Ran {
    let Some(v) = std::fs::read_to_string(file(project)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else { return Ran::default() };
    Ran {
        since_ms: v["since_ms"].as_u64().unwrap_or(0),
        handlers: v["ran"].as_array().into_iter().flatten().filter_map(|h| h.as_str().map(str::to_string)).collect(),
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

fn save(project: &Project, ran: &Ran) -> Result<(), Error> {
    let path = file(project);
    if let Some(dir) = path.parent() {
        crate::local_dir_ignored(dir);
    }
    let text = serde_json::to_string_pretty(&json!({ "since_ms": ran.since_ms, "ran": ran.handlers })).map_err(|e| e.to_string())? + "\n";
    std::fs::write(&path, text).map_err(|e| format!("não consegui gravar {}: {e}", path.display()))
}

/// Soma os handlers que rodaram (`form.handler`) ao que já estava; grava só se houver novidade.
/// Devolve quantos eram novos.
pub fn record(project: &Project, handlers: &[String]) -> Result<usize, Error> {
    let mut ran = load(project);
    let before = ran.handlers.len();
    ran.handlers.extend(handlers.iter().filter(|h| !h.is_empty()).cloned());
    let new = ran.handlers.len() - before;
    if new > 0 {
        if ran.since_ms == 0 {
            ran.since_ms = now_ms();
        }
        save(project, &ran)?;
    }
    Ok(new)
}

/// Esquece o que rodou (a cobertura recomeça do zero).
pub fn clear(project: &Project) -> Result<(), Error> {
    let path = file(project);
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("não consegui apagar {}: {e}", path.display())),
    }
}

/// A cobertura: os handlers dos forms que rodaram e os que nenhum teste ou uso tocou.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    pub covered: Vec<Handler>,
    pub missing: Vec<Handler>,
    pub since_ms: u64,
}

impl Report {
    pub fn total(&self) -> usize {
        self.covered.len() + self.missing.len()
    }

    /// A porcentagem coberta (100 sem handlers).
    pub fn percent(&self) -> f64 {
        if self.total() == 0 { 100.0 } else { self.covered.len() as f64 * 100.0 / self.total() as f64 }
    }
}

/// Separa os handlers do projeto pelo que rodou. Um nome sem o form (`btnSalvar_click`, de um
/// app que não soube o form) ou com um escopo que não é form do projeto (a linha de um
/// `DataRepeater`, que sobe até o form dono do handler) cobre o handler de mesmo nome em
/// qualquer form.
pub fn report(handlers: Vec<Handler>, ran: &Ran) -> Report {
    let forms: BTreeSet<&str> = handlers.iter().map(|h| h.form.as_str()).collect();
    let bare: BTreeSet<String> = ran
        .handlers
        .iter()
        .filter_map(|h| match h.rsplit_once('.') {
            None => Some(h.clone()),
            Some((scope, name)) if !forms.contains(scope) => Some(name.to_string()),
            Some(_) => None,
        })
        .collect();
    let (covered, missing) = handlers.into_iter().partition(|h| ran.handlers.contains(&h.key()) || bare.contains(&h.handler));
    Report { covered, missing, since_ms: ran.since_ms }
}

/// A cobertura do projeto agora.
pub fn project_report(project: &Project) -> Report {
    report(project_handlers(project), &load(project))
}

/// O relatório em JSON (o `coverage` do MCP e o `aylis-tool`): os números e os handlers que
/// faltam, cada um com o arquivo e a linha do código na linguagem do projeto (lido uma vez por
/// form).
pub fn report_json(project: &Project, report: &Report) -> Value {
    let mut by_form: Vec<(&str, Vec<&Handler>)> = Vec::new();
    for h in &report.missing {
        match by_form.iter_mut().find(|(f, _)| *f == h.form) {
            Some((_, list)) => list.push(h),
            None => by_form.push((&h.form, vec![h])),
        }
    }
    let code_dir = project.code_dir();
    let mut missing = Vec::new();
    for (form, list) in by_form {
        let names: Vec<String> = list.iter().map(|h| h.handler.clone()).collect();
        let places = crate::sync::handler_lines(project, project.language, form, &names);
        for (i, h) in list.iter().enumerate() {
            let mut item = json!({ "form": h.form, "handler": h.handler, "events": h.events });
            if let Some((file, lines)) = &places {
                let shown = file.strip_prefix(&code_dir).map_or_else(|_| file.display().to_string(), |p| p.display().to_string().replace('\\', "/"));
                item["file"] = json!(shown);
                if let Some(Some(line)) = lines.get(i) {
                    item["line"] = json!(line);
                }
            }
            missing.push(item);
        }
    }
    json!({
        "handlers": report.total(),
        "covered": report.covered.len(),
        "percent": (report.percent() * 10.0).round() / 10.0,
        "since_ms": report.since_ms,
        "missing": missing,
        "covered_handlers": report.covered.iter().map(Handler::key).collect::<Vec<_>>(),
    })
}
