//! `aylis-tool recording-to-test arquivo.aylisrec`: uma gravação de uso (a que reproduziu o
//! problema) vira roteiro de teste do app (item 38, 1e). O roteiro é o de
//! [`aylis_remote::script`] (o mesmo que `aylis-tool script` toca sem janela) e a gravação, a
//! do `EventRecorder` ([`aylis_controls::Recording`]).
//!
//! ```text
//! aylis-tool recording-to-test sessao.aylisrec [--out roteiro.json] [--no-waits]
//! ```
//!
//! Como cada acontecimento vira passo:
//! - apertar e soltar o botão no mesmo lugar, num botão, caixa de marcar, item de menu (as
//!   classes de [`CLICK_CLASSES`]) com nome: `{"click": "frmX.btnSalvar"}`;
//! - nos outros (uma lista, uma grade, um desenho) ou num componente sem nome: o ponteiro de
//!   verdade na posição gravada, `{"drag": "frmX", "from": [x, y], "to": [x, y]}`;
//! - apertar, arrastar e soltar: `{"drag": "frmX", "from": [...], "to": [...]}` (com o botão);
//! - texto digitado num campo com nome: `{"set": "frmX.edtNome", "prop": "text", "value": ...}`,
//!   o texto que o campo ficou (e o campo entra no `state`, para conferir no fim);
//! - atalho e tecla especial (`Ctrl+S`, `Enter`, `F9`): `{"key": "ctrl+s"}`;
//! - o tempo entre um passo e outro, de meio segundo para cima: `{"wait": ms}` (até 5 s; os
//!   timers do app andam), menos com `--no-waits`.
//!
//! Os eventos gravados (`on_click`...) são consequência dos passos e não entram; o que não dá
//! para tocar (texto secreto, que a gravação guarda como `*`; form sem nome; tecla que o
//! roteiro não conhece) e os erros gravados vão para `notes`, que o roteiro ignora.

use aylis_controls::Recording;
use serde_json::{Value, json};

/// As classes em que o clique é pelo nome (`ui.click`): o que o clique faz não depende de
/// onde foi. Nas outras, o ponteiro vai na posição gravada.
pub const CLICK_CLASSES: &[&str] = &[
    "Button", "SpeedButton", "SplitButton", "GlowButton", "CheckBox", "RadioButton", "Switch", "MenuItem", "Chip", "Badge", "Icon", "Image", "Label", "Link", "Card", "Avatar",
];

/// O que a conversão leva em conta.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Os `wait` entre os passos (o tempo gravado).
    pub waits: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { waits: true }
    }
}

fn s<'a>(e: &'a Value, k: &str) -> &'a str {
    e[k].as_str().unwrap_or_default()
}

fn xy(e: &Value) -> [f64; 2] {
    [e["x"].as_f64().unwrap_or(0.0), e["y"].as_f64().unwrap_or(0.0)]
}

/// Um nome que o roteiro acha: o caminho com nomes (`frmX.btnSalvar`), sem `Classe[n]`.
fn named(path: &str) -> Option<&str> {
    (!path.is_empty() && !path.contains('/') && !path.contains(' ') && !path.contains('"')).then_some(path)
}

/// A tecla da gravação (`Ctrl+Shift+N`, `F(9)`, `PageUp`) como o roteiro escreve (`ctrl+shift+n`,
/// `f9`); `None` se o roteiro não a conhece.
fn key_step(chord: &str) -> Option<String> {
    let mut parts: Vec<String> = chord.split('+').map(str::to_string).collect();
    // `Ctrl++`: a tecla é o próprio `+`.
    if chord.ends_with("++") {
        parts.retain(|p| !p.is_empty());
        parts.push("+".into());
    }
    let key = parts.pop()?;
    let key = match key.strip_prefix("F(").and_then(|k| k.strip_suffix(')')) {
        Some(n) => format!("f{n}"),
        None => key.to_lowercase(),
    };
    let text = parts.iter().map(|m| m.to_lowercase()).chain(std::iter::once(key)).collect::<Vec<_>>().join("+");
    aylis_remote::script::parse_key(&text).ok().map(|_| text)
}

