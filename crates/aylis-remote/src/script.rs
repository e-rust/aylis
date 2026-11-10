//! Roteiros de teste: uma sequência de ações de usuário (cliques, teclas, arrastos) tocada
//! num form, e o estado final de alguns componentes. O mesmo roteiro roda num app em Rust
//! (no próprio processo) e num app de outra linguagem (pelo aylis-host, que espera a lógica
//! responder a cada passo): o estado tem de sair igual.
//!
//! ```json
//! {
//!   "steps": [
//!     {"click": "btn7"},
//!     {"key": "ctrl+h"},
//!     {"item": "lvHistorico", "text": "42"},
//!     {"drag": "pcDesenho", "from": [10, 10], "to": [80, 40], "button": "right"},
//!     {"event": "vpVideo", "name": "on_end"},
//!     {"event": "dlgAbrir", "name": "on_select", "text": "foto.png"},
//!     {"set": "tbVolume", "prop": "value", "value": 30},
//!     {"locate": "cdsEstados", "field": "sigla", "value": "SP"},
//!     {"close": "frmProgresso"},
//!     {"wait": 500}
//!   ],
//!   "state": [
//!     {"component": "lblVisor", "prop": "text"},
//!     {"component": "pcDesenho", "invoke": "checksum"},
//!     {"component": "cdsEstados", "data": "record"},
//!     {"component": "cdsMaiores", "data": "rows"}
//!   ]
//! }
//! ```
//!
//! Ligado pela linha que o teste escreve no stdin do app de desenvolvimento ([`start_line`]):
//! `{"script": {...}, "output": "estado.json"}` toca o roteiro sem abrir janela e grava o
//! estado (o aylis-host grava também a tela em PNG ao lado, com o mesmo nome). Sem variável de
//! ambiente nem parâmetro; o app publicado (sem `dev`) não lê o stdin.

use aylis_core::{ComponentId, EventArgs, Key, KeyEvent, Modifiers, PointF, PointerButton, PointerEvent, Ui};
use serde_json::{Value, json};

/// Toca os passos no form. `pump` roda depois de cada passo: o aylis-host espera a lógica
/// da outra linguagem terminar o que o passo disparou.
pub fn play(ui: &mut Ui, form: ComponentId, script: &Value, pump: &mut dyn FnMut(&mut Ui)) -> Result<(), String> {
    ui.layout();
    pump(ui);
    let steps = script.get("steps").and_then(Value::as_array).ok_or("o roteiro não tem \"steps\"")?;
    for (n, step_value) in steps.iter().enumerate() {
        step(ui, form, step_value).map_err(|e| format!("passo {}: {e} ({step_value})", n + 1))?;
        ui.process_events();
        ui.layout();
        pump(ui);
        ui.layout();
    }
    Ok(())
}

fn find(ui: &Ui, name: &str) -> Result<ComponentId, String> {
    crate::resolve(ui, &Value::String(name.into()))
}

