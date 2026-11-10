//! iOS: escolher arquivos (`UIDocumentPickerViewController`, com cópia para o app), salvar
//! na pasta Documentos do app (aparece no app Arquivos), as permissões de cada framework
//! do sistema e a biometria (`LAContext`: Touch ID, Face ID).
//!
//! Os textos que o iOS mostra nas perguntas vêm do Info.plist, que a publicação monta com o
//! `reasons` do `Permissions` e o `reason` do `Biometrics`.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use block2::RcBlock;
use aylis_core::platform::{
    BiometricResult, BiometricService, Biometry, FileDialogOptions, FileDialogService, Permission, PermissionService, PermissionState,
};
use objc2::declare::ClassBuilder;
use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
use objc2::{class, msg_send, sel};
use objc2_foundation::NSString;

// Os frameworks usados só pelo nome da classe: o link garante que estão no app.
#[link(name = "LocalAuthentication", kind = "framework")]
#[link(name = "AVFoundation", kind = "framework")]
#[link(name = "Photos", kind = "framework")]
#[link(name = "UserNotifications", kind = "framework")]
#[link(name = "CoreLocation", kind = "framework")]
#[link(name = "Contacts", kind = "framework")]
#[link(name = "EventKit", kind = "framework")]
#[link(name = "CoreBluetooth", kind = "framework")]
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
unsafe extern "C" {}

type Object = *mut AnyObject;
const NIL: Object = std::ptr::null_mut();

fn ns(text: &str) -> objc2::rc::Retained<NSString> {
    NSString::from_str(text)
}

/// Um `NSString` do Objective-C como `String`.
unsafe fn string(value: Object) -> String {
    if value.is_null() {
        return String::new();
    }
    let utf8: *const std::ffi::c_char = unsafe { msg_send![value, UTF8String] };
    if utf8.is_null() { String::new() } else { unsafe { std::ffi::CStr::from_ptr(utf8) }.to_string_lossy().into_owned() }
}

// ---------------------------------------------------------------------------
// Arquivos

type FilesDone = Box<dyn FnOnce(Vec<String>) + Send>;

/// O pedido aberto (um seletor de cada vez) e o delegado dele.
static PICKING: Mutex<Option<FilesDone>> = Mutex::new(None);
static DELEGATE: Mutex<usize> = Mutex::new(0);

fn finish(paths: Vec<String>) {
    let done = PICKING.lock().ok().and_then(|mut p| p.take());
    if let Some(done) = done {
        done(paths);
    }
}

extern "C" fn picked(_this: *mut AnyObject, _cmd: Sel, _picker: Object, urls: Object) {
    let mut paths = Vec::new();
    unsafe {
        let count: usize = msg_send![urls, count];
        for i in 0..count {
            let url: Object = msg_send![urls, objectAtIndex: i];
            let path: Object = msg_send![url, path];
            let path = string(path);
            if !path.is_empty() {
                paths.push(path);
            }
        }
    }
    finish(paths);
}

extern "C" fn cancelled(_this: *mut AnyObject, _cmd: Sel, _picker: Object) {
    finish(Vec::new());
}

/// O delegado do seletor (uma classe criada uma vez, um objeto guardado para sempre).
fn delegate() -> Object {
    let mut slot = DELEGATE.lock().unwrap_or_else(|e| e.into_inner());
    if *slot == 0 {
        let class: &AnyClass = match AnyClass::get("AylisDocumentPickerDelegate") {
            Some(c) => c,
            None => {
                let Some(mut b) = ClassBuilder::new("AylisDocumentPickerDelegate", class!(NSObject)) else { return NIL };
                unsafe {
                    b.add_method(sel!(documentPicker:didPickDocumentsAtURLs:), picked as extern "C" fn(_, _, _, _));
                    b.add_method(sel!(documentPickerWasCancelled:), cancelled as extern "C" fn(_, _, _));
                }
                b.register()
            }
        };
        let object: Object = unsafe { msg_send![class, new] };
        *slot = object as usize;
    }
    *slot as Object
}

