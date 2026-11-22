//! Navegador: os controles de mídia do sistema pela Media Session API (o que toca na aba, no
//! controle de mídia do Chrome e do Edge, na tela bloqueada do celular e nas teclas de mídia)
//! para o `MediaSession`. Sem a API (navegadores antigos), o componente não liga.

use aylis_core::ComponentId;
use aylis_core::platform::{MediaCommand, MediaCommandSink, MediaSessionService, NowPlaying};
use js_sys::{Array, Function, Reflect, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::web_device::{call, get, object, set, window};

#[derive(Default)]
pub(crate) struct WebMediaSession {
    /// O endereço da capa (`blob:`), solto quando ela muda.
    cover_url: String,
    cover: Option<Vec<u8>>,
}

fn session() -> Option<JsValue> {
    let s = get(&get(&window(), "navigator"), "mediaSession");
    (!s.is_undefined() && !s.is_null()).then_some(s)
}

impl MediaSessionService for WebMediaSession {
    fn start(&mut self, _owner: Option<ComponentId>, commands: MediaCommandSink) -> Result<(), String> {
        let s = session().ok_or("o navegador não tem a Media Session API")?;
        let actions: [(&str, Option<MediaCommand>); 5] =
            [("play", Some(MediaCommand::Play)), ("pause", Some(MediaCommand::Pause)), ("stop", Some(MediaCommand::Stop)), ("nexttrack", Some(MediaCommand::Next)), ("previoustrack", Some(MediaCommand::Previous))];
        for (name, cmd) in actions {
            let sink = commands.clone();
            let handler = Closure::<dyn Fn(JsValue)>::new(move |_details: JsValue| {
                if let Some(c) = cmd {
                    sink(c);
                }
            });
            // Uma ação que o navegador não conhece dá erro: as outras seguem.
            let _ = call(&s, "setActionHandler", &[JsValue::from_str(name), handler.as_ref().clone()]);
            handler.forget();
        }
        let sink = commands;
        let seek = Closure::<dyn Fn(JsValue)>::new(move |details: JsValue| {
            if let Some(t) = get(&details, "seekTime").as_f64() {
                sink(MediaCommand::Seek(t));
            }
        });
        let _ = call(&s, "setActionHandler", &[JsValue::from_str("seekto"), seek.as_ref().clone()]);
        seek.forget();
        Ok(())
    }

    fn update(&mut self, now: &NowPlaying) {
        let Some(s) = session() else { return };
        if now.cover != self.cover {
            if !self.cover_url.is_empty() {
                let _ = call(&get(&window(), "URL"), "revokeObjectURL", &[JsValue::from_str(&self.cover_url)]);
                self.cover_url.clear();
            }
            if let Some(bytes) = &now.cover {
                let parts = Array::of1(&Uint8Array::from(bytes.as_slice()));
                let blob_class = get(&window(), "Blob");
                if let Ok(blob) = Reflect::construct(blob_class.unchecked_ref::<Function>(), &Array::of1(&parts)) {
                    self.cover_url = call(&get(&window(), "URL"), "createObjectURL", &[blob]).ok().and_then(|u| u.as_string()).unwrap_or_default();
                }
            }
            self.cover = now.cover.clone();
        }
        let init = object();
        set(&init, "title", now.title.as_str());
        set(&init, "artist", now.artist.as_str());
        set(&init, "album", now.album.as_str());
        if !self.cover_url.is_empty() {
            let art = object();
            set(&art, "src", self.cover_url.as_str());
            set(&init, "artwork", Array::of1(&art));
        }
        let meta_class = get(&window(), "MediaMetadata");
        if let Ok(meta) = Reflect::construct(meta_class.unchecked_ref::<Function>(), &Array::of1(&init)) {
            set(&s, "metadata", meta);
        }
        set(&s, "playbackState", if now.playing { "playing" } else { "paused" });
        if now.duration > 0.0 {
            let state = object();
            set(&state, "duration", now.duration);
            set(&state, "position", now.position.clamp(0.0, now.duration));
            let _ = call(&s, "setPositionState", &[state]);
        }
    }

    fn stop(&mut self) {
        if let Some(s) = session() {
            set(&s, "metadata", JsValue::NULL);
            set(&s, "playbackState", "none");
        }
    }
}
