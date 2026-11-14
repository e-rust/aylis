//! O rastro no roteiro de teste (item 38, 3b): `"profile"` mede os handlers enquanto o roteiro
//! toca, grava os tempos ao lado do estado e reprova o handler acima do limite.

use aylis_controls::{Button, Form};
use aylis_core::*;
use aylis_remote::script;
use serde_json::json;

fn app() -> (Ui, ComponentId) {
    let mut ui = Ui::new();
    aylis_controls::register(&mut ui);
    let form = ui.add_root(Form::new("frmX", 400.0, 300.0));
    let _ = ui.set_name(form, "frmX");
    for (name, handler, ms) in [("btnRapido", "btnRapidoClick", 0), ("btnLento", "btnLentoClick", 30)] {
        let b = ui.add(Button::new(name), form);
        let _ = ui.set_name(b, name);
        ui.set_prop(b, "on_click", PropValue::Event(Some(handler.into()))).unwrap();
        ui.register_handler(&format!("frmX.{handler}"), move |_, _, _| std::thread::sleep(std::time::Duration::from_millis(ms)));
    }
    ui.show(form).unwrap();
    (ui, form)
}

#[test]
fn the_script_measures_the_handlers_and_fails_over_the_limit() {
    let dir = std::env::temp_dir().join(format!("aylis_script_profile_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let output = dir.join("estado.json");

    // Sem limite: os tempos ao lado do estado.
    let (mut ui, form) = app();
    let steps = json!([{"click": "btnRapido"}, {"click": "btnLento"}, {"click": "btnLento"}]);
    let s = json!({"profile": true, "steps": steps});
    script::run_in_process(&mut ui, form, &s, Some(&output)).unwrap();
    let times: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(script::handler_times_path(&output)).unwrap()).unwrap();
    let slow = &times["handlers"]["frmX.btnLentoClick"];
    assert_eq!(slow["count"], 2, "{times}");
    assert!(slow["max_ms"].as_f64().unwrap() >= 25.0, "{times}");
    assert_eq!(times["handlers"]["frmX.btnRapidoClick"]["count"], 1);

    // Com limite por handler: o lento reprova, o rápido não.
    let (mut ui, form) = app();
    let s = json!({"profile": {"max_handler_ms": {"frmX.btnLentoClick": 5, "*": 1000}}, "steps": steps});
    let err = script::run_in_process(&mut ui, form, &s, Some(&output)).unwrap_err();
    assert!(err.contains("frmX.btnLentoClick levou") && !err.contains("btnRapido"), "{err}");

    // Sem `profile`, nada é medido nem gravado a mais.
    let (mut ui, form) = app();
    let _ = std::fs::remove_file(script::handler_times_path(&output));
    script::run_in_process(&mut ui, form, &json!({"steps": steps}), Some(&output)).unwrap();
    assert!(!script::handler_times_path(&output).exists());
    let _ = std::fs::remove_dir_all(&dir);
}
