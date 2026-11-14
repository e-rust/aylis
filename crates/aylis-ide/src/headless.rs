//! `aylis-ide --headless --mcp-stdio`: a IDE sem janela, com o MCP no stdio (item 38, 1d). Um
//! agente (ou um roteiro fixo, como o `aylis-tool ide-script`) cria um projeto, põe componentes,
//! gera, compila, executa e comanda o app, testando a IDE de ponta a ponta como um usuário, na
//! CI inclusive.
//!
//! As mensagens são as do servidor MCP da IDE ([`crate::mcp`]), sem cópia: esta IDE liga o
//! servidor só para si (em `127.0.0.1`, numa porta livre, com um token novo que não sai do
//! processo) e cada linha do stdin vai para ele; a resposta volta no stdout, uma por linha. Ela
//! não entra no registro das IDEs abertas (a ponte `aylis-tool mcp` e o duplo clique não a acham)
//! e não abre a tela inicial, a recuperação nem a consulta de versão nova.
//!
//! O laço da interface é o daqui: roda o que as chamadas do MCP agendam, os timers e a bomba da
//! IDE ([`crate::actions::pump_with`]: as compilações, o app em execução), e termina quando o
//! stdin fecha e não há chamada em andamento.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::task::Wake;

use crate::app::Ide;

/// Acorda o laço (uma tarefa da IDE andou).
struct Flag(Arc<AtomicBool>);

impl Wake for Flag {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

/// Roda a IDE sem janela, com o MCP no stdin e no stdout, até o stdin fechar.
pub fn run_mcp_stdio(mut ide: Ide) -> Result<(), String> {
    // O servidor de sempre (ligado nas Opções) fica desligado: esta IDE só fala pelo stdio.
    if let Some(server) = ide.state.borrow_mut().mcp.take() {
        server.stop();
    }
    let renderer = aylis_skia::SkiaRenderer::new();
    ide.ui.services_mut().text = Some(Box::new(renderer.text_service()));
    let mut token = [0u8; 32];
    aylis_tools::random_bytes(&mut token)?;
    let token: String = token.iter().map(|b| format!("{b:02x}")).collect();
    let server = crate::mcp::start(ide.ui.poster(), 0, token.clone())?;
    let access = aylis_tools::mcp::Access { port: server.port(), token };

    // O stdin numa thread: cada linha vai ao servidor (que agenda a chamada nesta thread, a da
    // interface, e espera a resposta); a resposta sai no stdout.
    let closed = Arc::new(AtomicBool::new(false));
    let busy = Arc::new(AtomicUsize::new(0));
    let woke = Arc::new(AtomicBool::new(true));
    {
        let (closed, busy, woke) = (closed.clone(), busy.clone(), woke.clone());
        std::thread::spawn(move || {
            use std::io::{BufRead, Write};
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                let Ok(line) = line else { break };
                if line.trim().is_empty() {
                    continue;
                }
                busy.fetch_add(1, Ordering::AcqRel);
                woke.store(true, Ordering::Release);
                let reply = aylis_tools::mcp::post(&access, &line);
                busy.fetch_sub(1, Ordering::AcqRel);
                let mut out = std::io::stdout().lock();
                match reply {
                    Ok(Some(body)) => {
                        let _ = writeln!(out, "{}", body.trim_end());
                    }
                    Ok(None) => {}
                    // Um pedido (com `id`) sempre tem resposta, mesmo de erro.
                    Err(e) => {
                        if let Some(id) = serde_json::from_str::<serde_json::Value>(&line).ok().and_then(|v| v.get("id").cloned()) {
                            let _ = writeln!(out, "{}", serde_json::json!({ "jsonrpc": "2.0", "id": id, "error": { "code": -32000, "message": e } }));
                        }
                    }
                }
                let _ = out.flush();
            }
            closed.store(true, Ordering::Release);
        });
    }

    let waker = std::task::Waker::from(Arc::new(Flag(woke.clone())));
    let mut pump = crate::actions::pump_with(ide.state.clone(), ide.ids.clone(), waker);
    let started = std::time::Instant::now();
    ide.ui.layout();
    loop {
        if closed.load(Ordering::Acquire) && busy.load(Ordering::Acquire) == 0 {
            break;
        }
        ide.ui.set_time(started.elapsed().as_secs_f64());
        let ran = ide.ui.process_posted();
        ide.ui.run_timers();
        pump(&mut ide.ui);
        ide.ui.layout();
        // Parado, dorme mais; com trabalho (uma chamada, uma tarefa que andou), volta logo.
        let active = ran > 0 || woke.swap(false, Ordering::AcqRel) || busy.load(Ordering::Acquire) > 0;
        std::thread::sleep(std::time::Duration::from_millis(if active { 2 } else { 20 }));
    }
    server.stop();
    Ok(())
}
