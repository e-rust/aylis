//! Usa a C ABI como um cliente C usaria (ponteiros e strings C).

use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

use aylis::*;
use serde_json::{Value, json};

fn exec(app: *mut AylisApp, command: Value) -> Value {
    let text = CString::new(command.to_string()).unwrap();
    unsafe {
        let raw = aylis_execute(app, text.as_ptr());
        let reply: Value = serde_json::from_str(CStr::from_ptr(raw).to_str().unwrap()).unwrap();
        aylis_string_free(raw);
        reply
    }
}

static EVENTS: AtomicUsize = AtomicUsize::new(0);

extern "C-unwind" fn on_event(ctx: *mut AylisCtx, event_json: *const c_char, user: *mut c_void) {
    EVENTS.fetch_add(1, Ordering::SeqCst);
    assert_eq!(user as usize, 42, "o ponteiro do usuário volta intacto");
    let event: Value = serde_json::from_str(unsafe { CStr::from_ptr(event_json) }.to_str().unwrap()).unwrap();
    let run = |command: Value| {
        let command = CString::new(command.to_string()).unwrap();
        unsafe {
            let raw = aylis_ctx_execute(ctx, command.as_ptr());
            aylis_string_free(raw);
        }
    };
    match event["handler"].as_str() {
        Some("ok_click") => {
            run(json!({ "cmd": "set", "target": "lbl", "props": { "text": "clicado" } }));
            // Um clique de dentro do callback: o evento dele chega aqui na hora (reentrante).
            run(json!({ "cmd": "click", "target": "outro" }));
        }
        Some("outro_click") => run(json!({ "cmd": "set", "target": "lbl", "props": { "text": "clicado e encadeado" } })),
        other => panic!("handler inesperado: {other:?}"),
    }
}

#[test]
fn execute_events_and_render() {
    let app = aylis_new();
    let reply = exec(app, json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "F", "width": 300, "height": 120, "children": [
            { "class": "Button", "name": "ok", "align": "top", "on_click": "ok_click" },
            { "class": "Button", "name": "outro", "align": "top", "on_click": "outro_click" },
            { "class": "Label", "name": "lbl", "align": "client" }
        ]
    }}));
    assert_eq!(reply["result"]["name"], "F");

    unsafe { aylis_set_event_callback(app, on_event, 42 as *mut c_void) };
    exec(app, json!({ "cmd": "click", "target": "ok" }));
    assert_eq!(EVENTS.load(Ordering::SeqCst), 2, "o clique encadeado também chegou");
    assert_eq!(exec(app, json!({ "cmd": "get", "target": "lbl", "prop": "text" }))["result"], "clicado e encadeado");

    let error = exec(app, json!({ "cmd": "get", "target": "nada", "prop": "text" }));
    assert!(error["error"].as_str().unwrap().contains("não encontrado"));

    let path = std::env::temp_dir().join("aylis_capi_test.png");
    let path_c = CString::new(path.to_str().unwrap()).unwrap();
    let form = CString::new("F").unwrap();
    assert_eq!(unsafe { aylis_render_png(app, form.as_ptr(), path_c.as_ptr()) }, 0);
    assert!(std::fs::metadata(&path).unwrap().len() > 1000);
    let _ = std::fs::remove_file(path);

    // strings inválidas não derrubam o processo
    let bad = CString::new("{ isto não é json").unwrap();
    unsafe {
        let raw = aylis_execute(app, bad.as_ptr());
        assert!(CStr::from_ptr(raw).to_str().unwrap().contains("JSON inválido"));
        aylis_string_free(raw);
        aylis_free(app);
    }
}

#[test]
fn execute_from_another_thread_is_refused() {
    let app = aylis_new() as usize;
    let reply = std::thread::spawn(move || exec(app as *mut AylisApp, json!({ "cmd": "find", "name": "x" }))).join().unwrap();
    assert!(reply["error"].as_str().unwrap().contains("aylis_post"), "{reply}");
    // Na thread que criou, vale.
    assert!(exec(app as *mut AylisApp, json!({ "cmd": "find", "name": "x" })).get("result").is_some());
    unsafe { aylis_free(app as *mut AylisApp) };
}
