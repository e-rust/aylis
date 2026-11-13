//! Android pelo JNI: o teclado virtual completo (a `dev.aylis.AylisActivity` que a
//! publicação põe no APK), a área de transferência, o "voltar" no form principal e a
//! reabertura da Activity num processo novo.
//!
//! O teclado manda edições ("apaga N antes do cursor, M depois, insere") na thread de
//! interface do Java; elas entram numa fila e acordam o laço de eventos do aylis, que as
//! aplica no controle com foco.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::jint;
use jni::{JNIEnv, JavaVM, NativeMethod};
use winit::event_loop::EventLoopProxy;
use winit::platform::android::activity::AndroidApp;

/// Uma mudança pedida pelo teclado do sistema.
pub(crate) enum Edit {
    Text { before: usize, after: usize, text: String },
    /// Código de tecla do Android (`KeyEvent.KEYCODE_*`).
    Key(i32),
}

struct State {
    vm: JavaVM,
    activity: GlobalRef,
    /// A Activity é a `AylisActivity` (o teclado completo está disponível).
    aylis_activity: bool,
}

static STATE: OnceLock<State> = OnceLock::new();
static QUEUE: Mutex<VecDeque<Edit>> = Mutex::new(VecDeque::new());
/// Eventos dos WebViews: (id do controle, 0 carregou / 1 navega / 2 mensagem, texto).
static WEB_EVENTS: Mutex<Vec<(u64, i32, String)>> = Mutex::new(Vec::new());
static WAKE: Mutex<Option<EventLoopProxy<()>>> = Mutex::new(None);

/// Guarda a VM e a Activity e registra as funções nativas da `AylisActivity`, se for ela.
pub(crate) fn init(app: &AndroidApp, proxy: EventLoopProxy<()>) {
    if let Ok(mut w) = WAKE.lock() {
        *w = Some(proxy);
    }
    if STATE.get().is_some() {
        return;
    }
    let Ok(vm) = (unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }) else { return };
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
    let Ok(mut env) = vm.attach_current_thread() else { return };
    let Ok(activity) = env.new_global_ref(&activity) else { return };
    let aylis_activity = register(&mut env, activity.as_obj()).is_ok();
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_clear();
    }
    drop(env);
    let _ = STATE.set(State { vm, activity, aylis_activity });
}

fn register(env: &mut JNIEnv, activity: &JObject) -> jni::errors::Result<()> {
    let loader = env.call_method(activity, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?.l()?;
    let name = env.new_string("dev.aylis.AylisActivity")?;
    let class: JClass = env.call_method(&loader, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[(&name).into()])?.l()?.into();
    if !env.is_instance_of(activity, &class)? {
        return Err(jni::errors::Error::JavaException);
    }
    env.register_native_methods(
        &class,
        &[
            NativeMethod { name: "nativeEdit".into(), sig: "(IILjava/lang/String;)V".into(), fn_ptr: native_edit as *mut _ },
            NativeMethod { name: "nativeKey".into(), sig: "(I)V".into(), fn_ptr: native_key as *mut _ },
            NativeMethod { name: "nativeWebEvent".into(), sig: "(JILjava/lang/String;)V".into(), fn_ptr: native_web_event as *mut _ },
            NativeMethod { name: "nativeNotification".into(), sig: "(JLjava/lang/String;)V".into(), fn_ptr: native_notification as *mut _ },
        ]
        .into_iter()
        .chain(super::android_device::natives())
        .collect::<Vec<_>>(),
    )?;
    // O link ou o compartilhar que abriu o app esperava os nativos.
    let _ = env.call_method(activity, "deliverPending", "()V", &[]);
    Ok(())
}

fn push(edit: Edit) {
    if let Ok(mut q) = QUEUE.lock() {
        q.push_back(edit);
    }
    wake();
}

fn wake() {
    if let Ok(w) = WAKE.lock() {
        if let Some(p) = w.as_ref() {
            let _ = p.send_event(());
        }
    }
}

extern "system" fn native_edit(mut env: JNIEnv, _class: JClass, before: jint, after: jint, text: JString) {
    let text: String = env.get_string(&text).map(Into::into).unwrap_or_default();
    push(Edit::Text { before: before.max(0) as usize, after: after.max(0) as usize, text });
}

extern "system" fn native_key(_env: JNIEnv, _class: JClass, code: jint) {
    push(Edit::Key(code));
}

extern "system" fn native_web_event(mut env: JNIEnv, _class: JClass, id: jni::sys::jlong, kind: jint, text: JString) {
    let text: String = env.get_string(&text).map(Into::into).unwrap_or_default();
    if let Ok(mut q) = WEB_EVENTS.lock() {
        q.push((id as u64, kind, text));
    }
    wake();
}

/// Entrega os toques nas notificações à thread da interface (definido pelo serviço).
static NOTIFY_POSTER: OnceLock<aylis_core::Poster> = OnceLock::new();

extern "system" fn native_notification(mut env: JNIEnv, _class: JClass, component: jni::sys::jlong, action: JString) {
    let action: String = env.get_string(&action).map(Into::into).unwrap_or_default();
    let Some(poster) = NOTIFY_POSTER.get() else { return };
    let component = aylis_core::ComponentId::from_u64(component as u64);
    poster.post(move |ui| {
        if action.is_empty() {
            ui.fire(component, "on_click", &aylis_core::EventArgs::None);
        } else {
            if !ui.fire(component, "on_action", &aylis_core::EventArgs::Text(action.clone())) {
                ui.fire(component, "on_button", &aylis_core::EventArgs::Text(action));
            }
        }
        ui.process_events();
    });
}

/// Um `String[]` do Java.
pub(crate) fn string_array<'a, 's>(env: &mut JNIEnv<'a>, items: impl ExactSizeIterator<Item = &'s str>) -> jni::errors::Result<jni::objects::JObjectArray<'a>> {
    let array = env.new_object_array(items.len() as i32, "java/lang/String", JObject::null())?;
    for (i, s) in items.enumerate() {
        let js = env.new_string(s)?;
        env.set_object_array_element(&array, i as i32, &js)?;
    }
    Ok(array)
}

