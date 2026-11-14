//! # aylis-capi
//!
//! C ABI do aylis (`aylis.dll` / `libaylis.so` / `libaylis.dylib`): use a interface a partir
//! de qualquer linguagem com FFI — C, Free Pascal, C#, Go, Python (ctypes)... — no **mesmo
//! processo**, com eventos síncronos.
//!
//! Tudo passa pelo mesmo protocolo JSON do `aylis-remote` (veja os comandos lá):
//!
//! ```c
//! AylisApp* app = aylis_new();
//! char* r = aylis_execute(app, "{\"cmd\":\"load\",\"layout\":{...}}");
//! aylis_string_free(r);
//! aylis_set_event_callback(app, on_event, NULL);
//! aylis_run(app, "FrmClientes");   // abre a janela e bloqueia até fechar
//! aylis_free(app);
//!
//! // Eventos chegam em JSON. Dentro do callback, use o `ctx` recebido:
//! void on_event(AylisCtx* ctx, const char* event_json, void* user) {
//!     char* r = aylis_ctx_execute(ctx, "{\"cmd\":\"set\",\"target\":\"lblStatus\",\"props\":{\"text\":\"Salvo\"}}");
//!     aylis_string_free(r);
//! }
//! ```
//!
//! Regras:
//! - toda string devolvida pelo aylis deve ser liberada com `aylis_string_free`;
//! - `aylis_execute` vale antes de `aylis_run`, na thread que criou o app; durante a
//!   execução, use o `ctx` dentro dos callbacks, ou `aylis_post` a partir de outras threads
//!   (executa na thread da tela). `aylis_execute` de outra thread devolve um erro (a tela
//!   não pode ser mexida por duas threads ao mesmo tempo);
//! - as respostas são `{"result": ...}` ou `{"error": "..."}`.

use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};

use aylis_core::Ui;
use aylis_platform_winit::{Options, Waker};
use serde_json::{Value, json};

/// Callback de eventos: `ctx` só vale durante a chamada. Pode chamar `aylis_ctx_execute`, e
/// um evento disparado por ele (um `click`) chega a este mesmo callback na hora. Trate as
/// exceções (C++, Pascal) dentro dele: uma exceção que escape encerra o processo com
/// mensagem (`C-unwind`), em vez de corromper a memória.
pub type AylisEventFn = extern "C-unwind" fn(ctx: *mut AylisCtx, event_json: *const c_char, user: *mut c_void);

/// Contexto entregue aos callbacks (é o `Ui` da tela).
#[repr(transparent)]
pub struct AylisCtx(Ui);

/// O aplicativo.
pub struct AylisApp {
    ui: Option<Ui>,
    /// Comandos postados por outras threads.
    posted: Arc<Mutex<Vec<String>>>,
    waker: Arc<Mutex<Option<Waker>>>,
    /// A thread que criou o app (a única que pode usar `aylis_execute`).
    owner: std::thread::ThreadId,
}

fn to_c(value: Value) -> *mut c_char {
    CString::new(value.to_string()).map_or(std::ptr::null_mut(), CString::into_raw)
}

fn error(message: impl Into<String>) -> *mut c_char {
    to_c(json!({ "error": message.into() }))
}

/// Lê uma string C como JSON.
///
/// # Safety
/// `text` precisa ser nula ou apontar para uma string C válida terminada em zero.
unsafe fn read_json(text: *const c_char) -> Result<Value, String> {
    if text.is_null() {
        return Err("string nula".into());
    }
    // SAFETY: garantido pelo chamador.
    let text = unsafe { CStr::from_ptr(text) }.to_str().map_err(|_| "a string não é UTF-8")?;
    serde_json::from_str(text).map_err(|e| format!("JSON inválido: {e}"))
}

fn run_command(ui: &mut Ui, command: &Value) -> Value {
    match aylis_remote::execute(ui, command) {
        Ok(result) => json!({ "result": result }),
        Err(e) => json!({ "error": e }),
    }
}

