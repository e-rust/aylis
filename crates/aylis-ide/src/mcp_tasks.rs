//! As chamadas do MCP que passam do prazo (M12 do Tabernáculo). A ferramenta roda na thread da
//! interface, e a conexão esperava 60 s e respondia "a IDE não respondeu", sem dizer se a ação
//! aconteceu: agora, passado o prazo, a resposta é "em andamento" com o número da tarefa, a
//! chamada continua, e o resultado fica guardado até o `task_status` pedir (ou o `status`
//! listar). Só a conexão espera aqui; a interface não.

use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

/// Quanto uma chamada espera antes de responder "em andamento": dentro do prazo de um cliente
/// MCP (como o `max_wait` do `build`).
pub const DEADLINE: Duration = Duration::from_secs(45);

/// Quantos resultados não lidos ficam guardados (os mais antigos saem).
const KEEP: usize = 20;

struct Task {
    id: u64,
    tool: String,
    started: Instant,
    /// O resultado do MCP (`content`, `isError`), quando terminou.
    result: Option<Value>,
}

static TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
static NEXT: AtomicU64 = AtomicU64::new(1);

fn text(text: impl Into<String>, is_error: bool) -> Value {
    json!({ "content": [{ "type": "text", "text": text.into() }], "isError": is_error })
}

/// A resposta de uma chamada que ainda está rodando.
fn in_progress(id: u64, tool: &str, seconds: u64) -> Value {
    let body = json!({
        "in_progress": true,
        "task": id,
        "tool": tool,
        "seconds": seconds,
        "note": format!("a IDE ainda está fazendo o {tool} depois de {seconds} s e a chamada continua: chame task_status com {{\"task\": {id}}} para esperar o fim e ver o resultado (status também lista as tarefas)")
    });
    text(body.to_string(), false)
}

/// Espera o resultado até `deadline`; passou, guarda a tarefa e responde "em andamento" (o
/// resultado chega depois, por outra thread). O canal fechado sem resposta: a IDE não executou.
pub fn wait_or_defer(rx: Receiver<Value>, tool: &str, deadline: Duration) -> Value {
    match rx.recv_timeout(deadline) {
        Ok(result) => result,
        Err(RecvTimeoutError::Disconnected) => text("a IDE não executou a chamada (sem o estado da IDE nesta janela)", true),
        Err(RecvTimeoutError::Timeout) => {
            let id = NEXT.fetch_add(1, Ordering::Relaxed);
            if let Ok(mut tasks) = TASKS.lock() {
                tasks.push(Task { id, tool: tool.to_string(), started: Instant::now().checked_sub(deadline).unwrap_or_else(Instant::now), result: None });
            }
            let spawned = std::thread::Builder::new().name("aylis-ide-mcp-task".into()).spawn(move || {
                let result = rx.recv().unwrap_or_else(|_| text("a IDE não terminou a chamada (fechou antes)", true));
                finish(id, result);
            });
            if spawned.is_err() {
                return text("a IDE não respondeu no prazo, e não consegui acompanhar a chamada", true);
            }
            in_progress(id, tool, deadline.as_secs())
        }
    }
}

fn finish(id: u64, result: Value) {
    let Ok(mut tasks) = TASKS.lock() else { return };
    if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
        t.result = Some(result);
    }
    // Os terminados que ninguém leu: só os últimos.
    let done = tasks.iter().filter(|t| t.result.is_some()).count();
    if done > KEEP {
        let mut extra = done - KEEP;
        tasks.retain(|t| {
            let drop = extra > 0 && t.result.is_some();
            if drop {
                extra -= 1;
            }
            !drop
        });
    }
}

/// As tarefas guardadas: as em andamento (há quanto tempo) e as terminadas não lidas.
pub fn list() -> Value {
    let tasks = TASKS.lock().map(|t| t.iter().map(|t| json!({ "task": t.id, "tool": t.tool, "seconds": t.started.elapsed().as_secs(), "done": t.result.is_some() })).collect::<Vec<_>>()).unwrap_or_default();
    Value::Array(tasks)
}

/// `task_status`: o resultado da tarefa `id`, esperando até `max_wait` (o resultado lido sai da
/// lista); sem `id`, a lista.
pub fn status(id: Option<u64>, max_wait: Duration) -> Value {
    let Some(id) = id else {
        return text(json!({ "tasks": list() }).to_string(), false);
    };
    let until = Instant::now() + max_wait;
    loop {
        let found = {
            let Ok(mut tasks) = TASKS.lock() else { return text("as tarefas estão indisponíveis", true) };
            match tasks.iter().position(|t| t.id == id) {
                None => return text(format!("não há a tarefa {id} (já foi lida, ou é de outra IDE): status lista as que há"), true),
                Some(i) if tasks[i].result.is_some() => Ok(tasks.remove(i).result.unwrap_or(Value::Null)),
                Some(i) => Err((tasks[i].tool.clone(), tasks[i].started.elapsed().as_secs())),
            }
        };
        match found {
            Ok(result) => return result,
            Err((tool, seconds)) if Instant::now() >= until => return in_progress(id, &tool, seconds),
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A chamada que passa do prazo responde "em andamento" com o número; o `task_status` espera
    /// e traz o resultado, uma vez só.
    #[test]
    fn a_slow_call_answers_in_progress_and_the_status_brings_the_result() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(300));
            let _ = tx.send(text("feito", false));
        });
        let first = wait_or_defer(rx, "generate_code", Duration::from_millis(20));
        let body: Value = serde_json::from_str(first["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(body["in_progress"], true);
        assert_eq!(body["tool"], "generate_code");
        let id = body["task"].as_u64().unwrap();
        assert!(list().as_array().unwrap().iter().any(|t| t["task"] == id), "a lista tem a tarefa");
        // Pouca espera: ainda em andamento.
        let again = status(Some(id), Duration::from_millis(1));
        assert!(again["content"][0]["text"].as_str().unwrap().contains("in_progress"));
        let done = status(Some(id), Duration::from_secs(5));
        assert_eq!(done["content"][0]["text"], "feito");
        assert_eq!(status(Some(id), Duration::from_millis(1))["isError"], true, "lido uma vez só");
    }

    #[test]
    fn a_quick_call_answers_directly() {
        let (tx, rx) = std::sync::mpsc::channel();
        tx.send(text("pronto", false)).unwrap();
        assert_eq!(wait_or_defer(rx, "save", Duration::from_secs(1))["content"][0]["text"], "pronto");
        let (tx, rx) = std::sync::mpsc::channel::<Value>();
        drop(tx);
        assert_eq!(wait_or_defer(rx, "save", Duration::from_secs(1))["isError"], true);
    }
}