/// As notificações do Android (`NotificationManager`, pela `AylisActivity`): canal,
/// prioridade, imagem, botões, progresso; o toque volta como `on_click`/`on_action`.
pub(crate) struct Notifications;

impl Notifications {
    pub(crate) fn new(poster: aylis_core::Poster) -> Self {
        let _ = NOTIFY_POSTER.set(poster);
        Self
    }
}

impl aylis_core::platform::NotificationService for Notifications {
    fn notify(&mut self, n: &aylis_core::platform::SystemNotification) -> Result<(), String> {
        use aylis_core::platform::NotificationPriority;
        let importance = match n.priority {
            NotificationPriority::Low => 2,
            NotificationPriority::Default => 3,
            NotificationPriority::High => 4,
        };
        // Sem os bytes, a imagem pode ser um arquivo do aparelho.
        let png = if n.image_png.is_empty() && !n.image.is_empty() { std::fs::read(&n.image).unwrap_or_default() } else { n.image_png.clone() };
        let progress = n.progress.map_or(-1, |p| (p.clamp(0.0, 1.0) * 1000.0) as i32);
        let channel = if n.channel.is_empty() { "geral" } else { n.channel.as_str() };
        let channel_name = if n.channel_name.is_empty() { channel } else { n.channel_name.as_str() };
        with_env(|env, a| {
            let ids = string_array(env, n.actions.iter().map(|(id, _)| id.as_str()))?;
            let titles = string_array(env, n.actions.iter().map(|(_, t)| t.as_str()))?;
            let (ch, chn, title, text) = (env.new_string(channel)?, env.new_string(channel_name)?, env.new_string(&n.title)?, env.new_string(&n.body)?);
            let png = env.byte_array_from_slice(&png)?;
            env.call_method(
                a,
                "showNotification",
                "(ILjava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;[BZ[Ljava/lang/String;[Ljava/lang/String;ZIJZ)V",
                &[
                    JValue::Int(n.id as i32),
                    (&ch).into(),
                    (&chn).into(),
                    JValue::Int(importance),
                    (&title).into(),
                    (&text).into(),
                    (&png).into(),
                    JValue::Bool(n.big_image.into()),
                    (&ids).into(),
                    (&titles).into(),
                    JValue::Bool(n.ongoing.into()),
                    JValue::Int(progress),
                    JValue::Long(n.component.map_or(0, |c| c.to_u64() as i64)),
                    JValue::Bool(n.full_screen.into()),
                ],
            )
            .map(|_| ())
        })
        .ok_or_else(|| "notificação: a AylisActivity não respondeu".to_string())
    }