/// Cria um aplicativo com todas as classes padrão registradas e o idioma pt-BR.
#[unsafe(no_mangle)]
pub extern "C" fn aylis_new() -> *mut AylisApp {
    let mut ui = Ui::new();
    aylis_remote::register_all(&mut ui);
    ui.services_mut().locale = Some(Box::new(aylis_core::platform::LocaleInfo::pt_br()));
    Box::into_raw(Box::new(AylisApp { ui: Some(ui), posted: Arc::default(), waker: Arc::default(), owner: std::thread::current().id() }))
}

/// Libera o aplicativo.
///
/// # Safety
/// `app` precisa ter vindo de `aylis_new` e não pode ser usado depois.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_free(app: *mut AylisApp) {
    if !app.is_null() {
        // SAFETY: garantido pelo chamador.
        drop(unsafe { Box::from_raw(app) });
    }
}

/// Libera uma string devolvida pelo aylis.
///
/// # Safety
/// `text` precisa ter vindo do aylis (ou ser nula).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_string_free(text: *mut c_char) {
    if !text.is_null() {
        // SAFETY: garantido pelo chamador.
        drop(unsafe { CString::from_raw(text) });
    }
}

/// Executa um comando JSON (antes de `aylis_run`). Devolve a resposta em JSON.
///
/// # Safety
/// `app` precisa ser válido e `command` uma string C válida.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_execute(app: *mut AylisApp, command: *const c_char) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        if app.is_null() {
            return error("app nulo");
        }
        // Antes de qualquer referência: de outra thread, recusa (o `owner` nunca muda).
        // SAFETY: `app` não é nulo e veio de `aylis_new` (garantido pelo chamador).
        if unsafe { std::ptr::addr_of!((*app).owner).read() } != std::thread::current().id() {
            return error("aylis_execute só na thread que criou o app; de outra thread use aylis_post");
        }
        // SAFETY: garantido pelo chamador.
        let Some(app) = (unsafe { app.as_mut() }) else { return error("app nulo") };
        let Some(ui) = app.ui.as_mut() else {
            return error("o aplicativo está rodando: use o ctx dos callbacks ou aylis_post");
        };
        // SAFETY: garantido pelo chamador.
        match unsafe { read_json(command) } {
            Ok(command) => to_c(run_command(ui, &command)),
            Err(e) => error(e),
        }
    }))
    .unwrap_or_else(|_| error("pânico interno no Aylis"))
}

/// Executa um comando de dentro de um callback de evento.
///
/// # Safety
/// `ctx` precisa ser o recebido no callback (só vale durante ele).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_ctx_execute(ctx: *mut AylisCtx, command: *const c_char) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: garantido pelo chamador.
        let Some(ctx) = (unsafe { ctx.as_mut() }) else { return error("ctx nulo") };
        // SAFETY: garantido pelo chamador.
        match unsafe { read_json(command) } {
            Ok(command) => to_c(run_command(&mut ctx.0, &command)),
            Err(e) => error(e),
        }
    }))
    .unwrap_or_else(|_| error("pânico interno no Aylis"))
}

/// Define o callback que recebe os eventos (em JSON) com handler por nome.
///
/// # Safety
/// `app` precisa ser válido; `callback` e `user` precisam continuar válidos enquanto o
/// aplicativo existir.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_set_event_callback(app: *mut AylisApp, callback: AylisEventFn, user: *mut c_void) -> c_int {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: garantido pelo chamador.
        let Some(ui) = (unsafe { app.as_mut() }).and_then(|a| a.ui.as_mut()) else { return -1 };
        let user = user as usize;
        aylis_remote::forward_events(ui, move |ui, event| {
            let Ok(text) = CString::new(event.to_string()) else { return };
            // `AylisCtx` é um invólucro transparente do `Ui`.
            let ctx = (ui as *mut Ui).cast::<AylisCtx>();
            callback(ctx, text.as_ptr(), user as *mut c_void);
        });
        0
    }))
    .unwrap_or(-5)
}

