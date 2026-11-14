//! As ferramentas do MCP que fecham o ciclo "mudar, conferir, testar, medir" (2b da proposta de
//! CI com LLM): `profile` (o Profiler pelo canal de desenvolvimento, resumido), `recording` (a
//! gravação de uso mais recente, resumida) e `coverage` (a cobertura de handlers). Rodam na
//! thread da conexão: só a consulta ao estado da IDE vai à thread da interface.

use std::path::PathBuf;
use std::sync::Mutex;

use aylis_core::Poster;
use serde_json::{Value, json};

use crate::mcp::on_ui;
use crate::profiler::Recording;

/// O que o `profile` mediu desde o `start` (somado a cada `take`).
static MEASURED: Mutex<Option<Recording>> = Mutex::new(None);

fn arg<'a>(args: &'a Value, key: &str) -> &'a str {
    args.get(key).and_then(Value::as_str).unwrap_or_default()
}

/// O canal com o app em execução.
fn link(poster: &Poster) -> Result<std::sync::Arc<crate::devlink::DevServer>, String> {
    on_ui(poster, |_, state, _| state.borrow().dev.clone().filter(|d| d.connected()))?
        .ok_or_else(|| "nenhum app em execução conectado (use run_without_debug e espere abrir; o app precisa ter sido compilado pela IDE, com a feature `dev`)".to_string())
}

/// `profile`: `start` liga a medição no app (do zero), `take` soma o que o app mediu e devolve
/// o resumo, `stop` pega o resto, desliga e resume.
pub fn profile(args: &Value, poster: &Poster) -> Result<Value, String> {
    let dev = link(poster)?;
    let app = args.get("app");
    let top = args.get("top").and_then(Value::as_u64).unwrap_or(10).clamp(1, 100) as usize;
    match arg(args, "action") {
        "start" => {
            dev.call_app(app, json!({ "cmd": "profile", "on": true }))?;
            *MEASURED.lock().map_err(|_| "profile ocupado")? = Some(Recording::default());
            Ok(json!({ "measuring": true, "note": "medindo: use o app (ou app_command) e chame profile com action take (o resumo até agora) ou stop (para e resume)" }))
        }
        "" | "take" | "stop" => {
            let stop = arg(args, "action") == "stop";
            let answer = dev.call_app(app, json!({ "cmd": "profile_take" }))?;
            if stop {
                dev.call_app(app, json!({ "cmd": "profile", "on": false }))?;
            }
            let mut measured = MEASURED.lock().map_err(|_| "profile ocupado")?;
            let rec = measured.get_or_insert_with(Recording::default);
            rec.add(&answer);
            let mut summary = rec.summary(top);
            summary["measuring"] = json!(!stop);
            if rec.blocks.is_empty() {
                summary["note"] = json!("nada medido: chame profile com action start antes e use o app");
            }
            let file = arg(args, "trace_file");
            if !file.is_empty() {
                crate::profiler::save_trace(rec, &PathBuf::from(file))?;
                summary["trace_file"] = json!(file);
            }
            Ok(summary)
        }
        other => Err(format!("action desconhecida: {other} (start, take ou stop)")),
    }
}

/// `recording`: a gravação mais recente do app do projeto (ou da IDE, ou a dada), resumida.
pub fn recording(args: &Value, poster: &Poster) -> Result<Value, String> {
    let last = args.get("last").and_then(Value::as_u64).unwrap_or(20).min(500) as usize;
    let file = arg(args, "file");
    let path = if !file.is_empty() {
        PathBuf::from(file)
    } else if arg(args, "source") == "ide" {
        let folder = crate::recordings::folder();
        crate::recordings::newest(std::slice::from_ref(&folder)).ok_or_else(|| format!("nenhuma gravação da IDE em {} (Ajuda → Gravar o uso da IDE)", folder.display()))?
    } else {
        let project = on_ui(poster, |_, state, _| state.borrow().project.clone())?.ok_or("abra um projeto antes (open_project), ou passe `file`")?;
        let folders = crate::recordings::app_folders(&project);
        crate::recordings::newest(&folders).ok_or_else(|| {
            let places: Vec<String> = folders.iter().map(|f| f.display().to_string()).collect();
            format!("nenhuma gravação do app: ele grava com um EventRecorder num form (procurei em {})", places.join(", "))
        })?
    };
    let rec = aylis_controls::Recording::open(&path)?;
    let mut summary = crate::recordings::summary_json(&rec, last);
    summary["file"] = json!(path.display().to_string());
    Ok(summary)
}

/// `coverage`: os handlers dos forms que nenhum uso ou teste do app disparou.
pub fn coverage(args: &Value, poster: &Poster) -> Result<Value, String> {
    let project = on_ui(poster, |_, state, _| {
        // O que os apps avisaram e ainda não foi para o disco.
        crate::coverage::collect(state);
        state.borrow().project.clone()
    })?
    .ok_or("abra um projeto antes (open_project)")?;
    if args.get("clear").and_then(Value::as_bool).unwrap_or(false) {
        aylis_project::coverage::clear(&project)?;
    }
    let report = aylis_project::coverage::project_report(&project);
    let mut out = aylis_project::coverage::report_json(&project, &report);
    if report.since_ms == 0 && report.total() > 0 {
        out["note"] = json!("nada rodou ainda: execute o app pela IDE (run_without_debug) e use-o, ou comande pelo app_command; cada handler que disparar sai da lista");
    }
    Ok(out)
}