/// Uma tecla que digita (letra, número, símbolo sem Ctrl/Alt): vem também como `text`, que é
/// o que vira passo.
fn types_text(chord: &str) -> bool {
    let lower = chord.to_lowercase();
    !lower.contains("ctrl+") && !lower.contains("alt+") && !lower.contains("meta+") && chord.trim_start_matches("Shift+").chars().count() == 1
}

/// A conversão: o roteiro (`steps`, `state`, `notes` e de onde veio).
pub fn script(recording: &Recording, options: Options) -> Value {
    let mut steps: Vec<Value> = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    let mut state: Vec<String> = Vec::new();
    let mut last_t: Option<f64> = None;
    // O botão apertado: o acontecimento e se arrastou.
    let mut down: Option<(&Value, bool)> = None;
    // O texto digitado no campo em foco, ainda sem passo: (campo, valor).
    let mut typing: Option<(String, String)> = None;

    let push = |steps: &mut Vec<Value>, t: f64, step: Value, last_t: &mut Option<f64>| {
        if let (true, Some(before)) = (options.waits, *last_t) {
            let gap = t - before;
            if gap >= 0.5 {
                steps.push(json!({ "wait": (gap.min(5.0) * 1000.0).round() as u64 }));
            }
        }
        *last_t = Some(t);
        steps.push(step);
    };
    let flush = |typing: &mut Option<(String, String)>, steps: &mut Vec<Value>, state: &mut Vec<String>| {
        if let Some((field, value)) = typing.take() {
            steps.push(json!({ "set": field, "prop": "text", "value": value }));
            if !state.contains(&field) {
                state.push(field);
            }
        }
    };

    for e in &recording.entries {
        let t = e["t"].as_f64().unwrap_or(0.0);
        let at = format!("{t:.1} s");
        match s(e, "type") {
            "pointer_down" => {
                flush(&mut typing, &mut steps, &mut state);
                down = (!e["blocked"].as_bool().unwrap_or(false)).then_some((e, false));
            }
            "drag" => {
                if let Some((_, moved)) = &mut down {
                    *moved = true;
                }
            }
            "pointer_up" => {
                let Some((start, _)) = down.take() else { continue };
                let (from, to) = (xy(start), xy(e));
                let far = (from[0] - to[0]).abs() + (from[1] - to[1]).abs() > 4.0;
                let form = s(start, "form");
                let button = s(start, "button");
                let target = s(start, "target");
                if far {
                    match named(form) {
                        Some(form) => {
                            let mut step = json!({ "drag": form, "from": from, "to": to });
                            if button == "right" {
                                step["button"] = json!("right");
                            }
                            push(&mut steps, t, step, &mut last_t);
                        }
                        None => notes.push(format!("{at}: arrasto em form sem nome ({form}): pulado")),
                    }
                    continue;
                }
                let by_name = named(target).filter(|_| button == "left" && CLICK_CLASSES.contains(&s(start, "target_class")));
                if let Some(target) = by_name {
                    push(&mut steps, t, json!({ "click": target }), &mut last_t);
                } else if let Some(form) = named(form) {
                    let mut step = json!({ "drag": form, "from": from, "to": from });
                    if button == "right" {
                        step["button"] = json!("right");
                    }
                    push(&mut steps, t, step, &mut last_t);
                } else {
                    notes.push(format!("{at}: clique em form sem nome ({form}) em {target}: pulado"));
                }
                if start["clicks"].as_u64().unwrap_or(1) > 1 {
                    notes.push(format!("{at}: clique duplo em {target}: o roteiro toca um clique só"));
                }
            }
            "key" => {
                let chord = s(e, "key");
                if chord == "*" || types_text(chord) {
                    continue;
                }
                flush(&mut typing, &mut steps, &mut state);
                match key_step(chord) {
                    Some(key) => push(&mut steps, t, json!({ "key": key }), &mut last_t),
                    None => notes.push(format!("{at}: a tecla {chord} não tem passo no roteiro: pulada")),
                }
            }
            "text" => {
                let text = s(e, "text");
                let field = s(e, "focus");
                if e["focus_masked"].as_bool().unwrap_or(false) || (!text.is_empty() && text.chars().all(|c| c == '*')) {
                    notes.push(format!("{at}: texto secreto em {field} (gravado como *): preencha no roteiro"));
                    continue;
                }
                let Some(field) = named(field) else {
                    notes.push(format!("{at}: texto \"{text}\" num campo sem nome ({field}): pulado"));
                    continue;
                };
                // O texto do campo na gravação já pode ter o que foi digitado agora.
                let shown = s(e, "focus_text");
                let value = match &typing {
                    Some((f, v)) if f == field => format!("{v}{text}"),
                    _ if shown.ends_with(text) => shown.to_string(),
                    _ => format!("{shown}{text}"),
                };
                if typing.as_ref().is_some_and(|(f, _)| f != field) {
                    flush(&mut typing, &mut steps, &mut state);
                }
                if typing.is_none() {
                    // O wait vem antes do primeiro caractere.
                    if let (true, Some(before)) = (options.waits, last_t) {
                        if t - before >= 0.5 {
                            steps.push(json!({ "wait": ((t - before).min(5.0) * 1000.0).round() as u64 }));
                        }
                    }
                }
                last_t = Some(t);
                typing = Some((field.to_string(), value));
            }
            "error" => notes.push(format!("{at}: erro gravado: {}", s(e, "message"))),
            _ => {}
        }
    }
    flush(&mut typing, &mut steps, &mut state);
    let mut out = json!({
        "source": {
            "app": recording.header["app"].clone(),
            "start_ms": recording.header["start_ms"].clone(),
        },
        "steps": steps,
        "state": state.iter().map(|c| json!({ "component": c, "prop": "text" })).collect::<Vec<_>>(),
    });
    if !notes.is_empty() {
        out["notes"] = json!(notes);
    }
    out
}

