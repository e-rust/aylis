//! Canal de desenvolvimento (só com a feature `dev`, que a IDE liga ao executar; a publicação
//! nunca): o app conecta na IDE, no endereço embutido nele na compilação, e atende os comandos
//! dela — os do Inspetor do app (`tree`, `props`, `set`, `highlight`...), o hot reload
//! (`reload`: um form, o tema ou as traduções, como texto), o rastro de eventos (`trace`) e o
//! que os testes pedem (`snapshot`, `close`). Sem variável de ambiente, parâmetro nem endereço
//! na URL: só a IDE que tem a chave privada e o segredo do projeto fala com o app (veja
//! `aylis-devlink`). Sem a IDE aberta, o app tenta de novo em silêncio.
//!
//! Cada mensagem é um comando com `"id"`; a resposta é `{"id": .., "ok": ..}` ou
//! `{"id": .., "error": ".."}`. O rastro vai sozinho: `{"trace": "..."}`.

use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use aylis_core::{ComponentId, Ui};
use aylis_devlink::Target;
use serde_json::{Value, json};

thread_local! {
    /// O último texto de cada form recebido (para o próximo mudar só o que mudou).
    static PREVIOUS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

/// O alvo embutido pelo `build.rs` (`aylis_dev.json`), lido uma vez.
static TARGET: OnceLock<Option<Target>> = OnceLock::new();

/// Guarda o alvo embutido no executável (vazio fora da feature `dev`).
pub(crate) fn set_embedded(text: &str) {
    let _ = TARGET.set(serde_json::from_str::<Value>(text).ok().and_then(|v| Target::from_json(&v)));
}

/// O alvo do canal, se o app foi compilado pela IDE com `dev`.
pub(crate) fn target() -> Option<Target> {
    TARGET.get().cloned().flatten()
}

/// Atende um comando da IDE (na thread da interface).
pub(crate) fn answer(ui: &mut Ui, mix: &BTreeMap<String, String>, command: &Value) -> Value {
    let id = command.get("id").cloned().unwrap_or(Value::Null);
    let result = match command.get("cmd").and_then(Value::as_str) {
        Some("reload") => reload(ui, mix, command),
        Some("trace") => trace(ui, command.get("on").and_then(Value::as_bool).unwrap_or(false)),
        Some("profile") => {
            if command.get("on").and_then(Value::as_bool).unwrap_or(false) {
                aylis_core::profile::start();
                // As amostras da pilha desta thread (a da interface): a árvore de chamadas.
                #[cfg(all(windows, target_arch = "x86_64"))]
                aylis_platform_winit::sampler::start();
            } else {
                aylis_core::profile::stop();
                #[cfg(all(windows, target_arch = "x86_64"))]
                aylis_platform_winit::sampler::stop();
            }
            Ok(Value::Null)
        }
        Some("profile_take") => Ok(profile_take()),
        Some("snapshot") => snapshot(ui, command),
        Some("close") => {
            ui.request_close();
            Ok(Value::Null)
        }
        _ => aylis_remote::execute(ui, command),
    };
    match result {
        Ok(v) => json!({ "id": id, "ok": v }),
        Err(e) => json!({ "id": id, "error": e }),
    }
}

/// `{"cmd": "reload", "kind": "form"|"theme"|"locale", "name": "frmX", "text": "..."}`.
fn reload(ui: &mut Ui, mix: &BTreeMap<String, String>, command: &Value) -> Result<Value, String> {
    let text = command.get("text").and_then(Value::as_str).ok_or("falta \"text\"")?;
    let kind = command.get("kind").and_then(Value::as_str).unwrap_or("form");
    let done = match kind {
        "form" => {
            let name = command.get("name").and_then(Value::as_str).ok_or("falta \"name\"")?;
            let previous = PREVIOUS.with(|p| p.borrow().get(&name.to_lowercase()).cloned());
            let done = aylis_project::apply_change(ui, aylis_project::Change::Form { name, previous: previous.as_deref() }, text)?;
            PREVIOUS.with(|p| p.borrow_mut().insert(name.to_lowercase(), text.to_string()));
            done
        }
        "theme" => aylis_project::apply_change(ui, aylis_project::Change::Theme { mix }, text)?,
        "locale" => aylis_project::apply_change(ui, aylis_project::Change::Locale, text)?,
        other => return Err(format!("reload: tipo desconhecido '{other}'")),
    };
    log(&format!("aylis: {done}"));
    Ok(Value::String(done))
}

/// `{"cmd": "trace", "on": true}`: liga o rastro de eventos (cada evento com handler vai para
/// a IDE).
fn trace(ui: &mut Ui, on: bool) -> Result<Value, String> {
    if on {
        ui.set_event_trace(Some(|ui: &Ui, id: ComponentId, event: &str, handler: &str| {
            let handler = if handler.is_empty() { "(código)" } else { handler };
            let line = format!("evento {}.{event} → {handler}", ui.path_of(id));
            // No navegador, também no console (no desktop a saída já vai para a IDE).
            #[cfg(target_arch = "wasm32")]
            log(&line);
            send_trace(line);
        }));
    } else {
        ui.set_event_trace(None::<fn(&Ui, ComponentId, &str, &str)>);
    }
    Ok(Value::Null)
}

/// `{"cmd": "profile_take"}`: os blocos medidos desde a última vez (veja
/// [`aylis_core::profile::to_json`]), com a memória do app nesse instante.
fn profile_take() -> Value {
    #[cfg(not(target_arch = "wasm32"))]
    aylis_core::profile::sample("memória", crate::dev_memory::live_bytes() as f64);
    // As imagens guardadas pelos renderizadores: quantas e quanto ocupam.
    let images = aylis_core::image_cache::stats();
    aylis_core::profile::sample("imagens no cache", images.iter().map(|s| s.images).sum::<usize>() as f64);
    aylis_core::profile::sample("memória das imagens", images.iter().map(|s| s.bytes).sum::<usize>() as f64);
    let (spans, samples, dropped) = aylis_core::profile::take();
    let stacks = aylis_core::profile::take_stacks();
    serde_json::from_str(&aylis_core::profile::to_json(&spans, &samples, &stacks, dropped)).unwrap_or(Value::Null)
}

/// `{"cmd": "snapshot", "form": "frmX"}`: a tela em PNG (base64), para os testes que a IDE
/// (ou o roteiro de teste) conduz pelo canal. Com `"target"`, só aquele componente (um
/// cartão, uma lista), como está na tela.
fn snapshot(ui: &mut Ui, command: &Value) -> Result<Value, String> {
    if let Some(target) = command.get("target").filter(|t| !t.is_null()) {
        let id = aylis_remote::resolve(ui, target)?;
        ui.layout();
        let image = ui.snapshot(id, 1.0).ok_or("sem desenho fora da tela nesta plataforma")?;
        let aylis_core::ImageSource::Encoded(png) = image.source() else { return Err("falha ao gerar o PNG".into()) };
        return Ok(Value::String(base64(png)));
    }
    let form = match command.get("form").filter(|f| !f.is_null()) {
        Some(f) => aylis_remote::resolve(ui, f)?,
        None => *ui.open_forms().first().ok_or("nenhum form aberto")?,
    };
    ui.layout();
    let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
    let background = ui.theme().color("background");
    // O serviço sai do `Ui` durante o desenho (a pintura precisa do `Ui` inteiro).
    let service = ui.services_mut().offscreen.take().ok_or("sem desenho fora da tela nesta plataforma")?;
    let png = service.render_png(w as u32, h as u32, 1.0, background, &mut |c: &mut dyn aylis_core::Canvas| ui.paint(form, c));
    ui.services_mut().offscreen = Some(service);
    Ok(Value::String(base64(&png.ok_or("falha ao gerar o PNG")?)))
}

fn base64(bytes: &[u8]) -> String {
    const ABC: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk.iter().enumerate().fold(0u32, |n, (i, b)| n | (*b as u32) << (16 - 8 * i));
        for i in 0..4 {
            out.push(if i <= chunk.len() { ABC[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' });
        }
    }
    out
}

fn log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&line.into());
    #[cfg(not(target_arch = "wasm32"))]
    aylis_core::stderr_line!("{line}");
}

/// Quem é o app, na primeira mensagem do aperto de mão.
fn hello(ui: &Ui) -> String {
    // O canal liga antes de a janela abrir: o form principal, se ainda não há form aberto.
    let form = ui.open_forms().first().copied().or_else(|| ui.main_form());
    let project = form.and_then(|f| ui.name(f).map(str::to_string)).unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    let pid = std::process::id();
    #[cfg(target_arch = "wasm32")]
    let pid = 0;
    json!({ "project": project, "pid": pid }).to_string()
}

#[cfg(not(target_arch = "wasm32"))]
static TRACE_OUT: std::sync::Mutex<Option<std::sync::mpsc::Sender<String>>> = std::sync::Mutex::new(None);

#[cfg(not(target_arch = "wasm32"))]
fn send_trace(line: String) {
    if let Some(tx) = TRACE_OUT.lock().ok().and_then(|t| t.clone()) {
        let _ = tx.send(json!({ "trace": line }).to_string());
    }
}

/// Conecta na IDE e passa a atender os comandos dela (numa thread; sem a IDE, tenta de novo
/// em silêncio).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn connect(ui: &Ui, target: Target, mix: BTreeMap<String, String>) {
    use std::sync::mpsc;
    use std::time::Duration;
    let poster = ui.poster();
    let (trace_tx, trace_rx) = mpsc::channel::<String>();
    if let Ok(mut t) = TRACE_OUT.lock() {
        *t = Some(trace_tx);
    }
    aylis_devlink::client::run(
        target,
        hello(ui),
        move |text| {
            let command: Value = match serde_json::from_str(text) {
                Ok(v) => v,
                Err(e) => return Some(json!({ "error": format!("JSON inválido: {e}") }).to_string()),
            };
            let id = command.get("id").cloned().unwrap_or(Value::Null);
            let (tx, rx) = mpsc::channel::<Value>();
            let mix = mix.clone();
            poster.post(move |ui| {
                let _ = tx.send(answer(ui, &mix, &command));
            });
            let reply = rx.recv_timeout(Duration::from_secs(5)).unwrap_or_else(|_| json!({ "id": id, "error": "o aplicativo não respondeu" }));
            Some(reply.to_string())
        },
        trace_rx,
    );
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// O canal no navegador: o socket, o aperto de mão em andamento e a sessão.
    static WEB: std::cell::RefCell<WebLink> = const { std::cell::RefCell::new(WebLink { socket: None, init: None, session: None }) };
}

