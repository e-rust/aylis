//! Android: escolher e salvar arquivos (o seletor de documentos do sistema), as
//! permissões em tempo de execução e a biometria (`BiometricPrompt`), pela `AylisActivity`.
//!
//! As respostas do Java chegam por `nativeFilesPicked`/`nativeBiometricResult` com o número
//! do pedido; cada pedido guarda aqui a função que entrega a resposta ao aylis.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{Duration, Instant};

use aylis_core::platform::{
    BiometricResult, BiometricService, Biometry, FileDialogOptions, FileDialogService, Permission, PermissionService, PermissionState,
};
use jni::objects::{JByteArray, JClass, JString, JValue};
use jni::sys::{jint, jlong};
use jni::{JNIEnv, NativeMethod};

use super::android_jni::{string_array, with_env};

type FilesDone = Box<dyn FnOnce(Vec<String>) + Send>;
type BiometricDone = Box<dyn FnOnce(BiometricResult) + Send>;

static NEXT: AtomicI64 = AtomicI64::new(1);
static FILES: Mutex<Option<HashMap<i64, FilesDone>>> = Mutex::new(None);
static BIOMETRICS: Mutex<Option<HashMap<i64, BiometricDone>>> = Mutex::new(None);
type LinkDone = Box<dyn FnOnce(Result<String, String>) + Send>;
static LINKS: Mutex<Option<HashMap<i64, LinkDone>>> = Mutex::new(None);

fn keep<T>(slot: &Mutex<Option<HashMap<i64, T>>>, done: T) -> i64 {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    if let Ok(mut s) = slot.lock() {
        s.get_or_insert_with(HashMap::new).insert(id, done);
    }
    id
}

fn take<T>(slot: &Mutex<Option<HashMap<i64, T>>>, id: i64) -> Option<T> {
    slot.lock().ok()?.as_mut()?.remove(&id)
}

/// As funções nativas desta parte (registradas junto com as do teclado).
pub(crate) fn natives() -> Vec<NativeMethod> {
    vec![
        NativeMethod { name: "nativeFilesPicked".into(), sig: "(JLjava/lang/String;)V".into(), fn_ptr: native_files_picked as *mut _ },
        NativeMethod { name: "nativeBiometricResult".into(), sig: "(JILjava/lang/String;)V".into(), fn_ptr: native_biometric_result as *mut _ },
        NativeMethod { name: "nativeIncoming".into(), sig: "(ILjava/lang/String;)V".into(), fn_ptr: native_incoming as *mut _ },
        NativeMethod { name: "nativeLinkResult".into(), sig: "(JILjava/lang/String;)V".into(), fn_ptr: native_link_result as *mut _ },
    ]
}

extern "system" fn native_files_picked(mut env: JNIEnv, _class: JClass, request: jlong, paths: JString) {
    let paths: String = env.get_string(&paths).map(Into::into).unwrap_or_default();
    let list = paths.split('\n').filter(|p| !p.is_empty()).map(str::to_string).collect();
    if let Some(done) = take(&FILES, request) {
        done(list);
    }
}

extern "system" fn native_biometric_result(mut env: JNIEnv, _class: JClass, request: jlong, code: jint, message: JString) {
    let message: String = env.get_string(&message).map(Into::into).unwrap_or_default();
    if let Some(done) = take(&BIOMETRICS, request) {
        done(match code {
            0 => BiometricResult::Success,
            1 => BiometricResult::Cancelled,
            _ => BiometricResult::Failed(message),
        });
    }
}

/// A versão do Android (`Build.VERSION.SDK_INT`).
fn sdk() -> i32 {
    with_env(|env, _| env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()).unwrap_or(26)
}

// ---------------------------------------------------------------------------
// Arquivos

/// O tipo MIME de uma extensão (para o filtro do seletor).
fn mime(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "txt" | "log" => "text/plain",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "text/xml",
        "html" | "htm" => "text/html",
        "zip" => "application/zip",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        "mp4" => "video/mp4",
        "mkv" => "video/x-matroska",
        "webm" => "video/webm",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        _ => return None,
    })
}

