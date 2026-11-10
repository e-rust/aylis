//! Navegador: notificações push pela Push API (Web Push com chave VAPID). A assinatura
//! (`PushSubscription` em JSON: o endpoint e as chaves) é o token. Quem recebe é o service
//! worker da publicação (`sw.js`), que mostra a notificação e repassa a mensagem à página
//! numa linha no formato de [`parse_push_line`]; o toque que abre a página vem no endereço
//! (`?aylis_push=`).

use std::cell::Cell;

use aylis_core::platform::{PushConfig, PushEvent, PushService, parse_push_line, push_event};
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;

use super::web_device::{call, get, object, set, wait, window};

thread_local! {
    /// O `show_in_foreground` do último registro: com a página aberta, o service worker não
    /// mostra a notificação; a página mostra se o app pediu.
    static FOREGROUND: Cell<bool> = const { Cell::new(true) };
    static LISTENING: Cell<bool> = const { Cell::new(false) };
}

fn service_worker() -> JsValue {
    get(&get(&window(), "navigator"), "serviceWorker")
}

fn error_text(e: &JsValue) -> String {
    get(e, "message").as_string().filter(|m| !m.is_empty()).unwrap_or_else(|| format!("{e:?}"))
}

/// A chave VAPID (base64url) em bytes, como o `subscribe` pede (decodificada pelo `atob`).
fn vapid_bytes(key: &str) -> Result<Uint8Array, String> {
    let mut b64 = key.trim().replace('-', "+").replace('_', "/");
    while b64.len() % 4 != 0 {
        b64.push('=');
    }
    let binary = call(&window(), "atob", &[JsValue::from_str(&b64)]).map_err(|_| aylis_core::messages::t("vapid_key não é base64url válido"))?;
    let bytes: Vec<u8> = binary.as_string().unwrap_or_default().chars().map(|c| c as u32 as u8).collect();
    Ok(Uint8Array::from(bytes.as_slice()))
}

#[derive(Default)]
pub(crate) struct WebPush;

impl WebPush {
    /// Ouve o service worker e pega o toque que abriu a página (uma vez, ao instalar).
    pub(crate) fn new() -> Self {
        if !LISTENING.with(|l| l.replace(true)) && !service_worker().is_undefined() {
            let on_message = Closure::<dyn FnMut(JsValue)>::new(|event: JsValue| {
                let Some(line) = get(&get(&event, "data"), "aylis_push").as_string() else { return };
                let Some(ev) = parse_push_line(&line) else { return };
                if let PushEvent::Message(m) = &ev {
                    // Escondida, o service worker já mostrou.
                    let visible = get(&get(&window(), "document"), "visibilityState").as_string().as_deref() == Some("visible");
                    if visible && FOREGROUND.with(Cell::get) && !(m.title.is_empty() && m.body.is_empty()) {
                        show(&m.title, &m.body, &line);
                    }
                }
                push_event(ev);
            });
            let _ = call(&service_worker(), "addEventListener", &[JsValue::from_str("message"), on_message.as_ref().clone()]);
            on_message.forget();
            opened_from_address();
        }
        WebPush
    }
}

/// A notificação mostrada pela página (com ela aberta e `show_in_foreground`).
fn show(title: &str, body: &str, line: &str) {
    let (title, body, line) = (title.to_string(), body.to_string(), line.to_string());
    spawn_local(async move {
        let Ok(reg) = wait(Ok(get(&service_worker(), "ready"))).await else { return };
        let options = object();
        set(&options, "body", body.as_str());
        let data = object();
        set(&data, "aylis_push", line.as_str());
        set(&options, "data", data);
        let _ = call(&reg, "showNotification", &[JsValue::from_str(&title), options]);
    });
}