/// Agenda um comando para rodar na thread da tela (pode ser chamada de qualquer thread,
/// a qualquer momento). Antes de `aylis_run`, roda assim que a janela abrir.
///
/// # Safety
/// `app` precisa ser válido e `command` uma string C válida.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_post(app: *const AylisApp, command: *const c_char) -> c_int {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: garantido pelo chamador; só os campos compartilhados (Arc) são usados.
        let Some(app) = (unsafe { app.as_ref() }) else { return -1 };
        if command.is_null() {
            return -1;
        }
        // SAFETY: garantido pelo chamador.
        let Ok(text) = (unsafe { CStr::from_ptr(command) }).to_str() else { return -1 };
        if let Ok(mut posted) = app.posted.lock() {
            posted.push(text.to_string());
        }
        if let Some(waker) = app.waker.lock().ok().and_then(|w| w.clone()) {
            waker.wake();
        }
        0
    }))
    .unwrap_or(-5)
}

/// Abre o form numa janela e bloqueia até ela fechar. Devolve 0 se tudo correu bem.
/// Depois disso o aplicativo não pode mais ser usado (só liberado).
///
/// # Safety
/// `app` precisa ser válido e `form` uma string C válida (nome do form).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_run(app: *mut AylisApp, form: *const c_char) -> c_int {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: garantido pelo chamador.
        let Some(app) = (unsafe { app.as_mut() }) else { return -1 };
        if form.is_null() {
            return -1;
        }
        // SAFETY: garantido pelo chamador.
        let Ok(form_name) = (unsafe { CStr::from_ptr(form) }).to_str() else { return -1 };
        let Some(ui) = app.ui.take() else { return -2 };
        let Some(form) = ui.find_path(form_name) else {
            app.ui = Some(ui);
            return -3;
        };
        let (posted, slot) = (app.posted.clone(), app.waker.clone());
        let result = aylis_platform_winit::run_with_pump(ui, form, Options::default(), move |waker| {
            if let Ok(mut s) = slot.lock() {
                *s = Some(waker);
            }
            Box::new(move |ui: &mut Ui| {
                let pending = posted.lock().map(|mut p| std::mem::take(&mut *p)).unwrap_or_default();
                for text in pending {
                    if let Ok(command) = serde_json::from_str::<Value>(&text) {
                        if let Some(reply) = aylis_remote::handle_message(ui, &command) {
                            aylis_core::stderr_line!("aylis: {reply}");
                        }
                    }
                }
            })
        });
        if let Ok(mut s) = app.waker.lock() {
            *s = None;
        }
        match result {
            Ok(()) => 0,
            Err(e) => {
                aylis_core::stderr_line!("aylis: {e}");
                -4
            }
        }
    }))
    .unwrap_or(-5)
}

/// Desenha o form num PNG (sem abrir janela). Útil para testes e documentação.
///
/// # Safety
/// `app` precisa ser válido; `form` e `path` strings C válidas.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aylis_render_png(app: *mut AylisApp, form: *const c_char, path: *const c_char) -> c_int {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: garantido pelo chamador.
        let Some(ui) = (unsafe { app.as_mut() }).and_then(|a| a.ui.as_mut()) else { return -1 };
        if form.is_null() || path.is_null() {
            return -1;
        }
        // SAFETY: garantido pelo chamador.
        let (Ok(form), Ok(path)) = (unsafe { CStr::from_ptr(form) }.to_str(), unsafe { CStr::from_ptr(path) }.to_str()) else {
            return -1;
        };
        let Some(form) = ui.find_path(form) else { return -3 };
        let renderer = aylis_skia::SkiaRenderer::new();
        if ui.services().text.is_none() {
            ui.services_mut().text = Some(Box::new(renderer.text_service()));
        }
        ui.layout();
        let (w, h) = ui.control(form).map_or((800.0, 600.0), |cb| (cb.width(), cb.height()));
        let background = ui.theme().color("background");
        match renderer.render_png(w as u32, h as u32, 1.0, background, |c| ui.paint(form, c)) {
            Some(png) if std::fs::write(path, &png).is_ok() => 0,
            _ => -4,
        }
    }))
    .unwrap_or(-5)
}
