//! Quanto custa o protocolo (item 13): uma tela comum (comandos `set`/`get` e eventos) e uma
//! carga grande (100 mil registros num ClientDataSet), medindo o JSON dos dois lados (montar o
//! texto, ler, executar). Roda em release e grava `medicoes/protocolo.json`:
//!
//! ```text
//! cargo test -p aylis-remote --release --test protocol_cost -- --ignored --nocapture
//! ```

use std::time::Instant;

use aylis_core::Ui;
use aylis_remote::*;
use serde_json::{Value, json};

fn ui() -> Ui {
    let mut ui = Ui::new();
    register_all(&mut ui);
    ui
}

/// Uma ida e volta como no `aylis-host`: o backend monta o texto, a tela lê, executa e responde
/// em texto, e o backend lê a resposta.
fn round_trip(ui: &mut Ui, command: &Value) -> usize {
    let line = command.to_string();
    let message: Value = serde_json::from_str(&line).unwrap();
    let reply = handle_message(ui, &message).map(|r| r.to_string()).unwrap_or_default();
    if !reply.is_empty() {
        let _: Value = serde_json::from_str(&reply).unwrap();
    }
    line.len() + reply.len()
}

#[test]
#[ignore = "medição: rode em release (veja o cabeçalho)"]
fn protocol_cost() {
    let mut ui = ui();
    run(&mut ui, json!({ "cmd": "hello", "protocol": PROTOCOL_VERSION }));
    let mut children = Vec::new();
    children.push(json!({ "class": "Button", "name": "btnOk", "align": "top", "height": 30, "on_click": "ok_click" }));
    for i in 0..40 {
        children.push(json!({ "class": "Edit", "name": format!("edt{i}"), "align": "top", "height": 28 }));
        children.push(json!({ "class": "Label", "name": format!("lbl{i}"), "align": "top", "height": 20, "text": "Campo" }));
    }
    run(&mut ui, json!({ "cmd": "load", "layout": { "root": { "class": "Form", "name": "frmTela", "width": 800, "height": 2400, "children": children } } }));

    // A tela comum: o que um handler faz (ler um campo, gravar outro), 20 mil vezes.
    let n = 20_000;
    let started = Instant::now();
    let mut bytes = 0;
    for i in 0..n {
        let k = i % 40;
        bytes += round_trip(&mut ui, &json!({ "id": i * 2, "cmd": "get", "target": format!("edt{k}"), "prop": "text" }));
        bytes += round_trip(&mut ui, &json!({ "cmd": "set", "target": format!("lbl{k}"), "props": { "text": format!("Valor {i}") } }));
    }
    let common = started.elapsed().as_secs_f64() * 1e6 / (2 * n) as f64;
    let common_bytes = bytes / (2 * n);

    // Um evento indo ao backend (o texto que o receptor monta).
    let events = std::rc::Rc::new(std::cell::RefCell::new(0usize));
    let e = events.clone();
    forward_events(&mut ui, move |_, event| *e.borrow_mut() += event.to_string().len());
    let started = Instant::now();
    for _ in 0..n {
        run(&mut ui, json!({ "cmd": "click", "target": "btnOk" }));
    }
    let event = started.elapsed().as_secs_f64() * 1e6 / n as f64;
    let event_bytes = *events.borrow() / n;

    // A carga grande: 100 mil registros (como os municípios e distritos do IBGE).
    run(&mut ui, json!({ "cmd": "load", "layout": { "root": { "class": "DataModule", "name": "dm", "children": [
        { "class": "ClientDataSet", "name": "cds", "children": [
            { "class": "Field", "name": "id", "type": "integer" },
            { "class": "Field", "name": "nome", "type": "string" },
            { "class": "Field", "name": "uf", "type": "string" },
            { "class": "Field", "name": "populacao", "type": "integer" },
            { "class": "Field", "name": "area", "type": "float" }
        ] }
    ] } } }));
    let rows: Vec<Value> = (0..100_000).map(|i| json!({ "id": i, "nome": format!("Município {i}"), "uf": "SP", "populacao": 10_000 + i, "area": 123.45 + i as f64 })).collect();
    let started = Instant::now();
    let text = json!({ "id": 1, "cmd": "data", "target": "cds", "op": "load", "rows": rows }).to_string();
    let build = started.elapsed();
    let started = Instant::now();
    let message: Value = serde_json::from_str(&text).unwrap();
    let parse = started.elapsed();
    let started = Instant::now();
    let reply = handle_message(&mut ui, &message).unwrap();
    let load = started.elapsed();
    assert!(reply.get("error").is_none(), "{reply}");
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 100_000);

    // A mesma carga no formato colunar.
    let columns = json!([
        (0..100_000).collect::<Vec<i64>>(),
        (0..100_000).map(|i| format!("Município {i}")).collect::<Vec<_>>(),
        vec!["SP"; 100_000],
        (0..100_000).map(|i| 10_000 + i).collect::<Vec<i64>>(),
        (0..100_000).map(|i| 123.45 + i as f64).collect::<Vec<f64>>(),
    ]);
    let started = Instant::now();
    let columnar = json!({ "id": 2, "cmd": "data", "target": "cds", "op": "load_columns", "fields": ["id", "nome", "uf", "populacao", "area"], "columns": columns }).to_string();
    let build_c = started.elapsed();
    let started = Instant::now();
    let message: Value = serde_json::from_str(&columnar).unwrap();
    let parse_c = started.elapsed();
    let started = Instant::now();
    let reply = handle_message(&mut ui, &message).unwrap();
    let load_c = started.elapsed();
    assert!(reply.get("error").is_none(), "{reply}");
    assert_eq!(run(&mut ui, json!({ "cmd": "data", "target": "cds", "op": "info" }))["count"], 100_000);

    let report = json!({
        "descricao": "Custo do protocolo JSON (item 13), em release, nesta máquina: uma tela comum (get e set de propriedades, eventos) e uma carga de 100 mil registros com 5 campos. Sem a ida e volta do processo (stdio ou WebSocket), que soma o tempo do sistema.",
        "tela_comum": { "comando_us": round(common), "bytes_por_comando": common_bytes, "evento_us": round(event), "bytes_por_evento": event_bytes },
        "carga_100_mil": { "bytes": text.len(), "montar_texto_ms": round(build.as_secs_f64() * 1e3), "ler_json_ms": round(parse.as_secs_f64() * 1e3), "carregar_ms": round(load.as_secs_f64() * 1e3) },
        "carga_100_mil_colunar": { "bytes": columnar.len(), "montar_texto_ms": round(build_c.as_secs_f64() * 1e3), "ler_json_ms": round(parse_c.as_secs_f64() * 1e3), "carregar_ms": round(load_c.as_secs_f64() * 1e3) },
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../../medicoes/protocolo.json");
    std::fs::write(out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
}

fn run(ui: &mut Ui, command: Value) -> Value {
    execute(ui, &command).unwrap_or_else(|e| panic!("{command}: {e}"))
}

fn round(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}
