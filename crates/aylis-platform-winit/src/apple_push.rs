//! iOS e macOS: notificações push pelo APNs. O token chega nos métodos do delegado do app
//! (`application:didRegisterForRemoteNotificationsWithDeviceToken:`), que o aylis acrescenta
//! à classe do delegado do winit em tempo de execução; as mensagens com o app aberto e os
//! toques, no delegado da `UNUserNotificationCenter`. A publicação põe o `aps-environment`
//! nos direitos do app (sem ele o sistema recusa o registro).

use std::sync::atomic::{AtomicBool, Ordering};

use block2::{Block, RcBlock};
use aylis_core::platform::{PushConfig, PushEvent, PushMessage, PushService, push_event};
use objc2::declare::ClassBuilder;
use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
use objc2::{class, msg_send, sel};
use objc2_foundation::NSString;

#[link(name = "UserNotifications", kind = "framework")]
unsafe extern "C" {}

type Object = *mut AnyObject;
const NIL: Object = std::ptr::null_mut();

/// O `show_in_foreground` do último registro.
static FOREGROUND: AtomicBool = AtomicBool::new(true);
static INSTALLED: AtomicBool = AtomicBool::new(false);

fn ns(text: &str) -> objc2::rc::Retained<NSString> {
    NSString::from_str(text)
}

unsafe fn string(value: Object) -> String {
    if value.is_null() {
        return String::new();
    }
    let is_string: bool = unsafe { msg_send![value, isKindOfClass: class!(NSString)] };
    if !is_string {
        return String::new();
    }
    let utf8: *const std::ffi::c_char = unsafe { msg_send![value, UTF8String] };
    if utf8.is_null() { String::new() } else { unsafe { std::ffi::CStr::from_ptr(utf8) }.to_string_lossy().into_owned() }
}

/// O aplicativo (`UIApplication` ou `NSApplication`).
unsafe fn application() -> Object {
    #[cfg(target_os = "ios")]
    let app: Object = unsafe { msg_send![class!(UIApplication), sharedApplication] };
    #[cfg(target_os = "macos")]
    let app: Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
    app
}

/// Os dados do app num `userInfo` do APNs (tudo menos o `aps`), em JSON.
unsafe fn data_json(user_info: Object) -> String {
    if user_info.is_null() {
        return "{}".into();
    }
    unsafe {
        let dict: Object = msg_send![user_info, mutableCopy];
        let _: () = msg_send![dict, removeObjectForKey: &*ns("aps")];
        let valid: bool = msg_send![class!(NSJSONSerialization), isValidJSONObject: dict];
        let mut out = String::from("{}");
        if valid {
            let data: Object = msg_send![class!(NSJSONSerialization), dataWithJSONObject: dict, options: 0usize, error: std::ptr::null_mut::<Object>()];
            if !data.is_null() {
                let len: usize = msg_send![data, length];
                let bytes: *const u8 = msg_send![data, bytes];
                if !bytes.is_null() {
                    out = String::from_utf8_lossy(std::slice::from_raw_parts(bytes, len)).into_owned();
                }
            }
        }
        let _: () = msg_send![dict, release];
        out
    }
}

/// A mensagem de um `userInfo` (o título e o texto do `aps.alert`, texto ou dicionário).
unsafe fn message_from_info(user_info: Object) -> PushMessage {
    let mut m = PushMessage { data: unsafe { data_json(user_info) }, ..Default::default() };
    if user_info.is_null() {
        return m;
    }
    unsafe {
        let aps: Object = msg_send![user_info, objectForKey: &*ns("aps")];
        if !aps.is_null() {
            let alert: Object = msg_send![aps, objectForKey: &*ns("alert")];
            if !alert.is_null() {
                let is_dict: bool = msg_send![alert, isKindOfClass: class!(NSDictionary)];
                if is_dict {
                    m.title = string(msg_send![alert, objectForKey: &*ns("title")]);
                    m.body = string(msg_send![alert, objectForKey: &*ns("body")]);
                } else {
                    m.body = string(alert);
                }
            }
        }
        // O id que o FCM põe quando envia pelo APNs; o APNs sozinho não manda um.
        m.id = string(msg_send![user_info, objectForKey: &*ns("gcm.message_id")]);
    }
    m
}

/// A mensagem de uma `UNNotification` (título e texto do conteúdo).
unsafe fn message_from_notification(notification: Object) -> (PushMessage, bool) {
    unsafe {
        let request: Object = msg_send![notification, request];
        let content: Object = msg_send![request, content];
        let info: Object = msg_send![content, userInfo];
        let aps: Object = if info.is_null() { NIL } else { msg_send![info, objectForKey: &*ns("aps")] };
        let mut m = message_from_info(info);
        let title = string(msg_send![content, title]);
        let body = string(msg_send![content, body]);
        if !title.is_empty() {
            m.title = title;
        }
        if !body.is_empty() {
            m.body = body;
        }
        (m, !aps.is_null())
    }
}

// --- os métodos acrescentados ao delegado do app ---

extern "C" fn did_register(_this: *mut AnyObject, _cmd: Sel, _app: Object, token: Object) {
    let hex = unsafe {
        let len: usize = msg_send![token, length];
        let bytes: *const u8 = msg_send![token, bytes];
        if bytes.is_null() { String::new() } else { std::slice::from_raw_parts(bytes, len).iter().map(|b| format!("{b:02x}")).collect() }
    };
    push_event(PushEvent::Token(hex));
}

