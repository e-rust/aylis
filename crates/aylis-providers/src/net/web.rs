//! A rede do Aylis no navegador: o `fetch` e o `WebSocket` do próprio navegador. O que ele não
//! deixa o app controlar fica documentado: os cabeçalhos da abertura do WebSocket, quantos
//! redirecionamentos seguir (o navegador segue até 20, ou nenhum com `max_redirects: 0`) e o
//! CORS e o conteúdo misto, que são do navegador.

// Com uma feature só, a outra metade fica sem uso.
#![cfg_attr(not(all(feature = "http", feature = "websocket")), allow(dead_code, unused_imports))]

use std::cell::RefCell;
use std::rc::Rc;

use aylis_core::providers::{WebSocketConnection, WebSocketEvent, WebSocketEvents, WebSocketRequest};
#[cfg(feature = "http")]
use aylis_core::providers::{HttpDone, HttpRequest, HttpResponse};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

fn describe(e: &JsValue) -> String {
    e.as_string().or_else(|| js_sys::Reflect::get(e, &"message".into()).ok().and_then(|m| m.as_string())).unwrap_or_else(|| format!("{e:?}"))
}

/// Roda `f` daqui a `ms` milissegundos (o `setTimeout` da janela).
fn later(ms: u32, f: impl FnOnce() + 'static) {
    if let Some(window) = web_sys::window() {
        let cb = Closure::once_into_js(f);
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(cb.unchecked_ref(), i32::try_from(ms).unwrap_or(i32::MAX));
    }
}

#[cfg(feature = "http")]
pub(super) fn fetch(request: HttpRequest, done: HttpDone) {
    wasm_bindgen_futures::spawn_local(async move { done(run(request).await) });
}

#[cfg(feature = "http")]
async fn run(request: HttpRequest) -> Result<HttpResponse, String> {
    let window = web_sys::window().ok_or("HTTP: sem window")?;
    let init = web_sys::RequestInit::new();
    init.set_method(&request.method);
    let headers = web_sys::Headers::new().map_err(|e| describe(&e))?;
    for (k, v) in &request.headers {
        headers.set(k, v).map_err(|e| format!("HTTP: cabeçalho '{k}': {}", describe(&e)))?;
    }
    init.set_headers(&headers);
    if !request.body.is_empty() {
        let body = js_sys::Uint8Array::from(request.body.as_slice());
        init.set_body(&body);
    }
    // Credenciais só para o mesmo site (o padrão do navegador), nunca para outro.
    init.set_credentials(web_sys::RequestCredentials::SameOrigin);
    init.set_redirect(if request.max_redirects == 0 { web_sys::RequestRedirect::Manual } else { web_sys::RequestRedirect::Follow });
    let abort = web_sys::AbortController::new().map_err(|e| describe(&e))?;
    init.set_signal(Some(&abort.signal()));
    let timed_out = Rc::new(std::cell::Cell::new(false));
    if request.timeout_ms > 0 {
        let (abort, flag) = (abort.clone(), timed_out.clone());
        later(request.timeout_ms, move || {
            flag.set(true);
            abort.abort();
        });
    }
    let timeout_error = || format!("HTTP: prazo esgotado ({} ms)", request.timeout_ms);
    let promise = window.fetch_with_str_and_init(&request.url, &init);
    let response = match wasm_bindgen_futures::JsFuture::from(promise).await {
        Ok(r) => r,
        Err(_) if timed_out.get() => return Err(timeout_error()),
        Err(e) => return Err(format!("HTTP: {}", describe(&e))),
    };
    let response: web_sys::Response = response.dyn_into().map_err(|_| "HTTP: resposta inválida")?;
    let limit = request.response_limit();
    let mut out_headers = Vec::new();
    if let Ok(Some(iter)) = js_sys::try_iter(&response.headers()) {
        for pair in iter.flatten() {
            let pair = js_sys::Array::from(&pair);
            if let (Some(k), Some(v)) = (pair.get(0).as_string(), pair.get(1).as_string()) {
                out_headers.push((k.to_ascii_lowercase(), v));
            }
        }
    }
    if let Some(len) = out_headers.iter().find(|(k, _)| k == "content-length").and_then(|(_, v)| v.parse::<u64>().ok()) {
        if len > limit {
            abort.abort();
            return Err(format!("HTTP: resposta maior que o limite ({limit} bytes)"));
        }
    }
    let buffer = match wasm_bindgen_futures::JsFuture::from(response.array_buffer().map_err(|e| describe(&e))?).await {
        Ok(b) => b,
        Err(_) if timed_out.get() => return Err(timeout_error()),
        Err(e) => return Err(format!("HTTP: {}", describe(&e))),
    };
    let bytes = js_sys::Uint8Array::new(&buffer);
    if u64::from(bytes.length()) > limit {
        return Err(format!("HTTP: resposta maior que o limite ({limit} bytes)"));
    }
    Ok(HttpResponse { status: response.status(), headers: out_headers, body: bytes.to_vec(), url: response.url() })
}

