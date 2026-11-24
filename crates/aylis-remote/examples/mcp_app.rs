//! Um app sem janela com o `McpServer`: uma ferramenta com resposta em JSON (e
//! `output_schema`), um recurso fixo, um modelo de recurso, um prompt e a ponte stdio. Para
//! ver com um cliente de verdade:
//!
//! ```text
//! cargo run -p aylis-remote --example mcp_app
//! ```
//!
//! O exemplo escreve a linha do Claude Code (`claude mcp add ...`) e a entrada do Claude
//! Desktop (`claude_desktop_config.json`), que inicia este mesmo executável com `--mcp-stdio`:
//! a ponte acha o app aberto e repassa as mensagens. Cada chamada aparece no terminal (o
//! `on_result`, com o tempo). Ctrl+C fecha.

use std::time::Duration;

use aylis_core::{EventValue, Ui};
use aylis_remote::mcp::{McpResult, McpTool, stdio};
use serde_json::json;

fn main() {
    // A opção `mcp_stdio` do projeto: num app do Aylis, o `Application` liga sozinho.
    stdio::set_enabled(true);
    stdio::set_app_version("1.2.0");
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--mcp-stdio") {
        // O Claude Desktop iniciou: só a ponte, sem servidor.
        if let Err(e) = stdio::run_bridge(args.get(i + 1).map(String::as_str)) {
            eprintln!("{e}");
        }
        return;
    }

    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    let layout = json!({ "aylis": 1, "root": { "class": "Form", "name": "frmEstoque", "children": [{
        "class": "McpServer", "name": "mcpEstoque", "server_name": "estoque-exemplo",
        "instructions": "Estoque de exemplo do Aylis. Use saldo para consultar um produto.",
        "children": [
            { "class": "McpTool", "name": "toolSaldo", "tool_name": "saldo", "title": "Saldo de um produto",
              "description": "O saldo em estoque de um produto, pelo código.", "params": "codigo: string",
              "output_schema": "codigo: string, saldo: integer", "read_only": true, "idempotent": true },
            { "class": "McpResource", "name": "resLeia", "uri": "app://leia-me", "title": "Leia-me",
              "mime_type": "text/markdown", "text": "# Estoque de exemplo\n\nOs produtos têm códigos de A1 a A9." },
            { "class": "McpResource", "name": "resProduto", "uri": "app://produtos/{codigo}", "title": "Um produto",
              "mime_type": "application/json" },
            { "class": "McpPrompt", "name": "prmRelatorio", "prompt_name": "relatorio", "title": "Relatório de um produto",
              "arguments": "codigo: o código do produto", "text": "Consulte o saldo de {codigo} e escreva um parágrafo sobre ele." }
        ]
    }] } });
    aylis_json::load_value(&mut ui, &layout, None).expect("o form de exemplo");
    let server = ui.find("mcpEstoque").unwrap();
    ui.on(ui.find("toolSaldo").unwrap(), "on_execute", |ui, _, args| {
        let codigo = McpTool::arguments(args)["codigo"].as_str().unwrap_or_default().to_uppercase();
        let saldo = codigo.bytes().map(u64::from).sum::<u64>() % 100;
        McpTool::reply(ui, json!({ "codigo": codigo, "saldo": saldo }));
    });
    ui.on(ui.find("resProduto").unwrap(), "on_read", |ui, _, args| {
        let a = McpTool::arguments(args);
        McpTool::reply(ui, json!({ "codigo": a["params"]["codigo"], "descricao": "Produto de exemplo", "unidade": "un" }));
    });
    ui.on(server, "on_result", |_, _, args| {
        if let Some(r) = McpResult::from_args(args) {
            println!("{}({}) -> {} em {} ms{}", r.tool, r.arguments, r.result, r.elapsed_ms, if r.is_error { " (erro)" } else { "" });
        }
    });
    ui.set_prop(server, "active", true).expect("ligar");
    println!("Claude Code:\n  {}\n", ui.invoke(server, "client_command", &[]).unwrap().to_str().unwrap_or_default());
    println!("Claude Desktop (claude_desktop_config.json):\n{}\n", ui.invoke(server, "stdio_config", &[]).unwrap().to_str().unwrap_or_default());
    println!("Esperando o agente (Ctrl+C fecha)...");
    loop {
        ui.process_posted();
        std::thread::sleep(Duration::from_millis(10));
        ui.advance(0.01);
    }
}
