//! O `on_message` do WebView chega ao backend com a URL de quem mandou no `info`, sem mudar
//! o `args`.

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::{Ui, WebMessage};
use aylis_remote::*;
use serde_json::json;

#[test]
fn on_message_carries_the_sender_url_in_info() {
    let mut ui = Ui::new();
    register_all(&mut ui);
    let events = Rc::new(RefCell::new(Vec::new()));
    let e = events.clone();
    forward_events(&mut ui, move |_, event| e.borrow_mut().push(event));
    execute(&mut ui, &json!({ "cmd": "create", "class": "Form", "name": "frmCheckout" })).unwrap();
    execute(&mut ui, &json!({
        "cmd": "create", "class": "WebView", "parent": "frmCheckout", "name": "webPagamento",
        "props": { "url": "https://pagamento.exemplo.com/", "on_message": "web_pagamento_message", "on_load": "web_load" }
    }))
    .unwrap();
    let web = ui.find_path("frmCheckout.webPagamento").unwrap();
    ui.send_message(web, &WebMessage { text: "pagamento_ok".into(), url: "https://pagamento.exemplo.com/retorno".into() });
    ui.fire(web, "on_load", &aylis_core::EventArgs::Text("https://pagamento.exemplo.com/".into()));
    let events = events.borrow();
    assert_eq!(events.len(), 2, "{events:?}");
    assert_eq!(events[0]["args"], "pagamento_ok");
    assert_eq!(events[0]["info"]["url"], "https://pagamento.exemplo.com/retorno");
    assert!(events[1].get("info").is_none(), "só os eventos que têm dados extras levam o info");
}