pub(crate) struct AndroidFileDialogs;

impl FileDialogService for AndroidFileDialogs {
    fn show(&mut self, options: FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>) {
        let request = keep(&FILES, done);
        let ok = if options.save {
            let name = if options.file_name.is_empty() { "arquivo".to_string() } else { aylis_core::files::file_name(&options.file_name).to_string() };
            let ext = name.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
            let mime = mime(ext).unwrap_or("application/octet-stream");
            with_env(|env, a| {
                let jname = env.new_string(&name)?;
                let jmime = env.new_string(mime)?;
                env.call_method(a, "createDocument", "(JLjava/lang/String;Ljava/lang/String;)V", &[JValue::Long(request), (&jname).into(), (&jmime).into()]).map(|_| ())
            })
        } else {
            let mut types: Vec<&str> = options.filters.iter().flat_map(|(_, exts)| exts.iter()).filter_map(|e| mime(e)).collect();
            types.dedup();
            // Um filtro com "*" (ou extensão sem tipo conhecido) aceita tudo.
            if options.filters.iter().flat_map(|(_, e)| e.iter()).any(|e| e == "*" || mime(e).is_none()) {
                types.clear();
            }
            with_env(|env, a| {
                let array = string_array(env, types.iter().copied())?;
                env.call_method(a, "openDocument", "(JZ[Ljava/lang/String;)V", &[JValue::Long(request), JValue::Bool(options.multiple.into()), (&array).into()]).map(|_| ())
            })
        };
        if ok.is_none() {
            if let Some(done) = take(&FILES, request) {
                done(Vec::new());
            }
        }
    }
}

/// `content://`: o documento que o SaveDialog devolveu (gravar) ou outro URI (ler).
struct ContentUri;

impl aylis_core::files::FileScheme for ContentUri {
    fn read(&self, path: &str) -> std::io::Result<Vec<u8>> {
        with_env(|env, a| {
            let uri = env.new_string(path)?;
            let bytes = env.call_method(a, "readUri", "(Ljava/lang/String;)[B", &[(&uri).into()])?.l()?;
            if bytes.is_null() {
                return Ok(None);
            }
            env.convert_byte_array(JByteArray::from(bytes)).map(Some)
        })
        .flatten()
        .ok_or_else(|| std::io::Error::other(format!("não consegui ler {path}")))
    }

    fn write(&self, path: &str, data: &[u8]) -> std::io::Result<()> {
        let ok = with_env(|env, a| {
            let uri = env.new_string(path)?;
            let bytes = env.byte_array_from_slice(data)?;
            env.call_method(a, "writeUri", "(Ljava/lang/String;[B)Z", &[(&uri).into(), (&bytes).into()])?.z()
        });
        if ok == Some(true) { Ok(()) } else { Err(std::io::Error::other(format!("não consegui gravar em {path}"))) }
    }
}

pub(crate) fn install_files() {
    aylis_core::files::set_scheme("content://", ContentUri);
}

// ---------------------------------------------------------------------------
// Permissões

/// As permissões do Android de cada uma (a primeira é a que decide a situação).
fn android_names(p: Permission, sdk: i32) -> Vec<&'static str> {
    match p {
        Permission::Camera => vec!["android.permission.CAMERA"],
        Permission::Microphone => vec!["android.permission.RECORD_AUDIO"],
        Permission::Location => vec!["android.permission.ACCESS_FINE_LOCATION", "android.permission.ACCESS_COARSE_LOCATION"],
        Permission::Notifications if sdk >= 33 => vec!["android.permission.POST_NOTIFICATIONS"],
        Permission::Notifications => vec![],
        Permission::Photos if sdk >= 33 => vec!["android.permission.READ_MEDIA_IMAGES", "android.permission.READ_MEDIA_VIDEO"],
        Permission::Photos => vec!["android.permission.READ_EXTERNAL_STORAGE"],
        Permission::Contacts => vec!["android.permission.READ_CONTACTS"],
        Permission::Calendar => vec!["android.permission.READ_CALENDAR", "android.permission.WRITE_CALENDAR"],
        Permission::Bluetooth if sdk >= 31 => vec!["android.permission.BLUETOOTH_CONNECT", "android.permission.BLUETOOTH_SCAN"],
        Permission::Bluetooth => vec![],
    }
}