/// `?aylis_push=<linha>`: o service worker abriu a página pelo toque na notificação.
fn opened_from_address() {
    let location = get(&window(), "location");
    let Some(search) = get(&location, "search").as_string() else { return };
    let (mut line, mut rest) = (None, Vec::new());
    for part in search.trim_start_matches('?').split('&').filter(|p| !p.is_empty()) {
        match part.strip_prefix("aylis_push=") {
            Some(value) => line = js_sys::decode_uri_component(&value.replace('+', " ")).ok().and_then(|s| s.as_string()),
            None => rest.push(part),
        }
    }
    let Some(line) = line else { return };
    if let Some(ev) = parse_push_line(&format!("opened\t{line}")) {
        push_event(ev);
    }
    // Tira do endereço: recarregar a página não repete o toque.
    let rest = rest.join("&");
    let path = get(&location, "pathname").as_string().unwrap_or_default();
    let url = if rest.is_empty() { path } else { format!("{path}?{rest}") };
    let history = get(&window(), "history");
    let _ = call(&history, "replaceState", &[JsValue::NULL, JsValue::from_str(""), JsValue::from_str(&url)]);
}

impl PushService for WebPush {
    fn available(&self) -> bool {
        !service_worker().is_undefined() && !get(&window(), "PushManager").is_undefined()
    }

    fn provider(&self) -> &'static str {
        "webpush"
    }

    fn register(&mut self, config: &PushConfig) {
        FOREGROUND.with(|f| f.set(config.show_in_foreground));
        let key = config.vapid_key.clone();
        spawn_local(async move {
            match subscribe(&key).await {
                Ok(token) => push_event(PushEvent::Token(token)),
                Err(e) => push_event(PushEvent::Error(e)),
            }
        });
    }

    fn unregister(&mut self) {
        spawn_local(async {
            let Ok(reg) = wait(call(&service_worker(), "getRegistration", &[])).await else { return };
            if reg.is_undefined() {
                return;
            }
            if let Ok(sub) = wait(call(&get(&reg, "pushManager"), "getSubscription", &[])).await {
                if !sub.is_null() {
                    let _ = wait(call(&sub, "unsubscribe", &[])).await;
                }
            }
        });
    }
}

async fn subscribe(key: &str) -> Result<String, String> {
    if key.trim().is_empty() {
        return Err(aylis_core::messages::t("Web Push: preencha vapid_key (a chave pública VAPID do servidor)"));
    }
    // A permissão de notificar (o navegador pergunta uma vez).
    let notification = get(&window(), "Notification");
    let state = wait(call(&notification, "requestPermission", &[])).await.map_err(|e| error_text(&e))?;
    if state.as_string().as_deref() != Some("granted") {
        return Err(aylis_core::messages::t("a permissão de notificar foi negada"));
    }
    // Sem service worker, o `ready` nunca responde: confere antes.
    let reg = wait(call(&service_worker(), "getRegistration", &[])).await.map_err(|e| error_text(&e))?;
    if reg.is_undefined() {
        return Err(aylis_core::messages::t("a página não tem service worker (publique para a web e sirva em https)"));
    }
    let reg = wait(Ok(get(&service_worker(), "ready"))).await.map_err(|e| error_text(&e))?;
    let manager = get(&reg, "pushManager");
    let options = object();
    set(&options, "userVisibleOnly", true);
    set(&options, "applicationServerKey", vapid_bytes(key)?);
    let sub = match wait(call(&manager, "subscribe", &[options.clone()])).await {
        Ok(sub) => sub,
        Err(first) => {
            // Assinada com outra chave: desfaz e assina de novo.
            let old = wait(call(&manager, "getSubscription", &[])).await.unwrap_or(JsValue::NULL);
            if old.is_null() || old.is_undefined() {
                return Err(error_text(&first));
            }
            let _ = wait(call(&old, "unsubscribe", &[])).await;
            wait(call(&manager, "subscribe", &[options])).await.map_err(|e| error_text(&e))?
        }
    };
    let json = js_sys::JSON::stringify(&sub).map_err(|e| error_text(&e))?;
    Ok(json.as_string().unwrap_or_default())
}