/// O controlador de cima (para apresentar o seletor por cima do app).
unsafe fn top_controller() -> Object {
    unsafe {
        let app: Object = msg_send![class!(UIApplication), sharedApplication];
        let window: Object = msg_send![app, keyWindow];
        if window.is_null() {
            return NIL;
        }
        let mut controller: Object = msg_send![window, rootViewController];
        loop {
            if controller.is_null() {
                return NIL;
            }
            let presented: Object = msg_send![controller, presentedViewController];
            if presented.is_null() {
                return controller;
            }
            controller = presented;
        }
    }
}

/// A pasta Documentos do app (o app Arquivos mostra, com `UIFileSharingEnabled`).
fn documents_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    std::path::Path::new(&home).join("Documents")
}

pub(crate) struct IosFileDialogs;

impl FileDialogService for IosFileDialogs {
    fn show(&mut self, options: FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>) {
        // Salvar: um arquivo na pasta Documentos do app (o app grava nele).
        if options.save {
            let name = if options.file_name.is_empty() { "arquivo".to_string() } else { aylis_core::files::file_name(&options.file_name).to_string() };
            let dir = documents_dir();
            let _ = std::fs::create_dir_all(&dir);
            done(vec![dir.join(name).display().to_string()]);
            return;
        }
        if let Ok(mut p) = PICKING.lock() {
            // Um seletor anterior que não respondeu: fica sem resposta.
            if let Some(old) = p.replace(done) {
                old(Vec::new());
            }
        }
        unsafe {
            // Os tipos (UTType) das extensões do filtro; sem filtro, qualquer item.
            let types: Object = msg_send![class!(NSMutableArray), array];
            let exts: Vec<&String> = options.filters.iter().flat_map(|(_, e)| e.iter()).filter(|e| *e != "*").collect();
            for ext in &exts {
                let t: Object = msg_send![class!(UTType), typeWithFilenameExtension: &*ns(ext)];
                if !t.is_null() {
                    let _: () = msg_send![types, addObject: t];
                }
            }
            let count: usize = msg_send![types, count];
            if count == 0 {
                let item: Object = msg_send![class!(UTType), typeWithIdentifier: &*ns("public.item")];
                let _: () = msg_send![types, addObject: item];
            }
            let picker: Object = msg_send![class!(UIDocumentPickerViewController), alloc];
            let picker: Object = msg_send![picker, initForOpeningContentTypes: types, asCopy: Bool::YES];
            let _: () = msg_send![picker, setAllowsMultipleSelection: Bool::new(options.multiple)];
            let _: () = msg_send![picker, setDelegate: delegate()];
            let top = top_controller();
            if top.is_null() || picker.is_null() {
                finish(Vec::new());
                return;
            }
            let _: () = msg_send![top, presentViewController: picker, animated: Bool::YES, completion: NIL];
        }
    }
}

// ---------------------------------------------------------------------------
// Permissões

/// A situação de agora (os códigos de cada framework viram os do aylis).
fn state_of(p: Permission) -> PermissionState {
    unsafe {
        match p {
            Permission::Camera | Permission::Microphone => {
                let media = ns(if p == Permission::Camera { "vide" } else { "soun" });
                let s: isize = msg_send![class!(AVCaptureDevice), authorizationStatusForMediaType: &*media];
                av_state(s)
            }
            Permission::Photos => {
                // PHAccessLevelReadWrite = 2; 3 liberada, 4 limitada (algumas fotos).
                let s: isize = msg_send![class!(PHPhotoLibrary), authorizationStatusForAccessLevel: 2isize];
                match s {
                    0 => PermissionState::Prompt,
                    3 | 4 => PermissionState::Granted,
                    _ => PermissionState::Denied,
                }
            }
            Permission::Location => {
                let manager = location_manager();
                let s: i32 = msg_send![manager, authorizationStatus];
                match s {
                    0 => PermissionState::Prompt,
                    3 | 4 => PermissionState::Granted,
                    _ => PermissionState::Denied,
                }
            }
            Permission::Contacts => {
                let s: isize = msg_send![class!(CNContactStore), authorizationStatusForEntityType: 0isize];
                av_state(s)
            }
            Permission::Calendar => {
                let s: isize = msg_send![class!(EKEventStore), authorizationStatusForEntityType: 0isize];
                match s {
                    0 => PermissionState::Prompt,
                    3 => PermissionState::Granted,
                    _ => PermissionState::Denied,
                }
            }
            Permission::Bluetooth => {
                let s: isize = msg_send![class!(CBManager), authorization];
                av_state(s)
            }
            Permission::Notifications => notification_state(),
        }
    }
}