    fn cancel(&mut self, id: u32) -> Result<(), String> {
        with_env(|env, a| env.call_method(a, "cancelNotification", "(I)V", &[JValue::Int(id as i32)]).map(|_| ())).ok_or_else(|| "notificação: a AylisActivity não respondeu".to_string())
    }
}

/// Os eventos dos WebViews desde a última vez. (As funções do WebView só são usadas com o
/// recurso `webview`; sem ele, ficam sem uso — daí o `allow`.)
#[cfg_attr(not(android_webview), allow(dead_code))]
pub(crate) fn take_web_events() -> Vec<(u64, i32, String)> {
    WEB_EVENTS.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default()
}

/// O WebView está disponível (a Activity é a `AylisActivity`).
pub(crate) fn has_webview() -> bool {
    STATE.get().is_some_and(|s| s.aylis_activity)
}

#[cfg_attr(not(android_webview), allow(dead_code))]
pub(crate) fn web_create(id: u64) {
    with_env(|env, a| env.call_method(a, "webCreate", "(J)V", &[JValue::Long(id as i64)]).map(|_| ()));
}

#[cfg_attr(not(android_webview), allow(dead_code))]
pub(crate) fn web_load(id: u64, url: &str, html: &str) {
    with_env(|env, a| {
        let (u, h) = (env.new_string(url)?, env.new_string(html)?);
        env.call_method(a, "webLoad", "(JLjava/lang/String;Ljava/lang/String;)V", &[JValue::Long(id as i64), (&u).into(), (&h).into()]).map(|_| ())
    });
}

/// Posição e tamanho em pixels da janela.
#[cfg_attr(not(android_webview), allow(dead_code))]
pub(crate) fn web_bounds(id: u64, x: i32, y: i32, width: i32, height: i32, visible: bool) {
    with_env(|env, a| {
        env.call_method(
            a,
            "webBounds",
            "(JIIIIZ)V",
            &[JValue::Long(id as i64), JValue::Int(x), JValue::Int(y), JValue::Int(width), JValue::Int(height), JValue::Bool(visible.into())],
        )
        .map(|_| ())
    });
}

/// 0 JavaScript, 1 voltar, 2 avançar, 3 recarregar, 4 fechar.
#[cfg_attr(not(android_webview), allow(dead_code))]
pub(crate) fn web_command(id: u64, cmd: i32, text: &str) {
    with_env(|env, a| {
        let t = env.new_string(text)?;
        env.call_method(a, "webCommand", "(JILjava/lang/String;)V", &[JValue::Long(id as i64), JValue::Int(cmd), (&t).into()]).map(|_| ())
    });
}

/// As edições que chegaram desde a última vez.
pub(crate) fn take() -> Vec<Edit> {
    QUEUE.lock().map(|mut q| q.drain(..).collect()).unwrap_or_default()
}

pub(crate) fn with_env<R>(f: impl FnOnce(&mut JNIEnv, &JObject) -> jni::errors::Result<R>) -> Option<R> {
    let state = STATE.get()?;
    let mut env = state.vm.attach_current_thread().ok()?;
    let result = f(&mut env, state.activity.as_obj());
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
    }
    result.ok()
}

/// Mostra ou esconde o teclado completo. `false` se a Activity não é a `AylisActivity`
/// (a plataforma usa então o teclado simples do winit).
pub(crate) fn show_keyboard(show: bool, kind: i32) -> bool {
    if !STATE.get().is_some_and(|s| s.aylis_activity) {
        return false;
    }
    with_env(|env, activity| env.call_method(activity, "showKeyboard", "(ZI)V", &[JValue::Bool(show.into()), JValue::Int(kind)]).map(|_| ()))
        .is_some()
}

/// Pinta as barras do sistema com `color` (ARGB) e ícones escuros se `light`.
pub(crate) fn set_system_bars(color: u32, light: bool) -> bool {
    if !STATE.get().is_some_and(|s| s.aylis_activity) {
        return false;
    }
    with_env(|env, activity| env.call_method(activity, "setSystemBars", "(IZ)V", &[JValue::Int(color as i32), JValue::Bool(light.into())]).map(|_| ()))
        .is_some()
}

/// "Voltar" no form principal: o app vai para segundo plano, como os apps do Android 12
/// em diante (a Activity continua viva e volta como estava).
pub(crate) fn move_task_to_back() -> bool {
    with_env(|env, activity| env.call_method(activity, "moveTaskToBack", "(Z)Z", &[JValue::Bool(1)])?.z()).unwrap_or(false)
}