fn step(ui: &mut Ui, form: ComponentId, step: &Value) -> Result<(), String> {
    let s = |k: &str| step.get(k).and_then(Value::as_str);
    if let Some(name) = s("click") {
        let id = find(ui, name)?;
        // Desabilitado não clica (como na tela).
        if ui.control(id).is_none_or(|c| c.enabled()) {
            ui.click(id);
        }
    } else if let Some(name) = s("item") {
        let id = find(ui, name)?;
        let text = s("text").ok_or("falta \"text\"")?;
        if !ui.click_item(id, text) {
            return Err(format!("{name} não tem o item '{text}'"));
        }
    } else if let Some(key) = s("key") {
        let event = parse_key(key)?;
        ui.key_down(form, event);
        if let Key::Char(c) = event.key {
            if !event.modifiers.ctrl && !event.modifiers.alt {
                ui.text_input(form, &c.to_string());
            }
        }
        ui.key_up(form, event);
    } else if let Some(name) = s("drag") {
        let id = find(ui, name)?;
        let origin = ui.absolute_rect(id).ok_or("sem posição")?;
        let point = |k: &str| -> Result<PointF, String> {
            let a = step.get(k).and_then(Value::as_array).ok_or(format!("falta \"{k}\" [x, y]"))?;
            let v = |i: usize| a.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
            Ok(PointF::new(origin.x + v(0), origin.y + v(1)))
        };
        let (from, to) = (point("from")?, point("to")?);
        let button = if s("button") == Some("right") { PointerButton::Secondary } else { PointerButton::Primary };
        let ev = |p: PointF| PointerEvent { position: p, button, ..Default::default() };
        ui.pointer_down(form, ev(from));
        // Uns pontos no caminho, como o mouse de verdade.
        for i in 1..=4 {
            let t = i as f32 / 4.0;
            ui.pointer_move(form, ev(PointF::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t)));
        }
        ui.pointer_up(form, ev(to));
    } else if let Some(name) = s("event") {
        let id = find(ui, name)?;
        let event = s("name").ok_or("falta \"name\"")?;
        // Com "text", o evento leva o texto (o arquivo escolhido num diálogo, por exemplo).
        let args = s("text").map_or(EventArgs::None, |t| EventArgs::Text(t.into()));
        ui.fire(id, event, &args);
    } else if let Some(ms) = step.get("wait").and_then(Value::as_f64) {
        // O tempo passa de verdade (a mídia toca) e o relógio da tela também: os timers
        // vencidos disparam, como na janela.
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        let now = ui.time() + ms / 1000.0;
        ui.set_time(now);
        ui.run_timers();
    } else if let Some(name) = s("locate") {
        // O registro escolhido na grade (o `Locate` do Delphi): o dataset vai até ele.
        let id = find(ui, name)?;
        let field = s("field").ok_or("falta \"field\"")?;
        let json = step.get("value").cloned().unwrap_or(Value::Null);
        let mut ds = aylis_data::data_set(ui, id);
        let value = aylis_data::json::json_to_value(&json, ds.view().field(field).map_err(|e| e.message)?)?;
        if !ds.locate(field, value).map_err(|e| e.message)? {
            return Err(format!("{name}: nenhum registro com {field} = {json}"));
        }
    } else if let Some(name) = s("close") {
        // O X da janela: passa pelo `on_close_query` do form, que pode recusar.
        let id = find(ui, name)?;
        ui.close(id);
    } else if let Some(name) = s("set") {
        let id = find(ui, name)?;
        let prop = s("prop").ok_or("falta \"prop\"")?;
        let value = step.get("value").ok_or("falta \"value\"")?;
        let value = aylis_json::prop_from_json(ui, id, prop, value)?;
        ui.set_prop(id, prop, value).map_err(|e| e.to_string())?;
    } else {
        return Err("passo desconhecido".into());
    }
    Ok(())
}

/// `"Enter"`, `"ctrl+h"`, `"shift+8"`, `"F9"`, `"7"`, `"+"`.
pub fn parse_key(text: &str) -> Result<KeyEvent, String> {
    let mut modifiers = Modifiers::default();
    let mut rest = text;
    loop {
        let lower = rest.to_lowercase();
        if let Some(r) = lower.strip_prefix("ctrl+") {
            modifiers.ctrl = true;
            rest = &rest[rest.len() - r.len()..];
        } else if let Some(r) = lower.strip_prefix("shift+") {
            modifiers.shift = true;
            rest = &rest[rest.len() - r.len()..];
        } else if let Some(r) = lower.strip_prefix("alt+") {
            modifiers.alt = true;
            rest = &rest[rest.len() - r.len()..];
        } else {
            break;
        }
    }
    let key = match rest.to_lowercase().as_str() {
        "enter" => Key::Enter,
        "escape" | "esc" => Key::Escape,
        "backspace" => Key::Backspace,
        "delete" | "del" => Key::Delete,
        "tab" => Key::Tab,
        "space" => Key::Space,
        "left" => Key::Left,
        "right" => Key::Right,
        "up" => Key::Up,
        "down" => Key::Down,
        "home" => Key::Home,
        "end" => Key::End,
        f if f.len() >= 2 && f.starts_with('f') && f[1..].parse::<u8>().is_ok() => Key::F(f[1..].parse().unwrap_or(1)),
        other => {
            let mut chars = other.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Key::Char(c),
                _ => return Err(format!("tecla desconhecida: {text}")),
            }
        }
    };
    Ok(KeyEvent { key, modifiers, repeat: false })
}