/// 1 liberada, 0 ainda não respondida, -1 negada (`AylisActivity.permissionState`).
fn android_state(name: &str) -> i32 {
    with_env(|env, a| {
        let n = env.new_string(name)?;
        env.call_method(a, "permissionState", "(Ljava/lang/String;)I", &[(&n).into()])?.i()
    })
    .unwrap_or(1)
}

pub(crate) struct AndroidPermissions;

impl PermissionService for AndroidPermissions {
    fn state(&self, permission: Permission) -> PermissionState {
        let names = android_names(permission, sdk());
        // Sem permissão em tempo de execução nesta versão: liberada pelo manifesto.
        let Some(first) = names.first() else { return PermissionState::Granted };
        match android_state(first) {
            1 => PermissionState::Granted,
            -1 => PermissionState::Denied,
            _ => PermissionState::Prompt,
        }
    }

    fn request(&mut self, permissions: Vec<Permission>, done: Box<dyn FnOnce(Vec<(Permission, PermissionState)>) + Send>) {
        // O sistema pergunta uma de cada vez; a thread espera cada resposta.
        std::thread::spawn(move || {
            let sdk = sdk();
            let mut answers = Vec::new();
            for p in permissions {
                let names = android_names(p, sdk);
                for name in &names {
                    let asked = with_env(|env, a| {
                        let n = env.new_string(name)?;
                        env.call_method(a, "ensurePermission", "(Ljava/lang/String;)Z", &[(&n).into()])?.z()
                    });
                    if asked == Some(false) {
                        let started = Instant::now();
                        while android_state(name) == 0 && started.elapsed() < Duration::from_secs(300) {
                            std::thread::sleep(Duration::from_millis(200));
                        }
                    }
                }
                let state = match names.first().map(|n| android_state(n)) {
                    None | Some(1) => PermissionState::Granted,
                    Some(-1) => PermissionState::Denied,
                    Some(_) => PermissionState::Prompt,
                };
                answers.push((p, state));
            }
            done(answers);
        });
    }
}

// ---------------------------------------------------------------------------
// Biometria

pub(crate) struct AndroidBiometrics;

impl BiometricService for AndroidBiometrics {
    fn availability(&self) -> Biometry {
        match with_env(|env, a| env.call_method(a, "biometricKind", "()I", &[])?.i()).unwrap_or(0) {
            1 => Biometry::Fingerprint,
            2 => Biometry::Face,
            3 => Biometry::Iris,
            4 => Biometry::Any,
            _ => Biometry::None,
        }
    }

