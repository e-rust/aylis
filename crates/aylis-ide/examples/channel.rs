//! Faz o papel da IDE no canal de desenvolvimento, para testar o app de verdade (desktop,
//! Android, navegador): abre o canal do projeto (o endereço, a chave da IDE e o segredo do
//! projeto, os mesmos que a IDE embute ao compilar), espera o app conectar, lê a árvore,
//! liga o rastro de eventos e roda os passos pedidos, na ordem:
//!
//! - `--reload <form.json> <nome-do-form>`: empurra o form (hot reload);
//! - `--click <frmX.btnY>`: clica;
//! - `--cmd '<json>'`: um comando do protocolo (`{"cmd": "media", "target": ..., "op": "play"}`);
//! - `--wait <ms>`: espera;
//! - `--snapshot <form> <arquivo.png>`: grava a tela do form;
//! - `--close`: fecha o app.
//!
//! ```text
//! cargo run -p aylis-ide --example channel -- <projeto.aylisproj> [passos...]
//! ```

use std::time::Duration;

use serde_json::{Value, json};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let project = aylis_project::Project::open(args.first().expect("o .aylisproj")).expect("projeto");
    let server = aylis_ide::devlink::for_project(None, &project).expect("canal");
    server.set_trace(true);
    println!("canal: {}", server.url());
    if !server.wait(Duration::from_secs(300)) {
        println!("o app não conectou");
        return;
    }
    println!("app conectado: {:?}", server.apps());
    match server.call(json!({ "cmd": "tree" })) {
        Ok(tree) => println!("árvore: {}", tree.to_string().chars().take(200).collect::<String>()),
        Err(e) => println!("árvore: erro {e}"),
    }
    let mut rest = args[1..].iter();
    while let Some(step) = rest.next() {
        let mut next = || rest.next().cloned().unwrap_or_default();
        match step.as_str() {
            "--reload" => {
                let (file, name) = (next(), next());
                let text = std::fs::read_to_string(&file).expect("form");
                println!("reload: {:?}", server.reload("form", &name, &text));
            }
            "--click" => println!("click: {:?}", server.call(json!({ "cmd": "click", "target": next() }))),
            "--cmd" => {
                let command: Value = serde_json::from_str(&next()).expect("--cmd: JSON");
                let answer = server.call(command.clone()).map(|v| v.to_string().chars().take(200).collect::<String>());
                println!("{}: {answer:?}", command["cmd"]);
            }
            "--wait" => std::thread::sleep(Duration::from_millis(next().parse().unwrap_or(1000))),
            "--snapshot" => {
                let (form, file) = (next(), next());
                match server.call(json!({ "cmd": "snapshot", "form": form })) {
                    Ok(Value::String(b64)) => {
                        std::fs::write(&file, unbase64(&b64)).expect("png");
                        println!("snapshot: {file}");
                    }
                    other => println!("snapshot: {other:?}"),
                }
            }
            "--close" => println!("close: {:?}", server.call(json!({ "cmd": "close" }))),
            other => println!("passo desconhecido: {other}"),
        }
        for line in server.take_traces() {
            println!("rastro: {line}");
        }
    }
}

fn unbase64(text: &str) -> Vec<u8> {
    const ABC: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let (mut out, mut acc, mut bits) = (Vec::new(), 0u32, 0);
    for c in text.bytes().filter_map(|c| ABC.iter().position(|&a| a == c)) {
        acc = (acc << 6) | c as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    out
}