extern "C" fn did_fail(_this: *mut AnyObject, _cmd: Sel, _app: Object, error: Object) {
    let text = unsafe { string(msg_send![error, localizedDescription]) };
    push_event(PushEvent::Error(format!("APNs: {text}")));
}

/// Mensagem silenciosa (`content-available`) com o app aberto ou em segundo plano.
#[cfg(target_os = "ios")]
extern "C" fn did_receive(_this: *mut AnyObject, _cmd: Sel, _app: Object, info: Object, handler: &Block<dyn Fn(usize)>) {
    push_event(PushEvent::Message(unsafe { message_from_info(info) }));
    // UIBackgroundFetchResultNewData
    handler.call((0,));
}

#[cfg(target_os = "macos")]
extern "C" fn did_receive(_this: *mut AnyObject, _cmd: Sel, _app: Object, info: Object) {
    push_event(PushEvent::Message(unsafe { message_from_info(info) }));
}

// --- o delegado da central de notificações ---

/// Uma notificação chegando com o app na frente: mostra (ou não) e avisa o app.
extern "C" fn will_present(_this: *mut AnyObject, _cmd: Sel, _center: Object, notification: Object, handler: &Block<dyn Fn(usize)>) {
    let (m, remote) = unsafe { message_from_notification(notification) };
    // selo | som | alerta | lista | faixa
    let show = 1 | 2 | 4 | 8 | 16;
    if remote {
        push_event(PushEvent::Message(m));
        handler.call((if FOREGROUND.load(Ordering::Relaxed) { show } else { 0 },));
    } else {
        handler.call((show,));
    }
}

/// O toque na notificação (com o app aberto ou abrindo por ela).
extern "C" fn did_respond(_this: *mut AnyObject, _cmd: Sel, _center: Object, response: Object, handler: &Block<dyn Fn()>) {
    let (m, remote) = unsafe { message_from_notification(msg_send![response, notification]) };
    if remote {
        push_event(PushEvent::Opened(m));
    }
    handler.call(());
}

/// Acrescenta os métodos ao delegado do app e põe o delegado da central (uma vez).
fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    unsafe {
        let app = application();
        let delegate: Object = msg_send![app, delegate];
        if !delegate.is_null() {
            let class: &AnyClass = msg_send![delegate, class];
            let class = class as *const AnyClass as *mut objc2::ffi::objc_class;
            let add = |name: Sel, imp: *const std::ffi::c_void, types: &std::ffi::CStr| {
                let imp: objc2::ffi::IMP = std::mem::transmute::<*const std::ffi::c_void, objc2::ffi::IMP>(imp);
                objc2::ffi::class_addMethod(class, name.as_ptr(), imp, types.as_ptr());
            };
            add(sel!(application:didRegisterForRemoteNotificationsWithDeviceToken:), did_register as *const std::ffi::c_void, c"v@:@@");
            add(sel!(application:didFailToRegisterForRemoteNotificationsWithError:), did_fail as *const std::ffi::c_void, c"v@:@@");
            #[cfg(target_os = "ios")]
            add(sel!(application:didReceiveRemoteNotification:fetchCompletionHandler:), did_receive as *const std::ffi::c_void, c"v@:@@@?");
            #[cfg(target_os = "macos")]
            add(sel!(application:didReceiveRemoteNotification:), did_receive as *const std::ffi::c_void, c"v@:@@");
        }
        let class: &AnyClass = match AnyClass::get("AylisPushCenterDelegate") {
            Some(c) => c,
            None => {
                let Some(mut b) = ClassBuilder::new("AylisPushCenterDelegate", class!(NSObject)) else { return };
                b.add_method(sel!(userNotificationCenter:willPresentNotification:withCompletionHandler:), will_present as extern "C" fn(_, _, _, _, _));
                b.add_method(sel!(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:), did_respond as extern "C" fn(_, _, _, _, _));
                b.register()
            }
        };
        // A central guarda o delegado sem reter: este fica para sempre.
        let object: Object = msg_send![class, new];
        let center: Object = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
        let _: () = msg_send![center, setDelegate: object];
    }
}

pub(crate) struct ApplePush;

impl ApplePush {
    /// Instala os delegados já na partida: o toque que abriu o app chega cedo.
    pub(crate) fn new() -> Self {
        install();
        ApplePush
    }
}

impl PushService for ApplePush {
    fn available(&self) -> bool {
        true
    }

    fn provider(&self) -> &'static str {
        "apns"
    }

    fn register(&mut self, config: &PushConfig) {
        FOREGROUND.store(config.show_in_foreground, Ordering::Relaxed);
        install();
        unsafe {
            let center: Object = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            let block = RcBlock::new(move |granted: Bool, _error: Object| {
                if !granted.as_bool() {
                    push_event(PushEvent::Error(aylis_core::messages::t("a permissão de notificar foi negada")));
                }
                // O token vale mesmo sem a permissão (mensagens silenciosas). Na thread
                // principal, como o sistema exige.
                let app = application();
                let _: () = msg_send![app, performSelectorOnMainThread: sel!(registerForRemoteNotifications), withObject: NIL, waitUntilDone: false];
            });
            // alerta | selo | som
            let _: () = msg_send![center, requestAuthorizationWithOptions: 7usize, completionHandler: &*block];
        }
    }

    fn unregister(&mut self) {
        unsafe {
            let app = application();
            let _: () = msg_send![app, performSelectorOnMainThread: sel!(unregisterForRemoteNotifications), withObject: NIL, waitUntilDone: false];
        }
    }
}