#[cfg(target_arch = "wasm32")]
struct WebLink {
    socket: Option<web_sys::WebSocket>,
    init: Option<aylis_devlink::Initiator>,
    session: Option<aylis_devlink::Session>,
}

#[cfg(target_arch = "wasm32")]
fn web_send(text: &str) {
    WEB.with(|w| {
        let mut w = w.borrow_mut();
        let (Some(socket), Some(session)) = (w.socket.clone(), w.session.as_mut()) else { return };
        if let Ok(frames) = session.seal(text) {
            for f in frames {
                let _ = socket.send_with_u8_array(&f);
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn send_trace(line: String) {
    web_send(&json!({ "trace": line }).to_string());
}

/// No navegador: o WebSocket da página, aberto já com o laço de eventos rodando; quando cai
/// (ou a IDE não está aberta), tenta de novo depois de uns segundos.
#[cfg(target_arch = "wasm32")]
pub(crate) fn connect(ui: &Ui, target: Target, mix: BTreeMap<String, String>) {
    ui.poster().post(move |ui| open_socket(ui, target, mix));
}

#[cfg(target_arch = "wasm32")]
fn open_socket(ui: &Ui, target: Target, mix: BTreeMap<String, String>) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    use web_sys::{BinaryType, MessageEvent, WebSocket};

    let Ok(ws) = WebSocket::new(&target.url()) else { return retry(ui, target, mix) };
    ws.set_binary_type(BinaryType::Arraybuffer);
    let hello = hello(ui);
    let (t1, ws1) = (target.clone(), ws.clone());
    let on_open = Closure::<dyn FnMut()>::new(move || {
        if let Ok((init, first)) = aylis_devlink::initiate(&t1, &hello) {
            WEB.with(|w| w.borrow_mut().init = Some(init));
            let _ = ws1.send_with_u8_array(&first);
        }
    });
    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
    let poster = ui.poster();
    let mix_again = mix.clone();
    let on_message = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let Ok(buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() else { return };
        let frame = js_sys::Uint8Array::new(&buffer).to_vec();
        let text = WEB.with(|w| {
            let mut w = w.borrow_mut();
            if let Some(init) = w.init.take() {
                w.session = init.finish(&frame).ok();
                return None;
            }
            w.session.as_mut().and_then(|s| s.open(&frame).ok().flatten())
        });
        let Some(text) = text else { return };
        let Ok(command) = serde_json::from_str::<Value>(&text) else { return };
        let mix = mix.clone();
        poster.post(move |ui| {
            let reply = answer(ui, &mix, &command);
            web_send(&reply.to_string());
        });
    });
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    let (poster, t2) = (ui.poster(), target);
    let on_close = Closure::<dyn FnMut()>::new(move || {
        WEB.with(|w| *w.borrow_mut() = WebLink { socket: None, init: None, session: None });
        let (t, m) = (t2.clone(), mix_again.clone());
        poster.post(move |ui| retry(ui, t, m));
    });
    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
    WEB.with(|w| w.borrow_mut().socket = Some(ws));
}

/// Tenta de novo daqui a 3 segundos (o temporizador do navegador).
#[cfg(target_arch = "wasm32")]
fn retry(ui: &Ui, target: Target, mix: BTreeMap<String, String>) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    let poster = ui.poster();
    let again = Closure::once_into_js(move || poster.post(move |ui| open_socket(ui, target, mix)));
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(again.unchecked_ref(), 3000);
    }
}