/// Pede ao Android a mesma Activity (a mesma intenção) numa tarefa nova; o processo atual
/// sai em seguida e o sistema cria outro para ela.
pub(crate) fn restart_activity(app: &AndroidApp) -> jni::errors::Result<()> {
    // Ponteiros do próprio android-activity, válidos enquanto o `app` existir.
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast())? };
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
    let mut env = vm.attach_current_thread()?;
    let intent = env.call_method(&activity, "getIntent", "()Landroid/content/Intent;", &[])?.l()?;
    // FLAG_ACTIVITY_NEW_TASK | FLAG_ACTIVITY_CLEAR_TASK
    env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[(0x1000_0000 | 0x0000_8000).into()])?;
    env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()])?;
    Ok(())
}

/// A área de transferência do Android (`ClipboardManager`).
pub(crate) struct Clipboard;

fn clipboard_manager<'a>(env: &mut JNIEnv<'a>, activity: &JObject) -> jni::errors::Result<JObject<'a>> {
    let name = env.new_string("clipboard")?;
    env.call_method(activity, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&name).into()])?.l()
}

impl aylis_core::platform::ClipboardService for Clipboard {
    fn get_text(&mut self) -> Option<String> {
        with_env(|env, activity| {
            let cm = clipboard_manager(env, activity)?;
            let clip = env.call_method(&cm, "getPrimaryClip", "()Landroid/content/ClipData;", &[])?.l()?;
            if clip.is_null() || env.call_method(&clip, "getItemCount", "()I", &[])?.i()? == 0 {
                return Ok(None);
            }
            let item = env.call_method(&clip, "getItemAt", "(I)Landroid/content/ClipData$Item;", &[JValue::Int(0)])?.l()?;
            let text = env.call_method(&item, "coerceToText", "(Landroid/content/Context;)Ljava/lang/CharSequence;", &[activity.into()])?.l()?;
            if text.is_null() {
                return Ok(None);
            }
            let s = env.call_method(&text, "toString", "()Ljava/lang/String;", &[])?.l()?;
            Ok(Some(env.get_string(&JString::from(s))?.into()))
        })
        .flatten()
    }

    fn set_text(&mut self, text: &str) {
        self.set_content(&aylis_core::clipboard::ClipboardContent::text(text));
    }

    /// O texto, com o HTML junto (`newHtmlText`) quando houver. A imagem e os formatos do app
    /// ficam na cópia do processo (o Android só passa conteúdo por `content://`).
    fn set_content(&mut self, content: &aylis_core::clipboard::ClipboardContent) {
        let text = content.text.as_deref().unwrap_or("");
        with_env(|env, activity| {
            let cm = clipboard_manager(env, activity)?;
            let label = env.new_string("aylis")?;
            let value = env.new_string(text)?;
            let clip = match &content.html {
                Some(html) => {
                    let html = env.new_string(html)?;
                    env.call_static_method(
                        "android/content/ClipData",
                        "newHtmlText",
                        "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/String;)Landroid/content/ClipData;",
                        &[(&label).into(), (&value).into(), (&html).into()],
                    )?
                }
                None => env.call_static_method(
                    "android/content/ClipData",
                    "newPlainText",
                    "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
                    &[(&label).into(), (&value).into()],
                )?,
            }
            .l()?;
            env.call_method(&cm, "setPrimaryClip", "(Landroid/content/ClipData;)V", &[(&clip).into()])?;
            Ok(())
        });
    }

    fn get_html(&mut self) -> Option<String> {
        with_env(|env, activity| {
            let cm = clipboard_manager(env, activity)?;
            let clip = env.call_method(&cm, "getPrimaryClip", "()Landroid/content/ClipData;", &[])?.l()?;
            if clip.is_null() || env.call_method(&clip, "getItemCount", "()I", &[])?.i()? == 0 {
                return Ok(None);
            }
            let item = env.call_method(&clip, "getItemAt", "(I)Landroid/content/ClipData$Item;", &[JValue::Int(0)])?.l()?;
            let html = env.call_method(&item, "getHtmlText", "()Ljava/lang/String;", &[])?.l()?;
            if html.is_null() {
                return Ok(None);
            }
            Ok(Some(env.get_string(&JString::from(html))?.into()))
        })
        .flatten()
    }
}
