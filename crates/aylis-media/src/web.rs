//! Navegador: o vídeo toca num `<video>` do próprio navegador (imagem e som, com o relógio
//! dele, e todos os formatos que ele souber). A cada quadro novo, a imagem é copiada por
//! um canvas e vai para o `VideoPlayer` como quadro "ao vivo" (o mesmo caminho do
//! [`FrameSink`](crate::FrameSink)), para ele desenhar com cantos, controles por cima e
//! animação, como nos outros sistemas.

use std::sync::{Arc, Mutex};

use aylis_core::{ComponentId, EventArgs, Poster};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

use crate::video::{Frame, Frames, MediaSource};

pub(crate) struct WebVideo {
    video: HtmlVideoElement,
    /// O endereço `blob:` criado para um vídeo que veio em bytes (liberado no fim).
    blob_url: Option<String>,
    interval: i32,
    _tick: Closure<dyn FnMut()>,
    _ended: Closure<dyn FnMut()>,
    _error: Closure<dyn FnMut()>,
    _retry: Closure<dyn FnMut(wasm_bindgen::JsValue)>,
}

fn js_err(e: wasm_bindgen::JsValue) -> String {
    if let Some(text) = e.as_string() {
        return text;
    }
    // As exceções do DOM (`NotAllowedError`, `NotFoundError`...): o nome e a mensagem.
    let field = |key: &str| js_sys::Reflect::get(&e, &key.into()).ok().and_then(|v| v.as_string()).unwrap_or_default();
    match (field("name").as_str(), field("message")) {
        ("NotAllowedError", _) => "o acesso foi negado (permissão do navegador)".into(),
        ("NotFoundError", _) => "nenhum aparelho encontrado".into(),
        ("NotReadableError", _) => "em uso por outro programa".into(),
        ("", _) => format!("{e:?}"),
        (name, message) => format!("{name}: {message}"),
    }
}

impl WebVideo {
    pub(crate) fn open(source: &MediaSource, frames: Arc<Mutex<Frames>>, poster: Poster, id: ComponentId) -> Result<Self, String> {
        let window = web_sys::window().ok_or("sem window")?;
        let document = window.document().ok_or("sem document")?;
        let video: HtmlVideoElement = document.create_element("video").map_err(js_err)?.dyn_into().map_err(|_| "video")?;
        let _ = video.set_attribute("playsinline", "");
        video.set_preload("auto");
        let blob_url = match source {
            MediaSource::File(path) => {
                video.set_src(&path.to_string_lossy());
                None
            }
            MediaSource::Bytes { data, extension } => {
                let array = js_sys::Uint8Array::from(data.as_slice());
                let parts = js_sys::Array::of1(&array);
                let options = web_sys::BlobPropertyBag::new();
                options.set_type(&format!("video/{}", if extension == "mov" { "quicktime" } else { extension.as_str() }));
                let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options).map_err(js_err)?;
                let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(js_err)?;
                video.set_src(&url);
                Some(url)
            }
        };

