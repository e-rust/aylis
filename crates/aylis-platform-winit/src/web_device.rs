//! Navegador: escolher arquivos (`<input type="file">`, o conteúdo fica na memória:
//! `aylis_core::files`), salvar baixando, permissões pela Permissions API e biometria pelo
//! WebAuthn com o autenticador do próprio aparelho (digital, rosto, Windows Hello).
//!
//! Tudo pelo `js_sys::Reflect`: as APIs mudam entre navegadores, e o que faltar vira
//! "indisponível" em vez de erro.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use aylis_core::platform::{
    BiometricResult, BiometricService, Biometry, FileDialogOptions, FileDialogService, Permission, PermissionService, PermissionState,
};
use js_sys::{Array, Function, Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};

pub(crate) fn get(target: &JsValue, key: &str) -> JsValue {
    Reflect::get(target, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
}

pub(crate) fn set(target: &JsValue, key: &str, value: impl Into<JsValue>) {
    let _ = Reflect::set(target, &JsValue::from_str(key), &value.into());
}

pub(crate) fn call(target: &JsValue, method: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let f: Function = get(target, method).dyn_into()?;
    f.apply(target, &args.iter().collect::<Array>())
}

pub(crate) fn window() -> JsValue {
    js_sys::global().into()
}

fn document() -> JsValue {
    get(&window(), "document")
}

pub(crate) fn object() -> JsValue {
    Object::new().into()
}

pub(crate) async fn wait(value: Result<JsValue, JsValue>) -> Result<JsValue, JsValue> {
    let promise: Promise = value?.dyn_into()?;
    JsFuture::from(promise).await
}

// ---------------------------------------------------------------------------
// Arquivos

pub(crate) struct WebFileDialogs;

/// Salvar no navegador: `download:/nome` baixa o que for gravado.
struct Download;

impl aylis_core::files::FileScheme for Download {
    fn read(&self, path: &str) -> std::io::Result<Vec<u8>> {
        Err(std::io::Error::new(std::io::ErrorKind::Unsupported, format!("{path}: só gravação (baixa o arquivo)")))
    }

    fn write(&self, path: &str, data: &[u8]) -> std::io::Result<()> {
        let fail = |e: JsValue| std::io::Error::other(format!("{e:?}"));
        let bytes = Uint8Array::from(data);
        let parts = Array::of1(&bytes);
        let blob = js_sys::Reflect::construct(&get(&window(), "Blob").dyn_into::<Function>().map_err(fail)?, &Array::of1(&parts)).map_err(fail)?;
        let url_class = get(&window(), "URL");
        let url = call(&url_class, "createObjectURL", &[blob]).map_err(fail)?;
        let link = call(&document(), "createElement", &[JsValue::from_str("a")]).map_err(fail)?;
        set(&link, "href", url.clone());
        set(&link, "download", aylis_core::files::file_name(path));
        call(&link, "click", &[]).map_err(fail)?;
        let _ = call(&url_class, "revokeObjectURL", &[url]);
        Ok(())
    }
}

pub(crate) fn install_files() {
    aylis_core::files::set_scheme("download:/", Download);
}

impl FileDialogService for WebFileDialogs {
    fn show(&mut self, options: FileDialogOptions, done: Box<dyn FnOnce(Vec<String>) + Send>) {
        // Salvar: o navegador baixa o que o app gravar nesse caminho.
        if options.save {
            let name = if options.file_name.is_empty() { "arquivo".to_string() } else { options.file_name.clone() };
            done(vec![format!("download:/{}", aylis_core::files::file_name(&name))]);
            return;
        }
        let Ok(input) = call(&document(), "createElement", &[JsValue::from_str("input")]) else {
            done(Vec::new());
            return;
        };
        set(&input, "type", "file");
        set(&input, "multiple", options.multiple);
        let accept: Vec<String> = options.filters.iter().flat_map(|(_, exts)| exts.iter()).filter(|e| *e != "*").map(|e| format!(".{e}")).collect();
        if !accept.is_empty() {
            set(&input, "accept", accept.join(","));
        }
        // Responde uma vez só: `change` (escolheu) ou `cancel` (fechou).
        let done = Rc::new(RefCell::new(Some(done)));
        let on_change = {
            let done = done.clone();
            let input = input.clone();
            Closure::<dyn FnMut()>::new(move || {
                let Some(done) = done.borrow_mut().take() else { return };
                let files = get(&input, "files");
                let count = get(&files, "length").as_f64().unwrap_or(0.0) as u32;
                let picked: Vec<JsValue> = (0..count).filter_map(|i| call(&files, "item", &[JsValue::from(i)]).ok()).collect();
                spawn_local(async move {
                    let mut paths = Vec::new();
                    for file in picked {
                        let name = get(&file, "name").as_string().unwrap_or_default();
                        if let Ok(buffer) = wait(call(&file, "arrayBuffer", &[])).await {
                            paths.push(aylis_core::files::put_memory(&name, Uint8Array::new(&buffer).to_vec()));
                        }
                    }
                    done(paths);
                });
            })
        };
        let on_cancel = {
            let done = done.clone();
            Closure::<dyn FnMut()>::new(move || {
                if let Some(done) = done.borrow_mut().take() {
                    done(Vec::new());
                }
            })
        };
        let _ = call(&input, "addEventListener", &[JsValue::from_str("change"), on_change.as_ref().clone()]);
        let _ = call(&input, "addEventListener", &[JsValue::from_str("cancel"), on_cancel.as_ref().clone()]);
        // Os tratadores vivem enquanto o campo existir.
        on_change.forget();
        on_cancel.forget();
        let _ = call(&input, "click", &[]);
    }
}

// ---------------------------------------------------------------------------
// Permissões

/// O nome na Permissions API (`None`: o navegador não tem).
fn web_name(p: Permission) -> Option<&'static str> {
    match p {
        Permission::Camera => Some("camera"),
        Permission::Microphone => Some("microphone"),
        Permission::Location => Some("geolocation"),
        Permission::Notifications => Some("notifications"),
        _ => None,
    }
}