    fn authenticate(&mut self, reason: &str, allow_device_credential: bool, done: Box<dyn FnOnce(BiometricResult) + Send>) {
        let request = keep(&BIOMETRICS, done);
        let ok = with_env(|env, a| {
            let title = env.new_string(reason)?;
            env.call_method(a, "authenticate", "(JLjava/lang/String;Z)V", &[JValue::Long(request), (&title).into(), JValue::Bool(allow_device_credential.into())]).map(|_| ())
        });
        if ok.is_none() {
            if let Some(done) = take(&BIOMETRICS, request) {
                done(BiometricResult::Failed(aylis_core::messages::t("biometria indisponível neste aparelho")));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AppLink: abrir endereços e apps, compartilhar, pedir a outro app, receber

extern "system" fn native_incoming(mut env: JNIEnv, _class: JClass, kind: jint, text: JString) {
    let text: String = env.get_string(&text).map(Into::into).unwrap_or_default();
    aylis_core::platform::push_incoming(if kind == 0 { aylis_core::platform::Incoming::Link(text) } else { aylis_core::platform::Incoming::Shared(text) });
}

extern "system" fn native_link_result(mut env: JNIEnv, _class: JClass, request: jlong, ok: jint, text: JString) {
    let text: String = env.get_string(&text).map(Into::into).unwrap_or_default();
    if let Some(done) = take(&LINKS, request) {
        done(if ok == 1 { Ok(text) } else if text.is_empty() { Err(aylis_core::messages::t("o outro app cancelou")) } else { Err(text) });
    }
}

pub(crate) struct AndroidLinks;

/// Chama um método `(String) -> boolean` da `AylisActivity`.
fn call_bool(method: &str, arg: &str) -> Option<bool> {
    with_env(|env, a| {
        let s = env.new_string(arg)?;
        env.call_method(a, method, "(Ljava/lang/String;)Z", &[(&s).into()])?.z()
    })
}

impl aylis_core::platform::LinkService for AndroidLinks {
    fn open_url(&mut self, url: &str) -> Result<(), String> {
        match call_bool("openUrl", url) {
            Some(true) => Ok(()),
            _ => Err(format!("{}: {url}", aylis_core::messages::t("nenhum app abre este endereço"))),
        }
    }

    fn open_app(&mut self, app: &str) -> Result<(), String> {
        match call_bool("openApp", app) {
            Some(true) => Ok(()),
            _ => Err(format!("{}: {app}", aylis_core::messages::t("app não encontrado (no Android 11+, liste o pacote em <queries> no manifesto)"))),
        }
    }

    fn share(&mut self, text: &str, files: &[String]) -> Result<(), String> {
        let ok = with_env(|env, a| {
            let t = env.new_string(text)?;
            let list = string_array(env, files.iter().map(String::as_str))?;
            env.call_method(a, "share", "(Ljava/lang/String;[Ljava/lang/String;)Z", &[(&t).into(), (&list).into()])?.z()
        });
        if ok == Some(true) { Ok(()) } else { Err(aylis_core::messages::t("não consegui compartilhar")) }
    }

    fn request(&mut self, action: &str, data: &str, done: Box<dyn FnOnce(Result<String, String>) + Send>) {
        let request = keep(&LINKS, done);
        let ok = with_env(|env, a| {
            let (act, dat) = (env.new_string(action)?, env.new_string(data)?);
            env.call_method(a, "request", "(JLjava/lang/String;Ljava/lang/String;)V", &[JValue::Long(request), (&act).into(), (&dat).into()]).map(|_| ())
        });
        if ok.is_none() {
            if let Some(done) = take(&LINKS, request) {
                done(Err(aylis_core::messages::t("pedir a outro app não existe aqui")));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Sensores, pela AylisActivity

pub(crate) struct AndroidSensors;

/// O número do sensor no Java (a ordem do `Sensor` do aylis).
fn sensor_kind(sensor: aylis_core::platform::Sensor) -> i32 {
    use aylis_core::platform::Sensor::*;
    match sensor {
        Location => 0,
        Battery => 1,
        Network => 2,
        Motion => 3,
        Compass => 4,
        Proximity => 5,
        Light => 6,
    }
}

impl aylis_core::platform::SensorService for AndroidSensors {
    fn available(&self, sensor: aylis_core::platform::Sensor) -> bool {
        with_env(|env, a| env.call_method(a, "sensorAvailable", "(I)Z", &[JValue::Int(sensor_kind(sensor))])?.z()).unwrap_or(false)
    }

    fn set_active(&mut self, sensor: aylis_core::platform::Sensor, on: bool) {
        with_env(|env, a| env.call_method(a, "sensorActive", "(IZ)V", &[JValue::Int(sensor_kind(sensor)), JValue::Bool(on.into())]).map(|_| ()));
    }

    fn read(&mut self, sensor: aylis_core::platform::Sensor) -> Option<Vec<f64>> {
        with_env(|env, a| {
            let array = env.call_method(a, "sensorRead", "(I)[D", &[JValue::Int(sensor_kind(sensor))])?.l()?;
            if array.is_null() {
                return Ok(None);
            }
            let array = jni::objects::JDoubleArray::from(array);
            let mut values = vec![0.0; env.get_array_length(&array)? as usize];
            env.get_double_array_region(&array, 0, &mut values)?;
            Ok(Some(values))
        })
        .flatten()
    }

    fn vibrate(&mut self, ms: u32) -> bool {
        with_env(|env, a| env.call_method(a, "vibrate", "(I)Z", &[JValue::Int(ms.min(i32::MAX as u32) as i32)])?.z()).unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// BackgroundService: o serviço em primeiro plano

pub(crate) struct AndroidBackground;

impl aylis_core::platform::BackgroundHost for AndroidBackground {
    fn start(&mut self, title: &str, text: &str, kind: &str, keep_alive: bool) -> Result<(), String> {
        with_env(|env, a| {
            let (t, x, k) = (env.new_string(title)?, env.new_string(text)?, env.new_string(kind)?);
            env.call_method(a, "startBackground", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V", &[(&t).into(), (&x).into(), (&k).into(), JValue::Bool(keep_alive.into())])
                .map(|_| ())
        })
        .ok_or_else(|| aylis_core::messages::t("não consegui ligar o serviço em segundo plano"))
    }

    fn stop(&mut self) {
        with_env(|env, a| env.call_method(a, "stopBackground", "()V", &[]).map(|_| ()));
    }
}

// ---------------------------------------------------------------------------
// Phone e LockScreen

pub(crate) struct AndroidPhone;

/// Chama um método `()` ou `() -> boolean` da `AylisActivity`.
fn call_void(method: &str) {
    with_env(|env, a| env.call_method(a, method, "()V", &[]).map(|_| ()));
}

fn call_flag(method: &str) -> bool {
    with_env(|env, a| env.call_method(a, method, "()Z", &[])?.z()).unwrap_or(false)
}

impl aylis_core::platform::PhoneService for AndroidPhone {
    fn dial(&mut self, number: &str) -> Result<(), String> {
        if call_bool("dial", number) == Some(true) { Ok(()) } else { Err(aylis_core::messages::t("este aparelho não tem discador")) }
    }

    fn call(&mut self, number: &str) -> Result<(), String> {
        if call_bool("call", number) == Some(true) { Ok(()) } else { Err(aylis_core::messages::t("sem a permissão de ligar (pedida agora; tente de novo)")) }
    }

    fn watch(&mut self, on: bool) {
        with_env(|env, a| env.call_method(a, "watchCalls", "(Z)V", &[JValue::Bool(on.into())]).map(|_| ()));
    }

    fn state(&mut self) -> (String, String) {
        let text = with_env(|env, a| {
            let s: JString = env.call_method(a, "callInfo", "()Ljava/lang/String;", &[])?.l()?.into();
            Ok(env.get_string(&s).map(String::from).unwrap_or_default())
        })
        .unwrap_or_default();
        let (state, number) = text.split_once(';').unwrap_or(("idle", ""));
        (state.to_string(), number.to_string())
    }

    fn request_dialer(&mut self) -> Result<(), String> {
        call_void("requestDialer");
        Ok(())
    }

    fn answer(&mut self) -> bool {
        call_flag("answer")
    }

    fn reject(&mut self) -> bool {
        call_flag("reject")
    }

    fn hang_up(&mut self) -> bool {
        call_flag("hangUp")
    }

    fn history(&mut self, max: u32) -> Result<String, String> {
        with_env(|env, a| {
            let s = env.call_method(a, "callHistory", "(I)Ljava/lang/String;", &[JValue::Int(max as i32)])?.l()?;
            if s.is_null() {
                return Ok(None);
            }
            Ok(env.get_string(&JString::from(s)).map(String::from).ok())
        })
        .flatten()
        .ok_or_else(|| aylis_core::messages::t("sem a permissão do histórico de chamadas (pedida agora; tente de novo)"))
    }

    fn lock_screen(&mut self, show_when_locked: bool, turn_screen_on: bool) {
        with_env(|env, a| env.call_method(a, "setLockScreen", "(ZZ)V", &[JValue::Bool(show_when_locked.into()), JValue::Bool(turn_screen_on.into())]).map(|_| ()));
    }

    fn dismiss_keyguard(&mut self) {
        call_void("dismissKeyguard");
    }
}
