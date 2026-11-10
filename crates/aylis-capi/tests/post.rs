//! `aylis_post`: comandos mandados de outras threads rodam na thread da tela, com a janela
//! aberta (`aylis_run`). Antes do `aylis_run`, ficam guardados e rodam quando ela abrir.
//!
//! Sem o executor de testes (`harness = false`): o laço de eventos precisa da thread
//! principal. Numa máquina sem tela, o teste avisa e passa.

use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::Mutex;
use std::thread::ThreadId;
use std::time::Duration;

use aylis::*;
use serde_json::{Value, json};

/// Eventos recebidos: (handler, thread onde o callback rodou).
static EVENTS: Mutex<Vec<(String, ThreadId)>> = Mutex::new(Vec::new());

extern "C-unwind" fn on_event(ctx: *mut AylisCtx, event_json: *const c_char, _user: *mut c_void) {
    let event: Value = serde_json::from_str(unsafe { CStr::from_ptr(event_json) }.to_str().unwrap()).unwrap();
    let handler = event["handler"].as_str().unwrap_or_default().to_string();
    // O callback pode mexer na tela pelo ctx: conta os cliques no rótulo.
    let get = CString::new(json!({ "cmd": "get", "target": "lbl", "prop": "text" }).to_string()).unwrap();
    let text = unsafe {
        let raw = aylis_ctx_execute(ctx, get.as_ptr());
        let reply: Value = serde_json::from_str(CStr::from_ptr(raw).to_str().unwrap()).unwrap();
        aylis_string_free(raw);
        reply["result"].as_str().unwrap_or_default().to_string()
    };
    let set = CString::new(json!({ "cmd": "set", "target": "lbl", "props": { "text": format!("{text}+") } }).to_string()).unwrap();
    unsafe { aylis_string_free(aylis_ctx_execute(ctx, set.as_ptr())) };
    EVENTS.lock().unwrap().push((handler, std::thread::current().id()));
}

fn post(app: usize, command: Value) -> i32 {
    let text = CString::new(command.to_string()).unwrap();
    unsafe { aylis_post(app as *const AylisApp, text.as_ptr()) }
}

fn main() {
    // Se o laço travar, o teste falha em vez de ficar parado.
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(60));
        eprintln!("aylis_post: a janela não fechou em 60 s");
        std::process::exit(1);
    });
    let app = aylis_new();
    let layout = json!({ "cmd": "load", "layout": {
        "class": "Form", "name": "F", "width": 240, "height": 100, "children": [
            { "class": "Button", "name": "ok", "align": "top", "on_click": "ok_click" },
            { "class": "Label", "name": "lbl", "align": "client" }
        ]
    }});
    let text = CString::new(layout.to_string()).unwrap();
    unsafe {
        aylis_string_free(aylis_execute(app, text.as_ptr()));
        aylis_set_event_callback(app, on_event, std::ptr::null_mut());
    }
    let address = app as usize;
    // Antes da janela: fica guardado.
    assert_eq!(post(address, json!({ "cmd": "click", "target": "ok" })), 0);
    assert_eq!(post(address, json!({ "cmd": "bogus" })), 0, "um comando inválido não derruba nada");
    assert_eq!(unsafe { aylis_post(app, std::ptr::null()) }, -1);
    // De outra thread, com a janela aberta: mais dois cliques e fechar.
    let poster = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        assert_eq!(post(address, json!({ "cmd": "click", "target": "ok" })), 0);
        assert_eq!(post(address, json!({ "cmd": "click", "target": "ok" })), 0);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(post(address, json!({ "cmd": "close" })), 0);
        std::thread::current().id()
    });
    let form = CString::new("F").unwrap();
    let status = unsafe { aylis_run(app, form.as_ptr()) };
    let poster_thread = poster.join().unwrap();
    if status == -4 {
        eprintln!("aylis_post: sem tela para abrir a janela; teste pulado");
        return;
    }
    assert_eq!(status, 0, "aylis_run");
    let events = EVENTS.lock().unwrap();
    assert_eq!(events.len(), 3, "o clique de antes da janela e os dois da outra thread: {events:?}");
    let ui_thread = std::thread::current().id();
    assert!(events.iter().all(|(h, t)| h == "ok_click" && *t == ui_thread), "rodaram na thread da tela: {events:?}");
    assert_ne!(poster_thread, ui_thread);
    unsafe { aylis_free(app) };
    println!("aylis_post: ok ({} eventos na thread da tela)", events.len());
}