fn parse_state(state: &str) -> PermissionState {
    match state {
        "granted" => PermissionState::Granted,
        "denied" => PermissionState::Denied,
        _ => PermissionState::Prompt,
    }
}

/// As respostas da Permissions API ficam guardadas (ela é assíncrona; `state` não).
#[derive(Clone, Default)]
pub(crate) struct WebPermissions {
    known: Arc<Mutex<HashMap<Permission, PermissionState>>>,
}

impl WebPermissions {
    pub(crate) fn new() -> Self {
        let me = Self::default();
        for p in Permission::ALL {
            me.refresh(p, None);
        }
        me
    }

    /// Consulta de novo; com `then`, avisa quando souber.
    fn refresh(&self, p: Permission, then: Option<Box<dyn FnOnce(PermissionState)>>) {
        let known = self.known.clone();
        spawn_local(async move {
            let state = query(p).await;
            if let Ok(mut k) = known.lock() {
                k.insert(p, state);
            }
            if let Some(then) = then {
                then(state);
            }
        });
    }
}

async fn query(p: Permission) -> PermissionState {
    let Some(name) = web_name(p) else { return PermissionState::Unavailable };
    // Notificações: o próprio Notification.permission.
    if p == Permission::Notifications {
        let n = get(&window(), "Notification");
        if n.is_undefined() {
            return PermissionState::Unavailable;
        }
        return match get(&n, "permission").as_string().as_deref() {
            Some("default") | None => PermissionState::Prompt,
            Some(s) => parse_state(s),
        };
    }
    let permissions = get(&get(&window(), "navigator"), "permissions");
    if permissions.is_undefined() {
        return PermissionState::Prompt;
    }
    let desc = object();
    set(&desc, "name", name);
    match wait(call(&permissions, "query", &[desc])).await {
        Ok(status) => parse_state(&get(&status, "state").as_string().unwrap_or_default()),
        // Firefox não consulta câmera e microfone: só pedindo se sabe.
        Err(_) => PermissionState::Prompt,
    }
}

/// Pede uma permissão do jeito que o navegador pergunta: usando o recurso.
async fn ask(p: Permission) {
    let navigator = get(&window(), "navigator");
    match p {
        Permission::Camera | Permission::Microphone => {
            let media = get(&navigator, "mediaDevices");
            let constraints = object();
            set(&constraints, if p == Permission::Camera { "video" } else { "audio" }, true);
            if let Ok(stream) = wait(call(&media, "getUserMedia", &[constraints])).await {
                // Só queria a permissão: solta o aparelho.
                if let Ok(tracks) = call(&stream, "getTracks", &[]) {
                    for t in Array::from(&tracks).iter() {
                        let _ = call(&t, "stop", &[]);
                    }
                }
            }
        }
        Permission::Notifications => {
            let n = get(&window(), "Notification");
            let _ = wait(call(&n, "requestPermission", &[])).await;
        }
        Permission::Location => {
            let geo = get(&navigator, "geolocation");
            let promise = Promise::new(&mut |resolve, _| {
                let ok = resolve.clone();
                let on_ok = Closure::once_into_js(move |_: JsValue| {
                    let _ = ok.call0(&JsValue::NULL);
                });
                let on_err = Closure::once_into_js(move |_: JsValue| {
                    let _ = resolve.call0(&JsValue::NULL);
                });
                let _ = call(&geo, "getCurrentPosition", &[on_ok, on_err]);
            });
            let _ = JsFuture::from(promise).await;
        }
        _ => {}
    }
}

impl PermissionService for WebPermissions {
    fn state(&self, permission: Permission) -> PermissionState {
        if web_name(permission).is_none() {
            return PermissionState::Unavailable;
        }
        self.known.lock().ok().and_then(|k| k.get(&permission).copied()).unwrap_or(PermissionState::Prompt)
    }