        let canvas: HtmlCanvasElement = document.create_element("canvas").map_err(js_err)?.dyn_into().map_err(|_| "canvas")?;
        let options = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&options, &"willReadFrequently".into(), &true.into());
        let ctx: CanvasRenderingContext2d = canvas
            .get_context_with_context_options("2d", &options)
            .map_err(js_err)?
            .ok_or("canvas 2d")?
            .dyn_into()
            .map_err(|_| "canvas 2d")?;

        // Copia o quadro quando o tempo do vídeo anda (parado, não gasta nada).
        let tick = {
            let (video, poster) = (video.clone(), poster.clone());
            let mut last = -1.0;
            Closure::<dyn FnMut()>::new(move || {
                if video.ready_state() < 2 {
                    return;
                }
                let (w, h) = (video.video_width(), video.video_height());
                let time = video.current_time();
                if w == 0 || h == 0 || time == last {
                    return;
                }
                last = time;
                if canvas.width() != w || canvas.height() != h {
                    canvas.set_width(w);
                    canvas.set_height(h);
                }
                if ctx.draw_image_with_html_video_element(&video, 0.0, 0.0).is_err() {
                    return;
                }
                let Ok(image) = ctx.get_image_data(0.0, 0.0, w as f64, h as f64) else { return };
                let frame = Frame::rgba(w, h, image.data().0, time);
                if let Ok(mut f) = frames.lock() {
                    f.live = true;
                    f.info.width = w;
                    f.info.height = h;
                    let d = video.duration();
                    f.info.duration = d.is_finite().then_some(d);
                    f.queue.clear();
                    f.queue.push_back(frame);
                }
                poster.post(|ui| ui.request_repaint());
            })
        };
        let interval = window
            .set_interval_with_callback_and_timeout_and_arguments_0(tick.as_ref().unchecked_ref(), 16)
            .map_err(js_err)?;

        let ended = {
            let poster = poster.clone();
            Closure::<dyn FnMut()>::new(move || poster.post(move |ui| { ui.fire(id, "on_end", &EventArgs::None); }))
        };
        video.set_onended(Some(ended.as_ref().unchecked_ref()));
        let error = {
            let (video, poster) = (video.clone(), poster.clone());
            Closure::<dyn FnMut()>::new(move || {
                let message = video.error().map(|e| e.message()).filter(|m| !m.is_empty()).unwrap_or_else(|| "o navegador não conseguiu abrir o vídeo".into());
                poster.post(move |ui| { ui.fire(id, "on_error", &EventArgs::Text(message)); });
            })
        };
        video.set_onerror(Some(error.as_ref().unchecked_ref()));
        // O navegador só deixa tocar com som depois de um clique; sem ele, toca mudo (como
        // o autoplay das páginas).
        let retry = {
            let video = video.clone();
            Closure::<dyn FnMut(wasm_bindgen::JsValue)>::new(move |_| {
                video.set_muted(true);
                let _ = video.play();
            })
        };
        Ok(Self { video, blob_url, interval, _tick: tick, _ended: ended, _error: error, _retry: retry })
    }

    /// A câmera do aparelho (`getUserMedia`; o navegador pede a permissão). `id` vazio: a
    /// padrão; `front`/`back`; senão o `deviceId`. O tamanho vai como o ideal da qualidade
    /// (ou o exato da `resolution`); ao abrir, o que a câmera oferece (o maior tamanho e a
    /// lanterna, onde o navegador diz) vai para `caps` e o componente recebe `on_ready`.
    pub(crate) fn open_camera(
        id: &str,
        request: &crate::camera::CameraRequest,
        caps: Arc<Mutex<Option<crate::camera::CameraCaps>>>,
        frames: Arc<Mutex<Frames>>,
        error: Arc<Mutex<Option<String>>>,
        poster: Poster,
        component: ComponentId,
    ) -> Result<Self, String> {
        let me = Self::open(&MediaSource::File(std::path::PathBuf::new()), frames, poster.clone(), component)?;
        let _ = me.video.remove_attribute("src");
        me.video.set_muted(true);
        let devices = web_sys::window().ok_or("sem window")?.navigator().media_devices().map_err(js_err)?;
        let constraints = web_sys::MediaStreamConstraints::new();
        let set = |obj: &js_sys::Object, key: &str, value: &wasm_bindgen::JsValue| {
            let _ = js_sys::Reflect::set(obj, &key.into(), value);
        };
        let video = js_sys::Object::new();
        match id.trim().to_ascii_lowercase().as_str() {
            "" => {}
            // `front`/`back`: a lente para aquele lado, se houver (sem `exact`: no desktop, a webcam).
            "front" => set(&video, "facingMode", &"user".into()),
            "back" => set(&video, "facingMode", &"environment".into()),
            _ => {
                let exact = js_sys::Object::new();
                set(&exact, "exact", &id.into());
                set(&video, "deviceId", &exact);
            }
        }
        // O tamanho: o exato pedido ou o ideal da qualidade (o navegador fica com o mais perto).
        let (w, h, how) = match request.resolution {
            Some((w, h)) => (w, h, "exact"),
            None => match request.quality {
                crate::camera::CameraQuality::Low => (640, 480, "ideal"),
                crate::camera::CameraQuality::Medium => (1280, 960, "ideal"),
                crate::camera::CameraQuality::High => (1920, 1440, "ideal"),
                crate::camera::CameraQuality::Max => (4096, 3072, "ideal"),
            },
        };
        for (key, value) in [("width", w), ("height", h)] {
            let c = js_sys::Object::new();
            set(&c, how, &value.into());
            set(&video, key, &c);
        }
        constraints.set_video(&video);
        let promise = devices.get_user_media_with_constraints(&constraints).map_err(js_err)?;
        let video = me.video.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(stream) => {
                    if let Ok(stream) = stream.dyn_into::<web_sys::MediaStream>() {
                        video.set_src_object(Some(&stream));
                        let _ = video.play();
                        if let Ok(mut slot) = caps.lock() {
                            *slot = Some(track_caps(&stream));
                        }
                        poster.post(move |ui| {
                            ui.fire(component, "on_ready", &EventArgs::None);
                        });
                    }
                }
                Err(e) => {
                    let message = format!("câmera: {}", js_err(e));
                    // A `CameraView` mostra o erro no lugar da imagem (como no desktop).
                    if let Ok(mut slot) = error.lock() {
                        *slot = Some(message.clone());
                    }
                    poster.post(move |ui| {
                        ui.fire(component, "on_error", &EventArgs::Text(message));
                    });
                }
            }
        });
        Ok(me)
    }

    /// Câmera: liga ou desliga a lanterna (`torch` nas restrições da trilha de vídeo; só onde
    /// o navegador e o aparelho deixam, como o Chrome no Android).
    pub(crate) fn set_torch(&self, on: bool) {
        let Some(track) = video_track(&self.video) else { return };
        let torch = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&torch, &"torch".into(), &on.into());
        let constraints = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&constraints, &"advanced".into(), &js_sys::Array::of1(&torch));
        if let Ok(apply) = js_sys::Reflect::get(&track, &"applyConstraints".into()).and_then(|f| f.dyn_into::<js_sys::Function>()) {
            let _ = apply.call1(&track, &constraints);
        }
    }

    pub(crate) fn play(&self) {
        if let Ok(promise) = self.video.play() {
            let _ = promise.catch(&self._retry);
        }
    }

    pub(crate) fn pause(&self) {
        let _ = self.video.pause();
    }

    pub(crate) fn seek(&self, seconds: f64) {
        self.video.set_current_time(seconds.max(0.0));
    }

    pub(crate) fn set_volume(&self, volume: f32, muted: bool) {
        self.video.set_volume(volume.clamp(0.0, 1.0) as f64);
        self.video.set_muted(muted);
    }

    pub(crate) fn set_looping(&self, looping: bool) {
        self.video.set_loop(looping);
    }

    pub(crate) fn position(&self) -> f64 {
        self.video.current_time()
    }

    pub(crate) fn playing(&self) -> bool {
        !self.video.paused() && !self.video.ended()
    }
}