/// O estado pedido no roteiro (`"state"`): uma lista com o valor de cada consulta.
pub fn state(ui: &mut Ui, script: &Value) -> Value {
    let Some(list) = script.get("state").and_then(Value::as_array) else { return json!([]) };
    let out: Vec<Value> = list
        .iter()
        .map(|q| {
            let name = q.get("component").and_then(Value::as_str).unwrap_or_default();
            let value = match find(ui, name) {
                Err(e) => json!({ "error": e }),
                Ok(id) => match (q.get("prop").and_then(Value::as_str), q.get("invoke").and_then(Value::as_str)) {
                    _ if q.get("data").is_some() => data_state(ui, id, q.get("data").and_then(Value::as_str).unwrap_or_default()),
                    (Some(prop), _) => ui.get_prop(id, prop).map(|v| aylis_json::prop_to_json(ui, &v)).unwrap_or_else(|e| json!({ "error": e.to_string() })),
                    (None, Some(method)) => ui.invoke(id, method, &[]).map(|v| aylis_json::prop_to_json(ui, &v)).unwrap_or_else(|e| json!({ "error": e })),
                    _ => json!({ "error": "informe \"prop\" ou \"invoke\"" }),
                },
            };
            let mut entry = q.clone();
            if let Some(o) = entry.as_object_mut() {
                o.insert("value".into(), value);
            }
            entry
        })
        .collect();
    Value::Array(out)
}

/// Um dataset no estado: `"record"` (o registro atual) ou `"rows"` (todos, na ordem).
fn data_state(ui: &Ui, id: ComponentId, what: &str) -> Value {
    if ui.get::<aylis_data::ClientDataSet>(id).is_none() {
        return json!({ "error": "não é um ClientDataSet" });
    }
    let v = aylis_data::view(ui, id);
    let names = v.field_names();
    let row = |i: Option<usize>| -> Value {
        let mut out = serde_json::Map::new();
        for name in &names {
            let value = match i {
                Some(i) => v.value_at(i, name),
                None => v.value(name),
            };
            out.insert(name.clone(), value.map(|x| aylis_data::json::value_to_json(&x)).unwrap_or(Value::Null));
        }
        Value::Object(out)
    };
    match what {
        "record" if v.record_count() > 0 => row(None),
        "record" => Value::Null,
        "rows" => Value::Array((0..v.record_count()).map(|i| row(Some(i))).collect()),
        other => json!({ "error": format!("\"data\": use \"record\" ou \"rows\" (veio \"{other}\")") }),
    }
}

/// A linha JSON que a IDE (ou o teste) escreve no stdin do app de desenvolvimento ao abrir:
/// `{"dev": {...alvo do canal}, "script": {...}, "output": "estado.json"}` (cada parte
/// opcional). Num terminal, sem stdin, com o stdin vazio (o F9 da IDE) ou sem nada em 2 s:
/// `None`, e o app abre normal.
#[cfg(not(target_arch = "wasm32"))]
pub fn start_line() -> Option<Value> {
    use std::io::IsTerminal;
    if std::io::stdin().is_terminal() {
        return None;
    }
    // Numa thread: um stdin aberto que nunca escreve não segura o app.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);
        let _ = tx.send(line);
    });
    let line = rx.recv_timeout(std::time::Duration::from_secs(2)).ok()?;
    serde_json::from_str(line.trim()).ok().filter(Value::is_object)
}

/// O roteiro e onde gravar o estado, da linha do stdin ([`start_line`]).
pub fn from_start(line: &Value) -> Option<(Value, Option<std::path::PathBuf>)> {
    let script = line.get("script").filter(|s| s.is_object())?.clone();
    Some((script, line.get("output").and_then(Value::as_str).filter(|p| !p.is_empty()).map(std::path::PathBuf::from)))
}

/// Grava o estado em `output` (ou no stdout, sem ele).
pub fn write_state(state: &Value, output: Option<&std::path::Path>) -> Result<(), String> {
    let text = serde_json::to_string_pretty(state).unwrap_or_default();
    match output {
        Some(path) => std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display())),
        None => {
            println!("{text}");
            Ok(())
        }
    }
}

/// Um app em Rust com o roteiro: toca, grava o estado e não abre janela.
pub fn run_in_process(ui: &mut Ui, form: ComponentId, script: &Value, output: Option<&std::path::Path>) -> Result<(), String> {
    let _ = ui.show(form);
    ui.process_events();
    play(ui, form, script, &mut |_| {})?;
    let st = state(ui, script);
    write_state(&st, output)
}