/// Uma conexão do `WebSocket` do navegador (os handlers vivem com ela).
struct WebWs {
    socket: web_sys::WebSocket,
    _handlers: Vec<Closure<dyn FnMut(JsValue)>>,
}

impl Drop for WebWs {
    fn drop(&mut self) {
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        self.socket.set_onerror(None);
        self.socket.set_onclose(None);
        let _ = self.socket.close();
    }
}

impl WebSocketConnection for WebWs {
    fn send_text(&mut self, text: &str) -> Result<(), String> {
        self.socket.send_with_str(text).map_err(|e| format!("WebSocket: {}", describe(&e)))
    }

    fn send_binary(&mut self, data: &[u8]) -> Result<(), String> {
        self.socket.send_with_u8_array(data).map_err(|e| format!("WebSocket: {}", describe(&e)))
    }

    fn close(&mut self, code: u16, reason: &str) {
        // O navegador só aceita 1000 ou 3000-4999 do app.
        let code = if code == 1000 || (3000..5000).contains(&code) { code } else { 1000 };
        let _ = self.socket.close_with_code_and_reason(code, reason);
    }
}

struct Failed;

impl WebSocketConnection for Failed {
    fn send_text(&mut self, _: &str) -> Result<(), String> {
        Err("WebSocket: a conexão não abriu".into())
    }
    fn send_binary(&mut self, _: &[u8]) -> Result<(), String> {
        Err("WebSocket: a conexão não abriu".into())
    }
    fn close(&mut self, _: u16, _: &str) {}
}

pub(super) fn websocket(request: WebSocketRequest, events: WebSocketEvents) -> Box<dyn WebSocketConnection> {
    let events = Rc::new(RefCell::new(events));
    let emit = {
        let events = events.clone();
        move |e: WebSocketEvent| {
            if let Ok(mut f) = events.try_borrow_mut() {
                f(e);
            }
        }
    };
    let protocols = js_sys::Array::new();
    for p in &request.protocols {
        protocols.push(&JsValue::from_str(p));
    }
    let socket = match web_sys::WebSocket::new_with_str_sequence(&request.url, &protocols) {
        Ok(s) => s,
        Err(e) => {
            let reason = format!("WebSocket: {}", describe(&e));
            emit(WebSocketEvent::Error(reason.clone()));
            emit(WebSocketEvent::Closed { code: 1006, reason });
            return Box::new(Failed);
        }
    };
    socket.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let limit = request.message_limit();
    let mut handlers: Vec<Closure<dyn FnMut(JsValue)>> = Vec::new();
    {
        let (emit, s) = (emit.clone(), socket.clone());
        let on = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| emit(WebSocketEvent::Open { protocol: s.protocol() }));
        socket.set_onopen(Some(on.as_ref().unchecked_ref()));
        handlers.push(on);
    }
    {
        let (emit, s) = (emit.clone(), socket.clone());
        let on = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
            let Ok(e) = e.dyn_into::<web_sys::MessageEvent>() else { return };
            let data = e.data();
            let size = data.as_string().map(|t| t.len() as u64).unwrap_or_else(|| data.dyn_ref::<js_sys::ArrayBuffer>().map_or(0, |b| u64::from(b.byte_length())));
            if size > limit {
                emit(WebSocketEvent::Error(format!("WebSocket: mensagem maior que o limite ({limit} bytes)")));
                let _ = s.close_with_code_and_reason(1009, "");
                return;
            }
            match data.as_string() {
                Some(text) => emit(WebSocketEvent::Text(text)),
                None => emit(WebSocketEvent::Binary(js_sys::Uint8Array::new(&data).to_vec())),
            }
        });
        socket.set_onmessage(Some(on.as_ref().unchecked_ref()));
        handlers.push(on);
    }
    {
        let emit = emit.clone();
        let on = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| emit(WebSocketEvent::Error("WebSocket: falha na conexão".into())));
        socket.set_onerror(Some(on.as_ref().unchecked_ref()));
        handlers.push(on);
    }
    {
        let emit = emit.clone();
        let on = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
            let (code, reason) = e.dyn_into::<web_sys::CloseEvent>().map(|c| (c.code(), c.reason())).unwrap_or((1006, String::new()));
            emit(WebSocketEvent::Closed { code, reason });
        });
        socket.set_onclose(Some(on.as_ref().unchecked_ref()));
        handlers.push(on);
    }
    if request.connect_timeout_ms > 0 {
        let s = socket.clone();
        later(request.connect_timeout_ms, move || {
            if s.ready_state() == web_sys::WebSocket::CONNECTING {
                emit(WebSocketEvent::Error("WebSocket: prazo esgotado ao abrir".into()));
                let _ = s.close();
            }
        });
    }
    Box::new(WebWs { socket, _handlers: handlers })
}