impl Drop for WebVideo {
    fn drop(&mut self) {
        if let Some(window) = web_sys::window() {
            window.clear_interval_with_handle(self.interval);
        }
        let _ = self.video.pause();
        // A câmera: desliga as trilhas (a luz da câmera apaga).
        if let Some(stream) = self.video.src_object().and_then(|s| s.dyn_into::<web_sys::MediaStream>().ok()) {
            for track in stream.get_tracks().iter() {
                if let Ok(track) = track.dyn_into::<web_sys::MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
        self.video.set_src_object(None);
        self.video.set_onended(None);
        self.video.set_onerror(None);
        let _ = self.video.remove_attribute("src");
        self.video.load();
        if let Some(url) = &self.blob_url {
            let _ = web_sys::Url::revoke_object_url(url);
        }
    }
}

/// A primeira trilha de vídeo da câmera ligada ao `<video>`.
fn video_track(video: &HtmlVideoElement) -> Option<wasm_bindgen::JsValue> {
    let stream = video.src_object()?;
    let tracks = js_sys::Reflect::get(&stream, &"getVideoTracks".into()).ok()?.dyn_into::<js_sys::Function>().ok()?.call0(&stream).ok()?;
    let first = js_sys::Array::from(&tracks).get(0);
    (!first.is_undefined()).then_some(first)
}

/// O que a trilha diz que a câmera oferece (`getCapabilities`, onde o navegador tem): o maior
/// tamanho e a lanterna; sem isso, o tamanho de agora (`getSettings`).
fn track_caps(stream: &web_sys::MediaStream) -> crate::camera::CameraCaps {
    let mut caps = crate::camera::CameraCaps::default();
    let tracks = stream.get_video_tracks();
    let track = tracks.get(0);
    if track.is_undefined() {
        return caps;
    }
    let call = |name: &str| js_sys::Reflect::get(&track, &name.into()).ok().and_then(|f| f.dyn_into::<js_sys::Function>().ok()).and_then(|f| f.call0(&track).ok());
    let number = |obj: &wasm_bindgen::JsValue, path: &[&str]| path.iter().try_fold(obj.clone(), |o, k| js_sys::Reflect::get(&o, &(*k).into()).ok()).and_then(|v| v.as_f64());
    if let Some(c) = call("getCapabilities") {
        if let (Some(w), Some(h)) = (number(&c, &["width", "max"]), number(&c, &["height", "max"])) {
            caps.sizes.push((w as u32, h as u32));
        }
        caps.has_flash = js_sys::Reflect::get(&c, &"torch".into()).ok().is_some_and(|t| t.as_bool() == Some(true));
    }
    if let Some(s) = call("getSettings") {
        if let (Some(w), Some(h)) = (number(&s, &["width"]), number(&s, &["height"])) {
            caps.sizes.push((w as u32, h as u32));
        }
    }
    caps.sizes = crate::camera::sort_sizes(std::mem::take(&mut caps.sizes));
    caps
}