    fn request(&mut self, permissions: Vec<Permission>, done: Box<dyn FnOnce(Vec<(Permission, PermissionState)>) + Send>) {
        let known = self.known.clone();
        // Uma de cada vez: o navegador mostra uma pergunta por vez.
        spawn_local(async move {
            let mut answers = Vec::new();
            for p in permissions {
                if web_name(p).is_none() {
                    answers.push((p, PermissionState::Unavailable));
                    continue;
                }
                if query(p).await == PermissionState::Prompt {
                    ask(p).await;
                }
                let state = query(p).await;
                if let Ok(mut k) = known.lock() {
                    k.insert(p, state);
                }
                answers.push((p, state));
            }
            done(answers);
        });
    }
}

// ---------------------------------------------------------------------------
// Biometria (WebAuthn)

const CREDENTIAL_KEY: &str = "aylis.webauthn.credential";

#[derive(Clone, Default)]
pub(crate) struct WebBiometrics {
    available: Arc<Mutex<Option<bool>>>,
}

impl WebBiometrics {
    pub(crate) fn new() -> Self {
        let me = Self::default();
        let available = me.available.clone();
        spawn_local(async move {
            let class = get(&window(), "PublicKeyCredential");
            let ok = if class.is_undefined() {
                false
            } else {
                wait(call(&class, "isUserVerifyingPlatformAuthenticatorAvailable", &[])).await.ok().and_then(|v| v.as_bool()).unwrap_or(false)
            };
            if let Ok(mut a) = available.lock() {
                *a = Some(ok);
            }
        });
        me
    }
}

fn random_bytes(n: u32) -> Uint8Array {
    let bytes = Uint8Array::new_with_length(n);
    let _ = call(&get(&window(), "crypto"), "getRandomValues", &[bytes.clone().into()]);
    bytes
}

fn storage() -> JsValue {
    get(&window(), "localStorage")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).filter_map(|i| u8::from_str_radix(text.get(i * 2..i * 2 + 2)?, 16).ok()).collect()
}

/// Verifica com a credencial guardada; sem ela, cria uma (a criação já exige a verificação).
async fn verify() -> BiometricResult {
    let credentials = get(&get(&window(), "navigator"), "credentials");
    let public_key = object();
    set(&public_key, "challenge", random_bytes(32));
    set(&public_key, "userVerification", "required");
    set(&public_key, "timeout", 60_000);
    let saved = call(&storage(), "getItem", &[JsValue::from_str(CREDENTIAL_KEY)]).ok().and_then(|v| v.as_string());
    let result = match saved {
        Some(id) => {
            let allowed = object();
            set(&allowed, "type", "public-key");
            set(&allowed, "id", Uint8Array::from(unhex(&id).as_slice()));
            set(&public_key, "allowCredentials", Array::of1(&allowed));
            let options = object();
            set(&options, "publicKey", public_key);
            wait(call(&credentials, "get", &[options])).await
        }
        None => {
            let rp = object();
            set(&rp, "name", get(&document(), "title").as_string().filter(|t| !t.is_empty()).unwrap_or_else(|| "aylis".into()));
            let user = object();
            set(&user, "id", random_bytes(16));
            set(&user, "name", "aylis");
            set(&user, "displayName", "aylis");
            let algorithms = Array::new();
            for alg in [-7, -257] {
                let p = object();
                set(&p, "type", "public-key");
                set(&p, "alg", alg);
                algorithms.push(&p);
            }
            let selection = object();
            set(&selection, "authenticatorAttachment", "platform");
            set(&selection, "userVerification", "required");
            set(&public_key, "rp", rp);
            set(&public_key, "user", user);
            set(&public_key, "pubKeyCredParams", algorithms);
            set(&public_key, "authenticatorSelection", selection);
            let options = object();
            set(&options, "publicKey", public_key);
            let created = wait(call(&credentials, "create", &[options])).await;
            if let Ok(credential) = &created {
                let raw = Uint8Array::new(&get(credential, "rawId")).to_vec();
                let _ = call(&storage(), "setItem", &[JsValue::from_str(CREDENTIAL_KEY), JsValue::from_str(&hex(&raw))]);
            }
            created
        }
    };
    match result {
        Ok(_) => BiometricResult::Success,
        Err(e) => match get(&e, "name").as_string().as_deref() {
            // O usuário fechou, ou o tempo acabou.
            Some("NotAllowedError" | "AbortError") => BiometricResult::Cancelled,
            Some(name) => BiometricResult::Failed(get(&e, "message").as_string().unwrap_or_else(|| name.to_string())),
            None => BiometricResult::Failed(format!("{e:?}")),
        },
    }
}

impl BiometricService for WebBiometrics {
    fn availability(&self) -> Biometry {
        match self.available.lock().ok().and_then(|a| *a) {
            Some(true) => Biometry::Any,
            _ => Biometry::None,
        }
    }

    fn authenticate(&mut self, _reason: &str, _allow_device_credential: bool, done: Box<dyn FnOnce(BiometricResult) + Send>) {
        spawn_local(async move { done(verify().await) });
    }
}
