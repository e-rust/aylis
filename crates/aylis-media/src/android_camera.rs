//! Câmera no Android: Camera2 do NDK (`libcamera2ndk`) com um `AImageReader` em YUV 4:2:0,
//! convertido para RGBA. A permissão é pedida pela `AylisActivity` na primeira vez.

use std::ffi::{CStr, CString};
use std::ptr::null_mut;
use std::time::{Duration, Instant};

use ndk_sys as sys;

use crate::video::{Frame, VideoDecoder, VideoInfo};

#[link(name = "camera2ndk")]
unsafe extern "C" {}

/// `ACAMERA_LENS_FACING` (seção LENS = 8, item 5).
const LENS_FACING: u32 = (8 << 16) + 5;
/// `ACAMERA_SENSOR_ORIENTATION` (seção SENSOR = 14, item 14): quantos graus o sensor está girado
/// em relação à tela na posição natural (90 na traseira e 270 na frontal, quase sempre).
const SENSOR_ORIENTATION: u32 = (14 << 16) + 14;
/// `ACAMERA_SCALER_AVAILABLE_STREAM_CONFIGURATIONS` (seção SCALER = 13, item 10): grupos de
/// quatro `i32` (formato, largura, altura, é entrada).
const STREAM_CONFIGURATIONS: u32 = (13 << 16) + 10;
/// `ACAMERA_FLASH_INFO_AVAILABLE` (seção FLASH_INFO = 5, item 0): a câmera tem flash.
const FLASH_AVAILABLE: u32 = 5 << 16;
/// `ACAMERA_FLASH_MODE` (seção FLASH = 4, item 2): 0 desligado, 2 lanterna (`TORCH`).
const FLASH_MODE: u32 = (4 << 16) + 2;
/// O tamanho de reserva, se a câmera não disser os dela (o de antes, para todas).
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

/// Os tamanhos de saída em YUV 4:2:0 da lista `(formato, largura, altura, é entrada)`.
fn yuv_sizes(configs: &[i32]) -> Vec<(u32, u32)> {
    let yuv = sys::AIMAGE_FORMATS::AIMAGE_FORMAT_YUV_420_888.0 as i32;
    let sizes = configs.chunks_exact(4).filter(|c| c[0] == yuv && c[3] == 0 && c[1] > 0 && c[2] > 0).map(|c| (c[1] as u32, c[2] as u32)).collect();
    crate::camera::sort_sizes(sizes)
}

/// Uma linha de aviso no logcat (etiqueta `aylis`).
pub(crate) fn log(line: &str) {
    #[link(name = "log")]
    unsafe extern "C" {
        fn __android_log_write(prio: i32, tag: *const std::ffi::c_char, text: *const std::ffi::c_char) -> i32;
    }
    if let Ok(text) = CString::new(line) {
        // 5 = ANDROID_LOG_WARN
        unsafe { __android_log_write(5, c"aylis".as_ptr(), text.as_ptr()) };
    }
}

/// A `AylisActivity` em uso (`AylisActivity.current`), achada pelo class loader do
/// aplicativo (o `FindClass` de uma thread nativa só vê as classes do sistema).
pub(crate) fn aylis_activity<'a>(env: &mut jni::JNIEnv<'a>) -> Option<jni::objects::JObject<'a>> {
    let ctx = ndk_context::android_context();
    let app = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
    let found = (|| -> jni::errors::Result<jni::objects::JObject<'a>> {
        let loader = env.call_method(&app, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?.l()?;
        let name = env.new_string("dev.aylis.AylisActivity")?;
        let class = env.call_method(&loader, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[(&name).into()])?.l()?;
        let class = jni::objects::JClass::from(class);
        env.get_static_field(&class, "current", "Ldev/aylis/AylisActivity;")?.l()
    })();
    match found {
        Ok(activity) if !activity.is_null() => Some(activity),
        Ok(_) => None,
        Err(e) => {
            let _ = env.exception_clear();
            log(&format!("AylisActivity: {e}"));
            None
        }
    }
}