/// Lê a gravação e grava o roteiro (em `out`, ou ao lado da gravação com `.test.json`).
pub fn convert(recording: &std::path::Path, out: Option<&std::path::Path>, options: Options) -> Result<std::path::PathBuf, String> {
    let rec = Recording::open(recording)?;
    let script = script(&rec, options);
    let out = out.map(std::path::Path::to_path_buf).unwrap_or_else(|| recording.with_extension("test.json"));
    let text = serde_json::to_string_pretty(&script).map_err(|e| e.to_string())? + "\n";
    std::fs::write(&out, text).map_err(|e| format!("{}: {e}", out.display()))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REC: &str = r#"{"type":"start","format":1,"app":"Cadastro","start_ms":1}
{"t":0.1,"type":"form_shown","form":"frmCliente"}
{"t":1.0,"type":"pointer_down","form":"frmCliente","x":20,"y":30,"target":"frmCliente.edtNome","target_class":"Edit","button":"left"}
{"t":1.1,"type":"pointer_up","form":"frmCliente","x":20,"y":30,"target":"frmCliente.edtNome","target_class":"Edit","button":"left"}
{"t":1.5,"type":"key","form":"frmCliente","key":"A","focus":"frmCliente.edtNome"}
{"t":1.5,"type":"text","form":"frmCliente","text":"a","focus":"frmCliente.edtNome","focus_class":"Edit","focus_text":"a"}
{"t":1.6,"type":"key","form":"frmCliente","key":"Shift+N","focus":"frmCliente.edtNome"}
{"t":1.6,"type":"text","form":"frmCliente","text":"N","focus":"frmCliente.edtNome","focus_class":"Edit","focus_text":"aN"}
{"t":1.7,"type":"key","form":"frmCliente","key":"*","focus":"frmCliente.edtSenha","focus_masked":true}
{"t":1.7,"type":"text","form":"frmCliente","text":"*","focus":"frmCliente.edtSenha","focus_masked":true}
{"t":3.0,"type":"key","form":"frmCliente","key":"Ctrl+S"}
{"t":3.1,"type":"key","form":"frmCliente","key":"F(9)"}
{"t":3.2,"type":"pointer_down","form":"frmCliente","x":200,"y":300,"target":"frmCliente.btnSalvar","target_class":"Button","target_text":"Salvar","button":"left"}
{"t":3.2,"type":"event","event":"on_click","component":"frmCliente.btnSalvar","handler":"btnSalvarClick"}
{"t":3.3,"type":"pointer_up","form":"frmCliente","x":200,"y":300,"button":"left"}
{"t":4.0,"type":"pointer_down","form":"frmCliente","x":10,"y":10,"target":"frmCliente.pbDesenho","target_class":"PaintBox","button":"right"}
{"t":4.1,"type":"drag","form":"frmCliente","x":40,"y":10}
{"t":4.2,"type":"pointer_up","form":"frmCliente","x":80,"y":40,"button":"right"}
{"t":4.3,"type":"pointer_down","form":"frmCliente","x":5,"y":6,"target":"frmCliente/Panel[0]/Button[1]","target_class":"Button","button":"left"}
{"t":4.3,"type":"pointer_up","form":"frmCliente","x":5,"y":6,"button":"left"}
{"t":4.4,"type":"key","form":"frmCliente","key":"PageUp"}
{"t":4.5,"type":"error","message":"tabela travada"}
"#;

    #[test]
    fn a_recording_becomes_a_script() {
        let rec = Recording::parse(REC).unwrap();
        let script = script(&rec, Options { waits: false });
        let steps = script["steps"].as_array().unwrap();
        assert_eq!(
            steps,
            &[
                json!({"drag": "frmCliente", "from": [20.0, 30.0], "to": [20.0, 30.0]}),
                json!({"set": "frmCliente.edtNome", "prop": "text", "value": "aN"}),
                json!({"key": "ctrl+s"}),
                json!({"key": "f9"}),
                json!({"click": "frmCliente.btnSalvar"}),
                json!({"drag": "frmCliente", "from": [10.0, 10.0], "to": [80.0, 40.0], "button": "right"}),
                json!({"drag": "frmCliente", "from": [5.0, 6.0], "to": [5.0, 6.0]}),
            ]
        );
        assert_eq!(script["state"], json!([{"component": "frmCliente.edtNome", "prop": "text"}]));
        let notes = script["notes"].as_array().unwrap();
        assert!(notes.iter().any(|n| n.as_str().unwrap().contains("texto secreto em frmCliente.edtSenha")));
        assert!(notes.iter().any(|n| n.as_str().unwrap().contains("PageUp")));
        assert!(notes.iter().any(|n| n.as_str().unwrap().contains("tabela travada")));
        assert_eq!(script["source"]["app"], "Cadastro");
    }

    #[test]
    fn the_time_between_steps_becomes_waits() {
        let rec = Recording::parse(REC).unwrap();
        let script = script(&rec, Options::default());
        let waits: Vec<u64> = script["steps"].as_array().unwrap().iter().filter_map(|s| s["wait"].as_u64()).collect();
        // Do clique no campo ao texto (0,4 s) não; do texto ao Ctrl+S (1,4 s) e do clique ao arrasto (0,9 s), sim.
        assert_eq!(waits, [1400, 900]);
    }

    #[test]
    fn keys_as_the_script_writes_them() {
        assert_eq!(key_step("Ctrl+Shift+N").as_deref(), Some("ctrl+shift+n"));
        assert_eq!(key_step("F(12)").as_deref(), Some("f12"));
        assert_eq!(key_step("Enter").as_deref(), Some("enter"));
        assert_eq!(key_step("Ctrl++").as_deref(), Some("ctrl++"));
        assert_eq!(key_step("PageUp"), None);
        assert!(types_text("Shift+N") && types_text("A") && !types_text("Ctrl+S") && !types_text("Enter"));
    }
}