/// 0 não perguntado, 1 restrito, 2 negado, 3 liberado (AVFoundation, Contacts, CoreBluetooth).
fn av_state(s: isize) -> PermissionState {
    match s {
        0 => PermissionState::Prompt,
        3 => PermissionState::Granted,
        _ => PermissionState::Denied,
    }
}

/// O gerente de localização (um só, vivo: a pergunta some se ele for liberado).
fn location_manager() -> Object {
    static MANAGER: Mutex<usize> = Mutex::new(0);
    let mut slot = MANAGER.lock().unwrap_or_else(|e| e.into_inner());
    if *slot == 0 {
        let m: Object = unsafe { msg_send![class!(CLLocationManager), new] };
        *slot = m as usize;
    }
    *slot as Object
}

/// As notificações só respondem de forma assíncrona: a última resposta fica guardada.
static NOTIFICATIONS: Mutex<Option<PermissionState>> = Mutex::new(None);

fn notification_state() -> PermissionState {
    refresh_notifications();
    NOTIFICATIONS.lock().ok().and_then(|s| *s).unwrap_or(PermissionState::Prompt)
}

fn refresh_notifications() {
    unsafe {
        let center: Object = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
        let block = RcBlock::new(|settings: Object| {
            // 0 não perguntado, 1 negado, 2 liberado, 3 provisório, 4 temporário.
            let s: isize = msg_send![settings, authorizationStatus];
            let state = match s {
                0 => PermissionState::Prompt,
                1 => PermissionState::Denied,
                _ => PermissionState::Granted,
            };
            if let Ok(mut slot) = NOTIFICATIONS.lock() {
                *slot = Some(state);
            }
        });
        let _: () = msg_send![center, getNotificationSettingsWithCompletionHandler: &*block];
    }
}

/// Pede uma permissão (a pergunta do sistema) e espera a resposta.
fn ask(p: Permission) {
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    unsafe {
        match p {
            Permission::Camera | Permission::Microphone => {
                let media = ns(if p == Permission::Camera { "vide" } else { "soun" });
                let tx = tx.clone();
                let block = RcBlock::new(move |_granted: Bool| {
                    let _ = tx.send(());
                });
                let _: () = msg_send![class!(AVCaptureDevice), requestAccessForMediaType: &*media, completionHandler: &*block];
            }
            Permission::Photos => {
                let tx = tx.clone();
                let block = RcBlock::new(move |_status: isize| {
                    let _ = tx.send(());
                });
                let _: () = msg_send![class!(PHPhotoLibrary), requestAuthorizationForAccessLevel: 2isize, handler: &*block];
            }
            Permission::Notifications => {
                let center: Object = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
                let tx = tx.clone();
                let block = RcBlock::new(move |_granted: Bool, _error: Object| {
                    let _ = tx.send(());
                });
                // alerta | selo | som
                let _: () = msg_send![center, requestAuthorizationWithOptions: 7usize, completionHandler: &*block];
            }
            Permission::Contacts => {
                let store: Object = msg_send![class!(CNContactStore), new];
                let tx = tx.clone();
                let block = RcBlock::new(move |_granted: Bool, _error: Object| {
                    let _ = tx.send(());
                });
                let _: () = msg_send![store, requestAccessForEntityType: 0isize, completionHandler: &*block];
            }
            Permission::Calendar => {
                let store: Object = msg_send![class!(EKEventStore), new];
                let tx = tx.clone();
                let block = RcBlock::new(move |_granted: Bool, _error: Object| {
                    let _ = tx.send(());
                });
                let full: bool = msg_send![store, respondsToSelector: sel!(requestFullAccessToEventsWithCompletion:)];
                if full {
                    let _: () = msg_send![store, requestFullAccessToEventsWithCompletion: &*block];
                } else {
                    let _: () = msg_send![store, requestAccessToEntityType: 0isize, completion: &*block];
                }
            }
            Permission::Location => {
                let _: () = msg_send![location_manager(), requestWhenInUseAuthorization];
            }
            Permission::Bluetooth => {
                // Criar o gerente faz a pergunta; ele não é liberado (some com a pergunta).
                let _central: Object = msg_send![class!(CBCentralManager), new];
            }
        }
    }
    // Localização e bluetooth não têm resposta própria: espera a situação mudar.
    let started = Instant::now();
    loop {
        if rx.recv_timeout(Duration::from_millis(250)).is_ok() {
            break;
        }
        if matches!(p, Permission::Location | Permission::Bluetooth) && state_of(p) != PermissionState::Prompt {
            break;
        }
        if started.elapsed() > Duration::from_secs(300) {
            break;
        }
    }
}