/// A rotação da tela em graus (0, 90, 180, 270), pela `AylisActivity`; 0 sem ela.
fn display_rotation() -> i32 {
    let ctx = ndk_context::android_context();
    // SAFETY: a VM do processo, mantida pelo android-activity.
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else { return 0 };
    let Ok(mut env) = vm.attach_current_thread() else { return 0 };
    let Some(activity) = aylis_activity(&mut env) else { return 0 };
    let rotation = (|| -> jni::errors::Result<i32> {
        let manager = env.call_method(&activity, "getWindowManager", "()Landroid/view/WindowManager;", &[])?.l()?;
        let display = env.call_method(&manager, "getDefaultDisplay", "()Landroid/view/Display;", &[])?.l()?;
        env.call_method(&display, "getRotation", "()I", &[])?.i()
    })();
    match rotation {
        Ok(r) => r.clamp(0, 3) * 90,
        Err(_) => {
            let _ = env.exception_clear();
            0
        }
    }
}

/// Pede a permissão `name` (`android.permission.CAMERA`...) pela `AylisActivity` e espera
/// a resposta do usuário (até 60 s). `true` se concedida (ou sem a `AylisActivity`).
pub(crate) fn ensure_permission(name: &str) -> bool {
    let ctx = ndk_context::android_context();
    // SAFETY: a VM e a activity do processo, mantidas pelo android-activity.
    let vm = match unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) } {
        Ok(vm) => vm,
        Err(e) => {
            log(&format!("permissão {name}: VM {e}"));
            return true;
        }
    };
    let mut env = match vm.attach_current_thread() {
        Ok(env) => env,
        Err(e) => {
            log(&format!("permissão {name}: thread {e}"));
            return true;
        }
    };
    // Sem a AylisActivity (outro host): deixa o sistema decidir.
    let Some(activity) = aylis_activity(&mut env) else { return true };
    let started = Instant::now();
    let Ok(jname) = env.new_string(name) else { return true };
    let asked = env.call_method(&activity, "ensurePermission", "(Ljava/lang/String;)Z", &[(&jname).into()]).and_then(|v| v.z());
    if let Ok(true) = asked {
        return true;
    }
    loop {
        // 1: concedida; 0: o usuário ainda não respondeu; -1: negada.
        let state = env.call_method(&activity, "permissionState", "(Ljava/lang/String;)I", &[(&jname).into()]).and_then(|v| v.i());
        match state {
            Ok(1) => return true,
            Ok(-1) => return false,
            Ok(_) => {}
            Err(e) => {
                // Sem a AylisActivity (outro host): deixa o sistema decidir.
                let _ = env.exception_describe();
                let _ = env.exception_clear();
                log(&format!("permissão {name}: {e}"));
                return true;
            }
        }
        if started.elapsed() > Duration::from_secs(300) {
            return false;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

/// A permissão já foi concedida? (sem esperar; `true` sem a `AylisActivity`)
pub(crate) fn has_permission(name: &str) -> bool {
    let ctx = ndk_context::android_context();
    // SAFETY: a VM e a activity do processo, mantidas pelo android-activity.
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else { return true };
    let Ok(mut env) = vm.attach_current_thread() else { return true };
    let Some(activity) = aylis_activity(&mut env) else { return true };
    let Ok(jname) = env.new_string(name) else { return true };
    match env.call_method(&activity, "hasPermission", "(Ljava/lang/String;)Z", &[(&jname).into()]).and_then(|v| v.z()) {
        Ok(g) => g,
        Err(e) => {
            let _ = env.exception_describe();
            let _ = env.exception_clear();
            log(&format!("permissão {name}: {e}"));
            true
        }
    }
}

/// As câmeras: (nome, id, para onde a lente aponta). O nome diz se é a traseira ou a frontal.
pub fn cameras() -> Vec<(String, String, Option<crate::camera::CameraFacing>)> {
    let mut out = Vec::new();
    // SAFETY: a lista e os metadados são liberados aqui mesmo.
    unsafe {
        let manager = sys::ACameraManager_create();
        if manager.is_null() {
            return out;
        }
        let mut list: *mut sys::ACameraIdList = null_mut();
        if sys::ACameraManager_getCameraIdList(manager, &mut list) == sys::camera_status_t::ACAMERA_OK && !list.is_null() {
            for i in 0..(*list).numCameras as usize {
                let raw = *(*list).cameraIds.add(i);
                let id = CStr::from_ptr(raw).to_string_lossy().into_owned();
                let mut meta: *mut sys::ACameraMetadata = null_mut();
                let mut facing = None;
                if sys::ACameraManager_getCameraCharacteristics(manager, raw, &mut meta) == sys::camera_status_t::ACAMERA_OK {
                    let mut entry: sys::ACameraMetadata_const_entry = std::mem::zeroed();
                    if sys::ACameraMetadata_getConstEntry(meta, LENS_FACING, &mut entry) == sys::camera_status_t::ACAMERA_OK && entry.count > 0 {
                        facing = Some(*entry.data.u8_);
                    }
                    sys::ACameraMetadata_free(meta);
                }
                let name = match facing {
                    Some(0) => aylis_core::messages::t("câmera frontal"),
                    Some(1) => aylis_core::messages::t("câmera traseira"),
                    _ => aylis_core::messages::t("câmera externa"),
                };
                let side = match facing {
                    Some(0) => Some(crate::camera::CameraFacing::Front),
                    Some(1) => Some(crate::camera::CameraFacing::Back),
                    Some(_) => Some(crate::camera::CameraFacing::External),
                    None => None,
                };
                out.push((format!("{name} ({id})"), id, side));
            }
            sys::ACameraManager_deleteCameraIdList(list);
        }
        sys::ACameraManager_delete(manager);
    }
    out
}

/// A câmera aberta, entregando quadros no tamanho escolhido pela qualidade pedida.
struct AndroidCamera {
    caps: crate::camera::CameraCaps,
    manager: *mut sys::ACameraManager,
    device: *mut sys::ACameraDevice,
    reader: *mut sys::AImageReader,
    session: *mut sys::ACameraCaptureSession,
    request: *mut sys::ACaptureRequest,
    output: *mut sys::ACaptureSessionOutput,
    container: *mut sys::ACaptureSessionOutputContainer,
    target: *mut sys::ACameraOutputTarget,
    info: VideoInfo,
    started: Instant,
    error: Option<String>,
    /// A montagem do sensor (graus) e se a lente é a da tela (o giro corre ao contrário).
    sensor: i32,
    front: bool,
    /// A rotação da tela (0, 90, 180, 270) e quando foi lida (muda quando o celular gira).
    display: i32,
    display_read: Option<Instant>,
    /// O tamanho em que a imagem aparece (o `set_output_size`): a conversão reduz até perto
    /// dele. E o tamanho cheio da câmera, girado.
    want: (u32, u32),
    full_size: (u32, u32),
    /// O último quadro cru, quando a tela mostra o reduzido (para a foto cheia).
    raw: std::sync::Arc<std::sync::Mutex<Option<RawYuv>>>,
    // As callbacks que o NDK guarda pelo ponteiro: vivem tanto quanto a câmera (o `Drop`
    // fecha a sessão e o aparelho antes de elas saírem).
    device_cb: Box<sys::ACameraDevice_StateCallbacks>,
    session_cb: Box<sys::ACameraCaptureSession_stateCallbacks>,
}

unsafe extern "C" fn device_event(_: *mut std::ffi::c_void, _: *mut sys::ACameraDevice) {}
unsafe extern "C" fn device_error(_: *mut std::ffi::c_void, _: *mut sys::ACameraDevice, _: std::os::raw::c_int) {}
unsafe extern "C" fn session_event(_: *mut std::ffi::c_void, _: *mut sys::ACameraCaptureSession) {}

fn check(status: sys::camera_status_t, what: &str) -> Result<(), String> {
    if status == sys::camera_status_t::ACAMERA_OK { Ok(()) } else { Err(format!("câmera: {what} ({})", status.0)) }
}

/// Abre a câmera `id` (de [`cameras`]).
pub fn open_camera(id: &str, request: &crate::camera::CameraRequest) -> Result<Box<dyn VideoDecoder>, String> {
    if !ensure_permission("android.permission.CAMERA") {
        return Err(aylis_core::messages::t("sem permissão para a câmera"));
    }
    let cid = CString::new(id).map_err(|e| e.to_string())?;
    let mut cam = AndroidCamera {
        caps: Default::default(),
        manager: null_mut(),
        device: null_mut(),
        reader: null_mut(),
        session: null_mut(),
        request: null_mut(),
        output: null_mut(),
        container: null_mut(),
        target: null_mut(),
        info: VideoInfo { width: WIDTH, height: HEIGHT, duration: None, fps: None },
        started: Instant::now(),
        error: None,
        sensor: 0,
        front: false,
        display: 0,
        display_read: None,
        want: (0, 0),
        full_size: (0, 0),
        raw: Default::default(),
        device_cb: Box::new(sys::ACameraDevice_StateCallbacks { context: null_mut(), onDisconnected: Some(device_event), onError: Some(device_error) }),
        session_cb: Box::new(sys::ACameraCaptureSession_stateCallbacks {
            context: null_mut(),
            onClosed: Some(session_event),
            onReady: Some(session_event),
            onActive: Some(session_event),
        }),
    };
    // SAFETY: cada objeto criado aqui é guardado em `cam` e liberado no `Drop`, inclusive
    // num erro no meio do caminho; as callbacks também ficam em `cam`.
    unsafe {
        cam.manager = sys::ACameraManager_create();
        if cam.manager.is_null() {
            return Err("câmera: ACameraManager".into());
        }
        check(sys::ACameraManager_openCamera(cam.manager, cid.as_ptr(), &mut *cam.device_cb, &mut cam.device), "abrir")?;
        let mut meta: *mut sys::ACameraMetadata = null_mut();
        if sys::ACameraManager_getCameraCharacteristics(cam.manager, cid.as_ptr(), &mut meta) == sys::camera_status_t::ACAMERA_OK {
            let mut entry: sys::ACameraMetadata_const_entry = std::mem::zeroed();
            if sys::ACameraMetadata_getConstEntry(meta, SENSOR_ORIENTATION, &mut entry) == sys::camera_status_t::ACAMERA_OK && entry.count > 0 {
                cam.sensor = *entry.data.i32_;
            }
            if sys::ACameraMetadata_getConstEntry(meta, LENS_FACING, &mut entry) == sys::camera_status_t::ACAMERA_OK && entry.count > 0 {
                cam.front = *entry.data.u8_ == 0;
            }
            if sys::ACameraMetadata_getConstEntry(meta, STREAM_CONFIGURATIONS, &mut entry) == sys::camera_status_t::ACAMERA_OK && entry.count > 0 {
                cam.caps.sizes = yuv_sizes(std::slice::from_raw_parts(entry.data.i32_, entry.count as usize));
            }
            if sys::ACameraMetadata_getConstEntry(meta, FLASH_AVAILABLE, &mut entry) == sys::camera_status_t::ACAMERA_OK && entry.count > 0 {
                cam.caps.has_flash = *entry.data.u8_ != 0;
            }
            sys::ACameraMetadata_free(meta);
        }
        let size = crate::camera::choose_size(&cam.caps.sizes, request).unwrap_or((WIDTH, HEIGHT));
        log(&format!("câmera {id}: {}x{} ({:?})", size.0, size.1, request.quality));
        cam.info.width = size.0;
        cam.info.height = size.1;
        if sys::AImageReader_new(size.0 as i32, size.1 as i32, sys::AIMAGE_FORMATS::AIMAGE_FORMAT_YUV_420_888.0 as i32, 3, &mut cam.reader) != sys::media_status_t::AMEDIA_OK {
            return Err("câmera: AImageReader".into());
        }
        let mut window: *mut sys::ANativeWindow = null_mut();
        if sys::AImageReader_getWindow(cam.reader, &mut window) != sys::media_status_t::AMEDIA_OK {
            return Err("câmera: janela do AImageReader".into());
        }
        check(sys::ACaptureSessionOutputContainer_create(&mut cam.container), "saídas")?;
        check(sys::ACaptureSessionOutput_create(window.cast(), &mut cam.output), "saída")?;
        check(sys::ACaptureSessionOutputContainer_add(cam.container, cam.output), "saída")?;
        check(sys::ACameraOutputTarget_create(window.cast(), &mut cam.target), "alvo")?;
        check(sys::ACameraDevice_createCaptureRequest(cam.device, sys::ACameraDevice_request_template::TEMPLATE_PREVIEW, &mut cam.request), "pedido")?;
        check(sys::ACaptureRequest_addTarget(cam.request, cam.target), "alvo")?;
        check(sys::ACameraDevice_createCaptureSession(cam.device, cam.container, &*cam.session_cb, &mut cam.session), "sessão")?;
        let mut requests = [cam.request];
        check(sys::ACameraCaptureSession_setRepeatingRequest(cam.session, null_mut(), 1, requests.as_mut_ptr(), null_mut()), "captura")?;
    }
    Ok(Box::new(cam))
}

impl AndroidCamera {
    /// O quadro mais recente em RGBA, se já chegou algum.
    fn take(&mut self) -> Option<(u32, u32, Vec<u8>)> {
        // SAFETY: a imagem é lida e devolvida ao leitor aqui mesmo.
        unsafe {
            let mut image: *mut sys::AImage = null_mut();
            if sys::AImageReader_acquireLatestImage(self.reader, &mut image) != sys::media_status_t::AMEDIA_OK || image.is_null() {
                return None;
            }
            let (mut w, mut h) = (0, 0);
            sys::AImage_getWidth(image, &mut w);
            sys::AImage_getHeight(image, &mut h);
            let plane = |i: i32| {
                let (mut data, mut len, mut row, mut pixel) = (null_mut(), 0, 0, 0);
                sys::AImage_getPlaneData(image, i, &mut data, &mut len);
                sys::AImage_getPlaneRowStride(image, i, &mut row);
                sys::AImage_getPlanePixelStride(image, i, &mut pixel);
                let slice: &[u8] = if data.is_null() { &[] } else { std::slice::from_raw_parts(data, len as usize) };
                (slice, row.max(0) as usize, pixel.max(1) as usize)
            };
            let ((py, ry, _), (pu, ru, su), (pv, rv, sv)) = (plane(0), plane(1), plane(2));
            let (w, h) = (w.max(0) as usize, h.max(0) as usize);
            // De pé na tela: o sensor está montado girado; a frontal gira no outro sentido.
            if self.display_read.is_none_or(|t| t.elapsed() > Duration::from_millis(500)) {
                self.display = display_rotation();
                self.display_read = Some(Instant::now());
            }
            let degrees = if self.front { self.sensor + self.display } else { self.sensor - self.display };
            // O tamanho cheio, já girado (o `frame_size`), e o passo até o tamanho da tela.
            let (fw, fh) = if degrees.rem_euclid(180) == 90 { (h, w) } else { (w, h) };
            self.full_size = (fw as u32, fh as u32);
            let step = match self.want {
                (ww, wh) if ww > 0 && wh > 0 => (fw / ww as usize).min(fh / wh as usize).max(1),
                _ => 1,
            };
            // Reduzido: o quadro cru fica guardado para a foto em tamanho cheio.
            if step > 1 {
                if let Ok(mut raw) = self.raw.lock() {
                    // Os mesmos buffers a cada quadro (18 MB num de 12 MP): só a cópia.
                    let r = raw.get_or_insert_with(|| RawYuv { y: Vec::new(), u: Vec::new(), v: Vec::new(), rows: (0, 0, 0), pixels: (1, 1), width: 0, height: 0, degrees: 0 });
                    for (to, from) in [(&mut r.y, py), (&mut r.u, pu), (&mut r.v, pv)] {
                        to.clear();
                        to.extend_from_slice(from);
                    }
                    (r.rows, r.pixels, r.width, r.height, r.degrees) = ((ry, ru, rv), (su, sv), w, h, degrees);
                }
            } else if let Ok(mut raw) = self.raw.lock() {
                *raw = None;
            }
            // BT.601 de faixa completa (o padrão das câmeras), já girado, nos vários núcleos.
            use crate::camera::{Plane, yuv420_to_rgba};
            let (w, h, out) = yuv420_to_rgba(
                Plane { data: py, row: ry, pixel: 1 },
                Plane { data: pu, row: ru, pixel: su },
                Plane { data: pv, row: rv, pixel: sv },
                w,
                h,
                degrees,
                step,
            );
            sys::AImage_delete(image);
            Some((w as u32, h as u32, out))
        }
    }
}

/// Uma cópia do último quadro da câmera em YUV, para a foto em tamanho cheio quando a tela
/// mostra os quadros reduzidos.
struct RawYuv {
    y: Vec<u8>,
    u: Vec<u8>,
    v: Vec<u8>,
    /// O passo da linha de Y, U e V e o do pixel de U e V.
    rows: (usize, usize, usize),
    pixels: (usize, usize),
    width: usize,
    height: usize,
    degrees: i32,
}

impl RawYuv {
    fn to_frame(&self) -> Frame {
        use crate::camera::{Plane, yuv420_to_rgba};
        let (w, h, px) = yuv420_to_rgba(
            Plane { data: &self.y, row: self.rows.0, pixel: 1 },
            Plane { data: &self.u, row: self.rows.1, pixel: self.pixels.0 },
            Plane { data: &self.v, row: self.rows.2, pixel: self.pixels.1 },
            self.width,
            self.height,
            self.degrees,
            1,
        );
        Frame::rgba(w as u32, h as u32, px, 0.0)
    }
}

impl VideoDecoder for AndroidCamera {
    fn info(&self) -> VideoInfo {
        self.info
    }

    fn seek(&mut self, _seconds: f64) {}

    fn next_frame(&mut self) -> Option<Frame> {
        let waited = Instant::now();
        loop {
            if let Some((w, h, px)) = self.take() {
                // O tamanho da câmera (o quadro pode vir reduzido para a tela).
                (self.info.width, self.info.height) = self.full_size;
                return Some(Frame::rgba(w, h, px, self.started.elapsed().as_secs_f64()));
            }
            if waited.elapsed() > Duration::from_secs(5) {
                self.error = Some(aylis_core::messages::t("a câmera não mandou imagem"));
                return None;
            }
            std::thread::sleep(Duration::from_millis(8));
        }
    }

    fn camera_caps(&self) -> Option<crate::camera::CameraCaps> {
        Some(self.caps.clone())
    }

    fn set_output_size(&mut self, width: u32, height: u32) {
        self.want = (width, height);
    }

    fn full_frame(&self) -> Option<crate::video::FullFrame> {
        let raw = self.raw.clone();
        Some(std::sync::Arc::new(move || raw.lock().ok()?.as_ref().map(RawYuv::to_frame)))
    }

    /// A lanterna: o modo do flash no pedido que se repete, mandado de novo à sessão.
    fn set_torch(&mut self, on: bool) -> bool {
        if !self.caps.has_flash || self.request.is_null() || self.session.is_null() {
            return false;
        }
        let mode: u8 = if on { 2 } else { 0 };
        // SAFETY: o pedido e a sessão vivem com a câmera (liberados no `Drop`).
        unsafe {
            if sys::ACaptureRequest_setEntry_u8(self.request, FLASH_MODE, 1, &mode) != sys::camera_status_t::ACAMERA_OK {
                return false;
            }
            let mut requests = [self.request];
            sys::ACameraCaptureSession_setRepeatingRequest(self.session, null_mut(), 1, requests.as_mut_ptr(), null_mut()) == sys::camera_status_t::ACAMERA_OK
        }
    }

    fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl Drop for AndroidCamera {
    fn drop(&mut self) {
        // SAFETY: na ordem inversa da criação; os nulos (erro no meio) são pulados.
        unsafe {
            if !self.session.is_null() {
                sys::ACameraCaptureSession_close(self.session);
            }
            if !self.request.is_null() {
                sys::ACaptureRequest_free(self.request);
            }
            if !self.target.is_null() {
                sys::ACameraOutputTarget_free(self.target);
            }
            if !self.output.is_null() {
                sys::ACaptureSessionOutput_free(self.output);
            }
            if !self.container.is_null() {
                sys::ACaptureSessionOutputContainer_free(self.container);
            }
            if !self.device.is_null() {
                sys::ACameraDevice_close(self.device);
            }
            if !self.reader.is_null() {
                sys::AImageReader_delete(self.reader);
            }
            if !self.manager.is_null() {
                sys::ACameraManager_delete(self.manager);
            }
        }
    }
}