pub(crate) struct IosPermissions;

impl IosPermissions {
    pub(crate) fn new() -> Self {
        refresh_notifications();
        Self
    }
}

impl PermissionService for IosPermissions {
    fn state(&self, permission: Permission) -> PermissionState {
        state_of(permission)
    }

    fn request(&mut self, permissions: Vec<Permission>, done: Box<dyn FnOnce(Vec<(Permission, PermissionState)>) + Send>) {
        // Uma pergunta por vez; a espera fica numa thread (a interface segue).
        std::thread::spawn(move || {
            let mut answers = Vec::new();
            for p in permissions {
                if state_of(p) == PermissionState::Prompt {
                    ask(p);
                }
                if p == Permission::Notifications {
                    refresh_notifications();
                    std::thread::sleep(Duration::from_millis(200));
                }
                answers.push((p, state_of(p)));
            }
            done(answers);
        });
    }
}

// ---------------------------------------------------------------------------
// Biometria

/// LAPolicyDeviceOwnerAuthenticationWithBiometrics = 1; ...Authentication (com o código) = 2.
fn policy(allow_device_credential: bool) -> isize {
    if allow_device_credential { 2 } else { 1 }
}

pub(crate) struct IosBiometrics;

impl BiometricService for IosBiometrics {
    fn availability(&self) -> Biometry {
        unsafe {
            let context: Object = msg_send![class!(LAContext), new];
            let ok: Bool = msg_send![context, canEvaluatePolicy: 1isize, error: std::ptr::null_mut::<Object>()];
            let kind: isize = msg_send![context, biometryType];
            let _: () = msg_send![context, release];
            if !ok.as_bool() {
                return Biometry::None;
            }
            // 1 Touch ID, 2 Face ID, 4 Optic ID (íris).
            match kind {
                1 => Biometry::Fingerprint,
                2 => Biometry::Face,
                4 => Biometry::Iris,
                _ => Biometry::Any,
            }
        }
    }

    fn authenticate(&mut self, reason: &str, allow_device_credential: bool, done: Box<dyn FnOnce(BiometricResult) + Send>) {
        let done = Mutex::new(Some(done));
        unsafe {
            let context: Object = msg_send![class!(LAContext), new];
            let block = RcBlock::new(move |success: Bool, error: Object| {
                let Some(done) = done.lock().ok().and_then(|mut d| d.take()) else { return };
                if success.as_bool() {
                    done(BiometricResult::Success);
                    return;
                }
                // LAError: -2 usuário cancelou, -4 sistema cancelou, -9 app cancelou, -3 pediu a senha.
                let code: isize = if error.is_null() { 0 } else { msg_send![error, code] };
                if matches!(code, -2 | -3 | -4 | -9) {
                    done(BiometricResult::Cancelled);
                } else {
                    let message: Object = if error.is_null() { NIL } else { msg_send![error, localizedDescription] };
                    done(BiometricResult::Failed(string(message)));
                }
            });
            let _: () = msg_send![context, evaluatePolicy: policy(allow_device_credential), localizedReason: &*ns(reason), reply: &*block];
            // O contexto vive até a resposta (o sistema segura enquanto pergunta).
        }
    }
}
